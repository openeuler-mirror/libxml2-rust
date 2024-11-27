/*
 * HTMLparser.c : an HTML 4.0 non-verifying parser
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */

#define IN_LIBXML

#include "libxml.h"

//#define DEBUG
//#define DEBUG_PUSH

#ifdef COMPILE_WITH_RUST

#ifdef LIBXML_HTML_ENABLED

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
#include <libxml/xmlerror.h>
#include <libxml/HTMLparser.h>
#include <libxml/HTMLtree.h>
#include <libxml/entities.h>
#include <libxml/encoding.h>
#include <libxml/valid.h>
#include <libxml/xmlIO.h>
#include <libxml/globals.h>
#include <libxml/uri.h>

#include "buf.h"
#include "enc.h"

#define HTML_MAX_NAMELEN 1000
#define HTML_PARSER_BIG_BUFFER_SIZE 1000
#define HTML_PARSER_BUFFER_SIZE 100

//struct libxml2_htmlparser_defined_param{
//	unsigned int html_max_namelen;
//	unsigned int html_parser_big_buffer_size;
//    unsigned int html_parser_buffer_size;
//    unsigned int input_chunk;
//};

extern const htmlElemDesc *htmlTagLookup_rust(const xmlChar *tag);

extern int htmlAutoCloseTag_rust(htmlDocPtr doc, const xmlChar *name, htmlNodePtr elem);

extern int htmlIsAutoClosed_rust(htmlDocPtr doc, htmlNodePtr elem);

extern int htmlIsScriptAttribute_rust(const xmlChar *name);

extern const htmlEntityDesc *htmlEntityLookup_rust(const xmlChar *name);

extern const htmlEntityDesc *htmlEntityValueLookup_rust(unsigned int value);

extern int UTF8ToHtml_rust(unsigned char *out, int *outlen,
                           const unsigned char *in, int *inlen);

extern int htmlEncodeEntities_rust(unsigned char *out, int *outlen,
                                   const unsigned char *in, int *inlen, int quoteChar);

extern htmlDocPtr htmlNewDocNoDtD_rust(const xmlChar *URI, const xmlChar *ExternalID);

extern htmlDocPtr htmlNewDoc_rust(const xmlChar *URI, const xmlChar *ExternalID);

extern const htmlEntityDesc *htmlParseEntityRef_rust(htmlParserCtxtPtr ctxt, const xmlChar **str);

extern int htmlParseCharRef_rust(htmlParserCtxtPtr ctxt);

extern void htmlParseElement_rust(htmlParserCtxtPtr ctxt);

extern void __htmlParseContent_rust(void *ctxt);

extern int htmlParseDocument_rust(htmlParserCtxtPtr ctxt);

extern void htmlFreeParserCtxt_rust(htmlParserCtxtPtr ctxt);

extern htmlParserCtxtPtr htmlNewParserCtxt_rust(void);

extern htmlParserCtxtPtr htmlCreateMemoryParserCtxt_rust(const char *buffer, int size);

extern int htmlParseChunk_rust(htmlParserCtxtPtr ctxt, const char *chunk, int size,
                               int terminate);

extern htmlParserCtxtPtr htmlCreatePushParserCtxt_rust(htmlSAXHandlerPtr sax, void *user_data,
                                                       const char *chunk, int size, const char *filename,
                                                       xmlCharEncoding enc);

extern htmlDocPtr htmlSAXParseDoc_rust(const xmlChar *cur, const char *encoding,
                                       htmlSAXHandlerPtr sax, void *userData);

extern htmlDocPtr htmlParseDoc_rust(const xmlChar *cur, const char *encoding);

extern htmlParserCtxtPtr
htmlCreateFileParserCtxt_rust(const char *filename, const char *encoding);

extern htmlDocPtr htmlSAXParseFile_rust(const char *filename, const char *encoding, htmlSAXHandlerPtr sax,
                                        void *userData);

extern htmlDocPtr htmlParseFile_rust(const char *filename, const char *encoding);

extern int htmlHandleOmittedElem_rust(int val);

