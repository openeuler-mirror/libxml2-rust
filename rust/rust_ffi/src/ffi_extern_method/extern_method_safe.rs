use super::super::ffi_defination::defination::*;
use super::super::ffi_extern_method::extern_method::*;

//static mut stderr: *mut _IO_FILE;
pub fn getStderr() -> *mut _IO_FILE {
    unsafe {
        return stderr;
    }
}

//static mut xmlFree: xmlFreeFunc;
pub fn xmlFree_safe(arg1: *mut ()) {
    unsafe {
        xmlFree.expect("non-null function pointer")(arg1);
    }
}

//static mut xmlMalloc: xmlMallocFunc;
pub fn xmlMalloc_safe(arg1: size_t) -> *mut () {
    unsafe {
        return xmlMalloc.expect("non-null function pointer")(arg1);
    }
}

//static mut xmlRealloc: xmlReallocFunc;
pub fn xmlRealloc_safe(arg1: *mut (), arg2: size_t) -> *mut () {
    unsafe {
        return xmlRealloc.expect("non-null function pointer")(arg1, arg2);
    }
}

//static mut xmlMallocAtomic: xmlMallocFunc;
pub fn xmlMallocAtomic_safe(arg1: size_t) -> *mut () {
    unsafe {
        return xmlMallocAtomic.expect("non-null function pointer")(arg1);
    }
}

//static xmlIsBaseCharGroup: xmlChRangeGroup;
pub fn getXmlIsBaseCharGroup() -> &'static xmlChRangeGroup {
    unsafe {
        return &xmlIsBaseCharGroup;
    }
}

//static xmlIsCombiningGroup: xmlChRangeGroup;
pub fn getXmlIsCombiningGroup() -> &'static xmlChRangeGroup {
    unsafe {
        return &xmlIsCombiningGroup;
    }
}

//static xmlIsDigitGroup: xmlChRangeGroup;
pub fn getXmlIsDigitGroup() -> &'static xmlChRangeGroup {
    unsafe {
        return &xmlIsDigitGroup;
    }
}

//static xmlIsExtenderGroup: xmlChRangeGroup;
pub fn getXmlIsExtenderGroup() -> &'static xmlChRangeGroup {
    unsafe {
        return &xmlIsExtenderGroup;
    }
}

//static xmlIsPubidChar_tab: [u8; 256];
pub fn getXmlIsPubidChar_tab(index: usize) -> i32 {
    unsafe {
        return xmlIsPubidChar_tab[index] as i32;
    }
}

pub fn xmlStrcmp_safe(str1: *const xmlChar, str2: *const xmlChar) -> i32 {
    unsafe {
        return xmlStrcmp(str1, str2);
    }
}

pub fn xmlStrdup_safe(cur: *const xmlChar) -> *mut xmlChar {
    unsafe {
        return xmlStrdup(cur);
    }
}

pub fn xmlStrlen_safe(str: *const xmlChar) -> i32 {
    unsafe {
        return xmlStrlen(str);
    }
}

pub fn __xmlLoaderErr_safe(ctx: *mut (), msg: *const i8, filename: *const i8) {
    unsafe {
        __xmlLoaderErr(ctx, msg, filename);
    }
}

pub fn memset_safe(arg1: *mut (), arg2: i32, arg3: u64) -> *mut () {
    unsafe {
        return memset(arg1, arg2, arg3);
    }
}

pub fn strcmp_safe(arg1: *const i8, arg2: *const i8) -> i32 {
    unsafe {
        return strcmp(arg1, arg2);
    }
}

pub fn xmlBufContent_safe(buf: *const xmlBuf) -> *mut xmlChar {
    unsafe {
        return xmlBufContent(buf);
    }
}

pub fn xmlBufEnd_safe(buf: xmlBufPtr) -> *mut xmlChar {
    unsafe {
        return xmlBufEnd(buf);
    }
}

pub fn xmlBufUse_safe(buf: xmlBufPtr) -> size_t {
    unsafe {
        return xmlBufUse(buf);
    }
}

pub fn xmlBufShrink_safe(buf: xmlBufPtr, len: size_t) -> size_t {
    unsafe {
        return xmlBufShrink(buf, len);
    }
}

pub fn xmlDictCreate_safe() -> xmlDictPtr {
    unsafe {
        return xmlDictCreate();
    }
}

pub fn xmlDictSetLimit_safe(dict: xmlDictPtr, limit: size_t) -> size_t {
    unsafe {
        return xmlDictSetLimit(dict, limit);
    }
}

pub fn xmlDictFree_safe(dict: xmlDictPtr) {
    unsafe {
        xmlDictFree(dict);
    }
}

// pub fn __xmlGenericError_safe() -> *mut xmlGenericErrorFunc {
//     unsafe {
//         return __xmlGenericError();
//     }
// }

pub fn xmlHashFree_safe(table: xmlHashTablePtr, f: xmlHashDeallocator) {
    unsafe {
        xmlHashFree(table, f);
    }
}

pub fn xmlHashDefaultDeallocator_safe(entry: *mut (), name: *const xmlChar) {
    unsafe {
        xmlHashDefaultDeallocator(entry, name);
    }
}

pub fn xmlGetCharEncodingHandler_safe(enc: xmlCharEncoding) -> xmlCharEncodingHandlerPtr {
    unsafe {
        return xmlGetCharEncodingHandler(enc);
    }
}

pub fn xmlCharEncCloseFunc_safe(handler: *mut xmlCharEncodingHandler) -> i32 {
    unsafe {
        return xmlCharEncCloseFunc(handler);
    }
}

