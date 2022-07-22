/*
 * parser.c : an XML 1.0 parser, namespaces and validity support are mostly
 *            implemented on top of the SAX interfaces
 *
 * References:
 *   The XML specification:
 *     http://www.w3.org/TR/REC-xml
 *   Original 1.0 version:
 *     http://www.w3.org/TR/1998/REC-xml-19980210
 *   XML second edition working draft
 *     http://www.w3.org/TR/2000/WD-xml-2e-20000814
 *
 * Okay this is a big file, the parser core is around 7000 lines, then it
 * is followed by the progressive parser top routines, then the various
 * high level APIs to call the parser and a few miscellaneous functions.
 * A number of helper functions and deprecated ones have been moved to
 * parserInternals.c to reduce this file size.
 * As much as possible the functions are associated with their relative
 * production in the XML specification. A few productions defining the
 * different ranges of character are actually implanted either in
 * parserInternals.h or parserInternals.c
 * The DOM tree build is realized from the default SAX callbacks in
 * the module SAX.c.
 * The routines doing the validation checks are in valid.c and called either
 * from the SAX callbacks or as standalone functions using a preparsed
 * document.
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */

/* To avoid EBCDIC trouble when parsing on zOS */
#if defined(__MVS__)
#pragma convert("ISO8859-1")
#endif

#define IN_LIBXML
#include "libxml.h"

#if defined(_WIN32) && !defined (__CYGWIN__)
#define XML_DIR_SEP '\\'
#else
#define XML_DIR_SEP '/'
#endif

#include <stdlib.h>
#include <limits.h>
#include <string.h>
#include <stdarg.h>
#include <stddef.h>
#include <libxml/xmlmemory.h>
#include <libxml/threads.h>
#include <libxml/globals.h>
#include <libxml/tree.h>
#include <libxml/parser.h>
#include <libxml/parserInternals.h>
#include <libxml/valid.h>
#include <libxml/entities.h>
#include <libxml/xmlerror.h>
#include <libxml/encoding.h>
#include <libxml/xmlIO.h>
#include <libxml/uri.h>
#ifdef LIBXML_CATALOG_ENABLED
#include <libxml/catalog.h>
#endif
#ifdef LIBXML_SCHEMAS_ENABLED
#include <libxml/xmlschemastypes.h>
#include <libxml/relaxng.h>
#endif
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

#include "buf.h"
#include "enc.h"

struct _xmlStartTag {
    const xmlChar *prefix;
    const xmlChar *URI;
    int line;
    int nsNr;
};

/************************************************************************
 *									*
 *	Arbitrary limits set in the parser. See XML_PARSE_HUGE		*
 *									*
 ************************************************************************/

#define XML_PARSER_BIG_ENTITY 1000
#define XML_PARSER_LOT_ENTITY 5000

/*
 * XML_PARSER_NON_LINEAR is the threshold where the ratio of parsed entity
 *    replacement over the size in byte of the input indicates that you have
 *    and exponential behaviour. A value of 10 correspond to at least 3 entity
 *    replacement per byte of input.
 */
#define XML_PARSER_NON_LINEAR 10

#define SAX2 1
#define XML_PARSER_BIG_BUFFER_SIZE 300
#define XML_PARSER_BUFFER_SIZE 100
#define SAX_COMPAT_MODE BAD_CAST "SAX compatibility mode document"

/**
 * XML_PARSER_CHUNK_SIZE
 *
 * When calling GROW that's the minimal amount of data
 * the parser expected to have received. It is not a hard
 * limit but an optimization when reading strings like Names
 * It is not strictly needed as long as inputs available characters
 * are followed by 0, which should be provided by the I/O level
 */
#define XML_PARSER_CHUNK_SIZE 100

/*
 * Macros for accessing the content. Those should be used only by the parser,
 * and not exported.
 *
 * Dirty macros, i.e. one often need to make assumption on the context to
 * use them
 *
 *   CUR_PTR return the current pointer to the xmlChar to be parsed.
 *           To be used with extreme caution since operations consuming
 *           characters may move the input buffer to a different location !
 *   CUR     returns the current xmlChar value, i.e. a 8 bit value if compiled
 *           This should be used internally by the parser
 *           only to compare to ASCII values otherwise it would break when
 *           running with UTF-8 encoding.
 *   RAW     same as CUR but in the input buffer, bypass any token
 *           extraction that may have been done
 *   NXT(n)  returns the n'th next xmlChar. Same as CUR is should be used only
 *           to compare on ASCII based substring.
 *   SKIP(n) Skip n xmlChar, and must also be used only to skip ASCII defined
 *           strings without newlines within the parser.
 *   NEXT1(l) Skip 1 xmlChar, and must also be used only to skip 1 non-newline ASCII
 *           defined char within the parser.
 * Clean macros, not dependent of an ASCII context, expect UTF-8 encoding
 *
 *   NEXT    Skip to the next character, this does the proper decoding
 *           in UTF-8 mode. It also pop-up unfinished entities on the fly.
 *   NEXTL(l) Skip the current unicode character of l xmlChars long.
 *   CUR_CHAR(l) returns the current unicode character (int), set l
 *           to the number of xmlChars used for the encoding [0-5].
 *   CUR_SCHAR  same but operate on a string instead of the context
 *   COPY_BUF  copy the current unicode char to the target buffer, increment
 *            the index
 *   GROW, SHRINK  handling of input buffers
 */

#define RAW (*ctxt->input->cur)
#define CUR (*ctxt->input->cur)
#define NXT(val) ctxt->input->cur[(val)]
#define CUR_PTR ctxt->input->cur
#define BASE_PTR ctxt->input->base

#define CMP4( s, c1, c2, c3, c4 ) \
  ( ((unsigned char *) s)[ 0 ] == c1 && ((unsigned char *) s)[ 1 ] == c2 && \
    ((unsigned char *) s)[ 2 ] == c3 && ((unsigned char *) s)[ 3 ] == c4 )
#define CMP5( s, c1, c2, c3, c4, c5 ) \
  ( CMP4( s, c1, c2, c3, c4 ) && ((unsigned char *) s)[ 4 ] == c5 )
#define CMP6( s, c1, c2, c3, c4, c5, c6 ) \
  ( CMP5( s, c1, c2, c3, c4, c5 ) && ((unsigned char *) s)[ 5 ] == c6 )
#define CMP7( s, c1, c2, c3, c4, c5, c6, c7 ) \
  ( CMP6( s, c1, c2, c3, c4, c5, c6 ) && ((unsigned char *) s)[ 6 ] == c7 )
#define CMP8( s, c1, c2, c3, c4, c5, c6, c7, c8 ) \
  ( CMP7( s, c1, c2, c3, c4, c5, c6, c7 ) && ((unsigned char *) s)[ 7 ] == c8 )
#define CMP9( s, c1, c2, c3, c4, c5, c6, c7, c8, c9 ) \
  ( CMP8( s, c1, c2, c3, c4, c5, c6, c7, c8 ) && \
    ((unsigned char *) s)[ 8 ] == c9 )
#define CMP10( s, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10 ) \
  ( CMP9( s, c1, c2, c3, c4, c5, c6, c7, c8, c9 ) && \
    ((unsigned char *) s)[ 9 ] == c10 )

#define SKIP(val) do {							\
    ctxt->input->cur += (val),ctxt->input->col+=(val);			\
    if (*ctxt->input->cur == 0)						\
        xmlParserInputGrow(ctxt->input, INPUT_CHUNK);			\
  } while (0)

#define SKIPL(val) do {							\
    int skipl;								\
    for(skipl=0; skipl<val; skipl++) {					\
	if (*(ctxt->input->cur) == '\n') {				\
	ctxt->input->line++; ctxt->input->col = 1;			\
	} else ctxt->input->col++;					\
	ctxt->input->cur++;						\
    }									\
    if (*ctxt->input->cur == 0)						\
        xmlParserInputGrow(ctxt->input, INPUT_CHUNK);			\
  } while (0)

#define SHRINK if ((ctxt->progressive == 0) &&				\
		   (ctxt->input->cur - ctxt->input->base > 2 * INPUT_CHUNK) && \
		   (ctxt->input->end - ctxt->input->cur < 2 * INPUT_CHUNK)) \
	xmlSHRINK (ctxt);

static void xmlSHRINK (xmlParserCtxtPtr ctxt) {
    xmlParserInputShrink(ctxt->input);
    if (*ctxt->input->cur == 0)
        xmlParserInputGrow(ctxt->input, INPUT_CHUNK);
}

#define GROW if ((ctxt->progressive == 0) &&				\
		 (ctxt->input->end - ctxt->input->cur < INPUT_CHUNK))	\
	xmlGROW (ctxt);

//static void xmlGROW (xmlParserCtxtPtr ctxt) {
//    ptrdiff_t curEnd = ctxt->input->end - ctxt->input->cur;
//    ptrdiff_t curBase = ctxt->input->cur - ctxt->input->base;
//
//    if (((curEnd > XML_MAX_LOOKUP_LIMIT) ||
//         (curBase > XML_MAX_LOOKUP_LIMIT)) &&
//        ((ctxt->input->buf) &&
//         (ctxt->input->buf->readcallback != xmlInputReadCallbackNop)) &&
//        ((ctxt->options & XML_PARSE_HUGE) == 0)) {
//        xmlFatalErr(ctxt, XML_ERR_INTERNAL_ERROR, "Huge input lookup");
//        xmlHaltParser(ctxt);
//        return;
//    }
//    xmlParserInputGrow(ctxt->input, INPUT_CHUNK);
//    if ((ctxt->input->cur > ctxt->input->end) ||
//        (ctxt->input->cur < ctxt->input->base)) {
//        xmlHaltParser(ctxt);
//        xmlFatalErr(ctxt, XML_ERR_INTERNAL_ERROR, "cur index out of bound");
//        return;
//    }
//    if ((ctxt->input->cur != NULL) && (*ctxt->input->cur == 0))
//        xmlParserInputGrow(ctxt->input, INPUT_CHUNK);
//}

#define SKIP_BLANKS xmlSkipBlankChars(ctxt)

#define NEXT xmlNextChar(ctxt)

#define NEXT1 {								\
	ctxt->input->col++;						\
	ctxt->input->cur++;						\
	if (*ctxt->input->cur == 0)					\
	    xmlParserInputGrow(ctxt->input, INPUT_CHUNK);		\
    }

#define NEXTL(l) do {							\
    if (*(ctxt->input->cur) == '\n') {					\
	ctxt->input->line++; ctxt->input->col = 1;			\
    } else ctxt->input->col++;						\
    ctxt->input->cur += l;				\
  } while (0)

#define CUR_CHAR(l) xmlCurrentChar(ctxt, &l)
#define CUR_SCHAR(s, l) xmlStringCurrentChar(ctxt, s, &l)

#define COPY_BUF(l,b,i,v)						\
    if (l == 1) b[i++] = (xmlChar) v;					\
    else i += xmlCopyCharMultiByte(&b[i],v)


/*
 * Macro used to grow the current buffer.
 * buffer##_size is expected to be a size_t
 * mem_error: is expected to handle memory allocation failures
 */
#define growBuffer(buffer, n) {						\
    xmlChar *tmp;							\
    size_t new_size = buffer##_size * 2 + n;                            \
    if (new_size < buffer##_size) goto mem_error;                       \
    tmp = (xmlChar *) xmlRealloc(buffer, new_size);                     \
    if (tmp == NULL) goto mem_error;					\
    buffer = tmp;							\
    buffer##_size = new_size;                                           \
}

#define GROW_PARSE_ATT_VALUE_INTERNAL(ctxt, in, start, end) \
    const xmlChar *oldbase = ctxt->input->base;\
    GROW;\
    if (ctxt->instate == XML_PARSER_EOF)\
        return(NULL);\
    if (oldbase != ctxt->input->base) {\
        ptrdiff_t delta = ctxt->input->base - oldbase;\
        start = start + delta;\
        in = in + delta;\
    }\
    end = ctxt->input->end;

/**
 * DICT_FREE:
 * @str:  a string
 *
 * Free a string if it is not owned by the "dict" dictionary in the
 * current scope
 */
#define DICT_FREE(str)						\
	if ((str) && ((!dict) ||				\
	    (xmlDictOwns(dict, (const xmlChar *)(str)) == 0)))	\
	    xmlFree((char *)(str));

/************************************************************************
 *									*
 *				Miscellaneous				*
 *									*
 ************************************************************************/

#ifdef LIBXML_XPATH_ENABLED
#include <libxml/xpath.h>
#endif

typedef struct _xmlDefAttrs xmlDefAttrs;
typedef xmlDefAttrs *xmlDefAttrsPtr;
struct _xmlDefAttrs {
    int nbAttrs;	/* number of defaulted attributes on that element */
    int maxAttrs;       /* the size of the array */
#if __STDC_VERSION__ >= 199901L
    /* Using a C99 flexible array member avoids UBSan errors. */
    const xmlChar *values[]; /* array of localname/prefix/values/external */
#else
    const xmlChar *values[5];
#endif
};


#ifdef COMPILE_WITH_RUST
// region1: COMPILE_WITH_RUST宏定义的部分表示需要与rust交互，只有在连接后才生效
extern int xmlHasFeature_rust(xmlFeature feature);
extern int xmlCheckLanguageID_rust(const xmlChar *lang);
extern int inputPush_rust(xmlParserCtxtPtr ctxt, xmlParserInputPtr value);
extern xmlParserInputPtr inputPop_rust(xmlParserCtxtPtr ctxt);
extern int nodePush_rust(xmlParserCtxtPtr ctxt, xmlNodePtr value);
extern xmlNodePtr nodePop_rust(xmlParserCtxtPtr ctxt);
extern int namePush_rust(xmlParserCtxtPtr ctxt, const xmlChar *value);
extern const xmlChar * namePop_rust(xmlParserCtxtPtr ctxt);

extern int xmlSkipBlankChars_rust(xmlParserCtxtPtr ctxt);
extern xmlChar xmlPopInput_rust(xmlParserCtxtPtr ctxt);
extern int xmlPushInput_rust(xmlParserCtxtPtr ctxt, xmlParserInputPtr input);
extern int xmlParseCharRef_rust(xmlParserCtxtPtr ctxt);
extern void xmlParserHandlePEReference_rust(xmlParserCtxtPtr ctxt);
extern xmlChar * xmlStringLenDecodeEntities_rust(xmlParserCtxtPtr ctxt, const xmlChar *str, int len,
                                                 int what, xmlChar end, xmlChar end2, xmlChar end3);
extern xmlChar * xmlStringDecodeEntities_rust(xmlParserCtxtPtr ctxt, const xmlChar *str, int what,
                                              xmlChar end, xmlChar end2, xmlChar end3);