extern int htmlElementAllowedHere_rust(const htmlElemDesc *parent, const xmlChar *elt);

extern htmlStatus htmlElementStatusHere_rust(const htmlElemDesc *parent, const htmlElemDesc *elt);

extern htmlStatus htmlAttrAllowed_rust(const htmlElemDesc *elt, const xmlChar *attr, int legacy);

extern htmlStatus htmlNodeStatus_rust(const htmlNodePtr node, int legacy);

extern void htmlCtxtReset_rust(htmlParserCtxtPtr ctxt);

extern int htmlCtxtUseOptions_rust(htmlParserCtxtPtr ctxt, int options);

extern htmlDocPtr htmlReadDoc_rust(const xmlChar *cur, const char *URL, const char *encoding, int options);

extern htmlDocPtr htmlReadFile_rust(const char *filename, const char *encoding, int options);

extern htmlDocPtr htmlReadMemory_rust(const char *buffer, int size, const char *URL, const char *encoding, int options);

extern htmlDocPtr htmlReadFd_rust(int fd, const char *URL, const char *encoding, int options);

extern htmlDocPtr htmlReadIO_rust(xmlInputReadCallback ioread, xmlInputCloseCallback ioclose,
                                  void *ioctx, const char *URL, const char *encoding, int options);

extern htmlDocPtr htmlCtxtReadDoc_rust(htmlParserCtxtPtr ctxt, const xmlChar *cur,
                                       const char *URL, const char *encoding, int options);

extern htmlDocPtr htmlCtxtReadFile_rust(htmlParserCtxtPtr ctxt, const char *filename,
                                        const char *encoding, int options);

extern htmlDocPtr htmlCtxtReadMemory_rust(htmlParserCtxtPtr ctxt, const char *buffer, int size,
                                          const char *URL, const char *encoding, int options);

extern htmlDocPtr htmlCtxtReadFd_rust(htmlParserCtxtPtr ctxt, int fd,
                                      const char *URL, const char *encoding, int options);

extern htmlDocPtr htmlCtxtReadIO_rust(htmlParserCtxtPtr ctxt, xmlInputReadCallback ioread,
                                      xmlInputCloseCallback ioclose, void *ioctx,
                                      const char *URL,
                                      const char *encoding, int options);

extern int htmlParseTryOrFinish_rust(htmlParserCtxtPtr ctxt, int terminate);

extern int htmlParseLookupSequence_rust(htmlParserCtxtPtr ctxt, xmlChar first,
                        xmlChar next, xmlChar third, int ignoreattrval);

extern void htmlInitAutoClose_rust();

static int htmlOmittedDefaultValue = 1;

xmlChar *htmlDecodeEntities(htmlParserCtxtPtr ctxt, int len,
                            xmlChar end, xmlChar end2, xmlChar end3);

static void htmlParseComment(htmlParserCtxtPtr ctxt);

/************************************************************************
 *									*
 *		Some factorized error routines				*
 *									*
 ************************************************************************/



/************************************************************************
 *									*
 *	Parser stacks related functions and macros		*
 *									*
 ************************************************************************/




/************************************************************************
 *									*
 *	The list of HTML elements and their properties		*
 *									*
 ************************************************************************/



/************************************************************************
 *									*
 *	functions to handle HTML specific data			*
 *									*
 ************************************************************************/

/**
 * htmlInitAutoClose:
 *
 * This is a no-op now.
 */
void
htmlInitAutoClose(void) {
    htmlInitAutoClose_rust();
}


/**
 * htmlTagLookup:
 * @tag:  The tag name in lowercase
 *
 * Lookup the HTML tag in the ElementTable
 *
 * Returns the related htmlElemDescPtr or NULL if not found.
 */
const htmlElemDesc *
htmlTagLookup(const xmlChar *tag) {
    return htmlTagLookup_rust(tag);
}


