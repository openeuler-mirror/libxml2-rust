use rust_ffi::ffi_defination::defination::*;
use rust_ffi::ffi_extern_method::extern_method::*;
use rust_ffi::ffi_extern_method::extern_method_safe::*;

const HTML_MAX_NAMELEN: i32 = 1000;
const HTML_PARSER_BIG_BUFFER_SIZE: i32 = 1000;
const HTML_PARSER_BUFFER_SIZE: i32 = 100;
const INPUT_CHUNK: i32 = 250;

fn UPPER(ctxt: htmlParserCtxtPtr) -> i32 {
    let mut __res: i32 = 0;
    if ::std::mem::size_of::<xmlChar>() as u64 > 1 {
        if 0 == 1 {
            let mut __c: i32 = CUR(ctxt);
            __res = (if __c < -128 || __c > 255 {
                __c
            } else {
                unsafe { *(*__ctype_toupper_loc_safe()).offset(__c as isize) }
            })
        } else {
            __res = (CUR(ctxt))
        }
    } else {
        unsafe { __res = *(*__ctype_toupper_loc_safe()).offset(*(*(*ctxt).input).cur as isize) }
    }
    __res
}

fn CUR(ctxt: htmlParserCtxtPtr) -> i32 {
    unsafe { *(*(*ctxt).input).cur as i32 }
}

fn RAW(ctxt: htmlParserCtxtPtr) -> i32 {
    let ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.token != 0 {
        -1
    } else {
        CUR(ctxt)
    }
}

fn NXT(ctxt: htmlParserCtxtPtr, val: i32) -> i32 {
    unsafe { *(*(*ctxt).input).cur.offset(val as isize) as i32 }
}

fn SHRINK_bool1(ctxt: htmlParserCtxtPtr, num: i64) -> bool {
    let result: i64 = unsafe { (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) } as i64;
    result > num
}

fn SHRINK_bool2(ctxt: htmlParserCtxtPtr, num: i64) -> bool {
    let mut result: i64 = unsafe { (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) } as i64;
    result < num
}

fn SHRINK(ctxt: htmlParserCtxtPtr) {
    let ctxtPtr = unsafe { &mut *ctxt };
    if SHRINK_bool1(ctxt, (2 * INPUT_CHUNK) as i64) && SHRINK_bool2(ctxt, (2 * INPUT_CHUNK) as i64)
    {
        unsafe { xmlParserInputShrink_safe(ctxtPtr.input) };
    }
}

fn SKIP(ctxt: htmlParserCtxtPtr, val: i32) {
    let inputPtr = unsafe { &mut *(*ctxt).input };
    unsafe {
        inputPtr.cur = inputPtr.cur.offset(val as isize);
    }
    inputPtr.col += val;
}

fn NEXTL(ctxt: htmlParserCtxtPtr, ql: i32) {
    let ctxtPtr = unsafe { &mut *ctxt };
    let inputPtr = unsafe { &mut *(*ctxt).input };
    if CUR(ctxt) == '\n' as i32 {
        inputPtr.line += 1;
        inputPtr.col = 1
    } else {
        inputPtr.col += 1
    }
    ctxtPtr.token = 0;
    unsafe {
        inputPtr.cur = inputPtr.cur.offset(ql as isize);
    }
}

fn GROW(ctxt: htmlParserCtxtPtr) {
    let ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.progressive == 0 && SHRINK_bool2(ctxt, INPUT_CHUNK as i64) {
        unsafe { xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK) };
    }
}

fn IS_CHAR(q: i32) -> bool {
    if q < 0x100 {
        (0x9 <= q && q <= 0xa) || q == 0xd || 0x20 <= q
    } else {
        (0x100 <= q && q <= 0xd7ff)
            || (0xe000 <= q && q <= 0xfffd)
            || (0x10000 <= q && q <= 0x10ffff)
    }
}

fn IS_CHAR_CH(c: i32) -> bool {
    (0x9 <= c && c <= 0xa) || c == 0xd || 0x20 <= c
}

fn IS_BLANK_CH(c: i32) -> bool {
    (c == 0x20) || ((c >= 0x9) && (c <= 0xa)) || (c == 0xd)
}

fn IS_BLANK(cur: i32) -> bool {
    (if cur < 0x100 {
        (cur == 0x20 || 0x9 <= cur && cur <= 0xa || cur == 0xd) as i32
    } else {
        0
    }) != 0
}

fn IS_LETTER(c: i32, group: *const xmlChRangeGroup) -> bool {
    ((if c < 0x100 {
        (0x41 <= c && c <= 0x5a
            || 0x61 <= c && c <= 0x7a
            || 0xc0 <= c && c <= 0xd6
            || 0xd8 <= c && c <= 0xf6
            || 0xf8 <= c) as i32
    } else {
        unsafe { xmlCharInRange_safe(c as u32, group) }
    }) != 0
        || (if c < 0x100 {
            0
        } else {
            (0x4e00 <= c && c <= 0x9fa5 || c == 0x3007 || 0x3021 <= c && c <= 0x3029) as i32
        }) != 0)
}

fn IS_DIGIT(c: i32, group: *const xmlChRangeGroup) -> bool {
    (if c < 0x100 {
        (0x30 <= c && c <= 0x39) as i32
    } else {
        unsafe { xmlCharInRange_safe(c as u32, group) }
    }) != 0
}

fn IS_COMBINING(c: i32, group: *const xmlChRangeGroup) -> bool {
    (if c < 0x100 {
        0
    } else {
        unsafe { xmlCharInRange_safe(c as u32, group) }
    }) != 0
}

fn IS_EXTENDER(c: i32, group: *const xmlChRangeGroup) -> bool {
    (if c < 0x100 {
        (c == 0xb7) as i32
    } else {
        unsafe { xmlCharInRange_safe(c as u32, group) }
    }) != 0
}

fn COPY_BUF(ql: i32, buf: *mut xmlChar, mut len: i32, q: i32) -> i32 {
    if ql == 1 {
        unsafe {
            *buf.offset(len as isize) = q as xmlChar;
        }
        len = len + 1;
    } else {
        unsafe {
            len += xmlCopyChar_safe(ql, buf.offset(len as isize), q);
        }
    }
    return len;
}

fn IS_ASCII_LETTER(c: i32) -> bool {
    ((c >= 0x41) && (c <= 0x5a)) || ((c >= 0x61) && (c <= 0x7a))
}

fn IS_ASCII_DIGIT(c: i32) -> bool {
    (c >= 0x30) && (c <= 0x39)
}

fn UPP(ctxt: htmlParserCtxtPtr, val: i32) -> i32 {
    let inputPtr = unsafe { &mut *(*ctxt).input };
    let mut __res: i32 = 0;
    if ::std::mem::size_of::<xmlChar>() as u64 > 1 {
        if 0 > 1 {
            let mut __c: i32 = unsafe { *inputPtr.cur.offset(val as isize) } as i32;
            __res = (if __c < -128 || __c > 255 {
                __c
            } else {
                unsafe { *(*__ctype_toupper_loc_safe()).offset(__c as isize) }
            })
        } else {
            unsafe { __res = toupper(*inputPtr.cur.offset(val as isize) as i32) }
        }
    } else {
        unsafe {
            __res =
                *(*__ctype_toupper_loc_safe()).offset(*inputPtr.cur.offset(val as isize) as isize)
        }
    }
    __res
}

#[inline]
fn bsearch(
    __key: *const (),
    __base: *const (),
    __nmemb: size_t,
    __size: size_t,
    __compar: __compar_fn_t,
) -> *mut () {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const () = 0 as *const ();
    let mut __comparison: i32 = 0;
    __l = 0 as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = (__l + __u) / 2;
        unsafe {
            __p = (__base as *const i8).offset((__idx * __size) as isize) as *mut ();
        }
        __comparison = unsafe {
            Some(__compar.expect("non-null function pointer")).expect("non-null function pointer")(
                __key, __p,
            )
        };
        if __comparison < 0 {
            __u = __idx
        } else if __comparison > 0 {
            __l = __idx + 1
        } else {
            return __p as *mut ();
        }
    }
    return 0 as *mut ();
}

#[inline]
pub fn toupper(mut __c: i32) -> i32 {
    return if __c >= -128 && __c < 256 {
        unsafe { *(*__ctype_toupper_loc_safe()).offset(__c as isize) }
    } else {
        __c
    };
}

static mut htmlOmittedDefaultValue: i32 = 1;
/* ***********************************************************************
 *									*
 *		Some factorized error routines				*
 *									*
 ************************************************************************/
/* *
 * htmlErrMemory:
 * @ctxt:  an HTML parser context
 * @extra:  extra information
 *
 * Handle a redefinition of attribute error
 */
fn htmlErrMemory(ctxt: xmlParserCtxtPtr, extra: *const i8) {
    unsafe {
        if !ctxt.is_null() && (*ctxt).disableSAX != 0 && (*ctxt).instate == XML_PARSER_EOF {
            return;
        }
    }
    if !ctxt.is_null() {
        let mut ctxtPtr = unsafe { &mut *ctxt };
        ctxtPtr.errNo = XML_ERR_NO_MEMORY as i32;
        ctxtPtr.instate = XML_PARSER_EOF;
        ctxtPtr.disableSAX = 1
    }
    if !extra.is_null() {
        __xmlRaiseError_safe_macro!(
            None,
            None,
            0 as *mut (),
            ctxt as *mut (),
            0 as *mut (),
            XML_FROM_PARSER as i32,
            XML_ERR_NO_MEMORY as i32,
            XML_ERR_FATAL,
            0 as *const i8,
            0,
            extra,
            0 as *const i8,
            0 as *const i8,
            0,
            0,
            b"Memory allocation failed : %s\n\x00" as *const u8 as *const i8,
            extra
        );
    } else {
        __xmlRaiseError_safe_macro!(
            None,
            None,
            0 as *mut (),
            ctxt as *mut (),
            0 as *mut (),
            XML_FROM_PARSER as i32,
            XML_ERR_NO_MEMORY as i32,
            XML_ERR_FATAL,
            0 as *const i8,
            0,
            0 as *const i8,
            0 as *const i8,
            0 as *const i8,
            0,
            0,
            b"Memory allocation failed\n\x00" as *const u8 as *const i8
        );
    };
}
/* *
 * htmlParseErr:
 * @ctxt:  an HTML parser context
 * @error:  the error number
 * @msg:  the error message
 * @str1:  string infor
 * @str2:  string infor
 *
 * Handle a fatal parser error, i.e. violating Well-Formedness constraints
 */
fn htmlParseErr(
    ctxt: xmlParserCtxtPtr,
    error: xmlParserErrors,
    msg: *const i8,
    str1: *const xmlChar,
    str2: *const xmlChar,
) {
    unsafe {
        if !ctxt.is_null() && (*ctxt).disableSAX != 0 && (*ctxt).instate == XML_PARSER_EOF {
            return;
        }
        if !ctxt.is_null() {
            (*ctxt).errNo = error as i32
        }
        __xmlRaiseError_safe_macro!(
            None,
            None,
            0 as *mut (),
            ctxt as *mut (),
            0 as *mut (),
            XML_FROM_HTML as i32,
            error as i32,
            XML_ERR_ERROR,
            0 as *const i8,
            0,
            str1 as *const i8,
            str2 as *const i8,
            0 as *const i8,
            0,
            0,
            msg,
            str1,
            str2
        );
        if !ctxt.is_null() {
            (*ctxt).wellFormed = 0
        };
    }
}
/* *
 * htmlParseErrInt:
 * @ctxt:  an HTML parser context
 * @error:  the error number
 * @msg:  the error message
 * @val:  integer info
 *
 * Handle a fatal parser error, i.e. violating Well-Formedness constraints
 */
fn htmlParseErrInt(ctxt: xmlParserCtxtPtr, error: xmlParserErrors, msg: *const i8, val: i32) {
    unsafe {
        if !ctxt.is_null() && (*ctxt).disableSAX != 0 && (*ctxt).instate == XML_PARSER_EOF {
            return;
        }
        if !ctxt.is_null() {
            (*ctxt).errNo = error as i32
        }
        __xmlRaiseError_safe_macro!(
            None,
            None,
            0 as *mut (),
            ctxt as *mut (),
            0 as *mut (),
            XML_FROM_HTML as i32,
            error as i32,
            XML_ERR_ERROR,
            0 as *const i8,
            0,
            0 as *const i8,
            0 as *const i8,
            0 as *const i8,
            val,
            0,
            msg,
            val
        );
        if !ctxt.is_null() {
            (*ctxt).wellFormed = 0
        };
    }
}
/* ***********************************************************************
 *									*
 *	Parser stacks related functions and macros		*
 *									*
 ************************************************************************/
/* *
 * htmlnamePush:
 * @ctxt:  an HTML parser context
 * @value:  the element name
 *
 * Pushes a new element name on top of the name stack
 *
 * Returns 0 in case of error, the index in the stack otherwise
 */
fn htmlnamePush(ctxt: htmlParserCtxtPtr, value: *const xmlChar) -> i32 {
    let ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.html < 3
        && unsafe {
            xmlStrEqual_safe(value, b"head\x00" as *const u8 as *const i8 as *mut xmlChar) != 0
        }
    {
        ctxtPtr.html = 3
    }
    if ctxtPtr.html < 10
        && unsafe {
            xmlStrEqual_safe(value, b"body\x00" as *const u8 as *const i8 as *mut xmlChar) != 0
        }
    {
        ctxtPtr.html = 10
    }
    if ctxtPtr.nameNr >= ctxtPtr.nameMax {
        ctxtPtr.nameMax *= 2;
        ctxtPtr.nameTab = unsafe {
            xmlRealloc_safe(
                ctxtPtr.nameTab as *mut *mut xmlChar as *mut (),
                ((ctxtPtr.nameMax as u64) * (::std::mem::size_of::<*const xmlChar>()) as u64),
            )
        } as *mut *const xmlChar;
        if ctxtPtr.nameTab.is_null() {
            htmlErrMemory(ctxt, 0 as *const i8);
            return 0;
        }
    }
    unsafe {
        *ctxtPtr.nameTab.offset(ctxtPtr.nameNr as isize) = value;
    }
    ctxtPtr.name = value;
    let nameNr_old = ctxtPtr.nameNr;
    ctxtPtr.nameNr = ctxtPtr.nameNr + 1;
    return nameNr_old;
}
/* *
 * htmlnamePop:
 * @ctxt: an HTML parser context
 *
 * Pops the top element name from the name stack
 *
 * Returns the name just removed
 */
fn htmlnamePop(ctxt: htmlParserCtxtPtr) -> *const xmlChar {
    let ctxtPtr = unsafe { &mut *ctxt };
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    if ctxtPtr.nameNr <= 0 {
        return 0 as *const xmlChar;
    }
    ctxtPtr.nameNr -= 1;
    if ctxtPtr.nameNr < 0 {
        return 0 as *const xmlChar;
    }
    if ctxtPtr.nameNr > 0 {
        unsafe { ctxtPtr.name = *ctxtPtr.nameTab.offset((ctxtPtr.nameNr - 1 as i32) as isize) }
    } else {
        ctxtPtr.name = 0 as *const xmlChar
    }
    unsafe {
        ret = *ctxtPtr.nameTab.offset(ctxtPtr.nameNr as isize);
    }
    unsafe {
        *ctxtPtr.nameTab.offset(ctxtPtr.nameNr as isize) = 0 as *const xmlChar;
    }
    return ret;
}
/* *
 * htmlNodeInfoPush:
 * @ctxt:  an HTML parser context
 * @value:  the node info
 *
 * Pushes a new element name on top of the node info stack
 *
 * Returns 0 in case of error, the index in the stack otherwise
 */
fn htmlNodeInfoPush(ctxt: htmlParserCtxtPtr, value: *mut htmlParserNodeInfo) -> i32 {
    let ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.nodeInfoNr >= ctxtPtr.nodeInfoMax {
        if ctxtPtr.nodeInfoMax == 0 {
            ctxtPtr.nodeInfoMax = 5
        }
        ctxtPtr.nodeInfoMax *= 2;
        ctxtPtr.nodeInfoTab = unsafe {
            xmlRealloc_safe(
                ctxtPtr.nodeInfoTab as *mut htmlParserNodeInfo as *mut (),
                (ctxtPtr.nodeInfoMax as u64) * (::std::mem::size_of::<xmlParserNodeInfo>()) as u64,
            )
        } as *mut htmlParserNodeInfo;
        if ctxtPtr.nodeInfoTab.is_null() {
            htmlErrMemory(ctxt, 0 as *const i8);
            return 0;
        }
    }
    unsafe {
        *ctxtPtr.nodeInfoTab.offset(ctxtPtr.nodeInfoNr as isize) = *value;
    }
    unsafe {
        ctxtPtr.nodeInfo =
            &mut *ctxtPtr.nodeInfoTab.offset(ctxtPtr.nodeInfoNr as isize) as *mut xmlParserNodeInfo;
    }
    let nodeInfoNr_old = ctxtPtr.nodeInfoNr;
    ctxtPtr.nodeInfoNr = ctxtPtr.nodeInfoNr + 1;
    return nodeInfoNr_old;
}
/* *
 * htmlNodeInfoPop:
 * @ctxt:  an HTML parser context
 *
 * Pops the top element name from the node info stack
 *
 * Returns 0 in case of error, the pointer to NodeInfo otherwise
 */
fn htmlNodeInfoPop(ctxt: htmlParserCtxtPtr) -> *mut htmlParserNodeInfo {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.nodeInfoNr <= 0 {
        return 0 as *mut htmlParserNodeInfo;
    }
    ctxtPtr.nodeInfoNr -= 1;
    if ctxtPtr.nodeInfoNr < 0 {
        return 0 as *mut htmlParserNodeInfo;
    }
    if ctxtPtr.nodeInfoNr > 0 {
        unsafe {
            ctxtPtr.nodeInfo = &mut *ctxtPtr
                .nodeInfoTab
                .offset((ctxtPtr.nodeInfoNr - 1) as isize)
                as *mut xmlParserNodeInfo
        }
    } else {
        ctxtPtr.nodeInfo = 0 as *mut xmlParserNodeInfo
    }
    unsafe {
        return &mut *ctxtPtr.nodeInfoTab.offset(ctxtPtr.nodeInfoNr as isize)
            as *mut xmlParserNodeInfo;
    }
}
/* *
 * htmlFindEncoding:
 * @the HTML parser context
 *
 * Ty to find and encoding in the current data available in the input
 * buffer this is needed to try to switch to the proper encoding when
 * one face a character error.
 * That's an heuristic, since it's operating outside of parsing it could
 * try to use a meta which had been commented out, that's the reason it
 * should only be used in case of error, not as a default.
 *
 * Returns an encoding string or NULL if not found, the string need to
 *   be freed
 */
fn htmlFindEncoding(ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut start: *const xmlChar = 0 as *const xmlChar;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut end: *const xmlChar = 0 as *const xmlChar;
    unsafe {
        if ctxt.is_null()
            || (*ctxt).input.is_null()
            || !(*(*ctxt).input).encoding.is_null()
            || (*(*ctxt).input).buf.is_null()
            || !(*(*(*ctxt).input).buf).encoder.is_null()
        {
            return 0 as *mut xmlChar;
        }
    }
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if inputPtr.cur.is_null() || inputPtr.end.is_null() {
        return 0 as *mut xmlChar;
    }
    start = inputPtr.cur;
    end = inputPtr.end;
    /* we also expect the input buffer to be zero terminated */
    let mut end_safe = unsafe { *end };
    if end_safe as i32 != 0 {
        return 0 as *mut xmlChar;
    }
    cur = unsafe {
        xmlStrcasestr_safe(
            start,
            b"HTTP-EQUIV\x00" as *const u8 as *const i8 as *mut xmlChar,
        )
    };
    if cur.is_null() {
        return 0 as *mut xmlChar;
    }
    cur = unsafe {
        xmlStrcasestr_safe(
            cur,
            b"CONTENT\x00" as *const u8 as *const i8 as *mut xmlChar,
        )
    };
    if cur.is_null() {
        return 0 as *mut xmlChar;
    }
    cur = unsafe {
        xmlStrcasestr_safe(
            cur,
            b"CHARSET=\x00" as *const u8 as *const i8 as *mut xmlChar,
        )
    };
    if cur.is_null() {
        return 0 as *mut xmlChar;
    }
    unsafe {
        cur = cur.offset(8 as isize);
    }
    start = cur;
    let mut cur_safe = unsafe { *cur };
    while 1 < 2 {
        unsafe {
            if (!((*cur) >= 'A' as u8 && (*cur) <= 'Z' as u8
                || (*cur) >= 'a' as u8 && (*cur) <= 'z' as u8
                || (*cur) >= '0' as u8 && (*cur) <= '9' as u8
                || (*cur) == '-' as u8
                || (*cur) == '_' as u8
                || (*cur) == ':' as u8
                || (*cur) == '/' as u8))
            {
                break;
            }
        }
        unsafe { cur = cur.offset(1) }
    }
    if cur == start {
        return 0 as *mut xmlChar;
    }
    unsafe {
        return xmlStrndup_safe(start, cur.offset_from(start) as i32);
    }
}
/* *
 * htmlCurrentChar:
 * @ctxt:  the HTML parser context
 * @len:  pointer to the length of the char read
 *
 * The current char value, if using UTF-8 this may actually span multiple
 * bytes in the input buffer. Implement the end of line normalization:
 * 2.11 End-of-Line Handling
 * If the encoding is unspecified, in the case we find an ISO-Latin-1
 * char, then the encoding converter is plugged in automatically.
 *
 * Returns the current char value and its length
 */
fn htmlCurrentChar(ctxt: xmlParserCtxtPtr, len: *mut i32) -> i32 {
    let ctxtPtr = unsafe { &mut *ctxt };
    let inputPtr = unsafe { &mut *(*ctxt).input };
    let current_block: i32;
    let mut cur: *const u8 = 0 as *const u8;
    let mut c: u8 = 0;
    let mut val: u32 = 0;
    if ctxtPtr.instate == XML_PARSER_EOF {
        return 0;
    }
    if ctxtPtr.token != 0 {
        unsafe {
            *len = 0;
        }
        return ctxtPtr.token;
    }
    if ctxtPtr.charset != XML_CHAR_ENCODING_UTF8 as i32 {
        let mut guess: *mut xmlChar = 0 as *mut xmlChar;
        let mut handler: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
        /*
         * Assume it's a fixed length encoding (1) with
         * a compatible encoding for the ASCII set, since
         * HTML constructs only use < 128 chars
         */
        if CUR(ctxt) < 0x80 {
            unsafe {
                *len = 1;
            }
            if CUR(ctxt) == 0 && inputPtr.cur < inputPtr.end {
                htmlParseErrInt(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"Char 0x%X out of allowed range\n\x00" as *const u8 as *const i8,
                    0,
                );
                return ' ' as i32;
            }
            return CUR(ctxt);
        }
        /*
         * Humm this is bad, do an automatic flow conversion
         */
        guess = htmlFindEncoding(ctxt);
        if guess.is_null() {
            unsafe { xmlSwitchEncoding_safe(ctxt, XML_CHAR_ENCODING_8859_1) };
        } else {
            if !inputPtr.encoding.is_null() {
                unsafe { xmlFree_safe(inputPtr.encoding as *mut xmlChar as *mut ()) };
            }
            inputPtr.encoding = guess;
            handler = unsafe { xmlFindCharEncodingHandler_safe(guess as *const i8) };
            if !handler.is_null() {
                /*
                 * Don't use UTF-8 encoder which isn't required and
                 * can produce invalid UTF-8.
                 */
                let mut handlerPtr = unsafe { &mut *handler };
                if unsafe {
                    xmlStrEqual_safe(
                        handlerPtr.name as *mut xmlChar,
                        b"UTF-8\x00" as *const u8 as *const i8 as *mut xmlChar,
                    )
                } == 0
                {
                    unsafe { xmlSwitchToEncoding_safe(ctxt, handler) };
                }
            } else {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INVALID_ENCODING,
                    b"Unsupported encoding %s\x00" as *const u8 as *const i8,
                    guess,
                    0 as *const xmlChar,
                );
            }
        }
        ctxtPtr.charset = XML_CHAR_ENCODING_UTF8 as i32
    }
    /*
     * We are supposed to handle UTF8, check it's valid
     * From rfc2044: encoding of the Unicode values on UTF-8:
     *
     * UCS-4 range (hex.)           UTF-8 octet sequence (binary)
     * 0000 0000-0000 007F   0xxxxxxx
     * 0000 0080-0000 07FF   110xxxxx 10xxxxxx
     * 0000 0800-0000 FFFF   1110xxxx 10xxxxxx 10xxxxxx
     *
     * Check for the 0x110000 limit too
     */
    cur = inputPtr.cur;
    unsafe {
        c = *cur;
    }
    if c as i32 & 0x80 != 0 {
        if !(c as i32 & 0x40 == 0) {
            unsafe {
                if *cur.offset(1 as isize) as i32 == 0 {
                    xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK);
                    cur = inputPtr.cur
                }
            }
            unsafe {
                if !(*cur.offset(1 as isize) as i32 & 0xc0 != 0x80) {
                    if c as i32 & 0xe0 == 0xe0 {
                        if *cur.offset(2 as isize) as i32 == 0 {
                            xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK);
                            cur = inputPtr.cur
                        }
                        if *cur.offset(2 as isize) as i32 & 0xc0 != 0x80 {
                            current_block = 1;
                        } else if c as i32 & 0xf0 == 0xf0 {
                            if *cur.offset(3 as isize) as i32 == 0 {
                                xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK);
                                cur = inputPtr.cur
                            }
                            if c as i32 & 0xf8 != 0xf0
                                || *cur.offset(3 as isize) as i32 & 0xc0 != 0x80
                            {
                                current_block = 1;
                            } else {
                                /* 4-byte code */
                                *len = 4;
                                val = ((*cur.offset(0 as isize) as i32 & 0x7) << 18) as u32;
                                val |= ((*cur.offset(1 as isize) as i32 & 0x3f) << 12) as u32;
                                val |= ((*cur.offset(2 as isize) as i32 & 0x3f) << 6) as u32;
                                val |= (*cur.offset(3 as isize) as i32 & 0x3f) as u32;
                                if val < 0x10000 as u32 {
                                    current_block = 1;
                                } else {
                                    current_block = 2;
                                }
                            }
                        } else {
                            /* 3-byte code */
                            *len = 3;
                            val = ((*cur.offset(0 as isize) as i32 & 0xf) << 12) as u32;
                            val |= ((*cur.offset(1 as isize) as i32 & 0x3f) << 6) as u32;
                            val |= (*cur.offset(2 as isize) as i32 & 0x3f) as u32;
                            if val < 0x800 as u32 {
                                current_block = 1;
                            } else {
                                current_block = 2;
                            }
                        }
                    } else {
                        /* 2-byte code */
                        *len = 2;
                        val = ((*cur.offset(0 as isize) as i32 & 0x1f) << 6) as u32;
                        val |= (*cur.offset(1 as isize) as i32 & 0x3f) as u32;
                        if val < 0x80 as u32 {
                            current_block = 1;
                        } else {
                            current_block = 2;
                        }
                    }
                    match current_block {
                        1 => {}
                        _ => {
                            if if val < 0x100 as u32 {
                                (0x9 as u32 <= val && val <= 0xa as u32
                                    || val == 0xd as u32
                                    || 0x20 as u32 <= val) as i32
                            } else {
                                (0x100 as u32 <= val && val <= 0xd7ff as u32
                                    || 0xe000 as u32 <= val && val <= 0xfffd as u32
                                    || 0x10000 as u32 <= val && val <= 0x10ffff as u32)
                                    as i32
                            } == 0
                            {
                                htmlParseErrInt(
                                    ctxt,
                                    XML_ERR_INVALID_CHAR,
                                    b"Char 0x%X out of allowed range\n\x00" as *const u8
                                        as *const i8,
                                    val as i32,
                                );
                            }
                            return val as i32;
                        }
                    }
                }
            }
        }
        /*
         * If we detect an UTF8 error that probably mean that the
         * input encoding didn't get properly advertised in the
         * declaration header. Report the error and switch the encoding
         * to ISO-Latin-1 (if you don't like this policy, just declare the
         * encoding !)
         */
        let mut buffer: [i8; 150] = [0; 150];
        unsafe {
            if inputPtr.end.offset_from(inputPtr.cur) as i64 >= 4 {
                snprintf_safe_macro!(
                    buffer.as_mut_ptr(),
                    149,
                    b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8 as *const i8,
                    *inputPtr.cur.offset(0 as isize) as i32,
                    *inputPtr.cur.offset(1 as isize) as i32,
                    *inputPtr.cur.offset(2 as isize) as i32,
                    *inputPtr.cur.offset(3 as isize) as i32
                );
            } else {
                snprintf_safe_macro!(
                    buffer.as_mut_ptr(),
                    149,
                    b"Bytes: 0x%02X\n\x00" as *const u8 as *const i8,
                    *inputPtr.cur.offset(0 as isize) as i32
                );
            }
        }
        unsafe {
            htmlParseErr(
                ctxt,
                XML_ERR_INVALID_ENCODING,
                b"Input is not proper UTF-8, indicate encoding !\n\x00" as *const u8 as *const i8,
                buffer.as_mut_ptr() as *mut xmlChar,
                0 as *const xmlChar,
            )
        };
        /*
         * Don't switch encodings twice. Note that if there's an encoder, we
         * shouldn't receive invalid UTF-8 anyway.
         *
         * Note that if ctxt->input->buf == NULL, switching encodings is
         * impossible, see Gitlab issue #34.
         */
        unsafe {
            if !inputPtr.buf.is_null() && (*inputPtr.buf).encoder.is_null() {
                xmlSwitchEncoding_safe(ctxt, XML_CHAR_ENCODING_8859_1);
            }
        }
        unsafe {
            *len = 1;
        }
        return CUR(ctxt);
    } else {
        if CUR(ctxt) == 0 && inputPtr.cur < inputPtr.end {
            htmlParseErrInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"Char 0x%X out of allowed range\n\x00" as *const u8 as *const i8,
                0,
            );
            unsafe {
                *len = 1;
            }
            return ' ' as i32;
        }
        /* 1-byte code */
        unsafe {
            *len = 1;
        }
        return CUR(ctxt);
    };
}
/* *
 * htmlSkipBlankChars:
 * @ctxt:  the HTML parser context
 *
 * skip all blanks character found at that point in the input streams.
 *
 * Returns the number of space chars skipped
 */
fn htmlSkipBlankChars(ctxt: xmlParserCtxtPtr) -> i32 {
    let ctxtPtr = unsafe { &mut *ctxt };
    let inputPtr = unsafe { &mut *(*ctxt).input };
    let mut res: i32 = 0;
    while IS_BLANK_CH(CUR(ctxt)) {
        if CUR(ctxt) == 0 && unsafe { xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK) <= 0 } {
            unsafe { xmlPopInput_safe(ctxt) };
        } else {
            if CUR(ctxt) == '\n' as i32 {
                inputPtr.line += 1;
                inputPtr.col = 1 as i32
            } else {
                inputPtr.col += 1
            }
            unsafe {
                inputPtr.cur = inputPtr.cur.offset(1);
            }
            if CUR(ctxt) == 0 {
                unsafe { xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK) };
            }
        }
        res += 1
    }
    return res;
}

static mut html_flow: [*const i8; 64] = [
    b"h1\x00" as *const u8 as *const i8,
    b"h2\x00" as *const u8 as *const i8,
    b"h3\x00" as *const u8 as *const i8,
    b"h4\x00" as *const u8 as *const i8,
    b"h5\x00" as *const u8 as *const i8,
    b"h6\x00" as *const u8 as *const i8,
    b"ul\x00" as *const u8 as *const i8,
    b"ol\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"menu\x00" as *const u8 as *const i8,
    b"pre\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"dl\x00" as *const u8 as *const i8,
    b"div\x00" as *const u8 as *const i8,
    b"center\x00" as *const u8 as *const i8,
    b"noscript\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    b"blockquote\x00" as *const u8 as *const i8,
    b"form\x00" as *const u8 as *const i8,
    b"isindex\x00" as *const u8 as *const i8,
    b"hr\x00" as *const u8 as *const i8,
    b"table\x00" as *const u8 as *const i8,
    b"fieldset\x00" as *const u8 as *const i8,
    b"address\x00" as *const u8 as *const i8,
    b"tt\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"big\x00" as *const u8 as *const i8,
    b"small\x00" as *const u8 as *const i8,
    b"em\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"img\x00" as *const u8 as *const i8,
    b"applet\x00" as *const u8 as *const i8,
    b"embed\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    b"font\x00" as *const u8 as *const i8,
    b"basefont\x00" as *const u8 as *const i8,
    b"br\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"map\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"sub\x00" as *const u8 as *const i8,
    b"sup\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    b"input\x00" as *const u8 as *const i8,
    b"select\x00" as *const u8 as *const i8,
    b"textarea\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"button\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut html_inline: [*const i8; 40] = [
    b"tt\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"big\x00" as *const u8 as *const i8,
    b"small\x00" as *const u8 as *const i8,
    b"em\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"img\x00" as *const u8 as *const i8,
    b"applet\x00" as *const u8 as *const i8,
    b"embed\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    b"font\x00" as *const u8 as *const i8,
    b"basefont\x00" as *const u8 as *const i8,
    b"br\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"map\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"sub\x00" as *const u8 as *const i8,
    b"sup\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    b"input\x00" as *const u8 as *const i8,
    b"select\x00" as *const u8 as *const i8,
    b"textarea\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"button\x00" as *const u8 as *const i8,
    0 as *const i8,
];
/* placeholders: elts with content but no subelements */
static mut html_pcdata: [*const i8; 1] = [0 as *const i8];
static mut html_attrs: [*const i8; 16] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut core_i18n_attrs: [*const i8; 7] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut core_attrs: [*const i8; 5] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut i18n_attrs: [*const i8; 3] = [
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    0 as *const i8,
];
/* Other declarations that should go inline ... */
static mut a_attrs: [*const i8; 29] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"charset\x00" as *const u8 as *const i8,
    b"type\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"href\x00" as *const u8 as *const i8,
    b"hreflang\x00" as *const u8 as *const i8,
    b"rel\x00" as *const u8 as *const i8,
    b"rev\x00" as *const u8 as *const i8,
    b"accesskey\x00" as *const u8 as *const i8,
    b"shape\x00" as *const u8 as *const i8,
    b"coords\x00" as *const u8 as *const i8,
    b"tabindex\x00" as *const u8 as *const i8,
    b"onfocus\x00" as *const u8 as *const i8,
    b"onblur\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut target_attr: [*const i8; 2] = [b"target\x00" as *const u8 as *const i8, 0 as *const i8];
static mut rows_cols_attr: [*const i8; 3] = [
    b"rows\x00" as *const u8 as *const i8,
    b"cols\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut alt_attr: [*const i8; 2] = [b"alt\x00" as *const u8 as *const i8, 0 as *const i8];
static mut src_alt_attrs: [*const i8; 3] = [
    b"src\x00" as *const u8 as *const i8,
    b"alt\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut href_attrs: [*const i8; 2] = [b"href\x00" as *const u8 as *const i8, 0 as *const i8];
static mut clear_attrs: [*const i8; 2] = [b"clear\x00" as *const u8 as *const i8, 0 as *const i8];
static mut inline_p: [*const i8; 41] = [
    b"tt\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"big\x00" as *const u8 as *const i8,
    b"small\x00" as *const u8 as *const i8,
    b"em\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"img\x00" as *const u8 as *const i8,
    b"applet\x00" as *const u8 as *const i8,
    b"embed\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    b"font\x00" as *const u8 as *const i8,
    b"basefont\x00" as *const u8 as *const i8,
    b"br\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"map\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"sub\x00" as *const u8 as *const i8,
    b"sup\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    b"input\x00" as *const u8 as *const i8,
    b"select\x00" as *const u8 as *const i8,
    b"textarea\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"button\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut flow_param: [*const i8; 65] = [
    b"h1\x00" as *const u8 as *const i8,
    b"h2\x00" as *const u8 as *const i8,
    b"h3\x00" as *const u8 as *const i8,
    b"h4\x00" as *const u8 as *const i8,
    b"h5\x00" as *const u8 as *const i8,
    b"h6\x00" as *const u8 as *const i8,
    b"ul\x00" as *const u8 as *const i8,
    b"ol\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"menu\x00" as *const u8 as *const i8,
    b"pre\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"dl\x00" as *const u8 as *const i8,
    b"div\x00" as *const u8 as *const i8,
    b"center\x00" as *const u8 as *const i8,
    b"noscript\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    b"blockquote\x00" as *const u8 as *const i8,
    b"form\x00" as *const u8 as *const i8,
    b"isindex\x00" as *const u8 as *const i8,
    b"hr\x00" as *const u8 as *const i8,
    b"table\x00" as *const u8 as *const i8,
    b"fieldset\x00" as *const u8 as *const i8,
    b"address\x00" as *const u8 as *const i8,
    b"tt\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"big\x00" as *const u8 as *const i8,
    b"small\x00" as *const u8 as *const i8,
    b"em\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"img\x00" as *const u8 as *const i8,
    b"applet\x00" as *const u8 as *const i8,
    b"embed\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    b"font\x00" as *const u8 as *const i8,
    b"basefont\x00" as *const u8 as *const i8,
    b"br\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"map\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"sub\x00" as *const u8 as *const i8,
    b"sup\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    b"input\x00" as *const u8 as *const i8,
    b"select\x00" as *const u8 as *const i8,
    b"textarea\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"button\x00" as *const u8 as *const i8,
    b"param\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut applet_attrs: [*const i8; 14] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"codebase\x00" as *const u8 as *const i8,
    b"archive\x00" as *const u8 as *const i8,
    b"alt\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"height\x00" as *const u8 as *const i8,
    b"width\x00" as *const u8 as *const i8,
    b"align\x00" as *const u8 as *const i8,
    b"hspace\x00" as *const u8 as *const i8,
    b"vspace\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut area_attrs: [*const i8; 9] = [
    b"shape\x00" as *const u8 as *const i8,
    b"coords\x00" as *const u8 as *const i8,
    b"href\x00" as *const u8 as *const i8,
    b"nohref\x00" as *const u8 as *const i8,
    b"tabindex\x00" as *const u8 as *const i8,
    b"accesskey\x00" as *const u8 as *const i8,
    b"onfocus\x00" as *const u8 as *const i8,
    b"onblur\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut basefont_attrs: [*const i8; 5] = [
    b"id\x00" as *const u8 as *const i8,
    b"size\x00" as *const u8 as *const i8,
    b"color\x00" as *const u8 as *const i8,
    b"face\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut quote_attrs: [*const i8; 17] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut body_contents: [*const i8; 66] = [
    b"h1\x00" as *const u8 as *const i8,
    b"h2\x00" as *const u8 as *const i8,
    b"h3\x00" as *const u8 as *const i8,
    b"h4\x00" as *const u8 as *const i8,
    b"h5\x00" as *const u8 as *const i8,
    b"h6\x00" as *const u8 as *const i8,
    b"ul\x00" as *const u8 as *const i8,
    b"ol\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"menu\x00" as *const u8 as *const i8,
    b"pre\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"dl\x00" as *const u8 as *const i8,
    b"div\x00" as *const u8 as *const i8,
    b"center\x00" as *const u8 as *const i8,
    b"noscript\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    b"blockquote\x00" as *const u8 as *const i8,
    b"form\x00" as *const u8 as *const i8,
    b"isindex\x00" as *const u8 as *const i8,
    b"hr\x00" as *const u8 as *const i8,
    b"table\x00" as *const u8 as *const i8,
    b"fieldset\x00" as *const u8 as *const i8,
    b"address\x00" as *const u8 as *const i8,
    b"tt\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"big\x00" as *const u8 as *const i8,
    b"small\x00" as *const u8 as *const i8,
    b"em\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"img\x00" as *const u8 as *const i8,
    b"applet\x00" as *const u8 as *const i8,
    b"embed\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    b"font\x00" as *const u8 as *const i8,
    b"basefont\x00" as *const u8 as *const i8,
    b"br\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"map\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"sub\x00" as *const u8 as *const i8,
    b"sup\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    b"input\x00" as *const u8 as *const i8,
    b"select\x00" as *const u8 as *const i8,
    b"textarea\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"button\x00" as *const u8 as *const i8,
    b"ins\x00" as *const u8 as *const i8,
    b"del\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut body_attrs: [*const i8; 18] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"onload\x00" as *const u8 as *const i8,
    b"onunload\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut body_depr: [*const i8; 7] = [
    b"background\x00" as *const u8 as *const i8,
    b"bgcolor\x00" as *const u8 as *const i8,
    b"text\x00" as *const u8 as *const i8,
    b"link\x00" as *const u8 as *const i8,
    b"vlink\x00" as *const u8 as *const i8,
    b"alink\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut button_attrs: [*const i8; 24] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"value\x00" as *const u8 as *const i8,
    b"type\x00" as *const u8 as *const i8,
    b"disabled\x00" as *const u8 as *const i8,
    b"tabindex\x00" as *const u8 as *const i8,
    b"accesskey\x00" as *const u8 as *const i8,
    b"onfocus\x00" as *const u8 as *const i8,
    b"onblur\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut col_attrs: [*const i8; 22] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"width\x00" as *const u8 as *const i8,
    b"align\x00" as *const u8 as *const i8,
    b"char\x00" as *const u8 as *const i8,
    b"charoff\x00" as *const u8 as *const i8,
    b"valign\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut col_elt: [*const i8; 2] = [b"col\x00" as *const u8 as *const i8, 0 as *const i8];
static mut edit_attrs: [*const i8; 18] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"datetime\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut compact_attrs: [*const i8; 17] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"compact\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut dl_contents: [*const i8; 3] = [
    b"dt\x00" as *const u8 as *const i8,
    b"dd\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut compact_attr: [*const i8; 2] =
    [b"compact\x00" as *const u8 as *const i8, 0 as *const i8];
static mut label_attr: [*const i8; 2] = [b"label\x00" as *const u8 as *const i8, 0 as *const i8];
static mut fieldset_contents: [*const i8; 64] = [
    b"h1\x00" as *const u8 as *const i8,
    b"h2\x00" as *const u8 as *const i8,
    b"h3\x00" as *const u8 as *const i8,
    b"h4\x00" as *const u8 as *const i8,
    b"h5\x00" as *const u8 as *const i8,
    b"h6\x00" as *const u8 as *const i8,
    b"ul\x00" as *const u8 as *const i8,
    b"ol\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"menu\x00" as *const u8 as *const i8,
    b"pre\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"dl\x00" as *const u8 as *const i8,
    b"div\x00" as *const u8 as *const i8,
    b"center\x00" as *const u8 as *const i8,
    b"noscript\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    b"blockquote\x00" as *const u8 as *const i8,
    b"form\x00" as *const u8 as *const i8,
    b"isindex\x00" as *const u8 as *const i8,
    b"hr\x00" as *const u8 as *const i8,
    b"table\x00" as *const u8 as *const i8,
    b"fieldset\x00" as *const u8 as *const i8,
    b"address\x00" as *const u8 as *const i8,
    b"tt\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"big\x00" as *const u8 as *const i8,
    b"small\x00" as *const u8 as *const i8,
    b"em\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"img\x00" as *const u8 as *const i8,
    b"applet\x00" as *const u8 as *const i8,
    b"embed\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    b"font\x00" as *const u8 as *const i8,
    b"basefont\x00" as *const u8 as *const i8,
    b"br\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"map\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"sub\x00" as *const u8 as *const i8,
    b"sup\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    b"input\x00" as *const u8 as *const i8,
    b"select\x00" as *const u8 as *const i8,
    b"textarea\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"button\x00" as *const u8 as *const i8,
    b"legend\x00" as *const u8 as *const i8,
];
static mut font_attrs: [*const i8; 10] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"size\x00" as *const u8 as *const i8,
    b"color\x00" as *const u8 as *const i8,
    b"face\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut form_contents: [*const i8; 62] = [
    b"h1\x00" as *const u8 as *const i8,
    b"h2\x00" as *const u8 as *const i8,
    b"h3\x00" as *const u8 as *const i8,
    b"h4\x00" as *const u8 as *const i8,
    b"h5\x00" as *const u8 as *const i8,
    b"h6\x00" as *const u8 as *const i8,
    b"ul\x00" as *const u8 as *const i8,
    b"ol\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"menu\x00" as *const u8 as *const i8,
    b"tt\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"big\x00" as *const u8 as *const i8,
    b"small\x00" as *const u8 as *const i8,
    b"em\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"img\x00" as *const u8 as *const i8,
    b"applet\x00" as *const u8 as *const i8,
    b"embed\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    b"font\x00" as *const u8 as *const i8,
    b"basefont\x00" as *const u8 as *const i8,
    b"br\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"map\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"sub\x00" as *const u8 as *const i8,
    b"sup\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    b"input\x00" as *const u8 as *const i8,
    b"select\x00" as *const u8 as *const i8,
    b"textarea\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"button\x00" as *const u8 as *const i8,
    b"pre\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"div\x00" as *const u8 as *const i8,
    b"center\x00" as *const u8 as *const i8,
    b"noscript\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    b"blockquote\x00" as *const u8 as *const i8,
    b"isindex\x00" as *const u8 as *const i8,
    b"hr\x00" as *const u8 as *const i8,
    b"table\x00" as *const u8 as *const i8,
    b"fieldset\x00" as *const u8 as *const i8,
    b"address\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut form_attrs: [*const i8; 23] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"method\x00" as *const u8 as *const i8,
    b"enctype\x00" as *const u8 as *const i8,
    b"accept\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"onsubmit\x00" as *const u8 as *const i8,
    b"onreset\x00" as *const u8 as *const i8,
    b"accept-charset\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut frame_attrs: [*const i8; 13] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"longdesc\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"src\x00" as *const u8 as *const i8,
    b"frameborder\x00" as *const u8 as *const i8,
    b"marginwidth\x00" as *const u8 as *const i8,
    b"marginheight\x00" as *const u8 as *const i8,
    b"noresize\x00" as *const u8 as *const i8,
    b"scrolling\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut frameset_attrs: [*const i8; 9] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"rows\x00" as *const u8 as *const i8,
    b"cols\x00" as *const u8 as *const i8,
    b"onload\x00" as *const u8 as *const i8,
    b"onunload\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut frameset_contents: [*const i8; 4] = [
    b"frameset\x00" as *const u8 as *const i8,
    b"frame\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut head_attrs: [*const i8; 4] = [
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"profile\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut head_contents: [*const i8; 9] = [
    b"title\x00" as *const u8 as *const i8,
    b"isindex\x00" as *const u8 as *const i8,
    b"base\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"meta\x00" as *const u8 as *const i8,
    b"link\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut hr_depr: [*const i8; 5] = [
    b"align\x00" as *const u8 as *const i8,
    b"noshade\x00" as *const u8 as *const i8,
    b"size\x00" as *const u8 as *const i8,
    b"width\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut version_attr: [*const i8; 2] =
    [b"version\x00" as *const u8 as *const i8, 0 as *const i8];
static mut html_content: [*const i8; 4] = [
    b"head\x00" as *const u8 as *const i8,
    b"body\x00" as *const u8 as *const i8,
    b"frameset\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut iframe_attrs: [*const i8; 15] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"longdesc\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"src\x00" as *const u8 as *const i8,
    b"frameborder\x00" as *const u8 as *const i8,
    b"marginwidth\x00" as *const u8 as *const i8,
    b"marginheight\x00" as *const u8 as *const i8,
    b"scrolling\x00" as *const u8 as *const i8,
    b"align\x00" as *const u8 as *const i8,
    b"height\x00" as *const u8 as *const i8,
    b"width\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut img_attrs: [*const i8; 22] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"longdesc\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"height\x00" as *const u8 as *const i8,
    b"width\x00" as *const u8 as *const i8,
    b"usemap\x00" as *const u8 as *const i8,
    b"ismap\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut embed_attrs: [*const i8; 23] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"align\x00" as *const u8 as *const i8,
    b"alt\x00" as *const u8 as *const i8,
    b"border\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"codebase\x00" as *const u8 as *const i8,
    b"frameborder\x00" as *const u8 as *const i8,
    b"height\x00" as *const u8 as *const i8,
    b"hidden\x00" as *const u8 as *const i8,
    b"hspace\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"palette\x00" as *const u8 as *const i8,
    b"pluginspace\x00" as *const u8 as *const i8,
    b"pluginurl\x00" as *const u8 as *const i8,
    b"src\x00" as *const u8 as *const i8,
    b"type\x00" as *const u8 as *const i8,
    b"units\x00" as *const u8 as *const i8,
    b"vspace\x00" as *const u8 as *const i8,
    b"width\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut input_attrs: [*const i8; 35] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"type\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"value\x00" as *const u8 as *const i8,
    b"checked\x00" as *const u8 as *const i8,
    b"disabled\x00" as *const u8 as *const i8,
    b"readonly\x00" as *const u8 as *const i8,
    b"size\x00" as *const u8 as *const i8,
    b"maxlength\x00" as *const u8 as *const i8,
    b"src\x00" as *const u8 as *const i8,
    b"alt\x00" as *const u8 as *const i8,
    b"usemap\x00" as *const u8 as *const i8,
    b"ismap\x00" as *const u8 as *const i8,
    b"tabindex\x00" as *const u8 as *const i8,
    b"accesskey\x00" as *const u8 as *const i8,
    b"onfocus\x00" as *const u8 as *const i8,
    b"onblur\x00" as *const u8 as *const i8,
    b"onselect\x00" as *const u8 as *const i8,
    b"onchange\x00" as *const u8 as *const i8,
    b"accept\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut prompt_attrs: [*const i8; 8] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"prompt\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut label_attrs: [*const i8; 20] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"for\x00" as *const u8 as *const i8,
    b"accesskey\x00" as *const u8 as *const i8,
    b"onfocus\x00" as *const u8 as *const i8,
    b"onblur\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut legend_attrs: [*const i8; 17] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"accesskey\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut align_attr: [*const i8; 2] = [b"align\x00" as *const u8 as *const i8, 0 as *const i8];
static mut link_attrs: [*const i8; 23] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"charset\x00" as *const u8 as *const i8,
    b"href\x00" as *const u8 as *const i8,
    b"hreflang\x00" as *const u8 as *const i8,
    b"type\x00" as *const u8 as *const i8,
    b"rel\x00" as *const u8 as *const i8,
    b"rev\x00" as *const u8 as *const i8,
    b"media\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut map_contents: [*const i8; 26] = [
    b"h1\x00" as *const u8 as *const i8,
    b"h2\x00" as *const u8 as *const i8,
    b"h3\x00" as *const u8 as *const i8,
    b"h4\x00" as *const u8 as *const i8,
    b"h5\x00" as *const u8 as *const i8,
    b"h6\x00" as *const u8 as *const i8,
    b"ul\x00" as *const u8 as *const i8,
    b"ol\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"menu\x00" as *const u8 as *const i8,
    b"pre\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"dl\x00" as *const u8 as *const i8,
    b"div\x00" as *const u8 as *const i8,
    b"center\x00" as *const u8 as *const i8,
    b"noscript\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    b"blockquote\x00" as *const u8 as *const i8,
    b"form\x00" as *const u8 as *const i8,
    b"isindex\x00" as *const u8 as *const i8,
    b"hr\x00" as *const u8 as *const i8,
    b"table\x00" as *const u8 as *const i8,
    b"fieldset\x00" as *const u8 as *const i8,
    b"address\x00" as *const u8 as *const i8,
    b"area\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut name_attr: [*const i8; 2] = [b"name\x00" as *const u8 as *const i8, 0 as *const i8];
static mut action_attr: [*const i8; 2] = [b"action\x00" as *const u8 as *const i8, 0 as *const i8];
static mut blockli_elt: [*const i8; 26] = [
    b"h1\x00" as *const u8 as *const i8,
    b"h2\x00" as *const u8 as *const i8,
    b"h3\x00" as *const u8 as *const i8,
    b"h4\x00" as *const u8 as *const i8,
    b"h5\x00" as *const u8 as *const i8,
    b"h6\x00" as *const u8 as *const i8,
    b"ul\x00" as *const u8 as *const i8,
    b"ol\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"menu\x00" as *const u8 as *const i8,
    b"pre\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"dl\x00" as *const u8 as *const i8,
    b"div\x00" as *const u8 as *const i8,
    b"center\x00" as *const u8 as *const i8,
    b"noscript\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    b"blockquote\x00" as *const u8 as *const i8,
    b"form\x00" as *const u8 as *const i8,
    b"isindex\x00" as *const u8 as *const i8,
    b"hr\x00" as *const u8 as *const i8,
    b"table\x00" as *const u8 as *const i8,
    b"fieldset\x00" as *const u8 as *const i8,
    b"address\x00" as *const u8 as *const i8,
    b"li\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut meta_attrs: [*const i8; 7] = [
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"http-equiv\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"scheme\x00" as *const u8 as *const i8,
    b"charset\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut content_attr: [*const i8; 2] =
    [b"content\x00" as *const u8 as *const i8, 0 as *const i8];
static mut type_attr: [*const i8; 2] = [b"type\x00" as *const u8 as *const i8, 0 as *const i8];
static mut noframes_content: [*const i8; 65] = [
    b"body\x00" as *const u8 as *const i8,
    b"h1\x00" as *const u8 as *const i8,
    b"h2\x00" as *const u8 as *const i8,
    b"h3\x00" as *const u8 as *const i8,
    b"h4\x00" as *const u8 as *const i8,
    b"h5\x00" as *const u8 as *const i8,
    b"h6\x00" as *const u8 as *const i8,
    b"ul\x00" as *const u8 as *const i8,
    b"ol\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"menu\x00" as *const u8 as *const i8,
    b"pre\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"dl\x00" as *const u8 as *const i8,
    b"div\x00" as *const u8 as *const i8,
    b"center\x00" as *const u8 as *const i8,
    b"noscript\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    b"blockquote\x00" as *const u8 as *const i8,
    b"form\x00" as *const u8 as *const i8,
    b"isindex\x00" as *const u8 as *const i8,
    b"hr\x00" as *const u8 as *const i8,
    b"table\x00" as *const u8 as *const i8,
    b"fieldset\x00" as *const u8 as *const i8,
    b"address\x00" as *const u8 as *const i8,
    b"tt\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"big\x00" as *const u8 as *const i8,
    b"small\x00" as *const u8 as *const i8,
    b"em\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"img\x00" as *const u8 as *const i8,
    b"applet\x00" as *const u8 as *const i8,
    b"embed\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    b"font\x00" as *const u8 as *const i8,
    b"basefont\x00" as *const u8 as *const i8,
    b"br\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"map\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"sub\x00" as *const u8 as *const i8,
    b"sup\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    b"input\x00" as *const u8 as *const i8,
    b"select\x00" as *const u8 as *const i8,
    b"textarea\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"button\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut object_contents: [*const i8; 65] = [
    b"h1\x00" as *const u8 as *const i8,
    b"h2\x00" as *const u8 as *const i8,
    b"h3\x00" as *const u8 as *const i8,
    b"h4\x00" as *const u8 as *const i8,
    b"h5\x00" as *const u8 as *const i8,
    b"h6\x00" as *const u8 as *const i8,
    b"ul\x00" as *const u8 as *const i8,
    b"ol\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"menu\x00" as *const u8 as *const i8,
    b"pre\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"dl\x00" as *const u8 as *const i8,
    b"div\x00" as *const u8 as *const i8,
    b"center\x00" as *const u8 as *const i8,
    b"noscript\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    b"blockquote\x00" as *const u8 as *const i8,
    b"form\x00" as *const u8 as *const i8,
    b"isindex\x00" as *const u8 as *const i8,
    b"hr\x00" as *const u8 as *const i8,
    b"table\x00" as *const u8 as *const i8,
    b"fieldset\x00" as *const u8 as *const i8,
    b"address\x00" as *const u8 as *const i8,
    b"tt\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"big\x00" as *const u8 as *const i8,
    b"small\x00" as *const u8 as *const i8,
    b"em\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"img\x00" as *const u8 as *const i8,
    b"applet\x00" as *const u8 as *const i8,
    b"embed\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    b"font\x00" as *const u8 as *const i8,
    b"basefont\x00" as *const u8 as *const i8,
    b"br\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"map\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"sub\x00" as *const u8 as *const i8,
    b"sup\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    b"input\x00" as *const u8 as *const i8,
    b"select\x00" as *const u8 as *const i8,
    b"textarea\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"button\x00" as *const u8 as *const i8,
    b"param\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut object_attrs: [*const i8; 29] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"declare\x00" as *const u8 as *const i8,
    b"classid\x00" as *const u8 as *const i8,
    b"codebase\x00" as *const u8 as *const i8,
    b"data\x00" as *const u8 as *const i8,
    b"type\x00" as *const u8 as *const i8,
    b"codetype\x00" as *const u8 as *const i8,
    b"archive\x00" as *const u8 as *const i8,
    b"standby\x00" as *const u8 as *const i8,
    b"height\x00" as *const u8 as *const i8,
    b"width\x00" as *const u8 as *const i8,
    b"usemap\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"tabindex\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut object_depr: [*const i8; 5] = [
    b"align\x00" as *const u8 as *const i8,
    b"border\x00" as *const u8 as *const i8,
    b"hspace\x00" as *const u8 as *const i8,
    b"vspace\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut ol_attrs: [*const i8; 4] = [
    b"type\x00" as *const u8 as *const i8,
    b"compact\x00" as *const u8 as *const i8,
    b"start\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut option_elt: [*const i8; 2] = [b"option\x00" as *const u8 as *const i8, 0 as *const i8];
static mut optgroup_attrs: [*const i8; 17] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"disabled\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut option_attrs: [*const i8; 20] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"disabled\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"selected\x00" as *const u8 as *const i8,
    b"value\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut param_attrs: [*const i8; 5] = [
    b"id\x00" as *const u8 as *const i8,
    b"value\x00" as *const u8 as *const i8,
    b"valuetype\x00" as *const u8 as *const i8,
    b"type\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut width_attr: [*const i8; 2] = [b"width\x00" as *const u8 as *const i8, 0 as *const i8];
static mut pre_content: [*const i8; 25] = [
    b"em\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"tt\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"a\x00" as *const u8 as *const i8,
    b"br\x00" as *const u8 as *const i8,
    b"script\x00" as *const u8 as *const i8,
    b"map\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut script_attrs: [*const i8; 6] = [
    b"charset\x00" as *const u8 as *const i8,
    b"src\x00" as *const u8 as *const i8,
    b"defer\x00" as *const u8 as *const i8,
    b"event\x00" as *const u8 as *const i8,
    b"for\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut language_attr: [*const i8; 2] =
    [b"language\x00" as *const u8 as *const i8, 0 as *const i8];
static mut select_content: [*const i8; 3] = [
    b"optgroup\x00" as *const u8 as *const i8,
    b"option\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut select_attrs: [*const i8; 24] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"size\x00" as *const u8 as *const i8,
    b"multiple\x00" as *const u8 as *const i8,
    b"disabled\x00" as *const u8 as *const i8,
    b"tabindex\x00" as *const u8 as *const i8,
    b"onfocus\x00" as *const u8 as *const i8,
    b"onblur\x00" as *const u8 as *const i8,
    b"onchange\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut style_attrs: [*const i8; 5] = [
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"media\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut table_attrs: [*const i8; 24] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"summary\x00" as *const u8 as *const i8,
    b"width\x00" as *const u8 as *const i8,
    b"border\x00" as *const u8 as *const i8,
    b"frame\x00" as *const u8 as *const i8,
    b"rules\x00" as *const u8 as *const i8,
    b"cellspacing\x00" as *const u8 as *const i8,
    b"cellpadding\x00" as *const u8 as *const i8,
    b"datapagesize\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut table_depr: [*const i8; 3] = [
    b"align\x00" as *const u8 as *const i8,
    b"bgcolor\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut table_contents: [*const i8; 8] = [
    b"caption\x00" as *const u8 as *const i8,
    b"col\x00" as *const u8 as *const i8,
    b"colgroup\x00" as *const u8 as *const i8,
    b"thead\x00" as *const u8 as *const i8,
    b"tfoot\x00" as *const u8 as *const i8,
    b"tbody\x00" as *const u8 as *const i8,
    b"tr\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut tr_elt: [*const i8; 2] = [b"tr\x00" as *const u8 as *const i8, 0 as *const i8];
static mut talign_attrs: [*const i8; 20] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"align\x00" as *const u8 as *const i8,
    b"char\x00" as *const u8 as *const i8,
    b"charoff\x00" as *const u8 as *const i8,
    b"valign\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut th_td_depr: [*const i8; 5] = [
    b"nowrap\x00" as *const u8 as *const i8,
    b"bgcolor\x00" as *const u8 as *const i8,
    b"width\x00" as *const u8 as *const i8,
    b"height\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut th_td_attr: [*const i8; 26] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"axis\x00" as *const u8 as *const i8,
    b"headers\x00" as *const u8 as *const i8,
    b"scope\x00" as *const u8 as *const i8,
    b"rowspan\x00" as *const u8 as *const i8,
    b"colspan\x00" as *const u8 as *const i8,
    b"align\x00" as *const u8 as *const i8,
    b"char\x00" as *const u8 as *const i8,
    b"charoff\x00" as *const u8 as *const i8,
    b"valign\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut textarea_attrs: [*const i8; 25] = [
    b"id\x00" as *const u8 as *const i8,
    b"class\x00" as *const u8 as *const i8,
    b"style\x00" as *const u8 as *const i8,
    b"title\x00" as *const u8 as *const i8,
    b"lang\x00" as *const u8 as *const i8,
    b"dir\x00" as *const u8 as *const i8,
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"name\x00" as *const u8 as *const i8,
    b"disabled\x00" as *const u8 as *const i8,
    b"readonly\x00" as *const u8 as *const i8,
    b"tabindex\x00" as *const u8 as *const i8,
    b"accesskey\x00" as *const u8 as *const i8,
    b"onfocus\x00" as *const u8 as *const i8,
    b"onblur\x00" as *const u8 as *const i8,
    b"onselect\x00" as *const u8 as *const i8,
    b"onchange\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut tr_contents: [*const i8; 3] = [
    b"th\x00" as *const u8 as *const i8,
    b"td\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut bgcolor_attr: [*const i8; 2] =
    [b"bgcolor\x00" as *const u8 as *const i8, 0 as *const i8];
static mut li_elt: [*const i8; 2] = [b"li\x00" as *const u8 as *const i8, 0 as *const i8];
static mut ul_depr: [*const i8; 3] = [
    b"type\x00" as *const u8 as *const i8,
    b"compact\x00" as *const u8 as *const i8,
    0 as *const i8,
];
static mut dir_attr: [*const i8; 2] = [b"dir\x00" as *const u8 as *const i8, 0 as *const i8];
static mut html40ElementTable: [htmlElemDesc; 92] = unsafe {
    [
        {
            let init = _htmlElemDesc {
                name: b"a\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"anchor \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: a_attrs.as_ptr() as *mut *const i8,
                attrs_depr: target_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"abbr\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"abbreviated form\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"acronym\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"address\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"information on author \x00" as *const u8 as *const i8,
                subelts: inline_p.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"applet\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 1,
                dtd: 1,
                isinline: 2,
                desc: b"java applet \x00" as *const u8 as *const i8,
                subelts: flow_param.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: applet_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"area\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"client-side image map area \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: area_attrs.as_ptr() as *mut *const i8,
                attrs_depr: target_attr.as_ptr() as *mut *const i8,
                attrs_req: alt_attr.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"b\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"bold text style\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"base\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"document base uri \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: target_attr.as_ptr() as *mut *const i8,
                attrs_req: href_attrs.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"basefont\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 1,
                dtd: 1,
                isinline: 1,
                desc: b"base font size \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: basefont_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"bdo\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"i18n bidi over-ride \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: core_i18n_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: dir_attr.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"big\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"large text style\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"blockquote\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"long quotation \x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: quote_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"body\x00" as *const u8 as *const i8,
                startTag: 1,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"document body \x00" as *const u8 as *const i8,
                subelts: body_contents.as_ptr() as *mut *const i8,
                defaultsubelt: b"div\x00" as *const u8 as *const i8,
                attrs_opt: body_attrs.as_ptr() as *mut *const i8,
                attrs_depr: body_depr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"br\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"forced line break \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: core_attrs.as_ptr() as *mut *const i8,
                attrs_depr: clear_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"button\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 2,
                desc: b"push button \x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: button_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"caption\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"table caption \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"center\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 1,
                dtd: 1,
                isinline: 0,
                desc: b"shorthand for div align=center \x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: html_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"cite\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"citation\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"code\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"computer code fragment\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"col\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"table column \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: col_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"colgroup\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"table column group \x00" as *const u8 as *const i8,
                subelts: col_elt.as_ptr() as *mut *const i8,
                defaultsubelt: b"col\x00" as *const u8 as *const i8,
                attrs_opt: col_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"dd\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"definition description \x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"del\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 2,
                desc: b"deleted text \x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: edit_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"dfn\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"instance definition\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"dir\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 1,
                dtd: 1,
                isinline: 0,
                desc: b"directory list\x00" as *const u8 as *const i8,
                subelts: blockli_elt.as_ptr() as *mut *const i8,
                defaultsubelt: b"li\x00" as *const u8 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: compact_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"div\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"generic language/style container\x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"dl\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"definition list \x00" as *const u8 as *const i8,
                subelts: dl_contents.as_ptr() as *mut *const i8,
                defaultsubelt: b"dd\x00" as *const u8 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: compact_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"dt\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"definition term \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"em\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"emphasis\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"embed\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 1,
                dtd: 1,
                isinline: 1,
                desc: b"generic embedded object \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: embed_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"fieldset\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"form control group \x00" as *const u8 as *const i8,
                subelts: fieldset_contents.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"font\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 1,
                dtd: 1,
                isinline: 1,
                desc: b"local change to font \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: font_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"form\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"interactive form \x00" as *const u8 as *const i8,
                subelts: form_contents.as_ptr() as *mut *const i8,
                defaultsubelt: b"fieldset\x00" as *const u8 as *const i8,
                attrs_opt: form_attrs.as_ptr() as *mut *const i8,
                attrs_depr: target_attr.as_ptr() as *mut *const i8,
                attrs_req: action_attr.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"frame\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 2,
                isinline: 0,
                desc: b"subwindow \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: frame_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"frameset\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 2,
                isinline: 0,
                desc: b"window subdivision\x00" as *const u8 as *const i8,
                subelts: frameset_contents.as_ptr() as *mut *const i8,
                defaultsubelt: b"noframes\x00" as *const u8 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: frameset_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"h1\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"heading \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"h2\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"heading \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"h3\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"heading \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"h4\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"heading \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"h5\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"heading \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"h6\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"heading \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"head\x00" as *const u8 as *const i8,
                startTag: 1,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"document head \x00" as *const u8 as *const i8,
                subelts: head_contents.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: head_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"hr\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"horizontal rule \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: hr_depr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"html\x00" as *const u8 as *const i8,
                startTag: 1,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"document root element \x00" as *const u8 as *const i8,
                subelts: html_content.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: i18n_attrs.as_ptr() as *mut *const i8,
                attrs_depr: version_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"i\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"italic text style\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"iframe\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 1,
                isinline: 2,
                desc: b"inline subwindow \x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: iframe_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"img\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"embedded image \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: img_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: src_alt_attrs.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"input\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"form control \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: input_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"ins\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 2,
                desc: b"inserted text\x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: edit_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"isindex\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 1,
                dtd: 1,
                isinline: 0,
                desc: b"single line prompt \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: prompt_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"kbd\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"text to be entered by the user\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"label\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"form field label text \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: label_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"legend\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"fieldset legend \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: legend_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"li\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 1,
                saveEndTag: 1,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"list item \x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"link\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"a media-independent link \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: link_attrs.as_ptr() as *mut *const i8,
                attrs_depr: target_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"map\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 2,
                desc: b"client-side image map \x00" as *const u8 as *const i8,
                subelts: map_contents.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: name_attr.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"menu\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 1,
                dtd: 1,
                isinline: 0,
                desc: b"menu list \x00" as *const u8 as *const i8,
                subelts: blockli_elt.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: compact_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"meta\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"generic metainformation \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: meta_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: content_attr.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"noframes\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 2,
                isinline: 0,
                desc: b"alternate content container for non frame-based rendering \x00" as *const u8
                    as *const i8,
                subelts: noframes_content.as_ptr() as *mut *const i8,
                defaultsubelt: b"body\x00" as *const u8 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"noscript\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"alternate content container for non script-based rendering \x00"
                    as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: b"div\x00" as *const u8 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"object\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 2,
                desc: b"generic embedded object \x00" as *const u8 as *const i8,
                subelts: object_contents.as_ptr() as *mut *const i8,
                defaultsubelt: b"div\x00" as *const u8 as *const i8,
                attrs_opt: object_attrs.as_ptr() as *mut *const i8,
                attrs_depr: object_depr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"ol\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"ordered list \x00" as *const u8 as *const i8,
                subelts: li_elt.as_ptr() as *mut *const i8,
                defaultsubelt: b"li\x00" as *const u8 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: ol_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"optgroup\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"option group \x00" as *const u8 as *const i8,
                subelts: option_elt.as_ptr() as *mut *const i8,
                defaultsubelt: b"option\x00" as *const u8 as *const i8,
                attrs_opt: optgroup_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: label_attr.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"option\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"selectable choice \x00" as *const u8 as *const i8,
                subelts: html_pcdata.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: option_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"p\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"paragraph \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: align_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"param\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 2,
                saveEndTag: 2,
                empty: 1,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"named property value \x00" as *const u8 as *const i8,
                subelts: 0 as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: param_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: name_attr.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"pre\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"preformatted text \x00" as *const u8 as *const i8,
                subelts: pre_content.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: width_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"q\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"short inline quotation \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: quote_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"s\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 1,
                dtd: 1,
                isinline: 1,
                desc: b"strike-through text style\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: html_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"samp\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"sample program output, scripts, etc.\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"script\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 2,
                desc: b"script statements \x00" as *const u8 as *const i8,
                subelts: html_pcdata.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: script_attrs.as_ptr() as *mut *const i8,
                attrs_depr: language_attr.as_ptr() as *mut *const i8,
                attrs_req: type_attr.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"select\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"option selector \x00" as *const u8 as *const i8,
                subelts: select_content.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: select_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"small\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"small text style\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"span\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"generic language/style container \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"strike\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 1,
                dtd: 1,
                isinline: 1,
                desc: b"strike-through text\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: html_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"strong\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"strong emphasis\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"style\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"style info \x00" as *const u8 as *const i8,
                subelts: html_pcdata.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: style_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: type_attr.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"sub\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"subscript\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"sup\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"superscript \x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"table\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"\x00" as *const u8 as *const i8,
                subelts: table_contents.as_ptr() as *mut *const i8,
                defaultsubelt: b"tr\x00" as *const u8 as *const i8,
                attrs_opt: table_attrs.as_ptr() as *mut *const i8,
                attrs_depr: table_depr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"tbody\x00" as *const u8 as *const i8,
                startTag: 1,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"table body \x00" as *const u8 as *const i8,
                subelts: tr_elt.as_ptr() as *mut *const i8,
                defaultsubelt: b"tr\x00" as *const u8 as *const i8,
                attrs_opt: talign_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"td\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"table data cell\x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: th_td_attr.as_ptr() as *mut *const i8,
                attrs_depr: th_td_depr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"textarea\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"multi-line text field \x00" as *const u8 as *const i8,
                subelts: html_pcdata.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: textarea_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: rows_cols_attr.as_ptr() as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"tfoot\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"table footer \x00" as *const u8 as *const i8,
                subelts: tr_elt.as_ptr() as *mut *const i8,
                defaultsubelt: b"tr\x00" as *const u8 as *const i8,
                attrs_opt: talign_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"th\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"table header cell\x00" as *const u8 as *const i8,
                subelts: html_flow.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: th_td_attr.as_ptr() as *mut *const i8,
                attrs_depr: th_td_depr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"thead\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 1,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"table header \x00" as *const u8 as *const i8,
                subelts: tr_elt.as_ptr() as *mut *const i8,
                defaultsubelt: b"tr\x00" as *const u8 as *const i8,
                attrs_opt: talign_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"title\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"document title \x00" as *const u8 as *const i8,
                subelts: html_pcdata.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: i18n_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"tr\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"table row \x00" as *const u8 as *const i8,
                subelts: tr_contents.as_ptr() as *mut *const i8,
                defaultsubelt: b"td\x00" as *const u8 as *const i8,
                attrs_opt: talign_attrs.as_ptr() as *mut *const i8,
                attrs_depr: bgcolor_attr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"tt\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"teletype or monospaced text style\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"u\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 3,
                saveEndTag: 0,
                empty: 0,
                depr: 1,
                dtd: 1,
                isinline: 1,
                desc: b"underlined text style\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: 0 as *mut *const i8,
                attrs_depr: html_attrs.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"ul\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 0,
                desc: b"unordered list \x00" as *const u8 as *const i8,
                subelts: li_elt.as_ptr() as *mut *const i8,
                defaultsubelt: b"li\x00" as *const u8 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: ul_depr.as_ptr() as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
        {
            let init = _htmlElemDesc {
                name: b"var\x00" as *const u8 as *const i8,
                startTag: 0,
                endTag: 0,
                saveEndTag: 0,
                empty: 0,
                depr: 0,
                dtd: 0,
                isinline: 1,
                desc: b"instance of a variable or program argument\x00" as *const u8 as *const i8,
                subelts: html_inline.as_ptr() as *mut *const i8,
                defaultsubelt: 0 as *const i8,
                attrs_opt: html_attrs.as_ptr() as *mut *const i8,
                attrs_depr: 0 as *mut *const i8,
                attrs_req: 0 as *mut *const i8,
            };
            init
        },
    ]
};
/*
 * start tags that imply the end of current element
 */
static mut htmlStartClose: [htmlStartCloseEntry; 251] = [
    {
        let init = htmlStartCloseEntry {
            oldTag: b"a\x00" as *const u8 as *const i8,
            newTag: b"a\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"a\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"a\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"a\x00" as *const u8 as *const i8,
            newTag: b"td\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"a\x00" as *const u8 as *const i8,
            newTag: b"th\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"address\x00" as *const u8 as *const i8,
            newTag: b"dd\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"address\x00" as *const u8 as *const i8,
            newTag: b"dl\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"address\x00" as *const u8 as *const i8,
            newTag: b"dt\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"address\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"address\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"address\x00" as *const u8 as *const i8,
            newTag: b"ul\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"b\x00" as *const u8 as *const i8,
            newTag: b"center\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"b\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"b\x00" as *const u8 as *const i8,
            newTag: b"td\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"b\x00" as *const u8 as *const i8,
            newTag: b"th\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"big\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"caption\x00" as *const u8 as *const i8,
            newTag: b"col\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"caption\x00" as *const u8 as *const i8,
            newTag: b"colgroup\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"caption\x00" as *const u8 as *const i8,
            newTag: b"tbody\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"caption\x00" as *const u8 as *const i8,
            newTag: b"tfoot\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"caption\x00" as *const u8 as *const i8,
            newTag: b"thead\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"caption\x00" as *const u8 as *const i8,
            newTag: b"tr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"col\x00" as *const u8 as *const i8,
            newTag: b"col\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"col\x00" as *const u8 as *const i8,
            newTag: b"colgroup\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"col\x00" as *const u8 as *const i8,
            newTag: b"tbody\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"col\x00" as *const u8 as *const i8,
            newTag: b"tfoot\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"col\x00" as *const u8 as *const i8,
            newTag: b"thead\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"col\x00" as *const u8 as *const i8,
            newTag: b"tr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"colgroup\x00" as *const u8 as *const i8,
            newTag: b"colgroup\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"colgroup\x00" as *const u8 as *const i8,
            newTag: b"tbody\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"colgroup\x00" as *const u8 as *const i8,
            newTag: b"tfoot\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"colgroup\x00" as *const u8 as *const i8,
            newTag: b"thead\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"colgroup\x00" as *const u8 as *const i8,
            newTag: b"tr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"dd\x00" as *const u8 as *const i8,
            newTag: b"dt\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"dir\x00" as *const u8 as *const i8,
            newTag: b"dd\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"dir\x00" as *const u8 as *const i8,
            newTag: b"dl\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"dir\x00" as *const u8 as *const i8,
            newTag: b"dt\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"dir\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"dir\x00" as *const u8 as *const i8,
            newTag: b"ul\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"dl\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"dl\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"dt\x00" as *const u8 as *const i8,
            newTag: b"dd\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"dt\x00" as *const u8 as *const i8,
            newTag: b"dl\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"font\x00" as *const u8 as *const i8,
            newTag: b"center\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"font\x00" as *const u8 as *const i8,
            newTag: b"td\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"font\x00" as *const u8 as *const i8,
            newTag: b"th\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"form\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h1\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h1\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h1\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h1\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h1\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h2\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h2\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h2\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h2\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h2\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h3\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h3\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h3\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h3\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h3\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h4\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h4\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h4\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h4\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h4\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h5\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h5\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h5\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h5\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h5\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h6\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h6\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h6\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h6\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"h6\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"a\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"abbr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"acronym\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"address\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"b\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"bdo\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"big\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"blockquote\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"body\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"br\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"center\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"cite\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"code\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"dd\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"dfn\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"dir\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"div\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"dl\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"dt\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"em\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"font\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"frameset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"h1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"h2\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"h3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"h4\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"h5\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"h6\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"hr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"i\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"iframe\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"img\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"kbd\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"listing\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"map\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"menu\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"ol\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"pre\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"q\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"s\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"samp\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"small\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"span\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"strike\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"strong\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"sub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"sup\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"tt\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"u\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"ul\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"var\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"head\x00" as *const u8 as *const i8,
            newTag: b"xmp\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"hr\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"i\x00" as *const u8 as *const i8,
            newTag: b"center\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"i\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"i\x00" as *const u8 as *const i8,
            newTag: b"td\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"i\x00" as *const u8 as *const i8,
            newTag: b"th\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"legend\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"li\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"link\x00" as *const u8 as *const i8,
            newTag: b"body\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"link\x00" as *const u8 as *const i8,
            newTag: b"frameset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"listing\x00" as *const u8 as *const i8,
            newTag: b"dd\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"listing\x00" as *const u8 as *const i8,
            newTag: b"dl\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"listing\x00" as *const u8 as *const i8,
            newTag: b"dt\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"listing\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"listing\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"listing\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"listing\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"listing\x00" as *const u8 as *const i8,
            newTag: b"ul\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"menu\x00" as *const u8 as *const i8,
            newTag: b"dd\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"menu\x00" as *const u8 as *const i8,
            newTag: b"dl\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"menu\x00" as *const u8 as *const i8,
            newTag: b"dt\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"menu\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"menu\x00" as *const u8 as *const i8,
            newTag: b"ul\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"ol\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"ol\x00" as *const u8 as *const i8,
            newTag: b"ul\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"option\x00" as *const u8 as *const i8,
            newTag: b"optgroup\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"option\x00" as *const u8 as *const i8,
            newTag: b"option\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"address\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"blockquote\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"body\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"caption\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"center\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"col\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"colgroup\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"dd\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"dir\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"div\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"dl\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"dt\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"frameset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"h1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"h2\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"h3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"h4\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"h5\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"h6\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"head\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"hr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"listing\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"menu\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"ol\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"pre\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"tbody\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"td\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"tfoot\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"th\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"title\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"tr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"ul\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"p\x00" as *const u8 as *const i8,
            newTag: b"xmp\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"pre\x00" as *const u8 as *const i8,
            newTag: b"dd\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"pre\x00" as *const u8 as *const i8,
            newTag: b"dl\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"pre\x00" as *const u8 as *const i8,
            newTag: b"dt\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"pre\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"pre\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"pre\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"pre\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"pre\x00" as *const u8 as *const i8,
            newTag: b"ul\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"s\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"script\x00" as *const u8 as *const i8,
            newTag: b"noscript\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"small\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"span\x00" as *const u8 as *const i8,
            newTag: b"td\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"span\x00" as *const u8 as *const i8,
            newTag: b"th\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"strike\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"style\x00" as *const u8 as *const i8,
            newTag: b"body\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"style\x00" as *const u8 as *const i8,
            newTag: b"frameset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"tbody\x00" as *const u8 as *const i8,
            newTag: b"tbody\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"tbody\x00" as *const u8 as *const i8,
            newTag: b"tfoot\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"td\x00" as *const u8 as *const i8,
            newTag: b"tbody\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"td\x00" as *const u8 as *const i8,
            newTag: b"td\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"td\x00" as *const u8 as *const i8,
            newTag: b"tfoot\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"td\x00" as *const u8 as *const i8,
            newTag: b"th\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"td\x00" as *const u8 as *const i8,
            newTag: b"tr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"tfoot\x00" as *const u8 as *const i8,
            newTag: b"tbody\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"th\x00" as *const u8 as *const i8,
            newTag: b"tbody\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"th\x00" as *const u8 as *const i8,
            newTag: b"td\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"th\x00" as *const u8 as *const i8,
            newTag: b"tfoot\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"th\x00" as *const u8 as *const i8,
            newTag: b"th\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"th\x00" as *const u8 as *const i8,
            newTag: b"tr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"thead\x00" as *const u8 as *const i8,
            newTag: b"tbody\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"thead\x00" as *const u8 as *const i8,
            newTag: b"tfoot\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"title\x00" as *const u8 as *const i8,
            newTag: b"body\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"title\x00" as *const u8 as *const i8,
            newTag: b"frameset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"tr\x00" as *const u8 as *const i8,
            newTag: b"tbody\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"tr\x00" as *const u8 as *const i8,
            newTag: b"tfoot\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"tr\x00" as *const u8 as *const i8,
            newTag: b"tr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"tt\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"u\x00" as *const u8 as *const i8,
            newTag: b"p\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"u\x00" as *const u8 as *const i8,
            newTag: b"td\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"u\x00" as *const u8 as *const i8,
            newTag: b"th\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"ul\x00" as *const u8 as *const i8,
            newTag: b"address\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"ul\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"ul\x00" as *const u8 as *const i8,
            newTag: b"menu\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"ul\x00" as *const u8 as *const i8,
            newTag: b"ol\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"ul\x00" as *const u8 as *const i8,
            newTag: b"pre\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"xmp\x00" as *const u8 as *const i8,
            newTag: b"dd\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"xmp\x00" as *const u8 as *const i8,
            newTag: b"dl\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"xmp\x00" as *const u8 as *const i8,
            newTag: b"dt\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"xmp\x00" as *const u8 as *const i8,
            newTag: b"fieldset\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"xmp\x00" as *const u8 as *const i8,
            newTag: b"form\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"xmp\x00" as *const u8 as *const i8,
            newTag: b"li\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"xmp\x00" as *const u8 as *const i8,
            newTag: b"table\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = htmlStartCloseEntry {
            oldTag: b"xmp\x00" as *const u8 as *const i8,
            newTag: b"ul\x00" as *const u8 as *const i8,
        };
        init
    },
];
/*
 * The list of HTML elements which are supposed not to have
 * CDATA content and where a p element will be implied
 *
 * TODO: extend that list by reading the HTML SGML DTD on
 *       implied paragraph
 */
static mut htmlNoContentElements: [*const i8; 3] = [
    b"html\x00" as *const u8 as *const i8,
    b"head\x00" as *const u8 as *const i8,
    0 as *const i8,
];
/*
 * The list of HTML attributes which are of content %Script;
 * NOTE: when adding ones, check htmlIsScriptAttribute() since
 *       it assumes the name starts with 'on'
 */
static mut htmlScriptAttributes: [*const i8; 18] = [
    b"onclick\x00" as *const u8 as *const i8,
    b"ondblclick\x00" as *const u8 as *const i8,
    b"onmousedown\x00" as *const u8 as *const i8,
    b"onmouseup\x00" as *const u8 as *const i8,
    b"onmouseover\x00" as *const u8 as *const i8,
    b"onmousemove\x00" as *const u8 as *const i8,
    b"onmouseout\x00" as *const u8 as *const i8,
    b"onkeypress\x00" as *const u8 as *const i8,
    b"onkeydown\x00" as *const u8 as *const i8,
    b"onkeyup\x00" as *const u8 as *const i8,
    b"onload\x00" as *const u8 as *const i8,
    b"onunload\x00" as *const u8 as *const i8,
    b"onfocus\x00" as *const u8 as *const i8,
    b"onblur\x00" as *const u8 as *const i8,
    b"onsubmit\x00" as *const u8 as *const i8,
    b"onreset\x00" as *const u8 as *const i8,
    b"onchange\x00" as *const u8 as *const i8,
    b"onselect\x00" as *const u8 as *const i8,
];
static mut htmlEndPriority: [elementPriority; 12] = [
    {
        let init = elementPriority {
            name: b"div\x00" as *const u8 as *const i8,
            priority: 150,
        };
        init
    },
    {
        let init = elementPriority {
            name: b"td\x00" as *const u8 as *const i8,
            priority: 160,
        };
        init
    },
    {
        let init = elementPriority {
            name: b"th\x00" as *const u8 as *const i8,
            priority: 160,
        };
        init
    },
    {
        let init = elementPriority {
            name: b"tr\x00" as *const u8 as *const i8,
            priority: 170,
        };
        init
    },
    {
        let init = elementPriority {
            name: b"thead\x00" as *const u8 as *const i8,
            priority: 180,
        };
        init
    },
    {
        let init = elementPriority {
            name: b"tbody\x00" as *const u8 as *const i8,
            priority: 180,
        };
        init
    },
    {
        let init = elementPriority {
            name: b"tfoot\x00" as *const u8 as *const i8,
            priority: 180,
        };
        init
    },
    {
        let init = elementPriority {
            name: b"table\x00" as *const u8 as *const i8,
            priority: 190,
        };
        init
    },
    {
        let init = elementPriority {
            name: b"head\x00" as *const u8 as *const i8,
            priority: 200,
        };
        init
    },
    {
        let init = elementPriority {
            name: b"body\x00" as *const u8 as *const i8,
            priority: 200,
        };
        init
    },
    {
        let init = elementPriority {
            name: b"html\x00" as *const u8 as *const i8,
            priority: 220,
        };
        init
    },
    {
        let init = elementPriority {
            name: 0 as *const i8,
            priority: 100,
        };
        init
    },
];
/* ***********************************************************************
 *									*
 *	functions to handle HTML specific data			*
 *									*
 ************************************************************************/
/* *
 * htmlInitAutoClose:
 *
 * This is a no-op now.
 */

pub fn htmlInitAutoClose_htmlparser() {}

extern "C" fn htmlCompareTags(key: *const (), member: *const ()) -> i32 {
    let tag: *const xmlChar = key as *const xmlChar;
    let desc: *const htmlElemDesc = member as *const htmlElemDesc;
    let descPtr = unsafe { &*desc };
    return unsafe { xmlStrcasecmp_safe(tag, descPtr.name as *mut xmlChar) };
}
/* *
 * htmlTagLookup:
 * @tag:  The tag name in lowercase
 *
 * Lookup the HTML tag in the ElementTable
 *
 * Returns the related htmlElemDescPtr or NULL if not found.
 */

pub fn htmlTagLookup(tag: *const xmlChar) -> *const htmlElemDesc {
    if tag.is_null() {
        return 0 as *const htmlElemDesc;
    }
    return bsearch(
        tag as *const (),
        unsafe { html40ElementTable.as_ptr() as *const () },
        ((::std::mem::size_of::<[htmlElemDesc; 92]>() as u64)
            / (::std::mem::size_of::<htmlElemDesc>()) as u64),
        ::std::mem::size_of::<htmlElemDesc>() as u64,
        Some(htmlCompareTags as unsafe extern "C" fn(_: *const (), _: *const ()) -> i32),
    ) as *const htmlElemDesc;
}
/* *
 * htmlGetEndPriority:
 * @name: The name of the element to look up the priority for.
 *
 * Return value: The "endtag" priority.
 **/
fn htmlGetEndPriority(name: *const xmlChar) -> i32 {
    let mut i: i32 = 0;
    while !getHtmlEndPriority(i as usize).name.is_null()
        && unsafe { xmlStrEqual_safe(getHtmlEndPriority(i as usize).name as *const xmlChar, name) }
            == 0
    {
        i += 1
    }
    return unsafe { getHtmlEndPriority(i as usize).priority };
}

extern "C" fn htmlCompareStartClose(vkey: *const (), member: *const ()) -> i32 {
    let key: *const htmlStartCloseEntry = vkey as *const htmlStartCloseEntry;
    let entry: *const htmlStartCloseEntry = member as *const htmlStartCloseEntry;
    let mut ret: i32 = 0;
    let keyPtr = unsafe { &*key };
    let entryPtr = unsafe { &*entry };
    ret = unsafe { strcmp_safe(keyPtr.oldTag, entryPtr.oldTag) };
    if ret == 0 {
        ret = unsafe { strcmp_safe(keyPtr.newTag, entryPtr.newTag) }
    }
    return ret;
}
/* *
 * htmlCheckAutoClose:
 * @newtag:  The new tag name
 * @oldtag:  The old tag name
 *
 * Checks whether the new tag is one of the registered valid tags for
 * closing old.
 *
 * Returns 0 if no, 1 if yes.
 */
fn htmlCheckAutoClose(newtag: *const xmlChar, oldtag: *const xmlChar) -> i32 {
    let mut key: htmlStartCloseEntry = htmlStartCloseEntry {
        oldTag: oldtag as *const i8,
        newTag: newtag as *const i8,
    };
    let mut res: *mut () = 0 as *mut ();
    unsafe {
        res = bsearch(
            &mut key as *mut htmlStartCloseEntry as *const (),
            htmlStartClose.as_ptr() as *const (),
            ((::std::mem::size_of::<[htmlStartCloseEntry; 251]>() as u64)
                / (::std::mem::size_of::<htmlStartCloseEntry>()) as u64),
            ::std::mem::size_of::<htmlStartCloseEntry>() as u64,
            Some(htmlCompareStartClose as unsafe extern "C" fn(_: *const (), _: *const ()) -> i32),
        );
    }
    return (res != 0 as *mut ()) as i32;
}
/* *
 * htmlAutoCloseOnClose:
 * @ctxt:  an HTML parser context
 * @newtag:  The new tag name
 * @force:  force the tag closure
 *
 * The HTML DTD allows an ending tag to implicitly close other tags.
 */
fn htmlAutoCloseOnClose(ctxt: htmlParserCtxtPtr, newtag: *const xmlChar) {
    let ctxtPtr = unsafe { &mut *ctxt };
    let mut info: *const htmlElemDesc = 0 as *const htmlElemDesc;
    let mut i: i32 = 0;
    let mut priority: i32 = 0;
    priority = htmlGetEndPriority(newtag);
    i = ctxtPtr.nameNr - 1;
    while i >= 0 {
        unsafe {
            if xmlStrEqual_safe(newtag, *ctxtPtr.nameTab.offset(i as isize)) != 0 {
                break;
            }
        }
        /*
         * A misplaced endtag can only close elements with lower
         * or equal priority, so if we find an element with higher
         * priority before we find an element with
         * matching name, we just ignore this endtag
         */
        unsafe {
            if htmlGetEndPriority(*ctxtPtr.nameTab.offset(i as isize)) > priority {
                return;
            }
        }
        i -= 1
    }
    if i < 0 {
        return;
    }

    while unsafe { xmlStrEqual_safe(newtag, ctxtPtr.name) } == 0 {
        info = htmlTagLookup(ctxtPtr.name);
        let info_condition = unsafe { !info.is_null() && (*info).endTag == 3 };
        if info_condition {
            htmlParseErr(
                ctxt,
                XML_ERR_TAG_NAME_MISMATCH,
                b"Opening and ending tag mismatch: %s and %s\n\x00" as *const u8 as *const i8,
                newtag,
                ctxtPtr.name,
            );
        }
        let sax_condition =
            unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() };
        if sax_condition {
            let saxPtr = unsafe { &mut *(*ctxt).sax };
            unsafe {
                xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, ctxtPtr.name)
            };
        }
        htmlnamePop(ctxt);
    }
}
/* *
 * htmlAutoCloseOnEnd:
 * @ctxt:  an HTML parser context
 *
 * Close all remaining tags at the end of the stream
 */
fn htmlAutoCloseOnEnd(ctxt: htmlParserCtxtPtr) {
    let ctxtPtr = unsafe { &mut *ctxt };
    let mut i: i32 = 0;
    if ctxtPtr.nameNr == 0 {
        return;
    }
    i = ctxtPtr.nameNr - 1;
    let mut sax_condition = false;
    while i >= 0 {
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() };
        if sax_condition {
            let saxPtr = unsafe { &mut *(*ctxt).sax };
            unsafe {
                xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, ctxtPtr.name)
            };
        }
        htmlnamePop(ctxt);
        i -= 1
    }
}
/* *
 * htmlAutoClose:
 * @ctxt:  an HTML parser context
 * @newtag:  The new tag name or NULL
 *
 * The HTML DTD allows a tag to implicitly close other tags.
 * The list is kept in htmlStartClose array. This function is
 * called when a new tag has been detected and generates the
 * appropriates closes if possible/needed.
 * If newtag is NULL this mean we are at the end of the resource
 * and we should check
 */
fn htmlAutoClose(ctxt: htmlParserCtxtPtr, newtag: *const xmlChar) {
    let ctxtPtr = unsafe { &mut *ctxt };
    let mut sax_condition = false;
    while !newtag.is_null()
        && !ctxtPtr.name.is_null()
        && htmlCheckAutoClose(newtag, ctxtPtr.name) != 0
    {
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() };
        if sax_condition {
            let saxPtr = unsafe { &mut *(*ctxt).sax };
            unsafe {
                xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, ctxtPtr.name)
            };
        }
        htmlnamePop(ctxt);
    }
    if newtag.is_null() {
        htmlAutoCloseOnEnd(ctxt);
        return;
    }
    while newtag.is_null()
        && !ctxtPtr.name.is_null()
        && (unsafe {
            xmlStrEqual_safe(
                ctxtPtr.name,
                b"head\x00" as *const u8 as *const i8 as *mut xmlChar,
            )
        } != 0
            || unsafe {
                xmlStrEqual_safe(
                    ctxtPtr.name,
                    b"body\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } != 0
            || unsafe {
                xmlStrEqual_safe(
                    ctxtPtr.name,
                    b"html\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } != 0)
    {
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() };
        if sax_condition {
            let saxPtr = unsafe { &mut *(*ctxt).sax };
            unsafe {
                xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, ctxtPtr.name)
            };
        }
        htmlnamePop(ctxt);
    }
}
/* *
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

pub fn htmlAutoCloseTag(doc: htmlDocPtr, name: *const xmlChar, elem: htmlNodePtr) -> i32 {
    let mut child: htmlNodePtr = 0 as *mut xmlNode;
    if elem.is_null() {
        return 1;
    }
    let elemPtr = unsafe { &mut *elem };
    if unsafe { xmlStrEqual_safe(name, elemPtr.name) } != 0 {
        return 0;
    }
    if htmlCheckAutoClose(elemPtr.name, name) != 0 {
        return 1;
    }
    child = elemPtr.children;
    while !child.is_null() {
        if htmlAutoCloseTag(doc, name, child) != 0 {
            return 1;
        }
        unsafe { child = (*child).next }
    }
    return 0;
}
/* *
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

pub fn htmlIsAutoClosed(doc: htmlDocPtr, elem: htmlNodePtr) -> i32 {
    let mut child: htmlNodePtr = 0 as *mut xmlNode;
    if elem.is_null() {
        return 1;
    }
    let mut elemPtr = unsafe { &mut *elem };
    child = elemPtr.children;
    while !child.is_null() {
        if htmlAutoCloseTag(doc, elemPtr.name, child) != 0 {
            return 1;
        }
        unsafe { child = (*child).next }
    }
    return 0;
}
/* *
 * htmlCheckImplied:
 * @ctxt:  an HTML parser context
 * @newtag:  The new tag name
 *
 * The HTML DTD allows a tag to exists only implicitly
 * called when a new tag has been detected and generates the
 * appropriates implicit tags if missing
 */
fn htmlCheckImplied(ctxt: htmlParserCtxtPtr, newtag: *const xmlChar) {
    let ctxtPtr = unsafe { &mut *ctxt };
    let mut i: i32 = 0;
    if ctxtPtr.options & HTML_PARSE_NOIMPLIED as i32 != 0 {
        return;
    }
    if unsafe { getHtmlOmittedDefaultValue() } == 0 {
        return;
    }
    if unsafe {
        xmlStrEqual_safe(
            newtag,
            b"html\x00" as *const u8 as *const i8 as *mut xmlChar,
        )
    } != 0
    {
        return;
    }
    let mut sax_condition = false;
    if ctxtPtr.nameNr <= 0 {
        htmlnamePush(ctxt, b"html\x00" as *const u8 as *const i8 as *mut xmlChar);
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
        if sax_condition {
            let saxPtr = unsafe { &mut *(*ctxt).sax };
            unsafe {
                xmlSAXHandler_startElement_safe(
                    saxPtr.startElement,
                    ctxtPtr.userData,
                    b"html\x00" as *const u8 as *const i8 as *mut xmlChar,
                    0 as *mut *const xmlChar,
                )
            };
        }
    }
    if unsafe {
        xmlStrEqual_safe(
            newtag,
            b"body\x00" as *const u8 as *const i8 as *mut xmlChar,
        )
    } != 0
        || unsafe {
            xmlStrEqual_safe(
                newtag,
                b"head\x00" as *const u8 as *const i8 as *mut xmlChar,
            )
        } != 0
    {
        return;
    }
    if ctxtPtr.nameNr <= 1
        && (unsafe {
            xmlStrEqual_safe(
                newtag,
                b"script\x00" as *const u8 as *const i8 as *mut xmlChar,
            )
        } != 0
            || unsafe {
                xmlStrEqual_safe(
                    newtag,
                    b"style\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } != 0
            || unsafe {
                xmlStrEqual_safe(
                    newtag,
                    b"meta\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } != 0
            || unsafe {
                xmlStrEqual_safe(
                    newtag,
                    b"link\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } != 0
            || unsafe {
                xmlStrEqual_safe(
                    newtag,
                    b"title\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } != 0
            || unsafe {
                xmlStrEqual_safe(
                    newtag,
                    b"base\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } != 0)
    {
        if ctxtPtr.html >= 3 {
            /* we already saw or generated an <head> before */
            return;
        }
        /*
         * dropped OBJECT ... i you put it first BODY will be
         * assumed !
         */
        htmlnamePush(ctxt, b"head\x00" as *const u8 as *const i8 as *mut xmlChar);
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
        if sax_condition {
            let saxPtr = unsafe { &mut *(*ctxt).sax };
            unsafe {
                xmlSAXHandler_startElement_safe(
                    saxPtr.startElement,
                    ctxtPtr.userData,
                    b"head\x00" as *const u8 as *const i8 as *mut xmlChar,
                    0 as *mut *const xmlChar,
                )
            };
        }
    } else if unsafe {
        xmlStrEqual_safe(
            newtag,
            b"noframes\x00" as *const u8 as *const i8 as *mut xmlChar,
        )
    } == 0
        && unsafe {
            xmlStrEqual_safe(
                newtag,
                b"frame\x00" as *const u8 as *const i8 as *mut xmlChar,
            )
        } == 0
        && unsafe {
            xmlStrEqual_safe(
                newtag,
                b"frameset\x00" as *const u8 as *const i8 as *mut xmlChar,
            )
        } == 0
    {
        if ctxtPtr.html >= 10 {
            /* we already saw or generated a <body> before */
            return;
        }
        i = 0;
        while i < ctxtPtr.nameNr {
            unsafe {
                if xmlStrEqual_safe(
                    *ctxtPtr.nameTab.offset(i as isize),
                    b"body\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                    return;
                }
            }
            unsafe {
                if xmlStrEqual_safe(
                    *ctxtPtr.nameTab.offset(i as isize),
                    b"head\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                    return;
                }
            }
            i += 1
        }
        htmlnamePush(ctxt, b"body\x00" as *const u8 as *const i8 as *mut xmlChar);
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
        if sax_condition {
            let saxPtr = unsafe { &mut *(*ctxt).sax };
            unsafe {
                xmlSAXHandler_startElement_safe(
                    saxPtr.startElement,
                    ctxtPtr.userData,
                    b"body\x00" as *const u8 as *const i8 as *mut xmlChar,
                    0 as *mut *const xmlChar,
                )
            };
        }
    };
}
/* *
 * htmlCheckParagraph
 * @ctxt:  an HTML parser context
 *
 * Check whether a p element need to be implied before inserting
 * characters in the current element.
 *
 * Returns 1 if a paragraph has been inserted, 0 if not and -1
 *         in case of error.
 */
fn htmlCheckParagraph(ctxt: htmlParserCtxtPtr) -> i32 {
    let mut tag: *const xmlChar = 0 as *const xmlChar;
    let mut i: i32 = 0;
    if ctxt.is_null() {
        return -(1 as i32);
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    tag = ctxtPtr.name;
    let mut sax_condition = false;
    if tag.is_null() {
        htmlAutoClose(ctxt, b"p\x00" as *const u8 as *const i8 as *mut xmlChar);
        htmlCheckImplied(ctxt, b"p\x00" as *const u8 as *const i8 as *mut xmlChar);
        htmlnamePush(ctxt, b"p\x00" as *const u8 as *const i8 as *mut xmlChar);
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
        if sax_condition {
            let saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_startElement_safe(
                saxPtr.startElement,
                ctxtPtr.userData,
                b"p\x00" as *const u8 as *const i8 as *mut xmlChar,
                0 as *mut *const xmlChar,
            );
        }
        return 1;
    }
    if unsafe { getHtmlOmittedDefaultValue() == 0 } {
        return 0;
    }
    i = 0;
    while !unsafe { getHtmlNoContentElements(i as usize).is_null() } {
        if unsafe {
            xmlStrEqual_safe(tag, getHtmlNoContentElements(i as usize) as *mut xmlChar) != 0
        } {
            htmlAutoClose(ctxt, b"p\x00" as *const u8 as *const i8 as *mut xmlChar);
            htmlCheckImplied(ctxt, b"p\x00" as *const u8 as *const i8 as *mut xmlChar);
            htmlnamePush(ctxt, b"p\x00" as *const u8 as *const i8 as *mut xmlChar);
            sax_condition =
                unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
            if sax_condition {
                let saxPtr = unsafe { &mut *(*ctxt).sax };
                unsafe {
                    xmlSAXHandler_startElement_safe(
                        saxPtr.startElement,
                        ctxtPtr.userData,
                        b"p\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *mut *const xmlChar,
                    )
                };
            }
            return 1;
        }
        i += 1
    }
    return 0;
}
/* *
 * htmlIsScriptAttribute:
 * @name:  an attribute name
 *
 * Check if an attribute is of content type Script
 *
 * Returns 1 is the attribute is a script 0 otherwise
 */

pub fn htmlIsScriptAttribute(name: *const xmlChar) -> i32 {
    let mut i: u32 = 0;
    if name.is_null() {
        return 0;
    }
    /*
     * all script attributes start with 'on'
     */
    unsafe {
        if *name.offset(0 as isize) as i32 != 'o' as i32
            || *name.offset(1 as isize) as i32 != 'n' as i32
        {
            return 0;
        }
    }
    i = 0;
    while (i as u64)
        < ((::std::mem::size_of::<[*const i8; 18]>() as u64)
            / (::std::mem::size_of::<*const i8>()) as u64)
    {
        if unsafe {
            xmlStrEqual_safe(name, getHtmlScriptAttributes(i as usize) as *const xmlChar) != 0
        } {
            return 1;
        }
        i = i + 1
    }
    return 0;
}
/* ***********************************************************************
 *									*
 *	The list of HTML predefined entities			*
 *									*
 ************************************************************************/
static mut html40EntitiesTable: [htmlEntityDesc; 253] = [
    {
        let init = _htmlEntityDesc {
            value: 34,
            name: b"quot\x00" as *const u8 as *const i8,
            desc: b"quotation mark = APL quote, U+0022 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 38,
            name: b"amp\x00" as *const u8 as *const i8,
            desc: b"ampersand, U+0026 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 39,
            name: b"apos\x00" as *const u8 as *const i8,
            desc: b"single quote\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 60,
            name: b"lt\x00" as *const u8 as *const i8,
            desc: b"less-than sign, U+003C ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 62,
            name: b"gt\x00" as *const u8 as *const i8,
            desc: b"greater-than sign, U+003E ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 160,
            name: b"nbsp\x00" as *const u8 as *const i8,
            desc: b"no-break space = non-breaking space, U+00A0 ISOnum\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 161,
            name: b"iexcl\x00" as *const u8 as *const i8,
            desc: b"inverted exclamation mark, U+00A1 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 162,
            name: b"cent\x00" as *const u8 as *const i8,
            desc: b"cent sign, U+00A2 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 163,
            name: b"pound\x00" as *const u8 as *const i8,
            desc: b"pound sign, U+00A3 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 164,
            name: b"curren\x00" as *const u8 as *const i8,
            desc: b"currency sign, U+00A4 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 165,
            name: b"yen\x00" as *const u8 as *const i8,
            desc: b"yen sign = yuan sign, U+00A5 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 166,
            name: b"brvbar\x00" as *const u8 as *const i8,
            desc: b"broken bar = broken vertical bar, U+00A6 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 167,
            name: b"sect\x00" as *const u8 as *const i8,
            desc: b"section sign, U+00A7 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 168,
            name: b"uml\x00" as *const u8 as *const i8,
            desc: b"diaeresis = spacing diaeresis, U+00A8 ISOdia\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 169,
            name: b"copy\x00" as *const u8 as *const i8,
            desc: b"copyright sign, U+00A9 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 170,
            name: b"ordf\x00" as *const u8 as *const i8,
            desc: b"feminine ordinal indicator, U+00AA ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 171,
            name:
            b"laquo\x00" as *const u8 as *const i8,
            desc:
            b"left-pointing double angle quotation mark = left pointing guillemet, U+00AB ISOnum\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 172,
            name: b"not\x00" as *const u8 as *const i8,
            desc: b"not sign, U+00AC ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 173,
            name: b"shy\x00" as *const u8 as *const i8,
            desc: b"soft hyphen = discretionary hyphen, U+00AD ISOnum\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 174,
            name: b"reg\x00" as *const u8 as *const i8,
            desc: b"registered sign = registered trade mark sign, U+00AE ISOnum\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 175,
            name: b"macr\x00" as *const u8 as *const i8,
            desc: b"macron = spacing macron = overline = APL overbar, U+00AF ISOdia\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 176,
            name: b"deg\x00" as *const u8 as *const i8,
            desc: b"degree sign, U+00B0 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 177,
            name: b"plusmn\x00" as *const u8 as *const i8,
            desc: b"plus-minus sign = plus-or-minus sign, U+00B1 ISOnum\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 178,
            name: b"sup2\x00" as *const u8 as *const i8,
            desc: b"superscript two = superscript digit two = squared, U+00B2 ISOnum\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 179,
            name: b"sup3\x00" as *const u8 as *const i8,
            desc: b"superscript three = superscript digit three = cubed, U+00B3 ISOnum\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 180,
            name: b"acute\x00" as *const u8 as *const i8,
            desc: b"acute accent = spacing acute, U+00B4 ISOdia\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 181,
            name: b"micro\x00" as *const u8 as *const i8,
            desc: b"micro sign, U+00B5 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 182,
            name: b"para\x00" as *const u8 as *const i8,
            desc: b"pilcrow sign = paragraph sign, U+00B6 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 183,
            name: b"middot\x00" as *const u8 as *const i8,
            desc: b"middle dot = Georgian comma Greek middle dot, U+00B7 ISOnum\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 184,
            name: b"cedil\x00" as *const u8 as *const i8,
            desc: b"cedilla = spacing cedilla, U+00B8 ISOdia\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 185,
            name: b"sup1\x00" as *const u8 as *const i8,
            desc: b"superscript one = superscript digit one, U+00B9 ISOnum\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 186,
            name: b"ordm\x00" as *const u8 as *const i8,
            desc: b"masculine ordinal indicator, U+00BA ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 187,
            name:
            b"raquo\x00" as *const u8 as *const i8,
            desc:
            b"right-pointing double angle quotation mark right pointing guillemet, U+00BB ISOnum\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 188,
            name: b"frac14\x00" as *const u8 as *const i8,
            desc: b"vulgar fraction one quarter = fraction one quarter, U+00BC ISOnum\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 189,
            name: b"frac12\x00" as *const u8 as *const i8,
            desc: b"vulgar fraction one half = fraction one half, U+00BD ISOnum\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 190,
            name: b"frac34\x00" as *const u8 as *const i8,
            desc: b"vulgar fraction three quarters = fraction three quarters, U+00BE ISOnum\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 191,
            name: b"iquest\x00" as *const u8 as *const i8,
            desc: b"inverted question mark = turned question mark, U+00BF ISOnum\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 192,
            name:
            b"Agrave\x00" as *const u8 as *const i8,
            desc:
            b"latin capital letter A with grave = latin capital letter A grave, U+00C0 ISOlat1\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 193,
            name: b"Aacute\x00" as *const u8 as *const i8,
            desc: b"latin capital letter A with acute, U+00C1 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 194,
            name: b"Acirc\x00" as *const u8 as *const i8,
            desc: b"latin capital letter A with circumflex, U+00C2 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 195,
            name: b"Atilde\x00" as *const u8 as *const i8,
            desc: b"latin capital letter A with tilde, U+00C3 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 196,
            name: b"Auml\x00" as *const u8 as *const i8,
            desc: b"latin capital letter A with diaeresis, U+00C4 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 197,
            name:
            b"Aring\x00" as *const u8 as *const i8,
            desc:
            b"latin capital letter A with ring above = latin capital letter A ring, U+00C5 ISOlat1\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 198,
            name: b"AElig\x00" as *const u8 as *const i8,
            desc: b"latin capital letter AE = latin capital ligature AE, U+00C6 ISOlat1\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 199,
            name: b"Ccedil\x00" as *const u8 as *const i8,
            desc: b"latin capital letter C with cedilla, U+00C7 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 200,
            name: b"Egrave\x00" as *const u8 as *const i8,
            desc: b"latin capital letter E with grave, U+00C8 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 201,
            name: b"Eacute\x00" as *const u8 as *const i8,
            desc: b"latin capital letter E with acute, U+00C9 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 202,
            name: b"Ecirc\x00" as *const u8 as *const i8,
            desc: b"latin capital letter E with circumflex, U+00CA ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 203,
            name: b"Euml\x00" as *const u8 as *const i8,
            desc: b"latin capital letter E with diaeresis, U+00CB ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 204,
            name: b"Igrave\x00" as *const u8 as *const i8,
            desc: b"latin capital letter I with grave, U+00CC ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 205,
            name: b"Iacute\x00" as *const u8 as *const i8,
            desc: b"latin capital letter I with acute, U+00CD ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 206,
            name: b"Icirc\x00" as *const u8 as *const i8,
            desc: b"latin capital letter I with circumflex, U+00CE ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 207,
            name: b"Iuml\x00" as *const u8 as *const i8,
            desc: b"latin capital letter I with diaeresis, U+00CF ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 208,
            name: b"ETH\x00" as *const u8 as *const i8,
            desc: b"latin capital letter ETH, U+00D0 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 209,
            name: b"Ntilde\x00" as *const u8 as *const i8,
            desc: b"latin capital letter N with tilde, U+00D1 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 210,
            name: b"Ograve\x00" as *const u8 as *const i8,
            desc: b"latin capital letter O with grave, U+00D2 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 211,
            name: b"Oacute\x00" as *const u8 as *const i8,
            desc: b"latin capital letter O with acute, U+00D3 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 212,
            name: b"Ocirc\x00" as *const u8 as *const i8,
            desc: b"latin capital letter O with circumflex, U+00D4 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 213,
            name: b"Otilde\x00" as *const u8 as *const i8,
            desc: b"latin capital letter O with tilde, U+00D5 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 214,
            name: b"Ouml\x00" as *const u8 as *const i8,
            desc: b"latin capital letter O with diaeresis, U+00D6 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 215,
            name: b"times\x00" as *const u8 as *const i8,
            desc: b"multiplication sign, U+00D7 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 216,
            name:
            b"Oslash\x00" as *const u8 as *const i8,
            desc:
            b"latin capital letter O with stroke latin capital letter O slash, U+00D8 ISOlat1\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 217,
            name: b"Ugrave\x00" as *const u8 as *const i8,
            desc: b"latin capital letter U with grave, U+00D9 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 218,
            name: b"Uacute\x00" as *const u8 as *const i8,
            desc: b"latin capital letter U with acute, U+00DA ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 219,
            name: b"Ucirc\x00" as *const u8 as *const i8,
            desc: b"latin capital letter U with circumflex, U+00DB ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 220,
            name: b"Uuml\x00" as *const u8 as *const i8,
            desc: b"latin capital letter U with diaeresis, U+00DC ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 221,
            name: b"Yacute\x00" as *const u8 as *const i8,
            desc: b"latin capital letter Y with acute, U+00DD ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 222,
            name: b"THORN\x00" as *const u8 as *const i8,
            desc: b"latin capital letter THORN, U+00DE ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 223,
            name: b"szlig\x00" as *const u8 as *const i8,
            desc: b"latin small letter sharp s = ess-zed, U+00DF ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 224,
            name: b"agrave\x00" as *const u8 as *const i8,
            desc:
                b"latin small letter a with grave = latin small letter a grave, U+00E0 ISOlat1\x00"
                    as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 225,
            name: b"aacute\x00" as *const u8 as *const i8,
            desc: b"latin small letter a with acute, U+00E1 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 226,
            name: b"acirc\x00" as *const u8 as *const i8,
            desc: b"latin small letter a with circumflex, U+00E2 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 227,
            name: b"atilde\x00" as *const u8 as *const i8,
            desc: b"latin small letter a with tilde, U+00E3 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 228,
            name: b"auml\x00" as *const u8 as *const i8,
            desc: b"latin small letter a with diaeresis, U+00E4 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 229,
            name:
            b"aring\x00" as *const u8 as *const i8,
            desc:
            b"latin small letter a with ring above = latin small letter a ring, U+00E5 ISOlat1\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 230,
            name: b"aelig\x00" as *const u8 as *const i8,
            desc: b"latin small letter ae = latin small ligature ae, U+00E6 ISOlat1\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 231,
            name: b"ccedil\x00" as *const u8 as *const i8,
            desc: b"latin small letter c with cedilla, U+00E7 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 232,
            name: b"egrave\x00" as *const u8 as *const i8,
            desc: b"latin small letter e with grave, U+00E8 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 233,
            name: b"eacute\x00" as *const u8 as *const i8,
            desc: b"latin small letter e with acute, U+00E9 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 234,
            name: b"ecirc\x00" as *const u8 as *const i8,
            desc: b"latin small letter e with circumflex, U+00EA ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 235,
            name: b"euml\x00" as *const u8 as *const i8,
            desc: b"latin small letter e with diaeresis, U+00EB ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 236,
            name: b"igrave\x00" as *const u8 as *const i8,
            desc: b"latin small letter i with grave, U+00EC ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 237,
            name: b"iacute\x00" as *const u8 as *const i8,
            desc: b"latin small letter i with acute, U+00ED ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 238,
            name: b"icirc\x00" as *const u8 as *const i8,
            desc: b"latin small letter i with circumflex, U+00EE ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 239,
            name: b"iuml\x00" as *const u8 as *const i8,
            desc: b"latin small letter i with diaeresis, U+00EF ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 240,
            name: b"eth\x00" as *const u8 as *const i8,
            desc: b"latin small letter eth, U+00F0 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 241,
            name: b"ntilde\x00" as *const u8 as *const i8,
            desc: b"latin small letter n with tilde, U+00F1 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 242,
            name: b"ograve\x00" as *const u8 as *const i8,
            desc: b"latin small letter o with grave, U+00F2 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 243,
            name: b"oacute\x00" as *const u8 as *const i8,
            desc: b"latin small letter o with acute, U+00F3 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 244,
            name: b"ocirc\x00" as *const u8 as *const i8,
            desc: b"latin small letter o with circumflex, U+00F4 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 245,
            name: b"otilde\x00" as *const u8 as *const i8,
            desc: b"latin small letter o with tilde, U+00F5 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 246,
            name: b"ouml\x00" as *const u8 as *const i8,
            desc: b"latin small letter o with diaeresis, U+00F6 ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 247,
            name: b"divide\x00" as *const u8 as *const i8,
            desc: b"division sign, U+00F7 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 248,
            name:
            b"oslash\x00" as *const u8 as *const i8,
            desc:
            b"latin small letter o with stroke, = latin small letter o slash, U+00F8 ISOlat1\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 249,
            name: b"ugrave\x00" as *const u8 as *const i8,
            desc: b"latin small letter u with grave, U+00F9 ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 250,
            name: b"uacute\x00" as *const u8 as *const i8,
            desc: b"latin small letter u with acute, U+00FA ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 251,
            name: b"ucirc\x00" as *const u8 as *const i8,
            desc: b"latin small letter u with circumflex, U+00FB ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 252,
            name: b"uuml\x00" as *const u8 as *const i8,
            desc: b"latin small letter u with diaeresis, U+00FC ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 253,
            name: b"yacute\x00" as *const u8 as *const i8,
            desc: b"latin small letter y with acute, U+00FD ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 254,
            name: b"thorn\x00" as *const u8 as *const i8,
            desc: b"latin small letter thorn with, U+00FE ISOlat1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 255,
            name: b"yuml\x00" as *const u8 as *const i8,
            desc: b"latin small letter y with diaeresis, U+00FF ISOlat1\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 338,
            name: b"OElig\x00" as *const u8 as *const i8,
            desc: b"latin capital ligature OE, U+0152 ISOlat2\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 339,
            name: b"oelig\x00" as *const u8 as *const i8,
            desc: b"latin small ligature oe, U+0153 ISOlat2\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 352,
            name: b"Scaron\x00" as *const u8 as *const i8,
            desc: b"latin capital letter S with caron, U+0160 ISOlat2\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 353,
            name: b"scaron\x00" as *const u8 as *const i8,
            desc: b"latin small letter s with caron, U+0161 ISOlat2\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 376,
            name: b"Yuml\x00" as *const u8 as *const i8,
            desc: b"latin capital letter Y with diaeresis, U+0178 ISOlat2\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 402,
            name: b"fnof\x00" as *const u8 as *const i8,
            desc: b"latin small f with hook = function = florin, U+0192 ISOtech\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 710,
            name: b"circ\x00" as *const u8 as *const i8,
            desc: b"modifier letter circumflex accent, U+02C6 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 732,
            name: b"tilde\x00" as *const u8 as *const i8,
            desc: b"small tilde, U+02DC ISOdia\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 913,
            name: b"Alpha\x00" as *const u8 as *const i8,
            desc: b"greek capital letter alpha, U+0391\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 914,
            name: b"Beta\x00" as *const u8 as *const i8,
            desc: b"greek capital letter beta, U+0392\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 915,
            name: b"Gamma\x00" as *const u8 as *const i8,
            desc: b"greek capital letter gamma, U+0393 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 916,
            name: b"Delta\x00" as *const u8 as *const i8,
            desc: b"greek capital letter delta, U+0394 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 917,
            name: b"Epsilon\x00" as *const u8 as *const i8,
            desc: b"greek capital letter epsilon, U+0395\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 918,
            name: b"Zeta\x00" as *const u8 as *const i8,
            desc: b"greek capital letter zeta, U+0396\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 919,
            name: b"Eta\x00" as *const u8 as *const i8,
            desc: b"greek capital letter eta, U+0397\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 920,
            name: b"Theta\x00" as *const u8 as *const i8,
            desc: b"greek capital letter theta, U+0398 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 921,
            name: b"Iota\x00" as *const u8 as *const i8,
            desc: b"greek capital letter iota, U+0399\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 922,
            name: b"Kappa\x00" as *const u8 as *const i8,
            desc: b"greek capital letter kappa, U+039A\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 923,
            name: b"Lambda\x00" as *const u8 as *const i8,
            desc: b"greek capital letter lambda, U+039B ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 924,
            name: b"Mu\x00" as *const u8 as *const i8,
            desc: b"greek capital letter mu, U+039C\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 925,
            name: b"Nu\x00" as *const u8 as *const i8,
            desc: b"greek capital letter nu, U+039D\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 926,
            name: b"Xi\x00" as *const u8 as *const i8,
            desc: b"greek capital letter xi, U+039E ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 927,
            name: b"Omicron\x00" as *const u8 as *const i8,
            desc: b"greek capital letter omicron, U+039F\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 928,
            name: b"Pi\x00" as *const u8 as *const i8,
            desc: b"greek capital letter pi, U+03A0 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 929,
            name: b"Rho\x00" as *const u8 as *const i8,
            desc: b"greek capital letter rho, U+03A1\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 931,
            name: b"Sigma\x00" as *const u8 as *const i8,
            desc: b"greek capital letter sigma, U+03A3 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 932,
            name: b"Tau\x00" as *const u8 as *const i8,
            desc: b"greek capital letter tau, U+03A4\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 933,
            name: b"Upsilon\x00" as *const u8 as *const i8,
            desc: b"greek capital letter upsilon, U+03A5 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 934,
            name: b"Phi\x00" as *const u8 as *const i8,
            desc: b"greek capital letter phi, U+03A6 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 935,
            name: b"Chi\x00" as *const u8 as *const i8,
            desc: b"greek capital letter chi, U+03A7\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 936,
            name: b"Psi\x00" as *const u8 as *const i8,
            desc: b"greek capital letter psi, U+03A8 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 937,
            name: b"Omega\x00" as *const u8 as *const i8,
            desc: b"greek capital letter omega, U+03A9 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 945,
            name: b"alpha\x00" as *const u8 as *const i8,
            desc: b"greek small letter alpha, U+03B1 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 946,
            name: b"beta\x00" as *const u8 as *const i8,
            desc: b"greek small letter beta, U+03B2 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 947,
            name: b"gamma\x00" as *const u8 as *const i8,
            desc: b"greek small letter gamma, U+03B3 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 948,
            name: b"delta\x00" as *const u8 as *const i8,
            desc: b"greek small letter delta, U+03B4 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 949,
            name: b"epsilon\x00" as *const u8 as *const i8,
            desc: b"greek small letter epsilon, U+03B5 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 950,
            name: b"zeta\x00" as *const u8 as *const i8,
            desc: b"greek small letter zeta, U+03B6 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 951,
            name: b"eta\x00" as *const u8 as *const i8,
            desc: b"greek small letter eta, U+03B7 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 952,
            name: b"theta\x00" as *const u8 as *const i8,
            desc: b"greek small letter theta, U+03B8 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 953,
            name: b"iota\x00" as *const u8 as *const i8,
            desc: b"greek small letter iota, U+03B9 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 954,
            name: b"kappa\x00" as *const u8 as *const i8,
            desc: b"greek small letter kappa, U+03BA ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 955,
            name: b"lambda\x00" as *const u8 as *const i8,
            desc: b"greek small letter lambda, U+03BB ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 956,
            name: b"mu\x00" as *const u8 as *const i8,
            desc: b"greek small letter mu, U+03BC ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 957,
            name: b"nu\x00" as *const u8 as *const i8,
            desc: b"greek small letter nu, U+03BD ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 958,
            name: b"xi\x00" as *const u8 as *const i8,
            desc: b"greek small letter xi, U+03BE ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 959,
            name: b"omicron\x00" as *const u8 as *const i8,
            desc: b"greek small letter omicron, U+03BF NEW\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 960,
            name: b"pi\x00" as *const u8 as *const i8,
            desc: b"greek small letter pi, U+03C0 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 961,
            name: b"rho\x00" as *const u8 as *const i8,
            desc: b"greek small letter rho, U+03C1 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 962,
            name: b"sigmaf\x00" as *const u8 as *const i8,
            desc: b"greek small letter final sigma, U+03C2 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 963,
            name: b"sigma\x00" as *const u8 as *const i8,
            desc: b"greek small letter sigma, U+03C3 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 964,
            name: b"tau\x00" as *const u8 as *const i8,
            desc: b"greek small letter tau, U+03C4 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 965,
            name: b"upsilon\x00" as *const u8 as *const i8,
            desc: b"greek small letter upsilon, U+03C5 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 966,
            name: b"phi\x00" as *const u8 as *const i8,
            desc: b"greek small letter phi, U+03C6 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 967,
            name: b"chi\x00" as *const u8 as *const i8,
            desc: b"greek small letter chi, U+03C7 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 968,
            name: b"psi\x00" as *const u8 as *const i8,
            desc: b"greek small letter psi, U+03C8 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 969,
            name: b"omega\x00" as *const u8 as *const i8,
            desc: b"greek small letter omega, U+03C9 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 977,
            name: b"thetasym\x00" as *const u8 as *const i8,
            desc: b"greek small letter theta symbol, U+03D1 NEW\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 978,
            name: b"upsih\x00" as *const u8 as *const i8,
            desc: b"greek upsilon with hook symbol, U+03D2 NEW\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 982,
            name: b"piv\x00" as *const u8 as *const i8,
            desc: b"greek pi symbol, U+03D6 ISOgrk3\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8194,
            name: b"ensp\x00" as *const u8 as *const i8,
            desc: b"en space, U+2002 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8195,
            name: b"emsp\x00" as *const u8 as *const i8,
            desc: b"em space, U+2003 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8201,
            name: b"thinsp\x00" as *const u8 as *const i8,
            desc: b"thin space, U+2009 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8204,
            name: b"zwnj\x00" as *const u8 as *const i8,
            desc: b"zero width non-joiner, U+200C NEW RFC 2070\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8205,
            name: b"zwj\x00" as *const u8 as *const i8,
            desc: b"zero width joiner, U+200D NEW RFC 2070\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8206,
            name: b"lrm\x00" as *const u8 as *const i8,
            desc: b"left-to-right mark, U+200E NEW RFC 2070\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8207,
            name: b"rlm\x00" as *const u8 as *const i8,
            desc: b"right-to-left mark, U+200F NEW RFC 2070\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8211,
            name: b"ndash\x00" as *const u8 as *const i8,
            desc: b"en dash, U+2013 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8212,
            name: b"mdash\x00" as *const u8 as *const i8,
            desc: b"em dash, U+2014 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8216,
            name: b"lsquo\x00" as *const u8 as *const i8,
            desc: b"left single quotation mark, U+2018 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8217,
            name: b"rsquo\x00" as *const u8 as *const i8,
            desc: b"right single quotation mark, U+2019 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8218,
            name: b"sbquo\x00" as *const u8 as *const i8,
            desc: b"single low-9 quotation mark, U+201A NEW\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8220,
            name: b"ldquo\x00" as *const u8 as *const i8,
            desc: b"left double quotation mark, U+201C ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8221,
            name: b"rdquo\x00" as *const u8 as *const i8,
            desc: b"right double quotation mark, U+201D ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8222,
            name: b"bdquo\x00" as *const u8 as *const i8,
            desc: b"double low-9 quotation mark, U+201E NEW\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8224,
            name: b"dagger\x00" as *const u8 as *const i8,
            desc: b"dagger, U+2020 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8225,
            name: b"Dagger\x00" as *const u8 as *const i8,
            desc: b"double dagger, U+2021 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8226,
            name: b"bull\x00" as *const u8 as *const i8,
            desc: b"bullet = black small circle, U+2022 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8230,
            name: b"hellip\x00" as *const u8 as *const i8,
            desc: b"horizontal ellipsis = three dot leader, U+2026 ISOpub\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8240,
            name: b"permil\x00" as *const u8 as *const i8,
            desc: b"per mille sign, U+2030 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8242,
            name: b"prime\x00" as *const u8 as *const i8,
            desc: b"prime = minutes = feet, U+2032 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8243,
            name: b"Prime\x00" as *const u8 as *const i8,
            desc: b"double prime = seconds = inches, U+2033 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8249,
            name: b"lsaquo\x00" as *const u8 as *const i8,
            desc: b"single left-pointing angle quotation mark, U+2039 ISO proposed\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8250,
            name: b"rsaquo\x00" as *const u8 as *const i8,
            desc: b"single right-pointing angle quotation mark, U+203A ISO proposed\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8254,
            name: b"oline\x00" as *const u8 as *const i8,
            desc: b"overline = spacing overscore, U+203E NEW\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8260,
            name: b"frasl\x00" as *const u8 as *const i8,
            desc: b"fraction slash, U+2044 NEW\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8364,
            name: b"euro\x00" as *const u8 as *const i8,
            desc: b"euro sign, U+20AC NEW\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8465,
            name: b"image\x00" as *const u8 as *const i8,
            desc: b"blackletter capital I = imaginary part, U+2111 ISOamso\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8472,
            name: b"weierp\x00" as *const u8 as *const i8,
            desc: b"script capital P = power set = Weierstrass p, U+2118 ISOamso\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8476,
            name: b"real\x00" as *const u8 as *const i8,
            desc: b"blackletter capital R = real part symbol, U+211C ISOamso\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8482,
            name: b"trade\x00" as *const u8 as *const i8,
            desc: b"trade mark sign, U+2122 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8501,
            name: b"alefsym\x00" as *const u8 as *const i8,
            desc: b"alef symbol = first transfinite cardinal, U+2135 NEW\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8592,
            name: b"larr\x00" as *const u8 as *const i8,
            desc: b"leftwards arrow, U+2190 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8593,
            name: b"uarr\x00" as *const u8 as *const i8,
            desc: b"upwards arrow, U+2191 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8594,
            name: b"rarr\x00" as *const u8 as *const i8,
            desc: b"rightwards arrow, U+2192 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8595,
            name: b"darr\x00" as *const u8 as *const i8,
            desc: b"downwards arrow, U+2193 ISOnum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8596,
            name: b"harr\x00" as *const u8 as *const i8,
            desc: b"left right arrow, U+2194 ISOamsa\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8629,
            name: b"crarr\x00" as *const u8 as *const i8,
            desc: b"downwards arrow with corner leftwards = carriage return, U+21B5 NEW\x00"
                as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8656,
            name: b"lArr\x00" as *const u8 as *const i8,
            desc: b"leftwards double arrow, U+21D0 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8657,
            name: b"uArr\x00" as *const u8 as *const i8,
            desc: b"upwards double arrow, U+21D1 ISOamsa\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8658,
            name: b"rArr\x00" as *const u8 as *const i8,
            desc: b"rightwards double arrow, U+21D2 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8659,
            name: b"dArr\x00" as *const u8 as *const i8,
            desc: b"downwards double arrow, U+21D3 ISOamsa\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8660,
            name: b"hArr\x00" as *const u8 as *const i8,
            desc: b"left right double arrow, U+21D4 ISOamsa\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8704,
            name: b"forall\x00" as *const u8 as *const i8,
            desc: b"for all, U+2200 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8706,
            name: b"part\x00" as *const u8 as *const i8,
            desc: b"partial differential, U+2202 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8707,
            name: b"exist\x00" as *const u8 as *const i8,
            desc: b"there exists, U+2203 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8709,
            name: b"empty\x00" as *const u8 as *const i8,
            desc: b"empty set = null set = diameter, U+2205 ISOamso\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8711,
            name: b"nabla\x00" as *const u8 as *const i8,
            desc: b"nabla = backward difference, U+2207 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8712,
            name: b"isin\x00" as *const u8 as *const i8,
            desc: b"element of, U+2208 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8713,
            name: b"notin\x00" as *const u8 as *const i8,
            desc: b"not an element of, U+2209 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8715,
            name: b"ni\x00" as *const u8 as *const i8,
            desc: b"contains as member, U+220B ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8719,
            name: b"prod\x00" as *const u8 as *const i8,
            desc: b"n-ary product = product sign, U+220F ISOamsb\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8721,
            name: b"sum\x00" as *const u8 as *const i8,
            desc: b"n-ary summation, U+2211 ISOamsb\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8722,
            name: b"minus\x00" as *const u8 as *const i8,
            desc: b"minus sign, U+2212 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8727,
            name: b"lowast\x00" as *const u8 as *const i8,
            desc: b"asterisk operator, U+2217 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8730,
            name: b"radic\x00" as *const u8 as *const i8,
            desc: b"square root = radical sign, U+221A ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8733,
            name: b"prop\x00" as *const u8 as *const i8,
            desc: b"proportional to, U+221D ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8734,
            name: b"infin\x00" as *const u8 as *const i8,
            desc: b"infinity, U+221E ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8736,
            name: b"ang\x00" as *const u8 as *const i8,
            desc: b"angle, U+2220 ISOamso\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8743,
            name: b"and\x00" as *const u8 as *const i8,
            desc: b"logical and = wedge, U+2227 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8744,
            name: b"or\x00" as *const u8 as *const i8,
            desc: b"logical or = vee, U+2228 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8745,
            name: b"cap\x00" as *const u8 as *const i8,
            desc: b"intersection = cap, U+2229 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8746,
            name: b"cup\x00" as *const u8 as *const i8,
            desc: b"union = cup, U+222A ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8747,
            name: b"int\x00" as *const u8 as *const i8,
            desc: b"integral, U+222B ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8756,
            name: b"there4\x00" as *const u8 as *const i8,
            desc: b"therefore, U+2234 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8764,
            name: b"sim\x00" as *const u8 as *const i8,
            desc: b"tilde operator = varies with = similar to, U+223C ISOtech\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8773,
            name: b"cong\x00" as *const u8 as *const i8,
            desc: b"approximately equal to, U+2245 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8776,
            name: b"asymp\x00" as *const u8 as *const i8,
            desc: b"almost equal to = asymptotic to, U+2248 ISOamsr\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8800,
            name: b"ne\x00" as *const u8 as *const i8,
            desc: b"not equal to, U+2260 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8801,
            name: b"equiv\x00" as *const u8 as *const i8,
            desc: b"identical to, U+2261 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8804,
            name: b"le\x00" as *const u8 as *const i8,
            desc: b"less-than or equal to, U+2264 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8805,
            name: b"ge\x00" as *const u8 as *const i8,
            desc: b"greater-than or equal to, U+2265 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8834,
            name: b"sub\x00" as *const u8 as *const i8,
            desc: b"subset of, U+2282 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8835,
            name: b"sup\x00" as *const u8 as *const i8,
            desc: b"superset of, U+2283 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8836,
            name: b"nsub\x00" as *const u8 as *const i8,
            desc: b"not a subset of, U+2284 ISOamsn\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8838,
            name: b"sube\x00" as *const u8 as *const i8,
            desc: b"subset of or equal to, U+2286 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8839,
            name: b"supe\x00" as *const u8 as *const i8,
            desc: b"superset of or equal to, U+2287 ISOtech\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8853,
            name: b"oplus\x00" as *const u8 as *const i8,
            desc: b"circled plus = direct sum, U+2295 ISOamsb\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8855,
            name: b"otimes\x00" as *const u8 as *const i8,
            desc: b"circled times = vector product, U+2297 ISOamsb\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8869,
            name: b"perp\x00" as *const u8 as *const i8,
            desc: b"up tack = orthogonal to = perpendicular, U+22A5 ISOtech\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8901,
            name: b"sdot\x00" as *const u8 as *const i8,
            desc: b"dot operator, U+22C5 ISOamsb\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8968,
            name: b"lceil\x00" as *const u8 as *const i8,
            desc: b"left ceiling = apl upstile, U+2308 ISOamsc\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8969,
            name: b"rceil\x00" as *const u8 as *const i8,
            desc: b"right ceiling, U+2309 ISOamsc\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8970,
            name: b"lfloor\x00" as *const u8 as *const i8,
            desc: b"left floor = apl downstile, U+230A ISOamsc\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 8971,
            name: b"rfloor\x00" as *const u8 as *const i8,
            desc: b"right floor, U+230B ISOamsc\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 9001,
            name: b"lang\x00" as *const u8 as *const i8,
            desc: b"left-pointing angle bracket = bra, U+2329 ISOtech\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 9002,
            name: b"rang\x00" as *const u8 as *const i8,
            desc: b"right-pointing angle bracket = ket, U+232A ISOtech\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 9674,
            name: b"loz\x00" as *const u8 as *const i8,
            desc: b"lozenge, U+25CA ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 9824,
            name: b"spades\x00" as *const u8 as *const i8,
            desc: b"black spade suit, U+2660 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 9827,
            name: b"clubs\x00" as *const u8 as *const i8,
            desc: b"black club suit = shamrock, U+2663 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 9829,
            name: b"hearts\x00" as *const u8 as *const i8,
            desc: b"black heart suit = valentine, U+2665 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let init = _htmlEntityDesc {
            value: 9830,
            name: b"diams\x00" as *const u8 as *const i8,
            desc: b"black diamond suit, U+2666 ISOpub\x00" as *const u8 as *const i8,
        };
        init
    },
];
/* ***********************************************************************
 *									*
 *		Commodity functions to handle entities			*
 *									*
 ************************************************************************/
/*
 * Macro used to grow the current buffer.
 */
/* *
 * htmlEntityLookup:
 * @name: the entity name
 *
 * Lookup the given entity in EntitiesTable
 *
 * TODO: the linear scan is really ugly, an hash table is really needed.
 *
 * Returns the associated htmlEntityDescPtr if found, NULL otherwise.
 */

pub fn htmlEntityLookup(name: *const xmlChar) -> *const htmlEntityDesc {
    let mut i: u32 = 0;
    i = 0;
    while (i as u64)
        < ((::std::mem::size_of::<[htmlEntityDesc; 253]>() as u64)
            / (::std::mem::size_of::<htmlEntityDesc>()) as u64)
    {
        if unsafe {
            xmlStrEqual_safe(
                name,
                getHtml40EntitiesTable(i as usize).name as *mut xmlChar,
            )
        } != 0
        {
            unsafe {
                return &*html40EntitiesTable.as_ptr().offset(i as isize) as *const htmlEntityDesc
                    as htmlEntityDescPtr as *const htmlEntityDesc;
            }
        }
        i = i + 1
    }
    return 0 as *const htmlEntityDesc;
}
/* *
 * htmlEntityValueLookup:
 * @value: the entity's unicode value
 *
 * Lookup the given entity in EntitiesTable
 *
 * TODO: the linear scan is really ugly, an hash table is really needed.
 *
 * Returns the associated htmlEntityDescPtr if found, NULL otherwise.
 */

pub fn htmlEntityValueLookup(value: u32) -> *const htmlEntityDesc {
    let mut i: u32 = 0;
    i = 0;
    while (i as u64)
        < ((::std::mem::size_of::<[htmlEntityDesc; 253]>() as u64)
            / (::std::mem::size_of::<htmlEntityDesc>()) as u64)
    {
        if unsafe { getHtml40EntitiesTable(i as usize).value >= value } {
            if unsafe { getHtml40EntitiesTable(i as usize).value > value } {
                break;
            }
            unsafe {
                return &*html40EntitiesTable.as_ptr().offset(i as isize) as *const htmlEntityDesc
                    as htmlEntityDescPtr as *const htmlEntityDesc;
            }
        } else {
            i = i + 1
        }
    }
    return 0 as *const htmlEntityDesc;
}
/* the description */
/*
 * There is only few public functions.
 */
/* *
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
pub fn UTF8ToHtml(mut out: *mut u8, outlen: *mut i32, mut in_0: *const u8, inlen: *mut i32) -> i32 {
    let mut processed: *const u8 = in_0;
    let mut outend: *const u8 = 0 as *const u8;
    let outstart: *const u8 = out;
    let instart: *const u8 = in_0;
    let mut inend: *const u8 = 0 as *const u8;
    let mut c: u32 = 0;
    let mut d: u32 = 0;
    let mut trailing: i32 = 0;
    if out.is_null() || outlen.is_null() || inlen.is_null() {
        return -1;
    }
    if in_0.is_null() {
        /*
         * initialization nothing to do
         */
        unsafe {
            *outlen = 0;
            *inlen = 0;
        }
        return 0;
    }
    unsafe {
        inend = in_0.offset(*inlen as isize);
    }
    unsafe {
        outend = out.offset(*outlen as isize);
    }
    while in_0 < inend {
        unsafe {
            d = *in_0 as u32;
        }
        unsafe {
            in_0 = in_0.offset(1);
        }
        if d < 0x80 {
            c = d;
            trailing = 0
        } else if d < 0xc0 {
            /* trailing byte in leading position */
            unsafe {
                *outlen = out.offset_from(outstart) as i32;
            }
            unsafe {
                *inlen = processed.offset_from(instart) as i32;
            }
            return -2;
        } else {
            if d < 0xe0 {
                c = d & 0x1f;
                trailing = 1
            } else if d < 0xf0 {
                c = d & 0xf;
                trailing = 2
            } else if d < 0xf8 {
                c = d & 0x7;
                trailing = 3
            } else {
                /* no chance for this in Ascii */
                unsafe {
                    *outlen = out.offset_from(outstart) as i32;
                }
                unsafe {
                    *inlen = processed.offset_from(instart) as i32;
                }
                return -2;
            }
        }
        unsafe {
            if (inend.offset_from(in_0) as i64) < trailing as i64 {
                break;
            }
        }
        while trailing != 0 {
            if in_0 >= inend || {
                unsafe {
                    d = *in_0 as u32;
                }
                unsafe {
                    in_0 = in_0.offset(1);
                }
                (d & 0xc0) != 0x80
            } {
                break;
            }
            c <<= 6;
            c |= d & 0x3f;
            trailing -= 1
        }
        /* assertion: c is a single UTF-4 value */
        if c < 0x80 {
            unsafe {
                if out.offset(1 as isize) >= outend as *mut u8 {
                    break;
                }
            }
            let out_old = out;
            unsafe {
                out = out.offset(1);
            }
            unsafe { *out_old = c as u8 }
        } else {
            let mut len: i32 = 0;
            let mut ent: *const htmlEntityDesc = 0 as *const htmlEntityDesc;
            let mut cp: *const i8 = 0 as *const i8;
            let mut nbuf: [i8; 16] = [0; 16];
            /*
             * Try to lookup a predefined HTML entity for it
             */
            ent = htmlEntityValueLookup(c);
            if ent.is_null() {
                snprintf_safe_macro!(
                    nbuf.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 16]>() as u64,
                    b"#%u\x00" as *const u8 as *const i8,
                    c
                );
                cp = nbuf.as_mut_ptr()
            } else {
                unsafe { cp = (*ent).name }
            }
            unsafe {
                len = strlen_safe(cp) as i32;
                if out.offset(2 as isize).offset(len as isize) >= outend as *mut u8 {
                    break;
                }
            }
            unsafe {
                *out = '&' as u8;
                out = out.offset(1);
                memcpy_safe(out as *mut (), cp as *const (), len as u64);
                out = out.offset(len as isize);
            }
            let out_old = out;
            unsafe {
                out = out.offset(1);
                *out_old = ';' as u8
            }
        }
        processed = in_0
    }
    unsafe {
        *outlen = out.offset_from(outstart) as i32;

        *inlen = processed.offset_from(instart) as i32;
    }
    return 0;
}
/* *
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
pub fn htmlEncodeEntities(
    mut out: *mut u8,
    outlen: *mut i32,
    mut in_0: *const u8,
    inlen: *mut i32,
    quoteChar: i32,
) -> i32 {
    let mut processed: *const u8 = in_0;
    let mut outend: *const u8 = 0 as *const u8;
    let outstart: *const u8 = out;
    let instart: *const u8 = in_0;
    let mut inend: *const u8 = 0 as *const u8;
    let mut c: u32 = 0;
    let mut d: u32 = 0;
    let mut trailing: i32 = 0;
    if out.is_null() || outlen.is_null() || inlen.is_null() || in_0.is_null() {
        return -1;
    }
    unsafe {
        outend = out.offset(*outlen as isize);
        inend = in_0.offset(*inlen as isize);
    }
    while in_0 < inend {
        unsafe {
            d = *in_0 as u32;
            in_0 = in_0.offset(1);
        }
        if d < 0x80 {
            c = d;
            trailing = 0
        } else if d < 0xc0 {
            /* trailing byte in leading position */
            unsafe {
                *outlen = out.offset_from(outstart) as i32;
            }
            unsafe {
                *inlen = processed.offset_from(instart) as i32;
            }
            return -2;
        } else {
            if d < 0xe0 {
                c = d & 0x1f;
                trailing = 1
            } else if d < 0xf0 {
                c = d & 0xf;
                trailing = 2
            } else if d < 0xf8 {
                c = d & 0x7;
                trailing = 3
            } else {
                /* no chance for this in Ascii */
                unsafe {
                    *outlen = out.offset_from(outstart) as i32;
                }
                unsafe {
                    *inlen = processed.offset_from(instart) as i32;
                }
                return -2;
            }
        }
        unsafe {
            if (inend.offset_from(in_0) as i64) < trailing as i64 {
                break;
            }
        }
        loop {
            if !(trailing != 0) {
                break;
            }
            trailing = trailing - 1;
            unsafe {
                d = *in_0 as u32;
            }
            unsafe {
                in_0 = in_0.offset(1);
            }
            if d & 0xc0 != 0x80 {
                unsafe {
                    *outlen = out.offset_from(outstart) as i32;
                }
                unsafe {
                    *inlen = processed.offset_from(instart) as i32;
                }
                return -2;
            }
            c <<= 6;
            c |= d & 0x3f
        }
        /* assertion: c is a single UTF-4 value */
        if c < 0x80
            && c != quoteChar as u32
            && c != '&' as u32
            && c != '<' as u32
            && c != '>' as u32
        {
            if out >= outend as *mut u8 {
                break;
            }
            let out_old = out;
            unsafe {
                out = out.offset(1);
            }
            unsafe { *out_old = c as u8 }
        } else {
            let mut ent: *const htmlEntityDesc = 0 as *const htmlEntityDesc;
            let mut cp: *const i8 = 0 as *const i8;
            let mut nbuf: [i8; 16] = [0; 16];
            let mut len: i32 = 0;
            /*
             * Try to lookup a predefined HTML entity for it
             */
            ent = htmlEntityValueLookup(c);
            if ent.is_null() {
                snprintf_safe_macro!(
                    nbuf.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 16]>() as u64,
                    b"#%u\x00" as *const u8 as *const i8,
                    c
                );
                cp = nbuf.as_mut_ptr()
            } else {
                unsafe { cp = (*ent).name }
            }
            len = unsafe { strlen_safe(cp) as i32 };
            unsafe {
                if out.offset(2 as isize).offset(len as isize) > outend as *mut u8 {
                    break;
                }
            }
            unsafe {
                *out = '&' as u8;
            }
            unsafe {
                out = out.offset(1);
            }
            unsafe {
                memcpy_safe(out as *mut (), cp as *const (), len as u64);
            }
            unsafe {
                out = out.offset(len as isize);
            }
            let out_old = out;
            unsafe {
                out = out.offset(1);
            }
            unsafe { *out_old = ';' as u8 }
        }
        processed = in_0
    }
    unsafe {
        *outlen = out.offset_from(outstart) as i32;
    }
    unsafe {
        *inlen = processed.offset_from(instart) as i32;
    }
    return 0;
}
/* ***********************************************************************
 *									*
 *		Commodity functions to handle streams			*
 *									*
 ************************************************************************/
/* *
 * htmlNewInputStream:
 * @ctxt:  an HTML parser context
 *
 * Create a new input stream structure
 * Returns the new input stream or NULL
 */
#[cfg(LIBXML_PUSH_ENABLED)]
fn htmlNewInputStream(ctxt: htmlParserCtxtPtr) -> htmlParserInputPtr {
    let mut input: htmlParserInputPtr = 0 as *mut xmlParserInput;
    input = unsafe {
        xmlMalloc_safe(::std::mem::size_of::<htmlParserInput>() as u64) as xmlParserInputPtr
    };
    if input.is_null() {
        htmlErrMemory(
            ctxt,
            b"couldn\'t allocate a new input stream\n\x00" as *const u8 as *const i8,
        );
        return 0 as htmlParserInputPtr;
    }
    unsafe {
        memset_safe(
            input as *mut (),
            0,
            ::std::mem::size_of::<htmlParserInput>() as u64,
        )
    };
    let inputPtr = unsafe { &mut *input };
    inputPtr.filename = 0 as *const i8;
    inputPtr.directory = 0 as *const i8;
    inputPtr.base = 0 as *const xmlChar;
    inputPtr.cur = 0 as *const xmlChar;
    inputPtr.buf = 0 as xmlParserInputBufferPtr;
    inputPtr.line = 1;
    inputPtr.col = 1;
    inputPtr.buf = 0 as xmlParserInputBufferPtr;
    inputPtr.free = None;
    inputPtr.version = 0 as *const xmlChar;
    inputPtr.consumed = 0 as u64;
    inputPtr.length = 0;
    return input;
}
/* ***********************************************************************
 *									*
 *		Commodity functions, cleanup needed ?			*
 *									*
 ************************************************************************/
/*
 * all tags allowing pc data from the html 4.01 loose dtd
 * NOTE: it might be more appropriate to integrate this information
 * into the html40ElementTable array but I don't want to risk any
 * binary incompatibility
 */
static mut allowPCData: [*const i8; 53] = [
    b"a\x00" as *const u8 as *const i8,
    b"abbr\x00" as *const u8 as *const i8,
    b"acronym\x00" as *const u8 as *const i8,
    b"address\x00" as *const u8 as *const i8,
    b"applet\x00" as *const u8 as *const i8,
    b"b\x00" as *const u8 as *const i8,
    b"bdo\x00" as *const u8 as *const i8,
    b"big\x00" as *const u8 as *const i8,
    b"blockquote\x00" as *const u8 as *const i8,
    b"body\x00" as *const u8 as *const i8,
    b"button\x00" as *const u8 as *const i8,
    b"caption\x00" as *const u8 as *const i8,
    b"center\x00" as *const u8 as *const i8,
    b"cite\x00" as *const u8 as *const i8,
    b"code\x00" as *const u8 as *const i8,
    b"dd\x00" as *const u8 as *const i8,
    b"del\x00" as *const u8 as *const i8,
    b"dfn\x00" as *const u8 as *const i8,
    b"div\x00" as *const u8 as *const i8,
    b"dt\x00" as *const u8 as *const i8,
    b"em\x00" as *const u8 as *const i8,
    b"font\x00" as *const u8 as *const i8,
    b"form\x00" as *const u8 as *const i8,
    b"h1\x00" as *const u8 as *const i8,
    b"h2\x00" as *const u8 as *const i8,
    b"h3\x00" as *const u8 as *const i8,
    b"h4\x00" as *const u8 as *const i8,
    b"h5\x00" as *const u8 as *const i8,
    b"h6\x00" as *const u8 as *const i8,
    b"i\x00" as *const u8 as *const i8,
    b"iframe\x00" as *const u8 as *const i8,
    b"ins\x00" as *const u8 as *const i8,
    b"kbd\x00" as *const u8 as *const i8,
    b"label\x00" as *const u8 as *const i8,
    b"legend\x00" as *const u8 as *const i8,
    b"li\x00" as *const u8 as *const i8,
    b"noframes\x00" as *const u8 as *const i8,
    b"noscript\x00" as *const u8 as *const i8,
    b"object\x00" as *const u8 as *const i8,
    b"p\x00" as *const u8 as *const i8,
    b"pre\x00" as *const u8 as *const i8,
    b"q\x00" as *const u8 as *const i8,
    b"s\x00" as *const u8 as *const i8,
    b"samp\x00" as *const u8 as *const i8,
    b"small\x00" as *const u8 as *const i8,
    b"span\x00" as *const u8 as *const i8,
    b"strike\x00" as *const u8 as *const i8,
    b"strong\x00" as *const u8 as *const i8,
    b"td\x00" as *const u8 as *const i8,
    b"th\x00" as *const u8 as *const i8,
    b"tt\x00" as *const u8 as *const i8,
    b"u\x00" as *const u8 as *const i8,
    b"var\x00" as *const u8 as *const i8,
];
/* *
 * areBlanks:
 * @ctxt:  an HTML parser context
 * @str:  a xmlChar *
 * @len:  the size of @str
 *
 * Is this a sequence of blank chars that one can ignore ?
 *
 * Returns 1 if ignorable 0 otherwise.
 */
fn areBlanks_htmlparser(ctxt: htmlParserCtxtPtr, str: *const xmlChar, len: i32) -> i32 {
    let mut i: u32 = 0;
    let mut j: i32 = 0;
    let mut lastChild: xmlNodePtr = 0 as *mut xmlNode;
    let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
    j = 0;
    while j < len {
        unsafe {
            if !IS_BLANK_CH(*str.offset(j as isize) as i32) {
                return 0;
            }
        }
        j += 1
    }
    if CUR(ctxt) == 0 {
        return 1;
    }
    if CUR(ctxt) != '<' as i32 {
        return 0;
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.name.is_null() {
        return 1;
    }
    if unsafe {
        xmlStrEqual_safe(
            ctxtPtr.name,
            b"html\x00" as *const u8 as *const i8 as *mut xmlChar,
        )
    } != 0
    {
        return 1;
    }
    if unsafe {
        xmlStrEqual_safe(
            ctxtPtr.name,
            b"head\x00" as *const u8 as *const i8 as *mut xmlChar,
        )
    } != 0
    {
        return 1;
    }
    /* Only strip CDATA children of the body tag for strict HTML DTDs */
    if unsafe {
        xmlStrEqual_safe(
            ctxtPtr.name,
            b"body\x00" as *const u8 as *const i8 as *mut xmlChar,
        )
    } != 0
        && !ctxtPtr.myDoc.is_null()
    {
        unsafe {
            dtd = xmlGetIntSubset_safe(ctxtPtr.myDoc as *const xmlDoc);
            let dtd_condition = !dtd.is_null() && !(*dtd).ExternalID.is_null();
            if dtd_condition {
                let mut dtdPtr = unsafe { &mut *dtd };
                if xmlStrcasecmp_safe(
                    dtdPtr.ExternalID,
                    b"-//W3C//DTD HTML 4.01//EN\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) == 0
                    || xmlStrcasecmp_safe(
                        dtdPtr.ExternalID,
                        b"-//W3C//DTD HTML 4//EN\x00" as *const u8 as *const i8 as *mut xmlChar,
                    ) == 0
                {
                    return 1;
                }
            }
        }
    }
    if ctxtPtr.node.is_null() {
        return 0;
    }
    lastChild = unsafe { xmlGetLastChild_safe(ctxtPtr.node as *const xmlNode) };
    unsafe {
        while !lastChild.is_null() && (*lastChild).type_0 as u32 == XML_COMMENT_NODE as u32 {
            lastChild = (*lastChild).prev
        }
    }
    if lastChild.is_null() {
        let mut nodePtr = unsafe { &mut *(*ctxt).node };
        if nodePtr.type_0 as u32 != XML_ELEMENT_NODE as u32 && !nodePtr.content.is_null() {
            return 0;
        }
        /* keep ws in constructs like ...<b> </b>...
        for all tags "b" allowing PCDATA */
        i = 0;
        while (i as u64)
            < ((::std::mem::size_of::<[*const i8; 53]>() as u64)
                / (::std::mem::size_of::<*const i8>()) as u64)
        {
            if unsafe { xmlStrEqual_safe(ctxtPtr.name, getAllowPCData(i as usize) as *mut xmlChar) }
                != 0
            {
                return 0;
            }
            i = i + 1
        }
    } else if unsafe { xmlNodeIsText_safe(lastChild as *const xmlNode) } != 0 {
        return 0;
    } else {
        /* keep ws in constructs like <p><b>xy</b> <i>z</i><p>
        for all tags "p" allowing PCDATA */
        i = 0;
        while (i as u64)
            < ((::std::mem::size_of::<[*const i8; 53]>() as u64)
                / (::std::mem::size_of::<*const i8>()) as u64)
        {
            unsafe {
                if xmlStrEqual_safe(
                    (*lastChild).name,
                    getAllowPCData(i as usize) as *mut xmlChar,
                ) != 0
                {
                    return 0;
                }
            }
            i = i + 1
        }
    }
    return 1;
}
/* *
 * htmlNewDocNoDtD:
 * @URI:  URI for the dtd, or NULL
 * @ExternalID:  the external ID of the DTD, or NULL
 *
 * Creates a new HTML document without a DTD node if @URI and @ExternalID
 * are NULL
 *
 * Returns a new document, do not initialize the DTD if not provided
 */

pub fn htmlNewDocNoDtD(URI: *const xmlChar, ExternalID: *const xmlChar) -> htmlDocPtr {
    let mut cur: xmlDocPtr = 0 as *mut xmlDoc;
    /*
     * Allocate a new document and fill the fields.
     */
    cur = unsafe { xmlMalloc_safe(::std::mem::size_of::<xmlDoc>() as u64) } as xmlDocPtr;
    if cur.is_null() {
        htmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"HTML document creation failed\n\x00" as *const u8 as *const i8,
        );
        return 0 as htmlDocPtr;
    }
    unsafe { memset_safe(cur as *mut (), 0, ::std::mem::size_of::<xmlDoc>() as u64) };
    let mut curPtr = unsafe { &mut *cur };
    curPtr.type_0 = XML_HTML_DOCUMENT_NODE;
    curPtr.version = 0 as *const xmlChar;
    curPtr.intSubset = 0 as *mut _xmlDtd;
    curPtr.doc = cur;
    curPtr.name = 0 as *mut i8;
    curPtr.children = 0 as *mut _xmlNode;
    curPtr.extSubset = 0 as *mut _xmlDtd;
    curPtr.oldNs = 0 as *mut _xmlNs;
    curPtr.encoding = 0 as *const xmlChar;
    curPtr.standalone = 1;
    curPtr.compression = 0;
    curPtr.ids = 0 as *mut ();
    curPtr.refs = 0 as *mut ();
    curPtr._private = 0 as *mut ();
    curPtr.charset = XML_CHAR_ENCODING_UTF8;
    curPtr.properties = XML_DOC_HTML as i32 | XML_DOC_USERBUILT as i32;
    if !ExternalID.is_null() || !URI.is_null() {
        unsafe {
            xmlCreateIntSubset_safe(
                cur,
                b"html\x00" as *const u8 as *const i8 as *mut xmlChar,
                ExternalID,
                URI,
            )
        };
    }
    return cur;
}
/* *
 * htmlNewDoc:
 * @URI:  URI for the dtd, or NULL
 * @ExternalID:  the external ID of the DTD, or NULL
 *
 * Creates a new HTML document
 *
 * Returns a new document
 */

pub fn htmlNewDoc(URI: *const xmlChar, ExternalID: *const xmlChar) -> htmlDocPtr {
    if URI.is_null() && ExternalID.is_null() {
        return htmlNewDocNoDtD(
            b"http://www.w3.org/TR/REC-html40/loose.dtd\x00" as *const u8 as *const i8
                as *mut xmlChar,
            b"-//W3C//DTD HTML 4.0 Transitional//EN\x00" as *const u8 as *const i8 as *mut xmlChar,
        );
    }
    return htmlNewDocNoDtD(URI, ExternalID);
}
/* *
 * htmlParseHTMLName:
 * @ctxt:  an HTML parser context
 *
 * parse an HTML tag or attribute name, note that we convert it to lowercase
 * since HTML names are not case-sensitive.
 *
 * Returns the Tag Name parsed or NULL
 */
fn htmlParseHTMLName(mut ctxt: htmlParserCtxtPtr) -> *const xmlChar {
    let mut i: i32 = 0;
    let mut loc: [xmlChar; 100] = [0; 100];
    if !IS_ASCII_LETTER(CUR(ctxt))
        && CUR(ctxt) != '_' as i32
        && CUR(ctxt) != ':' as i32
        && CUR(ctxt) != '.' as i32
    {
        return 0 as *const xmlChar;
    }
    while i < HTML_PARSER_BUFFER_SIZE
        && (IS_ASCII_LETTER(CUR(ctxt))
            || IS_ASCII_DIGIT(CUR(ctxt))
            || CUR(ctxt) == ':' as i32
            || CUR(ctxt) == '-' as i32
            || CUR(ctxt) == '_' as i32
            || CUR(ctxt) == '.' as i32)
    {
        if CUR(ctxt) >= 'A' as i32 && CUR(ctxt) <= 'Z' as i32 {
            loc[i as usize] = (CUR(ctxt) + 0x20) as xmlChar
        } else {
            loc[i as usize] = CUR(ctxt) as xmlChar
        }
        i += 1;
        unsafe { xmlNextChar_safe(ctxt) };
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    return unsafe { xmlDictLookup_safe(ctxtPtr.dict, loc.as_mut_ptr(), i) };
}
/* *
 * htmlParseHTMLName_nonInvasive:
 * @ctxt:  an HTML parser context
 *
 * parse an HTML tag or attribute name, note that we convert it to lowercase
 * since HTML names are not case-sensitive, this doesn't consume the data
 * from the stream, it's a look-ahead
 *
 * Returns the Tag Name parsed or NULL
 */
fn htmlParseHTMLName_nonInvasive(mut ctxt: htmlParserCtxtPtr) -> *const xmlChar {
    let mut i: i32 = 0;
    let mut loc: [xmlChar; 100] = [0; 100];
    if !(IS_ASCII_LETTER(NXT(ctxt, 1))) && NXT(ctxt, 1) != '_' as i32 && NXT(ctxt, 1) != ':' as i32
    {
        return 0 as *const xmlChar;
    }
    while i < HTML_PARSER_BUFFER_SIZE
        && (IS_ASCII_LETTER(NXT(ctxt, 1 + i))
            || IS_ASCII_DIGIT(NXT(ctxt, 1 + i))
            || NXT(ctxt, 1 + i) == ':' as i32
            || NXT(ctxt, 1 + i) == '-' as i32
            || NXT(ctxt, 1 + i) == '_' as i32)
    {
        if NXT(ctxt, 1 + i) >= 'A' as i32 && NXT(ctxt, 1 + i) <= 'Z' as i32 {
            loc[i as usize] = (NXT(ctxt, 1 + i) + 0x20) as xmlChar
        } else {
            loc[i as usize] = NXT(ctxt, 1 + i) as xmlChar
        }
        i += 1
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    return unsafe { xmlDictLookup_safe(ctxtPtr.dict, loc.as_mut_ptr(), i) };
}
/* *
 * htmlParseName:
 * @ctxt:  an HTML parser context
 *
 * parse an HTML name, this routine is case sensitive.
 *
 * Returns the Name parsed or NULL
 */
fn htmlParseName(ctxt: htmlParserCtxtPtr) -> *const xmlChar {
    let ctxtPtr = unsafe { &mut *ctxt };
    let inputPtr = unsafe { &mut *(*ctxt).input };
    let mut in_0: *const xmlChar;
    let ret: *const xmlChar;
    let mut count: i32 = 0;
    GROW(ctxt);
    /*
     * Accelerator for simple ASCII names
     */
    in_0 = inputPtr.cur;
    let mut in_0_safe = unsafe { *in_0 };
    if in_0_safe >= 0x61 && in_0_safe <= 0x7a
        || in_0_safe >= 0x41 && in_0_safe <= 0x5a
        || in_0_safe == '_' as u8
        || in_0_safe == ':' as u8
    {
        unsafe {
            in_0 = in_0.offset(1);
        }
        in_0_safe = unsafe { *in_0 };
        while in_0_safe >= 0x61 && in_0_safe <= 0x7a
            || in_0_safe >= 0x41 && in_0_safe <= 0x5a
            || in_0_safe >= 0x30 && in_0_safe <= 0x39
            || in_0_safe == '_' as u8
            || in_0_safe == '-' as u8
            || in_0_safe == ':' as u8
            || in_0_safe == '.' as u8
        {
            unsafe { in_0 = in_0.offset(1) }
            in_0_safe = unsafe { *in_0 };
        }
        if in_0 == inputPtr.end {
            return 0 as *const xmlChar;
        }
        if in_0_safe > 0 && in_0_safe < 0x80 {
            unsafe {
                count = in_0.offset_from(inputPtr.cur) as i32;
            }
            ret = unsafe { xmlDictLookup_safe(ctxtPtr.dict, inputPtr.cur, count) };
            inputPtr.cur = in_0;
            inputPtr.col += count;
            return ret;
        }
    }
    return htmlParseNameComplex(ctxt);
}
/* ***********************************************************************
 *									*
 *			The parser itself				*
 *	Relates to http://www.w3.org/TR/html40				*
 *									*
 ************************************************************************/
/* ***********************************************************************
 *									*
 *			The parser itself				*
 *									*
 ************************************************************************/
fn htmlParseNameComplex(ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let ctxtPtr = unsafe { &mut *ctxt };
    let inputPtr = unsafe { &mut *(*ctxt).input };
    let mut len: i32 = 0;
    let mut l: i32 = 0;
    let mut c: i32;
    let mut count: i32 = 0;
    let base: *const xmlChar = inputPtr.base;
    /*
     * Handler for more complex cases
     */
    GROW(ctxt);
    c = htmlCurrentChar(ctxt, &mut l);
    if c == ' ' as i32
        || c == '>' as i32
        || c == '/' as i32
        || (!IS_LETTER(c, unsafe { getXmlIsBaseCharGroup() }) && c != '_' as i32 && c != ':' as i32)
    {
        return 0 as *const xmlChar;
    }
    while c != ' ' as i32
        && c != '>' as i32
        && c != '/' as i32
        && (IS_LETTER(c, unsafe { getXmlIsBaseCharGroup() })
            || IS_DIGIT(c, unsafe { getXmlIsDigitGroup() })
            || c == '.' as i32
            || c == '-' as i32
            || c == '_' as i32
            || c == ':' as i32
            || IS_COMBINING(c, unsafe { getXmlIsCombiningGroup() })
            || IS_EXTENDER(c, unsafe { getXmlIsExtenderGroup() }))
    {
        let count_old = count;
        count = count + 1;
        if count_old > 100 {
            count = 0;
            GROW(ctxt);
        }
        len += l;
        NEXTL(ctxt, l);
        c = htmlCurrentChar(ctxt, &mut l);
        if inputPtr.base != base {
            /*
             * We changed encoding from an unknown encoding
             * Input buffer changed location, so we better start again
             */
            return htmlParseNameComplex(ctxt);
        }
    }
    if (unsafe { inputPtr.cur.offset_from(inputPtr.base) } as i64) < len as i64 {
        /* Sanity check */
        htmlParseErr(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"unexpected change of input buffer\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return 0 as *const xmlChar;
    }
    unsafe {
        return xmlDictLookup_safe(ctxtPtr.dict, inputPtr.cur.offset(-(len as isize)), len);
    }
}
/* *
 * htmlParseHTMLAttribute:
 * @ctxt:  an HTML parser context
 * @stop:  a char stop value
 *
 * parse an HTML attribute value till the stop (quote), if
 * stop is 0 then it stops at the first space
 *
 * Returns the attribute parsed or NULL
 */
fn htmlParseHTMLAttribute(ctxt: htmlParserCtxtPtr, stop: xmlChar) -> *mut xmlChar {
    let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
    let mut buffer_size: i32 = 0;
    let mut out: *mut xmlChar = 0 as *mut xmlChar;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut ent: *const htmlEntityDesc;
    /*
     * allocate a translation buffer.
     */
    buffer_size = HTML_PARSER_BUFFER_SIZE;
    buffer = unsafe {
        xmlMallocAtomic_safe(
            (buffer_size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
        )
    } as *mut xmlChar;
    if buffer.is_null() {
        htmlErrMemory(
            ctxt,
            b"buffer allocation failed\n\x00" as *const u8 as *const i8,
        );
        return 0 as *mut xmlChar;
    }
    out = buffer;
    /*
     * Ok loop until we reach one of the ending chars
     */
    while CUR(ctxt) != 0 && CUR(ctxt) != stop as i32 {
        if stop == 0 && CUR(ctxt) == '>' as i32 {
            break;
        }
        if stop == 0 && IS_BLANK_CH(CUR(ctxt)) {
            break;
        }
        unsafe {
            if CUR(ctxt) == '&' as i32 {
                if NXT(ctxt, 1) == '#' as i32 {
                    let mut c: u32 = 0;
                    let mut bits: i32 = 0;
                    c = htmlParseCharRef(ctxt) as u32;
                    if c < 0x80 {
                        *out = c as xmlChar;
                        out = out.offset(1);
                        bits = -6
                    } else if c < 0x800 {
                        *out = (c >> 6 & 0x1f | 0xc0) as xmlChar;
                        out = out.offset(1);
                        bits = 0
                    } else if c < 0x10000 {
                        *out = (c >> 12 & 0xf | 0xe0) as xmlChar;
                        out = out.offset(1);
                        bits = 6
                    } else {
                        *out = (c >> 18 & 0x7 | 0xf0) as xmlChar;
                        out = out.offset(1);
                        bits = 12
                    }
                    while bits >= 0 {
                        *out = (c >> bits & 0x3f | 0x80) as xmlChar;
                        out = out.offset(1);
                        bits -= 6
                    }
                    if out.offset_from(buffer) as i64 > (buffer_size - 100) as i64 {
                        let mut indx: i32 = out.offset_from(buffer) as i32;
                        let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                        buffer_size *= 2;
                        tmp = xmlRealloc_safe(
                            buffer as *mut (),
                            (buffer_size as u64)
                                .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                        ) as *mut xmlChar;
                        if tmp.is_null() {
                            htmlErrMemory(ctxt, b"growing buffer\n\x00" as *const u8 as *const i8);
                            xmlFree_safe(buffer as *mut ());
                            return 0 as *mut xmlChar;
                        }
                        buffer = tmp;
                        out = &mut *buffer.offset(indx as isize) as *mut xmlChar
                    }
                } else {
                    ent = htmlParseEntityRef(ctxt, &mut name);
                    if name.is_null() {
                        *out = '&' as xmlChar;
                        out = out.offset(1);
                        if out.offset_from(buffer) as i64 > (buffer_size - 100) as i64 {
                            let mut indx_0: i32 = out.offset_from(buffer) as i32;
                            let mut tmp_0: *mut xmlChar = 0 as *mut xmlChar;
                            buffer_size *= 2;
                            tmp_0 = xmlRealloc_safe(
                                buffer as *mut (),
                                (buffer_size as u64)
                                    .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                            ) as *mut xmlChar;
                            if tmp_0.is_null() {
                                htmlErrMemory(
                                    ctxt,
                                    b"growing buffer\n\x00" as *const u8 as *const i8,
                                );
                                xmlFree_safe(buffer as *mut ());
                                return 0 as *mut xmlChar;
                            }
                            buffer = tmp_0;
                            out = &mut *buffer.offset(indx_0 as isize) as *mut xmlChar
                        }
                    } else if ent.is_null() {
                        *out = '&' as xmlChar;
                        out = out.offset(1);
                        cur = name;
                        while *cur as i32 != 0 {
                            if out.offset_from(buffer) as i64 > (buffer_size - 100) as i64 {
                                let mut indx_1: i32 = out.offset_from(buffer) as i32;
                                let mut tmp_1: *mut xmlChar = 0 as *mut xmlChar;
                                buffer_size *= 2;
                                tmp_1 = xmlRealloc_safe(
                                    buffer as *mut (),
                                    (buffer_size as u64)
                                        .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                                ) as *mut xmlChar;
                                if tmp_1.is_null() {
                                    htmlErrMemory(
                                        ctxt,
                                        b"growing buffer\n\x00" as *const u8 as *const i8,
                                    );
                                    xmlFree_safe(buffer as *mut ());
                                    return 0 as *mut xmlChar;
                                }
                                buffer = tmp_1;
                                out = &mut *buffer.offset(indx_1 as isize) as *mut xmlChar
                            }
                            let cur_old = cur;
                            cur = cur.offset(1);
                            let out_old = out;
                            out = out.offset(1);
                            *out_old = *cur_old
                        }
                    } else {
                        let mut c_0: u32;
                        let mut bits_0: i32;
                        if out.offset_from(buffer) as i64 > (buffer_size - 100) as i64 {
                            let mut indx_2: i32 = out.offset_from(buffer) as i32;
                            let mut tmp_2: *mut xmlChar = 0 as *mut xmlChar;
                            buffer_size *= 2;
                            tmp_2 = xmlRealloc_safe(
                                buffer as *mut (),
                                (buffer_size as u64)
                                    .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                            ) as *mut xmlChar;
                            if tmp_2.is_null() {
                                htmlErrMemory(
                                    ctxt,
                                    b"growing buffer\n\x00" as *const u8 as *const i8,
                                );
                                xmlFree_safe(buffer as *mut ());
                                return 0 as *mut xmlChar;
                            }
                            buffer = tmp_2;
                            out = &mut *buffer.offset(indx_2 as isize) as *mut xmlChar
                        }
                        c_0 = (*ent).value;
                        if c_0 < 0x80 {
                            *out = c_0 as xmlChar;
                            out = out.offset(1);
                            bits_0 = -6
                        } else if c_0 < 0x800 {
                            *out = (c_0 >> 6 & 0x1f | 0xc0) as xmlChar;
                            out = out.offset(1);
                            bits_0 = 0
                        } else if c_0 < 0x10000 {
                            *out = (c_0 >> 12 & 0xf | 0xe0) as xmlChar;
                            out = out.offset(1);
                            bits_0 = 6
                        } else {
                            *out = (c_0 >> 18 & 0x7 | 0xf0) as xmlChar;
                            out = out.offset(1);
                            bits_0 = 12
                        }
                        while bits_0 >= 0 {
                            *out = (c_0 >> bits_0 & 0x3f | 0x80) as xmlChar;
                            out = out.offset(1);
                            bits_0 -= 6
                        }
                    }
                }
            } else {
                let mut c_1: u32;
                let mut bits_1: i32;
                let mut l: i32 = 0;
                if out.offset_from(buffer) as i64 > (buffer_size - 100) as i64 {
                    let mut indx_3: i32 = out.offset_from(buffer) as i32;
                    let mut tmp_3: *mut xmlChar = 0 as *mut xmlChar;
                    buffer_size *= 2;
                    tmp_3 = xmlRealloc_safe(
                        buffer as *mut (),
                        (buffer_size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                    ) as *mut xmlChar;
                    if tmp_3.is_null() {
                        htmlErrMemory(ctxt, b"growing buffer\n\x00" as *const u8 as *const i8);
                        xmlFree_safe(buffer as *mut ());
                        return 0 as *mut xmlChar;
                    }
                    buffer = tmp_3;
                    out = &mut *buffer.offset(indx_3 as isize) as *mut xmlChar
                }
                c_1 = htmlCurrentChar(ctxt, &mut l) as u32;
                if c_1 < 0x80 {
                    *out = c_1 as xmlChar;
                    out = out.offset(1);
                    bits_1 = -(6)
                } else if c_1 < 0x800 {
                    *out = (c_1 >> 6 & 0x1f | 0xc0) as xmlChar;
                    out = out.offset(1);
                    bits_1 = 0
                } else if c_1 < 0x10000 {
                    *out = (c_1 >> 12 & 0xf | 0xe0) as xmlChar;
                    out = out.offset(1);
                    bits_1 = 6
                } else {
                    *out = (c_1 >> 18 & 0x7 | 0xf0) as xmlChar;
                    out = out.offset(1);
                    bits_1 = 12
                }
                while bits_1 >= 0 {
                    *out = (c_1 >> bits_1 & 0x3f | 0x80) as xmlChar;
                    out = out.offset(1);
                    bits_1 -= 6
                }
                xmlNextChar_safe(ctxt);
            }
        }
    }
    unsafe {
        *out = 0 as xmlChar;
    }
    return buffer;
}
/* *
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

pub fn htmlParseEntityRef(
    ctxt: htmlParserCtxtPtr,
    str: *mut *const xmlChar,
) -> *const htmlEntityDesc {
    let mut name: *const xmlChar;
    let mut ent: *const htmlEntityDesc = 0 as *const htmlEntityDesc;
    if !str.is_null() {
        unsafe { *str = 0 as *const xmlChar }
    }
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            return 0 as *const htmlEntityDesc;
        }
    }
    if CUR(ctxt) == '&' as i32 {
        unsafe { xmlNextChar_safe(ctxt) };
        name = htmlParseName(ctxt);
        if name.is_null() {
            htmlParseErr(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"htmlParseEntityRef: no name\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            GROW(ctxt);
            if CUR(ctxt) == ';' as i32 {
                if !str.is_null() {
                    unsafe { *str = name }
                }
                /*
                 * Lookup the entity in the table.
                 */
                ent = htmlEntityLookup(name);
                if !ent.is_null() {
                    /* OK that's ugly !!! */
                    unsafe { xmlNextChar_safe(ctxt) };
                }
            } else {
                htmlParseErr(
                    ctxt,
                    XML_ERR_ENTITYREF_SEMICOL_MISSING,
                    b"htmlParseEntityRef: expecting \';\'\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                if !str.is_null() {
                    unsafe { *str = name }
                }
            }
        }
    }
    return ent;
}
/* *
 * htmlParseAttValue:
 * @ctxt:  an HTML parser context
 *
 * parse a value for an attribute
 * Note: the parser won't do substitution of entities here, this
 * will be handled later in xmlStringGetNodeList, unless it was * asked for ctxt->replaceEntities != 0
 *
 * Returns the AttValue parsed or NULL.
 */
fn htmlParseAttValue(ctxt: htmlParserCtxtPtr) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if CUR(ctxt) == '\"' as i32 {
        unsafe { xmlNextChar_safe(ctxt) };
        ret = htmlParseHTMLAttribute(ctxt, '\"' as xmlChar);
        if CUR(ctxt) != '\"' as i32 {
            htmlParseErr(
                ctxt,
                XML_ERR_ATTRIBUTE_NOT_FINISHED,
                b"AttValue: \" expected\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            unsafe { xmlNextChar_safe(ctxt) };
        }
    } else if CUR(ctxt) == '\'' as i32 {
        unsafe { xmlNextChar_safe(ctxt) };
        ret = htmlParseHTMLAttribute(ctxt, '\'' as xmlChar);
        if CUR(ctxt) != '\'' as i32 {
            htmlParseErr(
                ctxt,
                XML_ERR_ATTRIBUTE_NOT_FINISHED,
                b"AttValue: \' expected\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            unsafe { xmlNextChar_safe(ctxt) };
        }
    } else {
        /*
         * That's an HTMLism, the attribute value may not be quoted
         */
        ret = htmlParseHTMLAttribute(ctxt, 0 as xmlChar);
        if ret.is_null() {
            htmlParseErr(
                ctxt,
                XML_ERR_ATTRIBUTE_WITHOUT_VALUE,
                b"AttValue: no value found\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
    }
    return ret;
}
/* *
 * htmlParseSystemLiteral:
 * @ctxt:  an HTML parser context
 *
 * parse an HTML Literal
 *
 * [11] SystemLiteral ::= ('"' [^"]* '"') | ("'" [^']* "'")
 *
 * Returns the SystemLiteral parsed or NULL
 */
fn htmlParseSystemLiteral(ctxt: htmlParserCtxtPtr) -> *mut xmlChar {
    let mut len: size_t = 0;
    let mut startPosition: size_t = 0;
    let mut err: i32 = 0;
    let mut quote: i32;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if CUR(ctxt) != '\"' as i32 && CUR(ctxt) != '\'' as i32 {
        htmlParseErr(
            ctxt,
            XML_ERR_LITERAL_NOT_STARTED,
            b"SystemLiteral \" or \' expected\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return 0 as *mut xmlChar;
    }
    quote = CUR(ctxt);
    unsafe { xmlNextChar_safe(ctxt) };
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if inputPtr.cur < inputPtr.base {
        return ret;
    }
    unsafe {
        startPosition = inputPtr.cur.offset_from(inputPtr.base) as size_t;
    }
    while CUR(ctxt) != 0 && CUR(ctxt) != quote {
        /* TODO: Handle UTF-8 */
        if !IS_CHAR_CH(CUR(ctxt)) {
            htmlParseErrInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"Invalid char in SystemLiteral 0x%X\n\x00" as *const u8 as *const i8,
                CUR(ctxt),
            );
            err = 1
        }
        unsafe { xmlNextChar_safe(ctxt) };
        len = len.wrapping_add(1)
    }
    if CUR(ctxt) != quote {
        htmlParseErr(
            ctxt,
            XML_ERR_LITERAL_NOT_FINISHED,
            b"Unfinished SystemLiteral\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    } else {
        unsafe { xmlNextChar_safe(ctxt) };
        if err == 0 {
            unsafe {
                ret = xmlStrndup_safe(inputPtr.base.offset(startPosition as isize), len as i32)
            }
        }
    }
    return ret;
}
/* *
 * htmlParsePubidLiteral:
 * @ctxt:  an HTML parser context
 *
 * parse an HTML public literal
 *
 * [12] PubidLiteral ::= '"' PubidChar* '"' | "'" (PubidChar - "'")* "'"
 *
 * Returns the PubidLiteral parsed or NULL.
 */
fn htmlParsePubidLiteral(mut ctxt: htmlParserCtxtPtr) -> *mut xmlChar {
    let mut len: size_t = 0;
    let mut startPosition: size_t = 0;
    let mut err: i32 = 0;
    let mut quote: i32 = 0;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if CUR(ctxt) != '\"' as i32 && CUR(ctxt) != '\'' as i32 {
        htmlParseErr(
            ctxt,
            XML_ERR_LITERAL_NOT_STARTED,
            b"PubidLiteral \" or \' expected\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return 0 as *mut xmlChar;
    }
    quote = CUR(ctxt);
    unsafe { xmlNextChar_safe(ctxt) };
    /*
     * Name ::= (Letter | '_') (NameChar)*
     */
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if inputPtr.cur < inputPtr.base {
        return ret;
    }
    unsafe {
        startPosition = inputPtr.cur.offset_from(inputPtr.base) as size_t;
    }
    while CUR(ctxt) != 0 && CUR(ctxt) != quote {
        if unsafe { getXmlIsPubidChar_tab(CUR(ctxt) as usize) } == 0 {
            htmlParseErrInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"Invalid char in PubidLiteral 0x%X\n\x00" as *const u8 as *const i8,
                CUR(ctxt),
            );
            err = 1
        }
        len = len.wrapping_add(1);
        unsafe { xmlNextChar_safe(ctxt) };
    }
    if CUR(ctxt) != '\"' as i32 {
        htmlParseErr(
            ctxt,
            XML_ERR_LITERAL_NOT_FINISHED,
            b"Unfinished PubidLiteral\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    } else {
        unsafe { xmlNextChar_safe(ctxt) };
        if err == 0 {
            unsafe {
                ret = xmlStrndup_safe(inputPtr.base.offset(startPosition as isize), len as i32)
            }
        }
    }
    return ret;
}
/* *
 * htmlParseScript:
 * @ctxt:  an HTML parser context
 *
 * parse the content of an HTML SCRIPT or STYLE element
 * http://www.w3.org/TR/html4/sgml/dtd.html#Script
 * http://www.w3.org/TR/html4/sgml/dtd.html#StyleSheet
 * http://www.w3.org/TR/html4/types.html#type-script
 * http://www.w3.org/TR/html4/types.html#h-6.15
 * http://www.w3.org/TR/html4/appendix/notes.html#h-B.3.2.1
 *
 * Script data ( %Script; in the DTD) can be the content of the SCRIPT
 * element and the value of intrinsic event attributes. User agents must
 * not evaluate script data as HTML markup but instead must pass it on as * data to a script engine.
 * NOTES:
 * - The content is passed like CDATA
 * - the attributes for style and scripting "onXXX" are also described
 *   as CDATA but SGML allows entities references in attributes so their
 *   processing is identical as other attributes
 */
fn htmlParseScript(ctxt: htmlParserCtxtPtr) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    const BUF_SIZE: usize = (HTML_PARSER_BIG_BUFFER_SIZE + 5) as usize;
    let mut buf: [xmlChar; BUF_SIZE] = [0; BUF_SIZE];
    let mut nbchar: i32 = 0;
    let mut cur: i32;
    let mut l: i32 = 0;
    SHRINK(ctxt);
    cur = htmlCurrentChar(ctxt, &mut l);
    while cur != 0 {
        if cur == '<' as i32 && NXT(ctxt, 1) == '/' as i32 {
            /*
             * One should break here, the specification is clear:
             * Authors should therefore escape "</" within the content.
             * Escape mechanisms are specific to each scripting or
             * style sheet language.
             *
             * In recovery mode, only break if end tag match the
             * current tag, effectively ignoring all tags inside the
             * script/style block and treating the entire block as * CDATA.
             */
            if ctxtPtr.recovery != 0 {
                unsafe {
                    let mut inputPtr = &mut *(*ctxt).input;
                    if xmlStrncasecmp_safe(
                        ctxtPtr.name,
                        inputPtr.cur.offset(2),
                        xmlStrlen_safe(ctxtPtr.name),
                    ) == 0
                    {
                        break;
                        /* while */
                    } else {
                        htmlParseErr(
                            ctxt,
                            XML_ERR_TAG_NAME_MISMATCH,
                            b"Element %s embeds close tag\n\x00" as *const u8 as *const i8,
                            ctxtPtr.name,
                            0 as *const xmlChar,
                        );
                    }
                }
            } else if NXT(ctxt, 2) >= 'A' as i32 && NXT(ctxt, 2) <= 'Z' as i32
                || NXT(ctxt, 2) >= 'a' as i32 && NXT(ctxt, 2) <= 'z' as i32
            {
                break;
            }
        }
        unsafe {
            if IS_CHAR(cur) {
                nbchar = COPY_BUF(l, &mut *buf.as_mut_ptr(), nbchar, cur);
            } else {
                htmlParseErrInt(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"Invalid char in CDATA 0x%X\n\x00" as *const u8 as *const i8,
                    cur,
                );
            }
        }
        if nbchar >= HTML_PARSER_BIG_BUFFER_SIZE {
            buf[nbchar as usize] = 0;
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            if saxPtr.cdataBlock.is_some() {
                /*
                 * Insert as CDATA, which is the same as HTML_PRESERVE_NODE
                 */
                xmlSAXHandler_cdataBlock_safe(
                    saxPtr.cdataBlock,
                    ctxtPtr.userData,
                    buf.as_mut_ptr(),
                    nbchar,
                );
            } else if saxPtr.characters.is_some() {
                xmlSAXHandler_characters_safe(
                    saxPtr.characters,
                    ctxtPtr.userData,
                    buf.as_mut_ptr(),
                    nbchar,
                );
            }
            nbchar = 0
        }
        GROW(ctxt);
        NEXTL(ctxt, l);
        cur = htmlCurrentChar(ctxt, &mut l)
    }
    if nbchar != 0 && !ctxtPtr.sax.is_null() && ctxtPtr.disableSAX == 0 {
        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
        buf[nbchar as usize] = 0 as xmlChar;
        if saxPtr.cdataBlock.is_some() {
            /*
             * Insert as CDATA, which is the same as HTML_PRESERVE_NODE
             */
            xmlSAXHandler_cdataBlock_safe(
                saxPtr.cdataBlock,
                ctxtPtr.userData,
                buf.as_mut_ptr(),
                nbchar,
            );
        } else if saxPtr.characters.is_some() {
            xmlSAXHandler_characters_safe(
                saxPtr.characters,
                ctxtPtr.userData,
                buf.as_mut_ptr(),
                nbchar,
            );
        }
    };
}
/* *
 * htmlParseCharDataInternal:
 * @ctxt:  an HTML parser context
 * @readahead: optional read ahead character in ascii range
 *
 * parse a CharData section.
 * if we are within a CDATA section ']]>' marks an end of section.
 *
 * [14] CharData ::= [^<&]* - ([^<&]* ']]>' [^<&]*)
 */
fn htmlParseCharDataInternal(ctxt: htmlParserCtxtPtr, readahead: i32) {
    const BUF_SIZE: usize = (HTML_PARSER_BIG_BUFFER_SIZE + 6) as usize;
    let mut buf: [xmlChar; BUF_SIZE] = [0; BUF_SIZE];
    let mut nbchar: i32 = 0;
    let mut cur: i32;
    let mut l: i32 = 0;
    let mut chunk: i32 = 0;
    if readahead != 0 {
        let nbchar_old = nbchar;
        nbchar = nbchar + 1;
        buf[nbchar_old as usize] = readahead as xmlChar
    }
    SHRINK(ctxt);
    cur = htmlCurrentChar(ctxt, &mut l);
    let mut ctxtPtr = unsafe { &mut *ctxt };
    while (cur != '<' as i32 || ctxtPtr.token == '<' as i32)
        && (cur != '&' as i32 || ctxtPtr.token == '&' as i32)
        && cur != 0
    {
        unsafe {
            if !IS_CHAR(cur) {
                htmlParseErrInt(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"Invalid char in CDATA 0x%X\n\x00" as *const u8 as *const i8,
                    cur,
                );
            } else {
                let mut returnI: i32 = COPY_BUF(l, &mut *buf.as_mut_ptr(), nbchar, cur);
                nbchar = returnI;
            }
        }
        if nbchar >= HTML_PARSER_BIG_BUFFER_SIZE as i32 {
            buf[nbchar as usize] = 0;
            /*
             * Ok the segment is to be consumed as chars.
             */
            if !ctxtPtr.sax.is_null() && ctxtPtr.disableSAX == 0 {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                if areBlanks_htmlparser(ctxt, buf.as_mut_ptr(), nbchar) != 0 {
                    if ctxtPtr.keepBlanks != 0 {
                        if saxPtr.characters.is_some() {
                            xmlSAXHandler_characters_safe(
                                saxPtr.characters,
                                ctxtPtr.userData,
                                buf.as_mut_ptr(),
                                nbchar,
                            );
                        }
                    } else if saxPtr.ignorableWhitespace.is_some() {
                        xmlSAXHandler_ignorableWhitespace_safe(
                            saxPtr.ignorableWhitespace,
                            ctxtPtr.userData,
                            buf.as_mut_ptr(),
                            nbchar,
                        );
                    }
                } else {
                    htmlCheckParagraph(ctxt);
                    if saxPtr.characters.is_some() {
                        xmlSAXHandler_characters_safe(
                            saxPtr.characters,
                            ctxtPtr.userData,
                            buf.as_mut_ptr(),
                            nbchar,
                        );
                    }
                }
            }
            nbchar = 0
        }
        NEXTL(ctxt, l);
        chunk += 1;
        if chunk > HTML_PARSER_BUFFER_SIZE {
            chunk = 0;
            SHRINK(ctxt);
            GROW(ctxt);
        }
        cur = htmlCurrentChar(ctxt, &mut l);
        if cur == 0 {
            SHRINK(ctxt);
            GROW(ctxt);
            cur = htmlCurrentChar(ctxt, &mut l)
        }
    }
    if nbchar != 0 {
        buf[nbchar as usize] = 0 as xmlChar;
        /*
         * Ok the segment is to be consumed as chars.
         */
        if !ctxtPtr.sax.is_null() && ctxtPtr.disableSAX == 0 {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            if areBlanks_htmlparser(ctxt, buf.as_mut_ptr(), nbchar) != 0 {
                if ctxtPtr.keepBlanks != 0 {
                    if saxPtr.characters.is_some() {
                        xmlSAXHandler_characters_safe(
                            saxPtr.characters,
                            ctxtPtr.userData,
                            buf.as_mut_ptr(),
                            nbchar,
                        );
                    }
                } else if saxPtr.ignorableWhitespace.is_some() {
                    xmlSAXHandler_ignorableWhitespace_safe(
                        saxPtr.ignorableWhitespace,
                        ctxtPtr.userData,
                        buf.as_mut_ptr(),
                        nbchar,
                    );
                }
            } else {
                htmlCheckParagraph(ctxt);
                if saxPtr.characters.is_some() {
                    xmlSAXHandler_characters_safe(
                        saxPtr.characters,
                        ctxtPtr.userData,
                        buf.as_mut_ptr(),
                        nbchar,
                    );
                }
            }
        }
    } else {
        /*
         * Loop detection
         */
        if cur == 0 {
            ctxtPtr.instate = XML_PARSER_EOF
        };
    }
}
/* *
 * htmlParseCharData:
 * @ctxt:  an HTML parser context
 *
 * parse a CharData section.
 * if we are within a CDATA section ']]>' marks an end of section.
 *
 * [14] CharData ::= [^<&]* - ([^<&]* ']]>' [^<&]*)
 */
fn htmlParseCharData(ctxt: htmlParserCtxtPtr) {
    htmlParseCharDataInternal(ctxt, 0);
}
/* *
 * htmlParseExternalID:
 * @ctxt:  an HTML parser context
 * @publicID:  a xmlChar** receiving PubidLiteral
 *
 * Parse an External ID or a Public ID
 *
 * [75] ExternalID ::= 'SYSTEM' S SystemLiteral
 *                   | 'PUBLIC' S PubidLiteral S SystemLiteral
 *
 * [83] PublicID ::= 'PUBLIC' S PubidLiteral
 *
 * Returns the function returns SystemLiteral and in the second
 *                case publicID receives PubidLiteral, is strict is off
 *                it is possible to return NULL and have publicID set.
 */
fn htmlParseExternalID(ctxt: htmlParserCtxtPtr, publicID: *mut *mut xmlChar) -> *mut xmlChar {
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    if UPPER(ctxt) == 'S' as i32
        && UPP(ctxt, 1) == 'Y' as i32
        && UPP(ctxt, 2) == 'S' as i32
        && UPP(ctxt, 3) == 'T' as i32
        && UPP(ctxt, 4) == 'E' as i32
        && UPP(ctxt, 5) == 'M' as i32
    {
        SKIP(ctxt, 6);
        if !IS_BLANK_CH(CUR(ctxt)) {
            htmlParseErr(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after \'SYSTEM\'\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
        htmlSkipBlankChars(ctxt);
        URI = htmlParseSystemLiteral(ctxt);
        if URI.is_null() {
            htmlParseErr(
                ctxt,
                XML_ERR_URI_REQUIRED,
                b"htmlParseExternalID: SYSTEM, no URI\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
    } else if UPPER(ctxt) == 'P' as i32
        && UPP(ctxt, 1) == 'U' as i32
        && UPP(ctxt, 2) == 'B' as i32
        && UPP(ctxt, 3) == 'L' as i32
        && UPP(ctxt, 4) == 'I' as i32
        && UPP(ctxt, 5) == 'C' as i32
    {
        SKIP(ctxt, 6);
        if !IS_BLANK_CH(CUR(ctxt)) {
            htmlParseErr(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after \'PUBLIC\'\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
        htmlSkipBlankChars(ctxt);
        unsafe {
            *publicID = htmlParsePubidLiteral(ctxt);
        }
        unsafe {
            if (*publicID).is_null() {
                htmlParseErr(
                    ctxt,
                    XML_ERR_PUBID_REQUIRED,
                    b"htmlParseExternalID: PUBLIC, no Public Identifier\n\x00" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
        }
        htmlSkipBlankChars(ctxt);
        if CUR(ctxt) == '\"' as i32 || CUR(ctxt) == '\'' as i32 {
            URI = htmlParseSystemLiteral(ctxt)
        }
    }
    return URI;
}
/* *
 * xmlParsePI:
 * @ctxt:  an XML parser context
 *
 * parse an XML Processing Instruction.
 *
 * [16] PI ::= '<?' PITarget (S (Char* - (Char* '?>' Char*)))? '?>'
 */
fn htmlParsePI(ctxt: htmlParserCtxtPtr) {
    let ctxtPtr = unsafe { &mut *ctxt };
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: i32 = 0;
    let mut size: i32 = 100;
    let mut cur: i32;
    let mut l: i32 = 0;
    let target: *const xmlChar;
    let mut state: xmlParserInputState = XML_PARSER_START;
    let mut count: i32 = 0;
    let mut sax_condition = false;
    if RAW(ctxt) == '<' as i32 && NXT(ctxt, 1) == '?' as i32 {
        state = ctxtPtr.instate;
        ctxtPtr.instate = XML_PARSER_PI;
        /*
         * this is a Processing Instruction.
         */
        SKIP(ctxt, 2);
        SHRINK(ctxt);
        /*
         * Parse the target name and check for special support like
         * namespace.
         */
        target = htmlParseName(ctxt);
        if !target.is_null() {
            if RAW(ctxt) == '>' as i32 {
                SKIP(ctxt, 1);
                /*
                 * SAX: PI detected.
                 */
                sax_condition = unsafe {
                    !ctxtPtr.sax.is_null()
                        && ctxtPtr.disableSAX == 0
                        && (*(*ctxt).sax).processingInstruction.is_some()
                };
                if sax_condition {
                    let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                    xmlSAXHandler_processingInstruction_safe(
                        saxPtr.processingInstruction,
                        ctxtPtr.userData,
                        target,
                        0 as *const xmlChar,
                    );
                }
                ctxtPtr.instate = state;
                return;
            }
            buf = unsafe {
                xmlMallocAtomic_safe(
                    (size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                )
            } as *mut xmlChar;
            if buf.is_null() {
                htmlErrMemory(ctxt, 0 as *const i8);
                ctxtPtr.instate = state;
                return;
            }
            cur = CUR(ctxt);
            if !IS_BLANK(cur) {
                htmlParseErr(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"ParsePI: PI %s space expected\n\x00" as *const u8 as *const i8,
                    target,
                    0 as *const xmlChar,
                );
            }
            htmlSkipBlankChars(ctxt);
            cur = htmlCurrentChar(ctxt, &mut l);
            while cur != 0 && cur != '>' as i32 {
                if len + 5 >= size {
                    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                    size *= 2;
                    tmp = unsafe {
                        xmlRealloc_safe(
                            buf as *mut (),
                            (size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                        )
                    } as *mut xmlChar;
                    if tmp.is_null() {
                        htmlErrMemory(ctxt, 0 as *const i8);
                        unsafe { xmlFree_safe(buf as *mut ()) };
                        ctxtPtr.instate = state;
                        return;
                    }
                    buf = tmp
                }
                count += 1;
                if count > 50 {
                    GROW(ctxt);
                    count = 0
                }
                if IS_CHAR(cur) {
                    let mut returnI: i32 = COPY_BUF(l, buf, len, cur);
                    len = returnI;
                } else {
                    htmlParseErrInt(
                        ctxt,
                        XML_ERR_INVALID_CHAR,
                        b"Invalid char in processing instruction 0x%X\n\x00" as *const u8
                            as *const i8,
                        cur,
                    );
                }
                NEXTL(ctxt, l);
                cur = htmlCurrentChar(ctxt, &mut l);
                if cur == 0 {
                    SHRINK(ctxt);
                    GROW(ctxt);
                    cur = htmlCurrentChar(ctxt, &mut l)
                }
            }
            unsafe {
                *buf.offset(len as isize) = 0 as xmlChar;
            }
            if cur != '>' as i32 {
                htmlParseErr(
                    ctxt,
                    XML_ERR_PI_NOT_FINISHED,
                    b"ParsePI: PI %s never end ...\n\x00" as *const u8 as *const i8,
                    target,
                    0 as *const xmlChar,
                );
            } else {
                SKIP(ctxt, 1);
                /*
                 * SAX: PI detected.
                 */
                sax_condition = unsafe {
                    !ctxtPtr.sax.is_null()
                        && ctxtPtr.disableSAX == 0
                        && (*(*ctxt).sax).processingInstruction.is_some()
                };
                if sax_condition {
                    let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                    xmlSAXHandler_processingInstruction_safe(
                        saxPtr.processingInstruction,
                        ctxtPtr.userData,
                        target,
                        buf,
                    );
                }
            }
            unsafe { xmlFree_safe(buf as *mut ()) };
        } else {
            htmlParseErr(
                ctxt,
                XML_ERR_PI_NOT_STARTED,
                b"PI is not started correctly\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
        ctxtPtr.instate = state
    };
}
/* *
 * htmlParseComment:
 * @ctxt:  an HTML parser context
 *
 * Parse an XML (SGML) comment <!-- .... -->
 *
 * [15] Comment ::= '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
 */
fn htmlParseComment(ctxt: htmlParserCtxtPtr) {
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: i32;
    let mut size: i32 = 100;
    let mut q: i32;
    let mut ql: i32 = 0;
    let mut r: i32;
    let mut rl: i32 = 0;
    let mut cur: i32;
    let mut l: i32 = 0;
    let mut next: i32;
    let mut nl: i32 = 0;
    let mut state: xmlParserInputState = XML_PARSER_START;
    /*
     * Check that there is a comment right here.
     */
    if (RAW(ctxt) != '<' as i32)
        || (NXT(ctxt, 1) != '!' as i32)
        || (NXT(ctxt, 2) != '-' as i32)
        || (NXT(ctxt, 3) != '-' as i32)
    {
        return;
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    state = ctxtPtr.instate;
    ctxtPtr.instate = XML_PARSER_COMMENT;
    SHRINK(ctxt);
    SKIP(ctxt, 4);
    buf = unsafe {
        xmlMallocAtomic_safe((size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64))
    } as *mut xmlChar;
    if buf.is_null() {
        htmlErrMemory(
            ctxt,
            b"buffer allocation failed\n\x00" as *const u8 as *const i8,
        );
        ctxtPtr.instate = state;
        return;
    }
    len = 0;
    unsafe {
        *buf.offset(len as isize) = 0 as xmlChar;
    }
    q = htmlCurrentChar(ctxt, &mut ql);
    if q == 0 {
        htmlParseErr(
            ctxt,
            XML_ERR_COMMENT_NOT_FINISHED,
            b"Comment not terminated \n<!--%.50s\n\x00" as *const u8 as *const i8,
            buf,
            0 as *const xmlChar,
        );
        unsafe { xmlFree_safe(buf as *mut ()) };
        return;
    }
    NEXTL(ctxt, ql);
    r = htmlCurrentChar(ctxt, &mut rl);
    if r == 0 {
        htmlParseErr(
            ctxt,
            XML_ERR_COMMENT_NOT_FINISHED,
            b"Comment not terminated \n<!--%.50s\n\x00" as *const u8 as *const i8,
            buf,
            0 as *const xmlChar,
        );
        unsafe { xmlFree_safe(buf as *mut ()) };
        return;
    }
    NEXTL(ctxt, rl);
    cur = htmlCurrentChar(ctxt, &mut l);
    while (cur != 0) && ((cur != '>' as i32) || (r != '-' as i32) || (q != '-' as i32)) {
        NEXTL(ctxt, l);
        next = htmlCurrentChar(ctxt, &mut nl);
        if next == 0 {
            SHRINK(ctxt);
            GROW(ctxt);
            next = htmlCurrentChar(ctxt, &mut nl);
        }
        if (q == '-' as i32) && (r == '-' as i32) && (cur == '!' as i32) && (next == '>' as i32) {
            htmlParseErr(
                ctxt,
                XML_ERR_COMMENT_NOT_FINISHED,
                b"Comment incorrectly closed by '--!>'\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            cur = '>' as i32;
            break;
        }
        if (len + 5) >= size {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            size *= 2;
            tmp = unsafe {
                xmlRealloc_safe(
                    buf as *mut (),
                    (size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                )
            } as *mut xmlChar;
            if tmp.is_null() {
                unsafe { xmlFree_safe(buf as *mut ()) };
                htmlErrMemory(
                    ctxt,
                    b"growing buffer failed\n\x00" as *const u8 as *const i8,
                );
                ctxtPtr.instate = state;
                return;
            }
            buf = tmp;
        }
        if IS_CHAR(q) {
            let mut returnI: i32 = COPY_BUF(ql, buf, len, q);
            len = returnI;
        } else {
            htmlParseErrInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"Invalid char in comment 0x%X\n\x00" as *const u8 as *const i8,
                q,
            );
        }
        q = r;
        ql = rl;
        r = cur;
        rl = l;
        cur = next;
        l = nl;
    }
    unsafe {
        *buf.offset(len as isize) = 0;
    }
    if cur == '>' as i32 {
        unsafe { xmlNextChar_safe(ctxt) };
        let mut sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).comment.is_some() && ctxtPtr.disableSAX == 0
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_comment_safe(saxPtr.comment, ctxtPtr.userData, buf);
        }
        unsafe { xmlFree_safe(buf as *mut ()) };
        ctxtPtr.instate = state;
        return;
    }
    htmlParseErr(
        ctxt,
        XML_ERR_COMMENT_NOT_FINISHED,
        b"Comment not terminated \n<!--%.50s\n\x00" as *const u8 as *const i8,
        buf,
        0 as *const xmlChar,
    );
    unsafe { xmlFree_safe(buf as *mut ()) };
}
/* *
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

pub fn htmlParseCharRef(ctxt: htmlParserCtxtPtr) -> i32 {
    let mut val: i32 = 0;
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"htmlParseCharRef: context error\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            return 0;
        }
    }
    if CUR(ctxt) == '&' as i32
        && NXT(ctxt, 1) == '#' as i32
        && (NXT(ctxt, 2) == 'x' as i32 || NXT(ctxt, 2) == 'X' as i32)
    {
        SKIP(ctxt, 3);
        while CUR(ctxt) != ';' as i32 {
            if CUR(ctxt) >= '0' as i32 && CUR(ctxt) <= '9' as i32 {
                if val < 0x110000 {
                    val = val * 16 + (CUR(ctxt) - '0' as i32)
                }
            } else if CUR(ctxt) >= 'a' as i32 && CUR(ctxt) <= 'f' as i32 {
                if val < 0x110000 {
                    val = val * 16 + (CUR(ctxt) - 'a' as i32) + 10
                }
            } else if CUR(ctxt) >= 'A' as i32 && CUR(ctxt) <= 'F' as i32 {
                if val < 0x110000 {
                    val = val * 16 + (CUR(ctxt) - 'A' as i32) + 10
                }
            } else {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INVALID_HEX_CHARREF,
                    b"htmlParseCharRef: missing semicolon\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                break;
            }
            unsafe { xmlNextChar_safe(ctxt) };
        }
        if CUR(ctxt) == ';' as i32 {
            unsafe { xmlNextChar_safe(ctxt) };
        }
    } else if CUR(ctxt) == '&' as i32 && NXT(ctxt, 1) == '#' as i32 {
        SKIP(ctxt, 2);
        while CUR(ctxt) != ';' as i32 {
            if CUR(ctxt) >= '0' as i32 && CUR(ctxt) <= '9' as i32 {
                if val < 0x110000 {
                    val = val * 10 + (CUR(ctxt) - '0' as i32)
                }
            } else {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INVALID_DEC_CHARREF,
                    b"htmlParseCharRef: missing semicolon\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                break;
            }
            unsafe { xmlNextChar_safe(ctxt) };
        }
        if CUR(ctxt) == ';' as i32 {
            unsafe { xmlNextChar_safe(ctxt) };
        }
    } else {
        htmlParseErr(
            ctxt,
            XML_ERR_INVALID_CHARREF,
            b"htmlParseCharRef: invalid value\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    }
    /*
     * Check the value IS_CHAR ...
     */
    if IS_CHAR(val) {
        return val;
    } else {
        if val >= 0x110000 {
            htmlParseErr(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"htmlParseCharRef: value too large\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            htmlParseErrInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"htmlParseCharRef: invalid xmlChar value %d\n\x00" as *const u8 as *const i8,
                val,
            );
        }
    }
    return 0;
}
/* *
 * htmlParseDocTypeDecl:
 * @ctxt:  an HTML parser context
 *
 * parse a DOCTYPE declaration
 *
 * [28] doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S?
 *                      ('[' (markupdecl | PEReference | S)* ']' S?)? '>'
 */
fn htmlParseDocTypeDecl(ctxt: htmlParserCtxtPtr) {
    let name: *const xmlChar;
    let mut ExternalID: *mut xmlChar = 0 as *mut xmlChar;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    /*
     * We know that '<!DOCTYPE' has been detected.
     */
    SKIP(ctxt, 9);
    htmlSkipBlankChars(ctxt);
    /*
     * Parse the DOCTYPE name.
     */
    name = htmlParseName(ctxt);
    if name.is_null() {
        htmlParseErr(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"htmlParseDocTypeDecl : no DOCTYPE name !\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    }
    /*
     * Check that upper(name) == "HTML" !!!!!!!!!!!!!
     */
    htmlSkipBlankChars(ctxt);
    /*
     * Check for SystemID and ExternalID
     */
    URI = htmlParseExternalID(ctxt, &mut ExternalID);
    htmlSkipBlankChars(ctxt);
    /*
     * We should be at the end of the DOCTYPE declaration.
     */
    if CUR(ctxt) != '>' as i32 {
        htmlParseErr(
            ctxt,
            XML_ERR_DOCTYPE_NOT_FINISHED,
            b"DOCTYPE improperly terminated\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        /* Ignore bogus content */
        while CUR(ctxt) != 0 && CUR(ctxt) != '>' as i32 {
            unsafe { xmlNextChar_safe(ctxt) };
        }
    }
    if CUR(ctxt) == '>' as i32 {
        unsafe { xmlNextChar_safe(ctxt) };
    }
    /*
     * Create or update the document accordingly to the DOCTYPE
     */
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut sax_condition = unsafe {
        !ctxtPtr.sax.is_null() && (*(*ctxt).sax).internalSubset.is_some() && ctxtPtr.disableSAX == 0
    };
    if sax_condition {
        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
        xmlSAXHandler_internalSubset_safe(
            saxPtr.internalSubset,
            ctxtPtr.userData,
            name,
            ExternalID,
            URI,
        );
    }
    /*
     * Cleanup, since we don't use all those identifiers
     */
    if !URI.is_null() {
        unsafe { xmlFree_safe(URI as *mut ()) };
    }
    if !ExternalID.is_null() {
        unsafe { xmlFree_safe(ExternalID as *mut ()) };
    };
}
/* *
 * htmlParseAttribute:
 * @ctxt:  an HTML parser context
 * @value:  a xmlChar ** used to store the value of the attribute
 *
 * parse an attribute
 *
 * [41] Attribute ::= Name Eq AttValue
 *
 * [25] Eq ::= S? '=' S?
 *
 * With namespace:
 *
 * [NS 11] Attribute ::= QName Eq AttValue
 *
 * Also the case QName == xmlns:??? is handled independently as a namespace
 * definition.
 *
 * Returns the attribute name, and the value in *value.
 */
fn htmlParseAttribute(ctxt: htmlParserCtxtPtr, value: *mut *mut xmlChar) -> *const xmlChar {
    let name: *const xmlChar;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    unsafe {
        *value = 0 as *mut xmlChar;
    }
    name = htmlParseHTMLName(ctxt);
    if name.is_null() {
        htmlParseErr(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"error parsing attribute name\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return 0 as *const xmlChar;
    }
    /*
     * read the value
     */
    htmlSkipBlankChars(ctxt);
    if CUR(ctxt) == '=' as i32 {
        unsafe { xmlNextChar_safe(ctxt) };
        htmlSkipBlankChars(ctxt);
        val = htmlParseAttValue(ctxt)
    }
    unsafe {
        *value = val;
    }
    return name;
}
/* *
 * htmlCheckEncodingDirect:
 * @ctxt:  an HTML parser context
 * @attvalue: the attribute value
 *
 * Checks an attribute value to detect
 * the encoding
 * If a new encoding is detected the parser is switched to decode
 * it and pass UTF8
 */
fn htmlCheckEncodingDirect(ctxt: htmlParserCtxtPtr, mut encoding: *const xmlChar) {
    unsafe {
        if ctxt.is_null()
            || encoding.is_null()
            || (*ctxt).options & HTML_PARSE_IGNORE_ENC as i32 != 0
        {
            return;
        }
    }
    let mut buf_condition = false;
    let mut ctxtPtr = unsafe { &mut *ctxt };
    /* do not change encoding */
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if !inputPtr.encoding.is_null() {
        return;
    }
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        let mut handler: xmlCharEncodingHandlerPtr;
        let mut encoding_safe = unsafe { *encoding };
        while 1 < 2 {
            if !(encoding_safe == ' ' as u8 || encoding_safe == '\t' as u8) {
                break;
            }
            unsafe { encoding = encoding.offset(1) }
        }
        if !inputPtr.encoding.is_null() {
            unsafe { xmlFree_safe(inputPtr.encoding as *mut xmlChar as *mut ()) };
        }
        inputPtr.encoding = unsafe { xmlStrdup_safe(encoding) };
        enc = unsafe { xmlParseCharEncoding_safe(encoding as *const i8) };
        /*
         * registered set of known encodings
         */
        if enc != XML_CHAR_ENCODING_ERROR {
            buf_condition =
                unsafe { !inputPtr.buf.is_null() && (*(*(*ctxt).input).buf).encoder.is_null() };
            if (enc == XML_CHAR_ENCODING_UTF16LE
                || enc == XML_CHAR_ENCODING_UTF16BE
                || enc == XML_CHAR_ENCODING_UCS4LE
                || enc == XML_CHAR_ENCODING_UCS4BE)
                && buf_condition
            {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INVALID_ENCODING,
                    b"htmlCheckEncoding: wrong encoding meta\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            } else {
                unsafe { xmlSwitchEncoding_safe(ctxt, enc) };
            }
            ctxtPtr.charset = XML_CHAR_ENCODING_UTF8
        } else {
            /*
             * fallback for unknown encodings
             */
            handler = unsafe { xmlFindCharEncodingHandler_safe(encoding as *const i8) };
            if !handler.is_null() {
                unsafe { xmlSwitchToEncoding_safe(ctxt, handler) };
                ctxtPtr.charset = XML_CHAR_ENCODING_UTF8
            } else {
                htmlParseErr(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"htmlCheckEncoding: unknown encoding %s\n\x00" as *const u8 as *const i8,
                    encoding,
                    0 as *const xmlChar,
                );
            }
        }
        buf_condition = unsafe {
            !inputPtr.buf.is_null()
                && !(*(*(*ctxt).input).buf).encoder.is_null()
                && !(*(*(*ctxt).input).buf).raw.is_null()
                && !(*(*(*ctxt).input).buf).buffer.is_null()
        };
        if buf_condition {
            let mut bufPtr = unsafe { &mut *(*(*ctxt).input).buf };
            let mut nbchars: i32 = 0;
            let mut processed: i32 = 0;
            /*
             * convert as much as possible to the parser reading buffer.
             */
            unsafe {
                processed = inputPtr.cur.offset_from(inputPtr.base) as i32;
            }
            unsafe { xmlBufShrink_safe(bufPtr.buffer, processed as size_t) };
            nbchars = unsafe { xmlCharEncInput_safe(inputPtr.buf, 1) };
            unsafe { xmlBufResetInput_safe(bufPtr.buffer, ctxtPtr.input) };
            if nbchars < 0 {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INVALID_ENCODING,
                    b"htmlCheckEncoding: encoder error\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
        }
    };
}
/* *
 * htmlCheckEncoding:
 * @ctxt:  an HTML parser context
 * @attvalue: the attribute value
 *
 * Checks an http-equiv attribute from a Meta tag to detect
 * the encoding
 * If a new encoding is detected the parser is switched to decode
 * it and pass UTF8
 */
fn htmlCheckEncoding(ctxt: htmlParserCtxtPtr, attvalue: *const xmlChar) {
    let mut encoding: *const xmlChar = 0 as *const xmlChar;
    if attvalue.is_null() {
        return;
    }
    encoding = unsafe {
        xmlStrcasestr_safe(
            attvalue,
            b"charset\x00" as *const u8 as *const i8 as *mut xmlChar,
        )
    };
    if !encoding.is_null() {
        unsafe { encoding = encoding.offset(7) }
    }
    /*
     * skip blank
     */
    unsafe {
        if !encoding.is_null() && IS_BLANK_CH((*encoding) as i32) {
            encoding =
                xmlStrcasestr_safe(attvalue, b"=\x00" as *const u8 as *const i8 as *mut xmlChar)
        }
    }

    unsafe {
        if !encoding.is_null() && *encoding == '=' as u8 {
            encoding = encoding.offset(1);
            htmlCheckEncodingDirect(ctxt, encoding);
        };
    }
}
/* *
 * htmlCheckMeta:
 * @ctxt:  an HTML parser context
 * @atts:  the attributes values
 *
 * Checks an attributes from a Meta tag
 */
fn htmlCheckMeta(mut ctxt: htmlParserCtxtPtr, mut atts: *mut *const xmlChar) {
    let mut i: i32 = 0;
    let mut att: *const xmlChar = 0 as *const xmlChar;
    let mut value: *const xmlChar = 0 as *const xmlChar;
    let mut http: i32 = 0;
    let mut content: *const xmlChar = 0 as *const xmlChar;
    if ctxt.is_null() || atts.is_null() {
        return;
    }
    i = 0;
    unsafe {
        att = *atts.offset(i as isize);
    }
    i = i + 1;
    while !att.is_null() {
        unsafe {
            value = *atts.offset(i as isize);
        }
        i = i + 1;
        if !value.is_null()
            && unsafe {
                xmlStrcasecmp_safe(
                    att,
                    b"http-equiv\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } == 0
            && unsafe {
                xmlStrcasecmp_safe(
                    value,
                    b"Content-Type\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } == 0
        {
            http = 1
        } else if !value.is_null()
            && unsafe {
                xmlStrcasecmp_safe(
                    att,
                    b"charset\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } == 0
        {
            htmlCheckEncodingDirect(ctxt, value);
        } else if !value.is_null()
            && unsafe {
                xmlStrcasecmp_safe(
                    att,
                    b"content\x00" as *const u8 as *const i8 as *mut xmlChar,
                )
            } == 0
        {
            content = value
        }
        unsafe { att = *atts.offset(i as isize) }
        i = i + 1;
    }
    if http != 0 && !content.is_null() {
        htmlCheckEncoding(ctxt, content);
    };
}
/* *
 * htmlParseStartTag:
 * @ctxt:  an HTML parser context
 *
 * parse a start of tag either for rule element or
 * EmptyElement. In both case we don't parse the tag closing chars.
 *
 * [40] STag ::= '<' Name (S Attribute)* S? '>'
 *
 * [44] EmptyElemTag ::= '<' Name (S Attribute)* S? '/>'
 *
 * With namespace:
 *
 * [NS 8] STag ::= '<' QName (S Attribute)* S? '>'
 *
 * [NS 10] EmptyElement ::= '<' QName (S Attribute)* S? '/>'
 *
 * Returns 0 in case of success, -1 in case of error and 1 if discarded
 */
fn htmlParseStartTag(ctxt: htmlParserCtxtPtr) -> i32 {
    let mut current_block: i32;
    let name: *const xmlChar;
    let mut attname: *const xmlChar;
    let mut attvalue: *mut xmlChar = 0 as *mut xmlChar;
    let mut atts: *mut *const xmlChar;
    let mut nbatts: i32 = 0;
    let mut maxatts: i32;
    let mut meta: i32 = 0;
    let mut i: i32;
    let mut discardtag: i32 = 0;
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"htmlParseStartTag: context error\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            return -1;
        }
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.instate == XML_PARSER_EOF {
        return -1;
    }
    if CUR(ctxt) != '<' as i32 {
        return -1;
    }
    unsafe { xmlNextChar_safe(ctxt) };
    atts = ctxtPtr.atts;
    maxatts = ctxtPtr.maxatts;
    GROW(ctxt);
    name = htmlParseHTMLName(ctxt);
    if name.is_null() {
        htmlParseErr(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"htmlParseStartTag: invalid element name\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        /* if recover preserve text on classic misconstructs */
        let cur_safe = unsafe { *(*(*ctxt).input).cur };
        if ctxtPtr.recovery != 0
            && (IS_BLANK_CH(cur_safe as i32)
                || CUR(ctxt) == '<' as i32
                || CUR(ctxt) == '=' as i32
                || CUR(ctxt) == '>' as i32
                || (CUR(ctxt) >= '0' as i32 && CUR(ctxt) <= '9' as i32))
        {
            htmlParseCharDataInternal(ctxt, '<' as i32);
            return -1;
        }
        /* Dump the bogus tag like browsers do */
        while CUR(ctxt) != 0 && CUR(ctxt) != '>' as i32 && ctxtPtr.instate != XML_PARSER_EOF {
            unsafe { xmlNextChar_safe(ctxt) };
        }
        return -1;
    }
    if unsafe { xmlStrEqual_safe(name, b"meta\x00" as *const u8 as *const i8 as *mut xmlChar) } != 0
    {
        meta = 1
    }
    /*
     * Check for auto-closure of HTML elements.
     */
    htmlAutoClose(ctxt, name);
    /*
     * Check for implied HTML elements.
     */
    htmlCheckImplied(ctxt, name);
    /*
     * Avoid html at any level > 0, head at any level != 1
     * or any attempt to recurse body
     */
    if ctxtPtr.nameNr > 0
        && unsafe { xmlStrEqual_safe(name, b"html\x00" as *const u8 as *const i8 as *mut xmlChar) }
            != 0
    {
        htmlParseErr(
            ctxt,
            XML_HTML_STRUCURE_ERROR,
            b"htmlParseStartTag: misplaced <html> tag\n\x00" as *const u8 as *const i8,
            name,
            0 as *const xmlChar,
        );
        discardtag = 1;
        ctxtPtr.depth += 1
    }
    if ctxtPtr.nameNr != 1
        && unsafe { xmlStrEqual_safe(name, b"head\x00" as *const u8 as *const i8 as *mut xmlChar) }
            != 0
    {
        htmlParseErr(
            ctxt,
            XML_HTML_STRUCURE_ERROR,
            b"htmlParseStartTag: misplaced <head> tag\n\x00" as *const u8 as *const i8,
            name,
            0 as *const xmlChar,
        );
        discardtag = 1;
        ctxtPtr.depth += 1
    }
    if unsafe { xmlStrEqual_safe(name, b"body\x00" as *const u8 as *const i8 as *mut xmlChar) } != 0
    {
        let mut indx: i32;
        indx = 0;
        while indx < ctxtPtr.nameNr {
            unsafe {
                if xmlStrEqual_safe(
                    *ctxtPtr.nameTab.offset(indx as isize),
                    b"body\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                    htmlParseErr(
                        ctxt,
                        XML_HTML_STRUCURE_ERROR,
                        b"htmlParseStartTag: misplaced <body> tag\n\x00" as *const u8 as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                    discardtag = 1;
                    ctxtPtr.depth += 1
                }
            }
            indx += 1
        }
    }
    /*
     * Now parse the attributes, it ends up with the ending
     *
     * (S Attribute)* S?
     */
    htmlSkipBlankChars(ctxt);
    while CUR(ctxt) != 0
        && CUR(ctxt) != '>' as i32
        && (CUR(ctxt) != '/' as i32 || NXT(ctxt, 1) != '>' as i32)
    {
        GROW(ctxt);
        attname = htmlParseAttribute(ctxt, &mut attvalue);
        if !attname.is_null() {
            /*
             * Well formedness requires at most one declaration of an attribute
             */
            i = 0;
            loop {
                if !(i < nbatts) {
                    current_block = 1;
                    break;
                }
                unsafe {
                    if xmlStrEqual_safe(*atts.offset(i as isize), attname) != 0 {
                        htmlParseErr(
                            ctxt,
                            XML_ERR_ATTRIBUTE_REDEFINED,
                            b"Attribute %s redefined\n\x00" as *const u8 as *const i8,
                            attname,
                            0 as *const xmlChar,
                        );
                        if !attvalue.is_null() {
                            xmlFree_safe(attvalue as *mut ());
                        }
                        current_block = 2;
                        break;
                    } else {
                        i += 2
                    }
                }
            }
            match current_block {
                2 => {}
                _ =>
                /*
                 * Add the pair to atts
                 */
                {
                    if atts.is_null() {
                        maxatts = 22; /* allow for 10 attrs by default */
                        atts = unsafe {
                            xmlMalloc_safe((maxatts as u64).wrapping_mul(::std::mem::size_of::<
                                *mut xmlChar,
                            >(
                            )
                                as u64))
                        } as *mut *const xmlChar;
                        if atts.is_null() {
                            htmlErrMemory(ctxt, 0 as *const i8);
                            if !attvalue.is_null() {
                                unsafe { xmlFree_safe(attvalue as *mut ()) };
                            }
                            current_block = 2;
                        } else {
                            ctxtPtr.atts = atts;
                            ctxtPtr.maxatts = maxatts;
                            current_block = 3;
                        }
                    } else if nbatts + 4 > maxatts {
                        let mut n: *mut *const xmlChar = 0 as *mut *const xmlChar;
                        maxatts *= 2;
                        n = unsafe {
                            xmlRealloc_safe(
                                atts as *mut (),
                                (maxatts as u64)
                                    .wrapping_mul(::std::mem::size_of::<*const xmlChar>() as u64),
                            )
                        } as *mut *const xmlChar;
                        if n.is_null() {
                            htmlErrMemory(ctxt, 0 as *const i8);
                            if !attvalue.is_null() {
                                unsafe { xmlFree_safe(attvalue as *mut ()) };
                            }
                            current_block = 2;
                        } else {
                            atts = n;
                            ctxtPtr.atts = atts;
                            ctxtPtr.maxatts = maxatts;
                            current_block = 3;
                        }
                    } else {
                        current_block = 3;
                    }
                    unsafe {
                        match current_block {
                            2 => {}
                            _ => {
                                *atts.offset(nbatts as isize) = attname;
                                nbatts = nbatts + 1;
                                *atts.offset(nbatts as isize) = attvalue;
                                nbatts = nbatts + 1;
                                *atts.offset(nbatts as isize) = 0 as *const xmlChar;
                                *atts.offset((nbatts + 1) as isize) = 0 as *const xmlChar
                            }
                        }
                    }
                }
            }
        } else {
            if !attvalue.is_null() {
                unsafe { xmlFree_safe(attvalue as *mut ()) };
            }
            /* Dump the bogus attribute string up to the next blank or
             * the end of the tag. */
            while CUR(ctxt) != 0
                && !(IS_BLANK_CH(CUR(ctxt)))
                && CUR(ctxt) != '>' as i32
                && (CUR(ctxt) != '/' as i32 || NXT(ctxt, 1) != '>' as i32)
            {
                unsafe { xmlNextChar_safe(ctxt) };
            }
        }
        htmlSkipBlankChars(ctxt);
    }
    /*
     * Handle specific association to the META tag
     */
    if meta != 0 && nbatts != 0 {
        htmlCheckMeta(ctxt, atts);
    }
    /*
     * SAX: Start of Element !
     */
    if discardtag == 0 {
        htmlnamePush(ctxt, name);
        let mut sax_condition =
            unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            if nbatts != 0 {
                xmlSAXHandler_startElement_safe(saxPtr.startElement, ctxtPtr.userData, name, atts);
            } else {
                xmlSAXHandler_startElement_safe(
                    saxPtr.startElement,
                    ctxtPtr.userData,
                    name,
                    0 as *mut *const xmlChar,
                );
            }
        }
    }
    if !atts.is_null() {
        i = 1;
        while i < nbatts {
            unsafe {
                if !(*atts.offset(i as isize)).is_null() {
                    xmlFree_safe(*atts.offset(i as isize) as *mut xmlChar as *mut ());
                }
            }
            i += 2
        }
    }
    return discardtag;
}
/* *
 * htmlParseEndTag:
 * @ctxt:  an HTML parser context
 *
 * parse an end of tag
 *
 * [42] ETag ::= '</' Name S? '>'
 *
 * With namespace
 *
 * [NS 9] ETag ::= '</' QName S? '>'
 *
 * Returns 1 if the current level should be closed.
 */
fn htmlParseEndTag(ctxt: htmlParserCtxtPtr) -> i32 {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut oldname: *const xmlChar = 0 as *const xmlChar;
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    if CUR(ctxt) != '<' as i32 || NXT(ctxt, 1) != '/' as i32 {
        htmlParseErr(
            ctxt,
            XML_ERR_LTSLASH_REQUIRED,
            b"htmlParseEndTag: \'</\' not found\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return 0;
    }
    SKIP(ctxt, 2);
    name = htmlParseHTMLName(ctxt);
    if name.is_null() {
        return 0;
    }
    /*
     * We should definitely be at the ending "S? '>'" part
     */
    htmlSkipBlankChars(ctxt);
    if CUR(ctxt) != '>' as i32 {
        htmlParseErr(
            ctxt,
            XML_ERR_GT_REQUIRED,
            b"End tag : expected \'>\'\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        /* Skip to next '>' */
        while CUR(ctxt) != 0 && CUR(ctxt) != '>' as i32 {
            unsafe { xmlNextChar_safe(ctxt) };
        }
    }
    if CUR(ctxt) == '>' as i32 {
        unsafe { xmlNextChar_safe(ctxt) };
    }
    /*
     * if we ignored misplaced tags in htmlParseStartTag don't pop them
     * out now.
     */
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.depth > 0
        && (unsafe {
            xmlStrEqual_safe(name, b"html\x00" as *const u8 as *const i8 as *mut xmlChar)
        } != 0
            || unsafe {
                xmlStrEqual_safe(name, b"body\x00" as *const u8 as *const i8 as *mut xmlChar)
            } != 0
            || unsafe {
                xmlStrEqual_safe(name, b"head\x00" as *const u8 as *const i8 as *mut xmlChar)
            } != 0)
    {
        ctxtPtr.depth -= 1;
        return 0;
    }
    /*
     * If the name read is not one of the element in the parsing stack
     * then return, it's just an error.
     */
    i = ctxtPtr.nameNr - 1;
    while i >= 0 {
        unsafe {
            if xmlStrEqual_safe(name, *ctxtPtr.nameTab.offset(i as isize)) != 0 {
                break;
            }
        }
        i -= 1
    }
    if i < 0 {
        htmlParseErr(
            ctxt,
            XML_ERR_TAG_NAME_MISMATCH,
            b"Unexpected end tag : %s\n\x00" as *const u8 as *const i8,
            name,
            0 as *const xmlChar,
        );
        return 0;
    }
    /*
     * Check for auto-closure of HTML elements.
     */
    htmlAutoCloseOnClose(ctxt, name);
    /*
     * Well formedness constraints, opening and closing must match.
     * With the exception that the autoclose may have popped stuff out
     * of the stack.
     */
    if !ctxtPtr.name.is_null() && unsafe { xmlStrEqual_safe(ctxtPtr.name, name) } == 0 {
        htmlParseErr(
            ctxt,
            XML_ERR_TAG_NAME_MISMATCH,
            b"Opening and ending tag mismatch: %s and %s\n\x00" as *const u8 as *const i8,
            name,
            ctxtPtr.name,
        );
    }
    /*
     * SAX: End of Tag
     */
    oldname = ctxtPtr.name;
    if !oldname.is_null() && unsafe { xmlStrEqual_safe(oldname, name) } != 0 {
        let mut sax_condition =
            unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, name);
        }
        htmlNodeInfoPop(ctxt);
        htmlnamePop(ctxt);
        ret = 1
    } else {
        ret = 0
    }
    return ret;
}
/* *
 * htmlParseReference:
 * @ctxt:  an HTML parser context
 *
 * parse and handle entity references in content,
 * this will end-up in a call to character() since this is either a
 * CharRef, or a predefined entity.
 */
fn htmlParseReference(ctxt: htmlParserCtxtPtr) {
    let ctxtPtr = unsafe { &mut *ctxt };
    let mut sax_condition = false;
    let ent: *const htmlEntityDesc;
    let mut out: [xmlChar; 6] = [0; 6];
    let mut name: *const xmlChar = 0 as *const xmlChar;
    if CUR(ctxt) != '&' as i32 {
        return;
    }
    if NXT(ctxt, 1) == '#' as i32 {
        let mut c: u32;
        let mut bits: i32;
        let mut i: i32 = 0;
        c = htmlParseCharRef(ctxt) as u32;
        if c == 0 {
            return;
        }
        if c < 0x80 {
            out[i as usize] = c as xmlChar;
            i = i + 1;
            bits = -6
        } else if c < 0x800 {
            out[i as usize] = (c >> 6 & 0x1f | 0xc0) as xmlChar;
            i = i + 1;
            bits = 0
        } else if c < 0x10000 {
            out[i as usize] = (c >> 12 & 0xf | 0xe0) as xmlChar;
            i = i + 1;
            bits = 6
        } else {
            out[i as usize] = (c >> 18 & 0x7 | 0xf0) as xmlChar;
            i = i + 1;
            bits = 12
        }
        while bits >= 0 {
            out[i as usize] = (c >> bits & 0x3f | 0x80) as xmlChar;
            i = i + 1;
            bits -= 6
        }
        out[i as usize] = 0;
        htmlCheckParagraph(ctxt);
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some() };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData, out.as_mut_ptr(), i);
        }
    } else {
        ent = htmlParseEntityRef(ctxt, &mut name);
        if name.is_null() {
            htmlCheckParagraph(ctxt);
            sax_condition =
                unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some() };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_characters_safe(
                    saxPtr.characters,
                    ctxtPtr.userData,
                    b"&\x00" as *const u8 as *const i8 as *mut xmlChar,
                    1,
                );
            }
            return;
        }
        let flag = unsafe { ent.is_null() || !((*ent).value > 0 as u32) };
        if flag {
            htmlCheckParagraph(ctxt);
            sax_condition =
                unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some() };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_characters_safe(
                    saxPtr.characters,
                    ctxtPtr.userData,
                    b"&\x00" as *const u8 as *const i8 as *mut xmlChar,
                    1,
                );
                unsafe {
                    xmlSAXHandler_characters_safe(
                        saxPtr.characters,
                        ctxtPtr.userData,
                        name,
                        xmlStrlen_safe(name),
                    )
                };
                /* ctxt->sax->characters(ctxt->userData, BAD_CAST ";", 1); */
            }
        } else {
            let mut c_0: u32;
            let mut bits_0: i32;
            let mut i_0: i32 = 0;
            unsafe {
                c_0 = (*ent).value;
            }
            if c_0 < 0x80 {
                out[i_0 as usize] = c_0 as xmlChar;
                i_0 = i_0 + 1;
                bits_0 = -6
            } else if c_0 < 0x800 {
                out[i_0 as usize] = (c_0 >> 6 & 0x1f | 0xc0) as xmlChar;
                i_0 = i_0 + 1;
                bits_0 = 0
            } else if c_0 < 0x10000 {
                out[i_0 as usize] = (c_0 >> 12 & 0xf | 0xe0) as xmlChar;
                i_0 = i_0 + 1;
                bits_0 = 6
            } else {
                out[i_0 as usize] = (c_0 >> 18 & 0x7 | 0xf0) as xmlChar;
                i_0 = i_0 + 1;
                bits_0 = 12
            }
            while bits_0 >= 0 {
                out[i_0 as usize] = (c_0 >> bits_0 & 0x3f | 0x80) as xmlChar;
                i_0 = i_0 + 1;
                bits_0 -= 6
            }
            out[i_0 as usize] = 0 as xmlChar;
            htmlCheckParagraph(ctxt);
            sax_condition =
                unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some() };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_characters_safe(
                    saxPtr.characters,
                    ctxtPtr.userData,
                    out.as_mut_ptr(),
                    i_0,
                );
            }
        }
    };
}
/* *
 * htmlParseContent:
 * @ctxt:  an HTML parser context
 *
 * Parse a content: comment, sub-element, reference or text.
 * Kept for compatibility with old code
 */
fn htmlParseContent(ctxt: htmlParserCtxtPtr) {
    let ctxtPtr = unsafe { &mut *ctxt };
    let currentNode: *mut xmlChar;
    let mut depth: i32;
    let mut name: *const xmlChar;
    currentNode = unsafe { xmlStrdup_safe(ctxtPtr.name) };
    depth = ctxtPtr.nameNr;
    loop {
        GROW(ctxt);
        if ctxtPtr.instate == XML_PARSER_EOF {
            break;
        }
        /*
         * Our tag or one of it's parent or children is ending.
         */
        if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1) == '/' as i32 {
            if htmlParseEndTag(ctxt) != 0 && (!currentNode.is_null() || ctxtPtr.nameNr == 0) {
                if !currentNode.is_null() {
                    unsafe { xmlFree_safe(currentNode as *mut ()) };
                }
                return;
            }
            /* while */
        } else {
            if CUR(ctxt) == '<' as i32
                && (IS_ASCII_LETTER(NXT(ctxt, 1))
                    || NXT(ctxt, 1) == '_' as i32
                    || NXT(ctxt, 1) == ':' as i32)
            {
                name = htmlParseHTMLName_nonInvasive(ctxt);
                if name.is_null() {
                    htmlParseErr(
                        ctxt,
                        XML_ERR_NAME_REQUIRED,
                        b"htmlParseStartTag: invalid element name\n\x00" as *const u8 as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                    /* Dump the bogus tag like browsers do */
                    while CUR(ctxt) != 0 && CUR(ctxt) != '>' as i32 {
                        unsafe { xmlNextChar_safe(ctxt) };
                    }
                    if !currentNode.is_null() {
                        unsafe { xmlFree_safe(currentNode as *mut ()) };
                    }
                    return;
                }
                if !ctxtPtr.name.is_null() {
                    if htmlCheckAutoClose(name, ctxtPtr.name) == 1 {
                        htmlAutoClose(ctxt, name);
                        continue;
                    }
                }
            }
            /*
             * Has this node been popped out during parsing of
             * the next element
             */
            if ctxtPtr.nameNr > 0
                && depth >= ctxtPtr.nameNr
                && unsafe { xmlStrEqual_safe(currentNode, ctxtPtr.name) } == 0
            {
                if !currentNode.is_null() {
                    unsafe { xmlFree_safe(currentNode as *mut ()) };
                }
                return;
            }
            if CUR(ctxt) != 0
                && (unsafe {
                    xmlStrEqual_safe(
                        currentNode,
                        b"script\x00" as *const u8 as *const i8 as *mut xmlChar,
                    )
                } != 0
                    || unsafe {
                        xmlStrEqual_safe(
                            currentNode,
                            b"style\x00" as *const u8 as *const i8 as *mut xmlChar,
                        )
                    } != 0)
            {
                /*
                 * Handle SCRIPT/STYLE separately
                 */
                htmlParseScript(ctxt);
            } else {
                /*
                 * Sometimes DOCTYPE arrives in the middle of the document
                 */
                if CUR(ctxt) == '<' as i32
                    && NXT(ctxt, 1) == '!' as i32
                    && UPP(ctxt, 2) == 'D' as i32
                    && UPP(ctxt, 3) == 'O' as i32
                    && UPP(ctxt, 4) == 'C' as i32
                    && UPP(ctxt, 5) == 'T' as i32
                    && UPP(ctxt, 6) == 'Y' as i32
                    && UPP(ctxt, 7) == 'P' as i32
                    && UPP(ctxt, 8) == 'E' as i32
                {
                    htmlParseErr(
                        ctxt,
                        XML_HTML_STRUCURE_ERROR,
                        b"Misplaced DOCTYPE declaration\n\x00" as *const u8 as *const i8,
                        b"DOCTYPE\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                    htmlParseDocTypeDecl(ctxt);
                }
                /*
                 * First case :  a comment
                 */
                if CUR(ctxt) == '<' as i32
                    && NXT(ctxt, 1) == '!' as i32
                    && NXT(ctxt, 2) == '-' as i32
                    && NXT(ctxt, 3) == '-' as i32
                {
                    htmlParseComment(ctxt);
                } else if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1) == '?' as i32 {
                    htmlParsePI(ctxt);
                } else if CUR(ctxt) == '<' as i32 {
                    htmlParseElement(ctxt);
                } else if CUR(ctxt) == '&' as i32 {
                    htmlParseReference(ctxt);
                } else if CUR(ctxt) == 0 {
                    htmlAutoCloseOnEnd(ctxt);
                    break;
                } else {
                    /*
                     * Second case : a Processing Instruction.
                     */
                    /*
                     * Third case :  a sub-element.
                     */
                    /*
                     * Fourth case : a reference. If if has not been resolved,
                     *    parsing returns it's Name, create the node
                     */
                    /*
                     * Fifth case : end of the resource
                     */
                    /*
                     * Last case, text. Note that References are handled directly.
                     */
                    htmlParseCharData(ctxt);
                }
            }
            GROW(ctxt);
        }
    }
    if !currentNode.is_null() {
        unsafe { xmlFree_safe(currentNode as *mut ()) };
    };
}
/* *
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
pub fn htmlParseElement(ctxt: htmlParserCtxtPtr) {
    let mut sax_condition = false;
    let name: *const xmlChar;
    let mut currentNode: *mut xmlChar = 0 as *mut xmlChar;
    let info: *const htmlElemDesc;
    let mut node_info: htmlParserNodeInfo = htmlParserNodeInfo {
        node: 0 as *const _xmlNode,
        begin_pos: 0,
        begin_line: 0,
        end_pos: 0,
        end_line: 0,
    };
    let mut failed: i32;
    let mut depth: i32;
    let mut oldptr: *const xmlChar;
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"htmlParseElement: context error\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            return;
        }
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.instate == XML_PARSER_EOF {
        return;
    }
    /* Capture start position */
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if ctxtPtr.record_info != 0 {
        unsafe {
            node_info.begin_pos = inputPtr
                .consumed
                .wrapping_add(inputPtr.cur.offset_from(inputPtr.base) as u64);
        }
        node_info.begin_line = inputPtr.line as u64
    }
    failed = htmlParseStartTag(ctxt);
    name = ctxtPtr.name;
    if failed == -1 || name.is_null() {
        if CUR(ctxt) == '>' as i32 {
            unsafe { xmlNextChar_safe(ctxt) };
        }
        return;
    }
    /*
     * Lookup the info for that element.
     */
    info = htmlTagLookup(name);
    if info.is_null() {
        htmlParseErr(
            ctxt,
            XML_HTML_UNKNOWN_TAG,
            b"Tag %s invalid\n\x00" as *const u8 as *const i8,
            name,
            0 as *const xmlChar,
        );
    }
    /*
     * Check for an Empty Element labeled the XML/SGML way
     */
    if CUR(ctxt) == '/' as i32 && NXT(ctxt, 1) == '>' as i32 {
        SKIP(ctxt, 2);
        sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() && ctxtPtr.disableSAX == 0
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, name);
        }
        htmlnamePop(ctxt);
        return;
    }
    if CUR(ctxt) == '>' as i32 {
        unsafe { xmlNextChar_safe(ctxt) };
    } else {
        htmlParseErr(
            ctxt,
            XML_ERR_GT_REQUIRED,
            b"Couldn\'t find end of Start Tag %s\n\x00" as *const u8 as *const i8,
            name,
            0 as *const xmlChar,
        );
        /*
         * end of parsing of this node.
         */
        if unsafe { xmlStrEqual_safe(name, ctxtPtr.name) } != 0 {
            unsafe { nodePop_safe(ctxt) };
            htmlnamePop(ctxt);
        }
        /*
         * Capture end position and add node
         */
        if ctxtPtr.record_info != 0 {
            unsafe {
                node_info.end_pos = inputPtr
                    .consumed
                    .wrapping_add(inputPtr.cur.offset_from(inputPtr.base) as u64);
            }
            node_info.end_line = inputPtr.line as u64;
            node_info.node = ctxtPtr.node as *const _xmlNode;
            unsafe { xmlParserAddNodeInfo_safe(ctxt, &mut node_info) };
        }
        return;
    }
    /*
     * Check for an Empty Element from DTD definition
     */
    let flag = unsafe { info.is_null() && (*info).empty as i32 != 0 };
    if flag {
        sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() && ctxtPtr.disableSAX == 0
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, name);
        }
        htmlnamePop(ctxt);
        return;
    }

    /*
     * Parse the content of the element:
     */
    currentNode = unsafe { xmlStrdup_safe(ctxtPtr.name) };
    depth = ctxtPtr.nameNr;
    while CUR(ctxt) != 0 {
        oldptr = inputPtr.cur;
        htmlParseContent(ctxt);
        if oldptr == inputPtr.cur {
            break;
        }
        if ctxtPtr.nameNr < depth {
            break;
        }
    }
    /*
     * Capture end position and add node
     */
    if !currentNode.is_null() && ctxtPtr.record_info != 0 {
        unsafe {
            node_info.end_pos = inputPtr
                .consumed
                .wrapping_add(inputPtr.cur.offset_from(inputPtr.base) as u64);
        }
        node_info.end_line = inputPtr.line as u64;
        node_info.node = ctxtPtr.node as *const _xmlNode;
        unsafe { xmlParserAddNodeInfo_safe(ctxt, &mut node_info) };
    }
    if CUR(ctxt) == 0 {
        htmlAutoCloseOnEnd(ctxt);
    }
    if !currentNode.is_null() {
        unsafe { xmlFree_safe(currentNode as *mut ()) };
    };
}

fn htmlParserFinishElementParsing(ctxt: htmlParserCtxtPtr) {
    let ctxtPtr = unsafe { &mut *ctxt };
    /*
     * Capture end position and add node
     */
    if !ctxtPtr.node.is_null() && ctxtPtr.record_info != 0 {
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        let mut nodeInfoPtr = unsafe { &mut *(*ctxt).nodeInfo };
        unsafe {
            nodeInfoPtr.end_pos = inputPtr
                .consumed
                .wrapping_add(inputPtr.cur.offset_from(inputPtr.base) as u64);
        }
        nodeInfoPtr.end_line = inputPtr.line as u64;
        nodeInfoPtr.node = ctxtPtr.node as *const _xmlNode;
        unsafe { xmlParserAddNodeInfo_safe(ctxt, ctxtPtr.nodeInfo) };
        htmlNodeInfoPop(ctxt);
    }
    if CUR(ctxt) == 0 {
        htmlAutoCloseOnEnd(ctxt);
    };
}
/* *
 * htmlParseElementInternal:
 * @ctxt:  an HTML parser context
 *
 * parse an HTML element, new version, non recursive
 *
 * [39] element ::= EmptyElemTag | STag content ETag
 *
 * [41] Attribute ::= Name Eq AttValue
 */
fn htmlParseElementInternal(ctxt: htmlParserCtxtPtr) {
    let name: *const xmlChar;
    let info: *const htmlElemDesc;
    let mut node_info: htmlParserNodeInfo = {
        let mut init = _xmlParserNodeInfo {
            node: 0 as *const _xmlNode,
            begin_pos: 0,
            begin_line: 0,
            end_pos: 0,
            end_line: 0,
        };
        init
    };
    let mut failed: i32;
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"htmlParseElementInternal: context error\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            return;
        }
    }
    let ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.instate == XML_PARSER_EOF {
        return;
    }
    /* Capture start position */
    let inputPtr = unsafe { &mut *(*ctxt).input };
    if ctxtPtr.record_info != 0 {
        unsafe {
            node_info.begin_pos = inputPtr
                .consumed
                .wrapping_add(inputPtr.cur.offset_from(inputPtr.base) as u64);
        }
        node_info.begin_line = inputPtr.line as u64
    }
    failed = htmlParseStartTag(ctxt);
    name = ctxtPtr.name;
    if failed == -1 || name.is_null() {
        if CUR(ctxt) == '>' as i32 {
            unsafe { xmlNextChar_safe(ctxt) };
        }
        return;
    }
    /*
     * Lookup the info for that element.
     */
    info = htmlTagLookup(name);
    if info.is_null() {
        htmlParseErr(
            ctxt,
            XML_HTML_UNKNOWN_TAG,
            b"Tag %s invalid\n\x00" as *const u8 as *const i8,
            name,
            0 as *const xmlChar,
        );
    }
    /*
     * Check for an Empty Element labeled the XML/SGML way
     */
    let mut sax_condition = false;
    if CUR(ctxt) == '/' as i32 && NXT(ctxt, 1) == '>' as i32 {
        SKIP(ctxt, 2);
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, name);
        }
        htmlnamePop(ctxt);
        return;
    }
    if CUR(ctxt) == '>' as i32 {
        unsafe { xmlNextChar_safe(ctxt) };
    } else {
        htmlParseErr(
            ctxt,
            XML_ERR_GT_REQUIRED,
            b"Couldn\'t find end of Start Tag %s\n\x00" as *const u8 as *const i8,
            name,
            0 as *const xmlChar,
        );
        /*
         * end of parsing of this node.
         */
        if unsafe { xmlStrEqual_safe(name, ctxtPtr.name) } != 0 {
            unsafe { nodePop_safe(ctxt) };
            htmlnamePop(ctxt);
        }
        if ctxtPtr.record_info != 0 {
            htmlNodeInfoPush(ctxt, &mut node_info);
        }
        htmlParserFinishElementParsing(ctxt);
        return;
    }
    /*
     * Check for an Empty Element from DTD definition
     */
    let flag = unsafe { !info.is_null() && (*info).empty as i32 != 0 };
    if flag {
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, name);
        }
        htmlnamePop(ctxt);
        return;
    }

    if ctxtPtr.record_info != 0 {
        htmlNodeInfoPush(ctxt, &mut node_info);
    };
}
/* *
 * htmlParseContentInternal:
 * @ctxt:  an HTML parser context
 *
 * Parse a content: comment, sub-element, reference or text.
 * New version for non recursive htmlParseElementInternal
 */
fn htmlParseContentInternal(ctxt: htmlParserCtxtPtr) {
    let ctxtPtr = unsafe { &mut *ctxt };
    let mut currentNode: *mut xmlChar;
    let mut depth: i32;
    let mut name: *const xmlChar;
    currentNode = unsafe { xmlStrdup_safe(ctxtPtr.name) };
    depth = ctxtPtr.nameNr;
    loop {
        GROW(ctxt);
        if ctxtPtr.instate == XML_PARSER_EOF {
            break;
        }
        /*
         * Our tag or one of it's parent or children is ending.
         */
        if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1) == '/' as i32 {
            if htmlParseEndTag(ctxt) != 0 && (!currentNode.is_null() || ctxtPtr.nameNr == 0) {
                if !currentNode.is_null() {
                    unsafe { xmlFree_safe(currentNode as *mut ()) };
                }
                currentNode = unsafe { xmlStrdup_safe(ctxtPtr.name) };
                depth = ctxtPtr.nameNr
            }
            /* while */
        } else {
            if CUR(ctxt) == '<' as i32
                && (IS_ASCII_LETTER(NXT(ctxt, 1))
                    || NXT(ctxt, 1) == '_' as i32
                    || NXT(ctxt, 1) == ':' as i32)
            {
                name = htmlParseHTMLName_nonInvasive(ctxt);
                if name.is_null() {
                    htmlParseErr(
                        ctxt,
                        XML_ERR_NAME_REQUIRED,
                        b"htmlParseStartTag: invalid element name\n\x00" as *const u8 as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                    /* Dump the bogus tag like browsers do */
                    while CUR(ctxt) == 0 && CUR(ctxt) != '>' as i32 {
                        unsafe { xmlNextChar_safe(ctxt) };
                    }
                    htmlParserFinishElementParsing(ctxt);
                    if !currentNode.is_null() {
                        unsafe { xmlFree_safe(currentNode as *mut ()) };
                    }
                    currentNode = unsafe { xmlStrdup_safe(ctxtPtr.name) };
                    depth = ctxtPtr.nameNr;
                    continue;
                } else if !ctxtPtr.name.is_null() {
                    if htmlCheckAutoClose(name, ctxtPtr.name) == 1 {
                        htmlAutoClose(ctxt, name);
                        continue;
                    }
                }
            }
            /*
             * Has this node been popped out during parsing of
             * the next element
             */
            if ctxtPtr.nameNr > 0
                && depth >= ctxtPtr.nameNr
                && unsafe { xmlStrEqual_safe(currentNode, ctxtPtr.name) } == 0
            {
                htmlParserFinishElementParsing(ctxt);
                if !currentNode.is_null() {
                    unsafe { xmlFree_safe(currentNode as *mut ()) };
                }
                currentNode = unsafe { xmlStrdup_safe(ctxtPtr.name) };
                depth = ctxtPtr.nameNr
            } else {
                if CUR(ctxt) != 0
                    && (unsafe {
                        xmlStrEqual_safe(
                            currentNode,
                            b"script\x00" as *const u8 as *const i8 as *mut xmlChar,
                        )
                    } != 0
                        || unsafe {
                            xmlStrEqual_safe(
                                currentNode,
                                b"style\x00" as *const u8 as *const i8 as *mut xmlChar,
                            )
                        } != 0)
                {
                    /*
                     * Handle SCRIPT/STYLE separately
                     */
                    htmlParseScript(ctxt);
                } else {
                    /*
                     * Sometimes DOCTYPE arrives in the middle of the document
                     */
                    if CUR(ctxt) == '<' as i32
                        && NXT(ctxt, 1) == '!' as i32
                        && UPP(ctxt, 2) == 'D' as i32
                        && UPP(ctxt, 3) == 'O' as i32
                        && UPP(ctxt, 4) == 'C' as i32
                        && UPP(ctxt, 5) == 'T' as i32
                        && UPP(ctxt, 6) == 'Y' as i32
                        && UPP(ctxt, 7) == 'P' as i32
                        && UPP(ctxt, 8) == 'E' as i32
                    {
                        htmlParseErr(
                            ctxt,
                            XML_HTML_STRUCURE_ERROR,
                            b"Misplaced DOCTYPE declaration\n\x00" as *const u8 as *const i8,
                            b"DOCTYPE\x00" as *const u8 as *const i8 as *mut xmlChar,
                            0 as *const xmlChar,
                        );
                        htmlParseDocTypeDecl(ctxt);
                    }
                    /*
                     * First case :  a comment
                     */
                    if CUR(ctxt) == '<' as i32
                        && NXT(ctxt, 1) == '!' as i32
                        && NXT(ctxt, 2) == '-' as i32
                        && NXT(ctxt, 3) == '-' as i32
                    {
                        htmlParseComment(ctxt);
                    } else if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1) == '?' as i32 {
                        htmlParsePI(ctxt);
                    } else if CUR(ctxt) == '<' as i32 {
                        htmlParseElementInternal(ctxt);
                        if !currentNode.is_null() {
                            unsafe { xmlFree_safe(currentNode as *mut ()) };
                        }
                        currentNode = unsafe { xmlStrdup_safe(ctxtPtr.name) };
                        depth = ctxtPtr.nameNr
                    } else if CUR(ctxt) == '&' as i32 {
                        htmlParseReference(ctxt);
                    } else if CUR(ctxt) == 0 {
                        htmlAutoCloseOnEnd(ctxt);
                        break;
                    } else {
                        /*
                         * Second case : a Processing Instruction.
                         */
                        /*
                         * Third case :  a sub-element.
                         */
                        /*
                         * Fourth case : a reference. If if has not been resolved,
                         *    parsing returns it's Name, create the node
                         */
                        /*
                         * Fifth case : end of the resource
                         */
                        /*
                         * Last case, text. Note that References are handled directly.
                         */
                        htmlParseCharData(ctxt);
                    }
                }
                GROW(ctxt);
            }
        }
    }
    if !currentNode.is_null() {
        unsafe { xmlFree_safe(currentNode as *mut ()) };
    };
}
/*
 * internal function of HTML parser needed for xmlParseInNodeContext
 * but not part of the API
 */
/* *
 * htmlParseContent:
 * @ctxt:  an HTML parser context
 *
 * Parse a content: comment, sub-element, reference or text.
 * This is the entry point when called from parser.c
 */

pub fn __htmlParseContent_htmlparser(ctxt: *mut ()) {
    if !ctxt.is_null() {
        htmlParseContentInternal(ctxt as htmlParserCtxtPtr);
    };
}
/* *
 * htmlParseDocument:
 * @ctxt:  an HTML parser context
 *
 * parse an HTML document (and build a tree if using the standard SAX
 * interface).
 *
 * Returns 0, -1 in case of error. the parser context is augmented
 *                as a result of the parsing.
 */
pub fn htmlParseDocument(mut ctxt: htmlParserCtxtPtr) -> i32 {
    let mut start: [xmlChar; 4] = [0; 4];
    let mut enc: xmlCharEncoding;
    let dtd: xmlDtdPtr;
    unsafe { xmlInitParser_safe() };
    unsafe { htmlDefaultSAXHandlerInit_safe() };
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"htmlParseDocument: context error\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            return XML_ERR_INTERNAL_ERROR as i32;
        }
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    ctxtPtr.html = 1;
    ctxtPtr.linenumbers = 1;
    GROW(ctxt);
    /*
     * SAX: beginning of the document processing.
     */
    let mut sax_condition = false;
    sax_condition =
        unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some() };
    if sax_condition {
        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
        unsafe {
            xmlSAXHandler_setDocumentLocator_safe(
                saxPtr.setDocumentLocator,
                ctxtPtr.userData,
                __xmlDefaultSAXLocator_safe(),
            )
        };
    }
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    unsafe {
        if ctxtPtr.encoding.is_null() && inputPtr.end.offset_from(inputPtr.cur) as i64 >= 4 {
            /*
             * Get the 4 first bytes and decode the charset
             * if enc != XML_CHAR_ENCODING_NONE
             * plug some encoding conversion routines.
             */
            start[0 as usize] = RAW(ctxt) as xmlChar;
            start[1 as usize] = NXT(ctxt, 1) as xmlChar;
            start[2 as usize] = NXT(ctxt, 2) as xmlChar;
            start[3 as usize] = NXT(ctxt, 3) as xmlChar;
            enc = xmlDetectCharEncoding_safe(&mut *start.as_mut_ptr().offset(0 as isize), 4);
            if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
                xmlSwitchEncoding_safe(ctxt, enc);
            }
        }
    }
    /*
     * Wipe out everything which is before the first '<'
     */
    htmlSkipBlankChars(ctxt);
    if CUR(ctxt) == 0 {
        htmlParseErr(
            ctxt,
            XML_ERR_DOCUMENT_EMPTY,
            b"Document is empty\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    }
    sax_condition = unsafe {
        !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startDocument.is_some() && ctxtPtr.disableSAX == 0
    };
    if sax_condition {
        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
        xmlSAXHandler_startDocument_safe(saxPtr.startDocument, ctxtPtr.userData);
    }
    /*
     * Parse possible comments and PIs before any content
     */
    while CUR(ctxt) == '<' as i32
        && NXT(ctxt, 1) == '!' as i32
        && NXT(ctxt, 2) == '-' as i32
        && NXT(ctxt, 3) == '-' as i32
        || (CUR(ctxt) == '<' as i32 && NXT(ctxt, 1) == '?' as i32)
    {
        htmlParseComment(ctxt);
        htmlParsePI(ctxt);
        htmlSkipBlankChars(ctxt);
    }
    /*
     * Then possibly doc type declaration(s) and more Misc
     * (doctypedecl Misc*)?
     */
    if CUR(ctxt) == '<' as i32
        && NXT(ctxt, 1) == '!' as i32
        && UPP(ctxt, 2) == 'D' as i32
        && UPP(ctxt, 3) == 'O' as i32
        && UPP(ctxt, 4) == 'C' as i32
        && UPP(ctxt, 5) == 'T' as i32
        && UPP(ctxt, 6) == 'Y' as i32
        && UPP(ctxt, 7) == 'P' as i32
        && UPP(ctxt, 8) == 'E' as i32
    {
        htmlParseDocTypeDecl(ctxt);
    }
    htmlSkipBlankChars(ctxt);
    /*
     * Parse possible comments and PIs before any content
     */
    while CUR(ctxt) == '<' as i32
        && NXT(ctxt, 1) == '!' as i32
        && NXT(ctxt, 2) == '-' as i32
        && NXT(ctxt, 3) == '-' as i32
        || (CUR(ctxt) == '<' as i32 && NXT(ctxt, 1) == '?' as i32)
    {
        htmlParseComment(ctxt);
        htmlParsePI(ctxt);
        htmlSkipBlankChars(ctxt);
    }
    /*
     * Time to start parsing the tree itself
     */
    htmlParseContentInternal(ctxt);
    /*
     * autoclose
     */
    if CUR(ctxt) == 0 {
        htmlAutoCloseOnEnd(ctxt);
    }
    /*
     * SAX: end of the document processing.
     */
    sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endDocument.is_some() };
    if sax_condition {
        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
        xmlSAXHandler_endDocument_safe(saxPtr.endDocument, ctxtPtr.userData);
    }
    if ctxtPtr.options & HTML_PARSE_NODEFDTD as i32 == 0 && !ctxtPtr.myDoc.is_null() {
        dtd = unsafe { xmlGetIntSubset_safe(ctxtPtr.myDoc as *const xmlDoc) };
        if dtd.is_null() {
            let mut myDocPtr = unsafe { &mut *(*ctxt).myDoc };
            myDocPtr.intSubset = unsafe {
                xmlCreateIntSubset_safe(
                    ctxtPtr.myDoc,
                    b"html\x00" as *const u8 as *const i8 as *mut xmlChar,
                    b"-//W3C//DTD HTML 4.0 Transitional//EN\x00" as *const u8 as *const i8
                        as *mut xmlChar,
                    b"http://www.w3.org/TR/REC-html40/loose.dtd\x00" as *const u8 as *const i8
                        as *mut xmlChar,
                )
            }
        }
    }
    if ctxtPtr.wellFormed == 0 {
        return -1;
    }
    return 0;
}
/* ***********************************************************************
 *									*
 *			Parser contexts handling			*
 *									*
 ************************************************************************/
/* *
 * htmlInitParserCtxt:
 * @ctxt:  an HTML parser context
 *
 * Initialize a parser context
 *
 * Returns 0 in case of success and -1 in case of error
 */
fn htmlInitParserCtxt(ctxt: htmlParserCtxtPtr) -> i32 {
    let sax: *mut htmlSAXHandler;
    if ctxt.is_null() {
        return -1;
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    unsafe {
        memset_safe(
            ctxt as *mut (),
            0,
            ::std::mem::size_of::<htmlParserCtxt>() as u64,
        )
    };
    ctxtPtr.dict = unsafe { xmlDictCreate_safe() };
    if ctxtPtr.dict.is_null() {
        htmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"htmlInitParserCtxt: out of memory\n\x00" as *const u8 as *const i8,
        );
        return -1;
    }
    sax = unsafe { xmlMalloc_safe(::std::mem::size_of::<htmlSAXHandler>() as u64) }
        as *mut htmlSAXHandler;
    if sax.is_null() {
        htmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"htmlInitParserCtxt: out of memory\n\x00" as *const u8 as *const i8,
        );
        return -1;
    } else {
        unsafe {
            memset_safe(
                sax as *mut (),
                0,
                ::std::mem::size_of::<htmlSAXHandler>() as u64,
            )
        };
    }
    /* Allocate the Input stack */
    ctxtPtr.inputTab = unsafe {
        xmlMalloc_safe((5 as u64).wrapping_mul(::std::mem::size_of::<htmlParserInputPtr>() as u64))
    } as *mut htmlParserInputPtr;
    if ctxtPtr.inputTab.is_null() {
        htmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"htmlInitParserCtxt: out of memory\n\x00" as *const u8 as *const i8,
        );
        ctxtPtr.inputNr = 0;
        ctxtPtr.inputMax = 0;
        ctxtPtr.input = 0 as xmlParserInputPtr;
        return -1;
    }
    ctxtPtr.inputNr = 0;
    ctxtPtr.inputMax = 5;
    ctxtPtr.input = 0 as xmlParserInputPtr;
    ctxtPtr.version = 0 as *const xmlChar;
    ctxtPtr.encoding = 0 as *const xmlChar;
    ctxtPtr.standalone = -1;
    ctxtPtr.instate = XML_PARSER_START;
    /* Allocate the Node stack */
    ctxtPtr.nodeTab = unsafe {
        xmlMalloc_safe((10 as u64).wrapping_mul(::std::mem::size_of::<htmlNodePtr>() as u64))
    } as *mut htmlNodePtr;
    if ctxtPtr.nodeTab.is_null() {
        htmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"htmlInitParserCtxt: out of memory\n\x00" as *const u8 as *const i8,
        );
        ctxtPtr.nodeNr = 0;
        ctxtPtr.nodeMax = 0;
        ctxtPtr.node = 0 as xmlNodePtr;
        ctxtPtr.inputNr = 0;
        ctxtPtr.inputMax = 0;
        ctxtPtr.input = 0 as xmlParserInputPtr;
        return -1;
    }
    ctxtPtr.nodeNr = 0;
    ctxtPtr.nodeMax = 10;
    ctxtPtr.node = 0 as xmlNodePtr;
    /* Allocate the Name stack */
    ctxtPtr.nameTab = unsafe {
        xmlMalloc_safe((10 as u64).wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as u64))
    } as *mut *const xmlChar;
    if ctxtPtr.nameTab.is_null() {
        htmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"htmlInitParserCtxt: out of memory\n\x00" as *const u8 as *const i8,
        );
        ctxtPtr.nameNr = 0;
        ctxtPtr.nameMax = 0;
        ctxtPtr.name = 0 as *const xmlChar;
        ctxtPtr.nodeNr = 0;
        ctxtPtr.nodeMax = 0;
        ctxtPtr.node = 0 as xmlNodePtr;
        ctxtPtr.inputNr = 0;
        ctxtPtr.inputMax = 0;
        ctxtPtr.input = 0 as xmlParserInputPtr;
        return -(1);
    }
    ctxtPtr.nameNr = 0;
    ctxtPtr.nameMax = 10;
    ctxtPtr.name = 0 as *const xmlChar;
    ctxtPtr.nodeInfoTab = 0 as *mut xmlParserNodeInfo;
    ctxtPtr.nodeInfoNr = 0;
    ctxtPtr.nodeInfoMax = 0;
    if sax.is_null() {
        ctxtPtr.sax = unsafe { __htmlDefaultSAXHandler_safe() } as xmlSAXHandlerPtr
    } else {
        ctxtPtr.sax = sax;
        unsafe {
            memcpy_safe(
                sax as *mut (),
                __htmlDefaultSAXHandler_safe() as *const (),
                ::std::mem::size_of::<xmlSAXHandlerV1>() as u64,
            )
        };
    }
    ctxtPtr.userData = ctxt as *mut ();
    ctxtPtr.myDoc = 0 as xmlDocPtr;
    ctxtPtr.wellFormed = 1;
    ctxtPtr.replaceEntities = 0;
    ctxtPtr.linenumbers = unsafe { __xmlLineNumbersDefaultValue_safe() };
    ctxtPtr.keepBlanks = unsafe { __xmlKeepBlanksDefaultValue_safe() };
    ctxtPtr.html = 1;
    ctxtPtr.vctxt.finishDtd = 0xabcd1234;
    ctxtPtr.vctxt.userData = ctxt as *mut ();
    ctxtPtr.vctxt.error = Some(
        xmlParserValidityError as unsafe extern "C" fn(_: *mut (), _: *const i8, _: ...) -> (),
    );
    ctxtPtr.vctxt.warning = Some(
        xmlParserValidityWarning as unsafe extern "C" fn(_: *mut (), _: *const i8, _: ...) -> (),
    );
    ctxtPtr.record_info = 0;
    ctxtPtr.validate = 0;
    ctxtPtr.checkIndex = 0;
    ctxtPtr.catalogs = 0 as *mut ();
    unsafe { xmlInitNodeInfoSeq_safe(&mut ctxtPtr.node_seq) };
    return 0;
}
/* *
 * htmlFreeParserCtxt:
 * @ctxt:  an HTML parser context
 *
 * Free all the memory used by a parser context. However the parsed
 * document in ctxt->myDoc is not freed.
 */

pub fn htmlFreeParserCtxt(ctxt: htmlParserCtxtPtr) {
    unsafe { xmlFreeParserCtxt_safe(ctxt) };
}
/* *
 * htmlNewParserCtxt:
 *
 * Allocate and initialize a new parser context.
 *
 * Returns the htmlParserCtxtPtr or NULL in case of allocation error
 */

pub fn htmlNewParserCtxt() -> htmlParserCtxtPtr {
    let ctxt: xmlParserCtxtPtr;
    ctxt = unsafe { xmlMalloc_safe(::std::mem::size_of::<xmlParserCtxt>() as u64) }
        as xmlParserCtxtPtr;
    if ctxt.is_null() {
        htmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"NewParserCtxt: out of memory\n\x00" as *const u8 as *const i8,
        );
        return 0 as htmlParserCtxtPtr;
    }
    unsafe {
        memset_safe(
            ctxt as *mut (),
            0,
            ::std::mem::size_of::<xmlParserCtxt>() as u64,
        )
    };
    if htmlInitParserCtxt(ctxt) < 0 {
        htmlFreeParserCtxt(ctxt);
        return 0 as htmlParserCtxtPtr;
    }
    return ctxt;
}
/* *
 * htmlCreateMemoryParserCtxt:
 * @buffer:  a pointer to a char array
 * @size:  the size of the array
 *
 * Create a parser context for an HTML in-memory document.
 *
 * Returns the new parser context or NULL
 */

pub fn htmlCreateMemoryParserCtxt_htmlparser(buffer: *const i8, size: i32) -> htmlParserCtxtPtr {
    let ctxt: xmlParserCtxtPtr;
    let input: xmlParserInputPtr;
    let buf: xmlParserInputBufferPtr;
    if buffer.is_null() {
        return 0 as htmlParserCtxtPtr;
    }
    if size <= 0 {
        return 0 as htmlParserCtxtPtr;
    }
    ctxt = htmlNewParserCtxt();
    if ctxt.is_null() {
        return 0 as htmlParserCtxtPtr;
    }
    buf = unsafe { xmlParserInputBufferCreateMem_safe(buffer, size, XML_CHAR_ENCODING_NONE) };
    if buf.is_null() {
        return 0 as htmlParserCtxtPtr;
    }
    input = unsafe { xmlNewInputStream_safe(ctxt) };
    if input.is_null() {
        unsafe { xmlFreeParserCtxt_safe(ctxt) };
        return 0 as htmlParserCtxtPtr;
    }
    let mut inputPtr = unsafe { &mut (*input) };
    let mut bufPtr = unsafe { &mut (*buf) };
    inputPtr.filename = 0 as *const i8;
    inputPtr.buf = buf;
    unsafe { xmlBufResetInput_safe(bufPtr.buffer, input) };
    unsafe { inputPush_safe(ctxt, input) };
    return ctxt;
}
/* *
 * htmlCreateDocParserCtxt:
 * @cur:  a pointer to an array of xmlChar
 * @encoding:  a free form C string describing the HTML document encoding, or NULL
 *
 * Create a parser context for an HTML document.
 *
 * TODO: check the need to add encoding handling there
 *
 * Returns the new parser context or NULL
 */
fn htmlCreateDocParserCtxt(cur: *const xmlChar, encoding: *const i8) -> htmlParserCtxtPtr {
    let mut len: i32;
    let ctxt: htmlParserCtxtPtr;
    if cur.is_null() {
        return 0 as htmlParserCtxtPtr;
    }
    len = unsafe { xmlStrlen_safe(cur) };
    ctxt = unsafe { htmlCreateMemoryParserCtxt_safe(cur as *mut i8, len) };
    if ctxt.is_null() {
        return 0 as htmlParserCtxtPtr;
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        let mut handler: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        if !inputPtr.encoding.is_null() {
            unsafe { xmlFree_safe(inputPtr.encoding as *mut xmlChar as *mut ()) };
        }
        inputPtr.encoding = unsafe { xmlStrdup_safe(encoding as *const xmlChar) };
        enc = unsafe { xmlParseCharEncoding_safe(encoding) };
        /*
         * registered set of known encodings
         */
        if enc as i32 != XML_CHAR_ENCODING_ERROR as i32 {
            unsafe { xmlSwitchEncoding_safe(ctxt, enc) };
            if ctxtPtr.errNo == XML_ERR_UNSUPPORTED_ENCODING as i32 {
                htmlParseErr(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"Unsupported encoding %s\n\x00" as *const u8 as *const i8,
                    encoding as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
        } else {
            /*
             * fallback for unknown encodings
             */
            handler = unsafe { xmlFindCharEncodingHandler_safe(encoding) };
            if !handler.is_null() {
                unsafe { xmlSwitchToEncoding_safe(ctxt, handler) };
            } else {
                htmlParseErr(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"Unsupported encoding %s\n\x00" as *const u8 as *const i8,
                    encoding as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
        }
    }
    return ctxt;
}
/* ***********************************************************************
 *									*
 *	Progressive parsing interfaces				*
 *									*
 ************************************************************************/
/* *
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
#[cfg(LIBXML_PUSH_ENABLED)]
pub fn htmlParseLookupSequence(
    ctxt: htmlParserCtxtPtr,
    first: xmlChar,
    next: xmlChar,
    third: xmlChar,
    ignoreattrval: i32,
) -> i32 {
    let ctxtPtr = unsafe { &mut *ctxt };
    let mut DEBUG_PUSH: i32 = 0;
    match () {
        #[cfg(DEBUG_PUSH)]
        _ => {
            DEBUG_PUSH = 1;
        }
        #[cfg(not(DEBUG_PUSH))]
        _ => {}
    };
    let mut base: i32;
    let mut len: i32;
    let mut in_0: htmlParserInputPtr;
    let buf: *const xmlChar;
    let mut invalue: i32 = 0;
    let mut valdellim: i8 = 0;
    in_0 = ctxtPtr.input;
    if in_0.is_null() {
        return -1;
    }
    let mut in_0Ptr = unsafe { &mut *in_0 };
    unsafe {
        base = in_0Ptr.cur.offset_from(in_0Ptr.base) as i32;
    }
    if base < 0 {
        return -1;
    }
    if ctxtPtr.checkIndex > base as i64 {
        base = ctxtPtr.checkIndex as i32;
        if (ctxtPtr.hasPErefs & 1) == 0 {
            invalue = 0;
        } else {
            invalue = 1;
        }
    }
    if in_0Ptr.buf.is_null() {
        buf = in_0Ptr.base;
        len = in_0Ptr.length;
    } else {
        let mut in_0BufPtr = unsafe { &mut *(*in_0).buf };
        buf = unsafe { xmlBufContent_safe(in_0BufPtr.buffer as *const xmlBuf) };
        len = unsafe { xmlBufUse_safe(in_0BufPtr.buffer) } as i32;
    }
    /* take into account the sequence length */
    if third != 0 {
        len -= 2;
    } else if next != 0 {
        len -= 1;
    }
    while base < len {
        if ignoreattrval != 0 {
            unsafe {
                if *buf.offset(base as isize) == '\"' as u8
                    || *buf.offset(base as isize) == '\'' as u8
                {
                    if invalue != 0 {
                        if *buf.offset(base as isize) == valdellim as u8 {
                            invalue = 0;
                            base += 1;
                            continue;
                        }
                    } else {
                        valdellim = *buf.offset(base as isize) as i8;
                        invalue = 1;
                        base += 1;
                        continue;
                    }
                } else if invalue != 0 {
                    base += 1;
                    continue;
                }
            }
        }

        unsafe {
            if *buf.offset(base as isize) == first {
                if third as i32 != 0 {
                    if *buf.offset((base + 1) as isize) != next
                        || *buf.offset((base + 2) as isize) != third
                    {
                        base += 1;
                        continue;
                    }
                } else if next as i32 != 0 {
                    if *buf.offset((base + 1) as isize) != next {
                        base += 1;
                        continue;
                    }
                }
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    if next == 0 {
                        __xmlGenericError_safe_macro!(
                            __xmlGenericErrorContext_safe(),
                            b"HPP: lookup '%c' found at %d" as *const u8 as *const i8,
                            first as u32,
                            base as u32
                        );
                    } else if third == 0 {
                        __xmlGenericError_safe_macro!(
                            __xmlGenericErrorContext_safe(),
                            b"HPP: lookup '%c%c' found at %d\n" as *const u8 as *const i8,
                            first as u32,
                            next as u32,
                            base as u32
                        );
                    } else {
                        __xmlGenericError_safe_macro!(
                            __xmlGenericErrorContext_safe(),
                            b"HPP: lookup '%c%c%c' found at %d\n" as *const u8 as *const i8,
                            first as u32,
                            next as u32,
                            third as u32,
                            base as u32
                        );
                    }
                }
                return (base as i64 - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64) as i32;
            }
        }
        base += 1;
    }
    ctxtPtr.checkIndex = base as i64;
    if invalue != 0 {
        ctxtPtr.hasPErefs = ctxtPtr.hasPErefs | (1);
    } else {
        ctxtPtr.hasPErefs = ctxtPtr.hasPErefs & (-2);
    }
    if DEBUG_PUSH != 0 {
        if next as i32 == 0 {
            __xmlGenericError_safe_macro!(
                __xmlGenericErrorContext_safe(),
                b"HPP: lookup '%c' failed\n" as *const u8 as *const i8,
                first as u32
            );
        } else if third == 0 {
            __xmlGenericError_safe_macro!(
                __xmlGenericErrorContext_safe(),
                b"HPP: lookup '%c%c' failed\n" as *const u8 as *const i8,
                first as u32,
                next as u32
            );
        } else {
            __xmlGenericError_safe_macro!(
                __xmlGenericErrorContext_safe(),
                b"HPP: lookup '%c%c%c' failed\n" as *const u8 as *const i8,
                first as u32,
                next as u32,
                third as u32
            );
        }
    }
    return -1;
}
/* *
 * htmlParseLookupCommentEnd:
 * @ctxt: an HTML parser context
 *
 * Try to find a comment end tag in the input stream
 * The search includes "-->" as well as WHATWG-recommended incorrectly-closed tags.
 * (See https://html.spec.whatwg.org/multipage/parsing.html#parse-error-incorrectly-closed-comment)
 * This function has a side effect of (possibly) incrementing ctxt->checkIndex
 * to avoid rescanning sequences of bytes, it DOES change the state of the
 * parser, do not use liberally.
 * This wraps to htmlParseLookupSequence()
 *
 * Returns the index to the current parsing point if the full sequence is available, -1 otherwise.
 */
#[cfg(LIBXML_PUSH_ENABLED)]
fn htmlParseLookupCommentEnd(ctxt: htmlParserCtxtPtr) -> i32 {
    let ctxtPtr = unsafe { &mut *ctxt };
    let inputPtr = unsafe { &mut *(*ctxt).input };
    let mut mark: i32 = 0;
    let mut cur: i32 = unsafe { inputPtr.cur.offset_from(inputPtr.base) as i32 };
    while mark >= 0 {
        mark = htmlParseLookupSequence(ctxt, '-' as xmlChar, '-' as xmlChar, 0 as xmlChar, 0);
        if (mark < 0)
            || (NXT(ctxt, mark + 2) == '>' as i32)
            || ((NXT(ctxt, mark + 2) == '!' as i32) && (NXT(ctxt, mark + 3) == '>' as i32))
        {
            return mark;
        }
        ctxtPtr.checkIndex = (cur + mark + 1) as i64
    }
    return mark;
}
/* *
 * htmlParseTryOrFinish:
 * @ctxt:  an HTML parser context
 * @terminate:  last chunk indicator
 *
 * Try to progress on parsing
 *
 * Returns zero if no parsing was possible
 */
#[cfg(LIBXML_PUSH_ENABLED)]
pub fn htmlParseTryOrFinish(ctxt: htmlParserCtxtPtr, terminate: i32) -> i32 {
    let mut DEBUG_PUSH: i32 = 0;
    match () {
        #[cfg(DEBUG_PUSH)]
        _ => {
            DEBUG_PUSH = 1;
        }
        #[cfg(not(DEBUG_PUSH))]
        _ => {}
    };
    let mut ret: i32 = 0;
    let mut in_0: htmlParserInputPtr;
    let mut avail: ptrdiff_t = 0 as ptrdiff_t;
    let mut cur: xmlChar;
    let mut next: xmlChar;
    let mut node_info: htmlParserNodeInfo = htmlParserNodeInfo {
        node: 0 as *const _xmlNode,
        begin_pos: 0,
        begin_line: 0,
        end_pos: 0,
        end_line: 0,
    };
    let ctxtPtr = unsafe { &mut *ctxt };
    if DEBUG_PUSH != 0 {
        match ctxtPtr.instate {
            XML_PARSER_EOF => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try EOF\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_START => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try START\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_MISC => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try MISC\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_COMMENT => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try COMMENT\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_PROLOG => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try PROLOG\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_START_TAG => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try START_TAG\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_CONTENT => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try CONTENT\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_CDATA_SECTION => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try CDATA_SECTION\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_END_TAG => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try END_TAG\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_ENTITY_DECL => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try ENTITY_DECL\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_ENTITY_VALUE => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try ENTITY_VALUE\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_ATTRIBUTE_VALUE => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try ATTRIBUTE_VALUE\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_DTD => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try DTD\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_EPILOG => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try EPILOG\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_PI => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try PI\n" as *const u8 as *const i8
                );
            }
            XML_PARSER_SYSTEM_LITERAL => {
                __xmlGenericError_safe_macro!(
                    __xmlGenericErrorContext_safe(),
                    b"HPP: try SYSTEM_LITERAL\n" as *const u8 as *const i8
                );
            }
            _ => {}
        }
    }
    let mut sax_condition = false;
    loop {
        in_0 = ctxtPtr.input;
        if in_0.is_null() {
            break;
        }
        let in_0Ptr = unsafe { &mut *in_0 };
        if in_0Ptr.buf.is_null() {
            unsafe { avail = in_0Ptr.length as i64 - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64 }
        } else {
            unsafe {
                avail = xmlBufUse_safe((*(*in_0).buf).buffer) as ptrdiff_t
                    - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64
            }
        }
        if avail == 0 && terminate != 0 {
            htmlAutoCloseOnEnd(ctxt);
            if ctxtPtr.nameNr == 0 && ctxtPtr.instate != XML_PARSER_EOF {
                /*
                 * SAX: end of the document processing.
                 */
                ctxtPtr.instate = XML_PARSER_EOF;
                sax_condition =
                    unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endDocument.is_some() };
                if sax_condition {
                    let saxPtr = unsafe { &mut *(*ctxt).sax };
                    xmlSAXHandler_endDocument_safe(saxPtr.endDocument, ctxtPtr.userData);
                }
            }
        }
        if avail < 1 {
            break;
        }
        /*
         * This is done to make progress and avoid an infinite loop
         * if a parsing attempt was aborted by hitting a NUL byte. After
         * changing htmlCurrentChar, this probably isn't necessary anymore.
         * We should consider removing this check.
         */
        unsafe {
            cur = *in_0Ptr.cur.offset(0);
        }
        if cur == 0 {
            SKIP(ctxt, 1);
            continue;
        }
        match ctxtPtr.instate {
            XML_PARSER_EOF => {
                //Document parsing is done !
                break;
            }
            XML_PARSER_START => {
                //Very first chars read from the document flow
                unsafe {
                    cur = *in_0Ptr.cur.offset(0);
                }
                if IS_BLANK_CH(cur as i32) {
                    htmlSkipBlankChars(ctxt);
                    if in_0Ptr.buf.is_null() {
                        unsafe {
                            avail =
                                in_0Ptr.length as i64 - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64
                        }
                    } else {
                        unsafe {
                            avail = xmlBufUse_safe((*(*in_0).buf).buffer) as ptrdiff_t
                                - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64
                        }
                    }
                }
                sax_condition = unsafe {
                    !ctxtPtr.sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some()
                };
                if sax_condition {
                    let saxPtr = unsafe { &mut *(*ctxt).sax };
                    unsafe {
                        xmlSAXHandler_setDocumentLocator_safe(
                            saxPtr.setDocumentLocator,
                            ctxtPtr.userData,
                            __xmlDefaultSAXLocator_safe(),
                        )
                    };
                }
                sax_condition = unsafe {
                    !ctxtPtr.sax.is_null()
                        && (*(*ctxt).sax).startDocument.is_some()
                        && ctxtPtr.disableSAX == 0
                };
                if sax_condition {
                    let saxPtr = unsafe { &mut *(*ctxt).sax };
                    xmlSAXHandler_startDocument_safe(saxPtr.startDocument, ctxtPtr.userData);
                }
                unsafe {
                    cur = *in_0Ptr.cur.offset(0);
                }
                unsafe {
                    next = *in_0Ptr.cur.offset(1);
                }
                if cur == '<' as u8
                    && next == '!' as u8
                    && UPP(ctxt, 2) == 'D' as i32
                    && UPP(ctxt, 3) == 'O' as i32
                    && UPP(ctxt, 4) == 'C' as i32
                    && UPP(ctxt, 5) == 'T' as i32
                    && UPP(ctxt, 6) == 'Y' as i32
                    && UPP(ctxt, 7) == 'P' as i32
                    && UPP(ctxt, 8) == 'E' as i32
                {
                    if terminate == 0
                        && htmlParseLookupSequence(
                            ctxt,
                            '>' as xmlChar,
                            0 as xmlChar,
                            0 as xmlChar,
                            1,
                        ) < 0
                    {
                        break;
                    }
                    if DEBUG_PUSH != 0 {
                        __xmlGenericError_safe_macro!(
                            __xmlGenericErrorContext_safe(),
                            b"HPP: Parsing internal subset\n" as *const u8 as *const i8
                        );
                    }
                    htmlParseDocTypeDecl(ctxt);
                    ctxtPtr.instate = XML_PARSER_PROLOG;
                    if DEBUG_PUSH != 0 {
                        __xmlGenericError_safe_macro!(
                            __xmlGenericErrorContext_safe(),
                            b"HPP: entering PROLOG\n" as *const u8 as *const i8
                        );
                    }
                } else {
                    ctxtPtr.instate = XML_PARSER_MISC;
                    if DEBUG_PUSH != 0 {
                        __xmlGenericError_safe_macro!(
                            __xmlGenericErrorContext_safe(),
                            b"HPP: entering MISC\n" as *const u8 as *const i8
                        );
                    }
                }
            }
            XML_PARSER_MISC => {
                htmlSkipBlankChars(ctxt);
                if in_0Ptr.buf.is_null() {
                    unsafe {
                        avail = in_0Ptr.length as i64 - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64
                    }
                } else {
                    unsafe {
                        avail = xmlBufUse_safe((*(*in_0).buf).buffer) as ptrdiff_t
                            - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64
                    }
                }
                if avail < 1 {
                    break;
                }
                if avail < 2 {
                    if terminate == 0 {
                        break;
                    } else {
                        next = ' ' as xmlChar;
                    }
                } else {
                    unsafe { next = *in_0Ptr.cur.offset(1) }
                }
                unsafe {
                    cur = *in_0Ptr.cur.offset(0);
                }
                unsafe {
                    if cur == '<' as u8
                        && next == '!' as u8
                        && *in_0Ptr.cur.offset(2) == '-' as u8
                        && *in_0Ptr.cur.offset(3) == '-' as u8
                    {
                        if terminate == 0 && htmlParseLookupCommentEnd(ctxt) < 0 {
                            break;
                        }
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: Parsing Comment\n" as *const u8 as *const i8
                            );
                        }
                        htmlParseComment(ctxt);
                        ctxtPtr.instate = XML_PARSER_MISC;
                    } else if cur == '<' as u8 && next == '?' as u8 {
                        if terminate == 0
                            && htmlParseLookupSequence(
                                ctxt,
                                '>' as xmlChar,
                                0 as xmlChar,
                                0 as xmlChar,
                                0,
                            ) < 0
                        {
                            break;
                        }
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: Parsing PI\n" as *const u8 as *const i8
                            );
                        }
                        htmlParsePI(ctxt);
                        ctxtPtr.instate = XML_PARSER_MISC;
                    } else if cur == '<' as u8
                        && next == '!' as u8
                        && UPP(ctxt, 2) == 'D' as i32
                        && UPP(ctxt, 3) == 'O' as i32
                        && UPP(ctxt, 4) == 'C' as i32
                        && UPP(ctxt, 5) == 'T' as i32
                        && UPP(ctxt, 6) == 'Y' as i32
                        && UPP(ctxt, 7) == 'P' as i32
                        && UPP(ctxt, 8) == 'E' as i32
                    {
                        if terminate == 0
                            && htmlParseLookupSequence(
                                ctxt,
                                '>' as xmlChar,
                                0 as xmlChar,
                                0 as xmlChar,
                                1,
                            ) < 0
                        {
                            break;
                        }
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: Parsing internal subset\n" as *const u8 as *const i8
                            );
                        }
                        htmlParseDocTypeDecl(ctxt);
                        ctxtPtr.instate = XML_PARSER_PROLOG;
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: entering PROLOG\n" as *const u8 as *const i8
                            );
                        }
                    } else if cur == '<' as u8 && next == '!' as u8 && avail < 9 {
                        break;
                    } else {
                        ctxtPtr.instate = XML_PARSER_CONTENT;
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: entering START_TAG\n" as *const u8 as *const i8
                            );
                        }
                    }
                }
            }
            XML_PARSER_PROLOG => {
                htmlSkipBlankChars(ctxt);
                if in_0Ptr.buf.is_null() {
                    unsafe {
                        avail = in_0Ptr.length as i64 - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64
                    }
                } else {
                    unsafe {
                        avail = xmlBufUse_safe((*(*in_0).buf).buffer) as ptrdiff_t
                            - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64
                    }
                }
                if avail < 2 {
                    break;
                }
                unsafe {
                    cur = *in_0Ptr.cur.offset(0);
                    next = *in_0Ptr.cur.offset(1);
                    if cur == '<' as u8
                        && next == '!' as u8
                        && *in_0Ptr.cur.offset(2) == '-' as u8
                        && *in_0Ptr.cur.offset(3) == '-' as u8
                    {
                        if terminate == 0 && htmlParseLookupCommentEnd(ctxt) < 0 {
                            break;
                        }
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: Parsing Comment\n" as *const u8 as *const i8
                            );
                        }
                        htmlParseComment(ctxt);
                        ctxtPtr.instate = XML_PARSER_PROLOG;
                    } else if cur == '<' as u8 && next == '?' as u8 {
                        if terminate == 0
                            && htmlParseLookupSequence(
                                ctxt,
                                '>' as xmlChar,
                                0 as xmlChar,
                                0 as xmlChar,
                                0,
                            ) < 0
                        {
                            break;
                        }
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: Parsing PI\n" as *const u8 as *const i8
                            );
                        }
                        htmlParsePI(ctxt);
                        ctxtPtr.instate = XML_PARSER_PROLOG;
                    } else if cur == '<' as u8 && next == '!' as u8 && avail < 4 {
                        break;
                    } else {
                        ctxtPtr.instate = XML_PARSER_CONTENT;
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: entering START_TAG\n" as *const u8 as *const i8
                            );
                        }
                    }
                }
            }
            XML_PARSER_EPILOG => {
                if in_0Ptr.buf.is_null() {
                    unsafe {
                        avail = in_0Ptr.length as i64 - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64
                    }
                } else {
                    unsafe {
                        avail = xmlBufUse_safe((*(*in_0).buf).buffer) as ptrdiff_t
                            - in_0Ptr.cur.offset_from(in_0Ptr.base) as i64
                    }
                }
                if avail < 1 {
                    break;
                }
                unsafe {
                    cur = *in_0Ptr.cur.offset(0);
                }
                if IS_BLANK_CH(cur as i32) {
                    htmlParseCharData(ctxt);
                    break;
                }
                if avail < 2 {
                    break;
                }
                unsafe {
                    next = *in_0Ptr.cur.offset(1);
                }
                unsafe {
                    if cur == '<' as u8
                        && next == '!' as u8
                        && *in_0Ptr.cur.offset(2) == '-' as u8
                        && *in_0Ptr.cur.offset(3) == '-' as u8
                    {
                        if terminate == 0 && htmlParseLookupCommentEnd(ctxt) < 0 {
                            break;
                        }
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: Parsing Comment\n" as *const u8 as *const i8
                            );
                        }
                        htmlParseComment(ctxt);
                        ctxtPtr.instate = XML_PARSER_EPILOG;
                    } else if cur == '<' as u8 && next == '?' as u8 {
                        if terminate == 0
                            && htmlParseLookupSequence(
                                ctxt,
                                '>' as xmlChar,
                                0 as xmlChar,
                                0 as xmlChar,
                                0,
                            ) < 0
                        {
                            break;
                        }
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: Parsing PI\n" as *const u8 as *const i8
                            );
                        }
                        htmlParsePI(ctxt);
                        ctxtPtr.instate = XML_PARSER_EPILOG;
                    } else if cur == '<' as u8 && next == '!' as u8 && avail < 4 {
                        break;
                    } else {
                        ctxtPtr.errNo = XML_ERR_DOCUMENT_END as i32;
                        ctxtPtr.wellFormed = 0;
                        ctxtPtr.instate = XML_PARSER_EOF;
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: entering EOF\n" as *const u8 as *const i8
                            );
                        }
                        sax_condition =
                            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endDocument.is_some();
                        if sax_condition {
                            let mut saxPtr = &mut *(*ctxt).sax;
                            xmlSAXHandler_endDocument_safe(saxPtr.endDocument, ctxtPtr.userData);
                        }
                        break;
                    }
                }
            }
            XML_PARSER_START_TAG => {
                let name: *const xmlChar;
                let mut failed: i32;
                let info: *const htmlElemDesc;
                if avail < 1 {
                    break;
                }
                if avail < 2 {
                    if terminate == 0 {
                        break;
                    } else {
                        next = ' ' as xmlChar;
                    }
                } else {
                    unsafe { next = *in_0Ptr.cur.offset(1) }
                }
                unsafe {
                    cur = *in_0Ptr.cur.offset(0);
                }
                if cur != '<' as u8 {
                    ctxtPtr.instate = XML_PARSER_CONTENT;
                    if DEBUG_PUSH != 0 {
                        __xmlGenericError_safe_macro!(
                            __xmlGenericErrorContext_safe(),
                            b"HPP: entering CONTENT\n" as *const u8 as *const i8
                        );
                    }
                    continue;
                }
                if next == '/' as u8 {
                    ctxtPtr.instate = XML_PARSER_END_TAG;
                    ctxtPtr.checkIndex = 0;
                    if DEBUG_PUSH != 0 {
                        __xmlGenericError_safe_macro!(
                            __xmlGenericErrorContext_safe(),
                            b"HPP: entering END_TAG\n" as *const u8 as *const i8
                        );
                    }
                    continue;
                }
                if terminate == 0
                    && htmlParseLookupSequence(ctxt, '>' as xmlChar, 0 as xmlChar, 0 as xmlChar, 1)
                        < 0
                {
                    break;
                }
                /* Capture start position */
                let mut inputPtr = unsafe { &mut *(*ctxt).input };
                if ctxtPtr.record_info != 0 {
                    unsafe {
                        node_info.begin_pos = inputPtr
                            .consumed
                            .wrapping_add(inputPtr.cur.offset_from(inputPtr.base) as u64);
                    }
                    node_info.begin_line = inputPtr.line as u64;
                }
                failed = htmlParseStartTag(ctxt);
                name = ctxtPtr.name;
                if failed == -1 || name.is_null() {
                    if CUR(ctxt) == '>' as i32 {
                        unsafe { xmlNextChar_safe(ctxt) };
                    }
                    continue;
                }
                /*
                 * Lookup the info for that element.
                 */
                info = htmlTagLookup(name);
                if info.is_null() {
                    htmlParseErr(
                        ctxt,
                        XML_HTML_UNKNOWN_TAG,
                        b"Tag %s invalid\n\x00" as *const u8 as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                }
                if CUR(ctxt) == '/' as i32 && NXT(ctxt, 1) == '>' as i32 {
                    SKIP(ctxt, 2);
                    sax_condition =
                        unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() };
                    if sax_condition {
                        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                        xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, name);
                    }
                    htmlnamePop(ctxt);
                    ctxtPtr.instate = XML_PARSER_CONTENT;
                    if DEBUG_PUSH != 0 {
                        __xmlGenericError_safe_macro!(
                            __xmlGenericErrorContext_safe(),
                            b"HPP: entering CONTENT\n" as *const u8 as *const i8
                        );
                    }
                    continue;
                }
                if CUR(ctxt) == '>' as i32 {
                    unsafe { xmlNextChar_safe(ctxt) };
                } else {
                    htmlParseErr(
                        ctxt,
                        XML_ERR_GT_REQUIRED,
                        b"Couldn\'t find end of Start Tag %s\n\x00" as *const u8 as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                    /*
                     * end of parsing of this node.
                     */
                    if unsafe { xmlStrEqual_safe(name, ctxtPtr.name) } != 0 {
                        unsafe { nodePop_safe(ctxt) };
                        htmlnamePop(ctxt);
                    }
                    if ctxtPtr.record_info != 0 {
                        htmlNodeInfoPush(ctxt, &mut node_info);
                    }
                    ctxtPtr.instate = XML_PARSER_CONTENT;
                    if DEBUG_PUSH != 0 {
                        __xmlGenericError_safe_macro!(
                            __xmlGenericErrorContext_safe(),
                            b"HPP: entering CONTENT\n" as *const u8 as *const i8
                        );
                    }
                    continue;
                }
                let flag = unsafe { !info.is_null() && (*info).empty as i32 != 0 };
                if flag {
                    sax_condition =
                        unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() };
                    if sax_condition {
                        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                        xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, name);
                    }
                    htmlnamePop(ctxt);
                }

                if ctxtPtr.record_info != 0 {
                    htmlNodeInfoPush(ctxt, &mut node_info);
                }
                ctxtPtr.instate = XML_PARSER_CONTENT;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_CONTENT => {
                let mut chr: [xmlChar; 2] = [0 as xmlChar, 0 as xmlChar];
                if ctxtPtr.token != 0 {
                    chr[0 as usize] = ctxtPtr.token as xmlChar;
                    htmlCheckParagraph(ctxt);
                    sax_condition =
                        unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some() };
                    if sax_condition {
                        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                        xmlSAXHandler_characters_safe(
                            saxPtr.characters,
                            ctxtPtr.userData,
                            chr.as_mut_ptr(),
                            1,
                        );
                    }
                    ctxtPtr.token = 0;
                    ctxtPtr.checkIndex = 0
                }
                if avail == 1 && terminate != 0 {
                    unsafe {
                        cur = *in_0Ptr.cur.offset(0);
                    }
                    if cur != '<' as u8 && cur != '&' as u8 {
                        if !ctxtPtr.sax.is_null() {
                            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                            chr[0 as usize] = cur;
                            if IS_BLANK_CH(cur as i32) {
                                if ctxtPtr.keepBlanks != 0 {
                                    if saxPtr.characters.is_some() {
                                        xmlSAXHandler_characters_safe(
                                            saxPtr.characters,
                                            ctxtPtr.userData,
                                            chr.as_mut_ptr(),
                                            1,
                                        );
                                    }
                                } else {
                                    if saxPtr.ignorableWhitespace.is_some() {
                                        xmlSAXHandler_ignorableWhitespace_safe(
                                            saxPtr.ignorableWhitespace,
                                            ctxtPtr.userData,
                                            chr.as_mut_ptr(),
                                            1,
                                        );
                                    }
                                }
                            } else {
                                htmlCheckParagraph(ctxt);
                                if saxPtr.characters.is_some() {
                                    xmlSAXHandler_characters_safe(
                                        saxPtr.characters,
                                        ctxtPtr.userData,
                                        chr.as_mut_ptr(),
                                        1,
                                    );
                                }
                            }
                        }
                        ctxtPtr.token = 0;
                        ctxtPtr.checkIndex = 0;
                        unsafe {
                            in_0Ptr.cur = in_0Ptr.cur.offset(1);
                        }
                        continue;
                    }
                }
                if avail < 2 {
                    break;
                }
                unsafe {
                    cur = *in_0Ptr.cur.offset(0);
                }
                unsafe {
                    next = *in_0Ptr.cur.offset(1);
                }
                //cons = (*ctxt).nbChars;
                if unsafe {
                    xmlStrEqual_safe(
                        ctxtPtr.name,
                        b"script\x00" as *const u8 as *const i8 as *mut xmlChar,
                    )
                } != 0
                    || unsafe {
                        xmlStrEqual_safe(
                            ctxtPtr.name,
                            b"style\x00" as *const u8 as *const i8 as *mut xmlChar,
                        )
                    } != 0
                {
                    /*
                     * Handle SCRIPT/STYLE separately
                     */
                    if terminate == 0 {
                        let mut idx: i32 = 0;
                        let mut val: xmlChar = 0;
                        idx = htmlParseLookupSequence(
                            ctxt,
                            '<' as xmlChar,
                            '/' as xmlChar,
                            0 as xmlChar,
                            0,
                        );
                        if idx < 0 {
                            break;
                        }
                        unsafe {
                            val = *in_0Ptr.cur.offset((idx + 2) as isize);
                        }
                        if val == 0 {
                            break;
                        }
                    }
                    htmlParseScript(ctxt);
                    if cur == '<' as u8 && next == '/' as u8 {
                        ctxtPtr.instate = XML_PARSER_END_TAG;
                        ctxtPtr.checkIndex = 0;
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: entering END_TAG\n" as *const u8 as *const i8
                            );
                        }
                        continue;
                    }
                } else {
                    if cur == '<' as u8
                        && next == '!' as u8
                        && UPP(ctxt, 2) == 'D' as i32
                        && UPP(ctxt, 3) == 'O' as i32
                        && UPP(ctxt, 4) == 'C' as i32
                        && UPP(ctxt, 5) == 'T' as i32
                        && UPP(ctxt, 6) == 'Y' as i32
                        && UPP(ctxt, 7) == 'P' as i32
                        && UPP(ctxt, 8) == 'E' as i32
                    {
                        if terminate == 0
                            && htmlParseLookupSequence(
                                ctxt,
                                '>' as xmlChar,
                                0 as xmlChar,
                                0 as xmlChar,
                                1,
                            ) < 0
                        {
                            break;
                        }
                        htmlParseErr(
                            ctxt,
                            XML_HTML_STRUCURE_ERROR,
                            b"Misplaced DOCTYPE declaration\n\x00" as *const u8 as *const i8,
                            b"DOCTYPE\x00" as *const u8 as *const i8 as *mut xmlChar,
                            0 as *const xmlChar,
                        );
                        htmlParseDocTypeDecl(ctxt);
                    } else if cur == '<' as u8
                        && next == '!' as u8
                        && unsafe { *in_0Ptr.cur.offset(2) } == '-' as u8
                        && unsafe { *in_0Ptr.cur.offset(3) } == '-' as u8
                    {
                        if terminate == 0 && htmlParseLookupCommentEnd(ctxt) < 0 {
                            break;
                        }
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: Parsing Comment\n" as *const u8 as *const i8
                            );
                        }
                        htmlParseComment(ctxt);
                        ctxtPtr.instate = XML_PARSER_CONTENT;
                    } else if cur == '<' as u8 && next == '?' as u8 {
                        if terminate == 0
                            && htmlParseLookupSequence(
                                ctxt,
                                '>' as xmlChar,
                                0 as xmlChar,
                                0 as xmlChar,
                                0,
                            ) < 0
                        {
                            break;
                        }
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: Parsing PI\n" as *const u8 as *const i8
                            );
                        }
                        htmlParsePI(ctxt);
                        ctxtPtr.instate = XML_PARSER_CONTENT;
                    } else if cur == '<' as u8 && next == '!' as u8 && avail < 4 {
                        break;
                    } else if cur == '<' as u8 && next == '/' as u8 {
                        ctxtPtr.instate = XML_PARSER_END_TAG;
                        ctxtPtr.checkIndex = 0;
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: entering END_TAG\n" as *const u8 as *const i8
                            );
                        }
                        continue;
                    } else if cur == '<' as u8 {
                        if terminate == 0 && next == 0 {
                            break;
                        }
                        if IS_ASCII_LETTER(next as i32)
                            || next == '_' as u8
                            || next == ':' as u8
                            || next == '.' as u8
                        {
                            ctxtPtr.instate = XML_PARSER_START_TAG;
                            ctxtPtr.checkIndex = 0;
                            if DEBUG_PUSH != 0 {
                                __xmlGenericError_safe_macro!(
                                    __xmlGenericErrorContext_safe(),
                                    b"HPP: entering START_TAG\n" as *const u8 as *const i8
                                );
                            }
                        } else {
                            htmlParseErr(
                                ctxt,
                                XML_ERR_NAME_REQUIRED,
                                b"htmlParseTryOrFinish: invalid element name\n\x00" as *const u8
                                    as *const i8,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                            );
                            htmlCheckParagraph(ctxt);
                            sax_condition = unsafe {
                                !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some()
                            };
                            if sax_condition {
                                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                                unsafe {
                                    xmlSAXHandler_characters_safe(
                                        saxPtr.characters,
                                        ctxtPtr.userData,
                                        &*in_0Ptr.cur.offset(0),
                                        1,
                                    )
                                };
                            }
                            unsafe { xmlNextChar_safe(ctxt) };
                        }
                        continue;
                    } else {
                        if terminate == 0
                            && htmlParseLookupSequence(
                                ctxt,
                                '<' as xmlChar,
                                0 as xmlChar,
                                0 as xmlChar,
                                0,
                            ) < 0
                        {
                            break;
                        }
                        ctxtPtr.checkIndex = 0;
                        if DEBUG_PUSH != 0 {
                            __xmlGenericError_safe_macro!(
                                __xmlGenericErrorContext_safe(),
                                b"HPP: Parsing char data\n" as *const u8 as *const i8
                            );
                        }
                        while ctxtPtr.instate != XML_PARSER_START_TAG
                            && cur != '<' as u8
                            && in_0Ptr.cur < in_0Ptr.end
                        {
                            if cur == '&' as u8 {
                                htmlParseReference(ctxt);
                            } else {
                                htmlParseCharData(ctxt);
                            }
                            unsafe {
                                cur = *in_0Ptr.cur.offset(0);
                            }
                        }
                    }
                }
            }
            XML_PARSER_END_TAG => {
                if avail < 2 {
                    break;
                }
                if terminate == 0
                    && htmlParseLookupSequence(ctxt, '>' as xmlChar, 0 as xmlChar, 0 as xmlChar, 0)
                        < 0
                {
                    break;
                }
                htmlParseEndTag(ctxt);
                if ctxtPtr.nameNr == 0 {
                    ctxtPtr.instate = XML_PARSER_EPILOG;
                } else {
                    ctxtPtr.instate = XML_PARSER_CONTENT;
                }
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_CDATA_SECTION => {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"HPP: internal error, state == CDATA\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_DTD => {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"HPP: internal error, state == DTD\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_COMMENT => {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"HPP: internal error, state == COMMENT\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_PI => {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"HPP: internal error, state == PI\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_ENTITY_DECL => {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"HPP: internal error, state == ENTITY_DECL\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_ENTITY_VALUE => {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"HPP: internal error, state == ENTITY_VALUE\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_ATTRIBUTE_VALUE => {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"HPP: internal error, state == ATTRIBUTE_VALUE\n\x00" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                ctxtPtr.instate = XML_PARSER_START_TAG;
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering START_TAG\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_SYSTEM_LITERAL => {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"HPP: internal error, state == XML_PARSER_SYSTEM_LITERAL\n\x00" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_IGNORE => {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"HPP: internal error, state == XML_PARSER_IGNORE\n\x00" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            XML_PARSER_PUBLIC_LITERAL => {
                htmlParseErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"HPP: internal error, state == XML_PARSER_LITERAL\n\x00" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0;
                if DEBUG_PUSH != 0 {
                    __xmlGenericError_safe_macro!(
                        __xmlGenericErrorContext_safe(),
                        b"HPP: entering CONTENT\n" as *const u8 as *const i8
                    );
                }
            }
            _ => {}
        }
    }
    /* bad cut of input */
    if avail == 0 && terminate != 0 {
        htmlAutoCloseOnEnd(ctxt);
        if ctxtPtr.nameNr == 0 && ctxtPtr.instate != XML_PARSER_EOF {
            /*
             * SAX: end of the document processing.
             */
            ctxtPtr.instate = XML_PARSER_EOF;
            sax_condition =
                unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endDocument.is_some() };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_endDocument_safe(saxPtr.endDocument, ctxtPtr.userData);
            }
        }
    }
    if ctxtPtr.options & HTML_PARSE_NODEFDTD as i32 == 0
        && !ctxtPtr.myDoc.is_null()
        && (terminate != 0
            || ctxtPtr.instate == XML_PARSER_EOF
            || ctxtPtr.instate == XML_PARSER_EPILOG)
    {
        let dtd: xmlDtdPtr;
        dtd = unsafe { xmlGetIntSubset_safe(ctxtPtr.myDoc as *const xmlDoc) };
        if dtd.is_null() {
            let mut myDocPtr = unsafe { &mut *(*ctxt).myDoc };
            myDocPtr.intSubset = unsafe {
                xmlCreateIntSubset_safe(
                    ctxtPtr.myDoc,
                    b"html\x00" as *const u8 as *const i8 as *mut xmlChar,
                    b"-//W3C//DTD HTML 4.0 Transitional//EN\x00" as *const u8 as *const i8
                        as *mut xmlChar,
                    b"http://www.w3.org/TR/REC-html40/loose.dtd\x00" as *const u8 as *const i8
                        as *mut xmlChar,
                )
            }
        }
        if DEBUG_PUSH != 0 {
            __xmlGenericError_safe_macro!(
                __xmlGenericErrorContext_safe(),
                b"HPP: done %d\n" as *const u8 as *const i8,
                ret
            );
        }
    }
    return ret;
}
/* *
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
#[cfg(LIBXML_PUSH_ENABLED)]
pub fn htmlParseChunk(ctxt: htmlParserCtxtPtr, chunk: *const i8, size: i32, terminate: i32) -> i32 {
    let mut DEBUG_PUSH: i32 = 0;
    match () {
        #[cfg(DEBUG_PUSH)]
        _ => {
            DEBUG_PUSH = 1;
        }
        #[cfg(not(DEBUG_PUSH))]
        _ => {}
    };
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"htmlParseChunk: context error\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            return XML_ERR_INTERNAL_ERROR as i32;
        }
    }
    let ctxtPtr = unsafe { &mut *ctxt };
    let flag1 = unsafe {
        size > 0
            && !chunk.is_null()
            && !ctxtPtr.input.is_null()
            && !(*(*ctxt).input).buf.is_null()
            && ctxtPtr.instate != XML_PARSER_EOF
    };
    if flag1 {
        let inputPtr = unsafe { &mut *(*ctxt).input };
        let bufPtr = unsafe { &mut *(*(*ctxt).input).buf };
        let mut base: size_t = unsafe { xmlBufGetInputBase_safe(bufPtr.buffer, ctxtPtr.input) };
        let mut cur: size_t = unsafe { inputPtr.cur.offset_from(inputPtr.base) as size_t };
        let mut res: i32;
        res = unsafe { xmlParserInputBufferPush_safe(inputPtr.buf, size, chunk) };
        unsafe { xmlBufSetInputBaseCur_safe(bufPtr.buffer, ctxtPtr.input, base, cur) };
        if res < 0 {
            ctxtPtr.errNo = XML_PARSER_EOF as i32;
            ctxtPtr.disableSAX = 1;
            return XML_PARSER_EOF as i32;
        }
        if DEBUG_PUSH != 0 {
            __xmlGenericError_safe_macro!(
                __xmlGenericErrorContext_safe(),
                b"HPP: pushed %d\n" as *const u8 as *const i8,
                size
            );
        }
    } else if ctxtPtr.instate != XML_PARSER_EOF {
        let flag2 = unsafe { !ctxtPtr.input.is_null() && !(*(*ctxt).input).buf.is_null() };
        if flag2 {
            let inputPtr = unsafe { &mut *(*ctxt).input };
            let mut in_0: xmlParserInputBufferPtr = inputPtr.buf;
            let in_0Ptr = unsafe { &mut *in_0 };
            if !in_0Ptr.encoder.is_null() && !in_0Ptr.buffer.is_null() && !in_0Ptr.raw.is_null() {
                let mut nbchars: i32;
                let mut base_0: size_t =
                    unsafe { xmlBufGetInputBase_safe(in_0Ptr.buffer, ctxtPtr.input) };
                let mut current: size_t =
                    unsafe { inputPtr.cur.offset_from(inputPtr.base) as i64 as size_t };
                nbchars = unsafe { xmlCharEncInput_safe(in_0, terminate) };
                unsafe {
                    xmlBufSetInputBaseCur_safe(in_0Ptr.buffer, ctxtPtr.input, base_0, current)
                };
                if nbchars < 0 {
                    htmlParseErr(
                        ctxt,
                        XML_ERR_INVALID_ENCODING,
                        b"encoder error\n\x00" as *const u8 as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                    return XML_ERR_INVALID_ENCODING as i32;
                }
            }
        }
    }
    htmlParseTryOrFinish(ctxt, terminate);
    if terminate != 0 {
        if ctxtPtr.instate != XML_PARSER_EOF
            && ctxtPtr.instate != XML_PARSER_EPILOG
            && ctxtPtr.instate != XML_PARSER_MISC
        {
            ctxtPtr.errNo = XML_ERR_DOCUMENT_END as i32;
            ctxtPtr.wellFormed = 0
        }
        if ctxtPtr.instate != XML_PARSER_EOF {
            let mut sax_condition =
                unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endDocument.is_some() };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_endDocument_safe(saxPtr.endDocument, ctxtPtr.userData);
            }
        }
        ctxtPtr.instate = XML_PARSER_EOF
    }
    return ctxtPtr.errNo as i32;
}
/* ***********************************************************************
 *									*
 *			User entry points				*
 *									*
 ************************************************************************/
/* *
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
#[cfg(LIBXML_PUSH_ENABLED)]
pub fn htmlCreatePushParserCtxt(
    sax: htmlSAXHandlerPtr,
    user_data: *mut (),
    chunk: *const i8,
    size: i32,
    filename: *const i8,
    enc: xmlCharEncoding,
) -> htmlParserCtxtPtr {
    let mut DEBUG_PUSH: i32 = 0;
    match () {
        #[cfg(DEBUG_PUSH)]
        _ => {
            DEBUG_PUSH = 1;
        }
        #[cfg(not(DEBUG_PUSH))]
        _ => {}
    };
    let ctxt: htmlParserCtxtPtr;
    let inputStream: htmlParserInputPtr;
    let buf: xmlParserInputBufferPtr;
    unsafe { xmlInitParser_safe() };
    buf = unsafe { xmlAllocParserInputBuffer_safe(enc) };
    if buf.is_null() {
        return 0 as htmlParserCtxtPtr;
    }
    ctxt = htmlNewParserCtxt();
    if ctxt.is_null() {
        unsafe { xmlFreeParserInputBuffer_safe(buf) };
        return 0 as htmlParserCtxtPtr;
    }
    let ctxtPtr = unsafe { &mut *ctxt };
    let bufPtr = unsafe { &mut *buf };
    if enc as i32 == XML_CHAR_ENCODING_UTF8 || !bufPtr.encoder.is_null() {
        ctxtPtr.charset = XML_CHAR_ENCODING_UTF8
    }
    if !sax.is_null() {
        if ctxtPtr.sax != unsafe { __htmlDefaultSAXHandler_safe() } as xmlSAXHandlerPtr {
            unsafe { xmlFree_safe(ctxtPtr.sax as *mut ()) };
        }
        ctxtPtr.sax = unsafe { xmlMalloc_safe(::std::mem::size_of::<htmlSAXHandler>() as u64) }
            as htmlSAXHandlerPtr;
        if ctxtPtr.sax.is_null() {
            unsafe { xmlFree_safe(buf as *mut ()) };
            unsafe { xmlFree_safe(ctxt as *mut ()) };
            return 0 as htmlParserCtxtPtr;
        }
        unsafe {
            memcpy_safe(
                ctxtPtr.sax as *mut (),
                sax as *const (),
                ::std::mem::size_of::<htmlSAXHandler>() as u64,
            )
        };
        if !user_data.is_null() {
            ctxtPtr.userData = user_data
        }
    }
    if filename.is_null() {
        ctxtPtr.directory = 0 as *mut i8
    } else {
        ctxtPtr.directory = unsafe { xmlParserGetDirectory_safe(filename) }
    }
    inputStream = htmlNewInputStream(ctxt);
    if inputStream.is_null() {
        unsafe { xmlFreeParserCtxt_safe(ctxt) };
        unsafe { xmlFree_safe(buf as *mut ()) };
        return 0 as htmlParserCtxtPtr;
    }
    let mut inputStreamPtr = unsafe { &mut *inputStream };
    if filename.is_null() {
        inputStreamPtr.filename = 0 as *const i8
    } else {
        inputStreamPtr.filename =
            unsafe { xmlCanonicPath_safe(filename as *const xmlChar) } as *mut i8
    }
    inputStreamPtr.buf = buf;
    unsafe { xmlBufResetInput_safe(bufPtr.buffer, inputStream) };
    unsafe { inputPush_safe(ctxt, inputStream) };
    let mut flag = unsafe {
        size > 0 && !chunk.is_null() && !ctxtPtr.input.is_null() && !(*(*ctxt).input).buf.is_null()
    };
    if flag {
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        let mut bufPtr = unsafe { &mut *(*(*ctxt).input).buf };
        let mut base: size_t = unsafe { xmlBufGetInputBase_safe(bufPtr.buffer, ctxtPtr.input) };
        let mut cur: size_t = unsafe { inputPtr.cur.offset_from(inputPtr.base) as size_t };
        unsafe { xmlParserInputBufferPush_safe(inputPtr.buf, size, chunk) };
        unsafe { xmlBufSetInputBaseCur_safe(bufPtr.buffer, ctxtPtr.input, base, cur) };
        if DEBUG_PUSH != 0 {
            __xmlGenericError_safe_macro!(
                __xmlGenericErrorContext_safe(),
                b"HPP: pushed %d\n" as *const u8 as *const i8,
                size
            );
        }
    }
    ctxtPtr.progressive = 1;
    return ctxt;
}
/* LIBXML_PUSH_ENABLED */
/* *
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

pub fn htmlSAXParseDoc(
    cur: *const xmlChar,
    encoding: *const i8,
    sax: htmlSAXHandlerPtr,
    userData: *mut (),
) -> htmlDocPtr {
    let ret: htmlDocPtr;
    let ctxt: htmlParserCtxtPtr;
    unsafe { xmlInitParser_safe() };
    if cur.is_null() {
        return 0 as htmlDocPtr;
    }
    ctxt = htmlCreateDocParserCtxt(cur, encoding);
    if ctxt.is_null() {
        return 0 as htmlDocPtr;
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if !sax.is_null() {
        if !ctxtPtr.sax.is_null() {
            unsafe { xmlFree_safe(ctxtPtr.sax as *mut ()) };
        }
        ctxtPtr.sax = sax;
        ctxtPtr.userData = userData
    }
    htmlParseDocument(ctxt);
    ret = ctxtPtr.myDoc;
    if !sax.is_null() {
        ctxtPtr.sax = 0 as *mut _xmlSAXHandler;
        ctxtPtr.userData = 0 as *mut ()
    }
    htmlFreeParserCtxt(ctxt);
    return ret;
}
/* *
 * htmlParseDoc:
 * @cur:  a pointer to an array of xmlChar
 * @encoding:  a free form C string describing the HTML document encoding, or NULL
 *
 * parse an HTML in-memory document and build a tree.
 *
 * Returns the resulting document tree
 */

pub fn htmlParseDoc(cur: *const xmlChar, encoding: *const i8) -> htmlDocPtr {
    return htmlSAXParseDoc(cur, encoding, 0 as htmlSAXHandlerPtr, 0 as *mut ());
}
/* *
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

pub fn htmlCreateFileParserCtxt(filename: *const i8, encoding: *const i8) -> htmlParserCtxtPtr {
    let ctxt: htmlParserCtxtPtr;
    let inputStream: htmlParserInputPtr;
    let canonicFilename: *mut i8;
    /* htmlCharEncoding enc; */
    let content: *mut xmlChar;
    let mut content_line: *mut xmlChar = b"charset=\x00" as *const u8 as *const i8 as *mut xmlChar;
    if filename.is_null() {
        return 0 as htmlParserCtxtPtr;
    }
    ctxt = htmlNewParserCtxt();
    if ctxt.is_null() {
        return 0 as htmlParserCtxtPtr;
    }
    canonicFilename = unsafe { xmlCanonicPath_safe(filename as *const xmlChar) } as *mut i8;
    if canonicFilename.is_null() {
        match () {
            #[cfg(LIBXML_SAX1_ENABLED)]
            _ => {
                unsafe {
                    if (*__xmlDefaultSAXHandler()).error.is_some() {
                        //todo-unsafe-__xmlDefaultSAXHandler
                        (*__xmlDefaultSAXHandler())
                            .error
                            .expect("non-null function pointer")(
                            0 as *mut (),
                            b"out of memory\n\x00" as *const u8 as *const i8,
                        );
                    }
                }
            }
            #[cfg(not(LIBXML_SAX1_ENABLED))]
            _ => {}
        };
        unsafe { xmlFreeParserCtxt_safe(ctxt) };
        return 0 as htmlParserCtxtPtr;
    }
    inputStream = unsafe { xmlLoadExternalEntity_safe(canonicFilename, 0 as *const i8, ctxt) };
    unsafe { xmlFree_safe(canonicFilename as *mut ()) };
    if inputStream.is_null() {
        unsafe { xmlFreeParserCtxt_safe(ctxt) };
        return 0 as htmlParserCtxtPtr;
    }
    unsafe { inputPush_safe(ctxt, inputStream) };
    /* set encoding */
    if !encoding.is_null() {
        let mut l: size_t = unsafe { strlen_safe(encoding) };
        if l < 1000 {
            content = unsafe {
                xmlMallocAtomic_safe(
                    (xmlStrlen_safe(content_line) as u64)
                        .wrapping_add(l)
                        .wrapping_add(1),
                )
            } as *mut xmlChar;
            if !content.is_null() {
                unsafe { strcpy_safe(content as *mut i8, content_line as *mut i8) };
                unsafe { strcat_safe(content as *mut i8, encoding as *mut i8) };
                htmlCheckEncoding(ctxt, content);
                unsafe { xmlFree_safe(content as *mut ()) };
            }
        }
    }
    return ctxt;
}
/* *
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

pub fn htmlSAXParseFile(
    filename: *const i8,
    encoding: *const i8,
    sax: htmlSAXHandlerPtr,
    userData: *mut (),
) -> htmlDocPtr {
    let ret: htmlDocPtr;
    let ctxt: htmlParserCtxtPtr;
    let mut oldsax: htmlSAXHandlerPtr = 0 as htmlSAXHandlerPtr;
    unsafe { xmlInitParser_safe() };
    ctxt = htmlCreateFileParserCtxt(filename, encoding);
    if ctxt.is_null() {
        return 0 as htmlDocPtr;
    }
    let ctxtPtr = unsafe { &mut *ctxt };
    if !sax.is_null() {
        oldsax = ctxtPtr.sax;
        ctxtPtr.sax = sax;
        ctxtPtr.userData = userData
    }
    htmlParseDocument(ctxt);
    ret = ctxtPtr.myDoc;
    if !sax.is_null() {
        ctxtPtr.sax = oldsax;
        ctxtPtr.userData = 0 as *mut ()
    }
    htmlFreeParserCtxt(ctxt);
    return ret;
}
/* *
 * htmlParseFile:
 * @filename:  the filename
 * @encoding:  a free form C string describing the HTML document encoding, or NULL
 *
 * parse an HTML file and build a tree. Automatic support for ZLIB/Compress
 * compressed document is provided by default if found at compile-time.
 *
 * Returns the resulting document tree
 */

pub fn htmlParseFile(filename: *const i8, encoding: *const i8) -> htmlDocPtr {
    return htmlSAXParseFile(filename, encoding, 0 as htmlSAXHandlerPtr, 0 as *mut ());
}
/* *
 * htmlHandleOmittedElem:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for handling HTML omitted tags.
 *
 * Returns the last value for 0 for no handling, 1 for auto insertion.
 */

pub fn htmlHandleOmittedElem(val: i32) -> i32 {
    let mut old: i32 = getHtmlOmittedDefaultValue();
    setHtmlOmittedDefaultValue(val);
    return old;
}
/* *
 * htmlElementAllowedHere:
 * @parent: HTML parent element
 * @elt: HTML element
 *
 * Checks whether an HTML element may be a direct child of a parent element.
 * Note - doesn't check for deprecated elements
 *
 * Returns 1 if allowed; 0 otherwise.
 */

pub fn htmlElementAllowedHere(parent: *const htmlElemDesc, elt: *const xmlChar) -> i32 {
    let mut p: *mut *const i8;
    unsafe {
        if elt.is_null() || parent.is_null() || (*parent).subelts.is_null() {
            return 0;
        }

        p = (*parent).subelts;
        while !(*p).is_null() {
            if xmlStrcmp_safe(*p as *const xmlChar, elt) == 0 {
                return 1;
            }
            p = p.offset(1)
        }
    }
    return 0;
}
/* *
 * htmlElementStatusHere:
 * @parent: HTML parent element
 * @elt: HTML element
 *
 * Checks whether an HTML element may be a direct child of a parent element.
 * and if so whether it is valid or deprecated.
 *
 * Returns one of HTML_VALID, HTML_DEPRECATED, HTML_INVALID
 */

pub fn htmlElementStatusHere(parent: *const htmlElemDesc, elt: *const htmlElemDesc) -> htmlStatus {
    if parent.is_null() || elt.is_null() {
        return HTML_INVALID;
    }
    let eltPtr = unsafe { &*elt };
    if htmlElementAllowedHere(parent, eltPtr.name as *const xmlChar) == 0 {
        return HTML_INVALID;
    }
    return if eltPtr.dtd as i32 == 0 {
        HTML_VALID as i32
    } else {
        HTML_DEPRECATED as i32
    } as htmlStatus;
}
/* *
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

pub fn htmlAttrAllowed(elt: *const htmlElemDesc, attr: *const xmlChar, legacy: i32) -> htmlStatus {
    let mut p: *mut *const i8;
    if elt.is_null() || attr.is_null() {
        return HTML_INVALID;
    }
    let eltPtr = unsafe { &*elt };
    if !eltPtr.attrs_req.is_null() {
        p = eltPtr.attrs_req;
        unsafe {
            while !(*p).is_null() {
                if xmlStrcmp_safe(*p as *const xmlChar, attr) == 0 {
                    return HTML_REQUIRED;
                }
                p = p.offset(1)
            }
        }
    }
    if !eltPtr.attrs_opt.is_null() {
        p = eltPtr.attrs_opt;
        unsafe {
            while !(*p).is_null() {
                if xmlStrcmp_safe(*p as *const xmlChar, attr) == 0 {
                    return HTML_VALID;
                }
                p = p.offset(1)
            }
        }
    }
    if legacy != 0 && !eltPtr.attrs_depr.is_null() {
        p = eltPtr.attrs_depr;
        unsafe {
            while !(*p).is_null() {
                if xmlStrcmp_safe(*p as *const xmlChar, attr) == 0 {
                    return HTML_DEPRECATED;
                }
                p = p.offset(1)
            }
        }
    }
    return HTML_INVALID;
}
/* *
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

pub fn htmlNodeStatus(node: htmlNodePtr, legacy: i32) -> htmlStatus {
    if node.is_null() {
        return HTML_INVALID;
    }
    let nodePtr = unsafe { &mut *node };
    let parentPtr = unsafe { &mut *(*node).parent };
    match nodePtr.type_0 as u32 {
        XML_ELEMENT_NODE => {
            return if legacy != 0 {
                (if htmlElementAllowedHere(htmlTagLookup(parentPtr.name), nodePtr.name) != 0 {
                    HTML_VALID as i32
                } else {
                    HTML_INVALID as i32
                }) as u32
            } else {
                htmlElementStatusHere(htmlTagLookup(parentPtr.name), htmlTagLookup(nodePtr.name))
                    as u32
            } as htmlStatus;
        }
        XML_ATTRIBUTE_NODE => {
            return htmlAttrAllowed(htmlTagLookup(parentPtr.name), nodePtr.name, legacy);
        }
        _ => {
            return HTML_NA;
        }
    };
}
/* ***********************************************************************
 *									*
 *	New set (2.6.0) of simpler and more flexible APIs		*
 *									*
 ************************************************************************/
/* *
 * DICT_FREE:
 * @str:  a string
 *
 * Free a string if it is not owned by the "dict" dictionary in the
 * current scope
 */
/* *
 * htmlCtxtReset:
 * @ctxt: an HTML parser context
 *
 * Reset a parser context
 */

pub fn htmlCtxtReset(ctxt: htmlParserCtxtPtr) {
    let mut input: xmlParserInputPtr;
    let dict: xmlDictPtr;
    if ctxt.is_null() {
        return;
    }
    unsafe { xmlInitParser_safe() };
    let mut ctxtPtr = unsafe { &mut *ctxt };
    dict = ctxtPtr.dict;
    loop {
        input = unsafe { inputPop_safe(ctxt) };
        if input.is_null() {
            break;
        }
        /* Non consuming */
        unsafe { xmlFreeInputStream_safe(input) };
    }
    ctxtPtr.inputNr = 0;
    ctxtPtr.input = 0 as xmlParserInputPtr;
    ctxtPtr.spaceNr = 0;
    if !ctxtPtr.spaceTab.is_null() {
        unsafe {
            *ctxtPtr.spaceTab.offset(0 as isize) = -1;
            ctxtPtr.space = &mut *ctxtPtr.spaceTab.offset(0) as *mut i32
        }
    } else {
        ctxtPtr.space = 0 as *mut i32
    }
    ctxtPtr.nodeNr = 0;
    ctxtPtr.node = 0 as xmlNodePtr;
    ctxtPtr.nameNr = 0;
    ctxtPtr.name = 0 as *const xmlChar;
    if !ctxtPtr.version.is_null()
        && (dict.is_null() || unsafe { xmlDictOwns_safe(dict, ctxtPtr.version) } == 0)
    {
        unsafe { xmlFree_safe(ctxtPtr.version as *mut i8 as *mut ()) };
    }
    ctxtPtr.version = 0 as *const xmlChar;
    if !ctxtPtr.encoding.is_null()
        && (dict.is_null() || unsafe { xmlDictOwns_safe(dict, ctxtPtr.encoding) } == 0)
    {
        unsafe { xmlFree_safe(ctxtPtr.encoding as *mut i8 as *mut ()) };
    }
    ctxtPtr.encoding = 0 as *const xmlChar;
    if !ctxtPtr.directory.is_null()
        && (dict.is_null()
            || unsafe { xmlDictOwns_safe(dict, ctxtPtr.directory as *const xmlChar) } == 0)
    {
        unsafe { xmlFree_safe(ctxtPtr.directory as *mut ()) };
    }
    ctxtPtr.directory = 0 as *mut i8;
    if !ctxtPtr.extSubURI.is_null()
        && (dict.is_null()
            || unsafe { xmlDictOwns_safe(dict, ctxtPtr.extSubURI as *const xmlChar) } == 0)
    {
        unsafe { xmlFree_safe(ctxtPtr.extSubURI as *mut i8 as *mut ()) };
    }
    ctxtPtr.extSubURI = 0 as *mut xmlChar;
    if !ctxtPtr.extSubSystem.is_null()
        && (dict.is_null()
            || unsafe { xmlDictOwns_safe(dict, ctxtPtr.extSubSystem as *const xmlChar) } == 0)
    {
        unsafe { xmlFree_safe(ctxtPtr.extSubSystem as *mut i8 as *mut ()) };
    }
    ctxtPtr.extSubSystem = 0 as *mut xmlChar;
    if !ctxtPtr.myDoc.is_null() {
        unsafe { xmlFreeDoc_safe(ctxtPtr.myDoc) };
    }
    ctxtPtr.myDoc = 0 as xmlDocPtr;
    ctxtPtr.standalone = -1;
    ctxtPtr.hasExternalSubset = 0;
    ctxtPtr.hasPErefs = 0;
    ctxtPtr.html = 1;
    ctxtPtr.external = 0;
    ctxtPtr.instate = XML_PARSER_START;
    ctxtPtr.token = 0;
    ctxtPtr.wellFormed = 1;
    ctxtPtr.nsWellFormed = 1;
    ctxtPtr.disableSAX = 0;
    ctxtPtr.valid = 1;
    ctxtPtr.vctxt.userData = ctxt as *mut ();
    ctxtPtr.vctxt.error = Some(
        xmlParserValidityError as unsafe extern "C" fn(_: *mut (), _: *const i8, _: ...) -> (),
    );
    ctxtPtr.vctxt.warning = Some(
        xmlParserValidityWarning as unsafe extern "C" fn(_: *mut (), _: *const i8, _: ...) -> (),
    );
    ctxtPtr.record_info = 0;
    ctxtPtr.checkIndex = 0;
    ctxtPtr.inSubset = 0;
    ctxtPtr.errNo = XML_ERR_OK as i32;
    ctxtPtr.depth = 0;
    ctxtPtr.charset = XML_CHAR_ENCODING_NONE as i32;
    ctxtPtr.catalogs = 0 as *mut ();
    unsafe { xmlInitNodeInfoSeq_safe(&mut ctxtPtr.node_seq) };
    if !ctxtPtr.attsDefault.is_null() {
        unsafe {
            xmlHashFree_safe(
                ctxtPtr.attsDefault,
                Some(
                    xmlHashDefaultDeallocator
                        as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
                ),
            )
        };
        ctxtPtr.attsDefault = 0 as xmlHashTablePtr
    }
    if !ctxtPtr.attsSpecial.is_null() {
        unsafe { xmlHashFree_safe(ctxtPtr.attsSpecial, None) };
        ctxtPtr.attsSpecial = 0 as xmlHashTablePtr
    };
}
/* *
 * htmlCtxtUseOptions:
 * @ctxt: an HTML parser context
 * @options:  a combination of htmlParserOption(s)
 *
 * Applies the options to the parser context
 *
 * Returns 0 in case of success, the set of unknown or unimplemented options
 *         in case of error.
 */

pub fn htmlCtxtUseOptions(ctxt: htmlParserCtxtPtr, mut options: i32) -> i32 {
    if ctxt.is_null() {
        return -1;
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut saxPtr = unsafe { &mut *(*ctxt).sax };
    if options & HTML_PARSE_NOWARNING as i32 != 0 {
        saxPtr.warning = None;
        ctxtPtr.vctxt.warning = None;
        options -= XML_PARSE_NOWARNING as i32;
        ctxtPtr.options |= XML_PARSE_NOWARNING as i32
    }
    if options & HTML_PARSE_NOERROR as i32 != 0 {
        saxPtr.error = None;
        ctxtPtr.vctxt.error = None;
        saxPtr.fatalError = None;
        options -= XML_PARSE_NOERROR as i32;
        ctxtPtr.options |= XML_PARSE_NOERROR as i32
    }
    if options & HTML_PARSE_PEDANTIC as i32 != 0 {
        ctxtPtr.pedantic = 1;
        options -= XML_PARSE_PEDANTIC as i32;
        ctxtPtr.options |= XML_PARSE_PEDANTIC as i32
    } else {
        ctxtPtr.pedantic = 0
    }
    if options & XML_PARSE_NOBLANKS as i32 != 0 {
        ctxtPtr.keepBlanks = 0;
        saxPtr.ignorableWhitespace = Some(
            xmlSAX2IgnorableWhitespace
                as unsafe extern "C" fn(_: *mut (), _: *const xmlChar, _: i32) -> (),
        );
        options -= XML_PARSE_NOBLANKS as i32;
        ctxtPtr.options |= XML_PARSE_NOBLANKS as i32
    } else {
        ctxtPtr.keepBlanks = 1
    }
    if options & HTML_PARSE_RECOVER as i32 != 0 {
        ctxtPtr.recovery = 1;
        options -= HTML_PARSE_RECOVER as i32
    } else {
        ctxtPtr.recovery = 0
    }
    if options & HTML_PARSE_COMPACT as i32 != 0 {
        ctxtPtr.options |= HTML_PARSE_COMPACT as i32;
        options -= HTML_PARSE_COMPACT as i32
    }
    if options & XML_PARSE_HUGE as i32 != 0 {
        ctxtPtr.options |= XML_PARSE_HUGE as i32;
        options -= XML_PARSE_HUGE as i32
    }
    if options & HTML_PARSE_NODEFDTD as i32 != 0 {
        ctxtPtr.options |= HTML_PARSE_NODEFDTD as i32;
        options -= HTML_PARSE_NODEFDTD as i32
    }
    if options & HTML_PARSE_IGNORE_ENC as i32 != 0 {
        ctxtPtr.options |= HTML_PARSE_IGNORE_ENC as i32;
        options -= HTML_PARSE_IGNORE_ENC as i32
    }
    if options & HTML_PARSE_NOIMPLIED as i32 != 0 {
        ctxtPtr.options |= HTML_PARSE_NOIMPLIED as i32;
        options -= HTML_PARSE_NOIMPLIED as i32
    }
    ctxtPtr.dictNames = 0;
    return options;
}
/* *
 * htmlDoRead:
 * @ctxt:  an HTML parser context
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 * @reuse:  keep the context for reuse
 *
 * Common front-end for the htmlRead functions
 *
 * Returns the resulting document tree or NULL
 */
fn htmlDoRead(
    ctxt: htmlParserCtxtPtr,
    URL: *const i8,
    encoding: *const i8,
    options: i32,
    reuse: i32,
) -> htmlDocPtr {
    let mut ret: htmlDocPtr;
    htmlCtxtUseOptions(ctxt, options);
    let mut ctxtPtr = unsafe { &mut *ctxt };
    ctxtPtr.html = 1;
    if !encoding.is_null() {
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        let mut hdlr: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
        hdlr = unsafe { xmlFindCharEncodingHandler_safe(encoding) };
        if !hdlr.is_null() {
            unsafe { xmlSwitchToEncoding_safe(ctxt, hdlr) };
            if !inputPtr.encoding.is_null() {
                unsafe { xmlFree_safe(inputPtr.encoding as *mut xmlChar as *mut ()) };
            }
            inputPtr.encoding = unsafe { xmlStrdup_safe(encoding as *mut xmlChar) }
        }
    }
    let mut flag = unsafe {
        !URL.is_null() && !ctxtPtr.input.is_null() && (*(*ctxt).input).filename.is_null()
    };
    if flag {
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        inputPtr.filename = unsafe { xmlStrdup_safe(URL as *const xmlChar) } as *mut i8
    }
    htmlParseDocument(ctxt);
    ret = ctxtPtr.myDoc;
    ctxtPtr.myDoc = 0 as xmlDocPtr;
    if reuse == 0 {
        unsafe {
            if ctxtPtr.dictNames != 0 && !ret.is_null() && (*ret).dict == ctxtPtr.dict {
                ctxtPtr.dict = 0 as xmlDictPtr
            }
        }
        unsafe { xmlFreeParserCtxt_safe(ctxt) };
    }
    return ret;
}
/* *
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

pub fn htmlReadDoc(
    cur: *const xmlChar,
    URL: *const i8,
    encoding: *const i8,
    options: i32,
) -> htmlDocPtr {
    let mut ctxt: htmlParserCtxtPtr;
    if cur.is_null() {
        return 0 as htmlDocPtr;
    }
    unsafe { xmlInitParser_safe() };
    ctxt = htmlCreateDocParserCtxt(cur, 0 as *const i8);
    if ctxt.is_null() {
        return 0 as htmlDocPtr;
    }
    return htmlDoRead(ctxt, URL, encoding, options, 0);
}
/* *
 * htmlReadFile:
 * @filename:  a file or URL
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of htmlParserOption(s)
 *
 * parse an XML file from the filesystem or the network.
 *
 * Returns the resulting document tree
 */

pub fn htmlReadFile(filename: *const i8, encoding: *const i8, options: i32) -> htmlDocPtr {
    let mut ctxt: htmlParserCtxtPtr;
    unsafe { xmlInitParser_safe() };
    ctxt = htmlCreateFileParserCtxt(filename, encoding);
    if ctxt.is_null() {
        return 0 as htmlDocPtr;
    }
    return htmlDoRead(ctxt, 0 as *const i8, 0 as *const i8, options, 0);
}
/* *
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

pub fn htmlReadMemory(
    buffer: *const i8,
    size: i32,
    URL: *const i8,
    encoding: *const i8,
    options: i32,
) -> htmlDocPtr {
    let mut ctxt: htmlParserCtxtPtr;
    unsafe { xmlInitParser_safe() };
    ctxt = unsafe { xmlCreateMemoryParserCtxt_safe(buffer, size) };
    if ctxt.is_null() {
        return 0 as htmlDocPtr;
    }
    unsafe { htmlDefaultSAXHandlerInit_safe() };
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if !ctxtPtr.sax.is_null() {
        unsafe {
            memcpy_safe(
                ctxtPtr.sax as *mut (),
                __htmlDefaultSAXHandler_safe() as *const (),
                ::std::mem::size_of::<xmlSAXHandlerV1>() as u64,
            )
        };
    }
    return htmlDoRead(ctxt, URL, encoding, options, 0);
}
/* *
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

pub fn htmlReadFd(fd: i32, URL: *const i8, encoding: *const i8, options: i32) -> htmlDocPtr {
    let mut ctxt: htmlParserCtxtPtr;
    let mut input: xmlParserInputBufferPtr;
    let mut stream: xmlParserInputPtr;
    if fd < 0 {
        return 0 as htmlDocPtr;
    }
    unsafe { xmlInitParser_safe() };
    unsafe { xmlInitParser_safe() };
    input = unsafe { xmlParserInputBufferCreateFd_safe(fd, XML_CHAR_ENCODING_NONE) };
    if input.is_null() {
        return 0 as htmlDocPtr;
    }
    ctxt = unsafe { xmlNewParserCtxt_safe() };
    if ctxt.is_null() {
        unsafe { xmlFreeParserInputBuffer_safe(input) };
        return 0 as htmlDocPtr;
    }
    stream = unsafe { xmlNewIOInputStream_safe(ctxt, input, XML_CHAR_ENCODING_NONE) };
    if stream.is_null() {
        unsafe { xmlFreeParserInputBuffer_safe(input) };
        unsafe { xmlFreeParserCtxt_safe(ctxt) };
        return 0 as htmlDocPtr;
    }
    unsafe { inputPush_safe(ctxt, stream) };
    return htmlDoRead(ctxt, URL, encoding, options, 0);
}
/* *
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

pub fn htmlReadIO(
    ioread: xmlInputReadCallback,
    ioclose: xmlInputCloseCallback,
    ioctx: *mut (),
    URL: *const i8,
    encoding: *const i8,
    options: i32,
) -> htmlDocPtr {
    let mut ctxt: htmlParserCtxtPtr;
    let mut input: xmlParserInputBufferPtr;
    let mut stream: xmlParserInputPtr;
    if ioread.is_none() {
        return 0 as htmlDocPtr;
    }
    unsafe { xmlInitParser_safe() };
    input = unsafe {
        xmlParserInputBufferCreateIO_safe(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE)
    };
    if input.is_null() {
        if ioclose.is_some() {
            ioclose_safe(ioclose, ioctx);
        }
        return 0 as htmlDocPtr;
    }
    ctxt = htmlNewParserCtxt();
    if ctxt.is_null() {
        unsafe { xmlFreeParserInputBuffer_safe(input) };
        return 0 as htmlDocPtr;
    }
    stream = unsafe { xmlNewIOInputStream_safe(ctxt, input, XML_CHAR_ENCODING_NONE) };
    if stream.is_null() {
        unsafe { xmlFreeParserInputBuffer_safe(input) };
        unsafe { xmlFreeParserCtxt_safe(ctxt) };
        return 0 as htmlDocPtr;
    }
    unsafe { inputPush_safe(ctxt, stream) };
    return htmlDoRead(ctxt, URL, encoding, options, 0);
}
/* *
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

pub fn htmlCtxtReadDoc(
    ctxt: htmlParserCtxtPtr,
    cur: *const xmlChar,
    URL: *const i8,
    encoding: *const i8,
    options: i32,
) -> htmlDocPtr {
    let mut stream: xmlParserInputPtr;
    if cur.is_null() {
        return 0 as htmlDocPtr;
    }
    if ctxt.is_null() {
        return 0 as htmlDocPtr;
    }
    unsafe { xmlInitParser_safe() };
    htmlCtxtReset(ctxt);
    stream = unsafe { xmlNewStringInputStream_safe(ctxt, cur) };
    if stream.is_null() {
        return 0 as htmlDocPtr;
    }
    unsafe { inputPush_safe(ctxt, stream) };
    return htmlDoRead(ctxt, URL, encoding, options, 1);
}
/* *
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

pub fn htmlCtxtReadFile(
    ctxt: htmlParserCtxtPtr,
    filename: *const i8,
    encoding: *const i8,
    options: i32,
) -> htmlDocPtr {
    let mut stream: xmlParserInputPtr;
    if filename.is_null() {
        return 0 as htmlDocPtr;
    }
    if ctxt.is_null() {
        return 0 as htmlDocPtr;
    }
    unsafe { xmlInitParser_safe() };
    htmlCtxtReset(ctxt);
    stream = unsafe { xmlLoadExternalEntity_safe(filename, 0 as *const i8, ctxt) };
    if stream.is_null() {
        return 0 as htmlDocPtr;
    }
    unsafe { inputPush_safe(ctxt, stream) };
    return htmlDoRead(ctxt, 0 as *const i8, encoding, options, 1);
}
/* *
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

pub fn htmlCtxtReadMemory(
    ctxt: htmlParserCtxtPtr,
    buffer: *const i8,
    size: i32,
    URL: *const i8,
    encoding: *const i8,
    options: i32,
) -> htmlDocPtr {
    let mut input: xmlParserInputBufferPtr;
    let mut stream: xmlParserInputPtr;
    if ctxt.is_null() {
        return 0 as htmlDocPtr;
    }
    if buffer.is_null() {
        return 0 as htmlDocPtr;
    }
    unsafe { xmlInitParser_safe() };
    htmlCtxtReset(ctxt);
    input = unsafe { xmlParserInputBufferCreateMem_safe(buffer, size, XML_CHAR_ENCODING_NONE) };
    if input.is_null() {
        return 0 as htmlDocPtr;
    }
    stream = unsafe { xmlNewIOInputStream_safe(ctxt, input, XML_CHAR_ENCODING_NONE) };
    if stream.is_null() {
        unsafe { xmlFreeParserInputBuffer_safe(input) };
        return 0 as htmlDocPtr;
    }
    unsafe { inputPush_safe(ctxt, stream) };
    return htmlDoRead(ctxt, URL, encoding, options, 1);
}
/* *
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

pub fn htmlCtxtReadFd(
    ctxt: htmlParserCtxtPtr,
    fd: i32,
    URL: *const i8,
    encoding: *const i8,
    options: i32,
) -> htmlDocPtr {
    let mut input: xmlParserInputBufferPtr;
    let mut stream: xmlParserInputPtr;
    if fd < 0 {
        return 0 as htmlDocPtr;
    }
    if ctxt.is_null() {
        return 0 as htmlDocPtr;
    }
    unsafe { xmlInitParser_safe() };
    htmlCtxtReset(ctxt);
    input = unsafe { xmlParserInputBufferCreateFd_safe(fd, XML_CHAR_ENCODING_NONE) };
    if input.is_null() {
        return 0 as htmlDocPtr;
    }
    stream = unsafe { xmlNewIOInputStream_safe(ctxt, input, XML_CHAR_ENCODING_NONE) };
    if stream.is_null() {
        unsafe { xmlFreeParserInputBuffer_safe(input) };
        return 0 as htmlDocPtr;
    }
    unsafe { inputPush_safe(ctxt, stream) };
    return htmlDoRead(ctxt, URL, encoding, options, 1);
}
/* *
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

pub fn htmlCtxtReadIO(
    ctxt: htmlParserCtxtPtr,
    ioread: xmlInputReadCallback,
    ioclose: xmlInputCloseCallback,
    ioctx: *mut (),
    URL: *const i8,
    encoding: *const i8,
    options: i32,
) -> htmlDocPtr {
    let mut input: xmlParserInputBufferPtr;
    let mut stream: xmlParserInputPtr;
    if ioread.is_none() {
        return 0 as htmlDocPtr;
    }
    if ctxt.is_null() {
        return 0 as htmlDocPtr;
    }
    unsafe { xmlInitParser_safe() };
    htmlCtxtReset(ctxt);
    input = unsafe {
        xmlParserInputBufferCreateIO_safe(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE)
    };
    if input.is_null() {
        if ioclose.is_some() {
            ioclose_safe(ioclose, ioctx);
        }
        return 0 as htmlDocPtr;
    }
    stream = unsafe { xmlNewIOInputStream_safe(ctxt, input, XML_CHAR_ENCODING_NONE) };
    if stream.is_null() {
        unsafe { xmlFreeParserInputBuffer_safe(input) };
        return 0 as htmlDocPtr;
    }
    unsafe { inputPush_safe(ctxt, stream) };
    return htmlDoRead(ctxt, URL, encoding, options, 1);
}
/* LIBXML_HTML_ENABLED */

fn xmlSAXHandler_endElement_safe(func: endElementSAXFunc, arg1: *mut (), arg2: *const xmlChar) {
    unsafe {
        func.expect("non-null function pointer")(arg1, arg2);
    }
}

fn xmlSAXHandler_startElement_safe(
    func: startElementSAXFunc,
    arg1: *mut (),
    arg2: *const xmlChar,
    arg3: *mut *const xmlChar,
) {
    unsafe {
        func.expect("non-null function pointer")(arg1, arg2, arg3);
    }
}

fn xmlSAXHandler_cdataBlock_safe(
    func: cdataBlockSAXFunc,
    arg1: *mut (),
    arg2: *const xmlChar,
    arg3: i32,
) {
    unsafe {
        func.expect("non-null function pointer")(arg1, arg2, arg3);
    }
}

fn xmlSAXHandler_characters_safe(
    func: charactersSAXFunc,
    arg1: *mut (),
    arg2: *const xmlChar,
    arg3: i32,
) {
    unsafe {
        func.expect("non-null function pointer")(arg1, arg2, arg3);
    }
}

fn xmlSAXHandler_ignorableWhitespace_safe(
    func: ignorableWhitespaceSAXFunc,
    arg1: *mut (),
    arg2: *const xmlChar,
    arg3: i32,
) {
    unsafe {
        func.expect("non-null function pointer")(arg1, arg2, arg3);
    }
}

fn xmlSAXHandler_processingInstruction_safe(
    func: processingInstructionSAXFunc,
    arg1: *mut (),
    arg2: *const xmlChar,
    arg3: *const xmlChar,
) {
    unsafe {
        func.expect("non-null function pointer")(arg1, arg2, arg3);
    }
}

fn xmlSAXHandler_comment_safe(func: commentSAXFunc, arg1: *mut (), arg2: *const xmlChar) {
    unsafe {
        func.expect("non-null function pointer")(arg1, arg2);
    }
}

fn xmlSAXHandler_internalSubset_safe(
    func: internalSubsetSAXFunc,
    arg1: *mut (),
    arg2: *const xmlChar,
    arg3: *const xmlChar,
    arg4: *const xmlChar,
) {
    unsafe {
        func.expect("non-null function pointer")(arg1, arg2, arg3, arg4);
    }
}

fn xmlSAXHandler_endDocument_safe(func: endDocumentSAXFunc, arg1: *mut ()) {
    unsafe {
        func.expect("non-null function pointer")(arg1);
    }
}

fn xmlSAXHandler_startDocument_safe(func: startDocumentSAXFunc, arg1: *mut ()) {
    unsafe {
        func.expect("non-null function pointer")(arg1);
    }
}

fn xmlSAXHandler_setDocumentLocator_safe(
    func: setDocumentLocatorSAXFunc,
    arg1: *mut (),
    arg2: xmlSAXLocatorPtr,
) {
    unsafe {
        func.expect("non-null function pointer")(arg1, arg2);
    }
}

fn ioclose_safe(func: xmlInputCloseCallback, arg1: *mut ()) {
    unsafe {
        func.expect("non-null function pointer")(arg1);
    }
}

fn getHtmlEndPriority(index: usize) -> elementPriority {
    unsafe {
        return htmlEndPriority[index];
    }
}

fn getHtmlOmittedDefaultValue() -> i32 {
    unsafe {
        return htmlOmittedDefaultValue;
    }
}

fn setHtmlOmittedDefaultValue(val: i32) {
    unsafe {
        htmlOmittedDefaultValue = val;
    }
}

fn getHtmlNoContentElements(index: usize) -> *const i8 {
    unsafe {
        return htmlNoContentElements[index];
    }
}

fn getHtmlScriptAttributes(index: usize) -> *const i8 {
    unsafe {
        return htmlScriptAttributes[index];
    }
}

fn getHtml40EntitiesTable(index: usize) -> htmlEntityDesc {
    unsafe {
        return html40EntitiesTable[index];
    }
}

fn getAllowPCData(index: usize) -> *const i8 {
    unsafe {
        return allowPCData[index];
    }
}

// 暴露函数
#[no_mangle]
pub extern "C" fn libxml2_test_htmlErrMemory(ctxt: xmlParserCtxtPtr, extra: *const i8) {
    htmlErrMemory(ctxt, extra);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseContent(ctxt: xmlParserCtxtPtr) {
    htmlParseContent(ctxt);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlNodeInfoPush(ctxt: xmlParserCtxtPtr) -> i32 {
    let mut value = 0 as *mut htmlParserNodeInfo;
    return htmlNodeInfoPush(ctxt, value);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlNodeInfoPop(
    ctxt: xmlParserCtxtPtr,
    node_info: *mut xmlParserNodeInfo,
) -> *mut () {
    let ctxtPtr = unsafe { &mut *ctxt };
    unsafe {
        ctxtPtr.nodeInfoTab = xmlRealloc_safe(
            ctxtPtr.nodeInfoTab as *mut htmlParserNodeInfo as *mut (),
            (ctxtPtr.nodeInfoNr as u64) * (::std::mem::size_of::<xmlParserNodeInfo>()) as u64,
        ) as *mut htmlParserNodeInfo;
        *ctxtPtr.nodeInfoTab.offset(0) = *node_info;
    }
    return htmlNodeInfoPop(ctxt) as *mut ();
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParserFinishElementParsing(ctxt: xmlParserCtxtPtr) {
    return htmlParserFinishElementParsing(ctxt);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseElementInternal(ctxt: xmlParserCtxtPtr) {
    return htmlParseElementInternal(ctxt);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseErr(ctxt: xmlParserCtxtPtr) {
    return htmlParseErr(
        ctxt,
        XML_ERR_OK,
        0 as *const i8,
        0 as *const xmlChar,
        0 as *const xmlChar,
    );
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseErrInt(ctxt: xmlParserCtxtPtr) {
    return htmlParseErrInt(ctxt, XML_ERR_OK, 0 as *const i8, 0);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlCurrentChar(ctxt: xmlParserCtxtPtr, len: *mut i32) -> i32 {
    return htmlCurrentChar(ctxt, len);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlAutoClose(ctxt: xmlParserCtxtPtr, newtag: *const xmlChar) {
    return htmlAutoClose(ctxt, newtag);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlCheckImplied(ctxt: xmlParserCtxtPtr, newtag: *const xmlChar) {
    return htmlCheckImplied(ctxt, newtag);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseAttValue(ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    return htmlParseAttValue(ctxt);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseSystemLiteral(ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    return htmlParseSystemLiteral(ctxt);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParsePubidLiteral(ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    return htmlParsePubidLiteral(ctxt);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseCharDataInternal(ctxt: xmlParserCtxtPtr, readahead: i32) {
    return htmlParseCharDataInternal(ctxt, readahead);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlCreateDocParserCtxt(
    cur: *const xmlChar,
    encoding: *const i8,
) -> htmlParserCtxtPtr {
    return htmlCreateDocParserCtxt(cur, encoding);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseLookupSequence(
    ctxt: xmlParserCtxtPtr,
    first: xmlChar,
    next: xmlChar,
    third: xmlChar,
    ignoreattrval: i32,
) -> i32 {
    return htmlParseLookupSequence(ctxt, first, next, third, ignoreattrval);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseStartTag(ctxt: xmlParserCtxtPtr) -> i32 {
    return htmlParseStartTag(ctxt);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseEndTag(ctxt: xmlParserCtxtPtr) -> i32 {
    return htmlParseEndTag(ctxt);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlCheckEncodingDirect(
    ctxt: xmlParserCtxtPtr,
    encoding: *const xmlChar,
) {
    return htmlCheckEncodingDirect(ctxt, encoding);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlCheckEncoding(ctxt: xmlParserCtxtPtr, attvalue: *const xmlChar) {
    return htmlCheckEncoding(ctxt, attvalue);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlCheckMeta(ctxt: xmlParserCtxtPtr, atts: *mut *const xmlChar) {
    return htmlCheckMeta(ctxt, atts);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlCheckParagraph(ctxt: xmlParserCtxtPtr) -> i32 {
    return htmlCheckParagraph(ctxt);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseScript(ctxt: xmlParserCtxtPtr) {
    return htmlParseScript(ctxt);
}

#[no_mangle]
pub extern "C" fn libxml2_test_htmlParseExternalID(
    ctxt: xmlParserCtxtPtr,
    publicID: *mut *mut xmlChar,
) -> *mut xmlChar {
    return htmlParseExternalID(ctxt, publicID);
}
