/*
 * xpath.c: XML Path Language implementation
 *          XPath is a language for addressing parts of an XML document,
 *          designed to be used by both XSLT and XPointer
 *
 * Reference: W3C Recommendation 16 November 1999
 *     http://www.w3.org/TR/1999/REC-xpath-19991116
 * Public reference:
 *     http://www.w3.org/TR/xpath
 *
 * See Copyright for the status of this software
 *
 * Author: daniel@veillard.com
 *
 */

/* To avoid EBCDIC trouble when parsing on zOS */
#if defined(__MVS__)
#pragma convert("ISO8859-1")
#endif

#define IN_LIBXML
#include "libxml.h"

#include <limits.h>
#include <string.h>
#include <stddef.h>

#ifdef HAVE_SYS_TYPES_H
#include <sys/types.h>
#endif
#ifdef HAVE_MATH_H
#include <math.h>
#endif
#ifdef HAVE_FLOAT_H
#include <float.h>
#endif
#ifdef HAVE_CTYPE_H
#include <ctype.h>
#endif
#ifdef HAVE_SIGNAL_H
#include <signal.h>
#endif

#include <libxml/xmlmemory.h>
#include <libxml/tree.h>
#include <libxml/valid.h>
#include <libxml/xpath.h>
#include <libxml/xpathInternals.h>
#include <libxml/parserInternals.h>
#include <libxml/hash.h>
#ifdef LIBXML_XPTR_ENABLED
#include <libxml/xpointer.h>
#endif
#ifdef LIBXML_DEBUG_ENABLED
#include <libxml/debugXML.h>
#endif
#include <libxml/xmlerror.h>
#include <libxml/threads.h>
#include <libxml/globals.h>
#ifdef LIBXML_PATTERN_ENABLED
#include <libxml/pattern.h>
#endif

#include "buf.h"

unsigned int xmlParserMaxDepth = 256;
#ifdef LIBXML_PATTERN_ENABLED
#define XPATH_STREAMING
#endif

#define TODO                                          \
	xmlGenericError(xmlGenericErrorContext,           \
					"Unimplemented block at %s:%d\n", \
					__FILE__, __LINE__);

/**
 * WITH_TIM_SORT:
 *
 * Use the Timsort algorithm provided in timsort.h to sort
 * nodeset as this is a great improvement over the old Shell sort
 * used in xmlXPathNodeSetSort()
 */
#define WITH_TIM_SORT

/*
 * XP_OPTIMIZED_NON_ELEM_COMPARISON:
 * If defined, this will use xmlXPathCmpNodesExt() instead of
 * xmlXPathCmpNodes(). The new function is optimized comparison of
 * non-element nodes; actually it will speed up comparison only if
 * xmlXPathOrderDocElems() was called in order to index the elements of
 * a tree in document order; Libxslt does such an indexing, thus it will
 * benefit from this optimization.
 */
#define XP_OPTIMIZED_NON_ELEM_COMPARISON

/*
 * XP_OPTIMIZED_FILTER_FIRST:
 * If defined, this will optimize expressions like "key('foo', 'val')[b][1]"
 * in a way, that it stop evaluation at the first node.
 */
#define XP_OPTIMIZED_FILTER_FIRST

/*
 * XPATH_MAX_STEPS:
 * when compiling an XPath expression we arbitrary limit the maximum
 * number of step operation in the compiled expression. 1000000 is
 * an insanely large value which should never be reached under normal
 * circumstances
 */
#define XPATH_MAX_STEPS 1000000

/*
 * XPATH_MAX_STACK_DEPTH:
 * when evaluating an XPath expression we arbitrary limit the maximum
 * number of object allowed to be pushed on the stack. 1000000 is
 * an insanely large value which should never be reached under normal
 * circumstances
 */
#define XPATH_MAX_STACK_DEPTH 1000000

/*
 * XPATH_MAX_NODESET_LENGTH:
 * when evaluating an XPath expression nodesets are created and we
 * arbitrary limit the maximum length of those node set. 10000000 is
 * an insanely large value which should never be reached under normal
 * circumstances, one would first need to construct an in memory tree
 * with more than 10 millions nodes.
 */
#define XPATH_MAX_NODESET_LENGTH 10000000

/*
 * XPATH_MAX_RECRUSION_DEPTH:
 * Maximum amount of nested functions calls when parsing or evaluating
 * expressions
 */
#ifdef FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION
#define XPATH_MAX_RECURSION_DEPTH 500
#else
#define XPATH_MAX_RECURSION_DEPTH 5000
#endif

/*
 * Wrapper for the Timsort algorithm from timsort.h
 */
#ifdef WITH_TIM_SORT
#define SORT_NAME libxml_domnode
#define SORT_TYPE xmlNodePtr

#ifdef XP_OPTIMIZED_NON_ELEM_COMPARISON

#else

#endif
#include "timsort.h"
#endif /* WITH_TIM_SORT */

#ifdef LIBXML_XPATH_ENABLED

/*
 * TODO: when compatibility allows remove all "fake node libxslt" strings
 *       the test should just be name[0] = ' '
 */
#ifdef DEBUG_XPATH_EXPRESSION
#define DEBUG_STEP
#define DEBUG_EXPR
#define DEBUG_EVAL_COUNTS
#endif

#define STRANGE                                  \
	xmlGenericError(xmlGenericErrorContext,      \
					"Internal error at %s:%d\n", \
					__FILE__, __LINE__);

#define XP_HAS_CACHE(c) ((c != NULL) && ((c)->cache != NULL))

#define CUR (*ctxt->cur)
#define SKIP(val) ctxt->cur += (val)
#define NXT(val) ctxt->cur[(val)]
#define CUR_PTR ctxt->cur

#define COPY_BUF(l, b, i, v) \
	if (l == 1)              \
		b[i++] = (xmlChar)v; \
	else                     \
		i += xmlCopyChar(l, &b[i], v)

#define NEXTL(l) ctxt->cur += l

#define SKIP_BLANKS                   \
	while (IS_BLANK_CH(*(ctxt->cur))) \
	NEXT

#define CURRENT (*ctxt->cur)
#define NEXT ((*ctxt->cur) ? ctxt->cur++ : ctxt->cur)

#ifndef DBL_DIG
#define DBL_DIG 16
#endif
#ifndef DBL_EPSILON
#define DBL_EPSILON 1E-9
#endif

#define UPPER_DOUBLE 1E9
#define LOWER_DOUBLE 1E-5
#define LOWER_DOUBLE_EXP 5

#define INTEGER_DIGITS DBL_DIG
#define FRACTION_DIGITS (DBL_DIG + 1 + (LOWER_DOUBLE_EXP))
#define EXPONENT_DIGITS (3 + 2)

#define XML_NODESET_DEFAULT 10

#define CHECK_CTXT(ctxt)                                       \
	if (ctxt == NULL)                                          \
	{                                                          \
		__xmlRaiseError(NULL, NULL, NULL,                      \
						NULL, NULL, XML_FROM_XPATH,            \
						XML_ERR_INTERNAL_ERROR, XML_ERR_FATAL, \
						__FILE__, __LINE__,                    \
						NULL, NULL, NULL, 0, 0,                \
						"NULL context pointer\n");             \
		return (NULL);                                         \
	}

#define CHECK_CTXT_NEG(ctxt)                                   \
	if (ctxt == NULL)                                          \
	{                                                          \
		__xmlRaiseError(NULL, NULL, NULL,                      \
						NULL, NULL, XML_FROM_XPATH,            \
						XML_ERR_INTERNAL_ERROR, XML_ERR_FATAL, \
						__FILE__, __LINE__,                    \
						NULL, NULL, NULL, 0, 0,                \
						"NULL context pointer\n");             \
		return (-1);                                           \
	}

#define CHECK_CONTEXT(ctxt)                                          \
	if ((ctxt == NULL) || (ctxt->doc == NULL) ||                     \
		(ctxt->doc->children == NULL))                               \
	{                                                                \
		xmlXPatherror(ctxt, __FILE__, __LINE__, XPATH_INVALID_CTXT); \
		return (NULL);                                               \
	}

#define IS_FUNCTION 200
#define MAX_FRAC 20

#endif /*LIBXML_XPATH_ENABLED*/


#ifdef COMPILE_WITH_RUST

