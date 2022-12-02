/* *
* nameNsPush:
* @ctxt:  an XML parser context
* @value:  the element name
* @prefix:  the element prefix
* @URI:  the element namespace name
* @line:  the current line number for error messages
* @nsNr:  the number of namespaces pushed on the namespace table
*
* Pushes a new element name/prefix/URL on top of the name stack
*
* Returns -1 in case of error, the index in the stack otherwise
*/
unsafe fn nameNsPush(
    mut ctxt: xmlParserCtxtPtr,
    mut value: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
    mut line: i32,
    mut nsNr: i32,
) -> i32 {
    let mut current_block: u64;
    let mut tag: *mut xmlStartTag = 0 as *mut xmlStartTag;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).nameNr >= (safe_ctxt).nameMax {
        let mut tmp: *mut *const xmlChar = 0 as *mut *const xmlChar;
        let mut tmp2: *mut xmlStartTag = 0 as *mut xmlStartTag;
        (safe_ctxt).nameMax *= 2 as i32;
        tmp = xmlRealloc_safe(
            (safe_ctxt).nameTab as *mut *mut xmlChar as *mut (),
            ((safe_ctxt).nameMax as u64)
                .wrapping_mul(::std::mem::size_of::<*const xmlChar>() as u64),
        ) as *mut *const xmlChar;
        if tmp.is_null() {
            (safe_ctxt).nameMax /= 2 as i32;
            current_block = 678931122879618765;
        } else {
            (safe_ctxt).nameTab = tmp;
            tmp2 = xmlRealloc_safe(
                (safe_ctxt).pushTab as *mut *mut () as *mut (),
                ((safe_ctxt).nameMax as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlStartTag>() as u64),
            ) as *mut xmlStartTag;
            if tmp2.is_null() {
                (safe_ctxt).nameMax /= 2 as i32;
                current_block = 678931122879618765;
            } else {
                (safe_ctxt).pushTab = tmp2;
                current_block = 1054647088692577877;
            }
        }
    } else if (safe_ctxt).pushTab.is_null() {
        (safe_ctxt).pushTab = xmlMalloc_safe(
            ((safe_ctxt).nameMax as u64)
                .wrapping_mul(::std::mem::size_of::<xmlStartTag>() as u64),
        ) as *mut xmlStartTag;
        if (safe_ctxt).pushTab.is_null() {
            current_block = 678931122879618765;
        } else {
            current_block = 1054647088692577877;
        }
    } else {
        current_block = 1054647088692577877;
    }
    match current_block {
        678931122879618765 => {
            unsafe {
                xmlErrMemory(ctxt, 0 as *const i8);
            }
            return -(1 as i32);
        }
        _ => {
            unsafe {
                let ref mut fresh20 = *(*ctxt).nameTab.offset((safe_ctxt).nameNr as isize);
                *fresh20 = value;
                (safe_ctxt).name = value;
                tag = &mut *(*ctxt).pushTab.offset((safe_ctxt).nameNr as isize) as *mut xmlStartTag;
                (*tag).prefix = prefix;
                (*tag).URI = URI;
                (*tag).line = line;
                (*tag).nsNr = nsNr;
            }
            let fresh21 = (safe_ctxt).nameNr;
            (safe_ctxt).nameNr = (safe_ctxt).nameNr + 1;
            return fresh21;
        }
    };
}
/* *
* nameNsPop:
* @ctxt: an XML parser context
*
* Pops the top element/prefix/URI name from the name stack
*
* Returns the name just removed
*/
#[cfg(HAVE_parser_LIBXML_PUSH_ENABLED)]
unsafe fn nameNsPop(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).nameNr <= 0 as i32 {
        return 0 as *const xmlChar;
    }
    (safe_ctxt).nameNr -= 1;
    if (safe_ctxt).nameNr > 0 as i32 {
        (safe_ctxt).name = unsafe {
            *(*ctxt)
                .nameTab
                .offset(((safe_ctxt).nameNr - 1 as i32) as isize)
        };
    } else {
        (safe_ctxt).name = 0 as *const xmlChar
    }
    ret = unsafe { *(*ctxt).nameTab.offset((safe_ctxt).nameNr as isize) };
    unsafe {
        let ref mut fresh22 = *(*ctxt).nameTab.offset((safe_ctxt).nameNr as isize);
        *fresh22 = 0 as *const xmlChar;
    }
    return ret;
}
/* LIBXML_PUSH_ENABLED */
/* *
* namePush:
* @ctxt:  an XML parser context
* @value:  the element name
*
* Pushes a new element name on top of the name stack
*
* Returns -1 in case of error, the index in the stack otherwise
*/

pub unsafe fn namePush(mut ctxt: xmlParserCtxtPtr, mut value: *const xmlChar) -> i32 {
    if ctxt.is_null() {
        return -(1 as i32);
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).nameNr >= (safe_ctxt).nameMax {
        let mut tmp: *mut *const xmlChar = 0 as *mut *const xmlChar;
        tmp = xmlRealloc_safe(
            (safe_ctxt).nameTab as *mut *mut xmlChar as *mut (),
            (((safe_ctxt).nameMax * 2 as i32) as u64)
                .wrapping_mul(::std::mem::size_of::<*const xmlChar>() as u64),
        ) as *mut *const xmlChar;
        if tmp.is_null() {
            unsafe {
                xmlErrMemory(ctxt, 0 as *const i8);
            }
            return -(1 as i32);
        } else {
            (safe_ctxt).nameTab = tmp;
            (safe_ctxt).nameMax *= 2 as i32
        }
    }
    unsafe {
        let ref mut fresh23 = *(*ctxt).nameTab.offset((safe_ctxt).nameNr as isize);
        *fresh23 = value;
    }
    (safe_ctxt).name = value;
    let fresh24 = (safe_ctxt).nameNr;
    (safe_ctxt).nameNr = (safe_ctxt).nameNr + 1;
    return fresh24;
}
/* *
* namePop:
* @ctxt: an XML parser context
*
* Pops the top element name from the name stack
*
* Returns the name just removed
*/

pub unsafe fn namePop(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    if ctxt.is_null() || unsafe { (*ctxt).nameNr <= 0 as i32 } {
        return 0 as *const xmlChar;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    (safe_ctxt).nameNr -= 1;
    if (safe_ctxt).nameNr > 0 as i32 {
        (safe_ctxt).name = unsafe {
            *(*ctxt)
                .nameTab
                .offset(((safe_ctxt).nameNr - 1 as i32) as isize)
        };
    } else {
        (safe_ctxt).name = 0 as *const xmlChar
    }
    unsafe {
        ret = *(*ctxt).nameTab.offset((safe_ctxt).nameNr as isize);
        let ref mut fresh25 = *(*ctxt).nameTab.offset((safe_ctxt).nameNr as isize);
        *fresh25 = 0 as *const xmlChar;
    }
    return ret;
}
unsafe fn spacePush(mut ctxt: xmlParserCtxtPtr, mut val: i32) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).spaceNr >= (safe_ctxt).spaceMax {
        let mut tmp: *mut i32 = 0 as *mut i32;
        (safe_ctxt).spaceMax *= 2 as i32;
        tmp = xmlRealloc_safe(
            (safe_ctxt).spaceTab as *mut (),
            ((safe_ctxt).spaceMax as u64)
                .wrapping_mul(::std::mem::size_of::<i32>() as u64),
        ) as *mut i32;
        if tmp.is_null() {
            unsafe {
                xmlErrMemory(ctxt, 0 as *const i8);
            }
            (safe_ctxt).spaceMax /= 2 as i32;
            return -(1 as i32);
        }
        (safe_ctxt).spaceTab = tmp
    }
    unsafe {
        *(*ctxt).spaceTab.offset((safe_ctxt).spaceNr as isize) = val;
        (safe_ctxt).space =
            &mut *(*ctxt).spaceTab.offset((safe_ctxt).spaceNr as isize) as *mut i32;
    }
    let fresh26 = (safe_ctxt).spaceNr;
    (safe_ctxt).spaceNr = (safe_ctxt).spaceNr + 1;
    return fresh26;
}
unsafe fn spacePop(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let mut ret: i32 = 0;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).spaceNr <= 0 as i32 {
        return 0 as i32;
    }
    (safe_ctxt).spaceNr -= 1;
    if (safe_ctxt).spaceNr > 0 as i32 {
        (safe_ctxt).space = unsafe {
            &mut *(*ctxt)
                .spaceTab
                .offset(((safe_ctxt).spaceNr - 1 as i32) as isize)
                as *mut i32
        };
    } else {
        (safe_ctxt).space =
            unsafe { &mut *(*ctxt).spaceTab.offset(0 as i32 as isize) as *mut i32 };
    }
    unsafe {
        ret = *(*ctxt).spaceTab.offset((safe_ctxt).spaceNr as isize);
        *(*ctxt).spaceTab.offset((safe_ctxt).spaceNr as isize) = -(1 as i32);
    }
    return ret;
}
unsafe fn xmlSHRINK(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    xmlParserInputShrink_safe((safe_ctxt).input);
    if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
        xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
    };
}
unsafe fn xmlGROW(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut curEnd: ptrdiff_t =
        unsafe { (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64 };
    let mut curBase: ptrdiff_t =
        unsafe { (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64 };
    if (curEnd > 10000000 as i32 as i64
        || curBase > 10000000 as i32 as i64)
        && unsafe {
            (!(*(*ctxt).input).buf.is_null()
                && (*(*(*ctxt).input).buf).readcallback
                    != Some(
                        xmlInputReadCallbackNop
                            as unsafe extern "C" fn(
                                _: *mut (),
                                _: *mut i8,
                                _: i32,
                            ) -> i32,
                    ))
        }
        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
    {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"Huge input lookup\x00" as *const u8 as *const i8,
            );
            xmlHaltParser(ctxt);
        }
        return;
    }
    xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
    if unsafe {
        (*(*ctxt).input).cur > (*(*ctxt).input).end || (*(*ctxt).input).cur < (*(*ctxt).input).base
    } {
        unsafe {
            xmlHaltParser(ctxt);
            xmlFatalErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"cur index out of bound\x00" as *const u8 as *const i8,
            );
        }
        return;
    }
    if unsafe {
        !(*(*ctxt).input).cur.is_null() && *(*(*ctxt).input).cur as i32 == 0 as i32
    } {
        xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
    };
}
/* *
* xmlSkipBlankChars:
* @ctxt:  the XML parser context
*
* skip all blanks character found at that point in the input streams.
* It pops up finished entities in the process if allowable at that point.
*
* Returns the number of space chars skipped
*/

pub unsafe fn xmlSkipBlankChars(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let mut res: i32 = 0 as i32;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    /*
     * It's Okay to use CUR/NEXT here since all the blanks are on
     * the ASCII range.
     */
    if (safe_ctxt).instate as i32 != XML_PARSER_DTD as i32 {
        let mut cur: *const xmlChar = 0 as *const xmlChar;
        /*
         * if we are in the document content, go really fast
         */
        cur = unsafe { (*(*ctxt).input).cur };
        while unsafe { *cur } as i32 == 0x20 as i32
            || 0x9 as i32 <= unsafe { *cur } as i32
                && unsafe { *cur } as i32 <= 0xa as i32
            || unsafe { *cur } as i32 == 0xd as i32
        {
            if unsafe { *cur } as i32 == '\n' as i32 {
                unsafe {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as i32;
                }
            } else {
                unsafe { (*(*ctxt).input).col += 1 }
            }
            cur = unsafe { cur.offset(1) };
            res += 1;
            if unsafe { *cur } as i32 == 0 as i32 {
                unsafe {
                    (*(*ctxt).input).cur = cur;
                    xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
                    cur = (*(*ctxt).input).cur;
                }
            }
        }
        unsafe {
            (*(*ctxt).input).cur = cur;
        }
    } else {
        let mut expandPE: i32 = ((safe_ctxt).external != 0 as i32
            || (safe_ctxt).inputNr != 1 as i32)
            as i32;
        loop {
            if unsafe {
                *(*(*ctxt).input).cur as i32 == 0x20 as i32
                    || 0x9 as i32 <= *(*(*ctxt).input).cur as i32
                        && *(*(*ctxt).input).cur as i32 <= 0xa as i32
                    || *(*(*ctxt).input).cur as i32 == 0xd as i32
            } {
                /* CHECKED tstblanks.xml */
                xmlNextChar_safe(ctxt);
            } else if unsafe { *(*(*ctxt).input).cur as i32 == '%' as i32 } {
                /*
                 * Need to handle support of entities branching here
                 */
                if expandPE == 0 as i32
                    || unsafe {
                        (*(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                            == 0x20 as i32
                            || 0x9 as i32
                                <= *(*(*ctxt).input).cur.offset(1 as i32 as isize)
                                    as i32
                                && *(*(*ctxt).input).cur.offset(1 as i32 as isize)
                                    as i32
                                    <= 0xa as i32
                            || *(*(*ctxt).input).cur.offset(1 as i32 as isize)
                                as i32
                                == 0xd as i32)
                            || *(*(*ctxt).input).cur.offset(1 as i32 as isize)
                                as i32
                                == 0 as i32
                    }
                {
                    break;
                }
                unsafe {
                    xmlParsePEReference(ctxt);
                }
            } else {
                if unsafe { !(*(*(*ctxt).input).cur as i32 == 0 as i32) } {
                    break;
                }
                if (safe_ctxt).inputNr <= 1 as i32 {
                    break;
                }
                xmlPopInput_safe(ctxt);
            }
            /*
             * Also increase the counter when entering or exiting a PERef.
             * The spec says: "When a parameter-entity reference is recognized
             * in the DTD and included, its replacement text MUST be enlarged
             * by the attachment of one leading and one following space (#x20)
             * character."
             */
            res += 1
        }
    }
    return res;
}
/* ***********************************************************************
*									*
*		Commodity functions to handle entities			*
*									*
************************************************************************/
/* *
* xmlPopInput:
* @ctxt:  an XML parser context
*
* xmlPopInput: the current input pointed by ctxt->input came to an end
*          pop it and return the next char.
*
* Returns the current xmlChar in the parser context
*/

pub unsafe fn xmlPopInput_parser(mut ctxt: xmlParserCtxtPtr) -> xmlChar {
    if ctxt.is_null() || unsafe { (*ctxt).inputNr <= 1 as i32 } {
        return 0 as i32 as xmlChar;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if unsafe { *__xmlParserDebugEntities() != 0 } {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Popping input %d\n\x00" as *const u8 as *const i8,
                (safe_ctxt).inputNr,
            );
        }
    }
    if (safe_ctxt).inputNr > 1 as i32
        && (safe_ctxt).inSubset == 0 as i32
        && (safe_ctxt).instate as i32 != XML_PARSER_EOF as i32
    {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"Unfinished entity outside the DTD\x00" as *const u8 as *const i8,
            );
        }
    }
    xmlFreeInputStream_safe(inputPop_parser(ctxt));
    if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
        xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
    }
    return unsafe { *(*(*ctxt).input).cur };
}
/* *
* xmlPushInput:
* @ctxt:  an XML parser context
* @input:  an XML parser input fragment (entity, XML fragment ...).
*
* xmlPushInput: switch to a new input stream which is stacked on top
*               of the previous one(s).
* Returns -1 in case of error or the index in the input stack
*/

pub unsafe fn xmlPushInput(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputPtr,
) -> i32 {
    let mut ret: i32 = 0;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if input.is_null() {
        return -(1 as i32);
    }
    if unsafe { *__xmlParserDebugEntities() != 0 } {
        if !(safe_ctxt).input.is_null() && unsafe { !(*(*ctxt).input).filename.is_null() } {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"%s(%d): \x00" as *const u8 as *const i8,
                    (*(*ctxt).input).filename,
                    (*(*ctxt).input).line,
                );
            }
        }
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Pushing input %d : %.30s\n\x00" as *const u8 as *const i8,
                (safe_ctxt).inputNr + 1 as i32,
                (*input).cur,
            );
        }
    }
    if (safe_ctxt).inputNr > 40 as i32
        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
        || (safe_ctxt).inputNr > 1024 as i32
    {
        unsafe {
            xmlFatalErr(ctxt, XML_ERR_ENTITY_LOOP, 0 as *const i8);
            while (1 < 2) {
                if ((*ctxt).inputNr > 1) {
                    xmlFreeInputStream_safe(inputPop_parser(ctxt));
                } else {
                    break;
                }
            }
        }
        return -(1 as i32);
    }
    ret = inputPush_safe(ctxt, input);
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return -(1 as i32);
    }
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    return ret;
}
/* *
* xmlParseCharRef:
* @ctxt:  an XML parser context
*
* parse Reference declarations
*
* [66] CharRef ::= '&#' [0-9]+ ';' |
*                  '&#x' [0-9a-fA-F]+ ';'
*
* [ WFC: Legal Character ]
* Characters referred to using character references must match the
* production for Char.
*
* Returns the value parsed (as an int), 0 in case of error
*/

pub unsafe fn xmlParseCharRef(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let mut val: i32 = 0 as i32;
    let mut count: i32 = 0 as i32;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    /*
     * Using RAW/CUR/NEXT is okay since we are working on ASCII range here
     */
    if unsafe {
        *(*(*ctxt).input).cur as i32 == '&' as i32
            && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '#' as i32
            && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32 == 'x' as i32
    } {
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(3 as i32 as isize);
            (*(*ctxt).input).col += 3 as i32;
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
            xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
        }
        if unsafe {
            (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
        } {
            xmlGROW(ctxt);
        }

        while 1<2 {
            if unsafe { *(*(*ctxt).input).cur as i32 == ';' as i32 }{
                break;
            }
            /* loop blocked by count */
            let fresh27 = count;
            count = count + 1;
            if fresh27 > 20 as i32 {
                count = 0 as i32;
                if unsafe {
                    (safe_ctxt).progressive == 0 as i32
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                } {
                    xmlGROW(ctxt);
                }
                if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                    return 0 as i32;
                }
            }
            if unsafe {
                *(*(*ctxt).input).cur as i32 >= '0' as i32
                    && *(*(*ctxt).input).cur as i32 <= '9' as i32
            } {
                val = val * 16 as i32
                    + unsafe { (*(*(*ctxt).input).cur as i32 - '0' as i32) };
            } else if unsafe {
                *(*(*ctxt).input).cur as i32 >= 'a' as i32
                    && *(*(*ctxt).input).cur as i32 <= 'f' as i32
            } && count < 20 as i32
            {
                val = unsafe {
                    val * 16 as i32
                        + (*(*(*ctxt).input).cur as i32 - 'a' as i32)
                        + 10 as i32
                };
            } else if unsafe {
                *(*(*ctxt).input).cur as i32 >= 'A' as i32
                    && *(*(*ctxt).input).cur as i32 <= 'F' as i32
            } && count < 20 as i32
            {
                val = unsafe {
                    val * 16 as i32
                        + (*(*(*ctxt).input).cur as i32 - 'A' as i32)
                        + 10 as i32
                }
            } else {
                unsafe {
                    xmlFatalErr(ctxt, XML_ERR_INVALID_HEX_CHARREF, 0 as *const i8);
                }
                val = 0 as i32;
                break;
            }
            if val > 0x110000 as i32 {
                val = 0x110000 as i32
            }
            xmlNextChar_safe(ctxt);
            count += 1
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == ';' as i32 } {
            /* on purpose to avoid reentrancy problems with NEXT and SKIP */
            unsafe {
                (*(*ctxt).input).col += 1;
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
            }
        }
    } else if unsafe {
        *(*(*ctxt).input).cur as i32 == '&' as i32
            && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '#' as i32
    } {
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
            (*(*ctxt).input).col += 2 as i32;
        }

        if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
            xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
        }
        if unsafe {
            (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
        } {
            xmlGROW(ctxt);
        }
        while 1<2 {
            if unsafe { *(*(*ctxt).input).cur as i32 == ';' as i32 }{
                break;
            }
            /* loop blocked by count */
            let fresh28 = count;
            count = count + 1;
            if fresh28 > 20 as i32 {
                count = 0 as i32;
                if unsafe {
                    (*ctxt).progressive == 0 as i32
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                } {
                    xmlGROW(ctxt);
                }
                if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                    return 0 as i32;
                }
            }
            if unsafe {
                *(*(*ctxt).input).cur as i32 >= '0' as i32
                    && *(*(*ctxt).input).cur as i32 <= '9' as i32
            } {
                val = unsafe {
                    val * 10 as i32 + (*(*(*ctxt).input).cur as i32 - '0' as i32)
                };
                if val > 0x110000 as i32 {
                    val = 0x110000 as i32
                }
                xmlNextChar_safe(ctxt);
                count += 1
            } else {
                unsafe {
                    xmlFatalErr(ctxt, XML_ERR_INVALID_DEC_CHARREF, 0 as *const i8);
                }
                val = 0 as i32;
                break;
            }
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == ';' as i32 } {
            /* on purpose to avoid reentrancy problems with NEXT and SKIP */
            unsafe {
                (*(*ctxt).input).col += 1;
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
            }
        }
    } else {
        unsafe {
            xmlFatalErr(ctxt, XML_ERR_INVALID_CHARREF, 0 as *const i8);
        }
    }
    /*
     * [ WFC: Legal Character ]
     * Characters referred to using character references must match the
     * production for Char.
     */
    if val >= 0x110000 as i32 {
        unsafe {
            xmlFatalErrMsgInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"xmlParseCharRef: character reference out of bounds\n\x00" as *const u8
                    as *const i8,
                val,
            );
        }
    } else if if val < 0x100 as i32 {
        (0x9 as i32 <= val && val <= 0xa as i32
            || val == 0xd as i32
            || 0x20 as i32 <= val) as i32
    } else {
        (0x100 as i32 <= val && val <= 0xd7ff as i32
            || 0xe000 as i32 <= val && val <= 0xfffd as i32
            || 0x10000 as i32 <= val && val <= 0x10ffff as i32)
            as i32
    } != 0
    {
        return val;
    } else {
        unsafe {
            xmlFatalErrMsgInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"xmlParseCharRef: invalid xmlChar value %d\n\x00" as *const u8
                    as *const i8,
                val,
            );
        }
    }
    return 0 as i32;
}
/* *
* xmlParseStringCharRef:
* @ctxt:  an XML parser context
* @str:  a pointer to an index in the string
*
* parse Reference declarations, variant parsing from a string rather
* than an an input flow.
*
* [66] CharRef ::= '&#' [0-9]+ ';' |
*                  '&#x' [0-9a-fA-F]+ ';'
*
* [ WFC: Legal Character ]
* Characters referred to using character references must match the
* production for Char.
*
* Returns the value parsed (as an int), 0 in case of error, str will be
*         updated to the current value of the index
*/
unsafe fn xmlParseStringCharRef(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *mut *const xmlChar,
) -> i32 {
    let mut ptr: *const xmlChar = 0 as *const xmlChar;
    let mut cur: xmlChar = 0;
    let mut val: i32 = 0 as i32;
    if str.is_null() || unsafe { (*str).is_null() } {
        return 0 as i32;
    }
    unsafe {
        ptr = *str;
        cur = *ptr;
    }
    if cur as i32 == '&' as i32
        && unsafe {
            *ptr.offset(1 as i32 as isize) as i32 == '#' as i32
                && *ptr.offset(2 as i32 as isize) as i32 == 'x' as i32
        }
    {
        unsafe {
            ptr = ptr.offset(3 as i32 as isize);
            cur = *ptr;
        }
        while cur as i32 != ';' as i32 {
            /* Non input consuming loop */
            if cur as i32 >= '0' as i32 && cur as i32 <= '9' as i32 {
                val = val * 16 as i32 + (cur as i32 - '0' as i32)
            } else if cur as i32 >= 'a' as i32 && cur as i32 <= 'f' as i32 {
                val =
                    val * 16 as i32 + (cur as i32 - 'a' as i32) + 10 as i32
            } else if cur as i32 >= 'A' as i32 && cur as i32 <= 'F' as i32 {
                val =
                    val * 16 as i32 + (cur as i32 - 'A' as i32) + 10 as i32
            } else {
                unsafe {
                    xmlFatalErr(ctxt, XML_ERR_INVALID_HEX_CHARREF, 0 as *const i8);
                }
                val = 0 as i32;
                break;
            }
            if val > 0x110000 as i32 {
                val = 0x110000 as i32
            }
            unsafe {
                ptr = ptr.offset(1);
                cur = *ptr;
            }
        }
        if cur as i32 == ';' as i32 {
            ptr = unsafe { ptr.offset(1) }
        }
    } else if cur as i32 == '&' as i32
        && unsafe { *ptr.offset(1 as i32 as isize) as i32 == '#' as i32 }
    {
        unsafe {
            ptr = ptr.offset(2 as i32 as isize);
            cur = *ptr;
        }
        while cur as i32 != ';' as i32 {
            /* Non input consuming loops */
            if cur as i32 >= '0' as i32 && cur as i32 <= '9' as i32 {
                val = val * 10 as i32 + (cur as i32 - '0' as i32);
                if val > 0x110000 as i32 {
                    val = 0x110000 as i32
                }
                unsafe {
                    ptr = ptr.offset(1);
                    cur = *ptr
                }
            } else {
                unsafe {
                    xmlFatalErr(ctxt, XML_ERR_INVALID_DEC_CHARREF, 0 as *const i8);
                }
                val = 0 as i32;
                break;
            }
        }
        if cur as i32 == ';' as i32 {
            ptr = unsafe { ptr.offset(1) }
        }
    } else {
        unsafe {
            xmlFatalErr(ctxt, XML_ERR_INVALID_CHARREF, 0 as *const i8);
        }
        return 0 as i32;
    }
    unsafe { *str = ptr };
    /*
     * [ WFC: Legal Character ]
     * Characters referred to using character references must match the
     * production for Char.
     */
    if val >= 0x110000 as i32 {
        unsafe {
            xmlFatalErrMsgInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"xmlParseStringCharRef: character reference out of bounds\n\x00" as *const u8
                    as *const i8,
                val,
            );
        }
    } else if if val < 0x100 as i32 {
        (0x9 as i32 <= val && val <= 0xa as i32
            || val == 0xd as i32
            || 0x20 as i32 <= val) as i32
    } else {
        (0x100 as i32 <= val && val <= 0xd7ff as i32
            || 0xe000 as i32 <= val && val <= 0xfffd as i32
            || 0x10000 as i32 <= val && val <= 0x10ffff as i32)
            as i32
    } != 0
    {
        return val;
    } else {
        unsafe {
            xmlFatalErrMsgInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"xmlParseStringCharRef: invalid xmlChar value %d\n\x00" as *const u8
                    as *const i8,
                val,
            );
        }
    }
    return 0 as i32;
}
/* *
* xmlParserHandlePEReference:
* @ctxt:  the parser context
*
* [69] PEReference ::= '%' Name ';'
*
* [ WFC: No Recursion ]
* A parsed entity must not contain a recursive
* reference to itself, either directly or indirectly.
*
* [ WFC: Entity Declared ]
* In a document without any DTD, a document with only an internal DTD
* subset which contains no parameter entity references, or a document
* with "standalone='yes'", ...  ... The declaration of a parameter
* entity must precede any reference to it...
*
* [ VC: Entity Declared ]
* In a document with an external subset or external parameter entities
* with "standalone='no'", ...  ... The declaration of a parameter entity
* must precede any reference to it...
*
* [ WFC: In DTD ]
* Parameter-entity references may only appear in the DTD.
* NOTE: misleading but this is handled.
*
* A PEReference may have been detected in the current input stream
* the handling is done accordingly to
*      http://www.w3.org/TR/REC-xml#entproc
* i.e.
*   - Included in literal in entity values
*   - Included as Parameter Entity reference within DTDs
*/

pub unsafe fn xmlParserHandlePEReference(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    match (safe_ctxt).instate as i32 {
        8 => return,
        5 => return,
        6 => return,
        9 => return,
        -1 => {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_PEREF_AT_EOF, 0 as *const i8);
            }
            return;
        }
        4 | 0 | 1 => {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_PEREF_IN_PROLOG, 0 as *const i8);
            }
            return;
        }
        10 | 7 | 12 | 2 | 13 | 16 => {
            /* we just ignore it there */
            return;
        }
        14 => {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_PEREF_IN_EPILOG, 0 as *const i8);
            }
            return;
        }
        11 => {
            /*
             * NOTE: in the case of entity values, we don't do the
             *       substitution here since we need the literal
             *       entity value to be able to save the internal
             *       subset of the document.
             *       This will be handled by xmlStringDecodeEntities
             */
            return;
        }
        3 => {
            /*
             * [WFC: Well-Formedness Constraint: PEs in Internal Subset]
             * In the internal DTD subset, parameter-entity references
             * can occur only where markup declarations can occur, not
             * within markup declarations.
             * In that case this is handled in xmlParseMarkupDecl
             */
            if (safe_ctxt).external == 0 as i32 && (safe_ctxt).inputNr == 1 as i32 {
                return;
            }
            if unsafe {
                *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == 0x20 as i32
                    || 0x9 as i32
                        <= *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                            <= 0xa as i32
                    || *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        == 0xd as i32
                    || *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        == 0 as i32
            } {
                return;
            }
        }
        15 => return,
        _ => {}
    }
    unsafe {
        xmlParsePEReference(ctxt);
    }
}
/*
* Macro used to grow the current buffer.
* buffer##_size is expected to be a size_t
* mem_error: is expected to handle memory allocation failures
*/
/* *
* xmlStringLenDecodeEntities:
* @ctxt:  the parser context
* @str:  the input string
* @len: the string length
* @what:  combination of XML_SUBSTITUTE_REF and XML_SUBSTITUTE_PEREF
* @end:  an end marker xmlChar, 0 if none
* @end2:  an end marker xmlChar, 0 if none
* @end3:  an end marker xmlChar, 0 if none
*
* Takes a entity string content and process to do the adequate substitutions.
*
* [67] Reference ::= EntityRef | CharRef
*
* [69] PEReference ::= '%' Name ';'
*
* Returns A newly allocated string with the substitution done. The caller
*      must deallocate it !
*/

pub unsafe fn xmlStringLenDecodeEntities(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut len: i32,
    mut what: i32,
    mut end: xmlChar,
    mut end2: xmlChar,
    mut end3: xmlChar,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
    let mut buffer_size: size_t = 0 as i32 as size_t;
    let mut nbchars: size_t = 0 as i32 as size_t;
    let mut current: *mut xmlChar = 0 as *mut xmlChar;
    let mut rep: *mut xmlChar = 0 as *mut xmlChar;
    let mut last: *const xmlChar = 0 as *const xmlChar;
    let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut c: i32 = 0;
    let mut l: i32 = 0;
    if ctxt.is_null() || str.is_null() || len < 0 as i32 {
        return 0 as *mut xmlChar;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    last = unsafe { str.offset(len as isize) };
    if (safe_ctxt).depth > 40 as i32
        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
        || (safe_ctxt).depth > 1024 as i32
    {
        unsafe {
            xmlFatalErr(ctxt, XML_ERR_ENTITY_LOOP, 0 as *const i8);
        }
        return 0 as *mut xmlChar;
    }
    /*
     * allocate a translation buffer.
     */
    buffer_size = 300 as i32 as size_t;
    buffer = xmlMallocAtomic_safe(buffer_size) as *mut xmlChar;
    if buffer.is_null() {
        current_block = 13264933720371784297;
    } else {
        /*
         * OK loop until we reach one of the ending char or a size limit.
         * we are operating on already parsed values.
         */
        if str < last {
            unsafe {
                c = xmlStringCurrentChar(ctxt, str, &mut l);
            }
        } else {
            c = 0 as i32
        }
        's_81: loop {
            if !(c != 0 as i32
                && c != end as i32
                && c != end2 as i32
                && c != end3 as i32
                && (safe_ctxt).instate as i32 != XML_PARSER_EOF as i32)
            {
                current_block = 13810333397648094191;
                break;
            }
            if c == 0 as i32 {
                current_block = 13810333397648094191;
                break;
            }
            if c == '&' as i32
                && unsafe { *str.offset(1 as i32 as isize) as i32 == '#' as i32 }
            {
                let mut val: i32 = xmlParseStringCharRef(ctxt, &mut str);
                if val == 0 as i32 {
                    current_block = 7451279748152143041;
                    break;
                }
                if 0 as i32 == 1 as i32 {
                    let fresh29 = nbchars;
                    nbchars = nbchars.wrapping_add(1);
                    unsafe { *buffer.offset(fresh29 as isize) = val as xmlChar }
                } else {
                    nbchars = unsafe {
                        (nbchars as u64).wrapping_add(xmlCopyCharMultiByte(
                            &mut *buffer.offset(nbchars as isize),
                            val,
                        )
                            as u64) as size_t as size_t
                    };
                }
                if nbchars.wrapping_add(100 as i32 as u64) > buffer_size {
                    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                    let mut new_size: size_t = buffer_size
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(100 as i32 as u64);
                    if new_size < buffer_size {
                        current_block = 13264933720371784297;
                        break;
                    }
                    tmp = xmlRealloc_safe(buffer as *mut (), new_size) as *mut xmlChar;
                    if tmp.is_null() {
                        current_block = 13264933720371784297;
                        break;
                    }
                    buffer = tmp;
                    buffer_size = new_size
                }
            } else if c == '&' as i32 && what & 1 as i32 != 0 {
                if unsafe { *__xmlParserDebugEntities() != 0 } {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"String decoding Entity Reference: %.30s\n\x00" as *const u8
                                as *const i8,
                            str,
                        );
                    }
                }
                unsafe {
                    ent = xmlParseStringEntityRef(ctxt, &mut str);
                    xmlParserEntityCheck(
                        ctxt,
                        0 as i32 as size_t,
                        ent,
                        0 as i32 as size_t,
                    );
                }
                if !ent.is_null() {
                    (safe_ctxt).nbentities = unsafe {
                        (safe_ctxt)
                            .nbentities
                            .wrapping_add(((*ent).checked / 2 as i32) as u64)
                    }
                }
                if !ent.is_null()
                    && unsafe {
                        (*ent).etype as u32
                            == XML_INTERNAL_PREDEFINED_ENTITY as i32 as u32
                    }
                {
                    if unsafe { !(*ent).content.is_null() } {
                        if 0 as i32 == 1 as i32 {
                            let fresh30 = nbchars;
                            nbchars = nbchars.wrapping_add(1);
                            unsafe {
                                *buffer.offset(fresh30 as isize) =
                                    *(*ent).content.offset(0 as i32 as isize);
                            }
                        } else {
                            nbchars = unsafe {
                                (nbchars as u64).wrapping_add(xmlCopyCharMultiByte(
                                    &mut *buffer.offset(nbchars as isize),
                                    *(*ent).content.offset(0 as i32 as isize)
                                        as i32,
                                )
                                    as u64) as size_t
                                    as size_t
                            }
                        }
                        if nbchars.wrapping_add(100 as i32 as u64) > buffer_size {
                            let mut tmp_0: *mut xmlChar = 0 as *mut xmlChar;
                            let mut new_size_0: size_t = buffer_size
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(100 as i32 as u64);
                            if new_size_0 < buffer_size {
                                current_block = 13264933720371784297;
                                break;
                            }
                            tmp_0 = xmlRealloc_safe(buffer as *mut (), new_size_0)
                                as *mut xmlChar;
                            if tmp_0.is_null() {
                                current_block = 13264933720371784297;
                                break;
                            }
                            buffer = tmp_0;
                            buffer_size = new_size_0
                        }
                    } else {
                        unsafe {
                            xmlFatalErrMsg(
                                ctxt,
                                XML_ERR_INTERNAL_ERROR,
                                b"predefined entity has no content\n\x00" as *const u8
                                    as *const i8,
                            );
                        }
                        current_block = 7451279748152143041;
                        break;
                    }
                } else if !ent.is_null() && unsafe { !(*ent).content.is_null() } {
                    (safe_ctxt).depth += 1;
                    rep = unsafe {
                        xmlStringDecodeEntities(
                            ctxt,
                            (*ent).content,
                            what,
                            0 as i32 as xmlChar,
                            0 as i32 as xmlChar,
                            0 as i32 as xmlChar,
                        )
                    };
                    (safe_ctxt).depth -= 1;
                    if rep.is_null() {
                        unsafe {
                            *(*ent).content.offset(0 as i32 as isize) =
                                0 as i32 as xmlChar;
                        }
                        current_block = 7451279748152143041;
                        break;
                    } else {
                        current = rep;
                        while unsafe { *current } as i32 != 0 as i32 {
                            /* non input consuming loop */
                            let fresh31 = current;
                            current = unsafe { current.offset(1) };
                            let fresh32 = nbchars;
                            nbchars = nbchars.wrapping_add(1);
                            unsafe { *buffer.offset(fresh32 as isize) = *fresh31 };
                            if !(nbchars.wrapping_add(100 as i32 as u64)
                                > buffer_size)
                            {
                                continue;
                            }
                            if xmlParserEntityCheck(ctxt, nbchars, ent, 0 as i32 as size_t)
                                != 0
                            {
                                current_block = 7451279748152143041;
                                break 's_81;
                            }
                            let mut tmp_1: *mut xmlChar = 0 as *mut xmlChar;
                            let mut new_size_1: size_t = buffer_size
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(100 as i32 as u64);
                            if new_size_1 < buffer_size {
                                current_block = 13264933720371784297;
                                break 's_81;
                            }
                            tmp_1 = xmlRealloc_safe(buffer as *mut (), new_size_1)
                                as *mut xmlChar;
                            if tmp_1.is_null() {
                                current_block = 13264933720371784297;
                                break 's_81;
                            }
                            buffer = tmp_1;
                            buffer_size = new_size_1
                        }
                        xmlFree_safe(rep as *mut ());
                        rep = 0 as *mut xmlChar
                    }
                } else if !ent.is_null() {
                    let mut i: i32 = xmlStrlen_safe(unsafe { (*ent).name });
                    let mut cur: *const xmlChar = unsafe { (*ent).name };
                    let fresh33 = nbchars;
                    nbchars = nbchars.wrapping_add(1);
                    unsafe {
                        *buffer.offset(fresh33 as isize) = '&' as i32 as xmlChar;
                    }
                    if nbchars
                        .wrapping_add(i as u64)
                        .wrapping_add(100 as i32 as u64)
                        > buffer_size
                    {
                        let mut tmp_2: *mut xmlChar = 0 as *mut xmlChar;
                        let mut new_size_2: size_t = buffer_size
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(i as u64)
                            .wrapping_add(100 as i32 as u64);
                        if new_size_2 < buffer_size {
                            current_block = 13264933720371784297;
                            break;
                        }
                        tmp_2 = xmlRealloc_safe(buffer as *mut (), new_size_2)
                            as *mut xmlChar;
                        if tmp_2.is_null() {
                            current_block = 13264933720371784297;
                            break;
                        }
                        buffer = tmp_2;
                        buffer_size = new_size_2
                    }
                    while i > 0 as i32 {
                        let fresh34 = cur;
                        cur = unsafe { cur.offset(1) };
                        let fresh35 = nbchars;
                        nbchars = nbchars.wrapping_add(1);
                        unsafe { *buffer.offset(fresh35 as isize) = *fresh34 };
                        i -= 1
                    }
                    let fresh36 = nbchars;
                    nbchars = nbchars.wrapping_add(1);
                    unsafe { *buffer.offset(fresh36 as isize) = ';' as i32 as xmlChar }
                }
            } else if c == '%' as i32 && what & 2 as i32 != 0 {
                if unsafe { *__xmlParserDebugEntities() != 0 } {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"String decoding PE Reference: %.30s\n\x00" as *const u8
                                as *const i8,
                            str,
                        );
                    }
                }
                ent = unsafe { xmlParseStringPEReference(ctxt, &mut str) };
                xmlParserEntityCheck(
                    ctxt,
                    0 as i32 as size_t,
                    ent,
                    0 as i32 as size_t,
                );
                if !ent.is_null() {
                    (safe_ctxt).nbentities = unsafe {
                        (safe_ctxt)
                            .nbentities
                            .wrapping_add(((*ent).checked / 2 as i32) as u64)
                    };
                }
                if !ent.is_null() {
                    if unsafe { (*ent).content.is_null() } {
                        /*
                         * Note: external parsed entities will not be loaded,
                         * it is not required for a non-validating parser to
                         * complete external PEReferences coming from the
                         * internal subset
                         */
                        if (safe_ctxt).options & XML_PARSE_NOENT as i32 != 0 as i32
                            || (safe_ctxt).options & XML_PARSE_DTDVALID as i32
                                != 0 as i32
                            || (safe_ctxt).validate != 0 as i32
                        {
                            unsafe {
                                xmlLoadEntityContent(ctxt, ent);
                            }
                        } else {
                            unsafe {
                                xmlWarningMsg(
                                    ctxt,
                                    XML_ERR_ENTITY_PROCESSING,
                                    b"not validating will not read content for PE entity %s\n\x00"
                                        as *const u8
                                        as *const i8,
                                    (*ent).name,
                                    0 as *const xmlChar,
                                );
                            }
                        }
                    }
                    (safe_ctxt).depth += 1;
                    rep = unsafe {
                        xmlStringDecodeEntities(
                            ctxt,
                            (*ent).content,
                            what,
                            0 as i32 as xmlChar,
                            0 as i32 as xmlChar,
                            0 as i32 as xmlChar,
                        )
                    };

                    (safe_ctxt).depth -= 1;
                    if rep.is_null() {
                        if unsafe { !(*ent).content.is_null() } {
                            unsafe {
                                *(*ent).content.offset(0 as i32 as isize) =
                                    0 as i32 as xmlChar
                            }
                        }
                        current_block = 7451279748152143041;
                        break;
                    } else {
                        current = rep;
                        while unsafe { *current as i32 != 0 as i32 } {
                            /* non input consuming loop */
                            let fresh37 = current;
                            current = unsafe { current.offset(1) };
                            let fresh38 = nbchars;
                            nbchars = nbchars.wrapping_add(1);
                            unsafe { *buffer.offset(fresh38 as isize) = *fresh37 };
                            if !(nbchars.wrapping_add(100 as i32 as u64)
                                > buffer_size)
                            {
                                continue;
                            }
                            if xmlParserEntityCheck(ctxt, nbchars, ent, 0 as i32 as size_t)
                                != 0
                            {
                                current_block = 7451279748152143041;
                                break 's_81;
                            }
                            let mut tmp_3: *mut xmlChar = 0 as *mut xmlChar;
                            let mut new_size_3: size_t = buffer_size
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(100 as i32 as u64);
                            if new_size_3 < buffer_size {
                                current_block = 13264933720371784297;
                                break 's_81;
                            }
                            tmp_3 = xmlRealloc_safe(buffer as *mut (), new_size_3)
                                as *mut xmlChar;
                            if tmp_3.is_null() {
                                current_block = 13264933720371784297;
                                break 's_81;
                            }
                            buffer = tmp_3;
                            buffer_size = new_size_3
                        }
                        xmlFree_safe(rep as *mut ());
                        rep = 0 as *mut xmlChar
                    }
                }
            } else {
                if l == 1 as i32 {
                    let fresh39 = nbchars;
                    nbchars = nbchars.wrapping_add(1);
                    unsafe { *buffer.offset(fresh39 as isize) = c as xmlChar }
                } else {
                    nbchars = unsafe {
                        (nbchars as u64).wrapping_add(xmlCopyCharMultiByte(
                            &mut *buffer.offset(nbchars as isize),
                            c,
                        )
                            as u64) as size_t as size_t
                    }
                }
                str = unsafe { str.offset(l as isize) };
                if nbchars.wrapping_add(100 as i32 as u64) > buffer_size {
                    let mut tmp_4: *mut xmlChar = 0 as *mut xmlChar;
                    let mut new_size_4: size_t = buffer_size
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(100 as i32 as u64);
                    if new_size_4 < buffer_size {
                        current_block = 13264933720371784297;
                        break;
                    }
                    tmp_4 =
                        xmlRealloc_safe(buffer as *mut (), new_size_4) as *mut xmlChar;
                    if tmp_4.is_null() {
                        current_block = 13264933720371784297;
                        break;
                    }
                    buffer = tmp_4;
                    buffer_size = new_size_4
                }
            }
            if str < last {
                unsafe { c = xmlStringCurrentChar(ctxt, str, &mut l) }
            } else {
                c = 0 as i32
            }
        }
        match current_block {
            13264933720371784297 => {}
            7451279748152143041 => {}
            _ => {
                unsafe {
                    *buffer.offset(nbchars as isize) = 0 as i32 as xmlChar;
                }
                return buffer;
            }
        }
    }
    match current_block {
        13264933720371784297 => unsafe {
            xmlErrMemory(ctxt, 0 as *const i8);
        },
        _ => {}
    }
    if !rep.is_null() {
        xmlFree_safe(rep as *mut ());
    }
    if !buffer.is_null() {
        xmlFree_safe(buffer as *mut ());
    }
    return 0 as *mut xmlChar;
}
/* *
* xmlStringDecodeEntities:
* @ctxt:  the parser context
* @str:  the input string
* @what:  combination of XML_SUBSTITUTE_REF and XML_SUBSTITUTE_PEREF
* @end:  an end marker xmlChar, 0 if none
* @end2:  an end marker xmlChar, 0 if none
* @end3:  an end marker xmlChar, 0 if none
*
* Takes a entity string content and process to do the adequate substitutions.
*
* [67] Reference ::= EntityRef | CharRef
*
* [69] PEReference ::= '%' Name ';'
*
* Returns A newly allocated string with the substitution done. The caller
*      must deallocate it !
*/

pub unsafe fn xmlStringDecodeEntities(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut what: i32,
    mut end: xmlChar,
    mut end2: xmlChar,
    mut end3: xmlChar,
) -> *mut xmlChar {
    if ctxt.is_null() || str.is_null() {
        return 0 as *mut xmlChar;
    }
    return xmlStringLenDecodeEntities(ctxt, str, xmlStrlen_safe(str), what, end, end2, end3);
}
/* ***********************************************************************
*									*
*		Commodity functions, cleanup needed ?			*
*									*
************************************************************************/
/* *
* areBlanks:
* @ctxt:  an XML parser context
* @str:  a xmlChar *
* @len:  the size of @str
* @blank_chars: we know the chars are blanks
*
* Is this a sequence of blank chars that one can ignore ?
*
* Returns 1 if ignorable 0 otherwise.
*/
unsafe fn areBlanks(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut len: i32,
    mut blank_chars: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    let mut lastChild: xmlNodePtr = 0 as *mut xmlNode;
    /*
     * Don't spend time trying to differentiate them, the same callback is
     * used !
     */
    if unsafe { (*(*ctxt).sax).ignorableWhitespace == (*(*ctxt).sax).characters } {
        return 0 as i32;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    /*
     * Check for xml:space value.
     */
    if unsafe {
        (*ctxt).space.is_null()
            || *(*ctxt).space == 1 as i32
            || *(*ctxt).space == -(2 as i32)
    } {
        return 0 as i32;
    }
    /*
     * Check that the string is made of blanks
     */
    if blank_chars == 0 as i32 {
        i = 0 as i32;
        while i < len {
            if unsafe {
                !(*str.offset(i as isize) as i32 == 0x20 as i32
                    || 0x9 as i32 <= *str.offset(i as isize) as i32
                        && *str.offset(i as isize) as i32 <= 0xa as i32
                    || *str.offset(i as isize) as i32 == 0xd as i32)
            } {
                return 0 as i32;
            }
            i += 1
        }
    }
    /*
     * Look if the element is mixed content in the DTD if available
     */
    if (safe_ctxt).node.is_null() {
        return 0 as i32;
    }
    if !(safe_ctxt).myDoc.is_null() {
        ret = unsafe { xmlIsMixedElement((safe_ctxt).myDoc, (*(*ctxt).node).name) };
        if ret == 0 as i32 {
            return 1 as i32;
        }
        if ret == 1 as i32 {
            return 0 as i32;
        }
    }
    /*
     * Otherwise, heuristic :-\
     */
    if unsafe {
        *(*(*ctxt).input).cur as i32 != '<' as i32
            && *(*(*ctxt).input).cur as i32 != 0xd as i32
    } {
        return 0 as i32;
    }
    if unsafe {
        (*(*ctxt).node).children.is_null()
            && *(*(*ctxt).input).cur as i32 == '<' as i32
            && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '/' as i32
    } {
        return 0 as i32;
    }
    lastChild = xmlGetLastChild_safe((safe_ctxt).node as *const xmlNode);
    if lastChild.is_null() {
        if unsafe {
            (*(*ctxt).node).type_0 as u32
                != XML_ELEMENT_NODE as i32 as u32
                && !(*(*ctxt).node).content.is_null()
        } {
            return 0 as i32;
        }
    } else if xmlNodeIsText_safe(lastChild as *const xmlNode) != 0 {
        return 0 as i32;
    } else {
        if unsafe {
            !(*(*ctxt).node).children.is_null() && xmlNodeIsText((*(*ctxt).node).children) != 0
        } {
            return 0 as i32;
        }
    }
    return 1 as i32;
}
/* ***********************************************************************
*									*
*		Extra stuff for namespace support			*
*	Relates to http://www.w3.org/TR/WD-xml-names			*
*									*
************************************************************************/
/* *
* xmlSplitQName:
* @ctxt:  an XML parser context
* @name:  an XML parser context
* @prefix:  a xmlChar **
*
* parse an UTF8 encoded XML qualified name string
*
* [NS 5] QName ::= (Prefix ':')? LocalPart
*
* [NS 6] Prefix ::= NCName
*
* [NS 7] LocalPart ::= NCName
*
* Returns the local part, and prefix is updated
*   to get the Prefix if any.
*/

pub unsafe fn xmlSplitQName(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const xmlChar,
    mut prefix: *mut *mut xmlChar,
) -> *mut xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: i32 = 0 as i32;
    let mut max: i32 = 100 as i32;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: *const xmlChar = name;
    let mut c: i32 = 0;
    if prefix.is_null() {
        return 0 as *mut xmlChar;
    }
    unsafe { *prefix = 0 as *mut xmlChar };
    if cur.is_null() {
        return 0 as *mut xmlChar;
    }

    match () {
        #[cfg(HAVE_parser_XML_XML_NAMESPACE)]
        _ => {}
        #[cfg(not(HAVE_parser_XML_XML_NAMESPACE))]
        _ => {
            if unsafe { *cur.offset(0 as i32 as isize) as i32 == 'x' as i32 } {
                if unsafe { *cur.offset(1 as i32 as isize) as i32 == 'm' as i32 } {
                    if unsafe {
                        *cur.offset(2 as i32 as isize) as i32 == 'l' as i32
                    } {
                        if unsafe {
                            *cur.offset(3 as i32 as isize) as i32 == ':' as i32
                        } {
                            return xmlStrdup_safe(name);
                        }
                    }
                }
            }
        }
    };

    /* nasty but well=formed */
    if unsafe { *cur.offset(0 as i32 as isize) as i32 == ':' as i32 } {
        return xmlStrdup_safe(name);
    }
    let fresh40 = cur;
    unsafe {
        cur = cur.offset(1);
        c = *fresh40 as i32;
    }
    while c != 0 as i32 && c != ':' as i32 && len < max {
        /* tested bigname.xml */
        let fresh41 = len;
        len = len + 1;
        buf[fresh41 as usize] = c as xmlChar;
        let fresh42 = cur;
        unsafe {
            cur = cur.offset(1);
            c = *fresh42 as i32
        }
    }
    if len >= max {
        /*
         * Okay someone managed to make a huge name, so he's ready to pay
         * for the processing speed.
         */
        max = len * 2 as i32;
        buffer = xmlMallocAtomic_safe(
            (max as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
        ) as *mut xmlChar;
        if buffer.is_null() {
            unsafe {
                xmlErrMemory(ctxt, 0 as *const i8);
            }
            return 0 as *mut xmlChar;
        }
        memcpy_safe(
            buffer as *mut (),
            buf.as_mut_ptr() as *const (),
            len as u64,
        );
        while c != 0 as i32 && c != ':' as i32 {
            /* tested bigname.xml */
            if len + 10 as i32 > max {
                let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                max *= 2 as i32;
                tmp = xmlRealloc_safe(
                    buffer as *mut (),
                    (max as u64)
                        .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                ) as *mut xmlChar;
                if tmp.is_null() {
                    xmlFree_safe(buffer as *mut ());
                    unsafe {
                        xmlErrMemory(ctxt, 0 as *const i8);
                    }
                    return 0 as *mut xmlChar;
                }
                buffer = tmp
            }
            let fresh43 = len;
            len = len + 1;
            unsafe {
                *buffer.offset(fresh43 as isize) = c as xmlChar;
            }
            let fresh44 = cur;
            unsafe {
                cur = cur.offset(1);
                c = *fresh44 as i32;
            }
        }
        unsafe { *buffer.offset(len as isize) = 0 as i32 as xmlChar }
    }
    if c == ':' as i32 && unsafe { *cur as i32 == 0 as i32 } {
        if !buffer.is_null() {
            xmlFree_safe(buffer as *mut ());
        }
        unsafe {
            *prefix = 0 as *mut xmlChar;
        }
        return xmlStrdup_safe(name);
    }
    if buffer.is_null() {
        ret = xmlStrndup_safe(buf.as_mut_ptr(), len)
    } else {
        ret = buffer;
        buffer = 0 as *mut xmlChar;
        max = 100 as i32
    }
    if c == ':' as i32 {
        unsafe {
            c = *cur as i32;
            *prefix = ret;
        }
        if c == 0 as i32 {
            return xmlStrndup_safe(
                b"\x00" as *const u8 as *const i8 as *mut xmlChar,
                0 as i32,
            );
        }
        len = 0 as i32;
        /*
         * Check that the first character is proper to start
         * a new name
         */
        if !(c >= 0x61 as i32 && c <= 0x7a as i32
            || c >= 0x41 as i32 && c <= 0x5a as i32
            || c == '_' as i32
            || c == ':' as i32)
        {
            let mut l: i32 = 0;
            let mut first: i32 = unsafe { xmlStringCurrentChar(ctxt, cur, &mut l) };
            if !((if first < 0x100 as i32 {
                (0x41 as i32 <= first && first <= 0x5a as i32
                    || 0x61 as i32 <= first && first <= 0x7a as i32
                    || 0xc0 as i32 <= first && first <= 0xd6 as i32
                    || 0xd8 as i32 <= first && first <= 0xf6 as i32
                    || 0xf8 as i32 <= first) as i32
            } else {
                unsafe {
                    xmlCharInRange_safe(first as u32, unsafe { &xmlIsBaseCharGroup })
                }
            }) != 0
                || (if first < 0x100 as i32 {
                    0 as i32
                } else {
                    (0x4e00 as i32 <= first && first <= 0x9fa5 as i32
                        || first == 0x3007 as i32
                        || 0x3021 as i32 <= first && first <= 0x3029 as i32)
                        as i32
                }) != 0)
                && first != '_' as i32
            {
                unsafe {
                    xmlFatalErrMsgStr(
                        ctxt,
                        XML_NS_ERR_QNAME,
                        b"Name %s is not XML Namespace compliant\n\x00" as *const u8
                            as *const i8,
                        name,
                    );
                }
            }
        }
        cur = unsafe { cur.offset(1) };
        while c != 0 as i32 && len < max {
            /* tested bigname2.xml */
            let fresh45 = len;
            len = len + 1;
            buf[fresh45 as usize] = c as xmlChar;
            let fresh46 = cur;
            unsafe {
                cur = cur.offset(1);
                c = *fresh46 as i32
            }
        }
        if len >= max {
            /*
             * Okay someone managed to make a huge name, so he's ready to pay
             * for the processing speed.
             */
            max = len * 2 as i32;
            buffer = xmlMallocAtomic_safe(
                (max as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
            ) as *mut xmlChar;
            if buffer.is_null() {
                unsafe {
                    xmlErrMemory(ctxt, 0 as *const i8);
                }
                return 0 as *mut xmlChar;
            }
            memcpy_safe(
                buffer as *mut (),
                buf.as_mut_ptr() as *const (),
                len as u64,
            );
            while c != 0 as i32 {
                /* tested bigname2.xml */
                if len + 10 as i32 > max {
                    let mut tmp_0: *mut xmlChar = 0 as *mut xmlChar;
                    max *= 2 as i32;
                    tmp_0 = xmlRealloc_safe(
                        buffer as *mut (),
                        (max as u64)
                            .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                    ) as *mut xmlChar;
                    if tmp_0.is_null() {
                        unsafe {
                            xmlErrMemory(ctxt, 0 as *const i8);
                        }
                        xmlFree_safe(buffer as *mut ());
                        return 0 as *mut xmlChar;
                    }
                    buffer = tmp_0
                }
                let fresh47 = len;
                len = len + 1;
                unsafe {
                    *buffer.offset(fresh47 as isize) = c as xmlChar;
                }
                let fresh48 = cur;
                unsafe {
                    cur = cur.offset(1);
                    c = *fresh48 as i32
                }
            }
            unsafe { *buffer.offset(len as isize) = 0 as i32 as xmlChar }
        }
        if buffer.is_null() {
            ret = xmlStrndup_safe(buf.as_mut_ptr(), len)
        } else {
            ret = buffer
        }
    }
    return ret;
}
/* ***********************************************************************
*									*
*			The parser itself				*
*	Relates to http://www.w3.org/TR/REC-xml				*
*									*
************************************************************************/
/* ***********************************************************************
*									*
*	Routines to parse Name, NCName and NmToken			*
*									*
************************************************************************/

#[cfg(HAVE_parser_DEBUG)]
const nbParseName: i64 = 0 as i64;
#[cfg(HAVE_parser_DEBUG)]
const nbParseNmToken: i64 = 0 as i64;
#[cfg(HAVE_parser_DEBUG)]
const nbParseNCName: i64 = 0 as i64;
#[cfg(HAVE_parser_DEBUG)]
const nbParseNCNameComplex: i64 = 0 as i64;
#[cfg(HAVE_parser_DEBUG)]
const nbParseNameComplex: i64 = 0 as i64;
#[cfg(HAVE_parser_DEBUG)]
const nbParseStringName: i64 = 0 as i64;

/*
* The two following functions are related to the change of accepted
* characters for Name and NmToken in the Revision 5 of XML-1.0
* They correspond to the modified production [4] and the new production [4a]
* changes in that revision. Also note that the macros used for the
* productions Letter, Digit, CombiningChar and Extender are not needed
* anymore.
* We still keep compatibility to pre-revision5 parsing semantic if the
* new XML_PARSE_OLD10 option is given to the parser.
*/
unsafe fn xmlIsNameStartChar(mut ctxt: xmlParserCtxtPtr, mut c: i32) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).options & XML_PARSE_OLD10 as i32 == 0 as i32 {
        /*
         * Use the new checks of production [4] [4a] amd [5] of the
         * Update 5 of XML-1.0
         */
        if c != ' ' as i32
            && c != '>' as i32
            && c != '/' as i32
            && (c >= 'a' as i32 && c <= 'z' as i32
                || c >= 'A' as i32 && c <= 'Z' as i32
                || c == '_' as i32
                || c == ':' as i32
                || c >= 0xc0 as i32 && c <= 0xd6 as i32
                || c >= 0xd8 as i32 && c <= 0xf6 as i32
                || c >= 0xf8 as i32 && c <= 0x2ff as i32
                || c >= 0x370 as i32 && c <= 0x37d as i32
                || c >= 0x37f as i32 && c <= 0x1fff as i32
                || c >= 0x200c as i32 && c <= 0x200d as i32
                || c >= 0x2070 as i32 && c <= 0x218f as i32
                || c >= 0x2c00 as i32 && c <= 0x2fef as i32
                || c >= 0x3001 as i32 && c <= 0xd7ff as i32
                || c >= 0xf900 as i32 && c <= 0xfdcf as i32
                || c >= 0xfdf0 as i32 && c <= 0xfffd as i32
                || c >= 0x10000 as i32 && c <= 0xeffff as i32)
        {
            return 1 as i32;
        }
    } else if (if c < 0x100 as i32 {
        (0x41 as i32 <= c && c <= 0x5a as i32
            || 0x61 as i32 <= c && c <= 0x7a as i32
            || 0xc0 as i32 <= c && c <= 0xd6 as i32
            || 0xd8 as i32 <= c && c <= 0xf6 as i32
            || 0xf8 as i32 <= c) as i32
    } else {
        xmlCharInRange_safe(c as u32, unsafe { &xmlIsBaseCharGroup })
    }) != 0
        || (if c < 0x100 as i32 {
            0 as i32
        } else {
            (0x4e00 as i32 <= c && c <= 0x9fa5 as i32
                || c == 0x3007 as i32
                || 0x3021 as i32 <= c && c <= 0x3029 as i32)
                as i32
        }) != 0
        || c == '_' as i32
        || c == ':' as i32
    {
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe fn xmlIsNameChar(mut ctxt: xmlParserCtxtPtr, mut c: i32) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).options & XML_PARSE_OLD10 as i32 == 0 as i32 {
        /*
         * Use the new checks of production [4] [4a] amd [5] of the
         * Update 5 of XML-1.0
         */
        if c != ' ' as i32
            && c != '>' as i32
            && c != '/' as i32
            && (c >= 'a' as i32 && c <= 'z' as i32
                || c >= 'A' as i32 && c <= 'Z' as i32
                || c >= '0' as i32 && c <= '9' as i32
                || c == '_' as i32
                || c == ':' as i32
                || c == '-' as i32
                || c == '.' as i32
                || c == 0xb7 as i32
                || c >= 0xc0 as i32 && c <= 0xd6 as i32
                || c >= 0xd8 as i32 && c <= 0xf6 as i32
                || c >= 0xf8 as i32 && c <= 0x2ff as i32
                || c >= 0x300 as i32 && c <= 0x36f as i32
                || c >= 0x370 as i32 && c <= 0x37d as i32
                || c >= 0x37f as i32 && c <= 0x1fff as i32
                || c >= 0x200c as i32 && c <= 0x200d as i32
                || c >= 0x203f as i32 && c <= 0x2040 as i32
                || c >= 0x2070 as i32 && c <= 0x218f as i32
                || c >= 0x2c00 as i32 && c <= 0x2fef as i32
                || c >= 0x3001 as i32 && c <= 0xd7ff as i32
                || c >= 0xf900 as i32 && c <= 0xfdcf as i32
                || c >= 0xfdf0 as i32 && c <= 0xfffd as i32
                || c >= 0x10000 as i32 && c <= 0xeffff as i32)
        {
            return 1 as i32;
        }
    } else if (if c < 0x100 as i32 {
        (0x41 as i32 <= c && c <= 0x5a as i32
            || 0x61 as i32 <= c && c <= 0x7a as i32
            || 0xc0 as i32 <= c && c <= 0xd6 as i32
            || 0xd8 as i32 <= c && c <= 0xf6 as i32
            || 0xf8 as i32 <= c) as i32
    } else {
        xmlCharInRange_safe(c as u32, unsafe { &xmlIsBaseCharGroup })
    }) != 0
        || (if c < 0x100 as i32 {
            0 as i32
        } else {
            (0x4e00 as i32 <= c && c <= 0x9fa5 as i32
                || c == 0x3007 as i32
                || 0x3021 as i32 <= c && c <= 0x3029 as i32)
                as i32
        }) != 0
        || (if c < 0x100 as i32 {
            (0x30 as i32 <= c && c <= 0x39 as i32) as i32
        } else {
            xmlCharInRange_safe(c as u32, unsafe { &xmlIsDigitGroup })
        }) != 0
        || c == '.' as i32
        || c == '-' as i32
        || c == '_' as i32
        || c == ':' as i32
        || (if c < 0x100 as i32 {
            0 as i32
        } else {
            xmlCharInRange_safe(c as u32, unsafe { &xmlIsCombiningGroup })
        }) != 0
        || (if c < 0x100 as i32 {
            (c == 0xb7 as i32) as i32
        } else {
            xmlCharInRange_safe(c as u32, unsafe { &xmlIsExtenderGroup })
        }) != 0
    {
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe fn xmlParseNameComplex(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut len: i32 = 0 as i32;
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    let mut count: i32 = 0 as i32;

    match () {
        #[cfg(HAVE_parser_DEBUG)]
        _ => {
            nbParseNameComplex = nbParseNameComplex + 1;
        }
        #[cfg(not(HAVE_parser_DEBUG))]
        _ => {}
    };

    /*
     * Handler for more complex cases
     */
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return 0 as *const xmlChar;
    }
    c = unsafe { xmlCurrentChar(ctxt, &mut l) };
    if (safe_ctxt).options & XML_PARSE_OLD10 as i32 == 0 as i32 {
        /*
         * Use the new checks of production [4] [4a] amd [5] of the
         * Update 5 of XML-1.0
         */
        if c == ' ' as i32
            || c == '>' as i32
            || c == '/' as i32
            || !(c >= 'a' as i32 && c <= 'z' as i32
                || c >= 'A' as i32 && c <= 'Z' as i32
                || c == '_' as i32
                || c == ':' as i32
                || c >= 0xc0 as i32 && c <= 0xd6 as i32
                || c >= 0xd8 as i32 && c <= 0xf6 as i32
                || c >= 0xf8 as i32 && c <= 0x2ff as i32
                || c >= 0x370 as i32 && c <= 0x37d as i32
                || c >= 0x37f as i32 && c <= 0x1fff as i32
                || c >= 0x200c as i32 && c <= 0x200d as i32
                || c >= 0x2070 as i32 && c <= 0x218f as i32
                || c >= 0x2c00 as i32 && c <= 0x2fef as i32
                || c >= 0x3001 as i32 && c <= 0xd7ff as i32
                || c >= 0xf900 as i32 && c <= 0xfdcf as i32
                || c >= 0xfdf0 as i32 && c <= 0xfffd as i32
                || c >= 0x10000 as i32 && c <= 0xeffff as i32)
        {
            return 0 as *const xmlChar;
        }
        len += l;
        if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
            unsafe {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as i32
            }
        } else {
            unsafe { (*(*ctxt).input).col += 1 }
        }
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            c = xmlCurrentChar(ctxt, &mut l);
        }
        while c != ' ' as i32
            && c != '>' as i32
            && c != '/' as i32
            && (c >= 'a' as i32 && c <= 'z' as i32
                || c >= 'A' as i32 && c <= 'Z' as i32
                || c >= '0' as i32 && c <= '9' as i32
                || c == '_' as i32
                || c == ':' as i32
                || c == '-' as i32
                || c == '.' as i32
                || c == 0xb7 as i32
                || c >= 0xc0 as i32 && c <= 0xd6 as i32
                || c >= 0xd8 as i32 && c <= 0xf6 as i32
                || c >= 0xf8 as i32 && c <= 0x2ff as i32
                || c >= 0x300 as i32 && c <= 0x36f as i32
                || c >= 0x370 as i32 && c <= 0x37d as i32
                || c >= 0x37f as i32 && c <= 0x1fff as i32
                || c >= 0x200c as i32 && c <= 0x200d as i32
                || c >= 0x203f as i32 && c <= 0x2040 as i32
                || c >= 0x2070 as i32 && c <= 0x218f as i32
                || c >= 0x2c00 as i32 && c <= 0x2fef as i32
                || c >= 0x3001 as i32 && c <= 0xd7ff as i32
                || c >= 0xf900 as i32 && c <= 0xfdcf as i32
                || c >= 0xfdf0 as i32 && c <= 0xfffd as i32
                || c >= 0x10000 as i32 && c <= 0xeffff as i32)
        {
            let fresh49 = count;
            count = count + 1;
            if fresh49 > 100 as i32 {
                count = 0 as i32;
                if unsafe {
                    (*ctxt).progressive == 0 as i32
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                } {
                    xmlGROW(ctxt);
                }
                if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                    return 0 as *const xmlChar;
                }
            }
            len += l;
            if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
                unsafe {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as i32
                }
            } else {
                unsafe { (*(*ctxt).input).col += 1 }
            }
            unsafe {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                c = xmlCurrentChar(ctxt, &mut l)
            }
        }
    } else {
        if c == ' ' as i32
            || c == '>' as i32
            || c == '/' as i32
            || !((if c < 0x100 as i32 {
                (0x41 as i32 <= c && c <= 0x5a as i32
                    || 0x61 as i32 <= c && c <= 0x7a as i32
                    || 0xc0 as i32 <= c && c <= 0xd6 as i32
                    || 0xd8 as i32 <= c && c <= 0xf6 as i32
                    || 0xf8 as i32 <= c) as i32
            } else {
                xmlCharInRange_safe(c as u32, unsafe { &xmlIsBaseCharGroup })
            }) != 0
                || (if c < 0x100 as i32 {
                    0 as i32
                } else {
                    (0x4e00 as i32 <= c && c <= 0x9fa5 as i32
                        || c == 0x3007 as i32
                        || 0x3021 as i32 <= c && c <= 0x3029 as i32)
                        as i32
                }) != 0)
                && c != '_' as i32
                && c != ':' as i32
        {
            return 0 as *const xmlChar;
        }
        len += l;
        if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
            unsafe {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as i32
            }
        } else {
            unsafe { (*(*ctxt).input).col += 1 }
        }
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            c = xmlCurrentChar(ctxt, &mut l);
        }
        while c != ' ' as i32
            && c != '>' as i32
            && c != '/' as i32
            && ((if c < 0x100 as i32 {
                (0x41 as i32 <= c && c <= 0x5a as i32
                    || 0x61 as i32 <= c && c <= 0x7a as i32
                    || 0xc0 as i32 <= c && c <= 0xd6 as i32
                    || 0xd8 as i32 <= c && c <= 0xf6 as i32
                    || 0xf8 as i32 <= c) as i32
            } else {
                xmlCharInRange_safe(c as u32, unsafe { &xmlIsBaseCharGroup })
            }) != 0
                || (if c < 0x100 as i32 {
                    0 as i32
                } else {
                    (0x4e00 as i32 <= c && c <= 0x9fa5 as i32
                        || c == 0x3007 as i32
                        || 0x3021 as i32 <= c && c <= 0x3029 as i32)
                        as i32
                }) != 0
                || (if c < 0x100 as i32 {
                    (0x30 as i32 <= c && c <= 0x39 as i32) as i32
                } else {
                    xmlCharInRange_safe(c as u32, unsafe { &xmlIsDigitGroup })
                }) != 0
                || c == '.' as i32
                || c == '-' as i32
                || c == '_' as i32
                || c == ':' as i32
                || (if c < 0x100 as i32 {
                    0 as i32
                } else {
                    xmlCharInRange_safe(c as u32, unsafe { &xmlIsCombiningGroup })
                }) != 0
                || (if c < 0x100 as i32 {
                    (c == 0xb7 as i32) as i32
                } else {
                    xmlCharInRange_safe(c as u32, unsafe { &xmlIsExtenderGroup })
                }) != 0)
        {
            let fresh50 = count;
            count = count + 1;
            if fresh50 > 100 as i32 {
                count = 0 as i32;
                if unsafe {
                    (*ctxt).progressive == 0 as i32
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                } {
                    xmlGROW(ctxt);
                }
                if unsafe { (*ctxt).instate as i32 == XML_PARSER_EOF as i32 } {
                    return 0 as *const xmlChar;
                }
            }
            len += l;
            if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
                unsafe {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as i32
                }
            } else {
                unsafe { (*(*ctxt).input).col += 1 }
            }
            unsafe {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                c = xmlCurrentChar(ctxt, &mut l)
            }
        }
    }
    if len > 50000 as i32
        && unsafe { (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32 }
    {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_NAME_TOO_LONG,
                b"Name\x00" as *const u8 as *const i8,
            );
        }
        return 0 as *const xmlChar;
    }
    if unsafe {
        ((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64)
            < len as i64
    } {
        /*
         * There were a couple of bugs where PERefs lead to to a change
         * of the buffer. Check the buffer size to avoid passing an invalid
         * pointer to xmlDictLookup.
         */
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"unexpected change of input buffer\x00" as *const u8 as *const i8,
            );
        }
        return 0 as *const xmlChar;
    }
    if unsafe {
        *(*(*ctxt).input).cur as i32 == '\n' as i32
            && *(*(*ctxt).input).cur.offset(-(1 as i32) as isize) as i32
                == '\r' as i32
    } {
        return unsafe {
            xmlDictLookup_safe(
                (*ctxt).dict,
                (*(*ctxt).input)
                    .cur
                    .offset(-((len + 1 as i32) as isize)),
                len,
            )
        };
    }
    return unsafe {
        xmlDictLookup_safe(
            (*ctxt).dict,
            (*(*ctxt).input).cur.offset(-(len as isize)),
            len,
        )
    };
}
/* *
* xmlParseName:
* @ctxt:  an XML parser context
*
* parse an XML name.
*
* [4] NameChar ::= Letter | Digit | '.' | '-' | '_' | ':' |
*                  CombiningChar | Extender
*
* [5] Name ::= (Letter | '_' | ':') (NameChar)*
*
* [6] Names ::= Name (#x20 Name)*
*
* Returns the Name parsed or NULL
*/

pub unsafe fn xmlParseName(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    let mut count: i32 = 0 as i32;
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }

    match () {
        #[cfg(HAVE_parser_DEBUG)]
        _ => {
            nbParseName = nbParseName + 1;
        }
        #[cfg(not(HAVE_parser_DEBUG))]
        _ => {}
    };

    /*
     * Accelerator for simple ASCII names
     */
    in_0 = unsafe { (*(*ctxt).input).cur };
    if unsafe {
        *in_0 as i32 >= 0x61 as i32 && *in_0 as i32 <= 0x7a as i32
            || *in_0 as i32 >= 0x41 as i32
                && *in_0 as i32 <= 0x5a as i32
            || *in_0 as i32 == '_' as i32
            || *in_0 as i32 == ':' as i32
    } {
        in_0 = unsafe { in_0.offset(1) };
        while unsafe {
            *in_0 as i32 >= 0x61 as i32
                && *in_0 as i32 <= 0x7a as i32
                || *in_0 as i32 >= 0x41 as i32
                    && *in_0 as i32 <= 0x5a as i32
                || *in_0 as i32 >= 0x30 as i32
                    && *in_0 as i32 <= 0x39 as i32
                || *in_0 as i32 == '_' as i32
                || *in_0 as i32 == '-' as i32
                || *in_0 as i32 == ':' as i32
                || *in_0 as i32 == '.' as i32
        } {
            in_0 = unsafe { in_0.offset(1) };
        }
        if unsafe {
            *in_0 as i32 > 0 as i32 && (*in_0 as i32) < 0x80 as i32
        } {
            count =
                unsafe { in_0.offset_from((*(*ctxt).input).cur) as i64 as i32 };
            if count > 50000 as i32
                && unsafe { (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32 }
            {
                unsafe {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_NAME_TOO_LONG,
                        b"Name\x00" as *const u8 as *const i8,
                    );
                }
                return 0 as *const xmlChar;
            }
            unsafe {
                ret = xmlDictLookup_safe((*ctxt).dict, (*(*ctxt).input).cur, count);
                (*(*ctxt).input).cur = in_0;
                (*(*ctxt).input).col += count;
            }
            if ret.is_null() {
                unsafe {
                    xmlErrMemory(ctxt, 0 as *const i8);
                }
            }
            return ret;
        }
    }
    /* accelerator for special cases */
    return xmlParseNameComplex(ctxt);
}
unsafe fn xmlParseNCNameComplex(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut len: i32 = 0 as i32;
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut startPosition: size_t = 0 as i32 as size_t;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    match () {
        #[cfg(HAVE_parser_DEBUG)]
        _ => {
            nbParseNCNameComplex = nbParseNCNameComplex + 1;
        }
        #[cfg(not(HAVE_parser_DEBUG))]
        _ => {}
    };

    /*
     * Handler for more complex cases
     */
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    startPosition = unsafe {
        (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64 as size_t
    };
    c = unsafe { xmlCurrentChar(ctxt, &mut l) };
    if c == ' ' as i32
        || c == '>' as i32
        || c == '/' as i32
        || (xmlIsNameStartChar(ctxt, c) == 0 || c == ':' as i32)
    {
        return 0 as *const xmlChar;
    }
    while c != ' ' as i32
        && c != '>' as i32
        && c != '/' as i32
        && (xmlIsNameChar(ctxt, c) != 0 && c != ':' as i32)
    {
        let fresh51 = count;
        count = count + 1;
        if fresh51 > 100 as i32 {
            if len > 50000 as i32
                && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
            {
                unsafe {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_NAME_TOO_LONG,
                        b"NCName\x00" as *const u8 as *const i8,
                    );
                }
                return 0 as *const xmlChar;
            }
            count = 0 as i32;
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                }
            {
                xmlGROW(ctxt);
            }
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                return 0 as *const xmlChar;
            }
        }
        len += l;
        if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
            unsafe {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as i32
            }
        } else {
            unsafe { (*(*ctxt).input).col += 1 }
        }
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            c = xmlCurrentChar(ctxt, &mut l);
        }
        if c == 0 as i32 {
            count = 0 as i32;
            /*
             * when shrinking to extend the buffer we really need to preserve
             * the part of the name we already parsed. Hence rolling back
             * by current length.
             */
            unsafe { (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(-(l as isize)) };
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                }
            {
                xmlGROW(ctxt);
            }
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                return 0 as *const xmlChar;
            }
            unsafe {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                c = xmlCurrentChar(ctxt, &mut l)
            }
        }
    }
    if len > 50000 as i32
        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
    {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_NAME_TOO_LONG,
                b"NCName\x00" as *const u8 as *const i8,
            );
        }
        return 0 as *const xmlChar;
    }
    return unsafe {
        xmlDictLookup_safe(
            (*ctxt).dict,
            (*(*ctxt).input).base.offset(startPosition as isize),
            len,
        )
    };
}
/* *
* xmlParseNCName:
* @ctxt:  an XML parser context
* @len:  length of the string parsed
*
* parse an XML name.
*
* [4NS] NCNameChar ::= Letter | Digit | '.' | '-' | '_' |
*                      CombiningChar | Extender
*
* [5NS] NCName ::= (Letter | '_') (NCNameChar)*
*
* Returns the Name parsed or NULL
*/
unsafe fn xmlParseNCName(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut e: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    let mut count: i32 = 0 as i32;

    match () {
        #[cfg(HAVE_parser_DEBUG)]
        _ => {
            nbParseNCName = nbParseNCName + 1;
        }
        #[cfg(not(HAVE_parser_DEBUG))]
        _ => {}
    };

    /*
     * Accelerator for simple ASCII names
     */
    unsafe {
        in_0 = (*(*ctxt).input).cur;
        e = (*(*ctxt).input).end;
    }
    if unsafe {
        (*in_0 as i32 >= 0x61 as i32 && *in_0 as i32 <= 0x7a as i32
            || *in_0 as i32 >= 0x41 as i32
                && *in_0 as i32 <= 0x5a as i32
            || *in_0 as i32 == '_' as i32)
            && in_0 < e
    } {
        unsafe { in_0 = in_0.offset(1) };
        while unsafe {
            (*in_0 as i32 >= 0x61 as i32
                && *in_0 as i32 <= 0x7a as i32
                || *in_0 as i32 >= 0x41 as i32
                    && *in_0 as i32 <= 0x5a as i32
                || *in_0 as i32 >= 0x30 as i32
                    && *in_0 as i32 <= 0x39 as i32
                || *in_0 as i32 == '_' as i32
                || *in_0 as i32 == '-' as i32
                || *in_0 as i32 == '.' as i32)
        } && in_0 < e
        {
            in_0 = unsafe { in_0.offset(1) };
        }
        if !(in_0 >= e) {
            if unsafe {
                *in_0 as i32 > 0 as i32
                    && (*in_0 as i32) < 0x80 as i32
            } {
                count = unsafe {
                    in_0.offset_from((*(*ctxt).input).cur) as i64 as i32
                };
                if count > 50000 as i32
                    && unsafe {
                        (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
                    }
                {
                    unsafe {
                        xmlFatalErr(
                            ctxt,
                            XML_ERR_NAME_TOO_LONG,
                            b"NCName\x00" as *const u8 as *const i8,
                        );
                    }
                    return 0 as *const xmlChar;
                }
                unsafe {
                    ret = xmlDictLookup_safe((*ctxt).dict, (*(*ctxt).input).cur, count);
                    (*(*ctxt).input).cur = in_0;
                    (*(*ctxt).input).col += count;
                }
                if ret.is_null() {
                    unsafe {
                        xmlErrMemory(ctxt, 0 as *const i8);
                    }
                }
                return ret;
            }
        }
    }
    return xmlParseNCNameComplex(ctxt);
}
/* *
* xmlParseNameAndCompare:
* @ctxt:  an XML parser context
*
* parse an XML name and compares for match
* (specialized for endtag parsing)
*
* Returns NULL for an illegal name, (xmlChar*) 1 for success
* and the name for mismatch
*/
unsafe fn xmlParseNameAndCompare(
    mut ctxt: xmlParserCtxtPtr,
    mut other: *const xmlChar,
) -> *const xmlChar {
    let mut cmp: *const xmlChar = other;
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return 0 as *const xmlChar;
    }
    in_0 = unsafe { (*(*ctxt).input).cur };
    while unsafe {
        *in_0 as i32 != 0 as i32 && *in_0 as i32 == *cmp as i32
    } {
        unsafe {
            in_0 = in_0.offset(1);
            cmp = cmp.offset(1);
        }
    }
    if unsafe {
        *cmp as i32 == 0 as i32
            && (*in_0 as i32 == '>' as i32
                || (*in_0 as i32 == 0x20 as i32
                    || 0x9 as i32 <= *in_0 as i32
                        && *in_0 as i32 <= 0xa as i32
                    || *in_0 as i32 == 0xd as i32))
    } {
        /* success */
        unsafe {
            (*(*ctxt).input).col = ((*(*ctxt).input).col as i64
                + in_0.offset_from((*(*ctxt).input).cur) as i64)
                as i32;
            (*(*ctxt).input).cur = in_0;
        }
        return 1 as i32 as *const xmlChar;
    }
    /* failure (or end of input buffer), check with full function */
    ret = xmlParseName(ctxt);
    /* strings coming from the dictionary direct compare possible */
    if ret == other {
        return 1 as i32 as *const xmlChar;
    }
    return ret;
}
/* *
* xmlParseStringName:
* @ctxt:  an XML parser context
* @str:  a pointer to the string pointer (IN/OUT)
*
* parse an XML name.
*
* [4] NameChar ::= Letter | Digit | '.' | '-' | '_' | ':' |
*                  CombiningChar | Extender
*
* [5] Name ::= (Letter | '_' | ':') (NameChar)*
*
* [6] Names ::= Name (#x20 Name)*
*
* Returns the Name parsed or NULL. The @str pointer
* is updated to the current location in the string.
*/
unsafe fn xmlParseStringName(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *mut *const xmlChar,
) -> *mut xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut cur: *const xmlChar = unsafe { *str };
    let mut len: i32 = 0 as i32;
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    match () {
        #[cfg(HAVE_parser_DEBUG)]
        _ => {
            nbParseStringName = nbParseStringName + 1;
        }
        #[cfg(not(HAVE_parser_DEBUG))]
        _ => {}
    };

    c = unsafe { xmlStringCurrentChar(ctxt, cur, &mut l) };
    if unsafe { xmlIsNameStartChar(ctxt, c) == 0 } {
        return 0 as *mut xmlChar;
    }
    if l == 1 as i32 {
        let fresh52 = len;
        len = len + 1;
        buf[fresh52 as usize] = c as xmlChar
    } else {
        len += unsafe { xmlCopyCharMultiByte(&mut *buf.as_mut_ptr().offset(len as isize), c) };
    }
    unsafe {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(ctxt, cur, &mut l);
    }
    while xmlIsNameChar(ctxt, c) != 0 {
        if l == 1 as i32 {
            let fresh53 = len;
            len = len + 1;
            buf[fresh53 as usize] = c as xmlChar
        } else {
            len += unsafe { xmlCopyCharMultiByte(&mut *buf.as_mut_ptr().offset(len as isize), c) };
        }
        unsafe {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(ctxt, cur, &mut l);
        }
        if len >= 100 as i32 {
            /* test bigentname.xml */
            /*
             * Okay someone managed to make a huge name, so he's ready to pay
             * for the processing speed.
             */
            let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
            let mut max: i32 = len * 2 as i32;
            buffer = xmlMallocAtomic_safe(
                (max as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
            ) as *mut xmlChar;
            if buffer.is_null() {
                unsafe {
                    xmlErrMemory(ctxt, 0 as *const i8);
                }
                return 0 as *mut xmlChar;
            }
            memcpy_safe(
                buffer as *mut (),
                buf.as_mut_ptr() as *const (),
                len as u64,
            );
            while xmlIsNameChar(ctxt, c) != 0 {
                if len + 10 as i32 > max {
                    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                    if len > 50000 as i32
                        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
                    {
                        unsafe {
                            xmlFatalErr(
                                ctxt,
                                XML_ERR_NAME_TOO_LONG,
                                b"NCName\x00" as *const u8 as *const i8,
                            );
                        }
                        xmlFree_safe(buffer as *mut ());
                        return 0 as *mut xmlChar;
                    }
                    max *= 2 as i32;
                    tmp = xmlRealloc_safe(
                        buffer as *mut (),
                        (max as u64)
                            .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                    ) as *mut xmlChar;
                    if tmp.is_null() {
                        unsafe {
                            xmlErrMemory(ctxt, 0 as *const i8);
                        }
                        xmlFree_safe(buffer as *mut ());
                        return 0 as *mut xmlChar;
                    }
                    buffer = tmp
                }
                if l == 1 as i32 {
                    let fresh54 = len;
                    len = len + 1;
                    unsafe { *buffer.offset(fresh54 as isize) = c as xmlChar }
                } else {
                    len += unsafe { xmlCopyCharMultiByte(&mut *buffer.offset(len as isize), c) }
                }
                unsafe {
                    cur = cur.offset(l as isize);
                    c = xmlStringCurrentChar(ctxt, cur, &mut l);
                }
            }
            unsafe {
                *buffer.offset(len as isize) = 0 as i32 as xmlChar;
                *str = cur;
            }
            return buffer;
        }
    }
    if len > 50000 as i32
        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
    {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_NAME_TOO_LONG,
                b"NCName\x00" as *const u8 as *const i8,
            );
        }
        return 0 as *mut xmlChar;
    }
    unsafe {
        *str = cur;
    }
    return xmlStrndup_safe(buf.as_mut_ptr(), len);
}
/* *
* xmlParseNmtoken:
* @ctxt:  an XML parser context
*
* parse an XML Nmtoken.
*
* [7] Nmtoken ::= (NameChar)+
*
* [8] Nmtokens ::= Nmtoken (#x20 Nmtoken)*
*
* Returns the Nmtoken parsed or NULL
*/

pub unsafe fn xmlParseNmtoken(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut len: i32 = 0 as i32;
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut safe_ctxt = unsafe { &mut *ctxt };

    match () {
        #[cfg(HAVE_parser_DEBUG)]
        _ => {
            nbParseNmToken = nbParseNmToken + 1;
        }
        #[cfg(not(HAVE_parser_DEBUG))]
        _ => {}
    };

    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return 0 as *mut xmlChar;
    }
    unsafe {
        c = xmlCurrentChar(ctxt, &mut l);
    }
    while xmlIsNameChar(ctxt, c) != 0 {
        let fresh55 = count;
        count = count + 1;
        if fresh55 > 100 as i32 {
            count = 0 as i32;
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                }
            {
                xmlGROW(ctxt);
            }
        }
        if l == 1 as i32 {
            let fresh56 = len;
            len = len + 1;
            buf[fresh56 as usize] = c as xmlChar
        } else {
            len += unsafe { xmlCopyCharMultiByte(&mut *buf.as_mut_ptr().offset(len as isize), c) };
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
            unsafe {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as i32;
            }
        } else {
            unsafe {
                (*(*ctxt).input).col += 1;
            }
        }
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            c = xmlCurrentChar(ctxt, &mut l);
        }
        if c == 0 as i32 {
            count = 0 as i32;
            if unsafe {
                (*ctxt).progressive == 0 as i32
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
            } {
                xmlGROW(ctxt);
            }
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                return 0 as *mut xmlChar;
            }
            c = unsafe { xmlCurrentChar(ctxt, &mut l) };
        }
        if len >= 100 as i32 {
            /*
             * Okay someone managed to make a huge token, so he's ready to pay
             * for the processing speed.
             */
            let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
            let mut max: i32 = len * 2 as i32;
            buffer = xmlMallocAtomic_safe(
                (max as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
            ) as *mut xmlChar;
            if buffer.is_null() {
                unsafe {
                    xmlErrMemory(ctxt, 0 as *const i8);
                }
                return 0 as *mut xmlChar;
            }
            memcpy_safe(
                buffer as *mut (),
                buf.as_mut_ptr() as *const (),
                len as u64,
            );
            while xmlIsNameChar(ctxt, c) != 0 {
                let fresh57 = count;
                count = count + 1;
                if fresh57 > 100 as i32 {
                    count = 0 as i32;
                    if unsafe {
                        (*ctxt).progressive == 0 as i32
                            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                as i64)
                                < 250 as i32 as i64
                    } {
                        xmlGROW(ctxt);
                    }
                    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                        xmlFree_safe(buffer as *mut ());
                        return 0 as *mut xmlChar;
                    }
                }
                if len + 10 as i32 > max {
                    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                    if max > 50000 as i32
                        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
                    {
                        unsafe {
                            xmlFatalErr(
                                ctxt,
                                XML_ERR_NAME_TOO_LONG,
                                b"NmToken\x00" as *const u8 as *const i8,
                            );
                        }
                        xmlFree_safe(buffer as *mut ());
                        return 0 as *mut xmlChar;
                    }
                    max *= 2 as i32;
                    tmp = xmlRealloc_safe(
                        buffer as *mut (),
                        (max as u64)
                            .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                    ) as *mut xmlChar;
                    if tmp.is_null() {
                        unsafe {
                            xmlErrMemory(ctxt, 0 as *const i8);
                        }
                        xmlFree_safe(buffer as *mut ());
                        return 0 as *mut xmlChar;
                    }
                    buffer = tmp
                }
                if l == 1 as i32 {
                    let fresh58 = len;
                    len = len + 1;
                    unsafe { *buffer.offset(fresh58 as isize) = c as xmlChar };
                } else {
                    len += unsafe { xmlCopyCharMultiByte(&mut *buffer.offset(len as isize), c) };
                }
                if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
                    unsafe {
                        (*(*ctxt).input).line += 1;
                        (*(*ctxt).input).col = 1 as i32;
                    }
                } else {
                    unsafe {
                        (*(*ctxt).input).col += 1;
                    }
                }
                unsafe {
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                    c = xmlCurrentChar(ctxt, &mut l)
                }
            }
            unsafe {
                *buffer.offset(len as isize) = 0 as i32 as xmlChar;
            }
            return buffer;
        }
    }
    if len == 0 as i32 {
        return 0 as *mut xmlChar;
    }
    if len > 50000 as i32
        && unsafe { (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32 }
    {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_NAME_TOO_LONG,
                b"NmToken\x00" as *const u8 as *const i8,
            );
        }
        return 0 as *mut xmlChar;
    }
    return xmlStrndup_safe(buf.as_mut_ptr(), len);
}
/* *
* xmlParseEntityValue:
* @ctxt:  an XML parser context
* @orig:  if non-NULL store a copy of the original entity value
*
* parse a value for ENTITY declarations
*
* [9] EntityValue ::= '"' ([^%&"] | PEReference | Reference)* '"' |
*	               "'" ([^%&'] | PEReference | Reference)* "'"
*
* Returns the EntityValue parsed with reference substituted or NULL
*/

pub unsafe fn xmlParseEntityValue(
    mut ctxt: xmlParserCtxtPtr,
    mut orig: *mut *mut xmlChar,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: i32 = 0 as i32;
    let mut size: i32 = 100 as i32;
    let mut c: i32 = 0;
    let mut l: i32 = 0;
    let mut stop: xmlChar = 0;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if unsafe { *(*(*ctxt).input).cur as i32 == '\"' as i32 } {
        stop = '\"' as i32 as xmlChar
    } else if unsafe { *(*(*ctxt).input).cur as i32 == '\'' as i32 } {
        stop = '\'' as i32 as xmlChar
    } else {
        unsafe {
            xmlFatalErr(ctxt, XML_ERR_ENTITY_NOT_STARTED, 0 as *const i8);
        }
        return 0 as *mut xmlChar;
    }
    buf = xmlMallocAtomic_safe(
        (size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
    ) as *mut xmlChar;
    if buf.is_null() {
        unsafe {
            xmlErrMemory(ctxt, 0 as *const i8);
        }
        return 0 as *mut xmlChar;
    }
    /*
     * The content of the entity definition is copied in a buffer.
     */
    (safe_ctxt).instate = XML_PARSER_ENTITY_VALUE;
    input = (safe_ctxt).input;
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    if !((safe_ctxt).instate as i32 == XML_PARSER_EOF as i32) {
        xmlNextChar_safe(ctxt);
        unsafe { c = xmlCurrentChar(ctxt, &mut l) };
        loop
        /*
         * NOTE: 4.4.5 Included in Literal
         * When a parameter entity reference appears in a literal entity
         * value, ... a single or double quote character in the replacement
         * text is always treated as a normal data character and will not
         * terminate the literal.
         * In practice it means we stop the loop only when back at parsing
         * the initial entity and the quote is found
         */
        {
            if !((if c < 0x100 as i32 {
                (0x9 as i32 <= c && c <= 0xa as i32
                    || c == 0xd as i32
                    || 0x20 as i32 <= c) as i32
            } else {
                (0x100 as i32 <= c && c <= 0xd7ff as i32
                    || 0xe000 as i32 <= c && c <= 0xfffd as i32
                    || 0x10000 as i32 <= c && c <= 0x10ffff as i32)
                    as i32
            }) != 0
                && (c != stop as i32 || (safe_ctxt).input != input)
                && (safe_ctxt).instate as i32 != XML_PARSER_EOF as i32)
            {
                current_block = 13460095289871124136;
                break;
            }
            if len + 5 as i32 >= size {
                let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                size *= 2 as i32;
                tmp = xmlRealloc_safe(
                    buf as *mut (),
                    (size as u64)
                        .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                ) as *mut xmlChar;
                if tmp.is_null() {
                    unsafe {
                        xmlErrMemory(ctxt, 0 as *const i8);
                    }
                    current_block = 1624980031832806685;
                    break;
                } else {
                    buf = tmp
                }
            }
            if l == 1 as i32 {
                let fresh59 = len;
                len = len + 1;
                unsafe { *buf.offset(fresh59 as isize) = c as xmlChar }
            } else {
                len += unsafe { xmlCopyCharMultiByte(&mut *buf.offset(len as isize), c) };
            }
            if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
                unsafe {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as i32
                }
            } else {
                unsafe { (*(*ctxt).input).col += 1 }
            }
            unsafe {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            }
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                }
            {
                xmlGROW(ctxt);
            }
            unsafe { c = xmlCurrentChar(ctxt, &mut l) };
            if c == 0 as i32 {
                if (safe_ctxt).progressive == 0 as i32
                    && unsafe {
                        ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                    }
                {
                    xmlGROW(ctxt);
                }
                unsafe { c = xmlCurrentChar(ctxt, &mut l) };
            }
        }
        match current_block {
            1624980031832806685 => {}
            _ => {
                unsafe {
                    *buf.offset(len as isize) = 0 as i32 as xmlChar;
                }
                if !((safe_ctxt).instate as i32 == XML_PARSER_EOF as i32) {
                    if c != stop as i32 {
                        unsafe {
                            xmlFatalErr(
                                ctxt,
                                XML_ERR_ENTITY_NOT_FINISHED,
                                0 as *const i8,
                            );
                        }
                    } else {
                        xmlNextChar_safe(ctxt);
                        /*
                         * Raise problem w.r.t. '&' and '%' being used in non-entities
                         * reference constructs. Note Charref will be handled in
                         * xmlStringDecodeEntities()
                         */
                        cur = buf;
                        loop {
                            if !(unsafe { *cur } as i32 != 0 as i32) {
                                current_block = 7158658067966855297;
                                break;
                            }
                            /* non input consuming */
                            if unsafe {
                                *cur as i32 == '%' as i32
                                    || *cur as i32 == '&' as i32
                                        && *cur.offset(1 as i32 as isize) as i32
                                            != '#' as i32
                            } {
                                let mut name: *mut xmlChar = 0 as *mut xmlChar;
                                let mut tmp_0: xmlChar = unsafe { *cur };
                                let mut nameOk: i32 = 0 as i32;
                                unsafe {
                                    cur = cur.offset(1);
                                }
                                name = xmlParseStringName(ctxt, &mut cur);
                                if !name.is_null() {
                                    nameOk = 1 as i32;
                                    xmlFree_safe(name as *mut ());
                                }
                                if nameOk == 0 as i32
                                    || unsafe { *cur as i32 != ';' as i32 }
                                {
                                    unsafe {
                                        xmlFatalErrMsgInt(ctxt,
                                      XML_ERR_ENTITY_CHAR_ERROR,
                                      b"EntityValue: \'%c\' forbidden except for entities references\n\x00"
                                          as *const u8 as
                                          *const i8,
                                      tmp_0 as i32);
                                    }
                                    current_block = 1624980031832806685;
                                    break;
                                } else if tmp_0 as i32 == '%' as i32
                                    && (safe_ctxt).inSubset == 1 as i32
                                    && (safe_ctxt).inputNr == 1 as i32
                                {
                                    unsafe {
                                        xmlFatalErr(
                                            ctxt,
                                            XML_ERR_ENTITY_PE_INTERNAL,
                                            0 as *const i8,
                                        );
                                    }
                                    current_block = 1624980031832806685;
                                    break;
                                } else if unsafe { *cur as i32 == 0 as i32 } {
                                    current_block = 7158658067966855297;
                                    break;
                                }
                            }
                            cur = unsafe { cur.offset(1) };
                        }
                        match current_block {
                            1624980031832806685 => {}
                            _ => {
                                /*
                                 * Then PEReference entities are substituted.
                                 *
                                 * NOTE: 4.4.7 Bypassed
                                 * When a general entity reference appears in the EntityValue in
                                 * an entity declaration, it is bypassed and left as is.
                                 * so XML_SUBSTITUTE_REF is not set here.
                                 */
                                (safe_ctxt).depth += 1;
                                ret = xmlStringDecodeEntities(
                                    ctxt,
                                    buf,
                                    2 as i32,
                                    0 as i32 as xmlChar,
                                    0 as i32 as xmlChar,
                                    0 as i32 as xmlChar,
                                );
                                (safe_ctxt).depth -= 1;
                                if !orig.is_null() {
                                    unsafe {
                                        *orig = buf;
                                    }
                                    buf = 0 as *mut xmlChar
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !buf.is_null() {
        xmlFree_safe(buf as *mut ());
    }
    return ret;
}
/* *
* xmlParseAttValueComplex:
* @ctxt:  an XML parser context
* @len:   the resulting attribute len
* @normalize:  whether to apply the inner normalization
*
* parse a value for an attribute, this is the fallback function
* of xmlParseAttValue() when the attribute parsing requires handling
* of non-ASCII characters, or normalization compaction.
*
* Returns the AttValue parsed or NULL. The value has to be freed by the caller.
*/
unsafe fn xmlParseAttValueComplex(
    mut ctxt: xmlParserCtxtPtr,
    mut attlen: *mut i32,
    mut normalize: i32,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut limit: xmlChar = 0 as i32 as xmlChar;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut rep: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: size_t = 0 as i32 as size_t;
    let mut buf_size: size_t = 0 as i32 as size_t;
    let mut c: i32 = 0;
    let mut l: i32 = 0;
    let mut in_space: i32 = 0 as i32;
    let mut current: *mut xmlChar = 0 as *mut xmlChar;
    let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if unsafe {
        *(*(*ctxt).input).cur.offset(0 as i32 as isize) as i32 == '\"' as i32
    } {
        (safe_ctxt).instate = XML_PARSER_ATTRIBUTE_VALUE;
        limit = '\"' as i32 as xmlChar;
        xmlNextChar_safe(ctxt);
    } else if unsafe {
        *(*(*ctxt).input).cur.offset(0 as i32 as isize) as i32 == '\'' as i32
    } {
        limit = '\'' as i32 as xmlChar;
        (safe_ctxt).instate = XML_PARSER_ATTRIBUTE_VALUE;
        xmlNextChar_safe(ctxt);
    } else {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_ATTRIBUTE_NOT_STARTED,
                0 as *const i8,
            );
        }
        return 0 as *mut xmlChar;
    }
    /*
     * allocate a translation buffer.
     */
    buf_size = 100 as i32 as size_t;
    buf = xmlMallocAtomic_safe(buf_size) as *mut xmlChar;
    if buf.is_null() {
        current_block = 10140382788883813888;
    } else {
        /*
         * OK loop until we reach one of the ending char or a size limit.
         */
        unsafe {
            c = xmlCurrentChar(ctxt, &mut l);
        }
        's_99: loop {
            if unsafe {
                !(*(*(*ctxt).input).cur.offset(0 as i32 as isize) as i32
                    != limit as i32
                    && (if c < 0x100 as i32 {
                        (0x9 as i32 <= c && c <= 0xa as i32
                            || c == 0xd as i32
                            || 0x20 as i32 <= c) as i32
                    } else {
                        (0x100 as i32 <= c && c <= 0xd7ff as i32
                            || 0xe000 as i32 <= c && c <= 0xfffd as i32
                            || 0x10000 as i32 <= c && c <= 0x10ffff as i32)
                            as i32
                    }) != 0
                    && c != '<' as i32
                    && (safe_ctxt).instate as i32 != XML_PARSER_EOF as i32)
            } {
                current_block = 3166194604430448652;
                break;
            }
            /*
             * Impose a reasonable limit on attribute size, unless XML_PARSE_HUGE
             * special option is given
             */
            if len > 10000000 as i32 as u64
                && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
            {
                unsafe {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ATTRIBUTE_NOT_FINISHED,
                        b"AttValue length too long\n\x00" as *const u8 as *const i8,
                    );
                }
                current_block = 10140382788883813888;
                break;
            } else {
                if c == '&' as i32 {
                    in_space = 0 as i32;
                    if unsafe {
                        *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                            == '#' as i32
                    } {
                        let mut val: i32 = xmlParseCharRef(ctxt);
                        if val == '&' as i32 {
                            if (safe_ctxt).replaceEntities != 0 {
                                if len.wrapping_add(10 as i32 as u64) > buf_size {
                                    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                                    let mut new_size: size_t = buf_size
                                        .wrapping_mul(2 as i32 as u64)
                                        .wrapping_add(10 as i32 as u64);
                                    if new_size < buf_size {
                                        current_block = 10140382788883813888;
                                        break;
                                    }
                                    tmp = xmlRealloc_safe(buf as *mut (), new_size)
                                        as *mut xmlChar;
                                    if tmp.is_null() {
                                        current_block = 10140382788883813888;
                                        break;
                                    }
                                    buf = tmp;
                                    buf_size = new_size
                                }
                                let fresh60 = len;
                                len = len.wrapping_add(1);
                                unsafe { *buf.offset(fresh60 as isize) = '&' as i32 as xmlChar };
                            } else {
                                /*
                                 * The reparsing will be done in xmlStringGetNodeList()
                                 * called by the attribute() function in SAX.c
                                 */
                                if len.wrapping_add(10 as i32 as u64) > buf_size {
                                    let mut tmp_0: *mut xmlChar = 0 as *mut xmlChar;
                                    let mut new_size_0: size_t = buf_size
                                        .wrapping_mul(2 as i32 as u64)
                                        .wrapping_add(10 as i32 as u64);
                                    if new_size_0 < buf_size {
                                        current_block = 10140382788883813888;
                                        break;
                                    }
                                    tmp_0 = xmlRealloc_safe(buf as *mut (), new_size_0)
                                        as *mut xmlChar;
                                    if tmp_0.is_null() {
                                        current_block = 10140382788883813888;
                                        break;
                                    }
                                    buf = tmp_0;
                                    buf_size = new_size_0
                                }
                                let fresh61 = len;
                                len = len.wrapping_add(1);
                                unsafe {
                                    *buf.offset(fresh61 as isize) = '&' as i32 as xmlChar;
                                    let fresh62 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh62 as isize) = '#' as i32 as xmlChar;
                                    let fresh63 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh63 as isize) = '3' as i32 as xmlChar;
                                    let fresh64 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh64 as isize) = '8' as i32 as xmlChar;
                                    let fresh65 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh65 as isize) = ';' as i32 as xmlChar;
                                }
                            }
                        } else if val != 0 as i32 {
                            if len.wrapping_add(10 as i32 as u64) > buf_size {
                                let mut tmp_1: *mut xmlChar = 0 as *mut xmlChar;
                                let mut new_size_1: size_t = buf_size
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(10 as i32 as u64);
                                if new_size_1 < buf_size {
                                    current_block = 10140382788883813888;
                                    break;
                                }
                                tmp_1 = xmlRealloc_safe(buf as *mut (), new_size_1)
                                    as *mut xmlChar;
                                if tmp_1.is_null() {
                                    current_block = 10140382788883813888;
                                    break;
                                }
                                buf = tmp_1;
                                buf_size = new_size_1
                            }
                            len = unsafe {
                                (len as u64).wrapping_add(xmlCopyChar(
                                    0 as i32,
                                    &mut *buf.offset(len as isize),
                                    val,
                                )
                                    as u64) as size_t
                                    as size_t
                            };
                        }
                    } else {
                        ent = unsafe { xmlParseEntityRef(ctxt) };
                        (safe_ctxt).nbentities = (safe_ctxt).nbentities.wrapping_add(1);
                        if !ent.is_null() {
                            unsafe {
                                (*ctxt).nbentities = (*ctxt)
                                    .nbentities
                                    .wrapping_add((*ent).owner as u64)
                            };
                        }
                        if !ent.is_null()
                            && unsafe {
                                (*ent).etype as u32
                                    == XML_INTERNAL_PREDEFINED_ENTITY as i32 as u32
                            }
                        {
                            if len.wrapping_add(10 as i32 as u64) > buf_size {
                                let mut tmp_2: *mut xmlChar = 0 as *mut xmlChar;
                                let mut new_size_2: size_t = buf_size
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(10 as i32 as u64);
                                if new_size_2 < buf_size {
                                    current_block = 10140382788883813888;
                                    break;
                                }
                                tmp_2 = xmlRealloc_safe(buf as *mut (), new_size_2)
                                    as *mut xmlChar;
                                if tmp_2.is_null() {
                                    current_block = 10140382788883813888;
                                    break;
                                }
                                buf = tmp_2;
                                buf_size = new_size_2
                            }
                            if unsafe {
                                (*ctxt).replaceEntities == 0 as i32
                                    && *(*ent).content.offset(0 as i32 as isize)
                                        as i32
                                        == '&' as i32
                            } {
                                let fresh66 = len;
                                len = len.wrapping_add(1);
                                unsafe {
                                    *buf.offset(fresh66 as isize) = '&' as i32 as xmlChar;
                                    let fresh67 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh67 as isize) = '#' as i32 as xmlChar;
                                    let fresh68 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh68 as isize) = '3' as i32 as xmlChar;
                                    let fresh69 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh69 as isize) = '8' as i32 as xmlChar;
                                    let fresh70 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh70 as isize) = ';' as i32 as xmlChar
                                }
                            } else {
                                let fresh71 = len;
                                len = len.wrapping_add(1);
                                unsafe {
                                    *buf.offset(fresh71 as isize) =
                                        *(*ent).content.offset(0 as i32 as isize)
                                }
                            }
                        } else if !ent.is_null() && (safe_ctxt).replaceEntities != 0 as i32
                        {
                            if unsafe {
                                (*ent).etype as u32
                                    != XML_INTERNAL_PREDEFINED_ENTITY as i32 as u32
                            } {
                                (safe_ctxt).depth += 1;
                                rep = unsafe {
                                    xmlStringDecodeEntities(
                                        ctxt,
                                        (*ent).content,
                                        1 as i32,
                                        0 as i32 as xmlChar,
                                        0 as i32 as xmlChar,
                                        0 as i32 as xmlChar,
                                    )
                                };
                                (safe_ctxt).depth -= 1;
                                if !rep.is_null() {
                                    current = rep;
                                    while unsafe { *current } as i32 != 0 as i32 {
                                        /* non input consuming */
                                        if unsafe { *current } as i32 == 0xd as i32
                                            || unsafe { *current } as i32
                                                == 0xa as i32
                                            || unsafe { *current } as i32
                                                == 0x9 as i32
                                        {
                                            let fresh72 = len;
                                            len = len.wrapping_add(1);
                                            unsafe {
                                                *buf.offset(fresh72 as isize) =
                                                    0x20 as i32 as xmlChar;
                                                current = current.offset(1);
                                            }
                                        } else {
                                            let fresh73 = current;
                                            unsafe {
                                                current = current.offset(1);
                                                let fresh74 = len;
                                                len = len.wrapping_add(1);
                                                *buf.offset(fresh74 as isize) = *fresh73
                                            }
                                        }
                                        if !(len.wrapping_add(10 as i32 as u64)
                                            > buf_size)
                                        {
                                            continue;
                                        }
                                        let mut tmp_3: *mut xmlChar = 0 as *mut xmlChar;
                                        let mut new_size_3: size_t = buf_size
                                            .wrapping_mul(2 as i32 as u64)
                                            .wrapping_add(10 as i32 as u64);
                                        if new_size_3 < buf_size {
                                            current_block = 10140382788883813888;
                                            break 's_99;
                                        }
                                        tmp_3 =
                                            xmlRealloc_safe(buf as *mut (), new_size_3)
                                                as *mut xmlChar;
                                        if tmp_3.is_null() {
                                            current_block = 10140382788883813888;
                                            break 's_99;
                                        }
                                        buf = tmp_3;
                                        buf_size = new_size_3
                                    }
                                    xmlFree_safe(rep as *mut ());
                                    rep = 0 as *mut xmlChar
                                }
                            } else {
                                if len.wrapping_add(10 as i32 as u64) > buf_size {
                                    let mut tmp_4: *mut xmlChar = 0 as *mut xmlChar;
                                    let mut new_size_4: size_t = buf_size
                                        .wrapping_mul(2 as i32 as u64)
                                        .wrapping_add(10 as i32 as u64);
                                    if new_size_4 < buf_size {
                                        current_block = 10140382788883813888;
                                        break;
                                    }
                                    tmp_4 = xmlRealloc_safe(buf as *mut (), new_size_4)
                                        as *mut xmlChar;
                                    if tmp_4.is_null() {
                                        current_block = 10140382788883813888;
                                        break;
                                    }
                                    buf = tmp_4;
                                    buf_size = new_size_4
                                }
                                if unsafe { !(*ent).content.is_null() } {
                                    let fresh75 = len;
                                    len = len.wrapping_add(1);
                                    unsafe {
                                        *buf.offset(fresh75 as isize) =
                                            *(*ent).content.offset(0 as i32 as isize)
                                    };
                                }
                            }
                        } else if !ent.is_null() {
                            let mut safe_ent = unsafe { &mut *ent };
                            let mut i: i32 = xmlStrlen_safe((safe_ent).name);
                            let mut cur: *const xmlChar = (safe_ent).name;
                            /*
                             * This may look absurd but is needed to detect
                             * entities problems
                             */
                            if (safe_ent).etype as u32
                                != XML_INTERNAL_PREDEFINED_ENTITY as i32 as u32
                                && !(safe_ent).content.is_null()
                                && (safe_ent).checked == 0 as i32
                            {
                                let mut oldnbent: u64 = (safe_ctxt).nbentities;
                                let mut diff: u64 = 0;
                                (safe_ctxt).depth += 1;
                                rep = unsafe {
                                    xmlStringDecodeEntities(
                                        ctxt,
                                        (safe_ent).content,
                                        1 as i32,
                                        0 as i32 as xmlChar,
                                        0 as i32 as xmlChar,
                                        0 as i32 as xmlChar,
                                    )
                                };
                                (safe_ctxt).depth -= 1;
                                diff = (safe_ctxt)
                                    .nbentities
                                    .wrapping_sub(oldnbent)
                                    .wrapping_add(1 as i32 as u64);
                                if diff
                                    > (2147483647 as i32 / 2 as i32)
                                        as u64
                                {
                                    diff = (2147483647 as i32 / 2 as i32)
                                        as u64
                                }
                                (safe_ent).checked = diff
                                    .wrapping_mul(2 as i32 as u64)
                                    as i32;
                                if !rep.is_null() {
                                    if !xmlStrchr_safe(rep, '<' as i32 as xmlChar).is_null() {
                                        (safe_ent).checked |= 1 as i32
                                    }
                                    xmlFree_safe(rep as *mut ());
                                    rep = 0 as *mut xmlChar
                                } else {
                                    unsafe {
                                        *(*ent).content.offset(0 as i32 as isize) =
                                            0 as i32 as xmlChar
                                    };
                                }
                            }
                            /*
                             * Just output the reference
                             */
                            let fresh76 = len;
                            len = len.wrapping_add(1);
                            unsafe {
                                *buf.offset(fresh76 as isize) = '&' as i32 as xmlChar;
                            }
                            while len
                                .wrapping_add(i as u64)
                                .wrapping_add(10 as i32 as u64)
                                > buf_size
                            {
                                let mut tmp_5: *mut xmlChar = 0 as *mut xmlChar;
                                let mut new_size_5: size_t = buf_size
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(i as u64)
                                    .wrapping_add(10 as i32 as u64);
                                if new_size_5 < buf_size {
                                    current_block = 10140382788883813888;
                                    break 's_99;
                                }
                                tmp_5 = xmlRealloc_safe(buf as *mut (), new_size_5)
                                    as *mut xmlChar;
                                if tmp_5.is_null() {
                                    current_block = 10140382788883813888;
                                    break 's_99;
                                }
                                buf = tmp_5;
                                buf_size = new_size_5
                            }
                            while i > 0 as i32 {
                                let fresh77 = cur;
                                unsafe {
                                    cur = cur.offset(1);
                                    let fresh78 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh78 as isize) = *fresh77;
                                }
                                i -= 1
                            }
                            let fresh79 = len;
                            len = len.wrapping_add(1);
                            unsafe { *buf.offset(fresh79 as isize) = ';' as i32 as xmlChar };
                        }
                    }
                } else {
                    if c == 0x20 as i32
                        || c == 0xd as i32
                        || c == 0xa as i32
                        || c == 0x9 as i32
                    {
                        if len != 0 as i32 as u64 || normalize == 0 {
                            if normalize == 0 || in_space == 0 {
                                if l == 1 as i32 {
                                    let fresh80 = len;
                                    len = len.wrapping_add(1);
                                    unsafe {
                                        *buf.offset(fresh80 as isize) =
                                            0x20 as i32 as xmlChar
                                    };
                                } else {
                                    len = unsafe {
                                        (len as u64).wrapping_add(xmlCopyCharMultiByte(
                                            &mut *buf.offset(len as isize),
                                            0x20 as i32,
                                        )
                                            as u64)
                                            as size_t
                                            as size_t
                                    };
                                }
                                while len.wrapping_add(10 as i32 as u64)
                                    > buf_size
                                {
                                    let mut tmp_6: *mut xmlChar = 0 as *mut xmlChar;
                                    let mut new_size_6: size_t = buf_size
                                        .wrapping_mul(2 as i32 as u64)
                                        .wrapping_add(10 as i32 as u64);
                                    if new_size_6 < buf_size {
                                        current_block = 10140382788883813888;
                                        break 's_99;
                                    }
                                    tmp_6 = xmlRealloc_safe(buf as *mut (), new_size_6)
                                        as *mut xmlChar;
                                    if tmp_6.is_null() {
                                        current_block = 10140382788883813888;
                                        break 's_99;
                                    }
                                    buf = tmp_6;
                                    buf_size = new_size_6
                                }
                            }
                            in_space = 1 as i32
                        }
                    } else {
                        in_space = 0 as i32;
                        if l == 1 as i32 {
                            let fresh81 = len;
                            len = len.wrapping_add(1);
                            unsafe { *buf.offset(fresh81 as isize) = c as xmlChar };
                        } else {
                            len = unsafe {
                                (len as u64).wrapping_add(xmlCopyCharMultiByte(
                                    &mut *buf.offset(len as isize),
                                    c,
                                )
                                    as u64) as size_t
                                    as size_t
                            };
                        }
                        if len.wrapping_add(10 as i32 as u64) > buf_size {
                            let mut tmp_7: *mut xmlChar = 0 as *mut xmlChar;
                            let mut new_size_7: size_t = buf_size
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(10 as i32 as u64);
                            if new_size_7 < buf_size {
                                current_block = 10140382788883813888;
                                break;
                            }
                            tmp_7 = xmlRealloc_safe(buf as *mut (), new_size_7)
                                as *mut xmlChar;
                            if tmp_7.is_null() {
                                current_block = 10140382788883813888;
                                break;
                            }
                            buf = tmp_7;
                            buf_size = new_size_7
                        }
                    }
                    if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
                        unsafe {
                            (*(*ctxt).input).line += 1;
                            (*(*ctxt).input).col = 1 as i32
                        }
                    } else {
                        unsafe { (*(*ctxt).input).col += 1 }
                    }
                    unsafe { (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize) };
                }
                if unsafe {
                    (*ctxt).progressive == 0 as i32
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                } {
                    xmlGROW(ctxt);
                }
                unsafe { c = xmlCurrentChar(ctxt, &mut l) };
            }
        }
        match current_block {
            10140382788883813888 => {}
            _ => {
                if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                    current_block = 12690591973131353181;
                } else {
                    if in_space != 0 && normalize != 0 {
                        while len > 0 as i32 as u64
                            && unsafe {
                                *buf.offset(
                                    len.wrapping_sub(1 as i32 as u64) as isize
                                ) as i32
                                    == 0x20 as i32
                            }
                        {
                            len = len.wrapping_sub(1)
                        }
                    }
                    unsafe {
                        *buf.offset(len as isize) = 0 as i32 as xmlChar;
                    }
                    if unsafe { *(*(*ctxt).input).cur as i32 == '<' as i32 } {
                        unsafe {
                            xmlFatalErr(ctxt, XML_ERR_LT_IN_ATTRIBUTE, 0 as *const i8);
                        }
                    } else if unsafe {
                        *(*(*ctxt).input).cur as i32 != limit as i32
                    } {
                        if c != 0 as i32
                            && (if c < 0x100 as i32 {
                                (0x9 as i32 <= c && c <= 0xa as i32
                                    || c == 0xd as i32
                                    || 0x20 as i32 <= c)
                                    as i32
                            } else {
                                (0x100 as i32 <= c && c <= 0xd7ff as i32
                                    || 0xe000 as i32 <= c && c <= 0xfffd as i32
                                    || 0x10000 as i32 <= c && c <= 0x10ffff as i32)
                                    as i32
                            }) == 0
                        {
                            unsafe {
                                xmlFatalErrMsg(
                                    ctxt,
                                    XML_ERR_INVALID_CHAR,
                                    b"invalid character in attribute value\n\x00" as *const u8
                                        as *const i8,
                                );
                            }
                        } else {
                            unsafe {
                                xmlFatalErrMsg(
                                    ctxt,
                                    XML_ERR_ATTRIBUTE_NOT_FINISHED,
                                    b"AttValue: \' expected\n\x00" as *const u8
                                        as *const i8,
                                );
                            }
                        }
                    } else {
                        xmlNextChar_safe(ctxt);
                    }
                    /*
                     * There we potentially risk an overflow, don't allow attribute value of
                     * length more than INT_MAX it is a very reasonable assumption !
                     */
                    if len >= 2147483647 as i32 as u64 {
                        unsafe {
                            xmlFatalErrMsg(
                                ctxt,
                                XML_ERR_ATTRIBUTE_NOT_FINISHED,
                                b"AttValue length too long\n\x00" as *const u8
                                    as *const i8,
                            );
                        }
                    } else {
                        if !attlen.is_null() {
                            unsafe { *attlen = len as i32 };
                        }
                        return buf;
                    }
                    current_block = 10140382788883813888;
                }
            }
        }
    }
    match current_block {
        10140382788883813888 => unsafe {
            xmlErrMemory(ctxt, 0 as *const i8);
        },
        _ => {}
    }
    if !buf.is_null() {
        xmlFree_safe(buf as *mut ());
    }
    if !rep.is_null() {
        xmlFree_safe(rep as *mut ());
    }
    return 0 as *mut xmlChar;
}
/* *
* xmlParseAttValue:
* @ctxt:  an XML parser context
*
* parse a value for an attribute
* Note: the parser won't do substitution of entities here, this
* will be handled later in xmlStringGetNodeList
*
* [10] AttValue ::= '"' ([^<&"] | Reference)* '"' |
*                   "'" ([^<&'] | Reference)* "'"
*
* 3.3.3 Attribute-Value Normalization:
* Before the value of an attribute is passed to the application or
* checked for validity, the XML processor must normalize it as follows:
* - a character reference is processed by appending the referenced
*   character to the attribute value
* - an entity reference is processed by recursively processing the
*   replacement text of the entity
* - a whitespace character (#x20, #xD, #xA, #x9) is processed by
*   appending #x20 to the normalized value, except that only a single
*   #x20 is appended for a "#xD#xA" sequence that is part of an external
*   parsed entity or the literal entity value of an internal parsed entity
* - other characters are processed by appending them to the normalized value
* If the declared value is not CDATA, then the XML processor must further
* process the normalized attribute value by discarding any leading and
* trailing space (#x20) characters, and by replacing sequences of space
* (#x20) characters by a single space (#x20) character.
* All attributes for which no declaration has been read should be treated
* by a non-validating parser as if declared CDATA.
*
* Returns the AttValue parsed or NULL. The value has to be freed by the caller.
*/

pub unsafe fn xmlParseAttValue(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    if ctxt.is_null() || unsafe { (*ctxt).input.is_null() } {
        return 0 as *mut xmlChar;
    }
    return unsafe {
        xmlParseAttValueInternal(
            ctxt,
            0 as *mut i32,
            0 as *mut i32,
            0 as i32,
        )
    };
}
/* *
* xmlParseSystemLiteral:
* @ctxt:  an XML parser context
*
* parse an XML Literal
*
* [11] SystemLiteral ::= ('"' [^"]* '"') | ("'" [^']* "'")
*
* Returns the SystemLiteral parsed or NULL
*/

pub unsafe fn xmlParseSystemLiteral(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: i32 = 0 as i32;
    let mut size: i32 = 100 as i32;
    let mut cur: i32 = 0;
    let mut l: i32 = 0;
    let mut stop: xmlChar = 0;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut state: i32 = (safe_ctxt).instate as i32;
    let mut count: i32 = 0 as i32;
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < (2 as i32 * 250 as i32) as i64
        }
    {
        xmlSHRINK(ctxt);
    }
    if unsafe { *(*(*ctxt).input).cur as i32 == '\"' as i32 } {
        xmlNextChar_safe(ctxt);
        stop = '\"' as i32 as xmlChar
    } else if unsafe { *(*(*ctxt).input).cur as i32 == '\'' as i32 } {
        xmlNextChar_safe(ctxt);
        stop = '\'' as i32 as xmlChar
    } else {
        unsafe {
            xmlFatalErr(ctxt, XML_ERR_LITERAL_NOT_STARTED, 0 as *const i8);
        }
        return 0 as *mut xmlChar;
    }
    buf = xmlMallocAtomic_safe(
        (size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
    ) as *mut xmlChar;
    if buf.is_null() {
        unsafe {
            xmlErrMemory(ctxt, 0 as *const i8);
        }
        return 0 as *mut xmlChar;
    }
    (safe_ctxt).instate = XML_PARSER_SYSTEM_LITERAL;
    cur = unsafe { xmlCurrentChar(ctxt, &mut l) };
    while (if cur < 0x100 as i32 {
        (0x9 as i32 <= cur && cur <= 0xa as i32
            || cur == 0xd as i32
            || 0x20 as i32 <= cur) as i32
    } else {
        (0x100 as i32 <= cur && cur <= 0xd7ff as i32
            || 0xe000 as i32 <= cur && cur <= 0xfffd as i32
            || 0x10000 as i32 <= cur && cur <= 0x10ffff as i32)
            as i32
    }) != 0
        && cur != stop as i32
    {
        /* checked */
        if len + 5 as i32 >= size {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            if size > 50000 as i32
                && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
            {
                unsafe {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_NAME_TOO_LONG,
                        b"SystemLiteral\x00" as *const u8 as *const i8,
                    );
                }
                xmlFree_safe(buf as *mut ());
                (safe_ctxt).instate = state as xmlParserInputState;
                return 0 as *mut xmlChar;
            }
            size *= 2 as i32;
            tmp = xmlRealloc_safe(
                buf as *mut (),
                (size as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
            ) as *mut xmlChar;
            if tmp.is_null() {
                xmlFree_safe(buf as *mut ());
                unsafe {
                    xmlErrMemory(ctxt, 0 as *const i8);
                }
                (safe_ctxt).instate = state as xmlParserInputState;
                return 0 as *mut xmlChar;
            }
            buf = tmp
        }
        count += 1;
        if count > 50 as i32 {
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                        > (2 as i32 * 250 as i32) as i64
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < (2 as i32 * 250 as i32) as i64
                }
            {
                xmlSHRINK(ctxt);
            }
            if unsafe {
                (*ctxt).progressive == 0 as i32
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
            } {
                xmlGROW(ctxt);
            }
            count = 0 as i32;
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                xmlFree_safe(buf as *mut ());
                return 0 as *mut xmlChar;
            }
        }
        if l == 1 as i32 {
            let fresh82 = len;
            len = len + 1;
            unsafe { *buf.offset(fresh82 as isize) = cur as xmlChar }
        } else {
            len += unsafe { xmlCopyCharMultiByte(&mut *buf.offset(len as isize), cur) };
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
            unsafe {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as i32;
            }
        } else {
            unsafe {
                (*(*ctxt).input).col += 1;
            }
        }
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            cur = xmlCurrentChar(ctxt, &mut l);
        }
        if cur == 0 as i32 {
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                }
            {
                xmlGROW(ctxt);
            }
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                        > (2 as i32 * 250 as i32) as i64
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < (2 as i32 * 250 as i32) as i64
                }
            {
                xmlSHRINK(ctxt);
            }
            cur = unsafe { xmlCurrentChar(ctxt, &mut l) };
        }
    }
    unsafe {
        *buf.offset(len as isize) = 0 as i32 as xmlChar;
        (*ctxt).instate = state as xmlParserInputState;
    }
    if if cur < 0x100 as i32 {
        (0x9 as i32 <= cur && cur <= 0xa as i32
            || cur == 0xd as i32
            || 0x20 as i32 <= cur) as i32
    } else {
        (0x100 as i32 <= cur && cur <= 0xd7ff as i32
            || 0xe000 as i32 <= cur && cur <= 0xfffd as i32
            || 0x10000 as i32 <= cur && cur <= 0x10ffff as i32)
            as i32
    } == 0
    {
        unsafe {
            xmlFatalErr(ctxt, XML_ERR_LITERAL_NOT_FINISHED, 0 as *const i8);
        }
    } else {
        xmlNextChar_safe(ctxt);
    }
    return buf;
}
/* *
* xmlParsePubidLiteral:
* @ctxt:  an XML parser context
*
* parse an XML public literal
*
* [12] PubidLiteral ::= '"' PubidChar* '"' | "'" (PubidChar - "'")* "'"
*
* Returns the PubidLiteral parsed or NULL.
*/

pub unsafe fn xmlParsePubidLiteral(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: i32 = 0 as i32;
    let mut size: i32 = 100 as i32;
    let mut cur: xmlChar = 0;
    let mut stop: xmlChar = 0;
    let mut count: i32 = 0 as i32;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut oldstate: xmlParserInputState = (safe_ctxt).instate;
    if unsafe {
        (safe_ctxt).progressive == 0 as i32
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < (2 as i32 * 250 as i32) as i64
    } {
        xmlSHRINK(ctxt);
    }
    if unsafe { *(*(*ctxt).input).cur as i32 == '\"' as i32 } {
        xmlNextChar_safe(ctxt);
        stop = '\"' as i32 as xmlChar
    } else if unsafe { *(*(*ctxt).input).cur as i32 == '\'' as i32 } {
        xmlNextChar_safe(ctxt);
        stop = '\'' as i32 as xmlChar
    } else {
        unsafe {
            xmlFatalErr(ctxt, XML_ERR_LITERAL_NOT_STARTED, 0 as *const i8);
        }
        return 0 as *mut xmlChar;
    }
    buf = xmlMallocAtomic_safe(
        (size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
    ) as *mut xmlChar;
    if buf.is_null() {
        unsafe {
            xmlErrMemory(ctxt, 0 as *const i8);
        }
        return 0 as *mut xmlChar;
    }
    (safe_ctxt).instate = XML_PARSER_PUBLIC_LITERAL;
    cur = unsafe { *(*(*ctxt).input).cur };
    while unsafe { xmlIsPubidChar_tab[cur as usize] as i32 != 0 }
        && cur as i32 != stop as i32
    {
        /* checked */
        if len + 1 as i32 >= size {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            if size > 50000 as i32
                && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
            {
                unsafe {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_NAME_TOO_LONG,
                        b"Public ID\x00" as *const u8 as *const i8,
                    );
                }
                xmlFree_safe(buf as *mut ());
                return 0 as *mut xmlChar;
            }
            size *= 2 as i32;
            tmp = xmlRealloc_safe(
                buf as *mut (),
                (size as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
            ) as *mut xmlChar;
            if tmp.is_null() {
                unsafe {
                    xmlErrMemory(ctxt, 0 as *const i8);
                }
                xmlFree_safe(buf as *mut ());
                return 0 as *mut xmlChar;
            }
            buf = tmp
        }
        let fresh83 = len;
        len = len + 1;
        unsafe {
            *buf.offset(fresh83 as isize) = cur;
        }
        count += 1;
        if count > 50 as i32 {
            if unsafe {
                (*ctxt).progressive == 0 as i32
                    && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                        > (2 as i32 * 250 as i32) as i64
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < (2 as i32 * 250 as i32) as i64
            } {
                xmlSHRINK(ctxt);
            }
            if unsafe {
                (*ctxt).progressive == 0 as i32
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
            } {
                xmlGROW(ctxt);
            }
            count = 0 as i32;
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                xmlFree_safe(buf as *mut ());
                return 0 as *mut xmlChar;
            }
        }
        xmlNextChar_safe(ctxt);
        cur = unsafe { *(*(*ctxt).input).cur };
        if cur as i32 == 0 as i32 {
            if unsafe {
                (*ctxt).progressive == 0 as i32
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
            } {
                xmlGROW(ctxt);
            }
            if unsafe {
                (*ctxt).progressive == 0 as i32
                    && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                        > (2 as i32 * 250 as i32) as i64
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < (2 as i32 * 250 as i32) as i64
            } {
                xmlSHRINK(ctxt);
            }
            cur = unsafe { *(*(*ctxt).input).cur };
        }
    }
    unsafe {
        *buf.offset(len as isize) = 0 as i32 as xmlChar;
    }
    if cur as i32 != stop as i32 {
        unsafe {
            xmlFatalErr(ctxt, XML_ERR_LITERAL_NOT_FINISHED, 0 as *const i8);
        }
    } else {
        xmlNextChar_safe(ctxt);
    }
    (safe_ctxt).instate = oldstate;
    return buf;
}
/*
* used for the test in the inner loop of the char data testing
*/
static mut test_char_data: [u8; 256] = [
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x9 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0x20 as i32 as u8,
    0x21 as i32 as u8,
    0x22 as i32 as u8,
    0x23 as i32 as u8,
    0x24 as i32 as u8,
    0x25 as i32 as u8,
    0 as i32 as u8,
    0x27 as i32 as u8,
    0x28 as i32 as u8,
    0x29 as i32 as u8,
    0x2a as i32 as u8,
    0x2b as i32 as u8,
    0x2c as i32 as u8,
    0x2d as i32 as u8,
    0x2e as i32 as u8,
    0x2f as i32 as u8,
    0x30 as i32 as u8,
    0x31 as i32 as u8,
    0x32 as i32 as u8,
    0x33 as i32 as u8,
    0x34 as i32 as u8,
    0x35 as i32 as u8,
    0x36 as i32 as u8,
    0x37 as i32 as u8,
    0x38 as i32 as u8,
    0x39 as i32 as u8,
    0x3a as i32 as u8,
    0x3b as i32 as u8,
    0 as i32 as u8,
    0x3d as i32 as u8,
    0x3e as i32 as u8,
    0x3f as i32 as u8,
    0x40 as i32 as u8,
    0x41 as i32 as u8,
    0x42 as i32 as u8,
    0x43 as i32 as u8,
    0x44 as i32 as u8,
    0x45 as i32 as u8,
    0x46 as i32 as u8,
    0x47 as i32 as u8,
    0x48 as i32 as u8,
    0x49 as i32 as u8,
    0x4a as i32 as u8,
    0x4b as i32 as u8,
    0x4c as i32 as u8,
    0x4d as i32 as u8,
    0x4e as i32 as u8,
    0x4f as i32 as u8,
    0x50 as i32 as u8,
    0x51 as i32 as u8,
    0x52 as i32 as u8,
    0x53 as i32 as u8,
    0x54 as i32 as u8,
    0x55 as i32 as u8,
    0x56 as i32 as u8,
    0x57 as i32 as u8,
    0x58 as i32 as u8,
    0x59 as i32 as u8,
    0x5a as i32 as u8,
    0x5b as i32 as u8,
    0x5c as i32 as u8,
    0 as i32 as u8,
    0x5e as i32 as u8,
    0x5f as i32 as u8,
    0x60 as i32 as u8,
    0x61 as i32 as u8,
    0x62 as i32 as u8,
    0x63 as i32 as u8,
    0x64 as i32 as u8,
    0x65 as i32 as u8,
    0x66 as i32 as u8,
    0x67 as i32 as u8,
    0x68 as i32 as u8,
    0x69 as i32 as u8,
    0x6a as i32 as u8,
    0x6b as i32 as u8,
    0x6c as i32 as u8,
    0x6d as i32 as u8,
    0x6e as i32 as u8,
    0x6f as i32 as u8,
    0x70 as i32 as u8,
    0x71 as i32 as u8,
    0x72 as i32 as u8,
    0x73 as i32 as u8,
    0x74 as i32 as u8,
    0x75 as i32 as u8,
    0x76 as i32 as u8,
    0x77 as i32 as u8,
    0x78 as i32 as u8,
    0x79 as i32 as u8,
    0x7a as i32 as u8,
    0x7b as i32 as u8,
    0x7c as i32 as u8,
    0x7d as i32 as u8,
    0x7e as i32 as u8,
    0x7f as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
];
/* *
* xmlParseCharData:
* @ctxt:  an XML parser context
* @cdata:  int indicating whether we are within a CDATA section
*
* parse a CharData section.
* if we are within a CDATA section ']]>' marks an end of section.
*
* The right angle bracket (>) may be represented using the string "&gt;",
* and must, for compatibility, be escaped using "&gt;" or a character
* reference when it appears in the string "]]>" in content, when that
* string is not marking the end of a CDATA section.
*
* [14] CharData ::= [^<&]* - ([^<&]* ']]>' [^<&]*)
*/

pub unsafe fn xmlParseCharData(mut ctxt: xmlParserCtxtPtr, mut cdata: i32) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut current_block: u64;
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut nbchar: i32 = 0 as i32;
    let mut line: i32 = unsafe { (*(*ctxt).input).line };
    let mut col: i32 = unsafe { (*(*ctxt).input).col };
    let mut ccol: i32 = 0;
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < (2 as i32 * 250 as i32) as i64
    } {
        xmlSHRINK(ctxt);
    }
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    /*
     * Accelerated common case where input don't need to be
     * modified before passing it to the handler.
     */
    if cdata == 0 {
        in_0 = unsafe { (*(*ctxt).input).cur };
        loop {
            while unsafe { *in_0 as i32 == 0x20 as i32 } {
                unsafe {
                    in_0 = in_0.offset(1);
                    (*(*ctxt).input).col += 1;
                }
            }
            if unsafe { *in_0 as i32 == 0xa as i32 } {
                loop {
                    unsafe {
                        (*(*ctxt).input).line += 1;
                        (*(*ctxt).input).col = 1 as i32;
                        in_0 = in_0.offset(1);
                    }
                    if unsafe { !(*in_0 as i32 == 0xa as i32) } {
                        break;
                    }
                }
            } else {
                if unsafe { *in_0 as i32 == '<' as i32 } {
                    nbchar = unsafe {
                        in_0.offset_from((*(*ctxt).input).cur) as i64 as i32
                    };
                    if nbchar > 0 as i32 {
                        let mut tmp: *const xmlChar = unsafe { (*(*ctxt).input).cur };
                        unsafe { (*(*ctxt).input).cur = in_0 };
                        if unsafe {
                            !(*ctxt).sax.is_null()
                                && (*(*ctxt).sax).ignorableWhitespace != (*(*ctxt).sax).characters
                        } {
                            if areBlanks(ctxt, tmp, nbchar, 1 as i32) != 0 {
                                if unsafe { (*(*ctxt).sax).ignorableWhitespace.is_some() } {
                                    unsafe {
                                        (*(*ctxt).sax)
                                            .ignorableWhitespace
                                            .expect("non-null function pointer")(
                                            (*ctxt).userData,
                                            tmp,
                                            nbchar,
                                        );
                                    }
                                }
                            } else {
                                if unsafe { (*(*ctxt).sax).characters.is_some() } {
                                    unsafe {
                                        (*(*ctxt).sax)
                                            .characters
                                            .expect("non-null function pointer")(
                                            (*ctxt).userData,
                                            tmp,
                                            nbchar,
                                        );
                                    }
                                }
                                if unsafe { *(*ctxt).space } == -(1 as i32) {
                                    unsafe { *(*ctxt).space = -(2 as i32) }
                                }
                            }
                        } else if !(safe_ctxt).sax.is_null()
                            && unsafe { (*(*ctxt).sax).characters.is_some() }
                        {
                            unsafe {
                                (*(*ctxt).sax)
                                    .characters
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    tmp,
                                    nbchar,
                                );
                            }
                        }
                    }
                    return;
                }
                loop {
                    unsafe {
                        ccol = (*(*ctxt).input).col;
                        while test_char_data[*in_0 as usize] != 0 {
                            in_0 = in_0.offset(1);
                            ccol += 1
                        }
                        (*(*ctxt).input).col = ccol;
                        if *in_0 as i32 == 0xa as i32 {
                            loop {
                                (*(*ctxt).input).line += 1;
                                (*(*ctxt).input).col = 1 as i32;
                                in_0 = in_0.offset(1);
                                if !(*in_0 as i32 == 0xa as i32) {
                                    break;
                                }
                            }
                        } else {
                            if !(*in_0 as i32 == ']' as i32) {
                                break;
                            }
                            if *in_0.offset(1 as i32 as isize) as i32 == ']' as i32
                                && *in_0.offset(2 as i32 as isize) as i32
                                    == '>' as i32
                            {
                                xmlFatalErr(
                                    ctxt,
                                    XML_ERR_MISPLACED_CDATA_END,
                                    0 as *const i8,
                                );
                                (*(*ctxt).input).cur = in_0.offset(1 as i32 as isize);
                                return;
                            }
                            in_0 = in_0.offset(1);
                            (*(*ctxt).input).col += 1
                        }
                    }
                }
                nbchar = unsafe {
                    in_0.offset_from((*(*ctxt).input).cur) as i64 as i32
                };
                if nbchar > 0 as i32 {
                    if unsafe {
                        !(*ctxt).sax.is_null()
                            && (*(*ctxt).sax).ignorableWhitespace != (*(*ctxt).sax).characters
                            && (*(*(*ctxt).input).cur as i32 == 0x20 as i32
                                || 0x9 as i32 <= *(*(*ctxt).input).cur as i32
                                    && *(*(*ctxt).input).cur as i32 <= 0xa as i32
                                || *(*(*ctxt).input).cur as i32 == 0xd as i32)
                    } {
                        let mut tmp_0: *const xmlChar = unsafe { (*(*ctxt).input).cur };
                        unsafe { (*(*ctxt).input).cur = in_0 };
                        if areBlanks(ctxt, tmp_0, nbchar, 0 as i32) != 0 {
                            if unsafe { (*(*ctxt).sax).ignorableWhitespace.is_some() } {
                                unsafe {
                                    (*(*ctxt).sax)
                                        .ignorableWhitespace
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        tmp_0,
                                        nbchar,
                                    );
                                }
                            }
                        } else {
                            unsafe {
                                if (*(*ctxt).sax).characters.is_some() {
                                    (*(*ctxt).sax)
                                        .characters
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        tmp_0,
                                        nbchar,
                                    );
                                }
                                if *(*ctxt).space == -(1 as i32) {
                                    *(*ctxt).space = -(2 as i32)
                                }
                            }
                        }
                        unsafe {
                            line = (*(*ctxt).input).line;
                            col = (*(*ctxt).input).col
                        }
                    } else if !(safe_ctxt).sax.is_null() {
                        unsafe {
                            if (*(*ctxt).sax).characters.is_some() {
                                (*(*ctxt).sax)
                                    .characters
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    (*(*ctxt).input).cur,
                                    nbchar,
                                );
                            }
                            line = (*(*ctxt).input).line;
                            col = (*(*ctxt).input).col
                        }
                    }
                    /* something really bad happened in the SAX callback */
                    if (safe_ctxt).instate as i32 != XML_PARSER_CONTENT as i32 {
                        return;
                    }
                }
                unsafe {
                    (*(*ctxt).input).cur = in_0;
                    if *in_0 as i32 == 0xd as i32 {
                        in_0 = in_0.offset(1);
                        if *in_0 as i32 == 0xa as i32 {
                            (*(*ctxt).input).cur = in_0;
                            in_0 = in_0.offset(1);
                            (*(*ctxt).input).line += 1;
                            (*(*ctxt).input).col = 1 as i32;
                            current_block = 1917311967535052937;
                            /* while */
                        } else {
                            in_0 = in_0.offset(-1);
                            current_block = 17769492591016358583;
                        }
                    } else {
                        current_block = 17769492591016358583;
                    }
                }

                match current_block {
                    17769492591016358583 => {
                        if unsafe { *in_0 } as i32 == '<' as i32 {
                            return;
                        }
                        if unsafe { *in_0 } as i32 == '&' as i32 {
                            return;
                        }
                        if unsafe {
                            (*ctxt).progressive == 0 as i32
                                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                    as i64
                                    > (2 as i32 * 250 as i32) as i64
                                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                    as i64)
                                    < (2 as i32 * 250 as i32) as i64
                        } {
                            xmlSHRINK(ctxt);
                        }
                        if unsafe {
                            (*ctxt).progressive == 0 as i32
                                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                    as i64)
                                    < 250 as i32 as i64
                        } {
                            xmlGROW(ctxt);
                        }
                        if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                            return;
                        }
                        in_0 = unsafe { (*(*ctxt).input).cur };
                    }
                    _ => {}
                }
                if unsafe {
                    !(*in_0 as i32 >= 0x20 as i32
                        && *in_0 as i32 <= 0x7f as i32
                        || *in_0 as i32 == 0x9 as i32
                        || *in_0 as i32 == 0xa as i32)
                } {
                    break;
                }
            }
        }
        nbchar = 0 as i32
    }
    unsafe {
        (*(*ctxt).input).line = line;
        (*(*ctxt).input).col = col;
    }
    xmlParseCharDataComplex(ctxt, cdata);
}
/* *
* xmlParseCharDataComplex:
* @ctxt:  an XML parser context
* @cdata:  int indicating whether we are within a CDATA section
*
* parse a CharData section.this is the fallback function
* of xmlParseCharData() when the parsing requires handling
* of non-ASCII characters.
*/
unsafe fn xmlParseCharDataComplex(mut ctxt: xmlParserCtxtPtr, mut cdata: i32) {
    let mut buf: [xmlChar; 305] = [0; 305];
    let mut nbchar: i32 = 0 as i32;
    let mut cur: i32 = 0;
    let mut l: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < (2 as i32 * 250 as i32) as i64
    } {
        xmlSHRINK(ctxt);
    }
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    cur = unsafe { xmlCurrentChar(ctxt, &mut l) };
    while cur != '<' as i32
        && cur != '&' as i32
        && (if cur < 0x100 as i32 {
            (0x9 as i32 <= cur && cur <= 0xa as i32
                || cur == 0xd as i32
                || 0x20 as i32 <= cur) as i32
        } else {
            (0x100 as i32 <= cur && cur <= 0xd7ff as i32
                || 0xe000 as i32 <= cur && cur <= 0xfffd as i32
                || 0x10000 as i32 <= cur && cur <= 0x10ffff as i32)
                as i32
        }) != 0
    {
        /* test also done in xmlCurrentChar() */
        if cur == ']' as i32
            && unsafe {
                *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == ']' as i32
                    && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                        == '>' as i32
            }
        {
            if cdata != 0 {
                break;
            }
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_MISPLACED_CDATA_END, 0 as *const i8);
            }
        }
        if l == 1 as i32 {
            let fresh84 = nbchar;
            nbchar = nbchar + 1;
            buf[fresh84 as usize] = cur as xmlChar
        } else {
            nbchar += unsafe {
                xmlCopyCharMultiByte(&mut *buf.as_mut_ptr().offset(nbchar as isize), cur)
            };
        }
        if nbchar >= 300 as i32 {
            buf[nbchar as usize] = 0 as i32 as xmlChar;
            /*
             * OK the segment is to be consumed as chars.
             */
            if !(safe_ctxt).sax.is_null() && (safe_ctxt).disableSAX == 0 {
                if areBlanks(ctxt, buf.as_mut_ptr(), nbchar, 0 as i32) != 0 {
                    unsafe {
                        if (*(*ctxt).sax).ignorableWhitespace.is_some() {
                            (*(*ctxt).sax)
                                .ignorableWhitespace
                                .expect("non-null function pointer")(
                                (*ctxt).userData,
                                buf.as_mut_ptr(),
                                nbchar,
                            );
                        }
                    }
                } else {
                    unsafe {
                        if (*(*ctxt).sax).characters.is_some() {
                            (*(*ctxt).sax)
                                .characters
                                .expect("non-null function pointer")(
                                (*ctxt).userData,
                                buf.as_mut_ptr(),
                                nbchar,
                            );
                        }
                        if (*(*ctxt).sax).characters != (*(*ctxt).sax).ignorableWhitespace
                            && *(*ctxt).space == -(1 as i32)
                        {
                            *(*ctxt).space = -(2 as i32)
                        }
                    }
                }
            }
            nbchar = 0 as i32;
            /* something really bad happened in the SAX callback */
            if (safe_ctxt).instate as i32 != XML_PARSER_CONTENT as i32 {
                return;
            }
        }
        count += 1;
        if count > 50 as i32 {
            if unsafe {
                (*ctxt).progressive == 0 as i32
                    && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                        > (2 as i32 * 250 as i32) as i64
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < (2 as i32 * 250 as i32) as i64
            } {
                xmlSHRINK(ctxt);
            }
            if unsafe {
                (*ctxt).progressive == 0 as i32
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
            } {
                xmlGROW(ctxt);
            }
            count = 0 as i32;
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                return;
            }
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == '\n' as i32 } {
            unsafe {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as i32
            }
        } else {
            unsafe { (*(*ctxt).input).col += 1 }
        }
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            cur = xmlCurrentChar(ctxt, &mut l);
        }
    }
    if nbchar != 0 as i32 {
        buf[nbchar as usize] = 0 as i32 as xmlChar;
        /*
         * OK the segment is to be consumed as chars.
         */
        if !(safe_ctxt).sax.is_null() && (safe_ctxt).disableSAX == 0 {
            if areBlanks(ctxt, buf.as_mut_ptr(), nbchar, 0 as i32) != 0 {
                unsafe {
                    if (*(*ctxt).sax).ignorableWhitespace.is_some() {
                        (*(*ctxt).sax)
                            .ignorableWhitespace
                            .expect("non-null function pointer")(
                            (*ctxt).userData,
                            buf.as_mut_ptr(),
                            nbchar,
                        );
                    }
                }
            } else {
                unsafe {
                    if (*(*ctxt).sax).characters.is_some() {
                        (*(*ctxt).sax)
                            .characters
                            .expect("non-null function pointer")(
                            (*ctxt).userData,
                            buf.as_mut_ptr(),
                            nbchar,
                        );
                    }
                    if (*(*ctxt).sax).characters != (*(*ctxt).sax).ignorableWhitespace
                        && *(*ctxt).space == -(1 as i32)
                    {
                        *(*ctxt).space = -(2 as i32)
                    }
                }
            }
        }
    }
    if cur != 0 as i32
        && (if cur < 0x100 as i32 {
            (0x9 as i32 <= cur && cur <= 0xa as i32
                || cur == 0xd as i32
                || 0x20 as i32 <= cur) as i32
        } else {
            (0x100 as i32 <= cur && cur <= 0xd7ff as i32
                || 0xe000 as i32 <= cur && cur <= 0xfffd as i32
                || 0x10000 as i32 <= cur && cur <= 0x10ffff as i32)
                as i32
        }) == 0
    {
        /* Generate the error and skip the offending character */
        unsafe {
            xmlFatalErrMsgInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"PCDATA invalid Char value %d\n\x00" as *const u8 as *const i8,
                cur,
            );
        }
        unsafe {
            if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as i32
            } else {
                (*(*ctxt).input).col += 1
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize)
        };
    };
}
/* *
* xmlParseExternalID:
* @ctxt:  an XML parser context
* @publicID:  a xmlChar** receiving PubidLiteral
* @strict: indicate whether we should restrict parsing to only
*          production [75], see NOTE below
*
* Parse an External ID or a Public ID
*
* NOTE: Productions [75] and [83] interact badly since [75] can generate
*       'PUBLIC' S PubidLiteral S SystemLiteral
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

pub unsafe fn xmlParseExternalID(
    mut ctxt: xmlParserCtxtPtr,
    mut publicID: *mut *mut xmlChar,
    mut strict: i32,
) -> *mut xmlChar {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < (2 as i32 * 250 as i32) as i64
    } {
        xmlSHRINK(ctxt);
    }
    unsafe { *publicID = 0 as *mut xmlChar };
    if unsafe {
        *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == 'S' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'Y' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'S' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'E' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'M' as i32
    } {
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(6 as i32 as isize);
            (*(*ctxt).input).col += 6 as i32;
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
            xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
        }
        if xmlSkipBlankChars(ctxt) == 0 as i32 {
            unsafe {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after \'SYSTEM\'\n\x00" as *const u8 as *const i8,
                );
            }
        }
        URI = xmlParseSystemLiteral(ctxt);
        if URI.is_null() {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_URI_REQUIRED, 0 as *const i8);
            }
        }
    } else if unsafe {
        *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == 'P' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'U' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'B' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'L' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'I' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'C' as i32
    } {
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(6 as i32 as isize);
            (*(*ctxt).input).col += 6 as i32;
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
            xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
        }
        if xmlSkipBlankChars(ctxt) == 0 as i32 {
            unsafe {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after \'PUBLIC\'\n\x00" as *const u8 as *const i8,
                );
            }
        }
        unsafe {
            *publicID = xmlParsePubidLiteral(ctxt);
            if (*publicID).is_null() {
                xmlFatalErr(ctxt, XML_ERR_PUBID_REQUIRED, 0 as *const i8);
            }
        }
        if strict != 0 {
            /*
             * We don't handle [83] so "S SystemLiteral" is required.
             */
            if xmlSkipBlankChars(ctxt) == 0 as i32 {
                unsafe {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_SPACE_REQUIRED,
                        b"Space required after the Public Identifier\n\x00" as *const u8
                            as *const i8,
                    );
                }
            }
        } else {
            /*
             * We handle [83] so we return immediately, if
             * "S SystemLiteral" is not detected. We skip blanks if no
             * system literal was found, but this is harmless since we must
             * be at the end of a NotationDecl.
             */
            if xmlSkipBlankChars(ctxt) == 0 as i32 {
                return 0 as *mut xmlChar;
            }
            if unsafe {
                *(*(*ctxt).input).cur as i32 != '\'' as i32
                    && *(*(*ctxt).input).cur as i32 != '\"' as i32
            } {
                return 0 as *mut xmlChar;
            }
        }
        URI = xmlParseSystemLiteral(ctxt);
        if URI.is_null() {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_URI_REQUIRED, 0 as *const i8);
            }
        }
    }
    return URI;
}
/* *
* xmlParseCommentComplex:
* @ctxt:  an XML parser context
* @buf:  the already parsed part of the buffer
* @len:  number of bytes in the buffer
* @size:  allocated size of the buffer
*
* Skip an XML (SGML) comment <!-- .... -->
*  The spec says that "For compatibility, the string "--" (double-hyphen)
*  must not occur within comments. "
* This is the slow routine in case the accelerator for ascii didn't work
*
* [15] Comment ::= '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
*/
unsafe fn xmlParseCommentComplex(
    mut ctxt: xmlParserCtxtPtr,
    mut buf: *mut xmlChar,
    mut len: size_t,
    mut size: size_t,
) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut q: i32 = 0;
    let mut ql: i32 = 0;
    let mut r: i32 = 0;
    let mut rl: i32 = 0;
    let mut cur: i32 = 0;
    let mut l: i32 = 0;
    let mut count: size_t = 0 as i32 as size_t;
    let mut inputid: i32 = 0;
    inputid = unsafe { (*(*ctxt).input).id };
    if buf.is_null() {
        len = 0 as i32 as size_t;
        size = 100 as i32 as size_t;
        buf = xmlMallocAtomic_safe(
            size.wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
        ) as *mut xmlChar;
        if buf.is_null() {
            unsafe {
                xmlErrMemory(ctxt, 0 as *const i8);
            }
            return;
        }
    }
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    /* Assure there's enough input data */
    unsafe {
        q = xmlCurrentChar(ctxt, &mut ql);
    }
    if !(q == 0 as i32) {
        if if q < 0x100 as i32 {
            (0x9 as i32 <= q && q <= 0xa as i32
                || q == 0xd as i32
                || 0x20 as i32 <= q) as i32
        } else {
            (0x100 as i32 <= q && q <= 0xd7ff as i32
                || 0xe000 as i32 <= q && q <= 0xfffd as i32
                || 0x10000 as i32 <= q && q <= 0x10ffff as i32)
                as i32
        } == 0
        {
            unsafe {
                xmlFatalErrMsgInt(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"xmlParseComment: invalid xmlChar value %d\n\x00" as *const u8
                        as *const i8,
                    q,
                );
            }
            xmlFree_safe(buf as *mut ());
            return;
        }
        unsafe {
            if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as i32
            } else {
                (*(*ctxt).input).col += 1
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(ql as isize);
            r = xmlCurrentChar(ctxt, &mut rl);
        }
        if !(r == 0 as i32) {
            if if r < 0x100 as i32 {
                (0x9 as i32 <= r && r <= 0xa as i32
                    || r == 0xd as i32
                    || 0x20 as i32 <= r) as i32
            } else {
                (0x100 as i32 <= r && r <= 0xd7ff as i32
                    || 0xe000 as i32 <= r && r <= 0xfffd as i32
                    || 0x10000 as i32 <= r && r <= 0x10ffff as i32)
                    as i32
            } == 0
            {
                unsafe {
                    xmlFatalErrMsgInt(
                        ctxt,
                        XML_ERR_INVALID_CHAR,
                        b"xmlParseComment: invalid xmlChar value %d\n\x00" as *const u8
                            as *const i8,
                        q,
                    );
                }
                xmlFree_safe(buf as *mut ());
                return;
            }
            unsafe {
                if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as i32
                } else {
                    (*(*ctxt).input).col += 1
                }
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(rl as isize);
                cur = xmlCurrentChar(ctxt, &mut l);
            }
            if !(cur == 0 as i32) {
                while (if cur < 0x100 as i32 {
                    (0x9 as i32 <= cur && cur <= 0xa as i32
                        || cur == 0xd as i32
                        || 0x20 as i32 <= cur) as i32
                } else {
                    (0x100 as i32 <= cur && cur <= 0xd7ff as i32
                        || 0xe000 as i32 <= cur && cur <= 0xfffd as i32
                        || 0x10000 as i32 <= cur && cur <= 0x10ffff as i32)
                        as i32
                }) != 0
                    && (cur != '>' as i32 || r != '-' as i32 || q != '-' as i32)
                {
                    if r == '-' as i32 && q == '-' as i32 {
                        unsafe {
                            xmlFatalErr(ctxt, XML_ERR_HYPHEN_IN_COMMENT, 0 as *const i8);
                        }
                    }
                    if len > 10000000 as i32 as u64
                        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
                    {
                        unsafe {
                            xmlFatalErrMsgStr(
                                ctxt,
                                XML_ERR_COMMENT_NOT_FINISHED,
                                b"Comment too big found\x00" as *const u8 as *const i8,
                                0 as *const xmlChar,
                            );
                        }
                        xmlFree_safe(buf as *mut ());
                        return;
                    }
                    if len.wrapping_add(5 as i32 as u64) >= size {
                        let mut new_buf: *mut xmlChar = 0 as *mut xmlChar;
                        let mut new_size: size_t = 0;
                        new_size = size.wrapping_mul(2 as i32 as u64);
                        new_buf =
                            xmlRealloc_safe(buf as *mut (), new_size) as *mut xmlChar;
                        if new_buf.is_null() {
                            xmlFree_safe(buf as *mut ());
                            unsafe {
                                xmlErrMemory(ctxt, 0 as *const i8);
                            }
                            return;
                        }
                        buf = new_buf;
                        size = new_size
                    }
                    if ql == 1 as i32 {
                        let fresh85 = len;
                        len = len.wrapping_add(1);
                        unsafe { *buf.offset(fresh85 as isize) = q as xmlChar }
                    } else {
                        len = unsafe {
                            (len as u64).wrapping_add(xmlCopyCharMultiByte(
                                &mut *buf.offset(len as isize),
                                q,
                            )
                                as u64) as size_t as size_t
                        };
                    }
                    q = r;
                    ql = rl;
                    r = cur;
                    rl = l;
                    count = count.wrapping_add(1);
                    if count > 50 as i32 as u64 {
                        if unsafe {
                            (*ctxt).progressive == 0 as i32
                                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                    as i64
                                    > (2 as i32 * 250 as i32) as i64
                                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                    as i64)
                                    < (2 as i32 * 250 as i32) as i64
                        } {
                            xmlSHRINK(ctxt);
                        }
                        if unsafe {
                            (*ctxt).progressive == 0 as i32
                                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                    as i64)
                                    < 250 as i32 as i64
                        } {
                            xmlGROW(ctxt);
                        }
                        count = 0 as i32 as size_t;
                        if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                            xmlFree_safe(buf as *mut ());
                            return;
                        }
                    }
                    unsafe {
                        if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
                            (*(*ctxt).input).line += 1;
                            (*(*ctxt).input).col = 1 as i32
                        } else {
                            (*(*ctxt).input).col += 1
                        }
                        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                        cur = xmlCurrentChar(ctxt, &mut l);
                    }
                    if cur == 0 as i32 {
                        if unsafe {
                            (*ctxt).progressive == 0 as i32
                                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                    as i64
                                    > (2 as i32 * 250 as i32) as i64
                                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                    as i64)
                                    < (2 as i32 * 250 as i32) as i64
                        } {
                            xmlSHRINK(ctxt);
                        }
                        if unsafe {
                            (*ctxt).progressive == 0 as i32
                                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                    as i64)
                                    < 250 as i32 as i64
                        } {
                            xmlGROW(ctxt);
                        }
                        cur = unsafe { xmlCurrentChar(ctxt, &mut l) };
                    }
                }
                unsafe {
                    *buf.offset(len as isize) = 0 as i32 as xmlChar;
                }
                if cur == 0 as i32 {
                    unsafe {
                        xmlFatalErrMsgStr(
                            ctxt,
                            XML_ERR_COMMENT_NOT_FINISHED,
                            b"Comment not terminated \n<!--%.50s\n\x00" as *const u8
                                as *const i8,
                            buf,
                        );
                    }
                } else if if cur < 0x100 as i32 {
                    (0x9 as i32 <= cur && cur <= 0xa as i32
                        || cur == 0xd as i32
                        || 0x20 as i32 <= cur) as i32
                } else {
                    (0x100 as i32 <= cur && cur <= 0xd7ff as i32
                        || 0xe000 as i32 <= cur && cur <= 0xfffd as i32
                        || 0x10000 as i32 <= cur && cur <= 0x10ffff as i32)
                        as i32
                } == 0
                {
                    unsafe {
                        xmlFatalErrMsgInt(
                            ctxt,
                            XML_ERR_INVALID_CHAR,
                            b"xmlParseComment: invalid xmlChar value %d\n\x00" as *const u8
                                as *const i8,
                            cur,
                        );
                    }
                } else {
                    unsafe {
                        if inputid != (*(*ctxt).input).id {
                            xmlFatalErrMsg(
                                ctxt,
                                XML_ERR_ENTITY_BOUNDARY,
                                b"Comment doesn\'t start and stop in the same entity\n\x00"
                                    as *const u8
                                    as *const i8,
                            );
                        }
                    }
                    xmlNextChar_safe(ctxt);
                    unsafe {
                        if !(*ctxt).sax.is_null()
                            && (*(*ctxt).sax).comment.is_some()
                            && (*ctxt).disableSAX == 0
                        {
                            (*(*ctxt).sax).comment.expect("non-null function pointer")(
                                (*ctxt).userData,
                                buf,
                            );
                        }
                    }
                }
                xmlFree_safe(buf as *mut ());
                return;
            }
        }
    }
    unsafe {
        xmlFatalErrMsgStr(
            ctxt,
            XML_ERR_COMMENT_NOT_FINISHED,
            b"Comment not terminated\n\x00" as *const u8 as *const i8,
            0 as *const xmlChar,
        );
    }
    xmlFree_safe(buf as *mut ());
}
/* *
* xmlParseComment:
* @ctxt:  an XML parser context
*
* Skip an XML (SGML) comment <!-- .... -->
*  The spec says that "For compatibility, the string "--" (double-hyphen)
*  must not occur within comments. "
*
* [15] Comment ::= '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
*/

pub unsafe fn xmlParseComment(mut ctxt: xmlParserCtxtPtr) {
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut size: size_t = 100 as i32 as size_t;
    let mut len: size_t = 0 as i32 as size_t;
    let mut state: xmlParserInputState = XML_PARSER_START;
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut nbchar: size_t = 0 as i32 as size_t;
    let mut ccol: i32 = 0;
    let mut inputid: i32 = 0;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    /*
     * Check that there is a comment right here.
     */
    if unsafe {
        *(*(*ctxt).input).cur as i32 != '<' as i32
            || *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 != '!' as i32
            || *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32 != '-' as i32
            || *(*(*ctxt).input).cur.offset(3 as i32 as isize) as i32 != '-' as i32
    } {
        return;
    }
    state = (safe_ctxt).instate;
    (safe_ctxt).instate = XML_PARSER_COMMENT;
    unsafe {
        inputid = (*(*ctxt).input).id;
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(4 as i32 as isize);
        (*(*ctxt).input).col += 4 as i32;
    }
    if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
        xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
    }
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < (2 as i32 * 250 as i32) as i64
    } {
        xmlSHRINK(ctxt);
    }
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    /*
     * Accelerated common case where input don't need to be
     * modified before passing it to the handler.
     */
    in_0 = unsafe { (*(*ctxt).input).cur };
    loop {
        //@todo unsafe范围缩小
        unsafe {
            if *in_0 as i32 == 0xa as i32 {
                loop {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as i32;
                    in_0 = in_0.offset(1);
                    if !(*in_0 as i32 == 0xa as i32) {
                        break;
                    }
                }
            }
            loop {
                ccol = (*(*ctxt).input).col;
                while *in_0 as i32 > '-' as i32
                    && *in_0 as i32 <= 0x7f as i32
                    || *in_0 as i32 >= 0x20 as i32
                        && (*in_0 as i32) < '-' as i32
                    || *in_0 as i32 == 0x9 as i32
                {
                    in_0 = in_0.offset(1);
                    ccol += 1
                }
                (*(*ctxt).input).col = ccol;
                if *in_0 as i32 == 0xa as i32 {
                    loop {
                        (*(*ctxt).input).line += 1;
                        (*(*ctxt).input).col = 1 as i32;
                        in_0 = in_0.offset(1);
                        if !(*in_0 as i32 == 0xa as i32) {
                            break;
                        }
                    }
                } else {
                    nbchar = in_0.offset_from((*(*ctxt).input).cur) as i64 as size_t;
                    /*
                     * save current set of data
                     */
                    if nbchar > 0 as i32 as u64 {
                        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).comment.is_some() {
                            if buf.is_null() {
                                if *in_0 as i32 == '-' as i32
                                    && *in_0.offset(1 as i32 as isize) as i32
                                        == '-' as i32
                                {
                                    size = nbchar.wrapping_add(1 as i32 as u64)
                                } else {
                                    size =
                                        (100 as i32 as u64).wrapping_add(nbchar)
                                }
                                buf =
                                    xmlMallocAtomic_safe(size.wrapping_mul(::std::mem::size_of::<
                                        xmlChar,
                                    >(
                                    )
                                        as u64))
                                        as *mut xmlChar;
                                if buf.is_null() {
                                    xmlErrMemory(ctxt, 0 as *const i8);
                                    (*ctxt).instate = state;
                                    return;
                                }
                                len = 0 as i32 as size_t
                            } else if len
                                .wrapping_add(nbchar)
                                .wrapping_add(1 as i32 as u64)
                                >= size
                            {
                                let mut new_buf: *mut xmlChar = 0 as *mut xmlChar;
                                size = (size as u64).wrapping_add(
                                    len.wrapping_add(nbchar)
                                        .wrapping_add(100 as i32 as u64),
                                ) as size_t as size_t;
                                new_buf = xmlRealloc_safe(
                                    buf as *mut (),
                                    size.wrapping_mul(
                                        ::std::mem::size_of::<xmlChar>() as u64
                                    ),
                                ) as *mut xmlChar;
                                if new_buf.is_null() {
                                    xmlFree_safe(buf as *mut ());
                                    xmlErrMemory(ctxt, 0 as *const i8);
                                    (*ctxt).instate = state;
                                    return;
                                }
                                buf = new_buf
                            }
                            memcpy_safe(
                                &mut *buf.offset(len as isize) as *mut xmlChar as *mut (),
                                (*(*ctxt).input).cur as *const (),
                                nbchar,
                            );
                            len = (len as u64).wrapping_add(nbchar) as size_t as size_t;
                            *buf.offset(len as isize) = 0 as i32 as xmlChar
                        }
                    }
                    if len > 10000000 as i32 as u64
                        && (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
                    {
                        xmlFatalErrMsgStr(
                            ctxt,
                            XML_ERR_COMMENT_NOT_FINISHED,
                            b"Comment too big found\x00" as *const u8 as *const i8,
                            0 as *const xmlChar,
                        );
                        xmlFree_safe(buf as *mut ());
                        return;
                    }
                    (*(*ctxt).input).cur = in_0;
                    if *in_0 as i32 == 0xa as i32 {
                        in_0 = in_0.offset(1);
                        (*(*ctxt).input).line += 1;
                        (*(*ctxt).input).col = 1 as i32
                    }
                    if *in_0 as i32 == 0xd as i32 {
                        in_0 = in_0.offset(1);
                        if *in_0 as i32 == 0xa as i32 {
                            (*(*ctxt).input).cur = in_0;
                            in_0 = in_0.offset(1);
                            (*(*ctxt).input).line += 1;
                            (*(*ctxt).input).col = 1 as i32;
                            break;
                            /* while */
                        } else {
                            in_0 = in_0.offset(-1)
                        }
                    }
                    if (*ctxt).progressive == 0 as i32
                        && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                            > (2 as i32 * 250 as i32) as i64
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < (2 as i32 * 250 as i32) as i64
                    {
                        xmlSHRINK(ctxt);
                    }
                    if (*ctxt).progressive == 0 as i32
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                    {
                        xmlGROW(ctxt);
                    }
                    if (*ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                        xmlFree_safe(buf as *mut ());
                        return;
                    }
                    in_0 = (*(*ctxt).input).cur;
                    if !(*in_0 as i32 == '-' as i32) {
                        break;
                    }
                    if *in_0.offset(1 as i32 as isize) as i32 == '-' as i32 {
                        if *in_0.offset(2 as i32 as isize) as i32 == '>' as i32 {
                            if (*(*ctxt).input).id != inputid {
                                xmlFatalErrMsg(
                                    ctxt,
                                    XML_ERR_ENTITY_BOUNDARY,
                                    b"comment doesn\'t start and stop in the same entity\n\x00"
                                        as *const u8
                                        as *const i8,
                                );
                            }
                            (*(*ctxt).input).cur =
                                (*(*ctxt).input).cur.offset(3 as i32 as isize);
                            (*(*ctxt).input).col += 3 as i32;
                            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                            }
                            if !(*ctxt).sax.is_null()
                                && (*(*ctxt).sax).comment.is_some()
                                && (*ctxt).disableSAX == 0
                            {
                                if !buf.is_null() {
                                    (*(*ctxt).sax).comment.expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        buf,
                                    );
                                } else {
                                    (*(*ctxt).sax).comment.expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        b"\x00" as *const u8 as *const i8 as *mut xmlChar,
                                    );
                                }
                            }
                            if !buf.is_null() {
                                xmlFree_safe(buf as *mut ());
                            }
                            if (*ctxt).instate as i32 != XML_PARSER_EOF as i32 {
                                (*ctxt).instate = state
                            }
                            return;
                        }
                        if !buf.is_null() {
                            xmlFatalErrMsgStr(
                                ctxt,
                                XML_ERR_HYPHEN_IN_COMMENT,
                                b"Double hyphen within comment: <!--%.50s\n\x00" as *const u8
                                    as *const i8,
                                buf,
                            );
                        } else {
                            xmlFatalErrMsgStr(
                                ctxt,
                                XML_ERR_HYPHEN_IN_COMMENT,
                                b"Double hyphen within comment\n\x00" as *const u8
                                    as *const i8,
                                0 as *const xmlChar,
                            );
                        }
                        if (*ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                            xmlFree_safe(buf as *mut ());
                            return;
                        }
                        in_0 = in_0.offset(1);
                        (*(*ctxt).input).col += 1
                    }
                    in_0 = in_0.offset(1);
                    (*(*ctxt).input).col += 1
                }
            }
            if !(*in_0 as i32 >= 0x20 as i32
                && *in_0 as i32 <= 0x7f as i32
                || *in_0 as i32 == 0x9 as i32
                || *in_0 as i32 == 0xa as i32)
            {
                break;
            }
        }
    }
    xmlParseCommentComplex(ctxt, buf, len, size);
    (safe_ctxt).instate = state;
}
/* *
* xmlParsePITarget:
* @ctxt:  an XML parser context
*
* parse the name of a PI
*
* [17] PITarget ::= Name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))
*
* Returns the PITarget name or NULL
*/

pub unsafe fn xmlParsePITarget(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    name = xmlParseName(ctxt);
    if unsafe {
        !name.is_null()
            && (*name.offset(0 as i32 as isize) as i32 == 'x' as i32
                || *name.offset(0 as i32 as isize) as i32 == 'X' as i32)
            && (*name.offset(1 as i32 as isize) as i32 == 'm' as i32
                || *name.offset(1 as i32 as isize) as i32 == 'M' as i32)
            && (*name.offset(2 as i32 as isize) as i32 == 'l' as i32
                || *name.offset(2 as i32 as isize) as i32 == 'L' as i32)
    } {
        let mut i: i32 = 0;
        if unsafe {
            *name.offset(0 as i32 as isize) as i32 == 'x' as i32
                && *name.offset(1 as i32 as isize) as i32 == 'm' as i32
                && *name.offset(2 as i32 as isize) as i32 == 'l' as i32
                && *name.offset(3 as i32 as isize) as i32 == 0 as i32
        } {
            unsafe {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_RESERVED_XML_NAME,
                    b"XML declaration allowed only at the start of the document\n\x00" as *const u8
                        as *const i8,
                );
            }
            return name;
        } else {
            unsafe {
                if *name.offset(3 as i32 as isize) as i32 == 0 as i32 {
                    xmlFatalErr(ctxt, XML_ERR_RESERVED_XML_NAME, 0 as *const i8);
                    return name;
                }
            }
        }
        i = 0 as i32;
        while unsafe { !xmlW3CPIs[i as usize].is_null() } {
            if xmlStrEqual_safe(name, unsafe { xmlW3CPIs[i as usize] as *const xmlChar }) != 0 {
                return name;
            }
            i += 1
        }
        unsafe {
            xmlWarningMsg(
                ctxt,
                XML_ERR_RESERVED_XML_NAME,
                b"xmlParsePITarget: invalid name prefix \'xml\'\n\x00" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
    }
    if !name.is_null() && !xmlStrchr_safe(name, ':' as i32 as xmlChar).is_null() {
        unsafe {
            xmlNsErr(
                ctxt,
                XML_NS_ERR_COLON,
                b"colons are forbidden from PI names \'%s\'\n\x00" as *const u8
                    as *const i8,
                name,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
    }
    return name;
}
/* *
* xmlParseCatalogPI:
* @ctxt:  an XML parser context
* @catalog:  the PI value string
*
* parse an XML Catalog Processing Instruction.
*
* <?oasis-xml-catalog catalog="http://example.com/catalog.xml"?>
*
* Occurs only if allowed by the user and if happening in the Misc
* part of the document before any doctype information
* This will add the given catalog to the parsing context in order
* to be used if there is a resolution need further down in the document
*/

#[cfg(HAVE_parser_LIBXML_CATALOG_ENABLED)]
unsafe fn xmlParseCatalogPI(mut ctxt: xmlParserCtxtPtr, mut catalog: *const xmlChar) {
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
    let mut tmp: *const xmlChar = 0 as *const xmlChar;
    let mut base: *const xmlChar = 0 as *const xmlChar;
    let mut marker: xmlChar = 0;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    tmp = catalog;
    unsafe {
        while *tmp as i32 == 0x20 as i32
            || 0x9 as i32 <= *tmp as i32
                && *tmp as i32 <= 0xa as i32
            || *tmp as i32 == 0xd as i32
        {
            tmp = tmp.offset(1)
        }
    }
    if !(xmlStrncmp_safe(
        tmp,
        b"catalog\x00" as *const u8 as *const i8 as *mut xmlChar,
        7 as i32,
    ) != 0)
    {
        unsafe {
            tmp = tmp.offset(7 as i32 as isize);
            while *tmp as i32 == 0x20 as i32
                || 0x9 as i32 <= *tmp as i32
                    && *tmp as i32 <= 0xa as i32
                || *tmp as i32 == 0xd as i32
            {
                tmp = tmp.offset(1)
            }
            if *tmp as i32 != '=' as i32 {
                return;
            }
            tmp = tmp.offset(1);
            while *tmp as i32 == 0x20 as i32
                || 0x9 as i32 <= *tmp as i32
                    && *tmp as i32 <= 0xa as i32
                || *tmp as i32 == 0xd as i32
            {
                tmp = tmp.offset(1)
            }
            marker = *tmp;
        }

        if !(marker as i32 != '\'' as i32 && marker as i32 != '\"' as i32) {
            unsafe {
                tmp = tmp.offset(1);
                base = tmp;
                while *tmp as i32 != 0 as i32
                    && *tmp as i32 != marker as i32
                {
                    tmp = tmp.offset(1)
                }
            }
            if !(unsafe { *tmp } as i32 == 0 as i32) {
                unsafe {
                    URL =
                        xmlStrndup_safe(base, tmp.offset_from(base) as i64 as i32);
                    tmp = tmp.offset(1);
                    while *tmp as i32 == 0x20 as i32
                        || 0x9 as i32 <= *tmp as i32
                            && *tmp as i32 <= 0xa as i32
                        || *tmp as i32 == 0xd as i32
                    {
                        tmp = tmp.offset(1)
                    }
                }
                if !(unsafe { *tmp } as i32 != 0 as i32) {
                    if !URL.is_null() {
                        (safe_ctxt).catalogs = xmlCatalogAddLocal_safe((safe_ctxt).catalogs, URL);
                        xmlFree_safe(URL as *mut ());
                    }
                    return;
                }
            }
        }
    }
    unsafe {
        xmlWarningMsg(
            ctxt,
            XML_WAR_CATALOG_PI,
            b"Catalog PI syntax error: %s\n\x00" as *const u8 as *const i8,
            catalog,
            0 as *const xmlChar,
        );
    }
    if !URL.is_null() {
        xmlFree_safe(URL as *mut ());
    };
}
/* *
* xmlParsePI:
* @ctxt:  an XML parser context
*
* parse an XML Processing Instruction.
*
* [16] PI ::= '<?' PITarget (S (Char* - (Char* '?>' Char*)))? '?>'
*
* The processing is transferred to SAX once parsed.
*/

pub unsafe fn xmlParsePI(mut ctxt: xmlParserCtxtPtr) {
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: size_t = 0 as i32 as size_t;
    let mut size: size_t = 100 as i32 as size_t;
    let mut cur: i32 = 0;
    let mut l: i32 = 0;
    let mut target: *const xmlChar = 0 as *const xmlChar;
    let mut state: xmlParserInputState = XML_PARSER_START;
    let mut count: i32 = 0 as i32;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if unsafe {
        *(*(*ctxt).input).cur as i32 == '<' as i32
            && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '?' as i32
    } {
        let mut inputid: i32 = unsafe { (*(*ctxt).input).id };
        state = (safe_ctxt).instate;
        (safe_ctxt).instate = XML_PARSER_PI;
        /*
         * this is a Processing Instruction.
         */
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
            (*(*ctxt).input).col += 2 as i32;
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
            xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
        }
        if unsafe {
            (*ctxt).progressive == 0 as i32
                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                    > (2 as i32 * 250 as i32) as i64
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < (2 as i32 * 250 as i32) as i64
        } {
            xmlSHRINK(ctxt);
        }
        /*
         * Parse the target name and check for special support like
         * namespace.
         */
        target = xmlParsePITarget(ctxt);
        if !target.is_null() {
            if unsafe {
                *(*(*ctxt).input).cur as i32 == '?' as i32
                    && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        == '>' as i32
            } {
                if inputid != unsafe { (*(*ctxt).input).id } {
                    unsafe {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_ENTITY_BOUNDARY,
                            b"PI declaration doesn\'t start and stop in the same entity\n\x00"
                                as *const u8 as *const i8,
                        );
                    }
                }
                unsafe {
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
                    (*(*ctxt).input).col += 2 as i32;
                }
                if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
                    xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
                }
                /*
                 * SAX: PI detected.
                 */
                unsafe {
                    if !(*ctxt).sax.is_null()
                        && (*ctxt).disableSAX == 0
                        && (*(*ctxt).sax).processingInstruction.is_some()
                    {
                        (*(*ctxt).sax)
                            .processingInstruction
                            .expect("non-null function pointer")(
                            (*ctxt).userData,
                            target,
                            0 as *const xmlChar,
                        );
                    }
                }
                if (safe_ctxt).instate as i32 != XML_PARSER_EOF as i32 {
                    (safe_ctxt).instate = state
                }
                return;
            }
            buf = xmlMallocAtomic_safe(
                size.wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
            ) as *mut xmlChar;
            if buf.is_null() {
                unsafe {
                    xmlErrMemory(ctxt, 0 as *const i8);
                }
                (safe_ctxt).instate = state;
                return;
            }
            if xmlSkipBlankChars(ctxt) == 0 as i32 {
                unsafe {
                    xmlFatalErrMsgStr(
                        ctxt,
                        XML_ERR_SPACE_REQUIRED,
                        b"ParsePI: PI %s space expected\n\x00" as *const u8 as *const i8,
                        target,
                    );
                }
            }
            unsafe {
                cur = xmlCurrentChar(ctxt, &mut l);
            }
            while (if cur < 0x100 as i32 {
                (0x9 as i32 <= cur && cur <= 0xa as i32
                    || cur == 0xd as i32
                    || 0x20 as i32 <= cur) as i32
            } else {
                (0x100 as i32 <= cur && cur <= 0xd7ff as i32
                    || 0xe000 as i32 <= cur && cur <= 0xfffd as i32
                    || 0x10000 as i32 <= cur && cur <= 0x10ffff as i32)
                    as i32
            }) != 0
                && unsafe {
                    (cur != '?' as i32
                        || *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                            != '>' as i32)
                }
            {
                if len.wrapping_add(5 as i32 as u64) >= size {
                    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                    let mut new_size: size_t = size.wrapping_mul(2 as i32 as u64);
                    tmp = xmlRealloc_safe(buf as *mut (), new_size) as *mut xmlChar;
                    if tmp.is_null() {
                        unsafe {
                            xmlErrMemory(ctxt, 0 as *const i8);
                        }
                        xmlFree_safe(buf as *mut ());
                        (safe_ctxt).instate = state;
                        return;
                    }
                    buf = tmp;
                    size = new_size
                }
                count += 1;
                if count > 50 as i32 {
                    if unsafe {
                        (*ctxt).progressive == 0 as i32
                            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                as i64
                                > (2 as i32 * 250 as i32) as i64
                            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                as i64)
                                < (2 as i32 * 250 as i32) as i64
                    } {
                        xmlSHRINK(ctxt);
                    }
                    if unsafe {
                        (*ctxt).progressive == 0 as i32
                            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                as i64)
                                < 250 as i32 as i64
                    } {
                        xmlGROW(ctxt);
                    }
                    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                        xmlFree_safe(buf as *mut ());
                        return;
                    }
                    count = 0 as i32;
                    if len > 10000000 as i32 as u64
                        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
                    {
                        unsafe {
                            xmlFatalErrMsgStr(
                                ctxt,
                                XML_ERR_PI_NOT_FINISHED,
                                b"PI %s too big found\x00" as *const u8 as *const i8,
                                target,
                            );
                        }
                        xmlFree_safe(buf as *mut ());
                        (safe_ctxt).instate = state;
                        return;
                    }
                }
                if l == 1 as i32 {
                    let fresh86 = len;
                    len = len.wrapping_add(1);
                    unsafe { *buf.offset(fresh86 as isize) = cur as xmlChar };
                } else {
                    len = unsafe {
                        (len as u64).wrapping_add(xmlCopyCharMultiByte(
                            &mut *buf.offset(len as isize),
                            cur,
                        )
                            as u64) as size_t as size_t
                    };
                }
                unsafe {
                    if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
                        (*(*ctxt).input).line += 1;
                        (*(*ctxt).input).col = 1 as i32
                    } else {
                        (*(*ctxt).input).col += 1
                    }
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                    cur = xmlCurrentChar(ctxt, &mut l);
                }
                if cur == 0 as i32 {
                    if unsafe {
                        (*ctxt).progressive == 0 as i32
                            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                as i64
                                > (2 as i32 * 250 as i32) as i64
                            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                as i64)
                                < (2 as i32 * 250 as i32) as i64
                    } {
                        xmlSHRINK(ctxt);
                    }
                    if unsafe {
                        (*ctxt).progressive == 0 as i32
                            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                as i64)
                                < 250 as i32 as i64
                    } {
                        xmlGROW(ctxt);
                    }
                    cur = unsafe { xmlCurrentChar(ctxt, &mut l) };
                }
            }
            if len > 10000000 as i32 as u64
                && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
            {
                unsafe {
                    xmlFatalErrMsgStr(
                        ctxt,
                        XML_ERR_PI_NOT_FINISHED,
                        b"PI %s too big found\x00" as *const u8 as *const i8,
                        target,
                    );
                }
                xmlFree_safe(buf as *mut ());
                (safe_ctxt).instate = state;
                return;
            }
            unsafe { *buf.offset(len as isize) = 0 as i32 as xmlChar };
            if cur != '?' as i32 {
                unsafe {
                    xmlFatalErrMsgStr(
                        ctxt,
                        XML_ERR_PI_NOT_FINISHED,
                        b"ParsePI: PI %s never end ...\n\x00" as *const u8 as *const i8,
                        target,
                    );
                }
            } else {
                unsafe {
                    if inputid != (*(*ctxt).input).id {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_ENTITY_BOUNDARY,
                            b"PI declaration doesn\'t start and stop in the same entity\n\x00"
                                as *const u8 as *const i8,
                        );
                    }
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
                    (*(*ctxt).input).col += 2 as i32;
                    if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                        xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
                    }
                }

                match () {
                    #[cfg(HAVE_parser_LIBXML_CATALOG_ENABLED)]
                    _ => {
                        if (state as i32 == XML_PARSER_MISC as i32
                            || state as i32 == XML_PARSER_START as i32)
                            && xmlStrEqual_safe(
                                target,
                                b"oasis-xml-catalog\x00" as *const u8 as *const i8
                                    as *const xmlChar,
                            ) != 0
                        {
                            let mut allow: xmlCatalogAllow = xmlCatalogGetDefaults_safe();
                            if allow as u32
                                == XML_CATA_ALLOW_DOCUMENT as i32 as u32
                                || allow as u32
                                    == XML_CATA_ALLOW_ALL as i32 as u32
                            {
                                xmlParseCatalogPI(ctxt, buf);
                            }
                        }
                    }
                    #[cfg(not(HAVE_parser_LIBXML_CATALOG_ENABLED))]
                    _ => {}
                };

                /*
                 * SAX: PI detected.
                 */
                unsafe {
                    if !(*ctxt).sax.is_null()
                        && (*ctxt).disableSAX == 0
                        && (*(*ctxt).sax).processingInstruction.is_some()
                    {
                        (*(*ctxt).sax)
                            .processingInstruction
                            .expect("non-null function pointer")(
                            (*ctxt).userData, target, buf
                        );
                    }
                }
            }
            xmlFree_safe(buf as *mut ());
        } else {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_PI_NOT_STARTED, 0 as *const i8);
            }
        }
        if (safe_ctxt).instate as i32 != XML_PARSER_EOF as i32 {
            (safe_ctxt).instate = state
        }
    };
}
/* *
* xmlParseNotationDecl:
* @ctxt:  an XML parser context
*
* parse a notation declaration
*
* [82] NotationDecl ::= '<!NOTATION' S Name S (ExternalID |  PublicID) S? '>'
*
* Hence there is actually 3 choices:
*     'PUBLIC' S PubidLiteral
*     'PUBLIC' S PubidLiteral S SystemLiteral
* and 'SYSTEM' S SystemLiteral
*
* See the NOTE on xmlParseExternalID().
*/

pub unsafe fn xmlParseNotationDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut Pubid: *mut xmlChar = 0 as *mut xmlChar;
    let mut Systemid: *mut xmlChar = 0 as *mut xmlChar;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if unsafe {
        *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '!' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'N' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'O' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'A' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'I' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(8 as i32 as isize)
                as i32
                == 'O' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(9 as i32 as isize)
                as i32
                == 'N' as i32
    } {
        let mut inputid: i32 = unsafe { (*(*ctxt).input).id };
        if unsafe {
            (*ctxt).progressive == 0 as i32
                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                    > (2 as i32 * 250 as i32) as i64
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < (2 as i32 * 250 as i32) as i64
        } {
            xmlSHRINK(ctxt);
        }
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(10 as i32 as isize);
            (*(*ctxt).input).col += 10 as i32;
        }

        if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
            xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
        }
        if xmlSkipBlankChars(ctxt) == 0 as i32 {
            unsafe {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after \'<!NOTATION\'\n\x00" as *const u8
                        as *const i8,
                );
            }
            return;
        }
        name = xmlParseName(ctxt);
        if name.is_null() {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_NOTATION_NOT_STARTED, 0 as *const i8);
            }
            return;
        }
        if !xmlStrchr_safe(name, ':' as i32 as xmlChar).is_null() {
            xmlNsErr(
                ctxt,
                XML_NS_ERR_COLON,
                b"colons are forbidden from notation names \'%s\'\n\x00" as *const u8
                    as *const i8,
                name,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
        if xmlSkipBlankChars(ctxt) == 0 as i32 {
            unsafe {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after the NOTATION name\'\n\x00" as *const u8
                        as *const i8,
                );
            }
            return;
        }
        /*
         * Parse the IDs.
         */
        Systemid = xmlParseExternalID(ctxt, &mut Pubid, 0 as i32);
        xmlSkipBlankChars(ctxt);
        unsafe {
            if *(*(*ctxt).input).cur as i32 == '>' as i32 {
                if inputid != (*(*ctxt).input).id {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ENTITY_BOUNDARY,
                        b"Notation declaration doesn\'t start and stop in the same entity\n\x00"
                            as *const u8 as *const i8,
                    );
                }
                xmlNextChar_safe(ctxt);
                if !(*ctxt).sax.is_null()
                    && (*ctxt).disableSAX == 0
                    && (*(*ctxt).sax).notationDecl.is_some()
                {
                    (*(*ctxt).sax)
                        .notationDecl
                        .expect("non-null function pointer")(
                        (*ctxt).userData,
                        name,
                        Pubid,
                        Systemid,
                    );
                }
            } else {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_NOTATION_NOT_FINISHED,
                    0 as *const i8,
                );
            }
        }

        if !Systemid.is_null() {
            xmlFree_safe(Systemid as *mut ());
        }
        if !Pubid.is_null() {
            xmlFree_safe(Pubid as *mut ());
        }
    };
}
/* *
* xmlParseEntityDecl:
* @ctxt:  an XML parser context
*
* parse <!ENTITY declarations
*
* [70] EntityDecl ::= GEDecl | PEDecl
*
* [71] GEDecl ::= '<!ENTITY' S Name S EntityDef S? '>'
*
* [72] PEDecl ::= '<!ENTITY' S '%' S Name S PEDef S? '>'
*
* [73] EntityDef ::= EntityValue | (ExternalID NDataDecl?)
*
* [74] PEDef ::= EntityValue | ExternalID
*
* [76] NDataDecl ::= S 'NDATA' S Name
*
* [ VC: Notation Declared ]
* The Name must match the declared name of a notation.
*/

pub unsafe fn xmlParseEntityDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut value: *mut xmlChar = 0 as *mut xmlChar;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    let mut literal: *mut xmlChar = 0 as *mut xmlChar;
    let mut ndata: *const xmlChar = 0 as *const xmlChar;
    let mut isParameter: i32 = 0 as i32;
    let mut orig: *mut xmlChar = 0 as *mut xmlChar;
    /* GROW; done in the caller */
    //@todo 削减unsafe范围
    unsafe {
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '!' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'E' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'N' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'I' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'Y' as i32
        {
            let mut inputid: i32 = (*(*ctxt).input).id;
            if (*ctxt).progressive == 0 as i32
                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                    > (2 as i32 * 250 as i32) as i64
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < (2 as i32 * 250 as i32) as i64
            {
                xmlSHRINK(ctxt);
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(8 as i32 as isize);
            (*(*ctxt).input).col += 8 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            if xmlSkipBlankChars(ctxt) == 0 as i32 {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after \'<!ENTITY\'\n\x00" as *const u8 as *const i8,
                );
            }
            if *(*(*ctxt).input).cur as i32 == '%' as i32 {
                xmlNextChar_safe(ctxt);
                if xmlSkipBlankChars(ctxt) == 0 as i32 {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_SPACE_REQUIRED,
                        b"Space required after \'%%\'\n\x00" as *const u8 as *const i8,
                    );
                }
                isParameter = 1 as i32
            }
            name = xmlParseName(ctxt);
            if name.is_null() {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_NAME_REQUIRED,
                    b"xmlParseEntityDecl: no name\n\x00" as *const u8 as *const i8,
                );
                return;
            }
            if !xmlStrchr_safe(name, ':' as i32 as xmlChar).is_null() {
                xmlNsErr(
                    ctxt,
                    XML_NS_ERR_COLON,
                    b"colons are forbidden from entities names \'%s\'\n\x00" as *const u8
                        as *const i8,
                    name,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if xmlSkipBlankChars(ctxt) == 0 as i32 {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after the entity name\n\x00" as *const u8
                        as *const i8,
                );
            }
            (*ctxt).instate = XML_PARSER_ENTITY_DECL;
            /*
             * handle the various case of definitions...
             */
            if isParameter != 0 {
                if *(*(*ctxt).input).cur as i32 == '\"' as i32
                    || *(*(*ctxt).input).cur as i32 == '\'' as i32
                {
                    value = xmlParseEntityValue(ctxt, &mut orig);
                    if !value.is_null() {
                        if !(*ctxt).sax.is_null()
                            && (*ctxt).disableSAX == 0
                            && (*(*ctxt).sax).entityDecl.is_some()
                        {
                            (*(*ctxt).sax)
                                .entityDecl
                                .expect("non-null function pointer")(
                                (*ctxt).userData,
                                name,
                                XML_INTERNAL_PARAMETER_ENTITY as i32,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                                value,
                            );
                        }
                    }
                } else {
                    URI = xmlParseExternalID(ctxt, &mut literal, 1 as i32);
                    if URI.is_null() && literal.is_null() {
                        xmlFatalErr(ctxt, XML_ERR_VALUE_REQUIRED, 0 as *const i8);
                    }
                    if !URI.is_null() {
                        let mut uri: xmlURIPtr = 0 as *mut xmlURI;
                        uri = xmlParseURI_safe(URI as *const i8);
                        if uri.is_null() {
                            xmlErrMsgStr(
                                ctxt,
                                XML_ERR_INVALID_URI,
                                b"Invalid URI: %s\n\x00" as *const u8 as *const i8,
                                URI,
                            );
                            /*
                             * This really ought to be a well formedness error
                             * but the XML Core WG decided otherwise c.f. issue
                             * E26 of the XML erratas.
                             */
                        } else {
                            if !(*uri).fragment.is_null() {
                                /*
                                 * Okay this is foolish to block those but not
                                 * invalid URIs.
                                 */
                                xmlFatalErr(ctxt, XML_ERR_URI_FRAGMENT, 0 as *const i8);
                            } else if !(*ctxt).sax.is_null()
                                && (*ctxt).disableSAX == 0
                                && (*(*ctxt).sax).entityDecl.is_some()
                            {
                                (*(*ctxt).sax)
                                    .entityDecl
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    name,
                                    XML_EXTERNAL_PARAMETER_ENTITY as i32,
                                    literal,
                                    URI,
                                    0 as *mut xmlChar,
                                );
                            }
                            xmlFreeURI_safe(uri);
                        }
                    }
                }
            } else if *(*(*ctxt).input).cur as i32 == '\"' as i32
                || *(*(*ctxt).input).cur as i32 == '\'' as i32
            {
                value = xmlParseEntityValue(ctxt, &mut orig);
                if !(*ctxt).sax.is_null()
                    && (*ctxt).disableSAX == 0
                    && (*(*ctxt).sax).entityDecl.is_some()
                {
                    (*(*ctxt).sax)
                        .entityDecl
                        .expect("non-null function pointer")(
                        (*ctxt).userData,
                        name,
                        XML_INTERNAL_GENERAL_ENTITY as i32,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                        value,
                    );
                }
                /*
                 * For expat compatibility in SAX mode.
                 */
                if (*ctxt).myDoc.is_null()
                    || xmlStrEqual_safe(
                        (*(*ctxt).myDoc).version,
                        b"SAX compatibility mode document\x00" as *const u8 as *const i8
                            as *mut xmlChar,
                    ) != 0
                {
                    if (*ctxt).myDoc.is_null() {
                        (*ctxt).myDoc = xmlNewDoc_safe(
                            b"SAX compatibility mode document\x00" as *const u8
                                as *const i8 as *mut xmlChar,
                        );
                        if (*ctxt).myDoc.is_null() {
                            xmlErrMemory(
                                ctxt,
                                b"New Doc failed\x00" as *const u8 as *const i8,
                            );
                            return;
                        }
                        (*(*ctxt).myDoc).properties = XML_DOC_INTERNAL as i32
                    }
                    if (*(*ctxt).myDoc).intSubset.is_null() {
                        (*(*ctxt).myDoc).intSubset = xmlNewDtd(
                            (*ctxt).myDoc,
                            b"fake\x00" as *const u8 as *const i8 as *mut xmlChar,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                        )
                    }
                    xmlSAX2EntityDecl(
                        ctxt as *mut (),
                        name,
                        XML_INTERNAL_GENERAL_ENTITY as i32,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                        value,
                    );
                }
            } else {
                URI = xmlParseExternalID(ctxt, &mut literal, 1 as i32);
                if URI.is_null() && literal.is_null() {
                    xmlFatalErr(ctxt, XML_ERR_VALUE_REQUIRED, 0 as *const i8);
                }
                if !URI.is_null() {
                    let mut uri_0: xmlURIPtr = 0 as *mut xmlURI;
                    uri_0 = xmlParseURI_safe(URI as *const i8);
                    if uri_0.is_null() {
                        xmlErrMsgStr(
                            ctxt,
                            XML_ERR_INVALID_URI,
                            b"Invalid URI: %s\n\x00" as *const u8 as *const i8,
                            URI,
                        );
                        /*
                         * This really ought to be a well formedness error
                         * but the XML Core WG decided otherwise c.f. issue
                         * E26 of the XML erratas.
                         */
                    } else {
                        if !(*uri_0).fragment.is_null() {
                            /*
                             * Okay this is foolish to block those but not
                             * invalid URIs.
                             */
                            xmlFatalErr(ctxt, XML_ERR_URI_FRAGMENT, 0 as *const i8);
                        }
                        xmlFreeURI_safe(uri_0);
                    }
                }
                if *(*(*ctxt).input).cur as i32 != '>' as i32
                    && xmlSkipBlankChars(ctxt) == 0 as i32
                {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_SPACE_REQUIRED,
                        b"Space required before \'NDATA\'\n\x00" as *const u8
                            as *const i8,
                    );
                }
                if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
                    as i32
                    == 'N' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(1 as i32 as isize) as i32
                        == 'D' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(2 as i32 as isize) as i32
                        == 'A' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(3 as i32 as isize) as i32
                        == 'T' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(4 as i32 as isize) as i32
                        == 'A' as i32
                {
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(5 as i32 as isize);
                    (*(*ctxt).input).col += 5 as i32;
                    if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                        xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                    }
                    if xmlSkipBlankChars(ctxt) == 0 as i32 {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_SPACE_REQUIRED,
                            b"Space required after \'NDATA\'\n\x00" as *const u8
                                as *const i8,
                        );
                    }
                    ndata = xmlParseName(ctxt);
                    if !(*ctxt).sax.is_null()
                        && (*ctxt).disableSAX == 0
                        && (*(*ctxt).sax).unparsedEntityDecl.is_some()
                    {
                        (*(*ctxt).sax)
                            .unparsedEntityDecl
                            .expect("non-null function pointer")(
                            (*ctxt).userData,
                            name,
                            literal,
                            URI,
                            ndata,
                        );
                    }
                } else {
                    if !(*ctxt).sax.is_null()
                        && (*ctxt).disableSAX == 0
                        && (*(*ctxt).sax).entityDecl.is_some()
                    {
                        (*(*ctxt).sax)
                            .entityDecl
                            .expect("non-null function pointer")(
                            (*ctxt).userData,
                            name,
                            XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32,
                            literal,
                            URI,
                            0 as *mut xmlChar,
                        );
                    }
                    /*
                     * For expat compatibility in SAX mode.
                     * assuming the entity replacement was asked for
                     */
                    if (*ctxt).replaceEntities != 0 as i32
                        && ((*ctxt).myDoc.is_null()
                            || xmlStrEqual_safe(
                                (*(*ctxt).myDoc).version,
                                b"SAX compatibility mode document\x00" as *const u8
                                    as *const i8
                                    as *mut xmlChar,
                            ) != 0)
                    {
                        if (*ctxt).myDoc.is_null() {
                            (*ctxt).myDoc = xmlNewDoc_safe(
                                b"SAX compatibility mode document\x00" as *const u8
                                    as *const i8
                                    as *mut xmlChar,
                            );
                            if (*ctxt).myDoc.is_null() {
                                xmlErrMemory(
                                    ctxt,
                                    b"New Doc failed\x00" as *const u8 as *const i8,
                                );
                                return;
                            }
                            (*(*ctxt).myDoc).properties = XML_DOC_INTERNAL as i32
                        }
                        if (*(*ctxt).myDoc).intSubset.is_null() {
                            (*(*ctxt).myDoc).intSubset = xmlNewDtd(
                                (*ctxt).myDoc,
                                b"fake\x00" as *const u8 as *const i8 as *mut xmlChar,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                            )
                        }
                        xmlSAX2EntityDecl(
                            ctxt as *mut (),
                            name,
                            XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32,
                            literal,
                            URI,
                            0 as *mut xmlChar,
                        );
                    }
                }
            }
            if !((*ctxt).instate as i32 == XML_PARSER_EOF as i32) {
                xmlSkipBlankChars(ctxt);
                if *(*(*ctxt).input).cur as i32 != '>' as i32 {
                    xmlFatalErrMsgStr(
                        ctxt,
                        XML_ERR_ENTITY_NOT_FINISHED,
                        b"xmlParseEntityDecl: entity %s not terminated\n\x00" as *const u8
                            as *const i8,
                        name,
                    );
                    xmlHaltParser(ctxt);
                } else {
                    if inputid != (*(*ctxt).input).id {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_ENTITY_BOUNDARY,
                            b"Entity declaration doesn\'t start and stop in the same entity\n\x00"
                                as *const u8 as *const i8,
                        );
                    }
                    xmlNextChar_safe(ctxt);
                }
                if !orig.is_null() {
                    /*
                     * Ugly mechanism to save the raw entity value.
                     */
                    let mut cur: xmlEntityPtr = 0 as xmlEntityPtr;
                    if isParameter != 0 {
                        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).getParameterEntity.is_some() {
                            cur = (*(*ctxt).sax)
                                .getParameterEntity
                                .expect("non-null function pointer")(
                                (*ctxt).userData, name
                            )
                        }
                    } else {
                        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).getEntity.is_some() {
                            cur = (*(*ctxt).sax).getEntity.expect("non-null function pointer")(
                                (*ctxt).userData,
                                name,
                            )
                        }
                        if cur.is_null() && (*ctxt).userData == ctxt as *mut () {
                            cur = xmlSAX2GetEntity_safe(ctxt as *mut (), name)
                        }
                    }
                    if !cur.is_null() && (*cur).orig.is_null() {
                        (*cur).orig = orig;
                        orig = 0 as *mut xmlChar
                    }
                }
            }
            if !value.is_null() {
                xmlFree_safe(value as *mut ());
            }
            if !URI.is_null() {
                xmlFree_safe(URI as *mut ());
            }
            if !literal.is_null() {
                xmlFree_safe(literal as *mut ());
            }
            if !orig.is_null() {
                xmlFree_safe(orig as *mut ());
            }
        };
    }
}
/* *
* xmlParseDefaultDecl:
* @ctxt:  an XML parser context
* @value:  Receive a possible fixed default value for the attribute
*
* Parse an attribute default declaration
*
* [60] DefaultDecl ::= '#REQUIRED' | '#IMPLIED' | (('#FIXED' S)? AttValue)
*
* [ VC: Required Attribute ]
* if the default declaration is the keyword #REQUIRED, then the
* attribute must be specified for all elements of the type in the
* attribute-list declaration.
*
* [ VC: Attribute Default Legal ]
* The declared default value must meet the lexical constraints of
* the declared attribute type c.f. xmlValidateAttributeDecl()
*
* [ VC: Fixed Attribute Default ]
* if an attribute has a default value declared with the #FIXED
* keyword, instances of that attribute must match the default value.
*
* [ WFC: No < in Attribute Values ]
* handled in xmlParseAttValue()
*
* returns: XML_ATTRIBUTE_NONE, XML_ATTRIBUTE_REQUIRED, XML_ATTRIBUTE_IMPLIED
*          or XML_ATTRIBUTE_FIXED.
*/

pub unsafe fn xmlParseDefaultDecl(
    mut ctxt: xmlParserCtxtPtr,
    mut value: *mut *mut xmlChar,
) -> i32 {
    let mut val: i32 = 0;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    //@todo 削减unsafe范围
    unsafe {
        *value = 0 as *mut xmlChar;
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '#' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'R' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'E' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'Q' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'U' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'I' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'R' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'E' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(8 as i32 as isize)
                as i32
                == 'D' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(9 as i32 as isize);
            (*(*ctxt).input).col += 9 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            return XML_ATTRIBUTE_REQUIRED as i32;
        }
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '#' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'I' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'M' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'P' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'L' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'I' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'E' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'D' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(8 as i32 as isize);
            (*(*ctxt).input).col += 8 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            return XML_ATTRIBUTE_IMPLIED as i32;
        }
        val = XML_ATTRIBUTE_NONE as i32;
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '#' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'F' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'I' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'X' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'E' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'D' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(6 as i32 as isize);
            (*(*ctxt).input).col += 6 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            val = XML_ATTRIBUTE_FIXED as i32;
            if xmlSkipBlankChars(ctxt) == 0 as i32 {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after \'#FIXED\'\n\x00" as *const u8 as *const i8,
                );
            }
        }
        ret = xmlParseAttValue(ctxt);
        (*ctxt).instate = XML_PARSER_DTD;
        if ret.is_null() {
            xmlFatalErrMsg(
                ctxt,
                (*ctxt).errNo as xmlParserErrors,
                b"Attribute default value declaration error\n\x00" as *const u8
                    as *const i8,
            );
        } else {
            *value = ret
        }
    }
    return val;
}
/* *
* xmlParseNotationType:
* @ctxt:  an XML parser context
*
* parse an Notation attribute type.
*
* Note: the leading 'NOTATION' S part has already being parsed...
*
* [58] NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
*
* [ VC: Notation Attributes ]
* Values of this type must match one of the notation names included
* in the declaration; all notation names in the declaration must be declared.
*
* Returns: the notation attribute tree built while parsing
*/

pub unsafe fn xmlParseNotationType(mut ctxt: xmlParserCtxtPtr) -> xmlEnumerationPtr {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut ret: xmlEnumerationPtr = 0 as xmlEnumerationPtr;
    let mut last: xmlEnumerationPtr = 0 as xmlEnumerationPtr;
    let mut cur: xmlEnumerationPtr = 0 as *mut xmlEnumeration;
    let mut tmp: xmlEnumerationPtr = 0 as *mut xmlEnumeration;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    unsafe {
        if *(*(*ctxt).input).cur as i32 != '(' as i32 {
            xmlFatalErr(ctxt, XML_ERR_NOTATION_NOT_STARTED, 0 as *const i8);
            return 0 as xmlEnumerationPtr;
        }
    }

    if unsafe {
        (*ctxt).progressive == 0 as i32
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < (2 as i32 * 250 as i32) as i64
    } {
        xmlSHRINK(ctxt);
    }
    loop {
        xmlNextChar_safe(ctxt);
        xmlSkipBlankChars(ctxt);
        name = xmlParseName(ctxt);
        if name.is_null() {
            unsafe {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_NAME_REQUIRED,
                    b"Name expected in NOTATION declaration\n\x00" as *const u8
                        as *const i8,
                );
            }
            xmlFreeEnumeration_safe(ret);
            return 0 as xmlEnumerationPtr;
        }
        tmp = ret;
        while !tmp.is_null() {
            if xmlStrEqual_safe(name, unsafe { (*tmp).name }) != 0 {
                unsafe {
                    xmlValidityError(
                        ctxt,
                        XML_DTD_DUP_TOKEN,
                        b"standalone: attribute notation value token %s duplicated\n\x00"
                            as *const u8 as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                }
                if xmlDictOwns_safe((safe_ctxt).dict, name) == 0 {
                    xmlFree_safe(name as *mut xmlChar as *mut ());
                }
                break;
            } else {
                tmp = unsafe { (*tmp).next }
            }
        }
        if tmp.is_null() {
            cur = xmlCreateEnumeration_safe(name);
            if cur.is_null() {
                xmlFreeEnumeration_safe(ret);
                return 0 as xmlEnumerationPtr;
            }
            if last.is_null() {
                last = cur;
                ret = last
            } else {
                unsafe {
                    (*last).next = cur;
                }
                last = cur
            }
        }
        xmlSkipBlankChars(ctxt);
        if unsafe { !(*(*(*ctxt).input).cur as i32 == '|' as i32) } {
            break;
        }
    }
    if unsafe { *(*(*ctxt).input).cur as i32 != ')' as i32 } {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_NOTATION_NOT_FINISHED,
                0 as *const i8,
            );
        }
        xmlFreeEnumeration_safe(ret);
        return 0 as xmlEnumerationPtr;
    }
    xmlNextChar_safe(ctxt);
    return ret;
}
/* *
* xmlParseEnumerationType:
* @ctxt:  an XML parser context
*
* parse an Enumeration attribute type.
*
* [59] Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
*
* [ VC: Enumeration ]
* Values of this type must match one of the Nmtoken tokens in
* the declaration
*
* Returns: the enumeration attribute tree built while parsing
*/

pub unsafe fn xmlParseEnumerationType(mut ctxt: xmlParserCtxtPtr) -> xmlEnumerationPtr {
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: xmlEnumerationPtr = 0 as xmlEnumerationPtr;
    let mut last: xmlEnumerationPtr = 0 as xmlEnumerationPtr;
    let mut cur: xmlEnumerationPtr = 0 as *mut xmlEnumeration;
    let mut tmp: xmlEnumerationPtr = 0 as *mut xmlEnumeration;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    unsafe {
        if *(*(*ctxt).input).cur as i32 != '(' as i32 {
            xmlFatalErr(ctxt, XML_ERR_ATTLIST_NOT_STARTED, 0 as *const i8);
            return 0 as xmlEnumerationPtr;
        }
    }
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < (2 as i32 * 250 as i32) as i64
    } {
        xmlSHRINK(ctxt);
    }
    loop {
        xmlNextChar_safe(ctxt);
        xmlSkipBlankChars(ctxt);
        name = xmlParseNmtoken(ctxt);
        if name.is_null() {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_NMTOKEN_REQUIRED, 0 as *const i8);
            }
            return ret;
        }
        tmp = ret;
        while !tmp.is_null() {
            if xmlStrEqual_safe(name, unsafe { (*tmp).name }) != 0 {
                unsafe {
                    xmlValidityError(
                        ctxt,
                        XML_DTD_DUP_TOKEN,
                        b"standalone: attribute enumeration value token %s duplicated\n\x00"
                            as *const u8 as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                }
                if xmlDictOwns_safe((safe_ctxt).dict, name) == 0 {
                    xmlFree_safe(name as *mut ());
                }
                break;
            } else {
                tmp = unsafe { (*tmp).next };
            }
        }
        if tmp.is_null() {
            cur = xmlCreateEnumeration_safe(name);
            if xmlDictOwns_safe((safe_ctxt).dict, name) == 0 {
                xmlFree_safe(name as *mut ());
            }
            if cur.is_null() {
                xmlFreeEnumeration_safe(ret);
                return 0 as xmlEnumerationPtr;
            }
            if last.is_null() {
                last = cur;
                ret = last
            } else {
                unsafe {
                    (*last).next = cur;
                }
                last = cur
            }
        }
        xmlSkipBlankChars(ctxt);
        if unsafe { !(*(*(*ctxt).input).cur as i32 == '|' as i32) } {
            break;
        }
    }
    unsafe {
        if *(*(*ctxt).input).cur as i32 != ')' as i32 {
            xmlFatalErr(ctxt, XML_ERR_ATTLIST_NOT_FINISHED, 0 as *const i8);
            return ret;
        }
    }
    xmlNextChar_safe(ctxt);
    return ret;
}
/* *
* xmlParseEnumeratedType:
* @ctxt:  an XML parser context
* @tree:  the enumeration tree built while parsing
*
* parse an Enumerated attribute type.
*
* [57] EnumeratedType ::= NotationType | Enumeration
*
* [58] NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
*
*
* Returns: XML_ATTRIBUTE_ENUMERATION or XML_ATTRIBUTE_NOTATION
*/

pub unsafe fn xmlParseEnumeratedType(
    mut ctxt: xmlParserCtxtPtr,
    mut tree: *mut xmlEnumerationPtr,
) -> i32 {
    //@todo 削减unsafe范围
    unsafe {
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == 'N' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'O' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'A' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'I' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'O' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'N' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(8 as i32 as isize);
            (*(*ctxt).input).col += 8 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            if xmlSkipBlankChars(ctxt) == 0 as i32 {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after \'NOTATION\'\n\x00" as *const u8 as *const i8,
                );
                return 0 as i32;
            }
            *tree = xmlParseNotationType(ctxt);
            if (*tree).is_null() {
                return 0 as i32;
            }
            return XML_ATTRIBUTE_NOTATION as i32;
        }
        *tree = xmlParseEnumerationType(ctxt);
        if (*tree).is_null() {
            return 0 as i32;
        }
    }
    return XML_ATTRIBUTE_ENUMERATION as i32;
}
/* *
* xmlParseAttributeType:
* @ctxt:  an XML parser context
* @tree:  the enumeration tree built while parsing
*
* parse the Attribute list def for an element
*
* [54] AttType ::= StringType | TokenizedType | EnumeratedType
*
* [55] StringType ::= 'CDATA'
*
* [56] TokenizedType ::= 'ID' | 'IDREF' | 'IDREFS' | 'ENTITY' |
*                        'ENTITIES' | 'NMTOKEN' | 'NMTOKENS'
*
* Validity constraints for attribute values syntax are checked in
* xmlValidateAttributeValue()
*
* [ VC: ID ]
* Values of type ID must match the Name production. A name must not
* appear more than once in an XML document as a value of this type;
* i.e., ID values must uniquely identify the elements which bear them.
*
* [ VC: One ID per Element Type ]
* No element type may have more than one ID attribute specified.
*
* [ VC: ID Attribute Default ]
* An ID attribute must have a declared default of #IMPLIED or #REQUIRED.
*
* [ VC: IDREF ]
* Values of type IDREF must match the Name production, and values
* of type IDREFS must match Names; each IDREF Name must match the value
* of an ID attribute on some element in the XML document; i.e. IDREF
* values must match the value of some ID attribute.
*
* [ VC: Entity Name ]
* Values of type ENTITY must match the Name production, values
* of type ENTITIES must match Names; each Entity Name must match the
* name of an unparsed entity declared in the DTD.
*
* [ VC: Name Token ]
* Values of type NMTOKEN must match the Nmtoken production; values
* of type NMTOKENS must match Nmtokens.
*
* Returns the attribute type
*/

pub unsafe fn xmlParseAttributeType(
    mut ctxt: xmlParserCtxtPtr,
    mut tree: *mut xmlEnumerationPtr,
) -> i32 {
    //@todo 削减unsafe范围
    unsafe {
        if (*ctxt).progressive == 0 as i32
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < (2 as i32 * 250 as i32) as i64
        {
            xmlSHRINK(ctxt);
        }
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == 'C' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'D' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'A' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'A' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(5 as i32 as isize);
            (*(*ctxt).input).col += 5 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            return XML_ATTRIBUTE_CDATA as i32;
        } else {
            if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
                as i32
                == 'I' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                    as i32
                    == 'D' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                    as i32
                    == 'R' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                    as i32
                    == 'E' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                    as i32
                    == 'F' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                    as i32
                    == 'S' as i32
            {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(6 as i32 as isize);
                (*(*ctxt).input).col += 6 as i32;
                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                }
                return XML_ATTRIBUTE_IDREFS as i32;
            } else {
                if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
                    as i32
                    == 'I' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(1 as i32 as isize) as i32
                        == 'D' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(2 as i32 as isize) as i32
                        == 'R' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(3 as i32 as isize) as i32
                        == 'E' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(4 as i32 as isize) as i32
                        == 'F' as i32
                {
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(5 as i32 as isize);
                    (*(*ctxt).input).col += 5 as i32;
                    if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                        xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                    }
                    return XML_ATTRIBUTE_IDREF as i32;
                } else {
                    if *(*(*ctxt).input).cur as i32 == 'I' as i32
                        && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                            == 'D' as i32
                    {
                        (*(*ctxt).input).cur =
                            (*(*ctxt).input).cur.offset(2 as i32 as isize);
                        (*(*ctxt).input).col += 2 as i32;
                        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                            xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                        }
                        return XML_ATTRIBUTE_ID as i32;
                    } else {
                        if *((*(*ctxt).input).cur as *mut u8)
                            .offset(0 as i32 as isize)
                            as i32
                            == 'E' as i32
                            && *((*(*ctxt).input).cur as *mut u8)
                                .offset(1 as i32 as isize)
                                as i32
                                == 'N' as i32
                            && *((*(*ctxt).input).cur as *mut u8)
                                .offset(2 as i32 as isize)
                                as i32
                                == 'T' as i32
                            && *((*(*ctxt).input).cur as *mut u8)
                                .offset(3 as i32 as isize)
                                as i32
                                == 'I' as i32
                            && *((*(*ctxt).input).cur as *mut u8)
                                .offset(4 as i32 as isize)
                                as i32
                                == 'T' as i32
                            && *((*(*ctxt).input).cur as *mut u8)
                                .offset(5 as i32 as isize)
                                as i32
                                == 'Y' as i32
                        {
                            (*(*ctxt).input).cur =
                                (*(*ctxt).input).cur.offset(6 as i32 as isize);
                            (*(*ctxt).input).col += 6 as i32;
                            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                            }
                            return XML_ATTRIBUTE_ENTITY as i32;
                        } else {
                            if *((*(*ctxt).input).cur as *mut u8)
                                .offset(0 as i32 as isize)
                                as i32
                                == 'E' as i32
                                && *((*(*ctxt).input).cur as *mut u8)
                                    .offset(1 as i32 as isize)
                                    as i32
                                    == 'N' as i32
                                && *((*(*ctxt).input).cur as *mut u8)
                                    .offset(2 as i32 as isize)
                                    as i32
                                    == 'T' as i32
                                && *((*(*ctxt).input).cur as *mut u8)
                                    .offset(3 as i32 as isize)
                                    as i32
                                    == 'I' as i32
                                && *((*(*ctxt).input).cur as *mut u8)
                                    .offset(4 as i32 as isize)
                                    as i32
                                    == 'T' as i32
                                && *((*(*ctxt).input).cur as *mut u8)
                                    .offset(5 as i32 as isize)
                                    as i32
                                    == 'I' as i32
                                && *((*(*ctxt).input).cur as *mut u8)
                                    .offset(6 as i32 as isize)
                                    as i32
                                    == 'E' as i32
                                && *((*(*ctxt).input).cur as *mut u8)
                                    .offset(7 as i32 as isize)
                                    as i32
                                    == 'S' as i32
                            {
                                (*(*ctxt).input).cur =
                                    (*(*ctxt).input).cur.offset(8 as i32 as isize);
                                (*(*ctxt).input).col += 8 as i32;
                                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                                }
                                return XML_ATTRIBUTE_ENTITIES as i32;
                            } else {
                                if *((*(*ctxt).input).cur as *mut u8)
                                    .offset(0 as i32 as isize)
                                    as i32
                                    == 'N' as i32
                                    && *((*(*ctxt).input).cur as *mut u8)
                                        .offset(1 as i32 as isize)
                                        as i32
                                        == 'M' as i32
                                    && *((*(*ctxt).input).cur as *mut u8)
                                        .offset(2 as i32 as isize)
                                        as i32
                                        == 'T' as i32
                                    && *((*(*ctxt).input).cur as *mut u8)
                                        .offset(3 as i32 as isize)
                                        as i32
                                        == 'O' as i32
                                    && *((*(*ctxt).input).cur as *mut u8)
                                        .offset(4 as i32 as isize)
                                        as i32
                                        == 'K' as i32
                                    && *((*(*ctxt).input).cur as *mut u8)
                                        .offset(5 as i32 as isize)
                                        as i32
                                        == 'E' as i32
                                    && *((*(*ctxt).input).cur as *mut u8)
                                        .offset(6 as i32 as isize)
                                        as i32
                                        == 'N' as i32
                                    && *((*(*ctxt).input).cur as *mut u8)
                                        .offset(7 as i32 as isize)
                                        as i32
                                        == 'S' as i32
                                {
                                    (*(*ctxt).input).cur =
                                        (*(*ctxt).input).cur.offset(8 as i32 as isize);
                                    (*(*ctxt).input).col += 8 as i32;
                                    if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                                        xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                                    }
                                    return XML_ATTRIBUTE_NMTOKENS as i32;
                                } else {
                                    if *((*(*ctxt).input).cur as *mut u8)
                                        .offset(0 as i32 as isize)
                                        as i32
                                        == 'N' as i32
                                        && *((*(*ctxt).input).cur as *mut u8)
                                            .offset(1 as i32 as isize)
                                            as i32
                                            == 'M' as i32
                                        && *((*(*ctxt).input).cur as *mut u8)
                                            .offset(2 as i32 as isize)
                                            as i32
                                            == 'T' as i32
                                        && *((*(*ctxt).input).cur as *mut u8)
                                            .offset(3 as i32 as isize)
                                            as i32
                                            == 'O' as i32
                                        && *((*(*ctxt).input).cur as *mut u8)
                                            .offset(4 as i32 as isize)
                                            as i32
                                            == 'K' as i32
                                        && *((*(*ctxt).input).cur as *mut u8)
                                            .offset(5 as i32 as isize)
                                            as i32
                                            == 'E' as i32
                                        && *((*(*ctxt).input).cur as *mut u8)
                                            .offset(6 as i32 as isize)
                                            as i32
                                            == 'N' as i32
                                    {
                                        (*(*ctxt).input).cur =
                                            (*(*ctxt).input).cur.offset(7 as i32 as isize);
                                        (*(*ctxt).input).col += 7 as i32;
                                        if *(*(*ctxt).input).cur as i32 == 0 as i32
                                        {
                                            xmlParserInputGrow_safe(
                                                (*ctxt).input,
                                                250 as i32,
                                            );
                                        }
                                        return XML_ATTRIBUTE_NMTOKEN as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return xmlParseEnumeratedType(ctxt, tree);
}
/* *
* xmlParseAttributeListDecl:
* @ctxt:  an XML parser context
*
* : parse the Attribute list def for an element
*
* [52] AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
*
* [53] AttDef ::= S Name S AttType S DefaultDecl
*
*/

pub unsafe fn xmlParseAttributeListDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut elemName: *const xmlChar = 0 as *const xmlChar;
    let mut attrName: *const xmlChar = 0 as *const xmlChar;
    let mut tree: xmlEnumerationPtr = 0 as *mut xmlEnumeration;
    //@todo 削减unsafe范围
    unsafe {
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '!' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'A' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'L' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'I' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'S' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(8 as i32 as isize)
                as i32
                == 'T' as i32
        {
            let mut inputid: i32 = (*(*ctxt).input).id;
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(9 as i32 as isize);
            (*(*ctxt).input).col += 9 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            if xmlSkipBlankChars(ctxt) == 0 as i32 {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after \'<!ATTLIST\'\n\x00" as *const u8 as *const i8,
                );
            }
            elemName = xmlParseName(ctxt);
            if elemName.is_null() {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_NAME_REQUIRED,
                    b"ATTLIST: no name for Element\n\x00" as *const u8 as *const i8,
                );
                return;
            }
            xmlSkipBlankChars(ctxt);
            if (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
            {
                xmlGROW(ctxt);
            }
            while 1<2
            {
                if (!(*(*(*ctxt).input).cur as i32 != '>' as i32
                && (*ctxt).instate as i32 != XML_PARSER_EOF as i32)){
                    break;
                }
                let mut type_0: i32 = 0;
                let mut def: i32 = 0;
                let mut defaultValue: *mut xmlChar = 0 as *mut xmlChar;
                if (*ctxt).progressive == 0 as i32
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                {
                    xmlGROW(ctxt);
                }
                tree = 0 as xmlEnumerationPtr;
                attrName = xmlParseName(ctxt);
                if attrName.is_null() {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_NAME_REQUIRED,
                        b"ATTLIST: no name for Attribute\n\x00" as *const u8 as *const i8,
                    );
                    break;
                } else {
                    if (*ctxt).progressive == 0 as i32
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                    {
                        xmlGROW(ctxt);
                    }
                    if xmlSkipBlankChars(ctxt) == 0 as i32 {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_SPACE_REQUIRED,
                            b"Space required after the attribute name\n\x00" as *const u8
                                as *const i8,
                        );
                        break;
                    } else {
                        type_0 = xmlParseAttributeType(ctxt, &mut tree);
                        if type_0 <= 0 as i32 {
                            break;
                        }
                        if (*ctxt).progressive == 0 as i32
                            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                as i64)
                                < 250 as i32 as i64
                        {
                            xmlGROW(ctxt);
                        }
                        if xmlSkipBlankChars(ctxt) == 0 as i32 {
                            xmlFatalErrMsg(
                                ctxt,
                                XML_ERR_SPACE_REQUIRED,
                                b"Space required after the attribute type\n\x00" as *const u8
                                    as *const i8,
                            );
                            if !tree.is_null() {
                                xmlFreeEnumeration_safe(tree);
                            }
                            break;
                        } else {
                            def = xmlParseDefaultDecl(ctxt, &mut defaultValue);
                            if def <= 0 as i32 {
                                if !defaultValue.is_null() {
                                    xmlFree_safe(defaultValue as *mut ());
                                }
                                if !tree.is_null() {
                                    xmlFreeEnumeration_safe(tree);
                                }
                                break;
                            } else {
                                if type_0 != XML_ATTRIBUTE_CDATA as i32
                                    && !defaultValue.is_null()
                                {
                                    xmlAttrNormalizeSpace(defaultValue, defaultValue);
                                }
                                if (*ctxt).progressive == 0 as i32
                                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                        as i64)
                                        < 250 as i32 as i64
                                {
                                    xmlGROW(ctxt);
                                }
                                if *(*(*ctxt).input).cur as i32 != '>' as i32 {
                                    if xmlSkipBlankChars(ctxt) == 0 as i32 {
                                        xmlFatalErrMsg(ctxt,
                                   XML_ERR_SPACE_REQUIRED,
                                   b"Space required after the attribute default value\n\x00"
                                       as *const u8 as
                                       *const i8);
                                        if !defaultValue.is_null() {
                                            xmlFree_safe(defaultValue as *mut ());
                                        }
                                        if !tree.is_null() {
                                            xmlFreeEnumeration_safe(tree);
                                        }
                                        break;
                                    }
                                }
                                if !(*ctxt).sax.is_null()
                                    && (*ctxt).disableSAX == 0
                                    && (*(*ctxt).sax).attributeDecl.is_some()
                                {
                                    (*(*ctxt).sax)
                                        .attributeDecl
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        elemName,
                                        attrName,
                                        type_0,
                                        def,
                                        defaultValue,
                                        tree,
                                    );
                                } else if !tree.is_null() {
                                    xmlFreeEnumeration_safe(tree);
                                }
                                if (*ctxt).sax2 != 0
                                    && !defaultValue.is_null()
                                    && def != XML_ATTRIBUTE_IMPLIED as i32
                                    && def != XML_ATTRIBUTE_REQUIRED as i32
                                {
                                    xmlAddDefAttrs(ctxt, elemName, attrName, defaultValue);
                                }
                                if (*ctxt).sax2 != 0 {
                                    xmlAddSpecialAttr(ctxt, elemName, attrName, type_0);
                                }
                                if !defaultValue.is_null() {
                                    xmlFree_safe(defaultValue as *mut ());
                                }
                                if (*ctxt).progressive == 0 as i32
                                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                        as i64)
                                        < 250 as i32 as i64
                                {
                                    xmlGROW(ctxt);
                                }
                            }
                        }
                    }
                }
            }
            if *(*(*ctxt).input).cur as i32 == '>' as i32 {
                if inputid != (*(*ctxt).input).id {
                    xmlFatalErrMsg(ctxt, XML_ERR_ENTITY_BOUNDARY,
               b"Attribute list declaration doesn\'t start and stop in the same entity\n\x00"
                   as *const u8 as *const i8);
                }
                xmlNextChar_safe(ctxt);
            }
        };
    }
}
/* *
* xmlParseElementMixedContentDecl:
* @ctxt:  an XML parser context
* @inputchk:  the input used for the current entity, needed for boundary checks
*
* parse the declaration for a Mixed Element content
* The leading '(' and spaces have been skipped in xmlParseElementContentDecl
*
* [51] Mixed ::= '(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' |
*                '(' S? '#PCDATA' S? ')'
*
* [ VC: Proper Group/PE Nesting ] applies to [51] too (see [49])
*
* [ VC: No Duplicate Types ]
* The same name must not appear more than once in a single
* mixed-content declaration.
*
* returns: the list of the xmlElementContentPtr describing the element choices
*/

pub unsafe fn xmlParseElementMixedContentDecl(
    mut ctxt: xmlParserCtxtPtr,
    mut inputchk: i32,
) -> xmlElementContentPtr {
    let mut ret: xmlElementContentPtr = 0 as xmlElementContentPtr;
    let mut cur: xmlElementContentPtr = 0 as xmlElementContentPtr;
    let mut n: xmlElementContentPtr = 0 as *mut xmlElementContent;
    let mut elem: *const xmlChar = 0 as *const xmlChar;
    //@todo 削减unsafe范围
    unsafe {
        if (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        {
            xmlGROW(ctxt);
        }
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '#' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'P' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'C' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'D' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'A' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'A' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(7 as i32 as isize);
            (*(*ctxt).input).col += 7 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            xmlSkipBlankChars(ctxt);
            if (*ctxt).progressive == 0 as i32
                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                    > (2 as i32 * 250 as i32) as i64
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < (2 as i32 * 250 as i32) as i64
            {
                xmlSHRINK(ctxt);
            }
            if *(*(*ctxt).input).cur as i32 == ')' as i32 {
                if (*(*ctxt).input).id != inputchk {
                    xmlFatalErrMsg(ctxt, XML_ERR_ENTITY_BOUNDARY,
               b"Element content declaration doesn\'t start and stop in the same entity\n\x00"
                   as *const u8 as *const i8);
                }
                xmlNextChar_safe(ctxt);
                ret = xmlNewDocElementContent_safe(
                    (*ctxt).myDoc,
                    0 as *const xmlChar,
                    XML_ELEMENT_CONTENT_PCDATA,
                );
                if ret.is_null() {
                    return 0 as xmlElementContentPtr;
                }
                if *(*(*ctxt).input).cur as i32 == '*' as i32 {
                    (*ret).ocur = XML_ELEMENT_CONTENT_MULT;
                    xmlNextChar_safe(ctxt);
                }
                return ret;
            }
            if *(*(*ctxt).input).cur as i32 == '(' as i32
                || *(*(*ctxt).input).cur as i32 == '|' as i32
            {
                cur = xmlNewDocElementContent_safe(
                    (*ctxt).myDoc,
                    0 as *const xmlChar,
                    XML_ELEMENT_CONTENT_PCDATA,
                );
                ret = cur;
                if ret.is_null() {
                    return 0 as xmlElementContentPtr;
                }
            }
            while 1<2
            {
                if (!(*(*(*ctxt).input).cur as i32 == '|' as i32
                && (*ctxt).instate as i32 != XML_PARSER_EOF as i32)){
                    break;
                }
                xmlNextChar_safe(ctxt);
                if elem.is_null() {
                    ret = xmlNewDocElementContent_safe(
                        (*ctxt).myDoc,
                        0 as *const xmlChar,
                        XML_ELEMENT_CONTENT_OR,
                    );
                    if ret.is_null() {
                        xmlFreeDocElementContent_safe((*ctxt).myDoc, cur);
                        return 0 as xmlElementContentPtr;
                    }
                    (*ret).c1 = cur;
                    if !cur.is_null() {
                        (*cur).parent = ret
                    }
                    cur = ret
                } else {
                    n = xmlNewDocElementContent_safe(
                        (*ctxt).myDoc,
                        0 as *const xmlChar,
                        XML_ELEMENT_CONTENT_OR,
                    );
                    if n.is_null() {
                        xmlFreeDocElementContent_safe((*ctxt).myDoc, ret);
                        return 0 as xmlElementContentPtr;
                    }
                    (*n).c1 = xmlNewDocElementContent_safe(
                        (*ctxt).myDoc,
                        elem,
                        XML_ELEMENT_CONTENT_ELEMENT,
                    );
                    if !(*n).c1.is_null() {
                        (*(*n).c1).parent = n
                    }
                    (*cur).c2 = n;
                    if !n.is_null() {
                        (*n).parent = cur
                    }
                    cur = n
                }
                xmlSkipBlankChars(ctxt);
                elem = xmlParseName(ctxt);
                if elem.is_null() {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_NAME_REQUIRED,
                        b"xmlParseElementMixedContentDecl : Name expected\n\x00" as *const u8
                            as *const i8,
                    );
                    xmlFreeDocElementContent_safe((*ctxt).myDoc, ret);
                    return 0 as xmlElementContentPtr;
                }
                xmlSkipBlankChars(ctxt);
                if (*ctxt).progressive == 0 as i32
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                {
                    xmlGROW(ctxt);
                }
            }
            if *(*(*ctxt).input).cur as i32 == ')' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == '*' as i32
            {
                if !elem.is_null() {
                    (*cur).c2 = xmlNewDocElementContent_safe(
                        (*ctxt).myDoc,
                        elem,
                        XML_ELEMENT_CONTENT_ELEMENT,
                    );
                    if !(*cur).c2.is_null() {
                        (*(*cur).c2).parent = cur
                    }
                }
                if !ret.is_null() {
                    (*ret).ocur = XML_ELEMENT_CONTENT_MULT
                }
                if (*(*ctxt).input).id != inputchk {
                    xmlFatalErrMsg(ctxt, XML_ERR_ENTITY_BOUNDARY,
               b"Element content declaration doesn\'t start and stop in the same entity\n\x00"
                   as *const u8 as *const i8);
                }
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
                (*(*ctxt).input).col += 2 as i32;
                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                }
            } else {
                xmlFreeDocElementContent_safe((*ctxt).myDoc, ret);
                xmlFatalErr(ctxt, XML_ERR_MIXED_NOT_STARTED, 0 as *const i8);
                return 0 as xmlElementContentPtr;
            }
        } else {
            xmlFatalErr(ctxt, XML_ERR_PCDATA_REQUIRED, 0 as *const i8);
        }
    }
    return ret;
}
/* *
* xmlParseElementChildrenContentDeclPriv:
* @ctxt:  an XML parser context
* @inputchk:  the input used for the current entity, needed for boundary checks
* @depth: the level of recursion
*
* parse the declaration for a Mixed Element content
* The leading '(' and spaces have been skipped in xmlParseElementContentDecl
*
*
* [47] children ::= (choice | seq) ('?' | '*' | '+')?
*
* [48] cp ::= (Name | choice | seq) ('?' | '*' | '+')?
*
* [49] choice ::= '(' S? cp ( S? '|' S? cp )* S? ')'
*
* [50] seq ::= '(' S? cp ( S? ',' S? cp )* S? ')'
*
* [ VC: Proper Group/PE Nesting ] applies to [49] and [50]
* TODO Parameter-entity replacement text must be properly nested
*	with parenthesized groups. That is to say, if either of the
*	opening or closing parentheses in a choice, seq, or Mixed
*	construct is contained in the replacement text for a parameter
*	entity, both must be contained in the same replacement text. For
*	interoperability, if a parameter-entity reference appears in a
*	choice, seq, or Mixed construct, its replacement text should not
*	be empty, and neither the first nor last non-blank character of
*	the replacement text should be a connector (| or ,).
*
* Returns the tree of xmlElementContentPtr describing the element
*          hierarchy.
*/
unsafe fn xmlParseElementChildrenContentDeclPriv(
    mut ctxt: xmlParserCtxtPtr,
    mut inputchk: i32,
    mut depth: i32,
) -> xmlElementContentPtr {
    let mut ret: xmlElementContentPtr = 0 as xmlElementContentPtr;
    let mut cur: xmlElementContentPtr = 0 as xmlElementContentPtr;
    let mut last: xmlElementContentPtr = 0 as xmlElementContentPtr;
    let mut op: xmlElementContentPtr = 0 as xmlElementContentPtr;
    let mut elem: *const xmlChar = 0 as *const xmlChar;
    let mut type_0: xmlChar = 0 as i32 as xmlChar;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if depth > 128 as i32
        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
        || depth > 2048 as i32
    {
        unsafe {
            xmlFatalErrMsgInt(
                ctxt,
                XML_ERR_ELEMCONTENT_NOT_FINISHED,
                b"xmlParseElementChildrenContentDecl : depth %d too deep, use XML_PARSE_HUGE\n\x00"
                    as *const u8 as *const i8,
                depth,
            );
        }
        return 0 as xmlElementContentPtr;
    }
    xmlSkipBlankChars(ctxt);
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    if unsafe { *(*(*ctxt).input).cur as i32 == '(' as i32 } {
        let mut inputid: i32 = unsafe { (*(*ctxt).input).id };
        /* Recurse on first child */
        xmlNextChar_safe(ctxt);
        xmlSkipBlankChars(ctxt);
        ret = xmlParseElementChildrenContentDeclPriv(ctxt, inputid, depth + 1 as i32);
        cur = ret;
        if cur.is_null() {
            return 0 as xmlElementContentPtr;
        }
        xmlSkipBlankChars(ctxt);
        if unsafe {
            (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
        } {
            xmlGROW(ctxt);
        }
    } else {
        elem = xmlParseName(ctxt);
        if elem.is_null() {
            unsafe {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_ELEMCONTENT_NOT_STARTED,
                    0 as *const i8,
                );
            }
            return 0 as xmlElementContentPtr;
        }
        ret = xmlNewDocElementContent_safe((safe_ctxt).myDoc, elem, XML_ELEMENT_CONTENT_ELEMENT);
        cur = ret;
        if cur.is_null() {
            unsafe {
                xmlErrMemory(ctxt, 0 as *const i8);
            }
            return 0 as xmlElementContentPtr;
        }
        if (safe_ctxt).progressive == 0 as i32
            && unsafe {
                ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
            }
        {
            xmlGROW(ctxt);
        }
        unsafe {
            if *(*(*ctxt).input).cur as i32 == '?' as i32 {
                (*cur).ocur = XML_ELEMENT_CONTENT_OPT;
                xmlNextChar_safe(ctxt);
            } else if *(*(*ctxt).input).cur as i32 == '*' as i32 {
                (*cur).ocur = XML_ELEMENT_CONTENT_MULT;
                xmlNextChar_safe(ctxt);
            } else if *(*(*ctxt).input).cur as i32 == '+' as i32 {
                (*cur).ocur = XML_ELEMENT_CONTENT_PLUS;
                xmlNextChar_safe(ctxt);
            } else {
                (*cur).ocur = XML_ELEMENT_CONTENT_ONCE
            }
        }
        if (safe_ctxt).progressive == 0 as i32
            && unsafe {
                ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
            }
        {
            xmlGROW(ctxt);
        }
    }
    xmlSkipBlankChars(ctxt);
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < (2 as i32 * 250 as i32) as i64
        }
    {
        xmlSHRINK(ctxt);
    }
    while  1<2 {
        if(!(unsafe {
            *(*(*ctxt).input).cur as i32 != ')' as i32
                && (*ctxt).instate as i32 != XML_PARSER_EOF as i32
        })){
            break;
        }
        /*
         * Each loop we parse one separator and one element.
         */
        if unsafe { *(*(*ctxt).input).cur as i32 == ',' as i32 } {
            if type_0 as i32 == 0 as i32 {
                unsafe { type_0 = *(*(*ctxt).input).cur };
            } else if unsafe { type_0 as i32 != *(*(*ctxt).input).cur as i32 } {
                unsafe {
                    xmlFatalErrMsgInt(
                        ctxt,
                        XML_ERR_SEPARATOR_REQUIRED,
                        b"xmlParseElementChildrenContentDecl : \'%c\' expected\n\x00" as *const u8
                            as *const i8,
                        type_0 as i32,
                    );
                }
                if !last.is_null() && last != ret {
                    xmlFreeDocElementContent_safe((safe_ctxt).myDoc, last);
                }
                if !ret.is_null() {
                    xmlFreeDocElementContent_safe((safe_ctxt).myDoc, ret);
                }
                return 0 as xmlElementContentPtr;
            }
            xmlNextChar_safe(ctxt);
            op = xmlNewDocElementContent_safe(
                (safe_ctxt).myDoc,
                0 as *const xmlChar,
                XML_ELEMENT_CONTENT_SEQ,
            );
            if op.is_null() {
                if !last.is_null() && last != ret {
                    xmlFreeDocElementContent_safe((safe_ctxt).myDoc, last);
                }
                xmlFreeDocElementContent_safe((safe_ctxt).myDoc, ret);
                return 0 as xmlElementContentPtr;
            }
            if last.is_null() {
                unsafe {
                    (*op).c1 = ret;
                    if !ret.is_null() {
                        (*ret).parent = op
                    }
                }
                cur = op;
                ret = cur
            } else {
                unsafe {
                    (*cur).c2 = op;
                    if !op.is_null() {
                        (*op).parent = cur
                    }
                    (*op).c1 = last;
                    if !last.is_null() {
                        (*last).parent = op
                    }
                }
                cur = op;
                last = 0 as xmlElementContentPtr
            }
        } else if unsafe { *(*(*ctxt).input).cur as i32 == '|' as i32 } {
            if type_0 as i32 == 0 as i32 {
                type_0 = unsafe { *(*(*ctxt).input).cur }
            } else if unsafe { type_0 as i32 != *(*(*ctxt).input).cur as i32 } {
                unsafe {
                    xmlFatalErrMsgInt(
                        ctxt,
                        XML_ERR_SEPARATOR_REQUIRED,
                        b"xmlParseElementChildrenContentDecl : \'%c\' expected\n\x00" as *const u8
                            as *const i8,
                        type_0 as i32,
                    );
                }
                if !last.is_null() && last != ret {
                    xmlFreeDocElementContent_safe((safe_ctxt).myDoc, last);
                }
                if !ret.is_null() {
                    xmlFreeDocElementContent_safe((safe_ctxt).myDoc, ret);
                }
                return 0 as xmlElementContentPtr;
            }
            xmlNextChar_safe(ctxt);
            op = xmlNewDocElementContent_safe(
                (safe_ctxt).myDoc,
                0 as *const xmlChar,
                XML_ELEMENT_CONTENT_OR,
            );
            if op.is_null() {
                if !last.is_null() && last != ret {
                    xmlFreeDocElementContent_safe((safe_ctxt).myDoc, last);
                }
                if !ret.is_null() {
                    xmlFreeDocElementContent_safe((safe_ctxt).myDoc, ret);
                }
                return 0 as xmlElementContentPtr;
            }
            if last.is_null() {
                unsafe {
                    (*op).c1 = ret;
                    if !ret.is_null() {
                        (*ret).parent = op
                    }
                }
                cur = op;
                ret = cur
            } else {
                unsafe {
                    (*cur).c2 = op;
                    if !op.is_null() {
                        (*op).parent = cur
                    }
                    (*op).c1 = last;
                    if !last.is_null() {
                        (*last).parent = op
                    }
                }
                cur = op;
                last = 0 as xmlElementContentPtr
            }
        } else {
            unsafe {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_ELEMCONTENT_NOT_FINISHED,
                    0 as *const i8,
                );
            }
            if !last.is_null() && last != ret {
                xmlFreeDocElementContent_safe((safe_ctxt).myDoc, last);
            }
            if !ret.is_null() {
                xmlFreeDocElementContent_safe((safe_ctxt).myDoc, ret);
            }
            return 0 as xmlElementContentPtr;
        }
        if unsafe {
            (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
        } {
            xmlGROW(ctxt);
        }
        xmlSkipBlankChars(ctxt);
        if unsafe {
            (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
        } {
            xmlGROW(ctxt);
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == '(' as i32 } {
            let mut inputid_0: i32 = unsafe { (*(*ctxt).input).id };
            /*
             * Detect "Name | Name , Name" error
             */
            /*
             * Detect "Name , Name | Name" error
             */
            /* Recurse on second child */
            xmlNextChar_safe(ctxt);
            xmlSkipBlankChars(ctxt);
            last =
                xmlParseElementChildrenContentDeclPriv(ctxt, inputid_0, depth + 1 as i32);
            if last.is_null() {
                if !ret.is_null() {
                    xmlFreeDocElementContent_safe((safe_ctxt).myDoc, ret);
                }
                return 0 as xmlElementContentPtr;
            }
            xmlSkipBlankChars(ctxt);
        } else {
            elem = xmlParseName(ctxt);
            if elem.is_null() {
                unsafe {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_ELEMCONTENT_NOT_STARTED,
                        0 as *const i8,
                    );
                }
                if !ret.is_null() {
                    xmlFreeDocElementContent_safe((safe_ctxt).myDoc, ret);
                }
                return 0 as xmlElementContentPtr;
            }
            last =
                xmlNewDocElementContent_safe((safe_ctxt).myDoc, elem, XML_ELEMENT_CONTENT_ELEMENT);
            if last.is_null() {
                if !ret.is_null() {
                    xmlFreeDocElementContent_safe((safe_ctxt).myDoc, ret);
                }
                return 0 as xmlElementContentPtr;
            }
            unsafe {
                if *(*(*ctxt).input).cur as i32 == '?' as i32 {
                    (*last).ocur = XML_ELEMENT_CONTENT_OPT;
                    xmlNextChar_safe(ctxt);
                } else if *(*(*ctxt).input).cur as i32 == '*' as i32 {
                    (*last).ocur = XML_ELEMENT_CONTENT_MULT;
                    xmlNextChar_safe(ctxt);
                } else if *(*(*ctxt).input).cur as i32 == '+' as i32 {
                    (*last).ocur = XML_ELEMENT_CONTENT_PLUS;
                    xmlNextChar_safe(ctxt);
                } else {
                    (*last).ocur = XML_ELEMENT_CONTENT_ONCE
                }
            }
        }
        xmlSkipBlankChars(ctxt);
        if unsafe {
            (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
        } {
            xmlGROW(ctxt);
        }
    }
    if !cur.is_null() && !last.is_null() {
        unsafe {
            (*cur).c2 = last;
            if !last.is_null() {
                (*last).parent = cur
            }
        }
    }
    unsafe {
        if (*(*ctxt).input).id != inputchk {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_ENTITY_BOUNDARY,
                b"Element content declaration doesn\'t start and stop in the same entity\n\x00"
                    as *const u8 as *const i8,
            );
        }
    }
    xmlNextChar_safe(ctxt);
    unsafe {
        if *(*(*ctxt).input).cur as i32 == '?' as i32 {
            if !ret.is_null() {
                if (*ret).ocur as u32
                    == XML_ELEMENT_CONTENT_PLUS as i32 as u32
                    || (*ret).ocur as u32
                        == XML_ELEMENT_CONTENT_MULT as i32 as u32
                {
                    (*ret).ocur = XML_ELEMENT_CONTENT_MULT
                } else {
                    (*ret).ocur = XML_ELEMENT_CONTENT_OPT
                }
            }
            xmlNextChar_safe(ctxt);
        } else if *(*(*ctxt).input).cur as i32 == '*' as i32 {
            if !ret.is_null() {
                (*ret).ocur = XML_ELEMENT_CONTENT_MULT;
                cur = ret;
                /*
                 * Some normalization:
                 * (a | b* | c?)* == (a | b | c)*
                 */
                while !cur.is_null()
                    && (*cur).type_0 as u32
                        == XML_ELEMENT_CONTENT_OR as i32 as u32
                {
                    if !(*cur).c1.is_null()
                        && ((*(*cur).c1).ocur as u32
                            == XML_ELEMENT_CONTENT_OPT as i32 as u32
                            || (*(*cur).c1).ocur as u32
                                == XML_ELEMENT_CONTENT_MULT as i32 as u32)
                    {
                        (*(*cur).c1).ocur = XML_ELEMENT_CONTENT_ONCE
                    }
                    if !(*cur).c2.is_null()
                        && ((*(*cur).c2).ocur as u32
                            == XML_ELEMENT_CONTENT_OPT as i32 as u32
                            || (*(*cur).c2).ocur as u32
                                == XML_ELEMENT_CONTENT_MULT as i32 as u32)
                    {
                        (*(*cur).c2).ocur = XML_ELEMENT_CONTENT_ONCE
                    }
                    cur = (*cur).c2
                }
            }
            xmlNextChar_safe(ctxt);
        } else if *(*(*ctxt).input).cur as i32 == '+' as i32 {
            if !ret.is_null() {
                let mut found: i32 = 0 as i32;
                if (*ret).ocur as u32
                    == XML_ELEMENT_CONTENT_OPT as i32 as u32
                    || (*ret).ocur as u32
                        == XML_ELEMENT_CONTENT_MULT as i32 as u32
                {
                    (*ret).ocur = XML_ELEMENT_CONTENT_MULT
                } else {
                    (*ret).ocur = XML_ELEMENT_CONTENT_PLUS
                }
                /*
                 * Some normalization:
                 * (a | b*)+ == (a | b)*
                 * (a | b?)+ == (a | b)*
                 */
                while !cur.is_null()
                    && (*cur).type_0 as u32
                        == XML_ELEMENT_CONTENT_OR as i32 as u32
                {
                    if !(*cur).c1.is_null()
                        && ((*(*cur).c1).ocur as u32
                            == XML_ELEMENT_CONTENT_OPT as i32 as u32
                            || (*(*cur).c1).ocur as u32
                                == XML_ELEMENT_CONTENT_MULT as i32 as u32)
                    {
                        (*(*cur).c1).ocur = XML_ELEMENT_CONTENT_ONCE;
                        found = 1 as i32
                    }
                    if !(*cur).c2.is_null()
                        && ((*(*cur).c2).ocur as u32
                            == XML_ELEMENT_CONTENT_OPT as i32 as u32
                            || (*(*cur).c2).ocur as u32
                                == XML_ELEMENT_CONTENT_MULT as i32 as u32)
                    {
                        (*(*cur).c2).ocur = XML_ELEMENT_CONTENT_ONCE;
                        found = 1 as i32
                    }
                    cur = (*cur).c2
                }
                if found != 0 {
                    (*ret).ocur = XML_ELEMENT_CONTENT_MULT
                }
            }
            xmlNextChar_safe(ctxt);
        }
    }
    return ret;
}
/* *
* xmlParseElementChildrenContentDecl:
* @ctxt:  an XML parser context
* @inputchk:  the input used for the current entity, needed for boundary checks
*
* parse the declaration for a Mixed Element content
* The leading '(' and spaces have been skipped in xmlParseElementContentDecl
*
* [47] children ::= (choice | seq) ('?' | '*' | '+')?
*
* [48] cp ::= (Name | choice | seq) ('?' | '*' | '+')?
*
* [49] choice ::= '(' S? cp ( S? '|' S? cp )* S? ')'
*
* [50] seq ::= '(' S? cp ( S? ',' S? cp )* S? ')'
*
* [ VC: Proper Group/PE Nesting ] applies to [49] and [50]
* TODO Parameter-entity replacement text must be properly nested
*	with parenthesized groups. That is to say, if either of the
*	opening or closing parentheses in a choice, seq, or Mixed
*	construct is contained in the replacement text for a parameter
*	entity, both must be contained in the same replacement text. For
*	interoperability, if a parameter-entity reference appears in a
*	choice, seq, or Mixed construct, its replacement text should not
*	be empty, and neither the first nor last non-blank character of
*	the replacement text should be a connector (| or ,).
*
* Returns the tree of xmlElementContentPtr describing the element
*          hierarchy.
*/

pub unsafe fn xmlParseElementChildrenContentDecl(
    mut ctxt: xmlParserCtxtPtr,
    mut inputchk: i32,
) -> xmlElementContentPtr {
    /* stub left for API/ABI compat */
    return xmlParseElementChildrenContentDeclPriv(ctxt, inputchk, 1 as i32);
}
/* *
* xmlParseElementContentDecl:
* @ctxt:  an XML parser context
* @name:  the name of the element being defined.
* @result:  the Element Content pointer will be stored here if any
*
* parse the declaration for an Element content either Mixed or Children,
* the cases EMPTY and ANY are handled directly in xmlParseElementDecl
*
* [46] contentspec ::= 'EMPTY' | 'ANY' | Mixed | children
*
* returns: the type of element content XML_ELEMENT_TYPE_xxx
*/

pub unsafe fn xmlParseElementContentDecl(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const xmlChar,
    mut result: *mut xmlElementContentPtr,
) -> i32 {
    let mut tree: xmlElementContentPtr = 0 as xmlElementContentPtr;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut inputid: i32 = unsafe { (*(*ctxt).input).id };
    let mut res: i32 = 0;
    unsafe {
        *result = 0 as xmlElementContentPtr;
        if *(*(*ctxt).input).cur as i32 != '(' as i32 {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_ELEMCONTENT_NOT_STARTED,
                b"xmlParseElementContentDecl : %s \'(\' expected\n\x00" as *const u8
                    as *const i8,
                name,
            );
            return -(1 as i32);
        }
    }
    xmlNextChar_safe(ctxt);
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return -(1 as i32);
    }
    xmlSkipBlankChars(ctxt);
    if unsafe {
        *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '#' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'P' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'C' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'D' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'A' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'A' as i32
    } {
        tree = xmlParseElementMixedContentDecl(ctxt, inputid);
        res = XML_ELEMENT_TYPE_MIXED as i32
    } else {
        tree = xmlParseElementChildrenContentDeclPriv(ctxt, inputid, 1 as i32);
        res = XML_ELEMENT_TYPE_ELEMENT as i32
    }
    xmlSkipBlankChars(ctxt);
    unsafe { *result = tree };
    return res;
}
/* *
* xmlParseElementDecl:
* @ctxt:  an XML parser context
*
* parse an Element declaration.
*
* [45] elementdecl ::= '<!ELEMENT' S Name S contentspec S? '>'
*
* [ VC: Unique Element Type Declaration ]
* No element type may be declared more than once
*
* Returns the type of the element, or -1 in case of error
*/

pub unsafe fn xmlParseElementDecl(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut ret: i32 = -(1 as i32);
    let mut content: xmlElementContentPtr = 0 as xmlElementContentPtr;
    /* GROW; done in the caller */
    //@todo 削减unsafe范围
    unsafe {
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '!' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'E' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'L' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'E' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'M' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'E' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'N' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(8 as i32 as isize)
                as i32
                == 'T' as i32
        {
            let mut inputid: i32 = (*(*ctxt).input).id;
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(9 as i32 as isize);
            (*(*ctxt).input).col += 9 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            if xmlSkipBlankChars(ctxt) == 0 as i32 {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after \'ELEMENT\'\n\x00" as *const u8 as *const i8,
                );
                return -(1 as i32);
            }
            name = xmlParseName(ctxt);
            if name.is_null() {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_NAME_REQUIRED,
                    b"xmlParseElementDecl: no name for Element\n\x00" as *const u8
                        as *const i8,
                );
                return -(1 as i32);
            }
            if xmlSkipBlankChars(ctxt) == 0 as i32 {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after the element name\n\x00" as *const u8
                        as *const i8,
                );
            }
            if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
                as i32
                == 'E' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                    as i32
                    == 'M' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                    as i32
                    == 'P' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                    as i32
                    == 'T' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                    as i32
                    == 'Y' as i32
            {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(5 as i32 as isize);
                (*(*ctxt).input).col += 5 as i32;
                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                }
                /*
                 * Element must always be empty.
                 */
                ret = XML_ELEMENT_TYPE_EMPTY as i32
            } else if *(*(*ctxt).input).cur as i32 == 'A' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == 'N' as i32
                && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                    == 'Y' as i32
            {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(3 as i32 as isize);
                (*(*ctxt).input).col += 3 as i32;
                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                }
                /*
                 * Element is a generic container.
                 */
                ret = XML_ELEMENT_TYPE_ANY as i32
            } else if *(*(*ctxt).input).cur as i32 == '(' as i32 {
                ret = xmlParseElementContentDecl(ctxt, name, &mut content)
            } else {
                /*
                 * [ WFC: PEs in Internal Subset ] error handling.
                 */
                if *(*(*ctxt).input).cur as i32 == '%' as i32
                    && (*ctxt).external == 0 as i32
                    && (*ctxt).inputNr == 1 as i32
                {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_PEREF_IN_INT_SUBSET,
                        b"PEReference: forbidden within markup decl in internal subset\n\x00"
                            as *const u8 as *const i8,
                    );
                } else {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ELEMCONTENT_NOT_STARTED,
                        b"xmlParseElementDecl: \'EMPTY\', \'ANY\' or \'(\' expected\n\x00"
                            as *const u8 as *const i8,
                    );
                }
                return -(1 as i32);
            }
            xmlSkipBlankChars(ctxt);
            if *(*(*ctxt).input).cur as i32 != '>' as i32 {
                xmlFatalErr(ctxt, XML_ERR_GT_REQUIRED, 0 as *const i8);
                if !content.is_null() {
                    xmlFreeDocElementContent_safe((*ctxt).myDoc, content);
                }
            } else {
                if inputid != (*(*ctxt).input).id {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ENTITY_BOUNDARY,
                        b"Element declaration doesn\'t start and stop in the same entity\n\x00"
                            as *const u8 as *const i8,
                    );
                }
                xmlNextChar_safe(ctxt);
                if !(*ctxt).sax.is_null()
                    && (*ctxt).disableSAX == 0
                    && (*(*ctxt).sax).elementDecl.is_some()
                {
                    if !content.is_null() {
                        (*content).parent = 0 as *mut _xmlElementContent
                    }
                    (*(*ctxt).sax)
                        .elementDecl
                        .expect("non-null function pointer")(
                        (*ctxt).userData, name, ret, content
                    );
                    if !content.is_null() && (*content).parent.is_null() {
                        /*
                         * this is a trick: if xmlAddElementDecl is called,
                         * instead of copying the full tree it is plugged directly
                         * if called from the parser. Avoid duplicating the
                         * interfaces or change the API/ABI
                         */
                        xmlFreeDocElementContent_safe((*ctxt).myDoc, content);
                    }
                } else if !content.is_null() {
                    xmlFreeDocElementContent_safe((*ctxt).myDoc, content);
                }
            }
        }
    }
    return ret;
}
/* *
* xmlParseConditionalSections
* @ctxt:  an XML parser context
*
* [61] conditionalSect ::= includeSect | ignoreSect
* [62] includeSect ::= '<![' S? 'INCLUDE' S? '[' extSubsetDecl ']]>'
* [63] ignoreSect ::= '<![' S? 'IGNORE' S? '[' ignoreSectContents* ']]>'
* [64] ignoreSectContents ::= Ignore ('<![' ignoreSectContents ']]>' Ignore)*
* [65] Ignore ::= Char* - (Char* ('<![' | ']]>') Char*)
*/
unsafe fn xmlParseConditionalSections(mut ctxt: xmlParserCtxtPtr) {
    let mut inputIds: *mut i32 = 0 as *mut i32;
    let mut inputIdsSize: size_t = 0 as i32 as size_t;
    let mut depth: size_t = 0 as i32 as size_t;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    's_11: while  1<2 {
        if (!((safe_ctxt).instate as i32 != XML_PARSER_EOF as i32)){
            break;
        }
        unsafe {
            if *(*(*ctxt).input).cur as i32 == '<' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == '!' as i32
                && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                    == '[' as i32
            {
                let mut id: i32 = (*(*ctxt).input).id;
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(3 as i32 as isize);
                (*(*ctxt).input).col += 3 as i32;
                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                }
                xmlSkipBlankChars(ctxt);
                if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
                    as i32
                    == 'I' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(1 as i32 as isize) as i32
                        == 'N' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(2 as i32 as isize) as i32
                        == 'C' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(3 as i32 as isize) as i32
                        == 'L' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(4 as i32 as isize) as i32
                        == 'U' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(5 as i32 as isize) as i32
                        == 'D' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(6 as i32 as isize) as i32
                        == 'E' as i32
                {
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(7 as i32 as isize);
                    (*(*ctxt).input).col += 7 as i32;
                    if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                        xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                    }
                    xmlSkipBlankChars(ctxt);
                    if *(*(*ctxt).input).cur as i32 != '[' as i32 {
                        xmlFatalErr(ctxt, XML_ERR_CONDSEC_INVALID, 0 as *const i8);
                        xmlHaltParser(ctxt);
                        break;
                    } else {
                        if (*(*ctxt).input).id != id {
                            xmlFatalErrMsg(ctxt, XML_ERR_ENTITY_BOUNDARY,
                       b"All markup of the conditional section is not in the same entity\n\x00"
                           as *const u8 as
                           *const i8);
                        }
                        xmlNextChar_safe(ctxt);
                        if inputIdsSize <= depth {
                            let mut tmp: *mut i32 = 0 as *mut i32;
                            inputIdsSize = if inputIdsSize == 0 as i32 as u64 {
                                4 as i32 as u64
                            } else {
                                inputIdsSize.wrapping_mul(2 as i32 as u64)
                            };
                            tmp = xmlRealloc_safe(
                                inputIds as *mut (),
                                inputIdsSize.wrapping_mul(
                                    ::std::mem::size_of::<i32>() as u64
                                ),
                            ) as *mut i32;
                            if tmp.is_null() {
                                xmlErrMemory(ctxt, 0 as *const i8);
                                break;
                            } else {
                                inputIds = tmp
                            }
                        }
                        *inputIds.offset(depth as isize) = id;
                        depth = depth.wrapping_add(1)
                    }
                } else if *((*(*ctxt).input).cur as *mut u8)
                    .offset(0 as i32 as isize) as i32
                    == 'I' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(1 as i32 as isize) as i32
                        == 'G' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(2 as i32 as isize) as i32
                        == 'N' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(3 as i32 as isize) as i32
                        == 'O' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(4 as i32 as isize) as i32
                        == 'R' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(5 as i32 as isize) as i32
                        == 'E' as i32
                {
                    let mut state: i32 = 0;
                    let mut instate: xmlParserInputState = XML_PARSER_START;
                    let mut ignoreDepth: size_t = 0 as i32 as size_t;
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(6 as i32 as isize);
                    (*(*ctxt).input).col += 6 as i32;
                    if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                        xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                    }
                    xmlSkipBlankChars(ctxt);
                    if *(*(*ctxt).input).cur as i32 != '[' as i32 {
                        xmlFatalErr(ctxt, XML_ERR_CONDSEC_INVALID, 0 as *const i8);
                        xmlHaltParser(ctxt);
                        break;
                    } else {
                        if (*(*ctxt).input).id != id {
                            xmlFatalErrMsg(ctxt, XML_ERR_ENTITY_BOUNDARY,
                       b"All markup of the conditional section is not in the same entity\n\x00"
                           as *const u8 as
                           *const i8);
                        }
                        xmlNextChar_safe(ctxt);
                        /*
                         * Parse up to the end of the conditional section but disable
                         * SAX event generating DTD building in the meantime
                         */
                        state = (*ctxt).disableSAX;
                        instate = (*ctxt).instate;
                        if (*ctxt).recovery == 0 as i32 {
                            (*ctxt).disableSAX = 1 as i32
                        }
                        (*ctxt).instate = XML_PARSER_IGNORE;
                        while *(*(*ctxt).input).cur as i32 != 0 as i32 {
                            if *(*(*ctxt).input).cur as i32 == '<' as i32
                                && *(*(*ctxt).input).cur.offset(1 as i32 as isize)
                                    as i32
                                    == '!' as i32
                                && *(*(*ctxt).input).cur.offset(2 as i32 as isize)
                                    as i32
                                    == '[' as i32
                            {
                                (*(*ctxt).input).cur =
                                    (*(*ctxt).input).cur.offset(3 as i32 as isize);
                                (*(*ctxt).input).col += 3 as i32;
                                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                                }
                                ignoreDepth = ignoreDepth.wrapping_add(1);
                                /* Check for integer overflow */
                                if !(ignoreDepth == 0 as i32 as u64) {
                                    continue;
                                }
                                xmlErrMemory(ctxt, 0 as *const i8);
                                break 's_11;
                            } else if *(*(*ctxt).input).cur as i32 == ']' as i32
                                && *(*(*ctxt).input).cur.offset(1 as i32 as isize)
                                    as i32
                                    == ']' as i32
                                && *(*(*ctxt).input).cur.offset(2 as i32 as isize)
                                    as i32
                                    == '>' as i32
                            {
                                if ignoreDepth == 0 as i32 as u64 {
                                    break;
                                }
                                (*(*ctxt).input).cur =
                                    (*(*ctxt).input).cur.offset(3 as i32 as isize);
                                (*(*ctxt).input).col += 3 as i32;
                                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                                }
                                ignoreDepth = ignoreDepth.wrapping_sub(1)
                            } else {
                                xmlNextChar_safe(ctxt);
                            }
                        }
                        (*ctxt).disableSAX = state;
                        (*ctxt).instate = instate;
                        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                            xmlFatalErr(
                                ctxt,
                                XML_ERR_CONDSEC_NOT_FINISHED,
                                0 as *const i8,
                            );
                            break;
                        } else {
                            if (*(*ctxt).input).id != id {
                                xmlFatalErrMsg(ctxt, XML_ERR_ENTITY_BOUNDARY,
                           b"All markup of the conditional section is not in the same entity\n\x00"
                               as *const u8 as
                               *const i8);
                            }
                            (*(*ctxt).input).cur =
                                (*(*ctxt).input).cur.offset(3 as i32 as isize);
                            (*(*ctxt).input).col += 3 as i32;
                            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                            }
                        }
                    }
                } else {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_CONDSEC_INVALID_KEYWORD,
                        0 as *const i8,
                    );
                    xmlHaltParser(ctxt);
                    break;
                }
            } else if depth > 0 as i32 as u64
                && *(*(*ctxt).input).cur as i32 == ']' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == ']' as i32
                && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                    == '>' as i32
            {
                depth = depth.wrapping_sub(1);
                if (*(*ctxt).input).id != *inputIds.offset(depth as isize) {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ENTITY_BOUNDARY,
                        b"All markup of the conditional section is not in the same entity\n\x00"
                            as *const u8 as *const i8,
                    );
                }
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(3 as i32 as isize);
                (*(*ctxt).input).col += 3 as i32;
                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                }
            } else {
                let mut check: *const xmlChar = (*(*ctxt).input).cur;
                let mut cons: u32 = (*(*ctxt).input).consumed as u32;
                xmlParseMarkupDecl(ctxt);
                if (*(*ctxt).input).cur == check
                    && cons as u64 == (*(*ctxt).input).consumed
                {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_EXT_SUBSET_NOT_FINISHED,
                        0 as *const i8,
                    );
                    xmlHaltParser(ctxt);
                    break;
                }
            }
            if depth == 0 as i32 as u64 {
                break;
            }
            xmlSkipBlankChars(ctxt);
            if (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
            {
                xmlGROW(ctxt);
            }
        }
    }
    xmlFree_safe(inputIds as *mut ());
}
/* *
* xmlParseMarkupDecl:
* @ctxt:  an XML parser context
*
* parse Markup declarations
*
* [29] markupdecl ::= elementdecl | AttlistDecl | EntityDecl |
*                     NotationDecl | PI | Comment
*
* [ VC: Proper Declaration/PE Nesting ]
* Parameter-entity replacement text must be properly nested with
* markup declarations. That is to say, if either the first character
* or the last character of a markup declaration (markupdecl above) is
* contained in the replacement text for a parameter-entity reference,
* both must be contained in the same replacement text.
*
* [ WFC: PEs in Internal Subset ]
* In the internal DTD subset, parameter-entity references can occur
* only where markup declarations can occur, not within markup declarations.
* (This does not apply to references that occur in external parameter
* entities or to the external subset.)
*/

pub unsafe fn xmlParseMarkupDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    //@todo 削减unsafe范围
    unsafe {
        if *(*(*ctxt).input).cur as i32 == '<' as i32 {
            if *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '!' as i32
            {
                match *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32 {
                    69 => {
                        if *(*(*ctxt).input).cur.offset(3 as i32 as isize) as i32
                            == 'L' as i32
                        {
                            xmlParseElementDecl(ctxt);
                        } else if *(*(*ctxt).input).cur.offset(3 as i32 as isize)
                            as i32
                            == 'N' as i32
                        {
                            xmlParseEntityDecl(ctxt);
                        }
                    }
                    65 => {
                        xmlParseAttributeListDecl(ctxt);
                    }
                    78 => {
                        xmlParseNotationDecl(ctxt);
                    }
                    45 => {
                        xmlParseComment(ctxt);
                    }
                    _ => {}
                }
            } else if *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                == '?' as i32
            {
                xmlParsePI(ctxt);
            }
        }
    }
    /*
     * detect requirement to exit there and act accordingly
     * and avoid having instate overridden later on
     */
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return;
    }
    (safe_ctxt).instate = XML_PARSER_DTD;
}
/* *
* xmlParseTextDecl:
* @ctxt:  an XML parser context
*
* parse an XML declaration header for external entities
*
* [77] TextDecl ::= '<?xml' VersionInfo? EncodingDecl S? '?>'
*/

pub unsafe fn xmlParseTextDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut version: *mut xmlChar = 0 as *mut xmlChar;
    let mut encoding: *const xmlChar = 0 as *const xmlChar;
    let mut oldstate: i32 = 0;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    /*
     * We know that '<?xml' is here.
     */
    unsafe {
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '?' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'x' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'm' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'l' as i32
            && (*(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                == 0x20 as i32
                || 0x9 as i32
                    <= *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                    && *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                        <= 0xa as i32
                || *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                    == 0xd as i32)
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(5 as i32 as isize);
            (*(*ctxt).input).col += 5 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
        } else {
            xmlFatalErr(ctxt, XML_ERR_XMLDECL_NOT_STARTED, 0 as *const i8);
            return;
        }
    }
    /* Avoid expansion of parameter entities when skipping blanks. */
    oldstate = (safe_ctxt).instate as i32;
    (safe_ctxt).instate = XML_PARSER_START;
    if xmlSkipBlankChars(ctxt) == 0 as i32 {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space needed after \'<?xml\'\n\x00" as *const u8 as *const i8,
            );
        }
    }
    /*
     * We may have the VersionInfo here.
     */
    unsafe {
        version = xmlParseVersionInfo(ctxt);
    }
    if version.is_null() {
        version = xmlCharStrdup_safe(b"1.0\x00" as *const u8 as *const i8)
    } else if xmlSkipBlankChars(ctxt) == 0 as i32 {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space needed here\n\x00" as *const u8 as *const i8,
            );
        }
    }
    unsafe {
        (*(*ctxt).input).version = version;
    }
    /*
     * We must have the encoding declaration
     */
    unsafe {
        encoding = xmlParseEncodingDecl(ctxt);
    }
    if (safe_ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as i32 {
        /*
         * The XML REC instructs us to stop parsing right here
         */
        (safe_ctxt).instate = oldstate as xmlParserInputState;
        return;
    }
    if encoding.is_null() && (safe_ctxt).errNo == XML_ERR_OK as i32 {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_MISSING_ENCODING,
                b"Missing encoding in text declaration\n\x00" as *const u8 as *const i8,
            );
        }
    }
    xmlSkipBlankChars(ctxt);
    unsafe {
        if *(*(*ctxt).input).cur as i32 == '?' as i32
            && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '>' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
            (*(*ctxt).input).col += 2 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
        } else if *(*(*ctxt).input).cur as i32 == '>' as i32 {
            /* Deprecated old WD ... */
            xmlFatalErr(ctxt, XML_ERR_XMLDECL_NOT_FINISHED, 0 as *const i8);
            xmlNextChar_safe(ctxt);
        } else {
            xmlFatalErr(ctxt, XML_ERR_XMLDECL_NOT_FINISHED, 0 as *const i8);
            while *(*(*ctxt).input).cur as i32 != 0
                && *(*(*ctxt).input).cur as i32 != '>' as i32
            {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
            }
            xmlNextChar_safe(ctxt);
        }
    }
    (safe_ctxt).instate = oldstate as xmlParserInputState;
}
/* *
* xmlParseExternalSubset:
* @ctxt:  an XML parser context
* @ExternalID: the external identifier
* @SystemID: the system identifier (or URL)
*
* parse Markup declarations from an external subset
*
* [30] extSubset ::= textDecl? extSubsetDecl
*
* [31] extSubsetDecl ::= (markupdecl | conditionalSect | PEReference | S) *
*/

pub unsafe fn xmlParseExternalSubset(
    mut ctxt: xmlParserCtxtPtr,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    xmlDetectSAX2(ctxt);
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    if (safe_ctxt).encoding.is_null()
        && unsafe {
            (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64
                >= 4 as i32 as i64
        }
    {
        let mut start: [xmlChar; 4] = [0; 4];
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        unsafe {
            start[0 as i32 as usize] = *(*(*ctxt).input).cur;
            start[1 as i32 as usize] =
                *(*(*ctxt).input).cur.offset(1 as i32 as isize);
            start[2 as i32 as usize] =
                *(*(*ctxt).input).cur.offset(2 as i32 as isize);
            start[3 as i32 as usize] =
                *(*(*ctxt).input).cur.offset(3 as i32 as isize);
        }
        enc = xmlDetectCharEncoding_safe(start.as_mut_ptr(), 4 as i32);
        if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
            xmlSwitchEncoding_safe(ctxt, enc);
        }
    }
    if unsafe {
        *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '?' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'x' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'm' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'l' as i32
    } {
        xmlParseTextDecl(ctxt);
        if (safe_ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as i32 {
            /*
             * The XML REC instructs us to stop parsing right here
             */
            unsafe {
                xmlHaltParser(ctxt);
            }
            return;
        }
    }
    if (safe_ctxt).myDoc.is_null() {
        (safe_ctxt).myDoc =
            xmlNewDoc_safe(b"1.0\x00" as *const u8 as *const i8 as *mut xmlChar);
        if (safe_ctxt).myDoc.is_null() {
            unsafe {
                xmlErrMemory(
                    ctxt,
                    b"New Doc failed\x00" as *const u8 as *const i8,
                );
            }
            return;
        }
        unsafe { (*(*ctxt).myDoc).properties = XML_DOC_INTERNAL as i32 }
    }
    if !(safe_ctxt).myDoc.is_null() && unsafe { (*(*ctxt).myDoc).intSubset.is_null() } {
        xmlCreateIntSubset_safe((safe_ctxt).myDoc, 0 as *const xmlChar, ExternalID, SystemID);
    }
    (safe_ctxt).instate = XML_PARSER_DTD;
    (safe_ctxt).external = 1 as i32;
    xmlSkipBlankChars(ctxt);
    //@todo 削减unsafe范围
    unsafe {
        while *(*(*ctxt).input).cur as i32 == '<' as i32
            && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '?' as i32
            || *(*(*ctxt).input).cur as i32 == '<' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == '!' as i32
            || *(*(*ctxt).input).cur as i32 == '%' as i32
        {
            let mut check: *const xmlChar = (*(*ctxt).input).cur;
            let mut cons: u32 = (*(*ctxt).input).consumed as u32;
            if (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
            {
                xmlGROW(ctxt);
            }
            if *(*(*ctxt).input).cur as i32 == '<' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == '!' as i32
                && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                    == '[' as i32
            {
                xmlParseConditionalSections(ctxt);
            } else {
                xmlParseMarkupDecl(ctxt);
            }
            xmlSkipBlankChars(ctxt);
            if !((*(*ctxt).input).cur == check
                && cons as u64 == (*(*ctxt).input).consumed)
            {
                continue;
            }
            xmlFatalErr(
                ctxt,
                XML_ERR_EXT_SUBSET_NOT_FINISHED,
                0 as *const i8,
            );
            break;
        }
        if *(*(*ctxt).input).cur as i32 != 0 as i32 {
            xmlFatalErr(
                ctxt,
                XML_ERR_EXT_SUBSET_NOT_FINISHED,
                0 as *const i8,
            );
        };
    }
}
/* *
* xmlParseReference:
* @ctxt:  an XML parser context
*
* parse and handle entity references in content, depending on the SAX
* interface, this may end-up in a call to character() if this is a
* CharRef, a predefined entity, if there is no reference() callback.
* or if the parser was asked to switch to that mode.
*
* [67] Reference ::= EntityRef | CharRef
*/

pub unsafe fn xmlParseReference(mut ctxt: xmlParserCtxtPtr) {
    let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    let mut was_checked: i32 = 0;
    let mut list: xmlNodePtr = 0 as xmlNodePtr;
    let mut ret: xmlParserErrors = XML_ERR_OK;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if unsafe { *(*(*ctxt).input).cur as i32 != '&' as i32 } {
        return;
    }
    /*
     * Simple case of a CharRef
     */
    if unsafe {
        *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '#' as i32
    } {
        let mut i: i32 = 0 as i32;
        let mut out: [xmlChar; 16] = [0; 16];
        let mut hex: i32 =
            unsafe { *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32 };
        let mut value: i32 = xmlParseCharRef(ctxt);
        if value == 0 as i32 {
            return;
        }
        if (safe_ctxt).charset != XML_CHAR_ENCODING_UTF8 as i32 {
            /*
             * So we are using non-UTF-8 buffers
             * Check that the char fit on 8bits, if not
             * generate a CharRef.
             */
            if value <= 0xff as i32 {
                out[0 as i32 as usize] = value as xmlChar;
                out[1 as i32 as usize] = 0 as i32 as xmlChar;
                unsafe {
                    if !(*ctxt).sax.is_null()
                        && (*(*ctxt).sax).characters.is_some()
                        && (*ctxt).disableSAX == 0
                    {
                        (*(*ctxt).sax)
                            .characters
                            .expect("non-null function pointer")(
                            (*ctxt).userData,
                            out.as_mut_ptr(),
                            1 as i32,
                        );
                    }
                }
            } else {
                unsafe {
                    if hex == 'x' as i32 || hex == 'X' as i32 {
                        snprintf(
                            out.as_mut_ptr() as *mut i8,
                            ::std::mem::size_of::<[xmlChar; 16]>() as u64,
                            b"#x%X\x00" as *const u8 as *const i8,
                            value,
                        );
                    } else {
                        snprintf(
                            out.as_mut_ptr() as *mut i8,
                            ::std::mem::size_of::<[xmlChar; 16]>() as u64,
                            b"#%d\x00" as *const u8 as *const i8,
                            value,
                        );
                    }
                    if !(*ctxt).sax.is_null()
                        && (*(*ctxt).sax).reference.is_some()
                        && (*ctxt).disableSAX == 0
                    {
                        (*(*ctxt).sax).reference.expect("non-null function pointer")(
                            (*ctxt).userData,
                            out.as_mut_ptr(),
                        );
                    }
                }
            }
        } else {
            /*
             * Just encode the value in UTF-8
             */
            if 0 as i32 == 1 as i32 {
                let fresh87 = i;
                i = i + 1;
                out[fresh87 as usize] = value as xmlChar
            } else {
                i += unsafe {
                    xmlCopyCharMultiByte(&mut *out.as_mut_ptr().offset(i as isize), value)
                };
            }
            out[i as usize] = 0 as i32 as xmlChar;
            unsafe {
                if !(*ctxt).sax.is_null()
                    && (*(*ctxt).sax).characters.is_some()
                    && (*ctxt).disableSAX == 0
                {
                    (*(*ctxt).sax)
                        .characters
                        .expect("non-null function pointer")(
                        (*ctxt).userData, out.as_mut_ptr(), i
                    );
                }
            }
        }
        return;
    }
    /*
     * We are seeing an entity reference
     */
    ent = unsafe { xmlParseEntityRef(ctxt) };
    if ent.is_null() {
        return;
    }
    let mut safe_ent = unsafe { &mut *ent };
    if (safe_ctxt).wellFormed == 0 {
        return;
    }
    was_checked = (safe_ent).checked;
    /* special case of predefined entities */
    if (safe_ent).name.is_null()
        || (safe_ent).etype as u32
            == XML_INTERNAL_PREDEFINED_ENTITY as i32 as u32
    {
        val = (safe_ent).content;
        if val.is_null() {
            return;
        }
        /*
         * inline the entity.
         */
        unsafe {
            if !(safe_ctxt).sax.is_null()
                && (*(*ctxt).sax).characters.is_some()
                && (safe_ctxt).disableSAX == 0
            {
                (*(*ctxt).sax)
                    .characters
                    .expect("non-null function pointer")(
                    (safe_ctxt).userData,
                    val,
                    xmlStrlen_safe(val),
                );
            }
        }
        return;
    }
    /*
     * The first reference to the entity trigger a parsing phase
     * where the ent->children is filled with the result from
     * the parsing.
     * Note: external parsed entities will not be loaded, it is not
     * required for a non-validating parser, unless the parsing option
     * of validating, or substituting entities were given. Doing so is
     * far more secure as the parser will only process data coming from
     * the document entity by default.
     */
    if ((safe_ent).checked == 0 as i32
        || (safe_ent).children.is_null()
            && (safe_ctxt).options & XML_PARSE_NOENT as i32 != 0)
        && ((safe_ent).etype as u32
            != XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32 as u32
            || (safe_ctxt).options
                & (XML_PARSE_NOENT as i32 | XML_PARSE_DTDVALID as i32)
                != 0)
    {
        let mut oldnbent: u64 = (safe_ctxt).nbentities;
        let mut diff: u64 = 0;
        /*
         * This is a bit hackish but this seems the best
         * way to make sure both SAX and DOM entity support
         * behaves okay.
         */
        let mut user_data: *mut () = 0 as *mut ();
        if (safe_ctxt).userData == ctxt as *mut () {
            user_data = 0 as *mut ()
        } else {
            user_data = (safe_ctxt).userData
        }
        /*
         * Check that this entity is well formed
         * 4.3.2: An internal general parsed entity is well-formed
         * if its replacement text matches the production labeled
         * content.
         */
        if (safe_ent).etype as u32
            == XML_INTERNAL_GENERAL_ENTITY as i32 as u32
        {
            (safe_ctxt).depth += 1;
            ret = unsafe {
                xmlParseBalancedChunkMemoryInternal(ctxt, (safe_ent).content, user_data, &mut list)
            };
            (safe_ctxt).depth -= 1
        } else if (safe_ent).etype as u32
            == XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32 as u32
        {
            (safe_ctxt).depth += 1;
            ret = unsafe {
                xmlParseExternalEntityPrivate(
                    (safe_ctxt).myDoc,
                    ctxt,
                    (safe_ctxt).sax,
                    user_data,
                    (safe_ctxt).depth,
                    (safe_ent).URI,
                    (safe_ent).ExternalID,
                    &mut list,
                )
            };
            (safe_ctxt).depth -= 1
        } else {
            ret = XML_ERR_ENTITY_PE_INTERNAL;
            unsafe {
                xmlErrMsgStr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"invalid entity type found\n\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                );
            }
        }
        /*
         * Store the number of entities needing parsing for this entity
         * content and do checkings
         */
        diff = (safe_ctxt)
            .nbentities
            .wrapping_sub(oldnbent)
            .wrapping_add(1 as i32 as u64);
        if diff > (2147483647 as i32 / 2 as i32) as u64 {
            diff = (2147483647 as i32 / 2 as i32) as u64
        }
        (safe_ent).checked = diff.wrapping_mul(2 as i32 as u64) as i32;
        if !(safe_ent).content.is_null()
            && !xmlStrchr_safe((safe_ent).content, '<' as i32 as xmlChar).is_null()
        {
            (safe_ent).checked |= 1 as i32
        }
        if ret as u32 == XML_ERR_ENTITY_LOOP as i32 as u32 {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_ENTITY_LOOP, 0 as *const i8);
                xmlHaltParser(ctxt);
            }
            xmlFreeNodeList_safe(list);
            return;
        }
        if xmlParserEntityCheck(
            ctxt,
            0 as i32 as size_t,
            ent,
            0 as i32 as size_t,
        ) != 0
        {
            xmlFreeNodeList_safe(list);
            return;
        }
        if ret as u32 == XML_ERR_OK as i32 as u32 && !list.is_null() {
            if ((safe_ent).etype as u32
                == XML_INTERNAL_GENERAL_ENTITY as i32 as u32
                || (safe_ent).etype as u32
                    == XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32 as u32)
                && (safe_ent).children.is_null()
            {
                (safe_ent).children = list;
                /*
                 * Prune it directly in the generated document
                 * except for single text nodes.
                 */
                if unsafe {
                    (safe_ctxt).replaceEntities == 0 as i32
                        || (safe_ctxt).parseMode as u32
                            == XML_PARSE_READER as i32 as u32
                        || (*list).type_0 as u32
                            == XML_TEXT_NODE as i32 as u32
                            && (*list).next.is_null()
                } {
                    (safe_ent).owner = 1 as i32;
                    while !list.is_null() {
                        unsafe {
                            (*list).parent = ent as xmlNodePtr;
                            xmlSetTreeDoc_safe(list, (safe_ent).doc);
                            if (*list).next.is_null() {
                                (safe_ent).last = list
                            }
                            list = (*list).next
                        }
                    }
                    list = 0 as xmlNodePtr
                } else {
                    (safe_ent).owner = 0 as i32;
                    while !list.is_null() {
                        unsafe {
                            (*list).parent = (safe_ctxt).node;
                            (*list).doc = (safe_ctxt).myDoc;
                            if (*list).next.is_null() {
                                (safe_ent).last = list
                            }
                            list = (*list).next
                        }
                    }
                    list = (safe_ent).children;

                    match () {
                        #[cfg(HAVE_parser_LIBXML_LEGACY_ENABLED)]
                        _ => {
                            if (safe_ent).etype as u32
                                == XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32 as u32
                            {
                                unsafe {
                                    xmlAddEntityReference(ent, list, 0 as xmlNodePtr);
                                }
                            }
                        }
                        #[cfg(not(HAVE_parser_LIBXML_LEGACY_ENABLED))]
                        _ => {}
                    };

                    /* LIBXML_LEGACY_ENABLED */
                }
            } else {
                xmlFreeNodeList_safe(list);
                list = 0 as xmlNodePtr
            }
        } else if ret as u32 != XML_ERR_OK as i32 as u32
            && ret as u32 != XML_WAR_UNDECLARED_ENTITY as i32 as u32
        {
            unsafe {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNDECLARED_ENTITY,
                    b"Entity \'%s\' failed to parse\n\x00" as *const u8 as *const i8,
                    (safe_ent).name,
                );
            }
            if !(safe_ent).content.is_null() {
                unsafe {
                    *(*ent).content.offset(0 as i32 as isize) = 0 as i32 as xmlChar
                }
            }
            xmlParserEntityCheck(
                ctxt,
                0 as i32 as size_t,
                ent,
                0 as i32 as size_t,
            );
        } else if !list.is_null() {
            xmlFreeNodeList_safe(list);
            list = 0 as xmlNodePtr
        }
        if (safe_ent).checked == 0 as i32 {
            (safe_ent).checked = 2 as i32
        }
        /* Prevent entity from being parsed and expanded twice (Bug 760367). */
        was_checked = 0 as i32
    } else if (safe_ent).checked != 1 as i32 {
        (safe_ctxt).nbentities = (safe_ctxt)
            .nbentities
            .wrapping_add(((safe_ent).checked / 2 as i32) as u64)
    }
    /*
     * Now that the entity content has been gathered
     * provide it to the application, this can take different forms based
     * on the parsing modes.
     */
    if (safe_ent).children.is_null() {
        /*
         * Probably running in SAX mode and the callbacks don't
         * build the entity content. So unless we already went
         * though parsing for first checking go though the entity
         * content to generate callbacks associated to the entity
         */
        if was_checked != 0 as i32 {
            let mut user_data_0: *mut () = 0 as *mut ();
            /*
             * This is a bit hackish but this seems the best
             * way to make sure both SAX and DOM entity support
             * behaves okay.
             */
            if (safe_ctxt).userData == ctxt as *mut () {
                user_data_0 = 0 as *mut ()
            } else {
                user_data_0 = (safe_ctxt).userData
            }
            if (safe_ent).etype as u32
                == XML_INTERNAL_GENERAL_ENTITY as i32 as u32
            {
                (safe_ctxt).depth += 1;
                ret = unsafe {
                    xmlParseBalancedChunkMemoryInternal(
                        ctxt,
                        (safe_ent).content,
                        user_data_0,
                        0 as *mut xmlNodePtr,
                    )
                };
                (safe_ctxt).depth -= 1
            } else if (safe_ent).etype as u32
                == XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32 as u32
            {
                (safe_ctxt).depth += 1;
                ret = unsafe {
                    xmlParseExternalEntityPrivate(
                        (safe_ctxt).myDoc,
                        ctxt,
                        (safe_ctxt).sax,
                        user_data_0,
                        (safe_ctxt).depth,
                        (safe_ent).URI,
                        (safe_ent).ExternalID,
                        0 as *mut xmlNodePtr,
                    )
                };
                (safe_ctxt).depth -= 1
            } else {
                ret = XML_ERR_ENTITY_PE_INTERNAL;
                unsafe {
                    xmlErrMsgStr(
                        ctxt,
                        XML_ERR_INTERNAL_ERROR,
                        b"invalid entity type found\n\x00" as *const u8 as *const i8,
                        0 as *const xmlChar,
                    );
                }
            }
            if ret as u32 == XML_ERR_ENTITY_LOOP as i32 as u32 {
                unsafe {
                    xmlFatalErr(ctxt, XML_ERR_ENTITY_LOOP, 0 as *const i8);
                }
                return;
            }
        }
        if !(safe_ctxt).sax.is_null()
            && unsafe { (*(*ctxt).sax).reference.is_some() }
            && (safe_ctxt).replaceEntities == 0 as i32
            && (safe_ctxt).disableSAX == 0
        {
            /*
             * Entity reference callback comes second, it's somewhat
             * superfluous but a compatibility to historical behaviour
             */
            unsafe {
                (*(*ctxt).sax).reference.expect("non-null function pointer")(
                    (safe_ctxt).userData,
                    (safe_ent).name,
                );
            }
        }
        return;
    }

    /*
     * If we didn't get any children for the entity being built
     */
    if !(safe_ctxt).sax.is_null()
        && unsafe { (*(*ctxt).sax).reference.is_some() }
        && (safe_ctxt).replaceEntities == 0 as i32
        && (safe_ctxt).disableSAX == 0
    {
        /*
         * Create a node.
         */
        unsafe {
            (*(*ctxt).sax).reference.expect("non-null function pointer")(
                (safe_ctxt).userData,
                (safe_ent).name,
            );
        }
        return;
    }
    if (safe_ctxt).replaceEntities != 0 || (safe_ent).children.is_null() {
        /*
         * There is a problem on the handling of _private for entities
         * (bug 155816): Should we copy the content of the field from
         * the entity (possibly overwriting some value set by the user
         * when a copy is created), should we leave it alone, or should
         * we try to take care of different situations?  The problem
         * is exacerbated by the usage of this field by the xmlReader.
         * To fix this bug, we look at _private on the created node
         * and, if it's NULL, we copy in whatever was in the entity.
         * If it's not NULL we leave it alone.  This is somewhat of a
         * hack - maybe we should have further tests to determine
         * what to do.
         */
        if !(safe_ctxt).node.is_null() && !(safe_ent).children.is_null() {
            /*
             * Seems we are generating the DOM content, do
             * a simple tree copy for all references except the first
             * In the first occurrence list contains the replacement.
             */
            if list.is_null() && (safe_ent).owner == 0 as i32
                || (safe_ctxt).parseMode as u32
                    == XML_PARSE_READER as i32 as u32
            {
                let mut nw: xmlNodePtr = 0 as xmlNodePtr;
                let mut cur: xmlNodePtr = 0 as *mut xmlNode;
                let mut firstChild: xmlNodePtr = 0 as xmlNodePtr;
                /* LIBXML_LEGACY_ENABLED */
                (safe_ctxt).sizeentcopy = (safe_ctxt)
                    .sizeentcopy
                    .wrapping_add(((safe_ent).length + 5 as i32) as u64);
                if xmlParserEntityCheck(
                    ctxt,
                    0 as i32 as size_t,
                    ent,
                    (safe_ctxt).sizeentcopy,
                ) != 0
                {
                    return;
                }
                cur = (safe_ent).children;
                while !cur.is_null() {
                    nw = xmlDocCopyNode_safe(cur, (safe_ctxt).myDoc, 1 as i32);
                    unsafe {
                        if !nw.is_null() {
                            if (*nw)._private.is_null() {
                                (*nw)._private = (*cur)._private
                            }
                            if firstChild.is_null() {
                                firstChild = nw
                            }
                            nw = xmlAddChild_safe((safe_ctxt).node, nw)
                        }
                    }
                    if cur == (safe_ent).last {
                        unsafe {
                            if (safe_ctxt).parseMode as u32
                                == XML_PARSE_READER as i32 as u32
                                && !nw.is_null()
                                && (*nw).type_0 as u32
                                    == XML_ELEMENT_NODE as i32 as u32
                                && (*nw).children.is_null()
                            {
                                (*nw).extra = 1 as i32 as u16
                            }
                        }
                        break;
                    } else {
                        cur = unsafe { (*cur).next };
                    }
                }

                match () {
                    #[cfg(HAVE_parser_LIBXML_LEGACY_ENABLED)]
                    _ => {
                        if (safe_ent).etype as u32
                            == XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32 as u32
                        {
                            unsafe {
                                xmlAddEntityReference(ent, firstChild, nw);
                            }
                        }
                    }
                    #[cfg(not(HAVE_parser_LIBXML_LEGACY_ENABLEDb))]
                    _ => {}
                };
            } else if list.is_null() || (safe_ctxt).inputNr > 0 as i32 {
                let mut nw_0: xmlNodePtr = 0 as xmlNodePtr;
                let mut cur_0: xmlNodePtr = 0 as *mut xmlNode;
                let mut next: xmlNodePtr = 0 as *mut xmlNode;
                let mut last: xmlNodePtr = 0 as *mut xmlNode;
                let mut firstChild_0: xmlNodePtr = 0 as xmlNodePtr;
                /* LIBXML_LEGACY_ENABLED */
                (safe_ctxt).sizeentcopy = (safe_ctxt)
                    .sizeentcopy
                    .wrapping_add(((safe_ent).length + 5 as i32) as u64);
                if xmlParserEntityCheck(
                    ctxt,
                    0 as i32 as size_t,
                    ent,
                    (safe_ctxt).sizeentcopy,
                ) != 0
                {
                    return;
                }
                cur_0 = (safe_ent).children;
                (safe_ent).children = 0 as *mut _xmlNode;
                last = (safe_ent).last;
                (safe_ent).last = 0 as *mut _xmlNode;
                while !cur_0.is_null() {
                    unsafe {
                        next = (*cur_0).next;
                        (*cur_0).next = 0 as *mut _xmlNode;
                        (*cur_0).parent = 0 as *mut _xmlNode;
                        nw_0 = xmlDocCopyNode_safe(cur_0, (safe_ctxt).myDoc, 1 as i32);
                        if !nw_0.is_null() {
                            if (*nw_0)._private.is_null() {
                                (*nw_0)._private = (*cur_0)._private
                            }
                            if firstChild_0.is_null() {
                                firstChild_0 = cur_0
                            }
                            xmlAddChild_safe(ent as xmlNodePtr, nw_0);
                            xmlAddChild_safe((safe_ctxt).node, cur_0);
                        }
                    }

                    if cur_0 == last {
                        break;
                    }
                    cur_0 = next
                }
                if (safe_ent).owner == 0 as i32 {
                    (safe_ent).owner = 1 as i32
                }
                match () {
                    #[cfg(HAVE_parser_LIBXML_LEGACY_ENABLED)]
                    _ => {
                        if (safe_ent).etype as u32
                            == XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32 as u32
                        {
                            unsafe {
                                xmlAddEntityReference(ent, firstChild_0, nw_0);
                            }
                        }
                    }
                    #[cfg(not(HAVE_parser_LIBXML_LEGACY_ENABLED))]
                    _ => {}
                };
            } else {
                let mut nbktext: *const xmlChar = 0 as *const xmlChar;
                /*
                 * We are copying here, make sure there is no abuse
                 */
                /*
                 * Copy the entity child list and make it the new
                 * entity child list. The goal is to make sure any
                 * ID or REF referenced will be the one from the
                 * document content and not the entity copy.
                 */
                /*
                 * the name change is to avoid coalescing of the
                 * node with a possible previous text one which
                 * would make ent->children a dangling pointer
                 */
                nbktext = xmlDictLookup_safe(
                    (safe_ctxt).dict,
                    b"nbktext\x00" as *const u8 as *const i8 as *mut xmlChar,
                    -(1 as i32),
                );
                unsafe {
                    if (*(*ent).children).type_0 as u32
                        == XML_TEXT_NODE as i32 as u32
                    {
                        (*(*ent).children).name = nbktext
                    }
                    if (safe_ent).last != (safe_ent).children
                        && (*(*ent).last).type_0 as u32
                            == XML_TEXT_NODE as i32 as u32
                    {
                        (*(*ent).last).name = nbktext
                    }
                }
                xmlAddChildList_safe((safe_ctxt).node, (safe_ent).children);
            }
            /*
             * This is to avoid a nasty side effect, see
             * characters() in SAX.c
             */
            (safe_ctxt).nodemem = 0 as i32;
            (safe_ctxt).nodelen = 0 as i32;
            return;
        }
    };
}
/* *
* xmlParseEntityRef:
* @ctxt:  an XML parser context
*
* parse ENTITY references declarations
*
* [68] EntityRef ::= '&' Name ';'
*
* [ WFC: Entity Declared ]
* In a document without any DTD, a document with only an internal DTD
* subset which contains no parameter entity references, or a document
* with "standalone='yes'", the Name given in the entity reference
* must match that in an entity declaration, except that well-formed
* documents need not declare any of the following entities: amp, lt,
* gt, apos, quot.  The declaration of a parameter entity must precede
* any reference to it.  Similarly, the declaration of a general entity
* must precede any reference to it which appears in a default value in an
* attribute-list declaration. Note that if entities are declared in the
* external subset or in external parameter entities, a non-validating
* processor is not obligated to read and process their declarations;
* for such documents, the rule that an entity must be declared is a
* well-formedness constraint only if standalone='yes'.
*
* [ WFC: Parsed Entity ]
* An entity reference must not contain the name of an unparsed entity
*
* Returns the xmlEntityPtr if found, or NULL otherwise.
*/

pub unsafe fn xmlParseEntityRef(mut ctxt: xmlParserCtxtPtr) -> xmlEntityPtr {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut ent: xmlEntityPtr = 0 as xmlEntityPtr;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return 0 as xmlEntityPtr;
    }
    if unsafe { *(*(*ctxt).input).cur as i32 != '&' as i32 } {
        return 0 as xmlEntityPtr;
    }
    xmlNextChar_safe(ctxt);
    name = xmlParseName(ctxt);
    if name.is_null() {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"xmlParseEntityRef: no name\n\x00" as *const u8 as *const i8,
            );
        }
        return 0 as xmlEntityPtr;
    }
    if unsafe { *(*(*ctxt).input).cur as i32 != ';' as i32 } {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_ENTITYREF_SEMICOL_MISSING,
                0 as *const i8,
            );
        }
        return 0 as xmlEntityPtr;
    }
    xmlNextChar_safe(ctxt);
    /*
     * Predefined entities override any extra definition
     */
    if (safe_ctxt).options & XML_PARSE_OLDSAX as i32 == 0 as i32 {
        ent = xmlGetPredefinedEntity_safe(name);
        if !ent.is_null() {
            return ent;
        }
    }
    /*
     * Increase the number of entity references parsed
     */
    (safe_ctxt).nbentities = (safe_ctxt).nbentities.wrapping_add(1);
    /*
     * Ask first SAX for entity resolution, otherwise try the
     * entities which may have stored in the parser context.
     */
    if !(safe_ctxt).sax.is_null() {
        unsafe {
            if (*(*ctxt).sax).getEntity.is_some() {
                ent = (*(*ctxt).sax).getEntity.expect("non-null function pointer")(
                    (safe_ctxt).userData,
                    name,
                )
            }
        }
        if (safe_ctxt).wellFormed == 1 as i32
            && ent.is_null()
            && (safe_ctxt).options & XML_PARSE_OLDSAX as i32 != 0
        {
            ent = xmlGetPredefinedEntity_safe(name)
        }
        if (safe_ctxt).wellFormed == 1 as i32
            && ent.is_null()
            && (safe_ctxt).userData == ctxt as *mut ()
        {
            ent = xmlSAX2GetEntity_safe(ctxt as *mut (), name)
        }
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return 0 as xmlEntityPtr;
    }
    /*
     * [ WFC: Entity Declared ]
     * In a document without any DTD, a document with only an
     * internal DTD subset which contains no parameter entity
     * references, or a document with "standalone='yes'", the
     * Name given in the entity reference must match that in an
     * entity declaration, except that well-formed documents
     * need not declare any of the following entities: amp, lt,
     * gt, apos, quot.
     * The declaration of a parameter entity must precede any
     * reference to it.
     * Similarly, the declaration of a general entity must
     * precede any reference to it which appears in a default
     * value in an attribute-list declaration. Note that if
     * entities are declared in the external subset or in
     * external parameter entities, a non-validating processor
     * is not obligated to read and process their declarations;
     * for such documents, the rule that an entity must be
     * declared is a well-formedness constraint only if
     * standalone='yes'.
     */
    let mut safe_ent = unsafe { &mut *ent };
    if ent.is_null() {
        if (safe_ctxt).standalone == 1 as i32
            || (safe_ctxt).hasExternalSubset == 0 as i32
                && (safe_ctxt).hasPErefs == 0 as i32
        {
            unsafe {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNDECLARED_ENTITY,
                    b"Entity \'%s\' not defined\n\x00" as *const u8 as *const i8,
                    name,
                );
            }
        } else {
            unsafe {
                xmlErrMsgStr(
                    ctxt,
                    XML_WAR_UNDECLARED_ENTITY,
                    b"Entity \'%s\' not defined\n\x00" as *const u8 as *const i8,
                    name,
                );
                if (safe_ctxt).inSubset == 0 as i32
                    && !(safe_ctxt).sax.is_null()
                    && (*(*ctxt).sax).reference.is_some()
                {
                    (*(*ctxt).sax).reference.expect("non-null function pointer")(
                        (safe_ctxt).userData,
                        name,
                    );
                }
            }
        }
        xmlParserEntityCheck(
            ctxt,
            0 as i32 as size_t,
            ent,
            0 as i32 as size_t,
        );
        (safe_ctxt).valid = 0 as i32
    } else if (safe_ent).etype as u32
        == XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as i32 as u32
    {
        unsafe {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_UNPARSED_ENTITY,
                b"Entity reference to unparsed entity %s\n\x00" as *const u8 as *const i8,
                name,
            );
        }
    } else if (safe_ctxt).instate as i32 == XML_PARSER_ATTRIBUTE_VALUE as i32
        && (safe_ent).etype as u32
            == XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32 as u32
    {
        unsafe {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_ENTITY_IS_EXTERNAL,
                b"Attribute references external entity \'%s\'\n\x00" as *const u8
                    as *const i8,
                name,
            );
        }
    } else if (safe_ctxt).instate as i32 == XML_PARSER_ATTRIBUTE_VALUE as i32
        && !ent.is_null()
        && (safe_ent).etype as u32
            != XML_INTERNAL_PREDEFINED_ENTITY as i32 as u32
    {
        if ((safe_ent).checked & 1 as i32 != 0 || (safe_ent).checked == 0 as i32)
            && !(safe_ent).content.is_null()
            && !xmlStrchr_safe((safe_ent).content, '<' as i32 as xmlChar).is_null()
        {
            unsafe {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_LT_IN_ATTRIBUTE,
                    b"\'<\' in entity \'%s\' is not allowed in attributes values\n\x00" as *const u8
                        as *const i8,
                    name,
                );
            }
        }
    } else {
        /*
         * [ WFC: Parsed Entity ]
         * An entity reference must not contain the name of an
         * unparsed entity
         */
        /*
         * [ WFC: No External Entity References ]
         * Attribute values cannot contain direct or indirect
         * entity references to external entities.
         */
        /*
         * [ WFC: No < in Attribute Values ]
         * The replacement text of any entity referred to directly or
         * indirectly in an attribute value (other than "&lt;") must
         * not contain a <.
         */
        /*
         * Internal check, no parameter entities here ...
         */
        match (safe_ent).etype as u32 {
            4 | 5 => unsafe {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_ENTITY_IS_PARAMETER,
                    b"Attempt to reference the parameter entity \'%s\'\n\x00" as *const u8
                        as *const i8,
                    name,
                );
            },
            _ => {}
        }
    }
    /*
     * [ WFC: No Recursion ]
     * A parsed entity must not contain a recursive reference
     * to itself, either directly or indirectly.
     * Done somewhere else
     */
    return ent;
}
/* ***********************************************************************
*									*
*		Parser stacks related functions and macros		*
*									*
************************************************************************/
/* *
* xmlParseStringEntityRef:
* @ctxt:  an XML parser context
* @str:  a pointer to an index in the string
*
* parse ENTITY references declarations, but this version parses it from
* a string value.
*
* [68] EntityRef ::= '&' Name ';'
*
* [ WFC: Entity Declared ]
* In a document without any DTD, a document with only an internal DTD
* subset which contains no parameter entity references, or a document
* with "standalone='yes'", the Name given in the entity reference
* must match that in an entity declaration, except that well-formed
* documents need not declare any of the following entities: amp, lt,
* gt, apos, quot.  The declaration of a parameter entity must precede
* any reference to it.  Similarly, the declaration of a general entity
* must precede any reference to it which appears in a default value in an
* attribute-list declaration. Note that if entities are declared in the
* external subset or in external parameter entities, a non-validating
* processor is not obligated to read and process their declarations;
* for such documents, the rule that an entity must be declared is a
* well-formedness constraint only if standalone='yes'.
*
* [ WFC: Parsed Entity ]
* An entity reference must not contain the name of an unparsed entity
*
* Returns the xmlEntityPtr if found, or NULL otherwise. The str pointer
* is updated to the current location in the string.
*/
unsafe fn xmlParseStringEntityRef(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *mut *const xmlChar,
) -> xmlEntityPtr {
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut ptr: *const xmlChar = 0 as *const xmlChar;
    let mut cur: xmlChar = 0;
    let mut ent: xmlEntityPtr = 0 as xmlEntityPtr;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if str.is_null() || unsafe { (*str).is_null() } {
        return 0 as xmlEntityPtr;
    }
    unsafe {
        ptr = *str;
        cur = *ptr;
    }
    if cur as i32 != '&' as i32 {
        return 0 as xmlEntityPtr;
    }
    ptr = unsafe { ptr.offset(1) };
    name = xmlParseStringName(ctxt, &mut ptr);
    if name.is_null() {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"xmlParseStringEntityRef: no name\n\x00" as *const u8 as *const i8,
            );
            *str = ptr;
        }
        return 0 as xmlEntityPtr;
    }
    unsafe {
        if *ptr as i32 != ';' as i32 {
            xmlFatalErr(
                ctxt,
                XML_ERR_ENTITYREF_SEMICOL_MISSING,
                0 as *const i8,
            );
            xmlFree_safe(name as *mut ());
            *str = ptr;
            return 0 as xmlEntityPtr;
        }
        ptr = ptr.offset(1);
    }
    /*
     * Predefined entities override any extra definition
     */
    if (safe_ctxt).options & XML_PARSE_OLDSAX as i32 == 0 as i32 {
        ent = xmlGetPredefinedEntity_safe(name);
        if !ent.is_null() {
            xmlFree_safe(name as *mut ());
            unsafe {
                *str = ptr;
            }
            return ent;
        }
    }
    /*
     * Increase the number of entity references parsed
     */
    (safe_ctxt).nbentities = (safe_ctxt).nbentities.wrapping_add(1);
    /*
     * Ask first SAX for entity resolution, otherwise try the
     * entities which may have stored in the parser context.
     */
    if !(safe_ctxt).sax.is_null() {
        unsafe {
            if (*(*ctxt).sax).getEntity.is_some() {
                ent = (*(*ctxt).sax).getEntity.expect("non-null function pointer")(
                    (safe_ctxt).userData,
                    name,
                )
            }
        }
        if ent.is_null() && (safe_ctxt).options & XML_PARSE_OLDSAX as i32 != 0 {
            ent = xmlGetPredefinedEntity_safe(name)
        }
        if ent.is_null() && (safe_ctxt).userData == ctxt as *mut () {
            ent = xmlSAX2GetEntity_safe(ctxt as *mut (), name)
        }
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        xmlFree_safe(name as *mut ());
        return 0 as xmlEntityPtr;
    }
    /*
     * [ WFC: Entity Declared ]
     * In a document without any DTD, a document with only an
     * internal DTD subset which contains no parameter entity
     * references, or a document with "standalone='yes'", the
     * Name given in the entity reference must match that in an
     * entity declaration, except that well-formed documents
     * need not declare any of the following entities: amp, lt,
     * gt, apos, quot.
     * The declaration of a parameter entity must precede any
     * reference to it.
     * Similarly, the declaration of a general entity must
     * precede any reference to it which appears in a default
     * value in an attribute-list declaration. Note that if
     * entities are declared in the external subset or in
     * external parameter entities, a non-validating processor
     * is not obligated to read and process their declarations;
     * for such documents, the rule that an entity must be
     * declared is a well-formedness constraint only if
     * standalone='yes'.
     */
    let mut safe_ent = unsafe { &mut *ent };
    if ent.is_null() {
        if (safe_ctxt).standalone == 1 as i32
            || (safe_ctxt).hasExternalSubset == 0 as i32
                && (safe_ctxt).hasPErefs == 0 as i32
        {
            unsafe {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNDECLARED_ENTITY,
                    b"Entity \'%s\' not defined\n\x00" as *const u8 as *const i8,
                    name,
                );
            }
        } else {
            unsafe {
                xmlErrMsgStr(
                    ctxt,
                    XML_WAR_UNDECLARED_ENTITY,
                    b"Entity \'%s\' not defined\n\x00" as *const u8 as *const i8,
                    name,
                );
            }
        }
        xmlParserEntityCheck(
            ctxt,
            0 as i32 as size_t,
            ent,
            0 as i32 as size_t,
        );
    /* TODO ? check regressions ctxt->valid = 0; */
    } else if (safe_ent).etype as u32
        == XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as i32 as u32
    {
        unsafe {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_UNPARSED_ENTITY,
                b"Entity reference to unparsed entity %s\n\x00" as *const u8 as *const i8,
                name,
            );
        }
    } else if (safe_ctxt).instate as i32 == XML_PARSER_ATTRIBUTE_VALUE as i32
        && (safe_ent).etype as u32
            == XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32 as u32
    {
        unsafe {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_ENTITY_IS_EXTERNAL,
                b"Attribute references external entity \'%s\'\n\x00" as *const u8
                    as *const i8,
                name,
            );
        }
    } else if (safe_ctxt).instate as i32 == XML_PARSER_ATTRIBUTE_VALUE as i32
        && !ent.is_null()
        && !(safe_ent).content.is_null()
        && (safe_ent).etype as u32
            != XML_INTERNAL_PREDEFINED_ENTITY as i32 as u32
        && !xmlStrchr_safe((safe_ent).content, '<' as i32 as xmlChar).is_null()
    {
        unsafe {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_LT_IN_ATTRIBUTE,
                b"\'<\' in entity \'%s\' is not allowed in attributes values\n\x00" as *const u8
                    as *const i8,
                name,
            );
        }
    } else {
        /*
         * [ WFC: Parsed Entity ]
         * An entity reference must not contain the name of an
         * unparsed entity
         */
        /*
         * [ WFC: No External Entity References ]
         * Attribute values cannot contain direct or indirect
         * entity references to external entities.
         */
        /*
         * [ WFC: No < in Attribute Values ]
         * The replacement text of any entity referred to directly or
         * indirectly in an attribute value (other than "&lt;") must
         * not contain a <.
         */
        /*
         * Internal check, no parameter entities here ...
         */
        match (safe_ent).etype as u32 {
            4 | 5 => unsafe {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_ENTITY_IS_PARAMETER,
                    b"Attempt to reference the parameter entity \'%s\'\n\x00" as *const u8
                        as *const i8,
                    name,
                );
            },
            _ => {}
        }
    }
    /*
     * [ WFC: No Recursion ]
     * A parsed entity must not contain a recursive reference
     * to itself, either directly or indirectly.
     * Done somewhere else
     */
    xmlFree_safe(name as *mut ());
    unsafe {
        *str = ptr;
    }
    return ent;
}
/* *
* xmlParsePEReference:
* @ctxt:  an XML parser context
*
* parse PEReference declarations
* The entity content is handled directly by pushing it's content as
* a new input stream.
*
* [69] PEReference ::= '%' Name ';'
*
* [ WFC: No Recursion ]
* A parsed entity must not contain a recursive
* reference to itself, either directly or indirectly.
*
* [ WFC: Entity Declared ]
* In a document without any DTD, a document with only an internal DTD
* subset which contains no parameter entity references, or a document
* with "standalone='yes'", ...  ... The declaration of a parameter
* entity must precede any reference to it...
*
* [ VC: Entity Declared ]
* In a document with an external subset or external parameter entities
* with "standalone='no'", ...  ... The declaration of a parameter entity
* must precede any reference to it...
*
* [ WFC: In DTD ]
* Parameter-entity references may only appear in the DTD.
* NOTE: misleading but this is handled.
*/

pub unsafe fn xmlParsePEReference(mut ctxt: xmlParserCtxtPtr) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut entity: xmlEntityPtr = 0 as xmlEntityPtr;
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if unsafe { *(*(*ctxt).input).cur as i32 != '%' as i32 } {
        return;
    }
    xmlNextChar_safe(ctxt);
    name = xmlParseName(ctxt);
    if name.is_null() {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_PEREF_NO_NAME,
                b"PEReference: no name\n\x00" as *const u8 as *const i8,
            );
        }
        return;
    }
    unsafe {
        if *__xmlParserDebugEntities() != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"PEReference: %s\n\x00" as *const u8 as *const i8,
                name,
            );
        }
        if *(*(*ctxt).input).cur as i32 != ';' as i32 {
            xmlFatalErr(
                ctxt,
                XML_ERR_PEREF_SEMICOL_MISSING,
                0 as *const i8,
            );
            return;
        }
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    xmlNextChar_safe(ctxt);
    /*
     * Increase the number of entity references parsed
     */
    (safe_ctxt).nbentities = (safe_ctxt).nbentities.wrapping_add(1);
    /*
     * Request the entity from SAX
     */
    unsafe {
        if !(safe_ctxt).sax.is_null() && (*(*ctxt).sax).getParameterEntity.is_some() {
            entity = (*(*ctxt).sax)
                .getParameterEntity
                .expect("non-null function pointer")((safe_ctxt).userData, name)
        }
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return;
    }
    let mut safe_entity = unsafe { &mut *entity };
    if entity.is_null() {
        /*
         * [ WFC: Entity Declared ]
         * In a document without any DTD, a document with only an
         * internal DTD subset which contains no parameter entity
         * references, or a document with "standalone='yes'", ...
         * ... The declaration of a parameter entity must precede
         * any reference to it...
         */
        if (safe_ctxt).standalone == 1 as i32
            || (safe_ctxt).hasExternalSubset == 0 as i32
                && (safe_ctxt).hasPErefs == 0 as i32
        {
            unsafe {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNDECLARED_ENTITY,
                    b"PEReference: %%%s; not found\n\x00" as *const u8 as *const i8,
                    name,
                );
            }
        } else {
            /*
             * [ VC: Entity Declared ]
             * In a document with an external subset or external
             * parameter entities with "standalone='no'", ...
             * ... The declaration of a parameter entity must
             * precede any reference to it...
             */
            if (safe_ctxt).validate != 0 && (safe_ctxt).vctxt.error.is_some() {
                unsafe {
                    xmlValidityError(
                        ctxt,
                        XML_WAR_UNDECLARED_ENTITY,
                        b"PEReference: %%%s; not found\n\x00" as *const u8 as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                }
            } else {
                unsafe {
                    xmlWarningMsg(
                        ctxt,
                        XML_WAR_UNDECLARED_ENTITY,
                        b"PEReference: %%%s; not found\n\x00" as *const u8 as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                }
            }
            (safe_ctxt).valid = 0 as i32
        }
        xmlParserEntityCheck(
            ctxt,
            0 as i32 as size_t,
            0 as xmlEntityPtr,
            0 as i32 as size_t,
        );
    } else if (safe_entity).etype as u32
        != XML_INTERNAL_PARAMETER_ENTITY as i32 as u32
        && (safe_entity).etype as u32
            != XML_EXTERNAL_PARAMETER_ENTITY as i32 as u32
    {
        unsafe {
            xmlWarningMsg(
                ctxt,
                XML_WAR_UNDECLARED_ENTITY,
                b"Internal: %%%s; is not a parameter entity\n\x00" as *const u8
                    as *const i8,
                name,
                0 as *const xmlChar,
            );
        }
    } else {
        let mut start: [xmlChar; 4] = [0; 4];
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        if xmlParserEntityCheck(
            ctxt,
            0 as i32 as size_t,
            entity,
            0 as i32 as size_t,
        ) != 0
        {
            return;
        }
        if (safe_entity).etype as u32
            == XML_EXTERNAL_PARAMETER_ENTITY as i32 as u32
            && (safe_ctxt).options & XML_PARSE_NOENT as i32 == 0 as i32
            && (safe_ctxt).options & XML_PARSE_DTDVALID as i32 == 0 as i32
            && (safe_ctxt).options & XML_PARSE_DTDLOAD as i32 == 0 as i32
            && (safe_ctxt).options & XML_PARSE_DTDATTR as i32 == 0 as i32
            && (safe_ctxt).replaceEntities == 0 as i32
            && (safe_ctxt).validate == 0 as i32
        {
            return;
        }
        input = unsafe { xmlNewEntityInputStream(ctxt, entity) };
        if xmlPushInput(ctxt, input) < 0 as i32 {
            xmlFreeInputStream_safe(input);
            return;
        }
        if (safe_entity).etype as u32
            == XML_EXTERNAL_PARAMETER_ENTITY as i32 as u32
        {
            /*
             * Internal checking in case the entity quest barfed
             */
            /*
             * Get the 4 first bytes and decode the charset
             * if enc != XML_CHAR_ENCODING_NONE
             * plug some encoding conversion routines.
             * Note that, since we may have some non-UTF8
             * encoding (like UTF16, bug 135229), the 'length'
             * is not known, but we can calculate based upon
             * the amount of data in the buffer.
             */
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                }
            {
                xmlGROW(ctxt);
            }
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                return;
            }
            unsafe {
                if (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64
                    >= 4 as i32 as i64
                {
                    start[0 as i32 as usize] = *(*(*ctxt).input).cur;
                    start[1 as i32 as usize] =
                        *(*(*ctxt).input).cur.offset(1 as i32 as isize);
                    start[2 as i32 as usize] =
                        *(*(*ctxt).input).cur.offset(2 as i32 as isize);
                    start[3 as i32 as usize] =
                        *(*(*ctxt).input).cur.offset(3 as i32 as isize);
                    enc = xmlDetectCharEncoding_safe(start.as_mut_ptr(), 4 as i32);
                    if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
                        xmlSwitchEncoding_safe(ctxt, enc);
                    }
                }
                if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
                    as i32
                    == '<' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(1 as i32 as isize) as i32
                        == '?' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(2 as i32 as isize) as i32
                        == 'x' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(3 as i32 as isize) as i32
                        == 'm' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(4 as i32 as isize) as i32
                        == 'l' as i32
                    && (*(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                        == 0x20 as i32
                        || 0x9 as i32
                            <= *(*(*ctxt).input).cur.offset(5 as i32 as isize)
                                as i32
                            && *(*(*ctxt).input).cur.offset(5 as i32 as isize)
                                as i32
                                <= 0xa as i32
                        || *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                            == 0xd as i32)
                {
                    xmlParseTextDecl(ctxt);
                }
            }
        }
    }
    (safe_ctxt).hasPErefs = 1 as i32;
}
/* *
* xmlLoadEntityContent:
* @ctxt:  an XML parser context
* @entity: an unloaded system entity
*
* Load the original content of the given system entity from the
* ExternalID/SystemID given. This is to be used for Included in Literal
* http://www.w3.org/TR/REC-xml/#inliteral processing of entities references
*
* Returns 0 in case of success and -1 in case of failure
*/
unsafe fn xmlLoadEntityContent(
    mut ctxt: xmlParserCtxtPtr,
    mut entity: xmlEntityPtr,
) -> i32 {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut buf: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut safe_entity = unsafe { &mut *entity };
    if ctxt.is_null()
        || entity.is_null()
        || (safe_entity).etype as u32
            != XML_EXTERNAL_PARAMETER_ENTITY as i32 as u32
            && (safe_entity).etype as u32
                != XML_EXTERNAL_GENERAL_PARSED_ENTITY as i32 as u32
        || !(safe_entity).content.is_null()
    {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"xmlLoadEntityContent parameter error\x00" as *const u8 as *const i8,
            );
        }
        return -(1 as i32);
    }
    unsafe {
        if *__xmlParserDebugEntities() != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Reading %s entity content input\n\x00" as *const u8 as *const i8,
                (safe_entity).name,
            );
        }
    }
    buf = xmlBufferCreate_safe();
    if buf.is_null() {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"xmlLoadEntityContent parameter error\x00" as *const u8 as *const i8,
            );
        }
        return -(1 as i32);
    }
    input = unsafe { xmlNewEntityInputStream(ctxt, entity) };
    if input.is_null() {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"xmlLoadEntityContent input error\x00" as *const u8 as *const i8,
            );
        }
        xmlBufferFree_safe(buf);
        return -(1 as i32);
    }
    /*
     * Push the entity as the current input, read char by char
     * saving to the buffer until the end of the entity or an error
     */
    if xmlPushInput(ctxt, input) < 0 as i32 {
        xmlBufferFree_safe(buf);
        return -(1 as i32);
    }
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    unsafe {
        c = xmlCurrentChar(ctxt, &mut l);
    }
    while (safe_ctxt).input == input
        && unsafe { (*(*ctxt).input).cur < (*(*ctxt).input).end }
        && (if c < 0x100 as i32 {
            (0x9 as i32 <= c && c <= 0xa as i32
                || c == 0xd as i32
                || 0x20 as i32 <= c) as i32
        } else {
            (0x100 as i32 <= c && c <= 0xd7ff as i32
                || 0xe000 as i32 <= c && c <= 0xfffd as i32
                || 0x10000 as i32 <= c && c <= 0x10ffff as i32)
                as i32
        }) != 0
    {
        xmlBufferAdd_safe(buf, unsafe { (*(*ctxt).input).cur }, l);
        let fresh88 = count;
        count = count + 1;
        if fresh88 > 100 as i32 {
            count = 0 as i32;
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                }
            {
                xmlGROW(ctxt);
            }
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                xmlBufferFree_safe(buf);
                return -(1 as i32);
            }
        }
        unsafe {
            if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as i32
            } else {
                (*(*ctxt).input).col += 1
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            c = xmlCurrentChar(ctxt, &mut l);
        }
        if c == 0 as i32 {
            count = 0 as i32;

            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                }
            {
                xmlGROW(ctxt);
            }
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                xmlBufferFree_safe(buf);
                return -(1 as i32);
            }
            unsafe {
                c = xmlCurrentChar(ctxt, &mut l);
            }
        }
    }
    if (safe_ctxt).input == input && unsafe { (*(*ctxt).input).cur >= (*(*ctxt).input).end } {
        xmlPopInput_parser(ctxt);
    } else if if c < 0x100 as i32 {
        (0x9 as i32 <= c && c <= 0xa as i32
            || c == 0xd as i32
            || 0x20 as i32 <= c) as i32
    } else {
        (0x100 as i32 <= c && c <= 0xd7ff as i32
            || 0xe000 as i32 <= c && c <= 0xfffd as i32
            || 0x10000 as i32 <= c && c <= 0x10ffff as i32) as i32
    } == 0
    {
        unsafe {
            xmlFatalErrMsgInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"xmlLoadEntityContent: invalid char value %d\n\x00" as *const u8
                    as *const i8,
                c,
            );
        }
        xmlBufferFree_safe(buf);
        return -(1 as i32);
    }
    unsafe {
        (safe_entity).content = (*buf).content;
        (*buf).content = 0 as *mut xmlChar;
    }
    xmlBufferFree_safe(buf);
    return 0 as i32;
}
/* DEPR void xmlParserHandleReference(xmlParserCtxtPtr ctxt); */
/* *
* xmlParseStringPEReference:
* @ctxt:  an XML parser context
* @str:  a pointer to an index in the string
*
* parse PEReference declarations
*
* [69] PEReference ::= '%' Name ';'
*
* [ WFC: No Recursion ]
* A parsed entity must not contain a recursive
* reference to itself, either directly or indirectly.
*
* [ WFC: Entity Declared ]
* In a document without any DTD, a document with only an internal DTD
* subset which contains no parameter entity references, or a document
* with "standalone='yes'", ...  ... The declaration of a parameter
* entity must precede any reference to it...
*
* [ VC: Entity Declared ]
* In a document with an external subset or external parameter entities
* with "standalone='no'", ...  ... The declaration of a parameter entity
* must precede any reference to it...
*
* [ WFC: In DTD ]
* Parameter-entity references may only appear in the DTD.
* NOTE: misleading but this is handled.
*
* Returns the string of the entity content.
*         str is updated to the current value of the index
*/
unsafe fn xmlParseStringPEReference(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *mut *const xmlChar,
) -> xmlEntityPtr {
    let mut ptr: *const xmlChar = 0 as *const xmlChar;
    let mut cur: xmlChar = 0;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut entity: xmlEntityPtr = 0 as xmlEntityPtr;
    if str.is_null() || unsafe { (*str).is_null() } {
        return 0 as xmlEntityPtr;
    }
    unsafe {
        ptr = *str;
        cur = *ptr;
    }
    if cur as i32 != '%' as i32 {
        return 0 as xmlEntityPtr;
    }
    ptr = unsafe { ptr.offset(1) };
    name = xmlParseStringName(ctxt, &mut ptr);
    if name.is_null() {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"xmlParseStringPEReference: no name\n\x00" as *const u8 as *const i8,
            );
            *str = ptr;
        }
        return 0 as xmlEntityPtr;
    }
    cur = unsafe { *ptr };
    if cur as i32 != ';' as i32 {
        unsafe {
            xmlFatalErr(
                ctxt,
                XML_ERR_ENTITYREF_SEMICOL_MISSING,
                0 as *const i8,
            );
        }
        xmlFree_safe(name as *mut ());
        unsafe {
            *str = ptr;
        }
        return 0 as xmlEntityPtr;
    }
    ptr = unsafe { ptr.offset(1) };
    let mut safe_ctxt = unsafe { &mut *ctxt };
    /*
     * Increase the number of entity references parsed
     */
    (safe_ctxt).nbentities = (safe_ctxt).nbentities.wrapping_add(1);
    /*
     * Request the entity from SAX
     */
    unsafe {
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).getParameterEntity.is_some() {
            entity = (*(*ctxt).sax)
                .getParameterEntity
                .expect("non-null function pointer")((*ctxt).userData, name)
        }
    }
    let mut safe_entity = unsafe { &mut *entity };
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        xmlFree_safe(name as *mut ());
        unsafe { *str = ptr };
        return 0 as xmlEntityPtr;
    }
    if entity.is_null() {
        /*
         * [ WFC: Entity Declared ]
         * In a document without any DTD, a document with only an
         * internal DTD subset which contains no parameter entity
         * references, or a document with "standalone='yes'", ...
         * ... The declaration of a parameter entity must precede
         * any reference to it...
         */
        if (safe_ctxt).standalone == 1 as i32
            || (safe_ctxt).hasExternalSubset == 0 as i32
                && (safe_ctxt).hasPErefs == 0 as i32
        {
            unsafe {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNDECLARED_ENTITY,
                    b"PEReference: %%%s; not found\n\x00" as *const u8 as *const i8,
                    name,
                );
            }
        } else {
            /*
             * [ VC: Entity Declared ]
             * In a document with an external subset or external
             * parameter entities with "standalone='no'", ...
             * ... The declaration of a parameter entity must
             * precede any reference to it...
             */
            unsafe {
                xmlWarningMsg(
                    ctxt,
                    XML_WAR_UNDECLARED_ENTITY,
                    b"PEReference: %%%s; not found\n\x00" as *const u8 as *const i8,
                    name,
                    0 as *const xmlChar,
                );
            }
            (safe_ctxt).valid = 0 as i32
        }
        xmlParserEntityCheck(
            ctxt,
            0 as i32 as size_t,
            0 as xmlEntityPtr,
            0 as i32 as size_t,
        );
    } else if (safe_entity).etype as u32
        != XML_INTERNAL_PARAMETER_ENTITY as i32 as u32
        && (safe_entity).etype as u32
            != XML_EXTERNAL_PARAMETER_ENTITY as i32 as u32
    {
        unsafe {
            xmlWarningMsg(
                ctxt,
                XML_WAR_UNDECLARED_ENTITY,
                b"%%%s; is not a parameter entity\n\x00" as *const u8 as *const i8,
                name,
                0 as *const xmlChar,
            );
        }
    }
    (safe_ctxt).hasPErefs = 1 as i32;
    xmlFree_safe(name as *mut ());
    unsafe {
        *str = ptr;
    }
    return entity;
}
/*
* Internal checking in case the entity quest barfed
*/
/* *
* xmlParseDocTypeDecl:
* @ctxt:  an XML parser context
*
* parse a DOCTYPE declaration
*
* [28] doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S?
*                      ('[' (markupdecl | PEReference | S)* ']' S?)? '>'
*
* [ VC: Root Element Type ]
* The Name in the document type declaration must match the element
* type of the root element.
*/

pub unsafe fn xmlParseDocTypeDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut ExternalID: *mut xmlChar = 0 as *mut xmlChar;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    /*
     * We know that '<!DOCTYPE' has been detected.
     */
    unsafe {
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(9 as i32 as isize);
        (*(*ctxt).input).col += 9 as i32;
        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
            xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
        }
    }
    xmlSkipBlankChars(ctxt);
    /*
     * Parse the DOCTYPE name.
     */
    name = xmlParseName(ctxt);
    if name.is_null() {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"xmlParseDocTypeDecl : no DOCTYPE name !\n\x00" as *const u8
                    as *const i8,
            );
        }
    }
    (safe_ctxt).intSubName = name;
    xmlSkipBlankChars(ctxt);
    /*
     * Check for SystemID and ExternalID
     */
    URI = xmlParseExternalID(ctxt, &mut ExternalID, 1 as i32);
    if !URI.is_null() || !ExternalID.is_null() {
        (safe_ctxt).hasExternalSubset = 1 as i32
    }
    (safe_ctxt).extSubURI = URI;
    (safe_ctxt).extSubSystem = ExternalID;
    xmlSkipBlankChars(ctxt);
    /*
     * Create and update the internal subset.
     */
    unsafe {
        if !(safe_ctxt).sax.is_null()
            && (*(*ctxt).sax).internalSubset.is_some()
            && (safe_ctxt).disableSAX == 0
        {
            (*(*ctxt).sax)
                .internalSubset
                .expect("non-null function pointer")(
                (safe_ctxt).userData, name, ExternalID, URI
            );
        }
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return;
    }
    /*
     * Is there any internal subset declarations ?
     * they are handled separately in xmlParseInternalSubset()
     */
    unsafe {
        if *(*(*ctxt).input).cur as i32 == '[' as i32 {
            return;
        }
        /*
         * We should be at the end of the DOCTYPE declaration.
         */
        if *(*(*ctxt).input).cur as i32 != '>' as i32 {
            xmlFatalErr(ctxt, XML_ERR_DOCTYPE_NOT_FINISHED, 0 as *const i8);
        }
    }
    xmlNextChar_safe(ctxt);
}
/* *
* xmlParseInternalSubset:
* @ctxt:  an XML parser context
*
* parse the internal subset declaration
*
* [28 end] ('[' (markupdecl | PEReference | S)* ']' S?)? '>'
*/
unsafe fn xmlParseInternalSubset(mut ctxt: xmlParserCtxtPtr) {
    /*
     * Is there any DTD definition ?
     */
    //@todo 削减unsafe范围
    unsafe {
        if *(*(*ctxt).input).cur as i32 == '[' as i32 {
            let mut baseInputNr: i32 = (*ctxt).inputNr;
            (*ctxt).instate = XML_PARSER_DTD;
            xmlNextChar_safe(ctxt);
            /*
             * Parse the succession of Markup declarations and
             * PEReferences.
             * Subsequence (markupdecl | PEReference | S)*
             */
            while 1<2
            {
                if(!((*(*(*ctxt).input).cur as i32 != ']' as i32
                || (*ctxt).inputNr > baseInputNr)
                && (*ctxt).instate as i32 != XML_PARSER_EOF as i32)){
                    break;
                }
                let mut check: *const xmlChar = (*(*ctxt).input).cur;
                let mut cons: u32 = (*(*ctxt).input).consumed as u32;
                xmlSkipBlankChars(ctxt);
                xmlParseMarkupDecl(ctxt);
                xmlParsePEReference(ctxt);
                /*
                 * Conditional sections are allowed from external entities included
                 * by PE References in the internal subset.
                 */
                if (*ctxt).inputNr > 1 as i32
                    && !(*(*ctxt).input).filename.is_null()
                    && *(*(*ctxt).input).cur as i32 == '<' as i32
                    && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        == '!' as i32
                    && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                        == '[' as i32
                {
                    xmlParseConditionalSections(ctxt);
                }
                if !((*(*ctxt).input).cur == check
                    && cons as u64 == (*(*ctxt).input).consumed)
                {
                    continue;
                }
                xmlFatalErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"xmlParseInternalSubset: error detected in Markup declaration\n\x00"
                        as *const u8 as *const i8,
                );
                if !((*ctxt).inputNr > baseInputNr) {
                    break;
                }
                xmlPopInput(ctxt);
            }
            if *(*(*ctxt).input).cur as i32 == ']' as i32 {
                xmlNextChar_safe(ctxt);
                xmlSkipBlankChars(ctxt);
            }
        }
        /*
         * We should be at the end of the DOCTYPE declaration.
         */
        if *(*(*ctxt).input).cur as i32 != '>' as i32 {
            xmlFatalErr(ctxt, XML_ERR_DOCTYPE_NOT_FINISHED, 0 as *const i8);
            return;
        }
    }
    xmlNextChar_safe(ctxt);
}
/* *
* xmlParseAttribute:
* @ctxt:  an XML parser context
* @value:  a xmlChar ** used to store the value of the attribute
*
* parse an attribute
*
* [41] Attribute ::= Name Eq AttValue
*
* [ WFC: No External Entity References ]
* Attribute values cannot contain direct or indirect entity references
* to external entities.
*
* [ WFC: No < in Attribute Values ]
* The replacement text of any entity referred to directly or indirectly in
* an attribute value (other than "&lt;") must not contain a <.
*
* [ VC: Attribute Value Type ]
* The attribute must have been declared; the value must be of the type
* declared for it.
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
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlParseAttribute(
    mut ctxt: xmlParserCtxtPtr,
    mut value: *mut *mut xmlChar,
) -> *const xmlChar {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    unsafe { *value = 0 as *mut xmlChar };
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    name = xmlParseName(ctxt);
    if name.is_null() {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"error parsing attribute name\n\x00" as *const u8 as *const i8,
            );
        }
        return 0 as *const xmlChar;
    }
    /*
     * read the value
     */
    xmlSkipBlankChars(ctxt);
    if unsafe { *(*(*ctxt).input).cur as i32 == '=' as i32 } {
        xmlNextChar_safe(ctxt);
        xmlSkipBlankChars(ctxt);
        val = xmlParseAttValue(ctxt);
        (safe_ctxt).instate = XML_PARSER_CONTENT
    } else {
        unsafe {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_ATTRIBUTE_WITHOUT_VALUE,
                b"Specification mandates value for attribute %s\n\x00" as *const u8
                    as *const i8,
                name,
            );
        }
        return 0 as *const xmlChar;
    }
    /*
     * Check that xml:lang conforms to the specification
     * No more registered as an error, just generate a warning now
     * since this was deprecated in XML second edition
     */
    if (safe_ctxt).pedantic != 0
        && xmlStrEqual_safe(
            name,
            b"xml:lang\x00" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
    {
        if xmlCheckLanguageID(val) == 0 {
            unsafe {
                xmlWarningMsg(
                    ctxt,
                    XML_WAR_LANG_VALUE,
                    b"Malformed value for xml:lang : %s\n\x00" as *const u8 as *const i8,
                    val,
                    0 as *const xmlChar,
                );
            }
        }
    }
    /*
     * Check that xml:space conforms to the specification
     */
    if xmlStrEqual_safe(
        name,
        b"xml:space\x00" as *const u8 as *const i8 as *mut xmlChar,
    ) != 0
    {
        if xmlStrEqual_safe(
            val,
            b"default\x00" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
            unsafe { *(*ctxt).space = 0 as i32 }
        } else if xmlStrEqual_safe(
            val,
            b"preserve\x00" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
            unsafe { *(*ctxt).space = 1 as i32 }
        } else {
            unsafe {
                xmlWarningMsg(ctxt, XML_WAR_SPACE_VALUE,
          b"Invalid value \"%s\" for xml:space : \"default\" or \"preserve\" expected\n\x00"
              as *const u8 as *const i8, val,
          0 as *const xmlChar);
            }
        }
    }
    unsafe { *value = val };
    return name;
}
/* *
* xmlParseStartTag:
* @ctxt:  an XML parser context
*
* parse a start of tag either for rule element or
* EmptyElement. In both case we don't parse the tag closing chars.
*
* [40] STag ::= '<' Name (S Attribute)* S? '>'
*
* [ WFC: Unique Att Spec ]
* No attribute name may appear more than once in the same start-tag or
* empty-element tag.
*
* [44] EmptyElemTag ::= '<' Name (S Attribute)* S? '/>'
*
* [ WFC: Unique Att Spec ]
* No attribute name may appear more than once in the same start-tag or
* empty-element tag.
*
* With namespace:
*
* [NS 8] STag ::= '<' QName (S Attribute)* S? '>'
*
* [NS 10] EmptyElement ::= '<' QName (S Attribute)* S? '/>'
*
* Returns the element name parsed
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlParseStartTag(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut current_block: u64;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut attname: *const xmlChar = 0 as *const xmlChar;
    let mut attvalue: *mut xmlChar = 0 as *mut xmlChar;
    let mut atts: *mut *const xmlChar = (safe_ctxt).atts;
    let mut nbatts: i32 = 0 as i32;
    let mut maxatts: i32 = (safe_ctxt).maxatts;
    let mut i: i32 = 0;
    unsafe {
        if *(*(*ctxt).input).cur as i32 != '<' as i32 {
            return 0 as *const xmlChar;
        }
        (*(*ctxt).input).col += 1;
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
            xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
        }
    }
    name = xmlParseName(ctxt);
    if name.is_null() {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"xmlParseStartTag: invalid element name\n\x00" as *const u8 as *const i8,
            );
        }
        return 0 as *const xmlChar;
    }
    /*
     * Now parse the attributes, it ends up with the ending
     *
     * (S Attribute)* S?
     */
    xmlSkipBlankChars(ctxt);
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    while unsafe {
        *(*(*ctxt).input).cur as i32 != '>' as i32
            && (*(*(*ctxt).input).cur as i32 != '/' as i32
                || *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    != '>' as i32)
            && (0x9 as i32 <= *(*(*ctxt).input).cur as i32
                && *(*(*ctxt).input).cur as i32 <= 0xa as i32
                || *(*(*ctxt).input).cur as i32 == 0xd as i32
                || 0x20 as i32 <= *(*(*ctxt).input).cur as i32)
    } && (safe_ctxt).instate as i32 != XML_PARSER_EOF as i32
    {
        let mut q: *const xmlChar = unsafe { (*(*ctxt).input).cur };
        let mut cons: u32 = unsafe { (*(*ctxt).input).consumed as u32 };
        attname = xmlParseAttribute(ctxt, &mut attvalue);
        if !attname.is_null() && !attvalue.is_null() {
            /*
             * [ WFC: Unique Att Spec ]
             * No attribute name may appear more than once in the same
             * start-tag or empty-element tag.
             */
            i = 0 as i32;
            loop {
                if !(i < nbatts) {
                    current_block = 3437258052017859086;
                    break;
                }
                if unsafe { xmlStrEqual_safe(*atts.offset(i as isize), attname) != 0 } {
                    xmlErrAttributeDup(ctxt, 0 as *const xmlChar, attname);
                    xmlFree_safe(attvalue as *mut ());
                    current_block = 13942178848302774114;
                    break;
                } else {
                    i += 2 as i32
                }
            }
            match current_block {
                13942178848302774114 => {}
                _ =>
                /*
                 * Add the pair to atts
                 */
                {
                    if atts.is_null() {
                        maxatts = 22 as i32; /* allow for 10 attrs by default */
                        atts =
                            xmlMalloc_safe((maxatts as u64).wrapping_mul(
                                ::std::mem::size_of::<*mut xmlChar>() as u64,
                            )) as *mut *const xmlChar;
                        if atts.is_null() {
                            unsafe {
                                xmlErrMemory(ctxt, 0 as *const i8);
                            }
                            if !attvalue.is_null() {
                                xmlFree_safe(attvalue as *mut ());
                            }
                            current_block = 13942178848302774114;
                        } else {
                            (safe_ctxt).atts = atts;
                            (safe_ctxt).maxatts = maxatts;
                            current_block = 11763295167351361500;
                        }
                    } else if nbatts + 4 as i32 > maxatts {
                        let mut n: *mut *const xmlChar = 0 as *mut *const xmlChar;
                        maxatts *= 2 as i32;
                        n = xmlRealloc_safe(
                            atts as *mut (),
                            (maxatts as u64)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*const xmlChar>() as u64
                                ),
                        ) as *mut *const xmlChar;
                        if n.is_null() {
                            unsafe {
                                xmlErrMemory(ctxt, 0 as *const i8);
                            }
                            if !attvalue.is_null() {
                                xmlFree_safe(attvalue as *mut ());
                            }
                            current_block = 13942178848302774114;
                        } else {
                            atts = n;
                            (safe_ctxt).atts = atts;
                            (safe_ctxt).maxatts = maxatts;
                            current_block = 11763295167351361500;
                        }
                    } else {
                        current_block = 11763295167351361500;
                    }
                    match current_block {
                        13942178848302774114 => {}
                        _ => unsafe {
                            let fresh89 = nbatts;
                            nbatts = nbatts + 1;
                            let ref mut fresh90 = *atts.offset(fresh89 as isize);
                            *fresh90 = attname;
                            let fresh91 = nbatts;
                            nbatts = nbatts + 1;
                            let ref mut fresh92 = *atts.offset(fresh91 as isize);
                            *fresh92 = attvalue;
                            let ref mut fresh93 = *atts.offset(nbatts as isize);
                            *fresh93 = 0 as *const xmlChar;
                            let ref mut fresh94 =
                                *atts.offset((nbatts + 1 as i32) as isize);
                            *fresh94 = 0 as *const xmlChar
                        },
                    }
                }
            }
        } else if !attvalue.is_null() {
            xmlFree_safe(attvalue as *mut ());
        }
        if (safe_ctxt).progressive == 0 as i32
            && unsafe {
                ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
            }
        {
            xmlGROW(ctxt);
        }
        if unsafe {
            *(*(*ctxt).input).cur as i32 == '>' as i32
                || *(*(*ctxt).input).cur as i32 == '/' as i32
                    && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        == '>' as i32
        } {
            break;
        }
        if xmlSkipBlankChars(ctxt) == 0 as i32 {
            unsafe {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"attributes construct error\n\x00" as *const u8 as *const i8,
                );
            }
        }
        if unsafe {
            cons as u64 == (*(*ctxt).input).consumed
                && q == (*(*ctxt).input).cur
                && attname.is_null()
                && attvalue.is_null()
        } {
            unsafe {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"xmlParseStartTag: problem parsing attributes\n\x00" as *const u8
                        as *const i8,
                );
            }
            break;
        } else {
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                        > (2 as i32 * 250 as i32) as i64
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < (2 as i32 * 250 as i32) as i64
                }
            {
                xmlSHRINK(ctxt);
            }
            if (safe_ctxt).progressive == 0 as i32
                && unsafe {
                    ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
                }
            {
                xmlGROW(ctxt);
            }
        }
    }
    /*
     * SAX: Start of Element !
     */
    if !(safe_ctxt).sax.is_null()
        && unsafe { (*(*ctxt).sax).startElement.is_some() }
        && (safe_ctxt).disableSAX == 0
    {
        if nbatts > 0 as i32 {
            unsafe {
                (*(*ctxt).sax)
                    .startElement
                    .expect("non-null function pointer")(
                    (safe_ctxt).userData, name, atts
                );
            }
        } else {
            unsafe {
                (*(*ctxt).sax)
                    .startElement
                    .expect("non-null function pointer")(
                    (safe_ctxt).userData,
                    name,
                    0 as *mut *const xmlChar,
                );
            }
        }
    }
    if !atts.is_null() {
        /* Free only the content strings */
        i = 1 as i32;
        while i < nbatts {
            unsafe {
                if !(*atts.offset(i as isize)).is_null() {
                    xmlFree_safe(*atts.offset(i as isize) as *mut xmlChar as *mut ());
                }
            }
            i += 2 as i32
        }
    }
    return name;
}
/* *
* xmlParseEndTag1:
* @ctxt:  an XML parser context
* @line:  line of the start tag
* @nsNr:  number of namespaces on the start tag
*
* parse an end of tag
*
* [42] ETag ::= '</' Name S? '>'
*
* With namespace
*
* [NS 9] ETag ::= '</' QName S? '>'
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
unsafe fn xmlParseEndTag1(mut ctxt: xmlParserCtxtPtr, mut line: i32) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    //@todo 削减unsafe范围
    unsafe {
        if (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        {
            xmlGROW(ctxt);
        }
        if *(*(*ctxt).input).cur as i32 != '<' as i32
            || *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 != '/' as i32
        {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_LTSLASH_REQUIRED,
                b"xmlParseEndTag: \'</\' not found\n\x00" as *const u8 as *const i8,
            );
            return;
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
        (*(*ctxt).input).col += 2 as i32;
        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
            xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
        }
        name = xmlParseNameAndCompare(ctxt, (*ctxt).name);
        /*
         * We should definitely be at the ending "S? '>'" part
         */
        if (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        {
            xmlGROW(ctxt);
        }
        xmlSkipBlankChars(ctxt);
        if !(0x9 as i32 <= *(*(*ctxt).input).cur as i32
            && *(*(*ctxt).input).cur as i32 <= 0xa as i32
            || *(*(*ctxt).input).cur as i32 == 0xd as i32
            || 0x20 as i32 <= *(*(*ctxt).input).cur as i32)
            || *(*(*ctxt).input).cur as i32 != '>' as i32
        {
            xmlFatalErr(ctxt, XML_ERR_GT_REQUIRED, 0 as *const i8);
        } else {
            (*(*ctxt).input).col += 1;
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
        }
        /*
         * [ WFC: Element Type Match ]
         * The Name in an element's end-tag must match the element type in the
         * start-tag.
         *
         */
        if name != 1 as i32 as *mut xmlChar {
            if name.is_null() {
                name = b"unparsable\x00" as *const u8 as *const i8 as *mut xmlChar
            }
            xmlFatalErrMsgStrIntStr(
                ctxt,
                XML_ERR_TAG_NAME_MISMATCH,
                b"Opening and ending tag mismatch: %s line %d and %s\n\x00" as *const u8
                    as *const i8,
                (*ctxt).name,
                line,
                name,
            );
        }
        /*
         * SAX: End of Tag
         */
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endElement.is_some() && (*ctxt).disableSAX == 0
        {
            (*(*ctxt).sax)
                .endElement
                .expect("non-null function pointer")((*ctxt).userData, (*ctxt).name);
        }
    }
    namePop(ctxt);
    spacePop(ctxt);
}
/* *
* xmlParseEndTag:
* @ctxt:  an XML parser context
*
* parse an end of tag
*
* [42] ETag ::= '</' Name S? '>'
*
* With namespace
*
* [NS 9] ETag ::= '</' QName S? '>'
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlParseEndTag(mut ctxt: xmlParserCtxtPtr) {
    xmlParseEndTag1(ctxt, 0 as i32);
}
/* LIBXML_SAX1_ENABLED */
/* ***********************************************************************
*									*
*		      SAX 2 specific operations				*
*									*
************************************************************************/
/*
* xmlGetNamespace:
* @ctxt:  an XML parser context
* @prefix:  the prefix to lookup
*
* Lookup the namespace name for the @prefix (which ca be NULL)
* The prefix must come from the @ctxt->dict dictionary
*
* Returns the namespace name or NULL if not bound
*/
unsafe fn xmlGetNamespace(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *const xmlChar,
) -> *const xmlChar {
    let mut i: i32 = 0;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if prefix == (safe_ctxt).str_xml {
        return (safe_ctxt).str_xml_ns;
    }
    i = (safe_ctxt).nsNr - 2 as i32;
    while i >= 0 as i32 {
        unsafe {
            if *(*ctxt).nsTab.offset(i as isize) == prefix {
                if prefix.is_null()
                    && **(*ctxt).nsTab.offset((i + 1 as i32) as isize) as i32
                        == 0 as i32
                {
                    return 0 as *const xmlChar;
                }
                return *(*ctxt).nsTab.offset((i + 1 as i32) as isize);
            }
        }
        i -= 2 as i32
    }
    return 0 as *const xmlChar;
}
/* *
* xmlParseQName:
* @ctxt:  an XML parser context
* @prefix:  pointer to store the prefix part
*
* parse an XML Namespace QName
*
* [6]  QName  ::= (Prefix ':')? LocalPart
* [7]  Prefix  ::= NCName
* [8]  LocalPart  ::= NCName
*
* Returns the Name parsed or NULL
*/
unsafe fn xmlParseQName(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *mut *const xmlChar,
) -> *const xmlChar {
    let mut l: *const xmlChar = 0 as *const xmlChar;
    let mut p: *const xmlChar = 0 as *const xmlChar;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).progressive == 0 as i32
        && unsafe {
            ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        }
    {
        xmlGROW(ctxt);
    }
    l = xmlParseNCName(ctxt);
    if l.is_null() {
        unsafe {
            if *(*(*ctxt).input).cur as i32 == ':' as i32 {
                l = xmlParseName(ctxt);
                if !l.is_null() {
                    xmlNsErr(
                        ctxt,
                        XML_NS_ERR_QNAME,
                        b"Failed to parse QName \'%s\'\n\x00" as *const u8 as *const i8,
                        l,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                    *prefix = 0 as *const xmlChar;
                    return l;
                }
            }
        }
        return 0 as *const xmlChar;
    }
    if unsafe { *(*(*ctxt).input).cur as i32 == ':' as i32 } {
        xmlNextChar_safe(ctxt);
        p = l;
        l = xmlParseNCName(ctxt);
        if l.is_null() {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                return 0 as *const xmlChar;
            }
            xmlNsErr(
                ctxt,
                XML_NS_ERR_QNAME,
                b"Failed to parse QName \'%s:\'\n\x00" as *const u8 as *const i8,
                p,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            l = xmlParseNmtoken(ctxt);
            if l.is_null() {
                if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                    return 0 as *const xmlChar;
                }
                tmp = xmlBuildQName_safe(
                    b"\x00" as *const u8 as *const i8 as *mut xmlChar,
                    p,
                    0 as *mut xmlChar,
                    0 as i32,
                )
            } else {
                tmp = xmlBuildQName_safe(l, p, 0 as *mut xmlChar, 0 as i32);
                xmlFree_safe(l as *mut i8 as *mut ());
            }
            p = xmlDictLookup_safe((safe_ctxt).dict, tmp, -(1 as i32));
            if !tmp.is_null() {
                xmlFree_safe(tmp as *mut ());
            }
            unsafe {
                *prefix = 0 as *const xmlChar;
            }
            return p;
        }
        if unsafe { *(*(*ctxt).input).cur as i32 == ':' as i32 } {
            let mut tmp_0: *mut xmlChar = 0 as *mut xmlChar;
            xmlNsErr(
                ctxt,
                XML_NS_ERR_QNAME,
                b"Failed to parse QName \'%s:%s:\'\n\x00" as *const u8 as *const i8,
                p,
                l,
                0 as *const xmlChar,
            );
            xmlNextChar_safe(ctxt);
            tmp_0 = xmlParseName(ctxt) as *mut xmlChar;
            if !tmp_0.is_null() {
                tmp_0 = xmlBuildQName_safe(tmp_0, l, 0 as *mut xmlChar, 0 as i32);
                l = xmlDictLookup_safe((safe_ctxt).dict, tmp_0, -(1 as i32));
                if !tmp_0.is_null() {
                    xmlFree_safe(tmp_0 as *mut ());
                }
                unsafe {
                    *prefix = p;
                }
                return l;
            }
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                return 0 as *const xmlChar;
            }
            tmp_0 = xmlBuildQName_safe(
                b"\x00" as *const u8 as *const i8 as *mut xmlChar,
                l,
                0 as *mut xmlChar,
                0 as i32,
            );
            l = xmlDictLookup_safe((safe_ctxt).dict, tmp_0, -(1 as i32));
            if !tmp_0.is_null() {
                xmlFree_safe(tmp_0 as *mut ());
            }
            unsafe {
                *prefix = p;
            }
            return l;
        }
        unsafe {
            *prefix = p;
        }
    } else {
        unsafe {
            *prefix = 0 as *const xmlChar;
        }
    }
    return l;
}
/* *
* xmlParseQNameAndCompare:
* @ctxt:  an XML parser context
* @name:  the localname
* @prefix:  the prefix, if any.
*
* parse an XML name and compares for match
* (specialized for endtag parsing)
*
* Returns NULL for an illegal name, (xmlChar*) 1 for success
* and the name for mismatch
*/
unsafe fn xmlParseQNameAndCompare(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const xmlChar,
    mut prefix: *const xmlChar,
) -> *const xmlChar {
    let mut cmp: *const xmlChar = 0 as *const xmlChar;
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    let mut prefix2: *const xmlChar = 0 as *const xmlChar;
    if prefix.is_null() {
        return xmlParseNameAndCompare(ctxt, name);
    }
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    in_0 = unsafe { (*(*ctxt).input).cur };
    cmp = prefix;
    unsafe {
        while *in_0 as i32 != 0 as i32
            && *in_0 as i32 == *cmp as i32
        {
            in_0 = in_0.offset(1);
            cmp = cmp.offset(1)
        }
        if *cmp as i32 == 0 as i32 && *in_0 as i32 == ':' as i32 {
            in_0 = in_0.offset(1);
            cmp = name;
            while *in_0 as i32 != 0 as i32
                && *in_0 as i32 == *cmp as i32
            {
                in_0 = in_0.offset(1);
                cmp = cmp.offset(1)
            }
            if *cmp as i32 == 0 as i32
                && (*in_0 as i32 == '>' as i32
                    || (*in_0 as i32 == 0x20 as i32
                        || 0x9 as i32 <= *in_0 as i32
                            && *in_0 as i32 <= 0xa as i32
                        || *in_0 as i32 == 0xd as i32))
            {
                /* success */
                (*(*ctxt).input).col = ((*(*ctxt).input).col as i64
                    + in_0.offset_from((*(*ctxt).input).cur) as i64)
                    as i32;
                (*(*ctxt).input).cur = in_0;
                return 1 as i32 as *const xmlChar;
            }
        }
    }
    /*
     * all strings coms from the dictionary, equality can be done directly
     */
    ret = xmlParseQName(ctxt, &mut prefix2);
    if ret == name && prefix == prefix2 {
        return 1 as i32 as *const xmlChar;
    }
    return ret;
}
/* *
* xmlParseAttValueInternal:
* @ctxt:  an XML parser context
* @len:  attribute len result
* @alloc:  whether the attribute was reallocated as a new string
* @normalize:  if 1 then further non-CDATA normalization must be done
*
* parse a value for an attribute.
* NOTE: if no normalization is needed, the routine will return pointers
*       directly from the data buffer.
*
* 3.3.3 Attribute-Value Normalization:
* Before the value of an attribute is passed to the application or
* checked for validity, the XML processor must normalize it as follows:
* - a character reference is processed by appending the referenced
*   character to the attribute value
* - an entity reference is processed by recursively processing the
*   replacement text of the entity
* - a whitespace character (#x20, #xD, #xA, #x9) is processed by
*   appending #x20 to the normalized value, except that only a single
*   #x20 is appended for a "#xD#xA" sequence that is part of an external
*   parsed entity or the literal entity value of an internal parsed entity
* - other characters are processed by appending them to the normalized value
* If the declared value is not CDATA, then the XML processor must further
* process the normalized attribute value by discarding any leading and
* trailing space (#x20) characters, and by replacing sequences of space
* (#x20) characters by a single space (#x20) character.
* All attributes for which no declaration has been read should be treated
* by a non-validating parser as if declared CDATA.
*
* Returns the AttValue parsed or NULL. The value has to be freed by the
*     caller if it was copied, this can be detected by val[*len] == 0.
*/
unsafe fn xmlParseAttValueInternal(
    mut ctxt: xmlParserCtxtPtr,
    mut len: *mut i32,
    mut alloc: *mut i32,
    mut normalize: i32,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut limit: xmlChar = 0 as i32 as xmlChar;
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut start: *const xmlChar = 0 as *const xmlChar;
    let mut end: *const xmlChar = 0 as *const xmlChar;
    let mut last: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut line: i32 = 0;
    let mut col: i32 = 0;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    unsafe {
        in_0 = (*(*ctxt).input).cur as *mut xmlChar;
        line = (*(*ctxt).input).line;
        col = (*(*ctxt).input).col;
        if *in_0 as i32 != '\"' as i32 && *in_0 as i32 != '\'' as i32 {
            xmlFatalErr(
                ctxt,
                XML_ERR_ATTRIBUTE_NOT_STARTED,
                0 as *const i8,
            );
            return 0 as *mut xmlChar;
        }
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    (safe_ctxt).instate = XML_PARSER_ATTRIBUTE_VALUE;
    /*
     * try to handle in this routine the most common case where no
     * allocation of a new string is required and where content is
     * pure ASCII.
     */
    let fresh95 = in_0;
    unsafe {
        in_0 = in_0.offset(1);
        limit = *fresh95;
        col += 1;
        end = (*(*ctxt).input).end;
    }
    start = in_0;
    if in_0 >= end {
        let mut oldbase: *const xmlChar = unsafe { (*(*ctxt).input).base };
        if (safe_ctxt).progressive == 0 as i32
            && unsafe {
                ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
            }
        {
            xmlGROW(ctxt);
        }
        if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
            return 0 as *mut xmlChar;
        }
        unsafe {
            if oldbase != (*(*ctxt).input).base {
                let mut delta: ptrdiff_t =
                    (*(*ctxt).input).base.offset_from(oldbase) as i64;
                start = start.offset(delta as isize);
                in_0 = in_0.offset(delta as isize)
            }
            end = (*(*ctxt).input).end
        }
    }
    if normalize != 0 {
        /*
         * Skip any leading spaces
         */
        while in_0 < end
            && unsafe { *in_0 } as i32 != limit as i32
            && (unsafe { *in_0 } as i32 == 0x20 as i32
                || unsafe { *in_0 } as i32 == 0x9 as i32
                || unsafe { *in_0 } as i32 == 0xa as i32
                || unsafe { *in_0 } as i32 == 0xd as i32)
        {
            if unsafe { *in_0 } as i32 == 0xa as i32 {
                line += 1;
                col = 1 as i32
            } else {
                col += 1
            }
            in_0 = unsafe { in_0.offset(1) };
            start = in_0;
            if in_0 >= end {
                let mut oldbase_0: *const xmlChar = unsafe { (*(*ctxt).input).base };
                if unsafe {
                    (*ctxt).progressive == 0 as i32
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                } {
                    xmlGROW(ctxt);
                }
                if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                    return 0 as *mut xmlChar;
                }
                unsafe {
                    if oldbase_0 != (*(*ctxt).input).base {
                        let mut delta_0: ptrdiff_t =
                            (*(*ctxt).input).base.offset_from(oldbase_0) as i64;
                        start = start.offset(delta_0 as isize);
                        in_0 = in_0.offset(delta_0 as isize)
                    }
                    end = (*(*ctxt).input).end;
                    if in_0.offset_from(start) as i64
                        > 10000000 as i32 as i64
                        && (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
                    {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_ATTRIBUTE_NOT_FINISHED,
                            b"AttValue length too long\n\x00" as *const u8 as *const i8,
                        );
                        return 0 as *mut xmlChar;
                    }
                }
            }
        }
        while in_0 < end
            && unsafe { *in_0 } as i32 != limit as i32
            && unsafe { *in_0 } as i32 >= 0x20 as i32
            && unsafe { *in_0 } as i32 <= 0x7f as i32
            && unsafe { *in_0 } as i32 != '&' as i32
            && unsafe { *in_0 } as i32 != '<' as i32
        {
            col += 1;
            let fresh96 = in_0;
            in_0 = unsafe { in_0.offset(1) };
            if unsafe { *fresh96 } as i32 == 0x20 as i32
                && unsafe { *in_0 } as i32 == 0x20 as i32
            {
                break;
            }
            if in_0 >= end {
                let mut oldbase_1: *const xmlChar = unsafe { (*(*ctxt).input).base };
                if (safe_ctxt).progressive == 0 as i32
                    && unsafe {
                        ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                    }
                {
                    xmlGROW(ctxt);
                }
                if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                    return 0 as *mut xmlChar;
                }
                unsafe {
                    if oldbase_1 != (*(*ctxt).input).base {
                        let mut delta_1: ptrdiff_t =
                            (*(*ctxt).input).base.offset_from(oldbase_1) as i64;
                        start = start.offset(delta_1 as isize);
                        in_0 = in_0.offset(delta_1 as isize)
                    }
                    end = (*(*ctxt).input).end;
                    if in_0.offset_from(start) as i64
                        > 10000000 as i32 as i64
                        && (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
                    {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_ATTRIBUTE_NOT_FINISHED,
                            b"AttValue length too long\n\x00" as *const u8 as *const i8,
                        );
                        return 0 as *mut xmlChar;
                    }
                }
            }
        }
        last = in_0;
        /*
         * skip the trailing blanks
         */
        unsafe {
            while *last.offset(-(1 as i32) as isize) as i32 == 0x20 as i32
                && last > start
            {
                last = last.offset(-1)
            }
        }
        while in_0 < end
            && unsafe { *in_0 } as i32 != limit as i32
            && (unsafe { *in_0 } as i32 == 0x20 as i32
                || unsafe { *in_0 } as i32 == 0x9 as i32
                || unsafe { *in_0 } as i32 == 0xa as i32
                || unsafe { *in_0 } as i32 == 0xd as i32)
        {
            if unsafe { *in_0 } as i32 == 0xa as i32 {
                line += 1;
                col = 1 as i32
            } else {
                col += 1
            }
            in_0 = unsafe { in_0.offset(1) };
            if in_0 >= end {
                let mut oldbase_2: *const xmlChar = unsafe { (*(*ctxt).input).base };
                if (safe_ctxt).progressive == 0 as i32
                    && unsafe {
                        ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                    }
                {
                    xmlGROW(ctxt);
                }
                if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                    return 0 as *mut xmlChar;
                }
                unsafe {
                    if oldbase_2 != (*(*ctxt).input).base {
                        let mut delta_2: ptrdiff_t =
                            (*(*ctxt).input).base.offset_from(oldbase_2) as i64;
                        start = start.offset(delta_2 as isize);
                        in_0 = in_0.offset(delta_2 as isize);
                        last = last.offset(delta_2 as isize)
                    }
                    end = (*(*ctxt).input).end;
                    if in_0.offset_from(start) as i64
                        > 10000000 as i32 as i64
                        && (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
                    {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_ATTRIBUTE_NOT_FINISHED,
                            b"AttValue length too long\n\x00" as *const u8 as *const i8,
                        );
                        return 0 as *mut xmlChar;
                    }
                }
            }
        }
        unsafe {
            if in_0.offset_from(start) as i64 > 10000000 as i32 as i64
                && (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
            {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_ATTRIBUTE_NOT_FINISHED,
                    b"AttValue length too long\n\x00" as *const u8 as *const i8,
                );
                return 0 as *mut xmlChar;
            }
        }
        if unsafe { *in_0 } as i32 != limit as i32 {
            current_block = 78912414019074375;
        } else {
            current_block = 17736998403848444560;
        }
    } else {
        while in_0 < end
            && unsafe { *in_0 } as i32 != limit as i32
            && unsafe { *in_0 } as i32 >= 0x20 as i32
            && unsafe { *in_0 } as i32 <= 0x7f as i32
            && unsafe { *in_0 } as i32 != '&' as i32
            && unsafe { *in_0 } as i32 != '<' as i32
        {
            in_0 = unsafe { in_0.offset(1) };
            col += 1;
            if in_0 >= end {
                let mut oldbase_3: *const xmlChar = unsafe { (*(*ctxt).input).base };
                if (safe_ctxt).progressive == 0 as i32
                    && unsafe {
                        ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                    }
                {
                    xmlGROW(ctxt);
                }
                if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                    return 0 as *mut xmlChar;
                }
                unsafe {
                    if oldbase_3 != (*(*ctxt).input).base {
                        let mut delta_3: ptrdiff_t =
                            (*(*ctxt).input).base.offset_from(oldbase_3) as i64;
                        start = start.offset(delta_3 as isize);
                        in_0 = in_0.offset(delta_3 as isize)
                    }
                    end = (*(*ctxt).input).end;
                    if in_0.offset_from(start) as i64
                        > 10000000 as i32 as i64
                        && (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
                    {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_ATTRIBUTE_NOT_FINISHED,
                            b"AttValue length too long\n\x00" as *const u8 as *const i8,
                        );
                        return 0 as *mut xmlChar;
                    }
                }
            }
        }
        last = in_0;
        unsafe {
            if in_0.offset_from(start) as i64 > 10000000 as i32 as i64
                && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
            {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_ATTRIBUTE_NOT_FINISHED,
                    b"AttValue length too long\n\x00" as *const u8 as *const i8,
                );
                return 0 as *mut xmlChar;
            }
        }

        if unsafe { *in_0 } as i32 != limit as i32 {
            current_block = 78912414019074375;
        } else {
            current_block = 17736998403848444560;
        }
    }
    match current_block {
        78912414019074375 => {
            if !alloc.is_null() {
                unsafe { *alloc = 1 as i32 }
            }
            return xmlParseAttValueComplex(ctxt, len, normalize);
        }
        _ => {
            in_0 = unsafe { in_0.offset(1) };
            col += 1;
            if !len.is_null() {
                unsafe {
                    *len = last.offset_from(start) as i64 as i32;
                }
                ret = start as *mut xmlChar
            } else {
                if !alloc.is_null() {
                    unsafe { *alloc = 1 as i32 }
                }
                ret = unsafe {
                    xmlStrndup_safe(
                        start,
                        last.offset_from(start) as i64 as i32,
                    )
                };
            }
            unsafe {
                (*(*ctxt).input).cur = in_0;
                (*(*ctxt).input).line = line;
                (*(*ctxt).input).col = col;
                if !alloc.is_null() {
                    *alloc = 0 as i32
                }
            }
            return ret;
        }
    };
}
/* *
* xmlParseAttribute2:
* @ctxt:  an XML parser context
* @pref:  the element prefix
* @elem:  the element name
* @prefix:  a xmlChar ** used to store the value of the attribute prefix
* @value:  a xmlChar ** used to store the value of the attribute
* @len:  an int * to save the length of the attribute
* @alloc:  an int * to indicate if the attribute was allocated
*
* parse an attribute in the new SAX2 framework.
*
* Returns the attribute name, and the value in *value, .
*/
unsafe fn xmlParseAttribute2(
    mut ctxt: xmlParserCtxtPtr,
    mut pref: *const xmlChar,
    mut elem: *const xmlChar,
    mut prefix: *mut *const xmlChar,
    mut value: *mut *mut xmlChar,
    mut len: *mut i32,
    mut alloc: *mut i32,
) -> *const xmlChar {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    let mut internal_val: *mut xmlChar = 0 as *mut xmlChar;
    let mut normalize: i32 = 0 as i32;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    unsafe { *value = 0 as *mut xmlChar };
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    name = xmlParseQName(ctxt, prefix);
    if name.is_null() {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"error parsing attribute name\n\x00" as *const u8 as *const i8,
            );
        }
        return 0 as *const xmlChar;
    }
    /*
     * get the type if needed
     */
    if !(safe_ctxt).attsSpecial.is_null() {
        let mut type_0: i32 = 0;
        type_0 = xmlHashQLookup2_safe(
            (safe_ctxt).attsSpecial,
            pref,
            elem,
            unsafe { *prefix },
            name,
        ) as ptrdiff_t as i32;
        if type_0 != 0 as i32 {
            normalize = 1 as i32
        }
    }
    /*
     * read the value
     */
    xmlSkipBlankChars(ctxt);
    if unsafe { *(*(*ctxt).input).cur as i32 == '=' as i32 } {
        xmlNextChar_safe(ctxt);
        xmlSkipBlankChars(ctxt);
        val = xmlParseAttValueInternal(ctxt, len, alloc, normalize);
        if normalize != 0 {
            /*
             * Sometimes a second normalisation pass for spaces is needed
             * but that only happens if charrefs or entities references
             * have been used in the attribute value, i.e. the attribute
             * value have been extracted in an allocated string already.
             */
            if unsafe { *alloc != 0 } {
                let mut val2: *const xmlChar = 0 as *const xmlChar;
                val2 = xmlAttrNormalizeSpace2(ctxt, val, len);
                if !val2.is_null() && val2 != val {
                    xmlFree_safe(val as *mut ());
                    val = val2 as *mut xmlChar
                }
            }
        }
        (safe_ctxt).instate = XML_PARSER_CONTENT
    } else {
        unsafe {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_ATTRIBUTE_WITHOUT_VALUE,
                b"Specification mandates value for attribute %s\n\x00" as *const u8
                    as *const i8,
                name,
            );
        }
        return 0 as *const xmlChar;
    }
    if unsafe { *prefix } == (safe_ctxt).str_xml {
        /*
         * Check that xml:lang conforms to the specification
         * No more registered as an error, just generate a warning now
         * since this was deprecated in XML second edition
         */
        if (safe_ctxt).pedantic != 0
            && xmlStrEqual_safe(
                name,
                b"lang\x00" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
        {
            internal_val = xmlStrndup_safe(val, unsafe { *len });
            if xmlCheckLanguageID(internal_val) == 0 {
                unsafe {
                    xmlWarningMsg(
                        ctxt,
                        XML_WAR_LANG_VALUE,
                        b"Malformed value for xml:lang : %s\n\x00" as *const u8
                            as *const i8,
                        internal_val,
                        0 as *const xmlChar,
                    );
                }
            }
        }
        /*
         * Check that xml:space conforms to the specification
         */
        if xmlStrEqual_safe(
            name,
            b"space\x00" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
            internal_val = xmlStrndup_safe(val, unsafe { *len });
            if xmlStrEqual_safe(
                internal_val,
                b"default\x00" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
            {
                unsafe { *(*ctxt).space = 0 as i32 }
            } else if xmlStrEqual_safe(
                internal_val,
                b"preserve\x00" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
            {
                unsafe { *(*ctxt).space = 1 as i32 }
            } else {
                unsafe {
                    xmlWarningMsg(ctxt, XML_WAR_SPACE_VALUE,
              b"Invalid value \"%s\" for xml:space : \"default\" or \"preserve\" expected\n\x00"
                  as *const u8 as *const i8,
              internal_val, 0 as *const xmlChar);
                }
            }
        }
        if !internal_val.is_null() {
            xmlFree_safe(internal_val as *mut ());
        }
    }
    unsafe {
        *value = val;
    }
    return name;
}
/* *
* xmlParseStartTag2:
* @ctxt:  an XML parser context
*
* parse a start of tag either for rule element or
* EmptyElement. In both case we don't parse the tag closing chars.
* This routine is called when running SAX2 parsing
*
* [40] STag ::= '<' Name (S Attribute)* S? '>'
*
* [ WFC: Unique Att Spec ]
* No attribute name may appear more than once in the same start-tag or
* empty-element tag.
*
* [44] EmptyElemTag ::= '<' Name (S Attribute)* S? '/>'
*
* [ WFC: Unique Att Spec ]
* No attribute name may appear more than once in the same start-tag or
* empty-element tag.
*
* With namespace:
*
* [NS 8] STag ::= '<' QName (S Attribute)* S? '>'
*
* [NS 10] EmptyElement ::= '<' QName (S Attribute)* S? '/>'
*
* Returns the element name parsed
*/
unsafe fn xmlParseStartTag2(
    mut ctxt: xmlParserCtxtPtr,
    mut pref: *mut *const xmlChar,
    mut URI: *mut *const xmlChar,
    mut tlen: *mut i32,
) -> *const xmlChar {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut current_block: u64;
    let mut localname: *const xmlChar = 0 as *const xmlChar;
    let mut prefix: *const xmlChar = 0 as *const xmlChar;
    let mut attname: *const xmlChar = 0 as *const xmlChar;
    let mut aprefix: *const xmlChar = 0 as *const xmlChar;
    let mut nsname: *const xmlChar = 0 as *const xmlChar;
    let mut attvalue: *mut xmlChar = 0 as *mut xmlChar;
    let mut atts: *mut *const xmlChar = (safe_ctxt).atts;
    let mut maxatts: i32 = (safe_ctxt).maxatts;
    let mut nratts: i32 = 0;
    let mut nbatts: i32 = 0;
    let mut nbdef: i32 = 0;
    let mut inputid: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut nbNs: i32 = 0;
    let mut attval: i32 = 0;
    let mut cur: u64 = 0;
    let mut nsNr: i32 = (safe_ctxt).nsNr;
    unsafe {
        if *(*(*ctxt).input).cur as i32 != '<' as i32 {
            return 0 as *const xmlChar;
        }
        (*(*ctxt).input).col += 1;
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
            xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
        }
        /*
         * NOTE: it is crucial with the SAX2 API to never call SHRINK beyond that
         *       point since the attribute values may be stored as pointers to
         *       the buffer and calling SHRINK would destroy them !
         *       The Shrinking is only possible once the full set of attribute
         *       callbacks have been done.
         */
        if (*ctxt).progressive == 0 as i32
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > (2 as i32 * 250 as i32) as i64
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < (2 as i32 * 250 as i32) as i64
        {
            xmlSHRINK(ctxt);
        }
        cur = (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
            as u64;
        inputid = (*(*ctxt).input).id;
    }
    nbatts = 0 as i32;
    nratts = 0 as i32;
    nbdef = 0 as i32;
    nbNs = 0 as i32;
    attval = 0 as i32;
    /* Forget any namespaces added during an earlier parse of this element. */
    (safe_ctxt).nsNr = nsNr;
    localname = xmlParseQName(ctxt, &mut prefix);
    if localname.is_null() {
        unsafe {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"StartTag: invalid element name\n\x00" as *const u8 as *const i8,
            );
        }
        return 0 as *const xmlChar;
    }
    unsafe {
        *tlen = ((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
            as u64)
            .wrapping_sub(cur) as i32;
    }
    /*
     * Now parse the attributes, it ends up with the ending
     *
     * (S Attribute)* S?
     */
    xmlSkipBlankChars(ctxt);
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    loop {
        if unsafe {
            !(*(*(*ctxt).input).cur as i32 != '>' as i32
                && (*(*(*ctxt).input).cur as i32 != '/' as i32
                    || *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        != '>' as i32)
                && (0x9 as i32 <= *(*(*ctxt).input).cur as i32
                    && *(*(*ctxt).input).cur as i32 <= 0xa as i32
                    || *(*(*ctxt).input).cur as i32 == 0xd as i32
                    || 0x20 as i32 <= *(*(*ctxt).input).cur as i32)
                && (*ctxt).instate as i32 != XML_PARSER_EOF as i32)
        } {
            current_block = 9587810615301548814;
            break;
        }
        let mut q: *const xmlChar = unsafe { (*(*ctxt).input).cur };
        let mut cons: u32 = unsafe { (*(*ctxt).input).consumed as u32 };
        let mut len: i32 = -(1 as i32);
        let mut alloc: i32 = 0 as i32;
        attname = xmlParseAttribute2(
            ctxt,
            prefix,
            localname,
            &mut aprefix,
            &mut attvalue,
            &mut len,
            &mut alloc,
        );
        if !(attname.is_null() || attvalue.is_null()) {
            if len < 0 as i32 {
                len = xmlStrlen_safe(attvalue)
            }
            if attname == (safe_ctxt).str_xmlns && aprefix.is_null() {
                let mut URL: *const xmlChar = xmlDictLookup_safe((safe_ctxt).dict, attvalue, len);
                let mut uri: xmlURIPtr = 0 as *mut xmlURI;
                if URL.is_null() {
                    unsafe {
                        xmlErrMemory(
                            ctxt,
                            b"dictionary allocation failure\x00" as *const u8
                                as *const i8,
                        );
                    }
                    if !attvalue.is_null() && alloc != 0 as i32 {
                        xmlFree_safe(attvalue as *mut ());
                    }
                    localname = 0 as *const xmlChar;
                    current_block = 5568905272147674894;
                    break;
                } else {
                    if unsafe { *URL } as i32 != 0 as i32 {
                        uri = xmlParseURI_safe(URL as *const i8);
                        if uri.is_null() {
                            xmlNsErr(
                                ctxt,
                                XML_WAR_NS_URI,
                                b"xmlns: \'%s\' is not a valid URI\n\x00" as *const u8
                                    as *const i8,
                                URL,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                            );
                        } else {
                            if unsafe { (*uri).scheme.is_null() } {
                                xmlNsWarn(
                                    ctxt,
                                    XML_WAR_NS_URI_RELATIVE,
                                    b"xmlns: URI %s is not absolute\n\x00" as *const u8
                                        as *const i8,
                                    URL,
                                    0 as *const xmlChar,
                                    0 as *const xmlChar,
                                );
                            }
                            xmlFreeURI_safe(uri);
                        }
                        if URL == (safe_ctxt).str_xml_ns {
                            if attname != (safe_ctxt).str_xml {
                                xmlNsErr(
                                    ctxt,
                                    XML_NS_ERR_XML_NAMESPACE,
                                    b"xml namespace URI cannot be the default namespace\n\x00"
                                        as *const u8
                                        as *const i8,
                                    0 as *const xmlChar,
                                    0 as *const xmlChar,
                                    0 as *const xmlChar,
                                );
                            }
                            current_block = 1905178534984964470;
                        } else if len == 29 as i32
                            && xmlStrEqual_safe(
                                URL,
                                b"http://www.w3.org/2000/xmlns/\x00" as *const u8
                                    as *const i8
                                    as *mut xmlChar,
                            ) != 0
                        {
                            xmlNsErr(
                                ctxt,
                                XML_NS_ERR_XML_NAMESPACE,
                                b"reuse of the xmlns namespace name is forbidden\n\x00" as *const u8
                                    as *const i8,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                            );
                            current_block = 1905178534984964470;
                        } else {
                            current_block = 1623252117315916725;
                        }
                    } else {
                        current_block = 1623252117315916725;
                    }
                    match current_block {
                        1905178534984964470 => {}
                        _ => {
                            /*
                             * check that it's not a defined namespace
                             */
                            j = 1 as i32;
                            while j <= nbNs {
                                if unsafe {
                                    (*(*ctxt)
                                        .nsTab
                                        .offset(((*ctxt).nsNr - 2 as i32 * j) as isize))
                                    .is_null()
                                } {
                                    break;
                                }
                                j += 1
                            }
                            if j <= nbNs {
                                xmlErrAttributeDup(ctxt, 0 as *const xmlChar, attname);
                            } else if nsPush(ctxt, 0 as *const xmlChar, URL) > 0 as i32 {
                                nbNs += 1
                            }
                        }
                    }
                }
            } else if aprefix == (safe_ctxt).str_xmlns {
                let mut URL_0: *const xmlChar = xmlDictLookup_safe((safe_ctxt).dict, attvalue, len);
                let mut uri_0: xmlURIPtr = 0 as *mut xmlURI;
                if attname == (safe_ctxt).str_xml {
                    if URL_0 != (safe_ctxt).str_xml_ns {
                        xmlNsErr(
                            ctxt,
                            XML_NS_ERR_XML_NAMESPACE,
                            b"xml namespace prefix mapped to wrong URI\n\x00" as *const u8
                                as *const i8,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                        );
                    }
                } else if URL_0 == (safe_ctxt).str_xml_ns {
                    if attname != (safe_ctxt).str_xml {
                        xmlNsErr(
                            ctxt,
                            XML_NS_ERR_XML_NAMESPACE,
                            b"xml namespace URI mapped to wrong prefix\n\x00" as *const u8
                                as *const i8,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                        );
                    }
                } else if attname == (safe_ctxt).str_xmlns {
                    xmlNsErr(
                        ctxt,
                        XML_NS_ERR_XML_NAMESPACE,
                        b"redefinition of the xmlns prefix is forbidden\n\x00" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                } else if len == 29 as i32
                    && xmlStrEqual_safe(
                        URL_0,
                        b"http://www.w3.org/2000/xmlns/\x00" as *const u8 as *const i8
                            as *mut xmlChar,
                    ) != 0
                {
                    xmlNsErr(
                        ctxt,
                        XML_NS_ERR_XML_NAMESPACE,
                        b"reuse of the xmlns namespace name is forbidden\n\x00" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                } else if URL_0.is_null()
                    || unsafe {
                        *URL_0.offset(0 as i32 as isize) as i32 == 0 as i32
                    }
                {
                    xmlNsErr(
                        ctxt,
                        XML_NS_ERR_XML_NAMESPACE,
                        b"xmlns:%s: Empty XML namespace is not allowed\n\x00" as *const u8
                            as *const i8,
                        attname,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                } else {
                    uri_0 = xmlParseURI_safe(URL_0 as *const i8);
                    if uri_0.is_null() {
                        xmlNsErr(
                            ctxt,
                            XML_WAR_NS_URI,
                            b"xmlns:%s: \'%s\' is not a valid URI\n\x00" as *const u8
                                as *const i8,
                            attname,
                            URL_0,
                            0 as *const xmlChar,
                        );
                    } else {
                        if (safe_ctxt).pedantic != 0 && unsafe { (*uri_0).scheme.is_null() } {
                            xmlNsWarn(
                                ctxt,
                                XML_WAR_NS_URI_RELATIVE,
                                b"xmlns:%s: URI %s is not absolute\n\x00" as *const u8
                                    as *const i8,
                                attname,
                                URL_0,
                                0 as *const xmlChar,
                            );
                        }
                        xmlFreeURI_safe(uri_0);
                    }
                    /*
                     * check that it's not a defined namespace
                     */
                    j = 1 as i32;
                    while j <= nbNs {
                        if unsafe {
                            *(*ctxt)
                                .nsTab
                                .offset(((*ctxt).nsNr - 2 as i32 * j) as isize)
                                == attname
                        } {
                            break;
                        }
                        j += 1
                    }
                    if j <= nbNs {
                        xmlErrAttributeDup(ctxt, aprefix, attname);
                    } else if nsPush(ctxt, attname, URL_0) > 0 as i32 {
                        nbNs += 1
                    }
                }
            } else {
                /*
                 * Add the pair to atts
                 */
                if atts.is_null() || nbatts + 5 as i32 > maxatts {
                    if xmlCtxtGrowAttrs(ctxt, nbatts + 5 as i32) < 0 as i32 {
                        current_block = 1905178534984964470;
                    } else {
                        maxatts = (safe_ctxt).maxatts;
                        atts = (safe_ctxt).atts;
                        current_block = 2463987395154258233;
                    }
                } else {
                    current_block = 2463987395154258233;
                }
                match current_block {
                    1905178534984964470 => {}
                    _ => {
                        unsafe {
                            let fresh97 = nratts;
                            nratts = nratts + 1;
                            *(*ctxt).attallocs.offset(fresh97 as isize) = alloc;
                            let fresh98 = nbatts;
                            nbatts = nbatts + 1;
                            let ref mut fresh99 = *atts.offset(fresh98 as isize);
                            *fresh99 = attname;
                            let fresh100 = nbatts;
                            nbatts = nbatts + 1;
                            let ref mut fresh101 = *atts.offset(fresh100 as isize);
                            *fresh101 = aprefix;
                            /*
                             * The namespace URI field is used temporarily to point at the
                             * base of the current input buffer for non-alloced attributes.
                             * When the input buffer is reallocated, all the pointers become
                             * invalid, but they can be reconstructed later.
                             */
                            if alloc != 0 {
                                let fresh102 = nbatts;
                                nbatts = nbatts + 1;
                                let ref mut fresh103 = *atts.offset(fresh102 as isize);
                                *fresh103 = 0 as *const xmlChar
                            } else {
                                let fresh104 = nbatts;
                                nbatts = nbatts + 1;
                                let ref mut fresh105 = *atts.offset(fresh104 as isize);
                                *fresh105 = (*(*ctxt).input).base
                            }
                            let fresh106 = nbatts;
                            nbatts = nbatts + 1;
                            let ref mut fresh107 = *atts.offset(fresh106 as isize);
                            *fresh107 = attvalue;
                            attvalue = attvalue.offset(len as isize);
                            let fresh108 = nbatts;
                            nbatts = nbatts + 1;
                            let ref mut fresh109 = *atts.offset(fresh108 as isize);
                            *fresh109 = attvalue;
                        }
                        /*
                         * tag if some deallocation is needed
                         */
                        if alloc != 0 as i32 {
                            attval = 1 as i32
                        }
                        attvalue = 0 as *mut xmlChar
                    }
                }
                /* moved into atts */
            }
        }
        /*
         * Do not keep a namespace definition node
         */
        if !attvalue.is_null() && alloc != 0 as i32 {
            xmlFree_safe(attvalue as *mut ());
            attvalue = 0 as *mut xmlChar
        }
        if unsafe {
            (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
        } {
            xmlGROW(ctxt);
        }
        if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
            current_block = 9587810615301548814;
            break;
        }
        if unsafe {
            *(*(*ctxt).input).cur as i32 == '>' as i32
                || *(*(*ctxt).input).cur as i32 == '/' as i32
                    && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        == '>' as i32
        } {
            current_block = 9587810615301548814;
            break;
        }
        if xmlSkipBlankChars(ctxt) == 0 as i32 {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"attributes construct error\n\x00" as *const u8 as *const i8,
            );
            current_block = 9587810615301548814;
            break;
        } else if unsafe {
            cons as u64 == (*(*ctxt).input).consumed
                && q == (*(*ctxt).input).cur
                && attname.is_null()
                && attvalue.is_null()
        } {
            xmlFatalErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"xmlParseStartTag: problem parsing attributes\n\x00" as *const u8
                    as *const i8,
            );
            current_block = 9587810615301548814;
            break;
        } else if unsafe {
            (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
        } {
            xmlGROW(ctxt);
        }
    }
    match current_block {
        9587810615301548814 => {
            if unsafe { (*(*ctxt).input).id != inputid } {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"Unexpected change of input\n\x00" as *const u8 as *const i8,
                );
                localname = 0 as *const xmlChar
            } else {
                /* Reconstruct attribute value pointers. */
                i = 0 as i32;
                j = 0 as i32;
                while j < nratts {
                    unsafe {
                        if !(*atts.offset((i + 2 as i32) as isize)).is_null() {
                            /*
                             * Arithmetic on dangling pointers is technically undefined
                             * behavior, but well...
                             */
                            let mut offset: ptrdiff_t = (*(*ctxt).input)
                                .base
                                .offset_from(*atts.offset((i + 2 as i32) as isize))
                                as i64; /* Reset repurposed namespace URI */
                            let ref mut fresh110 = *atts.offset((i + 2 as i32) as isize); /* value */
                            *fresh110 = 0 as *const xmlChar;
                            let ref mut fresh111 = *atts.offset((i + 3 as i32) as isize);
                            *fresh111 = (*fresh111).offset(offset as isize);
                            let ref mut fresh112 = *atts.offset((i + 4 as i32) as isize);
                            *fresh112 = (*fresh112).offset(offset as isize)
                        }
                    }
                    i += 5 as i32;
                    j += 1
                }
                /*
                 * The attributes defaulting
                 */
                if !(safe_ctxt).attsDefault.is_null() {
                    let mut defaults: xmlDefAttrsPtr = 0 as *mut xmlDefAttrs;
                    defaults = xmlHashLookup2_safe((safe_ctxt).attsDefault, localname, prefix)
                        as xmlDefAttrsPtr;
                    if !defaults.is_null() {
                        i = 0 as i32;
                        loop {
                            unsafe {
                                if !(i < (*defaults).nbAttrs) {
                                    current_block = 981657943452992752;
                                    break;
                                }
                                attname = *(*defaults)
                                    .values
                                    .as_mut_ptr()
                                    .offset((5 as i32 * i) as isize);
                                aprefix = *(*defaults)
                                    .values
                                    .as_mut_ptr()
                                    .offset((5 as i32 * i + 1 as i32) as isize);
                            }

                            /*
                             * special work for namespaces defaulted defs
                             */
                            if attname == (safe_ctxt).str_xmlns && aprefix.is_null() {
                                /*
                                 * check that it's not a defined namespace
                                 */
                                j = 1 as i32;
                                while j <= nbNs {
                                    if unsafe {
                                        (*(*ctxt)
                                            .nsTab
                                            .offset(((*ctxt).nsNr - 2 as i32 * j) as isize))
                                        .is_null()
                                    } {
                                        break;
                                    }
                                    j += 1
                                }
                                if !(j <= nbNs) {
                                    nsname = xmlGetNamespace(ctxt, 0 as *const xmlChar);
                                    if unsafe {
                                        nsname
                                            != *(*defaults).values.as_mut_ptr().offset(
                                                (5 as i32 * i + 2 as i32) as isize,
                                            )
                                    } {
                                        if unsafe {
                                            nsPush(
                                                ctxt,
                                                0 as *const xmlChar,
                                                *(*defaults).values.as_mut_ptr().offset(
                                                    (5 as i32 * i + 2 as i32)
                                                        as isize,
                                                ),
                                            ) > 0 as i32
                                        } {
                                            nbNs += 1
                                        }
                                    }
                                }
                            } else if aprefix == (safe_ctxt).str_xmlns {
                                /*
                                 * check that it's not a defined namespace
                                 */
                                j = 1 as i32;
                                while j <= nbNs {
                                    if unsafe {
                                        *(*ctxt)
                                            .nsTab
                                            .offset(((*ctxt).nsNr - 2 as i32 * j) as isize)
                                            == attname
                                    } {
                                        break;
                                    }
                                    j += 1
                                }
                                if !(j <= nbNs) {
                                    nsname = xmlGetNamespace(ctxt, attname);
                                    if unsafe {
                                        nsname
                                            != *(*defaults)
                                                .values
                                                .as_mut_ptr()
                                                .offset(2 as i32 as isize)
                                    } {
                                        if unsafe {
                                            nsPush(
                                                ctxt,
                                                attname,
                                                *(*defaults).values.as_mut_ptr().offset(
                                                    (5 as i32 * i + 2 as i32)
                                                        as isize,
                                                ),
                                            ) > 0 as i32
                                        } {
                                            nbNs += 1
                                        }
                                    }
                                }
                            } else {
                                /*
                                 * check that it's not a defined attribute
                                 */
                                j = 0 as i32;
                                while j < nbatts {
                                    unsafe {
                                        if attname == *atts.offset(j as isize)
                                            && aprefix
                                                == *atts.offset((j + 1 as i32) as isize)
                                        {
                                            break;
                                        }
                                    }
                                    j += 5 as i32
                                }
                                if !(j < nbatts) {
                                    if atts.is_null() || nbatts + 5 as i32 > maxatts {
                                        if xmlCtxtGrowAttrs(ctxt, nbatts + 5 as i32)
                                            < 0 as i32
                                        {
                                            localname = 0 as *const xmlChar;
                                            current_block = 5568905272147674894;
                                            break;
                                        } else {
                                            maxatts = (safe_ctxt).maxatts;
                                            atts = (safe_ctxt).atts
                                        }
                                    }
                                    unsafe {
                                        let fresh113 = nbatts;
                                        nbatts = nbatts + 1;
                                        let ref mut fresh114 = *atts.offset(fresh113 as isize);
                                        *fresh114 = attname;
                                        let fresh115 = nbatts;
                                        nbatts = nbatts + 1;
                                        let ref mut fresh116 = *atts.offset(fresh115 as isize);
                                        *fresh116 = aprefix;
                                        if aprefix.is_null() {
                                            let fresh117 = nbatts;
                                            nbatts = nbatts + 1;
                                            let ref mut fresh118 = *atts.offset(fresh117 as isize);
                                            *fresh118 = 0 as *const xmlChar
                                        } else {
                                            let fresh119 = nbatts;
                                            nbatts = nbatts + 1;
                                            let ref mut fresh120 = *atts.offset(fresh119 as isize);
                                            *fresh120 = xmlGetNamespace(ctxt, aprefix)
                                        }
                                        let fresh121 = nbatts;
                                        nbatts = nbatts + 1;
                                        let ref mut fresh122 = *atts.offset(fresh121 as isize);
                                        *fresh122 = *(*defaults).values.as_mut_ptr().offset(
                                            (5 as i32 * i + 2 as i32) as isize,
                                        );
                                        let fresh123 = nbatts;
                                        nbatts = nbatts + 1;
                                        let ref mut fresh124 = *atts.offset(fresh123 as isize);
                                        *fresh124 = *(*defaults).values.as_mut_ptr().offset(
                                            (5 as i32 * i + 3 as i32) as isize,
                                        );
                                    }
                                    if (safe_ctxt).standalone == 1 as i32
                                        && unsafe {
                                            !(*(*defaults).values.as_mut_ptr().offset(
                                                (5 as i32 * i + 4 as i32) as isize,
                                            ))
                                            .is_null()
                                        }
                                    {
                                        xmlValidityError(ctxt,
                                     XML_DTD_STANDALONE_DEFAULTED,
                                     b"standalone: attribute %s on %s defaulted from external subset\n\x00"
                                         as *const u8 as
                                         *const i8,
                                     attname, localname);
                                    }
                                    nbdef += 1
                                }
                            }
                            i += 1
                        }
                    } else {
                        current_block = 981657943452992752;
                    }
                } else {
                    current_block = 981657943452992752;
                }
                match current_block {
                    5568905272147674894 => {}
                    _ => {
                        /*
                         * The attributes checkings
                         */
                        i = 0 as i32;
                        while i < nbatts {
                            /*
                             * The default namespace does not apply to attribute names.
                             */
                            if unsafe { !(*atts.offset((i + 1 as i32) as isize)).is_null() }
                            {
                                unsafe {
                                    nsname = xmlGetNamespace(
                                        ctxt,
                                        *atts.offset((i + 1 as i32) as isize),
                                    );
                                    if nsname.is_null() {
                                        xmlNsErr(
                                            ctxt,
                                            XML_NS_ERR_UNDEFINED_NAMESPACE,
                                            b"Namespace prefix %s for %s on %s is not defined\n\x00"
                                                as *const u8
                                                as *const i8,
                                            *atts.offset((i + 1 as i32) as isize),
                                            *atts.offset(i as isize),
                                            localname,
                                        );
                                    }
                                    let ref mut fresh125 =
                                        *atts.offset((i + 2 as i32) as isize);
                                    *fresh125 = nsname
                                }
                            } else {
                                nsname = 0 as *const xmlChar
                            }
                            /*
                             * [ WFC: Unique Att Spec ]
                             * No attribute name may appear more than once in the same
                             * start-tag or empty-element tag.
                             * As extended by the Namespace in XML REC.
                             */
                            j = 0 as i32;
                            while j < i {
                                unsafe {
                                    if *atts.offset(i as isize) == *atts.offset(j as isize) {
                                        if *atts.offset((i + 1 as i32) as isize)
                                            == *atts.offset((j + 1 as i32) as isize)
                                        {
                                            xmlErrAttributeDup(
                                                ctxt,
                                                *atts.offset((i + 1 as i32) as isize),
                                                *atts.offset(i as isize),
                                            );
                                            break;
                                        } else if !nsname.is_null()
                                            && *atts.offset((j + 2 as i32) as isize)
                                                == nsname
                                        {
                                            xmlNsErr(
                                                ctxt,
                                                XML_NS_ERR_ATTRIBUTE_REDEFINED,
                                                b"Namespaced Attribute %s in \'%s\' redefined\n\x00"
                                                    as *const u8
                                                    as *const i8,
                                                *atts.offset(i as isize),
                                                nsname,
                                                0 as *const xmlChar,
                                            );
                                            break;
                                        }
                                    }
                                }
                                j += 5 as i32
                            }
                            i += 5 as i32
                        }
                        nsname = xmlGetNamespace(ctxt, prefix);
                        if !prefix.is_null() && nsname.is_null() {
                            xmlNsErr(
                                ctxt,
                                XML_NS_ERR_UNDEFINED_NAMESPACE,
                                b"Namespace prefix %s on %s is not defined\n\x00" as *const u8
                                    as *const i8,
                                prefix,
                                localname,
                                0 as *const xmlChar,
                            );
                        }
                        unsafe {
                            *pref = prefix;
                            *URI = nsname;
                            /*
                             * SAX: Start of Element !
                             */
                            if !(*ctxt).sax.is_null()
                                && (*(*ctxt).sax).startElementNs.is_some()
                                && (*ctxt).disableSAX == 0
                            {
                                if nbNs > 0 as i32 {
                                    (*(*ctxt).sax)
                                        .startElementNs
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        localname,
                                        prefix,
                                        nsname,
                                        nbNs,
                                        &mut *(*ctxt).nsTab.offset(
                                            ((*ctxt).nsNr - 2 as i32 * nbNs) as isize,
                                        ),
                                        nbatts / 5 as i32,
                                        nbdef,
                                        atts,
                                    );
                                } else {
                                    (*(*ctxt).sax)
                                        .startElementNs
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        localname,
                                        prefix,
                                        nsname,
                                        0 as i32,
                                        0 as *mut *const xmlChar,
                                        nbatts / 5 as i32,
                                        nbdef,
                                        atts,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    /*
     * Free up attribute allocated strings if needed
     */
    if attval != 0 as i32 {
        i = 3 as i32;
        j = 0 as i32;
        while j < nratts {
            unsafe {
                if *(*ctxt).attallocs.offset(j as isize) != 0 as i32
                    && !(*atts.offset(i as isize)).is_null()
                {
                    xmlFree_safe(*atts.offset(i as isize) as *mut xmlChar as *mut ());
                }
            }
            i += 5 as i32;
            j += 1
        }
    }
    return localname;
}
/* *
* xmlParseEndTag2:
* @ctxt:  an XML parser context
* @line:  line of the start tag
* @nsNr:  number of namespaces on the start tag
*
* parse an end of tag
*
* [42] ETag ::= '</' Name S? '>'
*
* With namespace
*
* [NS 9] ETag ::= '</' QName S? '>'
*/
unsafe fn xmlParseEndTag2(mut ctxt: xmlParserCtxtPtr, mut tag: *const xmlStartTag) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    if unsafe {
        *(*(*ctxt).input).cur as i32 != '<' as i32
            || *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 != '/' as i32
    } {
        xmlFatalErr(ctxt, XML_ERR_LTSLASH_REQUIRED, 0 as *const i8);
        return;
    }
    unsafe {
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
        (*(*ctxt).input).col += 2 as i32;
        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
            xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
        }
    }
    let mut safe_tag = unsafe { *tag };
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_tag).prefix.is_null() {
        name = xmlParseNameAndCompare(ctxt, (safe_ctxt).name)
    } else {
        name = xmlParseQNameAndCompare(ctxt, (safe_ctxt).name, (safe_tag).prefix)
    }
    /*
     * We should definitely be at the ending "S? '>'" part
     */
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return;
    }
    xmlSkipBlankChars(ctxt);
    if unsafe {
        !(0x9 as i32 <= *(*(*ctxt).input).cur as i32
            && *(*(*ctxt).input).cur as i32 <= 0xa as i32
            || *(*(*ctxt).input).cur as i32 == 0xd as i32
            || 0x20 as i32 <= *(*(*ctxt).input).cur as i32)
            || *(*(*ctxt).input).cur as i32 != '>' as i32
    } {
        xmlFatalErr(ctxt, XML_ERR_GT_REQUIRED, 0 as *const i8);
    } else {
        unsafe {
            (*(*ctxt).input).col += 1;
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
        }
    }
    /*
     * [ WFC: Element Type Match ]
     * The Name in an element's end-tag must match the element type in the
     * start-tag.
     *
     */
    if name != 1 as i32 as *mut xmlChar {
        if name.is_null() {
            name = b"unparsable\x00" as *const u8 as *const i8 as *mut xmlChar
        }
        xmlFatalErrMsgStrIntStr(
            ctxt,
            XML_ERR_TAG_NAME_MISMATCH,
            b"Opening and ending tag mismatch: %s line %d and %s\n\x00" as *const u8
                as *const i8,
            (safe_ctxt).name,
            (safe_tag).line,
            name,
        );
    }
    /*
     * SAX: End of Tag
     */
    unsafe {
        if !(safe_ctxt).sax.is_null()
            && (*(*ctxt).sax).endElementNs.is_some()
            && (safe_ctxt).disableSAX == 0
        {
            (*(*ctxt).sax)
                .endElementNs
                .expect("non-null function pointer")(
                (safe_ctxt).userData,
                (safe_ctxt).name,
                (safe_tag).prefix,
                (safe_tag).URI,
            );
        }
    }
    spacePop(ctxt);
    if (safe_tag).nsNr != 0 as i32 {
        nsPop(ctxt, (safe_tag).nsNr);
    }
}
/* *
* xmlParseCDSect:
* @ctxt:  an XML parser context
*
* Parse escaped pure raw content.
*
* [18] CDSect ::= CDStart CData CDEnd
*
* [19] CDStart ::= '<![CDATA['
*
* [20] Data ::= (Char* - (Char* ']]>' Char*))
*
* [21] CDEnd ::= ']]>'
*/

pub unsafe fn xmlParseCDSect(mut ctxt: xmlParserCtxtPtr) {
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: i32 = 0 as i32;
    let mut size: i32 = 100 as i32;
    let mut r: i32 = 0;
    let mut rl: i32 = 0;
    let mut s: i32 = 0;
    let mut sl: i32 = 0;
    let mut cur: i32 = 0;
    let mut l: i32 = 0;
    let mut count: i32 = 0 as i32;
    /* Check 2.6.0 was NXT(0) not RAW */
    unsafe {
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '!' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == '[' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'C' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'D' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'A' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'A' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(8 as i32 as isize)
                as i32
                == '[' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(9 as i32 as isize);
            (*(*ctxt).input).col += 9 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
        } else {
            return;
        }
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    (safe_ctxt).instate = XML_PARSER_CDATA_SECTION;
    unsafe {
        r = xmlCurrentChar(ctxt, &mut rl);
    }
    if if r < 0x100 as i32 {
        (0x9 as i32 <= r && r <= 0xa as i32
            || r == 0xd as i32
            || 0x20 as i32 <= r) as i32
    } else {
        (0x100 as i32 <= r && r <= 0xd7ff as i32
            || 0xe000 as i32 <= r && r <= 0xfffd as i32
            || 0x10000 as i32 <= r && r <= 0x10ffff as i32) as i32
    } == 0
    {
        xmlFatalErr(ctxt, XML_ERR_CDATA_NOT_FINISHED, 0 as *const i8);
        (safe_ctxt).instate = XML_PARSER_CONTENT;
        return;
    }
    unsafe {
        if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as i32
        } else {
            (*(*ctxt).input).col += 1
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(rl as isize);
        s = xmlCurrentChar(ctxt, &mut sl);
    }
    if if s < 0x100 as i32 {
        (0x9 as i32 <= s && s <= 0xa as i32
            || s == 0xd as i32
            || 0x20 as i32 <= s) as i32
    } else {
        (0x100 as i32 <= s && s <= 0xd7ff as i32
            || 0xe000 as i32 <= s && s <= 0xfffd as i32
            || 0x10000 as i32 <= s && s <= 0x10ffff as i32) as i32
    } == 0
    {
        xmlFatalErr(ctxt, XML_ERR_CDATA_NOT_FINISHED, 0 as *const i8);
        (safe_ctxt).instate = XML_PARSER_CONTENT;
        return;
    }
    unsafe {
        if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as i32
        } else {
            (*(*ctxt).input).col += 1
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(sl as isize);
        cur = xmlCurrentChar(ctxt, &mut l);
    }
    buf = xmlMallocAtomic_safe(
        (size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
    ) as *mut xmlChar;
    if buf.is_null() {
        unsafe {
            xmlErrMemory(ctxt, 0 as *const i8);
        }
        return;
    }
    while (if cur < 0x100 as i32 {
        (0x9 as i32 <= cur && cur <= 0xa as i32
            || cur == 0xd as i32
            || 0x20 as i32 <= cur) as i32
    } else {
        (0x100 as i32 <= cur && cur <= 0xd7ff as i32
            || 0xe000 as i32 <= cur && cur <= 0xfffd as i32
            || 0x10000 as i32 <= cur && cur <= 0x10ffff as i32)
            as i32
    }) != 0
        && (r != ']' as i32 || s != ']' as i32 || cur != '>' as i32)
    {
        if len + 5 as i32 >= size {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            if size > 10000000 as i32
                && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
            {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_CDATA_NOT_FINISHED,
                    b"CData section too big found\x00" as *const u8 as *const i8,
                    0 as *const xmlChar,
                );
                xmlFree_safe(buf as *mut ());
                return;
            }
            tmp = xmlRealloc_safe(
                buf as *mut (),
                ((size * 2 as i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
            ) as *mut xmlChar;
            if tmp.is_null() {
                xmlFree_safe(buf as *mut ());
                unsafe {
                    xmlErrMemory(ctxt, 0 as *const i8);
                }
                return;
            }
            buf = tmp;
            size *= 2 as i32
        }
        if rl == 1 as i32 {
            let fresh126 = len;
            len = len + 1;
            unsafe { *buf.offset(fresh126 as isize) = r as xmlChar }
        } else {
            len += unsafe { xmlCopyCharMultiByte(&mut *buf.offset(len as isize), r) };
        }
        r = s;
        rl = sl;
        s = cur;
        sl = l;
        count += 1;
        if count > 50 as i32 {
            if unsafe {
                (*ctxt).progressive == 0 as i32
                    && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                        > (2 as i32 * 250 as i32) as i64
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < (2 as i32 * 250 as i32) as i64
            } {
                xmlSHRINK(ctxt);
            }
            if unsafe {
                (*ctxt).progressive == 0 as i32
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                        < 250 as i32 as i64
            } {
                xmlGROW(ctxt);
            }
            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                xmlFree_safe(buf as *mut ());
                return;
            }
            count = 0 as i32
        }
        unsafe {
            if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as i32
            } else {
                (*(*ctxt).input).col += 1
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            cur = xmlCurrentChar(ctxt, &mut l)
        }
    }
    unsafe {
        *buf.offset(len as isize) = 0 as i32 as xmlChar;
    }
    (safe_ctxt).instate = XML_PARSER_CONTENT;
    if cur != '>' as i32 {
        xmlFatalErrMsgStr(
            ctxt,
            XML_ERR_CDATA_NOT_FINISHED,
            b"CData section not finished\n%.50s\n\x00" as *const u8 as *const i8,
            buf,
        );
        xmlFree_safe(buf as *mut ());
        return;
    }
    unsafe {
        if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as i32
        } else {
            (*(*ctxt).input).col += 1
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
    }
    /*
     * OK the buffer is to be consumed as cdata.
     */
    if !(safe_ctxt).sax.is_null() && (safe_ctxt).disableSAX == 0 {
        unsafe {
            if (*(*ctxt).sax).cdataBlock.is_some() {
                (*(*ctxt).sax)
                    .cdataBlock
                    .expect("non-null function pointer")((*ctxt).userData, buf, len);
            } else if (*(*ctxt).sax).characters.is_some() {
                (*(*ctxt).sax)
                    .characters
                    .expect("non-null function pointer")((*ctxt).userData, buf, len);
            }
        }
    }
    xmlFree_safe(buf as *mut ());
}
/* *
* xmlParseContentInternal:
* @ctxt:  an XML parser context
*
* Parse a content sequence. Stops at EOF or '</'. Leaves checking of
* unexpected EOF to the caller.
*/
unsafe fn xmlParseContentInternal(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut nameNr: i32 = (safe_ctxt).nameNr;
    //@todo 削减unsafe范围
    unsafe {
        if (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        {
            xmlGROW(ctxt);
        }
        while *(*(*ctxt).input).cur as i32 != 0 as i32
            && (*ctxt).instate as i32 != XML_PARSER_EOF as i32
        {
            let mut test: *const xmlChar = (*(*ctxt).input).cur;
            let mut cons: u32 = (*(*ctxt).input).consumed as u32;
            let mut cur: *const xmlChar = (*(*ctxt).input).cur;
            /*
             * First case : a Processing Instruction.
             */
            if *cur as i32 == '<' as i32
                && *cur.offset(1 as i32 as isize) as i32 == '?' as i32
            {
                xmlParsePI(ctxt);
            } else if *((*(*ctxt).input).cur as *mut u8)
                .offset(0 as i32 as isize) as i32
                == '<' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                    as i32
                    == '!' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                    as i32
                    == '[' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                    as i32
                    == 'C' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                    as i32
                    == 'D' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                    as i32
                    == 'A' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                    as i32
                    == 'T' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                    as i32
                    == 'A' as i32
                && *((*(*ctxt).input).cur as *mut u8).offset(8 as i32 as isize)
                    as i32
                    == '[' as i32
            {
                xmlParseCDSect(ctxt);
            } else if *cur as i32 == '<' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == '!' as i32
                && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                    == '-' as i32
                && *(*(*ctxt).input).cur.offset(3 as i32 as isize) as i32
                    == '-' as i32
            {
                xmlParseComment(ctxt);
                (*ctxt).instate = XML_PARSER_CONTENT
            } else if *cur as i32 == '<' as i32 {
                if *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == '/' as i32
                {
                    if (*ctxt).nameNr <= nameNr {
                        break;
                    }
                    xmlParseElementEnd(ctxt);
                } else {
                    xmlParseElementStart(ctxt);
                }
            } else if *cur as i32 == '&' as i32 {
                xmlParseReference(ctxt);
            } else {
                /*
                 * Second case : a CDSection
                 */
                /* 2.6.0 test was *cur not RAW */
                /*
                 * Third case :  a comment
                 */
                /*
                 * Fourth case :  a sub-element.
                 */
                /*
                 * Fifth case : a reference. If if has not been resolved,
                 *    parsing returns it's Name, create the node
                 */
                /*
                 * Last case, text. Note that References are handled directly.
                 */
                xmlParseCharData(ctxt, 0 as i32);
            }
            if (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
            {
                xmlGROW(ctxt);
            }
            if (*ctxt).progressive == 0 as i32
                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                    > (2 as i32 * 250 as i32) as i64
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < (2 as i32 * 250 as i32) as i64
            {
                xmlSHRINK(ctxt);
            }
            if !(cons as u64 == (*(*ctxt).input).consumed && test == (*(*ctxt).input).cur)
            {
                continue;
            }
            xmlFatalErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"detected an error in element content\n\x00" as *const u8 as *const i8,
            );
            xmlHaltParser(ctxt);
            break;
        }
    }
}
/* *
* xmlParseContent:
* @ctxt:  an XML parser context
*
* Parse a content sequence. Stops at EOF or '</'.
*
* [43] content ::= (element | CharData | Reference | CDSect | PI | Comment)*
*/

pub unsafe fn xmlParseContent(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut nameNr: i32 = (safe_ctxt).nameNr;
    xmlParseContentInternal(ctxt);
    if (safe_ctxt).instate as i32 != XML_PARSER_EOF as i32
        && (safe_ctxt).nameNr > nameNr
    {
        unsafe {
            let mut name: *const xmlChar = *(*ctxt)
                .nameTab
                .offset(((*ctxt).nameNr - 1 as i32) as isize);
            let mut line: i32 = (*(*ctxt)
                .pushTab
                .offset(((*ctxt).nameNr - 1 as i32) as isize))
            .line;
            xmlFatalErrMsgStrIntStr(
                ctxt,
                XML_ERR_TAG_NOT_FINISHED,
                b"Premature end of data in tag %s line %d\n\x00" as *const u8
                    as *const i8,
                name,
                line,
                0 as *const xmlChar,
            );
        }
    };
}
/* *
* xmlParseElement:
* @ctxt:  an XML parser context
*
* parse an XML element
*
* [39] element ::= EmptyElemTag | STag content ETag
*
* [ WFC: Element Type Match ]
* The Name in an element's end-tag must match the element type in the
* start-tag.
*
*/

pub unsafe fn xmlParseElement(mut ctxt: xmlParserCtxtPtr) {
    if xmlParseElementStart(ctxt) != 0 as i32 {
        return;
    }
    xmlParseContentInternal(ctxt);
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return;
    }
    unsafe {
        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
            let mut name: *const xmlChar = *(*ctxt)
                .nameTab
                .offset(((*ctxt).nameNr - 1 as i32) as isize);
            let mut line: i32 = (*(*ctxt)
                .pushTab
                .offset(((*ctxt).nameNr - 1 as i32) as isize))
            .line;
            xmlFatalErrMsgStrIntStr(
                ctxt,
                XML_ERR_TAG_NOT_FINISHED,
                b"Premature end of data in tag %s line %d\n\x00" as *const u8
                    as *const i8,
                name,
                line,
                0 as *const xmlChar,
            );
            return;
        }
    }
    xmlParseElementEnd(ctxt);
}
/* *
* xmlParseElementStart:
* @ctxt:  an XML parser context
*
* Parse the start of an XML element. Returns -1 in case of error, 0 if an
* opening tag was parsed, 1 if an empty element was parsed.
*/
unsafe fn xmlParseElementStart(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut prefix: *const xmlChar = 0 as *const xmlChar;
    let mut URI: *const xmlChar = 0 as *const xmlChar;
    let mut node_info: xmlParserNodeInfo = xmlParserNodeInfo {
        node: 0 as *const _xmlNode,
        begin_pos: 0,
        begin_line: 0,
        end_pos: 0,
        end_line: 0,
    };
    let mut line: i32 = 0;
    let mut tlen: i32 = 0 as i32;
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    let mut nsNr: i32 = (safe_ctxt).nsNr;
    if (safe_ctxt).nameNr as u32 > unsafe { xmlParserMaxDepth }
        && (safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
    {
        xmlFatalErrMsgInt(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"Excessive depth in document: %d use XML_PARSE_HUGE option\n\x00" as *const u8
                as *const i8,
            unsafe { xmlParserMaxDepth as i32 },
        );
        unsafe {
            xmlHaltParser(ctxt);
        }
        return -(1 as i32);
    }
    /* Capture start position */
    if (safe_ctxt).record_info != 0 {
        unsafe {
            node_info.begin_pos = (*(*ctxt).input)
                .consumed
                .wrapping_add((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                    as i64 as u64);
            node_info.begin_line = (*(*ctxt).input).line as u64
        }
    }
    if (safe_ctxt).spaceNr == 0 as i32 || *(*ctxt).space == -(2 as i32)  {
        spacePush(ctxt, -(1 as i32));
    } else {
        unsafe {
            spacePush(ctxt, *(*ctxt).space);
        }
    }
    line = unsafe { (*(*ctxt).input).line };

    match () {
        #[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
        _ => {
            if (safe_ctxt).sax2 != 0 {
                /* LIBXML_SAX1_ENABLED */
                name = xmlParseStartTag2(ctxt, &mut prefix, &mut URI, &mut tlen)
            } else {
                name = xmlParseStartTag(ctxt)
            }
        }
        #[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))]
        _ => {
            name = xmlParseStartTag2(ctxt, &mut prefix, &mut URI, &mut tlen);
        }
    };

    /* LIBXML_SAX1_ENABLED */
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return -(1 as i32);
    }
    if name.is_null() {
        spacePop(ctxt);
        return -(1 as i32);
    }
    nameNsPush(ctxt, name, prefix, URI, line, (safe_ctxt).nsNr - nsNr);
    ret = (safe_ctxt).node;
    /*
     * [ VC: Root Element Type ]
     * The Name in the document type declaration must match the element
     * type of the root element.
     */

    match () {
        #[cfg(HAVE_parser_LIBXML_VALID_ENABLED)]
        _ => {
            if (safe_ctxt).validate != 0
                && (safe_ctxt).wellFormed != 0
                && !(safe_ctxt).myDoc.is_null()
                && !(safe_ctxt).node.is_null()
                && (safe_ctxt).node == unsafe { (*(*ctxt).myDoc).children }
            {
                (safe_ctxt).valid &= xmlValidateRoot_safe(&mut (safe_ctxt).vctxt, (safe_ctxt).myDoc)
            }
        }
        #[cfg(not(HAVE_parser_LIBXML_VALID_ENABLED))]
        _ => {}
    };

    /* LIBXML_VALID_ENABLED */
    /*
     * Check for an Empty Element.
     */
    //@todo 削减unsafe范围
    unsafe {
        if *(*(*ctxt).input).cur as i32 == '/' as i32
            && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '>' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
            (*(*ctxt).input).col += 2 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            if (*ctxt).sax2 != 0 {
                if !(*ctxt).sax.is_null()
                    && (*(*ctxt).sax).endElementNs.is_some()
                    && (*ctxt).disableSAX == 0
                {
                    (*(*ctxt).sax)
                        .endElementNs
                        .expect("non-null function pointer")(
                        (*ctxt).userData, name, prefix, URI
                    );
                }
            } else {
                match () {
                    #[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
                    _ => {
                        if !(*ctxt).sax.is_null()
                            && (*(*ctxt).sax).endElement.is_some()
                            && (*ctxt).disableSAX == 0
                        {
                            (*(*ctxt).sax)
                                .endElement
                                .expect("non-null function pointer")(
                                (*ctxt).userData, name
                            );
                        }
                    }
                    #[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))]
                    _ => {}
                };
            }
            namePop(ctxt);
            spacePop(ctxt);
            if nsNr != (*ctxt).nsNr {
                nsPop(ctxt, (*ctxt).nsNr - nsNr);
            }
            if !ret.is_null() && (*ctxt).record_info != 0 {
                node_info.end_pos = (*(*ctxt).input)
                    .consumed
                    .wrapping_add((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                        as i64 as u64);
                node_info.end_line = (*(*ctxt).input).line as u64;
                node_info.node = ret as *const _xmlNode;
                xmlParserAddNodeInfo_safe(ctxt, &mut node_info);
            }
            return 1 as i32;
        }
        if *(*(*ctxt).input).cur as i32 == '>' as i32 {
            (*(*ctxt).input).col += 1;
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
        } else {
            xmlFatalErrMsgStrIntStr(
                ctxt,
                XML_ERR_GT_REQUIRED,
                b"Couldn\'t find end of Start Tag %s line %d\n\x00" as *const u8
                    as *const i8,
                name,
                line,
                0 as *const xmlChar,
            );
            /* LIBXML_SAX1_ENABLED */
            /*
             * end of parsing of this node.
             */
            nodePop(ctxt);
            namePop(ctxt);
            spacePop(ctxt);
            if nsNr != (*ctxt).nsNr {
                nsPop(ctxt, (*ctxt).nsNr - nsNr);
            }
            /*
             * Capture end position and add node
             */
            if !ret.is_null() && (*ctxt).record_info != 0 {
                node_info.end_pos = (*(*ctxt).input)
                    .consumed
                    .wrapping_add((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                        as i64 as u64);
                node_info.end_line = (*(*ctxt).input).line as u64;
                node_info.node = ret as *const _xmlNode;
                xmlParserAddNodeInfo_safe(ctxt, &mut node_info);
            }
            return -(1 as i32);
        }
    }
    return 0 as i32;
}
/* *
* xmlParseElementEnd:
* @ctxt:  an XML parser context
*
* Parse the end of an XML element.
*/
unsafe fn xmlParseElementEnd(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut node_info: xmlParserNodeInfo = xmlParserNodeInfo {
        node: 0 as *const _xmlNode,
        begin_pos: 0,
        begin_line: 0,
        end_pos: 0,
        end_line: 0,
    };
    let mut ret: xmlNodePtr = (safe_ctxt).node;
    if (safe_ctxt).nameNr <= 0 as i32 {
        return;
    }
    /*
     * parse the end of tag: '</' should be here.
     */
    if (safe_ctxt).sax2 != 0 {
        unsafe {
            xmlParseEndTag2(
                ctxt,
                &mut *(*ctxt)
                    .pushTab
                    .offset(((safe_ctxt).nameNr - 1 as i32) as isize),
            );
        }
        namePop(ctxt);
    } else {
        match () {
            #[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
            _ => {
                xmlParseEndTag1(ctxt, 0 as i32);
            }
            #[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))]
            _ => {}
        };
    }
    /* LIBXML_SAX1_ENABLED */
    /*
     * Capture end position and add node
     */
    if !ret.is_null() && (safe_ctxt).record_info != 0 {
        unsafe {
            node_info.end_pos = (*(*ctxt).input)
                .consumed
                .wrapping_add((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                    as i64 as u64);
            node_info.end_line = (*(*ctxt).input).line as u64;
        }
        node_info.node = ret as *const _xmlNode;
        xmlParserAddNodeInfo_safe(ctxt, &mut node_info);
    };
}
/* *
* xmlParseVersionNum:
* @ctxt:  an XML parser context
*
* parse the XML version value.
*
* [26] VersionNum ::= '1.' [0-9]+
*
* In practice allow [0-9].[0-9]+ at that level
*
* Returns the string giving the XML version number, or NULL
*/

pub unsafe fn xmlParseVersionNum(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: i32 = 0 as i32;
    let mut size: i32 = 10 as i32;
    let mut cur: xmlChar = 0;
    buf = xmlMallocAtomic_safe(
        (size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
    ) as *mut xmlChar;
    if buf.is_null() {
        unsafe {
            xmlErrMemory(ctxt, 0 as *const i8);
        }
        return 0 as *mut xmlChar;
    }
    cur = unsafe { *(*(*ctxt).input).cur };
    if !(cur as i32 >= '0' as i32 && cur as i32 <= '9' as i32) {
        xmlFree_safe(buf as *mut ());
        return 0 as *mut xmlChar;
    }
    let fresh127 = len;
    len = len + 1;
    unsafe { *buf.offset(fresh127 as isize) = cur };
    xmlNextChar_safe(ctxt);
    unsafe { cur = *(*(*ctxt).input).cur };
    if cur as i32 != '.' as i32 {
        xmlFree_safe(buf as *mut ());
        return 0 as *mut xmlChar;
    }
    let fresh128 = len;
    len = len + 1;
    unsafe { *buf.offset(fresh128 as isize) = cur };
    xmlNextChar_safe(ctxt);
    unsafe { cur = *(*(*ctxt).input).cur };
    while cur as i32 >= '0' as i32 && cur as i32 <= '9' as i32 {
        if len + 1 as i32 >= size {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            size *= 2 as i32;
            tmp = xmlRealloc_safe(
                buf as *mut (),
                (size as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
            ) as *mut xmlChar;
            if tmp.is_null() {
                xmlFree_safe(buf as *mut ());
                unsafe {
                    xmlErrMemory(ctxt, 0 as *const i8);
                }
                return 0 as *mut xmlChar;
            }
            buf = tmp
        }
        let fresh129 = len;
        len = len + 1;
        unsafe {
            *buf.offset(fresh129 as isize) = cur;
            xmlNextChar_safe(ctxt);
            cur = *(*(*ctxt).input).cur
        }
    }
    unsafe {
        *buf.offset(len as isize) = 0 as i32 as xmlChar;
    }
    return buf;
}
/* *
* xmlParseVersionInfo:
* @ctxt:  an XML parser context
*
* parse the XML version.
*
* [24] VersionInfo ::= S 'version' Eq (' VersionNum ' | " VersionNum ")
*
* [25] Eq ::= S? '=' S?
*
* Returns the version string, e.g. "1.0"
*/

pub unsafe fn xmlParseVersionInfo(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut version: *mut xmlChar = 0 as *mut xmlChar;
    //@todo 削减unsafe范围
    unsafe {
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == 'v' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'e' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'r' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 's' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'i' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'o' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'n' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(7 as i32 as isize);
            (*(*ctxt).input).col += 7 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            xmlSkipBlankChars(ctxt);
            if *(*(*ctxt).input).cur as i32 != '=' as i32 {
                xmlFatalErr(ctxt, XML_ERR_EQUAL_REQUIRED, 0 as *const i8);
                return 0 as *mut xmlChar;
            }
            xmlNextChar_safe(ctxt);
            xmlSkipBlankChars(ctxt);
            if *(*(*ctxt).input).cur as i32 == '\"' as i32 {
                xmlNextChar_safe(ctxt);
                version = xmlParseVersionNum(ctxt);
                if *(*(*ctxt).input).cur as i32 != '\"' as i32 {
                    xmlFatalErr(ctxt, XML_ERR_STRING_NOT_CLOSED, 0 as *const i8);
                } else {
                    xmlNextChar_safe(ctxt);
                }
            } else if *(*(*ctxt).input).cur as i32 == '\'' as i32 {
                xmlNextChar_safe(ctxt);
                version = xmlParseVersionNum(ctxt);
                if *(*(*ctxt).input).cur as i32 != '\'' as i32 {
                    xmlFatalErr(ctxt, XML_ERR_STRING_NOT_CLOSED, 0 as *const i8);
                } else {
                    xmlNextChar_safe(ctxt);
                }
            } else {
                xmlFatalErr(ctxt, XML_ERR_STRING_NOT_STARTED, 0 as *const i8);
            }
        }
    }
    return version;
}
/* *
* xmlParseEncName:
* @ctxt:  an XML parser context
*
* parse the XML encoding name
*
* [81] EncName ::= [A-Za-z] ([A-Za-z0-9._] | '-')*
*
* Returns the encoding name value or NULL
*/

pub unsafe fn xmlParseEncName(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: i32 = 0 as i32;
    let mut size: i32 = 10 as i32;
    let mut cur: xmlChar = 0;
    cur = unsafe { *(*(*ctxt).input).cur };
    if cur as i32 >= 'a' as i32 && cur as i32 <= 'z' as i32
        || cur as i32 >= 'A' as i32 && cur as i32 <= 'Z' as i32
    {
        buf = xmlMallocAtomic_safe(
            (size as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
        ) as *mut xmlChar;
        if buf.is_null() {
            unsafe {
                xmlErrMemory(ctxt, 0 as *const i8);
            }
            return 0 as *mut xmlChar;
        }
        let fresh130 = len;
        len = len + 1;
        unsafe {
            *buf.offset(fresh130 as isize) = cur;
        }
        xmlNextChar_safe(ctxt);
        cur = unsafe { *(*(*ctxt).input).cur };
        while cur as i32 >= 'a' as i32 && cur as i32 <= 'z' as i32
            || cur as i32 >= 'A' as i32 && cur as i32 <= 'Z' as i32
            || cur as i32 >= '0' as i32 && cur as i32 <= '9' as i32
            || cur as i32 == '.' as i32
            || cur as i32 == '_' as i32
            || cur as i32 == '-' as i32
        {
            if len + 1 as i32 >= size {
                let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                size *= 2 as i32;
                tmp = xmlRealloc_safe(
                    buf as *mut (),
                    (size as u64)
                        .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                ) as *mut xmlChar;
                if tmp.is_null() {
                    unsafe {
                        xmlErrMemory(ctxt, 0 as *const i8);
                    }
                    xmlFree_safe(buf as *mut ());
                    return 0 as *mut xmlChar;
                }
                buf = tmp
            }
            let fresh131 = len;
            len = len + 1;
            unsafe {
                *buf.offset(fresh131 as isize) = cur;
            }
            xmlNextChar_safe(ctxt);
            cur = unsafe { *(*(*ctxt).input).cur };
            if cur as i32 == 0 as i32 {
                if unsafe {
                    (*ctxt).progressive == 0 as i32
                        && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                            > (2 as i32 * 250 as i32) as i64
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < (2 as i32 * 250 as i32) as i64
                } {
                    xmlSHRINK(ctxt);
                }
                if unsafe {
                    (*ctxt).progressive == 0 as i32
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                            < 250 as i32 as i64
                } {
                    xmlGROW(ctxt);
                }
                unsafe { cur = *(*(*ctxt).input).cur }
            }
        }
        unsafe { *buf.offset(len as isize) = 0 as i32 as xmlChar }
    } else {
        xmlFatalErr(ctxt, XML_ERR_ENCODING_NAME, 0 as *const i8);
    }
    return buf;
}
/* *
* xmlParseEncodingDecl:
* @ctxt:  an XML parser context
*
* parse the XML encoding declaration
*
* [80] EncodingDecl ::= S 'encoding' Eq ('"' EncName '"' |  "'" EncName "'")
*
* this setups the conversion filters.
*
* Returns the encoding value or NULL
*/

pub unsafe fn xmlParseEncodingDecl(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut encoding: *mut xmlChar = 0 as *mut xmlChar;
    xmlSkipBlankChars(ctxt);
    if unsafe {
        *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == 'e' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 'n' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'c' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'o' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'd' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'i' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'n' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'g' as i32
    } {
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(8 as i32 as isize);
            (*(*ctxt).input).col += 8 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
        }
        xmlSkipBlankChars(ctxt);
        if unsafe { *(*(*ctxt).input).cur as i32 != '=' as i32 } {
            xmlFatalErr(ctxt, XML_ERR_EQUAL_REQUIRED, 0 as *const i8);
            return 0 as *const xmlChar;
        }
        xmlNextChar_safe(ctxt);
        xmlSkipBlankChars(ctxt);
        if unsafe { *(*(*ctxt).input).cur as i32 == '\"' as i32 } {
            xmlNextChar_safe(ctxt);
            encoding = xmlParseEncName(ctxt);
            if unsafe { *(*(*ctxt).input).cur as i32 != '\"' as i32 } {
                xmlFatalErr(ctxt, XML_ERR_STRING_NOT_CLOSED, 0 as *const i8);
                xmlFree_safe(encoding as *mut ());
                return 0 as *const xmlChar;
            } else {
                xmlNextChar_safe(ctxt);
            }
        } else if unsafe { *(*(*ctxt).input).cur as i32 == '\'' as i32 } {
            xmlNextChar_safe(ctxt);
            encoding = xmlParseEncName(ctxt);
            if unsafe { *(*(*ctxt).input).cur as i32 != '\'' as i32 } {
                xmlFatalErr(ctxt, XML_ERR_STRING_NOT_CLOSED, 0 as *const i8);
                xmlFree_safe(encoding as *mut ());
                return 0 as *const xmlChar;
            } else {
                xmlNextChar_safe(ctxt);
            }
        } else {
            xmlFatalErr(ctxt, XML_ERR_STRING_NOT_STARTED, 0 as *const i8);
        }
        /*
         * Non standard parsing, allowing the user to ignore encoding
         */
        if (safe_ctxt).options & XML_PARSE_IGNORE_ENC as i32 != 0 {
            xmlFree_safe(encoding as *mut ());
            return 0 as *const xmlChar;
        }
        /*
         * UTF-16 encoding switch has already taken place at this stage,
         * more over the little-endian/big-endian selection is already done
         */
        if !encoding.is_null()
            && (xmlStrcasecmp_safe(
                encoding,
                b"UTF-16\x00" as *const u8 as *const i8 as *mut xmlChar,
            ) == 0
                || xmlStrcasecmp_safe(
                    encoding,
                    b"UTF16\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) == 0)
        {
            /*
             * If no encoding was passed to the parser, that we are
             * using UTF-16 and no decoder is present i.e. the
             * document is apparently UTF-8 compatible, then raise an
             * encoding mismatch fatal error
             */
            if unsafe {
                (*ctxt).encoding.is_null()
                    && !(*(*ctxt).input).buf.is_null()
                    && (*(*(*ctxt).input).buf).encoder.is_null()
            } {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_INVALID_ENCODING,
                    b"Document labelled UTF-16 but has UTF-8 content\n\x00" as *const u8
                        as *const i8,
                );
            }
            if !(safe_ctxt).encoding.is_null() {
                xmlFree_safe((safe_ctxt).encoding as *mut xmlChar as *mut ());
            }
            (safe_ctxt).encoding = encoding
        } else if !encoding.is_null()
            && (xmlStrcasecmp_safe(
                encoding,
                b"UTF-8\x00" as *const u8 as *const i8 as *mut xmlChar,
            ) == 0
                || xmlStrcasecmp_safe(
                    encoding,
                    b"UTF8\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) == 0)
        {
            if !(safe_ctxt).encoding.is_null() {
                xmlFree_safe((safe_ctxt).encoding as *mut xmlChar as *mut ());
            }
            (safe_ctxt).encoding = encoding
        } else if !encoding.is_null() {
            let mut handler: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
            unsafe {
                if !(*(*ctxt).input).encoding.is_null() {
                    xmlFree_safe((*(*ctxt).input).encoding as *mut xmlChar as *mut ());
                }
                (*(*ctxt).input).encoding = encoding;
            }

            handler = xmlFindCharEncodingHandler_safe(encoding as *const i8);
            if !handler.is_null() {
                if xmlSwitchToEncoding_safe(ctxt, handler) < 0 as i32 {
                    /*
                     * UTF-8 encoding is handled natively
                     */
                    /* failed to convert */
                    (safe_ctxt).errNo = XML_ERR_UNSUPPORTED_ENCODING as i32;
                    return 0 as *const xmlChar;
                }
            } else {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"Unsupported encoding %s\n\x00" as *const u8 as *const i8,
                    encoding,
                );
                return 0 as *const xmlChar;
            }
        }
    }
    return encoding;
}
/* *
* xmlParseSDDecl:
* @ctxt:  an XML parser context
*
* parse the XML standalone declaration
*
* [32] SDDecl ::= S 'standalone' Eq
*                 (("'" ('yes' | 'no') "'") | ('"' ('yes' | 'no')'"'))
*
* [ VC: Standalone Document Declaration ]
* TODO The standalone document declaration must have the value "no"
* if any external markup declarations contain declarations of:
*  - attributes with default values, if elements to which these
*    attributes apply appear in the document without specifications
*    of values for these attributes, or
*  - entities (other than amp, lt, gt, apos, quot), if references
*    to those entities appear in the document, or
*  - attributes with values subject to normalization, where the
*    attribute appears in the document with a value which will change
*    as a result of normalization, or
*  - element types with element content, if white space occurs directly
*    within any instance of those types.
*
* Returns:
*   1 if standalone="yes"
*   0 if standalone="no"
*  -2 if standalone attribute is missing or invalid
*	  (A standalone value of -2 means that the XML declaration was found,
*	   but no value was specified for the standalone attribute).
*/

pub unsafe fn xmlParseSDDecl(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let mut standalone: i32 = -(2 as i32);
    xmlSkipBlankChars(ctxt);
    //@todo 削减unsafe范围
    unsafe {
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == 's' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == 't' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'a' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'n' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'd' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'a' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'l' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'o' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(8 as i32 as isize)
                as i32
                == 'n' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(9 as i32 as isize)
                as i32
                == 'e' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(10 as i32 as isize);
            (*(*ctxt).input).col += 10 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
            xmlSkipBlankChars(ctxt);
            if *(*(*ctxt).input).cur as i32 != '=' as i32 {
                xmlFatalErr(ctxt, XML_ERR_EQUAL_REQUIRED, 0 as *const i8);
                return standalone;
            }
            xmlNextChar_safe(ctxt);
            xmlSkipBlankChars(ctxt);
            if *(*(*ctxt).input).cur as i32 == '\'' as i32 {
                xmlNextChar_safe(ctxt);
                if *(*(*ctxt).input).cur as i32 == 'n' as i32
                    && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        == 'o' as i32
                {
                    standalone = 0 as i32;
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
                    (*(*ctxt).input).col += 2 as i32;
                    if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                        xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                    }
                } else if *(*(*ctxt).input).cur as i32 == 'y' as i32
                    && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        == 'e' as i32
                    && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                        == 's' as i32
                {
                    standalone = 1 as i32;
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(3 as i32 as isize);
                    (*(*ctxt).input).col += 3 as i32;
                    if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                        xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                    }
                } else {
                    xmlFatalErr(ctxt, XML_ERR_STANDALONE_VALUE, 0 as *const i8);
                }
                if *(*(*ctxt).input).cur as i32 != '\'' as i32 {
                    xmlFatalErr(ctxt, XML_ERR_STRING_NOT_CLOSED, 0 as *const i8);
                } else {
                    xmlNextChar_safe(ctxt);
                }
            } else if *(*(*ctxt).input).cur as i32 == '\"' as i32 {
                xmlNextChar_safe(ctxt);
                if *(*(*ctxt).input).cur as i32 == 'n' as i32
                    && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        == 'o' as i32
                {
                    standalone = 0 as i32;
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
                    (*(*ctxt).input).col += 2 as i32;
                    if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                        xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                    }
                } else if *(*(*ctxt).input).cur as i32 == 'y' as i32
                    && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                        == 'e' as i32
                    && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                        == 's' as i32
                {
                    standalone = 1 as i32;
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(3 as i32 as isize);
                    (*(*ctxt).input).col += 3 as i32;
                    if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                        xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                    }
                } else {
                    xmlFatalErr(ctxt, XML_ERR_STANDALONE_VALUE, 0 as *const i8);
                }
                if *(*(*ctxt).input).cur as i32 != '\"' as i32 {
                    xmlFatalErr(ctxt, XML_ERR_STRING_NOT_CLOSED, 0 as *const i8);
                } else {
                    xmlNextChar_safe(ctxt);
                }
            } else {
                xmlFatalErr(ctxt, XML_ERR_STRING_NOT_STARTED, 0 as *const i8);
            }
        }
    }
    return standalone;
}
/* *
* xmlParseXMLDecl:
* @ctxt:  an XML parser context
*
* parse an XML declaration header
*
* [23] XMLDecl ::= '<?xml' VersionInfo EncodingDecl? SDDecl? S? '?>'
*/

pub unsafe fn xmlParseXMLDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut version: *mut xmlChar = 0 as *mut xmlChar;
    unsafe {
        /*
         * This value for standalone indicates that the document has an
         * XML declaration but it does not have a standalone attribute.
         * It will be overwritten later if a standalone attribute is found.
         */
        (*(*ctxt).input).standalone = -(2 as i32);
        /*
         * We know that '<?xml' is here.
         */
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(5 as i32 as isize);
        (*(*ctxt).input).col += 5 as i32;
        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
            xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
        }
        if !(*(*(*ctxt).input).cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *(*(*ctxt).input).cur as i32
                && *(*(*ctxt).input).cur as i32 <= 0xa as i32
            || *(*(*ctxt).input).cur as i32 == 0xd as i32)
        {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Blank needed after \'<?xml\'\n\x00" as *const u8 as *const i8,
            );
        }
    }
    xmlSkipBlankChars(ctxt);
    /*
     * We must have the VersionInfo here.
     */
    version = xmlParseVersionInfo(ctxt);
    if version.is_null() {
        xmlFatalErr(ctxt, XML_ERR_VERSION_MISSING, 0 as *const i8);
    } else {
        if xmlStrEqual_safe(
            version,
            b"1.0\x00" as *const u8 as *const i8 as *const xmlChar,
        ) == 0
        {
            /*
             * Changed here for XML-1.0 5th edition
             */
            if (safe_ctxt).options & XML_PARSE_OLD10 as i32 != 0 {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNKNOWN_VERSION,
                    b"Unsupported version \'%s\'\n\x00" as *const u8 as *const i8,
                    version,
                );
            } else if unsafe {
                *version.offset(0 as i32 as isize) as i32 == '1' as i32
                    && *version.offset(1 as i32 as isize) as i32 == '.' as i32
            } {
                xmlWarningMsg(
                    ctxt,
                    XML_WAR_UNKNOWN_VERSION,
                    b"Unsupported version \'%s\'\n\x00" as *const u8 as *const i8,
                    version,
                    0 as *const xmlChar,
                );
            } else {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNKNOWN_VERSION,
                    b"Unsupported version \'%s\'\n\x00" as *const u8 as *const i8,
                    version,
                );
            }
        }
        if !(safe_ctxt).version.is_null() {
            xmlFree_safe((safe_ctxt).version as *mut ());
        }
        (safe_ctxt).version = version
    }
    unsafe {
        /*
         * We may have the encoding declaration
         */
        if !(*(*(*ctxt).input).cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *(*(*ctxt).input).cur as i32
                && *(*(*ctxt).input).cur as i32 <= 0xa as i32
            || *(*(*ctxt).input).cur as i32 == 0xd as i32)
        {
            if *(*(*ctxt).input).cur as i32 == '?' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == '>' as i32
            {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
                (*(*ctxt).input).col += 2 as i32;
                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                }
                return;
            }
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Blank needed here\n\x00" as *const u8 as *const i8,
            );
        }
    }
    xmlParseEncodingDecl(ctxt);
    if (safe_ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as i32
        || (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32
    {
        /*
         * The XML REC instructs us to stop parsing right here
         */
        return;
    }
    unsafe {
        /*
         * We may have the standalone status.
         */
        if !(*(*ctxt).input).encoding.is_null()
            && !(*(*(*ctxt).input).cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *(*(*ctxt).input).cur as i32
                    && *(*(*ctxt).input).cur as i32 <= 0xa as i32
                || *(*(*ctxt).input).cur as i32 == 0xd as i32)
        {
            if *(*(*ctxt).input).cur as i32 == '?' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == '>' as i32
            {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
                (*(*ctxt).input).col += 2 as i32;
                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                }
                return;
            }
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Blank needed here\n\x00" as *const u8 as *const i8,
            );
        }
    }
    /*
     * We can grow the input buffer freely at that point
     */
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    xmlSkipBlankChars(ctxt);
    unsafe { (*(*ctxt).input).standalone = xmlParseSDDecl(ctxt) };
    xmlSkipBlankChars(ctxt);
    if unsafe {
        *(*(*ctxt).input).cur as i32 == '?' as i32
            && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '>' as i32
    } {
        unsafe {
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(2 as i32 as isize);
            (*(*ctxt).input).col += 2 as i32;
            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
            }
        }
    } else if unsafe { *(*(*ctxt).input).cur as i32 == '>' as i32 } {
        /* Deprecated old WD ... */
        xmlFatalErr(ctxt, XML_ERR_XMLDECL_NOT_FINISHED, 0 as *const i8);
        xmlNextChar_safe(ctxt);
    } else {
        xmlFatalErr(ctxt, XML_ERR_XMLDECL_NOT_FINISHED, 0 as *const i8);
        unsafe {
            while *(*(*ctxt).input).cur as i32 != 0
                && *(*(*ctxt).input).cur as i32 != '>' as i32
            {
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
            }
        }
        xmlNextChar_safe(ctxt);
    };
}
/* *
* xmlParseMisc:
* @ctxt:  an XML parser context
*
* parse an XML Misc* optional field.
*
* [27] Misc ::= Comment | PI |  S
*/

pub unsafe fn xmlParseMisc(mut ctxt: xmlParserCtxtPtr) {
    //@todo 削减unsafe范围
    unsafe {
        while (*ctxt).instate as i32 != XML_PARSER_EOF as i32
            && (*(*(*ctxt).input).cur as i32 == '<' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == '?' as i32
                || *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
                    as i32
                    == '<' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(1 as i32 as isize) as i32
                        == '!' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(2 as i32 as isize) as i32
                        == '-' as i32
                    && *((*(*ctxt).input).cur as *mut u8)
                        .offset(3 as i32 as isize) as i32
                        == '-' as i32
                || (*(*(*ctxt).input).cur as i32 == 0x20 as i32
                    || 0x9 as i32 <= *(*(*ctxt).input).cur as i32
                        && *(*(*ctxt).input).cur as i32 <= 0xa as i32
                    || *(*(*ctxt).input).cur as i32 == 0xd as i32))
        {
            if *(*(*ctxt).input).cur as i32 == '<' as i32
                && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                    == '?' as i32
            {
                xmlParsePI(ctxt);
            } else if *(*(*ctxt).input).cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *(*(*ctxt).input).cur as i32
                    && *(*(*ctxt).input).cur as i32 <= 0xa as i32
                || *(*(*ctxt).input).cur as i32 == 0xd as i32
            {
                xmlNextChar_safe(ctxt);
            } else {
                xmlParseComment(ctxt);
            }
        }
    }
}
/* *
* xmlParseDocument:
* @ctxt:  an XML parser context
*
* parse an XML document (and build a tree if using the standard SAX
* interface).
*
* [1] document ::= prolog element Misc*
*
* [22] prolog ::= XMLDecl? Misc* (doctypedecl Misc*)?
*
* Returns 0, -1 in case of error. the parser context is augmented
*                as a result of the parsing.
*/

pub unsafe fn xmlParseDocument(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut start: [xmlChar; 4] = [0; 4];
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    unsafe {
        xmlInitParser();
    }
    if ctxt.is_null() || (safe_ctxt).input.is_null() {
        return -(1 as i32);
    }
    if unsafe {
        (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
    } {
        xmlGROW(ctxt);
    }
    /*
     * SAX: detecting the level.
     */
    xmlDetectSAX2(ctxt);
    unsafe {
        /*
         * SAX: beginning of the document processing.
         */
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some() {
            (*(*ctxt).sax)
                .setDocumentLocator
                .expect("non-null function pointer")(
                (*ctxt).userData, __xmlDefaultSAXLocator()
            );
        }
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return -(1 as i32);
    }
    unsafe {
        if (*ctxt).encoding.is_null()
            && (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64
                >= 4 as i32 as i64
        {
            /*
             * Get the 4 first bytes and decode the charset
             * if enc != XML_CHAR_ENCODING_NONE
             * plug some encoding conversion routines.
             */
            start[0 as i32 as usize] = *(*(*ctxt).input).cur;
            start[1 as i32 as usize] =
                *(*(*ctxt).input).cur.offset(1 as i32 as isize);
            start[2 as i32 as usize] =
                *(*(*ctxt).input).cur.offset(2 as i32 as isize);
            start[3 as i32 as usize] =
                *(*(*ctxt).input).cur.offset(3 as i32 as isize);
            enc = xmlDetectCharEncoding_safe(
                &mut *start.as_mut_ptr().offset(0 as i32 as isize),
                4 as i32,
            );
            if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
                xmlSwitchEncoding_safe(ctxt, enc);
            }
        }
        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
            xmlFatalErr(ctxt, XML_ERR_DOCUMENT_EMPTY, 0 as *const i8);
            return -(1 as i32);
        }
        /*
         * Check for the XMLDecl in the Prolog.
         * do not GROW here to avoid the detected encoder to decode more
         * than just the first line, unless the amount of data is really
         * too small to hold "<?xml version="1.0" encoding="foo"
         */
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
            < 35 as i32 as i64
        {
            if (*ctxt).progressive == 0 as i32
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                    < 250 as i32 as i64
            {
                xmlGROW(ctxt);
            }
        }
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '?' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'x' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'm' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'l' as i32
            && (*(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                == 0x20 as i32
                || 0x9 as i32
                    <= *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                    && *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                        <= 0xa as i32
                || *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                    == 0xd as i32)
        {
            /*
             * Note that we will switch encoding on the fly.
             */
            xmlParseXMLDecl(ctxt);
            if (*ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as i32
                || (*ctxt).instate as i32 == XML_PARSER_EOF as i32
            {
                /*
                 * The XML REC instructs us to stop parsing right here
                 */
                return -(1 as i32);
            }
            (*ctxt).standalone = (*(*ctxt).input).standalone;
            xmlSkipBlankChars(ctxt);
        } else {
            (*ctxt).version = xmlCharStrdup_safe(b"1.0\x00" as *const u8 as *const i8)
        }
        if !(*ctxt).sax.is_null()
            && (*(*ctxt).sax).startDocument.is_some()
            && (*ctxt).disableSAX == 0
        {
            (*(*ctxt).sax)
                .startDocument
                .expect("non-null function pointer")((*ctxt).userData);
        }
        if (*ctxt).instate as i32 == XML_PARSER_EOF as i32 {
            return -(1 as i32);
        }
        if !(*ctxt).myDoc.is_null()
            && !(*ctxt).input.is_null()
            && !(*(*ctxt).input).buf.is_null()
            && (*(*(*ctxt).input).buf).compressed >= 0 as i32
        {
            (*(*ctxt).myDoc).compression = (*(*(*ctxt).input).buf).compressed
        }
        /*
         * The Misc part of the Prolog
         */
        if (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        {
            xmlGROW(ctxt);
        }
        xmlParseMisc(ctxt);
        /*
         * Then possibly doc type declaration(s) and more Misc
         * (doctypedecl Misc*)?
         */
        if (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        {
            xmlGROW(ctxt);
        }
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '!' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'D' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'O' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'C' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(5 as i32 as isize)
                as i32
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(6 as i32 as isize)
                as i32
                == 'Y' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(7 as i32 as isize)
                as i32
                == 'P' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(8 as i32 as isize)
                as i32
                == 'E' as i32
        {
            (*ctxt).inSubset = 1 as i32;
            xmlParseDocTypeDecl(ctxt);
            if *(*(*ctxt).input).cur as i32 == '[' as i32 {
                (*ctxt).instate = XML_PARSER_DTD;
                xmlParseInternalSubset(ctxt);
                if (*ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                    return -(1 as i32);
                }
            }
            /*
             * Create and update the external subset.
             */
            (*ctxt).inSubset = 2 as i32;
            if !(*ctxt).sax.is_null()
                && (*(*ctxt).sax).externalSubset.is_some()
                && (*ctxt).disableSAX == 0
            {
                (*(*ctxt).sax)
                    .externalSubset
                    .expect("non-null function pointer")(
                    (*ctxt).userData,
                    (*ctxt).intSubName,
                    (*ctxt).extSubSystem,
                    (*ctxt).extSubURI,
                );
            }
            if (*ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                return -(1 as i32);
            }
            (*ctxt).inSubset = 0 as i32;
            xmlCleanSpecialAttr(ctxt);
            (*ctxt).instate = XML_PARSER_PROLOG;
            xmlParseMisc(ctxt);
        }
        /*
         * Time to start parsing the tree itself
         */
        if (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        {
            xmlGROW(ctxt);
        }
        if *(*(*ctxt).input).cur as i32 != '<' as i32 {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_DOCUMENT_EMPTY,
                b"Start tag expected, \'<\' not found\n\x00" as *const u8 as *const i8,
            );
        } else {
            (*ctxt).instate = XML_PARSER_CONTENT;
            xmlParseElement(ctxt);
            (*ctxt).instate = XML_PARSER_EPILOG;
            /*
             * The Misc part at the end
             */
            xmlParseMisc(ctxt);
            if *(*(*ctxt).input).cur as i32 != 0 as i32 {
                xmlFatalErr(ctxt, XML_ERR_DOCUMENT_END, 0 as *const i8);
            }
            (*ctxt).instate = XML_PARSER_EOF
        }
        /*
         * SAX: end of the document processing.
         */
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
            (*(*ctxt).sax)
                .endDocument
                .expect("non-null function pointer")((*ctxt).userData);
        }
        /*
         * Remove locally kept entity definitions if the tree was not built
         */
        if !(*ctxt).myDoc.is_null()
            && xmlStrEqual_safe(
                (*(*ctxt).myDoc).version,
                b"SAX compatibility mode document\x00" as *const u8 as *const i8
                    as *mut xmlChar,
            ) != 0
        {
            xmlFreeDoc((*ctxt).myDoc);
            (*ctxt).myDoc = 0 as xmlDocPtr
        }
        if (*ctxt).wellFormed != 0 && !(*ctxt).myDoc.is_null() {
            (*(*ctxt).myDoc).properties |= XML_DOC_WELLFORMED as i32;
            if (*ctxt).valid != 0 {
                (*(*ctxt).myDoc).properties |= XML_DOC_DTDVALID as i32
            }
            if (*ctxt).nsWellFormed != 0 {
                (*(*ctxt).myDoc).properties |= XML_DOC_NSVALID as i32
            }
            if (*ctxt).options & XML_PARSE_OLD10 as i32 != 0 {
                (*(*ctxt).myDoc).properties |= XML_DOC_OLD10 as i32
            }
        }
    }
    if (safe_ctxt).wellFormed == 0 {
        (safe_ctxt).valid = 0 as i32;
        return -(1 as i32);
    }
    return 0 as i32;
}
/* *
* xmlParseExtParsedEnt:
* @ctxt:  an XML parser context
*
* parse a general parsed entity
* An external general parsed entity is well-formed if it matches the
* production labeled extParsedEnt.
*
* [78] extParsedEnt ::= TextDecl? content
*
* Returns 0, -1 in case of error. the parser context is augmented
*                as a result of the parsing.
*/

pub unsafe fn xmlParseExtParsedEnt(mut ctxt: xmlParserCtxtPtr) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut start: [xmlChar; 4] = [0; 4];
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    if ctxt.is_null() || (safe_ctxt).input.is_null() {
        return -(1 as i32);
    }
    xmlDefaultSAXHandlerInit_safe();
    xmlDetectSAX2(ctxt);
    unsafe {
        if (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        {
            xmlGROW(ctxt);
        }
        /*
         * SAX: beginning of the document processing.
         */
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some() {
            (*(*ctxt).sax)
                .setDocumentLocator
                .expect("non-null function pointer")(
                (*ctxt).userData, __xmlDefaultSAXLocator()
            );
        }
        /*
         * Get the 4 first bytes and decode the charset
         * if enc != XML_CHAR_ENCODING_NONE
         * plug some encoding conversion routines.
         */
        if (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64
            >= 4 as i32 as i64
        {
            start[0 as i32 as usize] = *(*(*ctxt).input).cur;
            start[1 as i32 as usize] =
                *(*(*ctxt).input).cur.offset(1 as i32 as isize);
            start[2 as i32 as usize] =
                *(*(*ctxt).input).cur.offset(2 as i32 as isize);
            start[3 as i32 as usize] =
                *(*(*ctxt).input).cur.offset(3 as i32 as isize);
            enc = xmlDetectCharEncoding_safe(start.as_mut_ptr(), 4 as i32);
            if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
                xmlSwitchEncoding_safe(ctxt, enc);
            }
        }
        if *(*(*ctxt).input).cur as i32 == 0 as i32 {
            xmlFatalErr(ctxt, XML_ERR_DOCUMENT_EMPTY, 0 as *const i8);
        }
        /*
         * Check for the XMLDecl in the Prolog.
         */
        if (*ctxt).progressive == 0 as i32
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as i64)
                < 250 as i32 as i64
        {
            xmlGROW(ctxt);
        }
        if *((*(*ctxt).input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '?' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'x' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'm' as i32
            && *((*(*ctxt).input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'l' as i32
            && (*(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                == 0x20 as i32
                || 0x9 as i32
                    <= *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                    && *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                        <= 0xa as i32
                || *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                    == 0xd as i32)
        {
            /*
             * Note that we will switch encoding on the fly.
             */
            xmlParseXMLDecl(ctxt);
            if (*ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as i32 {
                /*
                 * The XML REC instructs us to stop parsing right here
                 */
                return -(1 as i32);
            }
            xmlSkipBlankChars(ctxt);
        } else {
            (*ctxt).version = xmlCharStrdup_safe(b"1.0\x00" as *const u8 as *const i8)
        }
        if !(*ctxt).sax.is_null()
            && (*(*ctxt).sax).startDocument.is_some()
            && (*ctxt).disableSAX == 0
        {
            (*(*ctxt).sax)
                .startDocument
                .expect("non-null function pointer")((*ctxt).userData);
        }
    }
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return -(1 as i32);
    }
    /*
     * Doing validity checking on chunk doesn't make sense
     */
    (safe_ctxt).instate = XML_PARSER_CONTENT;
    (safe_ctxt).validate = 0 as i32;
    (safe_ctxt).loadsubset = 0 as i32;
    (safe_ctxt).depth = 0 as i32;
    xmlParseContent(ctxt);
    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
        return -(1 as i32);
    }
    unsafe {
        if *(*(*ctxt).input).cur as i32 == '<' as i32
            && *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32 == '/' as i32
        {
            xmlFatalErr(ctxt, XML_ERR_NOT_WELL_BALANCED, 0 as *const i8);
        } else if *(*(*ctxt).input).cur as i32 != 0 as i32 {
            xmlFatalErr(ctxt, XML_ERR_EXTRA_CONTENT, 0 as *const i8);
        }
        /*
         * SAX: end of the document processing.
         */
        if !(safe_ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
            (*(*ctxt).sax)
                .endDocument
                .expect("non-null function pointer")((*ctxt).userData);
        }
    }
    if (safe_ctxt).wellFormed == 0 {
        return -(1 as i32);
    }
    return 0 as i32;
}
/* ***********************************************************************
*									*
*		Progressive parsing interfaces				*
*									*
************************************************************************/
/* *
* xmlParseLookupSequence:
* @ctxt:  an XML parser context
* @first:  the first char to lookup
* @next:  the next char to lookup or zero
* @third:  the next char to lookup or zero
*
* Try to find if a sequence (first, next, third) or  just (first next) or
* (first) is available in the input stream.
* This function has a side effect of (possibly) incrementing ctxt->checkIndex
* to avoid rescanning sequences of bytes, it DOES change the state of the
* parser, do not use liberally.
*
* Returns the index to the current parsing point if the full sequence
*      is available, -1 otherwise.
*/
#[cfg(HAVE_parser_LIBXML_PUSH_ENABLED)]
unsafe fn xmlParseLookupSequence(
    mut ctxt: xmlParserCtxtPtr,
    mut first: xmlChar,
    mut next: xmlChar,
    mut third: xmlChar,
) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut base: i32 = 0;
    let mut len: i32 = 0;
    let mut in_0: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut buf: *const xmlChar = 0 as *const xmlChar;
    in_0 = (safe_ctxt).input;
    if in_0.is_null() {
        return -(1 as i32);
    }
    base = unsafe { (*in_0).cur.offset_from((*in_0).base) as i64 as i32 };
    if base < 0 as i32 {
        return -(1 as i32);
    }
    if (safe_ctxt).checkIndex > base as i64 {
        base = (safe_ctxt).checkIndex as i32
    }
    let mut safe_in_0 = unsafe { &mut *in_0 };
    if (safe_in_0).buf.is_null() {
        buf = (safe_in_0).base;
        len = (safe_in_0).length
    } else {
        unsafe {
            buf = xmlBufContent((*(*in_0).buf).buffer as *const xmlBuf);
            len = xmlBufUse((*(*in_0).buf).buffer) as i32;
        }
    }
    /* take into account the sequence length */
    if third != 0 {
        len -= 2 as i32
    } else if next != 0 {
        len -= 1
    }
    let mut current_block_20: u64;
    while base < len {
        if unsafe { *buf.offset(base as isize) as i32 == first as i32 } {
            unsafe {
                if third as i32 != 0 as i32 {
                    if *buf.offset((base + 1 as i32) as isize) as i32
                        != next as i32
                        || *buf.offset((base + 2 as i32) as isize) as i32
                            != third as i32
                    {
                        current_block_20 = 2370887241019905314;
                    } else {
                        current_block_20 = 18386322304582297246;
                    }
                } else if next as i32 != 0 as i32 {
                    if *buf.offset((base + 1 as i32) as isize) as i32
                        != next as i32
                    {
                        current_block_20 = 2370887241019905314;
                    } else {
                        current_block_20 = 18386322304582297246;
                    }
                } else {
                    current_block_20 = 18386322304582297246;
                }
            }
            match current_block_20 {
                2370887241019905314 => {}
                _ => {
                    (safe_ctxt).checkIndex = 0 as i32 as i64;

                    //#ifdef DEBUG_PUSH
                    match () {
                        #[cfg(HAVE_parser_DEBUG_PUSH)]
                        _ => unsafe {
                            if next as i32 == 0 as i32 {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"PP: lookup \'%c\' found at %d\n\x00" as *const u8
                                        as *const i8,
                                    first as i32,
                                    base,
                                );
                            } else if third as i32 == 0 as i32 {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"PP: lookup \'%c%c\' found at %d\n\x00" as *const u8
                                        as *const i8,
                                    first as i32,
                                    next as i32,
                                    base,
                                );
                            } else {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"PP: lookup \'%c%c%c\' found at %d\n\x00" as *const u8
                                        as *const i8,
                                    first as i32,
                                    next as i32,
                                    third as i32,
                                    base,
                                );
                            }
                        },
                        #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                        _ => {}
                    };

                    //#endif

                    return unsafe {
                        (base as i64
                            - (*in_0).cur.offset_from((*in_0).base) as i64)
                            as i32
                    };
                }
            }
        }
        base += 1
    }
    (safe_ctxt).checkIndex = base as i64;

    match () {
        #[cfg(HAVE_parser_DEBUG_PUSH)]
        _ => unsafe {
            if next as i32 == 0 as i32 {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"PP: lookup \'%c\' failed\n\x00" as *const u8 as *const i8,
                    first as i32,
                );
            } else if third as i32 == 0 as i32 {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"PP: lookup \'%c%c\' failed\n\x00" as *const u8 as *const i8,
                    first as i32,
                    next as i32,
                );
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"PP: lookup \'%c%c%c\' failed\n\x00" as *const u8 as *const i8,
                    first as i32,
                    next as i32,
                    third as i32,
                );
            }
        },
        #[cfg(not(HAVE_parser_DEBUG_PUSH))]
        _ => {}
    };

    return -(1 as i32);
}
/* *
* xmlParseGetLasts:
* @ctxt:  an XML parser context
* @lastlt:  pointer to store the last '<' from the input
* @lastgt:  pointer to store the last '>' from the input
*
* Lookup the last < and > in the current chunk
*/
#[cfg(HAVE_parser_LIBXML_PUSH_ENABLED)]
unsafe fn xmlParseGetLasts(
    mut ctxt: xmlParserCtxtPtr,
    mut lastlt: *mut *const xmlChar,
    mut lastgt: *mut *const xmlChar,
) {
    let mut tmp: *const xmlChar = 0 as *const xmlChar;
    if ctxt.is_null() || lastlt.is_null() || lastgt.is_null() {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Internal error: xmlParseGetLasts\n\x00" as *const u8 as *const i8,
            );
        }
        return;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if (safe_ctxt).progressive != 0 as i32 && (safe_ctxt).inputNr == 1 as i32 {
        unsafe {
            tmp = (*(*ctxt).input).end;
            tmp = tmp.offset(-1);
            while tmp >= (*(*ctxt).input).base && *tmp as i32 != '<' as i32 {
                tmp = tmp.offset(-1)
            }
            if tmp < (*(*ctxt).input).base {
                *lastlt = 0 as *const xmlChar;
                *lastgt = 0 as *const xmlChar
            } else {
                *lastlt = tmp;
                tmp = tmp.offset(1);
                while tmp < (*(*ctxt).input).end && *tmp as i32 != '>' as i32 {
                    if *tmp as i32 == '\'' as i32 {
                        tmp = tmp.offset(1);
                        while tmp < (*(*ctxt).input).end && *tmp as i32 != '\'' as i32 {
                            tmp = tmp.offset(1)
                        }
                        if tmp < (*(*ctxt).input).end {
                            tmp = tmp.offset(1)
                        }
                    } else if *tmp as i32 == '\"' as i32 {
                        tmp = tmp.offset(1);
                        while tmp < (*(*ctxt).input).end && *tmp as i32 != '\"' as i32 {
                            tmp = tmp.offset(1)
                        }
                        if tmp < (*(*ctxt).input).end {
                            tmp = tmp.offset(1)
                        }
                    } else {
                        tmp = tmp.offset(1)
                    }
                }
                if tmp < (*(*ctxt).input).end {
                    *lastgt = tmp
                } else {
                    tmp = *lastlt;
                    tmp = tmp.offset(-1);
                    while tmp >= (*(*ctxt).input).base && *tmp as i32 != '>' as i32 {
                        tmp = tmp.offset(-1)
                    }
                    if tmp >= (*(*ctxt).input).base {
                        *lastgt = tmp
                    } else {
                        *lastgt = 0 as *const xmlChar
                    }
                }
            }
        }
    } else {
        unsafe {
            *lastlt = 0 as *const xmlChar;
            *lastgt = 0 as *const xmlChar;
        }
    };
}
/* *
* xmlCheckCdataPush:
* @cur: pointer to the block of characters
* @len: length of the block in bytes
* @complete: 1 if complete CDATA block is passed in, 0 if partial block
*
* Check that the block of characters is okay as SCdata content [20]
*
* Returns the number of bytes to pass if okay, a negative index where an
*         UTF-8 error occurred otherwise
*/
#[cfg(HAVE_parser_LIBXML_PUSH_ENABLED)]
unsafe fn xmlCheckCdataPush(
    mut utf: *const xmlChar,
    mut len: i32,
    mut complete: i32,
) -> i32 {
    let mut ix: i32 = 0;
    let mut c: u8 = 0;
    let mut codepoint: i32 = 0;
    if utf.is_null() || len <= 0 as i32 {
        return 0 as i32;
    }
    ix = 0 as i32;
    while ix < len {
        /* string is 0-terminated */
        c = unsafe { *utf.offset(ix as isize) };
        if c as i32 & 0x80 as i32 == 0 as i32 {
            /* 1-byte code, starts with 10 */
            if c as i32 >= 0x20 as i32
            || c as i32 == 0xa as i32
            || c as i32 == 0xd as i32
            || c as i32 == 0x9 as i32{
                ix += 1
            } else {
                return -ix;
            }
        } else if c as i32 & 0xe0 as i32 == 0xc0 as i32 {
            /* 2-byte code, starts with 110 */
            if ix + 2 as i32 > len {
                return if complete != 0 { -ix } else { ix };
            }
            unsafe {
                if *utf.offset((ix + 1 as i32) as isize) as i32
                    & 0xc0 as i32
                    != 0x80 as i32
                {
                    return -ix;
                }
                codepoint = (*utf.offset(ix as isize) as i32 & 0x1f as i32)
                    << 6 as i32;
                codepoint |= *utf.offset((ix + 1 as i32) as isize) as i32
                    & 0x3f as i32;
            }
            if if codepoint < 0x100 as i32 {
                (0x9 as i32 <= codepoint && codepoint <= 0xa as i32
                    || codepoint == 0xd as i32
                    || 0x20 as i32 <= codepoint) as i32
            } else {
                (0x100 as i32 <= codepoint && codepoint <= 0xd7ff as i32
                    || 0xe000 as i32 <= codepoint && codepoint <= 0xfffd as i32
                    || 0x10000 as i32 <= codepoint && codepoint <= 0x10ffff as i32)
                    as i32
            } == 0
            {
                return -ix;
            }
            ix += 2 as i32
        } else if c as i32 & 0xf0 as i32 == 0xe0 as i32 {
            /* 3-byte code, starts with 1110 */
            if ix + 3 as i32 > len {
                return if complete != 0 { -ix } else { ix };
            } /* unknown encoding */
            if unsafe {
                *utf.offset((ix + 1 as i32) as isize) as i32 & 0xc0 as i32
                    != 0x80 as i32
                    || *utf.offset((ix + 2 as i32) as isize) as i32
                        & 0xc0 as i32
                        != 0x80 as i32
            } {
                return -ix;
            }
            unsafe {
                codepoint = (*utf.offset(ix as isize) as i32 & 0xf as i32)
                    << 12 as i32;
                codepoint |= (*utf.offset((ix + 1 as i32) as isize) as i32
                    & 0x3f as i32)
                    << 6 as i32;
                codepoint |= *utf.offset((ix + 2 as i32) as isize) as i32
                    & 0x3f as i32;
            }
            if if codepoint < 0x100 as i32 {
                (0x9 as i32 <= codepoint && codepoint <= 0xa as i32
                    || codepoint == 0xd as i32
                    || 0x20 as i32 <= codepoint) as i32
            } else {
                (0x100 as i32 <= codepoint && codepoint <= 0xd7ff as i32
                    || 0xe000 as i32 <= codepoint && codepoint <= 0xfffd as i32
                    || 0x10000 as i32 <= codepoint && codepoint <= 0x10ffff as i32)
                    as i32
            } == 0
            {
                return -ix;
            }
            ix += 3 as i32
        } else if c as i32 & 0xf8 as i32 == 0xf0 as i32 {
            /* 4-byte code, starts with 11110 */
            if ix + 4 as i32 > len {
                return if complete != 0 { -ix } else { ix };
            }
            unsafe {
                if *utf.offset((ix + 1 as i32) as isize) as i32
                    & 0xc0 as i32
                    != 0x80 as i32
                    || *utf.offset((ix + 2 as i32) as isize) as i32
                        & 0xc0 as i32
                        != 0x80 as i32
                    || *utf.offset((ix + 3 as i32) as isize) as i32
                        & 0xc0 as i32
                        != 0x80 as i32
                {
                    return -ix;
                }
                codepoint = (*utf.offset(ix as isize) as i32 & 0x7 as i32)
                    << 18 as i32;
                codepoint |= (*utf.offset((ix + 1 as i32) as isize) as i32
                    & 0x3f as i32)
                    << 12 as i32;
                codepoint |= (*utf.offset((ix + 2 as i32) as isize) as i32
                    & 0x3f as i32)
                    << 6 as i32;
                codepoint |= *utf.offset((ix + 3 as i32) as isize) as i32
                    & 0x3f as i32;
            }
            if if codepoint < 0x100 as i32 {
                (0x9 as i32 <= codepoint && codepoint <= 0xa as i32
                    || codepoint == 0xd as i32
                    || 0x20 as i32 <= codepoint) as i32
            } else {
                (0x100 as i32 <= codepoint && codepoint <= 0xd7ff as i32
                    || 0xe000 as i32 <= codepoint && codepoint <= 0xfffd as i32
                    || 0x10000 as i32 <= codepoint && codepoint <= 0x10ffff as i32)
                    as i32
            } == 0
            {
                return -ix;
            }
            ix += 4 as i32
        } else {
            return -ix;
        }
    }
    return ix;
}
/* *
* xmlParseTryOrFinish:
* @ctxt:  an XML parser context
* @terminate:  last chunk indicator
*
* Try to progress on parsing
*
* Returns zero if no parsing was possible
*/
#[cfg(HAVE_parser_LIBXML_PUSH_ENABLED)]
unsafe fn xmlParseTryOrFinish(
    mut ctxt: xmlParserCtxtPtr,
    mut terminate: i32,
) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut current_block: u64;
    let mut ret: i32 = 0 as i32;
    let mut avail: i32 = 0;
    let mut tlen: i32 = 0;
    let mut cur: xmlChar = 0;
    let mut next: xmlChar = 0;
    let mut lastlt: *const xmlChar = 0 as *const xmlChar;
    let mut lastgt: *const xmlChar = 0 as *const xmlChar;
    if (safe_ctxt).input.is_null() {
        return 0 as i32;
    }

    match () {
        #[cfg(HAVE_parser_DEBUG_PUSH)]
        _ => unsafe {
            match (safe_ctxt).instate as i32 {
                -1 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try EOF\n\x00" as *const u8 as *const i8,
                    );
                }
                0 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try START\n\x00" as *const u8 as *const i8,
                    );
                }
                1 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try MISC\n\x00" as *const u8 as *const i8,
                    );
                }
                5 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try COMMENT\n\x00" as *const u8 as *const i8,
                    );
                }
                4 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try PROLOG\n\x00" as *const u8 as *const i8,
                    );
                }
                6 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try START_TAG\n\x00" as *const u8 as *const i8,
                    );
                }
                7 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try CONTENT\n\x00" as *const u8 as *const i8,
                    );
                }
                8 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try CDATA_SECTION\n\x00" as *const u8 as *const i8,
                    );
                }
                9 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try END_TAG\n\x00" as *const u8 as *const i8,
                    );
                }
                10 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try ENTITY_DECL\n\x00" as *const u8 as *const i8,
                    );
                }
                11 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try ENTITY_VALUE\n\x00" as *const u8 as *const i8,
                    );
                }
                12 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try ATTRIBUTE_VALUE\n\x00" as *const u8 as *const i8,
                    );
                }
                3 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try DTD\n\x00" as *const u8 as *const i8,
                    );
                }
                14 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try EPILOG\n\x00" as *const u8 as *const i8,
                    );
                }
                2 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try PI\n\x00" as *const u8 as *const i8,
                    );
                }
                15 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: try IGNORE\n\x00" as *const u8 as *const i8,
                    );
                }
                _ => {}
            }
        },
        #[cfg(not(HAVE_parser_DEBUG_PUSH))]
        _ => {}
    };

    if !(safe_ctxt).input.is_null()
        && unsafe {
            (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                > 4096 as i32 as i64
        }
    {
        xmlSHRINK(ctxt);
        (safe_ctxt).checkIndex = 0 as i32 as i64
    }
    xmlParseGetLasts(ctxt, &mut lastlt, &mut lastgt);
    loop {
        if !((safe_ctxt).instate as i32 != XML_PARSER_EOF as i32) {
            current_block = 1672565932838553232;
            break;
        }
        if (safe_ctxt).errNo != XML_ERR_OK as i32
            && (safe_ctxt).disableSAX == 1 as i32
        {
            return 0 as i32;
        }
        if (safe_ctxt).input.is_null() {
            current_block = 1672565932838553232;
            break;
        }
        unsafe {
            if (*(*ctxt).input).buf.is_null() {
                avail = ((*(*ctxt).input).length as i64
                    - (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64)
                    as i32
            } else {
                /*
                 * If we are operating on converted input, try to flush
                 * remaining chars to avoid them stalling in the non-converted
                 * buffer. But do not do this in document start where
                 * encoding="..." may not have been read and we work on a
                 * guessed encoding.
                 */
                if (*ctxt).instate as i32 != XML_PARSER_START as i32
                    && !(*(*(*ctxt).input).buf).raw.is_null()
                    && xmlBufIsEmpty((*(*(*ctxt).input).buf).raw) == 0 as i32
                {
                    let mut base: size_t =
                        xmlBufGetInputBase((*(*(*ctxt).input).buf).buffer, (*ctxt).input);
                    let mut current: size_t =
                        (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                            as size_t;
                    xmlParserInputBufferPush(
                        (*(*ctxt).input).buf,
                        0 as i32,
                        b"\x00" as *const u8 as *const i8,
                    );
                    xmlBufSetInputBaseCur(
                        (*(*(*ctxt).input).buf).buffer,
                        (*ctxt).input,
                        base,
                        current,
                    );
                }
                avail = xmlBufUse((*(*(*ctxt).input).buf).buffer)
                    .wrapping_sub((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                        as i64 as u64) as i32
            }
        }
        if avail < 1 as i32 {
            current_block = 1672565932838553232;
            break;
        }
        match (safe_ctxt).instate as i32 {
            -1 => {
                current_block = 1672565932838553232;
                break;
            }
            0 => {
                if (safe_ctxt).charset == XML_CHAR_ENCODING_NONE as i32 {
                    let mut start: [xmlChar; 4] = [0; 4];
                    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
                    /*
                     * Very first chars read from the document flow.
                     */
                    if avail < 4 as i32 {
                        current_block = 1672565932838553232;
                        break;
                    }
                    unsafe {
                        /*
                         * Get the 4 first bytes and decode the charset
                         * if enc != XML_CHAR_ENCODING_NONE
                         * plug some encoding conversion routines,
                         * else xmlSwitchEncoding will set to (default)
                         * UTF8.
                         */
                        start[0 as i32 as usize] = *(*(*ctxt).input).cur;
                        start[1 as i32 as usize] =
                            *(*(*ctxt).input).cur.offset(1 as i32 as isize);
                        start[2 as i32 as usize] =
                            *(*(*ctxt).input).cur.offset(2 as i32 as isize);
                        start[3 as i32 as usize] =
                            *(*(*ctxt).input).cur.offset(3 as i32 as isize);
                    }
                    enc = xmlDetectCharEncoding_safe(start.as_mut_ptr(), 4 as i32);
                    xmlSwitchEncoding_safe(ctxt, enc);
                } else {
                    if avail < 2 as i32 {
                        current_block = 1672565932838553232;
                        break;
                    }
                    unsafe {
                        cur = *(*(*ctxt).input).cur.offset(0 as i32 as isize);
                        next = *(*(*ctxt).input).cur.offset(1 as i32 as isize);
                    }
                    if cur as i32 == 0 as i32 {
                        unsafe {
                            if !(*ctxt).sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some()
                            {
                                (*(*ctxt).sax)
                                    .setDocumentLocator
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    __xmlDefaultSAXLocator(),
                                );
                            }
                        }
                        xmlFatalErr(ctxt, XML_ERR_DOCUMENT_EMPTY, 0 as *const i8);
                        unsafe {
                            xmlHaltParser(ctxt);
                        }

                        match () {
                            #[cfg(HAVE_parser_DEBUG_PUSH)]
                            _ => unsafe {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"PP: entering EOF\n\x00" as *const u8 as *const i8,
                                );
                            },
                            #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                            _ => {}
                        };

                        unsafe {
                            if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
                                (*(*ctxt).sax)
                                    .endDocument
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData
                                );
                            }
                        }
                        current_block = 1672565932838553232;
                        break;
                    } else if cur as i32 == '<' as i32 && next as i32 == '?' as i32
                    {
                        /* PI or XML decl */
                        if avail < 5 as i32 {
                            return ret;
                        }
                        if terminate == 0
                            && xmlParseLookupSequence(
                                ctxt,
                                '?' as i32 as xmlChar,
                                '>' as i32 as xmlChar,
                                0 as i32 as xmlChar,
                            ) < 0 as i32
                        {
                            return ret;
                        }
                        unsafe {
                            if !(*ctxt).sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some()
                            {
                                (*(*ctxt).sax)
                                    .setDocumentLocator
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    __xmlDefaultSAXLocator(),
                                );
                            }
                            if *(*(*ctxt).input).cur.offset(2 as i32 as isize)
                                as i32
                                == 'x' as i32
                                && *(*(*ctxt).input).cur.offset(3 as i32 as isize)
                                    as i32
                                    == 'm' as i32
                                && *(*(*ctxt).input).cur.offset(4 as i32 as isize)
                                    as i32
                                    == 'l' as i32
                                && (*(*(*ctxt).input).cur.offset(5 as i32 as isize)
                                    as i32
                                    == 0x20 as i32
                                    || 0x9 as i32
                                        <= *(*(*ctxt).input).cur.offset(5 as i32 as isize)
                                            as i32
                                        && *(*(*ctxt).input).cur.offset(5 as i32 as isize)
                                            as i32
                                            <= 0xa as i32
                                    || *(*(*ctxt).input).cur.offset(5 as i32 as isize)
                                        as i32
                                        == 0xd as i32)
                            {
                                ret += 5 as i32;

                                match () {
                                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                                    _ => {
                                        (*__xmlGenericError()).expect("non-null function pointer")(
                                            *__xmlGenericErrorContext(),
                                            b"PP: Parsing XML Decl\n\x00" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                    _ => {}
                                };
                                xmlParseXMLDecl(ctxt);
                                if (*ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as i32 {
                                    /*
                                     * The XML REC instructs us to stop parsing right
                                     * here
                                     */
                                    xmlHaltParser(ctxt);
                                    return 0 as i32;
                                }
                                (*ctxt).standalone = (*(*ctxt).input).standalone;
                                if (*ctxt).encoding.is_null()
                                    && !(*(*ctxt).input).encoding.is_null()
                                {
                                    (*ctxt).encoding = xmlStrdup((*(*ctxt).input).encoding)
                                }
                                if !(*ctxt).sax.is_null()
                                    && (*(*ctxt).sax).startDocument.is_some()
                                    && (*ctxt).disableSAX == 0
                                {
                                    (*(*ctxt).sax)
                                        .startDocument
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData
                                    );
                                }
                                (*ctxt).instate = XML_PARSER_MISC;

                                match () {
                                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                                    _ => {
                                        (*__xmlGenericError()).expect("non-null function pointer")(
                                            *__xmlGenericErrorContext(),
                                            b"PP: entering MISC\n\x00" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                    _ => {}
                                };
                            } else {
                                (*ctxt).version = xmlCharStrdup_safe(
                                    b"1.0\x00" as *const u8 as *const i8,
                                );
                                if !(*ctxt).sax.is_null()
                                    && (*(*ctxt).sax).startDocument.is_some()
                                    && (*ctxt).disableSAX == 0
                                {
                                    (*(*ctxt).sax)
                                        .startDocument
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData
                                    );
                                }
                                (*ctxt).instate = XML_PARSER_MISC;

                                match () {
                                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                                    _ => {
                                        (*__xmlGenericError()).expect("non-null function pointer")(
                                            *__xmlGenericErrorContext(),
                                            b"PP: entering MISC\n\x00" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                    _ => {}
                                };
                            }
                        }
                    } else {
                        unsafe {
                            if !(*ctxt).sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some()
                            {
                                (*(*ctxt).sax)
                                    .setDocumentLocator
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    __xmlDefaultSAXLocator(),
                                );
                            }
                        }
                        (safe_ctxt).version =
                            xmlCharStrdup_safe(b"1.0\x00" as *const u8 as *const i8);
                        if (safe_ctxt).version.is_null() {
                            unsafe {
                                xmlErrMemory(ctxt, 0 as *const i8);
                            }
                        } else {
                            unsafe {
                                if !(*ctxt).sax.is_null()
                                    && (*(*ctxt).sax).startDocument.is_some()
                                    && (*ctxt).disableSAX == 0
                                {
                                    (*(*ctxt).sax)
                                        .startDocument
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData
                                    );
                                }
                                (*ctxt).instate = XML_PARSER_MISC;
                            }
                            match () {
                                #[cfg(HAVE_parser_DEBUG_PUSH)]
                                _ => unsafe {
                                    (*__xmlGenericError()).expect("non-null function pointer")(
                                        *__xmlGenericErrorContext(),
                                        b"PP: entering MISC\n\x00" as *const u8
                                            as *const i8,
                                    );
                                },
                                #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                _ => {}
                            };
                        }
                    }
                }
            }
            6 => {
                let mut name: *const xmlChar = 0 as *const xmlChar;
                let mut prefix: *const xmlChar = 0 as *const xmlChar;
                let mut URI: *const xmlChar = 0 as *const xmlChar;
                let mut line: i32 = unsafe { (*(*ctxt).input).line };
                let mut nsNr: i32 = (safe_ctxt).nsNr;
                if avail < 2 as i32 && (safe_ctxt).inputNr == 1 as i32 {
                    current_block = 1672565932838553232;
                    break;
                }
                cur = unsafe { *(*(*ctxt).input).cur.offset(0 as i32 as isize) };
                if cur as i32 != '<' as i32 {
                    xmlFatalErr(ctxt, XML_ERR_DOCUMENT_EMPTY, 0 as *const i8);
                    unsafe {
                        xmlHaltParser(ctxt);
                        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
                            (*(*ctxt).sax)
                                .endDocument
                                .expect("non-null function pointer")(
                                (*ctxt).userData
                            );
                        }
                    }
                    current_block = 1672565932838553232;
                    break;
                } else {
                    if terminate == 0 {
                        if (safe_ctxt).progressive != 0 {
                            /* > can be found unescaped in attribute values */
                            if lastgt.is_null() || unsafe { (*(*ctxt).input).cur } >= lastgt {
                                current_block = 1672565932838553232;
                                break;
                            }
                        } else if xmlParseLookupSequence(
                            ctxt,
                            '>' as i32 as xmlChar,
                            0 as i32 as xmlChar,
                            0 as i32 as xmlChar,
                        ) < 0 as i32
                        {
                            current_block = 1672565932838553232;
                            break;
                        }
                    }
                    if (safe_ctxt).spaceNr == 0 as i32
                    || unsafe { *(*ctxt).space } == -(2 as i32){
                        spacePush(ctxt, -(1 as i32));
                    } else {
                        spacePush(ctxt, unsafe { *(*ctxt).space });
                    }

                    match () {
                        #[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
                        _ => {
                            if (safe_ctxt).sax2 != 0 {
                                /* LIBXML_SAX1_ENABLED */
                                name = xmlParseStartTag2(ctxt, &mut prefix, &mut URI, &mut tlen)
                            } else {
                                name = xmlParseStartTag(ctxt)
                            }
                        }
                        #[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))]
                        _ => {
                            name = xmlParseStartTag(ctxt);
                        }
                    };

                    /* LIBXML_SAX1_ENABLED */
                    if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                        current_block = 1672565932838553232;
                        break;
                    }
                    if name.is_null() {
                        spacePop(ctxt);
                        unsafe {
                            xmlHaltParser(ctxt);
                            if !(safe_ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
                                (*(*ctxt).sax)
                                    .endDocument
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData
                                );
                            }
                        }
                        current_block = 1672565932838553232;
                        break;
                    } else {
                        /*
                         * [ VC: Root Element Type ]
                         * The Name in the document type declaration must match
                         * the element type of the root element.
                         */

                        match () {
                            #[cfg(HAVE_parser_LIBXML_VALID_ENABLED)]
                            _ => {
                                if (safe_ctxt).validate != 0
                                    && (safe_ctxt).wellFormed != 0
                                    && !(safe_ctxt).myDoc.is_null()
                                    && !(safe_ctxt).node.is_null()
                                    && (safe_ctxt).node == unsafe { (*(*ctxt).myDoc).children }
                                {
                                    (safe_ctxt).valid &= xmlValidateRoot_safe(
                                        &mut (safe_ctxt).vctxt,
                                        (safe_ctxt).myDoc,
                                    )
                                }
                            }
                            #[cfg(not(HAVE_parser_LIBXML_VALID_ENABLED))]
                            _ => {}
                        };

                        /* LIBXML_VALID_ENABLED */
                        /*
                         * Check for an Empty Element.
                         */
                        //@todo 削减unsafe范围
                        unsafe {
                            if *(*(*ctxt).input).cur as i32 == '/' as i32
                                && *(*(*ctxt).input).cur.offset(1 as i32 as isize)
                                    as i32
                                    == '>' as i32
                            {
                                (*(*ctxt).input).cur =
                                    (*(*ctxt).input).cur.offset(2 as i32 as isize);
                                (*(*ctxt).input).col += 2 as i32;
                                if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                                    xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                                }
                                if (*ctxt).sax2 != 0 {
                                    if !(*ctxt).sax.is_null()
                                        && (*(*ctxt).sax).endElementNs.is_some()
                                        && (*ctxt).disableSAX == 0
                                    {
                                        (*(*ctxt).sax)
                                            .endElementNs
                                            .expect("non-null function pointer")(
                                            (*ctxt).userData,
                                            name,
                                            prefix,
                                            URI,
                                        );
                                    }
                                    if (*ctxt).nsNr - nsNr > 0 as i32 {
                                        nsPop(ctxt, (*ctxt).nsNr - nsNr);
                                    }
                                } else {
                                    match () {
                                        #[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
                                        _ => {
                                            if !(*ctxt).sax.is_null()
                                                && (*(*ctxt).sax).endElement.is_some()
                                                && (*ctxt).disableSAX == 0
                                            {
                                                (*(*ctxt).sax)
                                                    .endElement
                                                    .expect("non-null function pointer")(
                                                    (*ctxt).userData,
                                                    name,
                                                );
                                            }
                                        }
                                        #[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))]
                                        _ => {}
                                    };
                                }

                                if (*ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                                    current_block = 1672565932838553232;
                                    break;
                                }
                                spacePop(ctxt);
                                if (*ctxt).nameNr == 0 as i32 {
                                    (*ctxt).instate = XML_PARSER_EPILOG
                                } else {
                                    (*ctxt).instate = XML_PARSER_CONTENT
                                }
                                (*ctxt).progressive = 1 as i32
                            } else {
                                if *(*(*ctxt).input).cur as i32 == '>' as i32 {
                                    xmlNextChar_safe(ctxt);
                                } else {
                                    xmlFatalErrMsgStr(
                                        ctxt,
                                        XML_ERR_GT_REQUIRED,
                                        b"Couldn\'t find end of Start Tag %s\n\x00" as *const u8
                                            as *const i8,
                                        name,
                                    );
                                    nodePop(ctxt);
                                    spacePop(ctxt);
                                }
                                nameNsPush(ctxt, name, prefix, URI, line, (*ctxt).nsNr - nsNr);
                                (*ctxt).instate = XML_PARSER_CONTENT;
                                (*ctxt).progressive = 1 as i32
                            }
                        }
                    }
                }
            }
            7 => {
                let mut test: *const xmlChar = 0 as *const xmlChar;
                let mut cons: u32 = 0;
                if avail < 2 as i32 && (safe_ctxt).inputNr == 1 as i32 {
                    current_block = 1672565932838553232;
                    break;
                }
                unsafe {
                    cur = *(*(*ctxt).input).cur.offset(0 as i32 as isize);
                    next = *(*(*ctxt).input).cur.offset(1 as i32 as isize);
                    test = (*(*ctxt).input).cur;
                    cons = (*(*ctxt).input).consumed as u32;
                }
                if cur as i32 == '<' as i32 && next as i32 == '/' as i32 {
                    (safe_ctxt).instate = XML_PARSER_END_TAG
                } else {
                    if cur as i32 == '<' as i32 && next as i32 == '?' as i32 {
                        if terminate == 0
                            && xmlParseLookupSequence(
                                ctxt,
                                '?' as i32 as xmlChar,
                                '>' as i32 as xmlChar,
                                0 as i32 as xmlChar,
                            ) < 0 as i32
                        {
                            (safe_ctxt).progressive = XML_PARSER_PI as i32;
                            current_block = 1672565932838553232;
                            break;
                        } else {
                            xmlParsePI(ctxt);
                            (safe_ctxt).instate = XML_PARSER_CONTENT;
                            (safe_ctxt).progressive = 1 as i32
                        }
                    } else if cur as i32 == '<' as i32 && next as i32 != '!' as i32
                    {
                        (safe_ctxt).instate = XML_PARSER_START_TAG;
                        continue;
                    } else if cur as i32 == '<' as i32
                        && next as i32 == '!' as i32
                        && unsafe {
                            *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                                == '-' as i32
                                && *(*(*ctxt).input).cur.offset(3 as i32 as isize)
                                    as i32
                                    == '-' as i32
                        }
                    {
                        let mut term: i32 = 0;
                        if avail < 4 as i32 {
                            current_block = 1672565932838553232;
                            break;
                        }
                        unsafe {
                            (*(*ctxt).input).cur =
                                (*(*ctxt).input).cur.offset(4 as i32 as isize);
                        }
                        term = xmlParseLookupSequence(
                            ctxt,
                            '-' as i32 as xmlChar,
                            '-' as i32 as xmlChar,
                            '>' as i32 as xmlChar,
                        );
                        unsafe {
                            (*(*ctxt).input).cur =
                                (*(*ctxt).input).cur.offset(-(4 as i32 as isize));
                        }
                        if terminate == 0 && term < 0 as i32 {
                            (safe_ctxt).progressive = XML_PARSER_COMMENT as i32;
                            current_block = 1672565932838553232;
                            break;
                        } else {
                            xmlParseComment(ctxt);
                            (safe_ctxt).instate = XML_PARSER_CONTENT;
                            (safe_ctxt).progressive = 1 as i32
                        }
                    } else if cur as i32 == '<' as i32
                        && unsafe {
                            *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32
                                == '!' as i32
                                && *(*(*ctxt).input).cur.offset(2 as i32 as isize)
                                    as i32
                                    == '[' as i32
                                && *(*(*ctxt).input).cur.offset(3 as i32 as isize)
                                    as i32
                                    == 'C' as i32
                                && *(*(*ctxt).input).cur.offset(4 as i32 as isize)
                                    as i32
                                    == 'D' as i32
                                && *(*(*ctxt).input).cur.offset(5 as i32 as isize)
                                    as i32
                                    == 'A' as i32
                                && *(*(*ctxt).input).cur.offset(6 as i32 as isize)
                                    as i32
                                    == 'T' as i32
                                && *(*(*ctxt).input).cur.offset(7 as i32 as isize)
                                    as i32
                                    == 'A' as i32
                                && *(*(*ctxt).input).cur.offset(8 as i32 as isize)
                                    as i32
                                    == '[' as i32
                        }
                    {
                        unsafe {
                            (*(*ctxt).input).cur =
                                (*(*ctxt).input).cur.offset(9 as i32 as isize);
                            (*(*ctxt).input).col += 9 as i32;
                            if *(*(*ctxt).input).cur as i32 == 0 as i32 {
                                xmlParserInputGrow_safe((*ctxt).input, 250 as i32);
                            }
                        }
                        (safe_ctxt).instate = XML_PARSER_CDATA_SECTION;
                        continue;
                    } else {
                        if cur as i32 == '<' as i32
                            && next as i32 == '!' as i32
                            && avail < 9 as i32
                        {
                            current_block = 1672565932838553232;
                            break;
                        }
                        if cur as i32 == '&' as i32 {
                            if terminate == 0
                                && xmlParseLookupSequence(
                                    ctxt,
                                    ';' as i32 as xmlChar,
                                    0 as i32 as xmlChar,
                                    0 as i32 as xmlChar,
                                ) < 0 as i32
                            {
                                current_block = 1672565932838553232;
                                break;
                            }
                            xmlParseReference(ctxt);
                        } else {
                            /* LIBXML_SAX1_ENABLED */
                            /* TODO Avoid the extra copy, handle directly !!! */
                            /*
                             * Goal of the following test is:
                             *  - minimize calls to the SAX 'character' callback
                             *    when they are mergeable
                             *  - handle an problem for isBlank when we only parse
                             *    a sequence of blank chars and the next one is
                             *    not available to check against '<' presence.
                             *  - tries to homogenize the differences in SAX
                             *    callbacks between the push and pull versions
                             *    of the parser.
                             */
                            if (safe_ctxt).inputNr == 1 as i32 && avail < 300 as i32
                            {
                                if terminate == 0 {
                                    if (safe_ctxt).progressive != 0 {
                                        if lastlt.is_null()
                                            || unsafe { (*(*ctxt).input).cur > lastlt }
                                        {
                                            current_block = 1672565932838553232;
                                            break;
                                        }
                                    } else if xmlParseLookupSequence(
                                        ctxt,
                                        '<' as i32 as xmlChar,
                                        0 as i32 as xmlChar,
                                        0 as i32 as xmlChar,
                                    ) < 0 as i32
                                    {
                                        current_block = 1672565932838553232;
                                        break;
                                    }
                                }
                            }
                            (safe_ctxt).checkIndex = 0 as i32 as i64;
                            xmlParseCharData(ctxt, 0 as i32);
                        }
                    }
                    unsafe {
                        if !(cons as u64 == (*(*ctxt).input).consumed
                            && test == (*(*ctxt).input).cur)
                        {
                            continue;
                        }
                    }
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_INTERNAL_ERROR,
                        b"detected an error in element content\n\x00" as *const u8
                            as *const i8,
                    );
                    unsafe {
                        xmlHaltParser(ctxt);
                    }
                }
            }
            9 => {
                if avail < 2 as i32 {
                    current_block = 1672565932838553232;
                    break;
                }
                if terminate == 0 {
                    if (safe_ctxt).progressive != 0 {
                        /* > can be found unescaped in attribute values */
                        if lastgt.is_null() || unsafe { (*(*ctxt).input).cur >= lastgt } {
                            current_block = 1672565932838553232;
                            break;
                        }
                    } else if xmlParseLookupSequence(
                        ctxt,
                        '>' as i32 as xmlChar,
                        0 as i32 as xmlChar,
                        0 as i32 as xmlChar,
                    ) < 0 as i32
                    {
                        current_block = 1672565932838553232;
                        break;
                    }
                }
                if (safe_ctxt).sax2 != 0 {
                    unsafe {
                        xmlParseEndTag2(
                            ctxt,
                            &mut *(*ctxt)
                                .pushTab
                                .offset(((*ctxt).nameNr - 1 as i32) as isize),
                        );
                    }
                    nameNsPop(ctxt);
                } else {
                    match () {
                        #[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
                        _ => {
                            xmlParseEndTag1(ctxt, 0 as i32);
                        }
                        #[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))]
                        _ => {}
                    };
                }
                /* LIBXML_SAX1_ENABLED */
                if !((safe_ctxt).instate as i32 == XML_PARSER_EOF as i32) {
                    if (safe_ctxt).nameNr == 0 as i32 {
                        (safe_ctxt).instate = XML_PARSER_EPILOG
                    } else {
                        (safe_ctxt).instate = XML_PARSER_CONTENT
                    }
                }
            }
            8 => {
                /*
                 * The Push mode need to have the SAX callback for
                 * cdataBlock merge back contiguous callbacks.
                 */
                let mut base_0: i32 = 0;
                base_0 = xmlParseLookupSequence(
                    ctxt,
                    ']' as i32 as xmlChar,
                    ']' as i32 as xmlChar,
                    '>' as i32 as xmlChar,
                );
                if base_0 < 0 as i32 {
                    if !(avail >= 300 as i32 + 2 as i32) {
                        current_block = 1672565932838553232;
                        break;
                    }
                    let mut tmp: i32 = 0;
                    tmp = unsafe {
                        xmlCheckCdataPush(
                            (*(*ctxt).input).cur,
                            300 as i32,
                            0 as i32,
                        )
                    };
                    if tmp < 0 as i32 {
                        tmp = -tmp;
                        unsafe {
                            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(tmp as isize);
                        }
                        current_block = 473085638830652887;
                        break;
                    } else {
                        if !(safe_ctxt).sax.is_null() && (safe_ctxt).disableSAX == 0 {
                            unsafe {
                                if (*(*ctxt).sax).cdataBlock.is_some() {
                                    (*(*ctxt).sax)
                                        .cdataBlock
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        (*(*ctxt).input).cur,
                                        tmp,
                                    );
                                } else if (*(*ctxt).sax).characters.is_some() {
                                    (*(*ctxt).sax)
                                        .characters
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        (*(*ctxt).input).cur,
                                        tmp,
                                    );
                                }
                            }
                        }
                        if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                            current_block = 1672565932838553232;
                            break;
                        }
                        let mut skipl: i32 = 0;
                        skipl = 0 as i32;
                        while skipl < tmp {
                            unsafe {
                                if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
                                    (*(*ctxt).input).line += 1;
                                    (*(*ctxt).input).col = 1 as i32
                                } else {
                                    (*(*ctxt).input).col += 1
                                }
                                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
                            }
                            skipl += 1
                        }
                        if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
                            xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
                        }
                        (safe_ctxt).checkIndex = 0 as i32 as i64;
                        current_block = 1672565932838553232;
                        break;
                    }
                } else {
                    let mut tmp_0: i32 = 0;
                    tmp_0 = unsafe {
                        xmlCheckCdataPush((*(*ctxt).input).cur, base_0, 1 as i32)
                    };
                    if tmp_0 < 0 as i32 || tmp_0 != base_0 {
                        tmp_0 = -tmp_0;
                        unsafe {
                            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(tmp_0 as isize)
                        };
                        current_block = 473085638830652887;
                        break;
                    } else {
                        if !(safe_ctxt).sax.is_null()
                            && base_0 == 0 as i32
                            && unsafe { (*(*ctxt).sax).cdataBlock.is_some() }
                            && (safe_ctxt).disableSAX == 0
                        {
                            unsafe {
                                /*
                                 * Special case to provide identical behaviour
                                 * between pull and push parsers on enpty CDATA
                                 * sections
                                 */
                                if (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                    as i64
                                    >= 9 as i32 as i64
                                    && strncmp(
                                        &*(*(*ctxt).input).cur.offset(-(9 as i32) as isize)
                                            as *const xmlChar
                                            as *const i8,
                                        b"<![CDATA[\x00" as *const u8 as *const i8,
                                        9 as i32 as u64,
                                    ) == 0
                                {
                                    (*(*ctxt).sax)
                                        .cdataBlock
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        b"\x00" as *const u8 as *const i8 as *mut xmlChar,
                                        0 as i32,
                                    );
                                }
                            }
                        } else if !(safe_ctxt).sax.is_null()
                            && base_0 > 0 as i32
                            && (safe_ctxt).disableSAX == 0
                        {
                            unsafe {
                                if (*(*ctxt).sax).cdataBlock.is_some() {
                                    (*(*ctxt).sax)
                                        .cdataBlock
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        (*(*ctxt).input).cur,
                                        base_0,
                                    );
                                } else if (*(*ctxt).sax).characters.is_some() {
                                    (*(*ctxt).sax)
                                        .characters
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        (*(*ctxt).input).cur,
                                        base_0,
                                    );
                                }
                            }
                        }
                        if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                            current_block = 1672565932838553232;
                            break;
                        }
                        let mut skipl_0: i32 = 0;
                        skipl_0 = 0 as i32;
                        while skipl_0 < base_0 + 3 as i32 {
                            unsafe {
                                if *(*(*ctxt).input).cur as i32 == '\n' as i32 {
                                    (*(*ctxt).input).line += 1;
                                    (*(*ctxt).input).col = 1 as i32
                                } else {
                                    (*(*ctxt).input).col += 1
                                }
                                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
                            }
                            skipl_0 += 1
                        }
                        if unsafe { *(*(*ctxt).input).cur as i32 == 0 as i32 } {
                            xmlParserInputGrow_safe((safe_ctxt).input, 250 as i32);
                        }
                        (safe_ctxt).checkIndex = 0 as i32 as i64;
                        (safe_ctxt).instate = XML_PARSER_CONTENT;

                        match () {
                            #[cfg(HAVE_parser_DEBUG_PUSH)]
                            _ => unsafe {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"PP: entering CONTENT\n\x00" as *const u8
                                        as *const i8,
                                );
                            },
                            #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                            _ => {}
                        };
                    }
                }
            }
            1 => {
                xmlSkipBlankChars(ctxt);
                unsafe {
                    if (*(*ctxt).input).buf.is_null() {
                        avail = ((*(*ctxt).input).length as i64
                            - (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                as i64) as i32
                    } else {
                        avail = xmlBufUse((*(*(*ctxt).input).buf).buffer)
                            .wrapping_sub((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                as i64
                                as u64) as i32
                    }
                    if avail < 2 as i32 {
                        current_block = 1672565932838553232;
                        break;
                    }
                    cur = *(*(*ctxt).input).cur.offset(0 as i32 as isize);
                    next = *(*(*ctxt).input).cur.offset(1 as i32 as isize);
                    if cur as i32 == '<' as i32 && next as i32 == '?' as i32 {
                        if terminate == 0
                            && xmlParseLookupSequence(
                                ctxt,
                                '?' as i32 as xmlChar,
                                '>' as i32 as xmlChar,
                                0 as i32 as xmlChar,
                            ) < 0 as i32
                        {
                            (*ctxt).progressive = XML_PARSER_PI as i32;
                            current_block = 1672565932838553232;
                            break;
                        } else {
                            match () {
                                #[cfg(HAVE_parser_DEBUG_PUSH)]
                                _ => {
                                    (*__xmlGenericError()).expect("non-null function pointer")(
                                        *__xmlGenericErrorContext(),
                                        b"PP: Parsing PI\n\x00" as *const u8 as *const i8,
                                    );
                                }
                                #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                _ => {}
                            };

                            xmlParsePI(ctxt);
                            if (*ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                                current_block = 1672565932838553232;
                                break;
                            }
                            (*ctxt).instate = XML_PARSER_MISC;
                            (*ctxt).progressive = 1 as i32;
                            (*ctxt).checkIndex = 0 as i32 as i64;
                        }
                    } else if cur as i32 == '<' as i32
                        && next as i32 == '!' as i32
                        && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                            == '-' as i32
                        && *(*(*ctxt).input).cur.offset(3 as i32 as isize) as i32
                            == '-' as i32
                    {
                        if terminate == 0
                            && xmlParseLookupSequence(
                                ctxt,
                                '-' as i32 as xmlChar,
                                '-' as i32 as xmlChar,
                                '>' as i32 as xmlChar,
                            ) < 0 as i32
                        {
                            (*ctxt).progressive = XML_PARSER_COMMENT as i32;
                            current_block = 1672565932838553232;
                            break;
                        } else {
                            match () {
                                #[cfg(HAVE_parser_DEBUG_PUSH)]
                                _ => {
                                    (*__xmlGenericError()).expect("non-null function pointer")(
                                        *__xmlGenericErrorContext(),
                                        b"PP: Parsing Comment\n\x00" as *const u8
                                            as *const i8,
                                    );
                                }
                                #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                _ => {}
                            };

                            xmlParseComment(ctxt);
                            if (*ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                                current_block = 1672565932838553232;
                                break;
                            }
                            (*ctxt).instate = XML_PARSER_MISC;
                            (*ctxt).progressive = 1 as i32;
                            (*ctxt).checkIndex = 0 as i32 as i64
                        }
                    } else if cur as i32 == '<' as i32
                        && next as i32 == '!' as i32
                        && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                            == 'D' as i32
                        && *(*(*ctxt).input).cur.offset(3 as i32 as isize) as i32
                            == 'O' as i32
                        && *(*(*ctxt).input).cur.offset(4 as i32 as isize) as i32
                            == 'C' as i32
                        && *(*(*ctxt).input).cur.offset(5 as i32 as isize) as i32
                            == 'T' as i32
                        && *(*(*ctxt).input).cur.offset(6 as i32 as isize) as i32
                            == 'Y' as i32
                        && *(*(*ctxt).input).cur.offset(7 as i32 as isize) as i32
                            == 'P' as i32
                        && *(*(*ctxt).input).cur.offset(8 as i32 as isize) as i32
                            == 'E' as i32
                    {
                        if terminate == 0
                            && xmlParseLookupSequence(
                                ctxt,
                                '>' as i32 as xmlChar,
                                0 as i32 as xmlChar,
                                0 as i32 as xmlChar,
                            ) < 0 as i32
                        {
                            (safe_ctxt).progressive = XML_PARSER_DTD as i32;
                            current_block = 1672565932838553232;
                            break;
                        } else {
                            match () {
                                #[cfg(HAVE_parser_DEBUG_PUSH)]
                                _ => {
                                    (*__xmlGenericError()).expect("non-null function pointer")(
                                        *__xmlGenericErrorContext(),
                                        b"PP: Parsing internal subset\n\x00" as *const u8
                                            as *const i8,
                                    );
                                }
                                #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                _ => {}
                            };

                            (safe_ctxt).inSubset = 1 as i32;
                            (safe_ctxt).progressive = 0 as i32;
                            (safe_ctxt).checkIndex = 0 as i32 as i64;
                            xmlParseDocTypeDecl(ctxt);
                            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                                current_block = 1672565932838553232;
                                break;
                            }
                            if *(*(*ctxt).input).cur as i32 == '[' as i32 {
                                (safe_ctxt).instate = XML_PARSER_DTD;

                                match () {
                                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                                    _ => {
                                        (*__xmlGenericError()).expect("non-null function pointer")(
                                            *__xmlGenericErrorContext(),
                                            b"PP: entering DTD\n\x00" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                    _ => {}
                                };
                            } else {
                                /*
                                 * Create and update the external subset.
                                 */
                                (safe_ctxt).inSubset = 2 as i32;
                                if !(safe_ctxt).sax.is_null()
                                    && (safe_ctxt).disableSAX == 0
                                    && (*(*ctxt).sax).externalSubset.is_some()
                                {
                                    (*(*ctxt).sax)
                                        .externalSubset
                                        .expect("non-null function pointer")(
                                        (safe_ctxt).userData,
                                        (safe_ctxt).intSubName,
                                        (safe_ctxt).extSubSystem,
                                        (safe_ctxt).extSubURI,
                                    );
                                }
                                (safe_ctxt).inSubset = 0 as i32;
                                xmlCleanSpecialAttr(ctxt);
                                (safe_ctxt).instate = XML_PARSER_PROLOG;

                                match () {
                                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                                    _ => {
                                        (*__xmlGenericError()).expect("non-null function pointer")(
                                            *__xmlGenericErrorContext(),
                                            b"PP: entering PROLOG\n\x00" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                    _ => {}
                                };
                            }
                        }
                    } else {
                        if cur as i32 == '<' as i32
                            && next as i32 == '!' as i32
                            && avail < 9 as i32
                        {
                            current_block = 1672565932838553232;
                            break;
                        }
                        (safe_ctxt).instate = XML_PARSER_START_TAG;
                        (safe_ctxt).progressive = XML_PARSER_START_TAG as i32;
                        xmlParseGetLasts(ctxt, &mut lastlt, &mut lastgt);

                        match () {
                            #[cfg(HAVE_parser_DEBUG_PUSH)]
                            _ => {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"PP: entering START_TAG\n\x00" as *const u8
                                        as *const i8,
                                );
                            }
                            #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                            _ => {}
                        };
                    }
                }
            }
            4 => {
                xmlSkipBlankChars(ctxt);
                unsafe {
                    if (*(*ctxt).input).buf.is_null() {
                        avail = ((*(*ctxt).input).length as i64
                            - (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                as i64) as i32
                    } else {
                        avail = xmlBufUse((*(*(*ctxt).input).buf).buffer)
                            .wrapping_sub((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                as i64
                                as u64) as i32
                    }
                    if avail < 2 as i32 {
                        current_block = 1672565932838553232;
                        break;
                    }
                    cur = *(*(*ctxt).input).cur.offset(0 as i32 as isize);
                    next = *(*(*ctxt).input).cur.offset(1 as i32 as isize);
                }
                if cur as i32 == '<' as i32 && next as i32 == '?' as i32 {
                    if terminate == 0
                        && xmlParseLookupSequence(
                            ctxt,
                            '?' as i32 as xmlChar,
                            '>' as i32 as xmlChar,
                            0 as i32 as xmlChar,
                        ) < 0 as i32
                    {
                        (safe_ctxt).progressive = XML_PARSER_PI as i32;
                        current_block = 1672565932838553232;
                        break;
                    } else {
                        match () {
                            #[cfg(HAVE_parser_DEBUG_PUSH)]
                            _ => unsafe {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"PP: Parsing PI\n\x00" as *const u8 as *const i8,
                                );
                            },
                            #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                            _ => {}
                        };

                        xmlParsePI(ctxt);
                        if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                            current_block = 1672565932838553232;
                            break;
                        }
                        (safe_ctxt).instate = XML_PARSER_PROLOG;
                        (safe_ctxt).progressive = 1 as i32
                    }
                } else if cur as i32 == '<' as i32
                    && next as i32 == '!' as i32
                    && unsafe {
                        *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                            == '-' as i32
                            && *(*(*ctxt).input).cur.offset(3 as i32 as isize)
                                as i32
                                == '-' as i32
                    }
                {
                    if terminate == 0
                        && xmlParseLookupSequence(
                            ctxt,
                            '-' as i32 as xmlChar,
                            '-' as i32 as xmlChar,
                            '>' as i32 as xmlChar,
                        ) < 0 as i32
                    {
                        (safe_ctxt).progressive = XML_PARSER_COMMENT as i32;
                        current_block = 1672565932838553232;
                        break;
                    } else {
                        match () {
                            #[cfg(HAVE_parser_DEBUG_PUSH)]
                            _ => unsafe {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"PP: Parsing Comment\n\x00" as *const u8
                                        as *const i8,
                                );
                            },
                            #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                            _ => {}
                        };

                        xmlParseComment(ctxt);
                        if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                            current_block = 1672565932838553232;
                            break;
                        }
                        (safe_ctxt).instate = XML_PARSER_PROLOG;
                        (safe_ctxt).progressive = 1 as i32
                    }
                } else {
                    if cur as i32 == '<' as i32
                        && next as i32 == '!' as i32
                        && avail < 4 as i32
                    {
                        current_block = 1672565932838553232;
                        break;
                    }
                    (safe_ctxt).instate = XML_PARSER_START_TAG;
                    if (safe_ctxt).progressive == 0 as i32 {
                        (safe_ctxt).progressive = XML_PARSER_START_TAG as i32
                    }
                    xmlParseGetLasts(ctxt, &mut lastlt, &mut lastgt);

                    match () {
                        #[cfg(HAVE_parser_DEBUG_PUSH)]
                        _ => unsafe {
                            (*__xmlGenericError()).expect("non-null function pointer")(
                                *__xmlGenericErrorContext(),
                                b"PP: entering START_TAG\n\x00" as *const u8 as *const i8,
                            );
                        },
                        #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                        _ => {}
                    };
                }
            }
            14 => {
                xmlSkipBlankChars(ctxt);
                unsafe {
                    if (*(*ctxt).input).buf.is_null() {
                        avail = ((*(*ctxt).input).length as i64
                            - (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                as i64) as i32
                    } else {
                        avail = xmlBufUse((*(*(*ctxt).input).buf).buffer)
                            .wrapping_sub((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                as i64
                                as u64) as i32
                    }
                    if avail < 2 as i32 {
                        current_block = 1672565932838553232;
                        break;
                    }
                    cur = *(*(*ctxt).input).cur.offset(0 as i32 as isize);
                    next = *(*(*ctxt).input).cur.offset(1 as i32 as isize);
                    if cur as i32 == '<' as i32 && next as i32 == '?' as i32 {
                        if terminate == 0
                            && xmlParseLookupSequence(
                                ctxt,
                                '?' as i32 as xmlChar,
                                '>' as i32 as xmlChar,
                                0 as i32 as xmlChar,
                            ) < 0 as i32
                        {
                            (safe_ctxt).progressive = XML_PARSER_PI as i32;
                            current_block = 1672565932838553232;
                            break;
                        } else {
                            match () {
                                #[cfg(HAVE_parser_DEBUG_PUSH)]
                                _ => {
                                    (*__xmlGenericError()).expect("non-null function pointer")(
                                        *__xmlGenericErrorContext(),
                                        b"PP: Parsing PI\n\x00" as *const u8 as *const i8,
                                    );
                                }
                                #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                _ => {}
                            };

                            xmlParsePI(ctxt);
                            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                                current_block = 1672565932838553232;
                                break;
                            }
                            (safe_ctxt).instate = XML_PARSER_EPILOG;
                            (safe_ctxt).progressive = 1 as i32
                        }
                    } else if cur as i32 == '<' as i32
                        && next as i32 == '!' as i32
                        && *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32
                            == '-' as i32
                        && *(*(*ctxt).input).cur.offset(3 as i32 as isize) as i32
                            == '-' as i32
                    {
                        if terminate == 0
                            && xmlParseLookupSequence(
                                ctxt,
                                '-' as i32 as xmlChar,
                                '-' as i32 as xmlChar,
                                '>' as i32 as xmlChar,
                            ) < 0 as i32
                        {
                            (safe_ctxt).progressive = XML_PARSER_COMMENT as i32;
                            current_block = 1672565932838553232;
                            break;
                        } else {
                            match () {
                                #[cfg(HAVE_parser_DEBUG_PUSH)]
                                _ => {
                                    (*__xmlGenericError()).expect("non-null function pointer")(
                                        *__xmlGenericErrorContext(),
                                        b"PP: Parsing Comment\n\x00" as *const u8
                                            as *const i8,
                                    );
                                }
                                #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                                _ => {}
                            };

                            xmlParseComment(ctxt);
                            if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                                current_block = 1672565932838553232;
                                break;
                            }
                            (safe_ctxt).instate = XML_PARSER_EPILOG;
                            (safe_ctxt).progressive = 1 as i32
                        }
                    } else {
                        if cur as i32 == '<' as i32
                            && next as i32 == '!' as i32
                            && avail < 4 as i32
                        {
                            current_block = 1672565932838553232;
                            break;
                        }
                        xmlFatalErr(ctxt, XML_ERR_DOCUMENT_END, 0 as *const i8);
                        xmlHaltParser(ctxt);

                        match () {
                            #[cfg(HAVE_parser_DEBUG_PUSH)]
                            _ => {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"PP: entering EOF\n\x00" as *const u8 as *const i8,
                                );
                            }
                            #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                            _ => {}
                        };

                        if !(safe_ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
                            (*(*ctxt).sax)
                                .endDocument
                                .expect("non-null function pointer")(
                                (safe_ctxt).userData
                            );
                        }
                        current_block = 1672565932838553232;
                        break;
                    }
                }
            }
            3 => {
                /*
                 * Sorry but progressive parsing of the internal subset
                 * is not expected to be supported. We first check that
                 * the full content of the internal subset is available and
                 * the parsing is launched only at that point.
                 * Internal subset ends up with "']' S? '>'" in an unescaped
                 * section and not in a ']]>' sequence which are conditional
                 * sections (whoever argued to keep that crap in XML deserve
                 * a place in hell !).
                 */
                let mut base_1: i32 = 0;
                let mut i: i32 = 0;
                let mut buf: *mut xmlChar = 0 as *mut xmlChar;
                let mut quote: xmlChar = 0 as i32 as xmlChar;
                let mut use_0: size_t = 0;
                unsafe {
                    base_1 = (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as i64
                        as i32;
                }
                if base_1 < 0 as i32 {
                    return 0 as i32;
                }
                if (safe_ctxt).checkIndex > base_1 as i64 {
                    base_1 = (safe_ctxt).checkIndex as i32
                }
                unsafe {
                    buf = xmlBufContent((*(*(*ctxt).input).buf).buffer as *const xmlBuf);
                    use_0 = xmlBufUse((*(*(*ctxt).input).buf).buffer);
                }
                's_1946: loop {
                    if !((base_1 as u32 as u64) < use_0) {
                        current_block = 10059826840140668507;
                        break;
                    }
                    if quote as i32 != 0 as i32 {
                        if unsafe {
                            *buf.offset(base_1 as isize) as i32 == quote as i32
                        } {
                            quote = 0 as i32 as xmlChar
                        }
                    } else {
                        if quote as i32 == 0 as i32
                            && unsafe { *buf.offset(base_1 as isize) as i32 == '<' as i32 }
                        {
                            let mut found: i32 = 0 as i32;
                            /* special handling of comments */
                            if unsafe {
                                ((base_1 as u32)
                                    .wrapping_add(4 as i32 as u32)
                                    as u64)
                                    < use_0
                                    && *buf.offset((base_1 + 1 as i32) as isize)
                                        as i32
                                        == '!' as i32
                                    && *buf.offset((base_1 + 2 as i32) as isize)
                                        as i32
                                        == '-' as i32
                                    && *buf.offset((base_1 + 3 as i32) as isize)
                                        as i32
                                        == '-' as i32
                            } {
                                while ((base_1 as u32)
                                    .wrapping_add(3 as i32 as u32)
                                    as u64)
                                    < use_0
                                {
                                    if unsafe {
                                        *buf.offset(base_1 as isize) as i32 == '-' as i32
                                            && *buf.offset((base_1 + 1 as i32) as isize)
                                                as i32
                                                == '-' as i32
                                            && *buf.offset((base_1 + 2 as i32) as isize)
                                                as i32
                                                == '>' as i32
                                    } {
                                        found = 1 as i32;
                                        base_1 += 2 as i32;
                                        break;
                                    } else {
                                        base_1 += 1
                                    }
                                }
                                if found == 0 {
                                    current_block = 10059826840140668507;
                                    break;
                                }
                                current_block = 16936879297222305916;
                            } else {
                                current_block = 9828016697359808143;
                            }
                        } else {
                            current_block = 9828016697359808143;
                        }
                        match current_block {
                            16936879297222305916 => {}
                            _ => {
                                unsafe {
                                    if *buf.offset(base_1 as isize) as i32 == '\"' as i32 {
                                        quote = '\"' as i32 as xmlChar
                                    } else if *buf.offset(base_1 as isize) as i32
                                        == '\'' as i32
                                    {
                                        quote = '\'' as i32 as xmlChar
                                    } else if *buf.offset(base_1 as isize) as i32
                                        == ']' as i32
                                    {
                                        if (base_1 as u32)
                                            .wrapping_add(1 as i32 as u32)
                                            as u64
                                            >= use_0
                                        {
                                            current_block = 10059826840140668507;
                                            break;
                                        }
                                        if *buf.offset((base_1 + 1 as i32) as isize)
                                            as i32
                                            == ']' as i32
                                        {
                                            /* conditional crap, skip both ']' ! */
                                            base_1 += 1
                                        } else {
                                            i = 1 as i32;
                                            loop {
                                                if !(((base_1 as u32)
                                                    .wrapping_add(i as u32)
                                                    as u64)
                                                    < use_0)
                                                {
                                                    current_block = 10059826840140668507;
                                                    break 's_1946;
                                                }
                                                if *buf.offset((base_1 + i) as isize) as i32
                                                    == '>' as i32
                                                {
                                                    current_block = 8979048619177278161;
                                                    break 's_1946;
                                                }
                                                if !(*buf.offset((base_1 + i) as isize)
                                                    as i32
                                                    == 0x20 as i32
                                                    || 0x9 as i32
                                                        <= *buf.offset((base_1 + i) as isize)
                                                            as i32
                                                        && *buf.offset((base_1 + i) as isize)
                                                            as i32
                                                            <= 0xa as i32
                                                    || *buf.offset((base_1 + i) as isize)
                                                        as i32
                                                        == 0xd as i32)
                                                {
                                                    break;
                                                }
                                                i += 1
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    base_1 += 1
                    /* for */
                }
                match current_block {
                    10059826840140668507 => {
                        /* for */
                        if quote as i32 == 0 as i32 {
                            (safe_ctxt).checkIndex = base_1 as i64
                        } else {
                            (safe_ctxt).checkIndex = 0 as i32 as i64
                        }

                        match () {
                            #[cfg(HAVE_parser_DEBUG_PUSH)]
                            _ => {
                                if next as i32 == 0 as i32 {
                                    unsafe {
                                        (*__xmlGenericError()).expect("non-null function pointer")(
                                            *__xmlGenericErrorContext(),
                                            b"PP: lookup of int subset end filed\n\x00" as *const u8
                                                as *const i8,
                                        );
                                    }
                                }
                            }
                            #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                            _ => {}
                        };

                        current_block = 1672565932838553232;
                        break;
                    }
                    _ => {
                        (safe_ctxt).checkIndex = 0 as i32 as i64;
                        xmlParseInternalSubset(ctxt);
                        if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                            current_block = 1672565932838553232;
                            break;
                        }
                        (safe_ctxt).inSubset = 2 as i32;
                        unsafe {
                            if !(safe_ctxt).sax.is_null()
                                && (safe_ctxt).disableSAX == 0
                                && (*(*ctxt).sax).externalSubset.is_some()
                            {
                                (*(*ctxt).sax)
                                    .externalSubset
                                    .expect("non-null function pointer")(
                                    (safe_ctxt).userData,
                                    (safe_ctxt).intSubName,
                                    (safe_ctxt).extSubSystem,
                                    (safe_ctxt).extSubURI,
                                );
                            }
                        }
                        (safe_ctxt).inSubset = 0 as i32;
                        xmlCleanSpecialAttr(ctxt);
                        if (safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
                            current_block = 1672565932838553232;
                            break;
                        }
                        (safe_ctxt).instate = XML_PARSER_PROLOG;
                        (safe_ctxt).checkIndex = 0 as i32 as i64;

                        match () {
                            #[cfg(HAVE_parser_DEBUG_PUSH)]
                            _ => unsafe {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"PP: entering PROLOG\n\x00" as *const u8
                                        as *const i8,
                                );
                            },
                            #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                            _ => {}
                        };
                    }
                }
            }
            5 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: internal error, state == COMMENT\n\x00" as *const u8
                            as *const i8,
                    );
                }
                (safe_ctxt).instate = XML_PARSER_CONTENT;

                match () {
                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                    _ => unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"PP: entering CONTENT\n\x00" as *const u8 as *const i8,
                        );
                    },
                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                    _ => {}
                };
            }
            15 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: internal error, state == IGNORE\x00" as *const u8
                            as *const i8,
                    );
                }
                (safe_ctxt).instate = XML_PARSER_DTD;

                match () {
                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                    _ => unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"PP: entering DTD\n\x00" as *const u8 as *const i8,
                        );
                    },
                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                    _ => {}
                };
            }
            2 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: internal error, state == PI\n\x00" as *const u8
                            as *const i8,
                    );
                }
                (safe_ctxt).instate = XML_PARSER_CONTENT;

                match () {
                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                    _ => unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"PP: entering CONTENT\n\x00" as *const u8 as *const i8,
                        );
                    },
                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                    _ => {}
                };
            }
            10 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: internal error, state == ENTITY_DECL\n\x00" as *const u8
                            as *const i8,
                    );
                }
                (safe_ctxt).instate = XML_PARSER_DTD;

                match () {
                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                    _ => unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"PP: entering DTD\n\x00" as *const u8 as *const i8,
                        );
                    },
                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                    _ => {}
                };
            }
            11 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: internal error, state == ENTITY_VALUE\n\x00" as *const u8
                            as *const i8,
                    );
                }
                (safe_ctxt).instate = XML_PARSER_CONTENT;

                match () {
                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                    _ => unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"PP: entering DTD\n\x00" as *const u8 as *const i8,
                        );
                    },
                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                    _ => {}
                };
            }
            12 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: internal error, state == ATTRIBUTE_VALUE\n\x00" as *const u8
                            as *const i8,
                    );
                }
                (safe_ctxt).instate = XML_PARSER_START_TAG;

                match () {
                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                    _ => unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"PP: entering START_TAG\n\x00" as *const u8 as *const i8,
                        );
                    },
                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                    _ => {}
                };
            }
            13 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: internal error, state == SYSTEM_LITERAL\n\x00" as *const u8
                            as *const i8,
                    );
                }
                (safe_ctxt).instate = XML_PARSER_START_TAG;

                match () {
                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                    _ => unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"PP: entering START_TAG\n\x00" as *const u8 as *const i8,
                        );
                    },
                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                    _ => {}
                };
            }
            16 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: internal error, state == PUBLIC_LITERAL\n\x00" as *const u8
                            as *const i8,
                    );
                }
                (safe_ctxt).instate = XML_PARSER_START_TAG;

                match () {
                    #[cfg(HAVE_parser_DEBUG_PUSH)]
                    _ => unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"PP: entering START_TAG\n\x00" as *const u8 as *const i8,
                        );
                    },
                    #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                    _ => {}
                };
            }
            _ => {}
        }
    }
    match current_block {
        1672565932838553232 =>
        /*
         * We didn't found the end of the Internal subset
         */
        /*
         * Document parsing is done !
         */
        {
            match () {
                #[cfg(HAVE_parser_DEBUG_PUSH)]
                _ => unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: done %d\n\x00" as *const u8 as *const i8,
                        ret,
                    );
                },
                #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                _ => {}
            };

            return ret;
        }
        _ => {
            let mut buffer: [i8; 150] = [0; 150];
            unsafe {
                snprintf(
                    buffer.as_mut_ptr(),
                    149 as i32 as u64,
                    b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8 as *const i8,
                    *(*(*ctxt).input).cur.offset(0 as i32 as isize) as i32,
                    *(*(*ctxt).input).cur.offset(1 as i32 as isize) as i32,
                    *(*(*ctxt).input).cur.offset(2 as i32 as isize) as i32,
                    *(*(*ctxt).input).cur.offset(3 as i32 as isize) as i32,
                );

                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"Input is not proper UTF-8, indicate encoding !\n%s\x00" as *const u8
                        as *const i8,
                    buffer.as_mut_ptr() as *mut xmlChar,
                    0 as *const xmlChar,
                );
            }
            return 0 as i32;
        }
    };
}

/* *
* xmlParseCheckTransition:
* @ctxt:  an XML parser context
* @chunk:  a char array
* @size:  the size in byte of the chunk
*
* Check depending on the current parser state if the chunk given must be
* processed immediately or one need more data to advance on parsing.
*
* Returns -1 in case of error, 0 if the push is not needed and 1 if needed
*/
#[cfg(HAVE_parser_LIBXML_PUSH_ENABLED)]
unsafe fn xmlParseCheckTransition(
    mut ctxt: xmlParserCtxtPtr,
    mut chunk: *const i8,
    mut size: i32,
) -> i32 {
    if ctxt.is_null() || chunk.is_null() || size < 0 as i32 {
        return -(1 as i32);
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    if safe_ctxt.instate as i32 == XML_PARSER_START_TAG as i32 {
        if unsafe {
            !memchr(
                chunk as *const (),
                '>' as i32,
                size as u64,
            )
            .is_null()
        } {
            return 1 as i32;
        }
        return 0 as i32;
    }
    if safe_ctxt.progressive == XML_PARSER_COMMENT as i32 {
        if unsafe {
            !memchr(
                chunk as *const (),
                '>' as i32,
                size as u64,
            )
            .is_null()
        } {
            return 1 as i32;
        }
        return 0 as i32;
    }
    if safe_ctxt.instate as i32 == XML_PARSER_CDATA_SECTION as i32 {
        if unsafe {
            !memchr(
                chunk as *const (),
                '>' as i32,
                size as u64,
            )
            .is_null()
        } {
            return 1 as i32;
        }
        return 0 as i32;
    }
    if safe_ctxt.progressive == XML_PARSER_PI as i32 {
        if unsafe {
            !memchr(
                chunk as *const (),
                '>' as i32,
                size as u64,
            )
            .is_null()
        } {
            return 1 as i32;
        }
        return 0 as i32;
    }
    if safe_ctxt.instate as i32 == XML_PARSER_END_TAG as i32 {
        if unsafe {
            !memchr(
                chunk as *const (),
                '>' as i32,
                size as u64,
            )
            .is_null()
        } {
            return 1 as i32;
        }
        return 0 as i32;
    }
    if safe_ctxt.progressive == XML_PARSER_DTD as i32
        || safe_ctxt.instate as i32 == XML_PARSER_DTD as i32
    {
        if unsafe {
            !memchr(
                chunk as *const (),
                '>' as i32,
                size as u64,
            )
            .is_null()
        } {
            return 1 as i32;
        }
        return 0 as i32;
    }
    return 1 as i32;
}

/* *
* xmlParseChunk:
* @ctxt:  an XML parser context
* @chunk:  an char array
* @size:  the size in byte of the chunk
* @terminate:  last chunk indicator
*
* Parse a Chunk of memory
*
* Returns zero if no error, the xmlParserErrors otherwise.
*/
#[cfg(HAVE_parser_LIBXML_PUSH_ENABLED)]
pub unsafe fn xmlParseChunk(
    mut ctxt: xmlParserCtxtPtr,
    mut chunk: *const i8,
    mut size: i32,
    mut terminate: i32,
) -> i32 {
    let mut end_in_lf: i32 = 0 as i32;
    let mut remain: i32 = 0 as i32;
    let mut old_avail: size_t = 0 as i32 as size_t;
    let mut avail: size_t = 0 as i32 as size_t;
    if ctxt.is_null() {
        return XML_ERR_INTERNAL_ERROR as i32;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    if safe_ctxt.errNo != XML_ERR_OK as i32 && safe_ctxt.disableSAX == 1 as i32 {
        return safe_ctxt.errNo;
    }
    if safe_ctxt.instate as i32 == XML_PARSER_EOF as i32 {
        return -(1 as i32);
    }
    if safe_ctxt.instate as i32 == XML_PARSER_START as i32 {
        unsafe {
            xmlDetectSAX2(ctxt);
        }
    }
    if size > 0 as i32
        && !chunk.is_null()
        && terminate == 0
        && unsafe {
            *chunk.offset((size - 1 as i32) as isize) as i32 == '\r' as i32
        }
    {
        end_in_lf = 1 as i32;
        size -= 1
    }
    loop {
        if size > 0 as i32
            && !chunk.is_null()
            && !safe_ctxt.input.is_null()
            && unsafe { !(*safe_ctxt.input).buf.is_null() }
            && safe_ctxt.instate as i32 != XML_PARSER_EOF as i32
        {
            let mut base: size_t = unsafe {
                xmlBufGetInputBase_safe((*(*safe_ctxt.input).buf).buffer, safe_ctxt.input)
            };
            let mut cur: size_t = unsafe {
                (*safe_ctxt.input).cur.offset_from((*safe_ctxt.input).base) as i64
                    as size_t
            };
            let mut res: i32 = 0;
            old_avail = unsafe { xmlBufUse((*(*safe_ctxt.input).buf).buffer) };
            /*
             * Specific handling if we autodetected an encoding, we should not
             * push more than the first line ... which depend on the encoding
             * And only push the rest once the final encoding was detected
             */
            unsafe {
                if safe_ctxt.instate as i32 == XML_PARSER_START as i32
                    && !safe_ctxt.input.is_null()
                    && !(*safe_ctxt.input).buf.is_null()
                    && !(*(*safe_ctxt.input).buf).encoder.is_null()
                {
                    let mut len: u32 = 45 as i32 as u32;
                    if !xmlStrcasestr(
                        (*(*(*safe_ctxt.input).buf).encoder).name as *mut xmlChar,
                        b"UTF-16\x00" as *const u8 as *const i8 as *mut xmlChar,
                    )
                    .is_null()
                        || !xmlStrcasestr(
                            (*(*(*safe_ctxt.input).buf).encoder).name as *mut xmlChar,
                            b"UTF16\x00" as *const u8 as *const i8 as *mut xmlChar,
                        )
                        .is_null()
                    {
                        len = 90 as i32 as u32
                    } else if !xmlStrcasestr(
                        (*(*(*safe_ctxt.input).buf).encoder).name as *mut xmlChar,
                        b"UCS-4\x00" as *const u8 as *const i8 as *mut xmlChar,
                    )
                    .is_null()
                        || !xmlStrcasestr(
                            (*(*(*safe_ctxt.input).buf).encoder).name as *mut xmlChar,
                            b"UCS4\x00" as *const u8 as *const i8 as *mut xmlChar,
                        )
                        .is_null()
                    {
                        len = 180 as i32 as u32
                    }
                    if (*(*safe_ctxt.input).buf).rawconsumed < len as u64 {
                        len = (len as u64)
                            .wrapping_sub((*(*safe_ctxt.input).buf).rawconsumed)
                            as u32 as u32
                    }
                    /*
                     * Change size for reading the initial declaration only
                     * if size is greater than len. Otherwise, memmove in xmlBufferAdd
                     * will blindly copy extra bytes from memory.
                     */
                    if size as u32 > len {
                        remain = (size as u32).wrapping_sub(len) as i32;
                        size = len as i32
                    } else {
                        remain = 0 as i32
                    }
                }
            }
            unsafe {
                res = xmlParserInputBufferPush_safe((*safe_ctxt.input).buf, size, chunk);
                xmlBufSetInputBaseCur_safe(
                    (*(*safe_ctxt.input).buf).buffer,
                    safe_ctxt.input,
                    base,
                    cur,
                );
            }
            if res < 0 as i32 {
                safe_ctxt.errNo = XML_PARSER_EOF as i32;
                xmlHaltParser(ctxt);
                return XML_PARSER_EOF as i32;
            }

            match () {
                #[cfg(HAVE_parser_DEBUG_PUSH)]
                _ => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"PP: pushed %d\n\x00" as *const u8 as *const i8,
                        size,
                    );
                }
                #[cfg(not(HAVE_parser_DEBUG_PUSH))]
                _ => {}
            };
        } else if safe_ctxt.instate as i32 != XML_PARSER_EOF as i32 {
            if !safe_ctxt.input.is_null() && unsafe { !(*safe_ctxt.input).buf.is_null() } {
                let mut in_0: xmlParserInputBufferPtr = unsafe { (*safe_ctxt.input).buf };
                unsafe {
                    if !(*in_0).encoder.is_null()
                        && !(*in_0).buffer.is_null()
                        && !(*in_0).raw.is_null()
                    {
                        let mut nbchars: i32 = 0;
                        let mut base_0: size_t =
                            xmlBufGetInputBase((*in_0).buffer, safe_ctxt.input);
                        let mut current: size_t =
                            (*safe_ctxt.input).cur.offset_from((*safe_ctxt.input).base)
                                as i64 as size_t;
                        nbchars = xmlCharEncInput(in_0, terminate);
                        xmlBufSetInputBaseCur((*in_0).buffer, safe_ctxt.input, base_0, current);
                        if nbchars < 0 as i32 {
                            /* TODO 2.6.0 */
                            (*__xmlGenericError()).expect("non-null function pointer")(
                                *__xmlGenericErrorContext(),
                                b"xmlParseChunk: encoder error\n\x00" as *const u8
                                    as *const i8,
                            );
                            xmlHaltParser(ctxt);
                            return XML_ERR_INVALID_ENCODING as i32;
                        }
                    }
                }
            }
        }
        if remain != 0 as i32 {
            unsafe {
                xmlParseTryOrFinish(ctxt, 0 as i32);
            }
        } else {
            unsafe {
                if !safe_ctxt.input.is_null() && !(*safe_ctxt.input).buf.is_null() {
                    avail = xmlBufUse((*(*safe_ctxt.input).buf).buffer)
                }
            }
            /*
             * Depending on the current state it may not be such
             * a good idea to try parsing if there is nothing in the chunk
             * which would be worth doing a parser state transition and we
             * need to wait for more data
             */
            unsafe {
                if terminate != 0
                    || avail > 10000000 as i32 as u64
                    || old_avail == 0 as i32 as u64
                    || avail == 0 as i32 as u64
                    || xmlParseCheckTransition(
                        ctxt,
                        &*(*safe_ctxt.input).base.offset(old_avail as isize) as *const xmlChar
                            as *const i8,
                        avail.wrapping_sub(old_avail) as i32,
                    ) != 0
                {
                    xmlParseTryOrFinish(ctxt, terminate);
                }
            }
        }
        if safe_ctxt.instate as i32 == XML_PARSER_EOF as i32 {
            return safe_ctxt.errNo;
        }
        unsafe {
            if !safe_ctxt.input.is_null()
                && ((*safe_ctxt.input).end.offset_from((*safe_ctxt.input).cur) as i64
                    > 10000000 as i32 as i64
                    || (*safe_ctxt.input).cur.offset_from((*safe_ctxt.input).base) as i64
                        > 10000000 as i32 as i64)
                && safe_ctxt.options & XML_PARSE_HUGE as i32 == 0 as i32
            {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"Huge input lookup\x00" as *const u8 as *const i8,
                );
                xmlHaltParser(ctxt);
            }
        }
        if safe_ctxt.errNo != XML_ERR_OK as i32 && safe_ctxt.disableSAX == 1 as i32
        {
            return safe_ctxt.errNo;
        }
        if !(remain != 0 as i32) {
            break;
        }
        unsafe {
            chunk = chunk.offset(size as isize);
        }
        size = remain;
        remain = 0 as i32
    }
    if end_in_lf == 1 as i32
        && !safe_ctxt.input.is_null()
        && unsafe { !(*safe_ctxt.input).buf.is_null() }
    {
        let mut base_1: size_t =
            unsafe { xmlBufGetInputBase((*(*safe_ctxt.input).buf).buffer, safe_ctxt.input) };
        let mut current_0: size_t = unsafe {
            (*safe_ctxt.input).cur.offset_from((*safe_ctxt.input).base) as i64 as size_t
        };
        unsafe {
            xmlParserInputBufferPush(
                (*safe_ctxt.input).buf,
                1 as i32,
                b"\r\x00" as *const u8 as *const i8,
            );
            xmlBufSetInputBaseCur(
                (*(*safe_ctxt.input).buf).buffer,
                safe_ctxt.input,
                base_1,
                current_0,
            );
        }
    }
    if terminate != 0 {
        /*
         * Check for termination
         */
        let mut cur_avail: i32 = 0 as i32;
        unsafe {
            if !safe_ctxt.input.is_null() {
                if (*safe_ctxt.input).buf.is_null() {
                    cur_avail = ((*safe_ctxt.input).length as i64
                        - (*safe_ctxt.input).cur.offset_from((*safe_ctxt.input).base)
                            as i64) as i32
                } else {
                    cur_avail = xmlBufUse((*(*safe_ctxt.input).buf).buffer)
                        .wrapping_sub((*safe_ctxt.input).cur.offset_from((*safe_ctxt.input).base)
                            as i64 as u64)
                        as i32
                }
            }
        }
        if safe_ctxt.instate as i32 != XML_PARSER_EOF as i32
            && safe_ctxt.instate as i32 != XML_PARSER_EPILOG as i32
        {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_DOCUMENT_END, 0 as *const i8);
            }
        }
        if safe_ctxt.instate as i32 == XML_PARSER_EPILOG as i32
            && cur_avail > 0 as i32
        {
            unsafe {
                xmlFatalErr(ctxt, XML_ERR_DOCUMENT_END, 0 as *const i8);
            }
        }
        if safe_ctxt.instate as i32 != XML_PARSER_EOF as i32 {
            if !safe_ctxt.sax.is_null() && unsafe { (*safe_ctxt.sax).endDocument.is_some() } {
                unsafe {
                    (*safe_ctxt.sax)
                        .endDocument
                        .expect("non-null function pointer")(safe_ctxt.userData);
                }
            }
        }
        safe_ctxt.instate = XML_PARSER_EOF
    }
    if safe_ctxt.wellFormed == 0 as i32 {
        return safe_ctxt.errNo as xmlParserErrors as i32;
    } else {
        return 0 as i32;
    };
}
/* ***********************************************************************
*									*
*		I/O front end functions to the parser			*
*									*
************************************************************************/
/* *
* xmlCreatePushParserCtxt:
* @sax:  a SAX handler
* @user_data:  The user data returned on SAX callbacks
* @chunk:  a pointer to an array of chars
* @size:  number of chars in the array
* @filename:  an optional file name or URI
*
* Create a parser context for using the XML parser in push mode.
* If @buffer and @size are non-NULL, the data is used to detect
* the encoding.  The remaining characters will be parsed so they
* don't need to be fed in again through xmlParseChunk.
* To allow content encoding detection, @size should be >= 4
* The value of @filename is used for fetching external entities
* and error/warning reports.
*
* Returns the new parser context or NULL
*/
#[cfg(HAVE_parser_LIBXML_PUSH_ENABLED)]
pub unsafe fn xmlCreatePushParserCtxt(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut chunk: *const i8,
    mut size: i32,
    mut filename: *const i8,
) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    /*
     * plug some encoding conversion routines
     */
    if !chunk.is_null() && size >= 4 as i32 {
        enc = xmlDetectCharEncoding_safe(chunk as *const xmlChar, size)
    }
    buf = xmlAllocParserInputBuffer_safe(enc);
    if buf.is_null() {
        return 0 as xmlParserCtxtPtr;
    }
    ctxt = xmlNewParserCtxt_safe();
    if ctxt.is_null() {
        xmlErrMemory(
            0 as xmlParserCtxtPtr,
            b"creating parser: out of memory\n\x00" as *const u8 as *const i8,
        );
        xmlFreeParserInputBuffer_safe(buf);
        return 0 as xmlParserCtxtPtr;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    safe_ctxt.dictNames = 1 as i32;
    if !sax.is_null() {
        match () {
            #[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
            _ => {
                if safe_ctxt.sax != __xmlDefaultSAXHandler_safe() as xmlSAXHandlerPtr {
                    /* LIBXML_SAX1_ENABLED */
                    xmlFree_safe(safe_ctxt.sax as *mut ());
                }
            }
            #[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))]
            _ => {
                xmlFree_safe(safe_ctxt.sax as *mut ());
            }
        };

        safe_ctxt.sax = xmlMalloc_safe(::std::mem::size_of::<xmlSAXHandler>() as u64)
            as xmlSAXHandlerPtr;
        if safe_ctxt.sax.is_null() {
            xmlErrMemory(ctxt, 0 as *const i8);
            xmlFreeParserInputBuffer_safe(buf);
            xmlFreeParserCtxt_safe(ctxt);
            return 0 as xmlParserCtxtPtr;
        }
        unsafe {
            memset(
                safe_ctxt.sax as *mut (),
                0 as i32,
                ::std::mem::size_of::<xmlSAXHandler>() as u64,
            );
            if (*sax).initialized == 0xdeedbeaf as u32 {
                memcpy(
                    safe_ctxt.sax as *mut (),
                    sax as *const (),
                    ::std::mem::size_of::<xmlSAXHandler>() as u64,
                );
            } else {
                memcpy(
                    safe_ctxt.sax as *mut (),
                    sax as *const (),
                    ::std::mem::size_of::<xmlSAXHandlerV1>() as u64,
                );
            }
        }
        if !user_data.is_null() {
            safe_ctxt.userData = user_data
        }
    }
    if filename.is_null() {
        safe_ctxt.directory = 0 as *mut i8
    } else {
        safe_ctxt.directory = xmlParserGetDirectory_safe(filename)
    }
    inputStream = xmlNewInputStream_safe(ctxt);
    if inputStream.is_null() {
        xmlFreeParserCtxt_safe(ctxt);
        xmlFreeParserInputBuffer_safe(buf);
        return 0 as xmlParserCtxtPtr;
    }
    let mut safe_inputStream = unsafe { &mut *inputStream };

    if filename.is_null() {
        safe_inputStream.filename = 0 as *const i8
    } else {
        safe_inputStream.filename =
            xmlCanonicPath_safe(filename as *const xmlChar) as *mut i8;
        if safe_inputStream.filename.is_null() {
            xmlFreeParserCtxt_safe(ctxt);
            xmlFreeParserInputBuffer_safe(buf);
            return 0 as xmlParserCtxtPtr;
        }
    }
    safe_inputStream.buf = buf;
    unsafe {
        xmlBufResetInput_safe((*safe_inputStream.buf).buffer, inputStream);
    }
    inputPush_safe(ctxt, inputStream);
    /*
     * If the caller didn't provide an initial 'chunk' for determining
     * the encoding, we set the context to XML_CHAR_ENCODING_NONE so
     * that it can be automatically determined later
     */
    if size == 0 as i32 || chunk.is_null() {
        safe_ctxt.charset = XML_CHAR_ENCODING_NONE as i32
    } else if !safe_ctxt.input.is_null() && unsafe { !(*safe_ctxt.input).buf.is_null() } {
        let mut base: size_t =
            unsafe { xmlBufGetInputBase_safe((*(*safe_ctxt.input).buf).buffer, safe_ctxt.input) };
        let mut cur: size_t = unsafe {
            (*safe_ctxt.input).cur.offset_from((*safe_ctxt.input).base) as i64 as size_t
        };
        unsafe {
            xmlParserInputBufferPush_safe((*safe_ctxt.input).buf, size, chunk);
            xmlBufSetInputBaseCur_safe(
                (*(*safe_ctxt.input).buf).buffer,
                safe_ctxt.input,
                base,
                cur,
            );
        }

        match () {
            #[cfg(HAVE_parser_DEBUG_PUSH)]
            _ => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"PP: pushed %d\n\x00" as *const u8 as *const i8,
                    size,
                );
            }
            #[cfg(not(HAVE_parser_DEBUG_PUSH))]
            _ => {}
        };
    }

    if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
        xmlSwitchEncoding_safe(ctxt, enc);
    }
    return ctxt;
}

/* LIBXML_PUSH_ENABLED */
/* *
* xmlHaltParser:
* @ctxt:  an XML parser context
*
* Blocks further parser processing don't override error
* for internal use
*/
unsafe fn xmlHaltParser(mut ctxt: xmlParserCtxtPtr) {
    if ctxt.is_null() {
        return;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    safe_ctxt.instate = XML_PARSER_EOF;
    safe_ctxt.disableSAX = 1 as i32;
    while 1<2 {
        if (!(safe_ctxt.inputNr > 1 as i32)){
            break;
        }
        xmlFreeInputStream_safe(inputPop_safe(ctxt));
    }
    if !safe_ctxt.input.is_null() {
        /*
         * in case there was a specific allocation deallocate before
         * overriding base
         */
        unsafe {
            if (*safe_ctxt.input).free.is_some() {
                (*safe_ctxt.input).free.expect("non-null function pointer")(
                    (*safe_ctxt.input).base as *mut xmlChar,
                );
                (*safe_ctxt.input).free = None
            }
            if !(*safe_ctxt.input).buf.is_null() {
                xmlFreeParserInputBuffer((*safe_ctxt.input).buf);
                (*safe_ctxt.input).buf = 0 as xmlParserInputBufferPtr
            }
            (*safe_ctxt.input).cur = b"\x00" as *const u8 as *const i8 as *mut xmlChar;
            (*safe_ctxt.input).length = 0 as i32;
            (*safe_ctxt.input).base = (*safe_ctxt.input).cur;
            (*safe_ctxt.input).end = (*safe_ctxt.input).cur
        }
    };
}
/* *
* xmlStopParser:
* @ctxt:  an XML parser context
*
* Blocks further parser processing
*/

pub unsafe fn xmlStopParser_parser(mut ctxt: xmlParserCtxtPtr) {
    if ctxt.is_null() {
        return;
    }
    xmlHaltParser(ctxt);
    let mut safe_ctxt = unsafe { &mut *ctxt };

    safe_ctxt.errNo = XML_ERR_USER_STOP as i32;
}
/* *
* xmlCreateIOParserCtxt:
* @sax:  a SAX handler
* @user_data:  The user data returned on SAX callbacks
* @ioread:  an I/O read function
* @ioclose:  an I/O close function
* @ioctx:  an I/O handler
* @enc:  the charset encoding if known
*
* Create a parser context for using the XML parser with an existing
* I/O stream
*
* Returns the new parser context or NULL
*/

pub unsafe fn xmlCreateIOParserCtxt(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut (),
    mut enc: xmlCharEncoding,
) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if ioread.is_none() {
        return 0 as xmlParserCtxtPtr;
    }
    buf = xmlParserInputBufferCreateIO_safe(ioread, ioclose, ioctx, enc);
    if buf.is_null() {
        if ioclose.is_some() {
            unsafe {
                ioclose.expect("non-null function pointer")(ioctx);
            }
        }
        return 0 as xmlParserCtxtPtr;
    }
    ctxt = xmlNewParserCtxt_safe();
    if ctxt.is_null() {
        xmlFreeParserInputBuffer_safe(buf);
        return 0 as xmlParserCtxtPtr;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    if !sax.is_null() {
        match () {
            #[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
            _ => {
                if safe_ctxt.sax != __xmlDefaultSAXHandler_safe() as xmlSAXHandlerPtr {
                    /* LIBXML_SAX1_ENABLED */
                    xmlFree_safe(safe_ctxt.sax as *mut ());
                }
            }
            #[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))]
            _ => {
                xmlFree_safe(safe_ctxt.sax as *mut ());
            }
        };

        safe_ctxt.sax = xmlMalloc_safe(::std::mem::size_of::<xmlSAXHandler>() as u64)
            as xmlSAXHandlerPtr;
        if safe_ctxt.sax.is_null() {
            xmlErrMemory(ctxt, 0 as *const i8);
            xmlFreeParserCtxt_safe(ctxt);
            return 0 as xmlParserCtxtPtr;
        }
        unsafe {
            memset(
                safe_ctxt.sax as *mut (),
                0 as i32,
                ::std::mem::size_of::<xmlSAXHandler>() as u64,
            );
            if (*sax).initialized == 0xdeedbeaf as u32 {
                memcpy(
                    safe_ctxt.sax as *mut (),
                    sax as *const (),
                    ::std::mem::size_of::<xmlSAXHandler>() as u64,
                );
            } else {
                memcpy(
                    safe_ctxt.sax as *mut (),
                    sax as *const (),
                    ::std::mem::size_of::<xmlSAXHandlerV1>() as u64,
                );
            }
        }
        if !user_data.is_null() {
            safe_ctxt.userData = user_data
        }
    }
    inputStream = xmlNewIOInputStream_safe(ctxt, buf, enc);
    if inputStream.is_null() {
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as xmlParserCtxtPtr;
    }
    inputPush_safe(ctxt, inputStream);
    return ctxt;
}

/* ***********************************************************************
*									*
*		Front ends when parsing a DTD				*
*									*
************************************************************************/
/* *
* xmlIOParseDTD:
* @sax:  the SAX handler block or NULL
* @input:  an Input Buffer
* @enc:  the charset encoding if known
*
* Load and parse a DTD
*
* Returns the resulting xmlDtdPtr or NULL in case of error.
* @input will be freed by the function in any case.
*/
#[cfg(HAVE_parser_LIBXML_VALID_ENABLED)]
pub unsafe fn xmlIOParseDTD(
    mut sax: xmlSAXHandlerPtr,
    mut input: xmlParserInputBufferPtr,
    mut enc: xmlCharEncoding,
) -> xmlDtdPtr {
    let mut ret: xmlDtdPtr = 0 as xmlDtdPtr;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut pinput: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut start: [xmlChar; 4] = [0; 4];
    if input.is_null() {
        return 0 as xmlDtdPtr;
    }
    ctxt = xmlNewParserCtxt_safe();
    if ctxt.is_null() {
        unsafe {
            xmlFreeParserInputBuffer(input);
        }
        return 0 as xmlDtdPtr;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    /* We are loading a DTD */
    safe_ctxt.options |= XML_PARSE_DTDLOAD as i32;
    /*
     * Set-up the SAX context
     */
    if !sax.is_null() {
        if !safe_ctxt.sax.is_null() {
            xmlFree_safe(safe_ctxt.sax as *mut ());
        }
        safe_ctxt.sax = sax;
        safe_ctxt.userData = ctxt as *mut ()
    }
    unsafe {
        xmlDetectSAX2(ctxt);
        /*
         * generate a parser input from the I/O handler
         */
        pinput = xmlNewIOInputStream(ctxt, input, XML_CHAR_ENCODING_NONE);
    }
    if pinput.is_null() {
        if !sax.is_null() {
            safe_ctxt.sax = 0 as *mut _xmlSAXHandler
        }
        xmlFreeParserInputBuffer_safe(input);
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as xmlDtdPtr;
    }
    /*
     * plug some encoding conversion routines here.
     */
    if unsafe { xmlPushInput(ctxt, pinput) < 0 as i32 } {
        if !sax.is_null() {
            safe_ctxt.sax = 0 as *mut _xmlSAXHandler
        }
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as xmlDtdPtr;
    }
    if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
        xmlSwitchEncoding_safe(ctxt, enc);
    }
    let mut safe_pinput = unsafe { *pinput };

    safe_pinput.filename = 0 as *const i8;
    safe_pinput.line = 1 as i32;
    safe_pinput.col = 1 as i32;
    unsafe {
        safe_pinput.base = (*safe_ctxt.input).cur;
        safe_pinput.cur = (*safe_ctxt.input).cur;
    }
    safe_pinput.free = None;
    /*
     * let's parse that entity knowing it's an external subset.
     */
    safe_ctxt.inSubset = 2 as i32;
    unsafe {
        safe_ctxt.myDoc = xmlNewDoc(b"1.0\x00" as *const u8 as *const i8 as *mut xmlChar);
    }
    if safe_ctxt.myDoc.is_null() {
        xmlErrMemory(
            ctxt,
            b"New Doc failed\x00" as *const u8 as *const i8,
        );
        return 0 as xmlDtdPtr;
    }
    unsafe {
        (*safe_ctxt.myDoc).properties = XML_DOC_INTERNAL as i32;
        (*safe_ctxt.myDoc).extSubset = xmlNewDtd(
            safe_ctxt.myDoc,
            b"none\x00" as *const u8 as *const i8 as *mut xmlChar,
            b"none\x00" as *const u8 as *const i8 as *mut xmlChar,
            b"none\x00" as *const u8 as *const i8 as *mut xmlChar,
        );
        if enc as i32 == XML_CHAR_ENCODING_NONE as i32
            && (*safe_ctxt.input).end.offset_from((*safe_ctxt.input).cur) as i64
                >= 4 as i32 as i64
        {
            /*
             * Get the 4 first bytes and decode the charset
             * if enc != XML_CHAR_ENCODING_NONE
             * plug some encoding conversion routines.
             */
            start[0 as i32 as usize] = *(*safe_ctxt.input).cur;
            start[1 as i32 as usize] =
                *(*safe_ctxt.input).cur.offset(1 as i32 as isize);
            start[2 as i32 as usize] =
                *(*safe_ctxt.input).cur.offset(2 as i32 as isize);
            start[3 as i32 as usize] =
                *(*safe_ctxt.input).cur.offset(3 as i32 as isize);
            enc = xmlDetectCharEncoding(start.as_mut_ptr(), 4 as i32);
            if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
                xmlSwitchEncoding(ctxt, enc);
            }
        }

        xmlParseExternalSubset(
            ctxt,
            b"none\x00" as *const u8 as *const i8 as *mut xmlChar,
            b"none\x00" as *const u8 as *const i8 as *mut xmlChar,
        );
    }
    if !safe_ctxt.myDoc.is_null() {
        if safe_ctxt.wellFormed != 0 {
            unsafe {
                ret = (*safe_ctxt.myDoc).extSubset;
                (*safe_ctxt.myDoc).extSubset = 0 as *mut _xmlDtd;
                if !ret.is_null() {
                    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
                    (*ret).doc = 0 as *mut _xmlDoc;
                    tmp = (*ret).children;
                    while !tmp.is_null() {
                        (*tmp).doc = 0 as *mut _xmlDoc;
                        tmp = (*tmp).next
                    }
                }
            }
        } else {
            ret = 0 as xmlDtdPtr
        }
        xmlFreeDoc_safe(safe_ctxt.myDoc);
        safe_ctxt.myDoc = 0 as xmlDocPtr
    }
    if !sax.is_null() {
        safe_ctxt.sax = 0 as *mut _xmlSAXHandler
    }
    xmlFreeParserCtxt_safe(ctxt);
    return ret;
}

/* *
* xmlSAXParseDTD:
* @sax:  the SAX handler block
* @ExternalID:  a NAME* containing the External ID of the DTD
* @SystemID:  a NAME* containing the URL to the DTD
*
* Load and parse an external subset.
*
* Returns the resulting xmlDtdPtr or NULL in case of error.
*/
#[cfg(HAVE_parser_LIBXML_VALID_ENABLED)]
pub unsafe fn xmlSAXParseDTD(
    mut sax: xmlSAXHandlerPtr,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) -> xmlDtdPtr {
    let mut ret: xmlDtdPtr = 0 as xmlDtdPtr;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut input: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    let mut systemIdCanonic: *mut xmlChar = 0 as *mut xmlChar;
    if ExternalID.is_null() && SystemID.is_null() {
        return 0 as xmlDtdPtr;
    }
    ctxt = xmlNewParserCtxt_safe();
    if ctxt.is_null() {
        return 0 as xmlDtdPtr;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    /* We are loading a DTD */
    safe_ctxt.options |= XML_PARSE_DTDLOAD as i32;
    /*
     * Set-up the SAX context
     */
    if !sax.is_null() {
        if !safe_ctxt.sax.is_null() {
            xmlFree_safe(safe_ctxt.sax as *mut ());
        }
        safe_ctxt.sax = sax;
        safe_ctxt.userData = ctxt as *mut ()
    }
    /*
     * Canonicalise the system ID
     */
    systemIdCanonic = xmlCanonicPath_safe(SystemID);
    if !SystemID.is_null() && systemIdCanonic.is_null() {
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as xmlDtdPtr;
    }
    /*
     * Ask the Entity resolver to load the damn thing
     */
    unsafe {
        if !safe_ctxt.sax.is_null() && (*safe_ctxt.sax).resolveEntity.is_some() {
            input = (*safe_ctxt.sax)
                .resolveEntity
                .expect("non-null function pointer")(
                safe_ctxt.userData,
                ExternalID,
                systemIdCanonic,
            )
        }
    }
    if input.is_null() {
        if !sax.is_null() {
            safe_ctxt.sax = 0 as *mut _xmlSAXHandler
        }
        xmlFreeParserCtxt_safe(ctxt);
        if !systemIdCanonic.is_null() {
            xmlFree_safe(systemIdCanonic as *mut ());
        }
        return 0 as xmlDtdPtr;
    }
    /*
     * plug some encoding conversion routines here.
     */
    if unsafe { xmlPushInput(ctxt, input) < 0 as i32 } {
        if !sax.is_null() {
            safe_ctxt.sax = 0 as *mut _xmlSAXHandler
        }
        xmlFreeParserCtxt_safe(ctxt);
        if !systemIdCanonic.is_null() {
            xmlFree_safe(systemIdCanonic as *mut ());
        }
        return 0 as xmlDtdPtr;
    }
    unsafe {
        if (*safe_ctxt.input).end.offset_from((*safe_ctxt.input).cur) as i64
            >= 4 as i32 as i64
        {
            enc = xmlDetectCharEncoding((*safe_ctxt.input).cur, 4 as i32);
            xmlSwitchEncoding(ctxt, enc);
        }
    }
    unsafe {
        if (*input).filename.is_null() {
            (*input).filename = systemIdCanonic as *mut i8
        } else {
            xmlFree_safe(systemIdCanonic as *mut ());
        }
        (*input).line = 1 as i32;
        (*input).col = 1 as i32;
        (*input).base = (*safe_ctxt.input).cur;
        (*input).cur = (*safe_ctxt.input).cur;
        (*input).free = None;
    }
    /*
     * let's parse that entity knowing it's an external subset.
     */
    safe_ctxt.inSubset = 2 as i32;
    unsafe {
        safe_ctxt.myDoc = xmlNewDoc(b"1.0\x00" as *const u8 as *const i8 as *mut xmlChar);
    }
    if safe_ctxt.myDoc.is_null() {
        xmlErrMemory(
            ctxt,
            b"New Doc failed\x00" as *const u8 as *const i8,
        );
        if !sax.is_null() {
            safe_ctxt.sax = 0 as *mut _xmlSAXHandler
        }
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as xmlDtdPtr;
    }
    unsafe {
        (*safe_ctxt.myDoc).properties = XML_DOC_INTERNAL as i32;
        (*safe_ctxt.myDoc).extSubset = xmlNewDtd(
            safe_ctxt.myDoc,
            b"none\x00" as *const u8 as *const i8 as *mut xmlChar,
            ExternalID,
            SystemID,
        );
    }
    unsafe {
        xmlParseExternalSubset(ctxt, ExternalID, SystemID);
        if !safe_ctxt.myDoc.is_null() {
            if safe_ctxt.wellFormed != 0 {
                ret = (*safe_ctxt.myDoc).extSubset;
                (*safe_ctxt.myDoc).extSubset = 0 as *mut _xmlDtd;
                if !ret.is_null() {
                    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
                    (*ret).doc = 0 as *mut _xmlDoc;
                    tmp = (*ret).children;
                    while !tmp.is_null() {
                        (*tmp).doc = 0 as *mut _xmlDoc;
                        tmp = (*tmp).next
                    }
                }
            } else {
                ret = 0 as xmlDtdPtr
            }
            xmlFreeDoc_safe(safe_ctxt.myDoc);
            safe_ctxt.myDoc = 0 as xmlDocPtr
        }
    }
    if !sax.is_null() {
        safe_ctxt.sax = 0 as *mut _xmlSAXHandler
    }
    xmlFreeParserCtxt_safe(ctxt);
    return ret;
}
/* *
* xmlParseDTD:
* @ExternalID:  a NAME* containing the External ID of the DTD
* @SystemID:  a NAME* containing the URL to the DTD
*
* Load and parse an external subset.
*
* Returns the resulting xmlDtdPtr or NULL in case of error.
*/
#[cfg(HAVE_parser_LIBXML_VALID_ENABLED)]
pub unsafe fn xmlParseDTD(
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) -> xmlDtdPtr {
    return xmlSAXParseDTD(0 as xmlSAXHandlerPtr, ExternalID, SystemID);
}
/* LIBXML_VALID_ENABLED */
/* ***********************************************************************
*									*
*		Front ends when parsing an Entity			*
*									*
************************************************************************/
/* *
* xmlParseCtxtExternalEntity:
* @ctx:  the existing parsing context
* @URL:  the URL for the entity to load
* @ID:  the System ID for the entity to load
* @lst:  the return value for the set of parsed nodes
*
* Parse an external general entity within an existing parsing context
* An external general parsed entity is well-formed if it matches the
* production labeled extParsedEnt.
*
* [78] extParsedEnt ::= TextDecl? content
*
* Returns 0 if the entity is well formed, -1 in case of args problem and
*    the parser error code otherwise
*/

pub unsafe fn xmlParseCtxtExternalEntity(
    mut ctx: xmlParserCtxtPtr,
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut lst: *mut xmlNodePtr,
) -> i32 {
    let mut userData: *mut () = 0 as *mut ();
    if ctx.is_null() {
        return -(1 as i32);
    }
    /*
     * If the user provided their own SAX callbacks, then reuse the
     * userData callback field, otherwise the expected setup in a
     * DOM builder is to have userData == ctxt
     */
    let mut safe_ctx = unsafe { &mut *ctx };

    if safe_ctx.userData == ctx as *mut () {
        userData = 0 as *mut ()
    } else {
        userData = safe_ctx.userData
    }
    return xmlParseExternalEntityPrivate(
        safe_ctx.myDoc,
        ctx,
        safe_ctx.sax,
        userData,
        safe_ctx.depth + 1 as i32,
        URL,
        ID,
        lst,
    ) as i32;
}

/* *
* xmlParseExternalEntityPrivate:
* @doc:  the document the chunk pertains to
* @oldctxt:  the previous parser context if available
* @sax:  the SAX handler block (possibly NULL)
* @user_data:  The user data returned on SAX callbacks (possibly NULL)
* @depth:  Used for loop detection, use 0
* @URL:  the URL for the entity to load
* @ID:  the System ID for the entity to load
* @list:  the return value for the set of parsed nodes
*
* Private version of xmlParseExternalEntity()
*
* Returns 0 if the entity is well formed, -1 in case of args problem and
*    the parser error code otherwise
*/
unsafe fn xmlParseExternalEntityPrivate(
    mut doc: xmlDocPtr,
    mut oldctxt: xmlParserCtxtPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut depth: i32,
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut list: *mut xmlNodePtr,
) -> xmlParserErrors {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut newDoc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut newRoot: xmlNodePtr = 0 as *mut xmlNode;
    let mut oldsax: xmlSAXHandlerPtr = 0 as xmlSAXHandlerPtr;
    let mut ret: xmlParserErrors = XML_ERR_OK;
    let mut start: [xmlChar; 4] = [0; 4];
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    let mut safe_oldctxt = unsafe { &mut *oldctxt };

    if depth > 40 as i32
        && (oldctxt.is_null()
            || safe_oldctxt.options & XML_PARSE_HUGE as i32 == 0 as i32)
        || depth > 1024 as i32
    {
        return XML_ERR_ENTITY_LOOP;
    }
    unsafe {
        if !list.is_null() {
            *list = 0 as xmlNodePtr
        }
    }

    if URL.is_null() && ID.is_null() {
        return XML_ERR_INTERNAL_ERROR;
    }
    if doc.is_null() {
        return XML_ERR_INTERNAL_ERROR;
    }
    ctxt = xmlCreateEntityParserCtxtInternal(URL, ID, 0 as *const xmlChar, oldctxt);
    if ctxt.is_null() {
        return XML_WAR_UNDECLARED_ENTITY;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    safe_ctxt.userData = ctxt as *mut ();
    if !sax.is_null() {
        oldsax = safe_ctxt.sax;
        safe_ctxt.sax = sax;
        if !user_data.is_null() {
            safe_ctxt.userData = user_data
        }
    }
    unsafe {
        xmlDetectSAX2(ctxt);
        newDoc = xmlNewDoc(b"1.0\x00" as *const u8 as *const i8 as *mut xmlChar);
    }

    if newDoc.is_null() {
        xmlFreeParserCtxt_safe(ctxt);
        return XML_ERR_INTERNAL_ERROR;
    }
    let mut safe_newDoc = unsafe { &mut *newDoc };
    let mut safe_doc = unsafe { &mut *doc };
    safe_newDoc.properties = XML_DOC_INTERNAL as i32;
    if !doc.is_null() {
        safe_newDoc.intSubset = safe_doc.intSubset;
        safe_newDoc.extSubset = safe_doc.extSubset;
        if !safe_doc.dict.is_null() {
            safe_newDoc.dict = safe_doc.dict;
            unsafe {
                xmlDictReference(safe_newDoc.dict);
            }
        }
        if !safe_doc.URL.is_null() {
            safe_newDoc.URL = xmlStrdup_safe(safe_doc.URL)
        }
    }
    unsafe {
        newRoot = xmlNewDocNode(
            newDoc,
            0 as xmlNsPtr,
            b"pseudoroot\x00" as *const u8 as *const i8 as *mut xmlChar,
            0 as *const xmlChar,
        );
    }
    if newRoot.is_null() {
        if !sax.is_null() {
            safe_ctxt.sax = oldsax
        }
        xmlFreeParserCtxt_safe(ctxt);
        safe_newDoc.intSubset = 0 as *mut _xmlDtd;
        safe_newDoc.extSubset = 0 as *mut _xmlDtd;
        xmlFreeDoc_safe(newDoc);
        return XML_ERR_INTERNAL_ERROR;
    }
    unsafe {
        xmlAddChild(newDoc as xmlNodePtr, newRoot);
        nodePush(ctxt, safe_newDoc.children);
    }
    let mut safe_newRoot = unsafe { &mut *newRoot };
    if doc.is_null() {
        safe_ctxt.myDoc = newDoc
    } else {
        safe_ctxt.myDoc = doc;
        safe_newRoot.doc = doc
    }
    /*
     * Get the 4 first bytes and decode the charset
     * if enc != XML_CHAR_ENCODING_NONE
     * plug some encoding conversion routines.
     */
    unsafe {
        if safe_ctxt.progressive == 0 as i32
            && ((*safe_ctxt.input).end.offset_from((*safe_ctxt.input).cur) as i64)
                < 250 as i32 as i64
        {
            xmlGROW(ctxt);
        }
        if (*safe_ctxt.input).end.offset_from((*safe_ctxt.input).cur) as i64
            >= 4 as i32 as i64
        {
            start[0 as i32 as usize] = *(*safe_ctxt.input).cur;
            start[1 as i32 as usize] =
                *(*safe_ctxt.input).cur.offset(1 as i32 as isize);
            start[2 as i32 as usize] =
                *(*safe_ctxt.input).cur.offset(2 as i32 as isize);
            start[3 as i32 as usize] =
                *(*safe_ctxt.input).cur.offset(3 as i32 as isize);
            enc = xmlDetectCharEncoding(start.as_mut_ptr(), 4 as i32);
            if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
                xmlSwitchEncoding(ctxt, enc);
            }
        }
        /*
         * Parse a possible text declaration first
         */
        if *((*safe_ctxt.input).cur as *mut u8).offset(0 as i32 as isize)
            as i32
            == '<' as i32
            && *((*safe_ctxt.input).cur as *mut u8).offset(1 as i32 as isize)
                as i32
                == '?' as i32
            && *((*safe_ctxt.input).cur as *mut u8).offset(2 as i32 as isize)
                as i32
                == 'x' as i32
            && *((*safe_ctxt.input).cur as *mut u8).offset(3 as i32 as isize)
                as i32
                == 'm' as i32
            && *((*safe_ctxt.input).cur as *mut u8).offset(4 as i32 as isize)
                as i32
                == 'l' as i32
            && (*(*safe_ctxt.input).cur.offset(5 as i32 as isize) as i32
                == 0x20 as i32
                || 0x9 as i32
                    <= *(*safe_ctxt.input).cur.offset(5 as i32 as isize) as i32
                    && *(*safe_ctxt.input).cur.offset(5 as i32 as isize) as i32
                        <= 0xa as i32
                || *(*safe_ctxt.input).cur.offset(5 as i32 as isize) as i32
                    == 0xd as i32)
        {
            xmlParseTextDecl(ctxt);
            /*
             * An XML-1.0 document can't reference an entity not XML-1.0
             */
            if xmlStrEqual(
                safe_oldctxt.version,
                b"1.0\x00" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
                && xmlStrEqual(
                    (*safe_ctxt.input).version,
                    b"1.0\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) == 0
            {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_VERSION_MISMATCH,
                    b"Version mismatch between document and entity\n\x00" as *const u8
                        as *const i8,
                );
            }
        }
    }
    safe_ctxt.instate = XML_PARSER_CONTENT;
    safe_ctxt.depth = depth;
    if !oldctxt.is_null() {
        safe_ctxt._private = safe_oldctxt._private;
        safe_ctxt.loadsubset = safe_oldctxt.loadsubset;
        safe_ctxt.validate = safe_oldctxt.validate;
        safe_ctxt.valid = safe_oldctxt.valid;
        safe_ctxt.replaceEntities = safe_oldctxt.replaceEntities;
        if safe_oldctxt.validate != 0 {
            safe_ctxt.vctxt.error = safe_oldctxt.vctxt.error;
            safe_ctxt.vctxt.warning = safe_oldctxt.vctxt.warning;
            safe_ctxt.vctxt.userData = safe_oldctxt.vctxt.userData
        }
        safe_ctxt.external = safe_oldctxt.external;
        if !safe_ctxt.dict.is_null() {
            xmlDictFree_safe(safe_ctxt.dict);
        }
        safe_ctxt.dict = safe_oldctxt.dict;
        unsafe {
            safe_ctxt.str_xml = xmlDictLookup(
                safe_ctxt.dict,
                b"xml\x00" as *const u8 as *const i8 as *mut xmlChar,
                3 as i32,
            );
            safe_ctxt.str_xmlns = xmlDictLookup(
                safe_ctxt.dict,
                b"xmlns\x00" as *const u8 as *const i8 as *mut xmlChar,
                5 as i32,
            );
            safe_ctxt.str_xml_ns = xmlDictLookup(
                safe_ctxt.dict,
                b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as *const i8
                    as *const xmlChar,
                36 as i32,
            );
        }
        safe_ctxt.dictNames = safe_oldctxt.dictNames;
        safe_ctxt.attsDefault = safe_oldctxt.attsDefault;
        safe_ctxt.attsSpecial = safe_oldctxt.attsSpecial;
        safe_ctxt.linenumbers = safe_oldctxt.linenumbers;
        safe_ctxt.record_info = safe_oldctxt.record_info;
        safe_ctxt.node_seq.maximum = safe_oldctxt.node_seq.maximum;
        safe_ctxt.node_seq.length = safe_oldctxt.node_seq.length;
        safe_ctxt.node_seq.buffer = safe_oldctxt.node_seq.buffer
    } else {
        /*
         * Doing validity checking on chunk without context
         * doesn't make sense
         */
        safe_ctxt._private = 0 as *mut ();
        safe_ctxt.validate = 0 as i32;
        safe_ctxt.external = 2 as i32;
        safe_ctxt.loadsubset = 0 as i32
    }
    unsafe {
        xmlParseContent(ctxt);
        if *(*safe_ctxt.input).cur as i32 == '<' as i32
            && *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32
                == '/' as i32
        {
            xmlFatalErr(ctxt, XML_ERR_NOT_WELL_BALANCED, 0 as *const i8);
        } else if *(*safe_ctxt.input).cur as i32 != 0 as i32 {
            xmlFatalErr(ctxt, XML_ERR_EXTRA_CONTENT, 0 as *const i8);
        }
        if safe_ctxt.node != safe_newDoc.children {
            xmlFatalErr(ctxt, XML_ERR_NOT_WELL_BALANCED, 0 as *const i8);
        }
    }
    if safe_ctxt.wellFormed == 0 {
        if safe_ctxt.errNo == 0 as i32 {
            ret = XML_ERR_INTERNAL_ERROR
        } else {
            ret = safe_ctxt.errNo as xmlParserErrors
        }
    } else {
        if !list.is_null() {
            let mut cur: xmlNodePtr = 0 as *mut xmlNode;
            /*
             * Return the newly created nodeset after unlinking it from
             * they pseudo parent.
             */
            unsafe {
                cur = (*safe_newDoc.children).children;
                *list = cur;
                while !cur.is_null() {
                    (*cur).parent = 0 as *mut _xmlNode;
                    cur = (*cur).next
                }
                (*safe_newDoc.children).children = 0 as *mut _xmlNode
            }
        }
        ret = XML_ERR_OK
    }
    /*
     * Record in the parent context the number of entities replacement
     * done when parsing that reference.
     */
    if !oldctxt.is_null() {
        safe_oldctxt.nbentities = safe_oldctxt.nbentities.wrapping_add(safe_ctxt.nbentities)
    }
    /*
     * Also record the size of the entity parsed
     */
    unsafe {
        if !safe_ctxt.input.is_null() && !oldctxt.is_null() {
            safe_oldctxt.sizeentities = safe_oldctxt
                .sizeentities
                .wrapping_add((*safe_ctxt.input).consumed);
            safe_oldctxt.sizeentities = safe_oldctxt
                .sizeentities
                .wrapping_add((*safe_ctxt.input).cur.offset_from((*safe_ctxt.input).base)
                    as i64 as u64)
        }
        /*
         * And record the last error if any
         */
        if !oldctxt.is_null() && safe_ctxt.lastError.code != XML_ERR_OK as i32 {
            xmlCopyError(&mut safe_ctxt.lastError, &mut safe_oldctxt.lastError);
        }
    }
    if !sax.is_null() {
        safe_ctxt.sax = oldsax
    }
    if !oldctxt.is_null() {
        safe_ctxt.dict = 0 as xmlDictPtr;
        safe_ctxt.attsDefault = 0 as xmlHashTablePtr;
        safe_ctxt.attsSpecial = 0 as xmlHashTablePtr;
        safe_oldctxt.validate = safe_ctxt.validate;
        safe_oldctxt.valid = safe_ctxt.valid;
        safe_oldctxt.node_seq.maximum = safe_ctxt.node_seq.maximum;
        safe_oldctxt.node_seq.length = safe_ctxt.node_seq.length;
        safe_oldctxt.node_seq.buffer = safe_ctxt.node_seq.buffer
    }
    safe_ctxt.node_seq.maximum = 0 as i32 as u64;
    safe_ctxt.node_seq.length = 0 as i32 as u64;
    safe_ctxt.node_seq.buffer = 0 as *mut xmlParserNodeInfo;
    xmlFreeParserCtxt_safe(ctxt);
    safe_newDoc.intSubset = 0 as *mut _xmlDtd;
    safe_newDoc.extSubset = 0 as *mut _xmlDtd;
    xmlFreeDoc_safe(newDoc);
    return ret;
}
/* *
* xmlParseExternalEntity:
* @doc:  the document the chunk pertains to
* @sax:  the SAX handler block (possibly NULL)
* @user_data:  The user data returned on SAX callbacks (possibly NULL)
* @depth:  Used for loop detection, use 0
* @URL:  the URL for the entity to load
* @ID:  the System ID for the entity to load
* @lst:  the return value for the set of parsed nodes
*
* Parse an external general entity
* An external general parsed entity is well-formed if it matches the
* production labeled extParsedEnt.
*
* [78] extParsedEnt ::= TextDecl? content
*
* Returns 0 if the entity is well formed, -1 in case of args problem and
*    the parser error code otherwise
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlParseExternalEntity(
    mut doc: xmlDocPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut depth: i32,
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut lst: *mut xmlNodePtr,
) -> i32 {
    return xmlParseExternalEntityPrivate(
        doc,
        0 as xmlParserCtxtPtr,
        sax,
        user_data,
        depth,
        URL,
        ID,
        lst,
    ) as i32;
}
/* *
* xmlParseBalancedChunkMemory:
* @doc:  the document the chunk pertains to (must not be NULL)
* @sax:  the SAX handler block (possibly NULL)
* @user_data:  The user data returned on SAX callbacks (possibly NULL)
* @depth:  Used for loop detection, use 0
* @string:  the input string in UTF8 or ISO-Latin (zero terminated)
* @lst:  the return value for the set of parsed nodes
*
* Parse a well-balanced chunk of an XML document
* called by the parser
* The allowed sequence for the Well Balanced Chunk is the one defined by
* the content production in the XML grammar:
*
* [43] content ::= (element | CharData | Reference | CDSect | PI | Comment)*
*
* Returns 0 if the chunk is well balanced, -1 in case of args problem and
*    the parser error code otherwise
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlParseBalancedChunkMemory(
    mut doc: xmlDocPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut depth: i32,
    mut string: *const xmlChar,
    mut lst: *mut xmlNodePtr,
) -> i32 {
    return xmlParseBalancedChunkMemoryRecover(
        doc,
        sax,
        user_data,
        depth,
        string,
        lst,
        0 as i32,
    );
}
/* LIBXML_LEGACY_ENABLED */
/* LIBXML_SAX1_ENABLED */
/* *
* xmlParseBalancedChunkMemoryInternal:
* @oldctxt:  the existing parsing context
* @string:  the input string in UTF8 or ISO-Latin (zero terminated)
* @user_data:  the user data field for the parser context
* @lst:  the return value for the set of parsed nodes
*
*
* Parse a well-balanced chunk of an XML document
* called by the parser
* The allowed sequence for the Well Balanced Chunk is the one defined by
* the content production in the XML grammar:
*
* [43] content ::= (element | CharData | Reference | CDSect | PI | Comment)*
*
* Returns XML_ERR_OK if the chunk is well balanced, and the parser
* error code otherwise
*
* In case recover is set to 1, the nodelist will not be empty even if
* the parsed chunk is not well balanced.
*/

unsafe fn xmlParseBalancedChunkMemoryInternal(
    mut oldctxt: xmlParserCtxtPtr,
    mut string: *const xmlChar,
    mut user_data: *mut (),
    mut lst: *mut xmlNodePtr,
) -> xmlParserErrors {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut newDoc: xmlDocPtr = 0 as xmlDocPtr;
    let mut newRoot: xmlNodePtr = 0 as *mut xmlNode;
    let mut oldsax: xmlSAXHandlerPtr = 0 as xmlSAXHandlerPtr;
    let mut content: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut size: i32 = 0;
    let mut ret: xmlParserErrors = XML_ERR_OK;

    // match () {
    //     #[cfg(HAVE_parser_SAX2)] _ => {
    //         let mut i: i32 = 0;
    //     }
    //     #[cfg(not(HAVE_parser_SAX2))] _ => {}
    // };

    let mut safe_oldctxt = unsafe { &mut *oldctxt };
    if safe_oldctxt.depth > 40 as i32
        && safe_oldctxt.options & XML_PARSE_HUGE as i32 == 0 as i32
        || safe_oldctxt.depth > 1024 as i32
    {
        return XML_ERR_ENTITY_LOOP;
    }
    unsafe {
        if !lst.is_null() {
            *lst = 0 as xmlNodePtr
        }
    }
    if string.is_null() {
        return XML_ERR_INTERNAL_ERROR;
    }
    size = xmlStrlen_safe(string);
    ctxt = xmlCreateMemoryParserCtxt_safe(string as *mut i8, size);
    if ctxt.is_null() {
        return XML_WAR_UNDECLARED_ENTITY;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    if !user_data.is_null() {
        safe_ctxt.userData = user_data
    } else {
        safe_ctxt.userData = ctxt as *mut ()
    }
    unsafe {
        if !safe_ctxt.dict.is_null() {
            xmlDictFree(safe_ctxt.dict);
        }
    }
    safe_ctxt.dict = safe_oldctxt.dict;
    safe_ctxt.input_id = safe_oldctxt.input_id + 1 as i32;
    unsafe {
        safe_ctxt.str_xml = xmlDictLookup(
            safe_ctxt.dict,
            b"xml\x00" as *const u8 as *const i8 as *mut xmlChar,
            3 as i32,
        );
        safe_ctxt.str_xmlns = xmlDictLookup(
            safe_ctxt.dict,
            b"xmlns\x00" as *const u8 as *const i8 as *mut xmlChar,
            5 as i32,
        );
        safe_ctxt.str_xml_ns = xmlDictLookup(
            safe_ctxt.dict,
            b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as *const i8
                as *const xmlChar,
            36 as i32,
        );
    }

    match () {
        #[cfg(HAVE_parser_SAX2)]
        _ => {
            unsafe {
                /* propagate namespaces down the entity */
                let mut i: i32 = 0;
                while i < safe_oldctxt.nsNr {
                    nsPush(
                        ctxt,
                        *safe_oldctxt.nsTab.offset(i as isize),
                        *safe_oldctxt.nsTab.offset((i + 1 as i32) as isize),
                    );
                    i += 2 as i32
                }
            }
        }
        #[cfg(not(HAVE_parser_SAX2))]
        _ => {}
    };

    oldsax = safe_ctxt.sax;
    safe_ctxt.sax = safe_oldctxt.sax;
    unsafe {
        xmlDetectSAX2(ctxt);
    }
    safe_ctxt.replaceEntities = safe_oldctxt.replaceEntities;
    safe_ctxt.options = safe_oldctxt.options;
    safe_ctxt._private = safe_oldctxt._private;
    if safe_oldctxt.myDoc.is_null() {
        unsafe {
            newDoc = xmlNewDoc(b"1.0\x00" as *const u8 as *const i8 as *mut xmlChar);
        }
        if newDoc.is_null() {
            safe_ctxt.sax = oldsax;
            safe_ctxt.dict = 0 as xmlDictPtr;
            xmlFreeParserCtxt_safe(ctxt);
            return XML_ERR_INTERNAL_ERROR;
        }
        let mut safe_newDoc = unsafe { &mut *newDoc };

        safe_newDoc.properties = XML_DOC_INTERNAL as i32;
        safe_newDoc.dict = safe_ctxt.dict;
        unsafe {
            xmlDictReference(safe_newDoc.dict);
        }
        safe_ctxt.myDoc = newDoc
    } else {
        safe_ctxt.myDoc = safe_oldctxt.myDoc;
        unsafe {
            content = (*safe_ctxt.myDoc).children;
            last = (*safe_ctxt.myDoc).last
        }
    }
    unsafe {
        newRoot = xmlNewDocNode(
            safe_ctxt.myDoc,
            0 as xmlNsPtr,
            b"pseudoroot\x00" as *const u8 as *const i8 as *mut xmlChar,
            0 as *const xmlChar,
        );
        if newRoot.is_null() {
            safe_ctxt.sax = oldsax;
            safe_ctxt.dict = 0 as xmlDictPtr;
            xmlFreeParserCtxt(ctxt);
            if !newDoc.is_null() {
                xmlFreeDoc(newDoc);
            }
            return XML_ERR_INTERNAL_ERROR;
        }
        (*safe_ctxt.myDoc).children = 0 as *mut _xmlNode;
        (*safe_ctxt.myDoc).last = 0 as *mut _xmlNode;
        xmlAddChild(safe_ctxt.myDoc as xmlNodePtr, newRoot);
        nodePush(ctxt, (*safe_ctxt.myDoc).children);
    }
    safe_ctxt.instate = XML_PARSER_CONTENT;
    safe_ctxt.depth = safe_oldctxt.depth + 1 as i32;
    safe_ctxt.validate = 0 as i32;
    safe_ctxt.loadsubset = safe_oldctxt.loadsubset;
    if safe_oldctxt.validate != 0 || safe_oldctxt.replaceEntities != 0 as i32 {
        /*
         * ID/IDREF registration will be done in xmlValidateElement below
         */
        safe_ctxt.loadsubset |= 8 as i32
    }
    safe_ctxt.dictNames = safe_oldctxt.dictNames;
    safe_ctxt.attsDefault = safe_oldctxt.attsDefault;
    safe_ctxt.attsSpecial = safe_oldctxt.attsSpecial;
    unsafe {
        xmlParseContent(ctxt);
        if *(*safe_ctxt.input).cur as i32 == '<' as i32
            && *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32
                == '/' as i32
        {
            xmlFatalErr(ctxt, XML_ERR_NOT_WELL_BALANCED, 0 as *const i8);
        } else if *(*safe_ctxt.input).cur as i32 != 0 as i32 {
            xmlFatalErr(ctxt, XML_ERR_EXTRA_CONTENT, 0 as *const i8);
        }
        if safe_ctxt.node != (*safe_ctxt.myDoc).children {
            xmlFatalErr(ctxt, XML_ERR_NOT_WELL_BALANCED, 0 as *const i8);
        }
    }
    if safe_ctxt.wellFormed == 0 {
        if safe_ctxt.errNo == 0 as i32 {
            ret = XML_ERR_INTERNAL_ERROR
        } else {
            ret = safe_ctxt.errNo as xmlParserErrors
        }
    } else {
        ret = XML_ERR_OK
    }
    if !lst.is_null() && ret as u32 == XML_ERR_OK as i32 as u32 {
        let mut cur: xmlNodePtr = 0 as *mut xmlNode;
        /*
         * Return the newly created nodeset after unlinking it from
         * they pseudo parent.
         */
        unsafe {
            cur = (*(*safe_ctxt.myDoc).children).children;
            *lst = cur;
        }
        while !cur.is_null() {
            match () {
                #[cfg(HAVE_parser_LIBXML_VALID_ENABLED)]
                _ => {
                    let mut safe_cur = unsafe { &mut *cur };

                    if safe_oldctxt.validate != 0
                        && safe_oldctxt.wellFormed != 0
                        && !safe_oldctxt.myDoc.is_null()
                        && unsafe { !(*safe_oldctxt.myDoc).intSubset.is_null() }
                        && safe_cur.type_0 as u32
                            == XML_ELEMENT_NODE as i32 as u32
                    {
                        unsafe {
                            safe_oldctxt.valid &=
                                xmlValidateElement(&mut safe_oldctxt.vctxt, safe_oldctxt.myDoc, cur)
                        }
                    }
                }
                #[cfg(not(HAVE_parser_LIBXML_VALID_ENABLED))]
                _ => {}
            };
            let mut safe_cur = unsafe { &mut *cur };
            safe_cur.parent = 0 as *mut _xmlNode;
            cur = safe_cur.next
        }
        unsafe { (*(*safe_ctxt.myDoc).children).children = 0 as *mut _xmlNode }
    }
    unsafe {
        if !safe_ctxt.myDoc.is_null() {
            xmlFreeNode((*safe_ctxt.myDoc).children);
            (*safe_ctxt.myDoc).children = content;
            (*safe_ctxt.myDoc).last = last
        }
    }
    /*
     * Record in the parent context the number of entities replacement
     * done when parsing that reference.
     */
    if !oldctxt.is_null() {
        safe_oldctxt.nbentities = safe_oldctxt.nbentities.wrapping_add(safe_ctxt.nbentities)
    }
    /*
     * Also record the last error if any
     */
    if safe_ctxt.lastError.code != XML_ERR_OK as i32 {
        unsafe {
            xmlCopyError(&mut safe_ctxt.lastError, &mut safe_oldctxt.lastError);
        }
    }
    safe_ctxt.sax = oldsax;
    safe_ctxt.dict = 0 as xmlDictPtr;
    safe_ctxt.attsDefault = 0 as xmlHashTablePtr;
    safe_ctxt.attsSpecial = 0 as xmlHashTablePtr;
    xmlFreeParserCtxt_safe(ctxt);
    if !newDoc.is_null() {
        xmlFreeDoc_safe(newDoc);
    }
    return ret;
}
/* *
* xmlParseInNodeContext:
* @node:  the context node
* @data:  the input string
* @datalen:  the input string length in bytes
* @options:  a combination of xmlParserOption
* @lst:  the return value for the set of parsed nodes
*
* Parse a well-balanced chunk of an XML document
* within the context (DTD, namespaces, etc ...) of the given node.
*
* The allowed sequence for the data is a Well Balanced Chunk defined by
* the content production in the XML grammar:
*
* [43] content ::= (element | CharData | Reference | CDSect | PI | Comment)*
*
* Returns XML_ERR_OK if the chunk is well balanced, and the parser
* error code otherwise
*/

pub unsafe fn xmlParseInNodeContext(
    mut node: xmlNodePtr,
    mut data: *const i8,
    mut datalen: i32,
    mut options: i32,
    mut lst: *mut xmlNodePtr,
) -> xmlParserErrors {
    match () {
        #[cfg(HAVE_parser_SAX2)]
        _ => {
            let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
            let mut doc: xmlDocPtr = 0 as xmlDocPtr;
            let mut fake: xmlNodePtr = 0 as *mut xmlNode;
            let mut cur: xmlNodePtr = 0 as *mut xmlNode;
            let mut nsnr: i32 = 0 as i32;
            let mut ret: xmlParserErrors = XML_ERR_OK;
            /*
             * check all input parameters, grab the document
             */
            if lst.is_null() || node.is_null() || data.is_null() || datalen < 0 as i32 {
                return XML_ERR_INTERNAL_ERROR;
            }
            let mut safe_node = unsafe { &mut *node };

            match safe_node.type_0 as u32 {
                1 | 2 | 3 | 4 | 5 | 7 | 8 | 9 | 13 => {}
                _ => return XML_ERR_INTERNAL_ERROR,
            }
            while !node.is_null()
                && safe_node.type_0 as u32
                    != XML_ELEMENT_NODE as i32 as u32
                && safe_node.type_0 as u32
                    != XML_DOCUMENT_NODE as i32 as u32
                && safe_node.type_0 as u32
                    != XML_HTML_DOCUMENT_NODE as i32 as u32
            {
                node = safe_node.parent
            }
            if node.is_null() {
                return XML_ERR_INTERNAL_ERROR;
            }
            if safe_node.type_0 as u32 == XML_ELEMENT_NODE as i32 as u32 {
                doc = safe_node.doc
            } else {
                doc = node as xmlDocPtr
            }
            if doc.is_null() {
                return XML_ERR_INTERNAL_ERROR;
            }
            /*
             * allocate a context and set-up everything not related to the
             * node position in the tree
             */
            let mut safe_doc = unsafe { &mut *doc };

            if safe_doc.type_0 as u32 == XML_DOCUMENT_NODE as i32 as u32 {
                ctxt = xmlCreateMemoryParserCtxt_safe(data as *mut i8, datalen)
            } else if safe_doc.type_0 as u32
                == XML_HTML_DOCUMENT_NODE as i32 as u32
            {
                match () {
                    #[cfg(HAVE_parser_LIBXML_HTML_ENABLED)]
                    _ => {
                        unsafe {
                            ctxt = htmlCreateMemoryParserCtxt(data as *mut i8, datalen);
                        }
                        /*
                         * When parsing in context, it makes no sense to add implied
                         * elements like html/body/etc...
                         */
                        options |= HTML_PARSE_NOIMPLIED as i32;
                    }
                    #[cfg(not(HAVE_parser_LIBXML_HTML_ENABLED))]
                    _ => {}
                };
            } else {
                return XML_ERR_INTERNAL_ERROR;
            }
            if ctxt.is_null() {
                return XML_ERR_NO_MEMORY;
            }
            /*
             * Use input doc's dict if present, else assure XML_PARSE_NODICT is set.
             * We need a dictionary for xmlDetectSAX2, so if there's no doc dict
             * we must wait until the last moment to free the original one.
             */
            let mut safe_ctxt = unsafe { &mut *ctxt };

            if !safe_doc.dict.is_null() {
                if !safe_ctxt.dict.is_null() {
                    unsafe {
                        xmlDictFree(safe_ctxt.dict);
                    }
                }
                safe_ctxt.dict = safe_doc.dict
            } else {
                options |= XML_PARSE_NODICT as i32
            }
            if !safe_doc.encoding.is_null() {
                let mut hdlr: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
                if !safe_ctxt.encoding.is_null() {
                    xmlFree_safe(safe_ctxt.encoding as *mut xmlChar as *mut ());
                }
                unsafe {
                    safe_ctxt.encoding = xmlStrdup(safe_doc.encoding);
                    hdlr = xmlFindCharEncodingHandler(safe_doc.encoding as *const i8);
                    if !hdlr.is_null() {
                        xmlSwitchToEncoding(ctxt, hdlr);
                    } else {
                        return XML_ERR_UNSUPPORTED_ENCODING;
                    }
                }
            }
            xmlCtxtUseOptionsInternal(ctxt, options, 0 as *const i8);
            unsafe {
                xmlDetectSAX2(ctxt);
            }
            safe_ctxt.myDoc = doc;
            /* parsing in context, i.e. as within existing content */
            safe_ctxt.input_id = 2 as i32;
            safe_ctxt.instate = XML_PARSER_CONTENT;
            unsafe {
                fake = xmlNewComment(0 as *const xmlChar);
            }
            if fake.is_null() {
                xmlFreeParserCtxt_safe(ctxt);
                return XML_ERR_NO_MEMORY;
            }
            unsafe {
                xmlAddChild(node, fake);
            }
            if safe_node.type_0 as u32 == XML_ELEMENT_NODE as i32 as u32 {
                unsafe {
                    nodePush(ctxt, node);
                }
                /*
                 * initialize the SAX2 namespaces stack
                 */
                cur = node;
                let mut safe_cur = unsafe { &mut *cur };
                while !cur.is_null()
                    && safe_cur.type_0 as u32
                        == XML_ELEMENT_NODE as i32 as u32
                {
                    let mut ns: xmlNsPtr = safe_cur.nsDef;
                    let mut iprefix: *const xmlChar = 0 as *const xmlChar;
                    let mut ihref: *const xmlChar = 0 as *const xmlChar;
                    while !ns.is_null() {
                        unsafe {
                            if !safe_ctxt.dict.is_null() {
                                iprefix = xmlDictLookup(
                                    safe_ctxt.dict,
                                    (*ns).prefix,
                                    -(1 as i32),
                                );
                                ihref =
                                    xmlDictLookup(safe_ctxt.dict, (*ns).href, -(1 as i32))
                            } else {
                                iprefix = (*ns).prefix;
                                ihref = (*ns).href
                            }
                        }
                        unsafe {
                            if xmlGetNamespace(ctxt, iprefix).is_null() {
                                nsPush(ctxt, iprefix, ihref);
                                nsnr += 1
                            }
                            ns = (*ns).next
                        }
                    }
                    cur = safe_cur.parent
                }
            }
            if safe_ctxt.validate != 0 || safe_ctxt.replaceEntities != 0 as i32 {
                /*
                 * ID/IDREF registration will be done in xmlValidateElement below
                 */
                safe_ctxt.loadsubset |= 8 as i32
            }

            match () {
                #[cfg(HAVE_parser_LIBXML_HTML_ENABLED)]
                _ => unsafe {
                    if safe_doc.type_0 as u32
                        == XML_HTML_DOCUMENT_NODE as i32 as u32
                    {
                        __htmlParseContent(ctxt as *mut ());
                    } else {
                        xmlParseContent(ctxt);
                    }
                },
                #[cfg(not(HAVE_parser_LIBXML_HTML_ENABLED))]
                _ => {
                    xmlParseContent(ctxt);
                }
            };
            unsafe {
                nsPop(ctxt, nsnr);
                if *(*safe_ctxt.input).cur as i32 == '<' as i32
                    && *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32
                        == '/' as i32
                {
                    xmlFatalErr(ctxt, XML_ERR_NOT_WELL_BALANCED, 0 as *const i8);
                } else if *(*safe_ctxt.input).cur as i32 != 0 as i32 {
                    xmlFatalErr(ctxt, XML_ERR_EXTRA_CONTENT, 0 as *const i8);
                }
                if !safe_ctxt.node.is_null() && safe_ctxt.node != node {
                    xmlFatalErr(ctxt, XML_ERR_NOT_WELL_BALANCED, 0 as *const i8);
                    safe_ctxt.wellFormed = 0 as i32
                }
            }
            if safe_ctxt.wellFormed == 0 {
                if safe_ctxt.errNo == 0 as i32 {
                    ret = XML_ERR_INTERNAL_ERROR
                } else {
                    ret = safe_ctxt.errNo as xmlParserErrors
                }
            } else {
                ret = XML_ERR_OK
            }
            /*
             * Return the newly created nodeset after unlinking it from
             * the pseudo sibling.
             */
            let mut safe_fake = unsafe { &mut *fake };

            cur = safe_fake.next;
            safe_fake.next = 0 as *mut _xmlNode;
            safe_node.last = fake;
            let mut safe_cur = unsafe { &mut *cur };
            if !cur.is_null() {
                safe_cur.prev = 0 as *mut _xmlNode
            }
            unsafe {
                *lst = cur;
            }
            while !cur.is_null() {
                safe_cur.parent = 0 as *mut _xmlNode;
                cur = safe_cur.next
            }
            unsafe {
                xmlUnlinkNode(fake);
                xmlFreeNode(fake);
                if ret as u32 != XML_ERR_OK as i32 as u32 {
                    xmlFreeNodeList(*lst);

                    *lst = 0 as xmlNodePtr
                }
            }
            if !safe_doc.dict.is_null() {
                safe_ctxt.dict = 0 as xmlDictPtr
            }
            xmlFreeParserCtxt_safe(ctxt);
            return ret;
        }
        #[cfg(not(HAVE_parser_SAX2))]
        _ => {
            return XML_ERR_INTERNAL_ERROR;
        }
    };
    /* !SAX2 */
}
/* *
* xmlParseBalancedChunkMemoryRecover:
* @doc:  the document the chunk pertains to (must not be NULL)
* @sax:  the SAX handler block (possibly NULL)
* @user_data:  The user data returned on SAX callbacks (possibly NULL)
* @depth:  Used for loop detection, use 0
* @string:  the input string in UTF8 or ISO-Latin (zero terminated)
* @lst:  the return value for the set of parsed nodes
* @recover: return nodes even if the data is broken (use 0)
*
*
* Parse a well-balanced chunk of an XML document
* called by the parser
* The allowed sequence for the Well Balanced Chunk is the one defined by
* the content production in the XML grammar:
*
* [43] content ::= (element | CharData | Reference | CDSect | PI | Comment)*
*
* Returns 0 if the chunk is well balanced, -1 in case of args problem and
*    the parser error code otherwise
*
* In case recover is set to 1, the nodelist will not be empty even if
* the parsed chunk is not well balanced, assuming the parsing succeeded to
* some extent.
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlParseBalancedChunkMemoryRecover(
    mut doc: xmlDocPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut depth: i32,
    mut string: *const xmlChar,
    mut lst: *mut xmlNodePtr,
    mut recover: i32,
) -> i32 {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut newDoc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut oldsax: xmlSAXHandlerPtr = 0 as xmlSAXHandlerPtr;
    let mut content: xmlNodePtr = 0 as *mut xmlNode;
    let mut newRoot: xmlNodePtr = 0 as *mut xmlNode;
    let mut size: i32 = 0;
    let mut ret: i32 = 0 as i32;
    if depth > 40 as i32 {
        return XML_ERR_ENTITY_LOOP as i32;
    }
    unsafe {
        if !lst.is_null() {
            *lst = 0 as xmlNodePtr
        }
    }
    if string.is_null() {
        return -(1 as i32);
    }
    size = xmlStrlen_safe(string);
    ctxt = xmlCreateMemoryParserCtxt_safe(string as *mut i8, size);
    if ctxt.is_null() {
        return -(1 as i32);
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    safe_ctxt.userData = ctxt as *mut ();
    if !sax.is_null() {
        oldsax = safe_ctxt.sax;
        safe_ctxt.sax = sax;
        if !user_data.is_null() {
            safe_ctxt.userData = user_data
        }
    }
    unsafe {
        newDoc = xmlNewDoc(b"1.0\x00" as *const u8 as *const i8 as *mut xmlChar);
    }
    unsafe {
        if newDoc.is_null() {
            xmlFreeParserCtxt(ctxt);
            return -(1 as i32);
        }
    }
    let mut safe_newDoc = unsafe { &mut *newDoc };
    let mut safe_doc = unsafe { &mut *doc };

    safe_newDoc.properties = XML_DOC_INTERNAL as i32;
    if !doc.is_null() && !safe_doc.dict.is_null() {
        unsafe {
            xmlDictFree(safe_ctxt.dict);
            safe_ctxt.dict = safe_doc.dict;
            xmlDictReference(safe_ctxt.dict);
            safe_ctxt.str_xml = xmlDictLookup(
                safe_ctxt.dict,
                b"xml\x00" as *const u8 as *const i8 as *mut xmlChar,
                3 as i32,
            );
            safe_ctxt.str_xmlns = xmlDictLookup(
                safe_ctxt.dict,
                b"xmlns\x00" as *const u8 as *const i8 as *mut xmlChar,
                5 as i32,
            );
            safe_ctxt.str_xml_ns = xmlDictLookup(
                safe_ctxt.dict,
                b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as *const i8
                    as *const xmlChar,
                36 as i32,
            );
        }
        safe_ctxt.dictNames = 1 as i32
    } else {
        xmlCtxtUseOptionsInternal(
            ctxt,
            XML_PARSE_NODICT as i32,
            0 as *const i8,
        );
    }
    /* doc == NULL is only supported for historic reasons */
    if !doc.is_null() {
        safe_newDoc.intSubset = safe_doc.intSubset;
        safe_newDoc.extSubset = safe_doc.extSubset
    }
    unsafe {
        newRoot = xmlNewDocNode(
            newDoc,
            0 as xmlNsPtr,
            b"pseudoroot\x00" as *const u8 as *const i8 as *mut xmlChar,
            0 as *const xmlChar,
        );
    }

    if newRoot.is_null() {
        if !sax.is_null() {
            safe_ctxt.sax = oldsax
        }
        xmlFreeParserCtxt_safe(ctxt);
        safe_newDoc.intSubset = 0 as *mut _xmlDtd;
        safe_newDoc.extSubset = 0 as *mut _xmlDtd;
        xmlFreeDoc_safe(newDoc);
        return -(1 as i32);
    }
    unsafe {
        xmlAddChild(newDoc as xmlNodePtr, newRoot);
        nodePush(ctxt, newRoot);
    }

    /* doc == NULL is only supported for historic reasons */
    if doc.is_null() {
        safe_ctxt.myDoc = newDoc
    } else {
        safe_ctxt.myDoc = newDoc;
        unsafe {
            (*safe_newDoc.children).doc = doc;
            /* Ensure that doc has XML spec namespace */
            xmlSearchNsByHref(
                doc,
                doc as xmlNodePtr,
                b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as *const i8
                    as *const xmlChar,
            );
        }
        safe_newDoc.oldNs = safe_doc.oldNs
    }
    safe_ctxt.instate = XML_PARSER_CONTENT;
    safe_ctxt.input_id = 2 as i32;
    safe_ctxt.depth = depth;
    /*
     * Doing validity checking on chunk doesn't make sense
     */
    safe_ctxt.validate = 0 as i32;
    safe_ctxt.loadsubset = 0 as i32;
    unsafe {
        xmlDetectSAX2(ctxt);
        let mut safe_doc = unsafe { &mut *doc };
        if !doc.is_null() {
            content = safe_doc.children;
            safe_doc.children = 0 as *mut _xmlNode;
            xmlParseContent(ctxt);
            safe_doc.children = content
        } else {
            xmlParseContent(ctxt);
        }
        if *(*safe_ctxt.input).cur as i32 == '<' as i32
            && *(*safe_ctxt.input).cur.offset(1 as i32 as isize) as i32
                == '/' as i32
        {
            xmlFatalErr(ctxt, XML_ERR_NOT_WELL_BALANCED, 0 as *const i8);
        } else if *(*safe_ctxt.input).cur as i32 != 0 as i32 {
            xmlFatalErr(ctxt, XML_ERR_EXTRA_CONTENT, 0 as *const i8);
        }
        if safe_ctxt.node != safe_newDoc.children {
            xmlFatalErr(ctxt, XML_ERR_NOT_WELL_BALANCED, 0 as *const i8);
        }
    }
    if safe_ctxt.wellFormed == 0 {
        if safe_ctxt.errNo == 0 as i32 {
            ret = 1 as i32
        } else {
            ret = safe_ctxt.errNo
        }
    } else {
        ret = 0 as i32
    }
    if !lst.is_null() && (ret == 0 as i32 || recover == 1 as i32) {
        let mut cur: xmlNodePtr = 0 as *mut xmlNode;
        /*
         * Return the newly created nodeset after unlinking it from
         * they pseudo parent.
         */
        unsafe {
            cur = (*safe_newDoc.children).children;
            *lst = cur;
        }

        while !cur.is_null() {
            unsafe {
                xmlSetTreeDoc(cur, doc);
            }
            let mut safe_cur = unsafe { &mut *cur };
            safe_cur.parent = 0 as *mut _xmlNode;
            cur = safe_cur.next
        }
        unsafe { (*safe_newDoc.children).children = 0 as *mut _xmlNode }
    }
    if !sax.is_null() {
        safe_ctxt.sax = oldsax
    }
    xmlFreeParserCtxt_safe(ctxt);
    safe_newDoc.intSubset = 0 as *mut _xmlDtd;
    safe_newDoc.extSubset = 0 as *mut _xmlDtd;
    /* This leaks the namespace list if doc == NULL */
    safe_newDoc.oldNs = 0 as *mut _xmlNs;
    xmlFreeDoc_safe(newDoc);
    return ret;
}
/* *
* xmlSAXParseEntity:
* @sax:  the SAX handler block
* @filename:  the filename
*
* parse an XML external entity out of context and build a tree.
* It use the given SAX function block to handle the parsing callback.
* If sax is NULL, fallback to the default DOM tree building routines.
*
* [78] extParsedEnt ::= TextDecl? content
*
* This correspond to a "Well Balanced" chunk
*
* Returns the resulting document tree
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlSAXParseEntity(
    mut sax: xmlSAXHandlerPtr,
    mut filename: *const i8,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    ctxt = xmlCreateFileParserCtxt(filename);
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    if !sax.is_null() {
        if !safe_ctxt.sax.is_null() {
            xmlFree_safe(safe_ctxt.sax as *mut ());
        }
        safe_ctxt.sax = sax;
        safe_ctxt.userData = 0 as *mut ()
    }
    unsafe {
        xmlParseExtParsedEnt(ctxt);
    }
    if safe_ctxt.wellFormed != 0 {
        ret = safe_ctxt.myDoc
    } else {
        ret = 0 as xmlDocPtr;
        xmlFreeDoc_safe(safe_ctxt.myDoc);
        safe_ctxt.myDoc = 0 as xmlDocPtr
    }
    if !sax.is_null() {
        safe_ctxt.sax = 0 as *mut _xmlSAXHandler
    }
    xmlFreeParserCtxt_safe(ctxt);
    return ret;
}

/* *
* xmlParseEntity:
* @filename:  the filename
*
* parse an XML external entity out of context and build a tree.
*
* [78] extParsedEnt ::= TextDecl? content
*
* This correspond to a "Well Balanced" chunk
*
* Returns the resulting document tree
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlParseEntity(mut filename: *const i8) -> xmlDocPtr {
    unsafe {
        return xmlSAXParseEntity(0 as xmlSAXHandlerPtr, filename);
    }
}
/* LIBXML_SAX1_ENABLED */
/* *
* xmlCreateEntityParserCtxtInternal:
* @URL:  the entity URL
* @ID:  the entity PUBLIC ID
* @base:  a possible base for the target URI
* @pctx:  parser context used to set options on new context
*
* Create a parser context for an external entity
* Automatic support for ZLIB/Compress compressed document is provided
* by default if found at compile-time.
*
* Returns the new parser context or NULL
*/
unsafe fn xmlCreateEntityParserCtxtInternal(
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut base: *const xmlChar,
    mut pctx: xmlParserCtxtPtr,
) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut directory: *mut i8 = 0 as *mut i8;
    let mut uri: *mut xmlChar = 0 as *mut xmlChar;
    ctxt = xmlNewParserCtxt_safe();
    if ctxt.is_null() {
        return 0 as xmlParserCtxtPtr;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    let mut safe_pctx = unsafe { &mut *pctx };

    if !pctx.is_null() {
        safe_ctxt.options = safe_pctx.options;
        safe_ctxt._private = safe_pctx._private;
        /*
         * this is a subparser of pctx, so the input_id should be
         * incremented to distinguish from main entity
         */
        safe_ctxt.input_id = safe_pctx.input_id + 1 as i32
    }
    /* Don't read from stdin. */
    unsafe {
        if xmlStrcmp(
            URL,
            b"-\x00" as *const u8 as *const i8 as *mut xmlChar,
        ) == 0 as i32
        {
            URL = b"./-\x00" as *const u8 as *const i8 as *mut xmlChar
        }
    }
    unsafe {
        uri = xmlBuildURI(URL, base);
    }
    if uri.is_null() {
        inputStream =
            xmlLoadExternalEntity_safe(URL as *mut i8, ID as *mut i8, ctxt);
        if inputStream.is_null() {
            xmlFreeParserCtxt_safe(ctxt);
            return 0 as xmlParserCtxtPtr;
        }
        inputPush_safe(ctxt, inputStream);
        if safe_ctxt.directory.is_null() && directory.is_null() {
            directory = xmlParserGetDirectory_safe(URL as *mut i8)
        }
        if safe_ctxt.directory.is_null() && !directory.is_null() {
            safe_ctxt.directory = directory
        }
    } else {
        inputStream =
            xmlLoadExternalEntity_safe(uri as *mut i8, ID as *mut i8, ctxt);
        if inputStream.is_null() {
            xmlFree_safe(uri as *mut ());
            xmlFreeParserCtxt_safe(ctxt);
            return 0 as xmlParserCtxtPtr;
        }
        inputPush_safe(ctxt, inputStream);
        if safe_ctxt.directory.is_null() && directory.is_null() {
            directory = xmlParserGetDirectory_safe(uri as *mut i8)
        }
        if safe_ctxt.directory.is_null() && !directory.is_null() {
            safe_ctxt.directory = directory
        }
        xmlFree_safe(uri as *mut ());
    }
    return ctxt;
}

/* *
* xmlCreateEntityParserCtxt:
* @URL:  the entity URL
* @ID:  the entity PUBLIC ID
* @base:  a possible base for the target URI
*
* Create a parser context for an external entity
* Automatic support for ZLIB/Compress compressed document is provided
* by default if found at compile-time.
*
* Returns the new parser context or NULL
*/

pub unsafe fn xmlCreateEntityParserCtxt(
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut base: *const xmlChar,
) -> xmlParserCtxtPtr {
    unsafe {
        return xmlCreateEntityParserCtxtInternal(URL, ID, base, 0 as xmlParserCtxtPtr);
    }
}
/* ***********************************************************************
*									*
*		Front ends when parsing from a file			*
*									*
************************************************************************/
/* *
* xmlCreateURLParserCtxt:
* @filename:  the filename or URL
* @options:  a combination of xmlParserOption
*
* Create a parser context for a file or URL content.
* Automatic support for ZLIB/Compress compressed document is provided
* by default if found at compile-time and for file accesses
*
* Returns the new parser context or NULL
*/

pub unsafe fn xmlCreateURLParserCtxt(
    mut filename: *const i8,
    mut options: i32,
) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut directory: *mut i8 = 0 as *mut i8;
    ctxt = xmlNewParserCtxt_safe();
    unsafe {
        if ctxt.is_null() {
            xmlErrMemory(
                0 as xmlParserCtxtPtr,
                b"cannot allocate parser context\x00" as *const u8 as *const i8,
            );
            return 0 as xmlParserCtxtPtr;
        }
    }
    if options != 0 {
        xmlCtxtUseOptionsInternal(ctxt, options, 0 as *const i8);
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    safe_ctxt.linenumbers = 1 as i32;
    inputStream = xmlLoadExternalEntity_safe(filename, 0 as *const i8, ctxt);
    if inputStream.is_null() {
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as xmlParserCtxtPtr;
    }
    inputPush_safe(ctxt, inputStream);
    if safe_ctxt.directory.is_null() && directory.is_null() {
        directory = xmlParserGetDirectory_safe(filename)
    }
    if safe_ctxt.directory.is_null() && !directory.is_null() {
        safe_ctxt.directory = directory
    }
    return ctxt;
}
/* *
* xmlCreateFileParserCtxt:
* @filename:  the filename
*
* Create a parser context for a file content.
* Automatic support for ZLIB/Compress compressed document is provided
* by default if found at compile-time.
*
* Returns the new parser context or NULL
*/

pub unsafe fn xmlCreateFileParserCtxt(mut filename: *const i8) -> xmlParserCtxtPtr {
    return xmlCreateURLParserCtxt(filename, 0 as i32);
}
/* *
* xmlSAXParseFileWithData:
* @sax:  the SAX handler block
* @filename:  the filename
* @recovery:  work in recovery mode, i.e. tries to read no Well Formed
*             documents
* @data:  the userdata
*
* parse an XML file and build a tree. Automatic support for ZLIB/Compress
* compressed document is provided by default if found at compile-time.
* It use the given SAX function block to handle the parsing callback.
* If sax is NULL, fallback to the default DOM tree building routines.
*
* User data (void *) is stored within the parser context in the
* context's _private member, so it is available nearly everywhere in libxml
*
* Returns the resulting document tree
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlSAXParseFileWithData(
    mut sax: xmlSAXHandlerPtr,
    mut filename: *const i8,
    mut recovery: i32,
    mut data: *mut (),
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    xmlInitParser_safe();
    unsafe {
        ctxt = xmlCreateFileParserCtxt(filename);
    }
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if !sax.is_null() {
        if !safe_ctxt.sax.is_null() {
            xmlFree_safe(safe_ctxt.sax as *mut ());
        }
        safe_ctxt.sax = sax
    }
    unsafe {
        xmlDetectSAX2(ctxt);
    }
    if !data.is_null() {
        safe_ctxt._private = data
    }
    if safe_ctxt.directory.is_null() {
        safe_ctxt.directory = xmlParserGetDirectory_safe(filename)
    }
    safe_ctxt.recovery = recovery;
    unsafe {
        xmlParseDocument(ctxt);
    }
    if safe_ctxt.wellFormed != 0 || recovery != 0 {
        ret = safe_ctxt.myDoc;
        unsafe {
            if !ret.is_null() && !(*safe_ctxt.input).buf.is_null() {
                if (*(*safe_ctxt.input).buf).compressed > 0 as i32 {
                    (*ret).compression = 9 as i32
                } else {
                    (*ret).compression = (*(*safe_ctxt.input).buf).compressed
                }
            }
        }
    } else {
        unsafe {
            ret = 0 as xmlDocPtr;
        }
        xmlFreeDoc_safe(safe_ctxt.myDoc);
        safe_ctxt.myDoc = 0 as xmlDocPtr
    }
    if !sax.is_null() {
        safe_ctxt.sax = 0 as *mut _xmlSAXHandler
    }
    xmlFreeParserCtxt_safe(ctxt);
    return ret;
}

/* *
* xmlSAXParseFile:
* @sax:  the SAX handler block
* @filename:  the filename
* @recovery:  work in recovery mode, i.e. tries to read no Well Formed
*             documents
*
* parse an XML file and build a tree. Automatic support for ZLIB/Compress
* compressed document is provided by default if found at compile-time.
* It use the given SAX function block to handle the parsing callback.
* If sax is NULL, fallback to the default DOM tree building routines.
*
* Returns the resulting document tree
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlSAXParseFile(
    mut sax: xmlSAXHandlerPtr,
    mut filename: *const i8,
    mut recovery: i32,
) -> xmlDocPtr {
    unsafe {
        return xmlSAXParseFileWithData(sax, filename, recovery, 0 as *mut ());
    };
}

/* *
* xmlRecoverDoc:
* @cur:  a pointer to an array of xmlChar
*
* parse an XML in-memory document and build a tree.
* In the case the document is not Well Formed, a attempt to build a
* tree is tried anyway
*
* Returns the resulting document tree or NULL in case of failure
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlRecoverDoc(mut cur: *const xmlChar) -> xmlDocPtr {
    return xmlSAXParseDoc(0 as xmlSAXHandlerPtr, cur, 1 as i32);
}
/* *
* xmlParseFile:
* @filename:  the filename
*
* parse an XML file and build a tree. Automatic support for ZLIB/Compress
* compressed document is provided by default if found at compile-time.
*
* Returns the resulting document tree if the file was wellformed,
* NULL otherwise.
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlParseFile(mut filename: *const i8) -> xmlDocPtr {
    unsafe {
        return xmlSAXParseFile(0 as xmlSAXHandlerPtr, filename, 0 as i32);
    }
}
/* *
* xmlRecoverFile:
* @filename:  the filename
*
* parse an XML file and build a tree. Automatic support for ZLIB/Compress
* compressed document is provided by default if found at compile-time.
* In the case the document is not Well Formed, it attempts to build
* a tree anyway
*
* Returns the resulting document tree or NULL in case of failure
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlRecoverFile(mut filename: *const i8) -> xmlDocPtr {
    return xmlSAXParseFile(0 as xmlSAXHandlerPtr, filename, 1 as i32);
}
/* *
* xmlSetupParserForBuffer:
* @ctxt:  an XML parser context
* @buffer:  a xmlChar * buffer
* @filename:  a file name
*
* Setup the parser context to parse a new buffer; Clears any prior
* contents from the parser context. The buffer parameter must not be
* NULL, but the filename parameter can be
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlSetupParserForBuffer(
    mut ctxt: xmlParserCtxtPtr,
    mut buffer: *const xmlChar,
    mut filename: *const i8,
) {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ctxt.is_null() || buffer.is_null() {
        return;
    }
    input = xmlNewInputStream_safe(ctxt);
    if input.is_null() {
        unsafe {
            xmlErrMemory(
                0 as xmlParserCtxtPtr,
                b"parsing new buffer: out of memory\n\x00" as *const u8 as *const i8,
            );
        }
        xmlClearParserCtxt(ctxt);
        return;
    }
    xmlClearParserCtxt(ctxt);
    let mut safe_input = unsafe { *input };
    if !filename.is_null() {
        safe_input.filename = xmlCanonicPath_safe(filename as *const xmlChar) as *mut i8
    }
    safe_input.base = buffer;
    safe_input.cur = buffer;
    unsafe {
        safe_input.end = &*buffer.offset((xmlStrlen
            as unsafe extern "C" fn(_: *const xmlChar) -> i32)(
            buffer
        ) as isize) as *const xmlChar;
    }
    inputPush_safe(ctxt, input);
}
/* *
* xmlSAXUserParseFile:
* @sax:  a SAX handler
* @user_data:  The user data returned on SAX callbacks
* @filename:  a file name
*
* parse an XML file and call the given SAX handler routines.
* Automatic support for ZLIB/Compress compressed document is provided
*
* Returns 0 in case of success or a error number otherwise
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlSAXUserParseFile(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut filename: *const i8,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    unsafe {
        ctxt = xmlCreateFileParserCtxt(filename);
    }
    if ctxt.is_null() {
        return -(1 as i32);
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.sax != __xmlDefaultSAXHandler_safe() as xmlSAXHandlerPtr {
        xmlFree_safe(safe_ctxt.sax as *mut ());
    }
    safe_ctxt.sax = sax;
    unsafe {
        xmlDetectSAX2(ctxt);
    }
    if !user_data.is_null() {
        safe_ctxt.userData = user_data
    }
    unsafe {
        xmlParseDocument(ctxt);
    }
    if safe_ctxt.wellFormed != 0 {
        ret = 0 as i32
    } else if safe_ctxt.errNo != 0 as i32 {
        ret = safe_ctxt.errNo
    } else {
        ret = -(1 as i32)
    }
    if !sax.is_null() {
        safe_ctxt.sax = 0 as *mut _xmlSAXHandler
    }
    if !safe_ctxt.myDoc.is_null() {
        xmlFreeDoc_safe(safe_ctxt.myDoc);
        safe_ctxt.myDoc = 0 as xmlDocPtr
    }
    xmlFreeParserCtxt_safe(ctxt);
    return ret;
}
/* LIBXML_SAX1_ENABLED */
/* ***********************************************************************
*									*
*		Front ends when parsing from memory			*
*									*
************************************************************************/
/* *
* xmlCreateMemoryParserCtxt:
* @buffer:  a pointer to a char array
* @size:  the size of the array
*
* Create a parser context for an XML in-memory document.
*
* Returns the new parser context or NULL
*/

pub unsafe fn xmlCreateMemoryParserCtxt_parser(
    mut buffer: *const i8,
    mut size: i32,
) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if buffer.is_null() {
        return 0 as xmlParserCtxtPtr;
    }
    if size <= 0 as i32 {
        return 0 as xmlParserCtxtPtr;
    }
    ctxt = xmlNewParserCtxt_safe();
    if ctxt.is_null() {
        return 0 as xmlParserCtxtPtr;
    }
    /* TODO: xmlParserInputBufferCreateStatic, requires some serious changes */
    buf = xmlParserInputBufferCreateMem_safe(buffer, size, XML_CHAR_ENCODING_NONE);
    if buf.is_null() {
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as xmlParserCtxtPtr;
    }
    input = xmlNewInputStream_safe(ctxt);
    if input.is_null() {
        xmlFreeParserInputBuffer_safe(buf);
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as xmlParserCtxtPtr;
    }
    unsafe {
        (*input).filename = 0 as *const i8;
        (*input).buf = buf;
        xmlBufResetInput((*(*input).buf).buffer, input);
    }
    inputPush_safe(ctxt, input);
    return ctxt;
}

/* *
* xmlSAXParseMemoryWithData:
* @sax:  the SAX handler block
* @buffer:  an pointer to a char array
* @size:  the size of the array
* @recovery:  work in recovery mode, i.e. tries to read no Well Formed
*             documents
* @data:  the userdata
*
* parse an XML in-memory block and use the given SAX function block
* to handle the parsing callback. If sax is NULL, fallback to the default
* DOM tree building routines.
*
* User data (void *) is stored within the parser context in the
* context's _private member, so it is available nearly everywhere in libxml
*
* Returns the resulting document tree
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlSAXParseMemoryWithData(
    mut sax: xmlSAXHandlerPtr,
    mut buffer: *const i8,
    mut size: i32,
    mut recovery: i32,
    mut data: *mut (),
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    xmlInitParser_safe();
    ctxt = xmlCreateMemoryParserCtxt_safe(buffer, size);
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if !sax.is_null() {
        if !safe_ctxt.sax.is_null() {
            xmlFree_safe(safe_ctxt.sax as *mut ());
        }
        safe_ctxt.sax = sax
    }
    unsafe {
        xmlDetectSAX2(ctxt);
    }
    if !data.is_null() {
        safe_ctxt._private = data
    }
    safe_ctxt.recovery = recovery;
    unsafe {
        xmlParseDocument(ctxt);
    }
    if safe_ctxt.wellFormed != 0 || recovery != 0 {
        ret = safe_ctxt.myDoc
    } else {
        ret = 0 as xmlDocPtr;
        xmlFreeDoc_safe(safe_ctxt.myDoc);
        safe_ctxt.myDoc = 0 as xmlDocPtr
    }
    if !sax.is_null() {
        safe_ctxt.sax = 0 as *mut _xmlSAXHandler
    }
    xmlFreeParserCtxt_safe(ctxt);
    return ret;
}

/* *
* xmlSAXParseMemory:
* @sax:  the SAX handler block
* @buffer:  an pointer to a char array
* @size:  the size of the array
* @recovery:  work in recovery mode, i.e. tries to read not Well Formed
*             documents
*
* parse an XML in-memory block and use the given SAX function block
* to handle the parsing callback. If sax is NULL, fallback to the default
* DOM tree building routines.
*
* Returns the resulting document tree
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlSAXParseMemory(
    mut sax: xmlSAXHandlerPtr,
    mut buffer: *const i8,
    mut size: i32,
    mut recovery: i32,
) -> xmlDocPtr {
    unsafe {
        return xmlSAXParseMemoryWithData(sax, buffer, size, recovery, 0 as *mut ());
    }
}
/* *
* xmlParseMemory:
* @buffer:  an pointer to a char array
* @size:  the size of the array
*
* parse an XML in-memory block and build a tree.
*
* Returns the resulting document tree
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlParseMemory(mut buffer: *const i8, mut size: i32) -> xmlDocPtr {
    return xmlSAXParseMemory(0 as xmlSAXHandlerPtr, buffer, size, 0 as i32);
}
/* *
* xmlRecoverMemory:
* @buffer:  an pointer to a char array
* @size:  the size of the array
*
* parse an XML in-memory block and build a tree.
* In the case the document is not Well Formed, an attempt to
* build a tree is tried anyway
*
* Returns the resulting document tree or NULL in case of error
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlRecoverMemory(
    mut buffer: *const i8,
    mut size: i32,
) -> xmlDocPtr {
    return xmlSAXParseMemory(0 as xmlSAXHandlerPtr, buffer, size, 1 as i32);
}
/* *
* xmlSAXUserParseMemory:
* @sax:  a SAX handler
* @user_data:  The user data returned on SAX callbacks
* @buffer:  an in-memory XML document input
* @size:  the length of the XML document in bytes
*
* A better SAX parsing routine.
* parse an XML in-memory buffer and call the given SAX handler routines.
*
* Returns 0 in case of success or a error number otherwise
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlSAXUserParseMemory(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut (),
    mut buffer: *const i8,
    mut size: i32,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    xmlInitParser_safe();
    ctxt = xmlCreateMemoryParserCtxt_safe(buffer, size);
    if ctxt.is_null() {
        return -(1 as i32);
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.sax != __xmlDefaultSAXHandler_safe() as xmlSAXHandlerPtr {
        xmlFree_safe(safe_ctxt.sax as *mut ());
    }
    safe_ctxt.sax = sax;
    unsafe {
        xmlDetectSAX2(ctxt);
    }
    if !user_data.is_null() {
        safe_ctxt.userData = user_data
    }
    unsafe {
        xmlParseDocument(ctxt);
    }
    if safe_ctxt.wellFormed != 0 {
        ret = 0 as i32
    } else if safe_ctxt.errNo != 0 as i32 {
        ret = safe_ctxt.errNo
    } else {
        ret = -(1 as i32)
    }
    if !sax.is_null() {
        safe_ctxt.sax = 0 as *mut _xmlSAXHandler
    }
    if !safe_ctxt.myDoc.is_null() {
        xmlFreeDoc_safe(safe_ctxt.myDoc);
        safe_ctxt.myDoc = 0 as xmlDocPtr
    }
    xmlFreeParserCtxt_safe(ctxt);
    return ret;
}
/* LIBXML_SAX1_ENABLED */
/* *
* xmlCreateDocParserCtxt:
* @cur:  a pointer to an array of xmlChar
*
* Creates a parser context for an XML in-memory document.
*
* Returns the new parser context or NULL
*/

pub unsafe fn xmlCreateDocParserCtxt(mut cur: *const xmlChar) -> xmlParserCtxtPtr {
    let mut len: i32 = 0;
    if cur.is_null() {
        return 0 as xmlParserCtxtPtr;
    }
    len = xmlStrlen_safe(cur);
    unsafe {
        return xmlCreateMemoryParserCtxt(cur as *const i8, len);
    }
}
/* *
* xmlSAXParseDoc:
* @sax:  the SAX handler block
* @cur:  a pointer to an array of xmlChar
* @recovery:  work in recovery mode, i.e. tries to read no Well Formed
*             documents
*
* parse an XML in-memory document and build a tree.
* It use the given SAX function block to handle the parsing callback.
* If sax is NULL, fallback to the default DOM tree building routines.
*
* Returns the resulting document tree
*/

#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlSAXParseDoc(
    mut sax: xmlSAXHandlerPtr,
    mut cur: *const xmlChar,
    mut recovery: i32,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut oldsax: xmlSAXHandlerPtr = 0 as xmlSAXHandlerPtr;
    if cur.is_null() {
        return 0 as xmlDocPtr;
    }
    unsafe {
        ctxt = xmlCreateDocParserCtxt(cur);
    }
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    let mut safe_ctxt = unsafe { &mut *ctxt };

    if !sax.is_null() {
        oldsax = safe_ctxt.sax;
        safe_ctxt.sax = sax;
        safe_ctxt.userData = 0 as *mut ()
    }
    unsafe {
        xmlDetectSAX2(ctxt);
        xmlParseDocument(ctxt);
    }
    if safe_ctxt.wellFormed != 0 || recovery != 0 {
        ret = safe_ctxt.myDoc
    } else {
        ret = 0 as xmlDocPtr;
        xmlFreeDoc_safe(safe_ctxt.myDoc);
        safe_ctxt.myDoc = 0 as xmlDocPtr
    }
    if !sax.is_null() {
        safe_ctxt.sax = oldsax
    }
    xmlFreeParserCtxt_safe(ctxt);
    return ret;
}

/* *
* xmlParseDoc:
* @cur:  a pointer to an array of xmlChar
*
* parse an XML in-memory document and build a tree.
*
* Returns the resulting document tree
*/
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
pub unsafe fn xmlParseDoc(mut cur: *const xmlChar) -> xmlDocPtr {
    unsafe {
        return xmlSAXParseDoc(0 as xmlSAXHandlerPtr, cur, 0 as i32);
    }
}
/* LIBXML_SAX1_ENABLED */
/* ***********************************************************************
*									*
*	Specific function to keep track of entities references		*
*	and used by the XSLT debugger					*
*									*
************************************************************************/
#[cfg(HAVE_parser_LIBXML_LEGACY_ENABLED)]
static mut xmlEntityRefFunc: xmlEntityReferenceFunc = None;
/* *
* xmlAddEntityReference:
* @ent : A valid entity
* @firstNode : A valid first node for children of entity
* @lastNode : A valid last node of children entity
*
* Notify of a reference to an entity of type XML_EXTERNAL_GENERAL_PARSED_ENTITY
*/

#[cfg(HAVE_parser_LIBXML_LEGACY_ENABLED)]
unsafe fn xmlAddEntityReference(
    mut ent: xmlEntityPtr,
    mut firstNode: xmlNodePtr,
    mut lastNode: xmlNodePtr,
) {
    unsafe {
        if xmlEntityRefFunc.is_some() {
            Some(xmlEntityRefFunc.expect("non-null function pointer"))
                .expect("non-null function pointer")(ent, firstNode, lastNode);
        };
    }
}
/* *
* xmlSetEntityReferenceFunc:
* @func: A valid function
*
* Set the function to call call back when a xml reference has been made
*/
#[cfg(HAVE_parser_LIBXML_LEGACY_ENABLED)]
pub unsafe fn xmlSetEntityReferenceFunc(mut func: xmlEntityReferenceFunc) {
    unsafe {
        xmlEntityRefFunc = func;
    }
}
static mut xmlParserInitialized: i32 = 0 as i32;
/* *
* xmlInitParser:
*
* Initialization function for the XML parser.
* This is not reentrant. Call once before processing in case of
* use in multithreaded programs.
*/

pub unsafe fn xmlInitParser_parser() {
    unsafe {
        if xmlParserInitialized != 0 as i32 {
            return;
        }
    }

    // if cfg!(HAVE_parser_WIN32) && cfg!(not(HAVE_parser_LIBXML_STATIC)) || cfg!(HAVE_parser_LIBXML_STATIC_FOR_DLL) {
    //     //#if defined(_WIN32) && (!defined(LIBXML_STATIC) || defined(LIBXML_STATIC_FOR_DLL))
    //     atexit(Some(xmlCleanupParser as
    //         unsafe extern "C" fn() -> ()));
    //     //#endif
    // }

    match () {
        #[cfg(HAVE_parser_WIN32)]
        _ => {
            match () {
                #[cfg(HAVE_parser_LIBXML_STATIC)]
                _ => {
                    match () {
                        #[cfg(HAVE_parser_LIBXML_STATIC_FOR_DLL)]
                        _ => {
                            atexit(Some(xmlCleanupParser as unsafe extern "C" fn() -> ()));
                        }
                        #[cfg(not(HAVE_parser_LIBXML_STATIC_FOR_DLL))]
                        _ => {}
                    };
                }
                #[cfg(not(HAVE_parser_LIBXML_STATIC))]
                _ => {
                    atexit(Some(xmlCleanupParser as unsafe extern "C" fn() -> ()));
                }
            };
        }
        #[cfg(not(HAVE_parser_WIN32))]
        _ => {
            match () {
                #[cfg(HAVE_parser_LIBXML_STATIC_FOR_DLL)]
                _ => {
                    atexit(Some(xmlCleanupParser as unsafe extern "C" fn() -> ()));
                }
                #[cfg(not(HAVE_parser_LIBXML_STATIC_FOR_DLL))]
                _ => {}
            };
        }
    };

    match () {
        #[cfg(HAVE_parser_LIBXML_THREAD_ENABLED)]
        _ => {
            __xmlGlobalInitMutexLock_safe();
            if unsafe { xmlParserInitialized == 0 as i32 } {
                xmlInitThreads_safe();
                xmlInitGlobals_safe();
                unsafe {
                    if *__xmlGenericError()
                        == Some(
                            xmlGenericErrorDefaultFunc
                                as unsafe extern "C" fn(
                                    _: *mut (),
                                    _: *const i8,
                                    _: ...
                                ) -> (),
                        )
                        || (*__xmlGenericError()).is_none()
                    {
                        initGenericErrorDefaultFunc(0 as *mut xmlGenericErrorFunc);
                    }
                }
                xmlInitMemory_safe();
                xmlInitializeDict_safe();
                xmlInitCharEncodingHandlers_safe();
                xmlDefaultSAXHandlerInit_safe();
                xmlRegisterDefaultInputCallbacks_safe();
                match () {
                    #[cfg(HAVE_parser_LIBXML_OUTPUT_ENABLED)]
                    _ => {
                        xmlRegisterDefaultOutputCallbacks_safe();
                    }
                    #[cfg(not(HAVE_parser_LIBXML_OUTPUT_ENABLED))]
                    _ => {}
                };
                /* LIBXML_OUTPUT_ENABLED */
                match () {
                    #[cfg(HAVE_parser_LIBXML_HTML_ENABLED)]
                    _ => {
                        htmlInitAutoClose_safe();
                        htmlDefaultSAXHandlerInit_safe();
                    }
                    #[cfg(not(HAVE_parser_LIBXML_HTML_ENABLED))]
                    _ => {}
                };
                match () {
                    #[cfg(HAVE_parser_LIBXML_XPATH_ENABLED)]
                    _ => {
                        xmlXPathInit_safe();
                    }
                    #[cfg(not(HAVE_parser_LIBXML_XPATH_ENABLED))]
                    _ => {}
                };
                unsafe { xmlParserInitialized = 1 as i32 }
            }
            __xmlGlobalInitMutexUnlock_safe();
        }

        #[cfg(not(HAVE_parser_LIBXML_THREAD_ENABLED))]
        _ => {
            unsafe {
                xmlInitThreads();
                xmlInitGlobals();
                if *__xmlGenericError()
                    == Some(
                        xmlGenericErrorDefaultFunc
                            as unsafe extern "C" fn(
                                _: *mut (),
                                _: *const i8,
                                _: ...
                            ) -> (),
                    )
                    || (*__xmlGenericError()).is_none()
                {
                    initGenericErrorDefaultFunc(0 as *mut xmlGenericErrorFunc);
                }
                xmlInitMemory();
                xmlInitializeDict();
                xmlInitCharEncodingHandlers();
                xmlDefaultSAXHandlerInit();
                xmlRegisterDefaultInputCallbacks();
                match () {
                    #[cfg(HAVE_parser_LIBXML_OUTPUT_ENABLED)]
                    _ => {
                        xmlRegisterDefaultOutputCallbacks();
                    }
                    #[cfg(not(HAVE_parser_LIBXML_OUTPUT_ENABLED))]
                    _ => {}
                };
                /* LIBXML_OUTPUT_ENABLED */
                match () {
                    #[cfg(HAVE_parser_LIBXML_HTML_ENABLED)]
                    _ => {
                        htmlInitAutoClose();
                        htmlDefaultSAXHandlerInit();
                    }
                    #[cfg(not(HAVE_parser_LIBXML_HTML_ENABLED))]
                    _ => {}
                };
                match () {
                    #[cfg(HAVE_parser_LIBXML_XPATH_ENABLED)]
                    _ => {
                        xmlXPathInit();
                    }
                    #[cfg(not(HAVE_parser_LIBXML_XPATH_ENABLED))]
                    _ => {}
                };
                xmlParserInitialized = 1 as i32;
            }
        }
    };
}
/* *
* xmlCleanupParser:
*
* This function name is somewhat misleading. It does not clean up
* parser state, it cleans up memory allocated by the library itself.
* It is a cleanup function for the XML library. It tries to reclaim all
* related global memory allocated for the library processing.
* It doesn't deallocate any document related memory. One should
* call xmlCleanupParser() only when the process has finished using
* the library and all XML/HTML documents built with it.
* See also xmlInitParser() which has the opposite function of preparing
* the library for operations.
*
* WARNING: if your application is multithreaded or has plugin support
*          calling this may crash the application if another thread or
*          a plugin is still using libxml2. It's sometimes very hard to
*          guess if libxml2 is in use in the application, some libraries
*          or plugins may use it without notice. In case of doubt abstain
*          from calling this function or do it just before calling exit()
*          to avoid leak reports from valgrind !
*/

pub unsafe fn xmlCleanupParser() {
    unsafe {
        if xmlParserInitialized == 0 {
            return;
        }
    } /* must be last if called not from the main thread */
    xmlCleanupCharEncodingHandlers_safe();
    match () {
        #[cfg(HAVE_parser_LIBXML_CATALOG_ENABLED)]
        _ => {
            xmlCatalogCleanup_safe();
        }
        #[cfg(not(HAVE_parser_LIBXML_CATALOG_ENABLED))]
        _ => {}
    };
    xmlDictCleanup_safe();
    xmlCleanupInputCallbacks_safe();
    match () {
        #[cfg(HAVE_parser_LIBXML_OUTPUT_ENABLED)]
        _ => {
            xmlCleanupOutputCallbacks_safe();
        }
        #[cfg(not(HAVE_parser_LIBXML_OUTPUT_ENABLED))]
        _ => {}
    };
    match () {
        #[cfg(HAVE_parser_LIBXML_SCHEMAS_ENABLED)]
        _ => {
            xmlSchemaCleanupTypes_safe();
            xmlRelaxNGCleanupTypes_safe();
        }
        #[cfg(not(HAVE_parser_LIBXML_SCHEMAS_ENABLED))]
        _ => {}
    };
    xmlResetLastError_safe();
    xmlCleanupGlobals_safe();
    xmlCleanupThreads_safe();
    xmlCleanupMemory_safe();
    unsafe {
        xmlParserInitialized = 0 as i32;
    }
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
* xmlCtxtReset:
* @ctxt: an XML parser context
*
* Reset a parser context
*/

pub unsafe fn xmlCtxtReset_parser(mut ctxt: xmlParserCtxtPtr) {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() {
        return;
    }
    dict = safe_ctxt.dict;
    loop {
        input = inputPop_safe(ctxt);
        if input.is_null() {
            break;
        }
        /* Non consuming */
        xmlFreeInputStream_safe(input);
    }
    safe_ctxt.inputNr = 0 as i32;
    safe_ctxt.input = 0 as xmlParserInputPtr;
    safe_ctxt.spaceNr = 0 as i32;
    unsafe {
        if !safe_ctxt.spaceTab.is_null() {
            *safe_ctxt.spaceTab.offset(0 as i32 as isize) = -(1 as i32);
            safe_ctxt.space =
                &mut *safe_ctxt.spaceTab.offset(0 as i32 as isize) as *mut i32
        } else {
            safe_ctxt.space = 0 as *mut i32
        }
    }
    safe_ctxt.nodeNr = 0 as i32;
    safe_ctxt.node = 0 as xmlNodePtr;
    safe_ctxt.nameNr = 0 as i32;
    safe_ctxt.name = 0 as *const xmlChar;
    if !safe_ctxt.version.is_null()
        && (dict.is_null() || xmlDictOwns_safe(dict, safe_ctxt.version) == 0 as i32)
    {
        xmlFree_safe(safe_ctxt.version as *mut i8 as *mut ());
    }
    safe_ctxt.version = 0 as *const xmlChar;
    if !safe_ctxt.encoding.is_null()
        && (dict.is_null() || xmlDictOwns_safe(dict, safe_ctxt.encoding) == 0 as i32)
    {
        xmlFree_safe(safe_ctxt.encoding as *mut i8 as *mut ());
    }
    safe_ctxt.encoding = 0 as *const xmlChar;
    if !safe_ctxt.directory.is_null()
        && (dict.is_null()
            || xmlDictOwns_safe(dict, safe_ctxt.directory as *const xmlChar) == 0 as i32)
    {
        xmlFree_safe(safe_ctxt.directory as *mut ());
    }
    safe_ctxt.directory = 0 as *mut i8;
    if !safe_ctxt.extSubURI.is_null()
        && (dict.is_null()
            || xmlDictOwns_safe(dict, safe_ctxt.extSubURI as *const xmlChar) == 0 as i32)
    {
        xmlFree_safe(safe_ctxt.extSubURI as *mut i8 as *mut ());
    }
    safe_ctxt.extSubURI = 0 as *mut xmlChar;
    if !safe_ctxt.extSubSystem.is_null()
        && (dict.is_null()
            || xmlDictOwns_safe(dict, safe_ctxt.extSubSystem as *const xmlChar) == 0 as i32)
    {
        xmlFree_safe(safe_ctxt.extSubSystem as *mut i8 as *mut ());
    }
    safe_ctxt.extSubSystem = 0 as *mut xmlChar;
    if !safe_ctxt.myDoc.is_null() {
        xmlFreeDoc_safe(safe_ctxt.myDoc);
    }
    safe_ctxt.myDoc = 0 as xmlDocPtr;
    safe_ctxt.standalone = -(1 as i32);
    safe_ctxt.hasExternalSubset = 0 as i32;
    safe_ctxt.hasPErefs = 0 as i32;
    safe_ctxt.html = 0 as i32;
    safe_ctxt.external = 0 as i32;
    safe_ctxt.instate = XML_PARSER_START;
    safe_ctxt.token = 0 as i32;
    safe_ctxt.wellFormed = 1 as i32;
    safe_ctxt.nsWellFormed = 1 as i32;
    safe_ctxt.disableSAX = 0 as i32;
    safe_ctxt.valid = 1 as i32;
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
    xmlInitNodeInfoSeq_safe(&mut safe_ctxt.node_seq);
    if !safe_ctxt.attsDefault.is_null() {
        xmlHashFree_safe(
            safe_ctxt.attsDefault,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
            ),
        );
        safe_ctxt.attsDefault = 0 as xmlHashTablePtr
    }
    if !safe_ctxt.attsSpecial.is_null() {
        xmlHashFree_safe(safe_ctxt.attsSpecial, None);
        safe_ctxt.attsSpecial = 0 as xmlHashTablePtr
    }

    match () {
        #[cfg(HAVE_parser_LIBXML_CATALOG_ENABLED)]
        _ => {
            if !safe_ctxt.catalogs.is_null() {
                xmlCatalogFreeLocal_safe(safe_ctxt.catalogs);
            }
        }
        #[cfg(not(HAVE_parser_LIBXML_CATALOG_ENABLED))]
        _ => {}
    };
    if safe_ctxt.lastError.code != XML_ERR_OK as i32 {
        xmlResetError_safe(&mut safe_ctxt.lastError);
    };
}
/* *
* xmlCtxtResetPush:
* @ctxt: an XML parser context
* @chunk:  a pointer to an array of chars
* @size:  number of chars in the array
* @filename:  an optional file name or URI
* @encoding:  the document encoding, or NULL
*
* Reset a push parser context
*
* Returns 0 in case of success and 1 in case of error
*/

pub unsafe fn xmlCtxtResetPush(
    mut ctxt: xmlParserCtxtPtr,
    mut chunk: *const i8,
    mut size: i32,
    mut filename: *const i8,
    mut encoding: *const i8,
) -> i32 {
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() {
        return 1 as i32;
    }
    if encoding.is_null() && !chunk.is_null() && size >= 4 as i32 {
        enc = xmlDetectCharEncoding_safe(chunk as *const xmlChar, size)
    }
    buf = xmlAllocParserInputBuffer_safe(enc);
    if buf.is_null() {
        return 1 as i32;
    }
    if ctxt.is_null() {
        xmlFreeParserInputBuffer_safe(buf);
        return 1 as i32;
    }
    xmlCtxtReset_safe(ctxt);
    if filename.is_null() {
        safe_ctxt.directory = 0 as *mut i8
    } else {
        safe_ctxt.directory = xmlParserGetDirectory_safe(filename)
    }
    inputStream = xmlNewInputStream_safe(ctxt);
    if inputStream.is_null() {
        xmlFreeParserInputBuffer_safe(buf);
        return 1 as i32;
    }
    let mut safe_inputStream = unsafe { &mut *inputStream };
    if filename.is_null() {
        safe_inputStream.filename = 0 as *const i8
    } else {
        safe_inputStream.filename =
            xmlCanonicPath_safe(filename as *const xmlChar) as *mut i8
    }
    safe_inputStream.buf = buf;
    let mut safe_buf = unsafe { &mut *buf };
    xmlBufResetInput_safe(safe_buf.buffer, inputStream);
    inputPush_safe(ctxt, inputStream);
    if size > 0 as i32
        && !chunk.is_null()
        && !safe_ctxt.input.is_null()
        && unsafe { !(*safe_ctxt.input).buf.is_null() }
    {
        unsafe {
            let mut base: size_t =
                xmlBufGetInputBase_safe((*(*safe_ctxt.input).buf).buffer, safe_ctxt.input);
            let mut cur: size_t = (*safe_ctxt.input).cur.offset_from((*safe_ctxt.input).base)
                as i64 as size_t;
            xmlParserInputBufferPush_safe((*safe_ctxt.input).buf, size, chunk);
            xmlBufSetInputBaseCur_safe(
                (*(*safe_ctxt.input).buf).buffer,
                safe_ctxt.input,
                base,
                cur,
            );
        }

        match () {
            #[cfg(HAVE_parser_DEBUG_PUSH)]
            _ => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"PP: pushed %d\n\x00" as *const u8 as *const i8,
                    size,
                );
            }
            #[cfg(not(HAVE_parser_DEBUG_PUSH))]
            _ => {}
        };
    }
    if !encoding.is_null() {
        let mut hdlr: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
        if !safe_ctxt.encoding.is_null() {
            unsafe {
                xmlFree.expect("non-null function pointer")(
                    safe_ctxt.encoding as *mut xmlChar as *mut (),
                );
            }
        }
        safe_ctxt.encoding = xmlStrdup_safe(encoding as *const xmlChar);
        hdlr = xmlFindCharEncodingHandler_safe(encoding);
        if !hdlr.is_null() {
            xmlSwitchToEncoding_safe(ctxt, hdlr);
        } else {
            unsafe {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNSUPPORTED_ENCODING,
                    b"Unsupported encoding %s\n\x00" as *const u8 as *const i8,
                    encoding as *mut xmlChar,
                );
            }
        }
    } else if enc as i32 != XML_CHAR_ENCODING_NONE as i32 {
        xmlSwitchEncoding_safe(ctxt, enc);
    }
    return 0 as i32;
}
/* *
* xmlCtxtUseOptionsInternal:
* @ctxt: an XML parser context
* @options:  a combination of xmlParserOption
* @encoding:  the user provided encoding to use
*
* Applies the options to the parser context
*
* Returns 0 in case of success, the set of unknown or unimplemented options
*         in case of error.
*/
unsafe fn xmlCtxtUseOptionsInternal(
    mut ctxt: xmlParserCtxtPtr,
    mut options: i32,
    mut encoding: *const i8,
) -> i32 {
    let mut safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() {
        return -(1 as i32);
    }
    if !encoding.is_null() {
        if !safe_ctxt.encoding.is_null() {
            xmlFree_safe(safe_ctxt.encoding as *mut xmlChar as *mut ());
        }
        safe_ctxt.encoding = xmlStrdup_safe(encoding as *const xmlChar)
    }
    if options & XML_PARSE_RECOVER as i32 != 0 {
        safe_ctxt.recovery = 1 as i32;
        options -= XML_PARSE_RECOVER as i32;
        safe_ctxt.options |= XML_PARSE_RECOVER as i32
    } else {
        safe_ctxt.recovery = 0 as i32
    }
    if options & XML_PARSE_DTDLOAD as i32 != 0 {
        safe_ctxt.loadsubset = 2 as i32;
        options -= XML_PARSE_DTDLOAD as i32;
        safe_ctxt.options |= XML_PARSE_DTDLOAD as i32
    } else {
        safe_ctxt.loadsubset = 0 as i32
    }
    if options & XML_PARSE_DTDATTR as i32 != 0 {
        safe_ctxt.loadsubset |= 4 as i32;
        options -= XML_PARSE_DTDATTR as i32;
        safe_ctxt.options |= XML_PARSE_DTDATTR as i32
    }
    if options & XML_PARSE_NOENT as i32 != 0 {
        safe_ctxt.replaceEntities = 1 as i32;
        /* ctxt->loadsubset |= XML_DETECT_IDS; */
        options -= XML_PARSE_NOENT as i32;
        safe_ctxt.options |= XML_PARSE_NOENT as i32
    } else {
        safe_ctxt.replaceEntities = 0 as i32
    }
    if options & XML_PARSE_PEDANTIC as i32 != 0 {
        safe_ctxt.pedantic = 1 as i32;
        options -= XML_PARSE_PEDANTIC as i32;
        safe_ctxt.options |= XML_PARSE_PEDANTIC as i32
    } else {
        safe_ctxt.pedantic = 0 as i32
    }
    if options & XML_PARSE_NOBLANKS as i32 != 0 {
        safe_ctxt.keepBlanks = 0 as i32;
        unsafe {
            (*safe_ctxt.sax).ignorableWhitespace = Some(
                xmlSAX2IgnorableWhitespace
                    as unsafe extern "C" fn(
                        _: *mut (),
                        _: *const xmlChar,
                        _: i32,
                    ) -> (),
            )
        };
        options -= XML_PARSE_NOBLANKS as i32;
        safe_ctxt.options |= XML_PARSE_NOBLANKS as i32
    } else {
        safe_ctxt.keepBlanks = 1 as i32
    }
    if options & XML_PARSE_DTDVALID as i32 != 0 {
        safe_ctxt.validate = 1 as i32;
        if options & XML_PARSE_NOWARNING as i32 != 0 {
            safe_ctxt.vctxt.warning = None
        }
        if options & XML_PARSE_NOERROR as i32 != 0 {
            safe_ctxt.vctxt.error = None
        }
        options -= XML_PARSE_DTDVALID as i32;
        safe_ctxt.options |= XML_PARSE_DTDVALID as i32
    } else {
        safe_ctxt.validate = 0 as i32
    }
    unsafe {
        if options & XML_PARSE_NOWARNING as i32 != 0 {
            (*safe_ctxt.sax).warning = None;
            options -= XML_PARSE_NOWARNING as i32
        }
        if options & XML_PARSE_NOERROR as i32 != 0 {
            (*safe_ctxt.sax).error = None;
            (*safe_ctxt.sax).fatalError = None;
            options -= XML_PARSE_NOERROR as i32
        }
    }
    match () {
        #[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]
        _ => unsafe {
            if options & XML_PARSE_SAX1 as i32 != 0 {
                (*safe_ctxt.sax).startElement = Some(
                    xmlSAX2StartElement
                        as unsafe extern "C" fn(
                            _: *mut (),
                            _: *const xmlChar,
                            _: *mut *const xmlChar,
                        ) -> (),
                );
                (*safe_ctxt.sax).endElement = Some(
                    xmlSAX2EndElement
                        as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
                );
                (*safe_ctxt.sax).startElementNs = None;
                (*safe_ctxt.sax).endElementNs = None;
                (*safe_ctxt.sax).initialized = 1 as i32 as u32;
                options -= XML_PARSE_SAX1 as i32;
                safe_ctxt.options |= XML_PARSE_SAX1 as i32
            }
        },
        #[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))]
        _ => {}
    };
    /* LIBXML_SAX1_ENABLED */
    if options & XML_PARSE_NODICT as i32 != 0 {
        safe_ctxt.dictNames = 0 as i32;
        options -= XML_PARSE_NODICT as i32;
        safe_ctxt.options |= XML_PARSE_NODICT as i32
    } else {
        safe_ctxt.dictNames = 1 as i32
    }
    if options & XML_PARSE_NOCDATA as i32 != 0 {
        unsafe {
            (*safe_ctxt.sax).cdataBlock = None;
        }
        options -= XML_PARSE_NOCDATA as i32;
        safe_ctxt.options |= XML_PARSE_NOCDATA as i32
    }
    if options & XML_PARSE_NSCLEAN as i32 != 0 {
        safe_ctxt.options |= XML_PARSE_NSCLEAN as i32;
        options -= XML_PARSE_NSCLEAN as i32
    }
    if options & XML_PARSE_NONET as i32 != 0 {
        safe_ctxt.options |= XML_PARSE_NONET as i32;
        options -= XML_PARSE_NONET as i32
    }
    if options & XML_PARSE_COMPACT as i32 != 0 {
        safe_ctxt.options |= XML_PARSE_COMPACT as i32;
        options -= XML_PARSE_COMPACT as i32
    }
    if options & XML_PARSE_OLD10 as i32 != 0 {
        safe_ctxt.options |= XML_PARSE_OLD10 as i32;
        options -= XML_PARSE_OLD10 as i32
    }
    if options & XML_PARSE_NOBASEFIX as i32 != 0 {
        safe_ctxt.options |= XML_PARSE_NOBASEFIX as i32;
        options -= XML_PARSE_NOBASEFIX as i32
    }
    if options & XML_PARSE_HUGE as i32 != 0 {
        safe_ctxt.options |= XML_PARSE_HUGE as i32;
        options -= XML_PARSE_HUGE as i32;
        if !safe_ctxt.dict.is_null() {
            xmlDictSetLimit_safe(safe_ctxt.dict, 0 as i32 as size_t);
        }
    }
    if options & XML_PARSE_OLDSAX as i32 != 0 {
        safe_ctxt.options |= XML_PARSE_OLDSAX as i32;
        options -= XML_PARSE_OLDSAX as i32
    }
    if options & XML_PARSE_IGNORE_ENC as i32 != 0 {
        safe_ctxt.options |= XML_PARSE_IGNORE_ENC as i32;
        options -= XML_PARSE_IGNORE_ENC as i32
    }
    if options & XML_PARSE_BIG_LINES as i32 != 0 {
        safe_ctxt.options |= XML_PARSE_BIG_LINES as i32;
        options -= XML_PARSE_BIG_LINES as i32
    }
    safe_ctxt.linenumbers = 1 as i32;
    return options;
}
/* *
* xmlCtxtUseOptions:
* @ctxt: an XML parser context
* @options:  a combination of xmlParserOption
*
* Applies the options to the parser context
*
* Returns 0 in case of success, the set of unknown or unimplemented options
*         in case of error.
*/

pub unsafe fn xmlCtxtUseOptions(
    mut ctxt: xmlParserCtxtPtr,
    mut options: i32,
) -> i32 {
    return xmlCtxtUseOptionsInternal(ctxt, options, 0 as *const i8);
}
/* *
* xmlDoRead:
* @ctxt:  an XML parser context
* @URL:  the base URL to use for the document
* @encoding:  the document encoding, or NULL
* @options:  a combination of xmlParserOption
* @reuse:  keep the context for reuse
*
* Common front-end for the xmlRead functions
*
* Returns the resulting document tree or NULL
*/

unsafe fn xmlDoRead(
    mut ctxt: xmlParserCtxtPtr,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
    mut reuse: i32,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = 0 as *mut xmlDoc;
    let mut safe_ctxt = unsafe { &mut *ctxt };

    unsafe {
        xmlCtxtUseOptionsInternal(ctxt, options, encoding);
    }
    if !encoding.is_null() {
        let mut hdlr: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
        hdlr = xmlFindCharEncodingHandler_safe(encoding);
        if !hdlr.is_null() {
            xmlSwitchToEncoding_safe(ctxt, hdlr);
        }
    }
    unsafe {
        if !URL.is_null() && !safe_ctxt.input.is_null() && (*safe_ctxt.input).filename.is_null() {
            (*safe_ctxt.input).filename = xmlStrdup(URL as *const xmlChar) as *mut i8
        }
    };
    unsafe { xmlParseDocument(ctxt) };
    if safe_ctxt.wellFormed != 0 || safe_ctxt.recovery != 0 {
        ret = safe_ctxt.myDoc
    } else {
        ret = 0 as xmlDocPtr;
        if !safe_ctxt.myDoc.is_null() {
            xmlFreeDoc_safe(safe_ctxt.myDoc);
        }
    }
    safe_ctxt.myDoc = 0 as xmlDocPtr;
    if reuse == 0 {
        xmlFreeParserCtxt_safe(ctxt);
    }
    return ret;
}
/* *
* xmlReadDoc:
* @cur:  a pointer to a zero terminated string
* @URL:  the base URL to use for the document
* @encoding:  the document encoding, or NULL
* @options:  a combination of xmlParserOption
*
* parse an XML in-memory document and build a tree.
*
* Returns the resulting document tree
*/

pub unsafe fn xmlReadDoc(
    mut cur: *const xmlChar,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    if cur.is_null() {
        return 0 as xmlDocPtr;
    }
    xmlInitParser_safe();
    unsafe { ctxt = xmlCreateDocParserCtxt(cur) };
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    return xmlDoRead(ctxt, URL, encoding, options, 0 as i32);
}
/* *
* xmlReadFile:
* @filename:  a file or URL
* @encoding:  the document encoding, or NULL
* @options:  a combination of xmlParserOption
*
* parse an XML file from the filesystem or the network.
*
* Returns the resulting document tree
*/

pub unsafe fn xmlReadFile(
    mut filename: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    xmlInitParser_safe();
    unsafe { ctxt = xmlCreateURLParserCtxt(filename, options) };
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    unsafe {
        return xmlDoRead(
            ctxt,
            0 as *const i8,
            encoding,
            options,
            0 as i32,
        );
    };
}
/* *
* xmlReadMemory:
* @buffer:  a pointer to a char array
* @size:  the size of the array
* @URL:  the base URL to use for the document
* @encoding:  the document encoding, or NULL
* @options:  a combination of xmlParserOption
*
* parse an XML in-memory document and build a tree.
*
* Returns the resulting document tree
*/

pub unsafe fn xmlReadMemory(
    mut buffer: *const i8,
    mut size: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    xmlInitParser_safe();
    ctxt = xmlCreateMemoryParserCtxt_safe(buffer, size);
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    unsafe { return xmlDoRead(ctxt, URL, encoding, options, 0 as i32) };
}
/* *
* xmlReadFd:
* @fd:  an open file descriptor
* @URL:  the base URL to use for the document
* @encoding:  the document encoding, or NULL
* @options:  a combination of xmlParserOption
*
* parse an XML from a file descriptor and build a tree.
* NOTE that the file descriptor will not be closed when the
*      reader is closed or reset.
*
* Returns the resulting document tree
*/

pub unsafe fn xmlReadFd(
    mut fd: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut safe_input = unsafe { &mut *input };
    if fd < 0 as i32 {
        return 0 as xmlDocPtr;
    }
    xmlInitParser_safe();
    input = xmlParserInputBufferCreateFd_safe(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return 0 as xmlDocPtr;
    }
    safe_input.closecallback = None;
    ctxt = xmlNewParserCtxt_safe();
    if ctxt.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        return 0 as xmlDocPtr;
    }
    stream = xmlNewIOInputStream_safe(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as xmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    unsafe { return xmlDoRead(ctxt, URL, encoding, options, 0 as i32) };
}
/* *
* xmlReadIO:
* @ioread:  an I/O read function
* @ioclose:  an I/O close function
* @ioctx:  an I/O handler
* @URL:  the base URL to use for the document
* @encoding:  the document encoding, or NULL
* @options:  a combination of xmlParserOption
*
* parse an XML document from I/O functions and source and build a tree.
*
* Returns the resulting document tree
*/

pub unsafe fn xmlReadIO(
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut (),
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ioread.is_none() {
        return 0 as xmlDocPtr;
    }
    xmlInitParser_safe();
    input = xmlParserInputBufferCreateIO_safe(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        unsafe {
            if ioclose.is_some() {
                ioclose.expect("non-null function pointer")(ioctx);
            }
        }
        return 0 as xmlDocPtr;
    }
    ctxt = xmlNewParserCtxt_safe();
    if ctxt.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        return 0 as xmlDocPtr;
    }
    stream = xmlNewIOInputStream_safe(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        xmlFreeParserCtxt_safe(ctxt);
        return 0 as xmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    unsafe { return xmlDoRead(ctxt, URL, encoding, options, 0 as i32) };
}
/* *
* xmlCtxtReadDoc:
* @ctxt:  an XML parser context
* @cur:  a pointer to a zero terminated string
* @URL:  the base URL to use for the document
* @encoding:  the document encoding, or NULL
* @options:  a combination of xmlParserOption
*
* parse an XML in-memory document and build a tree.
* This reuses the existing @ctxt parser context
*
* Returns the resulting document tree
*/

pub unsafe fn xmlCtxtReadDoc(
    mut ctxt: xmlParserCtxtPtr,
    mut cur: *const xmlChar,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if cur.is_null() {
        return 0 as xmlDocPtr;
    }
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    xmlInitParser_safe();
    xmlCtxtReset_safe(ctxt);
    stream = xmlNewStringInputStream_safe(ctxt, cur);
    if stream.is_null() {
        return 0 as xmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    unsafe { return xmlDoRead(ctxt, URL, encoding, options, 1 as i32) };
}
/* *
* xmlCtxtReadFile:
* @ctxt:  an XML parser context
* @filename:  a file or URL
* @encoding:  the document encoding, or NULL
* @options:  a combination of xmlParserOption
*
* parse an XML file from the filesystem or the network.
* This reuses the existing @ctxt parser context
*
* Returns the resulting document tree
*/

pub unsafe fn xmlCtxtReadFile(
    mut ctxt: xmlParserCtxtPtr,
    mut filename: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if filename.is_null() {
        return 0 as xmlDocPtr;
    }
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    xmlInitParser_safe();
    xmlCtxtReset_safe(ctxt);
    stream = xmlLoadExternalEntity_safe(filename, 0 as *const i8, ctxt);
    if stream.is_null() {
        return 0 as xmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    unsafe {
        return xmlDoRead(
            ctxt,
            0 as *const i8,
            encoding,
            options,
            1 as i32,
        );
    };
}
/* *
* xmlCtxtReadMemory:
* @ctxt:  an XML parser context
* @buffer:  a pointer to a char array
* @size:  the size of the array
* @URL:  the base URL to use for the document
* @encoding:  the document encoding, or NULL
* @options:  a combination of xmlParserOption
*
* parse an XML in-memory document and build a tree.
* This reuses the existing @ctxt parser context
*
* Returns the resulting document tree
*/

pub unsafe fn xmlCtxtReadMemory(
    mut ctxt: xmlParserCtxtPtr,
    mut buffer: *const i8,
    mut size: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    if buffer.is_null() {
        return 0 as xmlDocPtr;
    }
    xmlInitParser_safe();
    xmlCtxtReset_safe(ctxt);
    input = xmlParserInputBufferCreateMem_safe(buffer, size, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return 0 as xmlDocPtr;
    }
    stream = xmlNewIOInputStream_safe(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        return 0 as xmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    unsafe { return xmlDoRead(ctxt, URL, encoding, options, 1 as i32) };
}

pub unsafe fn xmlCtxtReadFd(
    mut ctxt: xmlParserCtxtPtr,
    mut fd: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut safe_input = unsafe { &mut *input };
    if fd < 0 as i32 {
        return 0 as xmlDocPtr;
    }
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    xmlInitParser_safe();
    xmlCtxtReset_safe(ctxt);
    input = xmlParserInputBufferCreateFd_safe(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return 0 as xmlDocPtr;
    }
    safe_input.closecallback = None;
    stream = xmlNewIOInputStream_safe(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        return 0 as xmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    unsafe { return xmlDoRead(ctxt, URL, encoding, options, 1 as i32) };
}

pub unsafe fn xmlCtxtReadIO(
    mut ctxt: xmlParserCtxtPtr,
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut (),
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlDocPtr {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut stream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ioread.is_none() {
        return 0 as xmlDocPtr;
    }
    if ctxt.is_null() {
        return 0 as xmlDocPtr;
    }
    xmlInitParser_safe();
    xmlCtxtReset_safe(ctxt);
    input = xmlParserInputBufferCreateIO_safe(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        unsafe {
            if ioclose.is_some() {
                ioclose.expect("non-null function pointer")(ioctx);
            }
        }
        return 0 as xmlDocPtr;
    }
    stream = xmlNewIOInputStream_safe(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer_safe(input);
        return 0 as xmlDocPtr;
    }
    inputPush_safe(ctxt, stream);
    unsafe { return xmlDoRead(ctxt, URL, encoding, options, 1 as i32) };
}
