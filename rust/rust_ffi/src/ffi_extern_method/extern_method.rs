use super::super::ffi_defination::defination::*;

extern "C" {

    pub fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> i32;

    pub fn __xmlRaiseError(
        schannel: xmlStructuredErrorFunc,
        channel: xmlGenericErrorFunc,
        data: *mut (),
        ctx: *mut (),
        node: *mut (),
        domain: i32,
        code: i32,
        level: xmlErrorLevel,
        file: *const i8,
        line: i32,
        str1: *const i8,
        str2: *const i8,
        str3: *const i8,
        int1: i32,
        col: i32,
        msg: *const i8,
        more_params: ...
    );

    pub static mut stderr: *mut _IO_FILE;

    pub fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;

    pub fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;

    pub fn xmlStrlen(str: *const xmlChar) -> i32;

    pub fn __xmlLoaderErr(ctx: *mut (), msg: *const i8, filename: *const i8);

    pub fn snprintf(arg1: *mut i8, arg2: u64, arg3: *const i8, more_params: ...) -> i32;

    pub fn memset(_: *mut (), _: i32, _: u64) -> *mut ();

    pub fn strcmp(_: *const i8, _: *const i8) -> i32;

    pub fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;

    pub fn xmlBufEnd(buf: xmlBufPtr) -> *mut xmlChar;

    pub fn xmlBufUse(buf: xmlBufPtr) -> size_t;

    pub fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;

    pub fn xmlDictCreate() -> xmlDictPtr;

    pub fn xmlDictSetLimit(dict: xmlDictPtr, limit: size_t) -> size_t;

    pub fn xmlDictFree(dict: xmlDictPtr);

    pub fn __xmlGenericError() -> *mut xmlGenericErrorFunc;

    pub fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);

    pub fn xmlHashDefaultDeallocator(entry: *mut (), name: *const xmlChar);

    pub fn xmlParserValidityError(ctx: *mut (), msg: *const i8, _: ...);

    pub fn xmlParserValidityWarning(ctx: *mut (), msg: *const i8, _: ...);

    pub fn xmlGetCharEncodingHandler(enc: xmlCharEncoding) -> xmlCharEncodingHandlerPtr;

    pub fn xmlCharEncCloseFunc(handler: *mut xmlCharEncodingHandler) -> i32;

    pub fn xmlParserInputBufferCreateFilename(
        URI: *const i8,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;

    pub fn xmlParserInputBufferRead(in_0: xmlParserInputBufferPtr, len: i32) -> i32;

    pub fn xmlParserInputBufferGrow(in_0: xmlParserInputBufferPtr, len: i32) -> i32;

    pub fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);

    pub fn xmlParserGetDirectory(filename: *const i8) -> *mut i8;

    pub fn xmlCheckHTTPInput(ctxt: xmlParserCtxtPtr, ret: xmlParserInputPtr) -> xmlParserInputPtr;

    pub fn __xmlSubstituteEntitiesDefaultValue() -> *mut i32;

    pub fn __xmlKeepBlanksDefaultValue() -> *mut i32;

    pub fn __xmlIndentTreeOutput() -> *mut i32;

    pub fn __xmlPedanticParserDefaultValue() -> *mut i32;

    pub fn __xmlLineNumbersDefaultValue() -> *mut i32;

    pub static mut xmlFree: xmlFreeFunc;

    pub fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;

    pub fn __xmlGetWarningsDefaultValue() -> *mut i32;

    pub fn xmlSAX2IgnorableWhitespace(ctx: *mut (), ch: *const xmlChar, len: i32);

    pub fn __xmlDoValidityCheckingDefaultValue() -> *mut i32;

    pub fn __xmlLoadExtDtdDefaultValue() -> *mut i32;

    pub fn __xmlGenericErrorContext() -> *mut *mut ();

    pub static mut xmlMalloc: xmlMallocFunc;

    pub fn xmlSAXVersion(hdlr: *mut xmlSAXHandler, version: i32) -> i32;

    pub fn xmlDefaultSAXHandlerInit();

    pub fn __xmlParserDebugEntities() -> *mut i32;

    pub static mut xmlRealloc: xmlReallocFunc;

    pub fn xmlLoadExternalEntity(
        URL: *const i8,
        ID: *const i8,
        ctxt: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr;

    pub fn xmlCharInRange(val: u32, group: *const xmlChRangeGroup) -> i32;

    pub static xmlIsBaseCharGroup: xmlChRangeGroup;

    pub fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;

    pub fn xmlCatalogFreeLocal(catalogs: *mut ());

    pub fn xmlBufCreate() -> xmlBufPtr;

    pub fn xmlBufIsEmpty(buf: xmlBufPtr) -> i32;

    pub fn xmlBufResetInput(buf: xmlBufPtr, input: xmlParserInputPtr) -> i32;

    pub fn xmlCharEncFirstLineInput(input: xmlParserInputBufferPtr, len: i32) -> i32;

    pub fn xmlCharEncInput(input: xmlParserInputBufferPtr, flush: i32) -> i32;

    pub fn xmlStrncmp(str1: *const xmlChar, str2: *const xmlChar, len: i32) -> i32;

    pub fn xmlStrcasecmp(str1: *const xmlChar, str2: *const xmlChar) -> i32;

    pub fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> i32;

    pub fn xmlStrcasestr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;

    pub fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;

    pub fn xmlCharStrdup(cur: *const i8) -> *mut xmlChar;

    pub fn xmlStrndup(cur: *const xmlChar, len: i32) -> *mut xmlChar;
    /*
     * internal function of HTML parser needed for xmlParseInNodeContext
     * but not part of the API
     */

    pub fn __htmlParseContent(ctx: *mut ());
    /*
     * internal global initialization critical section routines.
     */

    pub fn __xmlGlobalInitMutexLock();

    pub fn __xmlGlobalInitMutexUnlock();

    pub fn xmlInputReadCallbackNop(context: *mut (), buffer: *mut i8, len: i32) -> i32;

    pub fn memcpy(_: *mut (), _: *const (), _: u64) -> *mut ();

    pub fn memmove(_: *mut (), _: *const (), _: u64) -> *mut ();

    pub fn memchr(_: *const (), _: i32, _: u64) -> *mut ();

    pub fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;

    pub fn strlen(_: *const i8) -> u64;
    /* LIBXML_OUTPUT_ENABLED */
    /*
     * Interfaces for input
     */

    pub fn xmlCleanupInputCallbacks();

    pub fn xmlRegisterDefaultInputCallbacks();

    pub fn xmlAllocParserInputBuffer(enc: xmlCharEncoding) -> xmlParserInputBufferPtr;

    pub fn xmlParserInputBufferCreateFd(fd: i32, enc: xmlCharEncoding) -> xmlParserInputBufferPtr;

    pub fn xmlParserInputBufferCreateMem(
        mem: *const i8,
        size: i32,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;

    pub fn xmlParserInputBufferCreateIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut (),
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;

    pub fn xmlParserInputBufferPush(in_0: xmlParserInputBufferPtr, len: i32, buf: *const i8)
        -> i32;
    /*
     * Interfaces for output
     */

    pub fn xmlCleanupOutputCallbacks();

    pub fn xmlRegisterDefaultOutputCallbacks();

    pub fn xmlInitializeDict() -> i32;

    pub fn xmlDictReference(dict: xmlDictPtr) -> i32;
    /*
     * Lookup of entry in the dictionary.
     */

    pub fn xmlDictLookup(dict: xmlDictPtr, name: *const xmlChar, len: i32) -> *const xmlChar;

    pub fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> i32;
    /*
     * Cleanup function
     */

    pub fn xmlDictCleanup();

    pub fn xmlBuildQName(
        ncname: *const xmlChar,
        prefix: *const xmlChar,
        memory: *mut xmlChar,
        len: i32,
    ) -> *mut xmlChar;

    pub fn xmlSplitQName3(name: *const xmlChar, len: *mut i32) -> *const xmlChar;

    pub fn xmlBufferCreate() -> xmlBufferPtr;

    pub fn xmlBufferFree(buf: xmlBufferPtr);

    pub fn xmlBufferAdd(buf: xmlBufferPtr, str: *const xmlChar, len: i32) -> i32;
    /*
     * Creating/freeing new structures.
     */

    pub fn xmlCreateIntSubset(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;

    pub fn xmlNewDtd(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;

    pub fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;

    pub fn xmlFreeDoc(cur: xmlDocPtr);
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
    /*
     * Creating new nodes.
     */

    pub fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;

    pub fn xmlNewComment(content: *const xmlChar) -> xmlNodePtr;

    pub fn xmlDocCopyNode(node: xmlNodePtr, doc: xmlDocPtr, recursive: i32) -> xmlNodePtr;

    pub fn xmlGetLastChild(parent: *const xmlNode) -> xmlNodePtr;

    pub fn xmlNodeIsText(node: *const xmlNode) -> i32;
    /* LIBXML_TREE_ENABLED */

    pub fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;

    pub fn xmlAddChildList(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;

    pub fn xmlUnlinkNode(cur: xmlNodePtr);

    pub fn xmlFreeNodeList(cur: xmlNodePtr);

    pub fn xmlFreeNode(cur: xmlNodePtr);

    pub fn xmlSetTreeDoc(tree: xmlNodePtr, doc: xmlDocPtr);

    pub fn xmlSearchNsByHref(doc: xmlDocPtr, node: xmlNodePtr, href: *const xmlChar) -> xmlNsPtr;

    pub fn xmlHashCreateDict(size: i32, dict: xmlDictPtr) -> xmlHashTablePtr;

    pub fn xmlHashAddEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut (),
    ) -> i32;

    pub fn xmlHashUpdateEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut (),
        f: xmlHashDeallocator,
    ) -> i32;

    pub fn xmlHashRemoveEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        f: xmlHashDeallocator,
    ) -> i32;

    pub fn xmlHashLookup2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
    ) -> *mut ();

    pub fn xmlHashQLookup2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        prefix: *const xmlChar,
        name2: *const xmlChar,
        prefix2: *const xmlChar,
    ) -> *mut ();

    pub fn xmlHashSize(table: xmlHashTablePtr) -> i32;

    pub fn xmlHashScanFull(table: xmlHashTablePtr, f: xmlHashScannerFull, data: *mut ());

    pub fn initGenericErrorDefaultFunc(handler: *mut xmlGenericErrorFunc);

    pub fn xmlResetLastError();

    pub fn xmlResetError(err: xmlErrorPtr);

    pub fn xmlCopyError(from: xmlErrorPtr, to: xmlErrorPtr) -> i32;

    pub fn xmlNewDocElementContent(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_0: xmlElementContentType,
    ) -> xmlElementContentPtr;

    pub fn xmlFreeDocElementContent(doc: xmlDocPtr, cur: xmlElementContentPtr);

    pub fn xmlCreateEnumeration(name: *const xmlChar) -> xmlEnumerationPtr;

    pub fn xmlFreeEnumeration(cur: xmlEnumerationPtr);

    pub fn xmlValidateRoot(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> i32;

    pub fn xmlValidateElement(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, elem: xmlNodePtr) -> i32;

    pub fn xmlIsMixedElement(doc: xmlDocPtr, name: *const xmlChar) -> i32;

    pub fn xmlGetPredefinedEntity(name: *const xmlChar) -> xmlEntityPtr;

    pub fn xmlInitCharEncodingHandlers();

    pub fn xmlCleanupCharEncodingHandlers();

    pub fn xmlFindCharEncodingHandler(name: *const i8) -> xmlCharEncodingHandlerPtr;
    /*
     * Interfaces directly used by the parsers.
     */

    pub fn xmlDetectCharEncoding(in_0: *const u8, len: i32) -> xmlCharEncoding;

    pub fn xmlCleanupMemory();

    pub fn xmlInitMemory() -> i32;

    pub fn htmlDefaultSAXHandlerInit();

    pub fn xmlInitGlobals();
    /*
     * Library wide APIs.
     */

    pub fn xmlInitThreads();

    pub fn xmlCleanupThreads();

    pub fn xmlCleanupGlobals();

    pub static mut xmlMallocAtomic: xmlMallocFunc;

    pub fn xmlSAX2GetEntity(ctx: *mut (), name: *const xmlChar) -> xmlEntityPtr;

    pub fn xmlSAX2StartElement(ctx: *mut (), fullname: *const xmlChar, atts: *mut *const xmlChar);

    pub fn xmlSAX2EndElement(ctx: *mut (), name: *const xmlChar);

    pub fn __xmlDefaultSAXLocator() -> *mut xmlSAXLocator;

    pub fn xmlSAX2EntityDecl(
        ctx: *mut (),
        name: *const xmlChar,
        type_0: i32,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        content: *mut xmlChar,
    );

    pub static xmlIsCombiningGroup: xmlChRangeGroup;

    pub static xmlIsDigitGroup: xmlChRangeGroup;

    pub static xmlIsExtenderGroup: xmlChRangeGroup;

    pub fn htmlCreateMemoryParserCtxt(buffer: *const i8, size: i32) -> htmlParserCtxtPtr;

    pub static xmlIsPubidChar_tab: [u8; 256];

    pub fn htmlInitAutoClose();

    pub fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;

    pub fn xmlParseURI(str: *const i8) -> xmlURIPtr;

    pub fn xmlFreeURI(uri: xmlURIPtr);

    pub fn xmlCatalogCleanup();
    /*
     * Strictly minimal interfaces for per-document catalogs used
     * by the parser.
     */

    pub fn xmlCatalogAddLocal(catalogs: *mut (), URL: *const xmlChar) -> *mut ();

    pub fn xmlCatalogGetDefaults() -> xmlCatalogAllow;

    pub fn xmlSchemaCleanupTypes();

    pub fn xmlRelaxNGCleanupTypes();
    /* size_t xmlBufUse(const xmlBufPtr buf); */

    pub fn xmlBufGetInputBase(buf: xmlBufPtr, input: xmlParserInputPtr) -> size_t;

    pub fn xmlBufSetInputBaseCur(
        buf: xmlBufPtr,
        input: xmlParserInputPtr,
        base: size_t,
        cur: size_t,
    ) -> i32;
    /* LIBXML_XPATH_ENABLED */

    pub fn xmlXPathInit();
    /* LIBXML_LEGACY_ENABLED */
    /* ***********************************************************************
     *									*
     *				Miscellaneous				*
     *									*
     ************************************************************************/

    pub fn xmlGenericErrorDefaultFunc(ctx: *mut (), msg: *const i8, _: ...);
    //
    //  pub fn xmlStopParser(ctxt: xmlParserCtxtPtr);
    //
    //  pub fn xmlInitParser();
    //
    //  pub fn inputPop(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    //
    //  pub fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);

    pub fn xmlParserInputGrow(_: xmlParserInputPtr, _: i32) -> i32;

    pub fn xmlGetIntSubset(_: *const xmlDoc) -> xmlDtdPtr;

    pub fn xmlSwitchEncoding(_: xmlParserCtxtPtr, _: xmlCharEncoding) -> i32;

    pub fn xmlSwitchToEncoding(_: xmlParserCtxtPtr, _: xmlCharEncodingHandlerPtr) -> i32;

    pub fn xmlParserInputShrink(_: xmlParserInputPtr);

    pub fn xmlCopyChar(_: i32, _: *mut xmlChar, _: i32) -> i32;

    pub fn xmlNextChar(_: xmlParserCtxtPtr);

    pub fn xmlParserAddNodeInfo(_: xmlParserCtxtPtr, _: xmlParserNodeInfoPtr);

    pub fn nodePop(_: xmlParserCtxtPtr) -> xmlNodePtr;

    pub fn xmlParseCharEncoding(_: *const i8) -> xmlCharEncoding;

    pub fn xmlPopInput(_: xmlParserCtxtPtr) -> xmlChar;

    pub fn xmlStrncasecmp(_: *const xmlChar, _: *const xmlChar, _: i32) -> i32;

    pub fn __htmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;

    pub fn inputPush(_: xmlParserCtxtPtr, _: xmlParserInputPtr) -> i32;

    pub fn xmlFreeParserCtxt(_: xmlParserCtxtPtr);

    pub fn xmlInitNodeInfoSeq(_: xmlParserNodeInfoSeqPtr);

    pub fn xmlNewInputStream(_: xmlParserCtxtPtr) -> xmlParserInputPtr;

    pub fn xmlInitParser();

    pub fn xmlNewStringInputStream(_: xmlParserCtxtPtr, _: *const xmlChar) -> xmlParserInputPtr;

    pub fn xmlFreeInputStream(_: xmlParserInputPtr);

    pub fn inputPop(_: xmlParserCtxtPtr) -> xmlParserInputPtr;

    pub fn xmlNewIOInputStream(
        _: xmlParserCtxtPtr,
        _: xmlParserInputBufferPtr,
        _: xmlCharEncoding,
    ) -> xmlParserInputPtr;

    pub fn xmlNewParserCtxt() -> xmlParserCtxtPtr;

    pub fn xmlCreateMemoryParserCtxt(_: *const i8, _: i32) -> xmlParserCtxtPtr;

    pub fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;

    pub fn strcat(_: *mut i8, _: *const i8) -> *mut i8;

    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;

    pub fn xmlStopParser(ctxt: xmlParserCtxtPtr);

    pub fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);

    pub fn xmlStrPrintf(buf: *mut xmlChar, len: i32, msg: *const i8, _: ...) -> i32;

    pub fn xmlUTF8Strsize(utf: *const xmlChar, len: i32) -> i32;

    pub fn xmlUTF8Strpos(utf: *const xmlChar, pos: i32) -> *const xmlChar;

    pub fn xmlUTF8Strloc(utf: *const xmlChar, utfchar: *const xmlChar) -> i32;

    pub fn xmlUTF8Strsub(utf: *const xmlChar, start: i32, len: i32) -> *mut xmlChar;

    pub fn xmlUTF8Strlen(utf: *const xmlChar) -> i32;

    pub fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;

    pub fn xmlStrstr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;

    pub fn log10(_: libc::c_double) -> libc::c_double;

    pub fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;

    pub fn ceil(_: libc::c_double) -> libc::c_double;

    pub fn fabs(_: libc::c_double) -> libc::c_double;

    pub fn floor(_: libc::c_double) -> libc::c_double;

    pub fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;

    pub fn __isinff(__value: libc::c_float) -> i32;

    pub fn __isnanf(__value: libc::c_float) -> i32;

    pub fn __isinfl(__value: libc::c_float) -> i32;

    pub fn __isnanl(__value: libc::c_float) -> i32;

    pub fn __isinf(__value: libc::c_double) -> i32;

    pub fn __isnan(__value: libc::c_double) -> i32;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */

    pub fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;

    pub fn xmlGetNsList(doc: *const xmlDoc, node: *const xmlNode) -> *mut xmlNsPtr;

    pub fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;

    pub fn xmlNodeGetLang(cur: *const xmlNode) -> *mut xmlChar;

    pub fn xmlHashCreate(size: i32) -> xmlHashTablePtr;

    pub fn xmlHashAddEntry(table: xmlHashTablePtr, name: *const xmlChar, userdata: *mut ()) -> i32;

    pub fn xmlHashUpdateEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut (),
        f: xmlHashDeallocator,
    ) -> i32;
    /*
     * Remove an entry from the hash table.
     */

    pub fn xmlHashRemoveEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        f: xmlHashDeallocator,
    ) -> i32;
    /*
     * Retrieve the userdata.
     */

    pub fn xmlHashLookup(table: xmlHashTablePtr, name: *const xmlChar) -> *mut ();

    pub fn xmlGetID(doc: xmlDocPtr, ID: *const xmlChar) -> xmlAttrPtr;

    pub fn realloc(_: *mut (), _: u64) -> *mut ();

    pub fn free(__ptr: *mut ());

    pub fn exit(_: i32) -> !;
    /* array of locations */
    /*
     * Handling of location sets.
     */

    pub fn xmlXPtrLocationSetCreate(val: xmlXPathObjectPtr) -> xmlLocationSetPtr;

    pub fn xmlXPtrFreeLocationSet(obj: xmlLocationSetPtr);

    pub fn xmlXPtrLocationSetMerge(
        val1: xmlLocationSetPtr,
        val2: xmlLocationSetPtr,
    ) -> xmlLocationSetPtr;

    pub fn xmlXPtrNewRange(
        start: xmlNodePtr,
        startindex: i32,
        end: xmlNodePtr,
        endindex: i32,
    ) -> xmlXPathObjectPtr;

    pub fn xmlXPtrNewRangeNodeObject(
        start: xmlNodePtr,
        end: xmlXPathObjectPtr,
    ) -> xmlXPathObjectPtr;

    pub fn xmlXPtrLocationSetAdd(cur: xmlLocationSetPtr, val: xmlXPathObjectPtr);

    pub fn xmlXPtrWrapLocationSet(val: xmlLocationSetPtr) -> xmlXPathObjectPtr;

    pub fn xmlDebugDumpString(output: *mut FILE, str: *const xmlChar);

    pub fn xmlDebugDumpAttr(output: *mut FILE, attr: xmlAttrPtr, depth: i32);

    pub fn xmlDebugDumpOneNode(output: *mut FILE, node: xmlNodePtr, depth: i32);

    pub fn xmlFreePattern(comp: xmlPatternPtr);

    pub fn xmlFreePatternList(comp: xmlPatternPtr);

    pub fn xmlPatterncompile(
        pattern: *const xmlChar,
        dict: *mut xmlDict,
        flags: i32,
        namespaces: *mut *const xmlChar,
    ) -> xmlPatternPtr;

    pub fn xmlPatternStreamable(comp: xmlPatternPtr) -> i32;

    pub fn xmlPatternMaxDepth(comp: xmlPatternPtr) -> i32;

    pub fn xmlPatternMinDepth(comp: xmlPatternPtr) -> i32;

    pub fn xmlPatternFromRoot(comp: xmlPatternPtr) -> i32;

    pub fn xmlPatternGetStreamCtxt(comp: xmlPatternPtr) -> xmlStreamCtxtPtr;

    pub fn xmlFreeStreamCtxt(stream: xmlStreamCtxtPtr);

    pub fn xmlStreamPushNode(
        stream: xmlStreamCtxtPtr,
        name: *const xmlChar,
        ns: *const xmlChar,
        nodeType: i32,
    ) -> i32;

    pub fn xmlStreamPush(stream: xmlStreamCtxtPtr, name: *const xmlChar, ns: *const xmlChar)
        -> i32;

    pub fn xmlStreamPop(stream: xmlStreamCtxtPtr) -> i32;

    pub fn xmlStreamWantsAnyNode(stream: xmlStreamCtxtPtr) -> i32;

    pub fn xmlBufFree(buf: xmlBufPtr);

    pub fn xmlBufAdd(buf: xmlBufPtr, str: *const xmlChar, len: i32) -> i32;
}
