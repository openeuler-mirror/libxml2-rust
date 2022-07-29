//static mut stderr: *mut _IO_FILE;
fn getStderr() -> *mut _IO_FILE {
    unsafe {
        return stderr;
    }
}

//static mut xmlFree: xmlFreeFunc;
fn getXmlFree() -> xmlFreeFunc {
    unsafe {
        return xmlFree;
    }
}

//static mut xmlMalloc: xmlMallocFunc;
fn getXmlMalloc() -> xmlMallocFunc {
    unsafe {
        return xmlMalloc;
    }
}

//static mut xmlRealloc: xmlReallocFunc;
fn getXmlRealloc() -> xmlReallocFunc {
    unsafe {
        return xmlRealloc;
    }
}

//static mut xmlMallocAtomic: xmlMallocFunc;
fn getXmlMallocAtomic() -> xmlMallocFunc {
    unsafe {
        return xmlMallocAtomic;
    }
}

//static xmlIsBaseCharGroup: xmlChRangeGroup;
fn getXmlIsBaseCharGroup() -> &'static xmlChRangeGroup {
    unsafe {
        return &xmlIsBaseCharGroup;
    }
}

//static xmlIsCombiningGroup: xmlChRangeGroup;
fn getXmlIsCombiningGroup() -> &'static xmlChRangeGroup {
    unsafe {
        return &xmlIsCombiningGroup;
    }
}

//static xmlIsDigitGroup: xmlChRangeGroup;
fn getXmlIsDigitGroup() -> &'static xmlChRangeGroup {
    unsafe {
        return &xmlIsDigitGroup;
    }
}

//static xmlIsExtenderGroup: xmlChRangeGroup;
fn getXmlIsExtenderGroup() -> &'static xmlChRangeGroup {
    unsafe {
        return &xmlIsExtenderGroup;
    }
}

//static xmlIsPubidChar_tab: [libc::c_uchar; 256];
fn getXmlIsPubidChar_tab(index: usize) -> libc::c_int {
    unsafe {
        return xmlIsPubidChar_tab[index] as libc::c_int;
    }
}

fn xmlStrcmp_safe(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int {
    unsafe {
        return xmlStrcmp(str1, str2);
    }
}

fn xmlStrdup_safe(cur: *const xmlChar) -> *mut xmlChar {
    unsafe {
        return xmlStrdup(cur);
    }
}

fn xmlStrlen_safe(str: *const xmlChar) -> libc::c_int {
    unsafe {
        return xmlStrlen(str);
    }
}

fn __xmlLoaderErr_safe(
    ctx: *mut libc::c_void,
    msg: *const libc::c_char,
    filename: *const libc::c_char,
) {
    unsafe {
        __xmlLoaderErr(ctx, msg, filename);
    }
}

fn memset_safe(
    arg1: *mut libc::c_void,
    arg2: libc::c_int,
    arg3: libc::c_ulong,
) -> *mut libc::c_void {
    unsafe {
        return memset(arg1, arg2, arg3);
    }
}

fn strcmp_safe(arg1: *const libc::c_char, arg2: *const libc::c_char) -> libc::c_int {
    unsafe {
        return strcmp(arg1, arg2);
    }
}

fn xmlBufContent_safe(buf: *const xmlBuf) -> *mut xmlChar {
    unsafe {
        return xmlBufContent(buf);
    }
}

fn xmlBufEnd_safe(buf: xmlBufPtr) -> *mut xmlChar {
    unsafe {
        return xmlBufEnd(buf);
    }
}

fn xmlBufUse_safe(buf: xmlBufPtr) -> size_t {
    unsafe {
        return xmlBufUse(buf);
    }
}

fn xmlBufShrink_safe(buf: xmlBufPtr, len: size_t) -> size_t {
    unsafe {
        return xmlBufShrink(buf, len);
    }
}

fn xmlDictCreate_safe() -> xmlDictPtr {
    unsafe {
        return xmlDictCreate();
    }
}

fn xmlDictSetLimit_safe(dict: xmlDictPtr, limit: size_t) -> size_t {
    unsafe {
        return xmlDictSetLimit(dict, limit);
    }
}

fn xmlDictFree_safe(dict: xmlDictPtr) {
    unsafe {
        xmlDictFree(dict);
    }
}

fn __xmlGenericError_safe() -> *mut xmlGenericErrorFunc {
    unsafe {
        return __xmlGenericError();
    }
}

fn xmlHashFree_safe(table: xmlHashTablePtr, f: xmlHashDeallocator) {
    unsafe {
        xmlHashFree(table, f);
    }
}

fn xmlHashDefaultDeallocator_safe(entry: *mut libc::c_void, name: *const xmlChar) {
    unsafe {
        xmlHashDefaultDeallocator(entry, name);
    }
}

fn xmlGetCharEncodingHandler_safe(enc: xmlCharEncoding) -> xmlCharEncodingHandlerPtr {
    unsafe {
        return xmlGetCharEncodingHandler(enc);
    }
}

fn xmlCharEncCloseFunc_safe(handler: *mut xmlCharEncodingHandler) -> libc::c_int {
    unsafe {
        return xmlCharEncCloseFunc(handler);
    }
}

fn xmlParserInputBufferCreateFilename_safe(
    URI: *const libc::c_char,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    unsafe {
        return xmlParserInputBufferCreateFilename(URI, enc);
    }
}

fn xmlParserInputBufferRead_safe(in_0: xmlParserInputBufferPtr, len: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlParserInputBufferRead(in_0, len);
    }
}

fn xmlParserInputBufferGrow_safe(in_0: xmlParserInputBufferPtr, len: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlParserInputBufferGrow(in_0, len);
    }
}