extern int xmlXPathContextSetCache_rust(xmlXPathContextPtr ctxt, int active, int value, int options);
extern xmlXPathObjectPtr valuePop_rust(xmlXPathParserContextPtr ctxt);
extern int valuePush_rust(xmlXPathParserContextPtr ctxt, xmlXPathObjectPtr value);
extern int xmlXPathPopBoolean_rust(xmlXPathParserContextPtr ctxt);
extern double xmlXPathPopNumber_rust(xmlXPathParserContextPtr ctxt);
extern xmlChar *xmlXPathPopString_rust(xmlXPathParserContextPtr ctxt);
extern void *xmlXPathPopExternal_rust(xmlXPathParserContextPtr ctxt);
extern long xmlXPathOrderDocElems_rust(xmlDocPtr doc);
extern int xmlXPathCmpNodes_rust(xmlNodePtr node1, xmlNodePtr node2);
extern void xmlXPathNodeSetSort_rust(xmlNodeSetPtr set);
extern void xmlXPathNodeSetFreeNs_rust(xmlNsPtr ns);
extern xmlNodeSetPtr xmlXPathNodeSetCreate_rust(xmlNodePtr val);
extern int xmlXPathNodeSetContains_rust(xmlNodeSetPtr cur, xmlNodePtr val);
extern int xmlXPathNodeSetAddNs_rust(xmlNodeSetPtr cur, xmlNodePtr node, xmlNsPtr ns);
extern int xmlXPathNodeSetAdd_rust(xmlNodeSetPtr cur, xmlNodePtr val);
extern int xmlXPathNodeSetAddUnique_rust(xmlNodeSetPtr cur, xmlNodePtr val);
extern xmlNodeSetPtr xmlXPathNodeSetMerge_rust(xmlNodeSetPtr val1, xmlNodeSetPtr val2);
extern void xmlXPathNodeSetDel_rust(xmlNodeSetPtr cur, xmlNodePtr val);
extern void xmlXPathNodeSetRemove_rust(xmlNodeSetPtr cur, int val);
extern void xmlXPathFreeNodeSet_rust(xmlNodeSetPtr obj);
extern xmlXPathObjectPtr xmlXPathNewNodeSetList_rust(xmlNodeSetPtr val);
extern xmlNodeSetPtr xmlXPathDifference_rust(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2);
extern xmlNodeSetPtr xmlXPathIntersection_rust(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2);
extern xmlNodeSetPtr xmlXPathDistinctSorted_rust(xmlNodeSetPtr nodes);
extern xmlNodeSetPtr xmlXPathDistinct_rust(xmlNodeSetPtr nodes);
extern int xmlXPathHasSameNodes_rust(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2);
extern xmlNodeSetPtr xmlXPathNodeLeadingSorted_rust(xmlNodeSetPtr nodes, xmlNodePtr node);
extern xmlNodeSetPtr xmlXPathNodeLeading_rust(xmlNodeSetPtr nodes, xmlNodePtr node);
extern xmlNodeSetPtr xmlXPathLeadingSorted_rust(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2);
extern xmlNodeSetPtr xmlXPathLeading_rust(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2);
extern xmlNodeSetPtr xmlXPathNodeTrailingSorted_rust(xmlNodeSetPtr nodes, xmlNodePtr node);
extern xmlNodeSetPtr xmlXPathNodeTrailing_rust(xmlNodeSetPtr nodes, xmlNodePtr node);
extern xmlNodeSetPtr xmlXPathTrailingSorted_rust(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2);
extern xmlNodeSetPtr xmlXPathTrailing_rust(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2);
extern int xmlXPathRegisterFunc_rust(xmlXPathContextPtr ctxt, const xmlChar *name, xmlXPathFunction f);
extern void xmlXPathErr_rust(xmlXPathParserContextPtr ctxt, int error);
extern void xmlXPatherror_rust(xmlXPathParserContextPtr ctxt, const char *file ATTRIBUTE_UNUSED, int line ATTRIBUTE_UNUSED, int no);
extern xmlNodeSetPtr xmlXPathPopNodeSet_rust(xmlXPathParserContextPtr ctxt);
extern void xmlXPathFreeCompExpr_rust(xmlXPathCompExprPtr comp);
extern void xmlXPathDebugDumpObject_rust(FILE *output, xmlXPathObjectPtr cur, int depth);
extern void xmlXPathInit_rust();
extern double xmlXPathCastNodeToNumber_rust(xmlNodePtr node);
extern double xmlXPathCastNodeSetToNumber_rust(xmlNodeSetPtr ns);
extern void xmlXPathModValues_rust(xmlXPathParserContextPtr ctxt);
extern double xmlXPathStringEvalNumber_rust(const xmlChar *str);
extern double xmlXPathCastToNumber_rust(xmlXPathObjectPtr val);
extern int xmlXPathCastToBoolean_rust(xmlXPathObjectPtr val);
extern int xmlXPathIsNaN_rust(double val);
extern int xmlXPathIsInf_rust(double val);
extern xmlNodePtr xmlXPathNextNamespace_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern void xmlXPathDebugDumpCompExpr_rust(FILE *output, xmlXPathCompExprPtr comp, int depth);
extern void xmlXPathRegisterFuncLookup_rust(xmlXPathContextPtr ctxt, xmlXPathFuncLookupFunc f, void *funcCtxt);
extern xmlXPathFunction xmlXPathFunctionLookup_rust(xmlXPathContextPtr ctxt, const xmlChar *name);
extern int xmlXPathRegisterFuncNS_rust(xmlXPathContextPtr ctxt, const xmlChar *name, const xmlChar *ns_uri, xmlXPathFunction f);
extern xmlXPathFunction xmlXPathFunctionLookupNS_rust(xmlXPathContextPtr ctxt, const xmlChar *name, const xmlChar *ns_uri);
extern void xmlXPathRegisteredFuncsCleanup_rust(xmlXPathContextPtr ctxt);
extern int xmlXPathRegisterVariableNS_rust(xmlXPathContextPtr ctxt, const xmlChar *name, const xmlChar *ns_uri, xmlXPathObjectPtr value);
extern void xmlXPathRegisterVariableLookup_rust(xmlXPathContextPtr ctxt, xmlXPathVariableLookupFunc f, void *data);
extern xmlXPathObjectPtr xmlXPathVariableLookup_rust(xmlXPathContextPtr ctxt, const xmlChar *name);
extern xmlXPathObjectPtr xmlXPathVariableLookupNS_rust(xmlXPathContextPtr ctxt, const xmlChar *name, const xmlChar *ns_uri);
extern void xmlXPathRegisteredVariablesCleanup_rust(xmlXPathContextPtr ctxt);
extern int xmlXPathRegisterNs_rust(xmlXPathContextPtr ctxt, const xmlChar *prefix, const xmlChar *ns_uri);
extern const xmlChar * xmlXPathNsLookup_rust(xmlXPathContextPtr ctxt, const xmlChar *prefix);
extern xmlChar * xmlXPathCastNumberToString_rust(double val);
extern void xmlXPathFreeContext_rust(xmlXPathContextPtr ctxt);
extern xmlXPathObjectPtr xmlXPathConvertBoolean_rust(xmlXPathObjectPtr val);
extern void xmlXPathEvalExpr_rust(xmlXPathParserContextPtr ctxt);
extern xmlXPathObjectPtr xmlXPathCompiledEval_rust(xmlXPathCompExprPtr comp, xmlXPathContextPtr ctx);
extern int xmlXPathCompiledEvalToBoolean_rust(xmlXPathCompExprPtr comp, xmlXPathContextPtr ctxt);
extern xmlXPathCompExprPtr xmlXPathCompile_rust(const xmlChar *str);
extern xmlXPathCompExprPtr xmlXPathCtxtCompile_rust(xmlXPathContextPtr ctxt, const xmlChar *str);
extern xmlXPathParserContextPtr xmlXPathNewParserContext_rust(const xmlChar *str, xmlXPathContextPtr ctxt);
extern xmlXPathObjectPtr xmlXPathNodeEval_rust(xmlNodePtr node, const xmlChar *str, xmlXPathContextPtr ctx);
extern int xmlXPathSetContextNode_rust(xmlNodePtr node, xmlXPathContextPtr ctx);
extern int xmlXPathEvalPredicate_rust(xmlXPathContextPtr ctxt, xmlXPathObjectPtr res);
extern int xmlXPathEvaluatePredicateResult_rust(xmlXPathParserContextPtr ctxt, xmlXPathObjectPtr res);
extern int xmlXPathIsNodeType_rust(const xmlChar *name);
extern xmlChar *xmlXPathParseNCName_rust(xmlXPathParserContextPtr ctxt);
extern xmlChar *xmlXPathParseName_rust(xmlXPathParserContextPtr ctxt);
extern void xmlXPathRoundFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathFloorFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathCeilingFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathSumFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern xmlNodePtr xmlXPathNextAncestor_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern xmlNodePtr xmlXPathNextAncestorOrSelf_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern xmlNodePtr xmlXPathNextFollowingSibling_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern xmlNodePtr xmlXPathNextPrecedingSibling_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern xmlNodePtr xmlXPathNextFollowing_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern xmlNodePtr xmlXPathNextPreceding_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern xmlNodePtr xmlXPathNextAttribute_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern void xmlXPathRoot_rust(xmlXPathParserContextPtr ctxt);
extern void xmlXPathCountFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathIdFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathLocalNameFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathNamespaceURIFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathStringFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathStringLengthFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathConcatFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathContainsFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathStartsWithFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathSubstringFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathSubstringBeforeFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathSubstringAfterFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathNormalizeFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathTranslateFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathBooleanFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathNotFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathTrueFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathFalseFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathLangFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathNumberFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern xmlXPathObjectPtr xmlXPathEval_rust(const xmlChar *str, xmlXPathContextPtr ctx);
extern xmlXPathObjectPtr xmlXPathEvalExpression_rust(const xmlChar *str, xmlXPathContextPtr ctxt);
extern void xmlXPathRegisterAllFunctions_rust(xmlXPathContextPtr ctxt);
extern xmlNodePtr xmlXPathNextDescendantOrSelf_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern xmlNodePtr xmlXPathNextDescendant_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern xmlChar * xmlXPathCastToString_rust(xmlXPathObjectPtr val);
extern xmlXPathObjectPtr xmlXPathConvertString_rust(xmlXPathObjectPtr val);
extern xmlXPathContextPtr xmlXPathNewContext_rust(xmlDocPtr doc);
extern void xmlXPathFreeParserContext_rust(xmlXPathParserContextPtr ctxt);
extern int xmlXPathEqualValues_rust(xmlXPathParserContextPtr ctxt);
extern int xmlXPathNotEqualValues_rust(xmlXPathParserContextPtr ctxt);
extern xmlNodePtr xmlXPathNextChild_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern xmlNodePtr xmlXPathNextParent_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern void xmlXPathLastFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern void xmlXPathPositionFunction_rust(xmlXPathParserContextPtr ctxt, int nargs);
extern int xmlXPathCompareValues_rust(xmlXPathParserContextPtr ctxt, int inf, int strict);
extern void xmlXPathAddValues_rust(xmlXPathParserContextPtr ctxt);
extern void xmlXPathSubValues_rust(xmlXPathParserContextPtr ctxt);
extern void xmlXPathModValues_rust(xmlXPathParserContextPtr ctxt);
extern void xmlXPathDivValues_rust(xmlXPathParserContextPtr ctxt);
extern void xmlXPathMultValues_rust(xmlXPathParserContextPtr ctxt);
extern void xmlXPathValueFlipSign_rust(xmlXPathParserContextPtr ctxt);
extern xmlNodePtr xmlXPathNextSelf_rust(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);
extern int xmlXPathCastNodeSetToBoolean_rust(xmlNodeSetPtr ns);
extern int xmlXPathCastStringToBoolean_rust(const xmlChar *val);
extern int xmlXPathCastNumberToBoolean_rust(double val);
extern xmlXPathObjectPtr xmlXPathConvertNumber_rust(xmlXPathObjectPtr val);
extern xmlChar * xmlXPathCastNodeToString_rust(xmlNodePtr node);
extern xmlChar * xmlXPathCastBooleanToString_rust(int val);
extern void xmlXPathRegisteredNsCleanup_rust(xmlXPathContextPtr ctxt);
extern void xmlXPathFreeObject_rust(xmlXPathObjectPtr obj);
extern xmlXPathObjectPtr xmlXPathNewFloat_rust(double val);
extern xmlXPathObjectPtr xmlXPathNewBoolean_rust(int val);
extern xmlXPathObjectPtr xmlXPathNewString_rust(const xmlChar *val);
extern xmlXPathObjectPtr xmlXPathWrapString_rust(xmlChar *val);
extern xmlXPathObjectPtr xmlXPathNewCString_rust(const char *val);
extern double xmlXPathCastStringToNumber_rust(const xmlChar *val);
extern double xmlXPathCastBooleanToNumber_rust(int val);
extern xmlChar * xmlXPathCastNodeSetToString_rust(xmlNodeSetPtr ns);
extern xmlXPathObjectPtr xmlXPathWrapCString_rust(char *val);
extern void xmlXPathFreeNodeSetList_rust(xmlXPathObjectPtr obj);
extern xmlXPathObjectPtr xmlXPathWrapNodeSet_rust(xmlNodeSetPtr val);
extern xmlXPathObjectPtr xmlXPathNewValueTree_rust(xmlNodePtr val);
extern xmlXPathObjectPtr xmlXPathNewNodeSet_rust(xmlNodePtr val);
extern xmlXPathObjectPtr xmlXPathObjectCopy_rust(xmlXPathObjectPtr val);
extern xmlXPathObjectPtr xmlXPathWrapExternal_rust(void *val);
extern void xmlGenericErrorContextNodeSet_rust(FILE *output, xmlNodeSetPtr obj);
extern int xmlXPathRegisterVariable_rust(xmlXPathContextPtr ctxt, const xmlChar *name, xmlXPathObjectPtr value);


#if defined(LIBXML_XPATH_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED)

/************************************************************************
 *									*
 *			Floating point stuff				*
 *									*
 ************************************************************************/

double xmlXPathNAN;
double xmlXPathPINF;
double xmlXPathNINF;

/**
 * xmlXPathInit:
 *
 * Initialize the XPath environment
 */
ATTRIBUTE_NO_SANITIZE("float-divide-by-zero")
void xmlXPathInit(void)
{
	/* MSVC doesn't allow division by zero in constant expressions. */
	double zero = 0.0;
	xmlXPathNAN = 0.0 / zero;
	xmlXPathPINF = 1.0 / zero;
	xmlXPathNINF = -xmlXPathPINF;
	xmlXPathInit_rust();
}

/**
 * xmlXPathIsNaN:
 * @val:  a double value
 *
 * Returns 1 if the value is a NaN, 0 otherwise
 */
int xmlXPathIsNaN(double val)
{
	return xmlXPathIsNaN_rust(val);
}

/**
 * xmlXPathIsInf:
 * @val:  a double value
 *
 * Returns 1 if the value is +Infinite, -1 if -Infinite, 0 otherwise
 */
int xmlXPathIsInf(double val)
{
	return xmlXPathIsInf_rust(val);
}

#endif /* SCHEMAS or XPATH */

#ifdef LIBXML_XPATH_ENABLED


/**__xmlRaiseError
 * xmlXPathErr:
 * @ctxt:  a XPath parser context
 * @error:  the error code
 *
 * Handle an XPath error
 */
void xmlXPathErr(xmlXPathParserContextPtr ctxt, int error)
{
	xmlXPathErr_rust(ctxt, error);
}

/**
 * xmlXPatherror:
 * @ctxt:  the XPath Parser context
 * @file:  the file name
 * @line:  the line number
 * @no:  the error number
 *
 * Formats an error message.
 */
void xmlXPatherror(xmlXPathParserContextPtr ctxt, const char *file ATTRIBUTE_UNUSED,
				   int line ATTRIBUTE_UNUSED, int no)
{
	xmlXPatherror_rust(ctxt, file, line, no);
}
/**
 * xmlXPathFreeCompExpr:
 * @comp:  an XPATH comp
 *
 * Free up the memory allocated by @comp
 */
void xmlXPathFreeCompExpr(xmlXPathCompExprPtr comp)
{
	xmlXPathFreeCompExpr_rust(comp);
}

#ifdef LIBXML_DEBUG_ENABLED

void xmlXPathDebugDumpObject(FILE *output, xmlXPathObjectPtr cur, int depth)
{
	xmlXPathDebugDumpObject_rust(output, cur, depth);
}

/**
 * xmlXPathDebugDumpCompExpr:
 * @output:  the FILE * for the output
 * @comp:  the precompiled XPath expression
 * @depth:  the indentation level.
 *
 * Dumps the tree of the compiled XPath expression.
 */
void xmlXPathDebugDumpCompExpr(FILE *output, xmlXPathCompExprPtr comp,
							   int depth)
{
	xmlXPathDebugDumpCompExpr_rust(output, comp, depth);
}
#endif /* LIBXML_DEBUG_ENABLED */



int xmlXPathContextSetCache(xmlXPathContextPtr ctxt,
							int active,
							int value,
							int options)
{
	return xmlXPathContextSetCache_rust(ctxt, active, value, options);
}

/**
 * valuePop:
 * @ctxt: an XPath evaluation context
 *
 * Pops the top XPath object from the value stack
 *
 * Returns the XPath object just removed
 */
xmlXPathObjectPtr
valuePop(xmlXPathParserContextPtr ctxt)
{
	return valuePop_rust(ctxt);
}
/**
 * valuePush:
 * @ctxt:  an XPath evaluation context
 * @value:  the XPath object
 *
 * Pushes a new XPath object on top of the value stack. If value is NULL,
 * a memory error is recorded in the parser context.
 *
 * Returns the number of items on the value stack, or -1 in case of error.
 */
