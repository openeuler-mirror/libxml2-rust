use rust_ffi::ffi_defination::defination::*;
use rust_ffi::ffi_extern_method::extern_method::*;
use rust_ffi::ffi_extern_method::extern_method_safe::*;

pub unsafe fn xmlCheckVersion(mut version: i32) {
    let mut safe__xmlGenericError = unsafe { &mut *__xmlGenericError() };
    let mut safe__xmlGenericErrorContext = unsafe { &mut **__xmlGenericErrorContext() };
    let mut myversion: i32 = 20912 as i32;
    xmlInitParser_safe();
    if myversion / 10000 as i32 != version / 10000 as i32 {
        unsafe {
            safe__xmlGenericError.expect("non-null function pointer")(
                safe__xmlGenericErrorContext,
                b"Fatal: program compiled against libxml %d using libxml %d\n\x00" as *const u8
                    as *const i8,
                version / 10000 as i32,
                myversion / 10000 as i32,
            );
            fprintf(
                stderr,
                b"Fatal: program compiled against libxml %d using libxml %d\n\x00" as *const u8
                    as *const i8,
                version / 10000 as i32,
                myversion / 10000 as i32,
            );
        }
    }
    if (myversion / 100 as i32) < version / 100 as i32 {
        unsafe {
            safe__xmlGenericError.expect("non-null function pointer")(
                safe__xmlGenericErrorContext,
                b"Warning: program compiled against libxml %d using older %d\n\x00" as *const u8
                    as *const i8,
                version / 100 as i32,
                myversion / 100 as i32,
            );
        }
    };
}
/* LIBXML_LEGACY_ENABLED */
/*
 * internal only
 */
/* ***********************************************************************
 *									*
 *		Some factorized error routines				*
 *									*
 ************************************************************************/
/* *
 * xmlErrMemory:
 * @ctxt:  an XML parser context
 * @extra:  extra information
 *
 * Handle a redefinition of attribute error
 */

pub unsafe fn xmlErrMemory(mut ctxt: xmlParserCtxtPtr, mut extra: *const i8) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null()
        && safe_ctxt.disableSAX != 0 as i32
        && safe_ctxt.instate as i32 == XML_PARSER_EOF as i32
    {
        return;
    }
    if !ctxt.is_null() {
        safe_ctxt.errNo = XML_ERR_NO_MEMORY as i32;
        safe_ctxt.instate = XML_PARSER_EOF;
        safe_ctxt.disableSAX = 1 as i32
    }
    if !extra.is_null() {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut (),
                ctxt as *mut (),
                0 as *mut (),
                XML_FROM_PARSER as i32,
                XML_ERR_NO_MEMORY as i32,
                XML_ERR_FATAL,
                0 as *const i8,
                0 as i32,
                extra,
                0 as *const i8,
                0 as *const i8,
                0 as i32,
                0 as i32,
                b"Memory allocation failed : %s\n\x00" as *const u8 as *const i8,
                extra,
            );
        }
    } else {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut (),
                ctxt as *mut (),
                0 as *mut (),
                XML_FROM_PARSER as i32,
                XML_ERR_NO_MEMORY as i32,
                XML_ERR_FATAL,
                0 as *const i8,
                0 as i32,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as i32,
                0 as i32,
                b"Memory allocation failed\n\x00" as *const u8 as *const i8,
            );
        }
    };
}
/* internal error reporting */
/* *
 * __xmlErrEncoding:
 * @ctxt:  an XML parser context
 * @xmlerr:  the error number
 * @msg:  the error message
 * @str1:  an string info
 * @str2:  an string info
 *
 * Handle an encoding error
 */

pub unsafe fn __xmlErrEncoding(
    mut ctxt: xmlParserCtxtPtr,
    mut xmlerr: xmlParserErrors,
    mut msg: *const i8,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null()
        && safe_ctxt.disableSAX != 0 as i32
        && safe_ctxt.instate as i32 == XML_PARSER_EOF as i32
    {
        return;
    }
    if !ctxt.is_null() {
        safe_ctxt.errNo = xmlerr as i32
    }
    unsafe {
        __xmlRaiseError(
            None,
            None,
            0 as *mut (),
            ctxt as *mut (),
            0 as *mut (),
            XML_FROM_PARSER as i32,
            xmlerr as i32,
            XML_ERR_FATAL,
            0 as *const i8,
            0 as i32,
            str1 as *const i8,
            str2 as *const i8,
            0 as *const i8,
            0 as i32,
            0 as i32,
            msg,
            str1,
            str2,
        );
    }
    if !ctxt.is_null() {
        safe_ctxt.wellFormed = 0 as i32;
        if safe_ctxt.recovery == 0 as i32 {
            safe_ctxt.disableSAX = 1 as i32
        }
    };
}
/* *
 * xmlErrInternal:
 * @ctxt:  an XML parser context
 * @msg:  the error message
 * @str:  error information
 *
 * Handle an internal error
 */
unsafe fn xmlErrInternal(mut ctxt: xmlParserCtxtPtr, mut msg: *const i8, mut str: *const xmlChar) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null()
        && safe_ctxt.disableSAX != 0 as i32
        && safe_ctxt.instate as i32 == XML_PARSER_EOF as i32
    {
        return;
    }
    if !ctxt.is_null() {
        safe_ctxt.errNo = XML_ERR_INTERNAL_ERROR as i32
    }
    unsafe {
        __xmlRaiseError(
            None,
            None,
            0 as *mut (),
            ctxt as *mut (),
            0 as *mut (),
            XML_FROM_PARSER as i32,
            XML_ERR_INTERNAL_ERROR as i32,
            XML_ERR_FATAL,
            0 as *const i8,
            0 as i32,
            str as *const i8,
            0 as *const i8,
            0 as *const i8,
            0 as i32,
            0 as i32,
            msg,
            str,
        );
    }
    if !ctxt.is_null() {
        safe_ctxt.wellFormed = 0 as i32;
        if safe_ctxt.recovery == 0 as i32 {
            safe_ctxt.disableSAX = 1 as i32
        }
    };
}
/* *
 * xmlErrEncodingInt:
 * @ctxt:  an XML parser context
 * @error:  the error number
 * @msg:  the error message
 * @val:  an integer value
 *
 * n encoding error
 */
unsafe fn xmlErrEncodingInt(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const i8,
    mut val: i32,
) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null()
        && safe_ctxt.disableSAX != 0 as i32
        && safe_ctxt.instate as i32 == XML_PARSER_EOF as i32
    {
        return;
    }
    if !ctxt.is_null() {
        safe_ctxt.errNo = error as i32
    }
    unsafe {
        __xmlRaiseError(
            None,
            None,
            0 as *mut (),
            ctxt as *mut (),
            0 as *mut (),
            XML_FROM_PARSER as i32,
            error as i32,
            XML_ERR_FATAL,
            0 as *const i8,
            0 as i32,
            0 as *const i8,
            0 as *const i8,
            0 as *const i8,
            val,
            0 as i32,
            msg,
            val,
        );
    }
    if !ctxt.is_null() {
        safe_ctxt.wellFormed = 0 as i32;
        if safe_ctxt.recovery == 0 as i32 {
            safe_ctxt.disableSAX = 1 as i32
        }
    };
}
/*
 * Function to finish the work of the macros where needed.
 */
/* *
 * xmlIsLetter:
 * @c:  an unicode character (int)
 *
 * Check whether the character is allowed by the production
 * [84] Letter ::= BaseChar | Ideographic
 *
 * Returns 0 if not, non-zero otherwise
 */

pub unsafe fn xmlIsLetter(mut c: i32) -> i32 {
    return ((if c < 0x100 as i32 {
        (0x41 as i32 <= c && c <= 0x5a as i32
            || 0x61 as i32 <= c && c <= 0x7a as i32
            || 0xc0 as i32 <= c && c <= 0xd6 as i32
            || 0xd8 as i32 <= c && c <= 0xf6 as i32
            || 0xf8 as i32 <= c) as i32
    } else {
        unsafe { xmlCharInRange(c as u32, &xmlIsBaseCharGroup) }
    }) != 0
        || (if c < 0x100 as i32 {
            0 as i32
        } else {
            (0x4e00 as i32 <= c && c <= 0x9fa5 as i32
                || c == 0x3007 as i32
                || 0x3021 as i32 <= c && c <= 0x3029 as i32) as i32
        }) != 0) as i32;
}

pub unsafe fn check_buffer(mut in_0: xmlParserInputPtr) {
    let mut safe_in_0 = unsafe { &mut *in_0 };
    match () {
        #[cfg(HAVE_parserInternals_DEBUG_INPUT)]
        _ => {
            if safe_in_0.base != xmlBufContent((*safe_in_0.buf).buffer as *const xmlBuf) {
                (safe__xmlGenericError).expect("non-null function pointer")(
                    safe__xmlGenericErrorContext,
                    b"xmlParserInput: base mismatch problem\n\x00" as *const u8 as *const i8,
                );
            }
            if safe_in_0.cur < safe_in_0.base {
                (safe__xmlGenericError).expect("non-null function pointer")(
                    safe__xmlGenericErrorContext,
                    b"xmlParserInput: cur < base problem\n\x00" as *const u8 as *const i8,
                );
            }
            if safe_in_0.cur
                > safe_in_0
                    .base
                    .offset(xmlBufUse((*safe_in_0.buf).buffer) as isize)
            {
                (safe__xmlGenericError).expect("non-null function pointer")(
                    safe__xmlGenericErrorContext,
                    b"xmlParserInput: cur > base + use problem\n\x00" as *const u8 as *const i8,
                );
            }
            (safe__xmlGenericError).expect("non-null function pointer")(
                safe__xmlGenericErrorContext,
                b"buffer %x : content %x, cur %d, use %d\n\x00" as *const u8 as *const i8,
                in_0 as i32,
                xmlBufContent((*safe_in_0.buf).buffer as *const xmlBuf) as i32,
                safe_in_0.cur.offset_from(safe_in_0.base) as i64,
                xmlBufUse((*safe_in_0.buf).buffer),
            );
        }
        #[cfg(not(HAVE_parserInternals_DEBUG_INPUT))]
        _ => {}
    };
}

/* *
 * xmlParserInputRead:
 * @in:  an XML parser input
 * @len:  an indicative size for the lookahead
 *
 * This function was internal and is deprecated.
 *
 * Returns -1 as this is an error to use it.
 */

pub unsafe fn xmlParserInputRead(mut in_0: xmlParserInputPtr, mut len: i32) -> i32 {
    return -(1 as i32);
}
/* *
 * xmlParserInputGrow:
 * @in:  an XML parser input
 * @len:  an indicative size for the lookahead
 *
 * This function increase the input for the parser. It tries to
 * preserve pointers to the input buffer, and keep already read data
 *
 * Returns the amount of char read, or -1 in case of error, 0 indicate the
 * end of this entity
 */

