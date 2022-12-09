use super::super::rust_core::parser::*;
use super::super::rust_core::parserInternals::*;
use super::super::rust_core::xpath::*;
use super::super::rust_core::HTMLparser::*;
use rust_ffi::ffi_defination::defination::*;

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

#[no_mangle]
pub unsafe extern "C" fn xmlHasFeature_rust(mut feature: xmlFeature) -> i32 {
    let res: i32 = unsafe { xmlHasFeature(feature) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCheckLanguageID_rust(mut lang: *const xmlChar) -> i32 {
    let res: i32 = unsafe { xmlCheckLanguageID(lang) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn inputPush_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut value: xmlParserInputPtr,
) -> i32 {
    let res: i32 = unsafe { inputPush_parser(ctxt, value) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn inputPop_rust(mut ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr {
    let res: xmlParserInputPtr = unsafe { inputPop_parser(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn nodePush_rust(mut ctxt: xmlParserCtxtPtr, mut value: xmlNodePtr) -> i32 {
    let res: i32 = unsafe { nodePush(ctxt, value) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn nodePop_rust(mut ctxt: xmlParserCtxtPtr) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { nodePop_parser(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn namePush_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut value: *const xmlChar,
) -> i32 {
    let res: i32 = unsafe { namePush(ctxt, value) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn namePop_rust(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let res: *const xmlChar = unsafe { namePop(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSkipBlankChars_rust(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let res: i32 = unsafe { xmlSkipBlankChars(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlPopInput_rust(mut ctxt: xmlParserCtxtPtr) -> xmlChar {
    let res: xmlChar = unsafe { xmlPopInput_parser(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlPushInput_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputPtr,
) -> i32 {
    let res: i32 = unsafe { xmlPushInput(ctxt, input) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseCharRef_rust(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let res: i32 = unsafe { xmlParseCharRef(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParserHandlePEReference_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParserHandlePEReference(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlStringLenDecodeEntities_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut len: i32,
    mut what: i32,
    mut end: xmlChar,
    mut end2: xmlChar,
    mut end3: xmlChar,
) -> *mut xmlChar {
    let res: *mut xmlChar =
        unsafe { xmlStringLenDecodeEntities(ctxt, str, len, what, end, end2, end3) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlStringDecodeEntities_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut what: i32,
    mut end: xmlChar,
    mut end2: xmlChar,
    mut end3: xmlChar,
) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlStringDecodeEntities(ctxt, str, what, end, end2, end3) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSplitQName_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const xmlChar,
    mut prefix: *mut *mut xmlChar,
) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlSplitQName(ctxt, name, prefix) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseName_rust(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let res: *const xmlChar = unsafe { xmlParseName(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseNmtoken_rust(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlParseNmtoken(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseEntityValue_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut orig: *mut *mut xmlChar,
) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlParseEntityValue(ctxt, orig) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseAttValue_rust(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlParseAttValue(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseSystemLiteral_rust(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlParseSystemLiteral(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParsePubidLiteral_rust(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlParsePubidLiteral(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseCharData_rust(mut ctxt: xmlParserCtxtPtr, mut cdata: i32) {
    unsafe { xmlParseCharData(ctxt, cdata) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseExternalID_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut publicID: *mut *mut xmlChar,
    mut strict: i32,
) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlParseExternalID(ctxt, publicID, strict) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseComment_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseComment(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParsePITarget_rust(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let res: *const xmlChar = unsafe { xmlParsePITarget(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParsePI_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParsePI(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseNotationDecl_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseNotationDecl(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseEntityDecl_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseEntityDecl(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseDefaultDecl_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut value: *mut *mut xmlChar,
) -> i32 {
    let res: i32 = unsafe { xmlParseDefaultDecl(ctxt, value) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseNotationType_rust(
    mut ctxt: xmlParserCtxtPtr,
) -> xmlEnumerationPtr {
    let res: xmlEnumerationPtr = unsafe { xmlParseNotationType(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseEnumerationType_rust(
    mut ctxt: xmlParserCtxtPtr,
) -> xmlEnumerationPtr {
    let res: xmlEnumerationPtr = unsafe { xmlParseEnumerationType(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseEnumeratedType_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut tree: *mut xmlEnumerationPtr,
) -> i32 {
    let res: i32 = unsafe { xmlParseEnumeratedType(ctxt, tree) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseAttributeType_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut tree: *mut xmlEnumerationPtr,
) -> i32 {
    let res: i32 = unsafe { xmlParseAttributeType(ctxt, tree) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseAttributeListDecl_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseAttributeListDecl(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseElementMixedContentDecl_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut inputchk: i32,
) -> xmlElementContentPtr {
    let res: xmlElementContentPtr = unsafe { xmlParseElementMixedContentDecl(ctxt, inputchk) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseElementChildrenContentDecl_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut inputchk: i32,
) -> xmlElementContentPtr {
    let res: xmlElementContentPtr = unsafe { xmlParseElementChildrenContentDecl(ctxt, inputchk) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseElementContentDecl_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const xmlChar,
    mut result: *mut xmlElementContentPtr,
) -> i32 {
    let res: i32 = unsafe { xmlParseElementContentDecl(ctxt, name, result) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseElementDecl_rust(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let res: i32 = unsafe { xmlParseElementDecl(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseMarkupDecl_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseMarkupDecl(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseTextDecl_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseTextDecl(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseExternalSubset_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    unsafe { xmlParseExternalSubset(ctxt, ExternalID, SystemID) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseReference_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseReference(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseEntityRef_rust(mut ctxt: xmlParserCtxtPtr) -> xmlEntityPtr {
    let res: xmlEntityPtr = unsafe { xmlParseEntityRef(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParsePEReference_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParsePEReference(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseDocTypeDecl_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseDocTypeDecl(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseAttribute_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut value: *mut *mut xmlChar,
) -> *const xmlChar {
    let res: *const xmlChar = unsafe { xmlParseAttribute(ctxt, value) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseStartTag_rust(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let res: *const xmlChar = unsafe { xmlParseStartTag(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseEndTag_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseEndTag(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseCDSect_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseCDSect(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseContent_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseContent(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseElement_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseElement(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseVersionNum_rust(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlParseVersionNum(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseVersionInfo_rust(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlParseVersionInfo(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseEncName_rust(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlParseEncName(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseEncodingDecl_rust(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let res: *const xmlChar = unsafe { xmlParseEncodingDecl(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseSDDecl_rust(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let res: i32 = unsafe { xmlParseSDDecl(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseXMLDecl_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseXMLDecl(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseMisc_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlParseMisc(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseDocument_rust(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let res: i32 = unsafe { xmlParseDocument(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseExtParsedEnt_rust(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let res: i32 = unsafe { xmlParseExtParsedEnt(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseChunk_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut chunk: *const i8,
    mut size: i32,
    mut terminate: i32,
) -> i32 {
    let res: i32 = unsafe { xmlParseChunk(ctxt, chunk, size, terminate) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCreatePushParserCtxt_rust(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut chunk: *const i8,
    mut size: i32,
    mut filename: *const i8,
) -> xmlParserCtxtPtr {
    let res: xmlParserCtxtPtr =
        unsafe { xmlCreatePushParserCtxt(sax, user_data, chunk, size, filename) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlStopParser_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlStopParser_parser(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlCreateIOParserCtxt_rust(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut (),
    mut enc: xmlCharEncoding,
) -> xmlParserCtxtPtr {
    let res: xmlParserCtxtPtr =
        unsafe { xmlCreateIOParserCtxt(sax, user_data, ioread, ioclose, ioctx, enc) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlIOParseDTD_rust(
    mut sax: xmlSAXHandlerPtr,
    mut input: xmlParserInputBufferPtr,
    mut enc: xmlCharEncoding,
) -> xmlDtdPtr {
    let res: xmlDtdPtr = unsafe { xmlIOParseDTD(sax, input, enc) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseDTD_rust(
    mut sax: xmlSAXHandlerPtr,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) -> xmlDtdPtr {
    let res: xmlDtdPtr = unsafe { xmlSAXParseDTD(sax, ExternalID, SystemID) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseDTD_rust(
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) -> xmlDtdPtr {
    let res: xmlDtdPtr = unsafe { xmlParseDTD(ExternalID, SystemID) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseCtxtExternalEntity_rust(
    mut ctx: xmlParserCtxtPtr,
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut lst: *mut xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlParseCtxtExternalEntity(ctx, URL, ID, lst) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseExternalEntity_rust(
    mut doc: xmlDocPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut depth: i32,
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut lst: *mut xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlParseExternalEntity(doc, sax, user_data, depth, URL, ID, lst) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseBalancedChunkMemory_rust(
    mut doc: xmlDocPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut depth: i32,
    mut string: *const xmlChar,
    mut lst: *mut xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlParseBalancedChunkMemory(doc, sax, user_data, depth, string, lst) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseInNodeContext_rust(
    mut node: xmlNodePtr,
    mut data: *const i8,
    mut datalen: i32,
    mut options: i32,
    mut lst: *mut xmlNodePtr,
) -> xmlParserErrors {
    let res: xmlParserErrors = unsafe { xmlParseInNodeContext(node, data, datalen, options, lst) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseBalancedChunkMemoryRecover_rust(
    mut doc: xmlDocPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut depth: i32,
    mut string: *const xmlChar,
    mut lst: *mut xmlNodePtr,
    mut recover: i32,
) -> i32 {
    let res: i32 = unsafe {
        xmlParseBalancedChunkMemoryRecover(doc, sax, user_data, depth, string, lst, recover)
    };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseEntity_rust(
    mut sax: xmlSAXHandlerPtr,
    mut filename: *const i8,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlSAXParseEntity(sax, filename) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseEntity_rust(mut filename: *const i8) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlParseEntity(filename) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCreateEntityParserCtxt_rust(
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut base: *const xmlChar,
) -> xmlParserCtxtPtr {
    let res: xmlParserCtxtPtr = unsafe { xmlCreateEntityParserCtxt(URL, ID, base) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCreateURLParserCtxt_rust(
    mut filename: *const i8,
    mut options: i32,
) -> xmlParserCtxtPtr {
    let res: xmlParserCtxtPtr = unsafe { xmlCreateURLParserCtxt(filename, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCreateFileParserCtxt_rust(mut filename: *const i8) -> xmlParserCtxtPtr {
    let res: xmlParserCtxtPtr = unsafe { xmlCreateFileParserCtxt(filename) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseFileWithData_rust(
    mut sax: xmlSAXHandlerPtr,
    mut filename: *const i8,
    mut recovery: i32,
    mut data: *mut (),
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlSAXParseFileWithData(sax, filename, recovery, data) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseFile_rust(
    mut sax: xmlSAXHandlerPtr,
    mut filename: *const i8,
    mut recovery: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlSAXParseFile(sax, filename, recovery) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlRecoverDoc_rust(mut cur: *const xmlChar) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlRecoverDoc(cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseFile_rust(mut filename: *const i8) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlParseFile(filename) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlRecoverFile_rust(mut filename: *const i8) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlRecoverFile(filename) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSetupParserForBuffer_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut buffer: *const xmlChar,
    mut filename: *const i8,
) {
    unsafe { xmlSetupParserForBuffer(ctxt, buffer, filename) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlSAXUserParseFile_rust(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut filename: *const i8,
) -> i32 {
    let res: i32 = unsafe { xmlSAXUserParseFile(sax, user_data, filename) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCreateMemoryParserCtxt_rust(
    mut buffer: *const i8,
    mut size: i32,
) -> xmlParserCtxtPtr {
    let res: xmlParserCtxtPtr = unsafe { xmlCreateMemoryParserCtxt_parser(buffer, size) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseMemoryWithData_rust(
    mut sax: xmlSAXHandlerPtr,
    mut buffer: *const i8,
    mut size: i32,
    mut recovery: i32,
    mut data: *mut (),
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlSAXParseMemoryWithData(sax, buffer, size, recovery, data) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseMemory_rust(
    mut sax: xmlSAXHandlerPtr,
    mut buffer: *const i8,
    mut size: i32,
    mut recovery: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlSAXParseMemory(sax, buffer, size, recovery) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseMemory_rust(mut buffer: *const i8, mut size: i32) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlParseMemory(buffer, size) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlRecoverMemory_rust(mut buffer: *const i8, mut size: i32) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlRecoverMemory(buffer, size) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSAXUserParseMemory_rust(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut buffer: *const i8,
    mut size: i32,
) -> i32 {
    let res: i32 = unsafe { xmlSAXUserParseMemory(sax, user_data, buffer, size) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCreateDocParserCtxt_rust(mut cur: *const xmlChar) -> xmlParserCtxtPtr {
    let res: xmlParserCtxtPtr = unsafe { xmlCreateDocParserCtxt(cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseDoc_rust(
    mut sax: xmlSAXHandlerPtr,
    mut cur: *const xmlChar,
    mut recovery: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlSAXParseDoc(sax, cur, recovery) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParseDoc_rust(mut cur: *const xmlChar) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlParseDoc(cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSetEntityReferenceFunc_rust(mut func: xmlEntityReferenceFunc) {
    unsafe { xmlSetEntityReferenceFunc(func) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlInitParser_rust() {
    unsafe { xmlInitParser_parser() };
}

#[no_mangle]
pub unsafe extern "C" fn xmlCleanupParser_rust() {
    unsafe { xmlCleanupParser() };
}

#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReset_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlCtxtReset_parser(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlCtxtResetPush_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut chunk: *const i8,
    mut size: i32,
    mut filename: *const i8,
    mut encoding: *const i8,
) -> i32 {
    let res: i32 = unsafe { xmlCtxtResetPush(ctxt, chunk, size, filename, encoding) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCtxtUseOptions_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut options: i32,
) -> i32 {
    let res: i32 = unsafe { xmlCtxtUseOptions(ctxt, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlReadDoc_rust(
    mut cur: *const xmlChar,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlReadDoc(cur, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlReadFile_rust(
    mut filename: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlReadFile(filename, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlReadMemory_rust(
    mut buffer: *const i8,
    mut size: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlReadMemory(buffer, size, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlReadFd_rust(
    mut fd: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlReadFd(fd, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlReadIO_rust(
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut (),
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlReadIO(ioread, ioclose, ioctx, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReadDoc_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut cur: *const xmlChar,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlCtxtReadDoc(ctxt, cur, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReadFile_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut filename: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlCtxtReadFile(ctxt, filename, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReadMemory_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut buffer: *const i8,
    mut size: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlCtxtReadMemory(ctxt, buffer, size, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReadFd_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut fd: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr = unsafe { xmlCtxtReadFd(ctxt, fd, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReadIO_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut (),
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let res: xmlDocPtr =
        unsafe { xmlCtxtReadIO(ctxt, ioread, ioclose, ioctx, URL, encoding, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCheckVersion_rust(mut version: i32) {
    unsafe { xmlCheckVersion(version) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlErrMemory_rust(mut ctxt: xmlParserCtxtPtr, mut extra: *const i8) {
    unsafe { xmlErrMemory(ctxt, extra) };
}

#[no_mangle]
pub unsafe extern "C" fn __xmlErrEncoding_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut xmlerr: xmlParserErrors,
    mut msg: *const i8,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    unsafe { __xmlErrEncoding(ctxt, xmlerr, msg, str1, str2) };
}

// #[no_mangle]
// pub unsafe extern "C" fn xmlErrEncodingInt_rust(
//     mut ctxt: xmlParserCtxtPtr,
//     mut error: xmlParserErrors,
//     mut msg: *const i8,
//     mut val: i32,
// ) {
//     unsafe { xmlErrEncodingInt(ctxt, error, msg, val) };
// }

#[no_mangle]
pub unsafe extern "C" fn xmlIsLetter_rust(mut c: i32) -> i32 {
    let res: i32 = unsafe { xmlIsLetter(c) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParserInputGrow_rust(mut in_0: xmlParserInputPtr, mut l: i32) -> i32 {
    let res: i32 = unsafe { xmlParserInputGrow_parserInternals(in_0, l) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParserInputShrink_rust(mut in_0: xmlParserInputPtr) {
    unsafe { xmlParserInputShrink_parserInternals(in_0) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlNextChar_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlNextChar_parserInternals(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlCurrentChar_rust(mut ctxt: xmlParserCtxtPtr, mut len: *mut i32) -> i32 {
    let res: i32 = unsafe { xmlCurrentChar(ctxt, len) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlStringCurrentChar_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut cur: *const xmlChar,
    mut len: *mut i32,
) -> i32 {
    let res: i32 = unsafe { xmlStringCurrentChar(ctxt, cur, len) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCopyCharMultiByte_rust(mut out: *mut xmlChar, mut val: i32) -> i32 {
    let res: i32 = unsafe { xmlCopyCharMultiByte(out, val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCopyChar_rust(
    mut len: i32,
    mut out: *mut xmlChar,
    mut val: i32,
) -> i32 {
    let res: i32 = unsafe { xmlCopyChar_parserInternals(len, out, val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchEncoding_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut enc: xmlCharEncoding,
) -> i32 {
    let res: i32 = unsafe { xmlSwitchEncoding_parserInternals(ctxt, enc) };
    return res;
}

// #[no_mangle]
// pub unsafe extern "C" fn xmlSwitchInputEncodingInt_rust(
//     mut ctxt: xmlParserCtxtPtr,
//     mut input: xmlParserInputPtr,
//     mut handler: xmlCharEncodingHandlerPtr,
//     mut len: i32,
// ) -> i32 {
//     let res: i32 = unsafe { xmlSwitchInputEncodingInt(ctxt, input, handler, len) };
//     return res;
// }

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchInputEncoding_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputPtr,
    mut handler: xmlCharEncodingHandlerPtr,
) -> i32 {
    let res: i32 = unsafe { xmlSwitchInputEncoding(ctxt, input, handler) };
    return res;
}

// #[no_mangle]
// pub unsafe extern "C" fn xmlSwitchToEncodingInt_rust(
//     mut ctxt: xmlParserCtxtPtr,
//     mut handler: xmlCharEncodingHandlerPtr,
//     mut len: i32,
// ) -> i32 {
//     let res: i32 = unsafe { xmlSwitchToEncodingInt(ctxt, handler, len) };
//     return res;
// }

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchToEncoding_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut handler: xmlCharEncodingHandlerPtr,
) -> i32 {
    let res: i32 = unsafe { xmlSwitchToEncoding_parserInternals(ctxt, handler) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlFreeInputStream_rust(mut input: xmlParserInputPtr) {
    unsafe { xmlFreeInputStream_parserInternals(input) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlNewInputStream_rust(mut ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr {
    let res: xmlParserInputPtr = unsafe { xmlNewInputStream_parserInternals(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlNewIOInputStream_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputBufferPtr,
    mut enc: xmlCharEncoding,
) -> xmlParserInputPtr {
    let res: xmlParserInputPtr = unsafe { xmlNewIOInputStream_parserInternals(ctxt, input, enc) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlNewEntityInputStream_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut entity: xmlEntityPtr,
) -> xmlParserInputPtr {
    let res: xmlParserInputPtr = unsafe { xmlNewEntityInputStream(ctxt, entity) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlNewStringInputStream_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut buffer: *const xmlChar,
) -> xmlParserInputPtr {
    let res: xmlParserInputPtr = unsafe { xmlNewStringInputStream_parserInternals(ctxt, buffer) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlNewInputFromFile_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut filename: *const i8,
) -> xmlParserInputPtr {
    let res: xmlParserInputPtr = unsafe { xmlNewInputFromFile(ctxt, filename) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlInitParserCtxt_rust(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let res: i32 = unsafe { xmlInitParserCtxt(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlFreeParserCtxt_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlFreeParserCtxt_parserInternals(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlNewParserCtxt_rust() -> xmlParserCtxtPtr {
    let res: xmlParserCtxtPtr = unsafe { xmlNewParserCtxt_parserInternals() };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlClearParserCtxt_rust(mut ctxt: xmlParserCtxtPtr) {
    unsafe { xmlClearParserCtxt(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlParserFindNodeInfo_rust(
    ctx: xmlParserCtxtPtr,
    node: xmlNodePtr,
) -> *const xmlParserNodeInfo {
    let res: *const xmlParserNodeInfo = unsafe { xmlParserFindNodeInfo(ctx, node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlInitNodeInfoSeq_rust(mut seq: xmlParserNodeInfoSeqPtr) {
    unsafe { xmlInitNodeInfoSeq_parserInternals(seq) };
}

#[no_mangle]
pub unsafe fn xmlClearNodeInfoSeq_rust(mut seq: xmlParserNodeInfoSeqPtr) {
    unsafe { xmlClearNodeInfoSeq(seq) };
}

#[no_mangle]
pub unsafe fn xmlParserFindNodeInfoIndex_rust(
    seq: xmlParserNodeInfoSeqPtr,
    node: xmlNodePtr,
) -> u64 {
    let res: u64 = unsafe { xmlParserFindNodeInfoIndex(seq, node) };
    return res;
}

#[no_mangle]
pub unsafe fn xmlParserAddNodeInfo_rust(mut ctxt: xmlParserCtxtPtr, info: xmlParserNodeInfoPtr) {
    unsafe { xmlParserAddNodeInfo_parserInternals(ctxt, info) };
}
#[no_mangle]
pub unsafe fn xmlPedanticParserDefault_rust(mut val: i32) -> i32 {
    let res: i32 = unsafe { xmlPedanticParserDefault(val) };
    return res;
}

#[no_mangle]
pub unsafe fn xmlLineNumbersDefault_rust(mut val: i32) -> i32 {
    let res: i32 = unsafe { xmlLineNumbersDefault(val) };
    return res;
}

#[no_mangle]
pub unsafe fn xmlSubstituteEntitiesDefault_rust(mut val: i32) -> i32 {
    let res: i32 = unsafe { xmlSubstituteEntitiesDefault(val) };
    return res;
}

#[no_mangle]
pub unsafe fn xmlKeepBlanksDefault_rust(mut val: i32) -> i32 {
    let res: i32 = unsafe { xmlKeepBlanksDefault(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathContextSetCache_rust(
    mut ctxt: xmlXPathContextPtr,
    mut active: i32,
    mut value: i32,
    mut options: i32,
) -> i32 {
    let res: i32 = unsafe { xmlXPathContextSetCache(ctxt, active, value, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn valuePop_rust(mut ctxt: xmlXPathParserContextPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { valuePop(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn valuePush_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut value: xmlXPathObjectPtr,
) -> i32 {
    let res: i32 = unsafe { valuePush(ctxt, value) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopBoolean_rust(mut ctxt: xmlXPathParserContextPtr) -> i32 {
    let res: i32 = unsafe { xmlXPathPopBoolean(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopNumber_rust(
    mut ctxt: xmlXPathParserContextPtr,
) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathPopNumber(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopString_rust(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathPopString(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopExternal_rust(mut ctxt: xmlXPathParserContextPtr) -> *mut () {
    let res: *mut () = unsafe { xmlXPathPopExternal(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathOrderDocElems_rust(mut doc: xmlDocPtr) -> i64 {
    let res: i64 = unsafe { xmlXPathOrderDocElems(doc) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCmpNodes_rust(
    mut node1: xmlNodePtr,
    mut node2: xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathCmpNodes(node1, node2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetSort_rust(mut set: xmlNodeSetPtr) {
    unsafe { xmlXPathNodeSetSort(set) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetFreeNs_rust(mut ns: xmlNsPtr) {
    unsafe { xmlXPathNodeSetFreeNs(ns) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetCreate_rust(mut val: xmlNodePtr) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeSetCreate(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetContains_rust(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathNodeSetContains(cur, val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetAddNs_rust(
    mut cur: xmlNodeSetPtr,
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathNodeSetAddNs(cur, node, ns) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetAdd_rust(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathNodeSetAdd(cur, val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetAddUnique_rust(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathNodeSetAddUnique(cur, val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetMerge_rust(
    mut val1: xmlNodeSetPtr,
    mut val2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeSetMerge(val1, val2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetDel_rust(mut cur: xmlNodeSetPtr, mut val: xmlNodePtr) {
    unsafe { xmlXPathNodeSetDel(cur, val) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetRemove_rust(mut cur: xmlNodeSetPtr, mut val: i32) {
    unsafe { xmlXPathNodeSetRemove(cur, val) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeNodeSet_rust(mut obj: xmlNodeSetPtr) {
    unsafe { xmlXPathFreeNodeSet(obj) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewNodeSetList_rust(mut val: xmlNodeSetPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewNodeSetList(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDifference_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathDifference(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathIntersection_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathIntersection(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDistinctSorted_rust(mut nodes: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathDistinctSorted(nodes) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDistinct_rust(mut nodes: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathDistinct(nodes) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathHasSameNodes_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathHasSameNodes(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeLeadingSorted_rust(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeLeadingSorted(nodes, node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeLeading_rust(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeLeading(nodes, node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathLeadingSorted_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathLeadingSorted(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathLeading_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathLeading(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeTrailingSorted_rust(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeTrailingSorted(nodes, node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeTrailing_rust(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeTrailing(nodes, node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathTrailingSorted_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathTrailingSorted(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathTrailing_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathTrailing(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterFunc_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut f: xmlXPathFunction,
) -> i32 {
    let res: i32 = unsafe { xmlXPathRegisterFunc(ctxt, name, f) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathErr_rust(mut ctxt: xmlXPathParserContextPtr, mut error: i32) {
    unsafe { xmlXPathErr(ctxt, error) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPatherror_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut file: *const i8,
    mut line: i32,
    mut no: i32,
) {
    unsafe { xmlXPatherror(ctxt, file, line, no) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopNodeSet_rust(
    mut ctxt: xmlXPathParserContextPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathPopNodeSet(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeCompExpr_rust(mut comp: xmlXPathCompExprPtr) {
    unsafe { xmlXPathFreeCompExpr(comp) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathIsNaN_rust(mut val: libc::c_double) -> i32 {
    let res: i32 = unsafe { xmlXPathIsNaN(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathIsInf_rust(mut val: libc::c_double) -> i32 {
    let res: i32 = unsafe { xmlXPathIsInf(val) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDebugDumpObject_rust(
    mut output: *mut FILE,
    mut cur: xmlXPathObjectPtr,
    mut depth: i32,
) {
    unsafe { xmlXPathDebugDumpObject(output, cur, depth) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathInit_rust() {
    unsafe { xmlXPathInit_xpath() };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeToNumber_rust(mut node: xmlNodePtr) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathCastNodeToNumber(node) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeSetToNumber_rust(mut ns: xmlNodeSetPtr) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathCastNodeSetToNumber(ns) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathStringEvalNumber_rust(mut str: *const xmlChar) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathStringEvalNumber(str) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastToNumber_rust(mut val: xmlXPathObjectPtr) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathCastToNumber(val) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastToBoolean_rust(mut val: xmlXPathObjectPtr) -> i32 {
    let res: i32 = unsafe { xmlXPathCastToBoolean(val) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDebugDumpCompExpr_rust(
    mut output: *mut FILE,
    mut comp: xmlXPathCompExprPtr,
    mut depth: i32,
) {
    unsafe { xmlXPathDebugDumpCompExpr(output, comp, depth) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextNamespace_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextNamespace(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterFuncLookup_rust(
    mut ctxt: xmlXPathContextPtr,
    mut f: xmlXPathFuncLookupFunc,
    mut funcCtxt: *mut (),
) {
    unsafe { xmlXPathRegisterFuncLookup(ctxt, f, funcCtxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFunctionLookup_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
) -> xmlXPathFunction {
    let res: xmlXPathFunction = unsafe { xmlXPathFunctionLookup(ctxt, name) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterFuncNS_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut f: xmlXPathFunction,
) -> i32 {
    let res: i32 = unsafe { xmlXPathRegisterFuncNS(ctxt, name, ns_uri, f) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisteredFuncsCleanup_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathRegisteredFuncsCleanup(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterVariableNS_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut value: xmlXPathObjectPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathRegisterVariableNS(ctxt, name, ns_uri, value) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterVariableLookup_rust(
    mut ctxt: xmlXPathContextPtr,
    mut f: xmlXPathVariableLookupFunc,
    mut data: *mut (),
) {
    unsafe { xmlXPathRegisterVariableLookup(ctxt, f, data) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathVariableLookup_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathVariableLookup(ctxt, name) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathVariableLookupNS_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathVariableLookupNS(ctxt, name, ns_uri) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisteredVariablesCleanup_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathRegisteredVariablesCleanup(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNsLookup_rust(
    mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
) -> *const xmlChar {
    let res: *const xmlChar = unsafe { xmlXPathNsLookup(ctxt, prefix) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterNs_rust(
    mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> i32 {
    let res: i32 = unsafe { xmlXPathRegisterNs(ctxt, prefix, ns_uri) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFunctionLookupNS_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathFunction {
    let res: xmlXPathFunction = unsafe { xmlXPathFunctionLookupNS(ctxt, name, ns_uri) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNumberToString_rust(mut val: libc::c_double) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathCastNumberToString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeContext_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathFreeContext(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathConvertBoolean_rust(
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathConvertBoolean(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvalExpr_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathEvalExpr(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompiledEval_rust(
    mut comp: xmlXPathCompExprPtr,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathCompiledEval(comp, ctx) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompiledEvalToBoolean_rust(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathCompiledEvalToBoolean(comp, ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompile_rust(mut str: *const xmlChar) -> xmlXPathCompExprPtr {
    let res: xmlXPathCompExprPtr = unsafe { xmlXPathCompile(str) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCtxtCompile_rust(
    mut ctxt: xmlXPathContextPtr,
    mut str: *const xmlChar,
) -> xmlXPathCompExprPtr {
    let res: xmlXPathCompExprPtr = unsafe { xmlXPathCtxtCompile(ctxt, str) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewParserContext_rust(
    mut str: *const xmlChar,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathParserContextPtr {
    let res: xmlXPathParserContextPtr = unsafe { xmlXPathNewParserContext(str, ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeEval_rust(
    mut node: xmlNodePtr,
    mut str: *const xmlChar,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNodeEval(node, str, ctx) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSetContextNode_rust(
    mut node: xmlNodePtr,
    mut ctx: xmlXPathContextPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathSetContextNode(node, ctx) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvalPredicate_rust(
    mut ctxt: xmlXPathContextPtr,
    mut res: xmlXPathObjectPtr,
) -> i32 {
    let result: i32 = unsafe { xmlXPathEvalPredicate(ctxt, res) };
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvaluatePredicateResult_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut res: xmlXPathObjectPtr,
) -> i32 {
    let result: i32 = unsafe { xmlXPathEvaluatePredicateResult(ctxt, res) };
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathIsNodeType_rust(mut name: *const xmlChar) -> i32 {
    let result: i32 = unsafe { xmlXPathIsNodeType(name) };
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathParseNCName_rust(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathParseNCName(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathParseName_rust(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathParseName(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRoundFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathRoundFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFloorFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathFloorFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCeilingFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathCeilingFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSumFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathSumFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextAncestor_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextAncestor(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextAncestorOrSelf_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextAncestorOrSelf(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextFollowingSibling_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextFollowingSibling(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextPrecedingSibling_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextPrecedingSibling(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextFollowing_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextFollowing(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextPreceding_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextPreceding(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextAttribute_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextAttribute(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRoot_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathRoot(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCountFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathCountFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathIdFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathIdFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathLocalNameFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathLocalNameFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNamespaceURIFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathNamespaceURIFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathStringFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathStringFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathStringLengthFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathStringLengthFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathConcatFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathConcatFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathContainsFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathContainsFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathStartsWithFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathStartsWithFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubstringFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathSubstringFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubstringBeforeFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathSubstringBeforeFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNormalizeFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathNormalizeFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathTranslateFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathTranslateFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathBooleanFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathBooleanFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNotFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathNotFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathTrueFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathTrueFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFalseFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathFalseFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathLangFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathLangFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubstringAfterFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathSubstringAfterFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNumberFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathNumberFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathEval_rust(
    mut str: *const xmlChar,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathEval(str, ctx) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvalExpression_rust(
    mut str: *const xmlChar,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathEvalExpression(str, ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterAllFunctions_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathRegisterAllFunctions(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastToString_rust(mut val: xmlXPathObjectPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathCastToString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathConvertString_rust(
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathConvertString(val) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextDescendantOrSelf_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextDescendantOrSelf(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextDescendant_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextDescendant(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewContext_rust(mut doc: xmlDocPtr) -> xmlXPathContextPtr {
    let res: xmlXPathContextPtr = unsafe { xmlXPathNewContext(doc) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeParserContext_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathFreeParserContext(ctxt) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathEqualValues_rust(mut ctxt: xmlXPathParserContextPtr) -> i32 {
    let res: i32 = unsafe { xmlXPathEqualValues(ctxt) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNotEqualValues_rust(mut ctxt: xmlXPathParserContextPtr) -> i32 {
    let res: i32 = unsafe { xmlXPathNotEqualValues(ctxt) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextChild_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextChild(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextParent_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextParent(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathLastFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathLastFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPositionFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathPositionFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompareValues_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: i32,
    mut strict: i32,
) -> i32 {
    let res: i32 = unsafe { xmlXPathCompareValues(ctxt, inf, strict) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathAddValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathAddValues(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathSubValues(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathModValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathModValues(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDivValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathDivValues(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathMultValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathMultValues(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathValueFlipSign_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathValueFlipSign(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextSelf_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextSelf(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeSetToBoolean_rust(mut ns: xmlNodeSetPtr) -> i32 {
    let res: i32 = unsafe { xmlXPathCastNodeSetToBoolean(ns) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastStringToBoolean_rust(mut val: *const xmlChar) -> i32 {
    let res: i32 = unsafe { xmlXPathCastStringToBoolean(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNumberToBoolean_rust(mut val: libc::c_double) -> i32 {
    let res: i32 = unsafe { xmlXPathCastNumberToBoolean(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathConvertNumber_rust(
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathConvertNumber(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeToString_rust(mut node: xmlNodePtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathCastNodeToString(node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastBooleanToString_rust(mut val: i32) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathCastBooleanToString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisteredNsCleanup_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathRegisteredNsCleanup(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeObject_rust(mut obj: xmlXPathObjectPtr) {
    unsafe { xmlXPathFreeObject(obj) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewFloat_rust(mut val: libc::c_double) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewFloat(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewBoolean_rust(mut val: i32) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewBoolean(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewString_rust(mut val: *const xmlChar) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapString_rust(mut val: *mut xmlChar) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathWrapString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewCString_rust(mut val: *const i8) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewCString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastStringToNumber_rust(
    mut val: *const xmlChar,
) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathCastStringToNumber(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastBooleanToNumber_rust(mut val: i32) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathCastBooleanToNumber(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeSetToString_rust(mut ns: xmlNodeSetPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathCastNodeSetToString(ns) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapCString_rust(mut val: *mut i8) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathWrapCString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapNodeSet_rust(mut val: xmlNodeSetPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathWrapNodeSet(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeNodeSetList_rust(mut obj: xmlXPathObjectPtr) {
    unsafe { xmlXPathFreeNodeSetList(obj) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewValueTree_rust(mut val: xmlNodePtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewValueTree(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewNodeSet_rust(mut val: xmlNodePtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewNodeSet(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapExternal_rust(mut val: *mut ()) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathWrapExternal(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathObjectCopy_rust(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathObjectCopy(val) };
    return res;
}

#[no_mangle]
#[cfg(DEBUG_OR_DEBUG_STEP)]
pub unsafe extern "C" fn xmlGenericErrorContextNodeSet_rust(
    mut output: *mut FILE,
    mut obj: xmlNodeSetPtr,
) {
    unsafe { xmlGenericErrorContextNodeSet(output, obj) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterVariable_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut value: xmlXPathObjectPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathRegisterVariable(ctxt, name, value) };
    return res;
}
