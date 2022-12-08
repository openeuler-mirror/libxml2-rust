use super::super::ffi_defination::defination::*;
use super::super::ffi_extern_method::extern_method::*;

//static mut stderr: *mut _IO_FILE;
pub unsafe fn getStderr() -> *mut _IO_FILE {
    return stderr;
}

//static mut xmlFree: xmlFreeFunc;
pub unsafe fn xmlFree_safe(arg1: *mut ()) {
    xmlFree.expect("non-null function pointer")(arg1);
}

//static mut xmlMalloc: xmlMallocFunc;
pub unsafe fn xmlMalloc_safe(arg1: size_t) -> *mut () {
    return xmlMalloc.expect("non-null function pointer")(arg1);
}

//static mut xmlRealloc: xmlReallocFunc;
pub unsafe fn xmlRealloc_safe(arg1: *mut (), arg2: size_t) -> *mut () {
    return xmlRealloc.expect("non-null function pointer")(arg1, arg2);
}

//static mut xmlMallocAtomic: xmlMallocFunc;
pub unsafe fn xmlMallocAtomic_safe(arg1: size_t) -> *mut () {
    return xmlMallocAtomic.expect("non-null function pointer")(arg1);
}

//static xmlIsBaseCharGroup: xmlChRangeGroup;
pub unsafe fn getXmlIsBaseCharGroup() -> &'static xmlChRangeGroup {
    return &xmlIsBaseCharGroup;
}

//static xmlIsCombiningGroup: xmlChRangeGroup;
pub unsafe fn getXmlIsCombiningGroup() -> &'static xmlChRangeGroup {
    return &xmlIsCombiningGroup;
}

//static xmlIsDigitGroup: xmlChRangeGroup;
pub unsafe fn getXmlIsDigitGroup() -> &'static xmlChRangeGroup {
    return &xmlIsDigitGroup;
}

//static xmlIsExtenderGroup: xmlChRangeGroup;
pub unsafe fn getXmlIsExtenderGroup() -> &'static xmlChRangeGroup {
    return &xmlIsExtenderGroup;
}

//static xmlIsPubidChar_tab: [u8; 256];
pub unsafe fn getXmlIsPubidChar_tab(index: usize) -> i32 {
    return xmlIsPubidChar_tab[index] as i32;
}

pub unsafe fn xmlStrcmp_safe(str1: *const xmlChar, str2: *const xmlChar) -> i32 {
    return xmlStrcmp(str1, str2);
}

pub unsafe fn xmlStrdup_safe(cur: *const xmlChar) -> *mut xmlChar {
    return xmlStrdup(cur);
}

pub unsafe fn xmlStrlen_safe(str: *const xmlChar) -> i32 {
    return xmlStrlen(str);
}

pub unsafe fn __xmlLoaderErr_safe(ctx: *mut (), msg: *const i8, filename: *const i8) {
    __xmlLoaderErr(ctx, msg, filename);
}

pub unsafe fn memset_safe(arg1: *mut (), arg2: i32, arg3: u64) -> *mut () {
    return memset(arg1, arg2, arg3);
}

pub unsafe fn strcmp_safe(arg1: *const i8, arg2: *const i8) -> i32 {
    return strcmp(arg1, arg2);
}

pub unsafe fn xmlBufContent_safe(buf: *const xmlBuf) -> *mut xmlChar {
    return xmlBufContent(buf);
}

pub unsafe fn xmlBufEnd_safe(buf: xmlBufPtr) -> *mut xmlChar {
    return xmlBufEnd(buf);
}

pub unsafe fn xmlBufUse_safe(buf: xmlBufPtr) -> size_t {
    return xmlBufUse(buf);
}

pub unsafe fn xmlBufShrink_safe(buf: xmlBufPtr, len: size_t) -> size_t {
    return xmlBufShrink(buf, len);
}

pub unsafe fn xmlDictCreate_safe() -> xmlDictPtr {
    return xmlDictCreate();
}

pub unsafe fn xmlDictSetLimit_safe(dict: xmlDictPtr, limit: size_t) -> size_t {
    return xmlDictSetLimit(dict, limit);
}

pub unsafe fn xmlDictFree_safe(dict: xmlDictPtr) {
    xmlDictFree(dict);
}

// pub unsafe fn __xmlGenericError_safe() -> *mut xmlGenericErrorFunc {
// //         return __xmlGenericError();
// // }

pub unsafe fn xmlHashFree_safe(table: xmlHashTablePtr, f: xmlHashDeallocator) {
    xmlHashFree(table, f);
}

pub unsafe fn xmlHashDefaultDeallocator_safe(entry: *mut (), name: *const xmlChar) {
    xmlHashDefaultDeallocator(entry, name);
}

pub unsafe fn xmlGetCharEncodingHandler_safe(enc: xmlCharEncoding) -> xmlCharEncodingHandlerPtr {
    return xmlGetCharEncodingHandler(enc);
}

pub unsafe fn xmlCharEncCloseFunc_safe(handler: *mut xmlCharEncodingHandler) -> i32 {
    return xmlCharEncCloseFunc(handler);
}

pub unsafe fn xmlParserInputBufferCreateFilename_safe(
    URI: *const i8,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    return xmlParserInputBufferCreateFilename(URI, enc);
}

pub unsafe fn xmlParserInputBufferRead_safe(in_0: xmlParserInputBufferPtr, len: i32) -> i32 {
    return xmlParserInputBufferRead(in_0, len);
}