extern xmlChar * xmlSplitQName_rust(xmlParserCtxtPtr ctxt, const xmlChar *name, xmlChar **prefix);
extern const xmlChar * xmlParseName_rust(xmlParserCtxtPtr ctxt);
extern xmlChar * xmlParseNmtoken_rust(xmlParserCtxtPtr ctxt);
extern xmlChar * xmlParseEntityValue_rust(xmlParserCtxtPtr ctxt, xmlChar **orig);
extern xmlChar * xmlParseAttValue_rust(xmlParserCtxtPtr ctxt);
extern xmlChar * xmlParseSystemLiteral_rust(xmlParserCtxtPtr ctxt);
extern xmlChar * xmlParsePubidLiteral_rust(xmlParserCtxtPtr ctxt);

extern void xmlParseCharData_rust(xmlParserCtxtPtr ctxt, int cdata);
extern xmlChar * xmlParseExternalID_rust(xmlParserCtxtPtr ctxt, xmlChar **publicID, int strict);
extern void xmlParseComment_rust(xmlParserCtxtPtr ctxt);
extern const xmlChar * xmlParsePITarget_rust(xmlParserCtxtPtr ctxt);
extern void xmlParsePI_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseNotationDecl_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseEntityDecl_rust(xmlParserCtxtPtr ctxt);
extern int xmlParseDefaultDecl_rust(xmlParserCtxtPtr ctxt, xmlChar **value);
extern xmlEnumerationPtr xmlParseNotationType_rust(xmlParserCtxtPtr ctxt);
extern xmlEnumerationPtr xmlParseEnumerationType_rust(xmlParserCtxtPtr ctxt);
extern int xmlParseEnumeratedType_rust(xmlParserCtxtPtr ctxt, xmlEnumerationPtr *tree);
extern int xmlParseAttributeType_rust(xmlParserCtxtPtr ctxt, xmlEnumerationPtr *tree);
extern void xmlParseAttributeListDecl_rust(xmlParserCtxtPtr ctxt);
extern xmlElementContentPtr xmlParseElementMixedContentDecl_rust(xmlParserCtxtPtr ctxt, int inputchk);
extern xmlElementContentPtr xmlParseElementChildrenContentDecl_rust(xmlParserCtxtPtr ctxt, int inputchk);
extern int xmlParseElementContentDecl_rust(xmlParserCtxtPtr ctxt, const xmlChar *name,
                                           xmlElementContentPtr *result);
extern int xmlParseElementDecl_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseMarkupDecl_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseTextDecl_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseExternalSubset_rust(xmlParserCtxtPtr ctxt, const xmlChar *ExternalID,
                                        const xmlChar *SystemID);
extern void xmlParseReference_rust(xmlParserCtxtPtr ctxt);
extern xmlEntityPtr xmlParseEntityRef_rust(xmlParserCtxtPtr ctxt);
extern void xmlParsePEReference_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseDocTypeDecl_rust(xmlParserCtxtPtr ctxt);
//why--ok
extern const xmlChar * xmlParseAttribute_rust(xmlParserCtxtPtr ctxt, xmlChar **value);
extern const xmlChar * xmlParseStartTag_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseEndTag_rust(xmlParserCtxtPtr ctxt);
//----
extern void xmlParseCDSect_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseContent_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseElement_rust(xmlParserCtxtPtr ctxt);
extern xmlChar * xmlParseVersionNum_rust(xmlParserCtxtPtr ctxt);
extern xmlChar * xmlParseVersionInfo_rust(xmlParserCtxtPtr ctxt);
extern xmlChar * xmlParseEncName_rust(xmlParserCtxtPtr ctxt);
extern const xmlChar * xmlParseEncodingDecl_rust(xmlParserCtxtPtr ctxt);
extern int xmlParseSDDecl_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseXMLDecl_rust(xmlParserCtxtPtr ctxt);
extern void xmlParseMisc_rust(xmlParserCtxtPtr ctxt);
extern int xmlParseDocument_rust(xmlParserCtxtPtr ctxt);
extern int xmlParseExtParsedEnt_rust(xmlParserCtxtPtr ctxt);
extern int xmlParseChunk_rust(xmlParserCtxtPtr ctxt, const char *chunk, int size,
                              int terminate);
extern xmlParserCtxtPtr xmlCreatePushParserCtxt_rust(xmlSAXHandlerPtr sax, void *user_data,
                                                     const char *chunk, int size, const char *filename);
extern void xmlStopParser_rust(xmlParserCtxtPtr ctxt);
extern xmlParserCtxtPtr xmlCreateIOParserCtxt_rust(xmlSAXHandlerPtr sax, void *user_data,
                                                   xmlInputReadCallback ioread, xmlInputCloseCallback ioclose,
                                                   void *ioctx, xmlCharEncoding enc);
extern xmlDtdPtr xmlIOParseDTD_rust(xmlSAXHandlerPtr sax, xmlParserInputBufferPtr input,
                                    xmlCharEncoding enc);

extern xmlDtdPtr xmlSAXParseDTD_rust(xmlSAXHandlerPtr sax, const xmlChar *ExternalID,
                                     const xmlChar *SystemID);

extern xmlDtdPtr xmlParseDTD_rust(const xmlChar *ExternalID, const xmlChar *SystemID);
extern int xmlParseCtxtExternalEntity_rust(xmlParserCtxtPtr ctx, const xmlChar *URL,
                                           const xmlChar *ID, xmlNodePtr *lst);

extern int xmlParseExternalEntity_rust(xmlDocPtr doc, xmlSAXHandlerPtr sax, void *user_data,
                                       int depth, const xmlChar *URL, const xmlChar *ID, xmlNodePtr *lst);

extern int xmlParseBalancedChunkMemory_rust(xmlDocPtr doc, xmlSAXHandlerPtr sax,
                                            void *user_data, int depth, const xmlChar *string, xmlNodePtr *lst);
extern xmlParserErrors xmlParseInNodeContext_rust(xmlNodePtr node, const char *data, int datalen,
                                                  int options, xmlNodePtr *lst);

//why--ok
extern int xmlParseBalancedChunkMemoryRecover_rust(xmlDocPtr doc, xmlSAXHandlerPtr sax,
                                                   void *user_data, int depth, const xmlChar *string, xmlNodePtr *lst,
                                                   int recover);
extern xmlDocPtr xmlSAXParseEntity_rust(xmlSAXHandlerPtr sax, const char *filename);
extern xmlDocPtr xmlParseEntity_rust(const char *filename);
//----
extern xmlParserCtxtPtr xmlCreateEntityParserCtxt_rust(const xmlChar *URL, const xmlChar *ID,
                                                       const xmlChar *base);
extern xmlParserCtxtPtr xmlCreateURLParserCtxt_rust(const char *filename, int options);
extern xmlParserCtxtPtr xmlCreateFileParserCtxt_rust(const char *filename);
//why--ok
extern xmlDocPtr xmlSAXParseFileWithData_rust(xmlSAXHandlerPtr sax, const char *filename,
                                              int recovery, void *data);
extern xmlDocPtr xmlSAXParseFile_rust(xmlSAXHandlerPtr sax, const char *filename,
                                      int recovery);
extern xmlDocPtr xmlRecoverDoc_rust(const xmlChar *cur);
extern xmlDocPtr xmlParseFile_rust(const char *filename);
extern xmlDocPtr xmlRecoverFile_rust(const char *filename);
extern void xmlSetupParserForBuffer_rust(xmlParserCtxtPtr ctxt, const xmlChar* buffer,
                                         const char* filename);
extern int xmlSAXUserParseFile_rust(xmlSAXHandlerPtr sax, void *user_data,
                                    const char *filename);
//----
extern xmlParserCtxtPtr xmlCreateMemoryParserCtxt_rust(const char *buffer, int size);
//why--ok
extern xmlDocPtr xmlSAXParseMemoryWithData_rust(xmlSAXHandlerPtr sax, const char *buffer,
                                                int size, int recovery, void *data);
extern xmlDocPtr xmlSAXParseMemory_rust(xmlSAXHandlerPtr sax, const char *buffer,
                                        int size, int recovery);
extern xmlDocPtr xmlParseMemory_rust(const char *buffer, int size);

extern xmlDocPtr xmlRecoverMemory_rust(const char *buffer, int size);
extern int xmlSAXUserParseMemory_rust(xmlSAXHandlerPtr sax, void *user_data,
                                      const char *buffer, int size);
//----
extern xmlParserCtxtPtr xmlCreateDocParserCtxt_rust(const xmlChar *cur);
//why--ok
extern xmlDocPtr xmlSAXParseDoc_rust(xmlSAXHandlerPtr sax, const xmlChar *cur, int recovery);
extern xmlDocPtr xmlParseDoc_rust(const xmlChar *cur);
extern void xmlSetEntityReferenceFunc_rust(xmlEntityReferenceFunc func);
//----
extern void xmlInitParser_rust(void);
extern void xmlCleanupParser_rust(void);
extern void xmlCtxtReset_rust(xmlParserCtxtPtr ctxt);
extern int xmlCtxtResetPush_rust(xmlParserCtxtPtr ctxt, const char *chunk,
                                 int size, const char *filename, const char *encoding);
extern int xmlCtxtUseOptions_rust(xmlParserCtxtPtr ctxt, int options);
extern xmlDocPtr xmlReadDoc_rust(const xmlChar *cur, const char *URL, const char *encoding, int options);
extern xmlDocPtr xmlReadFile_rust(const char *filename, const char *encoding, int options);
extern xmlDocPtr xmlReadMemory_rust(const char *buffer, int size, const char *URL, const char *encoding, int options);
extern xmlDocPtr xmlReadFd_rust(int fd, const char *URL, const char *encoding, int options);
extern xmlDocPtr xmlReadIO_rust(xmlInputReadCallback ioread, xmlInputCloseCallback ioclose,
                                void *ioctx, const char *URL, const char *encoding, int options);
extern xmlDocPtr xmlCtxtReadDoc_rust(xmlParserCtxtPtr ctxt, const xmlChar *cur,
                                     const char *URL, const char *encoding, int options);
extern xmlDocPtr xmlCtxtReadFile_rust(xmlParserCtxtPtr ctxt, const char *filename,
                                      const char *encoding, int options);
extern xmlDocPtr xmlCtxtReadMemory_rust(xmlParserCtxtPtr ctxt, const char *buffer, int size,
                                        const char *URL, const char *encoding, int options);
extern xmlDocPtr xmlCtxtReadFd_rust(xmlParserCtxtPtr ctxt, int fd,
                                    const char *URL, const char *encoding, int options);
extern xmlDocPtr xmlCtxtReadIO_rust(xmlParserCtxtPtr ctxt, xmlInputReadCallback ioread,
                                    xmlInputCloseCallback ioclose, void *ioctx,
                                    const char *URL,const char *encoding, int options);

/**
  * xmlHasFeature:
  * @feature: the feature to be examined
  *
  * Examines if the library has been compiled with a given feature.
  *
  * Returns a non-zero value if the feature exist, otherwise zero.
  * Returns zero (0) if the feature does not exist or an unknown
  * unknown feature is requested, non-zero otherwise.
  */
int
xmlHasFeature(xmlFeature feature)
{
    switch (feature) {
	case XML_WITH_THREAD:
#ifdef LIBXML_THREAD_ENABLED
	    return(1);
#else
	    return(0);
#endif
        case XML_WITH_TREE:
#ifdef LIBXML_TREE_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_OUTPUT:
#ifdef LIBXML_OUTPUT_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_PUSH:
#ifdef LIBXML_PUSH_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_READER:
#ifdef LIBXML_READER_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_PATTERN:
#ifdef LIBXML_PATTERN_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_WRITER:
#ifdef LIBXML_WRITER_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_SAX1:
#ifdef LIBXML_SAX1_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_FTP:
#ifdef LIBXML_FTP_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_HTTP:
#ifdef LIBXML_HTTP_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_VALID:
#ifdef LIBXML_VALID_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_HTML:
#ifdef LIBXML_HTML_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_LEGACY:
#ifdef LIBXML_LEGACY_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_C14N:
#ifdef LIBXML_C14N_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_CATALOG:
#ifdef LIBXML_CATALOG_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_XPATH:
#ifdef LIBXML_XPATH_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_XPTR:
#ifdef LIBXML_XPTR_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_XINCLUDE:
#ifdef LIBXML_XINCLUDE_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_ICONV:
#ifdef LIBXML_ICONV_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_ISO8859X:
#ifdef LIBXML_ISO8859X_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_UNICODE:
#ifdef LIBXML_UNICODE_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_REGEXP:
#ifdef LIBXML_REGEXP_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_AUTOMATA:
#ifdef LIBXML_AUTOMATA_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_EXPR:
#ifdef LIBXML_EXPR_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_SCHEMAS:
#ifdef LIBXML_SCHEMAS_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_SCHEMATRON:
#ifdef LIBXML_SCHEMATRON_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_MODULES:
#ifdef LIBXML_MODULES_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_DEBUG:
#ifdef LIBXML_DEBUG_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_DEBUG_MEM:
#ifdef DEBUG_MEMORY_LOCATION
            return(1);
#else
            return(0);
#endif
        case XML_WITH_DEBUG_RUN:
#ifdef LIBXML_DEBUG_RUNTIME
            return(1);
#else
            return(0);
#endif
        case XML_WITH_ZLIB:
#ifdef LIBXML_ZLIB_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_LZMA:
#ifdef LIBXML_LZMA_ENABLED
            return(1);
#else
            return(0);
#endif
        case XML_WITH_ICU:
#ifdef LIBXML_ICU_ENABLED
            return(1);
#else
            return(0);
#endif
        default:
	    break;
     }
     return(0);
}