/**
 * htmlAutoCloseTag:
 * @doc:  the HTML document
 * @name:  The tag name
 * @elem:  the HTML element
 *
 * The HTML DTD allows a tag to implicitly close other tags.
 * The list is kept in htmlStartClose array. This function checks
 * if the element or one of it's children would autoclose the
 * given tag.
 *
 * Returns 1 if autoclose, 0 otherwise
 */
int
htmlAutoCloseTag(htmlDocPtr doc, const xmlChar *name, htmlNodePtr elem) {
    return htmlAutoCloseTag_rust(doc, name, elem);
}


/**
 * htmlIsAutoClosed:
 * @doc:  the HTML document
 * @elem:  the HTML element
 *
 * The HTML DTD allows a tag to implicitly close other tags.
 * The list is kept in htmlStartClose array. This function checks
 * if a tag is autoclosed by one of it's child
 *
 * Returns 1 if autoclosed, 0 otherwise
 */
int
htmlIsAutoClosed(htmlDocPtr doc, htmlNodePtr elem) {
    return htmlIsAutoClosed_rust(doc, elem);
}



/**
 * htmlIsScriptAttribute:
 * @name:  an attribute name
 *
 * Check if an attribute is of content type Script
 *
 * Returns 1 is the attribute is a script 0 otherwise
 */
int
htmlIsScriptAttribute(const xmlChar *name) {
    return htmlIsScriptAttribute_rust(name);
}

/************************************************************************
 *									*
 *	The list of HTML predefined entities			*
 *									*
 ************************************************************************/



/************************************************************************
 *									*
 *		Commodity functions to handle entities			*
 *									*
 ************************************************************************/

/**
 * htmlEntityLookup:
 * @name: the entity name
 *
 * Lookup the given entity in EntitiesTable
 *
 * TODO: the linear scan is really ugly, an hash table is really needed.
 *
 * Returns the associated htmlEntityDescPtr if found, NULL otherwise.
 */
const htmlEntityDesc *
htmlEntityLookup(const xmlChar *name) {
    return htmlEntityLookup_rust(name);
}

/**
 * htmlEntityValueLookup:
 * @value: the entity's unicode value
 *
 * Lookup the given entity in EntitiesTable
 *
 * TODO: the linear scan is really ugly, an hash table is really needed.
 *
 * Returns the associated htmlEntityDescPtr if found, NULL otherwise.
 */
const htmlEntityDesc *
htmlEntityValueLookup(unsigned int value) {
    return htmlEntityValueLookup_rust(value);
}

/**
 * UTF8ToHtml:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of UTF-8 chars
 * @inlen:  the length of @in
 *
 * Take a block of UTF-8 chars in and try to convert it to an ASCII
 * plus HTML entities block of chars out.
 *
 * Returns 0 if success, -2 if the transcoding fails, or -1 otherwise
 * The value of @inlen after return is the number of octets consumed
 *     as the return value is positive, else unpredictable.
 * The value of @outlen after return is the number of octets consumed.
 */
int
UTF8ToHtml(unsigned char *out, int *outlen,
           const unsigned char *in, int *inlen) {
    return UTF8ToHtml_rust(out,outlen,in,inlen);
}

/**
 * htmlEncodeEntities:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of UTF-8 chars
 * @inlen:  the length of @in
 * @quoteChar: the quote character to escape (' or ") or zero.
 *
 * Take a block of UTF-8 chars in and try to convert it to an ASCII
 * plus HTML entities block of chars out.
 *
 * Returns 0 if success, -2 if the transcoding fails, or -1 otherwise
 * The value of @inlen after return is the number of octets consumed
 *     as the return value is positive, else unpredictable.
 * The value of @outlen after return is the number of octets consumed.
 */
int
htmlEncodeEntities(unsigned char *out, int *outlen,
                   const unsigned char *in, int *inlen, int quoteChar) {
    return htmlEncodeEntities_rust(out,outlen,in,inlen,quoteChar);
}

/************************************************************************
 *									*
 *		Commodity functions to handle streams			*
 *									*
 ************************************************************************/

#ifdef LIBXML_PUSH_ENABLED
#endif