pub fn xmlParserInputBufferCreateFilename_safe(
    URI: *const i8,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    unsafe {
        return xmlParserInputBufferCreateFilename(URI, enc);
    }
}

pub fn xmlParserInputBufferRead_safe(in_0: xmlParserInputBufferPtr, len: i32) -> i32 {
    unsafe {
        return xmlParserInputBufferRead(in_0, len);
    }
}

pub fn xmlParserInputBufferGrow_safe(in_0: xmlParserInputBufferPtr, len: i32) -> i32 {
    unsafe {
        return xmlParserInputBufferGrow(in_0, len);
    }
}

pub fn xmlFreeParserInputBuffer_safe(in_0: xmlParserInputBufferPtr) {
    unsafe {
        xmlFreeParserInputBuffer(in_0);
    }
}

pub fn xmlParserGetDirectory_safe(filename: *const i8) -> *mut i8 {
    unsafe {
        return xmlParserGetDirectory(filename);
    }
}

pub fn xmlCheckHTTPInput_safe(ctxt: xmlParserCtxtPtr, ret: xmlParserInputPtr) -> xmlParserInputPtr {
    unsafe {
        return xmlCheckHTTPInput(ctxt, ret);
    }
}

pub fn __xmlSubstituteEntitiesDefaultValue_safe() -> *mut i32 {
    unsafe {
        return __xmlSubstituteEntitiesDefaultValue();
    }
}

pub fn __xmlKeepBlanksDefaultValue_safe() -> i32 {
    unsafe {
        return *__xmlKeepBlanksDefaultValue();
    }
}

pub fn __xmlIndentTreeOutput_safe() -> *mut i32 {
    unsafe {
        return __xmlIndentTreeOutput();
    }
}

pub fn __xmlPedanticParserDefaultValue_safe() -> *mut i32 {
    unsafe {
        return __xmlPedanticParserDefaultValue();
    }
}

pub fn __xmlLineNumbersDefaultValue_safe() -> i32 {
    unsafe {
        return *__xmlLineNumbersDefaultValue();
    }
}

pub fn __xmlDefaultSAXHandler_safe() -> *mut xmlSAXHandlerV1 {
    unsafe {
        return __xmlDefaultSAXHandler();
    }
}

pub fn __xmlGetWarningsDefaultValue_safe() -> *mut i32 {
    unsafe {
        return __xmlGetWarningsDefaultValue();
    }
}

pub fn xmlSAX2IgnorableWhitespace_safe(ctx: *mut (), ch: *const xmlChar, len: i32) {
    unsafe {
        xmlSAX2IgnorableWhitespace(ctx, ch, len);
    }
}

pub fn __xmlDoValidityCheckingDefaultValue_safe() -> *mut i32 {
    unsafe {
        return __xmlDoValidityCheckingDefaultValue();
    }
}

pub fn __xmlLoadExtDtdDefaultValue_safe() -> *mut i32 {
    unsafe {
        return __xmlLoadExtDtdDefaultValue();
    }
}

pub fn __xmlGenericErrorContext_safe() -> *mut () {
    unsafe {
        return *__xmlGenericErrorContext();
    }
}

pub fn xmlSAXVersion_safe(hdlr: *mut xmlSAXHandler, version: i32) -> i32 {
    unsafe {
        return xmlSAXVersion(hdlr, version);
    }
}

pub fn xmlDefaultSAXHandlerInit_safe() {
    unsafe {
        xmlDefaultSAXHandlerInit();
    }
}

pub fn __xmlParserDebugEntities_safe() -> *mut i32 {
    unsafe {
        return __xmlParserDebugEntities();
    }
}

pub fn xmlLoadExternalEntity_safe(
    URL: *const i8,
    ID: *const i8,
    ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    unsafe {
        return xmlLoadExternalEntity(URL, ID, ctxt);
    }
}

pub fn xmlCharInRange_safe(val: u32, group: *const xmlChRangeGroup) -> i32 {
    unsafe {
        return xmlCharInRange(val, group);
    }
}

pub fn xmlCanonicPath_safe(path: *const xmlChar) -> *mut xmlChar {
    unsafe {
        return xmlCanonicPath(path);
    }
}

pub fn xmlCatalogFreeLocal_safe(catalogs: *mut ()) {
    unsafe {
        xmlCatalogFreeLocal(catalogs);
    }
}

pub fn xmlBufCreate_safe() -> xmlBufPtr {
    unsafe {
        return xmlBufCreate();
    }
}

pub fn xmlBufIsEmpty_safe(buf: xmlBufPtr) -> i32 {
    unsafe {
        return xmlBufIsEmpty(buf);
    }
}

pub fn xmlBufResetInput_safe(buf: xmlBufPtr, input: xmlParserInputPtr) -> i32 {
    unsafe {
        return xmlBufResetInput(buf, input);
    }
}

pub fn xmlCharEncFirstLineInput_safe(input: xmlParserInputBufferPtr, len: i32) -> i32 {
    unsafe {
        return xmlCharEncFirstLineInput(input, len);
    }
}

pub fn xmlCharEncInput_safe(input: xmlParserInputBufferPtr, flush: i32) -> i32 {
    unsafe {
        return xmlCharEncInput(input, flush);
    }
}

pub fn xmlStrncmp_safe(str1: *const xmlChar, str2: *const xmlChar, len: i32) -> i32 {
    unsafe {
        return xmlStrncmp(str1, str2, len);
    }
}

pub fn xmlStrcasecmp_safe(str1: *const xmlChar, str2: *const xmlChar) -> i32 {
    unsafe {
        return xmlStrcasecmp(str1, str2);
    }
}

