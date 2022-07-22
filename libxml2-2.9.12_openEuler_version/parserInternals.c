/*
 * parserInternals.c : Internal routines (and obsolete ones) needed for the
 *                     XML and HTML parsers.
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */

#define IN_LIBXML
#include "libxml.h"

#if defined(_WIN32) && !defined(__CYGWIN__)
#define XML_DIR_SEP '\\'
#else
#define XML_DIR_SEP '/'
#endif

#include <string.h>
#ifdef HAVE_CTYPE_H
#include <ctype.h>
#endif
#ifdef HAVE_STDLIB_H
#include <stdlib.h>
#endif
#ifdef HAVE_SYS_STAT_H
#include <sys/stat.h>
#endif
#ifdef HAVE_FCNTL_H
#include <fcntl.h>
#endif
#ifdef HAVE_UNISTD_H
#include <unistd.h>
#endif
#ifdef LIBXML_ZLIB_ENABLED
#include <zlib.h>
#endif

#include <libxml/xmlmemory.h>
#include <libxml/tree.h>
#include <libxml/parser.h>
#include <libxml/parserInternals.h>
#include <libxml/valid.h>
#include <libxml/entities.h>
#include <libxml/xmlerror.h>
#include <libxml/encoding.h>
#include <libxml/valid.h>
#include <libxml/xmlIO.h>
#include <libxml/uri.h>
#include <libxml/dict.h>
#include <libxml/SAX.h>
#ifdef LIBXML_CATALOG_ENABLED
#include <libxml/catalog.h>
#endif
#include <libxml/globals.h>
#include <libxml/chvalid.h>

#define CUR(ctxt) ctxt->input->cur
#define END(ctxt) ctxt->input->end
#define VALID_CTXT(ctxt) (CUR(ctxt) <= END(ctxt))

#include "buf.h"
#include "enc.h"

/* #define DEBUG_INPUT */
/* #define DEBUG_STACK */
/* #define DEBUG_PUSH */

/* we need to keep enough input to show errors in context */
#define LINE_LEN 80

#ifdef DEBUG_INPUT
#define CHECK_BUFFER(in) check_buffer(in)

static void check_buffer(xmlParserInputPtr in)
{
    if (in->base != xmlBufContent(in->buf->buffer))
    {
        xmlGenericError(xmlGenericErrorContext,
                        "xmlParserInput: base mismatch problem\n");
    }
    if (in->cur < in->base)
    {
        xmlGenericError(xmlGenericErrorContext,
                        "xmlParserInput: cur < base problem\n");
    }
    if (in->cur > in->base + xmlBufUse(in->buf->buffer))
    {
        xmlGenericError(xmlGenericErrorContext,
                        "xmlParserInput: cur > base + use problem\n");
    }
    xmlGenericError(xmlGenericErrorContext, "buffer %x : content %x, cur %d, use %d\n",
                    (int)in, (int)xmlBufContent(in->buf->buffer), in->cur - in->base,
                    xmlBufUse(in->buf->buffer));
}
#else
#define CHECK_BUFFER(in)
#endif