/************************************************************************
 *									*
 *		Commodity functions, cleanup needed ?			*
 *									*
 ************************************************************************/

/**
 * htmlNewDocNoDtD:
 * @URI:  URI for the dtd, or NULL
 * @ExternalID:  the external ID of the DTD, or NULL
 *
 * Creates a new HTML document without a DTD node if @URI and @ExternalID
 * are NULL
 *
 * Returns a new document, do not initialize the DTD if not provided
 */
htmlDocPtr
htmlNewDocNoDtD(const xmlChar *URI, const xmlChar *ExternalID) {
    return htmlNewDocNoDtD_rust(URI,ExternalID);
}

/**
 * htmlNewDoc:
 * @URI:  URI for the dtd, or NULL
 * @ExternalID:  the external ID of the DTD, or NULL
 *
 * Creates a new HTML document
 *
 * Returns a new document
 */
htmlDocPtr
htmlNewDoc(const xmlChar *URI, const xmlChar *ExternalID) {
    return htmlNewDoc_rust(URI, ExternalID);
}


/************************************************************************
 *									*
 *			The parser itself				*
 *	Relates to http://www.w3.org/TR/html40				*
 *									*
 ************************************************************************/

/************************************************************************
 *									*
 *			The parser itself				*
 *									*
 ************************************************************************/

static const xmlChar *htmlParseNameComplex(xmlParserCtxtPtr ctxt);

/**
 * htmlParseEntityRef:
 * @ctxt:  an HTML parser context
 * @str:  location to store the entity name
 *
 * parse an HTML ENTITY references
 *
 * [68] EntityRef ::= '&' Name ';'
 *
 * Returns the associated htmlEntityDescPtr if found, or NULL otherwise,
 *         if non-NULL *str will have to be freed by the caller.
 */
const htmlEntityDesc *
htmlParseEntityRef(htmlParserCtxtPtr ctxt, const xmlChar **str) {
    return htmlParseEntityRef_rust(ctxt,str);
}

/**
 * htmlParseCharRef:
 * @ctxt:  an HTML parser context
 *
 * parse Reference declarations
 *
 * [66] CharRef ::= '&#' [0-9]+ ';' |
 *                  '&#x' [0-9a-fA-F]+ ';'
 *
 * Returns the value parsed (as an int)
 */
int
htmlParseCharRef(htmlParserCtxtPtr ctxt) {
    return htmlParseCharRef_rust(ctxt);
}

/**
 * htmlParseElement:
 * @ctxt:  an HTML parser context
 *
 * parse an HTML element, this is highly recursive
 * this is kept for compatibility with previous code versions
 *
 * [39] element ::= EmptyElemTag | STag content ETag
 *
 * [41] Attribute ::= Name Eq AttValue
 */

void
htmlParseElement(htmlParserCtxtPtr ctxt) {
    htmlParseElement_rust(ctxt);
}



/**
 * htmlParseContent:
 * @ctxt:  an HTML parser context
 *
 * Parse a content: comment, sub-element, reference or text.
 * This is the entry point when called from parser.c
 */

void
__htmlParseContent(void *ctxt) {
    __htmlParseContent_rust(ctxt);
}

/**
 * htmlParseDocument:
 * @ctxt:  an HTML parser context
 *
 * parse an HTML document (and build a tree if using the standard SAX
 * interface).
 *
 * Returns 0, -1 in case of error. the parser context is augmented
 *                as a result of the parsing.
 */

int
htmlParseDocument(htmlParserCtxtPtr ctxt) {
    return htmlParseDocument_rust(ctxt);
}


/************************************************************************
 *									*
 *			Parser contexts handling			*
 *									*
 ************************************************************************/

/**
 * htmlFreeParserCtxt:
 * @ctxt:  an HTML parser context
 *
 * Free all the memory used by a parser context. However the parsed
 * document in ctxt->myDoc is not freed.
 */

void
htmlFreeParserCtxt(htmlParserCtxtPtr ctxt) {
    htmlFreeParserCtxt_rust(ctxt);
}