int valuePush(xmlXPathParserContextPtr ctxt, xmlXPathObjectPtr value)
{
	return valuePush_rust(ctxt, value);
}

/**
 * xmlXPathPopBoolean:
 * @ctxt:  an XPath parser context
 *
 * Pops a boolean from the stack, handling conversion if needed.
 * Check error with #xmlXPathCheckError.
 *
 * Returns the boolean
 */
int xmlXPathPopBoolean(xmlXPathParserContextPtr ctxt)
{
	return xmlXPathPopBoolean_rust(ctxt);
}

/**
 * xmlXPathPopNumber:
 * @ctxt:  an XPath parser context
 *
 * Pops a number from the stack, handling conversion if needed.
 * Check error with #xmlXPathCheckError.
 *
 * Returns the number
 */
double
xmlXPathPopNumber(xmlXPathParserContextPtr ctxt)
{
	return xmlXPathPopNumber_rust(ctxt);
}

/**
 * xmlXPathPopString:
 * @ctxt:  an XPath parser context
 *
 * Pops a string from the stack, handling conversion if needed.
 * Check error with #xmlXPathCheckError.
 *
 * Returns the string
 */
xmlChar *
xmlXPathPopString(xmlXPathParserContextPtr ctxt)
{
	return xmlXPathPopString_rust(ctxt);
}

/**
 * xmlXPathPopNodeSet:
 * @ctxt:  an XPath parser context
 *
 * Pops a node-set from the stack, handling conversion if needed.
 * Check error with #xmlXPathCheckError.
 *
 * Returns the node-set
 */
xmlNodeSetPtr
xmlXPathPopNodeSet(xmlXPathParserContextPtr ctxt)
{
	return xmlXPathPopNodeSet_rust(ctxt);
}

/**
 * xmlXPathPopExternal:
 * @ctxt:  an XPath parser context
 *
 * Pops an external object from the stack, handling conversion if needed.
 * Check error with #xmlXPathCheckError.
 *
 * Returns the object
 */
void *
xmlXPathPopExternal(xmlXPathParserContextPtr ctxt)
{
	return xmlXPathPopExternal_rust(ctxt);
}

/*
 * Macros for accessing the content. Those should be used only by the parser,
 * and not exported.
 *
 * Dirty macros, i.e. one need to make assumption on the context to use them
 *
 *   CUR_PTR return the current pointer to the xmlChar to be parsed.
 *   CUR     returns the current xmlChar value, i.e. a 8 bit value
 *           in ISO-Latin or UTF-8.
 *           This should be used internally by the parser
 *           only to compare to ASCII values otherwise it would break when
 *           running with UTF-8 encoding.
 *   NXT(n)  returns the n'th next xmlChar. Same as CUR is should be used only
 *           to compare on ASCII based substring.
 *   SKIP(n) Skip n xmlChar, and must also be used only to skip ASCII defined
 *           strings within the parser.
 *   CURRENT Returns the current char value, with the full decoding of
 *           UTF-8 if we are using this mode. It returns an int.
 *   NEXT    Skip to the next character, this does the proper decoding
 *           in UTF-8 mode. It also pop-up unfinished entities on the fly.
 *           It returns the pointer to the current xmlChar.
 */


long xmlXPathOrderDocElems(xmlDocPtr doc)
{
	return xmlXPathOrderDocElems_rust(doc);
}

/**
 * xmlXPathCmpNodes:
 * @node1:  the first node
 * @node2:  the second node
 *
 * Compare two nodes w.r.t document order
 *
 * Returns -2 in case of error 1 if first point < second point, 0 if
 *         it's the same node, -1 otherwise
 */
int xmlXPathCmpNodes(xmlNodePtr node1, xmlNodePtr node2)
{
	return xmlXPathCmpNodes_rust(node1, node2);
}

/**
 * xmlXPathNodeSetSort:
 * @set:  the node set
 *
 * Sort the node set in document order
 */
void xmlXPathNodeSetSort(xmlNodeSetPtr set)
{
	xmlXPathNodeSetSort_rust(set);
}



/**
 * xmlXPathNodeSetFreeNs:
 * @ns:  the XPath namespace node found in a nodeset.
 *
 * Namespace nodes in libxml don't match the XPath semantic. In a node set
 * the namespace nodes are duplicated and the next pointer is set to the
 * parent node in the XPath semantic. Check if such a node needs to be freed
 */
void xmlXPathNodeSetFreeNs(xmlNsPtr ns)
{
	xmlXPathNodeSetFreeNs_rust(ns);
}

/**
 * xmlXPathNodeSetCreate:
 * @val:  an initial xmlNodePtr, or NULL
 *
 * Create a new xmlNodeSetPtr of type double and of value @val
 *
 * Returns the newly created object.
 */
xmlNodeSetPtr
xmlXPathNodeSetCreate(xmlNodePtr val)
{
	return xmlXPathNodeSetCreate_rust(val);
}

/**
 * xmlXPathNodeSetContains:
 * @cur:  the node-set
 * @val:  the node
 *
 * checks whether @cur contains @val
 *
 * Returns true (1) if @cur contains @val, false (0) otherwise
 */
int xmlXPathNodeSetContains(xmlNodeSetPtr cur, xmlNodePtr val)
{
	return xmlXPathNodeSetContains_rust(cur, val);
}

/**
 * xmlXPathNodeSetAddNs:
 * @cur:  the initial node set
 * @node:  the hosting node
 * @ns:  a the namespace node
 *
 * add a new namespace node to an existing NodeSet
 *
 * Returns 0 in case of success and -1 in case of error
 */
int xmlXPathNodeSetAddNs(xmlNodeSetPtr cur, xmlNodePtr node, xmlNsPtr ns)
{
	return xmlXPathNodeSetAddNs_rust(cur, node, ns);
}

/**
 * xmlXPathNodeSetAdd:
 * @cur:  the initial node set
 * @val:  a new xmlNodePtr
 *
 * add a new xmlNodePtr to an existing NodeSet
 *
 * Returns 0 in case of success, and -1 in case of error
 */
int xmlXPathNodeSetAdd(xmlNodeSetPtr cur, xmlNodePtr val)
{
	return xmlXPathNodeSetAdd_rust(cur, val);
}

/**
 * xmlXPathNodeSetAddUnique:
 * @cur:  the initial node set
 * @val:  a new xmlNodePtr
 *
 * add a new xmlNodePtr to an existing NodeSet, optimized version
 * when we are sure the node is not already in the set.
 *
 * Returns 0 in case of success and -1 in case of failure
 */
int xmlXPathNodeSetAddUnique(xmlNodeSetPtr cur, xmlNodePtr val)
{
	return xmlXPathNodeSetAddUnique_rust(cur, val);
}

/**
 * xmlXPathNodeSetMerge:
 * @val1:  the first NodeSet or NULL
 * @val2:  the second NodeSet
 *
 * Merges two nodesets, all nodes from @val2 are added to @val1
 * if @val1 is NULL, a new set is created and copied from @val2
 *
 * Returns @val1 once extended or NULL in case of error.
 */
xmlNodeSetPtr
xmlXPathNodeSetMerge(xmlNodeSetPtr val1, xmlNodeSetPtr val2)
{
	return xmlXPathNodeSetMerge_rust(val1, val2);
}

/**
 * xmlXPathNodeSetDel:
 * @cur:  the initial node set
 * @val:  an xmlNodePtr
 *
 * Removes an xmlNodePtr from an existing NodeSet
 */
void xmlXPathNodeSetDel(xmlNodeSetPtr cur, xmlNodePtr val)
{
	xmlXPathNodeSetDel_rust(cur, val);
}

/**
 * xmlXPathNodeSetRemove:
 * @cur:  the initial node set
 * @val:  the index to remove
 *
 * Removes an entry from an existing NodeSet list.
 */
void xmlXPathNodeSetRemove(xmlNodeSetPtr cur, int val)
{
	xmlXPathNodeSetRemove_rust(cur, val);
}

/**
 * xmlXPathFreeNodeSet:
 * @obj:  the xmlNodeSetPtr to free
 *
 * Free the NodeSet compound (not the actual nodes !).
 */
void xmlXPathFreeNodeSet(xmlNodeSetPtr obj)
{
	xmlXPathFreeNodeSet_rust(obj);
}


#if defined(DEBUG) || defined(DEBUG_STEP)
/**
 * xmlGenericErrorContextNodeSet:
 * @output:  a FILE * for the output
 * @obj:  the xmlNodeSetPtr to display
 *
 * Quick display of a NodeSet
 */
void xmlGenericErrorContextNodeSet(FILE *output, xmlNodeSetPtr obj)
{
	xmlGenericErrorContextNodeSet_rust(output, obj);
}
#endif

/**
 * xmlXPathNewNodeSet:
 * @val:  the NodePtr value
 *
 * Create a new xmlXPathObjectPtr of type NodeSet and initialize
 * it with the single Node @val
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathNewNodeSet(xmlNodePtr val)
{
	return xmlXPathNewNodeSet_rust(val);
}

/**
 * xmlXPathNewValueTree:
 * @val:  the NodePtr value
 *
 * Create a new xmlXPathObjectPtr of type Value Tree (XSLT) and initialize
 * it with the tree root @val
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathNewValueTree(xmlNodePtr val)
{
	return xmlXPathNewValueTree_rust(val);
}

/**
 * xmlXPathNewNodeSetList:
 * @val:  an existing NodeSet
 *
 * Create a new xmlXPathObjectPtr of type NodeSet and initialize
 * it with the Nodeset @val
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathNewNodeSetList(xmlNodeSetPtr val)
{
	return xmlXPathNewNodeSetList_rust(val);
}

/**
 * xmlXPathWrapNodeSet:
 * @val:  the NodePtr value
 *
 * Wrap the Nodeset @val in a new xmlXPathObjectPtr
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathWrapNodeSet(xmlNodeSetPtr val)
{
	return xmlXPathWrapNodeSet_rust(val);
}

/**
 * xmlXPathFreeNodeSetList:
 * @obj:  an existing NodeSetList object
 *
 * Free up the xmlXPathObjectPtr @obj but don't deallocate the objects in
 * the list contrary to xmlXPathFreeObject().
 */
void xmlXPathFreeNodeSetList(xmlXPathObjectPtr obj)
{
	xmlXPathFreeNodeSetList_rust(obj);
}

/**
 * xmlXPathDifference:
 * @nodes1:  a node-set
 * @nodes2:  a node-set
 *
 * Implements the EXSLT - Sets difference() function:
 *    node-set set:difference (node-set, node-set)
 *
 * Returns the difference between the two node sets, or nodes1 if
 *         nodes2 is empty
 */
xmlNodeSetPtr
xmlXPathDifference(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return xmlXPathDifference_rust(nodes1, nodes2);
}

/**
 * xmlXPathIntersection:
 * @nodes1:  a node-set
 * @nodes2:  a node-set
 *
 * Implements the EXSLT - Sets intersection() function:
 *    node-set set:intersection (node-set, node-set)
 *
 * Returns a node set comprising the nodes that are within both the
 *         node sets passed as arguments
 */
xmlNodeSetPtr
xmlXPathIntersection(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return xmlXPathIntersection_rust(nodes1, nodes2);
}

/**
 * xmlXPathDistinctSorted:
 * @nodes:  a node-set, sorted by document order
 *
 * Implements the EXSLT - Sets distinct() function:
 *    node-set set:distinct (node-set)
 *
 * Returns a subset of the nodes contained in @nodes, or @nodes if
 *         it is empty
 */
xmlNodeSetPtr
xmlXPathDistinctSorted(xmlNodeSetPtr nodes)
{
	return xmlXPathDistinctSorted_rust(nodes);
}

/**
 * xmlXPathDistinct:
 * @nodes:  a node-set
 *
 * Implements the EXSLT - Sets distinct() function:
 *    node-set set:distinct (node-set)
 * @nodes is sorted by document order, then #exslSetsDistinctSorted
 * is called with the sorted node-set
 *
 * Returns a subset of the nodes contained in @nodes, or @nodes if
 *         it is empty
 */
xmlNodeSetPtr
xmlXPathDistinct(xmlNodeSetPtr nodes)
{
	return xmlXPathDistinct_rust(nodes);
}

/**
 * xmlXPathHasSameNodes:
 * @nodes1:  a node-set
 * @nodes2:  a node-set
 *
 * Implements the EXSLT - Sets has-same-nodes function:
 *    boolean set:has-same-node(node-set, node-set)
 *
 * Returns true (1) if @nodes1 shares any node with @nodes2, false (0)
 *         otherwise
 */
int xmlXPathHasSameNodes(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return xmlXPathHasSameNodes_rust(nodes1, nodes2);
}