/**
 * xmlCheckLanguageID:
 * @lang:  pointer to the string value
 *
 * Checks that the value conforms to the LanguageID production:
 *
 * NOTE: this is somewhat deprecated, those productions were removed from
 *       the XML Second edition.
 *
 * [33] LanguageID ::= Langcode ('-' Subcode)*
 * [34] Langcode ::= ISO639Code |  IanaCode |  UserCode
 * [35] ISO639Code ::= ([a-z] | [A-Z]) ([a-z] | [A-Z])
 * [36] IanaCode ::= ('i' | 'I') '-' ([a-z] | [A-Z])+
 * [37] UserCode ::= ('x' | 'X') '-' ([a-z] | [A-Z])+
 * [38] Subcode ::= ([a-z] | [A-Z])+
 *
 * The current REC reference the successors of RFC 1766, currently 5646
 *
 * http://www.rfc-editor.org/rfc/rfc5646.txt
 * langtag       = language
 *                 ["-" script]
 *                 ["-" region]
 *                 *("-" variant)
 *                 *("-" extension)
 *                 ["-" privateuse]
 * language      = 2*3ALPHA            ; shortest ISO 639 code
 *                 ["-" extlang]       ; sometimes followed by
 *                                     ; extended language subtags
 *               / 4ALPHA              ; or reserved for future use
 *               / 5*8ALPHA            ; or registered language subtag
 *
 * extlang       = 3ALPHA              ; selected ISO 639 codes
 *                 *2("-" 3ALPHA)      ; permanently reserved
 *
 * script        = 4ALPHA              ; ISO 15924 code
 *
 * region        = 2ALPHA              ; ISO 3166-1 code
 *               / 3DIGIT              ; UN M.49 code
 *
 * variant       = 5*8alphanum         ; registered variants
 *               / (DIGIT 3alphanum)
 *
 * extension     = singleton 1*("-" (2*8alphanum))
 *
 *                                     ; Single alphanumerics
 *                                     ; "x" reserved for private use
 * singleton     = DIGIT               ; 0 - 9
 *               / %x41-57             ; A - W
 *               / %x59-5A             ; Y - Z
 *               / %x61-77             ; a - w
 *               / %x79-7A             ; y - z
 *
 * it sounds right to still allow Irregular i-xxx IANA and user codes too
 * The parser below doesn't try to cope with extension or privateuse
 * that could be added but that's not interoperable anyway
 *
 * Returns 1 if correct 0 otherwise
 **/
int xmlCheckLanguageID(const xmlChar * lang)
{
    return xmlCheckLanguageID_rust(lang);
}

/**
 * inputPush:
 * @ctxt:  an XML parser context
 * @value:  the parser input
 *
 * Pushes a new parser input on top of the input stack
 *
 * Returns -1 in case of error, the index in the stack otherwise
 */
int inputPush(xmlParserCtxtPtr ctxt, xmlParserInputPtr value)
{
    return inputPush_rust(ctxt, value);
}

/**
 * inputPop:
 * @ctxt: an XML parser context
 *
 * Pops the top parser input from the input stack
 *
 * Returns the input just removed
 */
xmlParserInputPtr inputPop(xmlParserCtxtPtr ctxt)
{
    return inputPop_rust(ctxt);
}

/**
 * nodePush:
 * @ctxt:  an XML parser context
 * @value:  the element node
 *
 * Pushes a new element node on top of the node stack
 *
 * Returns -1 in case of error, the index in the stack otherwise
 */
int nodePush(xmlParserCtxtPtr ctxt, xmlNodePtr value)
{
    return nodePush_rust(ctxt, value);
}

/**
 * nodePop:
 * @ctxt: an XML parser context
 *
 * Pops the top element node from the node stack
 *
 * Returns the node just removed
 */
xmlNodePtr nodePop(xmlParserCtxtPtr ctxt)
{
    return nodePop_rust(ctxt);
}

/**
 * namePush:
 * @ctxt:  an XML parser context
 * @value:  the element name
 *
 * Pushes a new element name on top of the name stack
 *
 * Returns -1 in case of error, the index in the stack otherwise
 */
int namePush(xmlParserCtxtPtr ctxt, const xmlChar * value)
{
    return namePush_rust(ctxt, value);
}

/**
 * namePop:
 * @ctxt: an XML parser context
 *
 * Pops the top element name from the name stack
 *
 * Returns the name just removed
 */
const xmlChar * namePop(xmlParserCtxtPtr ctxt)
{
    return namePop_rust(ctxt);
}

/**
 * xmlSkipBlankChars:
 * @ctxt:  the XML parser context
 *
 * skip all blanks character found at that point in the input streams.
 * It pops up finished entities in the process if allowable at that point.
 *
 * Returns the number of space chars skipped
 */
int xmlSkipBlankChars(xmlParserCtxtPtr ctxt) {
    return xmlSkipBlankChars_rust(ctxt);
}

/**
 * xmlPopInput:
 * @ctxt:  an XML parser context
 *
 * xmlPopInput: the current input pointed by ctxt->input came to an end
 *          pop it and return the next char.
 *
 * Returns the current xmlChar in the parser context
 */
xmlChar xmlPopInput(xmlParserCtxtPtr ctxt) {
    return xmlPopInput_rust(ctxt);
}
/**
 * xmlPushInput:
 * @ctxt:  an XML parser context
 * @input:  an XML parser input fragment (entity, XML fragment ...).
 *
 * xmlPushInput: switch to a new input stream which is stacked on top
 *               of the previous one(s).
 * Returns -1 in case of error or the index in the input stack
 */
int xmlPushInput(xmlParserCtxtPtr ctxt, xmlParserInputPtr input) {
    return xmlPushInput_rust(ctxt, input);
}

/**
 * xmlParseCharRef:
 * @ctxt:  an XML parser context
 *
 * parse Reference declarations
 *
 * [66] CharRef ::= '&#' [0-9]+ ';' |
 *                  '&#x' [0-9a-fA-F]+ ';'
 *
 * [ WFC: Legal Character ]
 * Characters referred to using character references must match the
 * production for Char.
 *
 * Returns the value parsed (as an int), 0 in case of error
 */
int xmlParseCharRef(xmlParserCtxtPtr ctxt) {
    return xmlParseCharRef_rust(ctxt);
}

/**
 * xmlParserHandlePEReference:
 * @ctxt:  the parser context
 *
 * [69] PEReference ::= '%' Name ';'
 *
 * [ WFC: No Recursion ]
 * A parsed entity must not contain a recursive
 * reference to itself, either directly or indirectly.
 *
 * [ WFC: Entity Declared ]
 * In a document without any DTD, a document with only an internal DTD
 * subset which contains no parameter entity references, or a document
 * with "standalone='yes'", ...  ... The declaration of a parameter
 * entity must precede any reference to it...
 *
 * [ VC: Entity Declared ]
 * In a document with an external subset or external parameter entities
 * with "standalone='no'", ...  ... The declaration of a parameter entity
 * must precede any reference to it...
 *
 * [ WFC: In DTD ]
 * Parameter-entity references may only appear in the DTD.
 * NOTE: misleading but this is handled.
 *
 * A PEReference may have been detected in the current input stream
 * the handling is done accordingly to
 *      http://www.w3.org/TR/REC-xml#entproc
 * i.e.
 *   - Included in literal in entity values
 *   - Included as Parameter Entity reference within DTDs
 */
void xmlParserHandlePEReference(xmlParserCtxtPtr ctxt) {
    xmlParserHandlePEReference_rust(ctxt);
}


/**
 * xmlStringLenDecodeEntities:
 * @ctxt:  the parser context
 * @str:  the input string
 * @len: the string length
 * @what:  combination of XML_SUBSTITUTE_REF and XML_SUBSTITUTE_PEREF
 * @end:  an end marker xmlChar, 0 if none
 * @end2:  an end marker xmlChar, 0 if none
 * @end3:  an end marker xmlChar, 0 if none
 *
 * Takes a entity string content and process to do the adequate substitutions.
 *
 * [67] Reference ::= EntityRef | CharRef
 *
 * [69] PEReference ::= '%' Name ';'
 *
 * Returns A newly allocated string with the substitution done. The caller
 *      must deallocate it !
 */
xmlChar * xmlStringLenDecodeEntities(xmlParserCtxtPtr ctxt, const xmlChar *str, int len,
                                     int what, xmlChar end, xmlChar  end2, xmlChar end3) {
    return xmlStringLenDecodeEntities_rust(ctxt, str, len, what, end, end2, end3);
}

/**
 * xmlStringDecodeEntities:
 * @ctxt:  the parser context
 * @str:  the input string
 * @what:  combination of XML_SUBSTITUTE_REF and XML_SUBSTITUTE_PEREF
 * @end:  an end marker xmlChar, 0 if none
 * @end2:  an end marker xmlChar, 0 if none
 * @end3:  an end marker xmlChar, 0 if none
 *
 * Takes a entity string content and process to do the adequate substitutions.
 *
 * [67] Reference ::= EntityRef | CharRef
 *
 * [69] PEReference ::= '%' Name ';'
 *
 * Returns A newly allocated string with the substitution done. The caller
 *      must deallocate it !
 */
xmlChar * xmlStringDecodeEntities(xmlParserCtxtPtr ctxt, const xmlChar *str, int what,
                                  xmlChar end, xmlChar  end2, xmlChar end3) {
    return xmlStringDecodeEntities_rust(ctxt, str, what, end, end2, end3);
}

/**
 * xmlSplitQName:
 * @ctxt:  an XML parser context
 * @name:  an XML parser context
 * @prefix:  a xmlChar **
 *
 * parse an UTF8 encoded XML qualified name string
 *
 * [NS 5] QName ::= (Prefix ':')? LocalPart
 *
 * [NS 6] Prefix ::= NCName
 *
 * [NS 7] LocalPart ::= NCName
 *
 * Returns the local part, and prefix is updated
 *   to get the Prefix if any.
 */
xmlChar * xmlSplitQName(xmlParserCtxtPtr ctxt, const xmlChar *name, xmlChar **prefix) {
    return xmlSplitQName_rust(ctxt, name, prefix);
}

/**
 * xmlParseName:
 * @ctxt:  an XML parser context
 *
 * parse an XML name.
 *
 * [4] NameChar ::= Letter | Digit | '.' | '-' | '_' | ':' |
 *                  CombiningChar | Extender
 *
 * [5] Name ::= (Letter | '_' | ':') (NameChar)*
 *
 * [6] Names ::= Name (#x20 Name)*
 *
 * Returns the Name parsed or NULL
 */
const xmlChar * xmlParseName(xmlParserCtxtPtr ctxt) {
    return xmlParseName_rust(ctxt);
}

/**
 * xmlParseNmtoken:
 * @ctxt:  an XML parser context
 *
 * parse an XML Nmtoken.
 *
 * [7] Nmtoken ::= (NameChar)+
 *
 * [8] Nmtokens ::= Nmtoken (#x20 Nmtoken)*
 *
 * Returns the Nmtoken parsed or NULL
 */
xmlChar * xmlParseNmtoken(xmlParserCtxtPtr ctxt) {
    return xmlParseNmtoken_rust(ctxt);
}

/**
 * xmlParseEntityValue:
 * @ctxt:  an XML parser context
 * @orig:  if non-NULL store a copy of the original entity value
 *
 * parse a value for ENTITY declarations
 *
 * [9] EntityValue ::= '"' ([^%&"] | PEReference | Reference)* '"' |
 *	               "'" ([^%&'] | PEReference | Reference)* "'"
 *
 * Returns the EntityValue parsed with reference substituted or NULL
 */
xmlChar * xmlParseEntityValue(xmlParserCtxtPtr ctxt, xmlChar **orig) {
    return xmlParseEntityValue_rust(ctxt, orig);
}

/**
 * xmlParseAttValue:
 * @ctxt:  an XML parser context
 *
 * parse a value for an attribute
 * Note: the parser won't do substitution of entities here, this
 * will be handled later in xmlStringGetNodeList
 *
 * [10] AttValue ::= '"' ([^<&"] | Reference)* '"' |
 *                   "'" ([^<&'] | Reference)* "'"
 *
 * 3.3.3 Attribute-Value Normalization:
 * Before the value of an attribute is passed to the application or
 * checked for validity, the XML processor must normalize it as follows:
 * - a character reference is processed by appending the referenced
 *   character to the attribute value
 * - an entity reference is processed by recursively processing the
 *   replacement text of the entity
 * - a whitespace character (#x20, #xD, #xA, #x9) is processed by
 *   appending #x20 to the normalized value, except that only a single
 *   #x20 is appended for a "#xD#xA" sequence that is part of an external
 *   parsed entity or the literal entity value of an internal parsed entity
 * - other characters are processed by appending them to the normalized value
 * If the declared value is not CDATA, then the XML processor must further
 * process the normalized attribute value by discarding any leading and
 * trailing space (#x20) characters, and by replacing sequences of space
 * (#x20) characters by a single space (#x20) character.
 * All attributes for which no declaration has been read should be treated
 * by a non-validating parser as if declared CDATA.
 *
 * Returns the AttValue parsed or NULL. The value has to be freed by the caller.
 */
xmlChar * xmlParseAttValue(xmlParserCtxtPtr ctxt) {
    return xmlParseAttValue_rust(ctxt);
}

/**
 * xmlParseSystemLiteral:
 * @ctxt:  an XML parser context
 *
 * parse an XML Literal
 *
 * [11] SystemLiteral ::= ('"' [^"]* '"') | ("'" [^']* "'")
 *
 * Returns the SystemLiteral parsed or NULL
 */
xmlChar * xmlParseSystemLiteral(xmlParserCtxtPtr ctxt) {
    return xmlParseSystemLiteral_rust(ctxt);
}

/**
 * xmlParsePubidLiteral:
 * @ctxt:  an XML parser context
 *
 * parse an XML public literal
 *
 * [12] PubidLiteral ::= '"' PubidChar* '"' | "'" (PubidChar - "'")* "'"
 *
 * Returns the PubidLiteral parsed or NULL.
 */
xmlChar * xmlParsePubidLiteral(xmlParserCtxtPtr ctxt) {
    return xmlParsePubidLiteral_rust(ctxt);
}

/**
 * xmlParseCharData:
 * @ctxt:  an XML parser context
 * @cdata:  int indicating whether we are within a CDATA section
 *
 * parse a CharData section.
 * if we are within a CDATA section ']]>' marks an end of section.
 *
 * The right angle bracket (>) may be represented using the string "&gt;",
 * and must, for compatibility, be escaped using "&gt;" or a character
 * reference when it appears in the string "]]>" in content, when that
 * string is not marking the end of a CDATA section.
 *
 * [14] CharData ::= [^<&]* - ([^<&]* ']]>' [^<&]*)
 */
void xmlParseCharData(xmlParserCtxtPtr ctxt, int cdata) {
    xmlParseCharData_rust(ctxt, cdata);
}

/**
 * xmlParseExternalID:
 * @ctxt:  an XML parser context
 * @publicID:  a xmlChar** receiving PubidLiteral
 * @strict: indicate whether we should restrict parsing to only
 *          production [75], see NOTE below
 *
 * Parse an External ID or a Public ID
 *
 * NOTE: Productions [75] and [83] interact badly since [75] can generate
 *       'PUBLIC' S PubidLiteral S SystemLiteral
 *
 * [75] ExternalID ::= 'SYSTEM' S SystemLiteral
 *                   | 'PUBLIC' S PubidLiteral S SystemLiteral
 *
 * [83] PublicID ::= 'PUBLIC' S PubidLiteral
 *
 * Returns the function returns SystemLiteral and in the second
 *                case publicID receives PubidLiteral, is strict is off
 *                it is possible to return NULL and have publicID set.
 */