pub fn xmlStrEqual_safe(str1: *const xmlChar, str2: *const xmlChar) -> i32 {
    unsafe {
        return xmlStrEqual(str1, str2);
    }
}

pub fn xmlStrcasestr_safe(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar {
    unsafe {
        return xmlStrcasestr(str, val);
    }
}

pub fn xmlStrchr_safe(str: *const xmlChar, val: xmlChar) -> *const xmlChar {
    unsafe {
        return xmlStrchr(str, val);
    }
}

pub fn xmlCharStrdup_safe(cur: *const i8) -> *mut xmlChar {
    unsafe {
        return xmlCharStrdup(cur);
    }
}

pub fn xmlStrndup_safe(cur: *const xmlChar, len: i32) -> *mut xmlChar {
    unsafe {
        return xmlStrndup(cur, len);
    }
}

pub fn __htmlParseContent_safe(ctx: *mut ()) {
    unsafe {
        __htmlParseContent(ctx);
    }
}

pub fn __xmlGlobalInitMutexLock_safe() {
    unsafe {
        __xmlGlobalInitMutexLock();
    }
}

pub fn __xmlGlobalInitMutexUnlock_safe() {
    unsafe {
        __xmlGlobalInitMutexUnlock();
    }
}

pub fn xmlInputReadCallbackNop_safe(context: *mut (), buffer: *mut i8, len: i32) -> i32 {
    unsafe {
        return xmlInputReadCallbackNop(context, buffer, len);
    }
}

pub fn memcpy_safe(arg1: *mut (), arg2: *const (), arg3: u64) -> *mut () {
    unsafe {
        return memcpy(arg1, arg2, arg3);
    }
}

pub fn memmove_safe(arg1: *mut (), arg2: *const (), arg3: u64) -> *mut () {
    unsafe {
        return memmove(arg1, arg2, arg3);
    }
}

pub fn memchr_safe(arg1: *const (), arg2: i32, arg3: u64) -> *mut () {
    unsafe {
        return memchr(arg1, arg2, arg3);
    }
}

pub fn strncmp_safe(arg1: *const i8, arg2: *const i8, arg3: u64) -> i32 {
    unsafe {
        return strncmp(arg1, arg2, arg3);
    }
}

pub fn strlen_safe(arg1: *const i8) -> u64 {
    unsafe {
        return strlen(arg1);
    }
}

pub fn xmlCleanupInputCallbacks_safe() {
    unsafe {
        xmlCleanupInputCallbacks();
    }
}

pub fn xmlRegisterDefaultInputCallbacks_safe() {
    unsafe {
        xmlRegisterDefaultInputCallbacks();
    }
}

pub fn xmlAllocParserInputBuffer_safe(enc: xmlCharEncoding) -> xmlParserInputBufferPtr {
    unsafe {
        return xmlAllocParserInputBuffer(enc);
    }
}

pub fn xmlParserInputBufferCreateFd_safe(fd: i32, enc: xmlCharEncoding) -> xmlParserInputBufferPtr {
    unsafe {
        return xmlParserInputBufferCreateFd(fd, enc);
    }
}

pub fn xmlParserInputBufferCreateMem_safe(
    mem: *const i8,
    size: i32,
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    unsafe {
        return xmlParserInputBufferCreateMem(mem, size, enc);
    }
}

pub fn xmlParserInputBufferCreateIO_safe(
    ioread: xmlInputReadCallback,
    ioclose: xmlInputCloseCallback,
    ioctx: *mut (),
    enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    unsafe {
        return xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, enc);
    }
}

pub fn xmlParserInputBufferPush_safe(
    in_0: xmlParserInputBufferPtr,
    len: i32,
    buf: *const i8,
) -> i32 {
    unsafe {
        return xmlParserInputBufferPush(in_0, len, buf);
    }
}

pub fn xmlCleanupOutputCallbacks_safe() {
    unsafe {
        xmlCleanupOutputCallbacks();
    }
}

pub fn xmlRegisterDefaultOutputCallbacks_safe() {
    unsafe {
        xmlRegisterDefaultOutputCallbacks();
    }
}

pub fn xmlInitializeDict_safe() -> i32 {
    unsafe {
        return xmlInitializeDict();
    }
}

pub fn xmlDictReference_safe(dict: xmlDictPtr) -> i32 {
    unsafe {
        return xmlDictReference(dict);
    }
}

pub fn xmlDictLookup_safe(dict: xmlDictPtr, name: *const xmlChar, len: i32) -> *const xmlChar {
    unsafe {
        return xmlDictLookup(dict, name, len);
    }
}

pub fn xmlDictOwns_safe(dict: xmlDictPtr, str: *const xmlChar) -> i32 {
    unsafe {
        return xmlDictOwns(dict, str);
    }
}

pub fn xmlDictCleanup_safe() {
    unsafe {
        xmlDictCleanup();
    }
}

pub fn xmlBuildQName_safe(
    ncname: *const xmlChar,
    prefix: *const xmlChar,
    memory: *mut xmlChar,
    len: i32,
) -> *mut xmlChar {
    unsafe {
        return xmlBuildQName(ncname, prefix, memory, len);
    }
}

pub fn xmlSplitQName3_safe(name: *const xmlChar, len: *mut i32) -> *const xmlChar {
    unsafe {
        return xmlSplitQName3(name, len);
    }
}

pub fn xmlBufferCreate_safe() -> xmlBufferPtr {
    unsafe {
        return xmlBufferCreate();
    }
}