/**
 * xmlXPathNodeLeadingSorted:
 * @nodes: a node-set, sorted by document order
 * @node: a node
 *
 * Implements the EXSLT - Sets leading() function:
 *    node-set set:leading (node-set, node-set)
 *
 * Returns the nodes in @nodes that precede @node in document order,
 *         @nodes if @node is NULL or an empty node-set if @nodes
 *         doesn't contain @node
 */
xmlNodeSetPtr
xmlXPathNodeLeadingSorted(xmlNodeSetPtr nodes, xmlNodePtr node)
{
	return xmlXPathNodeLeadingSorted_rust(nodes, node);
}

/**
 * xmlXPathNodeLeading:
 * @nodes:  a node-set
 * @node:  a node
 *
 * Implements the EXSLT - Sets leading() function:
 *    node-set set:leading (node-set, node-set)
 * @nodes is sorted by document order, then #exslSetsNodeLeadingSorted
 * is called.
 *
 * Returns the nodes in @nodes that precede @node in document order,
 *         @nodes if @node is NULL or an empty node-set if @nodes
 *         doesn't contain @node
 */
xmlNodeSetPtr
xmlXPathNodeLeading(xmlNodeSetPtr nodes, xmlNodePtr node)
{
	return xmlXPathNodeLeading_rust(nodes, node);
}

/**
 * xmlXPathLeadingSorted:
 * @nodes1:  a node-set, sorted by document order
 * @nodes2:  a node-set, sorted by document order
 *
 * Implements the EXSLT - Sets leading() function:
 *    node-set set:leading (node-set, node-set)
 *
 * Returns the nodes in @nodes1 that precede the first node in @nodes2
 *         in document order, @nodes1 if @nodes2 is NULL or empty or
 *         an empty node-set if @nodes1 doesn't contain @nodes2
 */
xmlNodeSetPtr
xmlXPathLeadingSorted(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return xmlXPathLeadingSorted_rust(nodes1, nodes2);
}
/**
 * xmlXPathLeading:
 * @nodes1:  a node-set
 * @nodes2:  a node-set
 *
 * Implements the EXSLT - Sets leading() function:
 *    node-set set:leading (node-set, node-set)
 * @nodes1 and @nodes2 are sorted by document order, then
 * #exslSetsLeadingSorted is called.
 *
 * Returns the nodes in @nodes1 that precede the first node in @nodes2
 *         in document order, @nodes1 if @nodes2 is NULL or empty or
 *         an empty node-set if @nodes1 doesn't contain @nodes2
 */
xmlNodeSetPtr
xmlXPathLeading(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return xmlXPathLeading_rust(nodes1, nodes2);
}

/**
 * xmlXPathNodeTrailingSorted:
 * @nodes: a node-set, sorted by document order
 * @node: a node
 *
 * Implements the EXSLT - Sets trailing() function:
 *    node-set set:trailing (node-set, node-set)
 *
 * Returns the nodes in @nodes that follow @node in document order,
 *         @nodes if @node is NULL or an empty node-set if @nodes
 *         doesn't contain @node
 */
xmlNodeSetPtr
xmlXPathNodeTrailingSorted(xmlNodeSetPtr nodes, xmlNodePtr node)
{
	return xmlXPathNodeTrailingSorted_rust(nodes, node);
}

/**
 * xmlXPathNodeTrailing:
 * @nodes:  a node-set
 * @node:  a node
 *
 * Implements the EXSLT - Sets trailing() function:
 *    node-set set:trailing (node-set, node-set)
 * @nodes is sorted by document order, then #xmlXPathNodeTrailingSorted
 * is called.
 *
 * Returns the nodes in @nodes that follow @node in document order,
 *         @nodes if @node is NULL or an empty node-set if @nodes
 *         doesn't contain @node
 */
xmlNodeSetPtr
xmlXPathNodeTrailing(xmlNodeSetPtr nodes, xmlNodePtr node)
{
	return xmlXPathNodeTrailing_rust(nodes, node);
}

/**
 * xmlXPathTrailingSorted:
 * @nodes1:  a node-set, sorted by document order
 * @nodes2:  a node-set, sorted by document order
 *
 * Implements the EXSLT - Sets trailing() function:
 *    node-set set:trailing (node-set, node-set)
 *
 * Returns the nodes in @nodes1 that follow the first node in @nodes2
 *         in document order, @nodes1 if @nodes2 is NULL or empty or
 *         an empty node-set if @nodes1 doesn't contain @nodes2
 */
xmlNodeSetPtr
xmlXPathTrailingSorted(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return xmlXPathTrailingSorted_rust(nodes1, nodes2);
}

/**
 * xmlXPathTrailing:
 * @nodes1:  a node-set
 * @nodes2:  a node-set
 *
 * Implements the EXSLT - Sets trailing() function:
 *    node-set set:trailing (node-set, node-set)
 * @nodes1 and @nodes2 are sorted by document order, then
 * #xmlXPathTrailingSorted is called.
 *
 * Returns the nodes in @nodes1 that follow the first node in @nodes2
 *         in document order, @nodes1 if @nodes2 is NULL or empty or
 *         an empty node-set if @nodes1 doesn't contain @nodes2
 */
xmlNodeSetPtr
xmlXPathTrailing(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return xmlXPathTrailing_rust(nodes1, nodes2);
}

/**
 * xmlXPathRegisterFunc:
 * @ctxt:  the XPath context
 * @name:  the function name
 * @f:  the function implementation or NULL
 *
 * Register a new function. If @f is NULL it unregisters the function
 *
 * Returns 0 in case of success, -1 in case of error
 */
int xmlXPathRegisterFunc(xmlXPathContextPtr ctxt, const xmlChar *name,
						 xmlXPathFunction f)
{
	return xmlXPathRegisterFunc_rust(ctxt, name, f);
}

/**
 * xmlXPathRegisterFuncNS:
 * @ctxt:  the XPath context
 * @name:  the function name
 * @ns_uri:  the function namespace URI
 * @f:  the function implementation or NULL
 *
 * Register a new function. If @f is NULL it unregisters the function
 *
 * Returns 0 in case of success, -1 in case of error
 */
int xmlXPathRegisterFuncNS(xmlXPathContextPtr ctxt, const xmlChar *name,
						   const xmlChar *ns_uri, xmlXPathFunction f)
{
	return xmlXPathRegisterFuncNS_rust(ctxt, name, ns_uri, f);
}

/**
 * xmlXPathRegisterFuncLookup:
 * @ctxt:  the XPath context
 * @f:  the lookup function
 * @funcCtxt:  the lookup data
 *
 * Registers an external mechanism to do function lookup.
 */
void xmlXPathRegisterFuncLookup(xmlXPathContextPtr ctxt,
								xmlXPathFuncLookupFunc f,
								void *funcCtxt)
{
	xmlXPathRegisterFuncLookup_rust(ctxt, f, funcCtxt);
}

/**
 * xmlXPathFunctionLookup:
 * @ctxt:  the XPath context
 * @name:  the function name
 *
 * Search in the Function array of the context for the given
 * function.
 *
 * Returns the xmlXPathFunction or NULL if not found
 */
xmlXPathFunction
xmlXPathFunctionLookup(xmlXPathContextPtr ctxt, const xmlChar *name)
{
	return xmlXPathFunctionLookup_rust(ctxt, name);
}

/**
 * xmlXPathFunctionLookupNS:
 * @ctxt:  the XPath context
 * @name:  the function name
 * @ns_uri:  the function namespace URI
 *
 * Search in the Function array of the context for the given
 * function.
 *
 * Returns the xmlXPathFunction or NULL if not found
 */
xmlXPathFunction
xmlXPathFunctionLookupNS(xmlXPathContextPtr ctxt, const xmlChar *name,
						 const xmlChar *ns_uri)
{
	return xmlXPathFunctionLookupNS_rust(ctxt, name, ns_uri);
}

/**
 * xmlXPathRegisteredFuncsCleanup:
 * @ctxt:  the XPath context
 *
 * Cleanup the XPath context data associated to registered functions
 */
void xmlXPathRegisteredFuncsCleanup(xmlXPathContextPtr ctxt)
{
	xmlXPathRegisteredFuncsCleanup_rust(ctxt);
}

int xmlXPathRegisterVariable(xmlXPathContextPtr ctxt, const xmlChar *name,
							 xmlXPathObjectPtr value)
{
	return xmlXPathRegisterVariable_rust(ctxt, name, value);
}

/**
 * xmlXPathRegisterVariableNS:
 * @ctxt:  the XPath context
 * @name:  the variable name
 * @ns_uri:  the variable namespace URI
 * @value:  the variable value or NULL
 *
 * Register a new variable value. If @value is NULL it unregisters
 * the variable
 *
 * Returns 0 in case of success, -1 in case of error
 */
int xmlXPathRegisterVariableNS(xmlXPathContextPtr ctxt, const xmlChar *name,
							   const xmlChar *ns_uri,
							   xmlXPathObjectPtr value)
{
	return xmlXPathRegisterVariableNS_rust(ctxt, name, ns_uri, value);
}

void xmlXPathRegisterVariableLookup(xmlXPathContextPtr ctxt,
									xmlXPathVariableLookupFunc f, void *data)
{
	xmlXPathRegisterVariableLookup_rust(ctxt, f, data);
}

xmlXPathObjectPtr
xmlXPathVariableLookup(xmlXPathContextPtr ctxt, const xmlChar *name)
{
	return xmlXPathVariableLookup_rust(ctxt, name);
}

/**
 * xmlXPathVariableLookupNS:
 * @ctxt:  the XPath context
 * @name:  the variable name
 * @ns_uri:  the variable namespace URI
 *
 * Search in the Variable array of the context for the given
 * variable value.
 *
 * Returns the a copy of the value or NULL if not found
 */
xmlXPathObjectPtr
xmlXPathVariableLookupNS(xmlXPathContextPtr ctxt, const xmlChar *name,
						 const xmlChar *ns_uri)
{
	return xmlXPathVariableLookupNS_rust(ctxt, name, ns_uri);
}

/**
 * xmlXPathRegisteredVariablesCleanup:
 * @ctxt:  the XPath context
 *
 * Cleanup the XPath context data associated to registered variables
 */
void xmlXPathRegisteredVariablesCleanup(xmlXPathContextPtr ctxt)
{
	xmlXPathRegisteredVariablesCleanup_rust(ctxt);
}

/**
 * xmlXPathRegisterNs:
 * @ctxt:  the XPath context
 * @prefix:  the namespace prefix cannot be NULL or empty string
 * @ns_uri:  the namespace name
 *
 * Register a new namespace. If @ns_uri is NULL it unregisters
 * the namespace
 *
 * Returns 0 in case of success, -1 in case of error
 */
int xmlXPathRegisterNs(xmlXPathContextPtr ctxt, const xmlChar *prefix,
					   const xmlChar *ns_uri)
{
	return xmlXPathRegisterNs_rust(ctxt, prefix, ns_uri);
}

/**
 * xmlXPathNsLookup:
 * @ctxt:  the XPath context
 * @prefix:  the namespace prefix value
 *
 * Search in the namespace declaration array of the context for the given
 * namespace name associated to the given prefix
 *
 * Returns the value or NULL if not found
 */
const xmlChar *
xmlXPathNsLookup(xmlXPathContextPtr ctxt, const xmlChar *prefix)
{
	return xmlXPathNsLookup_rust(ctxt, prefix);
}

void xmlXPathRegisteredNsCleanup(xmlXPathContextPtr ctxt)
{
	xmlXPathRegisteredNsCleanup_rust(ctxt);
}

xmlXPathObjectPtr
xmlXPathNewFloat(double val)
{
	return xmlXPathNewFloat_rust(val);
}

/**
 * xmlXPathNewBoolean:
 * @val:  the boolean value
 *
 * Create a new xmlXPathObjectPtr of type boolean and of value @val
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathNewBoolean(int val)
{
	return xmlXPathNewBoolean_rust(val);
}

/**
 * xmlXPathNewString:
 * @val:  the xmlChar * value
 *
 * Create a new xmlXPathObjectPtr of type string and of value @val
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathNewString(const xmlChar *val)
{
	return xmlXPathNewString_rust(val);
}

/**
 * xmlXPathWrapString:
 * @val:  the xmlChar * value
 *
 * Wraps the @val string into an XPath object.
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathWrapString(xmlChar *val)
{
	return xmlXPathWrapString_rust(val);
}

/**
 * xmlXPathNewCString:
 * @val:  the char * value
 *
 * Create a new xmlXPathObjectPtr of type string and of value @val
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathNewCString(const char *val)
{
	return xmlXPathNewCString_rust(val);
}

/**
 * xmlXPathWrapCString:
 * @val:  the char * value
 *
 * Wraps a string into an XPath object.
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathWrapCString(char *val)
{
	return xmlXPathWrapCString_rust(val);
}

/**
 * xmlXPathWrapExternal:
 * @val:  the user data
 *
 * Wraps the @val data into an XPath object.
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathWrapExternal(void *val)
{
	return xmlXPathWrapExternal_rust(val);
}

/**
 * xmlXPathObjectCopy:
 * @val:  the original object
 *
 * allocate a new copy of a given object
 *
 * Returns the newly created object.
 */