xmlChar * xmlParseExternalID(xmlParserCtxtPtr ctxt, xmlChar **publicID, int strict) {
    return xmlParseExternalID_rust(ctxt, publicID, strict);
}

/**
 * xmlParseComment:
 * @ctxt:  an XML parser context
 *
 * Skip an XML (SGML) comment <!-- .... -->
 *  The spec says that "For compatibility, the string "--" (double-hyphen)
 *  must not occur within comments. "
 *
 * [15] Comment ::= '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
 */
void xmlParseComment(xmlParserCtxtPtr ctxt) {
    return xmlParseComment_rust(ctxt);
}

/**
 * xmlParsePITarget:
 * @ctxt:  an XML parser context
 *
 * parse the name of a PI
 *
 * [17] PITarget ::= Name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))
 *
 * Returns the PITarget name or NULL
 */
const xmlChar * xmlParsePITarget(xmlParserCtxtPtr ctxt) {
    return xmlParsePITarget_rust(ctxt);
}

/**
 * xmlParsePI:
 * @ctxt:  an XML parser context
 *
 * parse an XML Processing Instruction.
 *
 * [16] PI ::= '<?' PITarget (S (Char* - (Char* '?>' Char*)))? '?>'
 *
 * The processing is transferred to SAX once parsed.
 */
void xmlParsePI(xmlParserCtxtPtr ctxt) {
    xmlParsePI_rust(ctxt);
}

/**
 * xmlParseNotationDecl:
 * @ctxt:  an XML parser context
 *
 * parse a notation declaration
 *
 * [82] NotationDecl ::= '<!NOTATION' S Name S (ExternalID |  PublicID) S? '>'
 *
 * Hence there is actually 3 choices:
 *     'PUBLIC' S PubidLiteral
 *     'PUBLIC' S PubidLiteral S SystemLiteral
 * and 'SYSTEM' S SystemLiteral
 *
 * See the NOTE on xmlParseExternalID().
 */
void xmlParseNotationDecl(xmlParserCtxtPtr ctxt) {
    xmlParseNotationDecl_rust(ctxt);
}

/**
 * xmlParseEntityDecl:
 * @ctxt:  an XML parser context
 *
 * parse <!ENTITY declarations
 *
 * [70] EntityDecl ::= GEDecl | PEDecl
 *
 * [71] GEDecl ::= '<!ENTITY' S Name S EntityDef S? '>'
 *
 * [72] PEDecl ::= '<!ENTITY' S '%' S Name S PEDef S? '>'
 *
 * [73] EntityDef ::= EntityValue | (ExternalID NDataDecl?)
 *
 * [74] PEDef ::= EntityValue | ExternalID
 *
 * [76] NDataDecl ::= S 'NDATA' S Name
 *
 * [ VC: Notation Declared ]
 * The Name must match the declared name of a notation.
 */
void xmlParseEntityDecl(xmlParserCtxtPtr ctxt) {
    xmlParseEntityDecl_rust(ctxt);
}

/**
 * xmlParseDefaultDecl:
 * @ctxt:  an XML parser context
 * @value:  Receive a possible fixed default value for the attribute
 *
 * Parse an attribute default declaration
 *
 * [60] DefaultDecl ::= '#REQUIRED' | '#IMPLIED' | (('#FIXED' S)? AttValue)
 *
 * [ VC: Required Attribute ]
 * if the default declaration is the keyword #REQUIRED, then the
 * attribute must be specified for all elements of the type in the
 * attribute-list declaration.
 *
 * [ VC: Attribute Default Legal ]
 * The declared default value must meet the lexical constraints of
 * the declared attribute type c.f. xmlValidateAttributeDecl()
 *
 * [ VC: Fixed Attribute Default ]
 * if an attribute has a default value declared with the #FIXED
 * keyword, instances of that attribute must match the default value.
 *
 * [ WFC: No < in Attribute Values ]
 * handled in xmlParseAttValue()
 *
 * returns: XML_ATTRIBUTE_NONE, XML_ATTRIBUTE_REQUIRED, XML_ATTRIBUTE_IMPLIED
 *          or XML_ATTRIBUTE_FIXED.
 */
int xmlParseDefaultDecl(xmlParserCtxtPtr ctxt, xmlChar **value) {
    return xmlParseDefaultDecl_rust(ctxt, value);
}

/**
 * xmlParseNotationType:
 * @ctxt:  an XML parser context
 *
 * parse an Notation attribute type.
 *
 * Note: the leading 'NOTATION' S part has already being parsed...
 *
 * [58] NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
 *
 * [ VC: Notation Attributes ]
 * Values of this type must match one of the notation names included
 * in the declaration; all notation names in the declaration must be declared.
 *
 * Returns: the notation attribute tree built while parsing
 */
xmlEnumerationPtr xmlParseNotationType(xmlParserCtxtPtr ctxt) {
    return xmlParseNotationType_rust(ctxt);
}

/**
 * xmlParseEnumerationType:
 * @ctxt:  an XML parser context
 *
 * parse an Enumeration attribute type.
 *
 * [59] Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
 *
 * [ VC: Enumeration ]
 * Values of this type must match one of the Nmtoken tokens in
 * the declaration
 *
 * Returns: the enumeration attribute tree built while parsing
 */
xmlEnumerationPtr xmlParseEnumerationType(xmlParserCtxtPtr ctxt) {
    return xmlParseEnumerationType_rust(ctxt);
}

/**
 * xmlParseEnumeratedType:
 * @ctxt:  an XML parser context
 * @tree:  the enumeration tree built while parsing
 *
 * parse an Enumerated attribute type.
 *
 * [57] EnumeratedType ::= NotationType | Enumeration
 *
 * [58] NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
 *
 *
 * Returns: XML_ATTRIBUTE_ENUMERATION or XML_ATTRIBUTE_NOTATION
 */
int xmlParseEnumeratedType(xmlParserCtxtPtr ctxt, xmlEnumerationPtr *tree) {
    return xmlParseEnumeratedType_rust(ctxt, tree);
}

/**
 * xmlParseAttributeType:
 * @ctxt:  an XML parser context
 * @tree:  the enumeration tree built while parsing
 *
 * parse the Attribute list def for an element
 *
 * [54] AttType ::= StringType | TokenizedType | EnumeratedType
 *
 * [55] StringType ::= 'CDATA'
 *
 * [56] TokenizedType ::= 'ID' | 'IDREF' | 'IDREFS' | 'ENTITY' |
 *                        'ENTITIES' | 'NMTOKEN' | 'NMTOKENS'
 *
 * Validity constraints for attribute values syntax are checked in
 * xmlValidateAttributeValue()
 *
 * [ VC: ID ]
 * Values of type ID must match the Name production. A name must not
 * appear more than once in an XML document as a value of this type;
 * i.e., ID values must uniquely identify the elements which bear them.
 *
 * [ VC: One ID per Element Type ]
 * No element type may have more than one ID attribute specified.
 *
 * [ VC: ID Attribute Default ]
 * An ID attribute must have a declared default of #IMPLIED or #REQUIRED.
 *
 * [ VC: IDREF ]
 * Values of type IDREF must match the Name production, and values
 * of type IDREFS must match Names; each IDREF Name must match the value
 * of an ID attribute on some element in the XML document; i.e. IDREF
 * values must match the value of some ID attribute.
 *
 * [ VC: Entity Name ]
 * Values of type ENTITY must match the Name production, values
 * of type ENTITIES must match Names; each Entity Name must match the
 * name of an unparsed entity declared in the DTD.
 *
 * [ VC: Name Token ]
 * Values of type NMTOKEN must match the Nmtoken production; values
 * of type NMTOKENS must match Nmtokens.
 *
 * Returns the attribute type
 */
int xmlParseAttributeType(xmlParserCtxtPtr ctxt, xmlEnumerationPtr *tree) {
    return xmlParseAttributeType_rust(ctxt, tree);
}

/**
 * xmlParseAttributeListDecl:
 * @ctxt:  an XML parser context
 *
 * : parse the Attribute list def for an element
 *
 * [52] AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
 *
 * [53] AttDef ::= S Name S AttType S DefaultDecl
 *
 */
void xmlParseAttributeListDecl(xmlParserCtxtPtr ctxt) {
    return xmlParseAttributeListDecl_rust(ctxt);
}

/**
 * xmlParseElementMixedContentDecl:
 * @ctxt:  an XML parser context
 * @inputchk:  the input used for the current entity, needed for boundary checks
 *
 * parse the declaration for a Mixed Element content
 * The leading '(' and spaces have been skipped in xmlParseElementContentDecl
 *
 * [51] Mixed ::= '(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' |
 *                '(' S? '#PCDATA' S? ')'
 *
 * [ VC: Proper Group/PE Nesting ] applies to [51] too (see [49])
 *
 * [ VC: No Duplicate Types ]
 * The same name must not appear more than once in a single
 * mixed-content declaration.
 *
 * returns: the list of the xmlElementContentPtr describing the element choices
 */
xmlElementContentPtr xmlParseElementMixedContentDecl(xmlParserCtxtPtr ctxt, int inputchk) {
    return xmlParseElementMixedContentDecl_rust(ctxt, inputchk);
}

/**
 * xmlParseElementChildrenContentDecl:
 * @ctxt:  an XML parser context
 * @inputchk:  the input used for the current entity, needed for boundary checks
 *
 * parse the declaration for a Mixed Element content
 * The leading '(' and spaces have been skipped in xmlParseElementContentDecl
 *
 * [47] children ::= (choice | seq) ('?' | '*' | '+')?
 *
 * [48] cp ::= (Name | choice | seq) ('?' | '*' | '+')?
 *
 * [49] choice ::= '(' S? cp ( S? '|' S? cp )* S? ')'
 *
 * [50] seq ::= '(' S? cp ( S? ',' S? cp )* S? ')'
 *
 * [ VC: Proper Group/PE Nesting ] applies to [49] and [50]
 * TODO Parameter-entity replacement text must be properly nested
 *	with parenthesized groups. That is to say, if either of the
 *	opening or closing parentheses in a choice, seq, or Mixed
 *	construct is contained in the replacement text for a parameter
 *	entity, both must be contained in the same replacement text. For
 *	interoperability, if a parameter-entity reference appears in a
 *	choice, seq, or Mixed construct, its replacement text should not
 *	be empty, and neither the first nor last non-blank character of
 *	the replacement text should be a connector (| or ,).
 *
 * Returns the tree of xmlElementContentPtr describing the element
 *          hierarchy.
 */
xmlElementContentPtr xmlParseElementChildrenContentDecl(xmlParserCtxtPtr ctxt, int inputchk) {
    /* stub left for API/ABI compat */
    return xmlParseElementChildrenContentDecl_rust(ctxt, inputchk);
}

/**
 * xmlParseElementContentDecl:
 * @ctxt:  an XML parser context
 * @name:  the name of the element being defined.
 * @result:  the Element Content pointer will be stored here if any
 *
 * parse the declaration for an Element content either Mixed or Children,
 * the cases EMPTY and ANY are handled directly in xmlParseElementDecl
 *
 * [46] contentspec ::= 'EMPTY' | 'ANY' | Mixed | children
 *
 * returns: the type of element content XML_ELEMENT_TYPE_xxx
 */
int xmlParseElementContentDecl(xmlParserCtxtPtr ctxt, const xmlChar *name, xmlElementContentPtr *result) {
    return xmlParseElementContentDecl_rust(ctxt, name, result);
}

/**
 * xmlParseElementDecl:
 * @ctxt:  an XML parser context
 *
 * parse an Element declaration.
 *
 * [45] elementdecl ::= '<!ELEMENT' S Name S contentspec S? '>'
 *
 * [ VC: Unique Element Type Declaration ]
 * No element type may be declared more than once
 *
 * Returns the type of the element, or -1 in case of error
 */
int xmlParseElementDecl(xmlParserCtxtPtr ctxt) {
    return xmlParseElementDecl_rust(ctxt);
}

/**
 * xmlParseMarkupDecl:
 * @ctxt:  an XML parser context
 *
 * parse Markup declarations
 *
 * [29] markupdecl ::= elementdecl | AttlistDecl | EntityDecl |
 *                     NotationDecl | PI | Comment
 *
 * [ VC: Proper Declaration/PE Nesting ]
 * Parameter-entity replacement text must be properly nested with
 * markup declarations. That is to say, if either the first character
 * or the last character of a markup declaration (markupdecl above) is
 * contained in the replacement text for a parameter-entity reference,
 * both must be contained in the same replacement text.
 *
 * [ WFC: PEs in Internal Subset ]
 * In the internal DTD subset, parameter-entity references can occur
 * only where markup declarations can occur, not within markup declarations.
 * (This does not apply to references that occur in external parameter
 * entities or to the external subset.)
 */
void xmlParseMarkupDecl(xmlParserCtxtPtr ctxt) {
    xmlParseMarkupDecl_rust(ctxt);
}

/**
 * xmlParseTextDecl:
 * @ctxt:  an XML parser context
 *
 * parse an XML declaration header for external entities
 *
 * [77] TextDecl ::= '<?xml' VersionInfo? EncodingDecl S? '?>'
 */
void xmlParseTextDecl(xmlParserCtxtPtr ctxt) {
    xmlParseTextDecl_rust(ctxt);
}

/**
 * xmlParseExternalSubset:
 * @ctxt:  an XML parser context
 * @ExternalID: the external identifier
 * @SystemID: the system identifier (or URL)
 *
 * parse Markup declarations from an external subset
 *
 * [30] extSubset ::= textDecl? extSubsetDecl
 *
 * [31] extSubsetDecl ::= (markupdecl | conditionalSect | PEReference | S) *
 */
void xmlParseExternalSubset(xmlParserCtxtPtr ctxt, const xmlChar *ExternalID, const xmlChar *SystemID) {

    xmlParseExternalSubset_rust(ctxt, ExternalID, SystemID);

}

/**
 * xmlParseReference:
 * @ctxt:  an XML parser context
 *
 * parse and handle entity references in content, depending on the SAX
 * interface, this may end-up in a call to character() if this is a
 * CharRef, a predefined entity, if there is no reference() callback.
 * or if the parser was asked to switch to that mode.
 *
 * [67] Reference ::= EntityRef | CharRef
 */
void xmlParseReference(xmlParserCtxtPtr ctxt) {
    xmlParseReference_rust(ctxt);
}

