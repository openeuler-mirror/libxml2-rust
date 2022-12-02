#[no_mangle]
pub unsafe extern "C" fn htmlInitAutoClose_rust() {
    unsafe {
        htmlInitAutoClose_htmlparser();
    }
}

#[no_mangle]
pub unsafe extern "C" fn htmlTagLookup_rust(mut tag: *const xmlChar) -> *const htmlElemDesc {
    let res: *const htmlElemDesc = unsafe { htmlTagLookup(tag) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlAutoCloseTag_rust(
    mut doc: htmlDocPtr,
    mut name: *const xmlChar,
    mut elem: htmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { htmlAutoCloseTag(doc, name, elem) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlIsAutoClosed_rust(mut doc: htmlDocPtr, mut elem: htmlNodePtr) -> i32 {
    let res: i32 = unsafe { htmlIsAutoClosed(doc, elem) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlIsScriptAttribute_rust(mut name: *const xmlChar) -> i32 {
    let res: i32 = unsafe { htmlIsScriptAttribute(name) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlEntityLookup_rust(mut name: *const xmlChar) -> *const htmlEntityDesc {
    let res: *const htmlEntityDesc = unsafe { htmlEntityLookup(name) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlEntityValueLookup_rust(mut value: u32) -> *const htmlEntityDesc {
    let res: *const htmlEntityDesc = unsafe { htmlEntityValueLookup(value) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn UTF8ToHtml_rust(
    mut out: *mut u8,
    mut outlen: *mut i32,
    mut in_0: *const u8,
    mut inlen: *mut i32,
) -> i32 {
    let res: i32 = unsafe { UTF8ToHtml(out, outlen, in_0, inlen) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlEncodeEntities_rust(
    mut out: *mut u8,
    mut outlen: *mut i32,
    mut in_0: *const u8,
    mut inlen: *mut i32,
    mut quoteChar: i32,
) -> i32 {
    let res: i32 = unsafe { htmlEncodeEntities(out, outlen, in_0, inlen, quoteChar) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlNewDocNoDtD_rust(
    mut URI: *const xmlChar,
    mut ExternalID: *const xmlChar,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlNewDocNoDtD(URI, ExternalID) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlNewDoc_rust(
    mut URI: *const xmlChar,
    mut ExternalID: *const xmlChar,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlNewDoc(URI, ExternalID) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlParseEntityRef_rust(
    mut ctxt: htmlParserCtxtPtr,
    mut str: *mut *const xmlChar,
) -> *const htmlEntityDesc {
    let res: *const htmlEntityDesc = unsafe { htmlParseEntityRef(ctxt, str) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlParseCharRef_rust(mut ctxt: htmlParserCtxtPtr) -> i32 {
    let res: i32 = unsafe { htmlParseCharRef(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlParseElement_rust(mut ctxt: htmlParserCtxtPtr) {
    unsafe {
        htmlParseElement(ctxt);
    }
}

#[no_mangle]
pub unsafe extern "C" fn __htmlParseContent_rust(mut ctxt: *mut ()) {
    unsafe {
        __htmlParseContent_htmlparser(ctxt);
    }
}

#[no_mangle]
pub unsafe extern "C" fn htmlParseDocument_rust(mut ctxt: htmlParserCtxtPtr) -> i32 {
    let res: i32 = unsafe { htmlParseDocument(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlFreeParserCtxt_rust(mut ctxt: htmlParserCtxtPtr) {
    unsafe {
        htmlFreeParserCtxt(ctxt);
    }
}

#[no_mangle]
pub unsafe extern "C" fn htmlNewParserCtxt_rust() -> htmlParserCtxtPtr {
    let res: htmlParserCtxtPtr = unsafe { htmlNewParserCtxt() };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlCreateMemoryParserCtxt_rust(
    mut buffer: *const i8,
    mut size: i32,
) -> htmlParserCtxtPtr {
    let res: htmlParserCtxtPtr = unsafe { htmlCreateMemoryParserCtxt_htmlparser(buffer, size) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlParseChunk_rust(
    mut ctxt: htmlParserCtxtPtr,
    mut chunk: *const i8,
    mut size: i32,
    mut terminate: i32,
) -> i32 {
    let mut res: i32 = 0 as i32;
    match () {
        #[cfg(LIBXML_PUSH_ENABLED)]
        _ => {
            res = unsafe { htmlParseChunk(ctxt, chunk, size, terminate) };
        }
        #[cfg(not(LIBXML_PUSH_ENABLED))]
        _ => {}
    };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlCreatePushParserCtxt_rust(
    mut sax: htmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut chunk: *const i8,
    mut size: i32,
    mut filename: *const i8,
    mut enc: xmlCharEncoding,
) -> htmlParserCtxtPtr {
    let mut res: htmlParserCtxtPtr = 0 as htmlParserCtxtPtr;
    match () {
        #[cfg(LIBXML_PUSH_ENABLED)]
        _ => {
            res = unsafe { htmlCreatePushParserCtxt(sax, user_data, chunk, size, filename, enc) };
        }
        #[cfg(not(LIBXML_PUSH_ENABLED))]
        _ => {}
    };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlSAXParseDoc_rust(
    mut cur: *const xmlChar,
    mut encoding: *const i8,
    mut sax: htmlSAXHandlerPtr,
    mut userData: *mut (),
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlSAXParseDoc(cur, encoding, sax, userData) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlParseDoc_rust(
    mut cur: *const xmlChar,
    mut encoding: *const i8,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlParseDoc(cur, encoding) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlCreateFileParserCtxt_rust(
    mut filename: *const i8,
    mut encoding: *const i8,
) -> htmlParserCtxtPtr {
    let res: htmlParserCtxtPtr = unsafe { htmlCreateFileParserCtxt(filename, encoding) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlSAXParseFile_rust(
    mut filename: *const i8,
    mut encoding: *const i8,
    mut sax: htmlSAXHandlerPtr,
    mut userData: *mut (),
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlSAXParseFile(filename, encoding, sax, userData) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlParseFile_rust(
    mut filename: *const i8,
    mut encoding: *const i8,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlParseFile(filename, encoding) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlHandleOmittedElem_rust(mut val: i32) -> i32 {
    let res: i32 = unsafe { htmlHandleOmittedElem(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlElementAllowedHere_rust(
    mut parent: *const htmlElemDesc,
    mut elt: *const xmlChar,
) -> i32 {
    let res: i32 = unsafe { htmlElementAllowedHere(parent, elt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlElementStatusHere_rust(
    mut parent: *const htmlElemDesc,
    mut elt: *const htmlElemDesc,
) -> htmlStatus {
    let res: htmlStatus = unsafe { htmlElementStatusHere(parent, elt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlAttrAllowed_rust(
    mut elt: *const htmlElemDesc,
    mut attr: *const xmlChar,
    mut legacy: i32,
) -> htmlStatus {
    let res: htmlStatus = unsafe { htmlAttrAllowed(elt, attr, legacy) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlNodeStatus_rust(node: htmlNodePtr, mut legacy: i32) -> htmlStatus {
    let res: htmlStatus = unsafe { htmlNodeStatus(node, legacy) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlCtxtReset_rust(mut ctxt: htmlParserCtxtPtr) {
    unsafe {
        htmlCtxtReset(ctxt);
    }
}

#[no_mangle]
pub unsafe extern "C" fn htmlCtxtUseOptions_rust(
    mut ctxt: htmlParserCtxtPtr,
    mut options: i32,
) -> i32 {
    let res: i32 = unsafe { htmlCtxtUseOptions(ctxt, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlReadDoc_rust(
    mut cur: *const xmlChar,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlReadDoc(cur, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlReadFile_rust(
    mut filename: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlReadFile(filename, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlReadMemory_rust(
    mut buffer: *const i8,
    mut size: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlReadMemory(buffer, size, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe fn htmlReadFd_rust(
    mut fd: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlReadFd(fd, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlReadIO_rust(
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut (),
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlReadIO(ioread, ioclose, ioctx, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlCtxtReadDoc_rust(
    mut ctxt: htmlParserCtxtPtr,
    mut cur: *const xmlChar,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlCtxtReadDoc(ctxt, cur, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlCtxtReadFile_rust(
    mut ctxt: htmlParserCtxtPtr,
    mut filename: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlCtxtReadFile(ctxt, filename, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlCtxtReadMemory_rust(
    mut ctxt: htmlParserCtxtPtr,
    mut buffer: *const i8,
    mut size: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlCtxtReadMemory(ctxt, buffer, size, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlCtxtReadFd_rust(
    mut ctxt: htmlParserCtxtPtr,
    mut fd: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> htmlDocPtr {
    let res: htmlDocPtr = unsafe { htmlCtxtReadFd(ctxt, fd, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlCtxtReadIO_rust(
    mut ctxt: htmlParserCtxtPtr,
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut (),
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> htmlDocPtr {
    let res: htmlDocPtr =
        unsafe { htmlCtxtReadIO(ctxt, ioread, ioclose, ioctx, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlParseTryOrFinish_rust(
    mut ctxt: htmlParserCtxtPtr,
    mut terminate: i32,
) -> i32 {
    let mut res: i32 = 0 as i32;

    match () {
        #[cfg(LIBXML_PUSH_ENABLED)]
        _ => {
            res = unsafe { htmlParseTryOrFinish(ctxt, terminate) };
        }
        #[cfg(not(LIBXML_PUSH_ENABLED))]
        _ => {}
    };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn htmlParseLookupSequence_rust(
    mut ctxt: htmlParserCtxtPtr,
    mut first: xmlChar,
    mut next: xmlChar,
    mut third: xmlChar,
    mut ignoreattrval: i32,
) -> i32 {
    let mut res: i32 = 0 as i32;
    match () {
        #[cfg(LIBXML_PUSH_ENABLED)]
        _ => {
            res = unsafe { htmlParseLookupSequence(ctxt, first, next, third, ignoreattrval) };
        }
        #[cfg(not(LIBXML_PUSH_ENABLED))]
        _ => {}
    };
    return res;
}