fn xmlFreeParserInputBuffer_safe(in_0: xmlParserInputBufferPtr) {
    unsafe {
        xmlFreeParserInputBuffer(in_0);
    }
}

fn xmlParserGetDirectory_safe(filename: *const libc::c_char) -> *mut libc::c_char {
    unsafe {
        return xmlParserGetDirectory(filename);
    }
}

fn xmlCheckHTTPInput_safe(ctxt: xmlParserCtxtPtr, ret: xmlParserInputPtr) -> xmlParserInputPtr {
    unsafe {
        return xmlCheckHTTPInput(ctxt, ret);
    }
}

fn __xmlSubstituteEntitiesDefaultValue_safe() -> *mut libc::c_int {
    unsafe {
        return __xmlSubstituteEntitiesDefaultValue();
    }
}

fn __xmlKeepBlanksDefaultValue_safe() -> *mut libc::c_int {
    unsafe {
        return __xmlKeepBlanksDefaultValue();
    }
}

fn __xmlIndentTreeOutput_safe() -> *mut libc::c_int {
    unsafe {
        return __xmlIndentTreeOutput();
    }
}

fn __xmlPedanticParserDefaultValue_safe() -> *mut libc::c_int {
    unsafe {
        return __xmlPedanticParserDefaultValue();
    }
}

fn __xmlLineNumbersDefaultValue_safe() -> *mut libc::c_int {
    unsafe {
        return __xmlLineNumbersDefaultValue();
    }
}

fn __xmlDefaultSAXHandler_safe() -> *mut xmlSAXHandlerV1 {
    unsafe {
        return __xmlDefaultSAXHandler();
    }
}

fn __xmlGetWarningsDefaultValue_safe() -> *mut libc::c_int {
    unsafe {
        return __xmlGetWarningsDefaultValue();
    }
}

fn xmlSAX2IgnorableWhitespace_safe(ctx: *mut libc::c_void, ch: *const xmlChar, len: libc::c_int) {
    unsafe {
        xmlSAX2IgnorableWhitespace(ctx, ch, len);
    }
}

fn __xmlDoValidityCheckingDefaultValue_safe() -> *mut libc::c_int {
    unsafe {
        return __xmlDoValidityCheckingDefaultValue();
    }
}

fn __xmlLoadExtDtdDefaultValue_safe() -> *mut libc::c_int {
    unsafe {
        return __xmlLoadExtDtdDefaultValue();
    }
}

fn __xmlGenericErrorContext_safe() -> *mut *mut libc::c_void {
    unsafe {
        return __xmlGenericErrorContext();
    }
}

fn xmlSAXVersion_safe(hdlr: *mut xmlSAXHandler, version: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlSAXVersion(hdlr, version);
    }
}

fn xmlDefaultSAXHandlerInit_safe() {
    unsafe {
        xmlDefaultSAXHandlerInit();
    }
}

fn __xmlParserDebugEntities_safe() -> *mut libc::c_int {
    unsafe {
        return __xmlParserDebugEntities();
    }
}

fn xmlLoadExternalEntity_safe(
    URL: *const libc::c_char,
    ID: *const libc::c_char,
    ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    unsafe {
        return xmlLoadExternalEntity(URL, ID, ctxt);
    }
}

fn xmlCharInRange_safe(val: libc::c_uint, group: *const xmlChRangeGroup) -> libc::c_int {
    unsafe {
        return xmlCharInRange(val, group);
    }
}

fn xmlCanonicPath_safe(path: *const xmlChar) -> *mut xmlChar {
    unsafe {
        return xmlCanonicPath(path);
    }
}

fn xmlCatalogFreeLocal_safe(catalogs: *mut libc::c_void) {
    unsafe {
        xmlCatalogFreeLocal(catalogs);
    }
}

fn xmlBufCreate_safe() -> xmlBufPtr {
    unsafe {
        return xmlBufCreate();
    }
}

fn xmlBufIsEmpty_safe(buf: xmlBufPtr) -> libc::c_int {
    unsafe {
        return xmlBufIsEmpty(buf);
    }
}

fn xmlBufResetInput_safe(buf: xmlBufPtr, input: xmlParserInputPtr) -> libc::c_int {
    unsafe {
        return xmlBufResetInput(buf, input);
    }
}

fn xmlCharEncFirstLineInput_safe(input: xmlParserInputBufferPtr, len: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlCharEncFirstLineInput(input, len);
    }
}

fn xmlCharEncInput_safe(input: xmlParserInputBufferPtr, flush: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlCharEncInput(input, flush);
    }
}

fn xmlStrncmp_safe(str1: *const xmlChar, str2: *const xmlChar, len: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlStrncmp(str1, str2, len);
    }
}