/**
 * htmlNewParserCtxt:
 *
 * Allocate and initialize a new parser context.
 *
 * Returns the htmlParserCtxtPtr or NULL in case of allocation error
 */

htmlParserCtxtPtr
htmlNewParserCtxt(void) {
    return htmlNewParserCtxt_rust();
}

/**
 * htmlCreateMemoryParserCtxt:
 * @buffer:  a pointer to a char array
 * @size:  the size of the array
 *
 * Create a parser context for an HTML in-memory document.
 *
 * Returns the new parser context or NULL
 */
htmlParserCtxtPtr
htmlCreateMemoryParserCtxt(const char *buffer, int size) {
    return htmlCreateMemoryParserCtxt_rust(buffer, size);
}


#ifdef LIBXML_PUSH_ENABLED
                                                                                                                        /************************************************************************
 *									*
 *	Progressive parsing interfaces				*
 *									*
 ************************************************************************/

/**
 * htmlParseLookupSequence:
 * @ctxt:  an HTML parser context
 * @first:  the first char to lookup
 * @next:  the next char to lookup or zero
 * @third:  the next char to lookup or zero
 * @ignoreattrval: skip over attribute values
 *
 * Try to find if a sequence (first, next, third) or  just (first next) or
 * (first) is available in the input stream.
 * This function has a side effect of (possibly) incrementing ctxt->checkIndex
 * to avoid rescanning sequences of bytes, it DOES change the state of the
 * parser, do not use liberally.
 * This is basically similar to xmlParseLookupSequence()
 *
 * Returns the index to the current parsing point if the full sequence
 *      is available, -1 otherwise.
 */
int
htmlParseLookupSequence(htmlParserCtxtPtr ctxt, xmlChar first,
                        xmlChar next, xmlChar third, int ignoreattrval)
{
    return htmlParseLookupSequence_rust(ctxt,first,next,third,ignoreattrval);
}



/**
 * htmlParseTryOrFinish:
 * @ctxt:  an HTML parser context
 * @terminate:  last chunk indicator
 *
 * Try to progress on parsing
 *
 * Returns zero if no parsing was possible
 */
int
htmlParseTryOrFinish(htmlParserCtxtPtr ctxt, int terminate) {
    return htmlParseTryOrFinish_rust(ctxt,terminate);
}

/**
 * htmlParseChunk:
 * @ctxt:  an HTML parser context
 * @chunk:  an char array
 * @size:  the size in byte of the chunk
 * @terminate:  last chunk indicator
 *
 * Parse a Chunk of memory
 *
 * Returns zero if no error, the xmlParserErrors otherwise.
 */
int
htmlParseChunk(htmlParserCtxtPtr ctxt, const char *chunk, int size,
              int terminate) {
    return htmlParseChunk_rust(ctxt,chunk,size,terminate);
}

/************************************************************************
 *									*
 *			User entry points				*
 *									*
 ************************************************************************/

/**
 * htmlCreatePushParserCtxt:
 * @sax:  a SAX handler
 * @user_data:  The user data returned on SAX callbacks
 * @chunk:  a pointer to an array of chars
 * @size:  number of chars in the array
 * @filename:  an optional file name or URI
 * @enc:  an optional encoding
 *
 * Create a parser context for using the HTML parser in push mode
 * The value of @filename is used for fetching external entities
 * and error/warning reports.
 *
 * Returns the new parser context or NULL
 */
htmlParserCtxtPtr
htmlCreatePushParserCtxt(htmlSAXHandlerPtr sax, void *user_data,
                         const char *chunk, int size, const char *filename,
             xmlCharEncoding enc) {
    return htmlCreatePushParserCtxt_rust(sax,user_data,chunk,size,filename,enc);
}
#endif /* LIBXML_PUSH_ENABLED */