xmlXPathObjectPtr
xmlXPathObjectCopy(xmlXPathObjectPtr val)
{
	return xmlXPathObjectCopy_rust(val);
}

/**
 * xmlXPathFreeObject:
 * @obj:  the object to free
 *
 * Free up an xmlXPathObjectPtr object.
 */
void xmlXPathFreeObject(xmlXPathObjectPtr obj)
{
	xmlXPathFreeObject_rust(obj);
}


/**
 * xmlXPathCastBooleanToString:
 * @val:  a boolean
 *
 * Converts a boolean to its string value.
 *
 * Returns a newly allocated string.
 */
xmlChar *
xmlXPathCastBooleanToString(int val)
{
	return xmlXPathCastBooleanToString_rust(val);
}

/**
 * xmlXPathCastNumberToString:
 * @val:  a number
 *
 * Converts a number to its string value.
 *
 * Returns a newly allocated string.
 */
xmlChar *
xmlXPathCastNumberToString(double val)
{
	return xmlXPathCastNumberToString_rust(val);
}

/**
 * xmlXPathCastNodeToString:
 * @node:  a node
 *
 * Converts a node to its string value.
 *
 * Returns a newly allocated string.
 */
xmlChar *
xmlXPathCastNodeToString(xmlNodePtr node)
{
	return xmlXPathCastNodeToString_rust(node);
}

/**
 * xmlXPathCastNodeSetToString:
 * @ns:  a node-set
 *
 * Converts a node-set to its string value.
 *
 * Returns a newly allocated string.
 */
xmlChar *
xmlXPathCastNodeSetToString(xmlNodeSetPtr ns)
{
	return xmlXPathCastNodeSetToString_rust(ns);
}

/**
 * xmlXPathCastToString:
 * @val:  an XPath object
 *
 * Converts an existing object to its string() equivalent
 *
 * Returns the allocated string value of the object, NULL in case of error.
 *         It's up to the caller to free the string memory with xmlFree().
 */
xmlChar *
xmlXPathCastToString(xmlXPathObjectPtr val)
{
	return xmlXPathCastToString_rust(val);
}

/**
 * xmlXPathConvertString:
 * @val:  an XPath object
 *
 * Converts an existing object to its string() equivalent
 *
 * Returns the new object, the old one is freed (or the operation
 *         is done directly on @val)
 */
xmlXPathObjectPtr
xmlXPathConvertString(xmlXPathObjectPtr val)
{
	return xmlXPathConvertString_rust(val);
}

/**
 * xmlXPathCastBooleanToNumber:
 * @val:  a boolean
 *
 * Converts a boolean to its number value
 *
 * Returns the number value
 */
double
xmlXPathCastBooleanToNumber(int val)
{
	return xmlXPathCastBooleanToNumber_rust(val);
}

/**
 * xmlXPathCastStringToNumber:
 * @val:  a string
 *
 * Converts a string to its number value
 *
 * Returns the number value
 */
double
xmlXPathCastStringToNumber(const xmlChar *val)
{
	return xmlXPathCastStringToNumber_rust(val);
}

/**
 * xmlXPathCastNodeToNumber:
 * @node:  a node
 *
 * Converts a node to its number value
 *
 * Returns the number value
 */
double
xmlXPathCastNodeToNumber(xmlNodePtr node)
{
	return xmlXPathCastNodeToNumber_rust(node);
}

/**
 * xmlXPathCastNodeSetToNumber:
 * @ns:  a node-set
 *
 * Converts a node-set to its number value
 *
 * Returns the number value
 */
double
xmlXPathCastNodeSetToNumber(xmlNodeSetPtr ns)
{
	return xmlXPathCastNodeSetToNumber_rust(ns);
}

/**
 * xmlXPathCastToNumber:
 * @val:  an XPath object
 *
 * Converts an XPath object to its number value
 *
 * Returns the number value
 */
double
xmlXPathCastToNumber(xmlXPathObjectPtr val)
{
	return xmlXPathCastToNumber_rust(val);
}

/**
 * xmlXPathConvertNumber:
 * @val:  an XPath object
 *
 * Converts an existing object to its number() equivalent
 *
 * Returns the new object, the old one is freed (or the operation
 *         is done directly on @val)
 */
xmlXPathObjectPtr
xmlXPathConvertNumber(xmlXPathObjectPtr val)
{
	return xmlXPathConvertNumber_rust(val);
}

/**
 * xmlXPathCastNumberToBoolean:
 * @val:  a number
 *
 * Converts a number to its boolean value
 *
 * Returns the boolean value
 */
int xmlXPathCastNumberToBoolean(double val)
{
	return xmlXPathCastNumberToBoolean_rust(val);
}

/**
 * xmlXPathCastStringToBoolean:
 * @val:  a string
 *
 * Converts a string to its boolean value
 *
 * Returns the boolean value
 */
int xmlXPathCastStringToBoolean(const xmlChar *val)
{
	return xmlXPathCastStringToBoolean_rust(val);
}

/**
 * xmlXPathCastNodeSetToBoolean:
 * @ns:  a node-set
 *
 * Converts a node-set to its boolean value
 *
 * Returns the boolean value
 */
int xmlXPathCastNodeSetToBoolean(xmlNodeSetPtr ns)
{
	return xmlXPathCastNodeSetToBoolean_rust(ns);
}

/**
 * xmlXPathCastToBoolean:
 * @val:  an XPath object
 *
 * Converts an XPath object to its boolean value
 *
 * Returns the boolean value
 */
int xmlXPathCastToBoolean(xmlXPathObjectPtr val)
{
	return xmlXPathCastToBoolean_rust(val);
}

/**
 * xmlXPathConvertBoolean:
 * @val:  an XPath object
 *
 * Converts an existing object to its boolean() equivalent
 *
 * Returns the new object, the old one is freed (or the operation
 *         is done directly on @val)
 */
xmlXPathObjectPtr
xmlXPathConvertBoolean(xmlXPathObjectPtr val)
{
	return xmlXPathConvertBoolean_rust(val);
}
/**
 * xmlXPathNewContext:
 * @doc:  the XML document
 *
 * Create a new xmlXPathContext
 *
 * Returns the xmlXPathContext just allocated. The caller will need to free it.
 */
xmlXPathContextPtr
xmlXPathNewContext(xmlDocPtr doc)
{
	return xmlXPathNewContext_rust(doc);
}

void xmlXPathFreeContext(xmlXPathContextPtr ctxt)
{
	xmlXPathFreeContext_rust(ctxt);
}

/**
 * xmlXPathNewParserContext:
 * @str:  the XPath expression
 * @ctxt:  the XPath context
 *
 * Create a new xmlXPathParserContext
 *
 * Returns the xmlXPathParserContext just allocated.
 */
xmlXPathParserContextPtr
xmlXPathNewParserContext(const xmlChar *str, xmlXPathContextPtr ctxt)
{
	return xmlXPathNewParserContext_rust(str, ctxt);
}

/**
 * xmlXPathFreeParserContext:
 * @ctxt:  the context to free
 *
 * Free up an xmlXPathParserContext
 */
void xmlXPathFreeParserContext(xmlXPathParserContextPtr ctxt)
{
	xmlXPathFreeParserContext_rust(ctxt);
}

/**
 * xmlXPathEqualValues:
 * @ctxt:  the XPath Parser context
 *
 * Implement the equal operation on XPath objects content: @arg1 == @arg2
 *
 * Returns 0 or 1 depending on the results of the test.
 */
int xmlXPathEqualValues(xmlXPathParserContextPtr ctxt)
{
	return xmlXPathEqualValues_rust(ctxt);
}

/**
 * xmlXPathNotEqualValues:
 * @ctxt:  the XPath Parser context
 *
 * Implement the equal operation on XPath objects content: @arg1 == @arg2
 *
 * Returns 0 or 1 depending on the results of the test.
 */
int xmlXPathNotEqualValues(xmlXPathParserContextPtr ctxt)
{
	return xmlXPathNotEqualValues_rust(ctxt);
}

/**
 * xmlXPathCompareValues:
 * @ctxt:  the XPath Parser context
 * @inf:  less than (1) or greater than (0)
 * @strict:  is the comparison strict
 *
 * Implement the compare operation on XPath objects:
 *     @arg1 < @arg2    (1, 1, ...
 *     @arg1 <= @arg2   (1, 0, ...
 *     @arg1 > @arg2    (0, 1, ...
 *     @arg1 >= @arg2   (0, 0, ...
 *
 * When neither object to be compared is a node-set and the operator is
 * <=, <, >=, >, then the objects are compared by converted both objects
 * to numbers and comparing the numbers according to IEEE 754. The <
 * comparison will be true if and only if the first number is less than the
 * second number. The <= comparison will be true if and only if the first
 * number is less than or equal to the second number. The > comparison
 * will be true if and only if the first number is greater than the second
 * number. The >= comparison will be true if and only if the first number
 * is greater than or equal to the second number.
 *
 * Returns 1 if the comparison succeeded, 0 if it failed
 */
int xmlXPathCompareValues(xmlXPathParserContextPtr ctxt, int inf, int strict)
{
	return xmlXPathCompareValues_rust(ctxt, inf, strict);
}

/**
 * xmlXPathValueFlipSign:
 * @ctxt:  the XPath Parser context
 *
 * Implement the unary - operation on an XPath object
 * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */
void xmlXPathValueFlipSign(xmlXPathParserContextPtr ctxt)
{
	xmlXPathValueFlipSign_rust(ctxt);
}

/**
 * xmlXPathAddValues:
 * @ctxt:  the XPath Parser context
 *
 * Implement the add operation on XPath objects:
 * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */
void xmlXPathAddValues(xmlXPathParserContextPtr ctxt)
{
	xmlXPathAddValues_rust(ctxt);
}

/**
 * xmlXPathSubValues:
 * @ctxt:  the XPath Parser context
 *
 * Implement the subtraction operation on XPath objects:
 * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */
void xmlXPathSubValues(xmlXPathParserContextPtr ctxt)
{
	xmlXPathSubValues_rust(ctxt);
}

/**
 * xmlXPathMultValues:
 * @ctxt:  the XPath Parser context
 *
 * Implement the multiply operation on XPath objects:
 * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */
void xmlXPathMultValues(xmlXPathParserContextPtr ctxt)
{
	xmlXPathMultValues_rust(ctxt);
}

/**
 * xmlXPathDivValues:
 * @ctxt:  the XPath Parser context
 *
 * Implement the div operation on XPath objects @arg1 / @arg2:
 * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */
ATTRIBUTE_NO_SANITIZE("float-divide-by-zero")
void xmlXPathDivValues(xmlXPathParserContextPtr ctxt)
{
	xmlXPathDivValues_rust(ctxt);
}

/**
 * xmlXPathModValues:
 * @ctxt:  the XPath Parser context
 *
 * Implement the mod operation on XPath objects: @arg1 / @arg2
 * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */
void xmlXPathModValues(xmlXPathParserContextPtr ctxt)
{
	xmlXPathModValues_rust(ctxt);
}

/************************************************************************
 *									*
 *		The traversal functions					*
 *									*
 ************************************************************************/

/*
 * A traversal function enumerates nodes along an axis.
 * Initially it must be called with NULL, and it indicates
 * termination on the axis by returning NULL.
 */
typedef xmlNodePtr (*xmlXPathTraversalFunction)(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);

/*
 * xmlXPathTraversalFunctionExt:
 * A traversal function enumerates nodes along an axis.
 * Initially it must be called with NULL, and it indicates
 * termination on the axis by returning NULL.
 * The context node of the traversal is specified via @contextNode.
 */
typedef xmlNodePtr (*xmlXPathTraversalFunctionExt)(xmlNodePtr cur, xmlNodePtr contextNode);

/*
 * xmlXPathNodeSetMergeFunction:
 * Used for merging node sets in xmlXPathCollectAndTest().
 */
typedef xmlNodeSetPtr (*xmlXPathNodeSetMergeFunction)(xmlNodeSetPtr, xmlNodeSetPtr);