fn xmlStrcasecmp_safe(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int {
    unsafe {
        return xmlStrcasecmp(str1, str2);
    }
}

fn xmlStrEqual_safe(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int {
    unsafe {
        return xmlStrEqual(str1, str2);
    }
}

fn xmlStrcasestr_safe(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar {
    unsafe {
        return xmlStrcasestr(str, val);
    }
}

fn xmlStrchr_safe(str: *const xmlChar, val: xmlChar) -> *const xmlChar {
    unsafe {
        return xmlStrchr(str, val);
    }
}

fn xmlCharStrdup_safe(cur: *const libc::c_char) -> *mut xmlChar {
    unsafe {
        return xmlCharStrdup(cur);
    }
}

fn xmlStrndup_safe(cur: *const xmlChar, len: libc::c_int) -> *mut xmlChar {
    unsafe {
        return xmlStrndup(cur, len);
    }
}

fn __htmlParseContent_safe(ctx: *mut libc::c_void) {
    unsafe {
        __htmlParseContent(ctx);
    }
}

fn __xmlGlobalInitMutexLock_safe() {
    unsafe {
        __xmlGlobalInitMutexLock();
    }
}

fn __xmlGlobalInitMutexUnlock_safe() {
    unsafe {
        __xmlGlobalInitMutexUnlock();
    }
}

fn xmlInputReadCallbackNop_safe(
    context: *mut libc::c_void,
    buffer: *mut libc::c_char,
    len: libc::c_int,
) -> libc::c_int {
    unsafe {
        return xmlInputReadCallbackNop(context, buffer, len);
    }
}

fn memcpy_safe(
    arg1: *mut libc::c_void,
    arg2: *const libc::c_void,
    arg3: libc::c_ulong,
) -> *mut libc::c_void {
    unsafe {
        return memcpy(arg1, arg2, arg3);
    }
}

fn memmove_safe(
    arg1: *mut libc::c_void,
    arg2: *const libc::c_void,
    arg3: libc::c_ulong,
) -> *mut libc::c_void {
    unsafe {
        return memmove(arg1, arg2, arg3);
    }
}

fn memchr_safe(
    arg1: *const libc::c_void,
    arg2: libc::c_int,
    arg3: libc::c_ulong,
) -> *mut libc::c_void {
    unsafe {
        return memchr(arg1, arg2, arg3);
    }
}

fn strncmp_safe(
    arg1: *const libc::c_char,
    arg2: *const libc::c_char,
    arg3: libc::c_ulong,
) -> libc::c_int {
    unsafe {
        return strncmp(arg1, arg2, arg3);
    }
}

fn strlen_safe(arg1: *const libc::c_char) -> libc::c_ulong {
    unsafe {
        return strlen(arg1);
    }
}

fn xmlCleanupInputCallbacks_safe() {
    unsafe {
        xmlCleanupInputCallbacks();
    }
}

fn xmlRegisterDefaultInputCallbacks_safe() {
    unsafe {
        xmlRegisterDefaultInputCallbacks();
    }
}

fn xmlAllocParserInputBuffer_safe(enc: xmlCharEncoding) -> xmlParserInputBufferPtr {
    unsafe {
        return xmlAllocParserInputBuffer(enc);
    }
}

fn xmlParserInputBufferCreateFd_safe(
    fd: libc::c_int,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    unsafe {
        return xmlParserInputBufferCreateFd(fd, enc);
    }
}

fn xmlParserInputBufferCreateMem_safe(
    mem: *const libc::c_char,
    size: libc::c_int,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    unsafe {
        return xmlParserInputBufferCreateMem(mem, size, enc);
    }
}

fn xmlParserInputBufferCreateIO_safe(
    ioread: xmlInputReadCallback,
    ioclose: xmlInputCloseCallback,
    ioctx: *mut libc::c_void,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    unsafe {
        return xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, enc);
    }
}

fn xmlParserInputBufferPush_safe(
    in_0: xmlParserInputBufferPtr,
    len: libc::c_int,
    buf: *const libc::c_char,
) -> libc::c_int {
    unsafe {
        return xmlParserInputBufferPush(in_0, len, buf);
    }
}

fn xmlCleanupOutputCallbacks_safe() {
    unsafe {
        xmlCleanupOutputCallbacks();
    }
}

fn xmlRegisterDefaultOutputCallbacks_safe() {
    unsafe {
        xmlRegisterDefaultOutputCallbacks();
    }
}

fn xmlInitializeDict_safe() -> libc::c_int {
    unsafe {
        return xmlInitializeDict();
    }
}

fn xmlDictReference_safe(dict: xmlDictPtr) -> libc::c_int {
    unsafe {
        return xmlDictReference(dict);
    }
}

fn xmlDictLookup_safe(dict: xmlDictPtr, name: *const xmlChar, len: libc::c_int) -> *const xmlChar {
    unsafe {
        return xmlDictLookup(dict, name, len);
    }
}

fn xmlDictOwns_safe(dict: xmlDictPtr, str: *const xmlChar) -> libc::c_int {
    unsafe {
        return xmlDictOwns(dict, str);
    }
}

fn xmlDictCleanup_safe() {
    unsafe {
        xmlDictCleanup();
    }
}

fn xmlBuildQName_safe(
    ncname: *const xmlChar,
    prefix: *const xmlChar,
    memory: *mut xmlChar,
    len: libc::c_int,
) -> *mut xmlChar {
    unsafe {
        return xmlBuildQName(ncname, prefix, memory, len);
    }
}

fn xmlSplitQName3_safe(name: *const xmlChar, len: *mut libc::c_int) -> *const xmlChar {
    unsafe {
        return xmlSplitQName3(name, len);
    }
}

fn xmlBufferCreate_safe() -> xmlBufferPtr {
    unsafe {
        return xmlBufferCreate();
    }
}

fn xmlBufferFree_safe(buf: xmlBufferPtr) {
    unsafe {
        xmlBufferFree(buf);
    }
}

fn xmlBufferAdd_safe(buf: xmlBufferPtr, str: *const xmlChar, len: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlBufferAdd(buf, str, len);
    }
}

fn xmlCreateIntSubset_safe(
    doc: xmlDocPtr,
    name: *const xmlChar,
    ExternalID: *const xmlChar,
    SystemID: *const xmlChar,
) -> xmlDtdPtr {
    unsafe {
        return xmlCreateIntSubset(doc, name, ExternalID, SystemID);
    }
}

fn xmlNewDtd_safe(
    doc: xmlDocPtr,
    name: *const xmlChar,
    ExternalID: *const xmlChar,
    SystemID: *const xmlChar,
) -> xmlDtdPtr {
    unsafe {
        return xmlNewDtd(doc, name, ExternalID, SystemID);
    }
}

fn xmlNewDoc_safe(version: *const xmlChar) -> xmlDocPtr {
    unsafe {
        return xmlNewDoc(version);
    }
}

fn xmlFreeDoc_safe(cur: xmlDocPtr) {
    unsafe {
        xmlFreeDoc(cur);
    }
}

fn xmlNewDocNode_safe(
    doc: xmlDocPtr,
    ns: xmlNsPtr,
    name: *const xmlChar,
    content: *const xmlChar,
) -> xmlNodePtr {
    unsafe {
        return xmlNewDocNode(doc, ns, name, content);
    }
}

fn xmlNewComment_safe(content: *const xmlChar) -> xmlNodePtr {
    unsafe {
        return xmlNewComment(content);
    }
}

fn xmlDocCopyNode_safe(node: xmlNodePtr, doc: xmlDocPtr, recursive: libc::c_int) -> xmlNodePtr {
    unsafe {
        return xmlDocCopyNode(node, doc, recursive);
    }
}

fn xmlGetLastChild_safe(parent: *const xmlNode) -> xmlNodePtr {
    unsafe {
        return xmlGetLastChild(parent);
    }
}

fn xmlNodeIsText_safe(node: *const xmlNode) -> libc::c_int {
    unsafe {
        return xmlNodeIsText(node);
    }
}

fn xmlAddChild_safe(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr {
    unsafe {
        return xmlAddChild(parent, cur);
    }
}

fn xmlAddChildList_safe(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr {
    unsafe {
        return xmlAddChildList(parent, cur);
    }
}

fn xmlUnlinkNode_safe(cur: xmlNodePtr) {
    unsafe {
        xmlUnlinkNode(cur);
    }
}

fn xmlFreeNodeList_safe(cur: xmlNodePtr) {
    unsafe {
        xmlFreeNodeList(cur);
    }
}

fn xmlFreeNode_safe(cur: xmlNodePtr) {
    unsafe {
        xmlFreeNode(cur);
    }
}

fn xmlSetTreeDoc_safe(tree: xmlNodePtr, doc: xmlDocPtr) {
    unsafe {
        xmlSetTreeDoc(tree, doc);
    }
}

fn xmlSearchNsByHref_safe(doc: xmlDocPtr, node: xmlNodePtr, href: *const xmlChar) -> xmlNsPtr {
    unsafe {
        return xmlSearchNsByHref(doc, node, href);
    }
}

fn xmlHashCreateDict_safe(size: libc::c_int, dict: xmlDictPtr) -> xmlHashTablePtr {
    unsafe {
        return xmlHashCreateDict(size, dict);
    }
}

fn xmlHashAddEntry2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
    userdata: *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        return xmlHashAddEntry2(table, name, name2, userdata);
    }
}

fn xmlHashUpdateEntry2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
    userdata: *mut libc::c_void,
    f: xmlHashDeallocator,
) -> libc::c_int {
    unsafe {
        return xmlHashUpdateEntry2(table, name, name2, userdata, f);
    }
}