#ifdef COMPILE_WITH_RUST
// region1: COMPILE_WITH_RUST宏定义的部分表示需要与rust交互，只有在连接后才生效
extern void xmlCheckVersion_rust(int version);
extern void xmlErrMemory_rust(xmlParserCtxtPtr ctxt, const char *extra);
extern void __xmlErrEncoding_rust(xmlParserCtxtPtr ctxt, xmlParserErrors xmlerr, const char *msg, const xmlChar *str1, const xmlChar *str2);
extern void xmlErrEncodingInt_rust(xmlParserCtxtPtr ctxt, xmlParserErrors error, const char *msg, int val);
extern void xmlErrInternal_rust(xmlParserCtxtPtr ctxt, const char *msg, const xmlChar *str);
extern int xmlIsLetter_rust(int c);
extern void check_buffer_rust(xmlParserInputPtr in);
extern int xmlParserInputRead_rust(xmlParserInputPtr in ATTRIBUTE_UNUSED, int len ATTRIBUTE_UNUSED);
extern int xmlParserInputGrow_rust(xmlParserInputPtr in, int len);
extern void xmlParserInputShrink_rust(xmlParserInputPtr in);
extern void xmlNextChar_rust(xmlParserCtxtPtr ctxt);
extern int xmlCurrentChar_rust(xmlParserCtxtPtr ctxt, int *len);
extern int xmlStringCurrentChar_rust(xmlParserCtxtPtr ctxt, const xmlChar *cur, int *len);
extern int xmlCopyCharMultiByte_rust(xmlChar *out, int val);
extern int xmlCopyChar_rust(int len ATTRIBUTE_UNUSED, xmlChar *out, int val);
extern int xmlSwitchEncoding_rust(xmlParserCtxtPtr ctxt, xmlCharEncoding enc);
extern int xmlSwitchInputEncodingInt_rust(xmlParserCtxtPtr ctxt, xmlParserInputPtr input, xmlCharEncodingHandlerPtr handler, int len);
extern int xmlSwitchInputEncoding_rust(xmlParserCtxtPtr ctxt, xmlParserInputPtr input, xmlCharEncodingHandlerPtr handler);
extern int xmlSwitchToEncodingInt_rust(xmlParserCtxtPtr ctxt, xmlCharEncodingHandlerPtr handler, int len);
extern int xmlSwitchToEncoding_rust(xmlParserCtxtPtr ctxt, xmlCharEncodingHandlerPtr handler);
extern void xmlFreeInputStream_rust(xmlParserInputPtr input);
extern xmlParserInputPtr xmlNewInputStream_rust(xmlParserCtxtPtr ctxt);
extern xmlParserInputPtr xmlNewIOInputStream_rust(xmlParserCtxtPtr ctxt, xmlParserInputBufferPtr input, xmlCharEncoding enc);
extern xmlParserInputPtr xmlNewEntityInputStream_rust(xmlParserCtxtPtr ctxt, xmlEntityPtr entity);
extern xmlParserInputPtr xmlNewStringInputStream_rust(xmlParserCtxtPtr ctxt, const xmlChar *buffer);
extern xmlParserInputPtr xmlNewInputFromFile_rust(xmlParserCtxtPtr ctxt, const char *filename);
extern int xmlInitParserCtxt_rust(xmlParserCtxtPtr ctxt);
extern void xmlFreeParserCtxt_rust(xmlParserCtxtPtr ctxt);
extern xmlParserCtxtPtr xmlNewParserCtxt_rust(void);
extern void xmlClearParserCtxt_rust(xmlParserCtxtPtr ctxt);
extern xmlParserNodeInfo *xmlParserFindNodeInfo_rust(const xmlParserCtxtPtr ctx, const xmlNodePtr node);
extern void xmlInitNodeInfoSeq_rust(xmlParserNodeInfoSeqPtr seq);
extern void xmlClearNodeInfoSeq_rust(xmlParserNodeInfoSeqPtr seq);
extern unsigned long xmlParserFindNodeInfoIndex_rust(const xmlParserNodeInfoSeqPtr seq, const xmlNodePtr node);
extern void xmlParserAddNodeInfo_rust(xmlParserCtxtPtr ctxt, const xmlParserNodeInfoPtr info);
extern int xmlPedanticParserDefault_rust(int val);
extern int xmlLineNumbersDefault_rust(int val);
extern int xmlSubstituteEntitiesDefault_rust(int val);
extern int xmlKeepBlanksDefault_rust(int val);

/**
 * xmlCheckVersion:
 * @version: the include version number
 *
 * check the compiled lib version against the include one.
 * This can warn or immediately kill the application
 */
void xmlCheckVersion(int version)
{
    xmlCheckVersion_rust(version);
}

/**
 * xmlErrMemory:
 * @ctxt:  an XML parser context
 * @extra:  extra information
 *
 * Handle a redefinition of attribute error
 */