pub unsafe fn xmlParserInputGrow_parserInternals(mut in_0: xmlParserInputPtr, mut len: i32) -> i32 {
    let mut safe_in_0 = unsafe { &mut *in_0 };

    let mut ret: i32 = 0;
    let mut indx: size_t = 0;
    let mut content: *const xmlChar = 0 as *const xmlChar;
    if in_0.is_null() || len < 0 as i32 {
        return -(1 as i32);
    }

    match () {
        #[cfg(HAVE_parserInternals_DEBUG_INPUT)]
        _ => {
            (safe__xmlGenericError).expect("non-null function pointer")(
                safe__xmlGenericErrorContext,
                b"Grow\n\x00" as *const u8 as *const i8,
            );
        }
        #[cfg(not(HAVE_parserInternals_DEBUG_INPUT))]
        _ => {}
    };

    if safe_in_0.buf.is_null() {
        return -(1 as i32);
    }
    if safe_in_0.base.is_null() {
        return -(1 as i32);
    }
    if safe_in_0.cur.is_null() {
        return -(1 as i32);
    }
    unsafe {
        if (*safe_in_0.buf).buffer.is_null() {
            return -(1 as i32);
        }
    }

    check_buffer(in_0);
    unsafe {
        indx = safe_in_0.cur.offset_from(safe_in_0.base) as i64 as size_t;
        if xmlBufUse((*safe_in_0.buf).buffer) > (indx as u32).wrapping_add(250 as i32 as u32) as u64
        {
            check_buffer(in_0);
            return 0 as i32;
        }
        if (*safe_in_0.buf).readcallback.is_some() {
            ret = xmlParserInputBufferGrow(safe_in_0.buf, len)
        } else {
            return 0 as i32;
        }
        /*
         * NOTE : in->base may be a "dangling" i.e. freed pointer in this
         *        block, but we use it really as an integer to do some
         *        pointer arithmetic. Insure will raise it as a bug but in
         *        that specific case, that's not !
         */
        content = xmlBufContent((*safe_in_0.buf).buffer as *const xmlBuf);
        if safe_in_0.base != content {
            /*
             * the buffer has been reallocated
             */
            indx = safe_in_0.cur.offset_from(safe_in_0.base) as i64 as size_t;
            safe_in_0.base = content;
            safe_in_0.cur = &*content.offset(indx as isize) as *const xmlChar
        }
        safe_in_0.end = xmlBufEnd((*safe_in_0.buf).buffer);
    }

    check_buffer(in_0);

    return ret;
}
/* *
 * xmlParserInputShrink:
 * @in:  an XML parser input
 *
 * This function removes used input for the parser.
 */

pub unsafe fn xmlParserInputShrink_parserInternals(mut in_0: xmlParserInputPtr) {
    let mut safe_in_0 = unsafe { &mut *in_0 };

    let mut used: size_t = 0;
    let mut ret: size_t = 0;
    let mut indx: size_t = 0;
    let mut content: *const xmlChar = 0 as *const xmlChar;

    match () {
        #[cfg(HAVE_parserInternals_DEBUG_INPUT)]
        _ => {
            (safe__xmlGenericError).expect("non-null function pointer")(
                safe__xmlGenericErrorContext,
                b"Shrink\n\x00" as *const u8 as *const i8,
            );
        }
        #[cfg(not(HAVE_parserInternals_DEBUG_INPUT))]
        _ => {}
    };

    if in_0.is_null() {
        return;
    }
    if safe_in_0.buf.is_null() {
        return;
    }
    if safe_in_0.base.is_null() {
        return;
    }
    if safe_in_0.cur.is_null() {
        return;
    }
    unsafe {
        if (*safe_in_0.buf).buffer.is_null() {
            return;
        }
    }

    check_buffer(in_0);
    unsafe {
        used = safe_in_0
            .cur
            .offset_from(xmlBufContent((*safe_in_0.buf).buffer as *const xmlBuf))
            as i64 as size_t;
        /*
         * Do not shrink on large buffers whose only a tiny fraction
         * was consumed
         */
        if used > 250 as i32 as u64 {
            ret = xmlBufShrink((*safe_in_0.buf).buffer, used.wrapping_sub(80 as i32 as u64));
            if ret > 0 as i32 as u64 {
                safe_in_0.cur = safe_in_0.cur.offset(-(ret as isize));
                safe_in_0.consumed = safe_in_0.consumed.wrapping_add(ret)
            }
            safe_in_0.end = xmlBufEnd((*safe_in_0.buf).buffer)
        }
    }

    check_buffer(in_0);
    unsafe {
        if xmlBufUse((*safe_in_0.buf).buffer) > 250 as i32 as u64 {
            return;
        }
        xmlParserInputBufferRead(safe_in_0.buf, 2 as i32 * 250 as i32);
        content = xmlBufContent((*safe_in_0.buf).buffer as *const xmlBuf);
        if safe_in_0.base != content {
            /*
             * the buffer has been reallocated
             */
            indx = safe_in_0.cur.offset_from(safe_in_0.base) as i64 as size_t;
            safe_in_0.base = content;
            safe_in_0.cur = &*content.offset(indx as isize) as *const xmlChar
        }
        safe_in_0.end = xmlBufEnd((*safe_in_0.buf).buffer);
    }

    check_buffer(in_0);
}
/* ***********************************************************************
 *									*
 *		UTF8 character input and related functions		*
 *									*
 ************************************************************************/
/* *
 * xmlNextChar:
 * @ctxt:  the XML parser context
 *
 * Skip to the next char input char.
 */