fn xmlHashRemoveEntry2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
    f: xmlHashDeallocator,
) -> libc::c_int {
    unsafe {
        return xmlHashRemoveEntry2(table, name, name2, f);
    }
}

fn xmlHashLookup2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
) -> *mut libc::c_void {
    unsafe {
        return xmlHashLookup2(table, name, name2);
    }
}

fn xmlHashQLookup2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    prefix: *const xmlChar,
    name2: *const xmlChar,
    prefix2: *const xmlChar,
) -> *mut libc::c_void {
    unsafe {
        return xmlHashQLookup2(table, name, prefix, name2, prefix2);
    }
}

fn xmlHashSize_safe(table: xmlHashTablePtr) -> libc::c_int {
    unsafe {
        return xmlHashSize(table);
    }
}

fn xmlHashScanFull_safe(table: xmlHashTablePtr, f: xmlHashScannerFull, data: *mut libc::c_void) {
    unsafe {
        xmlHashScanFull(table, f, data);
    }
}

fn initGenericErrorDefaultFunc_safe(handler: *mut xmlGenericErrorFunc) {
    unsafe {
        initGenericErrorDefaultFunc(handler);
    }
}

fn xmlResetLastError_safe() {
    unsafe {
        xmlResetLastError();
    }
}

fn xmlResetError_safe(err: xmlErrorPtr) {
    unsafe {
        xmlResetError(err);
    }
}

fn xmlCopyError_safe(from: xmlErrorPtr, to: xmlErrorPtr) -> libc::c_int {
    unsafe {
        return xmlCopyError(from, to);
    }
}

fn xmlNewDocElementContent_safe(
    doc: xmlDocPtr,
    name: *const xmlChar,
    type_0: xmlElementContentType,
) -> xmlElementContentPtr {
    unsafe {
        return xmlNewDocElementContent(doc, name, type_0);
    }
}

fn xmlFreeDocElementContent_safe(doc: xmlDocPtr, cur: xmlElementContentPtr) {
    unsafe {
        xmlFreeDocElementContent(doc, cur);
    }
}

fn xmlCreateEnumeration_safe(name: *const xmlChar) -> xmlEnumerationPtr {
    unsafe {
        return xmlCreateEnumeration(name);
    }
}

fn xmlFreeEnumeration_safe(cur: xmlEnumerationPtr) {
    unsafe {
        xmlFreeEnumeration(cur);
    }
}