/**
 * htmlSAXParseDoc:
 * @cur:  a pointer to an array of xmlChar
 * @encoding:  a free form C string describing the HTML document encoding, or NULL
 * @sax:  the SAX handler block
 * @userData: if using SAX, this pointer will be provided on callbacks.
 *
 * Parse an HTML in-memory document. If sax is not NULL, use the SAX callbacks
 * to handle parse events. If sax is NULL, fallback to the default DOM
 * behavior and return a tree.
 *
 * Returns the resulting document tree unless SAX is NULL or the document is
 *     not well formed.
 */

htmlDocPtr
htmlSAXParseDoc(const xmlChar *cur, const char *encoding,
                htmlSAXHandlerPtr sax, void *userData) {
    return htmlSAXParseDoc_rust(cur, encoding, sax, userData);
}

/**
 * htmlParseDoc:
 * @cur:  a pointer to an array of xmlChar
 * @encoding:  a free form C string describing the HTML document encoding, or NULL
 *
 * parse an HTML in-memory document and build a tree.
 *
 * Returns the resulting document tree
 */

htmlDocPtr
htmlParseDoc(const xmlChar *cur, const char *encoding) {
    return htmlParseDoc_rust(cur, encoding);
}


/**
 * htmlCreateFileParserCtxt:
 * @filename:  the filename
 * @encoding:  a free form C string describing the HTML document encoding, or NULL
 *
 * Create a parser context for a file content.
 * Automatic support for ZLIB/Compress compressed document is provided
 * by default if found at compile-time.
 *
 * Returns the new parser context or NULL
 */
htmlParserCtxtPtr
htmlCreateFileParserCtxt(const char *filename, const char *encoding) {
    return htmlCreateFileParserCtxt_rust(filename,encoding);
}

/**
 * htmlSAXParseFile:
 * @filename:  the filename
 * @encoding:  a free form C string describing the HTML document encoding, or NULL
 * @sax:  the SAX handler block
 * @userData: if using SAX, this pointer will be provided on callbacks.
 *
 * parse an HTML file and build a tree. Automatic support for ZLIB/Compress
 * compressed document is provided by default if found at compile-time.
 * It use the given SAX function block to handle the parsing callback.
 * If sax is NULL, fallback to the default DOM tree building routines.
 *
 * Returns the resulting document tree unless SAX is NULL or the document is
 *     not well formed.
 */

htmlDocPtr
htmlSAXParseFile(const char *filename, const char *encoding, htmlSAXHandlerPtr sax,
                 void *userData) {
    return htmlSAXParseFile_rust(filename,encoding,sax,userData);
}

/**
 * htmlParseFile:
 * @filename:  the filename
 * @encoding:  a free form C string describing the HTML document encoding, or NULL
 *
 * parse an HTML file and build a tree. Automatic support for ZLIB/Compress
 * compressed document is provided by default if found at compile-time.
 *
 * Returns the resulting document tree
 */

htmlDocPtr
htmlParseFile(const char *filename, const char *encoding) {
    return htmlParseFile_rust(filename, encoding);
}

/**
 * htmlHandleOmittedElem:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for handling HTML omitted tags.
 *
 * Returns the last value for 0 for no handling, 1 for auto insertion.
 */

int
htmlHandleOmittedElem(int val) {
    return htmlHandleOmittedElem_rust(val);
}

/**
 * htmlElementAllowedHere:
 * @parent: HTML parent element
 * @elt: HTML element
 *
 * Checks whether an HTML element may be a direct child of a parent element.
 * Note - doesn't check for deprecated elements
 *
 * Returns 1 if allowed; 0 otherwise.
 */
int
htmlElementAllowedHere(const htmlElemDesc *parent, const xmlChar *elt) {
    return htmlElementAllowedHere_rust(parent, elt);
}

/**
 * htmlElementStatusHere:
 * @parent: HTML parent element
 * @elt: HTML element
 *
 * Checks whether an HTML element may be a direct child of a parent element.
 * and if so whether it is valid or deprecated.
 *
 * Returns one of HTML_VALID, HTML_DEPRECATED, HTML_INVALID
 */
htmlStatus
htmlElementStatusHere(const htmlElemDesc *parent, const htmlElemDesc *elt) {
    return htmlElementStatusHere_rust(parent, elt);
}