/**
 * xmlXPathNextSelf:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "self" direction
 * The self axis contains just the context node itself
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextSelf(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextSelf_rust(ctxt, cur);
}

/**
 * xmlXPathNextChild:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "child" direction
 * The child axis contains the children of the context node in document order.
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextChild(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextChild_rust(ctxt, cur);
}

/**
 * xmlXPathNextDescendant:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "descendant" direction
 * the descendant axis contains the descendants of the context node in document
 * order; a descendant is a child or a child of a child and so on.
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextDescendant(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextDescendant_rust(ctxt, cur);
}

/**
 * xmlXPathNextDescendantOrSelf:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "descendant-or-self" direction
 * the descendant-or-self axis contains the context node and the descendants
 * of the context node in document order; thus the context node is the first
 * node on the axis, and the first child of the context node is the second node
 * on the axis
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextDescendantOrSelf(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextDescendantOrSelf_rust(ctxt, cur);
}

/**
 * xmlXPathNextParent:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "parent" direction
 * The parent axis contains the parent of the context node, if there is one.
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextParent(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextParent_rust(ctxt, cur);
}

/**
 * xmlXPathNextAncestor:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "ancestor" direction
 * the ancestor axis contains the ancestors of the context node; the ancestors
 * of the context node consist of the parent of context node and the parent's
 * parent and so on; the nodes are ordered in reverse document order; thus the
 * parent is the first node on the axis, and the parent's parent is the second
 * node on the axis
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextAncestor(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextAncestor_rust(ctxt, cur);
}

/**
 * xmlXPathNextAncestorOrSelf:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "ancestor-or-self" direction
 * he ancestor-or-self axis contains the context node and ancestors of
 * the context node in reverse document order; thus the context node is
 * the first node on the axis, and the context node's parent the second;
 * parent here is defined the same as with the parent axis.
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextAncestorOrSelf(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextAncestorOrSelf_rust(ctxt, cur);
}

/**
 * xmlXPathNextFollowingSibling:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "following-sibling" direction
 * The following-sibling axis contains the following siblings of the context
 * node in document order.
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextFollowingSibling(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextFollowingSibling_rust(ctxt, cur);
}

/**
 * xmlXPathNextPrecedingSibling:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "preceding-sibling" direction
 * The preceding-sibling axis contains the preceding siblings of the context
 * node in reverse document order; the first preceding sibling is first on the
 * axis; the sibling preceding that node is the second on the axis and so on.
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextPrecedingSibling(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextPrecedingSibling_rust(ctxt, cur);
}

/**
 * xmlXPathNextFollowing:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "following" direction
 * The following axis contains all nodes in the same document as the context
 * node that are after the context node in document order, excluding any
 * descendants and excluding attribute nodes and namespace nodes; the nodes
 * are ordered in document order
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextFollowing(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextFollowing_rust(ctxt, cur);
}

/**
 * xmlXPathNextPreceding:
 * @ctxt:  the XPath Parser context
 * @cur:  the current node in the traversal
 *
 * Traversal function for the "preceding" direction
 * the preceding axis contains all nodes in the same document as the context
 * node that are before the context node in document order, excluding any
 * ancestors and excluding attribute nodes and namespace nodes; the nodes are
 * ordered in reverse document order
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextPreceding(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextPreceding_rust(ctxt, cur);
}

/**
 * xmlXPathNextNamespace:
 * @ctxt:  the XPath Parser context
 * @cur:  the current attribute in the traversal
 *
 * Traversal function for the "namespace" direction
 * the namespace axis contains the namespace nodes of the context node;
 * the order of nodes on this axis is implementation-defined; the axis will
 * be empty unless the context node is an element
 *
 * We keep the XML namespace node at the end of the list.
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextNamespace(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextNamespace_rust(ctxt, cur);
}

/**
 * xmlXPathNextAttribute:
 * @ctxt:  the XPath Parser context
 * @cur:  the current attribute in the traversal
 *
 * Traversal function for the "attribute" direction
 * TODO: support DTD inherited default attributes
 *
 * Returns the next element following that axis
 */
xmlNodePtr
xmlXPathNextAttribute(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return xmlXPathNextAttribute_rust(ctxt, cur);
}
/**
 * xmlXPathRoot:
 * @ctxt:  the XPath Parser context
 *
 * Initialize the context to the root of the document
 */
void xmlXPathRoot(xmlXPathParserContextPtr ctxt)
{
	xmlXPathRoot_rust(ctxt);
}
/**
 * xmlXPathLastFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the last() XPath function
 *    number last()
 * The last function returns the number of nodes in the context node list.
 */
void xmlXPathLastFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathLastFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathPositionFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the position() XPath function
 *    number position()
 * The position function returns the position of the context node in the
 * context node list. The first position is 1, and so the last position
 * will be equal to last().
 */
void xmlXPathPositionFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathPositionFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathCountFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the count() XPath function
 *    number count(node-set)
 */
void xmlXPathCountFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathCountFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathIdFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the id() XPath function
 *    node-set id(object)
 * The id function selects elements by their unique ID
 * (see [5.2.1 Unique IDs]). When the argument to id is of type node-set,
 * then the result is the union of the result of applying id to the
 * string value of each of the nodes in the argument node-set. When the
 * argument to id is of any other type, the argument is converted to a
 * string as if by a call to the string function; the string is split
 * into a whitespace-separated list of tokens (whitespace is any sequence
 * of characters matching the production S); the result is a node-set
 * containing the elements in the same document as the context node that
 * have a unique ID equal to any of the tokens in the list.
 */
void xmlXPathIdFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathIdFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathLocalNameFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the local-name() XPath function
 *    string local-name(node-set?)
 * The local-name function returns a string containing the local part
 * of the name of the node in the argument node-set that is first in
 * document order. If the node-set is empty or the first node has no
 * name, an empty string is returned. If the argument is omitted it
 * defaults to the context node.
 */
void xmlXPathLocalNameFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathLocalNameFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathNamespaceURIFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the namespace-uri() XPath function
 *    string namespace-uri(node-set?)
 * The namespace-uri function returns a string containing the
 * namespace URI of the expanded name of the node in the argument
 * node-set that is first in document order. If the node-set is empty,
 * the first node has no name, or the expanded name has no namespace
 * URI, an empty string is returned. If the argument is omitted it
 * defaults to the context node.
 */
void xmlXPathNamespaceURIFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathNamespaceURIFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathStringFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the string() XPath function
 *    string string(object?)
 * The string function converts an object to a string as follows:
 *    - A node-set is converted to a string by returning the value of
 *      the node in the node-set that is first in document order.
 *      If the node-set is empty, an empty string is returned.
 *    - A number is converted to a string as follows
 *      + NaN is converted to the string NaN
 *      + positive zero is converted to the string 0
 *      + negative zero is converted to the string 0
 *      + positive infinity is converted to the string Infinity
 *      + negative infinity is converted to the string -Infinity
 *      + if the number is an integer, the number is represented in
 *        decimal form as a Number with no decimal point and no leading
 *        zeros, preceded by a minus sign (-) if the number is negative
 *      + otherwise, the number is represented in decimal form as a
 *        Number including a decimal point with at least one digit
 *        before the decimal point and at least one digit after the
 *        decimal point, preceded by a minus sign (-) if the number
 *        is negative; there must be no leading zeros before the decimal
 *        point apart possibly from the one required digit immediately
 *        before the decimal point; beyond the one required digit
 *        after the decimal point there must be as many, but only as
 *        many, more digits as are needed to uniquely distinguish the
 *        number from all other IEEE 754 numeric values.
 *    - The boolean false value is converted to the string false.
 *      The boolean true value is converted to the string true.
 *
 * If the argument is omitted, it defaults to a node-set with the
 * context node as its only member.
 */
void xmlXPathStringFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathStringFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathStringLengthFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the string-length() XPath function
 *    number string-length(string?)
 * The string-length returns the number of characters in the string
 * (see [3.6 Strings]). If the argument is omitted, it defaults to
 * the context node converted to a string, in other words the value
 * of the context node.
 */
void xmlXPathStringLengthFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathStringLengthFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathConcatFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the concat() XPath function
 *    string concat(string, string, string*)
 * The concat function returns the concatenation of its arguments.
 */
void xmlXPathConcatFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathConcatFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathContainsFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the contains() XPath function
 *    boolean contains(string, string)
 * The contains function returns true if the first argument string
 * contains the second argument string, and otherwise returns false.
 */
void xmlXPathContainsFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathContainsFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathStartsWithFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the starts-with() XPath function
 *    boolean starts-with(string, string)
 * The starts-with function returns true if the first argument string
 * starts with the second argument string, and otherwise returns false.
 */
void xmlXPathStartsWithFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathStartsWithFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathSubstringFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the substring() XPath function
 *    string substring(string, number, number?)
 * The substring function returns the substring of the first argument
 * starting at the position specified in the second argument with
 * length specified in the third argument. For example,
 * substring("12345",2,3) returns "234". If the third argument is not
 * specified, it returns the substring starting at the position specified
 * in the second argument and continuing to the end of the string. For
 * example, substring("12345",2) returns "2345".  More precisely, each
 * character in the string (see [3.6 Strings]) is considered to have a
 * numeric position: the position of the first character is 1, the position
 * of the second character is 2 and so on. The returned substring contains
 * those characters for which the position of the character is greater than
 * or equal to the second argument and, if the third argument is specified,
 * less than the sum of the second and third arguments; the comparisons
 * and addition used for the above follow the standard IEEE 754 rules. Thus:
 *  - substring("12345", 1.5, 2.6) returns "234"
 *  - substring("12345", 0, 3) returns "12"
 *  - substring("12345", 0 div 0, 3) returns ""
 *  - substring("12345", 1, 0 div 0) returns ""
 *  - substring("12345", -42, 1 div 0) returns "12345"
 *  - substring("12345", -1 div 0, 1 div 0) returns ""
 */
void xmlXPathSubstringFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathSubstringFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathSubstringBeforeFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the substring-before() XPath function
 *    string substring-before(string, string)
 * The substring-before function returns the substring of the first
 * argument string that precedes the first occurrence of the second
 * argument string in the first argument string, or the empty string
 * if the first argument string does not contain the second argument
 * string. For example, substring-before("1999/04/01","/") returns 1999.
 */
void xmlXPathSubstringBeforeFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathSubstringBeforeFunction_rust(ctxt, nargs);
}

void xmlXPathSubstringAfterFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathSubstringAfterFunction_rust(ctxt, nargs);
}

void xmlXPathNormalizeFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathNormalizeFunction_rust(ctxt, nargs);
}

void xmlXPathTranslateFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathTranslateFunction_rust(ctxt, nargs);
}

void xmlXPathBooleanFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathBooleanFunction_rust(ctxt, nargs);
}

void xmlXPathNotFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathNotFunction_rust(ctxt, nargs);
}

void xmlXPathTrueFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathTrueFunction_rust(ctxt, nargs);
}

void xmlXPathFalseFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathFalseFunction_rust(ctxt, nargs);
}

void xmlXPathLangFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathLangFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathNumberFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the number() XPath function
 *    number number(object?)
 */
void xmlXPathNumberFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathNumberFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathSumFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the sum() XPath function
 *    number sum(node-set)
 * The sum function returns the sum of the values of the nodes in
 * the argument node-set.
 */
void xmlXPathSumFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathSumFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathFloorFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the floor() XPath function
 *    number floor(number)
 * The floor function returns the largest (closest to positive infinity)
 * number that is not greater than the argument and that is an integer.
 */
void xmlXPathFloorFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathFloorFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathCeilingFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the ceiling() XPath function
 *    number ceiling(number)
 * The ceiling function returns the smallest (closest to negative infinity)
 * number that is not less than the argument and that is an integer.
 */
void xmlXPathCeilingFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathCeilingFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathRoundFunction:
 * @ctxt:  the XPath Parser context
 * @nargs:  the number of arguments
 *
 * Implement the round() XPath function
 *    number round(number)
 * The round function returns the number that is closest to the
 * argument and that is an integer. If there are two such numbers,
 * then the one that is closest to positive infinity is returned.
 */
void xmlXPathRoundFunction(xmlXPathParserContextPtr ctxt, int nargs)
{
	xmlXPathRoundFunction_rust(ctxt, nargs);
}

/**
 * xmlXPathParseNCName:
 * @ctxt:  the XPath Parser context
 *
 * parse an XML namespace non qualified name.
 *
 * [NS 3] NCName ::= (Letter | '_') (NCNameChar)*
 *
 * [NS 4] NCNameChar ::= Letter | Digit | '.' | '-' | '_' |
 *                       CombiningChar | Extender
 *
 * Returns the namespace name or NULL
 */

xmlChar *
xmlXPathParseNCName(xmlXPathParserContextPtr ctxt)
{
	return xmlXPathParseNCName_rust(ctxt);
}

/**
 * xmlXPathParseName:
 * @ctxt:  the XPath Parser context
 *
 * parse an XML name
 *
 * [4] NameChar ::= Letter | Digit | '.' | '-' | '_' | ':' |
 *                  CombiningChar | Extender
 *
 * [5] Name ::= (Letter | '_' | ':') (NameChar)*
 *
 * Returns the namespace name or NULL
 */

xmlChar *
xmlXPathParseName(xmlXPathParserContextPtr ctxt)
{
	return xmlXPathParseName_rust(ctxt);
}



/**
 * xmlXPathStringEvalNumber:
 * @str:  A string to scan
 *
 *  [30a]  Float  ::= Number ('e' Digits?)?
 *
 *  [30]   Number ::=   Digits ('.' Digits?)?
 *                    | '.' Digits
 *  [31]   Digits ::=   [0-9]+
 *
 * Compile a Number in the string
 * In complement of the Number expression, this function also handles
 * negative values : '-' Number.
 *
 * Returns the double value.
 */
double
xmlXPathStringEvalNumber(const xmlChar *str)
{
	return xmlXPathStringEvalNumber_rust(str);
}