pub fn xmlBufferFree_safe(buf: xmlBufferPtr) {
    unsafe {
        xmlBufferFree(buf);
    }
}

pub fn xmlBufferAdd_safe(buf: xmlBufferPtr, str: *const xmlChar, len: i32) -> i32 {
    unsafe {
        return xmlBufferAdd(buf, str, len);
    }
}

pub fn xmlCreateIntSubset_safe(
    doc: xmlDocPtr,
    name: *const xmlChar,
    ExternalID: *const xmlChar,
    SystemID: *const xmlChar,
) -> xmlDtdPtr {
    unsafe {
        return xmlCreateIntSubset(doc, name, ExternalID, SystemID);
    }
}

pub fn xmlNewDtd_safe(
    doc: xmlDocPtr,
    name: *const xmlChar,
    ExternalID: *const xmlChar,
    SystemID: *const xmlChar,
) -> xmlDtdPtr {
    unsafe {
        return xmlNewDtd(doc, name, ExternalID, SystemID);
    }
}

pub fn xmlNewDoc_safe(version: *const xmlChar) -> xmlDocPtr {
    unsafe {
        return xmlNewDoc(version);
    }
}

pub fn xmlFreeDoc_safe(cur: xmlDocPtr) {
    unsafe {
        xmlFreeDoc(cur);
    }
}

pub fn xmlNewDocNode_safe(
    doc: xmlDocPtr,
    ns: xmlNsPtr,
    name: *const xmlChar,
    content: *const xmlChar,
) -> xmlNodePtr {
    unsafe {
        return xmlNewDocNode(doc, ns, name, content);
    }
}

pub fn xmlNewComment_safe(content: *const xmlChar) -> xmlNodePtr {
    unsafe {
        return xmlNewComment(content);
    }
}

pub fn xmlDocCopyNode_safe(node: xmlNodePtr, doc: xmlDocPtr, recursive: i32) -> xmlNodePtr {
    unsafe {
        return xmlDocCopyNode(node, doc, recursive);
    }
}

pub fn xmlGetLastChild_safe(parent: *const xmlNode) -> xmlNodePtr {
    unsafe {
        return xmlGetLastChild(parent);
    }
}

pub fn xmlNodeIsText_safe(node: *const xmlNode) -> i32 {
    unsafe {
        return xmlNodeIsText(node);
    }
}

pub fn xmlAddChild_safe(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr {
    unsafe {
        return xmlAddChild(parent, cur);
    }
}

pub fn xmlAddChildList_safe(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr {
    unsafe {
        return xmlAddChildList(parent, cur);
    }
}

pub fn xmlUnlinkNode_safe(cur: xmlNodePtr) {
    unsafe {
        xmlUnlinkNode(cur);
    }
}

pub fn xmlFreeNodeList_safe(cur: xmlNodePtr) {
    unsafe {
        xmlFreeNodeList(cur);
    }
}

pub fn xmlFreeNode_safe(cur: xmlNodePtr) {
    unsafe {
        xmlFreeNode(cur);
    }
}

pub fn xmlSetTreeDoc_safe(tree: xmlNodePtr, doc: xmlDocPtr) {
    unsafe {
        xmlSetTreeDoc(tree, doc);
    }
}

pub fn xmlSearchNsByHref_safe(doc: xmlDocPtr, node: xmlNodePtr, href: *const xmlChar) -> xmlNsPtr {
    unsafe {
        return xmlSearchNsByHref(doc, node, href);
    }
}

pub fn xmlHashCreateDict_safe(size: i32, dict: xmlDictPtr) -> xmlHashTablePtr {
    unsafe {
        return xmlHashCreateDict(size, dict);
    }
}

pub fn xmlHashAddEntry2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
    userdata: *mut (),
) -> i32 {
    unsafe {
        return xmlHashAddEntry2(table, name, name2, userdata);
    }
}

pub fn xmlHashUpdateEntry2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
    userdata: *mut (),
    f: xmlHashDeallocator,
) -> i32 {
    unsafe {
        return xmlHashUpdateEntry2(table, name, name2, userdata, f);
    }
}

pub fn xmlHashRemoveEntry2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
    f: xmlHashDeallocator,
) -> i32 {
    unsafe {
        return xmlHashRemoveEntry2(table, name, name2, f);
    }
}

pub fn xmlHashLookup2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    name2: *const xmlChar,
) -> *mut () {
    unsafe {
        return xmlHashLookup2(table, name, name2);
    }
}

pub fn xmlHashQLookup2_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    prefix: *const xmlChar,
    name2: *const xmlChar,
    prefix2: *const xmlChar,
) -> *mut () {
    unsafe {
        return xmlHashQLookup2(table, name, prefix, name2, prefix2);
    }
}

pub fn xmlHashSize_safe(table: xmlHashTablePtr) -> i32 {
    unsafe {
        return xmlHashSize(table);
    }
}

pub fn xmlHashScanFull_safe(table: xmlHashTablePtr, f: xmlHashScannerFull, data: *mut ()) {
    unsafe {
        xmlHashScanFull(table, f, data);
    }
}

pub fn initGenericErrorDefaultFunc_safe(handler: *mut xmlGenericErrorFunc) {
    unsafe {
        initGenericErrorDefaultFunc(handler);
    }
}

pub fn xmlResetLastError_safe() {
    unsafe {
        xmlResetLastError();
    }
}

pub fn xmlResetError_safe(err: xmlErrorPtr) {
    unsafe {
        xmlResetError(err);
    }
}

pub fn xmlCopyError_safe(from: xmlErrorPtr, to: xmlErrorPtr) -> i32 {
    unsafe {
        return xmlCopyError(from, to);
    }
}