/**
 * xmlParseEntityRef:
 * @ctxt:  an XML parser context
 *
 * parse ENTITY references declarations
 *
 * [68] EntityRef ::= '&' Name ';'
 *
 * [ WFC: Entity Declared ]
 * In a document without any DTD, a document with only an internal DTD
 * subset which contains no parameter entity references, or a document
 * with "standalone='yes'", the Name given in the entity reference
 * must match that in an entity declaration, except that well-formed
 * documents need not declare any of the following entities: amp, lt,
 * gt, apos, quot.  The declaration of a parameter entity must precede
 * any reference to it.  Similarly, the declaration of a general entity
 * must precede any reference to it which appears in a default value in an
 * attribute-list declaration. Note that if entities are declared in the
 * external subset or in external parameter entities, a non-validating
 * processor is not obligated to read and process their declarations;
 * for such documents, the rule that an entity must be declared is a
 * well-formedness constraint only if standalone='yes'.
 *
 * [ WFC: Parsed Entity ]
 * An entity reference must not contain the name of an unparsed entity
 *
 * Returns the xmlEntityPtr if found, or NULL otherwise.
 */
xmlEntityPtr xmlParseEntityRef(xmlParserCtxtPtr ctxt) {
    return xmlParseEntityRef_rust(ctxt);
}

/**
 * xmlParsePEReference:
 * @ctxt:  an XML parser context
 *
 * parse PEReference declarations
 * The entity content is handled directly by pushing it's content as
 * a new input stream.
 *
 * [69] PEReference ::= '%' Name ';'
 *
 * [ WFC: No Recursion ]
 * A parsed entity must not contain a recursive
 * reference to itself, either directly or indirectly.
 *
 * [ WFC: Entity Declared ]
 * In a document without any DTD, a document with only an internal DTD
 * subset which contains no parameter entity references, or a document
 * with "standalone='yes'", ...  ... The declaration of a parameter
 * entity must precede any reference to it...
 *
 * [ VC: Entity Declared ]
 * In a document with an external subset or external parameter entities
 * with "standalone='no'", ...  ... The declaration of a parameter entity
 * must precede any reference to it...
 *
 * [ WFC: In DTD ]
 * Parameter-entity references may only appear in the DTD.
 * NOTE: misleading but this is handled.
 */
void xmlParsePEReference(xmlParserCtxtPtr ctxt)
{
    xmlParsePEReference_rust(ctxt);
}

/**
 * xmlParseDocTypeDecl:
 * @ctxt:  an XML parser context
 *
 * parse a DOCTYPE declaration
 *
 * [28] doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S?
 *                      ('[' (markupdecl | PEReference | S)* ']' S?)? '>'
 *
 * [ VC: Root Element Type ]
 * The Name in the document type declaration must match the element
 * type of the root element.
 */
void xmlParseDocTypeDecl(xmlParserCtxtPtr ctxt) {
    xmlParseDocTypeDecl_rust(ctxt);
}

#ifdef LIBXML_SAX1_ENABLED
/**
 * xmlParseAttribute:
 * @ctxt:  an XML parser context
 * @value:  a xmlChar ** used to store the value of the attribute
 *
 * parse an attribute
 *
 * [41] Attribute ::= Name Eq AttValue
 *
 * [ WFC: No External Entity References ]
 * Attribute values cannot contain direct or indirect entity references
 * to external entities.
 *
 * [ WFC: No < in Attribute Values ]
 * The replacement text of any entity referred to directly or indirectly in
 * an attribute value (other than "&lt;") must not contain a <.
 *
 * [ VC: Attribute Value Type ]
 * The attribute must have been declared; the value must be of the type
 * declared for it.
 *
 * [25] Eq ::= S? '=' S?
 *
 * With namespace:
 *
 * [NS 11] Attribute ::= QName Eq AttValue
 *
 * Also the case QName == xmlns:??? is handled independently as a namespace
 * definition.
 *
 * Returns the attribute name, and the value in *value.
 */

const xmlChar *
xmlParseAttribute(xmlParserCtxtPtr ctxt, xmlChar **value) {
    return xmlParseAttribute_rust(ctxt, value);
}

/**
 * xmlParseStartTag:
 * @ctxt:  an XML parser context
 *
 * parse a start of tag either for rule element or
 * EmptyElement. In both case we don't parse the tag closing chars.
 *
 * [40] STag ::= '<' Name (S Attribute)* S? '>'
 *
 * [ WFC: Unique Att Spec ]
 * No attribute name may appear more than once in the same start-tag or
 * empty-element tag.
 *
 * [44] EmptyElemTag ::= '<' Name (S Attribute)* S? '/>'
 *
 * [ WFC: Unique Att Spec ]
 * No attribute name may appear more than once in the same start-tag or
 * empty-element tag.
 *
 * With namespace:
 *
 * [NS 8] STag ::= '<' QName (S Attribute)* S? '>'
 *
 * [NS 10] EmptyElement ::= '<' QName (S Attribute)* S? '/>'
 *
 * Returns the element name parsed
 */

const xmlChar *
xmlParseStartTag(xmlParserCtxtPtr ctxt) {
    return xmlParseStartTag_rust(ctxt);
}

/**
 * xmlParseEndTag:
 * @ctxt:  an XML parser context
 *
 * parse an end of tag
 *
 * [42] ETag ::= '</' Name S? '>'
 *
 * With namespace
 *
 * [NS 9] ETag ::= '</' QName S? '>'
 */

void xmlParseEndTag(xmlParserCtxtPtr ctxt) {
    xmlParseEndTag_rust(ctxt);
}
#endif /* LIBXML_SAX1_ENABLED */

/**
 * xmlParseCDSect:
 * @ctxt:  an XML parser context
 *
 * Parse escaped pure raw content.
 *
 * [18] CDSect ::= CDStart CData CDEnd
 *
 * [19] CDStart ::= '<![CDATA['
 *
 * [20] Data ::= (Char* - (Char* ']]>' Char*))
 *
 * [21] CDEnd ::= ']]>'
 */
void xmlParseCDSect(xmlParserCtxtPtr ctxt) {
    xmlParseCDSect_rust(ctxt);
}

/**
 * xmlParseContent:
 * @ctxt:  an XML parser context
 *
 * Parse a content sequence. Stops at EOF or '</'.
 *
 * [43] content ::= (element | CharData | Reference | CDSect | PI | Comment)*
 */

void xmlParseContent(xmlParserCtxtPtr ctxt) {
    xmlParseContent_rust(ctxt);
}
/**
 * xmlParseElement:
 * @ctxt:  an XML parser context
 *
 * parse an XML element
 *
 * [39] element ::= EmptyElemTag | STag content ETag
 *
 * [ WFC: Element Type Match ]
 * The Name in an element's end-tag must match the element type in the
 * start-tag.
 *
 */

void xmlParseElement(xmlParserCtxtPtr ctxt) {
    xmlParseElement_rust(ctxt);
}

/**
 * xmlParseVersionNum:
 * @ctxt:  an XML parser context
 *
 * parse the XML version value.
 *
 * [26] VersionNum ::= '1.' [0-9]+
 *
 * In practice allow [0-9].[0-9]+ at that level
 *
 * Returns the string giving the XML version number, or NULL
 */
xmlChar * xmlParseVersionNum(xmlParserCtxtPtr ctxt) {
    return xmlParseVersionNum_rust(ctxt);
}

/**
 * xmlParseVersionInfo:
 * @ctxt:  an XML parser context
 *
 * parse the XML version.
 *
 * [24] VersionInfo ::= S 'version' Eq (' VersionNum ' | " VersionNum ")
 *
 * [25] Eq ::= S? '=' S?
 *
 * Returns the version string, e.g. "1.0"
 */

xmlChar * xmlParseVersionInfo(xmlParserCtxtPtr ctxt) {
    return xmlParseVersionInfo_rust(ctxt);
}

/**
 * xmlParseEncName:
 * @ctxt:  an XML parser context
 *
 * parse the XML encoding name
 *
 * [81] EncName ::= [A-Za-z] ([A-Za-z0-9._] | '-')*
 *
 * Returns the encoding name value or NULL
 */
xmlChar * xmlParseEncName(xmlParserCtxtPtr ctxt) {
    return xmlParseEncName_rust(ctxt);
}

/**
 * xmlParseEncodingDecl:
 * @ctxt:  an XML parser context
 *
 * parse the XML encoding declaration
 *
 * [80] EncodingDecl ::= S 'encoding' Eq ('"' EncName '"' |  "'" EncName "'")
 *
 * this setups the conversion filters.
 *
 * Returns the encoding value or NULL
 */
const xmlChar * xmlParseEncodingDecl(xmlParserCtxtPtr ctxt) {
    return xmlParseEncodingDecl_rust(ctxt);
}

/**
 * xmlParseSDDecl:
 * @ctxt:  an XML parser context
 *
 * parse the XML standalone declaration
 *
 * [32] SDDecl ::= S 'standalone' Eq
 *                 (("'" ('yes' | 'no') "'") | ('"' ('yes' | 'no')'"'))
 *
 * [ VC: Standalone Document Declaration ]
 * TODO The standalone document declaration must have the value "no"
 * if any external markup declarations contain declarations of:
 *  - attributes with default values, if elements to which these
 *    attributes apply appear in the document without specifications
 *    of values for these attributes, or
 *  - entities (other than amp, lt, gt, apos, quot), if references
 *    to those entities appear in the document, or
 *  - attributes with values subject to normalization, where the
 *    attribute appears in the document with a value which will change
 *    as a result of normalization, or
 *  - element types with element content, if white space occurs directly
 *    within any instance of those types.
 *
 * Returns:
 *   1 if standalone="yes"
 *   0 if standalone="no"
 *  -2 if standalone attribute is missing or invalid
 *	  (A standalone value of -2 means that the XML declaration was found,
 *	   but no value was specified for the standalone attribute).
 */
int xmlParseSDDecl(xmlParserCtxtPtr ctxt) {
    return xmlParseSDDecl_rust(ctxt);
}

/**
 * xmlParseXMLDecl:
 * @ctxt:  an XML parser context
 *
 * parse an XML declaration header
 *
 * [23] XMLDecl ::= '<?xml' VersionInfo EncodingDecl? SDDecl? S? '?>'
 */
void xmlParseXMLDecl(xmlParserCtxtPtr ctxt) {
    xmlParseXMLDecl_rust(ctxt);
}

/**
 * xmlParseMisc:
 * @ctxt:  an XML parser context
 *
 * parse an XML Misc* optional field.
 *
 * [27] Misc ::= Comment | PI |  S
 */
void xmlParseMisc(xmlParserCtxtPtr ctxt) {
    xmlParseMisc_rust(ctxt);
}

/**
 * xmlParseDocument:
 * @ctxt:  an XML parser context
 *
 * parse an XML document (and build a tree if using the standard SAX
 * interface).
 *
 * [1] document ::= prolog element Misc*
 *
 * [22] prolog ::= XMLDecl? Misc* (doctypedecl Misc*)?
 *
 * Returns 0, -1 in case of error. the parser context is augmented
 *                as a result of the parsing.
 */

int xmlParseDocument(xmlParserCtxtPtr ctxt) {
    return xmlParseDocument_rust(ctxt);
}
/**
 * xmlParseExtParsedEnt:
 * @ctxt:  an XML parser context
 *
 * parse a general parsed entity
 * An external general parsed entity is well-formed if it matches the
 * production labeled extParsedEnt.
 *
 * [78] extParsedEnt ::= TextDecl? content
 *
 * Returns 0, -1 in case of error. the parser context is augmented
 *                as a result of the parsing.
 */

int xmlParseExtParsedEnt(xmlParserCtxtPtr ctxt) {
    return xmlParseExtParsedEnt_rust(ctxt);
}

#ifdef LIBXML_PUSH_ENABLED
/**
 * xmlParseChunk:
 * @ctxt:  an XML parser context
 * @chunk:  an char array
 * @size:  the size in byte of the chunk
 * @terminate:  last chunk indicator
 *
 * Parse a Chunk of memory
 *
 * Returns zero if no error, the xmlParserErrors otherwise.
 */

int xmlParseChunk(xmlParserCtxtPtr ctxt, const char *chunk, int size,
              int terminate) {
    return xmlParseChunk_rust(ctxt, chunk, size, terminate);
}

/************************************************************************
 *									*
 *		I/O front end functions to the parser			*
 *									*
 ************************************************************************/

/**
 * xmlCreatePushParserCtxt:
 * @sax:  a SAX handler
 * @user_data:  The user data returned on SAX callbacks
 * @chunk:  a pointer to an array of chars
 * @size:  number of chars in the array
 * @filename:  an optional file name or URI
 *
 * Create a parser context for using the XML parser in push mode.
 * If @buffer and @size are non-NULL, the data is used to detect
 * the encoding.  The remaining characters will be parsed so they
 * don't need to be fed in again through xmlParseChunk.
 * To allow content encoding detection, @size should be >= 4
 * The value of @filename is used for fetching external entities
 * and error/warning reports.
 *
 * Returns the new parser context or NULL
 */
xmlParserCtxtPtr xmlCreatePushParserCtxt(xmlSAXHandlerPtr sax, void *user_data,
                        const char *chunk, int size, const char *filename) {
    return xmlCreatePushParserCtxt_rust(sax, user_data,chunk, size,filename);
}
#endif /* LIBXML_PUSH_ENABLED */

/**
 * xmlStopParser:
 * @ctxt:  an XML parser context
 *
 * Blocks further parser processing
 */
void xmlStopParser(xmlParserCtxtPtr ctxt) {
    xmlStopParser_rust(ctxt);
}

/**
 * xmlCreateIOParserCtxt:
 * @sax:  a SAX handler
 * @user_data:  The user data returned on SAX callbacks
 * @ioread:  an I/O read function
 * @ioclose:  an I/O close function
 * @ioctx:  an I/O handler
 * @enc:  the charset encoding if known
 *
 * Create a parser context for using the XML parser with an existing
 * I/O stream
 *
 * Returns the new parser context or NULL
 */
xmlParserCtxtPtr xmlCreateIOParserCtxt(xmlSAXHandlerPtr sax, void *user_data,
                                       xmlInputReadCallback   ioread, xmlInputCloseCallback  ioclose,
                                       void *ioctx, xmlCharEncoding enc) {

    return xmlCreateIOParserCtxt_rust(sax, user_data, ioread, ioclose, ioctx, enc);
}

#ifdef LIBXML_VALID_ENABLED
/************************************************************************
 *									*
 *		Front ends when parsing a DTD				*
 *									*
 ************************************************************************/

/**
 * xmlIOParseDTD:
 * @sax:  the SAX handler block or NULL
 * @input:  an Input Buffer
 * @enc:  the charset encoding if known
 *
 * Load and parse a DTD
 *
 * Returns the resulting xmlDtdPtr or NULL in case of error.
 * @input will be freed by the function in any case.
 */

xmlDtdPtr xmlIOParseDTD(xmlSAXHandlerPtr sax, xmlParserInputBufferPtr input,
	      xmlCharEncoding enc) {
    return xmlIOParseDTD_rust(sax, input,enc);
}