fn xmlValidateRoot_safe(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> libc::c_int {
    unsafe {
        return xmlValidateRoot(ctxt, doc);
    }
}

fn xmlValidateElement_safe(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, elem: xmlNodePtr) -> libc::c_int {
    unsafe {
        return xmlValidateElement(ctxt, doc, elem);
    }
}

fn xmlIsMixedElement_safe(doc: xmlDocPtr, name: *const xmlChar) -> libc::c_int {
    unsafe {
        return xmlIsMixedElement(doc, name);
    }
}

fn xmlGetPredefinedEntity_safe(name: *const xmlChar) -> xmlEntityPtr {
    unsafe {
        return xmlGetPredefinedEntity(name);
    }
}

fn xmlInitCharEncodingHandlers_safe() {
    unsafe {
        xmlInitCharEncodingHandlers();
    }
}

fn xmlCleanupCharEncodingHandlers_safe() {
    unsafe {
        xmlCleanupCharEncodingHandlers();
    }
}

fn xmlFindCharEncodingHandler_safe(name: *const libc::c_char) -> xmlCharEncodingHandlerPtr {
    unsafe {
        return xmlFindCharEncodingHandler(name);
    }
}

fn xmlDetectCharEncoding_safe(in_0: *const libc::c_uchar, len: libc::c_int) -> xmlCharEncoding {
    unsafe {
        return xmlDetectCharEncoding(in_0, len);
    }
}

fn xmlCleanupMemory_safe() {
    unsafe {
        xmlCleanupMemory();
    }
}

fn xmlInitMemory_safe() -> libc::c_int {
    unsafe {
        return xmlInitMemory();
    }
}

fn htmlDefaultSAXHandlerInit_safe() {
    unsafe {
        htmlDefaultSAXHandlerInit();
    }
}

fn xmlInitGlobals_safe() {
    unsafe {
        xmlInitGlobals();
    }
}

fn xmlInitThreads_safe() {
    unsafe {
        xmlInitThreads();
    }
}

fn xmlCleanupThreads_safe() {
    unsafe {
        xmlCleanupThreads();
    }
}

fn xmlCleanupGlobals_safe() {
    unsafe {
        xmlCleanupGlobals();
    }
}

fn xmlSAX2GetEntity_safe(ctx: *mut libc::c_void, name: *const xmlChar) -> xmlEntityPtr {
    unsafe {
        return xmlSAX2GetEntity(ctx, name);
    }
}

fn xmlSAX2StartElement_safe(
    ctx: *mut libc::c_void,
    fullname: *const xmlChar,
    atts: *mut *const xmlChar,
) {
    unsafe {
        xmlSAX2StartElement(ctx, fullname, atts);
    }
}

fn xmlSAX2EndElement_safe(ctx: *mut libc::c_void, name: *const xmlChar) {
    unsafe {
        xmlSAX2EndElement(ctx, name);
    }
}

fn __xmlDefaultSAXLocator_safe() -> *mut xmlSAXLocator {
    unsafe {
        return __xmlDefaultSAXLocator();
    }
}

fn xmlSAX2EntityDecl_safe(
    ctx: *mut libc::c_void,
    name: *const xmlChar,
    type_0: libc::c_int,
    publicId: *const xmlChar,
    systemId: *const xmlChar,
    content: *mut xmlChar,
) {
    unsafe {
        xmlSAX2EntityDecl(ctx, name, type_0, publicId, systemId, content);
    }
}

fn htmlCreateMemoryParserCtxt_safe(
    buffer: *const libc::c_char,
    size: libc::c_int,
) -> htmlParserCtxtPtr {
    unsafe {
        return htmlCreateMemoryParserCtxt(buffer, size);
    }
}

fn htmlInitAutoClose_safe() {
    unsafe {
        htmlInitAutoClose();
    }
}

fn xmlBuildURI_safe(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar {
    unsafe {
        return xmlBuildURI(URI, base);
    }
}

fn xmlParseURI_safe(str: *const libc::c_char) -> xmlURIPtr {
    unsafe {
        return xmlParseURI(str);
    }
}

fn xmlFreeURI_safe(uri: xmlURIPtr) {
    unsafe {
        xmlFreeURI(uri);
    }
}

fn xmlCatalogCleanup_safe() {
    unsafe {
        xmlCatalogCleanup();
    }
}

fn xmlCatalogAddLocal_safe(catalogs: *mut libc::c_void, URL: *const xmlChar) -> *mut libc::c_void {
    unsafe {
        return xmlCatalogAddLocal(catalogs, URL);
    }
}

fn xmlCatalogGetDefaults_safe() -> xmlCatalogAllow {
    unsafe {
        return xmlCatalogGetDefaults();
    }
}

fn xmlSchemaCleanupTypes_safe() {
    unsafe {
        xmlSchemaCleanupTypes();
    }
}

fn xmlRelaxNGCleanupTypes_safe() {
    unsafe {
        xmlRelaxNGCleanupTypes();
    }
}

fn xmlBufGetInputBase_safe(buf: xmlBufPtr, input: xmlParserInputPtr) -> size_t {
    unsafe {
        return xmlBufGetInputBase(buf, input);
    }
}

fn xmlBufSetInputBaseCur_safe(
    buf: xmlBufPtr,
    input: xmlParserInputPtr,
    base: size_t,
    cur: size_t,
) -> libc::c_int {
    unsafe {
        return xmlBufSetInputBaseCur(buf, input, base, cur);
    }
}

fn xmlXPathInit_safe() {
    unsafe {
        xmlXPathInit();
    }
}

fn xmlParserInputGrow_safe(arg1: xmlParserInputPtr, arg2: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlParserInputGrow(arg1, arg2);
    }
}

fn xmlGetIntSubset_safe(arg1: *const xmlDoc) -> xmlDtdPtr {
    unsafe {
        return xmlGetIntSubset(arg1);
    }
}

fn xmlSwitchEncoding_safe(arg1: xmlParserCtxtPtr, arg2: xmlCharEncoding) -> libc::c_int {
    unsafe {
        return xmlSwitchEncoding(arg1, arg2);
    }
}

fn xmlSwitchToEncoding_safe(
    arg1: xmlParserCtxtPtr,
    arg2: xmlCharEncodingHandlerPtr,
) -> libc::c_int {
    unsafe {
        return xmlSwitchToEncoding(arg1, arg2);
    }
}

fn xmlParserInputShrink_safe(arg1: xmlParserInputPtr) {
    unsafe {
        xmlParserInputShrink(arg1);
    }
}

fn xmlCopyChar_safe(arg1: libc::c_int, arg2: *mut xmlChar, arg3: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlCopyChar(arg1, arg2, arg3);
    }
}