pub unsafe fn xmlParserInputBufferGrow_safe(in_0: xmlParserInputBufferPtr, len: i32) -> i32 {
    return xmlParserInputBufferGrow(in_0, len);
}

pub unsafe fn xmlFreeParserInputBuffer_safe(in_0: xmlParserInputBufferPtr) {
    xmlFreeParserInputBuffer(in_0);
}

pub unsafe fn xmlParserGetDirectory_safe(filename: *const i8) -> *mut i8 {
    return xmlParserGetDirectory(filename);
}

pub unsafe fn xmlCheckHTTPInput_safe(
    ctxt: xmlParserCtxtPtr,
    ret: xmlParserInputPtr,
) -> xmlParserInputPtr {
    return xmlCheckHTTPInput(ctxt, ret);
}

pub unsafe fn __xmlSubstituteEntitiesDefaultValue_safe() -> *mut i32 {
    return __xmlSubstituteEntitiesDefaultValue();
}

pub unsafe fn __xmlKeepBlanksDefaultValue_safe() -> i32 {
    return *__xmlKeepBlanksDefaultValue();
}

pub unsafe fn __xmlIndentTreeOutput_safe() -> *mut i32 {
    return __xmlIndentTreeOutput();
}

pub unsafe fn __xmlPedanticParserDefaultValue_safe() -> *mut i32 {
    return __xmlPedanticParserDefaultValue();
}

pub unsafe fn __xmlLineNumbersDefaultValue_safe() -> i32 {
    return *__xmlLineNumbersDefaultValue();
}

pub unsafe fn __xmlDefaultSAXHandler_safe() -> *mut xmlSAXHandlerV1 {
    return __xmlDefaultSAXHandler();
}

pub unsafe fn __xmlGetWarningsDefaultValue_safe() -> *mut i32 {
    return __xmlGetWarningsDefaultValue();
}

pub unsafe fn xmlSAX2IgnorableWhitespace_safe(ctx: *mut (), ch: *const xmlChar, len: i32) {
    xmlSAX2IgnorableWhitespace(ctx, ch, len);
}

pub unsafe fn __xmlDoValidityCheckingDefaultValue_safe() -> *mut i32 {
    return __xmlDoValidityCheckingDefaultValue();
}

pub unsafe fn __xmlLoadExtDtdDefaultValue_safe() -> *mut i32 {
    return __xmlLoadExtDtdDefaultValue();
}

pub unsafe fn __xmlGenericErrorContext_safe() -> *mut () {
    return *__xmlGenericErrorContext();
}

pub unsafe fn xmlSAXVersion_safe(hdlr: *mut xmlSAXHandler, version: i32) -> i32 {
    return xmlSAXVersion(hdlr, version);
}

pub unsafe fn xmlDefaultSAXHandlerInit_safe() {
    xmlDefaultSAXHandlerInit();
}

pub unsafe fn __xmlParserDebugEntities_safe() -> *mut i32 {
    return __xmlParserDebugEntities();
}

pub unsafe fn xmlLoadExternalEntity_safe(
    URL: *const i8,
    ID: *const i8,
    ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    return xmlLoadExternalEntity(URL, ID, ctxt);
}

pub unsafe fn xmlCharInRange_safe(val: u32, group: *const xmlChRangeGroup) -> i32 {
    return xmlCharInRange(val, group);
}

pub unsafe fn xmlCanonicPath_safe(path: *const xmlChar) -> *mut xmlChar {
    return xmlCanonicPath(path);
}

pub unsafe fn xmlCatalogFreeLocal_safe(catalogs: *mut ()) {
    xmlCatalogFreeLocal(catalogs);
}

pub unsafe fn xmlBufCreate_safe() -> xmlBufPtr {
    return xmlBufCreate();
}

pub unsafe fn xmlBufIsEmpty_safe(buf: xmlBufPtr) -> i32 {
    return xmlBufIsEmpty(buf);
}

pub unsafe fn xmlBufResetInput_safe(buf: xmlBufPtr, input: xmlParserInputPtr) -> i32 {
    return xmlBufResetInput(buf, input);
}

pub unsafe fn xmlCharEncFirstLineInput_safe(input: xmlParserInputBufferPtr, len: i32) -> i32 {
    return xmlCharEncFirstLineInput(input, len);
}

pub unsafe fn xmlCharEncInput_safe(input: xmlParserInputBufferPtr, flush: i32) -> i32 {
    return xmlCharEncInput(input, flush);
}

pub unsafe fn xmlStrncmp_safe(str1: *const xmlChar, str2: *const xmlChar, len: i32) -> i32 {
    return xmlStrncmp(str1, str2, len);
}

pub unsafe fn xmlStrcasecmp_safe(str1: *const xmlChar, str2: *const xmlChar) -> i32 {
    return xmlStrcasecmp(str1, str2);
}

pub unsafe fn xmlStrEqual_safe(str1: *const xmlChar, str2: *const xmlChar) -> i32 {
    return xmlStrEqual(str1, str2);
}