pub fn xmlNewDocElementContent_safe(
    doc: xmlDocPtr,
    name: *const xmlChar,
    type_0: xmlElementContentType,
) -> xmlElementContentPtr {
    unsafe {
        return xmlNewDocElementContent(doc, name, type_0);
    }
}

pub fn xmlFreeDocElementContent_safe(doc: xmlDocPtr, cur: xmlElementContentPtr) {
    unsafe {
        xmlFreeDocElementContent(doc, cur);
    }
}

pub fn xmlCreateEnumeration_safe(name: *const xmlChar) -> xmlEnumerationPtr {
    unsafe {
        return xmlCreateEnumeration(name);
    }
}

pub fn xmlFreeEnumeration_safe(cur: xmlEnumerationPtr) {
    unsafe {
        xmlFreeEnumeration(cur);
    }
}

pub fn xmlValidateRoot_safe(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> i32 {
    unsafe {
        return xmlValidateRoot(ctxt, doc);
    }
}

pub fn xmlValidateElement_safe(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, elem: xmlNodePtr) -> i32 {
    unsafe {
        return xmlValidateElement(ctxt, doc, elem);
    }
}

pub fn xmlIsMixedElement_safe(doc: xmlDocPtr, name: *const xmlChar) -> i32 {
    unsafe {
        return xmlIsMixedElement(doc, name);
    }
}

pub fn xmlGetPredefinedEntity_safe(name: *const xmlChar) -> xmlEntityPtr {
    unsafe {
        return xmlGetPredefinedEntity(name);
    }
}

pub fn xmlInitCharEncodingHandlers_safe() {
    unsafe {
        xmlInitCharEncodingHandlers();
    }
}

pub fn xmlCleanupCharEncodingHandlers_safe() {
    unsafe {
        xmlCleanupCharEncodingHandlers();
    }
}

pub fn xmlFindCharEncodingHandler_safe(name: *const i8) -> xmlCharEncodingHandlerPtr {
    unsafe {
        return xmlFindCharEncodingHandler(name);
    }
}

pub fn xmlDetectCharEncoding_safe(in_0: *const u8, len: i32) -> xmlCharEncoding {
    unsafe {
        return xmlDetectCharEncoding(in_0, len);
    }
}

pub fn xmlCleanupMemory_safe() {
    unsafe {
        xmlCleanupMemory();
    }
}

pub fn xmlInitMemory_safe() -> i32 {
    unsafe {
        return xmlInitMemory();
    }
}

pub fn htmlDefaultSAXHandlerInit_safe() {
    unsafe {
        htmlDefaultSAXHandlerInit();
    }
}

pub fn xmlInitGlobals_safe() {
    unsafe {
        xmlInitGlobals();
    }
}

pub fn xmlInitThreads_safe() {
    unsafe {
        xmlInitThreads();
    }
}

pub fn xmlCleanupThreads_safe() {
    unsafe {
        xmlCleanupThreads();
    }
}

pub fn xmlCleanupGlobals_safe() {
    unsafe {
        xmlCleanupGlobals();
    }
}

pub fn xmlSAX2GetEntity_safe(ctx: *mut (), name: *const xmlChar) -> xmlEntityPtr {
    unsafe {
        return xmlSAX2GetEntity(ctx, name);
    }
}

pub fn xmlSAX2StartElement_safe(ctx: *mut (), fullname: *const xmlChar, atts: *mut *const xmlChar) {
    unsafe {
        xmlSAX2StartElement(ctx, fullname, atts);
    }
}

pub fn xmlSAX2EndElement_safe(ctx: *mut (), name: *const xmlChar) {
    unsafe {
        xmlSAX2EndElement(ctx, name);
    }
}

pub fn __xmlDefaultSAXLocator_safe() -> *mut xmlSAXLocator {
    unsafe {
        return __xmlDefaultSAXLocator();
    }
}

pub fn xmlSAX2EntityDecl_safe(
    ctx: *mut (),
    name: *const xmlChar,
    type_0: i32,
    publicId: *const xmlChar,
    systemId: *const xmlChar,
    content: *mut xmlChar,
) {
    unsafe {
        xmlSAX2EntityDecl(ctx, name, type_0, publicId, systemId, content);
    }
}

pub fn htmlCreateMemoryParserCtxt_safe(buffer: *const i8, size: i32) -> htmlParserCtxtPtr {
    unsafe {
        return htmlCreateMemoryParserCtxt(buffer, size);
    }
}

pub fn htmlInitAutoClose_safe() {
    unsafe {
        htmlInitAutoClose();
    }
}

pub fn xmlBuildURI_safe(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar {
    unsafe {
        return xmlBuildURI(URI, base);
    }
}

pub fn xmlParseURI_safe(str: *const i8) -> xmlURIPtr {
    unsafe {
        return xmlParseURI(str);
    }
}

pub fn xmlFreeURI_safe(uri: xmlURIPtr) {
    unsafe {
        xmlFreeURI(uri);
    }
}

pub fn xmlCatalogCleanup_safe() {
    unsafe {
        xmlCatalogCleanup();
    }
}

pub fn xmlCatalogAddLocal_safe(catalogs: *mut (), URL: *const xmlChar) -> *mut () {
    unsafe {
        return xmlCatalogAddLocal(catalogs, URL);
    }
}

pub fn xmlCatalogGetDefaults_safe() -> xmlCatalogAllow {
    unsafe {
        return xmlCatalogGetDefaults();
    }
}

pub fn xmlSchemaCleanupTypes_safe() {
    unsafe {
        xmlSchemaCleanupTypes();
    }
}

pub fn xmlRelaxNGCleanupTypes_safe() {
    unsafe {
        xmlRelaxNGCleanupTypes();
    }
}

pub fn xmlBufGetInputBase_safe(buf: xmlBufPtr, input: xmlParserInputPtr) -> size_t {
    unsafe {
        return xmlBufGetInputBase(buf, input);
    }
}

pub fn xmlBufSetInputBaseCur_safe(
    buf: xmlBufPtr,
    input: xmlParserInputPtr,
    base: size_t,
    cur: size_t,
) -> i32 {
    unsafe {
        return xmlBufSetInputBaseCur(buf, input, base, cur);
    }
}

pub fn xmlXPathInit_safe() {
    unsafe {
        xmlXPathInit();
    }
}

pub fn xmlParserInputGrow_safe(arg1: xmlParserInputPtr, arg2: i32) -> i32 {
    unsafe {
        return xmlParserInputGrow(arg1, arg2);
    }
}

pub fn xmlGetIntSubset_safe(arg1: *const xmlDoc) -> xmlDtdPtr {
    unsafe {
        return xmlGetIntSubset(arg1);
    }
}

pub fn xmlSwitchEncoding_safe(arg1: xmlParserCtxtPtr, arg2: xmlCharEncoding) -> i32 {
    unsafe {
        return xmlSwitchEncoding(arg1, arg2);
    }
}

pub fn xmlSwitchToEncoding_safe(arg1: xmlParserCtxtPtr, arg2: xmlCharEncodingHandlerPtr) -> i32 {
    unsafe {
        return xmlSwitchToEncoding(arg1, arg2);
    }
}

pub fn xmlParserInputShrink_safe(arg1: xmlParserInputPtr) {
    unsafe {
        xmlParserInputShrink(arg1);
    }
}

pub fn xmlCopyChar_safe(arg1: i32, arg2: *mut xmlChar, arg3: i32) -> i32 {
    unsafe {
        return xmlCopyChar(arg1, arg2, arg3);
    }
}

pub fn xmlNextChar_safe(arg1: xmlParserCtxtPtr) {
    unsafe {
        xmlNextChar(arg1);
    }
}

pub fn xmlParserAddNodeInfo_safe(arg1: xmlParserCtxtPtr, arg2: xmlParserNodeInfoPtr) {
    unsafe {
        xmlParserAddNodeInfo(arg1, arg2);
    }
}

pub fn nodePop_safe(arg1: xmlParserCtxtPtr) -> xmlNodePtr {
    unsafe {
        return nodePop(arg1);
    }
}

pub fn xmlParseCharEncoding_safe(arg1: *const i8) -> xmlCharEncoding {
    unsafe {
        return xmlParseCharEncoding(arg1);
    }
}

pub fn xmlPopInput_safe(arg1: xmlParserCtxtPtr) -> xmlChar {
    unsafe {
        return xmlPopInput(arg1);
    }
}

pub fn xmlStrncasecmp_safe(arg1: *const xmlChar, arg2: *const xmlChar, arg3: i32) -> i32 {
    unsafe {
        return xmlStrncasecmp(arg1, arg2, arg3);
    }
}

pub fn __htmlDefaultSAXHandler_safe() -> *mut xmlSAXHandlerV1 {
    unsafe {
        return __htmlDefaultSAXHandler();
    }
}

pub fn inputPush_safe(arg1: xmlParserCtxtPtr, arg2: xmlParserInputPtr) -> i32 {
    unsafe {
        return inputPush(arg1, arg2);
    }
}

pub fn xmlFreeParserCtxt_safe(arg1: xmlParserCtxtPtr) {
    unsafe {
        xmlFreeParserCtxt(arg1);
    }
}

pub fn xmlInitNodeInfoSeq_safe(arg1: xmlParserNodeInfoSeqPtr) {
    unsafe {
        xmlInitNodeInfoSeq(arg1);
    }
}

pub fn xmlNewInputStream_safe(arg1: xmlParserCtxtPtr) -> xmlParserInputPtr {
    unsafe {
        return xmlNewInputStream(arg1);
    }
}

pub fn xmlInitParser_safe() {
    unsafe {
        xmlInitParser();
    }
}

pub fn xmlNewStringInputStream_safe(
    arg1: xmlParserCtxtPtr,
    arg2: *const xmlChar,
) -> xmlParserInputPtr {
    unsafe {
        return xmlNewStringInputStream(arg1, arg2);
    }
}

pub fn xmlFreeInputStream_safe(arg1: xmlParserInputPtr) {
    unsafe {
        xmlFreeInputStream(arg1);
    }
}

pub fn inputPop_safe(arg1: xmlParserCtxtPtr) -> xmlParserInputPtr {
    unsafe {
        return inputPop(arg1);
    }
}

pub fn xmlNewIOInputStream_safe(
    arg1: xmlParserCtxtPtr,
    arg2: xmlParserInputBufferPtr,
    arg3: xmlCharEncoding,
) -> xmlParserInputPtr {
    unsafe {
        return xmlNewIOInputStream(arg1, arg2, arg3);
    }
}

pub fn xmlNewParserCtxt_safe() -> xmlParserCtxtPtr {
    unsafe {
        return xmlNewParserCtxt();
    }
}

pub fn xmlCreateMemoryParserCtxt_safe(arg1: *const i8, arg2: i32) -> xmlParserCtxtPtr {
    unsafe {
        return xmlCreateMemoryParserCtxt(arg1, arg2);
    }
}

pub fn strcpy_safe(arg1: *mut i8, arg2: *const i8) -> *mut i8 {
    unsafe {
        return strcpy(arg1, arg2);
    }
}

pub fn strcat_safe(arg1: *mut i8, arg2: *const i8) -> *mut i8 {
    unsafe {
        return strcat(arg1, arg2);
    }
}

pub fn __ctype_toupper_loc_safe() -> *mut *const __int32_t {
    unsafe {
        return __ctype_toupper_loc();
    }
}

pub fn xmlStopParser_safe(ctxt: xmlParserCtxtPtr) {
    unsafe {
        xmlStopParser(ctxt);
    }
}

pub fn xmlCtxtReset_safe(ctxt: xmlParserCtxtPtr) {
    unsafe {
        xmlCtxtReset(ctxt);
    }
}

pub fn xmlUTF8Strsize_safe(utf: *const xmlChar, len: i32) -> i32 {
    unsafe {
        return xmlUTF8Strsize(utf, len);
    }
}

pub fn xmlUTF8Strpos_safe(utf: *const xmlChar, pos: i32) -> *const xmlChar {
    unsafe {
        return xmlUTF8Strpos(utf, pos);
    }
}

pub fn xmlUTF8Strloc_safe(utf: *const xmlChar, utfchar: *const xmlChar) -> i32 {
    unsafe {
        return xmlUTF8Strloc(utf, utfchar);
    }
}

pub fn xmlUTF8Strsub_safe(utf: *const xmlChar, start: i32, len: i32) -> *mut xmlChar {
    unsafe {
        return xmlUTF8Strsub(utf, start, len);
    }
}

pub fn xmlUTF8Strlen_safe(utf: *const xmlChar) -> i32 {
    unsafe {
        return xmlUTF8Strlen(utf);
    }
}

pub fn xmlStrcat_safe(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar {
    unsafe {
        return xmlStrcat(cur, add);
    }
}

pub fn xmlStrstr_safe(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar {
    unsafe {
        return xmlStrstr(str, val);
    }
}

pub fn log10_safe(arg1: libc::c_double) -> libc::c_double {
    unsafe {
        return log10(arg1);
    }
}

pub fn pow_safe(arg1: libc::c_double, arg2: libc::c_double) -> libc::c_double {
    unsafe {
        return pow(arg1, arg2);
    }
}

pub fn ceil_safe(arg1: libc::c_double) -> libc::c_double {
    unsafe {
        return ceil(arg1);
    }
}

pub fn fabs_safe(arg1: libc::c_double) -> libc::c_double {
    unsafe {
        return fabs(arg1);
    }
}

pub fn floor_safe(arg1: libc::c_double) -> libc::c_double {
    unsafe {
        return floor(arg1);
    }
}

pub fn fmod_safe(arg1: libc::c_double, arg2: libc::c_double) -> libc::c_double {
    unsafe {
        return fmod(arg1, arg2);
    }
}

pub fn __isinff_safe(__value: libc::c_float) -> i32 {
    unsafe {
        return __isinff(__value);
    }
}

pub fn __isnanf_safe(__value: libc::c_float) -> i32 {
    unsafe {
        return __isnanf(__value);
    }
}

pub fn __isinfl_safe(__value: libc::c_float) -> i32 {
    unsafe {
        return __isinfl(__value);
    }
}

pub fn __isnanl_safe(__value: libc::c_float) -> i32 {
    unsafe {
        return __isnanl(__value);
    }
}

pub fn __isinf_safe(__value: libc::c_double) -> i32 {
    unsafe {
        return __isinf(__value);
    }
}

pub fn __isnan_safe(__value: libc::c_double) -> i32 {
    unsafe {
        return __isnan(__value);
    }
}

pub fn xmlDocGetRootElement_safe(doc: *const xmlDoc) -> xmlNodePtr {
    unsafe {
        return xmlDocGetRootElement(doc);
    }
}

pub fn xmlGetNsList_safe(doc: *const xmlDoc, node: *const xmlNode) -> *mut xmlNsPtr {
    unsafe {
        return xmlGetNsList(doc, node);
    }
}

pub fn xmlNodeGetContent_safe(cur: *const xmlNode) -> *mut xmlChar {
    unsafe {
        return xmlNodeGetContent(cur);
    }
}

pub fn xmlNodeGetLang_safe(cur: *const xmlNode) -> *mut xmlChar {
    unsafe {
        return xmlNodeGetLang(cur);
    }
}

pub fn xmlHashCreate_safe(size: i32) -> xmlHashTablePtr {
    unsafe {
        return xmlHashCreate(size);
    }
}

pub fn xmlHashAddEntry_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    userdata: *mut (),
) -> i32 {
    unsafe {
        return xmlHashAddEntry(table, name, userdata);
    }
}

pub fn xmlHashUpdateEntry_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    userdata: *mut (),
    f: xmlHashDeallocator,
) -> i32 {
    unsafe {
        return xmlHashUpdateEntry(table, name, userdata, f);
    }
}

pub fn xmlHashRemoveEntry_safe(
    table: xmlHashTablePtr,
    name: *const xmlChar,
    f: xmlHashDeallocator,
) -> i32 {
    unsafe {
        return xmlHashRemoveEntry(table, name, f);
    }
}

pub fn xmlHashLookup_safe(table: xmlHashTablePtr, name: *const xmlChar) -> *mut () {
    unsafe {
        return xmlHashLookup(table, name);
    }
}

pub fn xmlGetID_safe(doc: xmlDocPtr, ID: *const xmlChar) -> xmlAttrPtr {
    unsafe {
        return xmlGetID(doc, ID);
    }
}

pub fn realloc_safe(arg1: *mut (), arg2: u64) -> *mut () {
    unsafe {
        return realloc(arg1, arg2);
    }
}

pub fn free_safe(__ptr: *mut ()) {
    unsafe {
        free(__ptr);
    }
}

pub fn exit_safe(arg1: i32) -> ! {
    unsafe {
        return exit(arg1);
    }
}

pub fn xmlXPtrLocationSetCreate_safe(val: xmlXPathObjectPtr) -> xmlLocationSetPtr {
    unsafe {
        return xmlXPtrLocationSetCreate(val);
    }
}

pub fn xmlXPtrFreeLocationSet_safe(obj: xmlLocationSetPtr) {
    unsafe {
        xmlXPtrFreeLocationSet(obj);
    }
}

pub fn xmlXPtrLocationSetMerge_safe(
    val1: xmlLocationSetPtr,
    val2: xmlLocationSetPtr,
) -> xmlLocationSetPtr {
    unsafe {
        return xmlXPtrLocationSetMerge(val1, val2);
    }
}

pub fn xmlXPtrNewRange_safe(
    start: xmlNodePtr,
    startindex: i32,
    end: xmlNodePtr,
    endindex: i32,
) -> xmlXPathObjectPtr {
    unsafe {
        return xmlXPtrNewRange(start, startindex, end, endindex);
    }
}

pub fn xmlXPtrNewRangeNodeObject_safe(
    start: xmlNodePtr,
    end: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    unsafe {
        return xmlXPtrNewRangeNodeObject(start, end);
    }
}

pub fn xmlXPtrLocationSetAdd_safe(cur: xmlLocationSetPtr, val: xmlXPathObjectPtr) {
    unsafe {
        xmlXPtrLocationSetAdd(cur, val);
    }
}

pub fn xmlXPtrWrapLocationSet_safe(val: xmlLocationSetPtr) -> xmlXPathObjectPtr {
    unsafe {
        return xmlXPtrWrapLocationSet(val);
    }
}

pub fn xmlDebugDumpString_safe(output: *mut FILE, str: *const xmlChar) {
    unsafe {
        xmlDebugDumpString(output, str);
    }
}

pub fn xmlDebugDumpAttr_safe(output: *mut FILE, attr: xmlAttrPtr, depth: i32) {
    unsafe {
        xmlDebugDumpAttr(output, attr, depth);
    }
}

pub fn xmlDebugDumpOneNode_safe(output: *mut FILE, node: xmlNodePtr, depth: i32) {
    unsafe {
        xmlDebugDumpOneNode(output, node, depth);
    }
}

pub fn xmlFreePattern_safe(comp: xmlPatternPtr) {
    unsafe {
        xmlFreePattern(comp);
    }
}

pub fn xmlFreePatternList_safe(comp: xmlPatternPtr) {
    unsafe {
        xmlFreePatternList(comp);
    }
}

pub fn xmlPatterncompile_safe(
    pattern: *const xmlChar,
    dict: *mut xmlDict,
    flags: i32,
    namespaces: *mut *const xmlChar,
) -> xmlPatternPtr {
    unsafe {
        return xmlPatterncompile(pattern, dict, flags, namespaces);
    }
}

pub fn xmlPatternStreamable_safe(comp: xmlPatternPtr) -> i32 {
    unsafe {
        return xmlPatternStreamable(comp);
    }
}

pub fn xmlPatternMaxDepth_safe(comp: xmlPatternPtr) -> i32 {
    unsafe {
        return xmlPatternMaxDepth(comp);
    }
}

pub fn xmlPatternMinDepth_safe(comp: xmlPatternPtr) -> i32 {
    unsafe {
        return xmlPatternMinDepth(comp);
    }
}

pub fn xmlPatternFromRoot_safe(comp: xmlPatternPtr) -> i32 {
    unsafe {
        return xmlPatternFromRoot(comp);
    }
}

pub fn xmlPatternGetStreamCtxt_safe(comp: xmlPatternPtr) -> xmlStreamCtxtPtr {
    unsafe {
        return xmlPatternGetStreamCtxt(comp);
    }
}

pub fn xmlFreeStreamCtxt_safe(stream: xmlStreamCtxtPtr) {
    unsafe {
        xmlFreeStreamCtxt(stream);
    }
}

pub fn xmlStreamPushNode_safe(
    stream: xmlStreamCtxtPtr,
    name: *const xmlChar,
    ns: *const xmlChar,
    nodeType: i32,
) -> i32 {
    unsafe {
        return xmlStreamPushNode(stream, name, ns, nodeType);
    }
}

pub fn xmlStreamPush_safe(
    stream: xmlStreamCtxtPtr,
    name: *const xmlChar,
    ns: *const xmlChar,
) -> i32 {
    unsafe {
        return xmlStreamPush(stream, name, ns);
    }
}

pub fn xmlStreamPop_safe(stream: xmlStreamCtxtPtr) -> i32 {
    unsafe {
        return xmlStreamPop(stream);
    }
}

pub fn xmlStreamWantsAnyNode_safe(stream: xmlStreamCtxtPtr) -> i32 {
    unsafe {
        return xmlStreamWantsAnyNode(stream);
    }
}

pub fn xmlBufFree_safe(buf: xmlBufPtr) {
    unsafe {
        xmlBufFree(buf);
    }
}

pub fn xmlBufAdd_safe(buf: xmlBufPtr, str: *const xmlChar, len: i32) -> i32 {
    unsafe {
        return xmlBufAdd(buf, str, len);
    }
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