fn xmlNextChar_safe(arg1: xmlParserCtxtPtr) {
    unsafe {
        xmlNextChar(arg1);
    }
}

fn xmlParserAddNodeInfo_safe(arg1: xmlParserCtxtPtr, arg2: xmlParserNodeInfoPtr) {
    unsafe {
        xmlParserAddNodeInfo(arg1, arg2);
    }
}

fn nodePop_safe(arg1: xmlParserCtxtPtr) -> xmlNodePtr {
    unsafe {
        return nodePop(arg1);
    }
}

fn xmlParseCharEncoding_safe(arg1: *const libc::c_char) -> xmlCharEncoding {
    unsafe {
        return xmlParseCharEncoding(arg1);
    }
}

fn xmlPopInput_safe(arg1: xmlParserCtxtPtr) -> xmlChar {
    unsafe {
        return xmlPopInput(arg1);
    }
}

fn xmlStrncasecmp_safe(
    arg1: *const xmlChar,
    arg2: *const xmlChar,
    arg3: libc::c_int,
) -> libc::c_int {
    unsafe {
        return xmlStrncasecmp(arg1, arg2, arg3);
    }
}

fn __htmlDefaultSAXHandler_safe() -> *mut xmlSAXHandlerV1 {
    unsafe {
        return __htmlDefaultSAXHandler();
    }
}

fn inputPush_safe(arg1: xmlParserCtxtPtr, arg2: xmlParserInputPtr) -> libc::c_int {
    unsafe {
        return inputPush(arg1, arg2);
    }
}

fn xmlFreeParserCtxt_safe(arg1: xmlParserCtxtPtr) {
    unsafe {
        xmlFreeParserCtxt(arg1);
    }
}

fn xmlInitNodeInfoSeq_safe(arg1: xmlParserNodeInfoSeqPtr) {
    unsafe {
        xmlInitNodeInfoSeq(arg1);
    }
}

fn xmlNewInputStream_safe(arg1: xmlParserCtxtPtr) -> xmlParserInputPtr {
    unsafe {
        return xmlNewInputStream(arg1);
    }
}

fn xmlInitParser_safe() {
    unsafe {
        xmlInitParser();
    }
}

fn xmlNewStringInputStream_safe(arg1: xmlParserCtxtPtr, arg2: *const xmlChar) -> xmlParserInputPtr {
    unsafe {
        return xmlNewStringInputStream(arg1, arg2);
    }
}

fn xmlFreeInputStream_safe(arg1: xmlParserInputPtr) {
    unsafe {
        xmlFreeInputStream(arg1);
    }
}

fn inputPop_safe(arg1: xmlParserCtxtPtr) -> xmlParserInputPtr {
    unsafe {
        return inputPop(arg1);
    }
}

fn xmlNewIOInputStream_safe(
    arg1: xmlParserCtxtPtr,
    arg2: xmlParserInputBufferPtr,
    arg3: xmlCharEncoding,
) -> xmlParserInputPtr {
    unsafe {
        return xmlNewIOInputStream(arg1, arg2, arg3);
    }
}

fn xmlNewParserCtxt_safe() -> xmlParserCtxtPtr {
    unsafe {
        return xmlNewParserCtxt();
    }
}

fn xmlCreateMemoryParserCtxt_safe(
    arg1: *const libc::c_char,
    arg2: libc::c_int,
) -> xmlParserCtxtPtr {
    unsafe {
        return xmlCreateMemoryParserCtxt(arg1, arg2);
    }
}

fn strcpy_safe(arg1: *mut libc::c_char, arg2: *const libc::c_char) -> *mut libc::c_char {
    unsafe {
        return strcpy(arg1, arg2);
    }
}

fn strcat_safe(arg1: *mut libc::c_char, arg2: *const libc::c_char) -> *mut libc::c_char {
    unsafe {
        return strcat(arg1, arg2);
    }
}

fn __ctype_toupper_loc_safe() -> *mut *const __int32_t {
    unsafe {
        return __ctype_toupper_loc();
    }
}

fn xmlStopParser_safe(ctxt: xmlParserCtxtPtr) {
    unsafe {
        xmlStopParser(ctxt);
    }
}

fn xmlCtxtReset_safe(ctxt: xmlParserCtxtPtr) {
    unsafe {
        xmlCtxtReset(ctxt);
    }
}

fn xmlUTF8Strsize_safe(utf: *const xmlChar, len: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlUTF8Strsize(utf, len);
    }
}

fn xmlUTF8Strpos_safe(utf: *const xmlChar, pos: libc::c_int) -> *const xmlChar {
    unsafe {
        return xmlUTF8Strpos(utf, pos);
    }
}

fn xmlUTF8Strloc_safe(utf: *const xmlChar, utfchar: *const xmlChar) -> libc::c_int {
    unsafe {
        return xmlUTF8Strloc(utf, utfchar);
    }
}