void xmlErrMemory(xmlParserCtxtPtr ctxt, const char *extra)
{
    xmlErrMemory_rust(ctxt, extra);
}

/**
 * __xmlErrEncoding:
 * @ctxt:  an XML parser context
 * @xmlerr:  the error number
 * @msg:  the error message
 * @str1:  an string info
 * @str2:  an string info
 *
 * Handle an encoding error
 */
void __xmlErrEncoding(xmlParserCtxtPtr ctxt, xmlParserErrors xmlerr,
                      const char *msg, const xmlChar *str1, const xmlChar *str2)
{
    __xmlErrEncoding_rust(ctxt, xmlerr, msg, str1, str2);
}

/**
 * xmlIsLetter:
 * @c:  an unicode character (int)
 *
 * Check whether the character is allowed by the production
 * [84] Letter ::= BaseChar | Ideographic
 *
 * Returns 0 if not, non-zero otherwise
 */
int xmlIsLetter(int c)
{
    return xmlIsLetter_rust(c);
}

/**
 * xmlParserInputRead:
 * @in:  an XML parser input
 * @len:  an indicative size for the lookahead
 *
 * This function was internal and is deprecated.
 *
 * Returns -1 as this is an error to use it.
 */
int xmlParserInputRead(xmlParserInputPtr in ATTRIBUTE_UNUSED, int len ATTRIBUTE_UNUSED)
{
    return (-1);
}

/**
 * xmlParserInputGrow:
 * @in:  an XML parser input
 * @len:  an indicative size for the lookahead
 *
 * This function increase the input for the parser. It tries to
 * preserve pointers to the input buffer, and keep already read data
 *
 * Returns the amount of char read, or -1 in case of error, 0 indicate the
 * end of this entity
 */
int xmlParserInputGrow(xmlParserInputPtr in, int len)
{
    return xmlParserInputGrow_rust(in, len);
}

/**
 * xmlParserInputShrink:
 * @in:  an XML parser input
 *
 * This function removes used input for the parser.
 */
void xmlParserInputShrink(xmlParserInputPtr in)
{
    xmlParserInputShrink_rust(in);
}

/**
 * xmlNextChar:
 * @ctxt:  the XML parser context
 *
 * Skip to the next char input char.
 */
void xmlNextChar(xmlParserCtxtPtr ctxt)
{
    xmlNextChar_rust(ctxt);
}

/**
 * xmlCurrentChar:
 * @ctxt:  the XML parser context
 * @len:  pointer to the length of the char read
 *
 * The current char value, if using UTF-8 this may actually span multiple
 * bytes in the input buffer. Implement the end of line normalization:
 * 2.11 End-of-Line Handling
 * Wherever an external parsed entity or the literal entity value
 * of an internal parsed entity contains either the literal two-character
 * sequence "#xD#xA" or a standalone literal #xD, an XML processor
 * must pass to the application the single character #xA.
 * This behavior can conveniently be produced by normalizing all
 * line breaks to #xA on input, before parsing.)
 *
 * Returns the current char value and its length
 */

int xmlCurrentChar(xmlParserCtxtPtr ctxt, int *len)
{
    return xmlCurrentChar_rust(ctxt, len);
}

/**
 * xmlStringCurrentChar:
 * @ctxt:  the XML parser context
 * @cur:  pointer to the beginning of the char
 * @len:  pointer to the length of the char read
 *
 * The current char value, if using UTF-8 this may actually span multiple
 * bytes in the input buffer.
 *
 * Returns the current char value and its length
 */
int xmlStringCurrentChar(xmlParserCtxtPtr ctxt, const xmlChar *cur, int *len)
{
    return xmlStringCurrentChar_rust(ctxt, cur, len);
}

/**
 * xmlCopyCharMultiByte:
 * @out:  pointer to an array of xmlChar
 * @val:  the char value
 *
 * append the char value in the array
 *
 * Returns the number of xmlChar written
 */