/**
 * xmlSAXParseDTD:
 * @sax:  the SAX handler block
 * @ExternalID:  a NAME* containing the External ID of the DTD
 * @SystemID:  a NAME* containing the URL to the DTD
 *
 * Load and parse an external subset.
 *
 * Returns the resulting xmlDtdPtr or NULL in case of error.
 */

xmlDtdPtr xmlSAXParseDTD(xmlSAXHandlerPtr sax, const xmlChar *ExternalID,
                          const xmlChar *SystemID) {
    return xmlSAXParseDTD_rust(sax, ExternalID,SystemID);
}



/**
 * xmlParseDTD:
 * @ExternalID:  a NAME* containing the External ID of the DTD
 * @SystemID:  a NAME* containing the URL to the DTD
 *
 * Load and parse an external subset.
 *
 * Returns the resulting xmlDtdPtr or NULL in case of error.
 */

xmlDtdPtr xmlParseDTD(const xmlChar *ExternalID, const xmlChar *SystemID) {
    return xmlParseDTD_rust(ExternalID, SystemID);
}
#endif /* LIBXML_VALID_ENABLED */

/************************************************************************
 *									*
 *		Front ends when parsing an Entity			*
 *									*
 ************************************************************************/

/**
 * xmlParseCtxtExternalEntity:
 * @ctx:  the existing parsing context
 * @URL:  the URL for the entity to load
 * @ID:  the System ID for the entity to load
 * @lst:  the return value for the set of parsed nodes
 *
 * Parse an external general entity within an existing parsing context
 * An external general parsed entity is well-formed if it matches the
 * production labeled extParsedEnt.
 *
 * [78] extParsedEnt ::= TextDecl? content
 *
 * Returns 0 if the entity is well formed, -1 in case of args problem and
 *    the parser error code otherwise
 */

int xmlParseCtxtExternalEntity(xmlParserCtxtPtr ctx, const xmlChar *URL,
                               const xmlChar *ID, xmlNodePtr *lst) {
    return xmlParseCtxtExternalEntity_rust(ctx, URL, ID, lst);
}

#ifdef LIBXML_SAX1_ENABLED
/**
 * xmlParseExternalEntity:
 * @doc:  the document the chunk pertains to
 * @sax:  the SAX handler block (possibly NULL)
 * @user_data:  The user data returned on SAX callbacks (possibly NULL)
 * @depth:  Used for loop detection, use 0
 * @URL:  the URL for the entity to load
 * @ID:  the System ID for the entity to load
 * @lst:  the return value for the set of parsed nodes
 *
 * Parse an external general entity
 * An external general parsed entity is well-formed if it matches the
 * production labeled extParsedEnt.
 *
 * [78] extParsedEnt ::= TextDecl? content
 *
 * Returns 0 if the entity is well formed, -1 in case of args problem and
 *    the parser error code otherwise
 */

int xmlParseExternalEntity(xmlDocPtr doc, xmlSAXHandlerPtr sax, void *user_data,
	  int depth, const xmlChar *URL, const xmlChar *ID, xmlNodePtr *lst) {
    return xmlParseExternalEntity_rust(doc, sax, user_data,depth, URL, ID, lst);
}

/**
 * xmlParseBalancedChunkMemory:
 * @doc:  the document the chunk pertains to (must not be NULL)
 * @sax:  the SAX handler block (possibly NULL)
 * @user_data:  The user data returned on SAX callbacks (possibly NULL)
 * @depth:  Used for loop detection, use 0
 * @string:  the input string in UTF8 or ISO-Latin (zero terminated)
 * @lst:  the return value for the set of parsed nodes
 *
 * Parse a well-balanced chunk of an XML document
 * called by the parser
 * The allowed sequence for the Well Balanced Chunk is the one defined by
 * the content production in the XML grammar:
 *
 * [43] content ::= (element | CharData | Reference | CDSect | PI | Comment)*
 *
 * Returns 0 if the chunk is well balanced, -1 in case of args problem and
 *    the parser error code otherwise
 */

int xmlParseBalancedChunkMemory(xmlDocPtr doc, xmlSAXHandlerPtr sax,
     void *user_data, int depth, const xmlChar *string, xmlNodePtr *lst) {

    return xmlParseBalancedChunkMemory_rust(doc, sax, user_data, depth, string, lst);
}
#endif /* LIBXML_SAX1_ENABLED */

/**
 * xmlParseInNodeContext:
 * @node:  the context node
 * @data:  the input string
 * @datalen:  the input string length in bytes
 * @options:  a combination of xmlParserOption
 * @lst:  the return value for the set of parsed nodes
 *
 * Parse a well-balanced chunk of an XML document
 * within the context (DTD, namespaces, etc ...) of the given node.
 *
 * The allowed sequence for the data is a Well Balanced Chunk defined by
 * the content production in the XML grammar:
 *
 * [43] content ::= (element | CharData | Reference | CDSect | PI | Comment)*
 *
 * Returns XML_ERR_OK if the chunk is well balanced, and the parser
 * error code otherwise
 */
xmlParserErrors xmlParseInNodeContext(xmlNodePtr node, const char *data, int datalen,
                                      int options, xmlNodePtr *lst) {
    return xmlParseInNodeContext_rust(node,data, datalen,options, lst);
}

#ifdef LIBXML_SAX1_ENABLED
/**
 * xmlParseBalancedChunkMemoryRecover:
 * @doc:  the document the chunk pertains to (must not be NULL)
 * @sax:  the SAX handler block (possibly NULL)
 * @user_data:  The user data returned on SAX callbacks (possibly NULL)
 * @depth:  Used for loop detection, use 0
 * @string:  the input string in UTF8 or ISO-Latin (zero terminated)
 * @lst:  the return value for the set of parsed nodes
 * @recover: return nodes even if the data is broken (use 0)
 *
 *
 * Parse a well-balanced chunk of an XML document
 * called by the parser
 * The allowed sequence for the Well Balanced Chunk is the one defined by
 * the content production in the XML grammar:
 *
 * [43] content ::= (element | CharData | Reference | CDSect | PI | Comment)*
 *
 * Returns 0 if the chunk is well balanced, -1 in case of args problem and
 *    the parser error code otherwise
 *
 * In case recover is set to 1, the nodelist will not be empty even if
 * the parsed chunk is not well balanced, assuming the parsing succeeded to
 * some extent.
 */
int xmlParseBalancedChunkMemoryRecover(xmlDocPtr doc, xmlSAXHandlerPtr sax,
     void *user_data, int depth, const xmlChar *string, xmlNodePtr *lst,
     int recover) {
    return xmlParseBalancedChunkMemoryRecover_rust(doc, sax, user_data, depth, string, lst, recover);
}

/**
 * xmlSAXParseEntity:
 * @sax:  the SAX handler block
 * @filename:  the filename
 *
 * parse an XML external entity out of context and build a tree.
 * It use the given SAX function block to handle the parsing callback.
 * If sax is NULL, fallback to the default DOM tree building routines.
 *
 * [78] extParsedEnt ::= TextDecl? content
 *
 * This correspond to a "Well Balanced" chunk
 *
 * Returns the resulting document tree
 */

xmlDocPtr xmlSAXParseEntity(xmlSAXHandlerPtr sax, const char *filename) {
    return xmlSAXParseEntity_rust(sax, filename);
}

/**
 * xmlParseEntity:
 * @filename:  the filename
 *
 * parse an XML external entity out of context and build a tree.
 *
 * [78] extParsedEnt ::= TextDecl? content
 *
 * This correspond to a "Well Balanced" chunk
 *
 * Returns the resulting document tree
 */

xmlDocPtr xmlParseEntity(const char *filename) {
    return xmlParseEntity_rust(filename);
}
#endif /* LIBXML_SAX1_ENABLED */

/**
 * xmlCreateEntityParserCtxt:
 * @URL:  the entity URL
 * @ID:  the entity PUBLIC ID
 * @base:  a possible base for the target URI
 *
 * Create a parser context for an external entity
 * Automatic support for ZLIB/Compress compressed document is provided
 * by default if found at compile-time.
 *
 * Returns the new parser context or NULL
 */
xmlParserCtxtPtr xmlCreateEntityParserCtxt(const xmlChar *URL, const xmlChar *ID,
                                           const xmlChar *base) {
    return xmlCreateEntityParserCtxt_rust(URL, ID, base);
}

/************************************************************************
 *									*
 *		Front ends when parsing from a file			*
 *									*
 ************************************************************************/

/**
 * xmlCreateURLParserCtxt:
 * @filename:  the filename or URL
 * @options:  a combination of xmlParserOption
 *
 * Create a parser context for a file or URL content.
 * Automatic support for ZLIB/Compress compressed document is provided
 * by default if found at compile-time and for file accesses
 *
 * Returns the new parser context or NULL
 */
xmlParserCtxtPtr xmlCreateURLParserCtxt(const char *filename, int options)
{
    return xmlCreateURLParserCtxt_rust(filename, options);
}

/**
 * xmlCreateFileParserCtxt:
 * @filename:  the filename
 *
 * Create a parser context for a file content.
 * Automatic support for ZLIB/Compress compressed document is provided
 * by default if found at compile-time.
 *
 * Returns the new parser context or NULL
 */
xmlParserCtxtPtr xmlCreateFileParserCtxt(const char *filename)
{
    return xmlCreateFileParserCtxt_rust(filename);
}

#ifdef LIBXML_SAX1_ENABLED
/**
 * xmlSAXParseFileWithData:
 * @sax:  the SAX handler block
 * @filename:  the filename
 * @recovery:  work in recovery mode, i.e. tries to read no Well Formed
 *             documents
 * @data:  the userdata
 *
 * parse an XML file and build a tree. Automatic support for ZLIB/Compress
 * compressed document is provided by default if found at compile-time.
 * It use the given SAX function block to handle the parsing callback.
 * If sax is NULL, fallback to the default DOM tree building routines.
 *
 * User data (void *) is stored within the parser context in the
 * context's _private member, so it is available nearly everywhere in libxml
 *
 * Returns the resulting document tree
 */

xmlDocPtr xmlSAXParseFileWithData(xmlSAXHandlerPtr sax, const char *filename,
                        int recovery, void *data) {
    return xmlSAXParseFileWithData_rust(sax, filename, recovery, data);
}

/**
 * xmlSAXParseFile:
 * @sax:  the SAX handler block
 * @filename:  the filename
 * @recovery:  work in recovery mode, i.e. tries to read no Well Formed
 *             documents
 *
 * parse an XML file and build a tree. Automatic support for ZLIB/Compress
 * compressed document is provided by default if found at compile-time.
 * It use the given SAX function block to handle the parsing callback.
 * If sax is NULL, fallback to the default DOM tree building routines.
 *
 * Returns the resulting document tree
 */

xmlDocPtr xmlSAXParseFile(xmlSAXHandlerPtr sax, const char *filename,
                          int recovery) {
    return xmlSAXParseFile_rust(sax, filename, recovery);
}

/**
 * xmlRecoverDoc:
 * @cur:  a pointer to an array of xmlChar
 *
 * parse an XML in-memory document and build a tree.
 * In the case the document is not Well Formed, a attempt to build a
 * tree is tried anyway
 *
 * Returns the resulting document tree or NULL in case of failure
 */

xmlDocPtr xmlRecoverDoc(const xmlChar *cur) {
    return xmlRecoverDoc_rust(cur);
}

/**
 * xmlParseFile:
 * @filename:  the filename
 *
 * parse an XML file and build a tree. Automatic support for ZLIB/Compress
 * compressed document is provided by default if found at compile-time.
 *
 * Returns the resulting document tree if the file was wellformed,
 * NULL otherwise.
 */

xmlDocPtr xmlParseFile(const char *filename) {
    return xmlParseFile_rust(filename);
}

/**
 * xmlRecoverFile:
 * @filename:  the filename
 *
 * parse an XML file and build a tree. Automatic support for ZLIB/Compress
 * compressed document is provided by default if found at compile-time.
 * In the case the document is not Well Formed, it attempts to build
 * a tree anyway
 *
 * Returns the resulting document tree or NULL in case of failure
 */

xmlDocPtr xmlRecoverFile(const char *filename) {
    return xmlRecoverFile_rust(filename);
}


/**
 * xmlSetupParserForBuffer:
 * @ctxt:  an XML parser context
 * @buffer:  a xmlChar * buffer
 * @filename:  a file name
 *
 * Setup the parser context to parse a new buffer; Clears any prior
 * contents from the parser context. The buffer parameter must not be
 * NULL, but the filename parameter can be
 */
void xmlSetupParserForBuffer(xmlParserCtxtPtr ctxt, const xmlChar* buffer,
                             const char* filename)
{
    xmlSetupParserForBuffer_rust(ctxt, buffer, filename);
}

/**
 * xmlSAXUserParseFile:
 * @sax:  a SAX handler
 * @user_data:  The user data returned on SAX callbacks
 * @filename:  a file name
 *
 * parse an XML file and call the given SAX handler routines.
 * Automatic support for ZLIB/Compress compressed document is provided
 *
 * Returns 0 in case of success or a error number otherwise
 */
int xmlSAXUserParseFile(xmlSAXHandlerPtr sax, void *user_data,
                    const char *filename) {
    return xmlSAXUserParseFile_rust(sax, user_data, filename);
}
#endif /* LIBXML_SAX1_ENABLED */

/************************************************************************
 *									*
 *		Front ends when parsing from memory			*
 *									*
 ************************************************************************/

/**
 * xmlCreateMemoryParserCtxt:
 * @buffer:  a pointer to a char array
 * @size:  the size of the array
 *
 * Create a parser context for an XML in-memory document.
 *
 * Returns the new parser context or NULL
 */
xmlParserCtxtPtr xmlCreateMemoryParserCtxt(const char *buffer, int size) {
    return xmlCreateMemoryParserCtxt_rust(buffer, size);
}

#ifdef LIBXML_SAX1_ENABLED
/**
 * xmlSAXParseMemoryWithData:
 * @sax:  the SAX handler block
 * @buffer:  an pointer to a char array
 * @size:  the size of the array
 * @recovery:  work in recovery mode, i.e. tries to read no Well Formed
 *             documents
 * @data:  the userdata
 *
 * parse an XML in-memory block and use the given SAX function block
 * to handle the parsing callback. If sax is NULL, fallback to the default
 * DOM tree building routines.
 *
 * User data (void *) is stored within the parser context in the
 * context's _private member, so it is available nearly everywhere in libxml
 *
 * Returns the resulting document tree
 */