pub unsafe fn xmlStrcasestr_safe(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar {
    return xmlStrcasestr(str, val);
}

pub unsafe fn xmlStrchr_safe(str: *const xmlChar, val: xmlChar) -> *const xmlChar {
    return xmlStrchr(str, val);
}

pub unsafe fn xmlCharStrdup_safe(cur: *const i8) -> *mut xmlChar {
    return xmlCharStrdup(cur);
}

pub unsafe fn xmlStrndup_safe(cur: *const xmlChar, len: i32) -> *mut xmlChar {
    return xmlStrndup(cur, len);
}

pub unsafe fn __htmlParseContent_safe(ctx: *mut ()) {
    __htmlParseContent(ctx);
}

pub unsafe fn __xmlGlobalInitMutexLock_safe() {
    __xmlGlobalInitMutexLock();
}

pub unsafe fn __xmlGlobalInitMutexUnlock_safe() {
    __xmlGlobalInitMutexUnlock();
}

pub unsafe fn xmlInputReadCallbackNop_safe(context: *mut (), buffer: *mut i8, len: i32) -> i32 {
    return xmlInputReadCallbackNop(context, buffer, len);
}

pub unsafe fn memcpy_safe(arg1: *mut (), arg2: *const (), arg3: u64) -> *mut () {
    return memcpy(arg1, arg2, arg3);
}

pub unsafe fn memmove_safe(arg1: *mut (), arg2: *const (), arg3: u64) -> *mut () {
    return memmove(arg1, arg2, arg3);
}

pub unsafe fn memchr_safe(arg1: *const (), arg2: i32, arg3: u64) -> *mut () {
    return memchr(arg1, arg2, arg3);
}

pub unsafe fn strncmp_safe(arg1: *const i8, arg2: *const i8, arg3: u64) -> i32 {
    return strncmp(arg1, arg2, arg3);
}

pub unsafe fn strlen_safe(arg1: *const i8) -> u64 {
    return strlen(arg1);
}

pub unsafe fn xmlCleanupInputCallbacks_safe() {
    xmlCleanupInputCallbacks();
}

pub unsafe fn xmlRegisterDefaultInputCallbacks_safe() {
    xmlRegisterDefaultInputCallbacks();
}

pub unsafe fn xmlAllocParserInputBuffer_safe(enc: xmlCharEncoding) -> xmlParserInputBufferPtr {
    return xmlAllocParserInputBuffer(enc);
}

pub unsafe fn xmlParserInputBufferCreateFd_safe(
    fd: i32,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    return xmlParserInputBufferCreateFd(fd, enc);
}

pub unsafe fn xmlParserInputBufferCreateMem_safe(
    mem: *const i8,
    size: i32,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    return xmlParserInputBufferCreateMem(mem, size, enc);
}

pub unsafe fn xmlParserInputBufferCreateIO_safe(
    ioread: xmlInputReadCallback,
    ioclose: xmlInputCloseCallback,
    ioctx: *mut (),
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    return xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, enc);
}

pub unsafe fn xmlParserInputBufferPush_safe(
    in_0: xmlParserInputBufferPtr,
    len: i32,
    buf: *const i8,
) -> i32 {
    return xmlParserInputBufferPush(in_0, len, buf);
}

pub unsafe fn xmlCleanupOutputCallbacks_safe() {
    xmlCleanupOutputCallbacks();
}

pub unsafe fn xmlRegisterDefaultOutputCallbacks_safe() {
    xmlRegisterDefaultOutputCallbacks();
}

pub unsafe fn xmlInitializeDict_safe() -> i32 {
    return xmlInitializeDict();
}

pub unsafe fn xmlDictReference_safe(dict: xmlDictPtr) -> i32 {
    return xmlDictReference(dict);
}

pub unsafe fn xmlDictLookup_safe(
    dict: xmlDictPtr,
    name: *const xmlChar,
    len: i32,
) -> *const xmlChar {
    return xmlDictLookup(dict, name, len);
}

pub unsafe fn xmlDictOwns_safe(dict: xmlDictPtr, str: *const xmlChar) -> i32 {
    return xmlDictOwns(dict, str);
}

pub unsafe fn xmlDictCleanup_safe() {
    xmlDictCleanup();
}

pub unsafe fn xmlBuildQName_safe(
    ncname: *const xmlChar,
    prefix: *const xmlChar,
    memory: *mut xmlChar,
    len: i32,
) -> *mut xmlChar {
    return xmlBuildQName(ncname, prefix, memory, len);
}

pub unsafe fn xmlSplitQName3_safe(name: *const xmlChar, len: *mut i32) -> *const xmlChar {
    return xmlSplitQName3(name, len);
}

pub unsafe fn xmlBufferCreate_safe() -> xmlBufferPtr {
    return xmlBufferCreate();
}

pub unsafe fn xmlBufferFree_safe(buf: xmlBufferPtr) {
    xmlBufferFree(buf);
}

pub unsafe fn xmlBufferAdd_safe(buf: xmlBufferPtr, str: *const xmlChar, len: i32) -> i32 {
    return xmlBufferAdd(buf, str, len);
}

pub unsafe fn xmlCreateIntSubset_safe(
    doc: xmlDocPtr,
    name: *const xmlChar,
    ExternalID: *const xmlChar,
    SystemID: *const xmlChar,
) -> xmlDtdPtr {
    return xmlCreateIntSubset(doc, name, ExternalID, SystemID);
}

pub unsafe fn xmlNewDtd_safe(
    doc: xmlDocPtr,
    name: *const xmlChar,
    ExternalID: *const xmlChar,
    SystemID: *const xmlChar,
) -> xmlDtdPtr {
    return xmlNewDtd(doc, name, ExternalID, SystemID);
}

pub unsafe fn xmlNewDoc_safe(version: *const xmlChar) -> xmlDocPtr {
    return xmlNewDoc(version);
}

pub unsafe fn xmlFreeDoc_safe(cur: xmlDocPtr) {
    xmlFreeDoc(cur);
}