int xmlCopyCharMultiByte(xmlChar *out, int val)
{

    return xmlCopyCharMultiByte_rust(out, val);
}

/**
 * xmlCopyChar:
 * @len:  Ignored, compatibility
 * @out:  pointer to an array of xmlChar
 * @val:  the char value
 *
 * append the char value in the array
 *
 * Returns the number of xmlChar written
 */

int xmlCopyChar(int len ATTRIBUTE_UNUSED, xmlChar *out, int val)
{
    return xmlCopyChar_rust(len, out, val);
}

/**
 * xmlSwitchEncoding:
 * @ctxt:  the parser context
 * @enc:  the encoding value (number)
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */
int xmlSwitchEncoding(xmlParserCtxtPtr ctxt, xmlCharEncoding enc)
{
    return xmlSwitchEncoding_rust(ctxt, enc);
}

/**
 * xmlSwitchInputEncoding:
 * @ctxt:  the parser context
 * @input:  the input stream
 * @handler:  the encoding handler
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */
int xmlSwitchInputEncoding(xmlParserCtxtPtr ctxt, xmlParserInputPtr input,
                           xmlCharEncodingHandlerPtr handler)
{
    return xmlSwitchInputEncoding_rust(ctxt, input, handler);
}

/**
 * xmlSwitchToEncoding:
 * @ctxt:  the parser context
 * @handler:  the encoding handler
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */
int xmlSwitchToEncoding(xmlParserCtxtPtr ctxt, xmlCharEncodingHandlerPtr handler)
{
    return xmlSwitchToEncoding_rust(ctxt, handler);
}

/************************************************************************
 *									*
 *	Commodity functions to handle entities processing		*
 *									*
 ************************************************************************/

/**
 * xmlFreeInputStream:
 * @input:  an xmlParserInputPtr
 *
 * Free up an input stream.
 */
void xmlFreeInputStream(xmlParserInputPtr input)
{
    xmlFreeInputStream_rust(input);
}

/**
 * xmlNewInputStream:
 * @ctxt:  an XML parser context
 *
 * Create a new input stream structure.
 *
 * Returns the new input stream or NULL
 */
xmlParserInputPtr
xmlNewInputStream(xmlParserCtxtPtr ctxt)
{
    return xmlNewInputStream_rust(ctxt);
}

/**
 * xmlNewIOInputStream:
 * @ctxt:  an XML parser context
 * @input:  an I/O Input
 * @enc:  the charset encoding if known
 *
 * Create a new input stream structure encapsulating the @input into
 * a stream suitable for the parser.
 *
 * Returns the new input stream or NULL
 */
xmlParserInputPtr
xmlNewIOInputStream(xmlParserCtxtPtr ctxt, xmlParserInputBufferPtr input,
                    xmlCharEncoding enc)
{
    return xmlNewIOInputStream_rust(ctxt, input, enc);
}

/**
 * xmlNewEntityInputStream:
 * @ctxt:  an XML parser context
 * @entity:  an Entity pointer
 *
 * Create a new input stream based on an xmlEntityPtr
 *
 * Returns the new input stream or NULL
 */
xmlParserInputPtr
xmlNewEntityInputStream(xmlParserCtxtPtr ctxt, xmlEntityPtr entity)
{
    return xmlNewEntityInputStream_rust(ctxt, entity);
}

/**
 * xmlNewStringInputStream:
 * @ctxt:  an XML parser context
 * @buffer:  an memory buffer
 *
 * Create a new input stream based on a memory buffer.
 * Returns the new input stream
 */
xmlParserInputPtr
xmlNewStringInputStream(xmlParserCtxtPtr ctxt, const xmlChar *buffer)
{
    return xmlNewStringInputStream_rust(ctxt, buffer);
}

/**
 * xmlNewInputFromFile:
 * @ctxt:  an XML parser context
 * @filename:  the filename to use as entity
 *
 * Create a new input stream based on a file or an URL.
 *
 * Returns the new input stream or NULL in case of error
 */