/**
 * htmlAttrAllowed:
 * @elt: HTML element
 * @attr: HTML attribute
 * @legacy: whether to allow deprecated attributes
 *
 * Checks whether an attribute is valid for an element
 * Has full knowledge of Required and Deprecated attributes
 *
 * Returns one of HTML_REQUIRED, HTML_VALID, HTML_DEPRECATED, HTML_INVALID
 */
htmlStatus
htmlAttrAllowed(const htmlElemDesc *elt, const xmlChar *attr, int legacy) {
    return htmlAttrAllowed_rust(elt, attr, legacy);
}

/**
 * htmlNodeStatus:
 * @node: an htmlNodePtr in a tree
 * @legacy: whether to allow deprecated elements (YES is faster here
 *	for Element nodes)
 *
 * Checks whether the tree node is valid.  Experimental (the author
 *     only uses the HTML enhancements in a SAX parser)
 *
 * Return: for Element nodes, a return from htmlElementAllowedHere (if
 *	legacy allowed) or htmlElementStatusHere (otherwise).
 *	for Attribute nodes, a return from htmlAttrAllowed
 *	for other nodes, HTML_NA (no checks performed)
 */
htmlStatus
htmlNodeStatus(const htmlNodePtr node, int legacy) {
    return htmlNodeStatus_rust(node, legacy);
}
/************************************************************************
 *									*
 *	New set (2.6.0) of simpler and more flexible APIs		*
 *									*
 ************************************************************************/

/**
 * htmlCtxtReset:
 * @ctxt: an HTML parser context
 *
 * Reset a parser context
 */
void
htmlCtxtReset(htmlParserCtxtPtr ctxt) {
    htmlCtxtReset_rust(ctxt);
}

/**
 * htmlCtxtUseOptions:
 * @ctxt: an HTML parser context
 * @options:  a combination of htmlParserOption(s)
 *
 * Applies the options to the parser context
 *
 * Returns 0 in case of success, the set of unknown or unimplemented options
 *         in case of error.
 */
int
htmlCtxtUseOptions(htmlParserCtxtPtr ctxt, int options) {
    return htmlCtxtUseOptions_rust(ctxt, options);
}


/**
 * htmlReadDoc:
 * @cur:  a pointer to a zero terminated string
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an XML in-memory document and build a tree.
 *
 * Returns the resulting document tree
 */
htmlDocPtr
htmlReadDoc(const xmlChar *cur, const char *URL, const char *encoding, int options) {
    return htmlReadDoc_rust(cur, URL, encoding, options);
}

/**
 * htmlReadFile:
 * @filename:  a file or URL
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an XML file from the filesystem or the network.
 *
 * Returns the resulting document tree
 */
htmlDocPtr
htmlReadFile(const char *filename, const char *encoding, int options) {
    return htmlReadFile_rust(filename,encoding,options);
}

/**
 * htmlReadMemory:
 * @buffer:  a pointer to a char array
 * @size:  the size of the array
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an XML in-memory document and build a tree.
 *
 * Returns the resulting document tree
 */
htmlDocPtr
htmlReadMemory(const char *buffer, int size, const char *URL, const char *encoding, int options) {
    return htmlReadMemory_rust(buffer, size, URL, encoding, options);
}

/**
 * htmlReadFd:
 * @fd:  an open file descriptor
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an XML from a file descriptor and build a tree.
 *
 * Returns the resulting document tree
 */
htmlDocPtr
htmlReadFd(int fd, const char *URL, const char *encoding, int options) {
    return htmlReadFd_rust(fd, URL, encoding, options);
}

/**
 * htmlReadIO:
 * @ioread:  an I/O read function
 * @ioclose:  an I/O close function
 * @ioctx:  an I/O handler
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an HTML document from I/O functions and source and build a tree.
 *
 * Returns the resulting document tree
 */
htmlDocPtr
htmlReadIO(xmlInputReadCallback ioread, xmlInputCloseCallback ioclose,
           void *ioctx, const char *URL, const char *encoding, int options) {
    return htmlReadIO_rust(ioread, ioclose, ioctx, URL, encoding, options);
}