/**
 * xmlXPathIsNodeType:
 * @name:  a name string
 *
 * Is the name given a NodeType one.
 *
 *  [38]   NodeType ::=   'comment'
 *                    | 'text'
 *                    | 'processing-instruction'
 *                    | 'node'
 *
 * Returns 1 if true 0 otherwise
 */
int xmlXPathIsNodeType(const xmlChar *name)
{
	return xmlXPathIsNodeType_rust(name);
}

/**
 * xmlXPathEvalPredicate:
 * @ctxt:  the XPath context
 * @res:  the Predicate Expression evaluation result
 *
 * Evaluate a predicate result for the current node.
 * A PredicateExpr is evaluated by evaluating the Expr and converting
 * the result to a boolean. If the result is a number, the result will
 * be converted to true if the number is equal to the position of the
 * context node in the context node list (as returned by the position
 * function) and will be converted to false otherwise; if the result
 * is not a number, then the result will be converted as if by a call
 * to the boolean function.
 *
 * Returns 1 if predicate is true, 0 otherwise
 */
int xmlXPathEvalPredicate(xmlXPathContextPtr ctxt, xmlXPathObjectPtr res)
{
	return xmlXPathEvalPredicate_rust(ctxt, res);
}

/**
 * xmlXPathEvaluatePredicateResult:
 * @ctxt:  the XPath Parser context
 * @res:  the Predicate Expression evaluation result
 *
 * Evaluate a predicate result for the current node.
 * A PredicateExpr is evaluated by evaluating the Expr and converting
 * the result to a boolean. If the result is a number, the result will
 * be converted to true if the number is equal to the position of the
 * context node in the context node list (as returned by the position
 * function) and will be converted to false otherwise; if the result
 * is not a number, then the result will be converted as if by a call
 * to the boolean function.
 *
 * Returns 1 if predicate is true, 0 otherwise
 */
int xmlXPathEvaluatePredicateResult(xmlXPathParserContextPtr ctxt,
									xmlXPathObjectPtr res)
{
	return xmlXPathEvaluatePredicateResult_rust(ctxt, res);
}

/**
 * xmlXPathCtxtCompile:
 * @ctxt: an XPath context
 * @str:  the XPath expression
 *
 * Compile an XPath expression
 *
 * Returns the xmlXPathCompExprPtr resulting from the compilation or NULL.
 *         the caller has to free the object.
 */
xmlXPathCompExprPtr
xmlXPathCtxtCompile(xmlXPathContextPtr ctxt, const xmlChar *str)
{
	return xmlXPathCtxtCompile_rust(ctxt, str);
}

/**
 * xmlXPathCompile:
 * @str:  the XPath expression
 *
 * Compile an XPath expression
 *
 * Returns the xmlXPathCompExprPtr resulting from the compilation or NULL.
 *         the caller has to free the object.
 */
xmlXPathCompExprPtr
xmlXPathCompile(const xmlChar *str)
{
	return xmlXPathCompile_rust(str);
}

/**
 * xmlXPathCompiledEval:
 * @comp:  the compiled XPath expression
 * @ctx:  the XPath context
 *
 * Evaluate the Precompiled XPath expression in the given context.
 *
 * Returns the xmlXPathObjectPtr resulting from the evaluation or NULL.
 *         the caller has to free the object.
 */
xmlXPathObjectPtr
xmlXPathCompiledEval(xmlXPathCompExprPtr comp, xmlXPathContextPtr ctx)
{
	return xmlXPathCompiledEval_rust(comp, ctx);
}

/**
 * xmlXPathCompiledEvalToBoolean:
 * @comp:  the compiled XPath expression
 * @ctxt:  the XPath context
 *
 * Applies the XPath boolean() function on the result of the given
 * compiled expression.
 *
 * Returns 1 if the expression evaluated to true, 0 if to false and
 *         -1 in API and internal errors.
 */
int xmlXPathCompiledEvalToBoolean(xmlXPathCompExprPtr comp,
								  xmlXPathContextPtr ctxt)
{
	return xmlXPathCompiledEvalToBoolean_rust(comp, ctxt);
}

/**
 * xmlXPathEvalExpr:
 * @ctxt:  the XPath Parser context
 *
 * Parse and evaluate an XPath expression in the given context,
 * then push the result on the context stack
 */
void xmlXPathEvalExpr(xmlXPathParserContextPtr ctxt)
{
	xmlXPathEvalExpr_rust(ctxt);
}

/**
 * xmlXPathEval:
 * @str:  the XPath expression
 * @ctx:  the XPath context
 *
 * Evaluate the XPath Location Path in the given context.
 *
 * Returns the xmlXPathObjectPtr resulting from the evaluation or NULL.
 *         the caller has to free the object.
 */
xmlXPathObjectPtr
xmlXPathEval(const xmlChar *str, xmlXPathContextPtr ctx)
{
	return xmlXPathEval_rust(str, ctx);
}

/**
 * xmlXPathSetContextNode:
 * @node: the node to to use as the context node
 * @ctx:  the XPath context
 *
 * Sets 'node' as the context node. The node must be in the same
 * document as that associated with the context.
 *
 * Returns -1 in case of error or 0 if successful
 */
int xmlXPathSetContextNode(xmlNodePtr node, xmlXPathContextPtr ctx)
{
	return xmlXPathSetContextNode_rust(node, ctx);
}

/**
 * xmlXPathNodeEval:
 * @node: the node to to use as the context node
 * @str:  the XPath expression
 * @ctx:  the XPath context
 *
 * Evaluate the XPath Location Path in the given context. The node 'node'
 * is set as the context node. The context node is not restored.
 *
 * Returns the xmlXPathObjectPtr resulting from the evaluation or NULL.
 *         the caller has to free the object.
 */
xmlXPathObjectPtr
xmlXPathNodeEval(xmlNodePtr node, const xmlChar *str, xmlXPathContextPtr ctx)
{
	return xmlXPathNodeEval_rust(node, str, ctx);
}

/**
 * xmlXPathEvalExpression:
 * @str:  the XPath expression
 * @ctxt:  the XPath context
 *
 * Alias for xmlXPathEval().
 *
 * Returns the xmlXPathObjectPtr resulting from the evaluation or NULL.
 *         the caller has to free the object.
 */
xmlXPathObjectPtr
xmlXPathEvalExpression(const xmlChar *str, xmlXPathContextPtr ctxt)
{
	return xmlXPathEvalExpression_rust(str, ctxt);
}

/**
 * xmlXPathRegisterAllFunctions:
 * @ctxt:  the XPath context
 *
 * Registers all default XPath functions in this context
 */
void xmlXPathRegisterAllFunctions(xmlXPathContextPtr ctxt)
{
	xmlXPathRegisterAllFunctions_rust(ctxt);
}

#endif /* LIBXML_XPATH_ENABLED */

#else  /* not COMPILE_WITH_RUST */

int get_xp_optimized_non_elem_comparison(){
	printf("get_xp_optimized_non_elem_comparison in c");
#ifdef XP_OPTIMIZED_NON_ELEM_COMPARISON
	printf("XP_OPTIMIZED_NON_ELEM_COMPARISON in c");
    return 1;
#else
	printf("XP_OPTIMIZED_NON_ELEM_COMPARISON notin c");
    return 0;
#endif
}

int get_with_tim_sort(){
	printf("get_with_tim_sort in c");
#ifdef WITH_TIM_SORT
    printf("WITH_TIM_SORT in c");
    return 1;
#else
	printf("WITH_TIM_SORT notin c");
    return 0;
#endif
}

int get_xp_optimized_filter_first(){
	printf("get_xp_optimized_filter_first in c");
#ifdef XP_OPTIMIZED_FILTER_FIRST
	printf("XP_OPTIMIZED_FILTER_FIRST in c");
    return 1;
#else
	printf("XP_OPTIMIZED_FILTER_FIRST notin c");
    return 0;
#endif
}

int get_debug_eval_counts(){
	printf("get_debug_eval_counts in c");
#ifdef DEBUG_EVAL_COUNTS
    printf("DEBUG_EVAL_COUNTS in c");
    return 1;
#else
	printf("DEBUG_EVAL_COUNTS notin c");
    return 0;
#endif
}

int get_xpath_streaming(){
	printf("get_xpath_streaming in c");
#ifdef XPATH_STREAMING
    printf("XPATH_STREAMING in c");
    return 1;
#else
	printf("XPATH_STREAMING notin c");
    return 0;
#endif
}

int get_libxml_thread_enabled(){
	printf("get_libxml_thread_enabled in c");
#ifdef LIBXML_THREAD_ENABLED
    printf("LIBXML_THREAD_ENABLED in c");
    return 1;
#else
	printf("LIBXML_THREAD_ENABLED notin c");
    return 0;
#endif
}

int get_libxml_xptr_enabled(){
	printf("get_libxml_xptr_enabled in c");
#ifdef LIBXML_XPTR_ENABLED
	printf("LIBXML_XPTR_ENABLED in c");
    return 1;
#else
	printf("LIBXML_XPTR_ENABLED notin c");
    return 0;
#endif
}


int get_borlandc_or_msc_ver_and_msc_ver(){
#if defined(__BORLANDC__) || (defined(_MSC_VER) && (_MSC_VER == 1200))
	return 1;
#else
	return 0;
#endif
}


int get_xmlxpathnodesetsort(){
#ifdef xmlXPathNodeSetSort
	return 1;
#else
	return 0;
#endif
}


int get_gnuc(){
#ifdef __GNUC__
	return 1;
#else
	return 0;
#endif
}

int get_aix(){
#ifdef _AIX
	return 1;
#else
	return 0;
#endif
}

int get_debug_expr(){
#ifdef DEBUG_EXPR
	return 1;
#else
	return 0;
#endif
}

int get_libxml_docb_enabled(){
#ifdef LIBXML_DOCB_ENABLED
	return 1;
#else
	return 0;
#endif
}

int get_xp_default_cache_on(){
#ifdef XP_DEFAULT_CACHE_ON
	return 1;
#else
	return 0;
#endif
}


int get_xp_debug_obj_usage(){
#ifdef XP_DEBUG_OBJ_USAGE
	return 1;
#else
	return 0;
#endif
}

int get_xml_xml_namespace(){
#ifdef XML_XML_NAMESPACE
	return 1;
#else
	return 0;
#endif	
}

int get_debug_or_debug_step(){
#if defined(DEBUG) || defined(DEBUG_STEP)
	return 1;
#else
	return 0;
#endif	
}


int get_debug(){
#ifdef DEBUG
	return 1;
#else
	return 0;
#endif		
}

int get_isnan(){
#ifdef isnan
	return 1;
#else
	return 0;
#endif	
}

int get_isinf(){
#ifdef isinf
	return 1;
#else
	return 0;
#endif
}

int get_libxml_debug_enabled(){
#ifdef LIBXML_DEBUG_ENABLED
	return 1;
#else
	return 0;
#endif
}

int get_libxml_xpath_enabled_or_libxml_schemas_enabled(){
#if defined(LIBXML_XPATH_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED)
	return 1;
#else
	return 0;
#endif
}


int get_libxml_xpath_enabled(){
#ifdef LIBXML_XPATH_ENABLED
	return 1;
#else
	return 0;
#endif
}

int get_debug_step(){
#ifdef DEBUG_STEP
	return 1;
#else
	return 0;
#endif
}

#if defined(LIBXML_XPATH_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED)

/************************************************************************
 *									*
 *			Floating point stuff				*
 *									*
 ************************************************************************/

double xmlXPathNAN;
double xmlXPathPINF;
double xmlXPathNINF;

/**
 * xmlXPathInit:
 *
 * Initialize the XPath environment
 */
ATTRIBUTE_NO_SANITIZE("float-divide-by-zero")
void xmlXPathInit(void)
{

}

int xmlXPathIsNaN(double val)
{
	return 0;
}

int xmlXPathIsInf(double val)
{
	return 0;
}

#endif /* SCHEMAS or XPATH */

#ifdef LIBXML_XPATH_ENABLED


void xmlXPathErr(xmlXPathParserContextPtr ctxt, int error)
{

}

void xmlXPatherror(xmlXPathParserContextPtr ctxt, const char *file ATTRIBUTE_UNUSED,
				   int line ATTRIBUTE_UNUSED, int no)
{

}

void xmlXPathFreeCompExpr(xmlXPathCompExprPtr comp)
{

}

#ifdef LIBXML_DEBUG_ENABLED

void xmlXPathDebugDumpObject(FILE *output, xmlXPathObjectPtr cur, int depth)
{

}

void xmlXPathDebugDumpCompExpr(FILE *output, xmlXPathCompExprPtr comp,
							   int depth)
{

}
#endif /* LIBXML_DEBUG_ENABLED */


int xmlXPathContextSetCache(xmlXPathContextPtr ctxt,
							int active,
							int value,
							int options)
{
	return 0;
}

xmlXPathObjectPtr
valuePop(xmlXPathParserContextPtr ctxt)
{
	return NULL;
}