fn xmlUTF8Strsub_safe(utf: *const xmlChar, start: libc::c_int, len: libc::c_int) -> *mut xmlChar {
    unsafe {
        return xmlUTF8Strsub(utf, start, len);
    }
}

fn xmlUTF8Strlen_safe(utf: *const xmlChar) -> libc::c_int {
    unsafe {
        return xmlUTF8Strlen(utf);
    }
}

fn xmlStrcat_safe(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar {
    unsafe {
        return xmlStrcat(cur, add);
    }
}

fn xmlStrstr_safe(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar {
    unsafe {
        return xmlStrstr(str, val);
    }
}

fn log10_safe(arg1: libc::c_double) -> libc::c_double {
    unsafe {
        return log10(arg1);
    }
}

fn pow_safe(arg1: libc::c_double, arg2: libc::c_double) -> libc::c_double {
    unsafe {
        return pow(arg1, arg2);
    }
}

fn ceil_safe(arg1: libc::c_double) -> libc::c_double {
    unsafe {
        return ceil(arg1);
    }
}

fn fabs_safe(arg1: libc::c_double) -> libc::c_double {
    unsafe {
        return fabs(arg1);
    }
}

fn floor_safe(arg1: libc::c_double) -> libc::c_double {
    unsafe {
        return floor(arg1);
    }
}

fn fmod_safe(arg1: libc::c_double, arg2: libc::c_double) -> libc::c_double {
    unsafe {
        return fmod(arg1, arg2);
    }
}

fn __isinff_safe(__value: libc::c_float) -> libc::c_int {
    unsafe {
        return __isinff(__value);
    }
}

fn __isnanf_safe(__value: libc::c_float) -> libc::c_int {
    unsafe {
        return __isnanf(__value);
    }
}

fn __isinfl_safe(__value: libc::c_float) -> libc::c_int {
    unsafe {
        return __isinfl(__value);
    }
}

fn __isnanl_safe(__value: libc::c_float) -> libc::c_int {
    unsafe {
        return __isnanl(__value);
    }
}

fn __isinf_safe(__value: libc::c_double) -> libc::c_int {
    unsafe {
        return __isinf(__value);
    }
}

fn __isnan_safe(__value: libc::c_double) -> libc::c_int {
    unsafe {
        return __isnan(__value);
    }
}

fn xmlDocGetRootElement_safe(doc: *const xmlDoc) -> xmlNodePtr {
    unsafe {
        return xmlDocGetRootElement(doc);
    }
}

fn xmlGetNsList_safe(doc: *const xmlDoc, node: *const xmlNode) -> *mut xmlNsPtr {
    unsafe {
        return xmlGetNsList(doc, node);
    }
}

fn xmlNodeGetContent_safe(cur: *const xmlNode) -> *mut xmlChar {
    unsafe {
        return xmlNodeGetContent(cur);
    }
}

fn xmlNodeGetLang_safe(cur: *const xmlNode) -> *mut xmlChar {
    unsafe {
        return xmlNodeGetLang(cur);
    }
}

fn xmlHashCreate_safe(size: libc::c_int) -> xmlHashTablePtr {
    unsafe {
        return xmlHashCreate(size);
    }
}

fn xmlHashAddEntry_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    userdata: *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        return xmlHashAddEntry(table, name, userdata);
    }
}

fn xmlHashUpdateEntry_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    userdata: *mut libc::c_void,
    f: xmlHashDeallocator,
) -> libc::c_int {
    unsafe {
        return xmlHashUpdateEntry(table, name, userdata, f);
    }
}

fn xmlHashRemoveEntry_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    f: xmlHashDeallocator,
) -> libc::c_int {
    unsafe {
        return xmlHashRemoveEntry(table, name, f);
    }
}

fn xmlHashLookup_safe(table: xmlHashTablePtr, name: *const xmlChar) -> *mut libc::c_void {
    unsafe {
        return xmlHashLookup(table, name);
    }
}

fn xmlGetID_safe(doc: xmlDocPtr, ID: *const xmlChar) -> xmlAttrPtr {
    unsafe {
        return xmlGetID(doc, ID);
    }
}

fn realloc_safe(arg1: *mut libc::c_void, arg2: libc::c_ulong) -> *mut libc::c_void {
    unsafe {
        return realloc(arg1, arg2);
    }
}

fn free_safe(__ptr: *mut libc::c_void) {
    unsafe {
        free(__ptr);
    }
}

fn exit_safe(arg1: libc::c_int) -> ! {
    unsafe {
        return exit(arg1);
    }
}

fn xmlXPtrLocationSetCreate_safe(val: xmlXPathObjectPtr) -> xmlLocationSetPtr {
    unsafe {
        return xmlXPtrLocationSetCreate(val);
    }
}

fn xmlXPtrFreeLocationSet_safe(obj: xmlLocationSetPtr) {
    unsafe {
        xmlXPtrFreeLocationSet(obj);
    }
}

fn xmlXPtrLocationSetMerge_safe(
    val1: xmlLocationSetPtr,
    val2: xmlLocationSetPtr,
) -> xmlLocationSetPtr {
    unsafe {
        return xmlXPtrLocationSetMerge(val1, val2);
    }
}

fn xmlXPtrNewRange_safe(
    start: xmlNodePtr,
    startindex: libc::c_int,
    end: xmlNodePtr,
    endindex: libc::c_int,
) -> xmlXPathObjectPtr {
    unsafe {
        return xmlXPtrNewRange(start, startindex, end, endindex);
    }
}

fn xmlXPtrNewRangeNodeObject_safe(start: xmlNodePtr, end: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    unsafe {
        return xmlXPtrNewRangeNodeObject(start, end);
    }
}