xmlDocPtr xmlSAXParseMemoryWithData(xmlSAXHandlerPtr sax, const char *buffer,
	          int size, int recovery, void *data) {
    return xmlSAXParseMemoryWithData_rust(sax, buffer, size, recovery, data);
}
/**
 * xmlSAXParseMemory:
 * @sax:  the SAX handler block
 * @buffer:  an pointer to a char array
 * @size:  the size of the array
 * @recovery:  work in recovery mode, i.e. tries to read not Well Formed
 *             documents
 *
 * parse an XML in-memory block and use the given SAX function block
 * to handle the parsing callback. If sax is NULL, fallback to the default
 * DOM tree building routines.
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlSAXParseMemory(xmlSAXHandlerPtr sax, const char *buffer,
	          int size, int recovery) {
    return xmlSAXParseMemory_rust(sax, buffer, size, recovery);
}

/**
 * xmlParseMemory:
 * @buffer:  an pointer to a char array
 * @size:  the size of the array
 *
 * parse an XML in-memory block and build a tree.
 *
 * Returns the resulting document tree
 */

xmlDocPtr xmlParseMemory(const char *buffer, int size) {
   return xmlParseMemory_rust(buffer, size);
}

/**
 * xmlRecoverMemory:
 * @buffer:  an pointer to a char array
 * @size:  the size of the array
 *
 * parse an XML in-memory block and build a tree.
 * In the case the document is not Well Formed, an attempt to
 * build a tree is tried anyway
 *
 * Returns the resulting document tree or NULL in case of error
 */

xmlDocPtr xmlRecoverMemory(const char *buffer, int size) {
   return xmlRecoverMemory_rust(buffer, size);
}

/**
 * xmlSAXUserParseMemory:
 * @sax:  a SAX handler
 * @user_data:  The user data returned on SAX callbacks
 * @buffer:  an in-memory XML document input
 * @size:  the length of the XML document in bytes
 *
 * A better SAX parsing routine.
 * parse an XML in-memory buffer and call the given SAX handler routines.
 *
 * Returns 0 in case of success or a error number otherwise
 */
int xmlSAXUserParseMemory(xmlSAXHandlerPtr sax, void *user_data,
			  const char *buffer, int size) {
    return xmlSAXUserParseMemory_rust(sax, user_data, buffer, size);
}
#endif /* LIBXML_SAX1_ENABLED */

/**
 * xmlCreateDocParserCtxt:
 * @cur:  a pointer to an array of xmlChar
 *
 * Creates a parser context for an XML in-memory document.
 *
 * Returns the new parser context or NULL
 */
xmlParserCtxtPtr xmlCreateDocParserCtxt(const xmlChar *cur) {
    return xmlCreateDocParserCtxt_rust(cur);
}

#ifdef LIBXML_SAX1_ENABLED
/**
 * xmlSAXParseDoc:
 * @sax:  the SAX handler block
 * @cur:  a pointer to an array of xmlChar
 * @recovery:  work in recovery mode, i.e. tries to read no Well Formed
 *             documents
 *
 * parse an XML in-memory document and build a tree.
 * It use the given SAX function block to handle the parsing callback.
 * If sax is NULL, fallback to the default DOM tree building routines.
 *
 * Returns the resulting document tree
 */

xmlDocPtr xmlSAXParseDoc(xmlSAXHandlerPtr sax, const xmlChar *cur, int recovery) {
    return xmlSAXParseDoc_rust(sax, cur, recovery);
}
/**
 * xmlParseDoc:
 * @cur:  a pointer to an array of xmlChar
 *
 * parse an XML in-memory document and build a tree.
 *
 * Returns the resulting document tree
 */

xmlDocPtr xmlParseDoc(const xmlChar *cur) {
    return xmlParseDoc_rust(cur);
}
#endif /* LIBXML_SAX1_ENABLED */

#ifdef LIBXML_LEGACY_ENABLED
/**
 * xmlSetEntityReferenceFunc:
 * @func: A valid function
 *
 * Set the function to call call back when a xml reference has been made
 */
void xmlSetEntityReferenceFunc(xmlEntityReferenceFunc func)
{
    xmlSetEntityReferenceFunc_rust(func);
}
#endif /* LIBXML_LEGACY_ENABLED */

/**
 * xmlInitParser:
 *
 * Initialization function for the XML parser.
 * This is not reentrant. Call once before processing in case of
 * use in multithreaded programs.
 */
void xmlInitParser(void){
    xmlInitParser_rust();
}

/**
 * xmlCleanupParser:
 *
 * This function name is somewhat misleading. It does not clean up
 * parser state, it cleans up memory allocated by the library itself.
 * It is a cleanup function for the XML library. It tries to reclaim all
 * related global memory allocated for the library processing.
 * It doesn't deallocate any document related memory. One should
 * call xmlCleanupParser() only when the process has finished using
 * the library and all XML/HTML documents built with it.
 * See also xmlInitParser() which has the opposite function of preparing
 * the library for operations.
 *
 * WARNING: if your application is multithreaded or has plugin support
 *          calling this may crash the application if another thread or
 *          a plugin is still using libxml2. It's sometimes very hard to
 *          guess if libxml2 is in use in the application, some libraries
 *          or plugins may use it without notice. In case of doubt abstain
 *          from calling this function or do it just before calling exit()
 *          to avoid leak reports from valgrind !
 */
void xmlCleanupParser(void){
    xmlCleanupParser_rust();
}

/**
 * xmlCtxtReset:
 * @ctxt: an XML parser context
 *
 * Reset a parser context
 */
void xmlCtxtReset(xmlParserCtxtPtr ctxt){
    xmlCtxtReset_rust(ctxt);
}

/**
 * xmlCtxtResetPush:
 * @ctxt: an XML parser context
 * @chunk:  a pointer to an array of chars
 * @size:  number of chars in the array
 * @filename:  an optional file name or URI
 * @encoding:  the document encoding, or NULL
 *
 * Reset a push parser context
 *
 * Returns 0 in case of success and 1 in case of error
 */
int xmlCtxtResetPush(xmlParserCtxtPtr ctxt, const char *chunk,
                     int size, const char *filename, const char *encoding){
    return xmlCtxtResetPush_rust(ctxt, chunk, size, filename, encoding);
}

/**
 * xmlCtxtUseOptions:
 * @ctxt: an XML parser context
 * @options:  a combination of xmlParserOption
 *
 * Applies the options to the parser context
 *
 * Returns 0 in case of success, the set of unknown or unimplemented options
 *         in case of error.
 */
int xmlCtxtUseOptions(xmlParserCtxtPtr ctxt, int options)
{
    return xmlCtxtUseOptions_rust(ctxt, options);
}

/**
 * xmlReadDoc:
 * @cur:  a pointer to a zero terminated string
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML in-memory document and build a tree.
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlReadDoc(const xmlChar * cur, const char *URL, const char *encoding, int options)
{
    return xmlReadDoc_rust(cur, URL, encoding, options);
}

/**
 * xmlReadFile:
 * @filename:  a file or URL
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML file from the filesystem or the network.
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlReadFile(const char *filename, const char *encoding, int options)
{
    return xmlReadFile_rust(filename, encoding, options);
}

/**
 * xmlReadMemory:
 * @buffer:  a pointer to a char array
 * @size:  the size of the array
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML in-memory document and build a tree.
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlReadMemory(const char *buffer, int size, const char *URL, const char *encoding, int options)
{
    return xmlReadMemory_rust(buffer, size, URL, encoding, options);
}

/**
 * xmlReadFd:
 * @fd:  an open file descriptor
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML from a file descriptor and build a tree.
 * NOTE that the file descriptor will not be closed when the
 *      reader is closed or reset.
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlReadFd(int fd, const char *URL, const char *encoding, int options)
{
    return xmlReadFd_rust(fd, URL, encoding, options);
}

/**
 * xmlReadIO:
 * @ioread:  an I/O read function
 * @ioclose:  an I/O close function
 * @ioctx:  an I/O handler
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML document from I/O functions and source and build a tree.
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlReadIO(xmlInputReadCallback ioread, xmlInputCloseCallback ioclose,
                    void *ioctx, const char *URL, const char *encoding, int options)
{
    return xmlReadIO_rust(ioread, ioclose, ioctx, URL, encoding, options);
}

/**
 * xmlCtxtReadDoc:
 * @ctxt:  an XML parser context
 * @cur:  a pointer to a zero terminated string
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML in-memory document and build a tree.
 * This reuses the existing @ctxt parser context
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlCtxtReadDoc(xmlParserCtxtPtr ctxt, const xmlChar * cur,
                         const char *URL, const char *encoding, int options)
{
    return xmlCtxtReadDoc_rust(ctxt, cur, URL, encoding, options);
}

/**
 * xmlCtxtReadFile:
 * @ctxt:  an XML parser context
 * @filename:  a file or URL
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML file from the filesystem or the network.
 * This reuses the existing @ctxt parser context
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlCtxtReadFile(xmlParserCtxtPtr ctxt, const char *filename,
                          const char *encoding, int options)
{
    return xmlCtxtReadFile_rust(ctxt, filename, encoding, options);
}

/**
 * xmlCtxtReadMemory:
 * @ctxt:  an XML parser context
 * @buffer:  a pointer to a char array
 * @size:  the size of the array
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML in-memory document and build a tree.
 * This reuses the existing @ctxt parser context
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlCtxtReadMemory(xmlParserCtxtPtr ctxt, const char *buffer, int size,
                            const char *URL, const char *encoding, int options)
{
    return xmlCtxtReadMemory_rust(ctxt, buffer, size, URL, encoding, options);
}

/**
 * xmlCtxtReadFd:
 * @ctxt:  an XML parser context
 * @fd:  an open file descriptor
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML from a file descriptor and build a tree.
 * This reuses the existing @ctxt parser context
 * NOTE that the file descriptor will not be closed when the
 *      reader is closed or reset.
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlCtxtReadFd(xmlParserCtxtPtr ctxt, int fd,
                        const char *URL, const char *encoding, int options)
{
    return xmlCtxtReadFd_rust(ctxt, fd, URL, encoding, options);
}

/**
 * xmlCtxtReadIO:
 * @ctxt:  an XML parser context
 * @ioread:  an I/O read function
 * @ioclose:  an I/O close function
 * @ioctx:  an I/O handler
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML document from I/O functions and source and build a tree.
 * This reuses the existing @ctxt parser context
 *
 * Returns the resulting document tree
 */
xmlDocPtr xmlCtxtReadIO(xmlParserCtxtPtr ctxt, xmlInputReadCallback ioread,
                        xmlInputCloseCallback ioclose, void *ioctx,
                        const char *URL,
                        const char *encoding, int options)
{
    return xmlCtxtReadIO_rust(ctxt, ioread, ioclose, ioctx, URL, encoding, options);
}

// endregion1
#endif


