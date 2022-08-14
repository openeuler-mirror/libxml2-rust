#[no_mangle]
pub unsafe extern "C" fn xmlCheckVersion_rust(mut version: libc::c_int) {
    unsafe { xmlCheckVersion(version) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlErrMemory_rust(mut ctxt: xmlParserCtxtPtr, mut extra: *const libc::c_char) {
    unsafe { xmlErrMemory(ctxt, extra) };
}

#[no_mangle]
pub unsafe extern "C" fn __xmlErrEncoding_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut xmlerr: xmlParserErrors,
    mut msg: *const libc::c_char,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    unsafe { __xmlErrEncoding(ctxt, xmlerr, msg, str1, str2) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlErrEncodingInt_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const libc::c_char,
    mut val: libc::c_int,
) {
    unsafe { xmlErrEncodingInt(ctxt, error, msg, val) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlIsLetter_rust(mut c: libc::c_int) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlIsLetter(c) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParserInputGrow_rust(
    mut in_0: xmlParserInputPtr,
    mut l: libc::c_int,
) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlParserInputGrow_parserInternals(in_0, l) };
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
pub unsafe extern "C" fn xmlCurrentChar_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut len: *mut libc::c_int,
) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlCurrentChar(ctxt, len) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlStringCurrentChar_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut cur: *const xmlChar,
    mut len: *mut libc::c_int,
) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlStringCurrentChar(ctxt, cur, len) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCopyCharMultiByte_rust(
    mut out: *mut xmlChar,
    mut val: libc::c_int,
) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlCopyCharMultiByte(out, val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlCopyChar_rust(
    mut len: libc::c_int,
    mut out: *mut xmlChar,
    mut val: libc::c_int,
) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlCopyChar_parserInternals(len, out, val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchEncoding_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut enc: xmlCharEncoding,
) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlSwitchEncoding_parserInternals(ctxt, enc) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchInputEncodingInt_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputPtr,
    mut handler: xmlCharEncodingHandlerPtr,
    mut len: libc::c_int,
) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlSwitchInputEncodingInt(ctxt, input, handler, len) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchInputEncoding_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputPtr,
    mut handler: xmlCharEncodingHandlerPtr,
) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlSwitchInputEncoding(ctxt, input, handler) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchToEncodingInt_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut handler: xmlCharEncodingHandlerPtr,
    mut len: libc::c_int,
) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlSwitchToEncodingInt(ctxt, handler, len) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchToEncoding_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut handler: xmlCharEncodingHandlerPtr,
) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlSwitchToEncoding_parserInternals(ctxt, handler) };
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
    mut filename: *const libc::c_char,
) -> xmlParserInputPtr {
    let res: xmlParserInputPtr = unsafe { xmlNewInputFromFile(ctxt, filename) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlInitParserCtxt_rust(mut ctxt: xmlParserCtxtPtr) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlInitParserCtxt(ctxt) };
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
) -> libc::c_ulong {
    let res: libc::c_ulong = unsafe { xmlParserFindNodeInfoIndex(seq, node) };
    return res;
}

#[no_mangle]
pub unsafe fn xmlParserAddNodeInfo_rust(mut ctxt: xmlParserCtxtPtr, info: xmlParserNodeInfoPtr) {
    unsafe { xmlParserAddNodeInfo_parserInternals(ctxt, info) };
}
#[no_mangle]
pub unsafe fn xmlPedanticParserDefault_rust(mut val: libc::c_int) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlPedanticParserDefault(val) };
    return res;
}

#[no_mangle]
pub unsafe fn xmlLineNumbersDefault_rust(mut val: libc::c_int) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlLineNumbersDefault(val) };
    return res;
}

#[no_mangle]
pub unsafe fn xmlSubstituteEntitiesDefault_rust(mut val: libc::c_int) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlSubstituteEntitiesDefault(val) };
    return res;
}

#[no_mangle]
pub unsafe fn xmlKeepBlanksDefault_rust(mut val: libc::c_int) -> libc::c_int {
    let res: libc::c_int = unsafe { xmlKeepBlanksDefault(val) };
    return res;
}