xmlParserInputPtr
xmlNewInputFromFile(xmlParserCtxtPtr ctxt, const char *filename)
{
    return xmlNewInputFromFile_rust(ctxt, filename);
}

/************************************************************************
 *									*
 *		Commodity functions to handle parser contexts		*
 *									*
 ************************************************************************/

/**
 * xmlInitParserCtxt:
 * @ctxt:  an XML parser context
 *
 * Initialize a parser context
 *
 * Returns 0 in case of success and -1 in case of error
 */

int xmlInitParserCtxt(xmlParserCtxtPtr ctxt)
{
    return xmlInitParserCtxt_rust(ctxt);
}

/**
 * xmlFreeParserCtxt:
 * @ctxt:  an XML parser context
 *
 * Free all the memory used by a parser context. However the parsed
 * document in ctxt->myDoc is not freed.
 */

void xmlFreeParserCtxt(xmlParserCtxtPtr ctxt)
{
    xmlFreeParserCtxt_rust(ctxt);
}

/**
 * xmlNewParserCtxt:
 *
 * Allocate and initialize a new parser context.
 *
 * Returns the xmlParserCtxtPtr or NULL
 */

xmlParserCtxtPtr
xmlNewParserCtxt(void)
{
    return xmlNewParserCtxt_rust();
}

/************************************************************************
 *									*
 *		Handling of node information				*
 *									*
 ************************************************************************/

/**
 * xmlClearParserCtxt:
 * @ctxt:  an XML parser context
 *
 * Clear (release owned resources) and reinitialize a parser context
 */

void xmlClearParserCtxt(xmlParserCtxtPtr ctxt)
{
    xmlClearParserCtxt_rust(ctxt);
}

/**
 * xmlParserFindNodeInfo:
 * @ctx:  an XML parser context
 * @node:  an XML node within the tree
 *
 * Find the parser node info struct for a given node
 *
 * Returns an xmlParserNodeInfo block pointer or NULL
 */
const xmlParserNodeInfo *
xmlParserFindNodeInfo(const xmlParserCtxtPtr ctx, const xmlNodePtr node)
{
    return xmlParserFindNodeInfo_rust(ctx, node);
}

/**
 * xmlInitNodeInfoSeq:
 * @seq:  a node info sequence pointer
 *
 * -- Initialize (set to initial state) node info sequence
 */
void xmlInitNodeInfoSeq(xmlParserNodeInfoSeqPtr seq)
{
    xmlInitNodeInfoSeq_rust(seq);
}

/**
 * xmlClearNodeInfoSeq:
 * @seq:  a node info sequence pointer
 *
 * -- Clear (release memory and reinitialize) node
 *   info sequence
 */
void xmlClearNodeInfoSeq(xmlParserNodeInfoSeqPtr seq)
{
    xmlClearNodeInfoSeq_rust(seq);
}

/**
 * xmlParserFindNodeInfoIndex:
 * @seq:  a node info sequence pointer
 * @node:  an XML node pointer
 *
 *
 * xmlParserFindNodeInfoIndex : Find the index that the info record for
 *   the given node is or should be at in a sorted sequence
 *
 * Returns a long indicating the position of the record
 */
unsigned long
xmlParserFindNodeInfoIndex(const xmlParserNodeInfoSeqPtr seq,
                           const xmlNodePtr node)
{
    return xmlParserFindNodeInfoIndex_rust(seq, node);
}

/**
 * xmlParserAddNodeInfo:
 * @ctxt:  an XML parser context
 * @info:  a node info sequence pointer
 *
 * Insert node info record into the sorted sequence
 */
void xmlParserAddNodeInfo(xmlParserCtxtPtr ctxt,
                          const xmlParserNodeInfoPtr info)
{
    xmlParserAddNodeInfo_rust(ctxt, info);
}

/************************************************************************
 *									*
 *		Defaults settings					*
 *									*
 ************************************************************************/
/**
 * xmlPedanticParserDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for enabling pedantic warnings.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */

int xmlPedanticParserDefault(int val)
{
    return xmlPedanticParserDefault_rust(val);
}

/**
 * xmlLineNumbersDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for enabling line numbers in elements
 * contents. This may break on old application and is turned off by default.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */

int xmlLineNumbersDefault(int val)
{
    return xmlLineNumbersDefault_rust(val);
}

/**
 * xmlSubstituteEntitiesDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for default entity support.
 * Initially the parser always keep entity references instead of substituting
 * entity values in the output. This function has to be used to change the
 * default parser behavior
 * SAX::substituteEntities() has to be used for changing that on a file by
 * file basis.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */

int xmlSubstituteEntitiesDefault(int val)
{
    return xmlSubstituteEntitiesDefault_rust(val);
}

/**
 * xmlKeepBlanksDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for default blanks text nodes support.
 * The 1.x version of the parser used an heuristic to try to detect
 * ignorable white spaces. As a result the SAX callback was generating
 * xmlSAX2IgnorableWhitespace() callbacks instead of characters() one, and when
 * using the DOM output text nodes containing those blanks were not generated.
 * The 2.x and later version will switch to the XML standard way and
 * ignorableWhitespace() are only generated when running the parser in
 * validating mode and when the current element doesn't allow CDATA or
 * mixed content.
 * This function is provided as a way to force the standard behavior
 * on 1.X libs and to switch back to the old mode for compatibility when
 * running 1.X client code on 2.X . Upgrade of 1.X code should be done
 * by using xmlIsBlankNode() commodity function to detect the "empty"
 * nodes generated.
 * This value also affect autogeneration of indentation when saving code
 * if blanks sections are kept, indentation is not generated.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */
int xmlKeepBlanksDefault(int val)
{
    return xmlKeepBlanksDefault_rust(val);
}
// endregion1
#endif