/**
 * htmlCtxtReadDoc:
 * @ctxt:  an HTML parser context
 * @cur:  a pointer to a zero terminated string
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an XML in-memory document and build a tree.
 * This reuses the existing @ctxt parser context
 *
 * Returns the resulting document tree
 */
htmlDocPtr
htmlCtxtReadDoc(htmlParserCtxtPtr ctxt, const xmlChar *cur,
                const char *URL, const char *encoding, int options) {
    return htmlCtxtReadDoc_rust(ctxt, cur, URL, encoding, options);
}

/**
 * htmlCtxtReadFile:
 * @ctxt:  an HTML parser context
 * @filename:  a file or URL
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an XML file from the filesystem or the network.
 * This reuses the existing @ctxt parser context
 *
 * Returns the resulting document tree
 */
htmlDocPtr
htmlCtxtReadFile(htmlParserCtxtPtr ctxt, const char *filename,
                 const char *encoding, int options) {
    return htmlCtxtReadFile_rust(ctxt, filename, encoding, options);
}

/**
 * htmlCtxtReadMemory:
 * @ctxt:  an HTML parser context
 * @buffer:  a pointer to a char array
 * @size:  the size of the array
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an XML in-memory document and build a tree.
 * This reuses the existing @ctxt parser context
 *
 * Returns the resulting document tree
 */
htmlDocPtr
htmlCtxtReadMemory(htmlParserCtxtPtr ctxt, const char *buffer, int size,
                   const char *URL, const char *encoding, int options) {
    return htmlCtxtReadMemory_rust(ctxt, buffer, size, URL, encoding, options);
}

/**
 * htmlCtxtReadFd:
 * @ctxt:  an HTML parser context
 * @fd:  an open file descriptor
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an XML from a file descriptor and build a tree.
 * This reuses the existing @ctxt parser context
 *
 * Returns the resulting document tree
 */
htmlDocPtr
htmlCtxtReadFd(htmlParserCtxtPtr ctxt, int fd,
               const char *URL, const char *encoding, int options) {
    return htmlCtxtReadFd_rust(ctxt, fd, URL, encoding, options);
}

/**
 * htmlCtxtReadIO:
 * @ctxt:  an HTML parser context
 * @ioread:  an I/O read function
 * @ioclose:  an I/O close function
 * @ioctx:  an I/O handler
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an HTML document from I/O functions and source and build a tree.
 * This reuses the existing @ctxt parser context
 *
 * Returns the resulting document tree
 */
htmlDocPtr
htmlCtxtReadIO(htmlParserCtxtPtr ctxt, xmlInputReadCallback ioread,
               xmlInputCloseCallback ioclose, void *ioctx,
               const char *URL,
               const char *encoding, int options) {
    return htmlCtxtReadIO_rust(ctxt, ioread, ioclose, ioctx, URL, encoding, options);
}

#define bottom_HTMLparser
#include "elfgcchack.h"
#endif /* LIBXML_HTML_ENABLED */

#else  /* not COMPILE_WITH_RUST */

int get_libxml_push_enabled() {
#ifdef LIBXML_PUSH_ENABLED
    return 1;
#else
    return 0;
#endif
}


int get_libxml_sax1_enabled() {
#ifdef LIBXML_SAX1_ENABLED
    return 1;
#else
    return 0;
#endif
}

int get_libxml_iconv_enabled() {
#ifdef LIBXML_ICONV_ENABLED
    return 1;
#else
    return 0;
#endif
}

int get_libxml_icu_enabled() {
#ifdef LIBXML_ICU_ENABLED
    return 1;
#else
    return 0;
#endif
}

int get_libxml_regexp_enabled() {
#ifdef LIBXML_REGEXP_ENABLED
    return 1;
#else
    return 0;
#endif
}

int get_debug_push() {
#ifdef DEBUG_PUSH
    return 1;
#else
    return 0;
#endif
}

#endif /* COMPILE_WITH_RUST */