pub unsafe fn xmlNewDocNode_safe(
    doc: xmlDocPtr,
    ns: xmlNsPtr,
    name: *const xmlChar,
    content: *const xmlChar,
) -> xmlNodePtr {
    return xmlNewDocNode(doc, ns, name, content);
}

pub unsafe fn xmlNewComment_safe(content: *const xmlChar) -> xmlNodePtr {
    return xmlNewComment(content);
}

pub unsafe fn xmlDocCopyNode_safe(node: xmlNodePtr, doc: xmlDocPtr, recursive: i32) -> xmlNodePtr {
    return xmlDocCopyNode(node, doc, recursive);
}

pub unsafe fn xmlGetLastChild_safe(parent: *const xmlNode) -> xmlNodePtr {
    return xmlGetLastChild(parent);
}

pub unsafe fn xmlNodeIsText_safe(node: *const xmlNode) -> i32 {
    return xmlNodeIsText(node);
}

pub unsafe fn xmlAddChild_safe(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr {
    return xmlAddChild(parent, cur);
}

pub unsafe fn xmlAddChildList_safe(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr {
    return xmlAddChildList(parent, cur);
}

pub unsafe fn xmlUnlinkNode_safe(cur: xmlNodePtr) {
    xmlUnlinkNode(cur);
}

pub unsafe fn xmlFreeNodeList_safe(cur: xmlNodePtr) {
    xmlFreeNodeList(cur);
}

pub unsafe fn xmlFreeNode_safe(cur: xmlNodePtr) {
    xmlFreeNode(cur);
}

pub unsafe fn xmlSetTreeDoc_safe(tree: xmlNodePtr, doc: xmlDocPtr) {
    xmlSetTreeDoc(tree, doc);
}

pub unsafe fn xmlSearchNsByHref_safe(
    doc: xmlDocPtr,
    node: xmlNodePtr,
    href: *const xmlChar,
) -> xmlNsPtr {
    return xmlSearchNsByHref(doc, node, href);
}

pub unsafe fn xmlHashCreateDict_safe(size: i32, dict: xmlDictPtr) -> xmlHashTablePtr {
    return xmlHashCreateDict(size, dict);
}

pub unsafe fn xmlHashAddEntry2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
    userdata: *mut (),
) -> i32 {
    return xmlHashAddEntry2(table, name, name2, userdata);
}

pub unsafe fn xmlHashUpdateEntry2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
    userdata: *mut (),
    f: xmlHashDeallocator,
) -> i32 {
    return xmlHashUpdateEntry2(table, name, name2, userdata, f);
}

pub unsafe fn xmlHashRemoveEntry2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
    f: xmlHashDeallocator,
) -> i32 {
    return xmlHashRemoveEntry2(table, name, name2, f);
}

pub unsafe fn xmlHashLookup2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
) -> *mut () {
    return xmlHashLookup2(table, name, name2);
}

pub unsafe fn xmlHashQLookup2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    prefix: *const xmlChar,
    name2: *const xmlChar,
    prefix2: *const xmlChar,
) -> *mut () {
    return xmlHashQLookup2(table, name, prefix, name2, prefix2);
}

pub unsafe fn xmlHashSize_safe(table: xmlHashTablePtr) -> i32 {
    return xmlHashSize(table);
}

pub unsafe fn xmlHashScanFull_safe(table: xmlHashTablePtr, f: xmlHashScannerFull, data: *mut ()) {
    xmlHashScanFull(table, f, data);
}

pub unsafe fn initGenericErrorDefaultFunc_safe(handler: *mut xmlGenericErrorFunc) {
    initGenericErrorDefaultFunc(handler);
}

pub unsafe fn xmlResetLastError_safe() {
    xmlResetLastError();
}

pub unsafe fn xmlResetError_safe(err: xmlErrorPtr) {
    xmlResetError(err);
}

pub unsafe fn xmlCopyError_safe(from: xmlErrorPtr, to: xmlErrorPtr) -> i32 {
    return xmlCopyError(from, to);
}

pub unsafe fn xmlNewDocElementContent_safe(
    doc: xmlDocPtr,
    name: *const xmlChar,
    type_0: xmlElementContentType,
) -> xmlElementContentPtr {
    return xmlNewDocElementContent(doc, name, type_0);
}

pub unsafe fn xmlFreeDocElementContent_safe(doc: xmlDocPtr, cur: xmlElementContentPtr) {
    xmlFreeDocElementContent(doc, cur);
}

pub unsafe fn xmlCreateEnumeration_safe(name: *const xmlChar) -> xmlEnumerationPtr {
    return xmlCreateEnumeration(name);
}

pub unsafe fn xmlFreeEnumeration_safe(cur: xmlEnumerationPtr) {
    xmlFreeEnumeration(cur);
}

