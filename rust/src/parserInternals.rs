pub unsafe fn xmlCheckVersion(mut version: libc::c_int) {
    let mut myversion: libc::c_int = 20912 as libc::c_int;
    xmlInitParser();
    if myversion / 10000 as libc::c_int != version / 10000 as libc::c_int {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Fatal: program compiled against libxml %d using libxml %d\n\x00" as *const u8
                as *const libc::c_char,
            version / 10000 as libc::c_int,
            myversion / 10000 as libc::c_int,
        );
        fprintf(
            stderr,
            b"Fatal: program compiled against libxml %d using libxml %d\n\x00" as *const u8
                as *const libc::c_char,
            version / 10000 as libc::c_int,
            myversion / 10000 as libc::c_int,
        );
    }
    if (myversion / 100 as libc::c_int) < version / 100 as libc::c_int {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Warning: program compiled against libxml %d using older %d\n\x00" as *const u8
                as *const libc::c_char,
            version / 100 as libc::c_int,
            myversion / 100 as libc::c_int,
        );
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

pub unsafe fn xmlErrMemory(mut ctxt: xmlParserCtxtPtr, mut extra: *const libc::c_char) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as libc::c_int
        && (*ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = XML_ERR_NO_MEMORY as libc::c_int;
        (*ctxt).instate = XML_PARSER_EOF;
        (*ctxt).disableSAX = 1 as libc::c_int
    }
    if !extra.is_null() {
        __xmlRaiseError(
            None,
            None,
            0 as *mut libc::c_void,
            ctxt as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_PARSER as libc::c_int,
            XML_ERR_NO_MEMORY as libc::c_int,
            XML_ERR_FATAL,
            0 as *const libc::c_char,
            0 as libc::c_int,
            extra,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            b"Memory allocation failed : %s\n\x00" as *const u8 as *const libc::c_char,
            extra,
        );
    } else {
        __xmlRaiseError(
            None,
            None,
            0 as *mut libc::c_void,
            ctxt as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_PARSER as libc::c_int,
            XML_ERR_NO_MEMORY as libc::c_int,
            XML_ERR_FATAL,
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            b"Memory allocation failed\n\x00" as *const u8 as *const libc::c_char,
        );
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
    mut msg: *const libc::c_char,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as libc::c_int
        && (*ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = xmlerr as libc::c_int
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        ctxt as *mut libc::c_void,
        0 as *mut libc::c_void,
        XML_FROM_PARSER as libc::c_int,
        xmlerr as libc::c_int,
        XML_ERR_FATAL,
        0 as *const libc::c_char,
        0 as libc::c_int,
        str1 as *const libc::c_char,
        str2 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        msg,
        str1,
        str2,
    );
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as libc::c_int;
        if (*ctxt).recovery == 0 as libc::c_int {
            (*ctxt).disableSAX = 1 as libc::c_int
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
unsafe fn xmlErrInternal(
    mut ctxt: xmlParserCtxtPtr,
    mut msg: *const libc::c_char,
    mut str: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as libc::c_int
        && (*ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = XML_ERR_INTERNAL_ERROR as libc::c_int
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        ctxt as *mut libc::c_void,
        0 as *mut libc::c_void,
        XML_FROM_PARSER as libc::c_int,
        XML_ERR_INTERNAL_ERROR as libc::c_int,
        XML_ERR_FATAL,
        0 as *const libc::c_char,
        0 as libc::c_int,
        str as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        msg,
        str,
    );
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as libc::c_int;
        if (*ctxt).recovery == 0 as libc::c_int {
            (*ctxt).disableSAX = 1 as libc::c_int
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
    mut msg: *const libc::c_char,
    mut val: libc::c_int,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as libc::c_int
        && (*ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as libc::c_int
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        ctxt as *mut libc::c_void,
        0 as *mut libc::c_void,
        XML_FROM_PARSER as libc::c_int,
        error as libc::c_int,
        XML_ERR_FATAL,
        0 as *const libc::c_char,
        0 as libc::c_int,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        val,
        0 as libc::c_int,
        msg,
        val,
    );
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as libc::c_int;
        if (*ctxt).recovery == 0 as libc::c_int {
            (*ctxt).disableSAX = 1 as libc::c_int
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

pub unsafe fn xmlIsLetter(mut c: libc::c_int) -> libc::c_int {
    return ((if c < 0x100 as libc::c_int {
        (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
            || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
            || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
            || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
            || 0xf8 as libc::c_int <= c) as libc::c_int
    } else {
        xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as libc::c_int {
            0 as libc::c_int
        } else {
            (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                || c == 0x3007 as libc::c_int
                || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                as libc::c_int
        }) != 0) as libc::c_int;
}

pub unsafe fn check_buffer(mut in_0: xmlParserInputPtr) {
    match () {
        #[cfg(HAVE_parserInternals_DEBUG_INPUT)]
        _ => {
            if (*in_0).base != xmlBufContent((*(*in_0).buf).buffer as *const xmlBuf) {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlParserInput: base mismatch problem\n\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            if (*in_0).cur < (*in_0).base {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlParserInput: cur < base problem\n\x00" as *const u8 as *const libc::c_char,
                );
            }
            if (*in_0).cur
                > (*in_0)
                    .base
                    .offset(xmlBufUse((*(*in_0).buf).buffer) as isize)
            {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlParserInput: cur > base + use problem\n\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"buffer %x : content %x, cur %d, use %d\n\x00" as *const u8 as *const libc::c_char,
                in_0 as libc::c_int,
                xmlBufContent((*(*in_0).buf).buffer as *const xmlBuf) as libc::c_int,
                (*in_0).cur.offset_from((*in_0).base) as libc::c_long,
                xmlBufUse((*(*in_0).buf).buffer),
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

pub unsafe fn xmlParserInputRead(mut in_0: xmlParserInputPtr, mut len: libc::c_int) -> libc::c_int {
    return -(1 as libc::c_int);
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

pub unsafe fn xmlParserInputGrow_parserInternals(
    mut in_0: xmlParserInputPtr,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut indx: size_t = 0;
    let mut content: *const xmlChar = 0 as *const xmlChar;
    if in_0.is_null() || len < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }

    match () {
        #[cfg(HAVE_parserInternals_DEBUG_INPUT)]
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Grow\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        #[cfg(not(HAVE_parserInternals_DEBUG_INPUT))]
        _ => {}
    };

    if (*in_0).buf.is_null() {
        return -(1 as libc::c_int);
    }
    if (*in_0).base.is_null() {
        return -(1 as libc::c_int);
    }
    if (*in_0).cur.is_null() {
        return -(1 as libc::c_int);
    }
    if (*(*in_0).buf).buffer.is_null() {
        return -(1 as libc::c_int);
    }

    check_buffer(in_0);

    indx = (*in_0).cur.offset_from((*in_0).base) as libc::c_long as size_t;
    if xmlBufUse((*(*in_0).buf).buffer)
        > (indx as libc::c_uint).wrapping_add(250 as libc::c_int as libc::c_uint) as libc::c_ulong
    {
        check_buffer(in_0);
        return 0 as libc::c_int;
    }
    if (*(*in_0).buf).readcallback.is_some() {
        ret = xmlParserInputBufferGrow((*in_0).buf, len)
    } else {
        return 0 as libc::c_int;
    }
    /*
     * NOTE : in->base may be a "dangling" i.e. freed pointer in this
     *        block, but we use it really as an integer to do some
     *        pointer arithmetic. Insure will raise it as a bug but in
     *        that specific case, that's not !
     */
    content = xmlBufContent((*(*in_0).buf).buffer as *const xmlBuf);
    if (*in_0).base != content {
        /*
         * the buffer has been reallocated
         */
        indx = (*in_0).cur.offset_from((*in_0).base) as libc::c_long as size_t;
        (*in_0).base = content;
        (*in_0).cur = &*content.offset(indx as isize) as *const xmlChar
    }
    (*in_0).end = xmlBufEnd((*(*in_0).buf).buffer);

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
    let mut used: size_t = 0;
    let mut ret: size_t = 0;
    let mut indx: size_t = 0;
    let mut content: *const xmlChar = 0 as *const xmlChar;

    match () {
        #[cfg(HAVE_parserInternals_DEBUG_INPUT)]
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Shrink\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        #[cfg(not(HAVE_parserInternals_DEBUG_INPUT))]
        _ => {}
    };

    if in_0.is_null() {
        return;
    }
    if (*in_0).buf.is_null() {
        return;
    }
    if (*in_0).base.is_null() {
        return;
    }
    if (*in_0).cur.is_null() {
        return;
    }
    if (*(*in_0).buf).buffer.is_null() {
        return;
    }

    check_buffer(in_0);

    used = (*in_0)
        .cur
        .offset_from(xmlBufContent((*(*in_0).buf).buffer as *const xmlBuf))
        as libc::c_long as size_t;
    /*
     * Do not shrink on large buffers whose only a tiny fraction
     * was consumed
     */
    if used > 250 as libc::c_int as libc::c_ulong {
        ret = xmlBufShrink(
            (*(*in_0).buf).buffer,
            used.wrapping_sub(80 as libc::c_int as libc::c_ulong),
        );
        if ret > 0 as libc::c_int as libc::c_ulong {
            (*in_0).cur = (*in_0).cur.offset(-(ret as isize));
            (*in_0).consumed = (*in_0).consumed.wrapping_add(ret)
        }
        (*in_0).end = xmlBufEnd((*(*in_0).buf).buffer)
    }

    check_buffer(in_0);

    if xmlBufUse((*(*in_0).buf).buffer) > 250 as libc::c_int as libc::c_ulong {
        return;
    }
    xmlParserInputBufferRead((*in_0).buf, 2 as libc::c_int * 250 as libc::c_int);
    content = xmlBufContent((*(*in_0).buf).buffer as *const xmlBuf);
    if (*in_0).base != content {
        /*
         * the buffer has been reallocated
         */
        indx = (*in_0).cur.offset_from((*in_0).base) as libc::c_long as size_t;
        (*in_0).base = content;
        (*in_0).cur = &*content.offset(indx as isize) as *const xmlChar
    }
    (*in_0).end = xmlBufEnd((*(*in_0).buf).buffer);

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
    let mut current_block: u64;
    if ctxt.is_null()
        || (*ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int
        || (*ctxt).input.is_null()
    {
        return;
    }
    if !((*(*ctxt).input).cur <= (*(*ctxt).input).end) {
        xmlErrInternal(
            ctxt,
            b"Parser input data memory error\n\x00" as *const u8 as *const libc::c_char,
            0 as *const xmlChar,
        );
        (*ctxt).errNo = XML_ERR_INTERNAL_ERROR as libc::c_int;
        xmlStopParser(ctxt);
        return;
    }
    if *(*(*ctxt).input).cur as libc::c_int == 0 as libc::c_int
        && xmlParserInputGrow((*ctxt).input, 250 as libc::c_int) <= 0 as libc::c_int
    {
        return;
    }
    if (*ctxt).charset == XML_CHAR_ENCODING_UTF8 as libc::c_int {
        let mut cur: *const libc::c_uchar = 0 as *const libc::c_uchar;
        let mut c: libc::c_uchar = 0;
        /*
         *   2.11 End-of-Line Handling
         *   the literal two-character sequence "#xD#xA" or a standalone
         *   literal #xD, an XML processor must pass to the application
         *   the single character #xA.
         */
        if *(*(*ctxt).input).cur as libc::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as libc::c_int
        } else {
            (*(*ctxt).input).col += 1
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
        cur = (*(*ctxt).input).cur;
        c = *cur;
        if c as libc::c_int & 0x80 as libc::c_int != 0 {
            if c as libc::c_int == 0xc0 as libc::c_int {
                current_block = 14602029473953406903;
            } else {
                if *cur.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
                    xmlParserInputGrow((*ctxt).input, 250 as libc::c_int);
                    cur = (*(*ctxt).input).cur
                }
                if *cur.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                    != 0x80 as libc::c_int
                {
                    current_block = 14602029473953406903;
                } else if c as libc::c_int & 0xe0 as libc::c_int == 0xe0 as libc::c_int {
                    let mut val: libc::c_uint = 0;
                    if *cur.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
                        xmlParserInputGrow((*ctxt).input, 250 as libc::c_int);
                        cur = (*(*ctxt).input).cur
                    }
                    if *cur.offset(2 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                        != 0x80 as libc::c_int
                    {
                        current_block = 14602029473953406903;
                    } else {
                        if c as libc::c_int & 0xf0 as libc::c_int == 0xf0 as libc::c_int {
                            if *cur.offset(3 as libc::c_int as isize) as libc::c_int
                                == 0 as libc::c_int
                            {
                                xmlParserInputGrow((*ctxt).input, 250 as libc::c_int);
                                cur = (*(*ctxt).input).cur
                            }
                            if c as libc::c_int & 0xf8 as libc::c_int != 0xf0 as libc::c_int
                                || *cur.offset(3 as libc::c_int as isize) as libc::c_int
                                    & 0xc0 as libc::c_int
                                    != 0x80 as libc::c_int
                            {
                                current_block = 14602029473953406903;
                            } else {
                                /* 4-byte code */
                                (*(*ctxt).input).cur =
                                    (*(*ctxt).input).cur.offset(4 as libc::c_int as isize);
                                val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                                    & 0x7 as libc::c_int)
                                    << 18 as libc::c_int)
                                    as libc::c_uint;
                                val |= ((*cur.offset(1 as libc::c_int as isize) as libc::c_int
                                    & 0x3f as libc::c_int)
                                    << 12 as libc::c_int)
                                    as libc::c_uint;
                                val |= ((*cur.offset(2 as libc::c_int as isize) as libc::c_int
                                    & 0x3f as libc::c_int)
                                    << 6 as libc::c_int)
                                    as libc::c_uint;
                                val |= (*cur.offset(3 as libc::c_int as isize) as libc::c_int
                                    & 0x3f as libc::c_int)
                                    as libc::c_uint;
                                current_block = 15004371738079956865;
                            }
                        } else {
                            /* 3-byte code */
                            (*(*ctxt).input).cur =
                                (*(*ctxt).input).cur.offset(3 as libc::c_int as isize);
                            val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                                & 0xf as libc::c_int)
                                << 12 as libc::c_int)
                                as libc::c_uint;
                            val |= ((*cur.offset(1 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int)
                                << 6 as libc::c_int)
                                as libc::c_uint;
                            val |= (*cur.offset(2 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int)
                                as libc::c_uint;
                            current_block = 15004371738079956865;
                        }
                        match current_block {
                            14602029473953406903 => {}
                            _ => {
                                if val > 0xd7ff as libc::c_int as libc::c_uint
                                    && val < 0xe000 as libc::c_int as libc::c_uint
                                    || val > 0xfffd as libc::c_int as libc::c_uint
                                        && val < 0x10000 as libc::c_int as libc::c_uint
                                    || val >= 0x110000 as libc::c_int as libc::c_uint
                                {
                                    xmlErrEncodingInt(
                                        ctxt,
                                        XML_ERR_INVALID_CHAR,
                                        b"Char 0x%X out of allowed range\n\x00" as *const u8
                                            as *const libc::c_char,
                                        val as libc::c_int,
                                    );
                                }
                                current_block = 6072622540298447352;
                            }
                        }
                    }
                } else {
                    /* 2-byte code */
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as libc::c_int as isize);
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
                        || (*ctxt).input.is_null()
                        || ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as libc::c_long)
                            < 4 as libc::c_int as libc::c_long
                    {
                        __xmlErrEncoding(
                            ctxt,
                            XML_ERR_INVALID_CHAR,
                            b"Input is not proper UTF-8, indicate encoding !\n\x00" as *const u8
                                as *const libc::c_char,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                        );
                    } else {
                        let mut buffer: [libc::c_char; 150] = [0; 150];
                        snprintf(
                            buffer.as_mut_ptr(),
                            149 as libc::c_int as libc::c_ulong,
                            b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8
                                as *const libc::c_char,
                            *(*(*ctxt).input).cur.offset(0 as libc::c_int as isize) as libc::c_int,
                            *(*(*ctxt).input).cur.offset(1 as libc::c_int as isize) as libc::c_int,
                            *(*(*ctxt).input).cur.offset(2 as libc::c_int as isize) as libc::c_int,
                            *(*(*ctxt).input).cur.offset(3 as libc::c_int as isize) as libc::c_int,
                        );
                        __xmlErrEncoding(
                            ctxt,
                            XML_ERR_INVALID_CHAR,
                            b"Input is not proper UTF-8, indicate encoding !\n%s\x00" as *const u8
                                as *const libc::c_char,
                            buffer.as_mut_ptr() as *mut xmlChar,
                            0 as *const xmlChar,
                        );
                    }
                    (*ctxt).charset = XML_CHAR_ENCODING_8859_1 as libc::c_int;
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
                    return;
                }
            }
        } else {
            /* 1-byte code */
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
        }
    } else {
        /*
         * Assume it's a fixed length encoding (1) with
         * a compatible encoding for the ASCII set, since
         * XML constructs only use < 128 chars
         */
        if *(*(*ctxt).input).cur as libc::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as libc::c_int
        } else {
            (*(*ctxt).input).col += 1
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
    }
    if *(*(*ctxt).input).cur as libc::c_int == 0 as libc::c_int {
        xmlParserInputGrow((*ctxt).input, 250 as libc::c_int);
    };
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

pub unsafe fn xmlCurrentChar(mut ctxt: xmlParserCtxtPtr, mut len: *mut libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    if ctxt.is_null() || len.is_null() || (*ctxt).input.is_null() {
        return 0 as libc::c_int;
    }
    if (*ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
        return 0 as libc::c_int;
    }
    if *(*(*ctxt).input).cur as libc::c_int >= 0x20 as libc::c_int
        && *(*(*ctxt).input).cur as libc::c_int <= 0x7f as libc::c_int
    {
        *len = 1 as libc::c_int;
        return *(*(*ctxt).input).cur as libc::c_int;
    }
    if (*ctxt).charset == XML_CHAR_ENCODING_UTF8 as libc::c_int {
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
        let mut cur: *const libc::c_uchar = (*(*ctxt).input).cur;
        let mut c: libc::c_uchar = 0;
        let mut val: libc::c_uint = 0;
        c = *cur;
        if c as libc::c_int & 0x80 as libc::c_int != 0 {
            if !(c as libc::c_int & 0x40 as libc::c_int == 0 as libc::c_int
                || c as libc::c_int == 0xc0 as libc::c_int)
            {
                if *cur.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
                    xmlParserInputGrow((*ctxt).input, 250 as libc::c_int);
                    cur = (*(*ctxt).input).cur
                }
                if !(*cur.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                    != 0x80 as libc::c_int)
                {
                    if c as libc::c_int & 0xe0 as libc::c_int == 0xe0 as libc::c_int {
                        if *cur.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
                        {
                            xmlParserInputGrow((*ctxt).input, 250 as libc::c_int);
                            cur = (*(*ctxt).input).cur
                        }
                        if *cur.offset(2 as libc::c_int as isize) as libc::c_int
                            & 0xc0 as libc::c_int
                            != 0x80 as libc::c_int
                        {
                            current_block = 14852328631030688201;
                        } else if c as libc::c_int & 0xf0 as libc::c_int == 0xf0 as libc::c_int {
                            if *cur.offset(3 as libc::c_int as isize) as libc::c_int
                                == 0 as libc::c_int
                            {
                                xmlParserInputGrow((*ctxt).input, 250 as libc::c_int);
                                cur = (*(*ctxt).input).cur
                            }
                            if c as libc::c_int & 0xf8 as libc::c_int != 0xf0 as libc::c_int
                                || *cur.offset(3 as libc::c_int as isize) as libc::c_int
                                    & 0xc0 as libc::c_int
                                    != 0x80 as libc::c_int
                            {
                                current_block = 14852328631030688201;
                            } else {
                                /* 4-byte code */
                                *len = 4 as libc::c_int;
                                val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                                    & 0x7 as libc::c_int)
                                    << 18 as libc::c_int)
                                    as libc::c_uint;
                                val |= ((*cur.offset(1 as libc::c_int as isize) as libc::c_int
                                    & 0x3f as libc::c_int)
                                    << 12 as libc::c_int)
                                    as libc::c_uint;
                                val |= ((*cur.offset(2 as libc::c_int as isize) as libc::c_int
                                    & 0x3f as libc::c_int)
                                    << 6 as libc::c_int)
                                    as libc::c_uint;
                                val |= (*cur.offset(3 as libc::c_int as isize) as libc::c_int
                                    & 0x3f as libc::c_int)
                                    as libc::c_uint;
                                if val < 0x10000 as libc::c_int as libc::c_uint {
                                    current_block = 14852328631030688201;
                                } else {
                                    current_block = 3938820862080741272;
                                }
                            }
                        } else {
                            /* 3-byte code */
                            *len = 3 as libc::c_int;
                            val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                                & 0xf as libc::c_int)
                                << 12 as libc::c_int)
                                as libc::c_uint;
                            val |= ((*cur.offset(1 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int)
                                << 6 as libc::c_int)
                                as libc::c_uint;
                            val |= (*cur.offset(2 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int)
                                as libc::c_uint;
                            if val < 0x800 as libc::c_int as libc::c_uint {
                                current_block = 14852328631030688201;
                            } else {
                                current_block = 3938820862080741272;
                            }
                        }
                    } else {
                        /* 2-byte code */
                        *len = 2 as libc::c_int;
                        val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                            & 0x1f as libc::c_int)
                            << 6 as libc::c_int) as libc::c_uint;
                        val |= (*cur.offset(1 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int) as libc::c_uint;
                        if val < 0x80 as libc::c_int as libc::c_uint {
                            current_block = 14852328631030688201;
                        } else {
                            current_block = 3938820862080741272;
                        }
                    }
                    match current_block {
                        14852328631030688201 => {}
                        _ => {
                            if if val < 0x100 as libc::c_int as libc::c_uint {
                                (0x9 as libc::c_int as libc::c_uint <= val
                                    && val <= 0xa as libc::c_int as libc::c_uint
                                    || val == 0xd as libc::c_int as libc::c_uint
                                    || 0x20 as libc::c_int as libc::c_uint <= val)
                                    as libc::c_int
                            } else {
                                (0x100 as libc::c_int as libc::c_uint <= val
                                    && val <= 0xd7ff as libc::c_int as libc::c_uint
                                    || 0xe000 as libc::c_int as libc::c_uint <= val
                                        && val <= 0xfffd as libc::c_int as libc::c_uint
                                    || 0x10000 as libc::c_int as libc::c_uint <= val
                                        && val <= 0x10ffff as libc::c_int as libc::c_uint)
                                    as libc::c_int
                            } == 0
                            {
                                xmlErrEncodingInt(
                                    ctxt,
                                    XML_ERR_INVALID_CHAR,
                                    b"Char 0x%X out of allowed range\n\x00" as *const u8
                                        as *const libc::c_char,
                                    val as libc::c_int,
                                );
                            }
                            return val as libc::c_int;
                        }
                    }
                }
            }
            /*
             * An encoding problem may arise from a truncated input buffer
             * splitting a character in the middle. In that case do not raise
             * an error but return 0 to indicate an end of stream problem
             */
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as libc::c_long)
                < 4 as libc::c_int as libc::c_long
            {
                *len = 0 as libc::c_int;
                return 0 as libc::c_int;
            }
            /*
             * If we detect an UTF8 error that probably mean that the
             * input encoding didn't get properly advertised in the
             * declaration header. Report the error and switch the encoding
             * to ISO-Latin-1 (if you don't like this policy, just declare the
             * encoding !)
             */
            let mut buffer: [libc::c_char; 150] = [0; 150];
            snprintf(
                &mut *buffer.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_char,
                149 as libc::c_int as libc::c_ulong,
                b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8 as *const libc::c_char,
                *(*(*ctxt).input).cur.offset(0 as libc::c_int as isize) as libc::c_int,
                *(*(*ctxt).input).cur.offset(1 as libc::c_int as isize) as libc::c_int,
                *(*(*ctxt).input).cur.offset(2 as libc::c_int as isize) as libc::c_int,
                *(*(*ctxt).input).cur.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            __xmlErrEncoding(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"Input is not proper UTF-8, indicate encoding !\n%s\x00" as *const u8
                    as *const libc::c_char,
                buffer.as_mut_ptr() as *mut xmlChar,
                0 as *const xmlChar,
            );
            (*ctxt).charset = XML_CHAR_ENCODING_8859_1 as libc::c_int;
            *len = 1 as libc::c_int;
            return *(*(*ctxt).input).cur as libc::c_int;
        } else {
            /* 1-byte code */
            *len = 1 as libc::c_int;
            if *(*(*ctxt).input).cur as libc::c_int == 0 as libc::c_int {
                xmlParserInputGrow((*ctxt).input, 250 as libc::c_int);
            }
            if *(*(*ctxt).input).cur as libc::c_int == 0 as libc::c_int
                && (*(*ctxt).input).end > (*(*ctxt).input).cur
            {
                xmlErrEncodingInt(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"Char 0x0 out of allowed range\n\x00" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
            }
            if *(*(*ctxt).input).cur as libc::c_int == 0xd as libc::c_int {
                if *(*(*ctxt).input).cur.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0xa as libc::c_int
                {
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
                }
                return 0xa as libc::c_int;
            }
            return *(*(*ctxt).input).cur as libc::c_int;
        }
    } else {
        /*
         * Assume it's a fixed length encoding (1) with
         * a compatible encoding for the ASCII set, since
         * XML constructs only use < 128 chars
         */
        *len = 1 as libc::c_int;
        if *(*(*ctxt).input).cur as libc::c_int == 0xd as libc::c_int {
            if *(*(*ctxt).input).cur.offset(1 as libc::c_int as isize) as libc::c_int
                == 0xa as libc::c_int
            {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
            }
            return 0xa as libc::c_int;
        }
        return *(*(*ctxt).input).cur as libc::c_int;
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
    mut len: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    if len.is_null() || cur.is_null() {
        return 0 as libc::c_int;
    }
    if ctxt.is_null() || (*ctxt).charset == XML_CHAR_ENCODING_UTF8 as libc::c_int {
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
        let mut c: libc::c_uchar = 0;
        let mut val: libc::c_uint = 0;
        c = *cur;
        if c as libc::c_int & 0x80 as libc::c_int != 0 {
            if !(*cur.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                != 0x80 as libc::c_int)
            {
                if c as libc::c_int & 0xe0 as libc::c_int == 0xe0 as libc::c_int {
                    if *cur.offset(2 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                        != 0x80 as libc::c_int
                    {
                        current_block = 909593556805851584;
                    } else if c as libc::c_int & 0xf0 as libc::c_int == 0xf0 as libc::c_int {
                        if c as libc::c_int & 0xf8 as libc::c_int != 0xf0 as libc::c_int
                            || *cur.offset(3 as libc::c_int as isize) as libc::c_int
                                & 0xc0 as libc::c_int
                                != 0x80 as libc::c_int
                        {
                            current_block = 909593556805851584;
                        } else {
                            /* 4-byte code */
                            *len = 4 as libc::c_int;
                            val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                                & 0x7 as libc::c_int)
                                << 18 as libc::c_int)
                                as libc::c_uint;
                            val |= ((*cur.offset(1 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int)
                                << 12 as libc::c_int)
                                as libc::c_uint;
                            val |= ((*cur.offset(2 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int)
                                << 6 as libc::c_int)
                                as libc::c_uint;
                            val |= (*cur.offset(3 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int)
                                as libc::c_uint;
                            current_block = 11298138898191919651;
                        }
                    } else {
                        /* 3-byte code */
                        *len = 3 as libc::c_int;
                        val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                            & 0xf as libc::c_int)
                            << 12 as libc::c_int) as libc::c_uint;
                        val |= ((*cur.offset(1 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int)
                            << 6 as libc::c_int) as libc::c_uint;
                        val |= (*cur.offset(2 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int) as libc::c_uint;
                        current_block = 11298138898191919651;
                    }
                } else {
                    /* 2-byte code */
                    *len = 2 as libc::c_int;
                    val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                        & 0x1f as libc::c_int)
                        << 6 as libc::c_int) as libc::c_uint;
                    val |= (*cur.offset(1 as libc::c_int as isize) as libc::c_int
                        & 0x3f as libc::c_int) as libc::c_uint;
                    current_block = 11298138898191919651;
                }
                match current_block {
                    909593556805851584 => {}
                    _ => {
                        if if val < 0x100 as libc::c_int as libc::c_uint {
                            (0x9 as libc::c_int as libc::c_uint <= val
                                && val <= 0xa as libc::c_int as libc::c_uint
                                || val == 0xd as libc::c_int as libc::c_uint
                                || 0x20 as libc::c_int as libc::c_uint <= val)
                                as libc::c_int
                        } else {
                            (0x100 as libc::c_int as libc::c_uint <= val
                                && val <= 0xd7ff as libc::c_int as libc::c_uint
                                || 0xe000 as libc::c_int as libc::c_uint <= val
                                    && val <= 0xfffd as libc::c_int as libc::c_uint
                                || 0x10000 as libc::c_int as libc::c_uint <= val
                                    && val <= 0x10ffff as libc::c_int as libc::c_uint)
                                as libc::c_int
                        } == 0
                        {
                            xmlErrEncodingInt(
                                ctxt,
                                XML_ERR_INVALID_CHAR,
                                b"Char 0x%X out of allowed range\n\x00" as *const u8
                                    as *const libc::c_char,
                                val as libc::c_int,
                            );
                        }
                        return val as libc::c_int;
                    }
                }
            }
            /*
             * An encoding problem may arise from a truncated input buffer
             * splitting a character in the middle. In that case do not raise
             * an error but return 0 to indicate an end of stream problem
             */
            if ctxt.is_null()
                || (*ctxt).input.is_null()
                || ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as libc::c_long)
                    < 4 as libc::c_int as libc::c_long
            {
                *len = 0 as libc::c_int;
                return 0 as libc::c_int;
            }
            /*
             * If we detect an UTF8 error that probably mean that the
             * input encoding didn't get properly advertised in the
             * declaration header. Report the error and switch the encoding
             * to ISO-Latin-1 (if you don't like this policy, just declare the
             * encoding !)
             */
            let mut buffer: [libc::c_char; 150] = [0; 150];
            snprintf(
                buffer.as_mut_ptr(),
                149 as libc::c_int as libc::c_ulong,
                b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8 as *const libc::c_char,
                *(*(*ctxt).input).cur.offset(0 as libc::c_int as isize) as libc::c_int,
                *(*(*ctxt).input).cur.offset(1 as libc::c_int as isize) as libc::c_int,
                *(*(*ctxt).input).cur.offset(2 as libc::c_int as isize) as libc::c_int,
                *(*(*ctxt).input).cur.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            __xmlErrEncoding(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"Input is not proper UTF-8, indicate encoding !\n%s\x00" as *const u8
                    as *const libc::c_char,
                buffer.as_mut_ptr() as *mut xmlChar,
                0 as *const xmlChar,
            );
            *len = 1 as libc::c_int;
            return *cur as libc::c_int;
        } else {
            /* 1-byte code */
            *len = 1 as libc::c_int;
            return *cur as libc::c_int;
        }
    } else {
        /*
         * Assume it's a fixed length encoding (1) with
         * a compatible encoding for the ASCII set, since
         * XML constructs only use < 128 chars
         */
        *len = 1 as libc::c_int;
        return *cur as libc::c_int;
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

pub unsafe fn xmlCopyCharMultiByte(mut out: *mut xmlChar, mut val: libc::c_int) -> libc::c_int {
    if out.is_null() {
        return 0 as libc::c_int;
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
    if val >= 0x80 as libc::c_int {
        let mut savedout: *mut xmlChar = out;
        let mut bits: libc::c_int = 0;
        if val < 0x800 as libc::c_int {
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 = (val >> 6 as libc::c_int | 0xc0 as libc::c_int) as xmlChar;
            bits = 0 as libc::c_int
        } else if val < 0x10000 as libc::c_int {
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 = (val >> 12 as libc::c_int | 0xe0 as libc::c_int) as xmlChar;
            bits = 6 as libc::c_int
        } else if val < 0x110000 as libc::c_int {
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 = (val >> 18 as libc::c_int | 0xf0 as libc::c_int) as xmlChar;
            bits = 12 as libc::c_int
        } else {
            xmlErrEncodingInt(
                0 as xmlParserCtxtPtr,
                XML_ERR_INVALID_CHAR,
                b"Internal error, xmlCopyCharMultiByte 0x%X out of bound\n\x00" as *const u8
                    as *const libc::c_char,
                val,
            );
            return 0 as libc::c_int;
        }
        while bits >= 0 as libc::c_int {
            let fresh3 = out;
            out = out.offset(1);
            *fresh3 = (val >> bits & 0x3f as libc::c_int | 0x80 as libc::c_int) as xmlChar;
            bits -= 6 as libc::c_int
        }
        return out.offset_from(savedout) as libc::c_long as libc::c_int;
    }
    *out = val as xmlChar;
    return 1 as libc::c_int;
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
    mut len: libc::c_int,
    mut out: *mut xmlChar,
    mut val: libc::c_int,
) -> libc::c_int {
    if out.is_null() {
        return 0 as libc::c_int;
    }
    /* the len parameter is ignored */
    if val >= 0x80 as libc::c_int {
        return xmlCopyCharMultiByte(out, val);
    }
    *out = val as xmlChar;
    return 1 as libc::c_int;
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
) -> libc::c_int {
    let mut handler: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
    let mut len: libc::c_int = -(1 as libc::c_int);
    let mut ret: libc::c_int = 0;
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    match enc as libc::c_int {
        -1 => {
            __xmlErrEncoding(
                ctxt,
                XML_ERR_UNKNOWN_ENCODING,
                b"encoding unknown\n\x00" as *const u8 as *const libc::c_char,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            return -(1 as libc::c_int);
        }
        0 => {
            /* let's assume it's UTF-8 without the XML decl */
            (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as libc::c_int;
            return 0 as libc::c_int;
        }
        1 => {
            /* default encoding, no conversion should be needed */
            (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as libc::c_int;
            /*
             * Errata on XML-1.0 June 20 2001
             * Specific handling of the Byte Order Mark for
             * UTF-8
             */
            if !(*ctxt).input.is_null()
                && *(*(*ctxt).input).cur.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0xef as libc::c_int
                && *(*(*ctxt).input).cur.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0xbb as libc::c_int
                && *(*(*ctxt).input).cur.offset(2 as libc::c_int as isize) as libc::c_int
                    == 0xbf as libc::c_int
            {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(3 as libc::c_int as isize)
            }
            return 0 as libc::c_int;
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
            if !(*ctxt).input.is_null()
                && !(*(*ctxt).input).cur.is_null()
                && *(*(*ctxt).input).cur.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0xef as libc::c_int
                && *(*(*ctxt).input).cur.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0xbb as libc::c_int
                && *(*(*ctxt).input).cur.offset(2 as libc::c_int as isize) as libc::c_int
                    == 0xbf as libc::c_int
            {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(3 as libc::c_int as isize)
            }
            len = 90 as libc::c_int
        }
        9 => len = 90 as libc::c_int,
        5 | 4 | 7 | 8 => len = 180 as libc::c_int,
        6 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 22 | 19 | 20 | 21 => {
            len = 45 as libc::c_int
        }
        _ => {}
    }
    handler = xmlGetCharEncodingHandler(enc);
    if handler.is_null() {
        /*
         * Default handlers.
         */
        match enc as libc::c_int {
            22 => {
                /* default encoding, no conversion should be needed */
                (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as libc::c_int;
                return 0 as libc::c_int;
            }
            4 => {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"encoding not supported %s\n\x00" as *const u8 as *const libc::c_char,
                    b"USC4 little endian\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                    0 as *const xmlChar,
                );
            }
            5 => {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"encoding not supported %s\n\x00" as *const u8 as *const libc::c_char,
                    b"USC4 big endian\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                    0 as *const xmlChar,
                );
            }
            6 => {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"encoding not supported %s\n\x00" as *const u8 as *const libc::c_char,
                    b"EBCDIC\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                    0 as *const xmlChar,
                );
            }
            7 => {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"encoding not supported %s\n\x00" as *const u8 as *const libc::c_char,
                    b"UCS4 2143\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                    0 as *const xmlChar,
                );
            }
            8 => {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"encoding not supported %s\n\x00" as *const u8 as *const libc::c_char,
                    b"UCS4 3412\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                    0 as *const xmlChar,
                );
            }
            9 => {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"encoding not supported %s\n\x00" as *const u8 as *const libc::c_char,
                    b"UCS2\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
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
                if (*ctxt).inputNr == 1 as libc::c_int
                    && (*ctxt).encoding.is_null()
                    && !(*ctxt).input.is_null()
                    && !(*(*ctxt).input).encoding.is_null()
                {
                    (*ctxt).encoding = xmlStrdup((*(*ctxt).input).encoding)
                }
                (*ctxt).charset = enc as libc::c_int;
                return 0 as libc::c_int;
            }
            19 => {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"encoding not supported %s\n\x00" as *const u8 as *const libc::c_char,
                    b"ISO-2022-JP\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                    0 as *const xmlChar,
                );
            }
            20 => {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"encoding not supported %s\n\x00" as *const u8 as *const libc::c_char,
                    b"Shift_JIS\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                    0 as *const xmlChar,
                );
            }
            21 => {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"encoding not supported %s\n\x00" as *const u8 as *const libc::c_char,
                    b"EUC-JP\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                    0 as *const xmlChar,
                );
            }
            2 | 3 | _ => {}
        }
    }
    /*
     * TODO: We could recover from errors in external entities if we
     * didn't stop the parser. But most callers of this function don't
     * check the return value.
     */
    if handler.is_null() {
        xmlStopParser(ctxt);
        return -(1 as libc::c_int);
    }
    (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as libc::c_int;
    ret = xmlSwitchToEncodingInt(ctxt, handler, len);
    if ret < 0 as libc::c_int || (*ctxt).errNo == XML_I18N_CONV_FAILED as libc::c_int {
        /*
         * on encoding conversion errors, stop the parser
         */
        xmlStopParser(ctxt);
        (*ctxt).errNo = XML_I18N_CONV_FAILED as libc::c_int
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
    mut len: libc::c_int,
) -> libc::c_int {
    let mut nbchars: libc::c_int = 0;
    if handler.is_null() {
        return -(1 as libc::c_int);
    }
    if input.is_null() {
        return -(1 as libc::c_int);
    }
    if !(*input).buf.is_null() {
        if !(*(*input).buf).encoder.is_null() {
            /*
             * Check in case the auto encoding detection triggered
             * in already.
             */
            if (*(*input).buf).encoder == handler {
                return 0 as libc::c_int;
            }
            /*
            * "UTF-16" can be used for both LE and BE
            if ((!xmlStrncmp(BAD_CAST input->buf->encoder->name,
            BAD_CAST "UTF-16", 6)) &&
            (!xmlStrncmp(BAD_CAST handler->name,
            BAD_CAST "UTF-16", 6))) {
            return(0);
            }
            */
            /*
             * Note: this is a bit dangerous, but that's what it
             * takes to use nearly compatible signature for different
             * encodings.
             *
             * FIXME: Encoders might buffer partial byte sequences, so
             * this probably can't work. We should return an error and
             * make sure that callers never try to switch the encoding
             * twice.
             */
            xmlCharEncCloseFunc((*(*input).buf).encoder);
            (*(*input).buf).encoder = handler;
            return 0 as libc::c_int;
        }
        (*(*input).buf).encoder = handler;
        /*
         * Is there already some content down the pipe to convert ?
         */
        if xmlBufIsEmpty((*(*input).buf).buffer) == 0 as libc::c_int {
            let mut processed: libc::c_int = 0;
            let mut use_0: libc::c_uint = 0;
            /*
             * Specific handling of the Byte Order Mark for
             * UTF-16
             */
            if !(*handler).name.is_null()
                && (strcmp(
                    (*handler).name,
                    b"UTF-16LE\x00" as *const u8 as *const libc::c_char,
                ) == 0
                    || strcmp(
                        (*handler).name,
                        b"UTF-16\x00" as *const u8 as *const libc::c_char,
                    ) == 0)
                && *(*input).cur.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0xff as libc::c_int
                && *(*input).cur.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0xfe as libc::c_int
            {
                (*input).cur = (*input).cur.offset(2 as libc::c_int as isize)
            }
            if !(*handler).name.is_null()
                && strcmp(
                    (*handler).name,
                    b"UTF-16BE\x00" as *const u8 as *const libc::c_char,
                ) == 0
                && *(*input).cur.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0xfe as libc::c_int
                && *(*input).cur.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0xff as libc::c_int
            {
                (*input).cur = (*input).cur.offset(2 as libc::c_int as isize)
            }
            /*
             * Errata on XML-1.0 June 20 2001
             * Specific handling of the Byte Order Mark for
             * UTF-8
             */
            if !(*handler).name.is_null()
                && strcmp(
                    (*handler).name,
                    b"UTF-8\x00" as *const u8 as *const libc::c_char,
                ) == 0
                && *(*input).cur.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0xef as libc::c_int
                && *(*input).cur.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0xbb as libc::c_int
                && *(*input).cur.offset(2 as libc::c_int as isize) as libc::c_int
                    == 0xbf as libc::c_int
            {
                (*input).cur = (*input).cur.offset(3 as libc::c_int as isize)
            }
            /*
             * Shrink the current input buffer.
             * Move it as the raw buffer and create a new input buffer
             */
            processed = (*input).cur.offset_from((*input).base) as libc::c_long as libc::c_int;
            xmlBufShrink((*(*input).buf).buffer, processed as size_t);
            (*(*input).buf).raw = (*(*input).buf).buffer;
            (*(*input).buf).buffer = xmlBufCreate();
            (*(*input).buf).rawconsumed = processed as libc::c_ulong;
            use_0 = xmlBufUse((*(*input).buf).raw) as libc::c_uint;
            if (*ctxt).html != 0 {
                /*
                 * convert as much as possible of the buffer
                 */
                nbchars = xmlCharEncInput((*input).buf, 1 as libc::c_int)
            } else {
                /*
                 * convert just enough to get
                 * '<?xml version="1.0" encoding="xxx"?>'
                 * parsed with the autodetected encoding
                 * into the parser reading buffer.
                 */
                nbchars = xmlCharEncFirstLineInput((*input).buf, len)
            }
            xmlBufResetInput((*(*input).buf).buffer, input);
            if nbchars < 0 as libc::c_int {
                xmlErrInternal(
                    ctxt,
                    b"switching encoding: encoder error\n\x00" as *const u8 as *const libc::c_char,
                    0 as *const xmlChar,
                );
                return -(1 as libc::c_int);
            }
            (*(*input).buf).rawconsumed = (*(*input).buf)
                .rawconsumed
                .wrapping_add((use_0 as libc::c_ulong).wrapping_sub(xmlBufUse((*(*input).buf).raw)))
        }
        return 0 as libc::c_int;
    } else {
        if (*input).length == 0 as libc::c_int {
            /*
             * When parsing a static memory array one must know the
             * size to be able to convert the buffer.
             */
            xmlErrInternal(
                ctxt,
                b"switching encoding : no input\n\x00" as *const u8 as *const libc::c_char,
                0 as *const xmlChar,
            );
            /*
             * Callers assume that the input buffer takes ownership of the
             * encoding handler. xmlCharEncCloseFunc frees unregistered
             * handlers and avoids a memory leak.
             */
            xmlCharEncCloseFunc(handler);
            return -(1 as libc::c_int);
        }
    }
    /*
     * We should actually raise an error here, see issue #34.
     */
    xmlCharEncCloseFunc(handler);
    return 0 as libc::c_int;
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
) -> libc::c_int {
    return xmlSwitchInputEncodingInt(ctxt, input, handler, -(1 as libc::c_int));
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
    mut len: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if !handler.is_null() {
        if !(*ctxt).input.is_null() {
            ret = xmlSwitchInputEncodingInt(ctxt, (*ctxt).input, handler, len)
        } else {
            xmlErrInternal(
                ctxt,
                b"xmlSwitchToEncoding : no input\n\x00" as *const u8 as *const libc::c_char,
                0 as *const xmlChar,
            );
            return -(1 as libc::c_int);
        }
        /*
         * The parsing is now done in UTF8 natively
         */
        (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as libc::c_int
    } else {
        return -(1 as libc::c_int);
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
) -> libc::c_int {
    return xmlSwitchToEncodingInt(ctxt, handler, -(1 as libc::c_int));
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
    if !(*input).filename.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*input).filename as *mut libc::c_char as *mut libc::c_void,
        );
    }
    if !(*input).directory.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*input).directory as *mut libc::c_char as *mut libc::c_void,
        );
    }
    if !(*input).encoding.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*input).encoding as *mut libc::c_char as *mut libc::c_void,
        );
    }
    if !(*input).version.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*input).version as *mut libc::c_char as *mut libc::c_void,
        );
    }
    if (*input).free.is_some() && !(*input).base.is_null() {
        (*input).free.expect("non-null function pointer")((*input).base as *mut xmlChar);
    }
    if !(*input).buf.is_null() {
        xmlFreeParserInputBuffer((*input).buf);
    }
    xmlFree.expect("non-null function pointer")(input as *mut libc::c_void);
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
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    input = xmlMalloc.expect("non-null function pointer")(
        ::std::mem::size_of::<xmlParserInput>() as libc::c_ulong
    ) as xmlParserInputPtr;
    if input.is_null() {
        xmlErrMemory(
            ctxt,
            b"couldn\'t allocate a new input stream\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlParserInputPtr;
    }
    memset(
        input as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<xmlParserInput>() as libc::c_ulong,
    );
    (*input).line = 1 as libc::c_int;
    (*input).col = 1 as libc::c_int;
    (*input).standalone = -(1 as libc::c_int);
    /*
     * If the context is NULL the id cannot be initialized, but that
     * should not happen while parsing which is the situation where
     * the id is actually needed.
     */
    if !ctxt.is_null() {
        let fresh4 = (*ctxt).input_id;
        (*ctxt).input_id = (*ctxt).input_id + 1;
        (*input).id = fresh4
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
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if input.is_null() {
        return 0 as xmlParserInputPtr;
    }
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"new input from I/O\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    inputStream = xmlNewInputStream(ctxt);
    if inputStream.is_null() {
        return 0 as xmlParserInputPtr;
    }
    (*inputStream).filename = 0 as *const libc::c_char;
    (*inputStream).buf = input;
    xmlBufResetInput((*(*inputStream).buf).buffer, inputStream);
    if enc as libc::c_int != XML_CHAR_ENCODING_NONE as libc::c_int {
        xmlSwitchEncoding(ctxt, enc);
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
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if entity.is_null() {
        xmlErrInternal(
            ctxt,
            b"xmlNewEntityInputStream entity = NULL\n\x00" as *const u8 as *const libc::c_char,
            0 as *const xmlChar,
        );
        return 0 as xmlParserInputPtr;
    }
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"new input from entity: %s\n\x00" as *const u8 as *const libc::c_char,
            (*entity).name,
        );
    }
    if (*entity).content.is_null() {
        match (*entity).etype as libc::c_uint {
            3 => {
                xmlErrInternal(
                    ctxt,
                    b"Cannot parse entity %s\n\x00" as *const u8 as *const libc::c_char,
                    (*entity).name,
                );
            }
            2 | 5 => {
                return xmlLoadExternalEntity(
                    (*entity).URI as *mut libc::c_char,
                    (*entity).ExternalID as *mut libc::c_char,
                    ctxt,
                )
            }
            1 => {
                xmlErrInternal(
                    ctxt,
                    b"Internal entity %s without content !\n\x00" as *const u8
                        as *const libc::c_char,
                    (*entity).name,
                );
            }
            4 => {
                xmlErrInternal(
                    ctxt,
                    b"Internal parameter entity %s without content !\n\x00" as *const u8
                        as *const libc::c_char,
                    (*entity).name,
                );
            }
            6 => {
                xmlErrInternal(
                    ctxt,
                    b"Predefined entity %s without content !\n\x00" as *const u8
                        as *const libc::c_char,
                    (*entity).name,
                );
            }
            _ => {}
        }
        return 0 as xmlParserInputPtr;
    }
    input = xmlNewInputStream(ctxt);
    if input.is_null() {
        return 0 as xmlParserInputPtr;
    }
    if !(*entity).URI.is_null() {
        (*input).filename = xmlStrdup((*entity).URI as *mut xmlChar) as *mut libc::c_char
    }
    (*input).base = (*entity).content;
    if (*entity).length == 0 as libc::c_int {
        (*entity).length = xmlStrlen((*entity).content)
    }
    (*input).cur = (*entity).content;
    (*input).length = (*entity).length;
    (*input).end = &mut *(*entity).content.offset((*input).length as isize) as *mut xmlChar;
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
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if buffer.is_null() {
        xmlErrInternal(
            ctxt,
            b"xmlNewStringInputStream string = NULL\n\x00" as *const u8 as *const libc::c_char,
            0 as *const xmlChar,
        );
        return 0 as xmlParserInputPtr;
    }
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"new fixed input: %.30s\n\x00" as *const u8 as *const libc::c_char,
            buffer,
        );
    }
    input = xmlNewInputStream(ctxt);
    if input.is_null() {
        xmlErrMemory(
            ctxt,
            b"couldn\'t allocate a new input stream\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlParserInputPtr;
    }
    (*input).base = buffer;
    (*input).cur = buffer;
    (*input).length = xmlStrlen(buffer);
    (*input).end = &*buffer.offset((*input).length as isize) as *const xmlChar;
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
    mut filename: *const libc::c_char,
) -> xmlParserInputPtr {
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut directory: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"new input from file: %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
    }
    if ctxt.is_null() {
        return 0 as xmlParserInputPtr;
    }
    buf = xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
    if buf.is_null() {
        if filename.is_null() {
            __xmlLoaderErr(
                ctxt as *mut libc::c_void,
                b"failed to load external entity: NULL filename \n\x00" as *const u8
                    as *const libc::c_char,
                0 as *const libc::c_char,
            );
        } else {
            __xmlLoaderErr(
                ctxt as *mut libc::c_void,
                b"failed to load external entity \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                filename,
            );
        }
        return 0 as xmlParserInputPtr;
    }
    inputStream = xmlNewInputStream(ctxt);
    if inputStream.is_null() {
        return 0 as xmlParserInputPtr;
    }
    (*inputStream).buf = buf;
    inputStream = xmlCheckHTTPInput(ctxt, inputStream);
    if inputStream.is_null() {
        return 0 as xmlParserInputPtr;
    }
    if (*inputStream).filename.is_null() {
        URI = xmlStrdup(filename as *mut xmlChar)
    } else {
        URI = xmlStrdup((*inputStream).filename as *mut xmlChar)
    }
    directory = xmlParserGetDirectory(URI as *const libc::c_char);
    if !(*inputStream).filename.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*inputStream).filename as *mut libc::c_char as *mut libc::c_void,
        );
    }
    (*inputStream).filename = xmlCanonicPath(URI as *const xmlChar) as *mut libc::c_char;
    if !URI.is_null() {
        xmlFree.expect("non-null function pointer")(URI as *mut libc::c_char as *mut libc::c_void);
    }
    (*inputStream).directory = directory;
    xmlBufResetInput((*(*inputStream).buf).buffer, inputStream);
    if (*ctxt).directory.is_null() && !directory.is_null() {
        (*ctxt).directory = xmlStrdup(directory as *const xmlChar) as *mut libc::c_char
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

pub unsafe fn xmlInitParserCtxt(mut ctxt: xmlParserCtxtPtr) -> libc::c_int {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ctxt.is_null() {
        xmlErrInternal(
            0 as xmlParserCtxtPtr,
            b"Got NULL parser context\n\x00" as *const u8 as *const libc::c_char,
            0 as *const xmlChar,
        );
        return -(1 as libc::c_int);
    }
    xmlDefaultSAXHandlerInit();
    if (*ctxt).dict.is_null() {
        (*ctxt).dict = xmlDictCreate()
    }
    if (*ctxt).dict.is_null() {
        xmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"cannot initialize parser context\n\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    xmlDictSetLimit((*ctxt).dict, 10000000 as libc::c_int as size_t);
    if (*ctxt).sax.is_null() {
        (*ctxt).sax = xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<
            xmlSAXHandler,
        >() as libc::c_ulong) as *mut xmlSAXHandler
    }
    if (*ctxt).sax.is_null() {
        xmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"cannot initialize parser context\n\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    } else {
        xmlSAXVersion((*ctxt).sax, 2 as libc::c_int);
    }
    (*ctxt).maxatts = 0 as libc::c_int;
    (*ctxt).atts = 0 as *mut *const xmlChar;
    /* Allocate the Input stack */
    if (*ctxt).inputTab.is_null() {
        (*ctxt).inputTab = xmlMalloc.expect("non-null function pointer")(
            (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlParserInputPtr>() as libc::c_ulong),
        ) as *mut xmlParserInputPtr;
        (*ctxt).inputMax = 5 as libc::c_int
    }
    if (*ctxt).inputTab.is_null() {
        xmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"cannot initialize parser context\n\x00" as *const u8 as *const libc::c_char,
        );
        (*ctxt).inputNr = 0 as libc::c_int;
        (*ctxt).inputMax = 0 as libc::c_int;
        (*ctxt).input = 0 as xmlParserInputPtr;
        return -(1 as libc::c_int);
    }
    loop {
        input = inputPop(ctxt);
        if input.is_null() {
            break;
        }
        /* Non consuming */
        xmlFreeInputStream(input);
    }
    (*ctxt).inputNr = 0 as libc::c_int;
    (*ctxt).input = 0 as xmlParserInputPtr;
    (*ctxt).version = 0 as *const xmlChar;
    (*ctxt).encoding = 0 as *const xmlChar;
    (*ctxt).standalone = -(1 as libc::c_int);
    (*ctxt).hasExternalSubset = 0 as libc::c_int;
    (*ctxt).hasPErefs = 0 as libc::c_int;
    (*ctxt).html = 0 as libc::c_int;
    (*ctxt).external = 0 as libc::c_int;
    (*ctxt).instate = XML_PARSER_START;
    (*ctxt).token = 0 as libc::c_int;
    (*ctxt).directory = 0 as *mut libc::c_char;
    /* Allocate the Node stack */
    if (*ctxt).nodeTab.is_null() {
        (*ctxt).nodeTab = xmlMalloc.expect("non-null function pointer")(
            (10 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        (*ctxt).nodeMax = 10 as libc::c_int
    }
    if (*ctxt).nodeTab.is_null() {
        xmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"cannot initialize parser context\n\x00" as *const u8 as *const libc::c_char,
        );
        (*ctxt).nodeNr = 0 as libc::c_int;
        (*ctxt).nodeMax = 0 as libc::c_int;
        (*ctxt).node = 0 as xmlNodePtr;
        (*ctxt).inputNr = 0 as libc::c_int;
        (*ctxt).inputMax = 0 as libc::c_int;
        (*ctxt).input = 0 as xmlParserInputPtr;
        return -(1 as libc::c_int);
    }
    (*ctxt).nodeNr = 0 as libc::c_int;
    (*ctxt).node = 0 as xmlNodePtr;
    /* Allocate the Name stack */
    if (*ctxt).nameTab.is_null() {
        (*ctxt).nameTab = xmlMalloc.expect("non-null function pointer")(
            (10 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
        ) as *mut *const xmlChar;
        (*ctxt).nameMax = 10 as libc::c_int
    }
    if (*ctxt).nameTab.is_null() {
        xmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"cannot initialize parser context\n\x00" as *const u8 as *const libc::c_char,
        );
        (*ctxt).nodeNr = 0 as libc::c_int;
        (*ctxt).nodeMax = 0 as libc::c_int;
        (*ctxt).node = 0 as xmlNodePtr;
        (*ctxt).inputNr = 0 as libc::c_int;
        (*ctxt).inputMax = 0 as libc::c_int;
        (*ctxt).input = 0 as xmlParserInputPtr;
        (*ctxt).nameNr = 0 as libc::c_int;
        (*ctxt).nameMax = 0 as libc::c_int;
        (*ctxt).name = 0 as *const xmlChar;
        return -(1 as libc::c_int);
    }
    (*ctxt).nameNr = 0 as libc::c_int;
    (*ctxt).name = 0 as *const xmlChar;
    /* Allocate the space stack */
    if (*ctxt).spaceTab.is_null() {
        (*ctxt).spaceTab = xmlMalloc.expect("non-null function pointer")(
            (10 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        (*ctxt).spaceMax = 10 as libc::c_int
    }
    if (*ctxt).spaceTab.is_null() {
        xmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"cannot initialize parser context\n\x00" as *const u8 as *const libc::c_char,
        );
        (*ctxt).nodeNr = 0 as libc::c_int;
        (*ctxt).nodeMax = 0 as libc::c_int;
        (*ctxt).node = 0 as xmlNodePtr;
        (*ctxt).inputNr = 0 as libc::c_int;
        (*ctxt).inputMax = 0 as libc::c_int;
        (*ctxt).input = 0 as xmlParserInputPtr;
        (*ctxt).nameNr = 0 as libc::c_int;
        (*ctxt).nameMax = 0 as libc::c_int;
        (*ctxt).name = 0 as *const xmlChar;
        (*ctxt).spaceNr = 0 as libc::c_int;
        (*ctxt).spaceMax = 0 as libc::c_int;
        (*ctxt).space = 0 as *mut libc::c_int;
        return -(1 as libc::c_int);
    }
    (*ctxt).spaceNr = 1 as libc::c_int;
    (*ctxt).spaceMax = 10 as libc::c_int;
    *(*ctxt).spaceTab.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
    (*ctxt).space = &mut *(*ctxt).spaceTab.offset(0 as libc::c_int as isize) as *mut libc::c_int;
    (*ctxt).userData = ctxt as *mut libc::c_void;
    (*ctxt).myDoc = 0 as xmlDocPtr;
    (*ctxt).wellFormed = 1 as libc::c_int;
    (*ctxt).nsWellFormed = 1 as libc::c_int;
    (*ctxt).valid = 1 as libc::c_int;
    (*ctxt).loadsubset = *__xmlLoadExtDtdDefaultValue();
    if (*ctxt).loadsubset != 0 {
        (*ctxt).options |= XML_PARSE_DTDLOAD as libc::c_int
    }
    (*ctxt).validate = *__xmlDoValidityCheckingDefaultValue();
    (*ctxt).pedantic = *__xmlPedanticParserDefaultValue();
    if (*ctxt).pedantic != 0 {
        (*ctxt).options |= XML_PARSE_PEDANTIC as libc::c_int
    }
    (*ctxt).linenumbers = *__xmlLineNumbersDefaultValue();
    (*ctxt).keepBlanks = *__xmlKeepBlanksDefaultValue();
    if (*ctxt).keepBlanks == 0 as libc::c_int {
        (*(*ctxt).sax).ignorableWhitespace = Some(
            xmlSAX2IgnorableWhitespace
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *const xmlChar,
                    _: libc::c_int,
                ) -> (),
        );
        (*ctxt).options |= XML_PARSE_NOBLANKS as libc::c_int
    }
    (*ctxt).vctxt.finishDtd = 0xabcd1234 as libc::c_uint;
    (*ctxt).vctxt.userData = ctxt as *mut libc::c_void;
    (*ctxt).vctxt.error = Some(
        xmlParserValidityError
            as unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char, _: ...) -> (),
    );
    (*ctxt).vctxt.warning = Some(
        xmlParserValidityWarning
            as unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char, _: ...) -> (),
    );
    if (*ctxt).validate != 0 {
        if *__xmlGetWarningsDefaultValue() == 0 as libc::c_int {
            (*ctxt).vctxt.warning = None
        } else {
            (*ctxt).vctxt.warning = Some(
                xmlParserValidityWarning
                    as unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: *const libc::c_char,
                        _: ...
                    ) -> (),
            )
        }
        (*ctxt).vctxt.nodeMax = 0 as libc::c_int;
        (*ctxt).options |= XML_PARSE_DTDVALID as libc::c_int
    }
    (*ctxt).replaceEntities = *__xmlSubstituteEntitiesDefaultValue();
    if (*ctxt).replaceEntities != 0 {
        (*ctxt).options |= XML_PARSE_NOENT as libc::c_int
    }
    (*ctxt).record_info = 0 as libc::c_int;
    (*ctxt).checkIndex = 0 as libc::c_int as libc::c_long;
    (*ctxt).inSubset = 0 as libc::c_int;
    (*ctxt).errNo = XML_ERR_OK as libc::c_int;
    (*ctxt).depth = 0 as libc::c_int;
    (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as libc::c_int;
    (*ctxt).catalogs = 0 as *mut libc::c_void;
    (*ctxt).nbentities = 0 as libc::c_int as libc::c_ulong;
    (*ctxt).sizeentities = 0 as libc::c_int as libc::c_ulong;
    (*ctxt).sizeentcopy = 0 as libc::c_int as libc::c_ulong;
    (*ctxt).input_id = 1 as libc::c_int;
    xmlInitNodeInfoSeq(&mut (*ctxt).node_seq);
    return 0 as libc::c_int;
}
/* *
 * xmlFreeParserCtxt:
 * @ctxt:  an XML parser context
 *
 * Free all the memory used by a parser context. However the parsed
 * document in ctxt->myDoc is not freed.
 */