int valuePush(xmlXPathParserContextPtr ctxt, xmlXPathObjectPtr value)
{
	return 0;
}

int xmlXPathPopBoolean(xmlXPathParserContextPtr ctxt)
{
	return 0;
}

double
xmlXPathPopNumber(xmlXPathParserContextPtr ctxt)
{
	return 0;
}

xmlChar *
xmlXPathPopString(xmlXPathParserContextPtr ctxt)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathPopNodeSet(xmlXPathParserContextPtr ctxt)
{
	return NULL;
}

void *
xmlXPathPopExternal(xmlXPathParserContextPtr ctxt)
{

}

long xmlXPathOrderDocElems(xmlDocPtr doc)
{
	return 0;
}

int xmlXPathCmpNodes(xmlNodePtr node1, xmlNodePtr node2)
{
	return 0;
}

void xmlXPathNodeSetSort(xmlNodeSetPtr set)
{

}

void xmlXPathNodeSetFreeNs(xmlNsPtr ns)
{
	
}

xmlNodeSetPtr
xmlXPathNodeSetCreate(xmlNodePtr val)
{
	return NULL;
}

int xmlXPathNodeSetContains(xmlNodeSetPtr cur, xmlNodePtr val)
{
	return 0;
}

int xmlXPathNodeSetAddNs(xmlNodeSetPtr cur, xmlNodePtr node, xmlNsPtr ns)
{
	return 0;
}

int xmlXPathNodeSetAdd(xmlNodeSetPtr cur, xmlNodePtr val)
{
	return 0;
}

int xmlXPathNodeSetAddUnique(xmlNodeSetPtr cur, xmlNodePtr val)
{
	return 0;
}

xmlNodeSetPtr
xmlXPathNodeSetMerge(xmlNodeSetPtr val1, xmlNodeSetPtr val2)
{
	return NULL;
}

void xmlXPathNodeSetDel(xmlNodeSetPtr cur, xmlNodePtr val)
{

}

void xmlXPathNodeSetRemove(xmlNodeSetPtr cur, int val)
{

}

void xmlXPathFreeNodeSet(xmlNodeSetPtr obj)
{

}


#if defined(DEBUG) || defined(DEBUG_STEP)

void xmlGenericErrorContextNodeSet(FILE *output, xmlNodeSetPtr obj)
{

}
#endif

xmlXPathObjectPtr
xmlXPathNewNodeSet(xmlNodePtr val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathNewValueTree(xmlNodePtr val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathNewNodeSetList(xmlNodeSetPtr val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathWrapNodeSet(xmlNodeSetPtr val)
{
	return NULL;
}

void xmlXPathFreeNodeSetList(xmlXPathObjectPtr obj)
{

}

xmlNodeSetPtr
xmlXPathDifference(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathIntersection(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathDistinctSorted(xmlNodeSetPtr nodes)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathDistinct(xmlNodeSetPtr nodes)
{
	return NULL;
}

int xmlXPathHasSameNodes(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return 0;
}

xmlNodeSetPtr
xmlXPathNodeLeadingSorted(xmlNodeSetPtr nodes, xmlNodePtr node)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathNodeLeading(xmlNodeSetPtr nodes, xmlNodePtr node)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathLeadingSorted(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathLeading(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathNodeTrailingSorted(xmlNodeSetPtr nodes, xmlNodePtr node)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathNodeTrailing(xmlNodeSetPtr nodes, xmlNodePtr node)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathTrailingSorted(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return NULL;
}

xmlNodeSetPtr
xmlXPathTrailing(xmlNodeSetPtr nodes1, xmlNodeSetPtr nodes2)
{
	return NULL;
}

int xmlXPathRegisterFunc(xmlXPathContextPtr ctxt, const xmlChar *name,
						 xmlXPathFunction f)
{
	return 0;
}

int xmlXPathRegisterFuncNS(xmlXPathContextPtr ctxt, const xmlChar *name,
						   const xmlChar *ns_uri, xmlXPathFunction f)
{
	return 0;
}

void xmlXPathRegisterFuncLookup(xmlXPathContextPtr ctxt,
								xmlXPathFuncLookupFunc f,
								void *funcCtxt)
{

}

xmlXPathFunction
xmlXPathFunctionLookup(xmlXPathContextPtr ctxt, const xmlChar *name)
{
	return NULL;
}

xmlXPathFunction
xmlXPathFunctionLookupNS(xmlXPathContextPtr ctxt, const xmlChar *name,
						 const xmlChar *ns_uri)
{
	return NULL;
}

void xmlXPathRegisteredFuncsCleanup(xmlXPathContextPtr ctxt)
{

}

int xmlXPathRegisterVariable(xmlXPathContextPtr ctxt, const xmlChar *name,
							 xmlXPathObjectPtr value)
{
	return 0;
}

int xmlXPathRegisterVariableNS(xmlXPathContextPtr ctxt, const xmlChar *name,
							   const xmlChar *ns_uri,
							   xmlXPathObjectPtr value)
{
	return 0;
}

void xmlXPathRegisterVariableLookup(xmlXPathContextPtr ctxt,
									xmlXPathVariableLookupFunc f, void *data)
{

}

xmlXPathObjectPtr
xmlXPathVariableLookup(xmlXPathContextPtr ctxt, const xmlChar *name)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathVariableLookupNS(xmlXPathContextPtr ctxt, const xmlChar *name,
						 const xmlChar *ns_uri)
{
	return NULL;
}

void xmlXPathRegisteredVariablesCleanup(xmlXPathContextPtr ctxt)
{

}

int xmlXPathRegisterNs(xmlXPathContextPtr ctxt, const xmlChar *prefix,
					   const xmlChar *ns_uri)
{
	return 0;
}

const xmlChar *
xmlXPathNsLookup(xmlXPathContextPtr ctxt, const xmlChar *prefix)
{
	return NULL;
}

void xmlXPathRegisteredNsCleanup(xmlXPathContextPtr ctxt)
{

}

xmlXPathObjectPtr
xmlXPathNewFloat(double val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathNewBoolean(int val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathNewString(const xmlChar *val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathWrapString(xmlChar *val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathNewCString(const char *val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathWrapCString(char *val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathWrapExternal(void *val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathObjectCopy(xmlXPathObjectPtr val)
{
	return NULL;
}

void xmlXPathFreeObject(xmlXPathObjectPtr obj)
{

}

xmlChar *
xmlXPathCastBooleanToString(int val)
{
	return NULL;
}

xmlChar *
xmlXPathCastNumberToString(double val)
{
	return NULL;
}

xmlChar *
xmlXPathCastNodeToString(xmlNodePtr node)
{
	return NULL;
}

xmlChar *
xmlXPathCastNodeSetToString(xmlNodeSetPtr ns)
{
	return NULL;
}

xmlChar *
xmlXPathCastToString(xmlXPathObjectPtr val)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathConvertString(xmlXPathObjectPtr val)
{
	return NULL;
}

double
xmlXPathCastBooleanToNumber(int val)
{
	return 0;
}

double
xmlXPathCastStringToNumber(const xmlChar *val)
{
	return 0;
}

double
xmlXPathCastNodeToNumber(xmlNodePtr node)
{
	return 0;
}

double
xmlXPathCastNodeSetToNumber(xmlNodeSetPtr ns)
{
	return 0;
}

double
xmlXPathCastToNumber(xmlXPathObjectPtr val)
{
	return 0;
}

xmlXPathObjectPtr
xmlXPathConvertNumber(xmlXPathObjectPtr val)
{
	return NULL;
}

int xmlXPathCastNumberToBoolean(double val)
{
	return 0;
}

int xmlXPathCastStringToBoolean(const xmlChar *val)
{
	return 0;
}

int xmlXPathCastNodeSetToBoolean(xmlNodeSetPtr ns)
{
	return 0;
}

int xmlXPathCastToBoolean(xmlXPathObjectPtr val)
{
	return 0;
}

xmlXPathObjectPtr
xmlXPathConvertBoolean(xmlXPathObjectPtr val)
{
	return NULL;
}

xmlXPathContextPtr
xmlXPathNewContext(xmlDocPtr doc)
{
	return NULL;
}

void xmlXPathFreeContext(xmlXPathContextPtr ctxt)
{

}

xmlXPathParserContextPtr
xmlXPathNewParserContext(const xmlChar *str, xmlXPathContextPtr ctxt)
{
	return NULL;
}

void xmlXPathFreeParserContext(xmlXPathParserContextPtr ctxt)
{

}

int xmlXPathEqualValues(xmlXPathParserContextPtr ctxt)
{
	return 0;
}

int xmlXPathNotEqualValues(xmlXPathParserContextPtr ctxt)
{
	return 0;
}

int xmlXPathCompareValues(xmlXPathParserContextPtr ctxt, int inf, int strict)
{
	return 0;
}

void xmlXPathValueFlipSign(xmlXPathParserContextPtr ctxt)
{

}

void xmlXPathAddValues(xmlXPathParserContextPtr ctxt)
{

}

void xmlXPathSubValues(xmlXPathParserContextPtr ctxt)
{

}

void xmlXPathMultValues(xmlXPathParserContextPtr ctxt)
{

}

ATTRIBUTE_NO_SANITIZE("float-divide-by-zero")
void xmlXPathDivValues(xmlXPathParserContextPtr ctxt)
{

}

void xmlXPathModValues(xmlXPathParserContextPtr ctxt)
{

}

typedef xmlNodePtr (*xmlXPathTraversalFunction)(xmlXPathParserContextPtr ctxt, xmlNodePtr cur);

typedef xmlNodePtr (*xmlXPathTraversalFunctionExt)(xmlNodePtr cur, xmlNodePtr contextNode);

typedef xmlNodeSetPtr (*xmlXPathNodeSetMergeFunction)(xmlNodeSetPtr, xmlNodeSetPtr);

xmlNodePtr
xmlXPathNextSelf(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextChild(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextDescendant(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextDescendantOrSelf(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextParent(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextAncestor(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextAncestorOrSelf(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextFollowingSibling(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextPrecedingSibling(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextFollowing(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextPreceding(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextNamespace(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

xmlNodePtr
xmlXPathNextAttribute(xmlXPathParserContextPtr ctxt, xmlNodePtr cur)
{
	return NULL;
}

void xmlXPathRoot(xmlXPathParserContextPtr ctxt)
{

}

void xmlXPathLastFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathPositionFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathCountFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathIdFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathLocalNameFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathNamespaceURIFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathStringFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathStringLengthFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathConcatFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathContainsFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathStartsWithFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathSubstringFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathSubstringBeforeFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathSubstringAfterFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathNormalizeFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathTranslateFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathBooleanFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathNotFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathTrueFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathFalseFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathLangFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathNumberFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathSumFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathFloorFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathCeilingFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

void xmlXPathRoundFunction(xmlXPathParserContextPtr ctxt, int nargs)
{

}

xmlChar *
xmlXPathParseNCName(xmlXPathParserContextPtr ctxt)
{
	return NULL;
}

xmlChar *
xmlXPathParseName(xmlXPathParserContextPtr ctxt)
{
	return NULL;
}

double
xmlXPathStringEvalNumber(const xmlChar *str)
{
	return 0;
}

int xmlXPathIsNodeType(const xmlChar *name)
{
	return 0;
}

int xmlXPathEvalPredicate(xmlXPathContextPtr ctxt, xmlXPathObjectPtr res)
{
	return 0;
}

int xmlXPathEvaluatePredicateResult(xmlXPathParserContextPtr ctxt,
									xmlXPathObjectPtr res)
{
	return 0;
}

xmlXPathCompExprPtr
xmlXPathCtxtCompile(xmlXPathContextPtr ctxt, const xmlChar *str)
{
	return NULL;
}

xmlXPathCompExprPtr
xmlXPathCompile(const xmlChar *str)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathCompiledEval(xmlXPathCompExprPtr comp, xmlXPathContextPtr ctx)
{
	return NULL;
}

int xmlXPathCompiledEvalToBoolean(xmlXPathCompExprPtr comp,
								  xmlXPathContextPtr ctxt)
{
	return 0;
}

void xmlXPathEvalExpr(xmlXPathParserContextPtr ctxt)
{

}

xmlXPathObjectPtr
xmlXPathEval(const xmlChar *str, xmlXPathContextPtr ctx)
{
	return NULL;
}

int xmlXPathSetContextNode(xmlNodePtr node, xmlXPathContextPtr ctx)
{
	return 0;
}

xmlXPathObjectPtr
xmlXPathNodeEval(xmlNodePtr node, const xmlChar *str, xmlXPathContextPtr ctx)
{
	return NULL;
}

xmlXPathObjectPtr
xmlXPathEvalExpression(const xmlChar *str, xmlXPathContextPtr ctxt)
{
	return NULL;
}

void xmlXPathRegisterAllFunctions(xmlXPathContextPtr ctxt)
{

}

#endif /* LIBXML_XPATH_ENABLED */

#endif /* COMPILE_WITH_RUST */


#define bottom_xpath
#include "elfgcchack.h"