#ifndef COMPILE_WITH_RUST
// region2: 未被COMPILE_WITH_RUST宏定义的部分表示需要连接前需要对外界提供的内容，包括：对rust部分提供的宏出口，以及对外部提供的空壳方法
// region2.1: 宏获取方法
int get_parser_LIBXML_XINCLUDE_ENABLED(){
#ifdef LIBXML_XINCLUDE_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_THREAD_ENABLED(){
#ifdef LIBXML_THREAD_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_HTML_ENABLED(){
#ifdef LIBXML_HTML_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_HAVE_UNISTD_H(){
#ifdef HAVE_UNISTD_H
    return 1;
#else
    return 0;
#endif
}
int get_parser_DEBUG_PUSH(){
#ifdef DEBUG_PUSH
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_SCHEMATRON_ENABLED(){
#ifdef LIBXML_SCHEMATRON_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_SAX2(){
#ifdef SAX2
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_ICU_ENABLED(){
#ifdef LIBXML_ICU_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_UNICODE_ENABLED(){
#ifdef LIBXML_UNICODE_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_FTP_ENABLED(){
#ifdef LIBXML_FTP_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_HTTP_ENABLED(){
#ifdef LIBXML_HTTP_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_HAVE_ATTRIBUTE_DESTRUCTOR(){
#ifdef HAVE_ATTRIBUTE_DESTRUCTOR
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_STATIC(){
#ifdef LIBXML_STATIC
    return 1;
#else
    return 0;
#endif
}
int get_parser_WIN32(){
#ifdef _WIN32
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_DEBUG_RUNTIME(){
#ifdef LIBXML_DEBUG_RUNTIME
    return 1;
#else
    return 0;
#endif
}
int get_parser_STDC_VERSION_199901(){
#if __STDC_VERSION__ >= 199901L
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_CATALOG_ENABLED(){
#ifdef LIBXML_CATALOG_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_REGEXP_ENABLED(){
#ifdef LIBXML_REGEXP_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_LZMA_ENABLED(){
#ifdef LIBXML_LZMA_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_SCHEMAS_ENABLED(){
#ifdef LIBXML_SCHEMAS_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_HAVE_CTYPE_H(){
#ifdef HAVE_CTYPE_H
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_STATIC_FOR_DLL(){
#ifdef LIBXML_STATIC_FOR_DLL
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_READER_ENABLED(){
#ifdef LIBXML_READER_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_HAVE_FCNTL_H(){
#ifdef HAVE_FCNTL_H
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_ZLIB_ENABLED(){
#ifdef LIBXML_ZLIB_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_XML_XML_NAMESPACE(){
#ifdef XML_XML_NAMESPACE
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_PATTERN_ENABLED(){
#ifdef LIBXML_PATTERN_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_VALID_ENABLED(){
#ifdef LIBXML_VALID_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_C14N_ENABLED(){
#ifdef LIBXML_C14N_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_WRITER_ENABLED(){
#ifdef LIBXML_WRITER_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_AUTOMATA_ENABLED(){
#ifdef LIBXML_AUTOMATA_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_TREE_ENABLED(){
#ifdef LIBXML_TREE_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_XPTR_ENABLED(){
#ifdef LIBXML_XPTR_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_XPATH_ENABLED(){
#ifdef LIBXML_XPATH_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_DEBUG_MEMORY_LOCATION(){
#ifdef DEBUG_MEMORY_LOCATION
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_SAX1_ENABLED(){
#ifdef LIBXML_SAX1_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_DEBUG_ENABLED(){
#ifdef LIBXML_DEBUG_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_HAVE_SYS_STAT_H(){
#ifdef HAVE_SYS_STAT_H
    return 1;
#else
    return 0;
#endif
}
int get_parser_HAVE_STDLIB_H(){
#ifdef HAVE_STDLIB_H
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_ICONV_ENABLED(){
#ifdef LIBXML_ICONV_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_EXPR_ENABLED(){
#ifdef LIBXML_EXPR_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_DEBUG(){
#ifdef DEBUG
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_PUSH_ENABLED(){
#ifdef LIBXML_PUSH_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_LEGACY_ENABLED(){
#ifdef LIBXML_LEGACY_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_MODULES_ENABLED(){
#ifdef LIBXML_MODULES_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_OUTPUT_ENABLED(){
#ifdef LIBXML_OUTPUT_ENABLED
    return 1;
#else
    return 0;
#endif
}
int get_parser_LIBXML_ISO8859X_ENABLED(){
#ifdef LIBXML_ISO8859X_ENABLED
    return 1;
#else
    return 0;
#endif
}
// endregion2.1

// region2.2: 对外提供的空壳方法，以便编译可以通过

int
xmlHasFeature(xmlFeature feature)
{
    return(0);
}

int xmlCheckLanguageID(const xmlChar * lang)
{
    return 0;
}

int inputPush(xmlParserCtxtPtr ctxt, xmlParserInputPtr value)
{
    return 0;
}

xmlParserInputPtr inputPop(xmlParserCtxtPtr ctxt)
{
    return NULL;
}

int nodePush(xmlParserCtxtPtr ctxt, xmlNodePtr value)
{
    return 0;
}

xmlNodePtr nodePop(xmlParserCtxtPtr ctxt)
{
    return NULL;
}

int namePush(xmlParserCtxtPtr ctxt, const xmlChar * value)
{
    return 0;
}

const xmlChar * namePop(xmlParserCtxtPtr ctxt)
{
    return NULL;
}

int xmlSkipBlankChars(xmlParserCtxtPtr ctxt) {
    return 0;
}

xmlChar xmlPopInput(xmlParserCtxtPtr ctxt) {
    return NULL;
}

int xmlPushInput(xmlParserCtxtPtr ctxt, xmlParserInputPtr input) {
    return 0;
}

int xmlParseCharRef(xmlParserCtxtPtr ctxt) {
    return 0;
}

void xmlParserHandlePEReference(xmlParserCtxtPtr ctxt) {

}

xmlChar * xmlStringLenDecodeEntities(xmlParserCtxtPtr ctxt, const xmlChar *str, int len,
                                     int what, xmlChar end, xmlChar  end2, xmlChar end3) {
    return NULL;
}

xmlChar * xmlStringDecodeEntities(xmlParserCtxtPtr ctxt, const xmlChar *str, int what,
                                  xmlChar end, xmlChar  end2, xmlChar end3) {
    return NULL;
}

xmlChar * xmlSplitQName(xmlParserCtxtPtr ctxt, const xmlChar *name, xmlChar **prefix) {
    return NULL;
}

const xmlChar * xmlParseName(xmlParserCtxtPtr ctxt) {
    return NULL;
}

xmlChar * xmlParseNmtoken(xmlParserCtxtPtr ctxt) {
    return NULL;
}

xmlChar * xmlParseEntityValue(xmlParserCtxtPtr ctxt, xmlChar **orig) {
    return NULL;
}

xmlChar * xmlParseAttValue(xmlParserCtxtPtr ctxt) {
    return NULL;
}

xmlChar * xmlParseSystemLiteral(xmlParserCtxtPtr ctxt) {
    return NULL;
}

xmlChar * xmlParsePubidLiteral(xmlParserCtxtPtr ctxt) {
    return NULL;
}

void xmlParseCharData(xmlParserCtxtPtr ctxt, int cdata) {

}

xmlChar * xmlParseExternalID(xmlParserCtxtPtr ctxt, xmlChar **publicID, int strict) {
    return NULL;
}

void xmlParseComment(xmlParserCtxtPtr ctxt) {

}

const xmlChar * xmlParsePITarget(xmlParserCtxtPtr ctxt) {
    return NULL;
}

void xmlParsePI(xmlParserCtxtPtr ctxt) {

}

void xmlParseNotationDecl(xmlParserCtxtPtr ctxt) {

}

void xmlParseEntityDecl(xmlParserCtxtPtr ctxt) {

}

int xmlParseDefaultDecl(xmlParserCtxtPtr ctxt, xmlChar **value) {
    return 0;
}

xmlEnumerationPtr xmlParseNotationType(xmlParserCtxtPtr ctxt) {
    return NULL;
}

xmlEnumerationPtr xmlParseEnumerationType(xmlParserCtxtPtr ctxt) {
    return NULL;
}

int xmlParseEnumeratedType(xmlParserCtxtPtr ctxt, xmlEnumerationPtr *tree) {
    return 0;
}

int xmlParseAttributeType(xmlParserCtxtPtr ctxt, xmlEnumerationPtr *tree) {
    return 0;
}

void xmlParseAttributeListDecl(xmlParserCtxtPtr ctxt) {

}

xmlElementContentPtr xmlParseElementMixedContentDecl(xmlParserCtxtPtr ctxt, int inputchk) {
    return NULL;
}

xmlElementContentPtr xmlParseElementChildrenContentDecl(xmlParserCtxtPtr ctxt, int inputchk) {
    return NULL;
}

int xmlParseElementContentDecl(xmlParserCtxtPtr ctxt, const xmlChar *name, xmlElementContentPtr *result) {
    return 0;
}

int xmlParseElementDecl(xmlParserCtxtPtr ctxt) {
    return 0;
}

void xmlParseMarkupDecl(xmlParserCtxtPtr ctxt) {

}

void xmlParseTextDecl(xmlParserCtxtPtr ctxt) {

}

void xmlParseExternalSubset(xmlParserCtxtPtr ctxt, const xmlChar *ExternalID, const xmlChar *SystemID) {

}

void xmlParseReference(xmlParserCtxtPtr ctxt) {

}

xmlEntityPtr xmlParseEntityRef(xmlParserCtxtPtr ctxt) {
    return NULL;
}

void xmlParsePEReference(xmlParserCtxtPtr ctxt){

}

void xmlParseDocTypeDecl(xmlParserCtxtPtr ctxt) {

}

#ifdef LIBXML_SAX1_ENABLED
const xmlChar *
xmlParseAttribute(xmlParserCtxtPtr ctxt, xmlChar **value) {
    return NULL;
}

const xmlChar *
xmlParseStartTag(xmlParserCtxtPtr ctxt) {
    return NULL;
}

void xmlParseEndTag(xmlParserCtxtPtr ctxt) {

}
#endif

void xmlParseCDSect(xmlParserCtxtPtr ctxt) {

}

void xmlParseContent(xmlParserCtxtPtr ctxt) {

}

void xmlParseElement(xmlParserCtxtPtr ctxt) {

}

xmlChar * xmlParseVersionNum(xmlParserCtxtPtr ctxt) {
    return NULL;
}

xmlChar * xmlParseVersionInfo(xmlParserCtxtPtr ctxt) {
    return NULL;
}

xmlChar * xmlParseEncName(xmlParserCtxtPtr ctxt) {
    return NULL;
}

const xmlChar * xmlParseEncodingDecl(xmlParserCtxtPtr ctxt) {
    return NULL;
}

int xmlParseSDDecl(xmlParserCtxtPtr ctxt) {
    return 0;
}

void xmlParseXMLDecl(xmlParserCtxtPtr ctxt) {

}

void xmlParseMisc(xmlParserCtxtPtr ctxt) {

}

int xmlParseDocument(xmlParserCtxtPtr ctxt) {
    return 0;
}

int xmlParseExtParsedEnt(xmlParserCtxtPtr ctxt) {
    return 0;
}

#ifdef LIBXML_PUSH_ENABLED
int xmlParseChunk(xmlParserCtxtPtr ctxt, const char *chunk, int size,
              int terminate) {
    return 0;
}

xmlParserCtxtPtr xmlCreatePushParserCtxt(xmlSAXHandlerPtr sax, void *user_data,
                        const char *chunk, int size, const char *filename) {
    return NULL;
}
#endif /* LIBXML_PUSH_ENABLED */

void xmlStopParser(xmlParserCtxtPtr ctxt) {

}

xmlParserCtxtPtr xmlCreateIOParserCtxt(xmlSAXHandlerPtr sax, void *user_data,
                                       xmlInputReadCallback   ioread, xmlInputCloseCallback  ioclose,
                                       void *ioctx, xmlCharEncoding enc) {

    return NULL;
}

#ifdef LIBXML_VALID_ENABLED
xmlDtdPtr xmlIOParseDTD(xmlSAXHandlerPtr sax, xmlParserInputBufferPtr input,
	      xmlCharEncoding enc) {
    return NULL;
}

xmlDtdPtr xmlSAXParseDTD(xmlSAXHandlerPtr sax, const xmlChar *ExternalID,
                          const xmlChar *SystemID) {
    return NULL;
}

xmlDtdPtr xmlParseDTD(const xmlChar *ExternalID, const xmlChar *SystemID) {
    return NULL;
}
#endif /* LIBXML_VALID_ENABLED */

int xmlParseCtxtExternalEntity(xmlParserCtxtPtr ctx, const xmlChar *URL,
                               const xmlChar *ID, xmlNodePtr *lst) {
    return 0;
}

#ifdef LIBXML_SAX1_ENABLED
int xmlParseExternalEntity(xmlDocPtr doc, xmlSAXHandlerPtr sax, void *user_data,
	  int depth, const xmlChar *URL, const xmlChar *ID, xmlNodePtr *lst) {
    return 0;
}

int xmlParseBalancedChunkMemory(xmlDocPtr doc, xmlSAXHandlerPtr sax,
     void *user_data, int depth, const xmlChar *string, xmlNodePtr *lst) {

    return 0;
}
#endif /* LIBXML_SAX1_ENABLED */

xmlParserErrors xmlParseInNodeContext(xmlNodePtr node, const char *data, int datalen,
                                      int options, xmlNodePtr *lst) {
    return XML_ERR_OK;
}

#ifdef LIBXML_SAX1_ENABLED
int xmlParseBalancedChunkMemoryRecover(xmlDocPtr doc, xmlSAXHandlerPtr sax,
     void *user_data, int depth, const xmlChar *string, xmlNodePtr *lst,
     int recover) {
    return 0;
}

xmlDocPtr xmlSAXParseEntity(xmlSAXHandlerPtr sax, const char *filename) {
    return NULL;
}

xmlDocPtr xmlParseEntity(const char *filename) {
    return NULL;
}
#endif

xmlParserCtxtPtr xmlCreateEntityParserCtxt(const xmlChar *URL, const xmlChar *ID,
                                           const xmlChar *base) {
    return NULL;
}

xmlParserCtxtPtr xmlCreateURLParserCtxt(const char *filename, int options){
    return NULL;
}

xmlParserCtxtPtr xmlCreateFileParserCtxt(const char *filename){
    return NULL;
}

#ifdef LIBXML_SAX1_ENABLED
xmlDocPtr xmlSAXParseFileWithData(xmlSAXHandlerPtr sax, const char *filename,
                        int recovery, void *data) {
    return NULL;
}

xmlDocPtr xmlSAXParseFile(xmlSAXHandlerPtr sax, const char *filename,
                          int recovery) {
    return NULL;
}

xmlDocPtr xmlRecoverDoc(const xmlChar *cur) {
    return NULL;
}

xmlDocPtr xmlParseFile(const char *filename) {
    return NULL;
}

xmlDocPtr xmlRecoverFile(const char *filename) {
    return NULL;
}

void xmlSetupParserForBuffer(xmlParserCtxtPtr ctxt, const xmlChar* buffer,
                             const char* filename){

}

int xmlSAXUserParseFile(xmlSAXHandlerPtr sax, void *user_data,
                    const char *filename) {
    return 0;
}
#endif

xmlParserCtxtPtr xmlCreateMemoryParserCtxt(const char *buffer, int size) {
    return NULL;
}

#ifdef LIBXML_SAX1_ENABLED
xmlDocPtr xmlSAXParseMemoryWithData(xmlSAXHandlerPtr sax, const char *buffer,
	          int size, int recovery, void *data) {
    return NULL;
}

xmlDocPtr xmlSAXParseMemory(xmlSAXHandlerPtr sax, const char *buffer,
	          int size, int recovery) {
    return NULL;
}

xmlDocPtr xmlParseMemory(const char *buffer, int size) {
   return NULL;
}

xmlDocPtr xmlRecoverMemory(const char *buffer, int size) {
   return NULL;
}

int xmlSAXUserParseMemory(xmlSAXHandlerPtr sax, void *user_data,
			  const char *buffer, int size) {
    return 0;
}
#endif

xmlParserCtxtPtr xmlCreateDocParserCtxt(const xmlChar *cur) {
    return NULL;
}

#ifdef LIBXML_SAX1_ENABLED
xmlDocPtr xmlSAXParseDoc(xmlSAXHandlerPtr sax, const xmlChar *cur, int recovery) {
    return NULL;
}

xmlDocPtr xmlParseDoc(const xmlChar *cur) {
    return NULL;
}
#endif

#ifdef LIBXML_LEGACY_ENABLED
void xmlSetEntityReferenceFunc(xmlEntityReferenceFunc func){

}
#endif

void xmlInitParser(void){

}

void xmlCleanupParser(void){

}

void xmlCtxtReset(xmlParserCtxtPtr ctxt){

}

int xmlCtxtResetPush(xmlParserCtxtPtr ctxt, const char *chunk,
                     int size, const char *filename, const char *encoding){
    return 0;
}

int xmlCtxtUseOptions(xmlParserCtxtPtr ctxt, int options){
    return 0;
}

xmlDocPtr xmlReadDoc(const xmlChar * cur, const char *URL, const char *encoding, int options){
    return NULL;
}

xmlDocPtr xmlReadFile(const char *filename, const char *encoding, int options){
    return NULL;
}

xmlDocPtr xmlReadMemory(const char *buffer, int size, const char *URL, const char *encoding, int options){
    return NULL;
}

xmlDocPtr xmlReadFd(int fd, const char *URL, const char *encoding, int options){
    return NULL;
}

xmlDocPtr xmlReadIO(xmlInputReadCallback ioread, xmlInputCloseCallback ioclose,
                    void *ioctx, const char *URL, const char *encoding, int options){
    return NULL;
}

xmlDocPtr xmlCtxtReadDoc(xmlParserCtxtPtr ctxt, const xmlChar * cur,
                         const char *URL, const char *encoding, int options){
    return NULL;
}

xmlDocPtr xmlCtxtReadFile(xmlParserCtxtPtr ctxt, const char *filename,
                          const char *encoding, int options){
    return NULL;
}

xmlDocPtr xmlCtxtReadMemory(xmlParserCtxtPtr ctxt, const char *buffer, int size,
                            const char *URL, const char *encoding, int options){
    return NULL;
}

xmlDocPtr xmlCtxtReadFd(xmlParserCtxtPtr ctxt, int fd,
                        const char *URL, const char *encoding, int options){
    return NULL;
}

xmlDocPtr xmlCtxtReadIO(xmlParserCtxtPtr ctxt, xmlInputReadCallback ioread,
                        xmlInputCloseCallback ioclose, void *ioctx,
                        const char *URL,
                        const char *encoding, int options){
    return NULL;
}
// endregion2.2

// endregion2
#endif


#define bottom_parser
#include "elfgcchack.h"