pub unsafe fn xmlFreeParserCtxt_parserInternals(mut ctxt: xmlParserCtxtPtr) {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ctxt.is_null() {
        return;
    }
    loop {
        input = inputPop(ctxt);
        if input.is_null() {
            break;
        }
        /* Non consuming */
        xmlFreeInputStream(input);
    }
    if !(*ctxt).spaceTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).spaceTab as *mut libc::c_void);
    }
    if !(*ctxt).nameTab.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).nameTab as *mut *mut xmlChar as *mut libc::c_void,
        );
    }
    if !(*ctxt).nodeTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).nodeTab as *mut libc::c_void);
    }
    if !(*ctxt).nodeInfoTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).nodeInfoTab as *mut libc::c_void);
    }
    if !(*ctxt).inputTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).inputTab as *mut libc::c_void);
    }
    if !(*ctxt).version.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).version as *mut libc::c_char as *mut libc::c_void,
        );
    }
    if !(*ctxt).encoding.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).encoding as *mut libc::c_char as *mut libc::c_void,
        );
    }
    if !(*ctxt).extSubURI.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).extSubURI as *mut libc::c_char as *mut libc::c_void,
        );
    }
    if !(*ctxt).extSubSystem.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).extSubSystem as *mut libc::c_char as *mut libc::c_void,
        );
    }
    match () {
        #[cfg(HAVE_parserInternals_LIBXML_SAX1_ENABLED)]
        _ => {
            if !(*ctxt).sax.is_null() && (*ctxt).sax != __xmlDefaultSAXHandler() as xmlSAXHandlerPtr
            {
                /* LIBXML_SAX1_ENABLED */
                xmlFree.expect("non-null function pointer")((*ctxt).sax as *mut libc::c_void);
            }
        }
        #[cfg(not(HAVE_parserInternals_LIBXML_SAX1_ENABLED))]
        _ => {
            if !(*ctxt).sax.is_null() {
                /* LIBXML_SAX1_ENABLED */
                xmlFree.expect("non-null function pointer")((*ctxt).sax as *mut libc::c_void);
            }
        }
    };
    if !(*ctxt).directory.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).directory as *mut libc::c_void);
    }
    if !(*ctxt).vctxt.nodeTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).vctxt.nodeTab as *mut libc::c_void);
    }
    if !(*ctxt).atts.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).atts as *mut *mut xmlChar as *mut libc::c_void,
        );
    }
    if !(*ctxt).dict.is_null() {
        xmlDictFree((*ctxt).dict);
    }
    if !(*ctxt).nsTab.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).nsTab as *mut libc::c_char as *mut libc::c_void,
        );
    }
    if !(*ctxt).pushTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).pushTab as *mut libc::c_void);
    }
    if !(*ctxt).attallocs.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).attallocs as *mut libc::c_void);
    }
    if !(*ctxt).attsDefault.is_null() {
        xmlHashFree(
            (*ctxt).attsDefault,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: *const xmlChar) -> (),
            ),
        );
    }
    if !(*ctxt).attsSpecial.is_null() {
        xmlHashFree((*ctxt).attsSpecial, None);
    }
    if !(*ctxt).freeElems.is_null() {
        let mut cur: xmlNodePtr = 0 as *mut xmlNode;
        let mut next: xmlNodePtr = 0 as *mut xmlNode;
        cur = (*ctxt).freeElems;
        while !cur.is_null() {
            next = (*cur).next;
            xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void);
            cur = next
        }
    }
    if !(*ctxt).freeAttrs.is_null() {
        let mut cur_0: xmlAttrPtr = 0 as *mut xmlAttr;
        let mut next_0: xmlAttrPtr = 0 as *mut xmlAttr;
        cur_0 = (*ctxt).freeAttrs;
        while !cur_0.is_null() {
            next_0 = (*cur_0).next;
            xmlFree.expect("non-null function pointer")(cur_0 as *mut libc::c_void);
            cur_0 = next_0
        }
    }
    /*
     * cleanup the error strings
     */
    if !(*ctxt).lastError.message.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).lastError.message as *mut libc::c_void);
    }
    if !(*ctxt).lastError.file.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).lastError.file as *mut libc::c_void);
    }
    if !(*ctxt).lastError.str1.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).lastError.str1 as *mut libc::c_void);
    }
    if !(*ctxt).lastError.str2.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).lastError.str2 as *mut libc::c_void);
    }
    if !(*ctxt).lastError.str3.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).lastError.str3 as *mut libc::c_void);
    }

    match () {
        #[cfg(HAVE_parserInternals_LIBXML_CATALOG_ENABLED)]
        _ => {
            if !(*ctxt).catalogs.is_null() {
                xmlCatalogFreeLocal((*ctxt).catalogs);
            }
        }
        #[cfg(not(HAVE_parserInternals_LIBXML_CATALOG_ENABLED))]
        _ => {}
    };

    xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
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
    ctxt = xmlMalloc.expect("non-null function pointer")(
        ::std::mem::size_of::<xmlParserCtxt>() as libc::c_ulong
    ) as xmlParserCtxtPtr;
    if ctxt.is_null() {
        xmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"cannot allocate parser context\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlParserCtxtPtr;
    }
    memset(
        ctxt as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<xmlParserCtxt>() as libc::c_ulong,
    );
    if xmlInitParserCtxt(ctxt) < 0 as libc::c_int {
        xmlFreeParserCtxt(ctxt);
        return 0 as xmlParserCtxtPtr;
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
    if ctxt.is_null() {
        return;
    }
    xmlClearNodeInfoSeq(&mut (*ctxt).node_seq);
    xmlCtxtReset(ctxt);
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
    ctx: xmlParserCtxtPtr,
    node: xmlNodePtr,
) -> *const xmlParserNodeInfo {
    let mut pos: libc::c_ulong = 0;
    if ctx.is_null() || node.is_null() {
        return 0 as *const xmlParserNodeInfo;
    }
    /* Find position where node should be at */
    pos = xmlParserFindNodeInfoIndex(&mut (*ctx).node_seq, node);
    if pos < (*ctx).node_seq.length
        && (*(*ctx).node_seq.buffer.offset(pos as isize)).node == node as *const _xmlNode
    {
        return &mut *(*ctx).node_seq.buffer.offset(pos as isize) as *mut xmlParserNodeInfo;
    } else {
        return 0 as *const xmlParserNodeInfo;
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
    (*seq).length = 0 as libc::c_int as libc::c_ulong;
    (*seq).maximum = 0 as libc::c_int as libc::c_ulong;
    (*seq).buffer = 0 as *mut xmlParserNodeInfo;
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
    if !(*seq).buffer.is_null() {
        xmlFree.expect("non-null function pointer")((*seq).buffer as *mut libc::c_void);
    }
    xmlInitNodeInfoSeq(seq);
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

pub unsafe fn xmlParserFindNodeInfoIndex(
    seq: xmlParserNodeInfoSeqPtr,
    node: xmlNodePtr,
) -> libc::c_ulong {
    let mut upper: libc::c_ulong = 0;
    let mut lower: libc::c_ulong = 0;
    let mut middle: libc::c_ulong = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    if seq.is_null() || node.is_null() {
        return -(1 as libc::c_int) as libc::c_ulong;
    }
    /* Do a binary search for the key */
    lower = 1 as libc::c_int as libc::c_ulong;
    upper = (*seq).length;
    middle = 0 as libc::c_int as libc::c_ulong;
    while lower <= upper && found == 0 {
        middle = lower.wrapping_add(
            upper
                .wrapping_sub(lower)
                .wrapping_div(2 as libc::c_int as libc::c_ulong),
        );
        if node
            == (*(*seq)
                .buffer
                .offset(middle.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .node as xmlNodePtr
        {
            found = 1 as libc::c_int
        } else if node
            < (*(*seq)
                .buffer
                .offset(middle.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .node as xmlNodePtr
        {
            upper = middle.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            lower = middle.wrapping_add(1 as libc::c_int as libc::c_ulong)
        }
    }
    /* Return position */
    if middle == 0 as libc::c_int as libc::c_ulong
        || (*(*seq)
            .buffer
            .offset(middle.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        .node
            < node as *const _xmlNode
    {
        return middle;
    } else {
        return middle.wrapping_sub(1 as libc::c_int as libc::c_ulong);
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
    let mut pos: libc::c_ulong = 0;
    if ctxt.is_null() || info.is_null() {
        return;
    }
    /* Find pos and check to see if node is already in the sequence */
    pos = xmlParserFindNodeInfoIndex(&mut (*ctxt).node_seq, (*info).node as xmlNodePtr);
    if pos < (*ctxt).node_seq.length
        && !(*ctxt).node_seq.buffer.is_null()
        && (*(*ctxt).node_seq.buffer.offset(pos as isize)).node == (*info).node
    {
        *(*ctxt).node_seq.buffer.offset(pos as isize) = *info
    } else {
        /* Otherwise, we need to add new node to buffer */
        if (*ctxt)
            .node_seq
            .length
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            > (*ctxt).node_seq.maximum
            || (*ctxt).node_seq.buffer.is_null()
        {
            let mut tmp_buffer: *mut xmlParserNodeInfo = 0 as *mut xmlParserNodeInfo;
            let mut byte_size: libc::c_uint = 0;
            if (*ctxt).node_seq.maximum == 0 as libc::c_int as libc::c_ulong {
                (*ctxt).node_seq.maximum = 2 as libc::c_int as libc::c_ulong
            }
            byte_size = (::std::mem::size_of::<xmlParserNodeInfo>() as libc::c_ulong).wrapping_mul(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul((*ctxt).node_seq.maximum),
            ) as libc::c_uint;
            if (*ctxt).node_seq.buffer.is_null() {
                tmp_buffer = xmlMalloc.expect("non-null function pointer")(byte_size as size_t)
                    as *mut xmlParserNodeInfo
            } else {
                tmp_buffer = xmlRealloc.expect("non-null function pointer")(
                    (*ctxt).node_seq.buffer as *mut libc::c_void,
                    byte_size as size_t,
                ) as *mut xmlParserNodeInfo
            }
            if tmp_buffer.is_null() {
                xmlErrMemory(
                    ctxt,
                    b"failed to allocate buffer\n\x00" as *const u8 as *const libc::c_char,
                );
                return;
            }
            (*ctxt).node_seq.buffer = tmp_buffer;
            (*ctxt).node_seq.maximum = (*ctxt)
                .node_seq
                .maximum
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
        }
        /* If position is not at end, move elements out of the way */
        if pos != (*ctxt).node_seq.length {
            let mut i: libc::c_ulong = 0;
            i = (*ctxt).node_seq.length;
            while i > pos {
                *(*ctxt).node_seq.buffer.offset(i as isize) = *(*ctxt)
                    .node_seq
                    .buffer
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
                i = i.wrapping_sub(1)
            }
        }
        /* Copy element and increase length */
        *(*ctxt).node_seq.buffer.offset(pos as isize) = *info;
        (*ctxt).node_seq.length = (*ctxt).node_seq.length.wrapping_add(1)
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

pub unsafe fn xmlPedanticParserDefault(mut val: libc::c_int) -> libc::c_int {
    let mut old: libc::c_int = *__xmlPedanticParserDefaultValue();
    *__xmlPedanticParserDefaultValue() = val;
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

pub unsafe fn xmlLineNumbersDefault(mut val: libc::c_int) -> libc::c_int {
    let mut old: libc::c_int = *__xmlLineNumbersDefaultValue();
    *__xmlLineNumbersDefaultValue() = val;
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

pub unsafe fn xmlSubstituteEntitiesDefault(mut val: libc::c_int) -> libc::c_int {
    let mut old: libc::c_int = *__xmlSubstituteEntitiesDefaultValue();
    *__xmlSubstituteEntitiesDefaultValue() = val;
    return old;
}
/*
 * Input functions
 */
/*
 * Basic parsing Interfaces
 */
/* LIBXML_SAX1_ENABLED */
/* *
 * xmlKeepBlanksDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for default blanks text nodes support.
 * The 1.x version of the parser used an heuristic to try to detect
 * ignorable white spaces. As a result the SAX callback was generating
 * xmlSAX2IgnorableWhitespace() callbacks instead of characters() one, and when
 * using the DOM output text nodes containing those blanks were not generated.
 * The 2.x and later version will switch to the XML standard way and
 * ignorableWhitespace() are only generated when running the parser in
 * validating mode and when the current element doesn't allow CDATA or
 * mixed content.
 * This function is provided as a way to force the standard behavior
 * on 1.X libs and to switch back to the old mode for compatibility when
 * running 1.X client code on 2.X . Upgrade of 1.X code should be done
 * by using xmlIsBlankNode() commodity function to detect the "empty"
 * nodes generated.
 * This value also affect autogeneration of indentation when saving code
 * if blanks sections are kept, indentation is not generated.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */

pub unsafe fn xmlKeepBlanksDefault(mut val: libc::c_int) -> libc::c_int {
    let mut old: libc::c_int = *__xmlKeepBlanksDefaultValue();
    *__xmlKeepBlanksDefaultValue() = val;
    if val == 0 {
        *__xmlIndentTreeOutput() = 1 as libc::c_int
    }
    return old;
}