#ifndef COMPILE_WITH_RUST
// region2: 未被COMPILE_WITH_RUST宏定义的部分表示需要连接前需要对外界提供的内容，包括：对rust部分提供的宏出口，以及对外部提供的空壳方法
// region2.1: 宏获取方法
int get_parserInternals_WIN32()
{
#ifdef _WIN32
    return 1;
#else
    return 0;
#endif
}
int get_parserInternals_CYGWIN()
{
#ifdef __CYGWIN__
    return 1;
#else
    return 0;
#endif
}
int get_parserInternals_HAVE_CTYPE_H()
{
#ifdef HAVE_CTYPE_H
    return 1;
#else
    return 0;
#endif
}
int get_parserInternals_HAVE_STDLIB_H()
{
#ifdef HAVE_STDLIB_H
    return 1;
#else
    return 0;
#endif
}
int get_parserInternals_HAVE_SYS_STAT_H()
{
#ifdef HAVE_SYS_STAT_H
    return 1;
#else
    return 0;
#endif
}
int get_parserInternals_HAVE_FCNTL_H()
{
#ifdef HAVE_FCNTL_H
    return 1;
#else
    return 0;
#endif
}
int get_parserInternals_HAVE_UNISTD_H()
{
#ifdef HAVE_UNISTD_H
    return 1;
#else
    return 0;
#endif
}
int get_parserInternals_LIBXML_ZLIB_ENABLED()
{
#ifdef LIBXML_ZLIB_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parserInternals_LIBXML_CATALOG_ENABLED()
{
#ifdef LIBXML_CATALOG_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parserInternals_DEBUG_INPUT()
{
#ifdef DEBUG_INPUT
    return 1;
#else
    return 0;
#endif
}
int get_parserInternals_LIBXML_SAX1_ENABLED()
{
#ifdef LIBXML_SAX1_ENABLED
    return 1;
#else
    return 0;
#endif
}
// endregion2.1

// region2.2: 对外提供的空壳方法，以便编译可以通过
void xmlCheckVersion(int version)
{
}

void xmlErrMemory(xmlParserCtxtPtr ctxt, const char *extra)
{
}

void __xmlErrEncoding(xmlParserCtxtPtr ctxt, xmlParserErrors xmlerr,
                      const char *msg, const xmlChar *str1, const xmlChar *str2)
{
}

int xmlIsLetter(int c)
{
    return 0;
}

int xmlParserInputRead(xmlParserInputPtr in ATTRIBUTE_UNUSED, int len ATTRIBUTE_UNUSED)
{
    return (-1);
}

int xmlParserInputGrow(xmlParserInputPtr in, int len)
{
    return 0;
}

void xmlParserInputShrink(xmlParserInputPtr in)
{
}

void xmlNextChar(xmlParserCtxtPtr ctxt)
{
}

int xmlCurrentChar(xmlParserCtxtPtr ctxt, int *len)
{
    return 0;
}

int xmlStringCurrentChar(xmlParserCtxtPtr ctxt, const xmlChar *cur, int *len)
{
    return 0;
}

int xmlCopyCharMultiByte(xmlChar *out, int val)
{
    return 0;
}

int xmlCopyChar(int len ATTRIBUTE_UNUSED, xmlChar *out, int val)
{
    return 0;
}

int xmlSwitchEncoding(xmlParserCtxtPtr ctxt, xmlCharEncoding enc)
{
    return 0;
}

int xmlSwitchInputEncoding(xmlParserCtxtPtr ctxt, xmlParserInputPtr input,
                           xmlCharEncodingHandlerPtr handler)
{
    return 0;
}

int xmlSwitchToEncoding(xmlParserCtxtPtr ctxt, xmlCharEncodingHandlerPtr handler)
{
    return 0;
}

void xmlFreeInputStream(xmlParserInputPtr input)
{
}

xmlParserInputPtr xmlNewInputStream(xmlParserCtxtPtr ctxt)
{
    return NULL;
}

xmlParserInputPtr xmlNewIOInputStream(xmlParserCtxtPtr ctxt, xmlParserInputBufferPtr input,
                                      xmlCharEncoding enc)
{
    return NULL;
}

xmlParserInputPtr xmlNewEntityInputStream(xmlParserCtxtPtr ctxt, xmlEntityPtr entity)
{
    return NULL;
}

xmlParserInputPtr xmlNewStringInputStream(xmlParserCtxtPtr ctxt, const xmlChar *buffer)
{
    return NULL;
}

xmlParserInputPtr xmlNewInputFromFile(xmlParserCtxtPtr ctxt, const char *filename)
{
    return NULL;
}

int xmlInitParserCtxt(xmlParserCtxtPtr ctxt)
{
    return 0;
}

void xmlFreeParserCtxt(xmlParserCtxtPtr ctxt)
{
}

xmlParserCtxtPtr xmlNewParserCtxt(void)
{
    return NULL;
}

void xmlClearParserCtxt(xmlParserCtxtPtr ctxt)
{
}

const xmlParserNodeInfo *xmlParserFindNodeInfo(const xmlParserCtxtPtr ctx, const xmlNodePtr node)
{
    return NULL;
}

void xmlInitNodeInfoSeq(xmlParserNodeInfoSeqPtr seq)
{
}

void xmlClearNodeInfoSeq(xmlParserNodeInfoSeqPtr seq)
{
}

unsigned long xmlParserFindNodeInfoIndex(const xmlParserNodeInfoSeqPtr seq,
                                         const xmlNodePtr node)
{
    return 0;
}

void xmlParserAddNodeInfo(xmlParserCtxtPtr ctxt,
                          const xmlParserNodeInfoPtr info)
{
}

int xmlPedanticParserDefault(int val)
{
    return 0;
}

int xmlLineNumbersDefault(int val)
{
    return 0;
}

int xmlSubstituteEntitiesDefault(int val)
{
    return 0;
}

int xmlKeepBlanksDefault(int val)
{
    return 0;
}
// endregion2.2

// endregion2
#endif

#define bottom_parserInternals
#include "elfgcchack.h"
