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

#[no_mangle]
pub unsafe extern "C" fn xmlErrEncodingInt_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const i8,
    mut val: i32,
) {
    unsafe { xmlErrEncodingInt(ctxt, error, msg, val) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlIsLetter_rust(mut c: i32) -> i32 {
    let res: i32 = unsafe { xmlIsLetter(c) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlParserInputGrow_rust(
    mut in_0: xmlParserInputPtr,
    mut l: i32,
) -> i32 {
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
pub unsafe extern "C" fn xmlCurrentChar_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut len: *mut i32,
) -> i32 {
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
pub unsafe extern "C" fn xmlCopyCharMultiByte_rust(
    mut out: *mut xmlChar,
    mut val: i32,
) -> i32 {
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

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchInputEncodingInt_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputPtr,
    mut handler: xmlCharEncodingHandlerPtr,
    mut len: i32,
) -> i32 {
    let res: i32 = unsafe { xmlSwitchInputEncodingInt(ctxt, input, handler, len) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchInputEncoding_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputPtr,
    mut handler: xmlCharEncodingHandlerPtr,
) -> i32 {
    let res: i32 = unsafe { xmlSwitchInputEncoding(ctxt, input, handler) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlSwitchToEncodingInt_rust(
    mut ctxt: xmlParserCtxtPtr,
    mut handler: xmlCharEncodingHandlerPtr,
    mut len: i32,
) -> i32 {
    let res: i32 = unsafe { xmlSwitchToEncodingInt(ctxt, handler, len) };
    return res;
}

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