pub unsafe fn xmlValidateRoot_safe(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> i32 {
    return xmlValidateRoot(ctxt, doc);
}

pub unsafe fn xmlValidateElement_safe(
    ctxt: xmlValidCtxtPtr,
    doc: xmlDocPtr,
    elem: xmlNodePtr,
) -> i32 {
    return xmlValidateElement(ctxt, doc, elem);
}

pub unsafe fn xmlIsMixedElement_safe(doc: xmlDocPtr, name: *const xmlChar) -> i32 {
    return xmlIsMixedElement(doc, name);
}

pub unsafe fn xmlGetPredefinedEntity_safe(name: *const xmlChar) -> xmlEntityPtr {
    return xmlGetPredefinedEntity(name);
}

pub unsafe fn xmlInitCharEncodingHandlers_safe() {
    xmlInitCharEncodingHandlers();
}

pub unsafe fn xmlCleanupCharEncodingHandlers_safe() {
    xmlCleanupCharEncodingHandlers();
}

pub unsafe fn xmlFindCharEncodingHandler_safe(name: *const i8) -> xmlCharEncodingHandlerPtr {
    return xmlFindCharEncodingHandler(name);
}

pub unsafe fn xmlDetectCharEncoding_safe(in_0: *const u8, len: i32) -> xmlCharEncoding {
    return xmlDetectCharEncoding(in_0, len);
}

pub unsafe fn xmlCleanupMemory_safe() {
    xmlCleanupMemory();
}

pub unsafe fn xmlInitMemory_safe() -> i32 {
    return xmlInitMemory();
}

pub unsafe fn htmlDefaultSAXHandlerInit_safe() {
    htmlDefaultSAXHandlerInit();
}

pub unsafe fn xmlInitGlobals_safe() {
    xmlInitGlobals();
}

pub unsafe fn xmlInitThreads_safe() {
    xmlInitThreads();
}

pub unsafe fn xmlCleanupThreads_safe() {
    xmlCleanupThreads();
}

pub unsafe fn xmlCleanupGlobals_safe() {
    xmlCleanupGlobals();
}

pub unsafe fn xmlSAX2GetEntity_safe(ctx: *mut (), name: *const xmlChar) -> xmlEntityPtr {
    return xmlSAX2GetEntity(ctx, name);
}

pub unsafe fn xmlSAX2StartElement_safe(
    ctx: *mut (),
    fullname: *const xmlChar,
    atts: *mut *const xmlChar,
) {
    xmlSAX2StartElement(ctx, fullname, atts);
}

pub unsafe fn xmlSAX2EndElement_safe(ctx: *mut (), name: *const xmlChar) {
    xmlSAX2EndElement(ctx, name);
}

pub unsafe fn __xmlDefaultSAXLocator_safe() -> *mut xmlSAXLocator {
    return __xmlDefaultSAXLocator();
}

pub unsafe fn xmlSAX2EntityDecl_safe(
    ctx: *mut (),
    name: *const xmlChar,
    type_0: i32,
    publicId: *const xmlChar,
    systemId: *const xmlChar,
    content: *mut xmlChar,
) {
    xmlSAX2EntityDecl(ctx, name, type_0, publicId, systemId, content);
}

pub unsafe fn htmlCreateMemoryParserCtxt_safe(buffer: *const i8, size: i32) -> htmlParserCtxtPtr {
    return htmlCreateMemoryParserCtxt(buffer, size);
}

pub unsafe fn htmlInitAutoClose_safe() {
    htmlInitAutoClose();
}

pub unsafe fn xmlBuildURI_safe(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar {
    return xmlBuildURI(URI, base);
}

pub unsafe fn xmlParseURI_safe(str: *const i8) -> xmlURIPtr {
    return xmlParseURI(str);
}

pub unsafe fn xmlFreeURI_safe(uri: xmlURIPtr) {
    xmlFreeURI(uri);
}

pub unsafe fn xmlCatalogCleanup_safe() {
    xmlCatalogCleanup();
}

pub unsafe fn xmlCatalogAddLocal_safe(catalogs: *mut (), URL: *const xmlChar) -> *mut () {
    return xmlCatalogAddLocal(catalogs, URL);
}

pub unsafe fn xmlCatalogGetDefaults_safe() -> xmlCatalogAllow {
    return xmlCatalogGetDefaults();
}

pub unsafe fn xmlSchemaCleanupTypes_safe() {
    xmlSchemaCleanupTypes();
}

pub unsafe fn xmlRelaxNGCleanupTypes_safe() {
    xmlRelaxNGCleanupTypes();
}

pub unsafe fn xmlBufGetInputBase_safe(buf: xmlBufPtr, input: xmlParserInputPtr) -> size_t {
    return xmlBufGetInputBase(buf, input);
}

pub unsafe fn xmlBufSetInputBaseCur_safe(
    buf: xmlBufPtr,
    input: xmlParserInputPtr,
    base: size_t,
    cur: size_t,
) -> i32 {
    return xmlBufSetInputBaseCur(buf, input, base, cur);
}

pub unsafe fn xmlXPathInit_safe() {
    xmlXPathInit();
}

pub unsafe fn xmlParserInputGrow_safe(arg1: xmlParserInputPtr, arg2: i32) -> i32 {
    return xmlParserInputGrow(arg1, arg2);
}

pub unsafe fn xmlGetIntSubset_safe(arg1: *const xmlDoc) -> xmlDtdPtr {
    return xmlGetIntSubset(arg1);
}

pub unsafe fn xmlSwitchEncoding_safe(arg1: xmlParserCtxtPtr, arg2: xmlCharEncoding) -> i32 {
    return xmlSwitchEncoding(arg1, arg2);
}

pub unsafe fn xmlSwitchToEncoding_safe(
    arg1: xmlParserCtxtPtr,
    arg2: xmlCharEncodingHandlerPtr,
) -> i32 {
    return xmlSwitchToEncoding(arg1, arg2);
}

pub unsafe fn xmlParserInputShrink_safe(arg1: xmlParserInputPtr) {
    xmlParserInputShrink(arg1);
}

pub unsafe fn xmlCopyChar_safe(arg1: i32, arg2: *mut xmlChar, arg3: i32) -> i32 {
    return xmlCopyChar(arg1, arg2, arg3);
}

pub unsafe fn xmlNextChar_safe(arg1: xmlParserCtxtPtr) {
    xmlNextChar(arg1);
}

pub unsafe fn xmlParserAddNodeInfo_safe(arg1: xmlParserCtxtPtr, arg2: xmlParserNodeInfoPtr) {
    xmlParserAddNodeInfo(arg1, arg2);
}

pub unsafe fn nodePop_safe(arg1: xmlParserCtxtPtr) -> xmlNodePtr {
    return nodePop(arg1);
}

pub unsafe fn xmlParseCharEncoding_safe(arg1: *const i8) -> xmlCharEncoding {
    return xmlParseCharEncoding(arg1);
}

pub unsafe fn xmlPopInput_safe(arg1: xmlParserCtxtPtr) -> xmlChar {
    return xmlPopInput(arg1);
}

pub unsafe fn xmlStrncasecmp_safe(arg1: *const xmlChar, arg2: *const xmlChar, arg3: i32) -> i32 {
    return xmlStrncasecmp(arg1, arg2, arg3);
}

pub unsafe fn __htmlDefaultSAXHandler_safe() -> *mut xmlSAXHandlerV1 {
    return __htmlDefaultSAXHandler();
}

pub unsafe fn inputPush_safe(arg1: xmlParserCtxtPtr, arg2: xmlParserInputPtr) -> i32 {
    return inputPush(arg1, arg2);
}

pub unsafe fn xmlFreeParserCtxt_safe(arg1: xmlParserCtxtPtr) {
    xmlFreeParserCtxt(arg1);
}

pub unsafe fn xmlInitNodeInfoSeq_safe(arg1: xmlParserNodeInfoSeqPtr) {
    xmlInitNodeInfoSeq(arg1);
}

pub unsafe fn xmlNewInputStream_safe(arg1: xmlParserCtxtPtr) -> xmlParserInputPtr {
    return xmlNewInputStream(arg1);
}

pub unsafe fn xmlInitParser_safe() {
    xmlInitParser();
}

pub unsafe fn xmlNewStringInputStream_safe(
    arg1: xmlParserCtxtPtr,
    arg2: *const xmlChar,
) -> xmlParserInputPtr {
    return xmlNewStringInputStream(arg1, arg2);
}

pub unsafe fn xmlFreeInputStream_safe(arg1: xmlParserInputPtr) {
    xmlFreeInputStream(arg1);
}

pub unsafe fn inputPop_safe(arg1: xmlParserCtxtPtr) -> xmlParserInputPtr {
    return inputPop(arg1);
}

pub unsafe fn xmlNewIOInputStream_safe(
    arg1: xmlParserCtxtPtr,
    arg2: xmlParserInputBufferPtr,
    arg3: xmlCharEncoding,
) -> xmlParserInputPtr {
    return xmlNewIOInputStream(arg1, arg2, arg3);
}

pub unsafe fn xmlNewParserCtxt_safe() -> xmlParserCtxtPtr {
    return xmlNewParserCtxt();
}

pub unsafe fn xmlCreateMemoryParserCtxt_safe(arg1: *const i8, arg2: i32) -> xmlParserCtxtPtr {
    return xmlCreateMemoryParserCtxt(arg1, arg2);
}

pub unsafe fn strcpy_safe(arg1: *mut i8, arg2: *const i8) -> *mut i8 {
    return strcpy(arg1, arg2);
}

pub unsafe fn strcat_safe(arg1: *mut i8, arg2: *const i8) -> *mut i8 {
    return strcat(arg1, arg2);
}

pub unsafe fn __ctype_toupper_loc_safe() -> *mut *const __int32_t {
    return __ctype_toupper_loc();
}

pub unsafe fn xmlStopParser_safe(ctxt: xmlParserCtxtPtr) {
    xmlStopParser(ctxt);
}

pub unsafe fn xmlCtxtReset_safe(ctxt: xmlParserCtxtPtr) {
    xmlCtxtReset(ctxt);
}

pub unsafe fn xmlUTF8Strsize_safe(utf: *const xmlChar, len: i32) -> i32 {
    return xmlUTF8Strsize(utf, len);
}

pub unsafe fn xmlUTF8Strpos_safe(utf: *const xmlChar, pos: i32) -> *const xmlChar {
    return xmlUTF8Strpos(utf, pos);
}

pub unsafe fn xmlUTF8Strloc_safe(utf: *const xmlChar, utfchar: *const xmlChar) -> i32 {
    return xmlUTF8Strloc(utf, utfchar);
}

pub unsafe fn xmlUTF8Strsub_safe(utf: *const xmlChar, start: i32, len: i32) -> *mut xmlChar {
    return xmlUTF8Strsub(utf, start, len);
}

pub unsafe fn xmlUTF8Strlen_safe(utf: *const xmlChar) -> i32 {
    return xmlUTF8Strlen(utf);
}

pub unsafe fn xmlStrcat_safe(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar {
    return xmlStrcat(cur, add);
}

pub unsafe fn xmlStrstr_safe(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar {
    return xmlStrstr(str, val);
}

pub unsafe fn log10_safe(arg1: libc::c_double) -> libc::c_double {
    return log10(arg1);
}

pub unsafe fn pow_safe(arg1: libc::c_double, arg2: libc::c_double) -> libc::c_double {
    return pow(arg1, arg2);
}

pub unsafe fn ceil_safe(arg1: libc::c_double) -> libc::c_double {
    return ceil(arg1);
}

pub unsafe fn fabs_safe(arg1: libc::c_double) -> libc::c_double {
    return fabs(arg1);
}

pub unsafe fn floor_safe(arg1: libc::c_double) -> libc::c_double {
    return floor(arg1);
}

pub unsafe fn fmod_safe(arg1: libc::c_double, arg2: libc::c_double) -> libc::c_double {
    return fmod(arg1, arg2);
}

pub unsafe fn __isinff_safe(__value: libc::c_float) -> i32 {
    return __isinff(__value);
}

pub unsafe fn __isnanf_safe(__value: libc::c_float) -> i32 {
    return __isnanf(__value);
}

pub unsafe fn __isinfl_safe(__value: libc::c_float) -> i32 {
    return __isinfl(__value);
}

pub unsafe fn __isnanl_safe(__value: libc::c_float) -> i32 {
    return __isnanl(__value);
}

pub unsafe fn __isinf_safe(__value: libc::c_double) -> i32 {
    return __isinf(__value);
}

pub unsafe fn __isnan_safe(__value: libc::c_double) -> i32 {
    return __isnan(__value);
}

pub unsafe fn xmlDocGetRootElement_safe(doc: *const xmlDoc) -> xmlNodePtr {
    return xmlDocGetRootElement(doc);
}

pub unsafe fn xmlGetNsList_safe(doc: *const xmlDoc, node: *const xmlNode) -> *mut xmlNsPtr {
    return xmlGetNsList(doc, node);
}

pub unsafe fn xmlNodeGetContent_safe(cur: *const xmlNode) -> *mut xmlChar {
    return xmlNodeGetContent(cur);
}

pub unsafe fn xmlNodeGetLang_safe(cur: *const xmlNode) -> *mut xmlChar {
    return xmlNodeGetLang(cur);
}

pub unsafe fn xmlHashCreate_safe(size: i32) -> xmlHashTablePtr {
    return xmlHashCreate(size);
}

pub unsafe fn xmlHashAddEntry_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    userdata: *mut (),
) -> i32 {
    return xmlHashAddEntry(table, name, userdata);
}

pub unsafe fn xmlHashUpdateEntry_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    userdata: *mut (),
    f: xmlHashDeallocator,
) -> i32 {
    return xmlHashUpdateEntry(table, name, userdata, f);
}

pub unsafe fn xmlHashRemoveEntry_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    f: xmlHashDeallocator,
) -> i32 {
    return xmlHashRemoveEntry(table, name, f);
}

pub unsafe fn xmlHashLookup_safe(table: xmlHashTablePtr, name: *const xmlChar) -> *mut () {
    return xmlHashLookup(table, name);
}

pub unsafe fn xmlGetID_safe(doc: xmlDocPtr, ID: *const xmlChar) -> xmlAttrPtr {
    return xmlGetID(doc, ID);
}

pub unsafe fn realloc_safe(arg1: *mut (), arg2: u64) -> *mut () {
    return realloc(arg1, arg2);
}

pub unsafe fn free_safe(__ptr: *mut ()) {
    free(__ptr);
}

pub unsafe fn exit_safe(arg1: i32) -> ! {
    return exit(arg1);
}

pub unsafe fn xmlXPtrLocationSetCreate_safe(val: xmlXPathObjectPtr) -> xmlLocationSetPtr {
    return xmlXPtrLocationSetCreate(val);
}

pub unsafe fn xmlXPtrFreeLocationSet_safe(obj: xmlLocationSetPtr) {
    xmlXPtrFreeLocationSet(obj);
}

pub unsafe fn xmlXPtrLocationSetMerge_safe(
    val1: xmlLocationSetPtr,
    val2: xmlLocationSetPtr,
) -> xmlLocationSetPtr {
    return xmlXPtrLocationSetMerge(val1, val2);
}

pub unsafe fn xmlXPtrNewRange_safe(
    start: xmlNodePtr,
    startindex: i32,
    end: xmlNodePtr,
    endindex: i32,
) -> xmlXPathObjectPtr {
    return xmlXPtrNewRange(start, startindex, end, endindex);
}

pub unsafe fn xmlXPtrNewRangeNodeObject_safe(
    start: xmlNodePtr,
    end: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    return xmlXPtrNewRangeNodeObject(start, end);
}

pub unsafe fn xmlXPtrLocationSetAdd_safe(cur: xmlLocationSetPtr, val: xmlXPathObjectPtr) {
    xmlXPtrLocationSetAdd(cur, val);
}

pub unsafe fn xmlXPtrWrapLocationSet_safe(val: xmlLocationSetPtr) -> xmlXPathObjectPtr {
    return xmlXPtrWrapLocationSet(val);
}

pub unsafe fn xmlDebugDumpString_safe(output: *mut FILE, str: *const xmlChar) {
    xmlDebugDumpString(output, str);
}

pub unsafe fn xmlDebugDumpAttr_safe(output: *mut FILE, attr: xmlAttrPtr, depth: i32) {
    xmlDebugDumpAttr(output, attr, depth);
}

pub unsafe fn xmlDebugDumpOneNode_safe(output: *mut FILE, node: xmlNodePtr, depth: i32) {
    xmlDebugDumpOneNode(output, node, depth);
}

pub unsafe fn xmlFreePattern_safe(comp: xmlPatternPtr) {
    xmlFreePattern(comp);
}

pub unsafe fn xmlFreePatternList_safe(comp: xmlPatternPtr) {
    xmlFreePatternList(comp);
}

pub unsafe fn xmlPatterncompile_safe(
    pattern: *const xmlChar,
    dict: *mut xmlDict,
    flags: i32,
    namespaces: *mut *const xmlChar,
) -> xmlPatternPtr {
    return xmlPatterncompile(pattern, dict, flags, namespaces);
}

pub unsafe fn xmlPatternStreamable_safe(comp: xmlPatternPtr) -> i32 {
    return xmlPatternStreamable(comp);
}

pub unsafe fn xmlPatternMaxDepth_safe(comp: xmlPatternPtr) -> i32 {
    return xmlPatternMaxDepth(comp);
}

pub unsafe fn xmlPatternMinDepth_safe(comp: xmlPatternPtr) -> i32 {
    return xmlPatternMinDepth(comp);
}

pub unsafe fn xmlPatternFromRoot_safe(comp: xmlPatternPtr) -> i32 {
    return xmlPatternFromRoot(comp);
}

pub unsafe fn xmlPatternGetStreamCtxt_safe(comp: xmlPatternPtr) -> xmlStreamCtxtPtr {
    return xmlPatternGetStreamCtxt(comp);
}

pub unsafe fn xmlFreeStreamCtxt_safe(stream: xmlStreamCtxtPtr) {
    xmlFreeStreamCtxt(stream);
}

pub unsafe fn xmlStreamPushNode_safe(
    stream: xmlStreamCtxtPtr,
    name: *const xmlChar,
    ns: *const xmlChar,
    nodeType: i32,
) -> i32 {
    return xmlStreamPushNode(stream, name, ns, nodeType);
}

pub unsafe fn xmlStreamPush_safe(
    stream: xmlStreamCtxtPtr,
    name: *const xmlChar,
    ns: *const xmlChar,
) -> i32 {
    return xmlStreamPush(stream, name, ns);
}

pub unsafe fn xmlStreamPop_safe(stream: xmlStreamCtxtPtr) -> i32 {
    return xmlStreamPop(stream);
}

pub unsafe fn xmlStreamWantsAnyNode_safe(stream: xmlStreamCtxtPtr) -> i32 {
    return xmlStreamWantsAnyNode(stream);
}

pub unsafe fn xmlBufFree_safe(buf: xmlBufPtr) {
    xmlBufFree(buf);
}

pub unsafe fn xmlBufAdd_safe(buf: xmlBufPtr, str: *const xmlChar, len: i32) -> i32 {
    return xmlBufAdd(buf, str, len);
}

#[macro_export]
macro_rules! __xmlRaiseError_safe_macro {
    () => {};
    ($schannel:expr,$channel:expr, $data:expr, $ctx:expr, $node:expr,
    $domain:expr, $code:expr, $level:expr, $file:expr, $line:expr, $str1:expr,
    $str2:expr, $str3:expr, $int1:expr, $col:expr,
    $msg:expr$(, $more_params:expr)*) => {{
        unsafe { __xmlRaiseError($schannel,$channel,$data, $ctx, $node, $domain, $code, $level, $file, $line, $str1
        , $str2, $str3, $int1, $col, $msg$(, $more_params)*) }
    }};
}

#[macro_export]
macro_rules! snprintf_safe_macro {
    () => {};
    ($arg1:expr,$arg2:expr, $arg3:expr$(, $more_params:expr)*) => {{
        unsafe { snprintf($arg1, $arg2, $arg3$(, $more_params)*) }
    }};
}

#[macro_export]
macro_rules! __xmlGenericError_safe_macro {
    () => {};
    ($arg1:expr, $arg2:expr$(, $more_params:expr)*) => {{
        unsafe { (*__xmlGenericError()).expect("non-null function pointer")($arg1, $arg2$(, $more_params)*) }
    }};
}

// #[macro_export]
// macro_rules! fprintf_safe {
//     () => {return 0 as i32};
//     ($arg1:expr, $arg2:expr$(, $more_params:expr)*) => {{
//         unsafe { return fprintf($arg1, $arg2$(, $more_params)*) }
//     }};
// }
//
// #[macro_export]
// macro_rules! xmlParserValidityError_safe {
//     () => {};
//     ($ctx:expr, $msg:expr$(, $more_params:expr)*) => {{
//         unsafe { xmlParserValidityError($ctx, $msg$(, $more_params)*) }
//     }};
// }
//
// #[macro_export]
// macro_rules! xmlParserValidityWarning_safe {
//     () => {};
//     ($ctx:expr, $msg:expr$(, $more_params:expr)*) => {{
//         unsafe { xmlParserValidityWarning($ctx, $msg$(, $more_params)*) }
//     }};
// }
//
// #[macro_export]
// macro_rules! xmlGenericErrorDefaultFunc_safe {
//     () => {};
//     ($ctx:expr, $msg:expr$(, $more_params:expr)*) => {{
//         unsafe { xmlGenericErrorDefaultFunc($ctx, $msg$(, $more_params)*) }
//     }};
// }
//
// #[macro_export]
// macro_rules! xmlStrPrintf_safe {
//     () => {return 0 as i32};
//     ($buf:expr, $len:expr, $msg:expr$(, $more_params:expr)*) => {{
//         unsafe { return xmlStrPrintf($buf, $len, $msg$(, $more_params)*) }
//     }};
// }