pub unsafe fn xmlNextChar_parserInternals(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut current_block: u64;
    if ctxt.is_null()
        || safe_ctxt.instate as i32 == XML_PARSER_EOF as i32
        || safe_ctxt.input.is_null()
    {
        return;
    }
    unsafe {
        if !((*safe_ctxt.input).cur <= (*safe_ctxt.input).end) {
            xmlErrInternal(
                ctxt,
                b"Parser input data memory error\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
            );
            safe_ctxt.errNo = XML_ERR_INTERNAL_ERROR as i32;
            xmlStopParser(ctxt);
            return;
        }
        if *(*safe_ctxt.input).cur as i32 == 0 as i32
            && xmlParserInputGrow(safe_ctxt.input, 250 as i32) <= 0 as i32
        {
            return;
        }
    }
    if safe_ctxt.charset == XML_CHAR_ENCODING_UTF8 as i32 {
        let mut cur: *const u8 = 0 as *const u8;
        let mut c: u8 = 0;
        /*
         *   2.11 End-of-Line Handling
         *   the literal two-character sequence "#xD#xA" or a standalone
         *   literal #xD, an XML processor must pass to the application
         *   the single character #xA.
         */
        unsafe {
            if *(*safe_ctxt.input).cur as i32 == '\n' as i32 {
                (*safe_ctxt.input).line += 1;
                (*safe_ctxt.input).col = 1 as i32
            } else {
                (*safe_ctxt.input).col += 1
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
            cur = (*safe_ctxt.input).cur;
            c = *cur;
            if c as i32 & 0x80 as i32 != 0 {
                if c as i32 == 0xc0 as i32 {
                    current_block = 14602029473953406903;
                } else {
                    if *cur.offset(1 as i32 as isize) as i32 == 0 as i32 {
                        xmlParserInputGrow(safe_ctxt.input, 250 as i32);
                        cur = (*safe_ctxt.input).cur
                    }
                    if *cur.offset(1 as i32 as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
                        current_block = 14602029473953406903;
                    } else if c as i32 & 0xe0 as i32 == 0xe0 as i32 {
                        let mut val: u32 = 0;
                        if *cur.offset(2 as i32 as isize) as i32 == 0 as i32 {
                            xmlParserInputGrow(safe_ctxt.input, 250 as i32);
                            cur = (*safe_ctxt.input).cur
                        }
                        if *cur.offset(2 as i32 as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
                            current_block = 14602029473953406903;
                        } else {
                            if c as i32 & 0xf0 as i32 == 0xf0 as i32 {
                                if *cur.offset(3 as i32 as isize) as i32 == 0 as i32 {
                                    xmlParserInputGrow(safe_ctxt.input, 250 as i32);
                                    cur = (*safe_ctxt.input).cur
                                }
                                if c as i32 & 0xf8 as i32 != 0xf0 as i32
                                    || *cur.offset(3 as i32 as isize) as i32 & 0xc0 as i32
                                        != 0x80 as i32
                                {
                                    current_block = 14602029473953406903;
                                } else {
                                    /* 4-byte code */
                                    (*safe_ctxt.input).cur =
                                        (*safe_ctxt.input).cur.offset(4 as i32 as isize);
                                    val = ((*cur.offset(0 as i32 as isize) as i32 & 0x7 as i32)
                                        << 18 as i32)
                                        as u32;
                                    val |= ((*cur.offset(1 as i32 as isize) as i32 & 0x3f as i32)
                                        << 12 as i32)
                                        as u32;
                                    val |= ((*cur.offset(2 as i32 as isize) as i32 & 0x3f as i32)
                                        << 6 as i32)
                                        as u32;
                                    val |= (*cur.offset(3 as i32 as isize) as i32 & 0x3f as i32)
                                        as u32;
                                    current_block = 15004371738079956865;
                                }
                            } else {
                                /* 3-byte code */
                                (*safe_ctxt.input).cur =
                                    (*safe_ctxt.input).cur.offset(3 as i32 as isize);
                                val = ((*cur.offset(0 as i32 as isize) as i32 & 0xf as i32)
                                    << 12 as i32) as u32;
                                val |= ((*cur.offset(1 as i32 as isize) as i32 & 0x3f as i32)
                                    << 6 as i32) as u32;
                                val |= (*cur.offset(2 as i32 as isize) as i32 & 0x3f as i32) as u32;
                                current_block = 15004371738079956865;
                            }
                            match current_block {
                                14602029473953406903 => {}
                                _ => {
                                    if val > 0xd7ff as i32 as u32 && val < 0xe000 as i32 as u32
                                        || val > 0xfffd as i32 as u32 && val < 0x10000 as i32 as u32
                                        || val >= 0x110000 as i32 as u32
                                    {
                                        xmlErrEncodingInt(
                                            ctxt,
                                            XML_ERR_INVALID_CHAR,
                                            b"Char 0x%X out of allowed range\n\x00" as *const u8
                                                as *const i8,
                                            val as i32,
                                        );
                                    }
                                    current_block = 6072622540298447352;
                                }
                            }
                        }
                    } else {
                        /* 2-byte code */
                        (*safe_ctxt.input).cur = (*safe_ctxt.input).cur.offset(2 as i32 as isize);
                        current_block = 6072622540298447352;
                    }
                }
                match current_block {
                    6072622540298447352 => {}
                    _ => {
                        /*
                         * If we detect an UTF8 error that probably mean that the
                         * input encoding didn't get properly advertised in the
                         * declaration header. Report the error and switch the encoding
                         * to ISO-Latin-1 (if you don't like this policy, just declare the
                         * encoding !)
                         */
                        if ctxt.is_null()
                            || safe_ctxt.input.is_null()
                            || ((*safe_ctxt.input).end.offset_from((*safe_ctxt.input).cur) as i64)
                                < 4 as i32 as i64
                        {
                            __xmlErrEncoding(
                                ctxt,
                                XML_ERR_INVALID_CHAR,
                                b"Input is not proper UTF-8, indicate encoding !\n\x00" as *const u8
                                    as *const i8,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                            );
                        } else {
                            let mut buffer: [i8; 150] = [0; 150];
                            snprintf(
                                buffer.as_mut_ptr(),
                                149 as i32 as u64,
                                b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8
                                    as *const i8,
                                *(*safe_ctxt.input).cur.offset(0 as i32 as isize) as i32,
                                *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32,
                                *(*safe_ctxt.input).cur.offset(2 as i32 as isize) as i32,
                                *(*safe_ctxt.input).cur.offset(3 as i32 as isize) as i32,
                            );
                            __xmlErrEncoding(
                                ctxt,
                                XML_ERR_INVALID_CHAR,
                                b"Input is not proper UTF-8, indicate encoding !\n%s\x00"
                                    as *const u8 as *const i8,
                                buffer.as_mut_ptr() as *mut xmlChar,
                                0 as *const xmlChar,
                            );
                        }
                        safe_ctxt.charset = XML_CHAR_ENCODING_8859_1 as i32;
                        (*safe_ctxt.input).cur = (*safe_ctxt.input).cur.offset(1);
                        return;
                    }
                }
            } else {
                /* 1-byte code */
                (*safe_ctxt.input).cur = (*safe_ctxt.input).cur.offset(1)
            }
        }
    } else {
        /*
         * Assume it's a fixed length encoding (1) with
         * a compatible encoding for the ASCII set, since
         * XML constructs only use < 128 chars
         */
        unsafe {
            if *(*safe_ctxt.input).cur as i32 == '\n' as i32 {
                (*safe_ctxt.input).line += 1;
                (*safe_ctxt.input).col = 1 as i32
            } else {
                (*safe_ctxt.input).col += 1
            }
            (*safe_ctxt.input).cur = (*safe_ctxt.input).cur.offset(1)
        }
    }
    unsafe {
        if *(*safe_ctxt.input).cur as i32 == 0 as i32 {
            xmlParserInputGrow(safe_ctxt.input, 250 as i32);
        };
    }
}

/* *
 * xmlCurrentChar:
 * @ctxt:  the XML parser context
 * @len:  pointer to the length of the char read
 *
 * The current char value, if using UTF-8 this may actually span multiple
 * bytes in the input buffer. Implement the end of line normalization:
 * 2.11 End-of-Line Handling
 * Wherever an external parsed entity or the literal entity value
 * of an internal parsed entity contains either the literal two-character
 * sequence "#xD#xA" or a standalone literal #xD, an XML processor
 * must pass to the application the single character #xA.
 * This behavior can conveniently be produced by normalizing all
 * line breaks to #xA on input, before parsing.)
 *
 * Returns the current char value and its length
 */

pub unsafe fn xmlCurrentChar(mut ctxt: xmlParserCtxtPtr, mut len: *mut i32) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut current_block: u64;
    if ctxt.is_null() || len.is_null() || safe_ctxt.input.is_null() {
        return 0 as i32;
    }
    if safe_ctxt.instate as i32 == XML_PARSER_EOF as i32 {
        return 0 as i32;
    }
    unsafe {
        if *(*safe_ctxt.input).cur as i32 >= 0x20 as i32
            && *(*safe_ctxt.input).cur as i32 <= 0x7f as i32
        {
            *len = 1 as i32;
            return *(*safe_ctxt.input).cur as i32;
        }
    }
    if safe_ctxt.charset == XML_CHAR_ENCODING_UTF8 as i32 {
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
        unsafe {
            let mut cur: *const u8 = (*safe_ctxt.input).cur;
            let mut c: u8 = 0;
            let mut val: u32 = 0;
            c = *cur;
            if c as i32 & 0x80 as i32 != 0 {
                if !(c as i32 & 0x40 as i32 == 0 as i32 || c as i32 == 0xc0 as i32) {
                    if *cur.offset(1 as i32 as isize) as i32 == 0 as i32 {
                        xmlParserInputGrow(safe_ctxt.input, 250 as i32);
                        cur = (*safe_ctxt.input).cur
                    }
                    if !(*cur.offset(1 as i32 as isize) as i32 & 0xc0 as i32 != 0x80 as i32) {
                        if c as i32 & 0xe0 as i32 == 0xe0 as i32 {
                            if *cur.offset(2 as i32 as isize) as i32 == 0 as i32 {
                                xmlParserInputGrow(safe_ctxt.input, 250 as i32);
                                cur = (*safe_ctxt.input).cur
                            }
                            if *cur.offset(2 as i32 as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
                                current_block = 14852328631030688201;
                            } else if c as i32 & 0xf0 as i32 == 0xf0 as i32 {
                                if *cur.offset(3 as i32 as isize) as i32 == 0 as i32 {
                                    xmlParserInputGrow(safe_ctxt.input, 250 as i32);
                                    cur = (*safe_ctxt.input).cur
                                }
                                if c as i32 & 0xf8 as i32 != 0xf0 as i32
                                    || *cur.offset(3 as i32 as isize) as i32 & 0xc0 as i32
                                        != 0x80 as i32
                                {
                                    current_block = 14852328631030688201;
                                } else {
                                    /* 4-byte code */
                                    *len = 4 as i32;
                                    val = ((*cur.offset(0 as i32 as isize) as i32 & 0x7 as i32)
                                        << 18 as i32)
                                        as u32;
                                    val |= ((*cur.offset(1 as i32 as isize) as i32 & 0x3f as i32)
                                        << 12 as i32)
                                        as u32;
                                    val |= ((*cur.offset(2 as i32 as isize) as i32 & 0x3f as i32)
                                        << 6 as i32)
                                        as u32;
                                    val |= (*cur.offset(3 as i32 as isize) as i32 & 0x3f as i32)
                                        as u32;
                                    if val < 0x10000 as i32 as u32 {
                                        current_block = 14852328631030688201;
                                    } else {
                                        current_block = 3938820862080741272;
                                    }
                                }
                            } else {
                                /* 3-byte code */
                                *len = 3 as i32;
                                val = ((*cur.offset(0 as i32 as isize) as i32 & 0xf as i32)
                                    << 12 as i32) as u32;
                                val |= ((*cur.offset(1 as i32 as isize) as i32 & 0x3f as i32)
                                    << 6 as i32) as u32;
                                val |= (*cur.offset(2 as i32 as isize) as i32 & 0x3f as i32) as u32;
                                if val < 0x800 as i32 as u32 {
                                    current_block = 14852328631030688201;
                                } else {
                                    current_block = 3938820862080741272;
                                }
                            }
                        } else {
                            /* 2-byte code */
                            *len = 2 as i32;
                            val = ((*cur.offset(0 as i32 as isize) as i32 & 0x1f as i32)
                                << 6 as i32) as u32;
                            val |= (*cur.offset(1 as i32 as isize) as i32 & 0x3f as i32) as u32;
                            if val < 0x80 as i32 as u32 {
                                current_block = 14852328631030688201;
                            } else {
                                current_block = 3938820862080741272;
                            }
                        }
                        match current_block {
                            14852328631030688201 => {}
                            _ => {
                                if if val < 0x100 as i32 as u32 {
                                    (0x9 as i32 as u32 <= val && val <= 0xa as i32 as u32
                                        || val == 0xd as i32 as u32
                                        || 0x20 as i32 as u32 <= val)
                                        as i32
                                } else {
                                    (0x100 as i32 as u32 <= val && val <= 0xd7ff as i32 as u32
                                        || 0xe000 as i32 as u32 <= val
                                            && val <= 0xfffd as i32 as u32
                                        || 0x10000 as i32 as u32 <= val
                                            && val <= 0x10ffff as i32 as u32)
                                        as i32
                                } == 0
                                {
                                    xmlErrEncodingInt(
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
                /*
                 * An encoding problem may arise from a truncated input buffer
                 * splitting a character in the middle. In that case do not raise
                 * an error but return 0 to indicate an end of stream problem
                 */
                if ((*safe_ctxt.input).end.offset_from((*safe_ctxt.input).cur) as i64)
                    < 4 as i32 as i64
                {
                    *len = 0 as i32;
                    return 0 as i32;
                }
                /*
                 * If we detect an UTF8 error that probably mean that the
                 * input encoding didn't get properly advertised in the
                 * declaration header. Report the error and switch the encoding
                 * to ISO-Latin-1 (if you don't like this policy, just declare the
                 * encoding !)
                 */
                let mut buffer: [i8; 150] = [0; 150];
                snprintf(
                    &mut *buffer.as_mut_ptr().offset(0 as i32 as isize) as *mut i8,
                    149 as i32 as u64,
                    b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8 as *const i8,
                    *(*safe_ctxt.input).cur.offset(0 as i32 as isize) as i32,
                    *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32,
                    *(*safe_ctxt.input).cur.offset(2 as i32 as isize) as i32,
                    *(*safe_ctxt.input).cur.offset(3 as i32 as isize) as i32,
                );
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"Input is not proper UTF-8, indicate encoding !\n%s\x00" as *const u8
                        as *const i8,
                    buffer.as_mut_ptr() as *mut xmlChar,
                    0 as *const xmlChar,
                );
                safe_ctxt.charset = XML_CHAR_ENCODING_8859_1 as i32;
                *len = 1 as i32;
                return *(*safe_ctxt.input).cur as i32;
            } else {
                /* 1-byte code */
                *len = 1 as i32;
                if *(*safe_ctxt.input).cur as i32 == 0 as i32 {
                    xmlParserInputGrow(safe_ctxt.input, 250 as i32);
                }
                if *(*safe_ctxt.input).cur as i32 == 0 as i32
                    && (*safe_ctxt.input).end > (*safe_ctxt.input).cur
                {
                    xmlErrEncodingInt(
                        ctxt,
                        XML_ERR_INVALID_CHAR,
                        b"Char 0x0 out of allowed range\n\x00" as *const u8 as *const i8,
                        0 as i32,
                    );
                }
                if *(*safe_ctxt.input).cur as i32 == 0xd as i32 {
                    if *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32 == 0xa as i32 {
                        (*safe_ctxt.input).cur = (*safe_ctxt.input).cur.offset(1)
                    }
                    return 0xa as i32;
                }
                return *(*safe_ctxt.input).cur as i32;
            }
        }
    } else {
        /*
         * Assume it's a fixed length encoding (1) with
         * a compatible encoding for the ASCII set, since
         * XML constructs only use < 128 chars
         */
        unsafe {
            *len = 1 as i32;
            if *(*safe_ctxt.input).cur as i32 == 0xd as i32 {
                if *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32 == 0xa as i32 {
                    (*safe_ctxt.input).cur = (*safe_ctxt.input).cur.offset(1)
                }
                return 0xa as i32;
            }
            return *(*safe_ctxt.input).cur as i32;
        }
    };
}
/* *
 * xmlStringCurrentChar:
 * @ctxt:  the XML parser context
 * @cur:  pointer to the beginning of the char
 * @len:  pointer to the length of the char read
 *
 * The current char value, if using UTF-8 this may actually span multiple
 * bytes in the input buffer.
 *
 * Returns the current char value and its length
 */

pub unsafe fn xmlStringCurrentChar(
    mut ctxt: xmlParserCtxtPtr,
    mut cur: *const xmlChar,
    mut len: *mut i32,
) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut current_block: u64;
    if len.is_null() || cur.is_null() {
        return 0 as i32;
    }
    if ctxt.is_null() || safe_ctxt.charset == XML_CHAR_ENCODING_UTF8 as i32 {
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
        let mut c: u8 = 0;
        let mut val: u32 = 0;
        unsafe {
            c = *cur;
            if c as i32 & 0x80 as i32 != 0 {
                if !(*cur.offset(1 as i32 as isize) as i32 & 0xc0 as i32 != 0x80 as i32) {
                    if c as i32 & 0xe0 as i32 == 0xe0 as i32 {
                        if *cur.offset(2 as i32 as isize) as i32 & 0xc0 as i32 != 0x80 as i32 {
                            current_block = 909593556805851584;
                        } else if c as i32 & 0xf0 as i32 == 0xf0 as i32 {
                            if c as i32 & 0xf8 as i32 != 0xf0 as i32
                                || *cur.offset(3 as i32 as isize) as i32 & 0xc0 as i32
                                    != 0x80 as i32
                            {
                                current_block = 909593556805851584;
                            } else {
                                /* 4-byte code */
                                *len = 4 as i32;
                                val = ((*cur.offset(0 as i32 as isize) as i32 & 0x7 as i32)
                                    << 18 as i32) as u32;
                                val |= ((*cur.offset(1 as i32 as isize) as i32 & 0x3f as i32)
                                    << 12 as i32) as u32;
                                val |= ((*cur.offset(2 as i32 as isize) as i32 & 0x3f as i32)
                                    << 6 as i32) as u32;
                                val |= (*cur.offset(3 as i32 as isize) as i32 & 0x3f as i32) as u32;
                                current_block = 11298138898191919651;
                            }
                        } else {
                            /* 3-byte code */
                            *len = 3 as i32;
                            val = ((*cur.offset(0 as i32 as isize) as i32 & 0xf as i32)
                                << 12 as i32) as u32;
                            val |= ((*cur.offset(1 as i32 as isize) as i32 & 0x3f as i32)
                                << 6 as i32) as u32;
                            val |= (*cur.offset(2 as i32 as isize) as i32 & 0x3f as i32) as u32;
                            current_block = 11298138898191919651;
                        }
                    } else {
                        /* 2-byte code */
                        *len = 2 as i32;
                        val = ((*cur.offset(0 as i32 as isize) as i32 & 0x1f as i32) << 6 as i32)
                            as u32;
                        val |= (*cur.offset(1 as i32 as isize) as i32 & 0x3f as i32) as u32;
                        current_block = 11298138898191919651;
                    }
                    match current_block {
                        909593556805851584 => {}
                        _ => {
                            if if val < 0x100 as i32 as u32 {
                                (0x9 as i32 as u32 <= val && val <= 0xa as i32 as u32
                                    || val == 0xd as i32 as u32
                                    || 0x20 as i32 as u32 <= val)
                                    as i32
                            } else {
                                (0x100 as i32 as u32 <= val && val <= 0xd7ff as i32 as u32
                                    || 0xe000 as i32 as u32 <= val && val <= 0xfffd as i32 as u32
                                    || 0x10000 as i32 as u32 <= val
                                        && val <= 0x10ffff as i32 as u32)
                                    as i32
                            } == 0
                            {
                                xmlErrEncodingInt(
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
                /*
                 * An encoding problem may arise from a truncated input buffer
                 * splitting a character in the middle. In that case do not raise
                 * an error but return 0 to indicate an end of stream problem
                 */
                if ctxt.is_null()
                    || safe_ctxt.input.is_null()
                    || ((*safe_ctxt.input).end.offset_from((*safe_ctxt.input).cur) as i64)
                        < 4 as i32 as i64
                {
                    *len = 0 as i32;
                    return 0 as i32;
                }
                /*
                 * If we detect an UTF8 error that probably mean that the
                 * input encoding didn't get properly advertised in the
                 * declaration header. Report the error and switch the encoding
                 * to ISO-Latin-1 (if you don't like this policy, just declare the
                 * encoding !)
                 */
                let mut buffer: [i8; 150] = [0; 150];
                snprintf(
                    buffer.as_mut_ptr(),
                    149 as i32 as u64,
                    b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8 as *const i8,
                    *(*safe_ctxt.input).cur.offset(0 as i32 as isize) as i32,
                    *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32,
                    *(*safe_ctxt.input).cur.offset(2 as i32 as isize) as i32,
                    *(*safe_ctxt.input).cur.offset(3 as i32 as isize) as i32,
                );
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"Input is not proper UTF-8, indicate encoding !\n%s\x00" as *const u8
                        as *const i8,
                    buffer.as_mut_ptr() as *mut xmlChar,
                    0 as *const xmlChar,
                );
                *len = 1 as i32;
                return *cur as i32;
            } else {
                /* 1-byte code */
                *len = 1 as i32;
                return *cur as i32;
            }
        }
    } else {
        /*
         * Assume it's a fixed length encoding (1) with
         * a compatible encoding for the ASCII set, since
         * XML constructs only use < 128 chars
         */
        unsafe {
            *len = 1 as i32;
            return *cur as i32;
        }
    };
}

/* *
 * xmlCopyCharMultiByte:
 * @out:  pointer to an array of xmlChar
 * @val:  the char value
 *
 * append the char value in the array
 *
 * Returns the number of xmlChar written
 */

pub unsafe fn xmlCopyCharMultiByte(mut out: *mut xmlChar, mut val: i32) -> i32 {
    if out.is_null() {
        return 0 as i32;
    }
    /*
     * We are supposed to handle UTF8, check it's valid
     * From rfc2044: encoding of the Unicode values on UTF-8:
     *
     * UCS-4 range (hex.)           UTF-8 octet sequence (binary)
     * 0000 0000-0000 007F   0xxxxxxx
     * 0000 0080-0000 07FF   110xxxxx 10xxxxxx
     * 0000 0800-0000 FFFF   1110xxxx 10xxxxxx 10xxxxxx
     */
    if val >= 0x80 as i32 {
        let mut savedout: *mut xmlChar = out;
        let mut bits: i32 = 0;
        if val < 0x800 as i32 {
            let fresh0 = out;
            unsafe {
                out = out.offset(1);
                *fresh0 = (val >> 6 as i32 | 0xc0 as i32) as xmlChar;
            }

            bits = 0 as i32
        } else if val < 0x10000 as i32 {
            let fresh1 = out;
            unsafe {
                out = out.offset(1);
                *fresh1 = (val >> 12 as i32 | 0xe0 as i32) as xmlChar;
            }

            bits = 6 as i32
        } else if val < 0x110000 as i32 {
            let fresh2 = out;

            unsafe {
                out = out.offset(1);
                *fresh2 = (val >> 18 as i32 | 0xf0 as i32) as xmlChar;
            }
            bits = 12 as i32
        } else {
            unsafe {
                xmlErrEncodingInt(
                    0 as xmlParserCtxtPtr,
                    XML_ERR_INVALID_CHAR,
                    b"Internal error, xmlCopyCharMultiByte 0x%X out of bound\n\x00" as *const u8
                        as *const i8,
                    val,
                );
            }

            return 0 as i32;
        }
        while bits >= 0 as i32 {
            let fresh3 = out;
            unsafe {
                out = out.offset(1);
                *fresh3 = (val >> bits & 0x3f as i32 | 0x80 as i32) as xmlChar;
            }

            bits -= 6 as i32
        }
        unsafe {
            return out.offset_from(savedout) as i64 as i32;
        }
    }
    unsafe {
        *out = val as xmlChar;
    }
    return 1 as i32;
}
/* *
 * xmlCopyChar:
 * @len:  Ignored, compatibility
 * @out:  pointer to an array of xmlChar
 * @val:  the char value
 *
 * append the char value in the array
 *
 * Returns the number of xmlChar written
 */

pub unsafe fn xmlCopyChar_parserInternals(
    mut len: i32,
    mut out: *mut xmlChar,
    mut val: i32,
) -> i32 {
    if out.is_null() {
        return 0 as i32;
    }
    /* the len parameter is ignored */
    if val >= 0x80 as i32 {
        return xmlCopyCharMultiByte(out, val);
    }
    unsafe {
        *out = val as xmlChar;
    }
    return 1 as i32;
}
/* *
 * xmlSwitchEncoding:
 * @ctxt:  the parser context
 * @enc:  the encoding value (number)
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */

pub unsafe fn xmlSwitchEncoding_parserInternals(
    mut ctxt: xmlParserCtxtPtr,
    mut enc: xmlCharEncoding,
) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut handler: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
    let mut len: i32 = -(1 as i32);
    let mut ret: i32 = 0;
    if ctxt.is_null() {
        return -(1 as i32);
    }
    match enc as i32 {
        -1 => {
            unsafe {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_UNKNOWN_ENCODING,
                    b"encoding unknown\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }

            return -(1 as i32);
        }
        0 => {
            /* let's assume it's UTF-8 without the XML decl */
            safe_ctxt.charset = XML_CHAR_ENCODING_UTF8 as i32;
            return 0 as i32;
        }
        1 => {
            /* default encoding, no conversion should be needed */
            safe_ctxt.charset = XML_CHAR_ENCODING_UTF8 as i32;
            /*
             * Errata on XML-1.0 June 20 2001
             * Specific handling of the Byte Order Mark for
             * UTF-8
             */
            unsafe {
                if !safe_ctxt.input.is_null()
                    && *(*safe_ctxt.input).cur.offset(0 as i32 as isize) as i32 == 0xef as i32
                    && *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32 == 0xbb as i32
                    && *(*safe_ctxt.input).cur.offset(2 as i32 as isize) as i32 == 0xbf as i32
                {
                    (*safe_ctxt.input).cur = (*safe_ctxt.input).cur.offset(3 as i32 as isize)
                }
            }

            return 0 as i32;
        }
        2 | 3 => {
            /*The raw input characters are encoded
             *in UTF-16. As we expect this function
             *to be called after xmlCharEncInFunc, we expect
             *ctxt->input->cur to contain UTF-8 encoded characters.
             *So the raw UTF16 Byte Order Mark
             *has also been converted into
             *an UTF-8 BOM. Let's skip that BOM.
             */
            unsafe {
                if !safe_ctxt.input.is_null()
                    && !(*safe_ctxt.input).cur.is_null()
                    && *(*safe_ctxt.input).cur.offset(0 as i32 as isize) as i32 == 0xef as i32
                    && *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32 == 0xbb as i32
                    && *(*safe_ctxt.input).cur.offset(2 as i32 as isize) as i32 == 0xbf as i32
                {
                    (*safe_ctxt.input).cur = (*safe_ctxt.input).cur.offset(3 as i32 as isize)
                }
            }

            len = 90 as i32
        }
        9 => len = 90 as i32,
        5 | 4 | 7 | 8 => len = 180 as i32,
        6 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 22 | 19 | 20 | 21 => len = 45 as i32,
        _ => {}
    }
    unsafe {
        handler = xmlGetCharEncodingHandler(enc);
    }
    if handler.is_null() {
        /*
         * Default handlers.
         */
        unsafe {
            match enc as i32 {
                22 => {
                    /* default encoding, no conversion should be needed */
                    safe_ctxt.charset = XML_CHAR_ENCODING_UTF8 as i32;
                    return 0 as i32;
                }
                4 => {
                    __xmlErrEncoding(
                        ctxt,
                        XML_ERR_UNSUPPORTED_ENCODING,
                        b"encoding not supported %s\n\x00" as *const u8 as *const i8,
                        b"USC4 little endian\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                }
                5 => {
                    __xmlErrEncoding(
                        ctxt,
                        XML_ERR_UNSUPPORTED_ENCODING,
                        b"encoding not supported %s\n\x00" as *const u8 as *const i8,
                        b"USC4 big endian\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                }
                6 => {
                    __xmlErrEncoding(
                        ctxt,
                        XML_ERR_UNSUPPORTED_ENCODING,
                        b"encoding not supported %s\n\x00" as *const u8 as *const i8,
                        b"EBCDIC\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                }
                7 => {
                    __xmlErrEncoding(
                        ctxt,
                        XML_ERR_UNSUPPORTED_ENCODING,
                        b"encoding not supported %s\n\x00" as *const u8 as *const i8,
                        b"UCS4 2143\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                }
                8 => {
                    __xmlErrEncoding(
                        ctxt,
                        XML_ERR_UNSUPPORTED_ENCODING,
                        b"encoding not supported %s\n\x00" as *const u8 as *const i8,
                        b"UCS4 3412\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                }
                9 => {
                    __xmlErrEncoding(
                        ctxt,
                        XML_ERR_UNSUPPORTED_ENCODING,
                        b"encoding not supported %s\n\x00" as *const u8 as *const i8,
                        b"UCS2\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                }
                10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 => {
                    /*
                     * We used to keep the internal content in the
                     * document encoding however this turns being unmaintainable
                     * So xmlGetCharEncodingHandler() will return non-null
                     * values for this now.
                     */
                    if safe_ctxt.inputNr == 1 as i32
                        && safe_ctxt.encoding.is_null()
                        && !safe_ctxt.input.is_null()
                        && !(*safe_ctxt.input).encoding.is_null()
                    {
                        safe_ctxt.encoding = xmlStrdup((*safe_ctxt.input).encoding)
                    }
                    safe_ctxt.charset = enc as i32;
                    return 0 as i32;
                }
                19 => {
                    __xmlErrEncoding(
                        ctxt,
                        XML_ERR_UNSUPPORTED_ENCODING,
                        b"encoding not supported %s\n\x00" as *const u8 as *const i8,
                        b"ISO-2022-JP\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                }
                20 => {
                    __xmlErrEncoding(
                        ctxt,
                        XML_ERR_UNSUPPORTED_ENCODING,
                        b"encoding not supported %s\n\x00" as *const u8 as *const i8,
                        b"Shift_JIS\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                }
                21 => {
                    __xmlErrEncoding(
                        ctxt,
                        XML_ERR_UNSUPPORTED_ENCODING,
                        b"encoding not supported %s\n\x00" as *const u8 as *const i8,
                        b"EUC-JP\x00" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                }
                2 | 3 | _ => {}
            }
        }
    }
    /*
     * TODO: We could recover from errors in external entities if we
     * didn't stop the parser. But most callers of this function don't
     * check the return value.
     */
    if handler.is_null() {
        unsafe {
            xmlStopParser(ctxt);
        }
        return -(1 as i32);
    }
    safe_ctxt.charset = XML_CHAR_ENCODING_UTF8 as i32;
    ret = xmlSwitchToEncodingInt(ctxt, handler, len);
    if ret < 0 as i32 || safe_ctxt.errNo == XML_I18N_CONV_FAILED as i32 {
        /*
         * on encoding conversion errors, stop the parser
         */
        unsafe {
            xmlStopParser(ctxt);
        }
        safe_ctxt.errNo = XML_I18N_CONV_FAILED as i32
    }
    return ret;
}

/* *
 * xmlSwitchInputEncoding:
 * @ctxt:  the parser context
 * @input:  the input stream
 * @handler:  the encoding handler
 * @len:  the number of bytes to convert for the first line or -1
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */
unsafe fn xmlSwitchInputEncodingInt(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputPtr,
    mut handler: xmlCharEncodingHandlerPtr,
    mut len: i32,
) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut nbchars: i32 = 0;
    if handler.is_null() {
        return -(1 as i32);
    }
    if input.is_null() {
        return -(1 as i32);
    }
    let mut safe_input = unsafe { *input };
    unsafe {
        if !safe_input.buf.is_null() {
            if !(*safe_input.buf).encoder.is_null() {
                if (*safe_input.buf).encoder == handler {
                    return 0 as i32;
                }
                xmlCharEncCloseFunc((*safe_input.buf).encoder);
                (*safe_input.buf).encoder = handler;
                return 0 as i32;
            }
            (*safe_input.buf).encoder = handler;
            /*
             * Is there already some content down the pipe to convert ?
             */
            if xmlBufIsEmpty((*safe_input.buf).buffer) == 0 as i32 {
                let mut processed: i32 = 0;
                let mut use_0: u32 = 0;
                /*
                 * Specific handling of the Byte Order Mark for
                 * UTF-16
                 */
                if !(*handler).name.is_null()
                    && (strcmp((*handler).name, b"UTF-16LE\x00" as *const u8 as *const i8) == 0
                        || strcmp((*handler).name, b"UTF-16\x00" as *const u8 as *const i8) == 0)
                    && *safe_input.cur.offset(0 as i32 as isize) as i32 == 0xff as i32
                    && *safe_input.cur.offset(1 as i32 as isize) as i32 == 0xfe as i32
                {
                    safe_input.cur = safe_input.cur.offset(2 as i32 as isize)
                }
                if !(*handler).name.is_null()
                    && strcmp((*handler).name, b"UTF-16BE\x00" as *const u8 as *const i8) == 0
                    && *safe_input.cur.offset(0 as i32 as isize) as i32 == 0xfe as i32
                    && *safe_input.cur.offset(1 as i32 as isize) as i32 == 0xff as i32
                {
                    safe_input.cur = safe_input.cur.offset(2 as i32 as isize)
                }
                /*
                 * Errata on XML-1.0 June 20 2001
                 * Specific handling of the Byte Order Mark for
                 * UTF-8
                 */
                if !(*handler).name.is_null()
                    && strcmp((*handler).name, b"UTF-8\x00" as *const u8 as *const i8) == 0
                    && *safe_input.cur.offset(0 as i32 as isize) as i32 == 0xef as i32
                    && *safe_input.cur.offset(1 as i32 as isize) as i32 == 0xbb as i32
                    && *safe_input.cur.offset(2 as i32 as isize) as i32 == 0xbf as i32
                {
                    safe_input.cur = safe_input.cur.offset(3 as i32 as isize)
                }
                /*
                 * Shrink the current input buffer.
                 * Move it as the raw buffer and create a new input buffer
                 */
                processed = safe_input.cur.offset_from(safe_input.base) as i64 as i32;
                xmlBufShrink((*safe_input.buf).buffer, processed as size_t);
                (*safe_input.buf).raw = (*safe_input.buf).buffer;
                (*safe_input.buf).buffer = xmlBufCreate();
                (*safe_input.buf).rawconsumed = processed as u64;
                use_0 = xmlBufUse((*safe_input.buf).raw) as u32;
                if safe_ctxt.html != 0 {
                    /*
                     * convert as much as possible of the buffer
                     */
                    nbchars = xmlCharEncInput(safe_input.buf, 1 as i32)
                } else {
                    /*
                     * convert just enough to get
                     * '<?xml version="1.0" encoding="xxx"?>'
                     * parsed with the autodetected encoding
                     * into the parser reading buffer.
                     */
                    nbchars = xmlCharEncFirstLineInput(safe_input.buf, len)
                }
                xmlBufResetInput((*safe_input.buf).buffer, input);
                if nbchars < 0 as i32 {
                    xmlErrInternal(
                        ctxt,
                        b"switching encoding: encoder error\n\x00" as *const u8 as *const i8,
                        0 as *const xmlChar,
                    );
                    return -(1 as i32);
                }
                (*safe_input.buf).rawconsumed = (*safe_input.buf)
                    .rawconsumed
                    .wrapping_add((use_0 as u64).wrapping_sub(xmlBufUse((*safe_input.buf).raw)))
            }
            return 0 as i32;
        } else {
            if safe_input.length == 0 as i32 {
                /*
                 * When parsing a static memory array one must know the
                 * size to be able to convert the buffer.
                 */
                xmlErrInternal(
                    ctxt,
                    b"switching encoding : no input\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                );
                /*
                 * Callers assume that the input buffer takes ownership of the
                 * encoding handler. xmlCharEncCloseFunc frees unregistered
                 * handlers and avoids a memory leak.
                 */
                xmlCharEncCloseFunc(handler);
                return -(1 as i32);
            }
        }
        /*
         * We should actually raise an error here, see issue #34.
         */
        xmlCharEncCloseFunc(handler);
    }
    return 0 as i32;
}
/* *
 * xmlSwitchInputEncoding:
 * @ctxt:  the parser context
 * @input:  the input stream
 * @handler:  the encoding handler
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */

pub unsafe fn xmlSwitchInputEncoding(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputPtr,
    mut handler: xmlCharEncodingHandlerPtr,
) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    return xmlSwitchInputEncodingInt(ctxt, input, handler, -(1 as i32));
}
/* ***********************************************************************
 *									*
 *		Commodity functions to switch encodings			*
 *									*
 ************************************************************************/
/* *
 * xmlSwitchToEncodingInt:
 * @ctxt:  the parser context
 * @handler:  the encoding handler
 * @len: the length to convert or -1
 *
 * change the input functions when discovering the character encoding
 * of a given entity, and convert only @len bytes of the output, this
 * is needed on auto detect to allows any declared encoding later to
 * convert the actual content after the xmlDecl
 *
 * Returns 0 in case of success, -1 otherwise
 */
unsafe fn xmlSwitchToEncodingInt(
    mut ctxt: xmlParserCtxtPtr,
    mut handler: xmlCharEncodingHandlerPtr,
    mut len: i32,
) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };

    let mut ret: i32 = 0 as i32;
    if !handler.is_null() {
        unsafe {
            if !safe_ctxt.input.is_null() {
                ret = xmlSwitchInputEncodingInt(ctxt, safe_ctxt.input, handler, len)
            } else {
                xmlErrInternal(
                    ctxt,
                    b"xmlSwitchToEncoding : no input\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                );
                return -(1 as i32);
            }
        }
        /*
         * The parsing is now done in UTF8 natively
         */
        safe_ctxt.charset = XML_CHAR_ENCODING_UTF8 as i32
    } else {
        return -(1 as i32);
    }
    return ret;
}
/* *
 * xmlSwitchToEncoding:
 * @ctxt:  the parser context
 * @handler:  the encoding handler
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */

pub unsafe fn xmlSwitchToEncoding_parserInternals(
    mut ctxt: xmlParserCtxtPtr,
    mut handler: xmlCharEncodingHandlerPtr,
) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };

    return xmlSwitchToEncodingInt(ctxt, handler, -(1 as i32));
}
/* ***********************************************************************
 *									*
 *	Commodity functions to handle entities processing		*
 *									*
 ************************************************************************/
/* *
 * xmlFreeInputStream:
 * @input:  an xmlParserInputPtr
 *
 * Free up an input stream.
 */

pub unsafe fn xmlFreeInputStream_parserInternals(mut input: xmlParserInputPtr) {
    if input.is_null() {
        return;
    }
    let mut safe_input = unsafe { *input };

    if !safe_input.filename.is_null() {
        xmlFree_safe(safe_input.filename as *mut i8 as *mut ());
    }
    if !safe_input.directory.is_null() {
        xmlFree_safe(safe_input.directory as *mut i8 as *mut ());
    }
    if !safe_input.encoding.is_null() {
        xmlFree_safe(safe_input.encoding as *mut i8 as *mut ());
    }
    if !safe_input.version.is_null() {
        xmlFree_safe(safe_input.version as *mut i8 as *mut ());
    }
    unsafe {
        if safe_input.free.is_some() && !safe_input.base.is_null() {
            safe_input.free.expect("non-null function pointer")(safe_input.base as *mut xmlChar);
        }
        if !safe_input.buf.is_null() {
            xmlFreeParserInputBuffer(safe_input.buf);
        }
    }
    xmlFree_safe(input as *mut ());
}

/* *
 * xmlNewInputStream:
 * @ctxt:  an XML parser context
 *
 * Create a new input stream structure.
 *
 * Returns the new input stream or NULL
 */

pub unsafe fn xmlNewInputStream_parserInternals(mut ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr {
    let mut safe_ctxt = unsafe { &mut *ctxt };

    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    unsafe {
        input = xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlParserInput>() as u64,
        ) as xmlParserInputPtr;
        if input.is_null() {
            xmlErrMemory(
                ctxt,
                b"couldn\'t allocate a new input stream\n\x00" as *const u8 as *const i8,
            );
            return 0 as xmlParserInputPtr;
        }
        memset(
            input as *mut (),
            0 as i32,
            ::std::mem::size_of::<xmlParserInput>() as u64,
        );
        (*input).line = 1 as i32;
        (*input).col = 1 as i32;
        (*input).standalone = -(1 as i32);

        /*
         * If the context is NULL the id cannot be initialized, but that
         * should not happen while parsing which is the situation where
         * the id is actually needed.
         */
        if !ctxt.is_null() {
            let fresh4 = safe_ctxt.input_id;
            safe_ctxt.input_id = safe_ctxt.input_id + 1;
            (*input).id = fresh4
        }
    }
    return input;
}
/* *
 * xmlNewIOInputStream:
 * @ctxt:  an XML parser context
 * @input:  an I/O Input
 * @enc:  the charset encoding if known
 *
 * Create a new input stream structure encapsulating the @input into
 * a stream suitable for the parser.
 *
 * Returns the new input stream or NULL
 */

pub unsafe fn xmlNewIOInputStream_parserInternals(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputBufferPtr,
    mut enc: xmlCharEncoding,
) -> xmlParserInputPtr {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut safe__xmlGenericError = unsafe { &mut *__xmlGenericError() };
    let mut safe__xmlGenericErrorContext = unsafe { &mut **__xmlGenericErrorContext() };

    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if input.is_null() {
        return 0 as xmlParserInputPtr;
    }
    unsafe {
        if *__xmlParserDebugEntities() != 0 {
            (safe__xmlGenericError).expect("non-null function pointer")(
                safe__xmlGenericErrorContext,
                b"new input from I/O\n\x00" as *const u8 as *const i8,
            );
        }
    }
    unsafe {
        inputStream = xmlNewInputStream(ctxt);
    }
    if inputStream.is_null() {
        return 0 as xmlParserInputPtr;
    }
    let mut safe_inputStream = unsafe { &mut *inputStream };

    safe_inputStream.filename = 0 as *const i8;
    safe_inputStream.buf = input;
    unsafe {
        xmlBufResetInput((*safe_inputStream.buf).buffer, inputStream);
    }
    if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
        xmlSwitchEncoding_safe(ctxt, enc);
    }
    return inputStream;
}
/* *
 * xmlNewEntityInputStream:
 * @ctxt:  an XML parser context
 * @entity:  an Entity pointer
 *
 * Create a new input stream based on an xmlEntityPtr
 *
 * Returns the new input stream or NULL
 */

pub unsafe fn xmlNewEntityInputStream(
    mut ctxt: xmlParserCtxtPtr,
    mut entity: xmlEntityPtr,
) -> xmlParserInputPtr {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut safe__xmlGenericError = unsafe { &mut *__xmlGenericError() };
    let mut safe__xmlGenericErrorContext = unsafe { &mut **__xmlGenericErrorContext() };

    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    unsafe {
        if entity.is_null() {
            xmlErrInternal(
                ctxt,
                b"xmlNewEntityInputStream entity = NULL\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
            );
            return 0 as xmlParserInputPtr;
        }
        if *__xmlParserDebugEntities() != 0 {
            (safe__xmlGenericError).expect("non-null function pointer")(
                safe__xmlGenericErrorContext,
                b"new input from entity: %s\n\x00" as *const u8 as *const i8,
                (*entity).name,
            );
        }
        if (*entity).content.is_null() {
            match (*entity).etype as u32 {
                3 => {
                    xmlErrInternal(
                        ctxt,
                        b"Cannot parse entity %s\n\x00" as *const u8 as *const i8,
                        (*entity).name,
                    );
                }
                2 | 5 => {
                    return xmlLoadExternalEntity(
                        (*entity).URI as *mut i8,
                        (*entity).ExternalID as *mut i8,
                        ctxt,
                    )
                }
                1 => {
                    xmlErrInternal(
                        ctxt,
                        b"Internal entity %s without content !\n\x00" as *const u8 as *const i8,
                        (*entity).name,
                    );
                }
                4 => {
                    xmlErrInternal(
                        ctxt,
                        b"Internal parameter entity %s without content !\n\x00" as *const u8
                            as *const i8,
                        (*entity).name,
                    );
                }
                6 => {
                    xmlErrInternal(
                        ctxt,
                        b"Predefined entity %s without content !\n\x00" as *const u8 as *const i8,
                        (*entity).name,
                    );
                }
                _ => {}
            }
            return 0 as xmlParserInputPtr;
        }
    }
    unsafe {
        input = xmlNewInputStream(ctxt);
    }
    if input.is_null() {
        return 0 as xmlParserInputPtr;
    }
    unsafe {
        if !(*entity).URI.is_null() {
            (*input).filename = xmlStrdup((*entity).URI as *mut xmlChar) as *mut i8
        }
        (*input).base = (*entity).content;
        if (*entity).length == 0 as i32 {
            (*entity).length = xmlStrlen((*entity).content)
        }
        (*input).cur = (*entity).content;
        (*input).length = (*entity).length;
        (*input).end = &mut *(*entity).content.offset((*input).length as isize) as *mut xmlChar;
    }
    return input;
}

/* *
 * Input Streams.
 */
/* *
 * xmlNewStringInputStream:
 * @ctxt:  an XML parser context
 * @buffer:  an memory buffer
 *
 * Create a new input stream based on a memory buffer.
 * Returns the new input stream
 */

pub unsafe fn xmlNewStringInputStream_parserInternals(
    mut ctxt: xmlParserCtxtPtr,
    mut buffer: *const xmlChar,
) -> xmlParserInputPtr {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut safe__xmlGenericError = unsafe { &mut *__xmlGenericError() };
    let mut safe__xmlGenericErrorContext = unsafe { &mut **__xmlGenericErrorContext() };

    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    unsafe {
        if buffer.is_null() {
            xmlErrInternal(
                ctxt,
                b"xmlNewStringInputStream string = NULL\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
            );
            return 0 as xmlParserInputPtr;
        }
        if *__xmlParserDebugEntities() != 0 {
            (safe__xmlGenericError).expect("non-null function pointer")(
                safe__xmlGenericErrorContext,
                b"new fixed input: %.30s\n\x00" as *const u8 as *const i8,
                buffer,
            );
        }
        input = xmlNewInputStream(ctxt);
        if input.is_null() {
            xmlErrMemory(
                ctxt,
                b"couldn\'t allocate a new input stream\n\x00" as *const u8 as *const i8,
            );
            return 0 as xmlParserInputPtr;
        }
    }
    // let mut safe_input = unsafe { *input };
    unsafe {
        (*input).base = buffer;
        (*input).cur = buffer;
        (*input).length = xmlStrlen_safe(buffer);
        (*input).end = &*buffer.offset((*input).length as isize) as *const xmlChar;
    }
    return input;
}

/* *
 * xmlNewInputFromFile:
 * @ctxt:  an XML parser context
 * @filename:  the filename to use as entity
 *
 * Create a new input stream based on a file or an URL.
 *
 * Returns the new input stream or NULL in case of error
 */

pub unsafe fn xmlNewInputFromFile(
    mut ctxt: xmlParserCtxtPtr,
    mut filename: *const i8,
) -> xmlParserInputPtr {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut safe__xmlGenericError = unsafe { &mut *__xmlGenericError() };
    let mut safe__xmlGenericErrorContext = unsafe { &mut **__xmlGenericErrorContext() };

    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut directory: *mut i8 = 0 as *mut i8;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    unsafe {
        if *__xmlParserDebugEntities() != 0 {
            (safe__xmlGenericError).expect("non-null function pointer")(
                safe__xmlGenericErrorContext,
                b"new input from file: %s\n\x00" as *const u8 as *const i8,
                filename,
            );
        }
    }
    if ctxt.is_null() {
        return 0 as xmlParserInputPtr;
    }
    unsafe {
        buf = xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
        if buf.is_null() {
            if filename.is_null() {
                __xmlLoaderErr(
                    ctxt as *mut (),
                    b"failed to load external entity: NULL filename \n\x00" as *const u8
                        as *const i8,
                    0 as *const i8,
                );
            } else {
                __xmlLoaderErr(
                    ctxt as *mut (),
                    b"failed to load external entity \"%s\"\n\x00" as *const u8 as *const i8,
                    filename,
                );
            }
            return 0 as xmlParserInputPtr;
        }
        inputStream = xmlNewInputStream(ctxt);
    }
    if inputStream.is_null() {
        return 0 as xmlParserInputPtr;
    }
    let mut safe_inputStream = unsafe { &mut *inputStream };

    safe_inputStream.buf = buf;
    unsafe {
        inputStream = xmlCheckHTTPInput(ctxt, inputStream);
    }
    if inputStream.is_null() {
        return 0 as xmlParserInputPtr;
    }
    if safe_inputStream.filename.is_null() {
        URI = xmlStrdup_safe(filename as *mut xmlChar)
    } else {
        URI = xmlStrdup_safe(safe_inputStream.filename as *mut xmlChar)
    }
    directory = xmlParserGetDirectory_safe(URI as *const i8);
    if !safe_inputStream.filename.is_null() {
        xmlFree_safe(safe_inputStream.filename as *mut i8 as *mut ());
    }
    safe_inputStream.filename = xmlCanonicPath_safe(URI as *const xmlChar) as *mut i8;
    if !URI.is_null() {
        xmlFree_safe(URI as *mut i8 as *mut ());
    }
    safe_inputStream.directory = directory;
    unsafe {
        xmlBufResetInput((*safe_inputStream.buf).buffer, inputStream);
    }
    if safe_ctxt.directory.is_null() && !directory.is_null() {
        safe_ctxt.directory = xmlStrdup_safe(directory as *const xmlChar) as *mut i8
    }
    return inputStream;
}
/* ***********************************************************************
 *									*
 *		Commodity functions to handle parser contexts		*
 *									*
 ************************************************************************/
/* *
 * xmlInitParserCtxt:
 * @ctxt:  an XML parser context
 *
 * Initialize a parser context
 *
 * Returns 0 in case of success and -1 in case of error
 */

pub unsafe fn xmlInitParserCtxt(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };

    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ctxt.is_null() {
        unsafe {
            xmlErrInternal(
                0 as xmlParserCtxtPtr,
                b"Got NULL parser context\n\x00" as *const u8 as *const i8,
                0 as *const xmlChar,
            );
        }
        return -(1 as i32);
    }
    xmlDefaultSAXHandlerInit_safe();
    unsafe {
        if safe_ctxt.dict.is_null() {
            safe_ctxt.dict = xmlDictCreate()
        }
    }
    if safe_ctxt.dict.is_null() {
        unsafe {
            xmlErrMemory(
                0 as xmlParserCtxtPtr,
                b"cannot initialize parser context\n\x00" as *const u8 as *const i8,
            );
        }
        return -(1 as i32);
    }
    xmlDictSetLimit_safe(safe_ctxt.dict, 10000000 as i32 as size_t);
    unsafe {
        if safe_ctxt.sax.is_null() {
            safe_ctxt.sax = xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<
                xmlSAXHandler,
            >() as u64) as *mut xmlSAXHandler
        }
        if safe_ctxt.sax.is_null() {
            xmlErrMemory(
                0 as xmlParserCtxtPtr,
                b"cannot initialize parser context\n\x00" as *const u8 as *const i8,
            );
            return -(1 as i32);
        } else {
            xmlSAXVersion(safe_ctxt.sax, 2 as i32);
        }
    }
    safe_ctxt.maxatts = 0 as i32;
    safe_ctxt.atts = 0 as *mut *const xmlChar;
    /* Allocate the Input stack */
    unsafe {
        if safe_ctxt.inputTab.is_null() {
            safe_ctxt.inputTab = xmlMalloc.expect("non-null function pointer")(
                (5 as i32 as u64).wrapping_mul(::std::mem::size_of::<xmlParserInputPtr>() as u64),
            ) as *mut xmlParserInputPtr;
            safe_ctxt.inputMax = 5 as i32
        }
        if safe_ctxt.inputTab.is_null() {
            xmlErrMemory(
                0 as xmlParserCtxtPtr,
                b"cannot initialize parser context\n\x00" as *const u8 as *const i8,
            );
            safe_ctxt.inputNr = 0 as i32;
            safe_ctxt.inputMax = 0 as i32;
            safe_ctxt.input = 0 as xmlParserInputPtr;
            return -(1 as i32);
        }
    }
    loop {
        input = inputPop_safe(ctxt);
        if input.is_null() {
            break;
        }
        /* Non consuming */
        unsafe {
            xmlFreeInputStream(input);
        }
    }
    safe_ctxt.inputNr = 0 as i32;
    safe_ctxt.input = 0 as xmlParserInputPtr;
    safe_ctxt.version = 0 as *const xmlChar;
    safe_ctxt.encoding = 0 as *const xmlChar;
    safe_ctxt.standalone = -(1 as i32);
    safe_ctxt.hasExternalSubset = 0 as i32;
    safe_ctxt.hasPErefs = 0 as i32;
    safe_ctxt.html = 0 as i32;
    safe_ctxt.external = 0 as i32;
    safe_ctxt.instate = XML_PARSER_START;
    safe_ctxt.token = 0 as i32;
    safe_ctxt.directory = 0 as *mut i8;
    /* Allocate the Node stack */
    unsafe {
        if safe_ctxt.nodeTab.is_null() {
            safe_ctxt.nodeTab = xmlMalloc.expect("non-null function pointer")(
                (10 as i32 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            ) as *mut xmlNodePtr;
            safe_ctxt.nodeMax = 10 as i32
        }
        if safe_ctxt.nodeTab.is_null() {
            xmlErrMemory(
                0 as xmlParserCtxtPtr,
                b"cannot initialize parser context\n\x00" as *const u8 as *const i8,
            );
            safe_ctxt.nodeNr = 0 as i32;
            safe_ctxt.nodeMax = 0 as i32;
            safe_ctxt.node = 0 as xmlNodePtr;
            safe_ctxt.inputNr = 0 as i32;
            safe_ctxt.inputMax = 0 as i32;
            safe_ctxt.input = 0 as xmlParserInputPtr;
            return -(1 as i32);
        }
    }
    safe_ctxt.nodeNr = 0 as i32;
    safe_ctxt.node = 0 as xmlNodePtr;
    /* Allocate the Name stack */
    unsafe {
        if safe_ctxt.nameTab.is_null() {
            safe_ctxt.nameTab = xmlMalloc.expect("non-null function pointer")(
                (10 as i32 as u64).wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as u64),
            ) as *mut *const xmlChar;
            safe_ctxt.nameMax = 10 as i32
        }
        if safe_ctxt.nameTab.is_null() {
            xmlErrMemory(
                0 as xmlParserCtxtPtr,
                b"cannot initialize parser context\n\x00" as *const u8 as *const i8,
            );
            safe_ctxt.nodeNr = 0 as i32;
            safe_ctxt.nodeMax = 0 as i32;
            safe_ctxt.node = 0 as xmlNodePtr;
            safe_ctxt.inputNr = 0 as i32;
            safe_ctxt.inputMax = 0 as i32;
            safe_ctxt.input = 0 as xmlParserInputPtr;
            safe_ctxt.nameNr = 0 as i32;
            safe_ctxt.nameMax = 0 as i32;
            safe_ctxt.name = 0 as *const xmlChar;
            return -(1 as i32);
        }
    }
    safe_ctxt.nameNr = 0 as i32;
    safe_ctxt.name = 0 as *const xmlChar;
    /* Allocate the space stack */
    unsafe {
        if safe_ctxt.spaceTab.is_null() {
            safe_ctxt.spaceTab = xmlMalloc.expect("non-null function pointer")(
                (10 as i32 as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64),
            ) as *mut i32;
            safe_ctxt.spaceMax = 10 as i32
        }
        if safe_ctxt.spaceTab.is_null() {
            xmlErrMemory(
                0 as xmlParserCtxtPtr,
                b"cannot initialize parser context\n\x00" as *const u8 as *const i8,
            );
            safe_ctxt.nodeNr = 0 as i32;
            safe_ctxt.nodeMax = 0 as i32;
            safe_ctxt.node = 0 as xmlNodePtr;
            safe_ctxt.inputNr = 0 as i32;
            safe_ctxt.inputMax = 0 as i32;
            safe_ctxt.input = 0 as xmlParserInputPtr;
            safe_ctxt.nameNr = 0 as i32;
            safe_ctxt.nameMax = 0 as i32;
            safe_ctxt.name = 0 as *const xmlChar;
            safe_ctxt.spaceNr = 0 as i32;
            safe_ctxt.spaceMax = 0 as i32;
            safe_ctxt.space = 0 as *mut i32;
            return -(1 as i32);
        }
    }
    safe_ctxt.spaceNr = 1 as i32;
    safe_ctxt.spaceMax = 10 as i32;
    unsafe {
        *safe_ctxt.spaceTab.offset(0 as i32 as isize) = -(1 as i32);
        safe_ctxt.space = &mut *safe_ctxt.spaceTab.offset(0 as i32 as isize) as *mut i32;
    }
    safe_ctxt.userData = ctxt as *mut ();
    safe_ctxt.myDoc = 0 as xmlDocPtr;
    safe_ctxt.wellFormed = 1 as i32;
    safe_ctxt.nsWellFormed = 1 as i32;
    safe_ctxt.valid = 1 as i32;
    unsafe {
        safe_ctxt.loadsubset = *__xmlLoadExtDtdDefaultValue();
    }
    if safe_ctxt.loadsubset != 0 {
        safe_ctxt.options |= XML_PARSE_DTDLOAD as i32
    }
    unsafe {
        safe_ctxt.validate = *__xmlDoValidityCheckingDefaultValue();
        safe_ctxt.pedantic = *__xmlPedanticParserDefaultValue();

        if safe_ctxt.pedantic != 0 {
            safe_ctxt.options |= XML_PARSE_PEDANTIC as i32
        }
        safe_ctxt.linenumbers = *__xmlLineNumbersDefaultValue();
        safe_ctxt.keepBlanks = *__xmlKeepBlanksDefaultValue();
        if safe_ctxt.keepBlanks == 0 as i32 {
            (*safe_ctxt.sax).ignorableWhitespace = Some(
                xmlSAX2IgnorableWhitespace
                    as unsafe extern "C" fn(_: *mut (), _: *const xmlChar, _: i32) -> (),
            );
            safe_ctxt.options |= XML_PARSE_NOBLANKS as i32
        }
    }
    safe_ctxt.vctxt.finishDtd = 0xabcd1234 as u32;
    safe_ctxt.vctxt.userData = ctxt as *mut ();
    safe_ctxt.vctxt.error = Some(
        xmlParserValidityError as unsafe extern "C" fn(_: *mut (), _: *const i8, _: ...) -> (),
    );
    safe_ctxt.vctxt.warning = Some(
        xmlParserValidityWarning as unsafe extern "C" fn(_: *mut (), _: *const i8, _: ...) -> (),
    );
    if safe_ctxt.validate != 0 {
        unsafe {
            if *__xmlGetWarningsDefaultValue() == 0 as i32 {
                safe_ctxt.vctxt.warning = None
            } else {
                safe_ctxt.vctxt.warning = Some(
                    xmlParserValidityWarning
                        as unsafe extern "C" fn(_: *mut (), _: *const i8, _: ...) -> (),
                )
            }
        }
        safe_ctxt.vctxt.nodeMax = 0 as i32;
        safe_ctxt.options |= XML_PARSE_DTDVALID as i32
    }
    unsafe {
        safe_ctxt.replaceEntities = *__xmlSubstituteEntitiesDefaultValue();
    }
    if safe_ctxt.replaceEntities != 0 {
        safe_ctxt.options |= XML_PARSE_NOENT as i32
    }
    safe_ctxt.record_info = 0 as i32;
    safe_ctxt.checkIndex = 0 as i32 as i64;
    safe_ctxt.inSubset = 0 as i32;
    safe_ctxt.errNo = XML_ERR_OK as i32;
    safe_ctxt.depth = 0 as i32;
    safe_ctxt.charset = XML_CHAR_ENCODING_UTF8 as i32;
    safe_ctxt.catalogs = 0 as *mut ();
    safe_ctxt.nbentities = 0 as i32 as u64;
    safe_ctxt.sizeentities = 0 as i32 as u64;
    safe_ctxt.sizeentcopy = 0 as i32 as u64;
    safe_ctxt.input_id = 1 as i32;
    unsafe {
        xmlInitNodeInfoSeq(&mut safe_ctxt.node_seq);
    }
    return 0 as i32;
}
/* *
 * xmlFreeParserCtxt:
 * @ctxt:  an XML parser context
 *
 * Free all the memory used by a parser context. However the parsed
 * document in ctxt->myDoc is not freed.
 */

pub unsafe fn xmlFreeParserCtxt_parserInternals(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };

    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ctxt.is_null() {
        return;
    }
    loop {
        input = inputPop_safe(ctxt);
        if input.is_null() {
            break;
        }
        /* Non consuming */
        xmlFreeInputStream_safe(input);
    }
    if !safe_ctxt.spaceTab.is_null() {
        xmlFree_safe(safe_ctxt.spaceTab as *mut ());
    }
    if !safe_ctxt.nameTab.is_null() {
        xmlFree_safe(safe_ctxt.nameTab as *mut *mut xmlChar as *mut ());
    }
    if !safe_ctxt.nodeTab.is_null() {
        xmlFree_safe(safe_ctxt.nodeTab as *mut ());
    }
    if !safe_ctxt.nodeInfoTab.is_null() {
        xmlFree_safe(safe_ctxt.nodeInfoTab as *mut ());
    }
    if !safe_ctxt.inputTab.is_null() {
        xmlFree_safe(safe_ctxt.inputTab as *mut ());
    }
    if !safe_ctxt.version.is_null() {
        xmlFree_safe(safe_ctxt.version as *mut i8 as *mut ());
    }
    if !safe_ctxt.encoding.is_null() {
        xmlFree_safe(safe_ctxt.encoding as *mut i8 as *mut ());
    }
    if !safe_ctxt.extSubURI.is_null() {
        xmlFree_safe(safe_ctxt.extSubURI as *mut i8 as *mut ());
    }
    if !safe_ctxt.extSubSystem.is_null() {
        xmlFree_safe(safe_ctxt.extSubSystem as *mut i8 as *mut ());
    }
    match () {
        #[cfg(HAVE_parserInternals_LIBXML_SAX1_ENABLED)]
        _ => {
            if !safe_ctxt.sax.is_null()
                && safe_ctxt.sax != __xmlDefaultSAXHandler_safe() as xmlSAXHandlerPtr
            {
                /* LIBXML_SAX1_ENABLED */
                xmlFree_safe(safe_ctxt.sax as *mut ());
            }
        }
        #[cfg(not(HAVE_parserInternals_LIBXML_SAX1_ENABLED))]
        _ => {
            if !safe_ctxt.sax.is_null() {
                /* LIBXML_SAX1_ENABLED */
                xmlFree_safe(safe_ctxt.sax as *mut ());
            }
        }
    };
    if !safe_ctxt.directory.is_null() {
        xmlFree_safe(safe_ctxt.directory as *mut ());
    }
    if !safe_ctxt.vctxt.nodeTab.is_null() {
        xmlFree_safe(safe_ctxt.vctxt.nodeTab as *mut ());
    }
    if !safe_ctxt.atts.is_null() {
        xmlFree_safe(safe_ctxt.atts as *mut *mut xmlChar as *mut ());
    }
    if !safe_ctxt.dict.is_null() {
        xmlDictFree_safe(safe_ctxt.dict);
    }
    if !safe_ctxt.nsTab.is_null() {
        xmlFree_safe(safe_ctxt.nsTab as *mut i8 as *mut ());
    }
    if !safe_ctxt.pushTab.is_null() {
        xmlFree_safe(safe_ctxt.pushTab as *mut ());
    }
    if !safe_ctxt.attallocs.is_null() {
        xmlFree_safe(safe_ctxt.attallocs as *mut ());
    }
    if !safe_ctxt.attsDefault.is_null() {
        xmlHashFree_safe(
            safe_ctxt.attsDefault,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
            ),
        );
    }
    if !safe_ctxt.attsSpecial.is_null() {
        xmlHashFree_safe(safe_ctxt.attsSpecial, None);
    }
    if !safe_ctxt.freeElems.is_null() {
        let mut cur: xmlNodePtr = 0 as *mut xmlNode;
        let mut next: xmlNodePtr = 0 as *mut xmlNode;
        cur = safe_ctxt.freeElems;
        unsafe {
            while !cur.is_null() {
                next = (*cur).next;
                xmlFree_safe(cur as *mut ());
                cur = next
            }
        }
    }
    if !safe_ctxt.freeAttrs.is_null() {
        let mut cur_0: xmlAttrPtr = 0 as *mut xmlAttr;
        let mut next_0: xmlAttrPtr = 0 as *mut xmlAttr;
        cur_0 = safe_ctxt.freeAttrs;
        unsafe {
            while !cur_0.is_null() {
                next_0 = (*cur_0).next;
                xmlFree.expect("non-null function pointer")(cur_0 as *mut ());
                cur_0 = next_0
            }
        }
    }
    /*
     * cleanup the error strings
     */
    if !safe_ctxt.lastError.message.is_null() {
        xmlFree_safe(safe_ctxt.lastError.message as *mut ());
    }
    if !safe_ctxt.lastError.file.is_null() {
        xmlFree_safe(safe_ctxt.lastError.file as *mut ());
    }
    if !safe_ctxt.lastError.str1.is_null() {
        xmlFree_safe(safe_ctxt.lastError.str1 as *mut ());
    }
    if !safe_ctxt.lastError.str2.is_null() {
        xmlFree_safe(safe_ctxt.lastError.str2 as *mut ());
    }
    if !safe_ctxt.lastError.str3.is_null() {
        xmlFree_safe(safe_ctxt.lastError.str3 as *mut ());
    }

    match () {
        #[cfg(HAVE_parserInternals_LIBXML_CATALOG_ENABLED)]
        _ => {
            if !safe_ctxt.catalogs.is_null() {
                xmlCatalogFreeLocal_safe(safe_ctxt.catalogs);
            }
        }
        #[cfg(not(HAVE_parserInternals_LIBXML_CATALOG_ENABLED))]
        _ => {}
    };

    xmlFree_safe(ctxt as *mut ());
}
/* *
 * xmlNewParserCtxt:
 *
 * Allocate and initialize a new parser context.
 *
 * Returns the xmlParserCtxtPtr or NULL
 */

pub unsafe fn xmlNewParserCtxt_parserInternals() -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let res1: xmlParserCtxtPtr = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlParserCtxt>() as u64)
            as xmlParserCtxtPtr
    };
    ctxt = res1;
    unsafe {
        if ctxt.is_null() {
            xmlErrMemory(
                0 as xmlParserCtxtPtr,
                b"cannot allocate parser context\n\x00" as *const u8 as *const i8,
            );
            return 0 as xmlParserCtxtPtr;
        }
        memset(
            ctxt as *mut (),
            0 as i32,
            ::std::mem::size_of::<xmlParserCtxt>() as u64,
        );
        if xmlInitParserCtxt(ctxt) < 0 as i32 {
            xmlFreeParserCtxt(ctxt);
            return 0 as xmlParserCtxtPtr;
        }
    }
    return ctxt;
}
/* ***********************************************************************
 *									*
 *		Handling of node information				*
 *									*
 ************************************************************************/
/* *
 * xmlClearParserCtxt:
 * @ctxt:  an XML parser context
 *
 * Clear (release owned resources) and reinitialize a parser context
 */

pub unsafe fn xmlClearParserCtxt(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };

    if ctxt.is_null() {
        return;
    }
    xmlClearNodeInfoSeq(&mut safe_ctxt.node_seq);
    xmlCtxtReset_safe(ctxt);
}
/* *
 * xmlParserFindNodeInfo:
 * @ctx:  an XML parser context
 * @node:  an XML node within the tree
 *
 * Find the parser node info struct for a given node
 *
 * Returns an xmlParserNodeInfo block pointer or NULL
 */

pub unsafe fn xmlParserFindNodeInfo(
    ctxt: xmlParserCtxtPtr,
    node: xmlNodePtr,
) -> *const xmlParserNodeInfo {
    let mut safe_ctxt = unsafe { &mut *ctxt };

    let mut pos: u64 = 0;
    if ctxt.is_null() || node.is_null() {
        return 0 as *const xmlParserNodeInfo;
    }
    /* Find position where node should be at */
    pos = xmlParserFindNodeInfoIndex(&mut safe_ctxt.node_seq, node);
    unsafe {
        if pos < safe_ctxt.node_seq.length
            && (*safe_ctxt.node_seq.buffer.offset(pos as isize)).node == node as *const _xmlNode
        {
            return &mut *safe_ctxt.node_seq.buffer.offset(pos as isize) as *mut xmlParserNodeInfo;
        } else {
            return 0 as *const xmlParserNodeInfo;
        }
    };
}
/* *
 * xmlInitNodeInfoSeq:
 * @seq:  a node info sequence pointer
 *
 * -- Initialize (set to initial state) node info sequence
 */

pub unsafe fn xmlInitNodeInfoSeq_parserInternals(mut seq: xmlParserNodeInfoSeqPtr) {
    if seq.is_null() {
        return;
    }
    let safe_seq = unsafe { &mut *seq };
    safe_seq.length = 0 as i32 as u64;
    safe_seq.maximum = 0 as i32 as u64;
    safe_seq.buffer = 0 as *mut xmlParserNodeInfo;
}
/* *
 * xmlClearNodeInfoSeq:
 * @seq:  a node info sequence pointer
 *
 * -- Clear (release memory and reinitialize) node
 *   info sequence
 */

pub unsafe fn xmlClearNodeInfoSeq(mut seq: xmlParserNodeInfoSeqPtr) {
    if seq.is_null() {
        return;
    }
    unsafe {
        if !(*seq).buffer.is_null() {
            xmlFree_safe((*seq).buffer as *mut ());
        }
    }
    xmlInitNodeInfoSeq_safe(seq);
}
/* *
 * xmlParserFindNodeInfoIndex:
 * @seq:  a node info sequence pointer
 * @node:  an XML node pointer
 *
 *
 * xmlParserFindNodeInfoIndex : Find the index that the info record for
 *   the given node is or should be at in a sorted sequence
 *
 * Returns a long indicating the position of the record
 */

pub unsafe fn xmlParserFindNodeInfoIndex(seq: xmlParserNodeInfoSeqPtr, node: xmlNodePtr) -> u64 {
    let mut upper: u64 = 0;
    let mut lower: u64 = 0;
    let mut middle: u64 = 0;
    let mut found: i32 = 0 as i32;
    if seq.is_null() || node.is_null() {
        return -(1 as i32) as u64;
    }
    /* Do a binary search for the key */
    lower = 1 as i32 as u64;
    let res1 = unsafe { (*seq).length };
    upper = res1;
    middle = 0 as i32 as u64;
    while lower <= upper && found == 0 {
        middle = lower.wrapping_add(upper.wrapping_sub(lower).wrapping_div(2 as i32 as u64));
        let res2 = unsafe {
            (*(*seq)
                .buffer
                .offset(middle.wrapping_sub(1 as i32 as u64) as isize))
            .node as xmlNodePtr
        };
        if node == res2 {
            found = 1 as i32
        } else if node < res2 {
            upper = middle.wrapping_sub(1 as i32 as u64)
        } else {
            lower = middle.wrapping_add(1 as i32 as u64)
        }
    }
    /* Return position */
    if middle == 0 as i32 as u64
        || unsafe {
            (*(*seq)
                .buffer
                .offset(middle.wrapping_sub(1 as i32 as u64) as isize))
            .node
                < node as *const _xmlNode
        }
    {
        return middle;
    } else {
        return middle.wrapping_sub(1 as i32 as u64);
    };
}
/*
 * Recovery mode
 */
/* LIBXML_SAX1_ENABLED */
/*
 * Less common routines and SAX interfaces
 */
/* LIBXML_SAX1_ENABLED */
/* LIBXML_VALID_ENABLE */
/* LIBXML_SAX1_ENABLED */
/* LIBXML_SAX1_ENABLED */
/*
 * Parser contexts handling.
 */
/* LIBXML_SAX1_ENABLED */
/*
 * Reading/setting optional parsing features.
 */
/* LIBXML_LEGACY_ENABLED */
/*
 * Interfaces for the Push mode.
 */
/* LIBXML_PUSH_ENABLED */
/*
 * Special I/O mode.
 */
/*
 * Node infos.
 */
/* *
 * xmlParserAddNodeInfo:
 * @ctxt:  an XML parser context
 * @info:  a node info sequence pointer
 *
 * Insert node info record into the sorted sequence
 */

pub unsafe fn xmlParserAddNodeInfo_parserInternals(
    mut ctxt: xmlParserCtxtPtr,
    info: xmlParserNodeInfoPtr,
) {
    let mut safe_ctxt = unsafe { &mut *ctxt };

    let mut pos: u64 = 0;
    if ctxt.is_null() || info.is_null() {
        return;
    }
    /* Find pos and check to see if node is already in the sequence */
    let res1: u64 =
        unsafe { xmlParserFindNodeInfoIndex(&mut safe_ctxt.node_seq, (*info).node as xmlNodePtr) };
    pos = res1;
    if pos < safe_ctxt.node_seq.length
        && !safe_ctxt.node_seq.buffer.is_null()
        && unsafe { (*safe_ctxt.node_seq.buffer.offset(pos as isize)).node == (*info).node }
    {
        unsafe { *safe_ctxt.node_seq.buffer.offset(pos as isize) = *info }
    } else {
        /* Otherwise, we need to add new node to buffer */
        if safe_ctxt.node_seq.length.wrapping_add(1 as i32 as u64) > safe_ctxt.node_seq.maximum
            || safe_ctxt.node_seq.buffer.is_null()
        {
            let mut tmp_buffer: *mut xmlParserNodeInfo = 0 as *mut xmlParserNodeInfo;
            let mut byte_size: u32 = 0;
            if safe_ctxt.node_seq.maximum == 0 as i32 as u64 {
                safe_ctxt.node_seq.maximum = 2 as i32 as u64
            }
            byte_size = (::std::mem::size_of::<xmlParserNodeInfo>() as u64)
                .wrapping_mul((2 as i32 as u64).wrapping_mul(safe_ctxt.node_seq.maximum))
                as u32;
            if safe_ctxt.node_seq.buffer.is_null() {
                let res2: *mut xmlParserNodeInfo = unsafe {
                    xmlMalloc.expect("non-null function pointer")(byte_size as size_t)
                        as *mut xmlParserNodeInfo
                };
                tmp_buffer = res2;
            } else {
                let res3: *mut xmlParserNodeInfo = unsafe {
                    xmlRealloc.expect("non-null function pointer")(
                        safe_ctxt.node_seq.buffer as *mut (),
                        byte_size as size_t,
                    ) as *mut xmlParserNodeInfo
                };
                tmp_buffer = res3;
            }
            if tmp_buffer.is_null() {
                unsafe {
                    xmlErrMemory(
                        ctxt,
                        b"failed to allocate buffer\n\x00" as *const u8 as *const i8,
                    )
                };
                return;
            }
            safe_ctxt.node_seq.buffer = tmp_buffer;
            safe_ctxt.node_seq.maximum = safe_ctxt.node_seq.maximum.wrapping_mul(2 as i32 as u64)
        }
        /* If position is not at end, move elements out of the way */
        if pos != safe_ctxt.node_seq.length {
            let mut i: u64 = 0;
            i = safe_ctxt.node_seq.length;
            while i > pos {
                unsafe {
                    *safe_ctxt.node_seq.buffer.offset(i as isize) = *safe_ctxt
                        .node_seq
                        .buffer
                        .offset(i.wrapping_sub(1 as i32 as u64) as isize)
                };
                i = i.wrapping_sub(1)
            }
        }
        /* Copy element and increase length */
        unsafe { *safe_ctxt.node_seq.buffer.offset(pos as isize) = *info };
        safe_ctxt.node_seq.length = safe_ctxt.node_seq.length.wrapping_add(1)
    };
}
/* ***********************************************************************
 *									*
 *		Defaults settings					*
 *									*
 ************************************************************************/
/* *
 * xmlPedanticParserDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for enabling pedantic warnings.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */

pub unsafe fn xmlPedanticParserDefault(mut val: i32) -> i32 {
    let mut safe__xmlPedanticParserDefaultValue = unsafe { *__xmlPedanticParserDefaultValue() };
    let mut old: i32 = safe__xmlPedanticParserDefaultValue;
    safe__xmlPedanticParserDefaultValue = val;
    return old;
}
/* *
 * xmlLineNumbersDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for enabling line numbers in elements
 * contents. This may break on old application and is turned off by default.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */

pub unsafe fn xmlLineNumbersDefault(mut val: i32) -> i32 {
    let mut safe__xmlLineNumbersDefaultValue = unsafe { *__xmlLineNumbersDefaultValue() };
    let mut old: i32 = safe__xmlLineNumbersDefaultValue;
    safe__xmlLineNumbersDefaultValue = val;
    return old;
}
/* *
 * xmlSubstituteEntitiesDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for default entity support.
 * Initially the parser always keep entity references instead of substituting
 * entity values in the output. This function has to be used to change the
 * default parser behavior
 * SAX::substituteEntities() has to be used for changing that on a file by
 * file basis.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */

pub unsafe fn xmlSubstituteEntitiesDefault(mut val: i32) -> i32 {
    let mut safe__xmlSubstituteEntitiesDefaultValue =
        unsafe { *__xmlSubstituteEntitiesDefaultValue() };
    let mut old: i32 = safe__xmlSubstituteEntitiesDefaultValue;
    safe__xmlSubstituteEntitiesDefaultValue = val;
    return old;
}

pub unsafe fn xmlKeepBlanksDefault(mut val: i32) -> i32 {
    let mut safe__xmlKeepBlanksDefaultValue = unsafe { *__xmlKeepBlanksDefaultValue() };
    let mut old: i32 = safe__xmlKeepBlanksDefaultValue;
    safe__xmlKeepBlanksDefaultValue = val;
    if val == 0 {
        safe__xmlKeepBlanksDefaultValue = 1 as i32
    }
    return old;
}