fn xmlXPtrLocationSetAdd_safe(cur: xmlLocationSetPtr, val: xmlXPathObjectPtr) {
    unsafe {
        xmlXPtrLocationSetAdd(cur, val);
    }
}

fn xmlXPtrWrapLocationSet_safe(val: xmlLocationSetPtr) -> xmlXPathObjectPtr {
    unsafe {
        return xmlXPtrWrapLocationSet(val);
    }
}

fn xmlDebugDumpString_safe(output: *mut FILE, str: *const xmlChar) {
    unsafe {
        xmlDebugDumpString(output, str);
    }
}

fn xmlDebugDumpAttr_safe(output: *mut FILE, attr: xmlAttrPtr, depth: libc::c_int) {
    unsafe {
        xmlDebugDumpAttr(output, attr, depth);
    }
}

fn xmlDebugDumpOneNode_safe(output: *mut FILE, node: xmlNodePtr, depth: libc::c_int) {
    unsafe {
        xmlDebugDumpOneNode(output, node, depth);
    }
}

fn xmlFreePattern_safe(comp: xmlPatternPtr) {
    unsafe {
        xmlFreePattern(comp);
    }
}

fn xmlFreePatternList_safe(comp: xmlPatternPtr) {
    unsafe {
        xmlFreePatternList(comp);
    }
}

fn xmlPatterncompile_safe(
    pattern: *const xmlChar,
    dict: *mut xmlDict,
    flags: libc::c_int,
    namespaces: *mut *const xmlChar,
) -> xmlPatternPtr {
    unsafe {
        return xmlPatterncompile(pattern, dict, flags, namespaces);
    }
}

fn xmlPatternStreamable_safe(comp: xmlPatternPtr) -> libc::c_int {
    unsafe {
        return xmlPatternStreamable(comp);
    }
}

fn xmlPatternMaxDepth_safe(comp: xmlPatternPtr) -> libc::c_int {
    unsafe {
        return xmlPatternMaxDepth(comp);
    }
}

fn xmlPatternMinDepth_safe(comp: xmlPatternPtr) -> libc::c_int {
    unsafe {
        return xmlPatternMinDepth(comp);
    }
}

fn xmlPatternFromRoot_safe(comp: xmlPatternPtr) -> libc::c_int {
    unsafe {
        return xmlPatternFromRoot(comp);
    }
}

fn xmlPatternGetStreamCtxt_safe(comp: xmlPatternPtr) -> xmlStreamCtxtPtr {
    unsafe {
        return xmlPatternGetStreamCtxt(comp);
    }
}

fn xmlFreeStreamCtxt_safe(stream: xmlStreamCtxtPtr) {
    unsafe {
        xmlFreeStreamCtxt(stream);
    }
}

fn xmlStreamPushNode_safe(
    stream: xmlStreamCtxtPtr,
    name: *const xmlChar,
    ns: *const xmlChar,
    nodeType: libc::c_int,
) -> libc::c_int {
    unsafe {
        return xmlStreamPushNode(stream, name, ns, nodeType);
    }
}

fn xmlStreamPush_safe(
    stream: xmlStreamCtxtPtr,
    name: *const xmlChar,
    ns: *const xmlChar,
) -> libc::c_int {
    unsafe {
        return xmlStreamPush(stream, name, ns);
    }
}

fn xmlStreamPop_safe(stream: xmlStreamCtxtPtr) -> libc::c_int {
    unsafe {
        return xmlStreamPop(stream);
    }
}

fn xmlStreamWantsAnyNode_safe(stream: xmlStreamCtxtPtr) -> libc::c_int {
    unsafe {
        return xmlStreamWantsAnyNode(stream);
    }
}

fn xmlBufFree_safe(buf: xmlBufPtr) {
    unsafe {
        xmlBufFree(buf);
    }
}

fn xmlBufAdd_safe(buf: xmlBufPtr, str: *const xmlChar, len: libc::c_int) -> libc::c_int {
    unsafe {
        return xmlBufAdd(buf, str, len);
    }
}

// fn __xmlRaiseError_safe(schannel: xmlStructuredErrorFunc,
//                         channel: xmlGenericErrorFunc, data: *mut libc::c_void,
//                         ctx: *mut libc::c_void, node: *mut libc::c_void,
//                         domain: libc::c_int, code: libc::c_int,
//                         level: xmlErrorLevel, file: *const libc::c_char,
//                         line: libc::c_int, str1: *const libc::c_char,
//                         str2: *const libc::c_char, str3: *const libc::c_char,
//                         int1: libc::c_int, col: libc::c_int,
//                         msg: *const libc::c_char, _: ...) {
//     //todo
// }

// fn fprintf_safe(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int {
//     //todo
//     return 0 as libc::c_int;
// }

// fn snprintf_safe(_: *mut libc::c_char, _: libc::c_ulong,
//                  _: *const libc::c_char, _: ...) -> libc::c_int {
//     //todo
//     return 0 as libc::c_int;
// }

// fn xmlParserValidityError_safe(ctx: *mut libc::c_void,
//                                msg: *const libc::c_char, _: ...) {
//     //todo
// }

// fn xmlParserValidityWarning_safe(ctx: *mut libc::c_void,
//                                  msg: *const libc::c_char, _: ...) {
//     //todo
// }

// fn xmlGenericErrorDefaultFunc_safe(ctx: *mut libc::c_void,
//                               msg: *const libc::c_char, _: ...) {
//     //todo
// }

// fn xmlStrPrintf_safe(
//     buf: *mut xmlChar,
//     len: libc::c_int,
//     msg: *const libc::c_char,
//     _: ...) -> libc::c_int {
//     //todo
//     return 0 as libc::c_int;
// }
