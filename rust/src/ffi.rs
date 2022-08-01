#[link(name = "xml2")]
extern "C" {
    #[no_mangle]
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    #[no_mangle]
    fn __xmlRaiseError(
        schannel: xmlStructuredErrorFunc,
        channel: xmlGenericErrorFunc,
        data: *mut libc::c_void,
        ctx: *mut libc::c_void,
        node: *mut libc::c_void,
        domain: libc::c_int,
        code: libc::c_int,
        level: xmlErrorLevel,
        file: *const libc::c_char,
        line: libc::c_int,
        str1: *const libc::c_char,
        str2: *const libc::c_char,
        str3: *const libc::c_char,
        int1: libc::c_int,
        col: libc::c_int,
        msg: *const libc::c_char,
        more_params: ...
    );
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> libc::c_int;
    #[no_mangle]
    fn __xmlLoaderErr(
        ctx: *mut libc::c_void,
        msg: *const libc::c_char,
        filename: *const libc::c_char,
    );
    #[no_mangle]
    fn snprintf(
        arg1: *mut libc::c_char,
        arg2: libc::c_ulong,
        arg3: *const libc::c_char,
        more_params: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufEnd(buf: xmlBufPtr) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufUse(buf: xmlBufPtr) -> size_t;
    #[no_mangle]
    fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;
    #[no_mangle]
    fn xmlDictCreate() -> xmlDictPtr;
    #[no_mangle]
    fn xmlDictSetLimit(dict: xmlDictPtr, limit: size_t) -> size_t;
    #[no_mangle]
    fn xmlDictFree(dict: xmlDictPtr);
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);
    #[no_mangle]
    fn xmlHashDefaultDeallocator(entry: *mut libc::c_void, name: *const xmlChar);
    #[no_mangle]
    fn xmlParserValidityError(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    #[no_mangle]
    fn xmlParserValidityWarning(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    #[no_mangle]
    fn xmlGetCharEncodingHandler(enc: xmlCharEncoding) -> xmlCharEncodingHandlerPtr;
    #[no_mangle]
    fn xmlCharEncCloseFunc(handler: *mut xmlCharEncodingHandler) -> libc::c_int;
    #[no_mangle]
    fn xmlParserInputBufferCreateFilename(
        URI: *const libc::c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferRead(in_0: xmlParserInputBufferPtr, len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn xmlParserInputBufferGrow(in_0: xmlParserInputBufferPtr, len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    #[no_mangle]
    fn xmlParserGetDirectory(filename: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xmlCheckHTTPInput(ctxt: xmlParserCtxtPtr, ret: xmlParserInputPtr) -> xmlParserInputPtr;
    #[no_mangle]
    fn __xmlSubstituteEntitiesDefaultValue() -> *mut libc::c_int;
    #[no_mangle]
    fn __xmlKeepBlanksDefaultValue() -> *mut libc::c_int;
    #[no_mangle]
    fn __xmlIndentTreeOutput() -> *mut libc::c_int;
    #[no_mangle]
    fn __xmlPedanticParserDefaultValue() -> *mut libc::c_int;
    #[no_mangle]
    fn __xmlLineNumbersDefaultValue() -> *mut libc::c_int;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
    #[no_mangle]
    fn __xmlGetWarningsDefaultValue() -> *mut libc::c_int;
    #[no_mangle]
    fn xmlSAX2IgnorableWhitespace(ctx: *mut libc::c_void, ch: *const xmlChar, len: libc::c_int);
    #[no_mangle]
    fn __xmlDoValidityCheckingDefaultValue() -> *mut libc::c_int;
    #[no_mangle]
    fn __xmlLoadExtDtdDefaultValue() -> *mut libc::c_int;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut libc::c_void;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    fn xmlSAXVersion(hdlr: *mut xmlSAXHandler, version: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn xmlDefaultSAXHandlerInit();
    #[no_mangle]
    fn __xmlParserDebugEntities() -> *mut libc::c_int;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    fn xmlLoadExternalEntity(
        URL: *const libc::c_char,
        ID: *const libc::c_char,
        ctxt: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlCharInRange(val: libc::c_uint, group: *const xmlChRangeGroup) -> libc::c_int;
    #[no_mangle]
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    #[no_mangle]
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlCatalogFreeLocal(catalogs: *mut libc::c_void);
    #[no_mangle]
    fn xmlBufCreate() -> xmlBufPtr;
    #[no_mangle]
    fn xmlBufIsEmpty(buf: xmlBufPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlBufResetInput(buf: xmlBufPtr, input: xmlParserInputPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlCharEncFirstLineInput(input: xmlParserInputBufferPtr, len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn xmlCharEncInput(input: xmlParserInputBufferPtr, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn xmlStrncmp(str1: *const xmlChar, str2: *const xmlChar, len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn xmlStrcasecmp(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    #[no_mangle]
    fn xmlStrcasestr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlCharStrdup(cur: *const libc::c_char) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrndup(cur: *const xmlChar, len: libc::c_int) -> *mut xmlChar;
    /*
     * internal function of HTML parser needed for xmlParseInNodeContext
     * but not part of the API
     */
    #[no_mangle]
    fn __htmlParseContent(ctx: *mut libc::c_void);
    /*
     * internal global initialization critical section routines.
     */
    #[no_mangle]
    fn __xmlGlobalInitMutexLock();
    #[no_mangle]
    fn __xmlGlobalInitMutexUnlock();
    #[no_mangle]
    fn xmlInputReadCallbackNop(
        context: *mut libc::c_void,
        buffer: *mut libc::c_char,
        len: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* LIBXML_OUTPUT_ENABLED */
    /*
     * Interfaces for input
     */
    #[no_mangle]
    fn xmlCleanupInputCallbacks();
    #[no_mangle]
    fn xmlRegisterDefaultInputCallbacks();
    #[no_mangle]
    fn xmlAllocParserInputBuffer(enc: xmlCharEncoding) -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferCreateFd(
        fd: libc::c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferCreateMem(
        mem: *const libc::c_char,
        size: libc::c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferCreateIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut libc::c_void,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferPush(
        in_0: xmlParserInputBufferPtr,
        len: libc::c_int,
        buf: *const libc::c_char,
    ) -> libc::c_int;
    /*
     * Interfaces for output
     */
    #[no_mangle]
    fn xmlCleanupOutputCallbacks();
    #[no_mangle]
    fn xmlRegisterDefaultOutputCallbacks();
    #[no_mangle]
    fn xmlInitializeDict() -> libc::c_int;
    #[no_mangle]
    fn xmlDictReference(dict: xmlDictPtr) -> libc::c_int;
    /*
     * Lookup of entry in the dictionary.
     */
    #[no_mangle]
    fn xmlDictLookup(dict: xmlDictPtr, name: *const xmlChar, len: libc::c_int) -> *const xmlChar;
    #[no_mangle]
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> libc::c_int;
    /*
     * Cleanup function
     */
    #[no_mangle]
    fn xmlDictCleanup();
    #[no_mangle]
    fn xmlBuildQName(
        ncname: *const xmlChar,
        prefix: *const xmlChar,
        memory: *mut xmlChar,
        len: libc::c_int,
    ) -> *mut xmlChar;
    #[no_mangle]
    fn xmlSplitQName3(name: *const xmlChar, len: *mut libc::c_int) -> *const xmlChar;
    #[no_mangle]
    fn xmlBufferCreate() -> xmlBufferPtr;
    #[no_mangle]
    fn xmlBufferFree(buf: xmlBufferPtr);
    #[no_mangle]
    fn xmlBufferAdd(buf: xmlBufferPtr, str: *const xmlChar, len: libc::c_int) -> libc::c_int;
    /*
     * Creating/freeing new structures.
     */
    #[no_mangle]
    fn xmlCreateIntSubset(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
    #[no_mangle]
    fn xmlNewDtd(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
    #[no_mangle]
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
    /*
     * Creating new nodes.
     */
    #[no_mangle]
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewComment(content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlDocCopyNode(node: xmlNodePtr, doc: xmlDocPtr, recursive: libc::c_int) -> xmlNodePtr;
    #[no_mangle]
    fn xmlGetLastChild(parent: *const xmlNode) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNodeIsText(node: *const xmlNode) -> libc::c_int;
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlAddChildList(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlUnlinkNode(cur: xmlNodePtr);
    #[no_mangle]
    fn xmlFreeNodeList(cur: xmlNodePtr);
    #[no_mangle]
    fn xmlFreeNode(cur: xmlNodePtr);
    #[no_mangle]
    fn xmlSetTreeDoc(tree: xmlNodePtr, doc: xmlDocPtr);
    #[no_mangle]
    fn xmlSearchNsByHref(doc: xmlDocPtr, node: xmlNodePtr, href: *const xmlChar) -> xmlNsPtr;
    #[no_mangle]
    fn xmlHashCreateDict(size: libc::c_int, dict: xmlDictPtr) -> xmlHashTablePtr;
    #[no_mangle]
    fn xmlHashAddEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    fn xmlHashUpdateEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut libc::c_void,
        f: xmlHashDeallocator,
    ) -> libc::c_int;
    #[no_mangle]
    fn xmlHashRemoveEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        f: xmlHashDeallocator,
    ) -> libc::c_int;
    #[no_mangle]
    fn xmlHashLookup2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn xmlHashQLookup2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        prefix: *const xmlChar,
        name2: *const xmlChar,
        prefix2: *const xmlChar,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn xmlHashSize(table: xmlHashTablePtr) -> libc::c_int;
    #[no_mangle]
    fn xmlHashScanFull(table: xmlHashTablePtr, f: xmlHashScannerFull, data: *mut libc::c_void);
    #[no_mangle]
    fn initGenericErrorDefaultFunc(handler: *mut xmlGenericErrorFunc);
    #[no_mangle]
    fn xmlResetLastError();
    #[no_mangle]
    fn xmlResetError(err: xmlErrorPtr);
    #[no_mangle]
    fn xmlCopyError(from: xmlErrorPtr, to: xmlErrorPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlNewDocElementContent(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_0: xmlElementContentType,
    ) -> xmlElementContentPtr;
    #[no_mangle]
    fn xmlFreeDocElementContent(doc: xmlDocPtr, cur: xmlElementContentPtr);
    #[no_mangle]
    fn xmlCreateEnumeration(name: *const xmlChar) -> xmlEnumerationPtr;
    #[no_mangle]
    fn xmlFreeEnumeration(cur: xmlEnumerationPtr);
    #[no_mangle]
    fn xmlValidateRoot(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlValidateElement(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, elem: xmlNodePtr) -> libc::c_int;
    #[no_mangle]
    fn xmlIsMixedElement(doc: xmlDocPtr, name: *const xmlChar) -> libc::c_int;
    #[no_mangle]
    fn xmlGetPredefinedEntity(name: *const xmlChar) -> xmlEntityPtr;
    #[no_mangle]
    fn xmlInitCharEncodingHandlers();
    #[no_mangle]
    fn xmlCleanupCharEncodingHandlers();
    #[no_mangle]
    fn xmlFindCharEncodingHandler(name: *const libc::c_char) -> xmlCharEncodingHandlerPtr;
    /*
     * Interfaces directly used by the parsers.
     */
    #[no_mangle]
    fn xmlDetectCharEncoding(in_0: *const libc::c_uchar, len: libc::c_int) -> xmlCharEncoding;
    #[no_mangle]
    fn xmlCleanupMemory();
    #[no_mangle]
    fn xmlInitMemory() -> libc::c_int;
    #[no_mangle]
    fn htmlDefaultSAXHandlerInit();
    #[no_mangle]
    fn xmlInitGlobals();
    /*
     * Library wide APIs.
     */
    #[no_mangle]
    fn xmlInitThreads();
    #[no_mangle]
    fn xmlCleanupThreads();
    #[no_mangle]
    fn xmlCleanupGlobals();
    #[no_mangle]
    static mut xmlMallocAtomic: xmlMallocFunc;
    #[no_mangle]
    fn xmlSAX2GetEntity(ctx: *mut libc::c_void, name: *const xmlChar) -> xmlEntityPtr;
    #[no_mangle]
    fn xmlSAX2StartElement(
        ctx: *mut libc::c_void,
        fullname: *const xmlChar,
        atts: *mut *const xmlChar,
    );
    #[no_mangle]
    fn xmlSAX2EndElement(ctx: *mut libc::c_void, name: *const xmlChar);
    #[no_mangle]
    fn __xmlDefaultSAXLocator() -> *mut xmlSAXLocator;
    #[no_mangle]
    fn xmlSAX2EntityDecl(
        ctx: *mut libc::c_void,
        name: *const xmlChar,
        type_0: libc::c_int,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        content: *mut xmlChar,
    );
    #[no_mangle]
    static xmlIsCombiningGroup: xmlChRangeGroup;
    #[no_mangle]
    static xmlIsDigitGroup: xmlChRangeGroup;
    #[no_mangle]
    static xmlIsExtenderGroup: xmlChRangeGroup;
    #[no_mangle]
    fn htmlCreateMemoryParserCtxt(
        buffer: *const libc::c_char,
        size: libc::c_int,
    ) -> htmlParserCtxtPtr;
    #[no_mangle]
    static xmlIsPubidChar_tab: [libc::c_uchar; 256];
    #[no_mangle]
    fn htmlInitAutoClose();
    #[no_mangle]
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlParseURI(str: *const libc::c_char) -> xmlURIPtr;
    #[no_mangle]
    fn xmlFreeURI(uri: xmlURIPtr);
    #[no_mangle]
    fn xmlCatalogCleanup();
    /*
     * Strictly minimal interfaces for per-document catalogs used
     * by the parser.
     */
    #[no_mangle]
    fn xmlCatalogAddLocal(catalogs: *mut libc::c_void, URL: *const xmlChar) -> *mut libc::c_void;
    #[no_mangle]
    fn xmlCatalogGetDefaults() -> xmlCatalogAllow;
    #[no_mangle]
    fn xmlSchemaCleanupTypes();
    #[no_mangle]
    fn xmlRelaxNGCleanupTypes();
    /* size_t xmlBufUse(const xmlBufPtr buf); */
    #[no_mangle]
    fn xmlBufGetInputBase(buf: xmlBufPtr, input: xmlParserInputPtr) -> size_t;
    #[no_mangle]
    fn xmlBufSetInputBaseCur(
        buf: xmlBufPtr,
        input: xmlParserInputPtr,
        base: size_t,
        cur: size_t,
    ) -> libc::c_int;
    /* LIBXML_XPATH_ENABLED */
    #[no_mangle]
    fn xmlXPathInit();
    /* LIBXML_LEGACY_ENABLED */
    /* ***********************************************************************
     *									*
     *				Miscellaneous				*
     *									*
     ************************************************************************/
    #[no_mangle]
    fn xmlGenericErrorDefaultFunc(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    //  #[no_mangle]
    //  fn xmlStopParser(ctxt: xmlParserCtxtPtr);
    //  #[no_mangle]
    //  fn xmlInitParser();
    //  #[no_mangle]
    //  fn inputPop(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    //  #[no_mangle]
    //  fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);

    #[no_mangle]
    fn xmlParserInputGrow(_: xmlParserInputPtr, _: libc::c_int) -> libc::c_int;

    #[no_mangle]
    fn xmlGetIntSubset(_: *const xmlDoc) -> xmlDtdPtr;
    #[no_mangle]
    fn xmlSwitchEncoding(_: xmlParserCtxtPtr, _: xmlCharEncoding) -> libc::c_int;
    #[no_mangle]
    fn xmlSwitchToEncoding(_: xmlParserCtxtPtr, _: xmlCharEncodingHandlerPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlParserInputShrink(_: xmlParserInputPtr);
    #[no_mangle]
    fn xmlCopyChar(_: libc::c_int, _: *mut xmlChar, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn xmlNextChar(_: xmlParserCtxtPtr);
    #[no_mangle]
    fn xmlParserAddNodeInfo(_: xmlParserCtxtPtr, _: xmlParserNodeInfoPtr);
    #[no_mangle]
    fn nodePop(_: xmlParserCtxtPtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlParseCharEncoding(_: *const libc::c_char) -> xmlCharEncoding;
    #[no_mangle]
    fn xmlPopInput(_: xmlParserCtxtPtr) -> xmlChar;
    #[no_mangle]
    fn xmlStrncasecmp(_: *const xmlChar, _: *const xmlChar, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __htmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
    #[no_mangle]
    fn inputPush(_: xmlParserCtxtPtr, _: xmlParserInputPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlFreeParserCtxt(_: xmlParserCtxtPtr);
    #[no_mangle]
    fn xmlInitNodeInfoSeq(_: xmlParserNodeInfoSeqPtr);
    #[no_mangle]
    fn xmlNewInputStream(_: xmlParserCtxtPtr) -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlInitParser();
    #[no_mangle]
    fn xmlNewStringInputStream(_: xmlParserCtxtPtr, _: *const xmlChar) -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlFreeInputStream(_: xmlParserInputPtr);
    #[no_mangle]
    fn inputPop(_: xmlParserCtxtPtr) -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlNewIOInputStream(
        _: xmlParserCtxtPtr,
        _: xmlParserInputBufferPtr,
        _: xmlCharEncoding,
    ) -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    #[no_mangle]
    fn xmlCreateMemoryParserCtxt(_: *const libc::c_char, _: libc::c_int) -> xmlParserCtxtPtr;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;

    #[no_mangle]
    fn xmlStopParser(ctxt: xmlParserCtxtPtr);
    #[no_mangle]
    fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);

    #[no_mangle]
    fn xmlStrPrintf(
        buf: *mut xmlChar,
        len: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn xmlUTF8Strsize(utf: *const xmlChar, len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn xmlUTF8Strpos(utf: *const xmlChar, pos: libc::c_int) -> *const xmlChar;
    #[no_mangle]
    fn xmlUTF8Strloc(utf: *const xmlChar, utfchar: *const xmlChar) -> libc::c_int;
    #[no_mangle]
    fn xmlUTF8Strsub(utf: *const xmlChar, start: libc::c_int, len: libc::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlUTF8Strlen(utf: *const xmlChar) -> libc::c_int;
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrstr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn log10(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __isinff(__value: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn __isnanf(__value: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn __isinfl(__value: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn __isnanl(__value: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn __isinf(__value: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn __isnan(__value: libc::c_double) -> libc::c_int;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    #[no_mangle]
    fn xmlGetNsList(doc: *const xmlDoc, node: *const xmlNode) -> *mut xmlNsPtr;
    #[no_mangle]
    fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;
    #[no_mangle]
    fn xmlNodeGetLang(cur: *const xmlNode) -> *mut xmlChar;
    #[no_mangle]
    fn xmlHashCreate(size: libc::c_int) -> xmlHashTablePtr;
    #[no_mangle]
    fn xmlHashAddEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    fn xmlHashUpdateEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut libc::c_void,
        f: xmlHashDeallocator,
    ) -> libc::c_int;
    /*
     * Remove an entry from the hash table.
     */
    #[no_mangle]
    fn xmlHashRemoveEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        f: xmlHashDeallocator,
    ) -> libc::c_int;
    /*
     * Retrieve the userdata.
     */
    #[no_mangle]
    fn xmlHashLookup(table: xmlHashTablePtr, name: *const xmlChar) -> *mut libc::c_void;
    #[no_mangle]
    fn xmlGetID(doc: xmlDocPtr, ID: *const xmlChar) -> xmlAttrPtr;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    /* array of locations */
    /*
     * Handling of location sets.
     */
    #[no_mangle]
    fn xmlXPtrLocationSetCreate(val: xmlXPathObjectPtr) -> xmlLocationSetPtr;
    #[no_mangle]
    fn xmlXPtrFreeLocationSet(obj: xmlLocationSetPtr);
    #[no_mangle]
    fn xmlXPtrLocationSetMerge(
        val1: xmlLocationSetPtr,
        val2: xmlLocationSetPtr,
    ) -> xmlLocationSetPtr;
    #[no_mangle]
    fn xmlXPtrNewRange(
        start: xmlNodePtr,
        startindex: libc::c_int,
        end: xmlNodePtr,
        endindex: libc::c_int,
    ) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPtrNewRangeNodeObject(start: xmlNodePtr, end: xmlXPathObjectPtr) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPtrLocationSetAdd(cur: xmlLocationSetPtr, val: xmlXPathObjectPtr);
    #[no_mangle]
    fn xmlXPtrWrapLocationSet(val: xmlLocationSetPtr) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlDebugDumpString(output: *mut FILE, str: *const xmlChar);
    #[no_mangle]
    fn xmlDebugDumpAttr(output: *mut FILE, attr: xmlAttrPtr, depth: libc::c_int);
    #[no_mangle]
    fn xmlDebugDumpOneNode(output: *mut FILE, node: xmlNodePtr, depth: libc::c_int);
    #[no_mangle]
    fn xmlFreePattern(comp: xmlPatternPtr);
    #[no_mangle]
    fn xmlFreePatternList(comp: xmlPatternPtr);
    #[no_mangle]
    fn xmlPatterncompile(
        pattern: *const xmlChar,
        dict: *mut xmlDict,
        flags: libc::c_int,
        namespaces: *mut *const xmlChar,
    ) -> xmlPatternPtr;
    #[no_mangle]
    fn xmlPatternStreamable(comp: xmlPatternPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlPatternMaxDepth(comp: xmlPatternPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlPatternMinDepth(comp: xmlPatternPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlPatternFromRoot(comp: xmlPatternPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlPatternGetStreamCtxt(comp: xmlPatternPtr) -> xmlStreamCtxtPtr;
    #[no_mangle]
    fn xmlFreeStreamCtxt(stream: xmlStreamCtxtPtr);
    #[no_mangle]
    fn xmlStreamPushNode(
        stream: xmlStreamCtxtPtr,
        name: *const xmlChar,
        ns: *const xmlChar,
        nodeType: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn xmlStreamPush(
        stream: xmlStreamCtxtPtr,
        name: *const xmlChar,
        ns: *const xmlChar,
    ) -> libc::c_int;
    #[no_mangle]
    fn xmlStreamPop(stream: xmlStreamCtxtPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlStreamWantsAnyNode(stream: xmlStreamCtxtPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlBufFree(buf: xmlBufPtr);
    #[no_mangle]
    fn xmlBufAdd(buf: xmlBufPtr, str: *const xmlChar, len: libc::c_int) -> libc::c_int;
}
