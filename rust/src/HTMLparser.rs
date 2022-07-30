const HTML_MAX_NAMELEN: libc::c_int = 1000 as libc::c_int;
const HTML_PARSER_BIG_BUFFER_SIZE: libc::c_int = 1000 as libc::c_int;
const HTML_PARSER_BUFFER_SIZE: libc::c_int = 100 as libc::c_int;
const INPUT_CHUNK: libc::c_int = 250 as libc::c_int;

fn UPPER(mut ctxt: htmlParserCtxtPtr) -> libc::c_int {
    let mut __res: libc::c_int = 0;
    if ::std::mem::size_of::<xmlChar>() as libc::c_ulong >
        1 as libc::c_int as libc::c_ulong {
        if 0 != 0 {
            let mut __c: libc::c_int = CUR(ctxt);
            __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                __c
            } else {
                unsafe { *(*__ctype_toupper_loc_safe()).offset(__c as isize) }
            })
        } else {
            __res = toupper(CUR(ctxt))
        }
    } else {
        unsafe {
            __res = *(*__ctype_toupper_loc_safe()).offset(*(*(*ctxt).input).cur as libc::c_int as isize)
        }
    }
    __res
}

fn CUR(mut ctxt: htmlParserCtxtPtr) -> libc::c_int {
    unsafe { *(*(*ctxt).input).cur as libc::c_int }
}

fn RAW(mut ctxt: htmlParserCtxtPtr) -> libc::c_int {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.token != 0 {
        -(1 as libc::c_int)
    } else {
        CUR(ctxt)
    }
}

fn NXT(mut ctxt: htmlParserCtxtPtr, mut val: libc::c_int)
       -> libc::c_int {
    unsafe { *(*(*ctxt).input).cur.offset(val as isize) as libc::c_int }
}

fn SHRINK_bool1(mut ctxt: htmlParserCtxtPtr, mut num: libc::c_long) -> bool {
    let mut result: libc::c_long = unsafe { (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) } as libc::c_long;
    result > num
}

fn SHRINK_bool2(mut ctxt: htmlParserCtxtPtr, mut num: libc::c_long) -> bool {
    let mut result: libc::c_long = unsafe { (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) } as libc::c_long;
    result < num
}

fn SHRINK(mut ctxt: htmlParserCtxtPtr) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if SHRINK_bool1(ctxt, (2 * INPUT_CHUNK) as libc::c_long) && SHRINK_bool2(ctxt, (2 * INPUT_CHUNK) as libc::c_long) {
        xmlParserInputShrink_safe(ctxtPtr.input);
    }
}

fn SKIP(mut ctxt: htmlParserCtxtPtr, mut val: libc::c_int) {
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    unsafe { inputPtr.cur = inputPtr.cur.offset(val as isize); }
    inputPtr.col += val;
}

fn NEXTL(mut ctxt: htmlParserCtxtPtr, mut ql: libc::c_int) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if CUR(ctxt) == '\n' as i32 {
        inputPtr.line += 1;
        inputPtr.col = 1 as libc::c_int
    } else { inputPtr.col += 1 }
    ctxtPtr.token = 0 as libc::c_int;
    unsafe { inputPtr.cur = inputPtr.cur.offset(ql as isize); }
}

fn GROW(mut ctxt: htmlParserCtxtPtr) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.progressive == 0 as libc::c_int && SHRINK_bool2(ctxt, INPUT_CHUNK as libc::c_long) {
        xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK);
    }
}

fn IS_CHAR(mut q: libc::c_int) -> bool {
    if q < 0x100 as libc::c_int {
        (0x9 as libc::c_int <= q && q <= 0xa as libc::c_int) || q == 0xd as libc::c_int || 0x20 as libc::c_int <= q
    } else {
        (0x100 as libc::c_int <= q && q <= 0xd7ff as libc::c_int) || (0xe000 as libc::c_int <= q && q <= 0xfffd as libc::c_int) || (0x10000 as libc::c_int <= q && q <= 0x10ffff as libc::c_int)
    }
}

fn IS_CHAR_CH(mut c: libc::c_int) -> bool {
    (0x9 as libc::c_int <= c && c <= 0xa as libc::c_int) || c == 0xd as libc::c_int || 0x20 as libc::c_int <= c
}

fn IS_BLANK_CH(c: libc::c_int) -> bool {
    (c == 0x20 as libc::c_int) || ((c >= 0x9 as libc::c_int) && (c <= 0xa as libc::c_int)) || (c == 0xd as libc::c_int)
}

fn IS_BLANK(cur: libc::c_int) -> bool {
    (if cur < 0x100 as libc::c_int {
        (cur == 0x20 as libc::c_int || 0x9 as libc::c_int <= cur && cur <= 0xa as libc::c_int
            || cur == 0xd as libc::c_int) as libc::c_int
    } else { 0 as libc::c_int }) != 0
}

fn IS_LETTER(c: libc::c_int, group: *const xmlChRangeGroup) -> bool {
    ((if c < 0x100 as libc::c_int {
        (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int || 0xf8 as libc::c_int <= c) as libc::c_int
    } else {
        xmlCharInRange_safe(c as libc::c_uint, group)
    }) != 0 || (if c < 0x100 as libc::c_int {
        0 as libc::c_int
    } else {
        (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int || c == 0x3007 as libc::c_int || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int) as libc::c_int
    }) != 0)
}

fn IS_DIGIT(c: libc::c_int, group: *const xmlChRangeGroup) -> bool {
    (if c < 0x100 as libc::c_int {
        (0x30 as libc::c_int <= c && c <= 0x39 as libc::c_int)
            as libc::c_int
    } else {
        xmlCharInRange_safe(c as libc::c_uint, group)
    }) != 0
}

fn IS_COMBINING(c: libc::c_int, group: *const xmlChRangeGroup) -> bool {
    (if c < 0x100 as libc::c_int {
        0 as libc::c_int
    } else {
        xmlCharInRange_safe(c as libc::c_uint, group)
    }) != 0
}

fn IS_EXTENDER(c: libc::c_int, group: *const xmlChRangeGroup) -> bool {
    (if c < 0x100 as libc::c_int {
        (c == 0xb7 as libc::c_int) as libc::c_int
    } else {
        xmlCharInRange_safe(c as libc::c_uint, group)
    }) != 0
}

fn COPY_BUF(mut ql: libc::c_int, mut buf: *mut xmlChar,
            mut len: libc::c_int, mut q: libc::c_int)
            -> libc::c_int {
    if ql == 1 as libc::c_int {
        let fresh40 = len;
        len = len + 1;
        unsafe { *buf.offset(fresh40 as isize) = q as xmlChar; }
    } else {
        unsafe { len += xmlCopyChar_safe(ql, buf.offset(len as isize), q); }
    }
    return len;
}

fn IS_ASCII_LETTER(c: libc::c_int) -> bool {
    ((c >= 0x41 as libc::c_int) && (c <= 0x5a as libc::c_int)) || ((c >= 0x61 as libc::c_int) && (c <= 0x7a as libc::c_int))
}

fn IS_ASCII_DIGIT(c: libc::c_int) -> bool {
    (c >= 0x30 as libc::c_int) && (c <= 0x39 as libc::c_int)
}

fn UPP(ctxt: htmlParserCtxtPtr, val: libc::c_int) -> libc::c_int {
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    let mut __res: libc::c_int = 0;
    if ::std::mem::size_of::<xmlChar>() as libc::c_ulong >
        1 as libc::c_int as libc::c_ulong {
        if 0 != 0 {
            let mut __c: libc::c_int = unsafe { *inputPtr.cur.offset(val as isize) }
                as libc::c_int;
            __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                __c
            } else {
                unsafe { *(*__ctype_toupper_loc_safe()).offset(__c as isize) }
            })
        } else {
            unsafe {
                __res = toupper(*inputPtr.cur.offset(val as isize)
                    as libc::c_int)
            }
        }
    } else {
        unsafe {
            __res = *(*__ctype_toupper_loc_safe()).offset(*inputPtr.cur.offset(val
                as isize) as libc::c_int as isize)
        }
    }
    __res
}

#[inline]
fn bsearch(mut __key: *const libc::c_void,
           mut __base: *const libc::c_void,
           mut __nmemb: size_t, mut __size: size_t,
           mut __compar: __compar_fn_t)
           -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as libc::c_ulong);
        unsafe {
            __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize) as *mut libc::c_void;
        }
        __comparison = unsafe {
            Some(__compar.expect("non-null function pointer")).expect("non-null function pointer")(__key,
                                                                                                   __p)
        };
        if __comparison < 0 as libc::c_int {
            __u = __idx
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else { return __p as *mut libc::c_void; }
    }
    return 0 as *mut libc::c_void;
}

#[inline]
fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        unsafe { *(*__ctype_toupper_loc_safe()).offset(__c as isize) }
    } else { __c };
}

static mut htmlOmittedDefaultValue: libc::c_int = 1 as libc::c_int;
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
fn htmlErrMemory(mut ctxt: xmlParserCtxtPtr,
                 mut extra: *const libc::c_char) {
    unsafe {
        if !ctxt.is_null() && (*ctxt).disableSAX != 0 as libc::c_int && (*ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
            return;
        }
    }
    if !ctxt.is_null() {
        let mut ctxtPtr = unsafe { &mut *ctxt };
        ctxtPtr.errNo = XML_ERR_NO_MEMORY as libc::c_int;
        ctxtPtr.instate = XML_PARSER_EOF;
        ctxtPtr.disableSAX = 1 as libc::c_int
    }
    if !extra.is_null() {
        __xmlRaiseError_safe_macro!(None, None, 0 as *mut libc::c_void,
                            ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
                            XML_FROM_PARSER as libc::c_int,
                            XML_ERR_NO_MEMORY as libc::c_int, XML_ERR_FATAL,
                            0 as *const libc::c_char, 0 as libc::c_int, extra,
                            0 as *const libc::c_char, 0 as *const libc::c_char,
                            0 as libc::c_int, 0 as libc::c_int,
                            b"Memory allocation failed : %s\n\x00" as *const u8 as *const libc::c_char, extra);
    } else {
        __xmlRaiseError_safe_macro!(None, None, 0 as *mut libc::c_void,
                            ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
                            XML_FROM_PARSER as libc::c_int,
                            XML_ERR_NO_MEMORY as libc::c_int, XML_ERR_FATAL,
                            0 as *const libc::c_char, 0 as libc::c_int,
                            0 as *const libc::c_char, 0 as *const libc::c_char,
                            0 as *const libc::c_char, 0 as libc::c_int,
                            0 as libc::c_int,
                            b"Memory allocation failed\n\x00" as *const u8 as *const libc::c_char);
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
fn htmlParseErr(mut ctxt: xmlParserCtxtPtr,
                mut error: xmlParserErrors,
                mut msg: *const libc::c_char,
                mut str1: *const xmlChar,
                mut str2: *const xmlChar) {
    unsafe {
        if !ctxt.is_null() && (*ctxt).disableSAX != 0 as libc::c_int && (*ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
            return;
        }
        if !ctxt.is_null() { (*ctxt).errNo = error as libc::c_int }
        __xmlRaiseError_safe_macro!(None, None, 0 as *mut libc::c_void,
                        ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
                        XML_FROM_HTML as libc::c_int, error as libc::c_int,
                        XML_ERR_ERROR, 0 as *const libc::c_char, 0 as libc::c_int,
                        str1 as *const libc::c_char, str2 as *const libc::c_char,
                        0 as *const libc::c_char, 0 as libc::c_int,
                        0 as libc::c_int, msg, str1, str2);
        if !ctxt.is_null() { (*ctxt).wellFormed = 0 as libc::c_int };
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
fn htmlParseErrInt(mut ctxt: xmlParserCtxtPtr,
                   mut error: xmlParserErrors,
                   mut msg: *const libc::c_char,
                   mut val: libc::c_int) {
    unsafe {
        if !ctxt.is_null() && (*ctxt).disableSAX != 0 as libc::c_int && (*ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
            return;
        }
        if !ctxt.is_null() { (*ctxt).errNo = error as libc::c_int }
        __xmlRaiseError_safe_macro!(None, None, 0 as *mut libc::c_void,
                        ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
                        XML_FROM_HTML as libc::c_int, error as libc::c_int,
                        XML_ERR_ERROR, 0 as *const libc::c_char, 0 as libc::c_int,
                        0 as *const libc::c_char, 0 as *const libc::c_char,
                        0 as *const libc::c_char, val, 0 as libc::c_int, msg,
                        val);
        if !ctxt.is_null() { (*ctxt).wellFormed = 0 as libc::c_int };
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
fn htmlnamePush(mut ctxt: htmlParserCtxtPtr,
                mut value: *const xmlChar) -> libc::c_int {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.html < 3 as libc::c_int && xmlStrEqual_safe(value,
                                                           b"head\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
        ctxtPtr.html = 3 as libc::c_int
    }
    if ctxtPtr.html < 10 as libc::c_int && xmlStrEqual_safe(value,
                                                            b"body\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
        ctxtPtr.html = 10 as libc::c_int
    }
    if ctxtPtr.nameNr >= ctxtPtr.nameMax {
        ctxtPtr.nameMax *= 2 as libc::c_int;
        ctxtPtr.nameTab = xmlRealloc_safe(ctxtPtr.nameTab as *mut *mut xmlChar
                                              as *mut libc::c_void,
                                          (ctxtPtr.nameMax as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
                                              as libc::c_ulong))
            as *mut *const xmlChar;
        if ctxtPtr.nameTab.is_null() {
            htmlErrMemory(ctxt, 0 as *const libc::c_char);
            return 0 as libc::c_int;
        }
    }
    let mut fresh0 = unsafe { &mut *ctxtPtr.nameTab.offset(ctxtPtr.nameNr as isize) };
    *fresh0 = value;
    ctxtPtr.name = value;
    let fresh1 = ctxtPtr.nameNr;
    ctxtPtr.nameNr = ctxtPtr.nameNr + 1;
    return fresh1;
}
/* *
 * htmlnamePop:
 * @ctxt: an HTML parser context
 *
 * Pops the top element name from the name stack
 *
 * Returns the name just removed
 */
fn htmlnamePop(mut ctxt: htmlParserCtxtPtr)
               -> *const xmlChar {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    if ctxtPtr.nameNr <= 0 as libc::c_int { return 0 as *const xmlChar; }
    ctxtPtr.nameNr -= 1;
    if ctxtPtr.nameNr < 0 as libc::c_int { return 0 as *const xmlChar; }
    if ctxtPtr.nameNr > 0 as libc::c_int {
        unsafe {
            ctxtPtr.name = *ctxtPtr.nameTab.offset((ctxtPtr.nameNr - 1 as libc::c_int) as isize)
        }
    } else { ctxtPtr.name = 0 as *const xmlChar }
    unsafe { ret = *ctxtPtr.nameTab.offset(ctxtPtr.nameNr as isize); }
    let ref mut fresh2 = unsafe { *ctxtPtr.nameTab.offset(ctxtPtr.nameNr as isize) };
    *fresh2 = 0 as *const xmlChar;
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
fn htmlNodeInfoPush(mut ctxt: htmlParserCtxtPtr,
                    mut value: *mut htmlParserNodeInfo)
                    -> libc::c_int {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.nodeInfoNr >= ctxtPtr.nodeInfoMax {
        if ctxtPtr.nodeInfoMax == 0 as libc::c_int {
            ctxtPtr.nodeInfoMax = 5 as libc::c_int
        }
        ctxtPtr.nodeInfoMax *= 2 as libc::c_int;
        ctxtPtr.nodeInfoTab = xmlRealloc_safe(ctxtPtr.nodeInfoTab
                                                  as *mut htmlParserNodeInfo
                                                  as *mut libc::c_void,
                                              (ctxtPtr.nodeInfoMax
                                                  as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlParserNodeInfo>()
                                                  as libc::c_ulong))
            as *mut htmlParserNodeInfo;
        if ctxtPtr.nodeInfoTab.is_null() {
            htmlErrMemory(ctxt, 0 as *const libc::c_char);
            return 0 as libc::c_int;
        }
    }
    unsafe { *ctxtPtr.nodeInfoTab.offset(ctxtPtr.nodeInfoNr as isize) = *value; }
    unsafe {
        ctxtPtr.nodeInfo = &mut *ctxtPtr.nodeInfoTab.offset(ctxtPtr.nodeInfoNr as isize) as *mut xmlParserNodeInfo;
    }
    let fresh3 = ctxtPtr.nodeInfoNr;
    ctxtPtr.nodeInfoNr = ctxtPtr.nodeInfoNr + 1;
    return fresh3;
}
/* *
 * htmlNodeInfoPop:
 * @ctxt:  an HTML parser context
 *
 * Pops the top element name from the node info stack
 *
 * Returns 0 in case of error, the pointer to NodeInfo otherwise
 */
fn htmlNodeInfoPop(mut ctxt: htmlParserCtxtPtr)
                   -> *mut htmlParserNodeInfo {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.nodeInfoNr <= 0 as libc::c_int {
        return 0 as *mut htmlParserNodeInfo;
    }
    ctxtPtr.nodeInfoNr -= 1;
    if ctxtPtr.nodeInfoNr < 0 as libc::c_int {
        return 0 as *mut htmlParserNodeInfo;
    }
    if ctxtPtr.nodeInfoNr > 0 as libc::c_int {
        unsafe {
            ctxtPtr.nodeInfo = &mut *ctxtPtr.nodeInfoTab.offset((ctxtPtr.nodeInfoNr -
                1 as libc::c_int) as isize)
                as *mut xmlParserNodeInfo
        }
    } else { ctxtPtr.nodeInfo = 0 as *mut xmlParserNodeInfo }
    unsafe {
        return &mut *ctxtPtr.nodeInfoTab.offset(ctxtPtr.nodeInfoNr as isize) as *mut xmlParserNodeInfo;
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
fn htmlFindEncoding(mut ctxt: xmlParserCtxtPtr)
                    -> *mut xmlChar {
    let mut start: *const xmlChar = 0 as *const xmlChar;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut end: *const xmlChar = 0 as *const xmlChar;
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() || !(*(*ctxt).input).encoding.is_null() || (*(*ctxt).input).buf.is_null() || !(*(*(*ctxt).input).buf).encoder.is_null() {
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
    if end_safe as libc::c_int != 0 as libc::c_int { return 0 as *mut xmlChar; }
    cur = xmlStrcasestr_safe(start,
                             b"HTTP-EQUIV\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
    if cur.is_null() { return 0 as *mut xmlChar; }
    cur = xmlStrcasestr_safe(cur,
                             b"CONTENT\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
    if cur.is_null() { return 0 as *mut xmlChar; }
    cur = xmlStrcasestr_safe(cur,
                             b"CHARSET=\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
    if cur.is_null() { return 0 as *mut xmlChar; }
    unsafe { cur = cur.offset(8 as libc::c_int as isize); }
    start = cur;
    let mut cur_safe = unsafe { *cur };
    while cur_safe as libc::c_int >= 'A' as i32 && cur_safe as libc::c_int <= 'Z' as i32 || cur_safe as libc::c_int >= 'a' as i32 && cur_safe as libc::c_int <= 'z' as i32 || cur_safe as libc::c_int >= '0' as i32 && cur_safe as libc::c_int <= '9' as i32 || cur_safe as libc::c_int == '-' as i32 || cur_safe as libc::c_int == '_' as i32 || cur_safe as libc::c_int == ':' as i32 || cur_safe as libc::c_int == '/' as i32 {
        unsafe { cur = cur.offset(1) }
    }
    if cur == start { return 0 as *mut xmlChar; }
    unsafe {
        return xmlStrndup_safe(start,
                               cur.offset_from(start) as libc::c_long as libc::c_int);
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
fn htmlCurrentChar(mut ctxt: xmlParserCtxtPtr,
                   mut len: *mut libc::c_int)
                   -> libc::c_int {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    let mut current_block: i32;
    let mut cur: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut c: libc::c_uchar = 0;
    let mut val: libc::c_uint = 0;
    if ctxtPtr.instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
        return 0 as libc::c_int;
    }
    if ctxtPtr.token != 0 as libc::c_int {
        unsafe { *len = 0 as libc::c_int; }
        return ctxtPtr.token;
    }
    if ctxtPtr.charset != XML_CHAR_ENCODING_UTF8 as libc::c_int {
        let mut guess: *mut xmlChar = 0 as *mut xmlChar;
        let mut handler: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
        /*
         * Assume it's a fixed length encoding (1) with
         * a compatible encoding for the ASCII set, since
         * HTML constructs only use < 128 chars
         */
        if CUR(ctxt) < 0x80 as libc::c_int {
            unsafe { *len = 1 as libc::c_int; }
            if CUR(ctxt) == 0 as libc::c_int && inputPtr.cur < inputPtr.end {
                htmlParseErrInt(ctxt, XML_ERR_INVALID_CHAR,
                                b"Char 0x%X out of allowed range\n\x00" as *const u8 as *const libc::c_char,
                                0 as libc::c_int);
                return ' ' as i32;
            }
            return CUR(ctxt);
        }
        /*
         * Humm this is bad, do an automatic flow conversion
         */
        guess = htmlFindEncoding(ctxt);
        if guess.is_null() {
            xmlSwitchEncoding_safe(ctxt, XML_CHAR_ENCODING_8859_1);
        } else {
            if !inputPtr.encoding.is_null() {
                xmlFree_safe(inputPtr.encoding
                    as *mut xmlChar
                    as *mut libc::c_void);
            }
            inputPtr.encoding = guess;
            handler = xmlFindCharEncodingHandler_safe(guess as *const libc::c_char);
            if !handler.is_null() {
                /*
                 * Don't use UTF-8 encoder which isn't required and
                 * can produce invalid UTF-8.
                 */
                let mut handlerPtr = unsafe { &mut *handler };
                if xmlStrEqual_safe(handlerPtr.name as *mut xmlChar,
                                    b"UTF-8\x00" as *const u8 as *const libc::c_char as *mut xmlChar) == 0 {
                    xmlSwitchToEncoding_safe(ctxt, handler);
                }
            } else {
                htmlParseErr(ctxt, XML_ERR_INVALID_ENCODING,
                             b"Unsupported encoding %s\x00" as *const u8 as *const libc::c_char, guess,
                             0 as *const xmlChar);
            }
        }
        ctxtPtr.charset = XML_CHAR_ENCODING_UTF8 as libc::c_int
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
    unsafe { c = *cur; }
    if c as libc::c_int & 0x80 as libc::c_int != 0 {
        if !(c as libc::c_int & 0x40 as libc::c_int == 0 as libc::c_int) {
            unsafe {
                if *cur.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
                    xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK);
                    cur = inputPtr.cur
                }
            }
            unsafe {
                if !(*cur.offset(1 as libc::c_int as isize) as libc::c_int &
                    0xc0 as libc::c_int != 0x80 as libc::c_int) {
                    if c as libc::c_int & 0xe0 as libc::c_int == 0xe0 as libc::c_int {
                        if *cur.offset(2 as libc::c_int as isize) as libc::c_int
                            == 0 as libc::c_int {
                            xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK);
                            cur = inputPtr.cur
                        }
                        if *cur.offset(2 as libc::c_int as isize) as libc::c_int &
                            0xc0 as libc::c_int != 0x80 as libc::c_int {
                            current_block = 1;
                        } else if c as libc::c_int & 0xf0 as libc::c_int == 0xf0 as libc::c_int {
                            if *cur.offset(3 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
                                xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK);
                                cur = inputPtr.cur
                            }
                            if c as libc::c_int & 0xf8 as libc::c_int != 0xf0 as libc::c_int || *cur.offset(3 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int {
                                current_block = 1;
                            } else {
                                /* 4-byte code */
                                *len = 4 as libc::c_int;
                                val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int & 0x7 as libc::c_int) <<
                                    18 as libc::c_int) as libc::c_uint;
                                val |= ((*cur.offset(1 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int) <<
                                    12 as libc::c_int) as libc::c_uint;
                                val |= ((*cur.offset(2 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int) <<
                                    6 as libc::c_int) as libc::c_uint;
                                val |= (*cur.offset(3 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int) as libc::c_uint;
                                if val < 0x10000 as libc::c_int as libc::c_uint {
                                    current_block = 1;
                                } else { current_block = 2; }
                            }
                        } else {
                            /* 3-byte code */
                            *len = 3 as libc::c_int;
                            val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int & 0xf as libc::c_int) <<
                                12 as libc::c_int) as libc::c_uint;
                            val |= ((*cur.offset(1 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int) <<
                                6 as libc::c_int) as libc::c_uint;
                            val |= (*cur.offset(2 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int) as libc::c_uint;
                            if val < 0x800 as libc::c_int as libc::c_uint {
                                current_block = 1;
                            } else { current_block = 2; }
                        }
                    } else {
                        /* 2-byte code */
                        *len = 2 as libc::c_int;
                        val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int & 0x1f as libc::c_int) <<
                            6 as libc::c_int) as libc::c_uint;
                        val |= (*cur.offset(1 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int) as libc::c_uint;
                        if val < 0x80 as libc::c_int as libc::c_uint {
                            current_block = 1;
                        } else { current_block = 2; }
                    }
                    match current_block {
                        1 => {}
                        _ => {
                            if if val < 0x100 as libc::c_int as libc::c_uint {
                                (0x9 as libc::c_int as libc::c_uint <= val && val <= 0xa as libc::c_int as libc::c_uint
                                    || val == 0xd as libc::c_int as libc::c_uint
                                    || 0x20 as libc::c_int as libc::c_uint <= val) as libc::c_int
                            } else {
                                (0x100 as libc::c_int as libc::c_uint <= val && val <= 0xd7ff as libc::c_int as libc::c_uint
                                    || 0xe000 as libc::c_int as libc::c_uint <= val && val <= 0xfffd as libc::c_int as libc::c_uint || 0x10000 as libc::c_int as libc::c_uint <= val && val <= 0x10ffff as libc::c_int as libc::c_uint) as libc::c_int
                            } == 0 {
                                htmlParseErrInt(ctxt, XML_ERR_INVALID_CHAR,
                                                b"Char 0x%X out of allowed range\n\x00"
                                                    as *const u8 as *const libc::c_char,
                                                val as libc::c_int);
                            }
                            return val as libc::c_int;
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
        let mut buffer: [libc::c_char; 150] = [0; 150];
        unsafe {
            if inputPtr.end.offset_from(inputPtr.cur) as libc::c_long >= 4 as libc::c_int as libc::c_long {
                snprintf_safe_macro!(buffer.as_mut_ptr(), 149 as libc::c_int as libc::c_ulong,
                         b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8
                             as *const libc::c_char,
                         *inputPtr.cur.offset(0 as libc::c_int as isize)
                             as libc::c_int,
                         *inputPtr.cur.offset(1 as libc::c_int as isize)
                             as libc::c_int,
                         *inputPtr.cur.offset(2 as libc::c_int as isize)
                             as libc::c_int,
                         *inputPtr.cur.offset(3 as libc::c_int as isize)
                             as libc::c_int);
            } else {
                snprintf_safe_macro!(buffer.as_mut_ptr(), 149 as libc::c_int as libc::c_ulong,
                         b"Bytes: 0x%02X\n\x00" as *const u8 as *const libc::c_char,
                         *inputPtr.cur.offset(0 as libc::c_int as isize)
                             as libc::c_int);
            }
        }
        htmlParseErr(ctxt, XML_ERR_INVALID_ENCODING,
                     b"Input is not proper UTF-8, indicate encoding !\n\x00"
                         as *const u8 as *const libc::c_char,
                     buffer.as_mut_ptr() as *mut xmlChar,
                     0 as *const xmlChar);
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
        unsafe { *len = 1 as libc::c_int; }
        return CUR(ctxt);
    } else {
        if CUR(ctxt) == 0 as libc::c_int && inputPtr.cur < inputPtr.end {
            htmlParseErrInt(ctxt, XML_ERR_INVALID_CHAR,
                            b"Char 0x%X out of allowed range\n\x00" as *const u8 as *const libc::c_char,
                            0 as libc::c_int);
            unsafe { *len = 1 as libc::c_int; }
            return ' ' as i32;
        }
        /* 1-byte code */
        unsafe { *len = 1 as libc::c_int; }
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
fn htmlSkipBlankChars(mut ctxt: xmlParserCtxtPtr)
                      -> libc::c_int {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    let mut res: libc::c_int = 0 as libc::c_int;
    while IS_BLANK_CH(CUR(ctxt)) {
        if CUR(ctxt) == 0 as libc::c_int && xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK) <= 0 as libc::c_int {
            xmlPopInput_safe(ctxt);
        } else {
            if CUR(ctxt) == '\n' as i32 {
                inputPtr.line += 1;
                inputPtr.col = 1 as libc::c_int
            } else { inputPtr.col += 1 }
            unsafe { inputPtr.cur = inputPtr.cur.offset(1); }
            if CUR(ctxt) == 0 as libc::c_int {
                xmlParserInputGrow_safe(ctxtPtr.input, INPUT_CHUNK);
            }
        }
        res += 1
    }
    return res;
}

static mut html_flow: [*const libc::c_char; 64] = [b"h1\x00" as *const u8 as *const libc::c_char,
    b"h2\x00" as *const u8 as *const libc::c_char,
    b"h3\x00" as *const u8 as *const libc::c_char,
    b"h4\x00" as *const u8 as *const libc::c_char,
    b"h5\x00" as *const u8 as *const libc::c_char,
    b"h6\x00" as *const u8 as *const libc::c_char,
    b"ul\x00" as *const u8 as *const libc::c_char,
    b"ol\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"menu\x00" as *const u8 as *const libc::c_char,
    b"pre\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"dl\x00" as *const u8 as *const libc::c_char,
    b"div\x00" as *const u8 as *const libc::c_char,
    b"center\x00" as *const u8 as *const libc::c_char,
    b"noscript\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    b"blockquote\x00" as *const u8 as *const libc::c_char,
    b"form\x00" as *const u8 as *const libc::c_char,
    b"isindex\x00" as *const u8 as *const libc::c_char,
    b"hr\x00" as *const u8 as *const libc::c_char,
    b"table\x00" as *const u8 as *const libc::c_char,
    b"fieldset\x00" as *const u8 as *const libc::c_char,
    b"address\x00" as *const u8 as *const libc::c_char,
    b"tt\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"big\x00" as *const u8 as *const libc::c_char,
    b"small\x00" as *const u8 as *const libc::c_char,
    b"em\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"img\x00" as *const u8 as *const libc::c_char,
    b"applet\x00" as *const u8 as *const libc::c_char,
    b"embed\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    b"font\x00" as *const u8 as *const libc::c_char,
    b"basefont\x00" as *const u8 as *const libc::c_char,
    b"br\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"map\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"sub\x00" as *const u8 as *const libc::c_char,
    b"sup\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    b"input\x00" as *const u8 as *const libc::c_char,
    b"select\x00" as *const u8 as *const libc::c_char,
    b"textarea\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"button\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut html_inline: [*const libc::c_char; 40] = [b"tt\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"big\x00" as *const u8 as *const libc::c_char,
    b"small\x00" as *const u8 as *const libc::c_char,
    b"em\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"img\x00" as *const u8 as *const libc::c_char,
    b"applet\x00" as *const u8 as *const libc::c_char,
    b"embed\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    b"font\x00" as *const u8 as *const libc::c_char,
    b"basefont\x00" as *const u8 as *const libc::c_char,
    b"br\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"map\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"sub\x00" as *const u8 as *const libc::c_char,
    b"sup\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    b"input\x00" as *const u8 as *const libc::c_char,
    b"select\x00" as *const u8 as *const libc::c_char,
    b"textarea\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"button\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
/* placeholders: elts with content but no subelements */
static mut html_pcdata: [*const libc::c_char; 1] = [0 as *const libc::c_char];
static mut html_attrs: [*const libc::c_char; 16] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut core_i18n_attrs: [*const libc::c_char; 7] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut core_attrs: [*const libc::c_char; 5] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut i18n_attrs: [*const libc::c_char; 3] = [b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
/* Other declarations that should go inline ... */
static mut a_attrs: [*const libc::c_char; 29] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"charset\x00" as *const u8 as *const libc::c_char,
    b"type\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"href\x00" as *const u8 as *const libc::c_char,
    b"hreflang\x00" as *const u8 as *const libc::c_char,
    b"rel\x00" as *const u8 as *const libc::c_char,
    b"rev\x00" as *const u8 as *const libc::c_char,
    b"accesskey\x00" as *const u8 as *const libc::c_char,
    b"shape\x00" as *const u8 as *const libc::c_char,
    b"coords\x00" as *const u8 as *const libc::c_char,
    b"tabindex\x00" as *const u8 as *const libc::c_char,
    b"onfocus\x00" as *const u8 as *const libc::c_char,
    b"onblur\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut target_attr: [*const libc::c_char; 2] = [b"target\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut rows_cols_attr: [*const libc::c_char; 3] = [b"rows\x00" as *const u8 as *const libc::c_char,
    b"cols\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut alt_attr: [*const libc::c_char; 2] = [b"alt\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut src_alt_attrs: [*const libc::c_char; 3] = [b"src\x00" as *const u8 as *const libc::c_char,
    b"alt\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut href_attrs: [*const libc::c_char; 2] = [b"href\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut clear_attrs: [*const libc::c_char; 2] = [b"clear\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut inline_p: [*const libc::c_char; 41] = [b"tt\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"big\x00" as *const u8 as *const libc::c_char,
    b"small\x00" as *const u8 as *const libc::c_char,
    b"em\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"img\x00" as *const u8 as *const libc::c_char,
    b"applet\x00" as *const u8 as *const libc::c_char,
    b"embed\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    b"font\x00" as *const u8 as *const libc::c_char,
    b"basefont\x00" as *const u8 as *const libc::c_char,
    b"br\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"map\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"sub\x00" as *const u8 as *const libc::c_char,
    b"sup\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    b"input\x00" as *const u8 as *const libc::c_char,
    b"select\x00" as *const u8 as *const libc::c_char,
    b"textarea\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"button\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
static mut flow_param: [*const libc::c_char; 65] = [b"h1\x00" as *const u8 as *const libc::c_char,
    b"h2\x00" as *const u8 as *const libc::c_char,
    b"h3\x00" as *const u8 as *const libc::c_char,
    b"h4\x00" as *const u8 as *const libc::c_char,
    b"h5\x00" as *const u8 as *const libc::c_char,
    b"h6\x00" as *const u8 as *const libc::c_char,
    b"ul\x00" as *const u8 as *const libc::c_char,
    b"ol\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"menu\x00" as *const u8 as *const libc::c_char,
    b"pre\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"dl\x00" as *const u8 as *const libc::c_char,
    b"div\x00" as *const u8 as *const libc::c_char,
    b"center\x00" as *const u8 as *const libc::c_char,
    b"noscript\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    b"blockquote\x00" as *const u8 as *const libc::c_char,
    b"form\x00" as *const u8 as *const libc::c_char,
    b"isindex\x00" as *const u8 as *const libc::c_char,
    b"hr\x00" as *const u8 as *const libc::c_char,
    b"table\x00" as *const u8 as *const libc::c_char,
    b"fieldset\x00" as *const u8 as *const libc::c_char,
    b"address\x00" as *const u8 as *const libc::c_char,
    b"tt\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"big\x00" as *const u8 as *const libc::c_char,
    b"small\x00" as *const u8 as *const libc::c_char,
    b"em\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"img\x00" as *const u8 as *const libc::c_char,
    b"applet\x00" as *const u8 as *const libc::c_char,
    b"embed\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    b"font\x00" as *const u8 as *const libc::c_char,
    b"basefont\x00" as *const u8 as *const libc::c_char,
    b"br\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"map\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"sub\x00" as *const u8 as *const libc::c_char,
    b"sup\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    b"input\x00" as *const u8 as *const libc::c_char,
    b"select\x00" as *const u8 as *const libc::c_char,
    b"textarea\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"button\x00" as *const u8 as *const libc::c_char,
    b"param\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut applet_attrs: [*const libc::c_char; 14] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"codebase\x00" as *const u8 as *const libc::c_char,
    b"archive\x00" as *const u8 as *const libc::c_char,
    b"alt\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"height\x00" as *const u8 as *const libc::c_char,
    b"width\x00" as *const u8 as *const libc::c_char,
    b"align\x00" as *const u8 as *const libc::c_char,
    b"hspace\x00" as *const u8 as *const libc::c_char,
    b"vspace\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut area_attrs: [*const libc::c_char; 9] = [b"shape\x00" as *const u8 as *const libc::c_char,
    b"coords\x00" as *const u8 as *const libc::c_char,
    b"href\x00" as *const u8 as *const libc::c_char,
    b"nohref\x00" as *const u8 as *const libc::c_char,
    b"tabindex\x00" as *const u8 as *const libc::c_char,
    b"accesskey\x00" as *const u8 as *const libc::c_char,
    b"onfocus\x00" as *const u8 as *const libc::c_char,
    b"onblur\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut basefont_attrs: [*const libc::c_char; 5] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"size\x00" as *const u8 as *const libc::c_char,
    b"color\x00" as *const u8 as *const libc::c_char,
    b"face\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut quote_attrs: [*const libc::c_char; 17] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut body_contents: [*const libc::c_char; 66] = [b"h1\x00" as *const u8 as *const libc::c_char,
    b"h2\x00" as *const u8 as *const libc::c_char,
    b"h3\x00" as *const u8 as *const libc::c_char,
    b"h4\x00" as *const u8 as *const libc::c_char,
    b"h5\x00" as *const u8 as *const libc::c_char,
    b"h6\x00" as *const u8 as *const libc::c_char,
    b"ul\x00" as *const u8 as *const libc::c_char,
    b"ol\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"menu\x00" as *const u8 as *const libc::c_char,
    b"pre\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"dl\x00" as *const u8 as *const libc::c_char,
    b"div\x00" as *const u8 as *const libc::c_char,
    b"center\x00" as *const u8 as *const libc::c_char,
    b"noscript\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    b"blockquote\x00" as *const u8 as *const libc::c_char,
    b"form\x00" as *const u8 as *const libc::c_char,
    b"isindex\x00" as *const u8 as *const libc::c_char,
    b"hr\x00" as *const u8 as *const libc::c_char,
    b"table\x00" as *const u8 as *const libc::c_char,
    b"fieldset\x00" as *const u8 as *const libc::c_char,
    b"address\x00" as *const u8 as *const libc::c_char,
    b"tt\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"big\x00" as *const u8 as *const libc::c_char,
    b"small\x00" as *const u8 as *const libc::c_char,
    b"em\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"img\x00" as *const u8 as *const libc::c_char,
    b"applet\x00" as *const u8 as *const libc::c_char,
    b"embed\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    b"font\x00" as *const u8 as *const libc::c_char,
    b"basefont\x00" as *const u8 as *const libc::c_char,
    b"br\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"map\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"sub\x00" as *const u8 as *const libc::c_char,
    b"sup\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    b"input\x00" as *const u8 as *const libc::c_char,
    b"select\x00" as *const u8 as *const libc::c_char,
    b"textarea\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"button\x00" as *const u8 as *const libc::c_char,
    b"ins\x00" as *const u8 as *const libc::c_char,
    b"del\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut body_attrs: [*const libc::c_char; 18] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"onload\x00" as *const u8 as *const libc::c_char,
    b"onunload\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut body_depr: [*const libc::c_char; 7] = [b"background\x00" as *const u8 as *const libc::c_char,
    b"bgcolor\x00" as *const u8 as *const libc::c_char,
    b"text\x00" as *const u8 as *const libc::c_char,
    b"link\x00" as *const u8 as *const libc::c_char,
    b"vlink\x00" as *const u8 as *const libc::c_char,
    b"alink\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut button_attrs: [*const libc::c_char; 24] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"value\x00" as *const u8 as *const libc::c_char,
    b"type\x00" as *const u8 as *const libc::c_char,
    b"disabled\x00" as *const u8 as *const libc::c_char,
    b"tabindex\x00" as *const u8 as *const libc::c_char,
    b"accesskey\x00" as *const u8 as *const libc::c_char,
    b"onfocus\x00" as *const u8 as *const libc::c_char,
    b"onblur\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut col_attrs: [*const libc::c_char; 22] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"width\x00" as *const u8 as *const libc::c_char,
    b"align\x00" as *const u8 as *const libc::c_char,
    b"char\x00" as *const u8 as *const libc::c_char,
    b"charoff\x00" as *const u8 as *const libc::c_char,
    b"valign\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut col_elt: [*const libc::c_char; 2] = [b"col\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut edit_attrs: [*const libc::c_char; 18] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"datetime\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut compact_attrs: [*const libc::c_char; 17] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"compact\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut dl_contents: [*const libc::c_char; 3] = [b"dt\x00" as *const u8 as *const libc::c_char,
    b"dd\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
static mut compact_attr: [*const libc::c_char; 2] = [b"compact\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut label_attr: [*const libc::c_char; 2] = [b"label\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut fieldset_contents: [*const libc::c_char; 64] = [b"h1\x00" as *const u8 as *const libc::c_char,
    b"h2\x00" as *const u8 as *const libc::c_char,
    b"h3\x00" as *const u8 as *const libc::c_char,
    b"h4\x00" as *const u8 as *const libc::c_char,
    b"h5\x00" as *const u8 as *const libc::c_char,
    b"h6\x00" as *const u8 as *const libc::c_char,
    b"ul\x00" as *const u8 as *const libc::c_char,
    b"ol\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"menu\x00" as *const u8 as *const libc::c_char,
    b"pre\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"dl\x00" as *const u8 as *const libc::c_char,
    b"div\x00" as *const u8 as *const libc::c_char,
    b"center\x00" as *const u8 as *const libc::c_char,
    b"noscript\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    b"blockquote\x00" as *const u8 as *const libc::c_char,
    b"form\x00" as *const u8 as *const libc::c_char,
    b"isindex\x00" as *const u8 as *const libc::c_char,
    b"hr\x00" as *const u8 as *const libc::c_char,
    b"table\x00" as *const u8 as *const libc::c_char,
    b"fieldset\x00" as *const u8 as *const libc::c_char,
    b"address\x00" as *const u8 as *const libc::c_char,
    b"tt\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"big\x00" as *const u8 as *const libc::c_char,
    b"small\x00" as *const u8 as *const libc::c_char,
    b"em\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"img\x00" as *const u8 as *const libc::c_char,
    b"applet\x00" as *const u8 as *const libc::c_char,
    b"embed\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    b"font\x00" as *const u8 as *const libc::c_char,
    b"basefont\x00" as *const u8 as *const libc::c_char,
    b"br\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"map\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"sub\x00" as *const u8 as *const libc::c_char,
    b"sup\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    b"input\x00" as *const u8 as *const libc::c_char,
    b"select\x00" as *const u8 as *const libc::c_char,
    b"textarea\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"button\x00" as *const u8 as *const libc::c_char,
    b"legend\x00" as *const u8 as *const libc::c_char];
static mut font_attrs: [*const libc::c_char; 10] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"size\x00" as *const u8 as *const libc::c_char,
    b"color\x00" as *const u8 as *const libc::c_char,
    b"face\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut form_contents: [*const libc::c_char; 62] = [b"h1\x00" as *const u8 as *const libc::c_char,
    b"h2\x00" as *const u8 as *const libc::c_char,
    b"h3\x00" as *const u8 as *const libc::c_char,
    b"h4\x00" as *const u8 as *const libc::c_char,
    b"h5\x00" as *const u8 as *const libc::c_char,
    b"h6\x00" as *const u8 as *const libc::c_char,
    b"ul\x00" as *const u8 as *const libc::c_char,
    b"ol\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"menu\x00" as *const u8 as *const libc::c_char,
    b"tt\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"big\x00" as *const u8 as *const libc::c_char,
    b"small\x00" as *const u8 as *const libc::c_char,
    b"em\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"img\x00" as *const u8 as *const libc::c_char,
    b"applet\x00" as *const u8 as *const libc::c_char,
    b"embed\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    b"font\x00" as *const u8 as *const libc::c_char,
    b"basefont\x00" as *const u8 as *const libc::c_char,
    b"br\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"map\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"sub\x00" as *const u8 as *const libc::c_char,
    b"sup\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    b"input\x00" as *const u8 as *const libc::c_char,
    b"select\x00" as *const u8 as *const libc::c_char,
    b"textarea\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"button\x00" as *const u8 as *const libc::c_char,
    b"pre\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"div\x00" as *const u8 as *const libc::c_char,
    b"center\x00" as *const u8 as *const libc::c_char,
    b"noscript\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    b"blockquote\x00" as *const u8 as *const libc::c_char,
    b"isindex\x00" as *const u8 as *const libc::c_char,
    b"hr\x00" as *const u8 as *const libc::c_char,
    b"table\x00" as *const u8 as *const libc::c_char,
    b"fieldset\x00" as *const u8 as *const libc::c_char,
    b"address\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut form_attrs: [*const libc::c_char; 23] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"method\x00" as *const u8 as *const libc::c_char,
    b"enctype\x00" as *const u8 as *const libc::c_char,
    b"accept\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"onsubmit\x00" as *const u8 as *const libc::c_char,
    b"onreset\x00" as *const u8 as *const libc::c_char,
    b"accept-charset\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut frame_attrs: [*const libc::c_char; 13] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"longdesc\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"src\x00" as *const u8 as *const libc::c_char,
    b"frameborder\x00" as *const u8 as *const libc::c_char,
    b"marginwidth\x00" as *const u8 as *const libc::c_char,
    b"marginheight\x00" as *const u8 as *const libc::c_char,
    b"noresize\x00" as *const u8 as *const libc::c_char,
    b"scrolling\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut frameset_attrs: [*const libc::c_char; 9] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"rows\x00" as *const u8 as *const libc::c_char,
    b"cols\x00" as *const u8 as *const libc::c_char,
    b"onload\x00" as *const u8 as *const libc::c_char,
    b"onunload\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut frameset_contents: [*const libc::c_char; 4] = [b"frameset\x00" as *const u8 as *const libc::c_char,
    b"frame\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut head_attrs: [*const libc::c_char; 4] = [b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"profile\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut head_contents: [*const libc::c_char; 9] = [b"title\x00" as *const u8 as *const libc::c_char,
    b"isindex\x00" as *const u8 as *const libc::c_char,
    b"base\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"meta\x00" as *const u8 as *const libc::c_char,
    b"link\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut hr_depr: [*const libc::c_char; 5] = [b"align\x00" as *const u8 as *const libc::c_char,
    b"noshade\x00" as *const u8 as *const libc::c_char,
    b"size\x00" as *const u8 as *const libc::c_char,
    b"width\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut version_attr: [*const libc::c_char; 2] = [b"version\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut html_content: [*const libc::c_char; 4] = [b"head\x00" as *const u8 as *const libc::c_char,
    b"body\x00" as *const u8 as *const libc::c_char,
    b"frameset\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut iframe_attrs: [*const libc::c_char; 15] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"longdesc\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"src\x00" as *const u8 as *const libc::c_char,
    b"frameborder\x00" as *const u8 as *const libc::c_char,
    b"marginwidth\x00" as *const u8 as *const libc::c_char,
    b"marginheight\x00" as *const u8 as *const libc::c_char,
    b"scrolling\x00" as *const u8 as *const libc::c_char,
    b"align\x00" as *const u8 as *const libc::c_char,
    b"height\x00" as *const u8 as *const libc::c_char,
    b"width\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut img_attrs: [*const libc::c_char; 22] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"longdesc\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"height\x00" as *const u8 as *const libc::c_char,
    b"width\x00" as *const u8 as *const libc::c_char,
    b"usemap\x00" as *const u8 as *const libc::c_char,
    b"ismap\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut embed_attrs: [*const libc::c_char; 23] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"align\x00" as *const u8 as *const libc::c_char,
    b"alt\x00" as *const u8 as *const libc::c_char,
    b"border\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"codebase\x00" as *const u8 as *const libc::c_char,
    b"frameborder\x00" as *const u8 as *const libc::c_char,
    b"height\x00" as *const u8 as *const libc::c_char,
    b"hidden\x00" as *const u8 as *const libc::c_char,
    b"hspace\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"palette\x00" as *const u8 as *const libc::c_char,
    b"pluginspace\x00" as *const u8 as *const libc::c_char,
    b"pluginurl\x00" as *const u8 as *const libc::c_char,
    b"src\x00" as *const u8 as *const libc::c_char,
    b"type\x00" as *const u8 as *const libc::c_char,
    b"units\x00" as *const u8 as *const libc::c_char,
    b"vspace\x00" as *const u8 as *const libc::c_char,
    b"width\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut input_attrs: [*const libc::c_char; 35] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"type\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"value\x00" as *const u8 as *const libc::c_char,
    b"checked\x00" as *const u8 as *const libc::c_char,
    b"disabled\x00" as *const u8 as *const libc::c_char,
    b"readonly\x00" as *const u8 as *const libc::c_char,
    b"size\x00" as *const u8 as *const libc::c_char,
    b"maxlength\x00" as *const u8 as *const libc::c_char,
    b"src\x00" as *const u8 as *const libc::c_char,
    b"alt\x00" as *const u8 as *const libc::c_char,
    b"usemap\x00" as *const u8 as *const libc::c_char,
    b"ismap\x00" as *const u8 as *const libc::c_char,
    b"tabindex\x00" as *const u8 as *const libc::c_char,
    b"accesskey\x00" as *const u8 as *const libc::c_char,
    b"onfocus\x00" as *const u8 as *const libc::c_char,
    b"onblur\x00" as *const u8 as *const libc::c_char,
    b"onselect\x00" as *const u8 as *const libc::c_char,
    b"onchange\x00" as *const u8 as *const libc::c_char,
    b"accept\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut prompt_attrs: [*const libc::c_char; 8] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"prompt\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut label_attrs: [*const libc::c_char; 20] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"for\x00" as *const u8 as *const libc::c_char,
    b"accesskey\x00" as *const u8 as *const libc::c_char,
    b"onfocus\x00" as *const u8 as *const libc::c_char,
    b"onblur\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut legend_attrs: [*const libc::c_char; 17] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"accesskey\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut align_attr: [*const libc::c_char; 2] = [b"align\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut link_attrs: [*const libc::c_char; 23] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"charset\x00" as *const u8 as *const libc::c_char,
    b"href\x00" as *const u8 as *const libc::c_char,
    b"hreflang\x00" as *const u8 as *const libc::c_char,
    b"type\x00" as *const u8 as *const libc::c_char,
    b"rel\x00" as *const u8 as *const libc::c_char,
    b"rev\x00" as *const u8 as *const libc::c_char,
    b"media\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut map_contents: [*const libc::c_char; 26] = [b"h1\x00" as *const u8 as *const libc::c_char,
    b"h2\x00" as *const u8 as *const libc::c_char,
    b"h3\x00" as *const u8 as *const libc::c_char,
    b"h4\x00" as *const u8 as *const libc::c_char,
    b"h5\x00" as *const u8 as *const libc::c_char,
    b"h6\x00" as *const u8 as *const libc::c_char,
    b"ul\x00" as *const u8 as *const libc::c_char,
    b"ol\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"menu\x00" as *const u8 as *const libc::c_char,
    b"pre\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"dl\x00" as *const u8 as *const libc::c_char,
    b"div\x00" as *const u8 as *const libc::c_char,
    b"center\x00" as *const u8 as *const libc::c_char,
    b"noscript\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    b"blockquote\x00" as *const u8 as *const libc::c_char,
    b"form\x00" as *const u8 as *const libc::c_char,
    b"isindex\x00" as *const u8 as *const libc::c_char,
    b"hr\x00" as *const u8 as *const libc::c_char,
    b"table\x00" as *const u8 as *const libc::c_char,
    b"fieldset\x00" as *const u8 as *const libc::c_char,
    b"address\x00" as *const u8 as *const libc::c_char,
    b"area\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut name_attr: [*const libc::c_char; 2] = [b"name\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut action_attr: [*const libc::c_char; 2] = [b"action\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut blockli_elt: [*const libc::c_char; 26] = [b"h1\x00" as *const u8 as *const libc::c_char,
    b"h2\x00" as *const u8 as *const libc::c_char,
    b"h3\x00" as *const u8 as *const libc::c_char,
    b"h4\x00" as *const u8 as *const libc::c_char,
    b"h5\x00" as *const u8 as *const libc::c_char,
    b"h6\x00" as *const u8 as *const libc::c_char,
    b"ul\x00" as *const u8 as *const libc::c_char,
    b"ol\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"menu\x00" as *const u8 as *const libc::c_char,
    b"pre\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"dl\x00" as *const u8 as *const libc::c_char,
    b"div\x00" as *const u8 as *const libc::c_char,
    b"center\x00" as *const u8 as *const libc::c_char,
    b"noscript\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    b"blockquote\x00" as *const u8 as *const libc::c_char,
    b"form\x00" as *const u8 as *const libc::c_char,
    b"isindex\x00" as *const u8 as *const libc::c_char,
    b"hr\x00" as *const u8 as *const libc::c_char,
    b"table\x00" as *const u8 as *const libc::c_char,
    b"fieldset\x00" as *const u8 as *const libc::c_char,
    b"address\x00" as *const u8 as *const libc::c_char,
    b"li\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
static mut meta_attrs: [*const libc::c_char; 7] = [b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"http-equiv\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"scheme\x00" as *const u8 as *const libc::c_char,
    b"charset\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut content_attr: [*const libc::c_char; 2] = [b"content\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut type_attr: [*const libc::c_char; 2] = [b"type\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut noframes_content: [*const libc::c_char; 65] = [b"body\x00" as *const u8 as *const libc::c_char,
    b"h1\x00" as *const u8 as *const libc::c_char,
    b"h2\x00" as *const u8 as *const libc::c_char,
    b"h3\x00" as *const u8 as *const libc::c_char,
    b"h4\x00" as *const u8 as *const libc::c_char,
    b"h5\x00" as *const u8 as *const libc::c_char,
    b"h6\x00" as *const u8 as *const libc::c_char,
    b"ul\x00" as *const u8 as *const libc::c_char,
    b"ol\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"menu\x00" as *const u8 as *const libc::c_char,
    b"pre\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"dl\x00" as *const u8 as *const libc::c_char,
    b"div\x00" as *const u8 as *const libc::c_char,
    b"center\x00" as *const u8 as *const libc::c_char,
    b"noscript\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    b"blockquote\x00" as *const u8 as *const libc::c_char,
    b"form\x00" as *const u8 as *const libc::c_char,
    b"isindex\x00" as *const u8 as *const libc::c_char,
    b"hr\x00" as *const u8 as *const libc::c_char,
    b"table\x00" as *const u8 as *const libc::c_char,
    b"fieldset\x00" as *const u8 as *const libc::c_char,
    b"address\x00" as *const u8 as *const libc::c_char,
    b"tt\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"big\x00" as *const u8 as *const libc::c_char,
    b"small\x00" as *const u8 as *const libc::c_char,
    b"em\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"img\x00" as *const u8 as *const libc::c_char,
    b"applet\x00" as *const u8 as *const libc::c_char,
    b"embed\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    b"font\x00" as *const u8 as *const libc::c_char,
    b"basefont\x00" as *const u8 as *const libc::c_char,
    b"br\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"map\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"sub\x00" as *const u8 as *const libc::c_char,
    b"sup\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    b"input\x00" as *const u8 as *const libc::c_char,
    b"select\x00" as *const u8 as *const libc::c_char,
    b"textarea\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"button\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut object_contents: [*const libc::c_char; 65] = [b"h1\x00" as *const u8 as *const libc::c_char,
    b"h2\x00" as *const u8 as *const libc::c_char,
    b"h3\x00" as *const u8 as *const libc::c_char,
    b"h4\x00" as *const u8 as *const libc::c_char,
    b"h5\x00" as *const u8 as *const libc::c_char,
    b"h6\x00" as *const u8 as *const libc::c_char,
    b"ul\x00" as *const u8 as *const libc::c_char,
    b"ol\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"menu\x00" as *const u8 as *const libc::c_char,
    b"pre\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"dl\x00" as *const u8 as *const libc::c_char,
    b"div\x00" as *const u8 as *const libc::c_char,
    b"center\x00" as *const u8 as *const libc::c_char,
    b"noscript\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    b"blockquote\x00" as *const u8 as *const libc::c_char,
    b"form\x00" as *const u8 as *const libc::c_char,
    b"isindex\x00" as *const u8 as *const libc::c_char,
    b"hr\x00" as *const u8 as *const libc::c_char,
    b"table\x00" as *const u8 as *const libc::c_char,
    b"fieldset\x00" as *const u8 as *const libc::c_char,
    b"address\x00" as *const u8 as *const libc::c_char,
    b"tt\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"big\x00" as *const u8 as *const libc::c_char,
    b"small\x00" as *const u8 as *const libc::c_char,
    b"em\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"img\x00" as *const u8 as *const libc::c_char,
    b"applet\x00" as *const u8 as *const libc::c_char,
    b"embed\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    b"font\x00" as *const u8 as *const libc::c_char,
    b"basefont\x00" as *const u8 as *const libc::c_char,
    b"br\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"map\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"sub\x00" as *const u8 as *const libc::c_char,
    b"sup\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    b"input\x00" as *const u8 as *const libc::c_char,
    b"select\x00" as *const u8 as *const libc::c_char,
    b"textarea\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"button\x00" as *const u8 as *const libc::c_char,
    b"param\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut object_attrs: [*const libc::c_char; 29] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"declare\x00" as *const u8 as *const libc::c_char,
    b"classid\x00" as *const u8 as *const libc::c_char,
    b"codebase\x00" as *const u8 as *const libc::c_char,
    b"data\x00" as *const u8 as *const libc::c_char,
    b"type\x00" as *const u8 as *const libc::c_char,
    b"codetype\x00" as *const u8 as *const libc::c_char,
    b"archive\x00" as *const u8 as *const libc::c_char,
    b"standby\x00" as *const u8 as *const libc::c_char,
    b"height\x00" as *const u8 as *const libc::c_char,
    b"width\x00" as *const u8 as *const libc::c_char,
    b"usemap\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"tabindex\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut object_depr: [*const libc::c_char; 5] = [b"align\x00" as *const u8 as *const libc::c_char,
    b"border\x00" as *const u8 as *const libc::c_char,
    b"hspace\x00" as *const u8 as *const libc::c_char,
    b"vspace\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut ol_attrs: [*const libc::c_char; 4] = [b"type\x00" as *const u8 as *const libc::c_char,
    b"compact\x00" as *const u8 as *const libc::c_char,
    b"start\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut option_elt: [*const libc::c_char; 2] = [b"option\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut optgroup_attrs: [*const libc::c_char; 17] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"disabled\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut option_attrs: [*const libc::c_char; 20] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"disabled\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"selected\x00" as *const u8 as *const libc::c_char,
    b"value\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut param_attrs: [*const libc::c_char; 5] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"value\x00" as *const u8 as *const libc::c_char,
    b"valuetype\x00" as *const u8 as *const libc::c_char,
    b"type\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut width_attr: [*const libc::c_char; 2] = [b"width\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut pre_content: [*const libc::c_char; 25] = [b"em\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"tt\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"a\x00" as *const u8 as *const libc::c_char,
    b"br\x00" as *const u8 as *const libc::c_char,
    b"script\x00" as *const u8 as *const libc::c_char,
    b"map\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut script_attrs: [*const libc::c_char; 6] = [b"charset\x00" as *const u8 as *const libc::c_char,
    b"src\x00" as *const u8 as *const libc::c_char,
    b"defer\x00" as *const u8 as *const libc::c_char,
    b"event\x00" as *const u8 as *const libc::c_char,
    b"for\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut language_attr: [*const libc::c_char; 2] = [b"language\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut select_content: [*const libc::c_char; 3] = [b"optgroup\x00" as *const u8 as *const libc::c_char,
    b"option\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut select_attrs: [*const libc::c_char; 24] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"size\x00" as *const u8 as *const libc::c_char,
    b"multiple\x00" as *const u8 as *const libc::c_char,
    b"disabled\x00" as *const u8 as *const libc::c_char,
    b"tabindex\x00" as *const u8 as *const libc::c_char,
    b"onfocus\x00" as *const u8 as *const libc::c_char,
    b"onblur\x00" as *const u8 as *const libc::c_char,
    b"onchange\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut style_attrs: [*const libc::c_char; 5] = [b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"media\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut table_attrs: [*const libc::c_char; 24] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"summary\x00" as *const u8 as *const libc::c_char,
    b"width\x00" as *const u8 as *const libc::c_char,
    b"border\x00" as *const u8 as *const libc::c_char,
    b"frame\x00" as *const u8 as *const libc::c_char,
    b"rules\x00" as *const u8 as *const libc::c_char,
    b"cellspacing\x00" as *const u8 as *const libc::c_char,
    b"cellpadding\x00" as *const u8 as *const libc::c_char,
    b"datapagesize\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut table_depr: [*const libc::c_char; 3] = [b"align\x00" as *const u8 as *const libc::c_char,
    b"bgcolor\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut table_contents: [*const libc::c_char; 8] = [b"caption\x00" as *const u8 as *const libc::c_char,
    b"col\x00" as *const u8 as *const libc::c_char,
    b"colgroup\x00" as *const u8 as *const libc::c_char,
    b"thead\x00" as *const u8 as *const libc::c_char,
    b"tfoot\x00" as *const u8 as *const libc::c_char,
    b"tbody\x00" as *const u8 as *const libc::c_char,
    b"tr\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
static mut tr_elt: [*const libc::c_char; 2] = [b"tr\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
static mut talign_attrs: [*const libc::c_char; 20] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"align\x00" as *const u8 as *const libc::c_char,
    b"char\x00" as *const u8 as *const libc::c_char,
    b"charoff\x00" as *const u8 as *const libc::c_char,
    b"valign\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut th_td_depr: [*const libc::c_char; 5] = [b"nowrap\x00" as *const u8 as *const libc::c_char,
    b"bgcolor\x00" as *const u8 as *const libc::c_char,
    b"width\x00" as *const u8 as *const libc::c_char,
    b"height\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut th_td_attr: [*const libc::c_char; 26] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"axis\x00" as *const u8 as *const libc::c_char,
    b"headers\x00" as *const u8 as *const libc::c_char,
    b"scope\x00" as *const u8 as *const libc::c_char,
    b"rowspan\x00" as *const u8 as *const libc::c_char,
    b"colspan\x00" as *const u8 as *const libc::c_char,
    b"align\x00" as *const u8 as *const libc::c_char,
    b"char\x00" as *const u8 as *const libc::c_char,
    b"charoff\x00" as *const u8 as *const libc::c_char,
    b"valign\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut textarea_attrs: [*const libc::c_char; 25] = [b"id\x00" as *const u8 as *const libc::c_char,
    b"class\x00" as *const u8 as *const libc::c_char,
    b"style\x00" as *const u8 as *const libc::c_char,
    b"title\x00" as *const u8 as *const libc::c_char,
    b"lang\x00" as *const u8 as *const libc::c_char,
    b"dir\x00" as *const u8 as *const libc::c_char,
    b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"disabled\x00" as *const u8 as *const libc::c_char,
    b"readonly\x00" as *const u8 as *const libc::c_char,
    b"tabindex\x00" as *const u8 as *const libc::c_char,
    b"accesskey\x00" as *const u8 as *const libc::c_char,
    b"onfocus\x00" as *const u8 as *const libc::c_char,
    b"onblur\x00" as *const u8 as *const libc::c_char,
    b"onselect\x00" as *const u8 as *const libc::c_char,
    b"onchange\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut tr_contents: [*const libc::c_char; 3] = [b"th\x00" as *const u8 as *const libc::c_char,
    b"td\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
static mut bgcolor_attr: [*const libc::c_char; 2] = [b"bgcolor\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut li_elt: [*const libc::c_char; 2] = [b"li\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
static mut ul_depr: [*const libc::c_char; 3] = [b"type\x00" as *const u8 as *const libc::c_char,
    b"compact\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut dir_attr: [*const libc::c_char; 2] = [b"dir\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
static mut html40ElementTable: [htmlElemDesc; 92] = unsafe {
    [{
        let mut init = _htmlElemDesc {
            name:
            b"a\x00" as *const u8 as *const libc::c_char,
            startTag: 0 as libc::c_int as libc::c_char,
            endTag: 0 as libc::c_int as libc::c_char,
            saveEndTag: 0 as libc::c_int as libc::c_char,
            empty: 0 as libc::c_int as libc::c_char,
            depr: 0 as libc::c_int as libc::c_char,
            dtd: 0 as libc::c_int as libc::c_char,
            isinline: 1 as libc::c_int as libc::c_char,
            desc:
            b"anchor \x00" as *const u8 as *const libc::c_char,
            subelts:
            html_inline.as_ptr() as *mut *const libc::c_char,
            defaultsubelt: 0 as *const libc::c_char,
            attrs_opt:
            a_attrs.as_ptr() as *mut *const libc::c_char,
            attrs_depr:
            target_attr.as_ptr() as *mut *const libc::c_char,
            attrs_req:
            0 as *const *const libc::c_char as *mut *const libc::c_char,
        };
        init
    },
        {
            let mut init = _htmlElemDesc {
                name:
                b"abbr\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"abbreviated form\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"acronym\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"address\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"information on author \x00" as *const u8
                    as *const libc::c_char,
                subelts:
                inline_p.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"applet\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 2 as libc::c_int as libc::c_char,
                desc:
                b"java applet \x00" as *const u8 as *const libc::c_char,
                subelts:
                flow_param.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                applet_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"area\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"client-side image map area \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                area_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                target_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                alt_attr.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"b\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"bold text style\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"base\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"document base uri \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                target_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                href_attrs.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"basefont\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"base font size \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                basefont_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"bdo\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"i18n bidi over-ride \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                core_i18n_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                dir_attr.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"big\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"large text style\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"blockquote\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"long quotation \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                quote_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"body\x00" as *const u8 as *const libc::c_char,
                startTag: 1 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"document body \x00" as *const u8 as *const libc::c_char,
                subelts:
                body_contents.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"div\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                body_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                body_depr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"br\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"forced line break \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                core_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                clear_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"button\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 2 as libc::c_int as libc::c_char,
                desc:
                b"push button \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                button_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"caption\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"table caption \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"center\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"shorthand for div align=center \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"cite\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"citation\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"code\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"computer code fragment\x00" as *const u8
                    as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"col\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"table column \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                col_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"colgroup\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"table column group \x00" as *const u8 as *const libc::c_char,
                subelts:
                col_elt.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"col\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                col_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"dd\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"definition description \x00" as *const u8
                    as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"del\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 2 as libc::c_int as libc::c_char,
                desc:
                b"deleted text \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                edit_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"dfn\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"instance definition\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"dir\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"directory list\x00" as *const u8 as *const libc::c_char,
                subelts:
                blockli_elt.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"li\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                compact_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"div\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"generic language/style container\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"dl\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"definition list \x00" as *const u8 as *const libc::c_char,
                subelts:
                dl_contents.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"dd\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                compact_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"dt\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"definition term \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"em\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"emphasis\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"embed\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"generic embedded object \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                embed_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"fieldset\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"form control group \x00" as *const u8 as *const libc::c_char,
                subelts:
                fieldset_contents.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"font\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"local change to font \x00" as *const u8
                    as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                font_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"form\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"interactive form \x00" as *const u8 as *const libc::c_char,
                subelts:
                form_contents.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"fieldset\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                form_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                target_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                action_attr.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"frame\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 2 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"subwindow \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                frame_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"frameset\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 2 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"window subdivision\x00" as *const u8 as *const libc::c_char,
                subelts:
                frameset_contents.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"noframes\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                frameset_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"h1\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"heading \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"h2\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"heading \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"h3\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"heading \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"h4\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"heading \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"h5\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"heading \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"h6\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"heading \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"head\x00" as *const u8 as *const libc::c_char,
                startTag: 1 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"document head \x00" as *const u8 as *const libc::c_char,
                subelts:
                head_contents.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                head_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"hr\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"horizontal rule \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                hr_depr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"html\x00" as *const u8 as *const libc::c_char,
                startTag: 1 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"document root element \x00" as *const u8
                    as *const libc::c_char,
                subelts:
                html_content.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                i18n_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                version_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"i\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"italic text style\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"iframe\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 2 as libc::c_int as libc::c_char,
                desc:
                b"inline subwindow \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                iframe_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"img\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"embedded image \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                img_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                src_alt_attrs.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"input\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"form control \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                input_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"ins\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 2 as libc::c_int as libc::c_char,
                desc:
                b"inserted text\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                edit_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"isindex\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"single line prompt \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                prompt_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"kbd\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"text to be entered by the user\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"label\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"form field label text \x00" as *const u8
                    as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                label_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"legend\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"fieldset legend \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                legend_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"li\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 1 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"list item \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"link\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"a media-independent link \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                link_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                target_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"map\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 2 as libc::c_int as libc::c_char,
                desc:
                b"client-side image map \x00" as *const u8
                    as *const libc::c_char,
                subelts:
                map_contents.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                name_attr.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"menu\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"menu list \x00" as *const u8 as *const libc::c_char,
                subelts:
                blockli_elt.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                compact_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"meta\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"generic metainformation \x00" as *const u8 as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                meta_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                content_attr.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"noframes\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 2 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"alternate content container for non frame-based rendering \x00"
                    as *const u8 as *const libc::c_char,
                subelts:
                noframes_content.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"body\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"noscript\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"alternate content container for non script-based rendering \x00"
                    as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"div\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"object\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 2 as libc::c_int as libc::c_char,
                desc:
                b"generic embedded object \x00" as *const u8 as *const libc::c_char,
                subelts:
                object_contents.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"div\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                object_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                object_depr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"ol\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"ordered list \x00" as *const u8 as *const libc::c_char,
                subelts:
                li_elt.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"li\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                ol_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"optgroup\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"option group \x00" as *const u8 as *const libc::c_char,
                subelts:
                option_elt.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"option\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                optgroup_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                label_attr.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"option\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"selectable choice \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_pcdata.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                option_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"p\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"paragraph \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                align_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"param\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 2 as libc::c_int as libc::c_char,
                saveEndTag: 2 as libc::c_int as libc::c_char,
                empty: 1 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"named property value \x00" as *const u8
                    as *const libc::c_char,
                subelts:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                param_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                name_attr.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"pre\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"preformatted text \x00" as *const u8 as *const libc::c_char,
                subelts:
                pre_content.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                width_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"q\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"short inline quotation \x00" as *const u8
                    as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                quote_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"s\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"strike-through text style\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"samp\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"sample program output, scripts, etc.\x00"
                    as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"script\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 2 as libc::c_int as libc::c_char,
                desc:
                b"script statements \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_pcdata.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                script_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                language_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                type_attr.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"select\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"option selector \x00" as *const u8 as *const libc::c_char,
                subelts:
                select_content.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                select_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"small\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"small text style\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"span\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"generic language/style container \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"strike\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"strike-through text\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"strong\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"strong emphasis\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"style\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"style info \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_pcdata.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                style_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                type_attr.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"sub\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"subscript\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"sup\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"superscript \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"table\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"\x00" as *const u8 as *const libc::c_char,
                subelts:
                table_contents.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"tr\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                table_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                table_depr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"tbody\x00" as *const u8 as *const libc::c_char,
                startTag: 1 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"table body \x00" as *const u8 as *const libc::c_char,
                subelts:
                tr_elt.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"tr\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                talign_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"td\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"table data cell\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                th_td_attr.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                th_td_depr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"textarea\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"multi-line text field \x00" as *const u8
                    as *const libc::c_char,
                subelts:
                html_pcdata.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                textarea_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                rows_cols_attr.as_ptr() as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"tfoot\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"table footer \x00" as *const u8 as *const libc::c_char,
                subelts:
                tr_elt.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"tr\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                talign_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"th\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"table header cell\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_flow.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                th_td_attr.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                th_td_depr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"thead\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 1 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"table header \x00" as *const u8 as *const libc::c_char,
                subelts:
                tr_elt.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"tr\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                talign_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"title\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"document title \x00" as *const u8 as *const libc::c_char,
                subelts:
                html_pcdata.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                i18n_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"tr\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"table row \x00" as *const u8 as *const libc::c_char,
                subelts:
                tr_contents.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"td\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                talign_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                bgcolor_attr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"tt\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"teletype or monospaced text style\x00" as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"u\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 3 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 1 as libc::c_int as libc::c_char,
                dtd: 1 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"underlined text style\x00" as *const u8
                    as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_depr:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"ul\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 0 as libc::c_int as libc::c_char,
                desc:
                b"unordered list \x00" as *const u8 as *const libc::c_char,
                subelts:
                li_elt.as_ptr() as *mut *const libc::c_char,
                defaultsubelt:
                b"li\x00" as *const u8 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                ul_depr.as_ptr() as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        },
        {
            let mut init = _htmlElemDesc {
                name:
                b"var\x00" as *const u8 as *const libc::c_char,
                startTag: 0 as libc::c_int as libc::c_char,
                endTag: 0 as libc::c_int as libc::c_char,
                saveEndTag: 0 as libc::c_int as libc::c_char,
                empty: 0 as libc::c_int as libc::c_char,
                depr: 0 as libc::c_int as libc::c_char,
                dtd: 0 as libc::c_int as libc::c_char,
                isinline: 1 as libc::c_int as libc::c_char,
                desc:
                b"instance of a variable or program argument\x00"
                    as *const u8 as *const libc::c_char,
                subelts:
                html_inline.as_ptr() as *mut *const libc::c_char,
                defaultsubelt: 0 as *const libc::c_char,
                attrs_opt:
                html_attrs.as_ptr() as *mut *const libc::c_char,
                attrs_depr:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
                attrs_req:
                0 as *const *const libc::c_char as *mut *const libc::c_char,
            };
            init
        }]
};
/*
 * start tags that imply the end of current element
 */
static mut htmlStartClose: [htmlStartCloseEntry; 251] = [{
    let mut init = htmlStartCloseEntry {
        oldTag:
        b"a\x00" as *const u8 as *const libc::c_char,
        newTag:
        b"a\x00" as *const u8 as *const libc::c_char,
    };
    init
},
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"a\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"a\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"a\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"td\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"a\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"th\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"address\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dd\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"address\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"address\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"address\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"address\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"address\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"b\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"center\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"b\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"b\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"td\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"b\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"th\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"big\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"caption\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"col\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"caption\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"colgroup\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"caption\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"caption\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"caption\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"thead\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"caption\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"col\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"col\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"col\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"colgroup\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"col\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"col\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"col\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"thead\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"col\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"colgroup\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"colgroup\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"colgroup\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"colgroup\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"colgroup\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"thead\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"colgroup\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"dd\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"dir\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dd\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"dir\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"dir\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"dir\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"dir\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dd\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"font\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"center\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"font\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"td\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"font\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"th\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"form\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h1\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h1\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h1\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h1\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h1\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h2\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h2\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h2\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h2\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h2\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h3\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h3\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h3\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h3\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h3\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h4\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h4\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h4\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h4\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h4\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h5\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h5\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h5\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h5\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h5\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h6\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h6\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h6\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h6\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"h6\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"a\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"abbr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"acronym\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"address\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"b\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"bdo\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"big\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"blockquote\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"body\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"br\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"center\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"cite\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"code\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dd\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dfn\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dir\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"div\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"em\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"font\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"frameset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h2\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"hr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"i\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"iframe\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"img\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"kbd\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"listing\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"map\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"menu\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ol\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"q\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"s\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"samp\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"small\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"span\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"strike\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"strong\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"sub\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"sup\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"u\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"var\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"head\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"xmp\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"hr\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"i\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"center\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"i\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"i\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"td\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"i\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"th\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"legend\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"li\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"link\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"body\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"link\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"frameset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"listing\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dd\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"listing\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"listing\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"listing\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"listing\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"listing\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"listing\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"listing\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"menu\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dd\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"menu\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"menu\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"menu\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"menu\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"ol\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"ol\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"option\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"optgroup\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"option\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"option\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"address\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"blockquote\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"body\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"caption\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"center\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"col\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"colgroup\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dd\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dir\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"div\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"frameset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h2\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"h6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"head\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"hr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"listing\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"menu\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ol\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"td\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"th\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"title\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"p\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"xmp\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dd\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"s\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"script\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"noscript\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"small\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"span\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"td\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"span\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"th\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"strike\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"style\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"body\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"style\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"frameset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"td\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"td\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"td\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"td\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"td\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"th\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"td\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"th\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"th\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"td\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"th\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"th\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"th\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"th\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"thead\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"thead\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"title\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"body\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"title\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"frameset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"tr\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tbody\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"tr\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"tr\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"tr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"tt\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"u\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"p\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"u\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"td\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"u\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"th\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"address\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"menu\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ol\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"pre\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"xmp\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dd\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"xmp\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"xmp\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"dt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"xmp\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"fieldset\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"xmp\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"form\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"xmp\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"li\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"xmp\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"table\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = htmlStartCloseEntry {
            oldTag:
            b"xmp\x00" as *const u8 as *const libc::c_char,
            newTag:
            b"ul\x00" as *const u8 as *const libc::c_char,
        };
        init
    }];
/*
 * The list of HTML elements which are supposed not to have
 * CDATA content and where a p element will be implied
 *
 * TODO: extend that list by reading the HTML SGML DTD on
 *       implied paragraph
 */
static mut htmlNoContentElements: [*const libc::c_char; 3] = [b"html\x00" as *const u8 as *const libc::c_char,
    b"head\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char];
/*
 * The list of HTML attributes which are of content %Script;
 * NOTE: when adding ones, check htmlIsScriptAttribute() since
 *       it assumes the name starts with 'on'
 */
static mut htmlScriptAttributes: [*const libc::c_char; 18] = [b"onclick\x00" as *const u8 as *const libc::c_char,
    b"ondblclick\x00" as *const u8 as *const libc::c_char,
    b"onmousedown\x00" as *const u8 as *const libc::c_char,
    b"onmouseup\x00" as *const u8 as *const libc::c_char,
    b"onmouseover\x00" as *const u8 as *const libc::c_char,
    b"onmousemove\x00" as *const u8 as *const libc::c_char,
    b"onmouseout\x00" as *const u8 as *const libc::c_char,
    b"onkeypress\x00" as *const u8 as *const libc::c_char,
    b"onkeydown\x00" as *const u8 as *const libc::c_char,
    b"onkeyup\x00" as *const u8 as *const libc::c_char,
    b"onload\x00" as *const u8 as *const libc::c_char,
    b"onunload\x00" as *const u8 as *const libc::c_char,
    b"onfocus\x00" as *const u8 as *const libc::c_char,
    b"onblur\x00" as *const u8 as *const libc::c_char,
    b"onsubmit\x00" as *const u8 as *const libc::c_char,
    b"onreset\x00" as *const u8 as *const libc::c_char,
    b"onchange\x00" as *const u8 as *const libc::c_char,
    b"onselect\x00" as *const u8 as *const libc::c_char];
static mut htmlEndPriority: [elementPriority; 12] = [{
    let mut init = elementPriority {
        name:
        b"div\x00" as *const u8 as *const libc::c_char,
        priority: 150 as libc::c_int,
    };
    init
},
    {
        let mut init = elementPriority {
            name:
            b"td\x00" as *const u8 as *const libc::c_char,
            priority: 160 as libc::c_int,
        };
        init
    },
    {
        let mut init = elementPriority {
            name:
            b"th\x00" as *const u8 as *const libc::c_char,
            priority: 160 as libc::c_int,
        };
        init
    },
    {
        let mut init = elementPriority {
            name:
            b"tr\x00" as *const u8 as *const libc::c_char,
            priority: 170 as libc::c_int,
        };
        init
    },
    {
        let mut init = elementPriority {
            name:
            b"thead\x00" as *const u8 as *const libc::c_char,
            priority: 180 as libc::c_int,
        };
        init
    },
    {
        let mut init = elementPriority {
            name:
            b"tbody\x00" as *const u8 as *const libc::c_char,
            priority: 180 as libc::c_int,
        };
        init
    },
    {
        let mut init = elementPriority {
            name:
            b"tfoot\x00" as *const u8 as *const libc::c_char,
            priority: 180 as libc::c_int,
        };
        init
    },
    {
        let mut init = elementPriority {
            name:
            b"table\x00" as *const u8 as *const libc::c_char,
            priority: 190 as libc::c_int,
        };
        init
    },
    {
        let mut init = elementPriority {
            name:
            b"head\x00" as *const u8 as *const libc::c_char,
            priority: 200 as libc::c_int,
        };
        init
    },
    {
        let mut init = elementPriority {
            name:
            b"body\x00" as *const u8 as *const libc::c_char,
            priority: 200 as libc::c_int,
        };
        init
    },
    {
        let mut init = elementPriority {
            name:
            b"html\x00" as *const u8 as *const libc::c_char,
            priority: 220 as libc::c_int,
        };
        init
    },
    {
        let mut init = elementPriority {
            name: 0 as *const libc::c_char,
            priority: 100 as libc::c_int,
        };
        init
    }];
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

extern "C" fn htmlCompareTags(mut key: *const libc::c_void,
                              mut member: *const libc::c_void)
                              -> libc::c_int {
    let mut tag: *const xmlChar = key as *const xmlChar;
    let mut desc: *const htmlElemDesc = member as *const htmlElemDesc;
    let mut descPtr = unsafe { &*desc };
    return xmlStrcasecmp_safe(tag, descPtr.name as *mut xmlChar);
}
/* *
 * htmlTagLookup:
 * @tag:  The tag name in lowercase
 *
 * Lookup the HTML tag in the ElementTable
 *
 * Returns the related htmlElemDescPtr or NULL if not found.
 */

pub fn htmlTagLookup(mut tag:
                     *const xmlChar)
                     -> *const htmlElemDesc {
    if tag.is_null() { return 0 as *const htmlElemDesc; }
    unsafe {
        return bsearch(tag as *const libc::c_void,
                       html40ElementTable.as_ptr() as *const libc::c_void,
                       (::std::mem::size_of::<[htmlElemDesc; 92]>() as libc::c_ulong).wrapping_div(::std::mem::size_of::<htmlElemDesc>()
                           as libc::c_ulong),
                       ::std::mem::size_of::<htmlElemDesc>() as libc::c_ulong,
                       Some(htmlCompareTags as unsafe extern "C" fn(_: *const libc::c_void,
                                                                    _: *const libc::c_void)
                                                                    -> libc::c_int)) as *const htmlElemDesc;
    }
}
/* *
 * htmlGetEndPriority:
 * @name: The name of the element to look up the priority for.
 *
 * Return value: The "endtag" priority.
 **/
fn htmlGetEndPriority(mut name: *const xmlChar)
                      -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !getHtmlEndPriority(i as usize).name.is_null() && xmlStrEqual_safe(getHtmlEndPriority(i as usize).name as *const xmlChar, name) == 0 {
        i += 1
    }
    return getHtmlEndPriority(i as usize).priority;
}

extern "C" fn htmlCompareStartClose(mut vkey: *const libc::c_void,
                                    mut member: *const libc::c_void)
                                    -> libc::c_int {
    let mut key: *const htmlStartCloseEntry = vkey as *const htmlStartCloseEntry;
    let mut entry: *const htmlStartCloseEntry = member as *const htmlStartCloseEntry;
    let mut ret: libc::c_int = 0;
    let keyPtr = unsafe { &*key };
    let entryPtr = unsafe { &*entry };
    ret = strcmp_safe(keyPtr.oldTag, entryPtr.oldTag);
    if ret == 0 as libc::c_int {
        ret = strcmp_safe(keyPtr.newTag, entryPtr.newTag)
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
fn htmlCheckAutoClose(mut newtag: *const xmlChar,
                      mut oldtag: *const xmlChar)
                      -> libc::c_int {
    let mut key: htmlStartCloseEntry = htmlStartCloseEntry {
        oldTag: 0 as *const libc::c_char,
        newTag: 0 as *const libc::c_char,
    };
    let mut res: *mut libc::c_void = 0 as *mut libc::c_void;
    key.oldTag = oldtag as *const libc::c_char;
    key.newTag = newtag as *const libc::c_char;
    unsafe {
        res = bsearch(&mut key as *mut htmlStartCloseEntry as *const libc::c_void,
                      htmlStartClose.as_ptr() as *const libc::c_void,
                      (::std::mem::size_of::<[htmlStartCloseEntry; 251]>() as libc::c_ulong).wrapping_div(::std::mem::size_of::<htmlStartCloseEntry>()
                          as libc::c_ulong),
                      ::std::mem::size_of::<htmlStartCloseEntry>() as libc::c_ulong,
                      Some(htmlCompareStartClose as unsafe extern "C" fn(_: *const libc::c_void,
                                                                         _: *const libc::c_void)
                                                                         -> libc::c_int));
    }
    return (res != 0 as *mut libc::c_void) as libc::c_int;
}
/* *
 * htmlAutoCloseOnClose:
 * @ctxt:  an HTML parser context
 * @newtag:  The new tag name
 * @force:  force the tag closure
 *
 * The HTML DTD allows an ending tag to implicitly close other tags.
 */
fn htmlAutoCloseOnClose(mut ctxt: htmlParserCtxtPtr,
                        mut newtag: *const xmlChar) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut info: *const htmlElemDesc = 0 as *const htmlElemDesc;
    let mut i: libc::c_int = 0;
    let mut priority: libc::c_int = 0;
    priority = htmlGetEndPriority(newtag);
    i = ctxtPtr.nameNr - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        unsafe {
            if xmlStrEqual_safe(newtag,
                                *ctxtPtr.nameTab.offset(i as isize)) != 0 {
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
            if htmlGetEndPriority(*ctxtPtr.nameTab.offset(i as isize)) > priority
            {
                return;
            }
        }
        i -= 1
    }
    if i < 0 as libc::c_int { return; }

    while xmlStrEqual_safe(newtag, ctxtPtr.name) == 0 {
        info = htmlTagLookup(ctxtPtr.name);
        let info_condition = unsafe {
            !info.is_null() && (*info).endTag as libc::c_int == 3 as libc::c_int
        };
        if info_condition {
            htmlParseErr(ctxt, XML_ERR_TAG_NAME_MISMATCH,
                         b"Opening and ending tag mismatch: %s and %s\n\x00"
                             as *const u8 as *const libc::c_char, newtag,
                         ctxtPtr.name);
        }
        let sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some()
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData,
                                          ctxtPtr.name);
        }
        htmlnamePop(ctxt);
    };
}
/* *
 * htmlAutoCloseOnEnd:
 * @ctxt:  an HTML parser context
 *
 * Close all remaining tags at the end of the stream
 */
fn htmlAutoCloseOnEnd(mut ctxt: htmlParserCtxtPtr) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut i: libc::c_int = 0;
    if ctxtPtr.nameNr == 0 as libc::c_int { return; }
    i = ctxtPtr.nameNr - 1 as libc::c_int;
    let mut sax_condition = false;
    while i >= 0 as libc::c_int {
        sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some()
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData,
                                          ctxtPtr.name);
        }
        htmlnamePop(ctxt);
        i -= 1
    };
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
fn htmlAutoClose(mut ctxt: htmlParserCtxtPtr,
                 mut newtag: *const xmlChar) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut sax_condition = false;
    while !newtag.is_null() && !ctxtPtr.name.is_null() && htmlCheckAutoClose(newtag, ctxtPtr.name) != 0 {
        sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some()
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData,
                                          ctxtPtr.name);
        }
        htmlnamePop(ctxt);
    }
    if newtag.is_null() {
        htmlAutoCloseOnEnd(ctxt);
        return;
    }
    while newtag.is_null() && !ctxtPtr.name.is_null() && (xmlStrEqual_safe(ctxtPtr.name,
                                                                           b"head\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(ctxtPtr.name,
                                                                                                                                                                     b"body\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(ctxtPtr.name,
                                                                                                                                                                                                                                                               b"html\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0) {
        sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some()
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData,
                                          ctxtPtr.name);
        }
        htmlnamePop(ctxt);
    };
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

pub fn htmlAutoCloseTag(mut doc: htmlDocPtr,
                        mut name:
                        *const xmlChar,
                        mut elem:
                        htmlNodePtr)
                        -> libc::c_int {
    let mut child: htmlNodePtr = 0 as *mut xmlNode;
    if elem.is_null() { return 1 as libc::c_int; }
    let mut elemPtr = unsafe { &mut *elem };
    if xmlStrEqual_safe(name, elemPtr.name) != 0 {
        return 0 as libc::c_int;
    }
    if htmlCheckAutoClose(elemPtr.name, name) != 0 { return 1 as libc::c_int; }
    child = elemPtr.children;
    while !child.is_null() {
        if htmlAutoCloseTag(doc, name, child) != 0 {
            return 1 as libc::c_int;
        }
        unsafe { child = (*child).next }
    }
    return 0 as libc::c_int;
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

pub fn htmlIsAutoClosed(mut doc: htmlDocPtr,
                        mut elem:
                        htmlNodePtr)
                        -> libc::c_int {
    let mut child: htmlNodePtr = 0 as *mut xmlNode;
    if elem.is_null() { return 1 as libc::c_int; }
    let mut elemPtr = unsafe { &mut *elem };
    child = elemPtr.children;
    while !child.is_null() {
        if htmlAutoCloseTag(doc, elemPtr.name, child) != 0 {
            return 1 as libc::c_int;
        }
        unsafe { child = (*child).next }
    }
    return 0 as libc::c_int;
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
fn htmlCheckImplied(mut ctxt: htmlParserCtxtPtr,
                    mut newtag: *const xmlChar) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut i: libc::c_int = 0;
    if ctxtPtr.options & HTML_PARSE_NOIMPLIED as libc::c_int != 0 { return; }
    if getHtmlOmittedDefaultValue() == 0 { return; }
    if xmlStrEqual_safe(newtag,
                        b"html\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
        return;
    }
    let mut sax_condition = false;
    if ctxtPtr.nameNr <= 0 as libc::c_int {
        htmlnamePush(ctxt,
                     b"html\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_startElement_safe(saxPtr.startElement, ctxtPtr.userData,
                                            b"html\x00"
                                                as *const u8
                                                as *const libc::c_char
                                                as *mut xmlChar,
                                            0
                                                as *mut *const xmlChar);
        }
    }
    if xmlStrEqual_safe(newtag,
                        b"body\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(newtag,
                                                                                                                  b"head\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
        return;
    }
    if ctxtPtr.nameNr <= 1 as libc::c_int && (xmlStrEqual_safe(newtag,
                                                               b"script\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(newtag,
                                                                                                                                                           b"style\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(newtag,
                                                                                                                                                                                                                                                      b"meta\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(newtag,
                                                                                                                                                                                                                                                                                                                                                b"link\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(newtag,
                                                                                                                                                                                                                                                                                                                                                                                                                                          b"title\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(newtag,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     b"base\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0) {
        if ctxtPtr.html >= 3 as libc::c_int {
            /* we already saw or generated an <head> before */
            return;
        }
        /*
         * dropped OBJECT ... i you put it first BODY will be
         * assumed !
         */
        htmlnamePush(ctxt,
                     b"head\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_startElement_safe(saxPtr.startElement, ctxtPtr.userData,
                                            b"head\x00"
                                                as *const u8
                                                as *const libc::c_char
                                                as *mut xmlChar,
                                            0
                                                as *mut *const xmlChar);
        }
    } else if xmlStrEqual_safe(newtag,
                               b"noframes\x00" as *const u8 as *const libc::c_char as *mut xmlChar) == 0 && xmlStrEqual_safe(newtag,
                                                                                                                             b"frame\x00" as *const u8 as *const libc::c_char as *mut xmlChar) == 0 && xmlStrEqual_safe(newtag,
                                                                                                                                                                                                                        b"frameset\x00" as *const u8 as *const libc::c_char as *mut xmlChar) == 0 {
        if ctxtPtr.html >= 10 as libc::c_int {
            /* we already saw or generated a <body> before */
            return;
        }
        i = 0 as libc::c_int;
        while i < ctxtPtr.nameNr {
            unsafe {
                if xmlStrEqual_safe(*ctxtPtr.nameTab.offset(i as isize),
                                    b"body\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
                    return;
                }
            }
            unsafe {
                if xmlStrEqual_safe(*ctxtPtr.nameTab.offset(i as isize),
                                    b"head\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
                    return;
                }
            }
            i += 1
        }
        htmlnamePush(ctxt,
                     b"body\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_startElement_safe(saxPtr.startElement, ctxtPtr.userData,
                                            b"body\x00"
                                                as *const u8
                                                as *const libc::c_char
                                                as *mut xmlChar,
                                            0
                                                as *mut *const xmlChar);
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
fn htmlCheckParagraph(mut ctxt: htmlParserCtxtPtr)
                      -> libc::c_int {
    let mut tag: *const xmlChar = 0 as *const xmlChar;
    let mut i: libc::c_int = 0;
    if ctxt.is_null() { return -(1 as libc::c_int); }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    tag = ctxtPtr.name;
    let mut sax_condition = false;
    if tag.is_null() {
        htmlAutoClose(ctxt,
                      b"p\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
        htmlCheckImplied(ctxt,
                         b"p\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
        htmlnamePush(ctxt,
                     b"p\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
        sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_startElement_safe(saxPtr.startElement, ctxtPtr.userData,
                                            b"p\x00"
                                                as *const u8
                                                as *const libc::c_char
                                                as *mut xmlChar,
                                            0
                                                as *mut *const xmlChar);
        }
        return 1 as libc::c_int;
    }
    if getHtmlOmittedDefaultValue() == 0 { return 0 as libc::c_int; }
    i = 0 as libc::c_int;
    while !getHtmlNoContentElements(i as usize).is_null() {
        if xmlStrEqual_safe(tag,
                            getHtmlNoContentElements(i as usize) as *mut xmlChar) != 0 {
            htmlAutoClose(ctxt,
                          b"p\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
            htmlCheckImplied(ctxt,
                             b"p\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
            htmlnamePush(ctxt,
                         b"p\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
            sax_condition = unsafe { !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some() };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_startElement_safe(saxPtr.startElement, ctxtPtr.userData,
                                                b"p\x00"
                                                    as *const u8
                                                    as *const libc::c_char
                                                    as *mut xmlChar,
                                                0
                                                    as *mut *const xmlChar);
            }
            return 1 as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* *
 * htmlIsScriptAttribute:
 * @name:  an attribute name
 *
 * Check if an attribute is of content type Script
 *
 * Returns 1 is the attribute is a script 0 otherwise
 */

pub fn htmlIsScriptAttribute(mut name:
                             *const xmlChar)
                             -> libc::c_int {
    let mut i: libc::c_uint = 0;
    if name.is_null() { return 0 as libc::c_int; }
    /*
     * all script attributes start with 'on'
     */
    unsafe {
        if *name.offset(0 as libc::c_int as isize) as libc::c_int != 'o' as i32 || *name.offset(1 as libc::c_int as isize) as libc::c_int != 'n' as i32 {
            return 0 as libc::c_int;
        }
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
        (::std::mem::size_of::<[*const libc::c_char; 18]>() as libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
            as libc::c_ulong) {
        if xmlStrEqual_safe(name,
                            getHtmlScriptAttributes(i as usize) as *const xmlChar) != 0 {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
/* ***********************************************************************
 *									*
 *	The list of HTML predefined entities			*
 *									*
 ************************************************************************/
static mut html40EntitiesTable: [htmlEntityDesc; 253] = [{
    let mut init = _htmlEntityDesc {
        value: 34 as libc::c_int as libc::c_uint,
        name:
        b"quot\x00" as *const u8 as *const libc::c_char,
        desc:
        b"quotation mark = APL quote, U+0022 ISOnum\x00"
            as *const u8 as *const libc::c_char,
    };
    init
},
    {
        let mut init = _htmlEntityDesc {
            value: 38 as libc::c_int as libc::c_uint,
            name:
            b"amp\x00" as *const u8 as *const libc::c_char,
            desc:
            b"ampersand, U+0026 ISOnum\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 39 as libc::c_int as libc::c_uint,
            name:
            b"apos\x00" as *const u8 as *const libc::c_char,
            desc:
            b"single quote\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 60 as libc::c_int as libc::c_uint,
            name:
            b"lt\x00" as *const u8 as *const libc::c_char,
            desc:
            b"less-than sign, U+003C ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 62 as libc::c_int as libc::c_uint,
            name:
            b"gt\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greater-than sign, U+003E ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 160 as libc::c_int as libc::c_uint,
            name:
            b"nbsp\x00" as *const u8 as *const libc::c_char,
            desc:
            b"no-break space = non-breaking space, U+00A0 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 161 as libc::c_int as libc::c_uint,
            name:
            b"iexcl\x00" as *const u8 as *const libc::c_char,
            desc:
            b"inverted exclamation mark, U+00A1 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 162 as libc::c_int as libc::c_uint,
            name:
            b"cent\x00" as *const u8 as *const libc::c_char,
            desc:
            b"cent sign, U+00A2 ISOnum\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 163 as libc::c_int as libc::c_uint,
            name:
            b"pound\x00" as *const u8 as *const libc::c_char,
            desc:
            b"pound sign, U+00A3 ISOnum\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 164 as libc::c_int as libc::c_uint,
            name:
            b"curren\x00" as *const u8 as *const libc::c_char,
            desc:
            b"currency sign, U+00A4 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 165 as libc::c_int as libc::c_uint,
            name:
            b"yen\x00" as *const u8 as *const libc::c_char,
            desc:
            b"yen sign = yuan sign, U+00A5 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 166 as libc::c_int as libc::c_uint,
            name:
            b"brvbar\x00" as *const u8 as *const libc::c_char,
            desc:
            b"broken bar = broken vertical bar, U+00A6 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 167 as libc::c_int as libc::c_uint,
            name:
            b"sect\x00" as *const u8 as *const libc::c_char,
            desc:
            b"section sign, U+00A7 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 168 as libc::c_int as libc::c_uint,
            name:
            b"uml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"diaeresis = spacing diaeresis, U+00A8 ISOdia\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 169 as libc::c_int as libc::c_uint,
            name:
            b"copy\x00" as *const u8 as *const libc::c_char,
            desc:
            b"copyright sign, U+00A9 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 170 as libc::c_int as libc::c_uint,
            name:
            b"ordf\x00" as *const u8 as *const libc::c_char,
            desc:
            b"feminine ordinal indicator, U+00AA ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 171 as libc::c_int as libc::c_uint,
            name:
            b"laquo\x00" as *const u8 as *const libc::c_char,
            desc:
            b"left-pointing double angle quotation mark = left pointing guillemet, U+00AB ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 172 as libc::c_int as libc::c_uint,
            name:
            b"not\x00" as *const u8 as *const libc::c_char,
            desc:
            b"not sign, U+00AC ISOnum\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 173 as libc::c_int as libc::c_uint,
            name:
            b"shy\x00" as *const u8 as *const libc::c_char,
            desc:
            b"soft hyphen = discretionary hyphen, U+00AD ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 174 as libc::c_int as libc::c_uint,
            name:
            b"reg\x00" as *const u8 as *const libc::c_char,
            desc:
            b"registered sign = registered trade mark sign, U+00AE ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 175 as libc::c_int as libc::c_uint,
            name:
            b"macr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"macron = spacing macron = overline = APL overbar, U+00AF ISOdia\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 176 as libc::c_int as libc::c_uint,
            name:
            b"deg\x00" as *const u8 as *const libc::c_char,
            desc:
            b"degree sign, U+00B0 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 177 as libc::c_int as libc::c_uint,
            name:
            b"plusmn\x00" as *const u8 as *const libc::c_char,
            desc:
            b"plus-minus sign = plus-or-minus sign, U+00B1 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 178 as libc::c_int as libc::c_uint,
            name:
            b"sup2\x00" as *const u8 as *const libc::c_char,
            desc:
            b"superscript two = superscript digit two = squared, U+00B2 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 179 as libc::c_int as libc::c_uint,
            name:
            b"sup3\x00" as *const u8 as *const libc::c_char,
            desc:
            b"superscript three = superscript digit three = cubed, U+00B3 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 180 as libc::c_int as libc::c_uint,
            name:
            b"acute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"acute accent = spacing acute, U+00B4 ISOdia\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 181 as libc::c_int as libc::c_uint,
            name:
            b"micro\x00" as *const u8 as *const libc::c_char,
            desc:
            b"micro sign, U+00B5 ISOnum\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 182 as libc::c_int as libc::c_uint,
            name:
            b"para\x00" as *const u8 as *const libc::c_char,
            desc:
            b"pilcrow sign = paragraph sign, U+00B6 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 183 as libc::c_int as libc::c_uint,
            name:
            b"middot\x00" as *const u8 as *const libc::c_char,
            desc:
            b"middle dot = Georgian comma Greek middle dot, U+00B7 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 184 as libc::c_int as libc::c_uint,
            name:
            b"cedil\x00" as *const u8 as *const libc::c_char,
            desc:
            b"cedilla = spacing cedilla, U+00B8 ISOdia\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 185 as libc::c_int as libc::c_uint,
            name:
            b"sup1\x00" as *const u8 as *const libc::c_char,
            desc:
            b"superscript one = superscript digit one, U+00B9 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 186 as libc::c_int as libc::c_uint,
            name:
            b"ordm\x00" as *const u8 as *const libc::c_char,
            desc:
            b"masculine ordinal indicator, U+00BA ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 187 as libc::c_int as libc::c_uint,
            name:
            b"raquo\x00" as *const u8 as *const libc::c_char,
            desc:
            b"right-pointing double angle quotation mark right pointing guillemet, U+00BB ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 188 as libc::c_int as libc::c_uint,
            name:
            b"frac14\x00" as *const u8 as *const libc::c_char,
            desc:
            b"vulgar fraction one quarter = fraction one quarter, U+00BC ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 189 as libc::c_int as libc::c_uint,
            name:
            b"frac12\x00" as *const u8 as *const libc::c_char,
            desc:
            b"vulgar fraction one half = fraction one half, U+00BD ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 190 as libc::c_int as libc::c_uint,
            name:
            b"frac34\x00" as *const u8 as *const libc::c_char,
            desc:
            b"vulgar fraction three quarters = fraction three quarters, U+00BE ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 191 as libc::c_int as libc::c_uint,
            name:
            b"iquest\x00" as *const u8 as *const libc::c_char,
            desc:
            b"inverted question mark = turned question mark, U+00BF ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 192 as libc::c_int as libc::c_uint,
            name:
            b"Agrave\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter A with grave = latin capital letter A grave, U+00C0 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 193 as libc::c_int as libc::c_uint,
            name:
            b"Aacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter A with acute, U+00C1 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 194 as libc::c_int as libc::c_uint,
            name:
            b"Acirc\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter A with circumflex, U+00C2 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 195 as libc::c_int as libc::c_uint,
            name:
            b"Atilde\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter A with tilde, U+00C3 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 196 as libc::c_int as libc::c_uint,
            name:
            b"Auml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter A with diaeresis, U+00C4 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 197 as libc::c_int as libc::c_uint,
            name:
            b"Aring\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter A with ring above = latin capital letter A ring, U+00C5 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 198 as libc::c_int as libc::c_uint,
            name:
            b"AElig\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter AE = latin capital ligature AE, U+00C6 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 199 as libc::c_int as libc::c_uint,
            name:
            b"Ccedil\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter C with cedilla, U+00C7 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 200 as libc::c_int as libc::c_uint,
            name:
            b"Egrave\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter E with grave, U+00C8 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 201 as libc::c_int as libc::c_uint,
            name:
            b"Eacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter E with acute, U+00C9 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 202 as libc::c_int as libc::c_uint,
            name:
            b"Ecirc\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter E with circumflex, U+00CA ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 203 as libc::c_int as libc::c_uint,
            name:
            b"Euml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter E with diaeresis, U+00CB ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 204 as libc::c_int as libc::c_uint,
            name:
            b"Igrave\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter I with grave, U+00CC ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 205 as libc::c_int as libc::c_uint,
            name:
            b"Iacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter I with acute, U+00CD ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 206 as libc::c_int as libc::c_uint,
            name:
            b"Icirc\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter I with circumflex, U+00CE ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 207 as libc::c_int as libc::c_uint,
            name:
            b"Iuml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter I with diaeresis, U+00CF ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 208 as libc::c_int as libc::c_uint,
            name:
            b"ETH\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter ETH, U+00D0 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 209 as libc::c_int as libc::c_uint,
            name:
            b"Ntilde\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter N with tilde, U+00D1 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 210 as libc::c_int as libc::c_uint,
            name:
            b"Ograve\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter O with grave, U+00D2 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 211 as libc::c_int as libc::c_uint,
            name:
            b"Oacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter O with acute, U+00D3 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 212 as libc::c_int as libc::c_uint,
            name:
            b"Ocirc\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter O with circumflex, U+00D4 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 213 as libc::c_int as libc::c_uint,
            name:
            b"Otilde\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter O with tilde, U+00D5 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 214 as libc::c_int as libc::c_uint,
            name:
            b"Ouml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter O with diaeresis, U+00D6 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 215 as libc::c_int as libc::c_uint,
            name:
            b"times\x00" as *const u8 as *const libc::c_char,
            desc:
            b"multiplication sign, U+00D7 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 216 as libc::c_int as libc::c_uint,
            name:
            b"Oslash\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter O with stroke latin capital letter O slash, U+00D8 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 217 as libc::c_int as libc::c_uint,
            name:
            b"Ugrave\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter U with grave, U+00D9 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 218 as libc::c_int as libc::c_uint,
            name:
            b"Uacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter U with acute, U+00DA ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 219 as libc::c_int as libc::c_uint,
            name:
            b"Ucirc\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter U with circumflex, U+00DB ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 220 as libc::c_int as libc::c_uint,
            name:
            b"Uuml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter U with diaeresis, U+00DC ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 221 as libc::c_int as libc::c_uint,
            name:
            b"Yacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter Y with acute, U+00DD ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 222 as libc::c_int as libc::c_uint,
            name:
            b"THORN\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter THORN, U+00DE ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 223 as libc::c_int as libc::c_uint,
            name:
            b"szlig\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter sharp s = ess-zed, U+00DF ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 224 as libc::c_int as libc::c_uint,
            name:
            b"agrave\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter a with grave = latin small letter a grave, U+00E0 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 225 as libc::c_int as libc::c_uint,
            name:
            b"aacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter a with acute, U+00E1 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 226 as libc::c_int as libc::c_uint,
            name:
            b"acirc\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter a with circumflex, U+00E2 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 227 as libc::c_int as libc::c_uint,
            name:
            b"atilde\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter a with tilde, U+00E3 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 228 as libc::c_int as libc::c_uint,
            name:
            b"auml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter a with diaeresis, U+00E4 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 229 as libc::c_int as libc::c_uint,
            name:
            b"aring\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter a with ring above = latin small letter a ring, U+00E5 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 230 as libc::c_int as libc::c_uint,
            name:
            b"aelig\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter ae = latin small ligature ae, U+00E6 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 231 as libc::c_int as libc::c_uint,
            name:
            b"ccedil\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter c with cedilla, U+00E7 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 232 as libc::c_int as libc::c_uint,
            name:
            b"egrave\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter e with grave, U+00E8 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 233 as libc::c_int as libc::c_uint,
            name:
            b"eacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter e with acute, U+00E9 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 234 as libc::c_int as libc::c_uint,
            name:
            b"ecirc\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter e with circumflex, U+00EA ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 235 as libc::c_int as libc::c_uint,
            name:
            b"euml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter e with diaeresis, U+00EB ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 236 as libc::c_int as libc::c_uint,
            name:
            b"igrave\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter i with grave, U+00EC ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 237 as libc::c_int as libc::c_uint,
            name:
            b"iacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter i with acute, U+00ED ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 238 as libc::c_int as libc::c_uint,
            name:
            b"icirc\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter i with circumflex, U+00EE ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 239 as libc::c_int as libc::c_uint,
            name:
            b"iuml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter i with diaeresis, U+00EF ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 240 as libc::c_int as libc::c_uint,
            name:
            b"eth\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter eth, U+00F0 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 241 as libc::c_int as libc::c_uint,
            name:
            b"ntilde\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter n with tilde, U+00F1 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 242 as libc::c_int as libc::c_uint,
            name:
            b"ograve\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter o with grave, U+00F2 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 243 as libc::c_int as libc::c_uint,
            name:
            b"oacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter o with acute, U+00F3 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 244 as libc::c_int as libc::c_uint,
            name:
            b"ocirc\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter o with circumflex, U+00F4 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 245 as libc::c_int as libc::c_uint,
            name:
            b"otilde\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter o with tilde, U+00F5 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 246 as libc::c_int as libc::c_uint,
            name:
            b"ouml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter o with diaeresis, U+00F6 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 247 as libc::c_int as libc::c_uint,
            name:
            b"divide\x00" as *const u8 as *const libc::c_char,
            desc:
            b"division sign, U+00F7 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 248 as libc::c_int as libc::c_uint,
            name:
            b"oslash\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter o with stroke, = latin small letter o slash, U+00F8 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 249 as libc::c_int as libc::c_uint,
            name:
            b"ugrave\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter u with grave, U+00F9 ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 250 as libc::c_int as libc::c_uint,
            name:
            b"uacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter u with acute, U+00FA ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 251 as libc::c_int as libc::c_uint,
            name:
            b"ucirc\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter u with circumflex, U+00FB ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 252 as libc::c_int as libc::c_uint,
            name:
            b"uuml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter u with diaeresis, U+00FC ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 253 as libc::c_int as libc::c_uint,
            name:
            b"yacute\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter y with acute, U+00FD ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 254 as libc::c_int as libc::c_uint,
            name:
            b"thorn\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter thorn with, U+00FE ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 255 as libc::c_int as libc::c_uint,
            name:
            b"yuml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter y with diaeresis, U+00FF ISOlat1\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 338 as libc::c_int as libc::c_uint,
            name:
            b"OElig\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital ligature OE, U+0152 ISOlat2\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 339 as libc::c_int as libc::c_uint,
            name:
            b"oelig\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small ligature oe, U+0153 ISOlat2\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 352 as libc::c_int as libc::c_uint,
            name:
            b"Scaron\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter S with caron, U+0160 ISOlat2\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 353 as libc::c_int as libc::c_uint,
            name:
            b"scaron\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small letter s with caron, U+0161 ISOlat2\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 376 as libc::c_int as libc::c_uint,
            name:
            b"Yuml\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin capital letter Y with diaeresis, U+0178 ISOlat2\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 402 as libc::c_int as libc::c_uint,
            name:
            b"fnof\x00" as *const u8 as *const libc::c_char,
            desc:
            b"latin small f with hook = function = florin, U+0192 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 710 as libc::c_int as libc::c_uint,
            name:
            b"circ\x00" as *const u8 as *const libc::c_char,
            desc:
            b"modifier letter circumflex accent, U+02C6 ISOpub\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 732 as libc::c_int as libc::c_uint,
            name:
            b"tilde\x00" as *const u8 as *const libc::c_char,
            desc:
            b"small tilde, U+02DC ISOdia\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 913 as libc::c_int as libc::c_uint,
            name:
            b"Alpha\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter alpha, U+0391\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 914 as libc::c_int as libc::c_uint,
            name:
            b"Beta\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter beta, U+0392\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 915 as libc::c_int as libc::c_uint,
            name:
            b"Gamma\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter gamma, U+0393 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 916 as libc::c_int as libc::c_uint,
            name:
            b"Delta\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter delta, U+0394 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 917 as libc::c_int as libc::c_uint,
            name:
            b"Epsilon\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter epsilon, U+0395\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 918 as libc::c_int as libc::c_uint,
            name:
            b"Zeta\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter zeta, U+0396\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 919 as libc::c_int as libc::c_uint,
            name:
            b"Eta\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter eta, U+0397\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 920 as libc::c_int as libc::c_uint,
            name:
            b"Theta\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter theta, U+0398 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 921 as libc::c_int as libc::c_uint,
            name:
            b"Iota\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter iota, U+0399\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 922 as libc::c_int as libc::c_uint,
            name:
            b"Kappa\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter kappa, U+039A\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 923 as libc::c_int as libc::c_uint,
            name:
            b"Lambda\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter lambda, U+039B ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 924 as libc::c_int as libc::c_uint,
            name:
            b"Mu\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter mu, U+039C\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 925 as libc::c_int as libc::c_uint,
            name:
            b"Nu\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter nu, U+039D\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 926 as libc::c_int as libc::c_uint,
            name:
            b"Xi\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter xi, U+039E ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 927 as libc::c_int as libc::c_uint,
            name:
            b"Omicron\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter omicron, U+039F\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 928 as libc::c_int as libc::c_uint,
            name:
            b"Pi\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter pi, U+03A0 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 929 as libc::c_int as libc::c_uint,
            name:
            b"Rho\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter rho, U+03A1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 931 as libc::c_int as libc::c_uint,
            name:
            b"Sigma\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter sigma, U+03A3 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 932 as libc::c_int as libc::c_uint,
            name:
            b"Tau\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter tau, U+03A4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 933 as libc::c_int as libc::c_uint,
            name:
            b"Upsilon\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter upsilon, U+03A5 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 934 as libc::c_int as libc::c_uint,
            name:
            b"Phi\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter phi, U+03A6 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 935 as libc::c_int as libc::c_uint,
            name:
            b"Chi\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter chi, U+03A7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 936 as libc::c_int as libc::c_uint,
            name:
            b"Psi\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter psi, U+03A8 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 937 as libc::c_int as libc::c_uint,
            name:
            b"Omega\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek capital letter omega, U+03A9 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 945 as libc::c_int as libc::c_uint,
            name:
            b"alpha\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter alpha, U+03B1 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 946 as libc::c_int as libc::c_uint,
            name:
            b"beta\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter beta, U+03B2 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 947 as libc::c_int as libc::c_uint,
            name:
            b"gamma\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter gamma, U+03B3 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 948 as libc::c_int as libc::c_uint,
            name:
            b"delta\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter delta, U+03B4 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 949 as libc::c_int as libc::c_uint,
            name:
            b"epsilon\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter epsilon, U+03B5 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 950 as libc::c_int as libc::c_uint,
            name:
            b"zeta\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter zeta, U+03B6 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 951 as libc::c_int as libc::c_uint,
            name:
            b"eta\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter eta, U+03B7 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 952 as libc::c_int as libc::c_uint,
            name:
            b"theta\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter theta, U+03B8 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 953 as libc::c_int as libc::c_uint,
            name:
            b"iota\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter iota, U+03B9 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 954 as libc::c_int as libc::c_uint,
            name:
            b"kappa\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter kappa, U+03BA ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 955 as libc::c_int as libc::c_uint,
            name:
            b"lambda\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter lambda, U+03BB ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 956 as libc::c_int as libc::c_uint,
            name:
            b"mu\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter mu, U+03BC ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 957 as libc::c_int as libc::c_uint,
            name:
            b"nu\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter nu, U+03BD ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 958 as libc::c_int as libc::c_uint,
            name:
            b"xi\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter xi, U+03BE ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 959 as libc::c_int as libc::c_uint,
            name:
            b"omicron\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter omicron, U+03BF NEW\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 960 as libc::c_int as libc::c_uint,
            name:
            b"pi\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter pi, U+03C0 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 961 as libc::c_int as libc::c_uint,
            name:
            b"rho\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter rho, U+03C1 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 962 as libc::c_int as libc::c_uint,
            name:
            b"sigmaf\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter final sigma, U+03C2 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 963 as libc::c_int as libc::c_uint,
            name:
            b"sigma\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter sigma, U+03C3 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 964 as libc::c_int as libc::c_uint,
            name:
            b"tau\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter tau, U+03C4 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 965 as libc::c_int as libc::c_uint,
            name:
            b"upsilon\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter upsilon, U+03C5 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 966 as libc::c_int as libc::c_uint,
            name:
            b"phi\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter phi, U+03C6 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 967 as libc::c_int as libc::c_uint,
            name:
            b"chi\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter chi, U+03C7 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 968 as libc::c_int as libc::c_uint,
            name:
            b"psi\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter psi, U+03C8 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 969 as libc::c_int as libc::c_uint,
            name:
            b"omega\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter omega, U+03C9 ISOgrk3\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 977 as libc::c_int as libc::c_uint,
            name:
            b"thetasym\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek small letter theta symbol, U+03D1 NEW\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 978 as libc::c_int as libc::c_uint,
            name:
            b"upsih\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek upsilon with hook symbol, U+03D2 NEW\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 982 as libc::c_int as libc::c_uint,
            name:
            b"piv\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greek pi symbol, U+03D6 ISOgrk3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8194 as libc::c_int as libc::c_uint,
            name:
            b"ensp\x00" as *const u8 as *const libc::c_char,
            desc:
            b"en space, U+2002 ISOpub\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8195 as libc::c_int as libc::c_uint,
            name:
            b"emsp\x00" as *const u8 as *const libc::c_char,
            desc:
            b"em space, U+2003 ISOpub\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8201 as libc::c_int as libc::c_uint,
            name:
            b"thinsp\x00" as *const u8 as *const libc::c_char,
            desc:
            b"thin space, U+2009 ISOpub\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8204 as libc::c_int as libc::c_uint,
            name:
            b"zwnj\x00" as *const u8 as *const libc::c_char,
            desc:
            b"zero width non-joiner, U+200C NEW RFC 2070\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8205 as libc::c_int as libc::c_uint,
            name:
            b"zwj\x00" as *const u8 as *const libc::c_char,
            desc:
            b"zero width joiner, U+200D NEW RFC 2070\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8206 as libc::c_int as libc::c_uint,
            name:
            b"lrm\x00" as *const u8 as *const libc::c_char,
            desc:
            b"left-to-right mark, U+200E NEW RFC 2070\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8207 as libc::c_int as libc::c_uint,
            name:
            b"rlm\x00" as *const u8 as *const libc::c_char,
            desc:
            b"right-to-left mark, U+200F NEW RFC 2070\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8211 as libc::c_int as libc::c_uint,
            name:
            b"ndash\x00" as *const u8 as *const libc::c_char,
            desc:
            b"en dash, U+2013 ISOpub\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8212 as libc::c_int as libc::c_uint,
            name:
            b"mdash\x00" as *const u8 as *const libc::c_char,
            desc:
            b"em dash, U+2014 ISOpub\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8216 as libc::c_int as libc::c_uint,
            name:
            b"lsquo\x00" as *const u8 as *const libc::c_char,
            desc:
            b"left single quotation mark, U+2018 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8217 as libc::c_int as libc::c_uint,
            name:
            b"rsquo\x00" as *const u8 as *const libc::c_char,
            desc:
            b"right single quotation mark, U+2019 ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8218 as libc::c_int as libc::c_uint,
            name:
            b"sbquo\x00" as *const u8 as *const libc::c_char,
            desc:
            b"single low-9 quotation mark, U+201A NEW\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8220 as libc::c_int as libc::c_uint,
            name:
            b"ldquo\x00" as *const u8 as *const libc::c_char,
            desc:
            b"left double quotation mark, U+201C ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8221 as libc::c_int as libc::c_uint,
            name:
            b"rdquo\x00" as *const u8 as *const libc::c_char,
            desc:
            b"right double quotation mark, U+201D ISOnum\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8222 as libc::c_int as libc::c_uint,
            name:
            b"bdquo\x00" as *const u8 as *const libc::c_char,
            desc:
            b"double low-9 quotation mark, U+201E NEW\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8224 as libc::c_int as libc::c_uint,
            name:
            b"dagger\x00" as *const u8 as *const libc::c_char,
            desc:
            b"dagger, U+2020 ISOpub\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8225 as libc::c_int as libc::c_uint,
            name:
            b"Dagger\x00" as *const u8 as *const libc::c_char,
            desc:
            b"double dagger, U+2021 ISOpub\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8226 as libc::c_int as libc::c_uint,
            name:
            b"bull\x00" as *const u8 as *const libc::c_char,
            desc:
            b"bullet = black small circle, U+2022 ISOpub\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8230 as libc::c_int as libc::c_uint,
            name:
            b"hellip\x00" as *const u8 as *const libc::c_char,
            desc:
            b"horizontal ellipsis = three dot leader, U+2026 ISOpub\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8240 as libc::c_int as libc::c_uint,
            name:
            b"permil\x00" as *const u8 as *const libc::c_char,
            desc:
            b"per mille sign, U+2030 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8242 as libc::c_int as libc::c_uint,
            name:
            b"prime\x00" as *const u8 as *const libc::c_char,
            desc:
            b"prime = minutes = feet, U+2032 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8243 as libc::c_int as libc::c_uint,
            name:
            b"Prime\x00" as *const u8 as *const libc::c_char,
            desc:
            b"double prime = seconds = inches, U+2033 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8249 as libc::c_int as libc::c_uint,
            name:
            b"lsaquo\x00" as *const u8 as *const libc::c_char,
            desc:
            b"single left-pointing angle quotation mark, U+2039 ISO proposed\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8250 as libc::c_int as libc::c_uint,
            name:
            b"rsaquo\x00" as *const u8 as *const libc::c_char,
            desc:
            b"single right-pointing angle quotation mark, U+203A ISO proposed\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8254 as libc::c_int as libc::c_uint,
            name:
            b"oline\x00" as *const u8 as *const libc::c_char,
            desc:
            b"overline = spacing overscore, U+203E NEW\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8260 as libc::c_int as libc::c_uint,
            name:
            b"frasl\x00" as *const u8 as *const libc::c_char,
            desc:
            b"fraction slash, U+2044 NEW\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8364 as libc::c_int as libc::c_uint,
            name:
            b"euro\x00" as *const u8 as *const libc::c_char,
            desc:
            b"euro sign, U+20AC NEW\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8465 as libc::c_int as libc::c_uint,
            name:
            b"image\x00" as *const u8 as *const libc::c_char,
            desc:
            b"blackletter capital I = imaginary part, U+2111 ISOamso\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8472 as libc::c_int as libc::c_uint,
            name:
            b"weierp\x00" as *const u8 as *const libc::c_char,
            desc:
            b"script capital P = power set = Weierstrass p, U+2118 ISOamso\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8476 as libc::c_int as libc::c_uint,
            name:
            b"real\x00" as *const u8 as *const libc::c_char,
            desc:
            b"blackletter capital R = real part symbol, U+211C ISOamso\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8482 as libc::c_int as libc::c_uint,
            name:
            b"trade\x00" as *const u8 as *const libc::c_char,
            desc:
            b"trade mark sign, U+2122 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8501 as libc::c_int as libc::c_uint,
            name:
            b"alefsym\x00" as *const u8 as *const libc::c_char,
            desc:
            b"alef symbol = first transfinite cardinal, U+2135 NEW\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8592 as libc::c_int as libc::c_uint,
            name:
            b"larr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"leftwards arrow, U+2190 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8593 as libc::c_int as libc::c_uint,
            name:
            b"uarr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"upwards arrow, U+2191 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8594 as libc::c_int as libc::c_uint,
            name:
            b"rarr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"rightwards arrow, U+2192 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8595 as libc::c_int as libc::c_uint,
            name:
            b"darr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"downwards arrow, U+2193 ISOnum\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8596 as libc::c_int as libc::c_uint,
            name:
            b"harr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"left right arrow, U+2194 ISOamsa\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8629 as libc::c_int as libc::c_uint,
            name:
            b"crarr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"downwards arrow with corner leftwards = carriage return, U+21B5 NEW\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8656 as libc::c_int as libc::c_uint,
            name:
            b"lArr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"leftwards double arrow, U+21D0 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8657 as libc::c_int as libc::c_uint,
            name:
            b"uArr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"upwards double arrow, U+21D1 ISOamsa\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8658 as libc::c_int as libc::c_uint,
            name:
            b"rArr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"rightwards double arrow, U+21D2 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8659 as libc::c_int as libc::c_uint,
            name:
            b"dArr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"downwards double arrow, U+21D3 ISOamsa\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8660 as libc::c_int as libc::c_uint,
            name:
            b"hArr\x00" as *const u8 as *const libc::c_char,
            desc:
            b"left right double arrow, U+21D4 ISOamsa\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8704 as libc::c_int as libc::c_uint,
            name:
            b"forall\x00" as *const u8 as *const libc::c_char,
            desc:
            b"for all, U+2200 ISOtech\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8706 as libc::c_int as libc::c_uint,
            name:
            b"part\x00" as *const u8 as *const libc::c_char,
            desc:
            b"partial differential, U+2202 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8707 as libc::c_int as libc::c_uint,
            name:
            b"exist\x00" as *const u8 as *const libc::c_char,
            desc:
            b"there exists, U+2203 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8709 as libc::c_int as libc::c_uint,
            name:
            b"empty\x00" as *const u8 as *const libc::c_char,
            desc:
            b"empty set = null set = diameter, U+2205 ISOamso\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8711 as libc::c_int as libc::c_uint,
            name:
            b"nabla\x00" as *const u8 as *const libc::c_char,
            desc:
            b"nabla = backward difference, U+2207 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8712 as libc::c_int as libc::c_uint,
            name:
            b"isin\x00" as *const u8 as *const libc::c_char,
            desc:
            b"element of, U+2208 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8713 as libc::c_int as libc::c_uint,
            name:
            b"notin\x00" as *const u8 as *const libc::c_char,
            desc:
            b"not an element of, U+2209 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8715 as libc::c_int as libc::c_uint,
            name:
            b"ni\x00" as *const u8 as *const libc::c_char,
            desc:
            b"contains as member, U+220B ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8719 as libc::c_int as libc::c_uint,
            name:
            b"prod\x00" as *const u8 as *const libc::c_char,
            desc:
            b"n-ary product = product sign, U+220F ISOamsb\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8721 as libc::c_int as libc::c_uint,
            name:
            b"sum\x00" as *const u8 as *const libc::c_char,
            desc:
            b"n-ary summation, U+2211 ISOamsb\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8722 as libc::c_int as libc::c_uint,
            name:
            b"minus\x00" as *const u8 as *const libc::c_char,
            desc:
            b"minus sign, U+2212 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8727 as libc::c_int as libc::c_uint,
            name:
            b"lowast\x00" as *const u8 as *const libc::c_char,
            desc:
            b"asterisk operator, U+2217 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8730 as libc::c_int as libc::c_uint,
            name:
            b"radic\x00" as *const u8 as *const libc::c_char,
            desc:
            b"square root = radical sign, U+221A ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8733 as libc::c_int as libc::c_uint,
            name:
            b"prop\x00" as *const u8 as *const libc::c_char,
            desc:
            b"proportional to, U+221D ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8734 as libc::c_int as libc::c_uint,
            name:
            b"infin\x00" as *const u8 as *const libc::c_char,
            desc:
            b"infinity, U+221E ISOtech\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8736 as libc::c_int as libc::c_uint,
            name:
            b"ang\x00" as *const u8 as *const libc::c_char,
            desc:
            b"angle, U+2220 ISOamso\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8743 as libc::c_int as libc::c_uint,
            name:
            b"and\x00" as *const u8 as *const libc::c_char,
            desc:
            b"logical and = wedge, U+2227 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8744 as libc::c_int as libc::c_uint,
            name:
            b"or\x00" as *const u8 as *const libc::c_char,
            desc:
            b"logical or = vee, U+2228 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8745 as libc::c_int as libc::c_uint,
            name:
            b"cap\x00" as *const u8 as *const libc::c_char,
            desc:
            b"intersection = cap, U+2229 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8746 as libc::c_int as libc::c_uint,
            name:
            b"cup\x00" as *const u8 as *const libc::c_char,
            desc:
            b"union = cup, U+222A ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8747 as libc::c_int as libc::c_uint,
            name:
            b"int\x00" as *const u8 as *const libc::c_char,
            desc:
            b"integral, U+222B ISOtech\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8756 as libc::c_int as libc::c_uint,
            name:
            b"there4\x00" as *const u8 as *const libc::c_char,
            desc:
            b"therefore, U+2234 ISOtech\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8764 as libc::c_int as libc::c_uint,
            name:
            b"sim\x00" as *const u8 as *const libc::c_char,
            desc:
            b"tilde operator = varies with = similar to, U+223C ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8773 as libc::c_int as libc::c_uint,
            name:
            b"cong\x00" as *const u8 as *const libc::c_char,
            desc:
            b"approximately equal to, U+2245 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8776 as libc::c_int as libc::c_uint,
            name:
            b"asymp\x00" as *const u8 as *const libc::c_char,
            desc:
            b"almost equal to = asymptotic to, U+2248 ISOamsr\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8800 as libc::c_int as libc::c_uint,
            name:
            b"ne\x00" as *const u8 as *const libc::c_char,
            desc:
            b"not equal to, U+2260 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8801 as libc::c_int as libc::c_uint,
            name:
            b"equiv\x00" as *const u8 as *const libc::c_char,
            desc:
            b"identical to, U+2261 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8804 as libc::c_int as libc::c_uint,
            name:
            b"le\x00" as *const u8 as *const libc::c_char,
            desc:
            b"less-than or equal to, U+2264 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8805 as libc::c_int as libc::c_uint,
            name:
            b"ge\x00" as *const u8 as *const libc::c_char,
            desc:
            b"greater-than or equal to, U+2265 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8834 as libc::c_int as libc::c_uint,
            name:
            b"sub\x00" as *const u8 as *const libc::c_char,
            desc:
            b"subset of, U+2282 ISOtech\x00" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8835 as libc::c_int as libc::c_uint,
            name:
            b"sup\x00" as *const u8 as *const libc::c_char,
            desc:
            b"superset of, U+2283 ISOtech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8836 as libc::c_int as libc::c_uint,
            name:
            b"nsub\x00" as *const u8 as *const libc::c_char,
            desc:
            b"not a subset of, U+2284 ISOamsn\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8838 as libc::c_int as libc::c_uint,
            name:
            b"sube\x00" as *const u8 as *const libc::c_char,
            desc:
            b"subset of or equal to, U+2286 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8839 as libc::c_int as libc::c_uint,
            name:
            b"supe\x00" as *const u8 as *const libc::c_char,
            desc:
            b"superset of or equal to, U+2287 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8853 as libc::c_int as libc::c_uint,
            name:
            b"oplus\x00" as *const u8 as *const libc::c_char,
            desc:
            b"circled plus = direct sum, U+2295 ISOamsb\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8855 as libc::c_int as libc::c_uint,
            name:
            b"otimes\x00" as *const u8 as *const libc::c_char,
            desc:
            b"circled times = vector product, U+2297 ISOamsb\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8869 as libc::c_int as libc::c_uint,
            name:
            b"perp\x00" as *const u8 as *const libc::c_char,
            desc:
            b"up tack = orthogonal to = perpendicular, U+22A5 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8901 as libc::c_int as libc::c_uint,
            name:
            b"sdot\x00" as *const u8 as *const libc::c_char,
            desc:
            b"dot operator, U+22C5 ISOamsb\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8968 as libc::c_int as libc::c_uint,
            name:
            b"lceil\x00" as *const u8 as *const libc::c_char,
            desc:
            b"left ceiling = apl upstile, U+2308 ISOamsc\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8969 as libc::c_int as libc::c_uint,
            name:
            b"rceil\x00" as *const u8 as *const libc::c_char,
            desc:
            b"right ceiling, U+2309 ISOamsc\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8970 as libc::c_int as libc::c_uint,
            name:
            b"lfloor\x00" as *const u8 as *const libc::c_char,
            desc:
            b"left floor = apl downstile, U+230A ISOamsc\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 8971 as libc::c_int as libc::c_uint,
            name:
            b"rfloor\x00" as *const u8 as *const libc::c_char,
            desc:
            b"right floor, U+230B ISOamsc\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 9001 as libc::c_int as libc::c_uint,
            name:
            b"lang\x00" as *const u8 as *const libc::c_char,
            desc:
            b"left-pointing angle bracket = bra, U+2329 ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 9002 as libc::c_int as libc::c_uint,
            name:
            b"rang\x00" as *const u8 as *const libc::c_char,
            desc:
            b"right-pointing angle bracket = ket, U+232A ISOtech\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 9674 as libc::c_int as libc::c_uint,
            name:
            b"loz\x00" as *const u8 as *const libc::c_char,
            desc:
            b"lozenge, U+25CA ISOpub\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 9824 as libc::c_int as libc::c_uint,
            name:
            b"spades\x00" as *const u8 as *const libc::c_char,
            desc:
            b"black spade suit, U+2660 ISOpub\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 9827 as libc::c_int as libc::c_uint,
            name:
            b"clubs\x00" as *const u8 as *const libc::c_char,
            desc:
            b"black club suit = shamrock, U+2663 ISOpub\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 9829 as libc::c_int as libc::c_uint,
            name:
            b"hearts\x00" as *const u8 as *const libc::c_char,
            desc:
            b"black heart suit = valentine, U+2665 ISOpub\x00"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = _htmlEntityDesc {
            value: 9830 as libc::c_int as libc::c_uint,
            name:
            b"diams\x00" as *const u8 as *const libc::c_char,
            desc:
            b"black diamond suit, U+2666 ISOpub\x00" as *const u8 as *const libc::c_char,
        };
        init
    }];
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

pub fn htmlEntityLookup(mut name:
                        *const xmlChar)
                        -> *const htmlEntityDesc {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
        (::std::mem::size_of::<[htmlEntityDesc; 253]>() as libc::c_ulong).wrapping_div(::std::mem::size_of::<htmlEntityDesc>()
            as libc::c_ulong) {
        if xmlStrEqual_safe(name,
                            getHtml40EntitiesTable(i as usize).name as *mut xmlChar) != 0 {
            unsafe {
                return &*html40EntitiesTable.as_ptr().offset(i as isize) as *const htmlEntityDesc as htmlEntityDescPtr as *const htmlEntityDesc;
            }
        }
        i = i.wrapping_add(1)
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

pub fn htmlEntityValueLookup(mut value:
                             libc::c_uint)
                             -> *const htmlEntityDesc {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
        (::std::mem::size_of::<[htmlEntityDesc; 253]>() as libc::c_ulong).wrapping_div(::std::mem::size_of::<htmlEntityDesc>()
            as libc::c_ulong) {
        if getHtml40EntitiesTable(i as usize).value >= value {
            if getHtml40EntitiesTable(i as usize).value > value { break; }
            unsafe {
                return &*html40EntitiesTable.as_ptr().offset(i as isize) as *const htmlEntityDesc as htmlEntityDescPtr as *const htmlEntityDesc;
            }
        } else { i = i.wrapping_add(1) }
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
pub fn UTF8ToHtml(mut out: *mut libc::c_uchar,
                  mut outlen: *mut libc::c_int,
                  mut in_0: *const libc::c_uchar,
                  mut inlen: *mut libc::c_int)
                  -> libc::c_int {
    let mut processed: *const libc::c_uchar = in_0;
    let mut outend: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut outstart: *const libc::c_uchar = out;
    let mut instart: *const libc::c_uchar = in_0;
    let mut inend: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut c: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    let mut trailing: libc::c_int = 0;
    if out.is_null() || outlen.is_null() || inlen.is_null() {
        return -(1 as libc::c_int);
    }
    if in_0.is_null() {
        /*
	 * initialization nothing to do
	 */
        unsafe { *outlen = 0 as libc::c_int; }
        unsafe { *inlen = 0 as libc::c_int; }
        return 0 as libc::c_int;
    }
    unsafe { inend = in_0.offset(*inlen as isize); }
    unsafe { outend = out.offset(*outlen as isize); }
    while in_0 < inend {
        let fresh4 = in_0;
        unsafe { in_0 = in_0.offset(1); }
        unsafe { d = *fresh4 as libc::c_uint; }
        if d < 0x80 as libc::c_int as libc::c_uint {
            c = d;
            trailing = 0 as libc::c_int
        } else if d < 0xc0 as libc::c_int as libc::c_uint {
            /* trailing byte in leading position */
            unsafe {
                *outlen = out.offset_from(outstart) as libc::c_long as libc::c_int;
            }
            unsafe {
                *inlen = processed.offset_from(instart) as libc::c_long as libc::c_int;
            }
            return -(2 as libc::c_int);
        } else {
            if d < 0xe0 as libc::c_int as libc::c_uint {
                c = d & 0x1f as libc::c_int as libc::c_uint;
                trailing = 1 as libc::c_int
            } else if d < 0xf0 as libc::c_int as libc::c_uint {
                c = d & 0xf as libc::c_int as libc::c_uint;
                trailing = 2 as libc::c_int
            } else if d < 0xf8 as libc::c_int as libc::c_uint {
                c = d & 0x7 as libc::c_int as libc::c_uint;
                trailing = 3 as libc::c_int
            } else {
                /* no chance for this in Ascii */
                unsafe {
                    *outlen = out.offset_from(outstart) as libc::c_long as libc::c_int;
                }
                unsafe {
                    *inlen = processed.offset_from(instart) as libc::c_long as libc::c_int;
                }
                return -(2 as libc::c_int);
            }
        }
        unsafe {
            if (inend.offset_from(in_0) as libc::c_long) <
                trailing as libc::c_long {
                break;
            }
        }
        while trailing != 0 {
            if in_0 >= inend || {
                let fresh5 = in_0;
                unsafe { in_0 = in_0.offset(1); }
                unsafe { d = *fresh5 as libc::c_uint; }
                (d & 0xc0 as libc::c_int as libc::c_uint) != 0x80 as libc::c_int as libc::c_uint
            } {
                break;
            }
            c <<= 6 as libc::c_int;
            c |= d & 0x3f as libc::c_int as libc::c_uint;
            trailing -= 1
        }
        /* assertion: c is a single UTF-4 value */
        if c < 0x80 as libc::c_int as libc::c_uint {
            unsafe {
                if out.offset(1 as libc::c_int as isize) >= outend as *mut libc::c_uchar {
                    break;
                }
            }
            let fresh6 = out;
            unsafe { out = out.offset(1); }
            unsafe { *fresh6 = c as libc::c_uchar }
        } else {
            let mut len: libc::c_int = 0;
            let mut ent: *const htmlEntityDesc = 0 as *const htmlEntityDesc;
            let mut cp: *const libc::c_char = 0 as *const libc::c_char;
            let mut nbuf: [libc::c_char; 16] = [0; 16];
            /*
	     * Try to lookup a predefined HTML entity for it
	     */
            ent = htmlEntityValueLookup(c);
            if ent.is_null() {
                snprintf_safe_macro!(nbuf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                             b"#%u\x00" as *const u8 as *const libc::c_char, c);
                cp = nbuf.as_mut_ptr()
            } else { unsafe { cp = (*ent).name } }
            len = strlen_safe(cp) as libc::c_int;
            unsafe {
                if out.offset(2 as libc::c_int as isize).offset(len as isize) >= outend as *mut libc::c_uchar {
                    break;
                }
            }
            let fresh7 = out;
            unsafe { out = out.offset(1); }
            unsafe { *fresh7 = '&' as i32 as libc::c_uchar; }
            memcpy_safe(out as *mut libc::c_void, cp as *const libc::c_void,
                        len as libc::c_ulong);
            unsafe { out = out.offset(len as isize); }
            let fresh8 = out;
            unsafe { out = out.offset(1); }
            unsafe { *fresh8 = ';' as i32 as libc::c_uchar }
        }
        processed = in_0
    }
    unsafe {
        *outlen = out.offset_from(outstart) as libc::c_long as libc::c_int;
    }
    unsafe {
        *inlen = processed.offset_from(instart) as libc::c_long as libc::c_int;
    }
    return 0 as libc::c_int;
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
pub fn htmlEncodeEntities(mut out:
                          *mut libc::c_uchar,
                          mut outlen:
                          *mut libc::c_int,
                          mut in_0:
                          *const libc::c_uchar,
                          mut inlen:
                          *mut libc::c_int,
                          mut quoteChar:
                          libc::c_int)
                          -> libc::c_int {
    let mut processed: *const libc::c_uchar = in_0;
    let mut outend: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut outstart: *const libc::c_uchar = out;
    let mut instart: *const libc::c_uchar = in_0;
    let mut inend: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut c: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    let mut trailing: libc::c_int = 0;
    if out.is_null() || outlen.is_null() || inlen.is_null() || in_0.is_null()
    {
        return -(1 as libc::c_int);
    }
    unsafe { outend = out.offset(*outlen as isize); }
    unsafe { inend = in_0.offset(*inlen as isize); }
    while in_0 < inend {
        let fresh9 = in_0;
        unsafe { in_0 = in_0.offset(1); }
        unsafe { d = *fresh9 as libc::c_uint; }
        if d < 0x80 as libc::c_int as libc::c_uint {
            c = d;
            trailing = 0 as libc::c_int
        } else if d < 0xc0 as libc::c_int as libc::c_uint {
            /* trailing byte in leading position */
            unsafe {
                *outlen = out.offset_from(outstart) as libc::c_long as libc::c_int;
            }
            unsafe {
                *inlen = processed.offset_from(instart) as libc::c_long as libc::c_int;
            }
            return -(2 as libc::c_int);
        } else {
            if d < 0xe0 as libc::c_int as libc::c_uint {
                c = d & 0x1f as libc::c_int as libc::c_uint;
                trailing = 1 as libc::c_int
            } else if d < 0xf0 as libc::c_int as libc::c_uint {
                c = d & 0xf as libc::c_int as libc::c_uint;
                trailing = 2 as libc::c_int
            } else if d < 0xf8 as libc::c_int as libc::c_uint {
                c = d & 0x7 as libc::c_int as libc::c_uint;
                trailing = 3 as libc::c_int
            } else {
                /* no chance for this in Ascii */
                unsafe {
                    *outlen = out.offset_from(outstart) as libc::c_long as libc::c_int;
                }
                unsafe {
                    *inlen = processed.offset_from(instart) as libc::c_long as libc::c_int;
                }
                return -(2 as libc::c_int);
            }
        }
        unsafe {
            if (inend.offset_from(in_0) as libc::c_long) <
                trailing as libc::c_long {
                break;
            }
        }
        loop {
            let fresh10 = trailing;
            trailing = trailing - 1;
            if !(fresh10 != 0) { break; }
            let fresh11 = in_0;
            unsafe { in_0 = in_0.offset(1); }
            unsafe { d = *fresh11 as libc::c_uint; }
            if d & 0xc0 as libc::c_int as libc::c_uint != 0x80 as libc::c_int as libc::c_uint {
                unsafe {
                    *outlen = out.offset_from(outstart) as libc::c_long as libc::c_int;
                }
                unsafe {
                    *inlen = processed.offset_from(instart) as libc::c_long as libc::c_int;
                }
                return -(2 as libc::c_int);
            }
            c <<= 6 as libc::c_int;
            c |= d & 0x3f as libc::c_int as libc::c_uint
        }
        /* assertion: c is a single UTF-4 value */
        if c < 0x80 as libc::c_int as libc::c_uint && c != quoteChar as libc::c_uint && c != '&' as i32 as libc::c_uint && c != '<' as i32 as libc::c_uint && c != '>' as i32 as libc::c_uint {
            if out >= outend as *mut libc::c_uchar { break; }
            let fresh12 = out;
            unsafe { out = out.offset(1); }
            unsafe { *fresh12 = c as libc::c_uchar }
        } else {
            let mut ent: *const htmlEntityDesc = 0 as *const htmlEntityDesc;
            let mut cp: *const libc::c_char = 0 as *const libc::c_char;
            let mut nbuf: [libc::c_char; 16] = [0; 16];
            let mut len: libc::c_int = 0;
            /*
	     * Try to lookup a predefined HTML entity for it
	     */
            ent = htmlEntityValueLookup(c);
            if ent.is_null() {
                snprintf_safe_macro!(nbuf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                             b"#%u\x00" as *const u8 as *const libc::c_char, c);
                cp = nbuf.as_mut_ptr()
            } else { unsafe { cp = (*ent).name } }
            len = strlen_safe(cp) as libc::c_int;
            unsafe {
                if out.offset(2 as libc::c_int as isize).offset(len as isize) >
                    outend as *mut libc::c_uchar {
                    break;
                }
            }
            let fresh13 = out;
            unsafe { out = out.offset(1); }
            unsafe { *fresh13 = '&' as i32 as libc::c_uchar; }
            memcpy_safe(out as *mut libc::c_void, cp as *const libc::c_void,
                        len as libc::c_ulong);
            unsafe { out = out.offset(len as isize); }
            let fresh14 = out;
            unsafe { out = out.offset(1); }
            unsafe { *fresh14 = ';' as i32 as libc::c_uchar }
        }
        processed = in_0
    }
    unsafe {
        *outlen = out.offset_from(outstart) as libc::c_long as libc::c_int;
    }
    unsafe {
        *inlen = processed.offset_from(instart) as libc::c_long as libc::c_int;
    }
    return 0 as libc::c_int;
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
fn htmlNewInputStream(mut ctxt: htmlParserCtxtPtr)
                      -> htmlParserInputPtr {
    let mut input: htmlParserInputPtr = 0 as *mut xmlParserInput;
    input = xmlMalloc_safe(::std::mem::size_of::<htmlParserInput>()
        as libc::c_ulong) as xmlParserInputPtr;
    if input.is_null() {
        htmlErrMemory(ctxt,
                      b"couldn\'t allocate a new input stream\n\x00" as *const u8 as *const libc::c_char);
        return 0 as htmlParserInputPtr;
    }
    memset_safe(input as *mut libc::c_void, 0 as libc::c_int,
                ::std::mem::size_of::<htmlParserInput>() as libc::c_ulong);
    let mut inputPtr = unsafe { &mut *input };
    inputPtr.filename = 0 as *const libc::c_char;
    inputPtr.directory = 0 as *const libc::c_char;
    inputPtr.base = 0 as *const xmlChar;
    inputPtr.cur = 0 as *const xmlChar;
    inputPtr.buf = 0 as xmlParserInputBufferPtr;
    inputPtr.line = 1 as libc::c_int;
    inputPtr.col = 1 as libc::c_int;
    inputPtr.buf = 0 as xmlParserInputBufferPtr;
    inputPtr.free = None;
    inputPtr.version = 0 as *const xmlChar;
    inputPtr.consumed = 0 as libc::c_int as libc::c_ulong;
    inputPtr.length = 0 as libc::c_int;
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
static mut allowPCData: [*const libc::c_char; 53] = [b"a\x00" as *const u8 as *const libc::c_char,
    b"abbr\x00" as *const u8 as *const libc::c_char,
    b"acronym\x00" as *const u8 as *const libc::c_char,
    b"address\x00" as *const u8 as *const libc::c_char,
    b"applet\x00" as *const u8 as *const libc::c_char,
    b"b\x00" as *const u8 as *const libc::c_char,
    b"bdo\x00" as *const u8 as *const libc::c_char,
    b"big\x00" as *const u8 as *const libc::c_char,
    b"blockquote\x00" as *const u8 as *const libc::c_char,
    b"body\x00" as *const u8 as *const libc::c_char,
    b"button\x00" as *const u8 as *const libc::c_char,
    b"caption\x00" as *const u8 as *const libc::c_char,
    b"center\x00" as *const u8 as *const libc::c_char,
    b"cite\x00" as *const u8 as *const libc::c_char,
    b"code\x00" as *const u8 as *const libc::c_char,
    b"dd\x00" as *const u8 as *const libc::c_char,
    b"del\x00" as *const u8 as *const libc::c_char,
    b"dfn\x00" as *const u8 as *const libc::c_char,
    b"div\x00" as *const u8 as *const libc::c_char,
    b"dt\x00" as *const u8 as *const libc::c_char,
    b"em\x00" as *const u8 as *const libc::c_char,
    b"font\x00" as *const u8 as *const libc::c_char,
    b"form\x00" as *const u8 as *const libc::c_char,
    b"h1\x00" as *const u8 as *const libc::c_char,
    b"h2\x00" as *const u8 as *const libc::c_char,
    b"h3\x00" as *const u8 as *const libc::c_char,
    b"h4\x00" as *const u8 as *const libc::c_char,
    b"h5\x00" as *const u8 as *const libc::c_char,
    b"h6\x00" as *const u8 as *const libc::c_char,
    b"i\x00" as *const u8 as *const libc::c_char,
    b"iframe\x00" as *const u8 as *const libc::c_char,
    b"ins\x00" as *const u8 as *const libc::c_char,
    b"kbd\x00" as *const u8 as *const libc::c_char,
    b"label\x00" as *const u8 as *const libc::c_char,
    b"legend\x00" as *const u8 as *const libc::c_char,
    b"li\x00" as *const u8 as *const libc::c_char,
    b"noframes\x00" as *const u8 as *const libc::c_char,
    b"noscript\x00" as *const u8 as *const libc::c_char,
    b"object\x00" as *const u8 as *const libc::c_char,
    b"p\x00" as *const u8 as *const libc::c_char,
    b"pre\x00" as *const u8 as *const libc::c_char,
    b"q\x00" as *const u8 as *const libc::c_char,
    b"s\x00" as *const u8 as *const libc::c_char,
    b"samp\x00" as *const u8 as *const libc::c_char,
    b"small\x00" as *const u8 as *const libc::c_char,
    b"span\x00" as *const u8 as *const libc::c_char,
    b"strike\x00" as *const u8 as *const libc::c_char,
    b"strong\x00" as *const u8 as *const libc::c_char,
    b"td\x00" as *const u8 as *const libc::c_char,
    b"th\x00" as *const u8 as *const libc::c_char,
    b"tt\x00" as *const u8 as *const libc::c_char,
    b"u\x00" as *const u8 as *const libc::c_char,
    b"var\x00" as *const u8 as *const libc::c_char];
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
fn areBlanks_htmlparser(mut ctxt: htmlParserCtxtPtr,
                        mut str: *const xmlChar, mut len: libc::c_int)
                        -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_int = 0;
    let mut lastChild: xmlNodePtr = 0 as *mut xmlNode;
    let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
    j = 0 as libc::c_int;
    while j < len {
        unsafe {
            if !IS_BLANK_CH(*str.offset(j as isize) as libc::c_int) {
                return 0 as libc::c_int;
            }
        }
        j += 1
    }
    if CUR(ctxt) == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if CUR(ctxt) != '<' as i32 {
        return 0 as libc::c_int;
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.name.is_null() { return 1 as libc::c_int; }
    if xmlStrEqual_safe(ctxtPtr.name,
                        b"html\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
        return 1 as libc::c_int;
    }
    if xmlStrEqual_safe(ctxtPtr.name,
                        b"head\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
        return 1 as libc::c_int;
    }
    /* Only strip CDATA children of the body tag for strict HTML DTDs */
    if xmlStrEqual_safe(ctxtPtr.name,
                        b"body\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 && !ctxtPtr.myDoc.is_null() {
        dtd = xmlGetIntSubset_safe(ctxtPtr.myDoc as *const xmlDoc);
        let mut dtd_condition = unsafe { !dtd.is_null() && !(*dtd).ExternalID.is_null() };
        if dtd_condition {
            let mut dtdPtr = unsafe { &mut *dtd };
            if xmlStrcasecmp_safe(dtdPtr.ExternalID,
                                  b"-//W3C//DTD HTML 4.01//EN\x00"
                                      as *const u8 as *const libc::c_char as *mut xmlChar) == 0 || xmlStrcasecmp_safe(dtdPtr.ExternalID,
                                                                                                                      b"-//W3C//DTD HTML 4//EN\x00"
                                                                                                                          as *const u8 as *const libc::c_char as *mut xmlChar) == 0 {
                return 1 as libc::c_int;
            }
        }
    }
    if ctxtPtr.node.is_null() { return 0 as libc::c_int; }
    lastChild = xmlGetLastChild_safe(ctxtPtr.node as *const xmlNode);
    unsafe {
        while !lastChild.is_null() && (*lastChild).type_0 as libc::c_uint == XML_COMMENT_NODE as libc::c_int as libc::c_uint {
            lastChild = (*lastChild).prev
        }
    }
    if lastChild.is_null() {
        let mut nodePtr = unsafe { &mut *(*ctxt).node };
        if nodePtr.type_0 as libc::c_uint != XML_ELEMENT_NODE as libc::c_int as libc::c_uint && !nodePtr.content.is_null() {
            return 0 as libc::c_int;
        }
        /* keep ws in constructs like ...<b> </b>...
	   for all tags "b" allowing PCDATA */
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) <
            (::std::mem::size_of::<[*const libc::c_char; 53]>() as libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                as libc::c_ulong) {
            if xmlStrEqual_safe(ctxtPtr.name,
                                getAllowPCData(i as usize) as *mut xmlChar) != 0 {
                return 0 as libc::c_int;
            }
            i = i.wrapping_add(1)
        }
    } else if xmlNodeIsText_safe(lastChild as *const xmlNode) != 0
    {
        return 0 as libc::c_int;
    } else {
        /* keep ws in constructs like <p><b>xy</b> <i>z</i><p>
	   for all tags "p" allowing PCDATA */
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) <
            (::std::mem::size_of::<[*const libc::c_char; 53]>() as libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                as libc::c_ulong) {
            unsafe {
                if xmlStrEqual_safe((*lastChild).name,
                                    getAllowPCData(i as usize) as *mut xmlChar) != 0 {
                    return 0 as libc::c_int;
                }
            }
            i = i.wrapping_add(1)
        }
    }
    return 1 as libc::c_int;
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

pub fn htmlNewDocNoDtD(mut URI:
                       *const xmlChar,
                       mut ExternalID:
                       *const xmlChar)
                       -> htmlDocPtr {
    let mut cur: xmlDocPtr = 0 as *mut xmlDoc;
    /*
     * Allocate a new document and fill the fields.
     */
    cur = xmlMalloc_safe(::std::mem::size_of::<xmlDoc>()
        as libc::c_ulong) as xmlDocPtr;
    if cur.is_null() {
        htmlErrMemory(0 as xmlParserCtxtPtr,
                      b"HTML document creation failed\n\x00" as *const u8 as *const libc::c_char);
        return 0 as htmlDocPtr;
    }
    memset_safe(cur as *mut libc::c_void, 0 as libc::c_int,
                ::std::mem::size_of::<xmlDoc>() as libc::c_ulong);
    let mut curPtr = unsafe { &mut *cur };
    curPtr.type_0 = XML_HTML_DOCUMENT_NODE;
    curPtr.version = 0 as *const xmlChar;
    curPtr.intSubset = 0 as *mut _xmlDtd;
    curPtr.doc = cur;
    curPtr.name = 0 as *mut libc::c_char;
    curPtr.children = 0 as *mut _xmlNode;
    curPtr.extSubset = 0 as *mut _xmlDtd;
    curPtr.oldNs = 0 as *mut _xmlNs;
    curPtr.encoding = 0 as *const xmlChar;
    curPtr.standalone = 1 as libc::c_int;
    curPtr.compression = 0 as libc::c_int;
    curPtr.ids = 0 as *mut libc::c_void;
    curPtr.refs = 0 as *mut libc::c_void;
    curPtr._private = 0 as *mut libc::c_void;
    curPtr.charset = XML_CHAR_ENCODING_UTF8 as libc::c_int;
    curPtr.properties = XML_DOC_HTML as libc::c_int | XML_DOC_USERBUILT as libc::c_int;
    if !ExternalID.is_null() || !URI.is_null() {
        xmlCreateIntSubset_safe(cur,
                                b"html\x00" as *const u8 as *const libc::c_char as *mut xmlChar, ExternalID, URI);
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

pub fn htmlNewDoc(mut URI: *const xmlChar,
                  mut ExternalID:
                  *const xmlChar)
                  -> htmlDocPtr {
    if URI.is_null() && ExternalID.is_null() {
        return htmlNewDocNoDtD(b"http://www.w3.org/TR/REC-html40/loose.dtd\x00"
                                   as *const u8 as *const libc::c_char as *mut xmlChar,
                               b"-//W3C//DTD HTML 4.0 Transitional//EN\x00"
                                   as *const u8 as *const libc::c_char as *mut xmlChar);
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
fn htmlParseHTMLName(mut ctxt: htmlParserCtxtPtr)
                     -> *const xmlChar {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut loc: [xmlChar; 100] = [0; 100];
    if !IS_ASCII_LETTER(CUR(ctxt)) && CUR(ctxt) != '_' as i32 && CUR(ctxt) != ':' as i32 && CUR(ctxt) != '.' as i32 {
        return 0 as *const xmlChar;
    }
    while i < 100 as libc::c_int && (IS_ASCII_LETTER(CUR(ctxt)) || IS_ASCII_DIGIT(CUR(ctxt)) || CUR(ctxt) == ':' as i32 || CUR(ctxt) == '-' as i32 || CUR(ctxt) == '_' as i32 || CUR(ctxt) == '.' as i32) {
        if CUR(ctxt) >= 'A' as i32 && CUR(ctxt) <= 'Z' as i32 {
            loc[i as usize] = (CUR(ctxt) + 0x20 as libc::c_int)
                as xmlChar
        } else {
            loc[i as usize] = CUR(ctxt) as xmlChar
        }
        i += 1;
        xmlNextChar_safe(ctxt);
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    return xmlDictLookup_safe(ctxtPtr.dict, loc.as_mut_ptr(), i);
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
fn htmlParseHTMLName_nonInvasive(mut ctxt:
                                 htmlParserCtxtPtr)
                                 -> *const xmlChar {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut loc: [xmlChar; 100] = [0; 100];
    if !(IS_ASCII_LETTER(NXT(ctxt, 1 as libc::c_int))) && NXT(ctxt, 1 as libc::c_int) != '_' as i32 && NXT(ctxt, 1 as libc::c_int) != ':' as i32 {
        return 0 as *const xmlChar;
    }
    while i < 100 as libc::c_int && (IS_ASCII_LETTER(NXT(ctxt, 1 as libc::c_int + i)) || IS_ASCII_DIGIT(NXT(ctxt, 1 as libc::c_int + i)) || NXT(ctxt, 1 as libc::c_int + i) == ':' as i32 || NXT(ctxt, 1 as libc::c_int + i) == '-' as i32 || NXT(ctxt, 1 as libc::c_int + i) == '_' as i32) {
        if NXT(ctxt, 1 as libc::c_int + i) >= 'A' as i32 && NXT(ctxt, 1 as libc::c_int + i) <= 'Z' as i32 {
            loc[i as usize] = (NXT(ctxt, 1 as libc::c_int + i) + 0x20 as libc::c_int) as xmlChar
        } else {
            loc[i as usize] = NXT(ctxt, 1 as libc::c_int + i) as xmlChar
        }
        i += 1
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    return xmlDictLookup_safe(ctxtPtr.dict, loc.as_mut_ptr(), i);
}
/* *
 * htmlParseName:
 * @ctxt:  an HTML parser context
 *
 * parse an HTML name, this routine is case sensitive.
 *
 * Returns the Name parsed or NULL
 */
fn htmlParseName(mut ctxt: htmlParserCtxtPtr)
                 -> *const xmlChar {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    let mut count: libc::c_int = 0 as libc::c_int;
    GROW(ctxt);
    /*
     * Accelerator for simple ASCII names
     */
    in_0 = inputPtr.cur;
    let mut in_0_safe = unsafe { *in_0 };
    if in_0_safe as libc::c_int >= 0x61 as libc::c_int && in_0_safe as libc::c_int <= 0x7a as libc::c_int || in_0_safe as libc::c_int >= 0x41 as libc::c_int && in_0_safe as libc::c_int <= 0x5a as libc::c_int || in_0_safe as libc::c_int == '_' as i32 || in_0_safe as libc::c_int == ':' as i32 {
        unsafe { in_0 = in_0.offset(1); }
        in_0_safe = unsafe { *in_0 };
        while in_0_safe as libc::c_int >= 0x61 as libc::c_int && in_0_safe as libc::c_int <= 0x7a as libc::c_int || in_0_safe as libc::c_int >= 0x41 as libc::c_int && in_0_safe as libc::c_int <= 0x5a as libc::c_int || in_0_safe as libc::c_int >= 0x30 as libc::c_int && in_0_safe as libc::c_int <= 0x39 as libc::c_int || in_0_safe as libc::c_int == '_' as i32 || in_0_safe as libc::c_int == '-' as i32 || in_0_safe as libc::c_int == ':' as i32 || in_0_safe as libc::c_int == '.' as i32 {
            unsafe { in_0 = in_0.offset(1) }
            in_0_safe = unsafe { *in_0 };
        }
        if in_0 == inputPtr.end { return 0 as *const xmlChar; }
        if in_0_safe as libc::c_int > 0 as libc::c_int && (in_0_safe as libc::c_int) < 0x80 as libc::c_int {
            unsafe {
                count = in_0.offset_from(inputPtr.cur) as libc::c_long as libc::c_int;
            }
            ret = xmlDictLookup_safe(ctxtPtr.dict,
                                     inputPtr.cur, count);
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
fn htmlParseNameComplex(mut ctxt: xmlParserCtxtPtr)
                        -> *const xmlChar {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut base: *const xmlChar = inputPtr.base;
    /*
     * Handler for more complex cases
     */
    GROW(ctxt);
    c = htmlCurrentChar(ctxt, &mut l);
    if c == ' ' as i32 || c == '>' as i32 || c == '/' as i32 || (!IS_LETTER(c, getXmlIsBaseCharGroup())
        && c != '_' as i32 && c != ':' as i32) {
        return 0 as *const xmlChar;
    }
    while c != ' ' as i32 && c != '>' as i32 && c != '/' as i32 && (IS_LETTER(c, getXmlIsBaseCharGroup()) || IS_DIGIT(c, getXmlIsDigitGroup()) || c == '.' as i32 || c == '-' as i32 || c == '_' as i32 || c == ':' as i32 || IS_COMBINING(c, getXmlIsCombiningGroup()) || IS_EXTENDER(c, getXmlIsExtenderGroup())) {
        let fresh15 = count;
        count = count + 1;
        if fresh15 > 100 as libc::c_int {
            count = 0 as libc::c_int;
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
    unsafe {
        if (inputPtr.cur.offset_from(inputPtr.base) as libc::c_long) < len as libc::c_long {
            /* Sanity check */
            htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                         b"unexpected change of input buffer\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
            return 0 as *const xmlChar;
        }
    }
    unsafe {
        return xmlDictLookup_safe(ctxtPtr.dict,
                                  inputPtr.cur.offset(-(len as isize)),
                                  len);
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
fn htmlParseHTMLAttribute(mut ctxt: htmlParserCtxtPtr,
                          stop: xmlChar) -> *mut xmlChar {
    let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
    let mut buffer_size: libc::c_int = 0 as libc::c_int;
    let mut out: *mut xmlChar = 0 as *mut xmlChar;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut ent: *const htmlEntityDesc = 0 as *const htmlEntityDesc;
    /*
     * allocate a translation buffer.
     */
    buffer_size = 100 as libc::c_int;
    buffer = xmlMallocAtomic_safe((buffer_size as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
        as libc::c_ulong))
        as *mut xmlChar;
    if buffer.is_null() {
        htmlErrMemory(ctxt,
                      b"buffer allocation failed\n\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut xmlChar;
    }
    out = buffer;
    /*
     * Ok loop until we reach one of the ending chars
     */
    while CUR(ctxt) != 0 as libc::c_int && CUR(ctxt) != stop as libc::c_int {
        if stop as libc::c_int == 0 as libc::c_int && CUR(ctxt) == '>' as i32 {
            break;
        }
        if stop as libc::c_int == 0 as libc::c_int && IS_BLANK_CH(CUR(ctxt)) {
            break;
        }
        unsafe {
            if CUR(ctxt) == '&' as i32 {
                if NXT(ctxt, 1 as libc::c_int) == '#' as i32 {
                    let mut c: libc::c_uint = 0;
                    let mut bits: libc::c_int = 0;
                    c = htmlParseCharRef(ctxt) as libc::c_uint;
                    if c < 0x80 as libc::c_int as libc::c_uint {
                        let fresh16 = out;
                        unsafe { out = out.offset(1); }
                        unsafe { *fresh16 = c as xmlChar; }
                        bits = -(6 as libc::c_int)
                    } else if c < 0x800 as libc::c_int as libc::c_uint {
                        let fresh17 = out;
                        unsafe { out = out.offset(1); }
                        unsafe {
                            *fresh17 = (c >> 6 as libc::c_int &
                                0x1f as libc::c_int as libc::c_uint |
                                0xc0 as libc::c_int as libc::c_uint) as xmlChar;
                        }
                        bits = 0 as libc::c_int
                    } else if c < 0x10000 as libc::c_int as libc::c_uint {
                        let fresh18 = out;
                        unsafe { out = out.offset(1); }
                        unsafe {
                            *fresh18 = (c >> 12 as libc::c_int &
                                0xf as libc::c_int as libc::c_uint |
                                0xe0 as libc::c_int as libc::c_uint) as xmlChar;
                        }
                        bits = 6 as libc::c_int
                    } else {
                        let fresh19 = out;
                        unsafe { out = out.offset(1); }
                        unsafe {
                            *fresh19 = (c >> 18 as libc::c_int &
                                0x7 as libc::c_int as libc::c_uint |
                                0xf0 as libc::c_int as libc::c_uint) as xmlChar;
                        }
                        bits = 12 as libc::c_int
                    }
                    while bits >= 0 as libc::c_int {
                        let fresh20 = out;
                        unsafe { out = out.offset(1); }
                        unsafe {
                            *fresh20 = (c >> bits & 0x3f as libc::c_int as libc::c_uint |
                                0x80 as libc::c_int as libc::c_uint) as xmlChar;
                        }
                        bits -= 6 as libc::c_int
                    }
                    if out.offset_from(buffer) as libc::c_long >
                        (buffer_size - 100 as libc::c_int) as libc::c_long {
                        let mut indx: libc::c_int = out.offset_from(buffer) as libc::c_long as libc::c_int;
                        let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                        buffer_size *= 2 as libc::c_int;
                        tmp = xmlRealloc_safe(buffer
                                                  as *mut libc::c_void,
                                              (buffer_size
                                                  as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                  as libc::c_ulong))
                            as *mut xmlChar;
                        if tmp.is_null() {
                            htmlErrMemory(ctxt,
                                          b"growing buffer\n\x00" as *const u8 as *const libc::c_char);
                            xmlFree_safe(buffer as *mut libc::c_void);
                            return 0 as *mut xmlChar;
                        }
                        buffer = tmp;
                        out = &mut *buffer.offset(indx as isize) as *mut xmlChar
                    }
                } else {
                    ent = htmlParseEntityRef(ctxt, &mut name);
                    if name.is_null() {
                        let fresh21 = out;
                        out = out.offset(1);
                        *fresh21 = '&' as i32 as xmlChar;
                        if out.offset_from(buffer) as libc::c_long >
                            (buffer_size - 100 as libc::c_int) as libc::c_long
                        {
                            let mut indx_0: libc::c_int = out.offset_from(buffer) as libc::c_long
                                as libc::c_int;
                            let mut tmp_0: *mut xmlChar = 0 as *mut xmlChar;
                            buffer_size *= 2 as libc::c_int;
                            tmp_0 = xmlRealloc_safe(buffer
                                                        as *mut libc::c_void,
                                                    (buffer_size
                                                        as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                        as libc::c_ulong))
                                as *mut xmlChar;
                            if tmp_0.is_null() {
                                htmlErrMemory(ctxt,
                                              b"growing buffer\n\x00" as *const u8
                                                  as *const libc::c_char);
                                xmlFree_safe(buffer
                                    as *mut libc::c_void);
                                return 0 as *mut xmlChar;
                            }
                            buffer = tmp_0;
                            out = &mut *buffer.offset(indx_0 as isize) as *mut xmlChar
                        }
                    } else if ent.is_null() {
                        let fresh22 = out;
                        out = out.offset(1);
                        *fresh22 = '&' as i32 as xmlChar;
                        cur = name;
                        while *cur as libc::c_int != 0 as libc::c_int {
                            if out.offset_from(buffer) as libc::c_long >
                                (buffer_size - 100 as libc::c_int) as libc::c_long {
                                let mut indx_1: libc::c_int = out.offset_from(buffer) as libc::c_long as libc::c_int;
                                let mut tmp_1: *mut xmlChar = 0 as *mut xmlChar;
                                buffer_size *= 2 as libc::c_int;
                                tmp_1 = xmlRealloc_safe(buffer
                                                            as *mut libc::c_void,
                                                        (buffer_size
                                                            as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                            as libc::c_ulong))
                                    as *mut xmlChar;
                                if tmp_1.is_null() {
                                    htmlErrMemory(ctxt,
                                                  b"growing buffer\n\x00" as *const u8 as *const libc::c_char);
                                    xmlFree_safe(buffer
                                        as *mut libc::c_void);
                                    return 0 as *mut xmlChar;
                                }
                                buffer = tmp_1;
                                out = &mut *buffer.offset(indx_1 as isize) as *mut xmlChar
                            }
                            let fresh23 = cur;
                            cur = cur.offset(1);
                            let fresh24 = out;
                            out = out.offset(1);
                            *fresh24 = *fresh23
                        }
                    } else {
                        let mut c_0: libc::c_uint = 0;
                        let mut bits_0: libc::c_int = 0;
                        if out.offset_from(buffer) as libc::c_long >
                            (buffer_size - 100 as libc::c_int) as libc::c_long
                        {
                            let mut indx_2: libc::c_int = out.offset_from(buffer) as libc::c_long
                                as libc::c_int;
                            let mut tmp_2: *mut xmlChar = 0 as *mut xmlChar;
                            buffer_size *= 2 as libc::c_int;
                            tmp_2 = xmlRealloc_safe(buffer
                                                        as *mut libc::c_void,
                                                    (buffer_size
                                                        as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                        as libc::c_ulong))
                                as *mut xmlChar;
                            if tmp_2.is_null() {
                                htmlErrMemory(ctxt,
                                              b"growing buffer\n\x00" as *const u8
                                                  as *const libc::c_char);
                                xmlFree_safe(buffer
                                    as *mut libc::c_void);
                                return 0 as *mut xmlChar;
                            }
                            buffer = tmp_2;
                            out = &mut *buffer.offset(indx_2 as isize) as *mut xmlChar
                        }
                        c_0 = (*ent).value;
                        if c_0 < 0x80 as libc::c_int as libc::c_uint {
                            let fresh25 = out;
                            out = out.offset(1);
                            *fresh25 = c_0 as xmlChar;
                            bits_0 = -(6 as libc::c_int)
                        } else if c_0 < 0x800 as libc::c_int as libc::c_uint {
                            let fresh26 = out;
                            out = out.offset(1);
                            *fresh26 = (c_0 >> 6 as libc::c_int &
                                0x1f as libc::c_int as libc::c_uint |
                                0xc0 as libc::c_int as libc::c_uint) as xmlChar;
                            bits_0 = 0 as libc::c_int
                        } else if c_0 < 0x10000 as libc::c_int as libc::c_uint {
                            let fresh27 = out;
                            out = out.offset(1);
                            *fresh27 = (c_0 >> 12 as libc::c_int &
                                0xf as libc::c_int as libc::c_uint |
                                0xe0 as libc::c_int as libc::c_uint) as xmlChar;
                            bits_0 = 6 as libc::c_int
                        } else {
                            let fresh28 = out;
                            out = out.offset(1);
                            *fresh28 = (c_0 >> 18 as libc::c_int &
                                0x7 as libc::c_int as libc::c_uint |
                                0xf0 as libc::c_int as libc::c_uint) as xmlChar;
                            bits_0 = 12 as libc::c_int
                        }
                        while bits_0 >= 0 as libc::c_int {
                            let fresh29 = out;
                            out = out.offset(1);
                            *fresh29 = (c_0 >> bits_0 &
                                0x3f as libc::c_int as libc::c_uint |
                                0x80 as libc::c_int as libc::c_uint) as xmlChar;
                            bits_0 -= 6 as libc::c_int
                        }
                    }
                }
            } else {
                let mut c_1: libc::c_uint = 0;
                let mut bits_1: libc::c_int = 0;
                let mut l: libc::c_int = 0;
                if out.offset_from(buffer) as libc::c_long >
                    (buffer_size - 100 as libc::c_int) as libc::c_long {
                    let mut indx_3: libc::c_int = out.offset_from(buffer) as libc::c_long as libc::c_int;
                    let mut tmp_3: *mut xmlChar = 0 as *mut xmlChar;
                    buffer_size *= 2 as libc::c_int;
                    tmp_3 = xmlRealloc_safe(buffer as *mut libc::c_void,
                                            (buffer_size
                                                as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                as libc::c_ulong))
                        as *mut xmlChar;
                    if tmp_3.is_null() {
                        htmlErrMemory(ctxt,
                                      b"growing buffer\n\x00" as *const u8 as *const libc::c_char);
                        xmlFree_safe(buffer as *mut libc::c_void);
                        return 0 as *mut xmlChar;
                    }
                    buffer = tmp_3;
                    out = &mut *buffer.offset(indx_3 as isize) as *mut xmlChar
                }
                c_1 = htmlCurrentChar(ctxt, &mut l) as libc::c_uint;
                if c_1 < 0x80 as libc::c_int as libc::c_uint {
                    let fresh30 = out;
                    out = out.offset(1);
                    *fresh30 = c_1 as xmlChar;
                    bits_1 = -(6 as libc::c_int)
                } else if c_1 < 0x800 as libc::c_int as libc::c_uint {
                    let fresh31 = out;
                    out = out.offset(1);
                    *fresh31 = (c_1 >> 6 as libc::c_int &
                        0x1f as libc::c_int as libc::c_uint |
                        0xc0 as libc::c_int as libc::c_uint) as xmlChar;
                    bits_1 = 0 as libc::c_int
                } else if c_1 < 0x10000 as libc::c_int as libc::c_uint {
                    let fresh32 = out;
                    out = out.offset(1);
                    *fresh32 = (c_1 >> 12 as libc::c_int &
                        0xf as libc::c_int as libc::c_uint |
                        0xe0 as libc::c_int as libc::c_uint) as xmlChar;
                    bits_1 = 6 as libc::c_int
                } else {
                    let fresh33 = out;
                    out = out.offset(1);
                    *fresh33 = (c_1 >> 18 as libc::c_int &
                        0x7 as libc::c_int as libc::c_uint |
                        0xf0 as libc::c_int as libc::c_uint) as xmlChar;
                    bits_1 = 12 as libc::c_int
                }
                while bits_1 >= 0 as libc::c_int {
                    let fresh34 = out;
                    out = out.offset(1);
                    *fresh34 = (c_1 >> bits_1 & 0x3f as libc::c_int as libc::c_uint |
                        0x80 as libc::c_int as libc::c_uint) as xmlChar;
                    bits_1 -= 6 as libc::c_int
                }
                xmlNextChar_safe(ctxt);
            }
        }
    }
    unsafe { *out = 0 as libc::c_int as xmlChar; }
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

pub fn htmlParseEntityRef(mut ctxt:
                          htmlParserCtxtPtr,
                          mut str:
                          *mut *const xmlChar)
                          -> *const htmlEntityDesc {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut ent: *const htmlEntityDesc = 0 as *const htmlEntityDesc;
    if !str.is_null() { unsafe { *str = 0 as *const xmlChar } }
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            return 0 as *const htmlEntityDesc;
        }
    }
    if CUR(ctxt) == '&' as i32 {
        xmlNextChar_safe(ctxt);
        name = htmlParseName(ctxt);
        if name.is_null() {
            htmlParseErr(ctxt, XML_ERR_NAME_REQUIRED,
                         b"htmlParseEntityRef: no name\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
        } else {
            GROW(ctxt);
            if CUR(ctxt) == ';' as i32 {
                if !str.is_null() { unsafe { *str = name } }
                /*
		 * Lookup the entity in the table.
		 */
                ent = htmlEntityLookup(name);
                if !ent.is_null() {
                    /* OK that's ugly !!! */
                    xmlNextChar_safe(ctxt);
                }
            } else {
                htmlParseErr(ctxt, XML_ERR_ENTITYREF_SEMICOL_MISSING,
                             b"htmlParseEntityRef: expecting \';\'\n\x00" as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                if !str.is_null() { unsafe { *str = name } }
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
fn htmlParseAttValue(mut ctxt: htmlParserCtxtPtr)
                     -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if CUR(ctxt) == '\"' as i32 {
        xmlNextChar_safe(ctxt);
        ret = htmlParseHTMLAttribute(ctxt, '\"' as i32 as xmlChar);
        if CUR(ctxt) != '\"' as i32 {
            htmlParseErr(ctxt, XML_ERR_ATTRIBUTE_NOT_FINISHED,
                         b"AttValue: \" expected\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
        } else { xmlNextChar_safe(ctxt); }
    } else if CUR(ctxt) == '\'' as i32 {
        xmlNextChar_safe(ctxt);
        ret = htmlParseHTMLAttribute(ctxt, '\'' as i32 as xmlChar);
        if CUR(ctxt) != '\'' as i32 {
            htmlParseErr(ctxt, XML_ERR_ATTRIBUTE_NOT_FINISHED,
                         b"AttValue: \' expected\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
        } else { xmlNextChar_safe(ctxt); }
    } else {
        /*
	 * That's an HTMLism, the attribute value may not be quoted
	 */
        ret = htmlParseHTMLAttribute(ctxt, 0 as libc::c_int as xmlChar);
        if ret.is_null() {
            htmlParseErr(ctxt, XML_ERR_ATTRIBUTE_WITHOUT_VALUE,
                         b"AttValue: no value found\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
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
fn htmlParseSystemLiteral(mut ctxt: htmlParserCtxtPtr)
                          -> *mut xmlChar {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut startPosition: size_t = 0 as libc::c_int as size_t;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut quote: libc::c_int = 0;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if CUR(ctxt) != '\"' as i32 && CUR(ctxt) != '\'' as i32 {
        htmlParseErr(ctxt, XML_ERR_LITERAL_NOT_STARTED,
                     b"SystemLiteral \" or \' expected\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                     0 as *const xmlChar);
        return 0 as *mut xmlChar;
    }
    quote = CUR(ctxt);
    xmlNextChar_safe(ctxt);
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if inputPtr.cur < inputPtr.base { return ret; }
    unsafe {
        startPosition = inputPtr.cur.offset_from(inputPtr.base) as libc::c_long as size_t;
    }
    while CUR(ctxt) != 0 as libc::c_int && CUR(ctxt) != quote {
        /* TODO: Handle UTF-8 */
        if !IS_CHAR_CH(CUR(ctxt)) {
            htmlParseErrInt(ctxt, XML_ERR_INVALID_CHAR,
                            b"Invalid char in SystemLiteral 0x%X\n\x00" as *const u8 as *const libc::c_char,
                            CUR(ctxt));
            err = 1 as libc::c_int
        }
        xmlNextChar_safe(ctxt);
        len = len.wrapping_add(1)
    }
    if CUR(ctxt) != quote {
        htmlParseErr(ctxt, XML_ERR_LITERAL_NOT_FINISHED,
                     b"Unfinished SystemLiteral\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                     0 as *const xmlChar);
    } else {
        xmlNextChar_safe(ctxt);
        if err == 0 as libc::c_int {
            unsafe {
                ret = xmlStrndup_safe(inputPtr.base.offset(startPosition
                    as isize),
                                      len as libc::c_int)
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
fn htmlParsePubidLiteral(mut ctxt: htmlParserCtxtPtr)
                         -> *mut xmlChar {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut startPosition: size_t = 0 as libc::c_int as size_t;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut quote: libc::c_int = 0;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if CUR(ctxt) != '\"' as i32 && CUR(ctxt) != '\'' as i32 {
        htmlParseErr(ctxt, XML_ERR_LITERAL_NOT_STARTED,
                     b"PubidLiteral \" or \' expected\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                     0 as *const xmlChar);
        return 0 as *mut xmlChar;
    }
    quote = CUR(ctxt);
    xmlNextChar_safe(ctxt);
    /*
     * Name ::= (Letter | '_') (NameChar)*
     */
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if inputPtr.cur < inputPtr.base { return ret; }
    unsafe {
        startPosition = inputPtr.cur.offset_from(inputPtr.base) as libc::c_long as size_t;
    }
    while CUR(ctxt) != 0 as libc::c_int && CUR(ctxt) != quote {
        if getXmlIsPubidChar_tab(CUR(ctxt) as usize) == 0 {
            htmlParseErrInt(ctxt, XML_ERR_INVALID_CHAR,
                            b"Invalid char in PubidLiteral 0x%X\n\x00" as *const u8 as *const libc::c_char,
                            CUR(ctxt));
            err = 1 as libc::c_int
        }
        len = len.wrapping_add(1);
        xmlNextChar_safe(ctxt);
    }
    if CUR(ctxt) != '\"' as i32 {
        htmlParseErr(ctxt, XML_ERR_LITERAL_NOT_FINISHED,
                     b"Unfinished PubidLiteral\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                     0 as *const xmlChar);
    } else {
        xmlNextChar_safe(ctxt);
        if err == 0 as libc::c_int {
            unsafe {
                ret = xmlStrndup_safe(inputPtr.base.offset(startPosition
                    as isize),
                                      len as libc::c_int)
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
fn htmlParseScript(mut ctxt: htmlParserCtxtPtr) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    const BUF_SIZE: usize = (HTML_PARSER_BIG_BUFFER_SIZE + 5) as usize;
    let mut buf: [xmlChar; BUF_SIZE] = [0; BUF_SIZE];
    let mut nbchar: libc::c_int = 0 as libc::c_int;
    let mut cur: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    SHRINK(ctxt);
    cur = htmlCurrentChar(ctxt, &mut l);
    while cur != 0 as libc::c_int {
        if cur == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '/' as i32 {
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
                    let mut inputPtr = unsafe { &mut *(*ctxt).input };
                    if xmlStrncasecmp_safe(ctxtPtr.name,
                                           inputPtr.cur.offset(2
                                               as libc::c_int
                                               as isize),
                                           xmlStrlen_safe(ctxtPtr.name))
                        == 0 as libc::c_int {
                        break;
                        /* while */
                    } else {
                        htmlParseErr(ctxt, XML_ERR_TAG_NAME_MISMATCH,
                                     b"Element %s embeds close tag\n\x00" as *const u8 as *const libc::c_char,
                                     ctxtPtr.name, 0 as *const xmlChar);
                    }
                }
            } else if NXT(ctxt, 2 as libc::c_int) >= 'A' as i32 && NXT(ctxt, 2 as libc::c_int) <= 'Z' as i32 || NXT(ctxt, 2 as libc::c_int) >= 'a' as i32 && NXT(ctxt, 2 as libc::c_int) <= 'z' as i32 {
                break;
            }
        }
        unsafe {
            if IS_CHAR(cur) {
                let mut returnI: libc::c_int = COPY_BUF(l, &mut *buf.as_mut_ptr(), nbchar, cur);
                nbchar = returnI;
            } else {
                htmlParseErrInt(ctxt, XML_ERR_INVALID_CHAR,
                                b"Invalid char in CDATA 0x%X\n\x00" as *const u8 as *const libc::c_char, cur);
            }
        }
        if nbchar >= HTML_PARSER_BIG_BUFFER_SIZE {
            buf[nbchar as usize] = 0 as libc::c_int as xmlChar;
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            if saxPtr.cdataBlock.is_some() {
                /*
		 * Insert as CDATA, which is the same as HTML_PRESERVE_NODE
		 */
                xmlSAXHandler_cdataBlock_safe(saxPtr.cdataBlock, ctxtPtr.userData,
                                              buf.as_mut_ptr(),
                                              nbchar);
            } else if saxPtr.characters.is_some() {
                xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                              buf.as_mut_ptr(),
                                              nbchar);
            }
            nbchar = 0 as libc::c_int
        }
        GROW(ctxt);
        NEXTL(ctxt, l);
        cur = htmlCurrentChar(ctxt, &mut l)
    }
    if nbchar != 0 as libc::c_int && !ctxtPtr.sax.is_null() && ctxtPtr.disableSAX == 0 {
        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
        buf[nbchar as usize] = 0 as libc::c_int as xmlChar;
        if saxPtr.cdataBlock.is_some() {
            /*
	     * Insert as CDATA, which is the same as HTML_PRESERVE_NODE
	     */
            xmlSAXHandler_cdataBlock_safe(saxPtr.cdataBlock, ctxtPtr.userData,
                                          buf.as_mut_ptr(),
                                          nbchar);
        } else if saxPtr.characters.is_some() {
            xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                          buf.as_mut_ptr(),
                                          nbchar);
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
fn htmlParseCharDataInternal(mut ctxt: htmlParserCtxtPtr,
                             mut readahead: libc::c_int) {
    const BUF_SIZE: usize = (HTML_PARSER_BIG_BUFFER_SIZE + 6) as usize;
    let mut buf: [xmlChar; BUF_SIZE] = [0; BUF_SIZE];
    let mut nbchar: libc::c_int = 0 as libc::c_int;
    let mut cur: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut chunk: libc::c_int = 0 as libc::c_int;
    if readahead != 0 {
        let fresh36 = nbchar;
        nbchar = nbchar + 1;
        buf[fresh36 as usize] = readahead as xmlChar
    }
    SHRINK(ctxt);
    cur = htmlCurrentChar(ctxt, &mut l);
    let mut ctxtPtr = unsafe { &mut *ctxt };
    while (cur != '<' as i32 || ctxtPtr.token == '<' as i32) && (cur != '&' as i32 || ctxtPtr.token == '&' as i32) && cur != 0 as libc::c_int {
        unsafe {
            if !IS_CHAR(cur) {
                htmlParseErrInt(ctxt, XML_ERR_INVALID_CHAR,
                                b"Invalid char in CDATA 0x%X\n\x00" as *const u8
                                    as *const libc::c_char, cur);
            } else {
                let mut returnI: libc::c_int = COPY_BUF(l, &mut *buf.as_mut_ptr(), nbchar, cur);
                nbchar = returnI;
            }
        }
        if nbchar >= HTML_PARSER_BIG_BUFFER_SIZE as libc::c_int {
            buf[nbchar as usize] = 0 as libc::c_int as xmlChar;
            /*
	     * Ok the segment is to be consumed as chars.
	     */
            if !ctxtPtr.sax.is_null() && ctxtPtr.disableSAX == 0 {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                if areBlanks_htmlparser(ctxt, buf.as_mut_ptr(), nbchar) != 0 {
                    if ctxtPtr.keepBlanks != 0 {
                        if saxPtr.characters.is_some() {
                            xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                                          buf.as_mut_ptr(),
                                                          nbchar);
                        }
                    } else if saxPtr.ignorableWhitespace.is_some() {
                        xmlSAXHandler_ignorableWhitespace_safe(saxPtr.ignorableWhitespace, ctxtPtr.userData,
                                                               buf.as_mut_ptr(),
                                                               nbchar);
                    }
                } else {
                    htmlCheckParagraph(ctxt);
                    if saxPtr.characters.is_some() {
                        xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                                      buf.as_mut_ptr(),
                                                      nbchar);
                    }
                }
            }
            nbchar = 0 as libc::c_int
        }
        NEXTL(ctxt, l);
        chunk += 1;
        if chunk > 100 as libc::c_int {
            chunk = 0 as libc::c_int;
            SHRINK(ctxt);
            GROW(ctxt);
        }
        cur = htmlCurrentChar(ctxt, &mut l);
        if cur == 0 as libc::c_int {
            SHRINK(ctxt);
            GROW(ctxt);
            cur = htmlCurrentChar(ctxt, &mut l)
        }
    }
    if nbchar != 0 as libc::c_int {
        buf[nbchar as usize] = 0 as libc::c_int as xmlChar;
        /*
	 * Ok the segment is to be consumed as chars.
	 */
        if !ctxtPtr.sax.is_null() && ctxtPtr.disableSAX == 0 {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            if areBlanks_htmlparser(ctxt, buf.as_mut_ptr(), nbchar) != 0 {
                if ctxtPtr.keepBlanks != 0 {
                    if saxPtr.characters.is_some() {
                        xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                                      buf.as_mut_ptr(),
                                                      nbchar);
                    }
                } else if saxPtr.ignorableWhitespace.is_some() {
                    xmlSAXHandler_ignorableWhitespace_safe(saxPtr.ignorableWhitespace, ctxtPtr.userData,
                                                           buf.as_mut_ptr(),
                                                           nbchar);
                }
            } else {
                htmlCheckParagraph(ctxt);
                if saxPtr.characters.is_some() {
                    xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                                  buf.as_mut_ptr(),
                                                  nbchar);
                }
            }
        }
    } else if cur == 0 as libc::c_int { ctxtPtr.instate = XML_PARSER_EOF };
}
/*
	 * Loop detection
	 */
/* *
 * htmlParseCharData:
 * @ctxt:  an HTML parser context
 *
 * parse a CharData section.
 * if we are within a CDATA section ']]>' marks an end of section.
 *
 * [14] CharData ::= [^<&]* - ([^<&]* ']]>' [^<&]*)
 */
fn htmlParseCharData(mut ctxt: htmlParserCtxtPtr) {
    htmlParseCharDataInternal(ctxt, 0 as libc::c_int);
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
fn htmlParseExternalID(mut ctxt: htmlParserCtxtPtr,
                       mut publicID: *mut *mut xmlChar)
                       -> *mut xmlChar {
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    if UPPER(ctxt) == 'S' as i32 && UPP(ctxt, 1 as libc::c_int) == 'Y' as i32 && UPP(ctxt, 2 as libc::c_int) == 'S' as i32 && UPP(ctxt, 3 as libc::c_int) == 'T' as i32 && UPP(ctxt, 4 as libc::c_int) == 'E' as i32 && UPP(ctxt, 5 as libc::c_int) == 'M' as i32 {
        SKIP(ctxt, 6 as libc::c_int);
        if !IS_BLANK_CH(CUR(ctxt)) {
            htmlParseErr(ctxt, XML_ERR_SPACE_REQUIRED,
                         b"Space required after \'SYSTEM\'\n\x00" as *const u8
                             as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
        }
        htmlSkipBlankChars(ctxt);
        URI = htmlParseSystemLiteral(ctxt);
        if URI.is_null() {
            htmlParseErr(ctxt, XML_ERR_URI_REQUIRED,
                         b"htmlParseExternalID: SYSTEM, no URI\n\x00" as *const u8 as *const libc::c_char,
                         0 as *const xmlChar, 0 as *const xmlChar);
        }
    } else if UPPER(ctxt) == 'P' as i32 && UPP(ctxt, 1 as libc::c_int) == 'U' as i32 && UPP(ctxt, 2 as libc::c_int) == 'B' as i32 && UPP(ctxt, 3 as libc::c_int) == 'L' as i32 && UPP(ctxt, 4 as libc::c_int) == 'I' as i32 && UPP(ctxt, 5 as libc::c_int) == 'C' as i32 {
        SKIP(ctxt, 6 as libc::c_int);
        if !IS_BLANK_CH(CUR(ctxt)) {
            htmlParseErr(ctxt, XML_ERR_SPACE_REQUIRED,
                         b"Space required after \'PUBLIC\'\n\x00" as *const u8
                             as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
        }
        htmlSkipBlankChars(ctxt);
        unsafe { *publicID = htmlParsePubidLiteral(ctxt); }
        unsafe {
            if (*publicID).is_null() {
                htmlParseErr(ctxt, XML_ERR_PUBID_REQUIRED,
                             b"htmlParseExternalID: PUBLIC, no Public Identifier\n\x00"
                                 as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
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
fn htmlParsePI(mut ctxt: htmlParserCtxtPtr) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = 100 as libc::c_int;
    let mut cur: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut target: *const xmlChar = 0 as *const xmlChar;
    let mut state: xmlParserInputState = XML_PARSER_START;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut sax_condition = false;
    if RAW(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '?' as i32 {
        state = ctxtPtr.instate;
        ctxtPtr.instate = XML_PARSER_PI;
        /*
	 * this is a Processing Instruction.
	 */
        SKIP(ctxt, 2 as libc::c_int);
        SHRINK(ctxt);
        /*
	 * Parse the target name and check for special support like
	 * namespace.
	 */
        target = htmlParseName(ctxt);
        if !target.is_null() {
            if RAW(ctxt) == '>' as i32 {
                SKIP(ctxt, 1 as libc::c_int);
                /*
		 * SAX: PI detected.
		 */
                sax_condition = unsafe {
                    !ctxtPtr.sax.is_null() && ctxtPtr.disableSAX == 0 && (*(*ctxt).sax).processingInstruction.is_some()
                };
                if sax_condition {
                    let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                    xmlSAXHandler_processingInstruction_safe(saxPtr.processingInstruction, ctxtPtr.userData,
                                                             target,
                                                             0 as *const xmlChar);
                }
                ctxtPtr.instate = state;
                return;
            }
            buf = xmlMallocAtomic_safe((size as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                as libc::c_ulong))
                as *mut xmlChar;
            if buf.is_null() {
                htmlErrMemory(ctxt, 0 as *const libc::c_char);
                ctxtPtr.instate = state;
                return;
            }
            cur = CUR(ctxt);
            if !IS_BLANK(cur) {
                htmlParseErr(ctxt, XML_ERR_SPACE_REQUIRED,
                             b"ParsePI: PI %s space expected\n\x00" as *const u8 as *const libc::c_char, target,
                             0 as *const xmlChar);
            }
            htmlSkipBlankChars(ctxt);
            cur = htmlCurrentChar(ctxt, &mut l);
            while cur != 0 as libc::c_int && cur != '>' as i32 {
                if len + 5 as libc::c_int >= size {
                    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                    size *= 2 as libc::c_int;
                    tmp = xmlRealloc_safe(buf as *mut libc::c_void,
                                          (size
                                              as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                              as libc::c_ulong))
                        as *mut xmlChar;
                    if tmp.is_null() {
                        htmlErrMemory(ctxt, 0 as *const libc::c_char);
                        xmlFree_safe(buf as *mut libc::c_void);
                        ctxtPtr.instate = state;
                        return;
                    }
                    buf = tmp
                }
                count += 1;
                if count > 50 as libc::c_int {
                    GROW(ctxt);
                    count = 0 as libc::c_int
                }
                if IS_CHAR(cur) {
                    let mut returnI: libc::c_int = COPY_BUF(l, buf, len, cur);
                    len = returnI;
                } else {
                    htmlParseErrInt(ctxt, XML_ERR_INVALID_CHAR,
                                    b"Invalid char in processing instruction 0x%X\n\x00" as *const u8 as *const libc::c_char, cur);
                }
                NEXTL(ctxt, l);
                cur = htmlCurrentChar(ctxt, &mut l);
                if cur == 0 as libc::c_int {
                    SHRINK(ctxt);
                    GROW(ctxt);
                    cur = htmlCurrentChar(ctxt, &mut l)
                }
            }
            unsafe { *buf.offset(len as isize) = 0 as libc::c_int as xmlChar; }
            if cur != '>' as i32 {
                htmlParseErr(ctxt, XML_ERR_PI_NOT_FINISHED,
                             b"ParsePI: PI %s never end ...\n\x00" as *const u8 as *const libc::c_char, target,
                             0 as *const xmlChar);
            } else {
                SKIP(ctxt, 1 as libc::c_int);
                /*
		 * SAX: PI detected.
		 */
                sax_condition = unsafe {
                    !ctxtPtr.sax.is_null() && ctxtPtr.disableSAX == 0 && (*(*ctxt).sax).processingInstruction.is_some()
                };
                if sax_condition {
                    let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                    xmlSAXHandler_processingInstruction_safe(saxPtr.processingInstruction, ctxtPtr.userData,
                                                             target, buf);
                }
            }
            xmlFree_safe(buf as *mut libc::c_void);
        } else {
            htmlParseErr(ctxt, XML_ERR_PI_NOT_STARTED,
                         b"PI is not started correctly\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
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
fn htmlParseComment(mut ctxt: htmlParserCtxtPtr) {
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 100 as libc::c_int;
    let mut q: libc::c_int = 0;
    let mut ql: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut rl: libc::c_int = 0;
    let mut cur: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut nl: libc::c_int = 0;
    let mut state: xmlParserInputState = XML_PARSER_START;
    /*
     * Check that there is a comment right here.
     */
    if (RAW(ctxt) != '<' as i32) || (NXT(ctxt, 1 as libc::c_int) != '!' as i32) || (NXT(ctxt, 2 as libc::c_int) != '-' as i32) || (NXT(ctxt, 3 as libc::c_int) != '-' as i32) {
        return;
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    state = ctxtPtr.instate;
    ctxtPtr.instate = XML_PARSER_COMMENT;
    SHRINK(ctxt);
    SKIP(ctxt, 4 as libc::c_int);
    buf = xmlMallocAtomic_safe((size as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
        as libc::c_ulong))
        as *mut xmlChar;
    if buf.is_null() {
        htmlErrMemory(ctxt,
                      b"buffer allocation failed\n\x00" as *const u8 as *const libc::c_char);
        ctxtPtr.instate = state;
        return;
    }
    len = 0 as libc::c_int;
    unsafe { *buf.offset(len as isize) = 0 as libc::c_int as xmlChar; }
    q = htmlCurrentChar(ctxt, &mut ql);
    if q == 0 {
        htmlParseErr(ctxt, XML_ERR_COMMENT_NOT_FINISHED,
                     b"Comment not terminated \n<!--%.50s\n\x00" as *const u8 as *const libc::c_char, buf, 0 as *const xmlChar);
        xmlFree_safe(buf as *mut libc::c_void);
        return;
    }
    NEXTL(ctxt, ql);
    r = htmlCurrentChar(ctxt, &mut rl);
    if r == 0 {
        htmlParseErr(ctxt, XML_ERR_COMMENT_NOT_FINISHED,
                     b"Comment not terminated \n<!--%.50s\n\x00" as *const u8 as *const libc::c_char, buf, 0 as *const xmlChar);
        xmlFree_safe(buf as *mut libc::c_void);
        return;
    }
    NEXTL(ctxt, rl);
    cur = htmlCurrentChar(ctxt, &mut l);
    while (cur != 0 as i32) && ((cur != '>' as i32) || (r != '-' as i32) || (q != '-' as i32)) {
        NEXTL(ctxt, l);
        next = htmlCurrentChar(ctxt, &mut nl);
        if next == 0 as i32 {
            SHRINK(ctxt);
            GROW(ctxt);
            next = htmlCurrentChar(ctxt, &mut nl);
        }
        if (q == '-' as i32) && (r == '-' as i32) && (cur == '!' as i32) && (next == '>' as i32) {
            htmlParseErr(ctxt, XML_ERR_COMMENT_NOT_FINISHED,
                         b"Comment incorrectly closed by '--!>'\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar, 0 as *const xmlChar);
            cur = '>' as i32;
            break;
        }
        if (len + 5 as libc::c_int) >= size {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            size *= 2 as libc::c_int;
            tmp = xmlRealloc_safe(buf as *mut libc::c_void,
                                  (size
                                      as libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                      as libc::c_ulong))
                as *mut xmlChar;
            if tmp.is_null() {
                xmlFree_safe(buf as *mut libc::c_void);
                htmlErrMemory(ctxt,
                              b"growing buffer failed\n\x00" as *const u8 as *const libc::c_char);
                ctxtPtr.instate = state;
                return;
            }
            buf = tmp;
        }
        if IS_CHAR(q) {
            let mut returnI: libc::c_int = COPY_BUF(ql, buf, len, q);
            len = returnI;
        } else {
            htmlParseErrInt(ctxt, XML_ERR_INVALID_CHAR,
                            b"Invalid char in comment 0x%X\n\x00" as *const u8 as *const libc::c_char, q);
        }
        q = r;
        ql = rl;
        r = cur;
        rl = l;
        cur = next;
        l = nl;
    }
    unsafe { *buf.offset(len as isize) = 0 as libc::c_int as xmlChar; }
    if cur == '>' as i32 {
        xmlNextChar_safe(ctxt);
        let mut sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).comment.is_some()
                && ctxtPtr.disableSAX == 0
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_comment_safe(saxPtr.comment, ctxtPtr.userData, buf);
        }
        xmlFree_safe(buf as *mut libc::c_void);
        ctxtPtr.instate = state;
        return;
    }
    htmlParseErr(ctxt, XML_ERR_COMMENT_NOT_FINISHED,
                 b"Comment not terminated \n<!--%.50s\n\x00" as *const u8 as *const libc::c_char, buf, 0 as *const xmlChar);
    xmlFree_safe(buf as *mut libc::c_void);
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

pub fn htmlParseCharRef(mut ctxt:
                        htmlParserCtxtPtr)
                        -> libc::c_int {
    let mut val: libc::c_int = 0 as libc::c_int;
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                         b"htmlParseCharRef: context error\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
            return 0 as libc::c_int;
        }
    }
    if CUR(ctxt) == '&' as i32 && NXT(ctxt, 1 as libc::c_int) == '#' as i32 && (NXT(ctxt, 2 as libc::c_int) == 'x' as i32 || NXT(ctxt, 2 as libc::c_int) == 'X' as i32) {
        SKIP(ctxt, 3 as libc::c_int);
        while CUR(ctxt) != ';' as i32 {
            if CUR(ctxt) >= '0' as i32 && CUR(ctxt) <= '9' as i32 {
                if val < 0x110000 as libc::c_int {
                    val = val * 16 as libc::c_int +
                        (CUR(ctxt) -
                            '0' as i32)
                }
            } else if CUR(ctxt) >= 'a' as i32 && CUR(ctxt) <= 'f' as i32 {
                if val < 0x110000 as libc::c_int {
                    val = val * 16 as libc::c_int +
                        (CUR(ctxt) -
                            'a' as i32) + 10 as libc::c_int
                }
            } else if CUR(ctxt) >= 'A' as i32 && CUR(ctxt) <= 'F' as i32 {
                if val < 0x110000 as libc::c_int {
                    val = val * 16 as libc::c_int +
                        (CUR(ctxt) -
                            'A' as i32) + 10 as libc::c_int
                }
            } else {
                htmlParseErr(ctxt, XML_ERR_INVALID_HEX_CHARREF,
                             b"htmlParseCharRef: missing semicolon\n\x00" as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                break;
            }
            xmlNextChar_safe(ctxt);
        }
        if CUR(ctxt) == ';' as i32 {
            xmlNextChar_safe(ctxt);
        }
    } else if CUR(ctxt) == '&' as i32 && NXT(ctxt, 1 as libc::c_int) == '#' as i32 {
        SKIP(ctxt, 2 as libc::c_int);
        while CUR(ctxt) != ';' as i32 {
            if CUR(ctxt) >= '0' as i32 && CUR(ctxt) <= '9' as i32 {
                if val < 0x110000 as libc::c_int {
                    val = val * 10 as libc::c_int +
                        (CUR(ctxt) -
                            '0' as i32)
                }
                xmlNextChar_safe(ctxt);
            } else {
                htmlParseErr(ctxt, XML_ERR_INVALID_DEC_CHARREF,
                             b"htmlParseCharRef: missing semicolon\n\x00" as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                break;
            }
        }
        if CUR(ctxt) == ';' as i32 {
            xmlNextChar_safe(ctxt);
        }
    } else {
        htmlParseErr(ctxt, XML_ERR_INVALID_CHARREF,
                     b"htmlParseCharRef: invalid value\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                     0 as *const xmlChar);
    }
    /*
     * Check the value IS_CHAR ...
     */
    if IS_CHAR(val) {
        return val;
    } else {
        if val >= 0x110000 as libc::c_int {
            htmlParseErr(ctxt, XML_ERR_INVALID_CHAR,
                         b"htmlParseCharRef: value too large\n\x00" as *const u8 as *const libc::c_char,
                         0 as *const xmlChar, 0 as *const xmlChar);
        } else {
            htmlParseErrInt(ctxt, XML_ERR_INVALID_CHAR,
                            b"htmlParseCharRef: invalid xmlChar value %d\n\x00"
                                as *const u8 as *const libc::c_char, val);
        }
    }
    return 0 as libc::c_int;
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
fn htmlParseDocTypeDecl(mut ctxt: htmlParserCtxtPtr) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut ExternalID: *mut xmlChar = 0 as *mut xmlChar;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    /*
     * We know that '<!DOCTYPE' has been detected.
     */
    SKIP(ctxt, 9 as libc::c_int);
    htmlSkipBlankChars(ctxt);
    /*
     * Parse the DOCTYPE name.
     */
    name = htmlParseName(ctxt);
    if name.is_null() {
        htmlParseErr(ctxt, XML_ERR_NAME_REQUIRED,
                     b"htmlParseDocTypeDecl : no DOCTYPE name !\n\x00" as *const u8 as *const libc::c_char,
                     0 as *const xmlChar, 0 as *const xmlChar);
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
        htmlParseErr(ctxt, XML_ERR_DOCTYPE_NOT_FINISHED,
                     b"DOCTYPE improperly terminated\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                     0 as *const xmlChar);
        /* Ignore bogus content */
        while CUR(ctxt) != 0 as libc::c_int && CUR(ctxt) != '>' as i32 {
            xmlNextChar_safe(ctxt);
        }
    }
    if CUR(ctxt) == '>' as i32 {
        xmlNextChar_safe(ctxt);
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
        xmlSAXHandler_internalSubset_safe(saxPtr.internalSubset, ctxtPtr.userData,
                                          name, ExternalID, URI);
    }
    /*
     * Cleanup, since we don't use all those identifiers
     */
    if !URI.is_null() {
        xmlFree_safe(URI as *mut libc::c_void);
    }
    if !ExternalID.is_null() {
        xmlFree_safe(ExternalID as *mut libc::c_void);
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
fn htmlParseAttribute(mut ctxt: htmlParserCtxtPtr,
                      mut value: *mut *mut xmlChar)
                      -> *const xmlChar {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    unsafe { *value = 0 as *mut xmlChar; }
    name = htmlParseHTMLName(ctxt);
    if name.is_null() {
        htmlParseErr(ctxt, XML_ERR_NAME_REQUIRED,
                     b"error parsing attribute name\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                     0 as *const xmlChar);
        return 0 as *const xmlChar;
    }
    /*
     * read the value
     */
    htmlSkipBlankChars(ctxt);
    if CUR(ctxt) == '=' as i32 {
        xmlNextChar_safe(ctxt);
        htmlSkipBlankChars(ctxt);
        val = htmlParseAttValue(ctxt)
    }
    unsafe { *value = val; }
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
fn htmlCheckEncodingDirect(mut ctxt: htmlParserCtxtPtr,
                           mut encoding: *const xmlChar) {
    unsafe {
        if ctxt.is_null() || encoding.is_null() || (*ctxt).options & HTML_PARSE_IGNORE_ENC as libc::c_int != 0 {
            return;
        }
    }
    let mut buf_condition = false;
    let mut ctxtPtr = unsafe { &mut *ctxt };
    /* do not change encoding */
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if !inputPtr.encoding.is_null() { return; }
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        let mut handler: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
        let mut encoding_safe = unsafe { *encoding };
        while encoding_safe as libc::c_int == ' ' as i32 || encoding_safe as libc::c_int == '\t' as i32 {
            unsafe { encoding = encoding.offset(1) }
        }
        if !inputPtr.encoding.is_null() {
            xmlFree_safe(inputPtr.encoding
                as *mut xmlChar as *mut libc::c_void);
        }
        inputPtr.encoding = xmlStrdup_safe(encoding);
        enc = xmlParseCharEncoding_safe(encoding as *const libc::c_char);
        /*
	 * registered set of known encodings
	 */
        if enc as libc::c_int != XML_CHAR_ENCODING_ERROR as libc::c_int {
            buf_condition = unsafe {
                !inputPtr.buf.is_null() && (*(*(*ctxt).input).buf).encoder.is_null()
            };
            if (enc as libc::c_int == XML_CHAR_ENCODING_UTF16LE as libc::c_int
                || enc as libc::c_int == XML_CHAR_ENCODING_UTF16BE as libc::c_int || enc as libc::c_int == XML_CHAR_ENCODING_UCS4LE as libc::c_int || enc as libc::c_int == XML_CHAR_ENCODING_UCS4BE as libc::c_int) && buf_condition {
                htmlParseErr(ctxt, XML_ERR_INVALID_ENCODING,
                             b"htmlCheckEncoding: wrong encoding meta\n\x00"
                                 as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
            } else { xmlSwitchEncoding_safe(ctxt, enc); }
            ctxtPtr.charset = XML_CHAR_ENCODING_UTF8 as libc::c_int
        } else {
            /*
	     * fallback for unknown encodings
	     */
            handler = xmlFindCharEncodingHandler_safe(encoding as *const libc::c_char);
            if !handler.is_null() {
                xmlSwitchToEncoding_safe(ctxt, handler);
                ctxtPtr.charset = XML_CHAR_ENCODING_UTF8 as libc::c_int
            } else {
                htmlParseErr(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                             b"htmlCheckEncoding: unknown encoding %s\n\x00"
                                 as *const u8 as *const libc::c_char,
                             encoding, 0 as *const xmlChar);
            }
        }
        buf_condition = unsafe {
            !inputPtr.buf.is_null() && !(*(*(*ctxt).input).buf).encoder.is_null() && !(*(*(*ctxt).input).buf).raw.is_null() && !(*(*(*ctxt).input).buf).buffer.is_null()
        };
        if buf_condition {
            let mut bufPtr = unsafe { &mut *(*(*ctxt).input).buf };
            let mut nbchars: libc::c_int = 0;
            let mut processed: libc::c_int = 0;
            /*
	     * convert as much as possible to the parser reading buffer.
	     */
            unsafe {
                processed = inputPtr.cur.offset_from(inputPtr.base)
                    as libc::c_long as libc::c_int;
            }
            xmlBufShrink_safe(bufPtr.buffer,
                              processed as size_t);
            nbchars = xmlCharEncInput_safe(inputPtr.buf, 1 as libc::c_int);
            xmlBufResetInput_safe(bufPtr.buffer, ctxtPtr.input);
            if nbchars < 0 as libc::c_int {
                htmlParseErr(ctxt, XML_ERR_INVALID_ENCODING,
                             b"htmlCheckEncoding: encoder error\n\x00" as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
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
fn htmlCheckEncoding(mut ctxt: htmlParserCtxtPtr,
                     mut attvalue: *const xmlChar) {
    let mut encoding: *const xmlChar = 0 as *const xmlChar;
    if attvalue.is_null() { return; }
    encoding = xmlStrcasestr_safe(attvalue,
                                  b"charset\x00" as *const u8 as *const libc::c_char as *mut xmlChar);
    if !encoding.is_null() {
        unsafe { encoding = encoding.offset(7 as libc::c_int as isize) }
    }
    /*
     * skip blank
     */
    unsafe {
        if !encoding.is_null() && IS_BLANK_CH((*encoding) as libc::c_int) {
            encoding = xmlStrcasestr_safe(attvalue,
                                          b"=\x00" as *const u8 as *const libc::c_char as *mut xmlChar)
        }
    }

    unsafe {
        if !encoding.is_null() && *encoding as libc::c_int == '=' as i32 {
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
fn htmlCheckMeta(mut ctxt: htmlParserCtxtPtr,
                 mut atts: *mut *const xmlChar) {
    let mut i: libc::c_int = 0;
    let mut att: *const xmlChar = 0 as *const xmlChar;
    let mut value: *const xmlChar = 0 as *const xmlChar;
    let mut http: libc::c_int = 0 as libc::c_int;
    let mut content: *const xmlChar = 0 as *const xmlChar;
    if ctxt.is_null() || atts.is_null() { return; }
    i = 0 as libc::c_int;
    let fresh40 = i;
    i = i + 1;
    unsafe { att = *atts.offset(fresh40 as isize); }
    while !att.is_null() {
        let fresh41 = i;
        i = i + 1;
        unsafe { value = *atts.offset(fresh41 as isize); }
        if !value.is_null() && xmlStrcasecmp_safe(att,
                                                  b"http-equiv\x00" as *const u8 as *const libc::c_char as *mut xmlChar) == 0 && xmlStrcasecmp_safe(value,
                                                                                                                                                    b"Content-Type\x00" as *const u8
                                                                                                                                                        as *const libc::c_char as *mut xmlChar) == 0 {
            http = 1 as libc::c_int
        } else if !value.is_null() && xmlStrcasecmp_safe(att,
                                                         b"charset\x00" as *const u8 as *const libc::c_char as *mut xmlChar) == 0 {
            htmlCheckEncodingDirect(ctxt, value);
        } else if !value.is_null() && xmlStrcasecmp_safe(att,
                                                         b"content\x00" as *const u8 as *const libc::c_char as *mut xmlChar) == 0 {
            content = value
        }
        let fresh42 = i;
        i = i + 1;
        unsafe { att = *atts.offset(fresh42 as isize) }
    }
    if http != 0 && !content.is_null() { htmlCheckEncoding(ctxt, content); };
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
fn htmlParseStartTag(mut ctxt: htmlParserCtxtPtr)
                     -> libc::c_int {
    let mut current_block: i32;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut attname: *const xmlChar = 0 as *const xmlChar;
    let mut attvalue: *mut xmlChar = 0 as *mut xmlChar;
    let mut atts: *mut *const xmlChar = 0 as *mut *const xmlChar;
    let mut nbatts: libc::c_int = 0 as libc::c_int;
    let mut maxatts: libc::c_int = 0;
    let mut meta: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut discardtag: libc::c_int = 0 as libc::c_int;
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                         b"htmlParseStartTag: context error\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
            return -(1 as libc::c_int);
        }
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
        return -(1 as libc::c_int);
    }
    if CUR(ctxt) != '<' as i32 {
        return -(1 as libc::c_int);
    }
    xmlNextChar_safe(ctxt);
    atts = ctxtPtr.atts;
    maxatts = ctxtPtr.maxatts;
    GROW(ctxt);
    name = htmlParseHTMLName(ctxt);
    if name.is_null() {
        htmlParseErr(ctxt, XML_ERR_NAME_REQUIRED,
                     b"htmlParseStartTag: invalid element name\n\x00" as *const u8 as *const libc::c_char,
                     0 as *const xmlChar, 0 as *const xmlChar);
        /* if recover preserve text on classic misconstructs */
        let cur_safe = unsafe { *(*(*ctxt).input).cur };
        if ctxtPtr.recovery != 0 && (IS_BLANK_CH(cur_safe as libc::c_int) || CUR(ctxt) == '<' as i32 || CUR(ctxt) == '=' as i32 || CUR(ctxt) == '>' as i32 || (CUR(ctxt) >= '0' as i32 && CUR(ctxt) <= '9' as i32)) {
            htmlParseCharDataInternal(ctxt, '<' as i32);
            return -(1 as libc::c_int);
        }
        /* Dump the bogus tag like browsers do */
        while CUR(ctxt) != 0 as libc::c_int && CUR(ctxt) != '>' as i32 && ctxtPtr.instate as libc::c_int != XML_PARSER_EOF as libc::c_int {
            xmlNextChar_safe(ctxt);
        }
        return -(1 as libc::c_int);
    }
    if xmlStrEqual_safe(name,
                        b"meta\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
        meta = 1 as libc::c_int
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
    if ctxtPtr.nameNr > 0 as libc::c_int && xmlStrEqual_safe(name,
                                                             b"html\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
        htmlParseErr(ctxt, XML_HTML_STRUCURE_ERROR,
                     b"htmlParseStartTag: misplaced <html> tag\n\x00" as *const u8 as *const libc::c_char, name,
                     0 as *const xmlChar);
        discardtag = 1 as libc::c_int;
        ctxtPtr.depth += 1
    }
    if ctxtPtr.nameNr != 1 as libc::c_int && xmlStrEqual_safe(name,
                                                              b"head\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
        htmlParseErr(ctxt, XML_HTML_STRUCURE_ERROR,
                     b"htmlParseStartTag: misplaced <head> tag\n\x00" as *const u8 as *const libc::c_char, name,
                     0 as *const xmlChar);
        discardtag = 1 as libc::c_int;
        ctxtPtr.depth += 1
    }
    if xmlStrEqual_safe(name,
                        b"body\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
        let mut indx: libc::c_int = 0;
        indx = 0 as libc::c_int;
        while indx < ctxtPtr.nameNr {
            unsafe {
                if xmlStrEqual_safe(*ctxtPtr.nameTab.offset(indx as isize),
                                    b"body\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
                    htmlParseErr(ctxt, XML_HTML_STRUCURE_ERROR,
                                 b"htmlParseStartTag: misplaced <body> tag\n\x00"
                                     as *const u8 as *const libc::c_char, name,
                                 0 as *const xmlChar);
                    discardtag = 1 as libc::c_int;
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
    while CUR(ctxt) != 0 as libc::c_int && CUR(ctxt) != '>' as i32 && (CUR(ctxt) != '/' as i32 || NXT(ctxt, 1 as libc::c_int) != '>' as i32) {
        GROW(ctxt);
        attname = htmlParseAttribute(ctxt, &mut attvalue);
        if !attname.is_null() {
            /*
	     * Well formedness requires at most one declaration of an attribute
	     */
            i = 0 as libc::c_int;
            loop {
                if !(i < nbatts) {
                    current_block = 1;
                    break;
                }
                unsafe {
                    if xmlStrEqual_safe(*atts.offset(i as isize),
                                        attname) != 0 {
                        htmlParseErr(ctxt, XML_ERR_ATTRIBUTE_REDEFINED,
                                     b"Attribute %s redefined\n\x00" as *const u8
                                         as *const libc::c_char, attname,
                                     0 as *const xmlChar);
                        if !attvalue.is_null() {
                            xmlFree_safe(attvalue as *mut libc::c_void);
                        }
                        current_block = 2;
                        break;
                    } else { i += 2 as libc::c_int }
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
                            maxatts = 22 as libc::c_int; /* allow for 10 attrs by default */
                            atts = xmlMalloc_safe((maxatts
                                as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
                                as libc::c_ulong))
                                as *mut *const xmlChar;
                            if atts.is_null() {
                                htmlErrMemory(ctxt, 0 as *const libc::c_char);
                                if !attvalue.is_null() {
                                    xmlFree_safe(attvalue as *mut libc::c_void);
                                }
                                current_block = 2;
                            } else {
                                ctxtPtr.atts = atts;
                                ctxtPtr.maxatts = maxatts;
                                current_block = 3;
                            }
                        } else if nbatts + 4 as libc::c_int > maxatts {
                            let mut n: *mut *const xmlChar = 0 as *mut *const xmlChar;
                            maxatts *= 2 as libc::c_int;
                            n = xmlRealloc_safe(atts
                                                    as *mut libc::c_void,
                                                (maxatts
                                                    as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
                                                    as libc::c_ulong))
                                as *mut *const xmlChar;
                            if n.is_null() {
                                htmlErrMemory(ctxt, 0 as *const libc::c_char);
                                if !attvalue.is_null() {
                                    xmlFree_safe(attvalue as *mut libc::c_void);
                                }
                                current_block = 2;
                            } else {
                                atts = n;
                                ctxtPtr.atts = atts;
                                ctxtPtr.maxatts = maxatts;
                                current_block = 3;
                            }
                        } else { current_block = 3; }
                        unsafe {
                            match current_block {
                                2 => {}
                                _ => {
                                    let fresh43 = nbatts;
                                    nbatts = nbatts + 1;
                                    let ref mut fresh44 = *atts.offset(fresh43 as isize);
                                    *fresh44 = attname;
                                    let fresh45 = nbatts;
                                    nbatts = nbatts + 1;
                                    let ref mut fresh46 = *atts.offset(fresh45 as isize);
                                    *fresh46 = attvalue;
                                    let ref mut fresh47 = *atts.offset(nbatts as isize);
                                    *fresh47 = 0 as *const xmlChar;
                                    let ref mut fresh48 = *atts.offset((nbatts + 1 as libc::c_int) as isize);
                                    *fresh48 = 0 as *const xmlChar
                                }
                            }
                        }
                    }
            }
        } else {
            if !attvalue.is_null() {
                xmlFree_safe(attvalue as *mut libc::c_void);
            }
            /* Dump the bogus attribute string up to the next blank or
	     * the end of the tag. */
            while CUR(ctxt) != 0 as libc::c_int && !(IS_BLANK_CH(CUR(ctxt))) && CUR(ctxt) != '>' as i32 && (CUR(ctxt) != '/' as i32 || NXT(ctxt, 1 as libc::c_int) != '>' as i32) {
                xmlNextChar_safe(ctxt);
            }
        }
        htmlSkipBlankChars(ctxt);
    }
    /*
     * Handle specific association to the META tag
     */
    if meta != 0 && nbatts != 0 as libc::c_int { htmlCheckMeta(ctxt, atts); }
    /*
     * SAX: Start of Element !
     */
    if discardtag == 0 {
        htmlnamePush(ctxt, name);
        let mut sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startElement.is_some()
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            if nbatts != 0 as libc::c_int {
                xmlSAXHandler_startElement_safe(saxPtr.startElement, ctxtPtr.userData,
                                                name,
                                                atts);
            } else {
                xmlSAXHandler_startElement_safe(saxPtr.startElement, ctxtPtr.userData,
                                                name,
                                                0
                                                    as *mut *const xmlChar);
            }
        }
    }
    if !atts.is_null() {
        i = 1 as libc::c_int;
        while i < nbatts {
            unsafe {
                if !(*atts.offset(i as isize)).is_null() {
                    xmlFree_safe(*atts.offset(i as isize)
                        as *mut xmlChar
                        as *mut libc::c_void);
                }
            }
            i += 2 as libc::c_int
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
fn htmlParseEndTag(mut ctxt: htmlParserCtxtPtr)
                   -> libc::c_int {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut oldname: *const xmlChar = 0 as *const xmlChar;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if CUR(ctxt) != '<' as i32 || NXT(ctxt, 1 as libc::c_int) != '/' as i32 {
        htmlParseErr(ctxt, XML_ERR_LTSLASH_REQUIRED,
                     b"htmlParseEndTag: \'</\' not found\n\x00" as *const u8
                         as *const libc::c_char, 0 as *const xmlChar,
                     0 as *const xmlChar);
        return 0 as libc::c_int;
    }
    SKIP(ctxt, 2 as libc::c_int);
    name = htmlParseHTMLName(ctxt);
    if name.is_null() { return 0 as libc::c_int; }
    /*
     * We should definitely be at the ending "S? '>'" part
     */
    htmlSkipBlankChars(ctxt);
    if CUR(ctxt) != '>' as i32 {
        htmlParseErr(ctxt, XML_ERR_GT_REQUIRED,
                     b"End tag : expected \'>\'\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                     0 as *const xmlChar);
        /* Skip to next '>' */
        while CUR(ctxt) != 0 as libc::c_int && CUR(ctxt) != '>' as i32 {
            xmlNextChar_safe(ctxt);
        }
    }
    if CUR(ctxt) == '>' as i32 {
        xmlNextChar_safe(ctxt);
    }
    /*
     * if we ignored misplaced tags in htmlParseStartTag don't pop them
     * out now.
     */
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.depth > 0 as libc::c_int && (xmlStrEqual_safe(name,
                                                             b"html\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(name,
                                                                                                                                                       b"body\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(name,
                                                                                                                                                                                                                                                 b"head\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0) {
        ctxtPtr.depth -= 1;
        return 0 as libc::c_int;
    }
    /*
     * If the name read is not one of the element in the parsing stack
     * then return, it's just an error.
     */
    i = ctxtPtr.nameNr - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        unsafe {
            if xmlStrEqual_safe(name,
                                *ctxtPtr.nameTab.offset(i as isize)) != 0 {
                break;
            }
        }
        i -= 1
    }
    if i < 0 as libc::c_int {
        htmlParseErr(ctxt, XML_ERR_TAG_NAME_MISMATCH,
                     b"Unexpected end tag : %s\n\x00" as *const u8 as *const libc::c_char, name, 0 as *const xmlChar);
        return 0 as libc::c_int;
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
    if !ctxtPtr.name.is_null() && xmlStrEqual_safe(ctxtPtr.name, name) == 0 {
        htmlParseErr(ctxt, XML_ERR_TAG_NAME_MISMATCH,
                     b"Opening and ending tag mismatch: %s and %s\n\x00" as *const u8 as *const libc::c_char, name,
                     ctxtPtr.name);
    }
    /*
     * SAX: End of Tag
     */
    oldname = ctxtPtr.name;
    if !oldname.is_null() && xmlStrEqual_safe(oldname, name) != 0 {
        let mut sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some()
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData,
                                          name);
        }
        htmlNodeInfoPop(ctxt);
        htmlnamePop(ctxt);
        ret = 1 as libc::c_int
    } else { ret = 0 as libc::c_int }
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
fn htmlParseReference(mut ctxt: htmlParserCtxtPtr) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut sax_condition = false;
    let mut ent: *const htmlEntityDesc = 0 as *const htmlEntityDesc;
    let mut out: [xmlChar; 6] = [0; 6];
    let mut name: *const xmlChar = 0 as *const xmlChar;
    if CUR(ctxt) != '&' as i32 { return; }
    if NXT(ctxt, 1 as libc::c_int) == '#' as i32 {
        let mut c: libc::c_uint = 0;
        let mut bits: libc::c_int = 0;
        let mut i: libc::c_int = 0 as libc::c_int;
        c = htmlParseCharRef(ctxt) as libc::c_uint;
        if c == 0 as libc::c_int as libc::c_uint { return; }
        if c < 0x80 as libc::c_int as libc::c_uint {
            let fresh49 = i;
            i = i + 1;
            out[fresh49 as usize] = c as xmlChar;
            bits = -(6 as libc::c_int)
        } else if c < 0x800 as libc::c_int as libc::c_uint {
            let fresh50 = i;
            i = i + 1;
            out[fresh50 as usize] = (c >> 6 as libc::c_int & 0x1f as libc::c_int as libc::c_uint |
                0xc0 as libc::c_int as libc::c_uint) as xmlChar;
            bits = 0 as libc::c_int
        } else if c < 0x10000 as libc::c_int as libc::c_uint {
            let fresh51 = i;
            i = i + 1;
            out[fresh51 as usize] = (c >> 12 as libc::c_int & 0xf as libc::c_int as libc::c_uint |
                0xe0 as libc::c_int as libc::c_uint) as xmlChar;
            bits = 6 as libc::c_int
        } else {
            let fresh52 = i;
            i = i + 1;
            out[fresh52 as usize] = (c >> 18 as libc::c_int & 0x7 as libc::c_int as libc::c_uint |
                0xf0 as libc::c_int as libc::c_uint) as xmlChar;
            bits = 12 as libc::c_int
        }
        while bits >= 0 as libc::c_int {
            let fresh53 = i;
            i = i + 1;
            out[fresh53 as usize] = (c >> bits & 0x3f as libc::c_int as libc::c_uint |
                0x80 as libc::c_int as libc::c_uint) as xmlChar;
            bits -= 6 as libc::c_int
        }
        out[i as usize] = 0 as libc::c_int as xmlChar;
        htmlCheckParagraph(ctxt);
        sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some()
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                          out.as_mut_ptr(),
                                          i);
        }
    } else {
        ent = htmlParseEntityRef(ctxt, &mut name);
        if name.is_null() {
            htmlCheckParagraph(ctxt);
            sax_condition = unsafe {
                !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some()
            };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                              b"&\x00"
                                                  as *const u8
                                                  as *const libc::c_char
                                                  as *mut xmlChar,
                                              1
                                                  as libc::c_int);
            }
            return;
        }
        let flag = unsafe {
            ent.is_null() || !((*ent).value > 0 as libc::c_int as libc::c_uint)
        };
        if flag {
            htmlCheckParagraph(ctxt);
            sax_condition = unsafe {
                !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some()
            };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                              b"&\x00"
                                                  as *const u8
                                                  as *const libc::c_char
                                                  as *mut xmlChar,
                                              1
                                                  as libc::c_int);
                xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                              name,
                                              xmlStrlen_safe(name));
                /* ctxt->sax->characters(ctxt->userData, BAD_CAST ";", 1); */
            }
        } else {
            let mut c_0: libc::c_uint = 0;
            let mut bits_0: libc::c_int = 0;
            let mut i_0: libc::c_int = 0 as libc::c_int;
            unsafe { c_0 = (*ent).value; }
            if c_0 < 0x80 as libc::c_int as libc::c_uint {
                let fresh54 = i_0;
                i_0 = i_0 + 1;
                out[fresh54 as usize] = c_0 as xmlChar;
                bits_0 = -(6 as libc::c_int)
            } else if c_0 < 0x800 as libc::c_int as libc::c_uint {
                let fresh55 = i_0;
                i_0 = i_0 + 1;
                out[fresh55 as usize] = (c_0 >> 6 as libc::c_int &
                    0x1f as libc::c_int as libc::c_uint |
                    0xc0 as libc::c_int as libc::c_uint) as xmlChar;
                bits_0 = 0 as libc::c_int
            } else if c_0 < 0x10000 as libc::c_int as libc::c_uint {
                let fresh56 = i_0;
                i_0 = i_0 + 1;
                out[fresh56 as usize] = (c_0 >> 12 as libc::c_int &
                    0xf as libc::c_int as libc::c_uint |
                    0xe0 as libc::c_int as libc::c_uint) as xmlChar;
                bits_0 = 6 as libc::c_int
            } else {
                let fresh57 = i_0;
                i_0 = i_0 + 1;
                out[fresh57 as usize] = (c_0 >> 18 as libc::c_int &
                    0x7 as libc::c_int as libc::c_uint |
                    0xf0 as libc::c_int as libc::c_uint) as xmlChar;
                bits_0 = 12 as libc::c_int
            }
            while bits_0 >= 0 as libc::c_int {
                let fresh58 = i_0;
                i_0 = i_0 + 1;
                out[fresh58 as usize] = (c_0 >> bits_0 & 0x3f as libc::c_int as libc::c_uint |
                    0x80 as libc::c_int as libc::c_uint) as xmlChar;
                bits_0 -= 6 as libc::c_int
            }
            out[i_0 as usize] = 0 as libc::c_int as xmlChar;
            htmlCheckParagraph(ctxt);
            sax_condition = unsafe {
                !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some()
            };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                              out.as_mut_ptr(),
                                              i_0);
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
fn htmlParseContent(mut ctxt: htmlParserCtxtPtr) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut currentNode: *mut xmlChar = 0 as *mut xmlChar;
    let mut depth: libc::c_int = 0;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    currentNode = xmlStrdup_safe(ctxtPtr.name);
    depth = ctxtPtr.nameNr;
    loop {
        GROW(ctxt);
        if ctxtPtr.instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
            break;
        }
        /*
	 * Our tag or one of it's parent or children is ending.
	 */
        if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '/' as i32 {
            if htmlParseEndTag(ctxt) != 0 && (!currentNode.is_null() || ctxtPtr.nameNr == 0 as libc::c_int) {
                if !currentNode.is_null() {
                    xmlFree_safe(currentNode as *mut libc::c_void);
                }
                return;
            }
            /* while */
        } else {
            if CUR(ctxt) == '<' as i32 && (IS_ASCII_LETTER(NXT(ctxt, 1 as libc::c_int)) || NXT(ctxt, 1 as libc::c_int) == '_' as i32 || NXT(ctxt, 1 as libc::c_int) == ':' as i32) {
                name = htmlParseHTMLName_nonInvasive(ctxt);
                if name.is_null() {
                    htmlParseErr(ctxt, XML_ERR_NAME_REQUIRED,
                                 b"htmlParseStartTag: invalid element name\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 0 as *const xmlChar, 0 as *const xmlChar);
                    /* Dump the bogus tag like browsers do */
                    while CUR(ctxt) != 0 as libc::c_int && CUR(ctxt) != '>' as i32 {
                        xmlNextChar_safe(ctxt);
                    }
                    if !currentNode.is_null() {
                        xmlFree_safe(currentNode as *mut libc::c_void);
                    }
                    return;
                }
                if !ctxtPtr.name.is_null() {
                    if htmlCheckAutoClose(name, ctxtPtr.name) == 1 as libc::c_int {
                        htmlAutoClose(ctxt, name);
                        continue;
                    }
                }
            }
            /*
	 * Has this node been popped out during parsing of
	 * the next element
	 */
            if ctxtPtr.nameNr > 0 as libc::c_int && depth >= ctxtPtr.nameNr && xmlStrEqual_safe(currentNode, ctxtPtr.name) == 0
            {
                if !currentNode.is_null() {
                    xmlFree_safe(currentNode as *mut libc::c_void);
                }
                return;
            }
            if CUR(ctxt) != 0 as libc::c_int && (xmlStrEqual_safe(currentNode,
                                                                  b"script\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(currentNode,
                                                                                                                                                              b"style\x00" as *const u8
                                                                                                                                                                  as *const libc::c_char
                                                                                                                                                                  as *mut xmlChar) != 0)
            {
                /*
	     * Handle SCRIPT/STYLE separately
	     */
                htmlParseScript(ctxt);
            } else {
                /*
	     * Sometimes DOCTYPE arrives in the middle of the document
	     */
                if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '!' as i32 && UPP(ctxt, 2 as libc::c_int) == 'D' as i32 && UPP(ctxt, 3 as libc::c_int) == 'O' as i32 && UPP(ctxt, 4 as libc::c_int) == 'C' as i32 && UPP(ctxt, 5 as libc::c_int) == 'T' as i32 && UPP(ctxt, 6 as libc::c_int) == 'Y' as i32 && UPP(ctxt, 7 as libc::c_int) == 'P' as i32 && UPP(ctxt, 8 as libc::c_int) == 'E' as i32 {
                    htmlParseErr(ctxt, XML_HTML_STRUCURE_ERROR,
                                 b"Misplaced DOCTYPE declaration\n\x00" as *const u8 as *const libc::c_char,
                                 b"DOCTYPE\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                                 0 as *const xmlChar);
                    htmlParseDocTypeDecl(ctxt);
                }
                /*
	     * First case :  a comment
	     */
                if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '!' as i32 && NXT(ctxt, 2 as libc::c_int) == '-' as i32 && NXT(ctxt, 3 as libc::c_int) == '-' as i32 {
                    htmlParseComment(ctxt);
                } else if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '?' as i32 {
                    htmlParsePI(ctxt);
                } else if CUR(ctxt) == '<' as i32 {
                    htmlParseElement(ctxt);
                } else if CUR(ctxt) == '&' as i32 {
                    htmlParseReference(ctxt);
                } else if CUR(ctxt) == 0 as libc::c_int {
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
        xmlFree_safe(currentNode as *mut libc::c_void);
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
pub fn htmlParseElement(mut ctxt:
                        htmlParserCtxtPtr) {
    let mut sax_condition = false;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut currentNode: *mut xmlChar = 0 as *mut xmlChar;
    let mut info: *const htmlElemDesc = 0 as *const htmlElemDesc;
    let mut node_info: htmlParserNodeInfo = htmlParserNodeInfo {
        node: 0 as *const _xmlNode,
        begin_pos: 0,
        begin_line: 0,
        end_pos: 0,
        end_line: 0,
    };
    let mut failed: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut oldptr: *const xmlChar = 0 as *const xmlChar;
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                         b"htmlParseElement: context error\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
            return;
        }
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
        return;
    }
    /* Capture start position */
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if ctxtPtr.record_info != 0 {
        unsafe {
            node_info.begin_pos = inputPtr.consumed.wrapping_add(inputPtr.cur.offset_from(inputPtr.base)
                as libc::c_long as libc::c_ulong);
        }
        node_info.begin_line = inputPtr.line as libc::c_ulong
    }
    failed = htmlParseStartTag(ctxt);
    name = ctxtPtr.name;
    if failed == -(1 as libc::c_int) || name.is_null() {
        if CUR(ctxt) == '>' as i32 {
            xmlNextChar_safe(ctxt);
        }
        return;
    }
    /*
     * Lookup the info for that element.
     */
    info = htmlTagLookup(name);
    if info.is_null() {
        htmlParseErr(ctxt, XML_HTML_UNKNOWN_TAG,
                     b"Tag %s invalid\n\x00" as *const u8 as *const libc::c_char, name, 0 as *const xmlChar);
    }
    /*
     * Check for an Empty Element labeled the XML/SGML way
     */
    if CUR(ctxt) == '/' as i32 && NXT(ctxt, 1 as libc::c_int) == '>' as i32 {
        SKIP(ctxt, 2 as libc::c_int);
        sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() && ctxtPtr.disableSAX == 0
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData,
                                          name);
        }
        htmlnamePop(ctxt);
        return;
    }
    if CUR(ctxt) == '>' as i32 {
        xmlNextChar_safe(ctxt);
    } else {
        htmlParseErr(ctxt, XML_ERR_GT_REQUIRED,
                     b"Couldn\'t find end of Start Tag %s\n\x00" as *const u8
                         as *const libc::c_char, name, 0 as *const xmlChar);
        /*
     * end of parsing of this node.
     */
        if xmlStrEqual_safe(name, ctxtPtr.name) != 0 {
            nodePop_safe(ctxt);
            htmlnamePop(ctxt);
        }
        /*
     * Capture end position and add node
     */
        if ctxtPtr.record_info != 0 {
            unsafe {
                node_info.end_pos = inputPtr.consumed.wrapping_add(inputPtr.cur.offset_from(inputPtr.base)
                    as libc::c_long as libc::c_ulong);
            }
            node_info.end_line = inputPtr.line as libc::c_ulong;
            node_info.node = ctxtPtr.node as *const _xmlNode;
            xmlParserAddNodeInfo_safe(ctxt, &mut node_info);
        }
        return;
    }
    /*
     * Check for an Empty Element from DTD definition
     */
    let flag = unsafe { info.is_null() && (*info).empty as libc::c_int != 0 };
    if flag {
        sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some() && ctxtPtr.disableSAX == 0
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData,
                                          name);
        }
        htmlnamePop(ctxt);
        return;
    }

    /*
     * Parse the content of the element:
     */
    currentNode = xmlStrdup_safe(ctxtPtr.name);
    depth = ctxtPtr.nameNr;
    while CUR(ctxt) != 0 as libc::c_int {
        oldptr = inputPtr.cur;
        htmlParseContent(ctxt);
        if oldptr == inputPtr.cur { break; }
        if ctxtPtr.nameNr < depth { break; }
    }
    /*
     * Capture end position and add node
     */
    if !currentNode.is_null() && ctxtPtr.record_info != 0 {
        unsafe {
            node_info.end_pos = inputPtr.consumed.wrapping_add(inputPtr.cur.offset_from(inputPtr.base)
                as libc::c_long as libc::c_ulong);
        }
        node_info.end_line = inputPtr.line as libc::c_ulong;
        node_info.node = ctxtPtr.node as *const _xmlNode;
        xmlParserAddNodeInfo_safe(ctxt, &mut node_info);
    }
    if CUR(ctxt) == 0 as libc::c_int {
        htmlAutoCloseOnEnd(ctxt);
    }
    if !currentNode.is_null() {
        xmlFree_safe(currentNode as *mut libc::c_void);
    };
}

fn htmlParserFinishElementParsing(mut ctxt:
                                  htmlParserCtxtPtr) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    /*
     * Capture end position and add node
     */
    if !ctxtPtr.node.is_null() && ctxtPtr.record_info != 0 {
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        let mut nodeInfoPtr = unsafe { &mut *(*ctxt).nodeInfo };
        unsafe {
            nodeInfoPtr.end_pos = inputPtr.consumed.wrapping_add(inputPtr.cur.offset_from(inputPtr.base)
                as libc::c_long as libc::c_ulong);
        }
        nodeInfoPtr.end_line = inputPtr.line as libc::c_ulong;
        nodeInfoPtr.node = ctxtPtr.node as *const _xmlNode;
        xmlParserAddNodeInfo_safe(ctxt, ctxtPtr.nodeInfo);
        htmlNodeInfoPop(ctxt);
    }
    if CUR(ctxt) == 0 as libc::c_int {
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
fn htmlParseElementInternal(mut ctxt: htmlParserCtxtPtr) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut info: *const htmlElemDesc = 0 as *const htmlElemDesc;
    let mut node_info: htmlParserNodeInfo = {
        let mut init = _xmlParserNodeInfo {
            node: 0 as *const _xmlNode,
            begin_pos:
            0 as libc::c_int as libc::c_ulong,
            begin_line:
            0 as libc::c_int as libc::c_ulong,
            end_pos: 0 as libc::c_int as libc::c_ulong,
            end_line:
            0 as libc::c_int as libc::c_ulong,
        };
        init
    };
    let mut failed: libc::c_int = 0;
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                         b"htmlParseElementInternal: context error\n\x00" as *const u8 as *const libc::c_char,
                         0 as *const xmlChar, 0 as *const xmlChar);
            return;
        }
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if ctxtPtr.instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
        return;
    }
    /* Capture start position */
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    if ctxtPtr.record_info != 0 {
        unsafe {
            node_info.begin_pos = inputPtr.consumed.wrapping_add(inputPtr.cur.offset_from(inputPtr.base)
                as libc::c_long as libc::c_ulong);
        }
        node_info.begin_line = inputPtr.line as libc::c_ulong
    }
    failed = htmlParseStartTag(ctxt);
    name = ctxtPtr.name;
    if failed == -(1 as libc::c_int) || name.is_null() {
        if CUR(ctxt) == '>' as i32 {
            xmlNextChar_safe(ctxt);
        }
        return;
    }
    /*
 * Lookup the info for that element.
 */
    info = htmlTagLookup(name);
    if info.is_null() {
        htmlParseErr(ctxt, XML_HTML_UNKNOWN_TAG,
                     b"Tag %s invalid\n\x00" as *const u8 as *const libc::c_char, name, 0 as *const xmlChar);
    }
    /*
 * Check for an Empty Element labeled the XML/SGML way
 */
    let mut sax_condition = false;
    if CUR(ctxt) == '/' as i32 && NXT(ctxt, 1 as libc::c_int) == '>' as i32 {
        SKIP(ctxt, 2 as libc::c_int);
        sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some()
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData,
                                          name);
        }
        htmlnamePop(ctxt);
        return;
    }
    if CUR(ctxt) == '>' as i32 {
        xmlNextChar_safe(ctxt);
    } else {
        htmlParseErr(ctxt, XML_ERR_GT_REQUIRED,
                     b"Couldn\'t find end of Start Tag %s\n\x00" as *const u8
                         as *const libc::c_char, name, 0 as *const xmlChar);
        /*
 * end of parsing of this node.
 */
        if xmlStrEqual_safe(name, ctxtPtr.name) != 0 {
            nodePop_safe(ctxt);
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
    let flag = unsafe { !info.is_null() && (*info).empty as libc::c_int != 0 };
    if flag {
        sax_condition = unsafe {
            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some()
        };
        if sax_condition {
            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
            xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData,
                                          name);
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
fn htmlParseContentInternal(mut ctxt: htmlParserCtxtPtr) {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut currentNode: *mut xmlChar = 0 as *mut xmlChar;
    let mut depth: libc::c_int = 0;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    currentNode = xmlStrdup_safe(ctxtPtr.name);
    depth = ctxtPtr.nameNr;
    loop {
        GROW(ctxt);
        if ctxtPtr.instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
            break;
        }
        /*
	 * Our tag or one of it's parent or children is ending.
	 */
        if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '/' as i32 {
            if htmlParseEndTag(ctxt) != 0 && (!currentNode.is_null() || ctxtPtr.nameNr == 0 as libc::c_int) {
                if !currentNode.is_null() {
                    xmlFree_safe(currentNode as *mut libc::c_void);
                }
                currentNode = xmlStrdup_safe(ctxtPtr.name);
                depth = ctxtPtr.nameNr
            }
            /* while */
        } else {
            if CUR(ctxt) == '<' as i32 && (IS_ASCII_LETTER(NXT(ctxt, 1 as libc::c_int)) || NXT(ctxt, 1) == '_' as i32 || NXT(ctxt, 1) == ':' as i32) {
                name = htmlParseHTMLName_nonInvasive(ctxt);
                if name.is_null() {
                    htmlParseErr(ctxt, XML_ERR_NAME_REQUIRED,
                                 b"htmlParseStartTag: invalid element name\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 0 as *const xmlChar, 0 as *const xmlChar);
                    /* Dump the bogus tag like browsers do */
                    while CUR(ctxt) == 0 as libc::c_int && CUR(ctxt) != '>' as i32 {
                        xmlNextChar_safe(ctxt);
                    }
                    htmlParserFinishElementParsing(ctxt);
                    if !currentNode.is_null() {
                        xmlFree_safe(currentNode as *mut libc::c_void);
                    }
                    currentNode = xmlStrdup_safe(ctxtPtr.name);
                    depth = ctxtPtr.nameNr;
                    continue;
                } else if !ctxtPtr.name.is_null() {
                    if htmlCheckAutoClose(name, ctxtPtr.name) == 1 as libc::c_int {
                        htmlAutoClose(ctxt, name);
                        continue;
                    }
                }
            }
            /*
	 * Has this node been popped out during parsing of
	 * the next element
	 */
            if ctxtPtr.nameNr > 0 as libc::c_int && depth >= ctxtPtr.nameNr && xmlStrEqual_safe(currentNode, ctxtPtr.name) == 0
            {
                htmlParserFinishElementParsing(ctxt);
                if !currentNode.is_null() {
                    xmlFree_safe(currentNode as *mut libc::c_void);
                }
                currentNode = xmlStrdup_safe(ctxtPtr.name);
                depth = ctxtPtr.nameNr
            } else {
                if CUR(ctxt) != 0 as libc::c_int && (xmlStrEqual_safe(currentNode,
                                                                      b"script\x00" as *const u8
                                                                          as *const libc::c_char
                                                                          as *mut xmlChar) != 0
                    || xmlStrEqual_safe(currentNode,
                                        b"style\x00" as *const u8 as *const libc::c_char
                                            as *mut xmlChar)
                    != 0) {
                    /*
	     * Handle SCRIPT/STYLE separately
	     */
                    htmlParseScript(ctxt);
                } else {
                    /*
	     * Sometimes DOCTYPE arrives in the middle of the document
	     */
                    if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '!' as i32 && UPP(ctxt, 2 as libc::c_int) == 'D' as i32 && UPP(ctxt, 3 as libc::c_int) == 'O' as i32 && UPP(ctxt, 4 as libc::c_int) == 'C' as i32 && UPP(ctxt, 5 as libc::c_int) == 'T' as i32 && UPP(ctxt, 6 as libc::c_int) == 'Y' as i32 && UPP(ctxt, 7 as libc::c_int) == 'P' as i32 && UPP(ctxt, 8 as libc::c_int) == 'E' as i32 {
                        htmlParseErr(ctxt, XML_HTML_STRUCURE_ERROR,
                                     b"Misplaced DOCTYPE declaration\n\x00" as *const u8 as *const libc::c_char,
                                     b"DOCTYPE\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                                     0 as *const xmlChar);
                        htmlParseDocTypeDecl(ctxt);
                    }
                    /*
	     * First case :  a comment
	     */
                    if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '!' as i32 && NXT(ctxt, 2 as libc::c_int) == '-' as i32 && NXT(ctxt, 3 as libc::c_int) == '-' as i32 {
                        htmlParseComment(ctxt);
                    } else if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '?' as i32 {
                        htmlParsePI(ctxt);
                    } else if CUR(ctxt) == '<' as i32 {
                        htmlParseElementInternal(ctxt);
                        if !currentNode.is_null() {
                            xmlFree_safe(currentNode
                                as *mut libc::c_void);
                        }
                        currentNode = xmlStrdup_safe(ctxtPtr.name);
                        depth = ctxtPtr.nameNr
                    } else if CUR(ctxt) == '&' as i32 {
                        htmlParseReference(ctxt);
                    } else if CUR(ctxt) == 0 as libc::c_int {
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
        xmlFree_safe(currentNode as *mut libc::c_void);
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

pub fn __htmlParseContent_htmlparser(mut ctxt: *mut libc::c_void) {
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
pub fn htmlParseDocument(mut ctxt:
                         htmlParserCtxtPtr)
                         -> libc::c_int {
    let mut start: [xmlChar; 4] = [0; 4];
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
    xmlInitParser_safe();
    htmlDefaultSAXHandlerInit_safe();
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                         b"htmlParseDocument: context error\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
            return XML_ERR_INTERNAL_ERROR as libc::c_int;
        }
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    ctxtPtr.html = 1 as libc::c_int;
    ctxtPtr.linenumbers = 1 as libc::c_int;
    GROW(ctxt);
    /*
 * SAX: beginning of the document processing.
 */
    let mut sax_condition = false;
    sax_condition = unsafe {
        !ctxtPtr.sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some()
    };
    if sax_condition {
        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
        xmlSAXHandler_setDocumentLocator_safe(saxPtr.setDocumentLocator, ctxtPtr.userData,
                                              __xmlDefaultSAXLocator_safe());
    }
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    unsafe {
        if ctxtPtr.encoding.is_null() && inputPtr.end.offset_from(inputPtr.cur) as libc::c_long >= 4 as libc::c_int as libc::c_long {
            /*
     * Get the 4 first bytes and decode the charset
     * if enc != XML_CHAR_ENCODING_NONE
     * plug some encoding conversion routines.
     */
            start[0 as libc::c_int as usize] = RAW(ctxt) as xmlChar;
            start[1 as libc::c_int as usize] = NXT(ctxt, 1 as libc::c_int) as xmlChar;
            start[2 as libc::c_int as usize] = NXT(ctxt, 2 as libc::c_int) as xmlChar;
            start[3 as libc::c_int as usize] = NXT(ctxt, 3 as libc::c_int) as xmlChar;
            enc = xmlDetectCharEncoding_safe(&mut *start.as_mut_ptr().offset(0
                as libc::c_int
                as isize),
                                             4 as libc::c_int);
            if enc as libc::c_int != XML_CHAR_ENCODING_NONE as libc::c_int {
                xmlSwitchEncoding_safe(ctxt, enc);
            }
        }
    }
    /*
 * Wipe out everything which is before the first '<'
 */
    htmlSkipBlankChars(ctxt);
    if CUR(ctxt) == 0 as libc::c_int {
        htmlParseErr(ctxt, XML_ERR_DOCUMENT_EMPTY,
                     b"Document is empty\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                     0 as *const xmlChar);
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
    while CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '!' as i32 && NXT(ctxt, 2 as libc::c_int) == '-' as i32 && NXT(ctxt, 3 as libc::c_int) == '-' as i32 || (CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '?' as i32) {
        htmlParseComment(ctxt);
        htmlParsePI(ctxt);
        htmlSkipBlankChars(ctxt);
    }
    /*
 * Then possibly doc type declaration(s) and more Misc
 * (doctypedecl Misc*)?
 */
    if CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '!' as i32 && UPP(ctxt, 2 as libc::c_int) == 'D' as i32 && UPP(ctxt, 3 as libc::c_int) == 'O' as i32 && UPP(ctxt, 4 as libc::c_int) == 'C' as i32 && UPP(ctxt, 5 as libc::c_int) == 'T' as i32 && UPP(ctxt, 6 as libc::c_int) == 'Y' as i32 && UPP(ctxt, 7 as libc::c_int) == 'P' as i32 && UPP(ctxt, 8 as libc::c_int) == 'E' as i32 {
        htmlParseDocTypeDecl(ctxt);
    }
    htmlSkipBlankChars(ctxt);
    /*
 * Parse possible comments and PIs before any content
 */
    while CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '!' as i32 && NXT(ctxt, 2 as libc::c_int) == '-' as i32 && NXT(ctxt, 3 as libc::c_int) == '-' as i32 || (CUR(ctxt) == '<' as i32 && NXT(ctxt, 1 as libc::c_int) == '?' as i32) {
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
    if CUR(ctxt) == 0 as libc::c_int {
        htmlAutoCloseOnEnd(ctxt);
    }
    /*
 * SAX: end of the document processing.
 */
    sax_condition = unsafe {
        !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endDocument.is_some()
    };
    if sax_condition {
        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
        xmlSAXHandler_endDocument_safe(saxPtr.endDocument, ctxtPtr.userData);
    }
    if ctxtPtr.options & HTML_PARSE_NODEFDTD as libc::c_int == 0 && !ctxtPtr.myDoc.is_null() {
        dtd = xmlGetIntSubset_safe(ctxtPtr.myDoc as *const xmlDoc);
        if dtd.is_null() {
            let mut myDocPtr = unsafe { &mut *(*ctxt).myDoc };
            myDocPtr.intSubset = xmlCreateIntSubset_safe(ctxtPtr.myDoc,
                                                         b"html\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                                                         b"-//W3C//DTD HTML 4.0 Transitional//EN\x00"
                                                             as *const u8 as *const libc::c_char as *mut xmlChar,
                                                         b"http://www.w3.org/TR/REC-html40/loose.dtd\x00"
                                                             as *const u8 as *const libc::c_char as *mut xmlChar)
        }
    }
    if ctxtPtr.wellFormed == 0 { return -(1 as libc::c_int); }
    return 0 as libc::c_int;
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
fn htmlInitParserCtxt(mut ctxt: htmlParserCtxtPtr)
                      -> libc::c_int {
    let mut sax: *mut htmlSAXHandler = 0 as *mut htmlSAXHandler;
    if ctxt.is_null() { return -(1 as libc::c_int); }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    memset_safe(ctxt as *mut libc::c_void, 0 as libc::c_int,
                ::std::mem::size_of::<htmlParserCtxt>() as libc::c_ulong);
    ctxtPtr.dict = xmlDictCreate_safe();
    if ctxtPtr.dict.is_null() {
        htmlErrMemory(0 as xmlParserCtxtPtr,
                      b"htmlInitParserCtxt: out of memory\n\x00" as *const u8
                          as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    sax = xmlMalloc_safe(::std::mem::size_of::<htmlSAXHandler>()
        as libc::c_ulong) as *mut htmlSAXHandler;
    if sax.is_null() {
        htmlErrMemory(0 as xmlParserCtxtPtr,
                      b"htmlInitParserCtxt: out of memory\n\x00" as *const u8
                          as *const libc::c_char);
        return -(1 as libc::c_int);
    } else {
        memset_safe(sax as *mut libc::c_void, 0 as libc::c_int,
                    ::std::mem::size_of::<htmlSAXHandler>() as libc::c_ulong);
    }
    /* Allocate the Input stack */
    ctxtPtr.inputTab = xmlMalloc_safe((5 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<htmlParserInputPtr>()
        as libc::c_ulong))
        as *mut htmlParserInputPtr;
    if ctxtPtr.inputTab.is_null() {
        htmlErrMemory(0 as xmlParserCtxtPtr,
                      b"htmlInitParserCtxt: out of memory\n\x00" as *const u8
                          as *const libc::c_char);
        ctxtPtr.inputNr = 0 as libc::c_int;
        ctxtPtr.inputMax = 0 as libc::c_int;
        ctxtPtr.input = 0 as xmlParserInputPtr;
        return -(1 as libc::c_int);
    }
    ctxtPtr.inputNr = 0 as libc::c_int;
    ctxtPtr.inputMax = 5 as libc::c_int;
    ctxtPtr.input = 0 as xmlParserInputPtr;
    ctxtPtr.version = 0 as *const xmlChar;
    ctxtPtr.encoding = 0 as *const xmlChar;
    ctxtPtr.standalone = -(1 as libc::c_int);
    ctxtPtr.instate = XML_PARSER_START;
    /* Allocate the Node stack */
    ctxtPtr.nodeTab = xmlMalloc_safe((10 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<htmlNodePtr>()
        as libc::c_ulong))
        as *mut htmlNodePtr;
    if ctxtPtr.nodeTab.is_null() {
        htmlErrMemory(0 as xmlParserCtxtPtr,
                      b"htmlInitParserCtxt: out of memory\n\x00" as *const u8
                          as *const libc::c_char);
        ctxtPtr.nodeNr = 0 as libc::c_int;
        ctxtPtr.nodeMax = 0 as libc::c_int;
        ctxtPtr.node = 0 as xmlNodePtr;
        ctxtPtr.inputNr = 0 as libc::c_int;
        ctxtPtr.inputMax = 0 as libc::c_int;
        ctxtPtr.input = 0 as xmlParserInputPtr;
        return -(1 as libc::c_int);
    }
    ctxtPtr.nodeNr = 0 as libc::c_int;
    ctxtPtr.nodeMax = 10 as libc::c_int;
    ctxtPtr.node = 0 as xmlNodePtr;
    /* Allocate the Name stack */
    ctxtPtr.nameTab = xmlMalloc_safe((10 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
        as libc::c_ulong))
        as *mut *const xmlChar;
    if ctxtPtr.nameTab.is_null() {
        htmlErrMemory(0 as xmlParserCtxtPtr,
                      b"htmlInitParserCtxt: out of memory\n\x00" as *const u8
                          as *const libc::c_char);
        ctxtPtr.nameNr = 0 as libc::c_int;
        ctxtPtr.nameMax = 0 as libc::c_int;
        ctxtPtr.name = 0 as *const xmlChar;
        ctxtPtr.nodeNr = 0 as libc::c_int;
        ctxtPtr.nodeMax = 0 as libc::c_int;
        ctxtPtr.node = 0 as xmlNodePtr;
        ctxtPtr.inputNr = 0 as libc::c_int;
        ctxtPtr.inputMax = 0 as libc::c_int;
        ctxtPtr.input = 0 as xmlParserInputPtr;
        return -(1 as libc::c_int);
    }
    ctxtPtr.nameNr = 0 as libc::c_int;
    ctxtPtr.nameMax = 10 as libc::c_int;
    ctxtPtr.name = 0 as *const xmlChar;
    ctxtPtr.nodeInfoTab = 0 as *mut xmlParserNodeInfo;
    ctxtPtr.nodeInfoNr = 0 as libc::c_int;
    ctxtPtr.nodeInfoMax = 0 as libc::c_int;
    if sax.is_null() {
        ctxtPtr.sax = __htmlDefaultSAXHandler_safe() as xmlSAXHandlerPtr
    } else {
        ctxtPtr.sax = sax;
        memcpy_safe(sax as *mut libc::c_void,
                    __htmlDefaultSAXHandler_safe() as *const libc::c_void,
                    ::std::mem::size_of::<xmlSAXHandlerV1>() as libc::c_ulong);
    }
    ctxtPtr.userData = ctxt as *mut libc::c_void;
    ctxtPtr.myDoc = 0 as xmlDocPtr;
    ctxtPtr.wellFormed = 1 as libc::c_int;
    ctxtPtr.replaceEntities = 0 as libc::c_int;
    ctxtPtr.linenumbers = __xmlLineNumbersDefaultValue_safe();
    ctxtPtr.keepBlanks = __xmlKeepBlanksDefaultValue_safe();
    ctxtPtr.html = 1 as libc::c_int;
    ctxtPtr.vctxt.finishDtd = 0xabcd1234 as libc::c_uint;
    ctxtPtr.vctxt.userData = ctxt as *mut libc::c_void;
    ctxtPtr.vctxt.error = Some(xmlParserValidityError as unsafe extern "C" fn(_: *mut libc::c_void,
                                                                              _: *const libc::c_char, _: ...) -> ());
    ctxtPtr.vctxt.warning = Some(xmlParserValidityWarning as unsafe extern "C" fn(_: *mut libc::c_void,
                                                                                  _: *const libc::c_char, _: ...) -> ());
    ctxtPtr.record_info = 0 as libc::c_int;
    ctxtPtr.validate = 0 as libc::c_int;
    ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
    ctxtPtr.catalogs = 0 as *mut libc::c_void;
    xmlInitNodeInfoSeq_safe(&mut ctxtPtr.node_seq);
    return 0 as libc::c_int;
}
/* *
 * htmlFreeParserCtxt:
 * @ctxt:  an HTML parser context
 *
 * Free all the memory used by a parser context. However the parsed
 * document in ctxt->myDoc is not freed.
 */

pub fn htmlFreeParserCtxt(mut ctxt:
                          htmlParserCtxtPtr) {
    xmlFreeParserCtxt_safe(ctxt);
}
/* *
 * htmlNewParserCtxt:
 *
 * Allocate and initialize a new parser context.
 *
 * Returns the htmlParserCtxtPtr or NULL in case of allocation error
 */

pub fn htmlNewParserCtxt()
    -> htmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    ctxt = xmlMalloc_safe(::std::mem::size_of::<xmlParserCtxt>()
        as libc::c_ulong) as xmlParserCtxtPtr;
    if ctxt.is_null() {
        htmlErrMemory(0 as xmlParserCtxtPtr,
                      b"NewParserCtxt: out of memory\n\x00" as *const u8 as *const libc::c_char);
        return 0 as htmlParserCtxtPtr;
    }
    memset_safe(ctxt as *mut libc::c_void, 0 as libc::c_int,
                ::std::mem::size_of::<xmlParserCtxt>() as libc::c_ulong);
    if htmlInitParserCtxt(ctxt) < 0 as libc::c_int {
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

pub fn htmlCreateMemoryParserCtxt_htmlparser(mut buffer:
                                             *const libc::c_char,
                                             mut size:
                                             libc::c_int)
                                             -> htmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if buffer.is_null() { return 0 as htmlParserCtxtPtr; }
    if size <= 0 as libc::c_int { return 0 as htmlParserCtxtPtr; }
    ctxt = htmlNewParserCtxt();
    if ctxt.is_null() { return 0 as htmlParserCtxtPtr; }
    buf = xmlParserInputBufferCreateMem_safe(buffer, size,
                                             XML_CHAR_ENCODING_NONE);
    if buf.is_null() { return 0 as htmlParserCtxtPtr; }
    input = xmlNewInputStream_safe(ctxt);
    if input.is_null() {
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as htmlParserCtxtPtr;
    }
    let mut inputPtr = unsafe { &mut (*input) };
    let mut bufPtr = unsafe { &mut (*buf) };
    inputPtr.filename = 0 as *const libc::c_char;
    inputPtr.buf = buf;
    xmlBufResetInput_safe(bufPtr.buffer, input);
    inputPush_safe(ctxt, input);
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
fn htmlCreateDocParserCtxt(mut cur: *const xmlChar,
                           mut encoding:
                           *const libc::c_char)
                           -> htmlParserCtxtPtr {
    let mut len: libc::c_int = 0;
    let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    if cur.is_null() { return 0 as htmlParserCtxtPtr; }
    len = xmlStrlen_safe(cur);
    ctxt = htmlCreateMemoryParserCtxt_safe(cur as *mut libc::c_char,
                                           len);
    if ctxt.is_null() { return 0 as htmlParserCtxtPtr; }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        let mut handler: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        if !inputPtr.encoding.is_null() {
            xmlFree_safe(inputPtr.encoding
                as *mut xmlChar as *mut libc::c_void);
        }
        inputPtr.encoding = xmlStrdup_safe(encoding as *const xmlChar);
        enc = xmlParseCharEncoding_safe(encoding);
        /*
	 * registered set of known encodings
	 */
        if enc as libc::c_int != XML_CHAR_ENCODING_ERROR as libc::c_int {
            xmlSwitchEncoding_safe(ctxt, enc);
            if ctxtPtr.errNo == XML_ERR_UNSUPPORTED_ENCODING as libc::c_int {
                htmlParseErr(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                             b"Unsupported encoding %s\n\x00" as *const u8 as *const libc::c_char,
                             encoding as *const xmlChar, 0 as *const xmlChar);
            }
        } else {
            /*
	     * fallback for unknown encodings
	     */
            handler = xmlFindCharEncodingHandler_safe(encoding);
            if !handler.is_null() {
                xmlSwitchToEncoding_safe(ctxt, handler);
            } else {
                htmlParseErr(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                             b"Unsupported encoding %s\n\x00" as *const u8 as *const libc::c_char,
                             encoding as *const xmlChar, 0 as *const xmlChar);
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
fn htmlParseLookupSequence(mut ctxt: htmlParserCtxtPtr,
                           mut first: xmlChar,
                           mut next: xmlChar,
                           mut third: xmlChar,
                           mut ignoreattrval: libc::c_int)
                           -> libc::c_int {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut DEBUG_PUSH: libc::c_int = 0 as libc::c_int;
    match () {
        #[cfg(DEBUG_PUSH)]      _ => {
            DEBUG_PUSH = 1 as libc::c_int;
        }
        #[cfg(not(DEBUG_PUSH))]      _ => {}
    };
    let mut base: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut in_0: htmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut buf: *const xmlChar = 0 as *const xmlChar;
    let mut invalue: libc::c_int = 0 as libc::c_int;
    let mut valdellim: libc::c_char = 0 as libc::c_int as libc::c_char;
    in_0 = ctxtPtr.input;
    if in_0.is_null() { return -(1 as libc::c_int); }
    let mut in_0Ptr = unsafe { &mut *in_0 };
    unsafe {
        base = in_0Ptr.cur.offset_from(in_0Ptr.base) as libc::c_long as libc::c_int;
    }
    if base < 0 as libc::c_int { return -(1 as libc::c_int); }
    if ctxtPtr.checkIndex > base as libc::c_long {
        base = ctxtPtr.checkIndex as libc::c_int;
        if (ctxtPtr.hasPErefs & 1 as libc::c_int) == 0 {
            invalue = 0 as libc::c_int;
        } else {
            invalue = 1 as libc::c_int;
        }
    }
    if in_0Ptr.buf.is_null() {
        buf = in_0Ptr.base;
        len = in_0Ptr.length;
    } else {
        let mut in_0BufPtr = unsafe { &mut *(*in_0).buf };
        buf = xmlBufContent_safe(in_0BufPtr.buffer as *const xmlBuf);
        len = xmlBufUse_safe(in_0BufPtr.buffer) as libc::c_int;
    }
    /* take into account the sequence length */
    if third != 0 { len -= 2 as libc::c_int; } else if next != 0 { len -= 1; }
    while base < len {
        if ignoreattrval != 0 {
            unsafe {
                if *buf.offset(base as isize) as libc::c_int == '\"' as i32 || *buf.offset(base as isize) as libc::c_int == '\'' as i32 {
                    if invalue != 0 {
                        if *buf.offset(base as isize) as libc::c_int == valdellim as libc::c_int {
                            invalue = 0 as libc::c_int;
                            base += 1 as libc::c_int;
                            continue;
                        }
                    } else {
                        valdellim = *buf.offset(base as isize) as libc::c_char;
                        invalue = 1 as libc::c_int;
                        base += 1 as libc::c_int;
                        continue;
                    }
                } else if invalue != 0 {
                    base += 1 as libc::c_int;
                    continue;
                }
            }
        }

        unsafe {
            if *buf.offset(base as isize) as libc::c_int == first as libc::c_int {
                if third as libc::c_int != 0 as libc::c_int {
                    if *buf.offset((base + 1 as libc::c_int) as isize) as libc::c_int != next as libc::c_int || *buf.offset((base + 2 as libc::c_int) as isize)
                        as libc::c_int != third as libc::c_int {
                        base += 1 as libc::c_int;
                        continue;
                    }
                } else if next as libc::c_int != 0 as libc::c_int {
                    if *buf.offset((base + 1 as libc::c_int) as isize) as libc::c_int != next as libc::c_int {
                        base += 1 as libc::c_int;
                        continue;
                    }
                }
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 {
                    if next as libc::c_int == 0 as libc::c_int {
                        __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: lookup '%c' found at %d" as *const u8 as *const libc::c_char,
                                                                                   first as libc::c_uint, base as libc::c_uint);
                    } else if third as libc::c_int == 0 as libc::c_int {
                        __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: lookup '%c%c' found at %d\n" as *const u8 as *const libc::c_char,
                                                                                   first as libc::c_uint, next as libc::c_uint, base as libc::c_uint);
                    } else {
                        __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: lookup '%c%c%c' found at %d\n" as *const u8 as *const libc::c_char,
                                                                                   first as libc::c_uint, next as libc::c_uint,
                                                                                   third as libc::c_uint, base as libc::c_uint);
                    }
                }
                return (base as libc::c_long -
                    in_0Ptr.cur.offset_from(in_0Ptr.base)
                        as libc::c_long) as libc::c_int;
            }
        }
        base += 1 as libc::c_int;
    }
    ctxtPtr.checkIndex = base as libc::c_long;
    if invalue != 0 {
        ctxtPtr.hasPErefs = ctxtPtr.hasPErefs | (1 as libc::c_int);
    } else {
        ctxtPtr.hasPErefs = ctxtPtr.hasPErefs & (-2 as libc::c_int);
    }
    if DEBUG_PUSH != 0 {
        if next as libc::c_int == 0 as libc::c_int {
            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                           b"HPP: lookup '%c' failed\n" as *const u8 as *const libc::c_char, first as libc::c_uint);
        } else if third as libc::c_int == 0 as libc::c_int {
            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                           b"HPP: lookup '%c%c' failed\n" as *const u8 as *const libc::c_char, first as libc::c_uint, next as libc::c_uint);
        } else {
            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                       b"HPP: lookup '%c%c%c' failed\n" as *const u8 as *const libc::c_char, first as libc::c_uint, next as libc::c_uint,
                                                                       third as libc::c_uint);
        }
    }
    return -(1 as libc::c_int);
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
fn htmlParseLookupCommentEnd(mut ctxt: htmlParserCtxtPtr)
                             -> libc::c_int {
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut inputPtr = unsafe { &mut *(*ctxt).input };
    let mut mark: libc::c_int = 0 as libc::c_int;
    let mut cur: libc::c_int = unsafe { inputPtr.cur.offset_from(inputPtr.base) as libc::c_long as libc::c_int };
    while mark >= 0 as libc::c_int {
        mark = htmlParseLookupSequence(ctxt, '-' as i32 as xmlChar,
                                       '-' as i32 as xmlChar,
                                       0 as libc::c_int as xmlChar,
                                       0 as libc::c_int);
        if (mark < 0 as libc::c_int) || (NXT(ctxt, mark + 2 as libc::c_int) == '>' as i32) || ((NXT(ctxt, mark + 2 as libc::c_int) == '!' as i32) && (NXT(ctxt, mark + 3 as libc::c_int) == '>' as i32)) {
            return mark;
        }
        ctxtPtr.checkIndex = (cur + mark + 1 as libc::c_int) as libc::c_long
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
fn htmlParseTryOrFinish(mut ctxt: htmlParserCtxtPtr,
                        mut terminate: libc::c_int)
                        -> libc::c_int {
    let mut DEBUG_PUSH: libc::c_int = 0 as libc::c_int;
    match () {
        #[cfg(DEBUG_PUSH)]      _ => {
            DEBUG_PUSH = 1 as libc::c_int;
        }
        #[cfg(not(DEBUG_PUSH))]      _ => {}
    };
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut in_0: htmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut avail: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
    let mut cur: xmlChar = 0;
    let mut next: xmlChar = 0;
    let mut node_info: htmlParserNodeInfo = htmlParserNodeInfo {
        node: 0 as *const _xmlNode,
        begin_pos: 0,
        begin_line: 0,
        end_pos: 0,
        end_line: 0,
    };
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if DEBUG_PUSH != 0 as libc::c_int {
        match ctxtPtr.instate {
            XML_PARSER_EOF => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try EOF\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_START => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try START\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_MISC => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try MISC\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_COMMENT => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try COMMENT\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_PROLOG => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try PROLOG\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_START_TAG => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try START_TAG\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_CONTENT => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try CONTENT\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_CDATA_SECTION => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try CDATA_SECTION\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_END_TAG => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try END_TAG\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_ENTITY_DECL => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try ENTITY_DECL\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_ENTITY_VALUE => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try ENTITY_VALUE\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_ATTRIBUTE_VALUE => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try ATTRIBUTE_VALUE\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_DTD => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try DTD\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_EPILOG => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try EPILOG\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_PI => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try PI\n" as *const u8 as *const libc::c_char);
            }
            XML_PARSER_SYSTEM_LITERAL => {
                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                               b"HPP: try SYSTEM_LITERAL\n" as *const u8 as *const libc::c_char);
            }
            _ => {}
        }
    }
    let mut sax_condition = false;
    loop {
        in_0 = ctxtPtr.input;
        if in_0.is_null() { break; }
        let mut in_0Ptr = unsafe { &mut *in_0 };
        if in_0Ptr.buf.is_null() {
            unsafe {
                avail = in_0Ptr.length as libc::c_long -
                    in_0Ptr.cur.offset_from(in_0Ptr.base) as libc::c_long
            }
        } else {
            unsafe {
                avail = xmlBufUse_safe((*(*in_0).buf).buffer) as ptrdiff_t
                    -
                    in_0Ptr.cur.offset_from(in_0Ptr.base) as libc::c_long
            }
        }
        if avail == 0 as libc::c_int as libc::c_long && terminate != 0 {
            htmlAutoCloseOnEnd(ctxt);
            if ctxtPtr.nameNr == 0 as libc::c_int && ctxtPtr.instate as libc::c_int != XML_PARSER_EOF as libc::c_int {
                /*
		 * SAX: end of the document processing.
		 */
                ctxtPtr.instate = XML_PARSER_EOF;
                sax_condition = unsafe {
                    !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endDocument.is_some()
                };
                if sax_condition {
                    let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                    xmlSAXHandler_endDocument_safe(saxPtr.endDocument, ctxtPtr.userData);
                }
            }
        }
        if avail < 1 as libc::c_int as libc::c_long { break; }
        /*
         * This is done to make progress and avoid an infinite loop
         * if a parsing attempt was aborted by hitting a NUL byte. After
         * changing htmlCurrentChar, this probably isn't necessary anymore.
         * We should consider removing this check.
         */
        unsafe { cur = *in_0Ptr.cur.offset(0 as libc::c_int as isize); }
        if cur as libc::c_int == 0 as libc::c_int {
            SKIP(ctxt, 1 as libc::c_int);
            continue;
        }
        match ctxtPtr.instate {
            XML_PARSER_EOF => { //Document parsing is done !
                break;
            }
            XML_PARSER_START => {//Very first chars read from the document flow
                unsafe { cur = *in_0Ptr.cur.offset(0 as libc::c_int as isize); }
                if IS_BLANK_CH(cur as libc::c_int) {
                    htmlSkipBlankChars(ctxt);
                    if in_0Ptr.buf.is_null() {
                        unsafe {
                            avail = in_0Ptr.length as libc::c_long -
                                in_0Ptr.cur.offset_from(in_0Ptr.base)
                                    as libc::c_long
                        }
                    } else {
                        unsafe {
                            avail = xmlBufUse_safe((*(*in_0).buf).buffer)
                                as ptrdiff_t -
                                in_0Ptr.cur.offset_from(in_0Ptr.base)
                                    as libc::c_long
                        }
                    }
                }
                sax_condition = unsafe {
                    !ctxtPtr.sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some()
                };
                if sax_condition {
                    let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                    xmlSAXHandler_setDocumentLocator_safe(saxPtr.setDocumentLocator, ctxtPtr.userData,
                                                          __xmlDefaultSAXLocator_safe());
                }
                sax_condition = unsafe {
                    !ctxtPtr.sax.is_null() && (*(*ctxt).sax).startDocument.is_some() && ctxtPtr.disableSAX == 0
                };
                if sax_condition {
                    let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                    xmlSAXHandler_startDocument_safe(saxPtr.startDocument, ctxtPtr.userData);
                }
                unsafe { cur = *in_0Ptr.cur.offset(0 as libc::c_int as isize); }
                unsafe { next = *in_0Ptr.cur.offset(1 as libc::c_int as isize); }
                if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && UPP(ctxt, 2 as libc::c_int) == 'D' as i32 && UPP(ctxt, 3 as libc::c_int) == 'O' as i32 && UPP(ctxt, 4 as libc::c_int) == 'C' as i32 && UPP(ctxt, 5 as libc::c_int) == 'T' as i32 && UPP(ctxt, 6 as libc::c_int) == 'Y' as i32 && UPP(ctxt, 7 as libc::c_int) == 'P' as i32 && UPP(ctxt, 8 as libc::c_int) == 'E' as i32 {
                    if terminate == 0 && htmlParseLookupSequence(ctxt,
                                                                 '>' as i32 as xmlChar,
                                                                 0 as libc::c_int as xmlChar,
                                                                 0 as libc::c_int as xmlChar,
                                                                 1 as libc::c_int) <
                        0 as libc::c_int {
                        break;
                    }
                    if DEBUG_PUSH != 0 as libc::c_int {
                        unsafe {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                       b"HPP: Parsing internal subset\n" as *const u8 as *const libc::c_char);
                        }
                    }
                    htmlParseDocTypeDecl(ctxt);
                    ctxtPtr.instate = XML_PARSER_PROLOG;
                    if DEBUG_PUSH != 0 as libc::c_int {
                        __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                       b"HPP: entering PROLOG\n" as *const u8 as *const libc::c_char);
                    }
                } else {
                    ctxtPtr.instate = XML_PARSER_MISC;
                    if DEBUG_PUSH != 0 as libc::c_int {
                        __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                       b"HPP: entering MISC\n" as *const u8 as *const libc::c_char);
                    }
                }
            }
            XML_PARSER_MISC => {
                htmlSkipBlankChars(ctxt);
                if in_0Ptr.buf.is_null() {
                    unsafe {
                        avail = in_0Ptr.length as libc::c_long -
                            in_0Ptr.cur.offset_from(in_0Ptr.base)
                                as libc::c_long
                    }
                } else {
                    unsafe {
                        avail = xmlBufUse_safe((*(*in_0).buf).buffer)
                            as ptrdiff_t -
                            in_0Ptr.cur.offset_from(in_0Ptr.base)
                                as libc::c_long
                    }
                }
                if avail < 1 as libc::c_int as libc::c_long { break; }
                if avail < 2 as libc::c_int as libc::c_long {
                    if terminate == 0 {
                        break;
                    } else {
                        next = ' ' as i32 as xmlChar;
                    }
                } else {
                    unsafe { next = *in_0Ptr.cur.offset(1 as libc::c_int as isize) }
                }
                unsafe { cur = *in_0Ptr.cur.offset(0 as libc::c_int as isize); }
                unsafe {
                    if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && *in_0Ptr.cur.offset(2 as libc::c_int as isize) as libc::c_int == '-' as i32 && *in_0Ptr.cur.offset(3 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                        if terminate == 0 && htmlParseLookupCommentEnd(ctxt) <
                            0 as libc::c_int {
                            break;
                        }
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: Parsing Comment\n" as *const u8 as *const libc::c_char);
                        }
                        htmlParseComment(ctxt);
                        ctxtPtr.instate = XML_PARSER_MISC;
                    } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '?' as i32 {
                        if terminate == 0 && htmlParseLookupSequence(ctxt,
                                                                     '>' as i32 as xmlChar,
                                                                     0 as libc::c_int as xmlChar,
                                                                     0 as libc::c_int as xmlChar,
                                                                     0 as libc::c_int) <
                            0 as libc::c_int {
                            break;
                        }
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: Parsing PI\n" as *const u8 as *const libc::c_char);
                        }
                        htmlParsePI(ctxt);
                        ctxtPtr.instate = XML_PARSER_MISC;
                    } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && UPP(ctxt, 2 as libc::c_int) == 'D' as i32 && UPP(ctxt, 3 as libc::c_int) == 'O' as i32 && UPP(ctxt, 4 as libc::c_int) == 'C' as i32 && UPP(ctxt, 5 as libc::c_int) == 'T' as i32 && UPP(ctxt, 6 as libc::c_int) == 'Y' as i32 && UPP(ctxt, 7 as libc::c_int) == 'P' as i32 && UPP(ctxt, 8 as libc::c_int) == 'E' as i32 {
                        if terminate == 0 && htmlParseLookupSequence(ctxt,
                                                                     '>' as i32 as xmlChar,
                                                                     0 as libc::c_int as xmlChar,
                                                                     0 as libc::c_int as xmlChar,
                                                                     1 as libc::c_int) <
                            0 as libc::c_int {
                            break;
                        }
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: Parsing internal subset\n" as *const u8 as *const libc::c_char);
                        }
                        htmlParseDocTypeDecl(ctxt);
                        ctxtPtr.instate = XML_PARSER_PROLOG;
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: entering PROLOG\n" as *const u8 as *const libc::c_char);
                        }
                    } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && avail < 9 as libc::c_int as libc::c_long {
                        break;
                    } else {
                        ctxtPtr.instate = XML_PARSER_CONTENT;
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: entering START_TAG\n" as *const u8 as *const libc::c_char);
                        }
                    }
                }
            }
            XML_PARSER_PROLOG => {
                htmlSkipBlankChars(ctxt);
                if in_0Ptr.buf.is_null() {
                    unsafe {
                        avail = in_0Ptr.length as libc::c_long -
                            in_0Ptr.cur.offset_from(in_0Ptr.base)
                                as libc::c_long
                    }
                } else {
                    unsafe {
                        avail = xmlBufUse_safe((*(*in_0).buf).buffer)
                            as ptrdiff_t -
                            in_0Ptr.cur.offset_from(in_0Ptr.base)
                                as libc::c_long
                    }
                }
                if avail < 2 as libc::c_int as libc::c_long { break; }
                unsafe { cur = *in_0Ptr.cur.offset(0 as libc::c_int as isize); }
                unsafe { next = *in_0Ptr.cur.offset(1 as libc::c_int as isize); }
                unsafe {
                    if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && *in_0Ptr.cur.offset(2 as libc::c_int as isize) as libc::c_int == '-' as i32 && *in_0Ptr.cur.offset(3 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                        if terminate == 0 && htmlParseLookupCommentEnd(ctxt) <
                            0 as libc::c_int {
                            break;
                        }
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: Parsing Comment\n" as *const u8 as *const libc::c_char);
                        }
                        htmlParseComment(ctxt);
                        ctxtPtr.instate = XML_PARSER_PROLOG;
                    } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '?' as i32 {
                        if terminate == 0 && htmlParseLookupSequence(ctxt,
                                                                     '>' as i32 as xmlChar,
                                                                     0 as libc::c_int as xmlChar,
                                                                     0 as libc::c_int as xmlChar,
                                                                     0 as libc::c_int) <
                            0 as libc::c_int {
                            break;
                        }
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: Parsing PI\n" as *const u8 as *const libc::c_char);
                        }
                        htmlParsePI(ctxt);
                        ctxtPtr.instate = XML_PARSER_PROLOG;
                    } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && avail < 4 as libc::c_int as libc::c_long {
                        break;
                    } else {
                        ctxtPtr.instate = XML_PARSER_CONTENT;
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: entering START_TAG\n" as *const u8 as *const libc::c_char);
                        }
                    }
                }
            }
            XML_PARSER_EPILOG => {
                if in_0Ptr.buf.is_null() {
                    unsafe {
                        avail = in_0Ptr.length as libc::c_long -
                            in_0Ptr.cur.offset_from(in_0Ptr.base)
                                as libc::c_long
                    }
                } else {
                    unsafe {
                        avail = xmlBufUse_safe((*(*in_0).buf).buffer)
                            as ptrdiff_t -
                            in_0Ptr.cur.offset_from(in_0Ptr.base)
                                as libc::c_long
                    }
                }
                if avail < 1 as libc::c_int as libc::c_long { break; }
                unsafe { cur = *in_0Ptr.cur.offset(0 as libc::c_int as isize); }
                if IS_BLANK_CH(cur as libc::c_int) {
                    htmlParseCharData(ctxt);
                    break;
                }
                if avail < 2 as libc::c_int as libc::c_long { break; }
                unsafe { next = *in_0Ptr.cur.offset(1 as libc::c_int as isize); }
                unsafe {
                    if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && *in_0Ptr.cur.offset(2 as libc::c_int as isize)
                        as libc::c_int == '-' as i32 && *in_0Ptr.cur.offset(3 as libc::c_int as isize)
                        as libc::c_int == '-' as i32 {
                        if terminate == 0 && htmlParseLookupCommentEnd(ctxt) <
                            0 as libc::c_int {
                            break;
                        }
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: Parsing Comment\n" as *const u8 as *const libc::c_char);
                        }
                        htmlParseComment(ctxt);
                        ctxtPtr.instate = XML_PARSER_EPILOG;
                    } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '?' as i32 {
                        if terminate == 0 && htmlParseLookupSequence(ctxt,
                                                                     '>' as i32 as xmlChar,
                                                                     0 as libc::c_int as xmlChar,
                                                                     0 as libc::c_int as xmlChar,
                                                                     0 as libc::c_int) <
                            0 as libc::c_int {
                            break;
                        }
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: Parsing PI\n" as *const u8 as *const libc::c_char);
                        }
                        htmlParsePI(ctxt);
                        ctxtPtr.instate = XML_PARSER_EPILOG;
                    } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && avail < 4 as libc::c_int as libc::c_long {
                        break;
                    } else {
                        ctxtPtr.errNo = XML_ERR_DOCUMENT_END as libc::c_int;
                        ctxtPtr.wellFormed = 0 as libc::c_int;
                        ctxtPtr.instate = XML_PARSER_EOF;
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: entering EOF\n" as *const u8 as *const libc::c_char);
                        }
                        sax_condition = unsafe {
                            !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endDocument.is_some()
                        };
                        if sax_condition {
                            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                            xmlSAXHandler_endDocument_safe(saxPtr.endDocument, ctxtPtr.userData);
                        }
                        break;
                    }
                }
            }
            XML_PARSER_START_TAG => {
                let mut name: *const xmlChar = 0 as *const xmlChar;
                let mut failed: libc::c_int = 0;
                let mut info: *const htmlElemDesc = 0 as *const htmlElemDesc;
                if avail < 1 as libc::c_int as libc::c_long { break; }
                if avail < 2 as libc::c_int as libc::c_long {
                    if terminate == 0 { break; } else {
                        next = ' ' as i32 as xmlChar;
                    }
                } else {
                    unsafe { next = *in_0Ptr.cur.offset(1 as libc::c_int as isize) }
                }
                unsafe { cur = *in_0Ptr.cur.offset(0 as libc::c_int as isize); }
                if cur as libc::c_int != '<' as i32 {
                    ctxtPtr.instate = XML_PARSER_CONTENT;
                    if DEBUG_PUSH != 0 as libc::c_int {
                        __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                       b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                    }
                    continue;
                }
                if next as libc::c_int == '/' as i32 {
                    ctxtPtr.instate = XML_PARSER_END_TAG;
                    ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                    if DEBUG_PUSH != 0 as libc::c_int {
                        __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                       b"HPP: entering END_TAG\n" as *const u8 as *const libc::c_char);
                    }
                    continue;
                }
                if terminate == 0 && htmlParseLookupSequence(ctxt,
                                                             '>' as i32 as xmlChar,
                                                             0 as libc::c_int as xmlChar,
                                                             0 as libc::c_int as xmlChar,
                                                             1 as libc::c_int) <
                    0 as libc::c_int {
                    break;
                }
                /* Capture start position */
                let mut inputPtr = unsafe { &mut *(*ctxt).input };
                if ctxtPtr.record_info != 0 {
                    unsafe {
                        node_info.begin_pos = inputPtr.consumed.wrapping_add(
                            inputPtr.cur.offset_from(inputPtr.base)
                                as libc::c_long
                                as libc::c_ulong);
                    }
                    node_info.begin_line = inputPtr.line as libc::c_ulong;
                }
                failed = htmlParseStartTag(ctxt);
                name = ctxtPtr.name;
                if failed == -(1 as libc::c_int) || name.is_null() {
                    if CUR(ctxt) == '>' as i32 {
                        xmlNextChar_safe(ctxt);
                    }
                    continue;
                }
                /*
* Lookup the info for that element.
*/
                info = htmlTagLookup(name);
                if info.is_null() {
                    htmlParseErr(ctxt, XML_HTML_UNKNOWN_TAG,
                                 b"Tag %s invalid\n\x00" as *const u8 as *const libc::c_char, name,
                                 0 as *const xmlChar);
                }
                if CUR(ctxt) == '/' as i32 && NXT(ctxt, 1 as libc::c_int) == '>' as i32 {
                    SKIP(ctxt, 2 as libc::c_int);
                    sax_condition = unsafe {
                        !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some()
                    };
                    if sax_condition {
                        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                        xmlSAXHandler_endElement_safe(saxPtr.endElement, ctxtPtr.userData, name);
                    }
                    htmlnamePop(ctxt);
                    ctxtPtr.instate = XML_PARSER_CONTENT;
                    if DEBUG_PUSH != 0 as libc::c_int {
                        __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                       b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                    }
                    continue;
                }
                if CUR(ctxt) == '>' as i32 {
                    xmlNextChar_safe(ctxt);
                } else {
                    htmlParseErr(ctxt, XML_ERR_GT_REQUIRED,
                                 b"Couldn\'t find end of Start Tag %s\n\x00"
                                     as *const u8 as *const libc::c_char, name,
                                 0 as *const xmlChar);
                    /*
 * end of parsing of this node.
 */
                    if xmlStrEqual_safe(name, ctxtPtr.name) != 0 {
                        nodePop_safe(ctxt);
                        htmlnamePop(ctxt);
                    }
                    if ctxtPtr.record_info != 0 {
                        htmlNodeInfoPush(ctxt, &mut node_info);
                    }
                    ctxtPtr.instate = XML_PARSER_CONTENT;
                    if DEBUG_PUSH != 0 as libc::c_int {
                        __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                       b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                    }
                    continue;
                }
                let flag = unsafe { !info.is_null() && (*info).empty as libc::c_int != 0 };
                if flag {
                    sax_condition = unsafe {
                        !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endElement.is_some()
                    };
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
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_CONTENT => {
                let mut chr: [xmlChar; 2] = [0 as libc::c_int as xmlChar,
                    0 as libc::c_int as xmlChar];
                if ctxtPtr.token != 0 as libc::c_int {
                    chr[0 as libc::c_int as usize] = ctxtPtr.token as xmlChar;
                    htmlCheckParagraph(ctxt);
                    sax_condition = unsafe {
                        !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some()
                    };
                    if sax_condition {
                        let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                        xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                                      chr.as_mut_ptr(),
                                                      1
                                                          as libc::c_int);
                    }
                    ctxtPtr.token = 0 as libc::c_int;
                    ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long
                }
                if avail == 1 as libc::c_int as libc::c_long && terminate != 0 {
                    unsafe { cur = *in_0Ptr.cur.offset(0 as libc::c_int as isize); }
                    if cur as libc::c_int != '<' as i32 && cur as libc::c_int != '&' as i32 {
                        if !ctxtPtr.sax.is_null() {
                            let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                            chr[0 as libc::c_int as usize] = cur;
                            if IS_BLANK_CH(cur as libc::c_int) {
                                if ctxtPtr.keepBlanks != 0 {
                                    if saxPtr.characters.is_some() {
                                        xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                                                      chr.as_mut_ptr(),
                                                                      1
                                                                          as libc::c_int);
                                    }
                                } else {
                                    if saxPtr.ignorableWhitespace.is_some() {
                                        xmlSAXHandler_ignorableWhitespace_safe(saxPtr.ignorableWhitespace, ctxtPtr.userData,
                                                                               chr.as_mut_ptr(),
                                                                               1
                                                                                   as libc::c_int);
                                    }
                                }
                            } else {
                                htmlCheckParagraph(ctxt);
                                if saxPtr.characters.is_some() {
                                    xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                                                  chr.as_mut_ptr(),
                                                                  1
                                                                      as libc::c_int);
                                }
                            }
                        }
                        ctxtPtr.token = 0 as libc::c_int;
                        ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                        unsafe { in_0Ptr.cur = in_0Ptr.cur.offset(1); }
                        continue;
                    }
                }
                if avail < 2 as libc::c_int as libc::c_long { break; }
                unsafe { cur = *in_0Ptr.cur.offset(0 as libc::c_int as isize); }
                unsafe { next = *in_0Ptr.cur.offset(1 as libc::c_int as isize); }
                //cons = (*ctxt).nbChars;
                if xmlStrEqual_safe(ctxtPtr.name,
                                    b"script\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 || xmlStrEqual_safe(ctxtPtr.name,
                                                                                                                                b"style\x00" as *const u8 as *const libc::c_char as *mut xmlChar) != 0 {
                    /*
         * Handle SCRIPT/STYLE separately
         */
                    if terminate == 0 {
                        let mut idx: libc::c_int = 0;
                        let mut val: xmlChar = 0;
                        idx = htmlParseLookupSequence(ctxt,
                                                      '<' as i32 as xmlChar,
                                                      '/' as i32 as xmlChar,
                                                      0 as libc::c_int as xmlChar,
                                                      0 as libc::c_int);
                        if idx < 0 as libc::c_int { break; }
                        unsafe {
                            val = *in_0Ptr.cur.offset((idx + 2 as libc::c_int)
                                as isize);
                        }
                        if val as libc::c_int == 0 as libc::c_int {
                            break;
                        }
                    }
                    htmlParseScript(ctxt);
                    if cur as libc::c_int == '<' as i32 && next as libc::c_int == '/' as i32 {
                        ctxtPtr.instate = XML_PARSER_END_TAG;
                        ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                        if DEBUG_PUSH != 0 as libc::c_int {
                            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                           b"HPP: entering END_TAG\n" as *const u8 as *const libc::c_char);
                        }
                        continue;
                    }
                } else {
                    unsafe {
                        if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && UPP(ctxt, 2 as libc::c_int) == 'D' as i32 && UPP(ctxt, 3 as libc::c_int) == 'O' as i32 && UPP(ctxt, 4 as libc::c_int) == 'C' as i32 && UPP(ctxt, 5 as libc::c_int) == 'T' as i32 && UPP(ctxt, 6 as libc::c_int) == 'Y' as i32 && UPP(ctxt, 7 as libc::c_int) == 'P' as i32 && UPP(ctxt, 8 as libc::c_int) == 'E' as i32 {
                            if terminate == 0 && htmlParseLookupSequence(ctxt,
                                                                         '>' as i32 as xmlChar,
                                                                         0 as libc::c_int as xmlChar,
                                                                         0 as libc::c_int as xmlChar,
                                                                         1 as libc::c_int) <
                                0 as libc::c_int {
                                break;
                            }
                            htmlParseErr(ctxt, XML_HTML_STRUCURE_ERROR,
                                         b"Misplaced DOCTYPE declaration\n\x00" as *const u8 as *const libc::c_char,
                                         b"DOCTYPE\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                                         0 as *const xmlChar);
                            htmlParseDocTypeDecl(ctxt);
                        } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && *in_0Ptr.cur.offset(2 as libc::c_int as isize) as libc::c_int == '-' as i32 && *in_0Ptr.cur.offset(3 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                            if terminate == 0 && htmlParseLookupCommentEnd(ctxt) <
                                0 as libc::c_int {
                                break;
                            }
                            if DEBUG_PUSH != 0 as libc::c_int {
                                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                               b"HPP: Parsing Comment\n" as *const u8 as *const libc::c_char);
                            }
                            htmlParseComment(ctxt);
                            ctxtPtr.instate = XML_PARSER_CONTENT;
                        } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '?' as i32 {
                            if terminate == 0 && htmlParseLookupSequence(ctxt,
                                                                         '>' as i32 as xmlChar,
                                                                         0 as libc::c_int as xmlChar,
                                                                         0 as libc::c_int as xmlChar,
                                                                         0 as libc::c_int) <
                                0 as libc::c_int {
                                break;
                            }
                            if DEBUG_PUSH != 0 as libc::c_int {
                                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                               b"HPP: Parsing PI\n" as *const u8 as *const libc::c_char);
                            }
                            htmlParsePI(ctxt);
                            ctxtPtr.instate = XML_PARSER_CONTENT;
                        } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '!' as i32 && avail < 4 as libc::c_int as libc::c_long {
                            break;
                        } else if cur as libc::c_int == '<' as i32 && next as libc::c_int == '/' as i32 {
                            ctxtPtr.instate = XML_PARSER_END_TAG;
                            ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                            if DEBUG_PUSH != 0 as libc::c_int {
                                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                               b"HPP: entering END_TAG\n" as *const u8 as *const libc::c_char);
                            }
                            continue;
                        } else if cur as libc::c_int == '<' as i32 {
                            if terminate == 0 && next as libc::c_int == 0 {
                                break;
                            }
                            if IS_ASCII_LETTER(next as libc::c_int) || next as libc::c_int == '_' as i32 || next as libc::c_int == ':' as i32 || next as libc::c_int == '.' as i32 {
                                ctxtPtr.instate = XML_PARSER_START_TAG;
                                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                                if DEBUG_PUSH != 0 as libc::c_int {
                                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                                   b"HPP: entering START_TAG\n" as *const u8 as *const libc::c_char);
                                }
                            } else {
                                htmlParseErr(ctxt, XML_ERR_NAME_REQUIRED,
                                             b"htmlParseTryOrFinish: invalid element name\n\x00"
                                                 as *const u8 as *const libc::c_char,
                                             0 as *const xmlChar,
                                             0 as *const xmlChar);
                                htmlCheckParagraph(ctxt);
                                sax_condition = unsafe {
                                    !ctxtPtr.sax.is_null() && (*(*ctxt).sax).characters.is_some()
                                };
                                if sax_condition {
                                    let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                                    xmlSAXHandler_characters_safe(saxPtr.characters, ctxtPtr.userData,
                                                                  &*in_0Ptr.cur.offset(0
                                                                      as libc::c_int
                                                                      as isize),
                                                                  1
                                                                      as libc::c_int);
                                }
                                xmlNextChar_safe(ctxt);
                            }
                            continue;
                        } else {
                            if terminate == 0 && htmlParseLookupSequence(ctxt,
                                                                         '<' as i32 as xmlChar,
                                                                         0 as libc::c_int as xmlChar,
                                                                         0 as libc::c_int as xmlChar,
                                                                         0 as libc::c_int) <
                                0 as libc::c_int {
                                break;
                            }
                            ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                            if DEBUG_PUSH != 0 as libc::c_int {
                                __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                               b"HPP: Parsing char data\n" as *const u8 as *const libc::c_char);
                            }
                            while ctxtPtr.instate != XML_PARSER_START_TAG && cur as libc::c_int != '<' as i32 && in_0Ptr.cur < in_0Ptr.end {
                                if cur as libc::c_int == '&' as i32 {
                                    htmlParseReference(ctxt);
                                } else {
                                    htmlParseCharData(ctxt);
                                }
                                unsafe { cur = *in_0Ptr.cur.offset((0 as libc::c_int) as isize); }
                            }
                        }
                    }
                }
            }
            XML_PARSER_END_TAG => {
                if avail < 2 as libc::c_int as libc::c_long { break; }
                if terminate == 0 && htmlParseLookupSequence(ctxt,
                                                             '>' as i32 as xmlChar,
                                                             0 as libc::c_int as xmlChar,
                                                             0 as libc::c_int as xmlChar,
                                                             0 as libc::c_int) <
                    0 as libc::c_int {
                    break;
                }
                htmlParseEndTag(ctxt);
                if ctxtPtr.nameNr == 0 as libc::c_int {
                    ctxtPtr.instate = XML_PARSER_EPILOG;
                } else { ctxtPtr.instate = XML_PARSER_CONTENT; }
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_CDATA_SECTION => {
                htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                             b"HPP: internal error, state == CDATA\n\x00"
                                 as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_DTD => {
                htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                             b"HPP: internal error, state == DTD\n\x00" as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_COMMENT => {
                htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                             b"HPP: internal error, state == COMMENT\n\x00"
                                 as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_PI => {
                htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                             b"HPP: internal error, state == PI\n\x00" as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_ENTITY_DECL => {
                htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                             b"HPP: internal error, state == ENTITY_DECL\n\x00"
                                 as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_ENTITY_VALUE => {
                htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                             b"HPP: internal error, state == ENTITY_VALUE\n\x00"
                                 as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_ATTRIBUTE_VALUE => {
                htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                             b"HPP: internal error, state == ATTRIBUTE_VALUE\n\x00"
                                 as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                ctxtPtr.instate = XML_PARSER_START_TAG;
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering START_TAG\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_SYSTEM_LITERAL => {
                htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                             b"HPP: internal error, state == XML_PARSER_SYSTEM_LITERAL\n\x00"
                                 as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_IGNORE => {
                htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                             b"HPP: internal error, state == XML_PARSER_IGNORE\n\x00"
                                 as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            XML_PARSER_PUBLIC_LITERAL => {
                htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                             b"HPP: internal error, state == XML_PARSER_LITERAL\n\x00"
                                 as *const u8 as *const libc::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
                ctxtPtr.instate = XML_PARSER_CONTENT;
                ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
                if DEBUG_PUSH != 0 as libc::c_int {
                    __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                                   b"HPP: entering CONTENT\n" as *const u8 as *const libc::c_char);
                }
            }
            _ => {}
        }
    }
    /* bad cut of input */
    if avail == 0 as libc::c_int as libc::c_long && terminate != 0 {
        htmlAutoCloseOnEnd(ctxt);
        if ctxtPtr.nameNr == 0 as libc::c_int && ctxtPtr.instate as libc::c_int != XML_PARSER_EOF as libc::c_int {
            /*
            * SAX: end of the document processing.
            */
            ctxtPtr.instate = XML_PARSER_EOF;
            sax_condition = unsafe {
                !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endDocument.is_some()
            };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_endDocument_safe(saxPtr.endDocument, ctxtPtr.userData);
            }
        }
    }
    if ctxtPtr.options & HTML_PARSE_NODEFDTD as libc::c_int == 0 && !ctxtPtr.myDoc.is_null() && (terminate != 0 || ctxtPtr.instate as libc::c_int == XML_PARSER_EOF as libc::c_int || ctxtPtr.instate as libc::c_int == XML_PARSER_EPILOG as libc::c_int) {
        let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
        dtd = xmlGetIntSubset_safe(ctxtPtr.myDoc as *const xmlDoc);
        if dtd.is_null() {
            let mut myDocPtr = unsafe { &mut *(*ctxt).myDoc };
            myDocPtr.intSubset = xmlCreateIntSubset_safe(ctxtPtr.myDoc,
                                                         b"html\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                                                         b"-//W3C//DTD HTML 4.0 Transitional//EN\x00"
                                                             as *const u8 as *const libc::c_char as *mut xmlChar,
                                                         b"http://www.w3.org/TR/REC-html40/loose.dtd\x00"
                                                             as *const u8 as *const libc::c_char as *mut xmlChar)
        }
        if DEBUG_PUSH != 0 as libc::c_int {
            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                           b"HPP: done %d\n" as *const u8 as *const libc::c_char, ret);
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
pub fn htmlParseChunk(mut ctxt:
                      htmlParserCtxtPtr,
                      mut chunk:
                      *const libc::c_char,
                      mut size: libc::c_int,
                      mut terminate:
                      libc::c_int)
                      -> libc::c_int {
    let mut DEBUG_PUSH: libc::c_int = 0 as libc::c_int;
    match () {
        #[cfg(DEBUG_PUSH)]      _ => {
            DEBUG_PUSH = 1 as libc::c_int;
        }
        #[cfg(not(DEBUG_PUSH))]      _ => {}
    };
    unsafe {
        if ctxt.is_null() || (*ctxt).input.is_null() {
            htmlParseErr(ctxt, XML_ERR_INTERNAL_ERROR,
                         b"htmlParseChunk: context error\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                         0 as *const xmlChar);
            return XML_ERR_INTERNAL_ERROR as libc::c_int;
        }
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut flag1 = unsafe {
        size > 0 as libc::c_int && !chunk.is_null() && !ctxtPtr.input.is_null()
            && !(*(*ctxt).input).buf.is_null() && ctxtPtr.instate as libc::c_int != XML_PARSER_EOF as libc::c_int
    };
    if flag1 {
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        let mut bufPtr = unsafe { &mut *(*(*ctxt).input).buf };
        let mut base: size_t = xmlBufGetInputBase_safe(bufPtr.buffer, ctxtPtr.input);
        let mut cur: size_t = unsafe {
            inputPtr.cur.offset_from(inputPtr.base)
                as libc::c_long as size_t
        };
        let mut res: libc::c_int = 0;
        res = xmlParserInputBufferPush_safe(inputPtr.buf, size, chunk);
        xmlBufSetInputBaseCur_safe(bufPtr.buffer, ctxtPtr.input,
                                   base, cur);
        if res < 0 as libc::c_int {
            ctxtPtr.errNo = XML_PARSER_EOF as libc::c_int;
            ctxtPtr.disableSAX = 1 as libc::c_int;
            return XML_PARSER_EOF as libc::c_int;
        }
        if DEBUG_PUSH != 0 {
            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                           b"HPP: pushed %d\n" as *const u8 as *const libc::c_char, size);
        }
    } else if ctxtPtr.instate as libc::c_int != XML_PARSER_EOF as libc::c_int {
        let mut flag2 = unsafe { !ctxtPtr.input.is_null() && !(*(*ctxt).input).buf.is_null() };
        if flag2 {
            let mut inputPtr = unsafe { &mut *(*ctxt).input };
            let mut in_0: xmlParserInputBufferPtr = inputPtr.buf;
            let mut in_0Ptr = unsafe { &mut *in_0 };
            if !in_0Ptr.encoder.is_null() && !in_0Ptr.buffer.is_null() && !in_0Ptr.raw.is_null() {
                let mut nbchars: libc::c_int = 0;
                let mut base_0: size_t = xmlBufGetInputBase_safe(in_0Ptr.buffer, ctxtPtr.input);
                let mut current: size_t = unsafe {
                    inputPtr.cur.offset_from(inputPtr.base)
                        as libc::c_long as size_t
                };
                nbchars = xmlCharEncInput_safe(in_0, terminate);
                if nbchars < 0 as libc::c_int {
                    htmlParseErr(ctxt, XML_ERR_INVALID_ENCODING,
                                 b"encoder error\n\x00" as *const u8 as *const libc::c_char, 0 as *const xmlChar,
                                 0 as *const xmlChar);
                    return XML_ERR_INVALID_ENCODING as libc::c_int;
                }
                xmlBufSetInputBaseCur_safe(in_0Ptr.buffer, ctxtPtr.input, base_0,
                                           current);
            }
        }
    }
    htmlParseTryOrFinish(ctxt, terminate);
    if terminate != 0 {
        if ctxtPtr.instate as libc::c_int != XML_PARSER_EOF as libc::c_int && ctxtPtr.instate as libc::c_int != XML_PARSER_EPILOG as libc::c_int && ctxtPtr.instate as libc::c_int != XML_PARSER_MISC as libc::c_int {
            ctxtPtr.errNo = XML_ERR_DOCUMENT_END as libc::c_int;
            ctxtPtr.wellFormed = 0 as libc::c_int
        }
        if ctxtPtr.instate as libc::c_int != XML_PARSER_EOF as libc::c_int {
            let mut sax_condition = unsafe {
                !ctxtPtr.sax.is_null() && (*(*ctxt).sax).endDocument.is_some()
            };
            if sax_condition {
                let mut saxPtr = unsafe { &mut *(*ctxt).sax };
                xmlSAXHandler_endDocument_safe(saxPtr.endDocument, ctxtPtr.userData);
            }
        }
        ctxtPtr.instate = XML_PARSER_EOF
    }
    return ctxtPtr.errNo as xmlParserErrors as libc::c_int;
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
pub fn htmlCreatePushParserCtxt(mut sax:
                                htmlSAXHandlerPtr,
                                mut user_data:
                                *mut libc::c_void,
                                mut chunk:
                                *const libc::c_char,
                                mut size:
                                libc::c_int,
                                mut filename:
                                *const libc::c_char,
                                mut enc:
                                xmlCharEncoding)
                                -> htmlParserCtxtPtr {
    let mut DEBUG_PUSH: libc::c_int = 0 as libc::c_int;
    match () {
        #[cfg(DEBUG_PUSH)]      _ => {
            DEBUG_PUSH = 1 as libc::c_int;
        }
        #[cfg(not(DEBUG_PUSH))]      _ => {}
    };
    let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut inputStream: htmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    xmlInitParser_safe();
    buf = xmlAllocParserInputBuffer_safe(enc);
    if buf.is_null() { return 0 as htmlParserCtxtPtr; }
    ctxt = htmlNewParserCtxt();
    if ctxt.is_null() {
        xmlFreeParserInputBuffer_safe(buf);
        return 0 as htmlParserCtxtPtr;
    }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut bufPtr = unsafe { &mut *buf };
    if enc as libc::c_int == XML_CHAR_ENCODING_UTF8 as libc::c_int || !bufPtr.encoder.is_null() {
        ctxtPtr.charset = XML_CHAR_ENCODING_UTF8 as libc::c_int
    }
    if !sax.is_null() {
        if ctxtPtr.sax != __htmlDefaultSAXHandler_safe() as xmlSAXHandlerPtr {
            xmlFree_safe(ctxtPtr.sax as *mut libc::c_void);
        }
        ctxtPtr.sax = xmlMalloc_safe(::std::mem::size_of::<htmlSAXHandler>()
            as libc::c_ulong)
            as htmlSAXHandlerPtr;
        if ctxtPtr.sax.is_null() {
            xmlFree_safe(buf as *mut libc::c_void);
            xmlFree_safe(ctxt as *mut libc::c_void);
            return 0 as htmlParserCtxtPtr;
        }
        memcpy_safe(ctxtPtr.sax as *mut libc::c_void, sax as *const libc::c_void,
                    ::std::mem::size_of::<htmlSAXHandler>() as libc::c_ulong);
        if !user_data.is_null() { ctxtPtr.userData = user_data }
    }
    if filename.is_null() {
        ctxtPtr.directory = 0 as *mut libc::c_char
    } else { ctxtPtr.directory = xmlParserGetDirectory_safe(filename) }
    inputStream = htmlNewInputStream(ctxt);
    if inputStream.is_null() {
        xmlFreeParserCtxt_safe(ctxt);
        xmlFree_safe(buf as *mut libc::c_void);
        return 0 as htmlParserCtxtPtr;
    }
    let mut inputStreamPtr = unsafe { &mut *inputStream };
    if filename.is_null() {
        inputStreamPtr.filename = 0 as *const libc::c_char
    } else {
        inputStreamPtr.filename = xmlCanonicPath_safe(filename as *const xmlChar) as *mut libc::c_char
    }
    inputStreamPtr.buf = buf;
    xmlBufResetInput_safe(bufPtr.buffer, inputStream);
    inputPush_safe(ctxt, inputStream);
    let mut flag = unsafe {
        size > 0 as libc::c_int && !chunk.is_null() && !ctxtPtr.input.is_null()
            && !(*(*ctxt).input).buf.is_null()
    };
    if flag {
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        let mut bufPtr = unsafe { &mut *(*(*ctxt).input).buf };
        let mut base: size_t = xmlBufGetInputBase_safe(bufPtr.buffer, ctxtPtr.input);
        let mut cur: size_t = unsafe {
            inputPtr.cur.offset_from(inputPtr.base)
                as libc::c_long as size_t
        };
        xmlParserInputBufferPush_safe(inputPtr.buf, size, chunk);
        xmlBufSetInputBaseCur_safe(bufPtr.buffer, ctxtPtr.input,
                                   base, cur);
        if DEBUG_PUSH != 0 {
            __xmlGenericError_safe_macro!(__xmlGenericErrorContext_safe(),
                                                                           b"HPP: pushed %d\n" as *const u8 as *const libc::c_char, size);
        }
    }
    ctxtPtr.progressive = 1 as libc::c_int;
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

pub fn htmlSAXParseDoc(mut cur:
                       *const xmlChar,
                       mut encoding:
                       *const libc::c_char,
                       mut sax:
                       htmlSAXHandlerPtr,
                       mut userData:
                       *mut libc::c_void)
                       -> htmlDocPtr {
    let mut ret: htmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    xmlInitParser_safe();
    if cur.is_null() { return 0 as htmlDocPtr; }
    ctxt = htmlCreateDocParserCtxt(cur, encoding);
    if ctxt.is_null() { return 0 as htmlDocPtr; }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if !sax.is_null() {
        if !ctxtPtr.sax.is_null() {
            xmlFree_safe(ctxtPtr.sax as *mut libc::c_void);
        }
        ctxtPtr.sax = sax;
        ctxtPtr.userData = userData
    }
    htmlParseDocument(ctxt);
    ret = ctxtPtr.myDoc;
    if !sax.is_null() {
        ctxtPtr.sax = 0 as *mut _xmlSAXHandler;
        ctxtPtr.userData = 0 as *mut libc::c_void
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

pub fn htmlParseDoc(mut cur: *const xmlChar,
                    mut encoding:
                    *const libc::c_char)
                    -> htmlDocPtr {
    return htmlSAXParseDoc(cur, encoding,
                           0 as htmlSAXHandlerPtr,
                           0 as *mut libc::c_void);
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

pub fn htmlCreateFileParserCtxt(mut filename:
                                *const libc::c_char,
                                mut encoding:
                                *const libc::c_char)
                                -> htmlParserCtxtPtr {
    let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut inputStream: htmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut canonicFilename: *mut libc::c_char = 0 as *mut libc::c_char;
    /* htmlCharEncoding enc; */
    let mut content: *mut xmlChar = 0 as *mut xmlChar;
    let mut content_line: *mut xmlChar = b"charset=\x00" as *const u8 as *const libc::c_char as *mut xmlChar;
    if filename.is_null() { return 0 as htmlParserCtxtPtr; }
    ctxt = htmlNewParserCtxt();
    if ctxt.is_null() { return 0 as htmlParserCtxtPtr; }
    canonicFilename = xmlCanonicPath_safe(filename as *const xmlChar) as *mut libc::c_char;
    if canonicFilename.is_null() {
        match () {
            #[cfg(LIBXML_SAX1_ENABLED)]      _ => {
                unsafe {
                    if (*__xmlDefaultSAXHandler()).error.is_some() {//todo-unsafe-__xmlDefaultSAXHandler
                        (*__xmlDefaultSAXHandler()).error.expect("non-null function pointer")
                            (0
                                 as *mut libc::c_void,
                             b"out of memory\n\x00"
                                 as *const u8
                                 as *const libc::c_char);
                    }
                }
            }
            #[cfg(not(LIBXML_SAX1_ENABLED))] _ => {}
        };
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as htmlParserCtxtPtr;
    }
    inputStream = xmlLoadExternalEntity_safe(canonicFilename, 0 as *const libc::c_char,
                                             ctxt);
    xmlFree_safe(canonicFilename as *mut libc::c_void);
    if inputStream.is_null() {
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as htmlParserCtxtPtr;
    }
    inputPush_safe(ctxt, inputStream);
    /* set encoding */
    if !encoding.is_null() {
        let mut l: size_t = strlen_safe(encoding);
        if l < 1000 as libc::c_int as libc::c_ulong {
            content = xmlMallocAtomic_safe((xmlStrlen_safe(content_line)
                as libc::c_ulong).wrapping_add(l).wrapping_add(1
                as libc::c_int
                as libc::c_ulong))
                as *mut xmlChar;
            if !content.is_null() {
                strcpy_safe(content as *mut libc::c_char,
                            content_line as *mut libc::c_char);
                strcat_safe(content as *mut libc::c_char,
                            encoding as *mut libc::c_char);
                htmlCheckEncoding(ctxt, content);
                xmlFree_safe(content as *mut libc::c_void);
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

pub fn htmlSAXParseFile(mut filename:
                        *const libc::c_char,
                        mut encoding:
                        *const libc::c_char,
                        mut sax:
                        htmlSAXHandlerPtr,
                        mut userData:
                        *mut libc::c_void)
                        -> htmlDocPtr {
    let mut ret: htmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut oldsax: htmlSAXHandlerPtr = 0 as htmlSAXHandlerPtr;
    xmlInitParser_safe();
    ctxt = htmlCreateFileParserCtxt(filename, encoding);
    if ctxt.is_null() { return 0 as htmlDocPtr; }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if !sax.is_null() {
        oldsax = ctxtPtr.sax;
        ctxtPtr.sax = sax;
        ctxtPtr.userData = userData
    }
    htmlParseDocument(ctxt);
    ret = ctxtPtr.myDoc;
    if !sax.is_null() {
        ctxtPtr.sax = oldsax;
        ctxtPtr.userData = 0 as *mut libc::c_void
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

pub fn htmlParseFile(mut filename:
                     *const libc::c_char,
                     mut encoding:
                     *const libc::c_char)
                     -> htmlDocPtr {
    return htmlSAXParseFile(filename, encoding,
                            0 as htmlSAXHandlerPtr,
                            0 as *mut libc::c_void);
}
/* *
 * htmlHandleOmittedElem:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for handling HTML omitted tags.
 *
 * Returns the last value for 0 for no handling, 1 for auto insertion.
 */

pub fn htmlHandleOmittedElem(mut val:
                             libc::c_int)
                             -> libc::c_int {
    let mut old: libc::c_int = getHtmlOmittedDefaultValue();
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

pub fn htmlElementAllowedHere(mut parent:
                              *const htmlElemDesc,
                              mut elt:
                              *const xmlChar)
                              -> libc::c_int {
    let mut p: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    unsafe {
        if elt.is_null() || parent.is_null() || (*parent).subelts.is_null() {
            return 0 as libc::c_int;
        }

        p = (*parent).subelts;
        while !(*p).is_null() {
            if xmlStrcmp_safe(*p as *const xmlChar, elt) == 0 {
                return 1 as libc::c_int;
            }
            p = p.offset(1)
        }
    }
    return 0 as libc::c_int;
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

pub fn htmlElementStatusHere(mut parent:
                             *const htmlElemDesc,
                             mut elt:
                             *const htmlElemDesc)
                             -> htmlStatus {
    if parent.is_null() || elt.is_null() { return HTML_INVALID; }
    let eltPtr = unsafe { &*elt };
    if htmlElementAllowedHere(parent,
                              eltPtr.name as *const xmlChar)
        == 0 {
        return HTML_INVALID;
    }
    return if eltPtr.dtd as libc::c_int == 0 as libc::c_int {
        HTML_VALID as libc::c_int
    } else { HTML_DEPRECATED as libc::c_int } as htmlStatus;
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

pub fn htmlAttrAllowed(mut elt:
                       *const htmlElemDesc,
                       mut attr:
                       *const xmlChar,
                       mut legacy:
                       libc::c_int)
                       -> htmlStatus {
    let mut p: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if elt.is_null() || attr.is_null() { return HTML_INVALID; }
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

pub fn htmlNodeStatus(node: htmlNodePtr,
                      mut legacy:
                      libc::c_int)
                      -> htmlStatus {
    if node.is_null() { return HTML_INVALID; }
    let mut nodePtr = unsafe { &mut *node };
    let mut parentPtr = unsafe { &mut *(*node).parent };
    match nodePtr.type_0 as libc::c_uint {
        1 => {
            return if legacy != 0 {
                (if htmlElementAllowedHere(htmlTagLookup(parentPtr.name),
                                           nodePtr.name)
                    != 0 {
                    HTML_VALID as libc::c_int
                } else { HTML_INVALID as libc::c_int }) as libc::c_uint
            } else {
                htmlElementStatusHere(htmlTagLookup(parentPtr.name),
                                      htmlTagLookup(nodePtr.name))
                    as libc::c_uint
            } as htmlStatus;
        }
        2 => {
            return htmlAttrAllowed(htmlTagLookup(parentPtr.name),
                                   nodePtr.name, legacy);
        }
        _ => { return HTML_NA; }
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

pub fn htmlCtxtReset(mut ctxt:
                     htmlParserCtxtPtr) {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    if ctxt.is_null() { return; }
    xmlInitParser_safe();
    let mut ctxtPtr = unsafe { &mut *ctxt };
    dict = ctxtPtr.dict;
    loop {
        input = inputPop_safe(ctxt);
        if input.is_null() { break; }
        /* Non consuming */
        xmlFreeInputStream_safe(input);
    }
    ctxtPtr.inputNr = 0 as libc::c_int;
    ctxtPtr.input = 0 as xmlParserInputPtr;
    ctxtPtr.spaceNr = 0 as libc::c_int;
    if !ctxtPtr.spaceTab.is_null() {
        unsafe {
            *ctxtPtr.spaceTab.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
        }
        unsafe {
            ctxtPtr.space = &mut *ctxtPtr.spaceTab.offset(0 as libc::c_int as isize) as *mut libc::c_int
        }
    } else { ctxtPtr.space = 0 as *mut libc::c_int }
    ctxtPtr.nodeNr = 0 as libc::c_int;
    ctxtPtr.node = 0 as xmlNodePtr;
    ctxtPtr.nameNr = 0 as libc::c_int;
    ctxtPtr.name = 0 as *const xmlChar;
    if !ctxtPtr.version.is_null() && (dict.is_null() || xmlDictOwns_safe(dict, ctxtPtr.version) == 0 as libc::c_int) {
        xmlFree_safe(ctxtPtr.version as *mut libc::c_char as *mut libc::c_void);
    }
    ctxtPtr.version = 0 as *const xmlChar;
    if !ctxtPtr.encoding.is_null() && (dict.is_null() || xmlDictOwns_safe(dict, ctxtPtr.encoding) == 0 as libc::c_int) {
        xmlFree_safe(ctxtPtr.encoding as *mut libc::c_char as *mut libc::c_void);
    }
    ctxtPtr.encoding = 0 as *const xmlChar;
    if !ctxtPtr.directory.is_null() && (dict.is_null() || xmlDictOwns_safe(dict,
                                                                           ctxtPtr.directory as *const xmlChar) == 0 as libc::c_int) {
        xmlFree_safe(ctxtPtr.directory as *mut libc::c_void);
    }
    ctxtPtr.directory = 0 as *mut libc::c_char;
    if !ctxtPtr.extSubURI.is_null() && (dict.is_null() || xmlDictOwns_safe(dict,
                                                                           ctxtPtr.extSubURI as *const xmlChar) == 0 as libc::c_int) {
        xmlFree_safe(ctxtPtr.extSubURI as *mut libc::c_char as *mut libc::c_void);
    }
    ctxtPtr.extSubURI = 0 as *mut xmlChar;
    if !ctxtPtr.extSubSystem.is_null() && (dict.is_null() || xmlDictOwns_safe(dict,
                                                                              ctxtPtr.extSubSystem as *const xmlChar) == 0 as libc::c_int) {
        xmlFree_safe(ctxtPtr.extSubSystem as *mut libc::c_char as *mut libc::c_void);
    }
    ctxtPtr.extSubSystem = 0 as *mut xmlChar;
    if !ctxtPtr.myDoc.is_null() { xmlFreeDoc_safe(ctxtPtr.myDoc); }
    ctxtPtr.myDoc = 0 as xmlDocPtr;
    ctxtPtr.standalone = -(1 as libc::c_int);
    ctxtPtr.hasExternalSubset = 0 as libc::c_int;
    ctxtPtr.hasPErefs = 0 as libc::c_int;
    ctxtPtr.html = 1 as libc::c_int;
    ctxtPtr.external = 0 as libc::c_int;
    ctxtPtr.instate = XML_PARSER_START;
    ctxtPtr.token = 0 as libc::c_int;
    ctxtPtr.wellFormed = 1 as libc::c_int;
    ctxtPtr.nsWellFormed = 1 as libc::c_int;
    ctxtPtr.disableSAX = 0 as libc::c_int;
    ctxtPtr.valid = 1 as libc::c_int;
    ctxtPtr.vctxt.userData = ctxt as *mut libc::c_void;
    ctxtPtr.vctxt.error = Some(xmlParserValidityError as unsafe extern "C" fn(_: *mut libc::c_void,
                                                                              _: *const libc::c_char, _: ...) -> ());
    ctxtPtr.vctxt.warning = Some(xmlParserValidityWarning as unsafe extern "C" fn(_: *mut libc::c_void,
                                                                                  _: *const libc::c_char, _: ...) -> ());
    ctxtPtr.record_info = 0 as libc::c_int;
    ctxtPtr.checkIndex = 0 as libc::c_int as libc::c_long;
    ctxtPtr.inSubset = 0 as libc::c_int;
    ctxtPtr.errNo = XML_ERR_OK as libc::c_int;
    ctxtPtr.depth = 0 as libc::c_int;
    ctxtPtr.charset = XML_CHAR_ENCODING_NONE as libc::c_int;
    ctxtPtr.catalogs = 0 as *mut libc::c_void;
    xmlInitNodeInfoSeq_safe(&mut ctxtPtr.node_seq);
    if !ctxtPtr.attsDefault.is_null() {
        xmlHashFree_safe(ctxtPtr.attsDefault,
                         Some(xmlHashDefaultDeallocator
                             as unsafe extern "C" fn(_:
                                                     *mut libc::c_void,
                                                     _:
                                                     *const xmlChar)
                                                     -> ()));
        ctxtPtr.attsDefault = 0 as xmlHashTablePtr
    }
    if !ctxtPtr.attsSpecial.is_null() {
        xmlHashFree_safe(ctxtPtr.attsSpecial, None);
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

pub fn htmlCtxtUseOptions(mut ctxt:
                          htmlParserCtxtPtr,
                          mut options:
                          libc::c_int)
                          -> libc::c_int {
    if ctxt.is_null() { return -(1 as libc::c_int); }
    let mut ctxtPtr = unsafe { &mut *ctxt };
    let mut saxPtr = unsafe { &mut *(*ctxt).sax };
    if options & HTML_PARSE_NOWARNING as libc::c_int != 0 {
        saxPtr.warning = None;
        ctxtPtr.vctxt.warning = None;
        options -= XML_PARSE_NOWARNING as libc::c_int;
        ctxtPtr.options |= XML_PARSE_NOWARNING as libc::c_int
    }
    if options & HTML_PARSE_NOERROR as libc::c_int != 0 {
        saxPtr.error = None;
        ctxtPtr.vctxt.error = None;
        saxPtr.fatalError = None;
        options -= XML_PARSE_NOERROR as libc::c_int;
        ctxtPtr.options |= XML_PARSE_NOERROR as libc::c_int
    }
    if options & HTML_PARSE_PEDANTIC as libc::c_int != 0 {
        ctxtPtr.pedantic = 1 as libc::c_int;
        options -= XML_PARSE_PEDANTIC as libc::c_int;
        ctxtPtr.options |= XML_PARSE_PEDANTIC as libc::c_int
    } else { ctxtPtr.pedantic = 0 as libc::c_int }
    if options & XML_PARSE_NOBLANKS as libc::c_int != 0 {
        ctxtPtr.keepBlanks = 0 as libc::c_int;
        saxPtr.ignorableWhitespace = Some(xmlSAX2IgnorableWhitespace as unsafe extern "C" fn(_: *mut libc::c_void,
                                                                                             _: *const xmlChar, _: libc::c_int)
                                                                                             -> ());
        options -= XML_PARSE_NOBLANKS as libc::c_int;
        ctxtPtr.options |= XML_PARSE_NOBLANKS as libc::c_int
    } else { ctxtPtr.keepBlanks = 1 as libc::c_int }
    if options & HTML_PARSE_RECOVER as libc::c_int != 0 {
        ctxtPtr.recovery = 1 as libc::c_int;
        options -= HTML_PARSE_RECOVER as libc::c_int
    } else { ctxtPtr.recovery = 0 as libc::c_int }
    if options & HTML_PARSE_COMPACT as libc::c_int != 0 {
        ctxtPtr.options |= HTML_PARSE_COMPACT as libc::c_int;
        options -= HTML_PARSE_COMPACT as libc::c_int
    }
    if options & XML_PARSE_HUGE as libc::c_int != 0 {
        ctxtPtr.options |= XML_PARSE_HUGE as libc::c_int;
        options -= XML_PARSE_HUGE as libc::c_int
    }
    if options & HTML_PARSE_NODEFDTD as libc::c_int != 0 {
        ctxtPtr.options |= HTML_PARSE_NODEFDTD as libc::c_int;
        options -= HTML_PARSE_NODEFDTD as libc::c_int
    }
    if options & HTML_PARSE_IGNORE_ENC as libc::c_int != 0 {
        ctxtPtr.options |= HTML_PARSE_IGNORE_ENC as libc::c_int;
        options -= HTML_PARSE_IGNORE_ENC as libc::c_int
    }
    if options & HTML_PARSE_NOIMPLIED as libc::c_int != 0 {
        ctxtPtr.options |= HTML_PARSE_NOIMPLIED as libc::c_int;
        options -= HTML_PARSE_NOIMPLIED as libc::c_int
    }
    ctxtPtr.dictNames = 0 as libc::c_int;
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
fn htmlDoRead(mut ctxt: htmlParserCtxtPtr,
              mut URL: *const libc::c_char,
              mut encoding: *const libc::c_char,
              mut options: libc::c_int,
              mut reuse: libc::c_int) -> htmlDocPtr {
    let mut ret: htmlDocPtr = 0 as *mut xmlDoc;
    htmlCtxtUseOptions(ctxt, options);
    let mut ctxtPtr = unsafe { &mut *ctxt };
    ctxtPtr.html = 1 as libc::c_int;
    if !encoding.is_null() {
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        let mut hdlr: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
        hdlr = xmlFindCharEncodingHandler_safe(encoding);
        if !hdlr.is_null() {
            xmlSwitchToEncoding_safe(ctxt, hdlr);
            if !inputPtr.encoding.is_null() {
                xmlFree_safe(inputPtr.encoding
                    as *mut xmlChar
                    as *mut libc::c_void);
            }
            inputPtr.encoding = xmlStrdup_safe(encoding as *mut xmlChar)
        }
    }
    let mut flag = unsafe {
        !URL.is_null() && !ctxtPtr.input.is_null() && (*(*ctxt).input).filename.is_null()
    };
    if flag {
        let mut inputPtr = unsafe { &mut *(*ctxt).input };
        inputPtr.filename = xmlStrdup_safe(URL as *const xmlChar) as *mut libc::c_char
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
        xmlFreeParserCtxt_safe(ctxt);
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

pub fn htmlReadDoc(mut cur: *const xmlChar,
                   mut URL:
                   *const libc::c_char,
                   mut encoding:
                   *const libc::c_char,
                   mut options: libc::c_int)
                   -> htmlDocPtr {
    let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    if cur.is_null() { return 0 as htmlDocPtr; }
    xmlInitParser_safe();
    ctxt = htmlCreateDocParserCtxt(cur, 0 as *const libc::c_char);
    if ctxt.is_null() { return 0 as htmlDocPtr; }
    return htmlDoRead(ctxt, URL, encoding, options, 0 as libc::c_int);
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

pub fn htmlReadFile(mut filename:
                    *const libc::c_char,
                    mut encoding:
                    *const libc::c_char,
                    mut options:
                    libc::c_int)
                    -> htmlDocPtr {
    let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    xmlInitParser_safe();
    ctxt = htmlCreateFileParserCtxt(filename, encoding);
    if ctxt.is_null() { return 0 as htmlDocPtr; }
    return htmlDoRead(ctxt, 0 as *const libc::c_char,
                      0 as *const libc::c_char, options, 0 as libc::c_int);
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

pub fn htmlReadMemory(mut buffer:
                      *const libc::c_char,
                      mut size: libc::c_int,
                      mut URL:
                      *const libc::c_char,
                      mut encoding:
                      *const libc::c_char,
                      mut options:
                      libc::c_int)
                      -> htmlDocPtr {
    let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    xmlInitParser_safe();
    ctxt = xmlCreateMemoryParserCtxt_safe(buffer, size);
    if ctxt.is_null() { return 0 as htmlDocPtr; }
    htmlDefaultSAXHandlerInit_safe();
    let mut ctxtPtr = unsafe { &mut *ctxt };
    if !ctxtPtr.sax.is_null() {
        memcpy_safe(ctxtPtr.sax as *mut libc::c_void,
                    __htmlDefaultSAXHandler_safe() as *const libc::c_void,
                    ::std::mem::size_of::<xmlSAXHandlerV1>() as libc::c_ulong);
    }
    return htmlDoRead(ctxt, URL, encoding, options, 0 as libc::c_int);
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

pub fn htmlReadFd(mut fd: libc::c_int,
                  mut URL:
                  *const libc::c_char,
                  mut encoding:
                  *const libc::c_char,
                  mut options: libc::c_int)
                  -> htmlDocPtr {
    let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if fd < 0 as libc::c_int { return 0 as htmlDocPtr; }
    xmlInitParser_safe();
    xmlInitParser_safe();
    input = xmlParserInputBufferCreateFd_safe(fd,
                                              XML_CHAR_ENCODING_NONE);
    if input.is_null() { return 0 as htmlDocPtr; }
    ctxt = xmlNewParserCtxt_safe();
    if ctxt.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        return 0 as htmlDocPtr;
    }
    stream = xmlNewIOInputStream_safe(ctxt, input,
                                      XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as htmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    return htmlDoRead(ctxt, URL, encoding, options, 0 as libc::c_int);
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

pub fn htmlReadIO(mut ioread:
                  xmlInputReadCallback,
                  mut ioclose:
                  xmlInputCloseCallback,
                  mut ioctx:
                  *mut libc::c_void,
                  mut URL:
                  *const libc::c_char,
                  mut encoding:
                  *const libc::c_char,
                  mut options: libc::c_int)
                  -> htmlDocPtr {
    let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ioread.is_none() { return 0 as htmlDocPtr; }
    xmlInitParser_safe();
    input = xmlParserInputBufferCreateIO_safe(ioread, ioclose, ioctx,
                                              XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose_safe(ioclose, ioctx);
        }
        return 0 as htmlDocPtr;
    }
    ctxt = htmlNewParserCtxt();
    if ctxt.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        return 0 as htmlDocPtr;
    }
    stream = xmlNewIOInputStream_safe(ctxt, input,
                                      XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as htmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    return htmlDoRead(ctxt, URL, encoding, options, 0 as libc::c_int);
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

pub fn htmlCtxtReadDoc(mut ctxt:
                       htmlParserCtxtPtr,
                       mut cur:
                       *const xmlChar,
                       mut URL:
                       *const libc::c_char,
                       mut encoding:
                       *const libc::c_char,
                       mut options:
                       libc::c_int)
                       -> htmlDocPtr {
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if cur.is_null() { return 0 as htmlDocPtr; }
    if ctxt.is_null() { return 0 as htmlDocPtr; }
    xmlInitParser_safe();
    htmlCtxtReset(ctxt);
    stream = xmlNewStringInputStream_safe(ctxt, cur);
    if stream.is_null() { return 0 as htmlDocPtr; }
    inputPush_safe(ctxt, stream);
    return htmlDoRead(ctxt, URL, encoding, options, 1 as libc::c_int);
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

pub fn htmlCtxtReadFile(mut ctxt:
                        htmlParserCtxtPtr,
                        mut filename:
                        *const libc::c_char,
                        mut encoding:
                        *const libc::c_char,
                        mut options:
                        libc::c_int)
                        -> htmlDocPtr {
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if filename.is_null() { return 0 as htmlDocPtr; }
    if ctxt.is_null() { return 0 as htmlDocPtr; }
    xmlInitParser_safe();
    htmlCtxtReset(ctxt);
    stream = xmlLoadExternalEntity_safe(filename,
                                        0 as *const libc::c_char, ctxt);
    if stream.is_null() { return 0 as htmlDocPtr; }
    inputPush_safe(ctxt, stream);
    return htmlDoRead(ctxt, 0 as *const libc::c_char, encoding, options,
                      1 as libc::c_int);
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

pub fn htmlCtxtReadMemory(mut ctxt:
                          htmlParserCtxtPtr,
                          mut buffer:
                          *const libc::c_char,
                          mut size:
                          libc::c_int,
                          mut URL:
                          *const libc::c_char,
                          mut encoding:
                          *const libc::c_char,
                          mut options:
                          libc::c_int)
                          -> htmlDocPtr {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ctxt.is_null() { return 0 as htmlDocPtr; }
    if buffer.is_null() { return 0 as htmlDocPtr; }
    xmlInitParser_safe();
    htmlCtxtReset(ctxt);
    input = xmlParserInputBufferCreateMem_safe(buffer, size,
                                               XML_CHAR_ENCODING_NONE);
    if input.is_null() { return 0 as htmlDocPtr; }
    stream = xmlNewIOInputStream_safe(ctxt, input,
                                      XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        return 0 as htmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    return htmlDoRead(ctxt, URL, encoding, options, 1 as libc::c_int);
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

pub fn htmlCtxtReadFd(mut ctxt:
                      htmlParserCtxtPtr,
                      mut fd: libc::c_int,
                      mut URL:
                      *const libc::c_char,
                      mut encoding:
                      *const libc::c_char,
                      mut options:
                      libc::c_int)
                      -> htmlDocPtr {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if fd < 0 as libc::c_int { return 0 as htmlDocPtr; }
    if ctxt.is_null() { return 0 as htmlDocPtr; }
    xmlInitParser_safe();
    htmlCtxtReset(ctxt);
    input = xmlParserInputBufferCreateFd_safe(fd,
                                              XML_CHAR_ENCODING_NONE);
    if input.is_null() { return 0 as htmlDocPtr; }
    stream = xmlNewIOInputStream_safe(ctxt, input,
                                      XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        return 0 as htmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    return htmlDoRead(ctxt, URL, encoding, options, 1 as libc::c_int);
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

pub fn htmlCtxtReadIO(mut ctxt:
                      htmlParserCtxtPtr,
                      mut ioread:
                      xmlInputReadCallback,
                      mut ioclose:
                      xmlInputCloseCallback,
                      mut ioctx:
                      *mut libc::c_void,
                      mut URL:
                      *const libc::c_char,
                      mut encoding:
                      *const libc::c_char,
                      mut options:
                      libc::c_int)
                      -> htmlDocPtr {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ioread.is_none() { return 0 as htmlDocPtr; }
    if ctxt.is_null() { return 0 as htmlDocPtr; }
    xmlInitParser_safe();
    htmlCtxtReset(ctxt);
    input = xmlParserInputBufferCreateIO_safe(ioread, ioclose, ioctx,
                                              XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose_safe(ioclose, ioctx);
        }
        return 0 as htmlDocPtr;
    }
    stream = xmlNewIOInputStream_safe(ctxt, input,
                                      XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        return 0 as htmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    return htmlDoRead(ctxt, URL, encoding, options, 1 as libc::c_int);
}
/* LIBXML_HTML_ENABLED */

fn xmlSAXHandler_endElement_safe(func: endElementSAXFunc, arg1: *mut libc::c_void, arg2: *const xmlChar) {
    unsafe { func.expect("non-null function pointer")(arg1, arg2); }
}

fn xmlSAXHandler_startElement_safe(func: startElementSAXFunc, arg1: *mut libc::c_void, arg2: *const xmlChar,
                                   arg3: *mut *const xmlChar) {
    unsafe { func.expect("non-null function pointer")(arg1, arg2, arg3); }
}

fn xmlSAXHandler_cdataBlock_safe(func: cdataBlockSAXFunc, arg1: *mut libc::c_void, arg2: *const xmlChar,
                                 arg3: libc::c_int) {
    unsafe { func.expect("non-null function pointer")(arg1, arg2, arg3); }
}

fn xmlSAXHandler_characters_safe(func: charactersSAXFunc, arg1: *mut libc::c_void, arg2: *const xmlChar,
                                 arg3: libc::c_int) {
    unsafe { func.expect("non-null function pointer")(arg1, arg2, arg3); }
}

fn xmlSAXHandler_ignorableWhitespace_safe(func: ignorableWhitespaceSAXFunc, arg1: *mut libc::c_void, arg2: *const xmlChar,
                                          arg3: libc::c_int) {
    unsafe { func.expect("non-null function pointer")(arg1, arg2, arg3); }
}

fn xmlSAXHandler_processingInstruction_safe(func: processingInstructionSAXFunc, arg1: *mut libc::c_void, arg2: *const xmlChar,
                                            arg3: *const xmlChar) {
    unsafe { func.expect("non-null function pointer")(arg1, arg2, arg3); }
}

fn xmlSAXHandler_comment_safe(func: commentSAXFunc, arg1: *mut libc::c_void, arg2: *const xmlChar) {
    unsafe { func.expect("non-null function pointer")(arg1, arg2); }
}

fn xmlSAXHandler_internalSubset_safe(func: internalSubsetSAXFunc, arg1: *mut libc::c_void, arg2: *const xmlChar,
                                     arg3: *const xmlChar, arg4: *const xmlChar) {
    unsafe { func.expect("non-null function pointer")(arg1, arg2, arg3, arg4); }
}

fn xmlSAXHandler_endDocument_safe(func: endDocumentSAXFunc, arg1: *mut libc::c_void) {
    unsafe { func.expect("non-null function pointer")(arg1); }
}

fn xmlSAXHandler_startDocument_safe(func: startDocumentSAXFunc, arg1: *mut libc::c_void) {
    unsafe { func.expect("non-null function pointer")(arg1); }
}

fn xmlSAXHandler_setDocumentLocator_safe(func: setDocumentLocatorSAXFunc, arg1: *mut libc::c_void,
                                         arg2: xmlSAXLocatorPtr) {
    unsafe { func.expect("non-null function pointer")(arg1, arg2); }
}

fn ioclose_safe(func: xmlInputCloseCallback, arg1: *mut libc::c_void) {
    unsafe { func.expect("non-null function pointer")(arg1); }
}

fn getHtmlEndPriority(index: usize) -> elementPriority {
    unsafe { return htmlEndPriority[index]; }
}

fn getHtmlOmittedDefaultValue() -> libc::c_int {
    unsafe { return htmlOmittedDefaultValue; }
}

fn setHtmlOmittedDefaultValue(val: libc::c_int) {
    unsafe { htmlOmittedDefaultValue = val; }
}

fn getHtmlNoContentElements(index: usize) -> *const libc::c_char {
    unsafe { return htmlNoContentElements[index]; }
}

fn getHtmlScriptAttributes(index: usize) -> *const libc::c_char {
    unsafe { return htmlScriptAttributes[index]; }
}

fn getHtml40EntitiesTable(index: usize) -> htmlEntityDesc {
    unsafe { return html40EntitiesTable[index]; }
}

fn getAllowPCData(index: usize) -> *const libc::c_char {
    unsafe { return allowPCData[index]; }
}