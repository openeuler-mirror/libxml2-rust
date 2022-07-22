
#[no_mangle]
pub extern "C" fn htmlInitAutoClose_rust() {
    println!("fn {} begin", "htmlInitAutoClose");
    unsafe {
        htmlInitAutoClose_htmlparser();
    }
    println!("fn {} ok", "htmlInitAutoClose");
}

#[no_mangle]
pub extern "C" fn htmlTagLookup_rust(mut tag: *const xmlChar)
                                     -> *const htmlElemDesc {
    println!("fn {} begin", "htmlTagLookup");
    let res: *const htmlElemDesc = unsafe {
        htmlTagLookup(tag)
    };
    println!("fn {} ok", "htmlTagLookup");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlAutoCloseTag_rust(mut doc: htmlDocPtr,
                                        mut name: *const xmlChar,
                                        mut elem: htmlNodePtr)
                                        -> libc::c_int {
    println!("fn {} begin", "htmlAutoCloseTag");
    let res: libc::c_int = unsafe {
        htmlAutoCloseTag(doc, name, elem)
    };
    println!("fn {} ok", "htmlAutoCloseTag");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlIsAutoClosed_rust(mut doc: htmlDocPtr,
                                        mut elem: htmlNodePtr)
                                        -> libc::c_int {
    println!("fn {} begin", "htmlIsAutoClosed");
    let res: libc::c_int = unsafe {
        htmlIsAutoClosed(doc, elem)
    };
    println!("fn {} ok", "htmlIsAutoClosed");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlIsScriptAttribute_rust(mut name: *const xmlChar)
                                             -> libc::c_int {
    println!("fn {} begin", "htmlIsScriptAttribute");
    let res: libc::c_int = unsafe {
        htmlIsScriptAttribute(name)
    };
    println!("fn {} ok", "htmlIsScriptAttribute");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlEntityLookup_rust(mut name: *const xmlChar)
                                        -> *const htmlEntityDesc {
    println!("fn {} begin", "htmlEntityLookup");
    let res: *const htmlEntityDesc = unsafe {
        htmlEntityLookup(name)
    };
    println!("fn {} ok", "htmlEntityLookup");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlEntityValueLookup_rust(mut value: libc::c_uint)
                                             -> *const htmlEntityDesc {
    println!("fn {} begin", "htmlEntityValueLookup");
    let res: *const htmlEntityDesc = unsafe {
        htmlEntityValueLookup(value)
    };
    println!("fn {} ok", "htmlEntityValueLookup");
    return res;
}

#[no_mangle]
pub extern "C" fn UTF8ToHtml_rust(mut out: *mut libc::c_uchar,
                                  mut outlen: *mut libc::c_int,
                                  mut in_0: *const libc::c_uchar,
                                  mut inlen: *mut libc::c_int)
                                  -> libc::c_int {
    println!("fn {} begin", "UTF8ToHtml");
    let res: libc::c_int = unsafe {
        UTF8ToHtml(out, outlen, in_0, inlen)
    };
    println!("fn {} ok", "UTF8ToHtml");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlEncodeEntities_rust(mut out: *mut libc::c_uchar,
                                          mut outlen: *mut libc::c_int,
                                          mut in_0: *const libc::c_uchar,
                                          mut inlen: *mut libc::c_int,
                                          mut quoteChar: libc::c_int)
                                          -> libc::c_int {
    println!("fn {} begin", "htmlEncodeEntities");
    let res: libc::c_int = unsafe {
        htmlEncodeEntities(out, outlen, in_0, inlen, quoteChar)
    };
    println!("fn {} ok", "htmlEncodeEntities");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlNewDocNoDtD_rust(mut URI: *const xmlChar,
                                       mut ExternalID: *const xmlChar)
                                       -> htmlDocPtr {
    println!("fn {} begin", "htmlNewDocNoDtD");
    let res: htmlDocPtr = unsafe {
        htmlNewDocNoDtD(URI, ExternalID)
    };
    println!("fn {} ok", "htmlNewDocNoDtD");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlNewDoc_rust(mut URI: *const xmlChar,
                                  mut ExternalID: *const xmlChar)
                                  -> htmlDocPtr {
    println!("fn {} begin", "htmlNewDoc");
    let res: htmlDocPtr = unsafe {
        htmlNewDoc(URI, ExternalID)
    };
    println!("fn {} ok", "htmlNewDoc");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlParseEntityRef_rust(mut ctxt: htmlParserCtxtPtr,
                                          mut str: *mut *const xmlChar)
                                          -> *const htmlEntityDesc {
    println!("fn {} begin", "htmlParseEntityRef");
    let res: *const htmlEntityDesc = unsafe {
        htmlParseEntityRef(ctxt, str)
    };
    println!("fn {} ok", "htmlParseEntityRef");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlParseCharRef_rust(mut ctxt: htmlParserCtxtPtr)
                                        -> libc::c_int {
    println!("fn {} begin", "htmlParseCharRef");
    let res: libc::c_int = unsafe {
        htmlParseCharRef(ctxt)
    };
    println!("fn {} ok", "htmlParseCharRef");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlParseElement_rust(mut ctxt: htmlParserCtxtPtr) {
    println!("fn {} begin", "htmlParseElement");
    unsafe {
        htmlParseElement(ctxt);
    }
    println!("fn {} ok", "htmlParseElement");
}

#[no_mangle]
pub extern "C" fn __htmlParseContent_rust(mut ctxt: *mut libc::c_void) {
    println!("fn {} begin", "__htmlParseContent");
    unsafe {
        __htmlParseContent_htmlparser(ctxt);
    }
    println!("fn {} ok", "__htmlParseContent");
}

#[no_mangle]
pub extern "C" fn htmlParseDocument_rust(mut ctxt: htmlParserCtxtPtr)
                                         -> libc::c_int {
    println!("fn {} begin", "htmlParseDocument");
    let res: libc::c_int = unsafe {
        htmlParseDocument(ctxt)
    };
    println!("fn {} ok", "htmlParseDocument");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlFreeParserCtxt_rust(mut ctxt: htmlParserCtxtPtr) {
    println!("fn {} begin", "htmlFreeParserCtxt");
    unsafe {
        htmlFreeParserCtxt(ctxt);
    }
    println!("fn {} ok", "htmlFreeParserCtxt");
}

#[no_mangle]
pub extern "C" fn htmlNewParserCtxt_rust() -> htmlParserCtxtPtr {
    println!("fn {} begin", "htmlNewParserCtxt");
    let res: htmlParserCtxtPtr = unsafe {
        htmlNewParserCtxt()
    };
    println!("fn {} ok", "htmlNewParserCtxt");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlCreateMemoryParserCtxt_rust(mut buffer:
                                                  *const libc::c_char,
                                                  mut size: libc::c_int)
                                                  -> htmlParserCtxtPtr {
    println!("fn {} begin", "htmlCreateMemoryParserCtxt");
    let res: htmlParserCtxtPtr = unsafe {
        htmlCreateMemoryParserCtxt_htmlparser
        (buffer, size)
    };
    println!("fn {} ok", "htmlCreateMemoryParserCtxt");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlParseChunk_rust(mut ctxt: htmlParserCtxtPtr,
                                      mut chunk: *const libc::c_char,
                                      mut size: libc::c_int,
                                      mut terminate: libc::c_int)
                                      -> libc::c_int {
    println!("fn {} begin", "htmlParseChunk");
    let mut res: libc::c_int = 0 as libc::c_int;
    match () {
        #[cfg(LIBXML_PUSH_ENABLED)]      _ => {
            res = unsafe {
                htmlParseChunk(ctxt, chunk, size, terminate)
            };
        }
        #[cfg(not(LIBXML_PUSH_ENABLED))]      _ => {}
    };
    println!("fn {} ok", "htmlParseChunk");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlCreatePushParserCtxt_rust(mut sax: htmlSAXHandlerPtr,
                                                mut user_data:
                                                *mut libc::c_void,
                                                mut chunk:
                                                *const libc::c_char,
                                                mut size: libc::c_int,
                                                mut filename:
                                                *const libc::c_char,
                                                mut enc: xmlCharEncoding)
                                                -> htmlParserCtxtPtr {
    println!("fn {} begin", "htmlCreatePushParserCtxt");
    let mut res: htmlParserCtxtPtr = 0 as htmlParserCtxtPtr;
    match () {
        #[cfg(LIBXML_PUSH_ENABLED)]      _ => {
            res = unsafe {
                htmlCreatePushParserCtxt(sax, user_data, chunk, size, filename, enc)
            };
        }
        #[cfg(not(LIBXML_PUSH_ENABLED))]      _ => {}
    };
    println!("fn {} ok", "htmlCreatePushParserCtxt");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlSAXParseDoc_rust(mut cur: *const xmlChar,
                                       mut encoding: *const libc::c_char,
                                       mut sax: htmlSAXHandlerPtr,
                                       mut userData: *mut libc::c_void)
                                       -> htmlDocPtr {
    println!("fn {} begin", "htmlSAXParseDoc");
    let res: htmlDocPtr = unsafe {
        htmlSAXParseDoc(cur, encoding, sax, userData)
    };
    println!("fn {} ok", "htmlSAXParseDoc");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlParseDoc_rust(mut cur: *const xmlChar,
                                    mut encoding: *const libc::c_char)
                                    -> htmlDocPtr {
    println!("fn {} begin", "htmlParseDoc");
    let res: htmlDocPtr = unsafe {
        htmlParseDoc(cur, encoding)
    };
    println!("fn {} ok", "htmlParseDoc");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlCreateFileParserCtxt_rust(mut filename:
                                                *const libc::c_char,
                                                mut encoding:
                                                *const libc::c_char)
                                                -> htmlParserCtxtPtr {
    println!("fn {} begin", "htmlCreateFileParserCtxt");
    let res: htmlParserCtxtPtr = unsafe {
        htmlCreateFileParserCtxt(filename, encoding)
    };
    println!("fn {} ok", "htmlCreateFileParserCtxt");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlSAXParseFile_rust(mut filename: *const libc::c_char,
                                        mut encoding: *const libc::c_char,
                                        mut sax: htmlSAXHandlerPtr,
                                        mut userData: *mut libc::c_void)
                                        -> htmlDocPtr {
    println!("fn {} begin", "htmlSAXParseFile");
    let res: htmlDocPtr = unsafe {
        htmlSAXParseFile(filename, encoding, sax, userData)
    };
    println!("fn {} ok", "htmlSAXParseFile");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlParseFile_rust(mut filename: *const libc::c_char,
                                     mut encoding: *const libc::c_char)
                                     -> htmlDocPtr {
    println!("fn {} begin", "htmlParseFile");
    let res: htmlDocPtr = unsafe {
        htmlParseFile(filename, encoding)
    };
    println!("fn {} ok", "htmlParseFile");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlHandleOmittedElem_rust(mut val: libc::c_int)
                                             -> libc::c_int {
    println!("fn {} begin", "htmlHandleOmittedElem");
    let res: libc::c_int = unsafe {
        htmlHandleOmittedElem(val)
    };
    println!("fn {} ok", "htmlHandleOmittedElem");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlElementAllowedHere_rust(mut parent:
                                              *const htmlElemDesc,
                                              mut elt: *const xmlChar)
                                              -> libc::c_int {
    println!("fn {} begin", "htmlElementAllowedHere");
    let res: libc::c_int = unsafe {
        htmlElementAllowedHere(parent, elt)
    };
    println!("fn {} ok", "htmlElementAllowedHere");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlElementStatusHere_rust(mut parent:
                                             *const htmlElemDesc,
                                             mut elt: *const htmlElemDesc)
                                             -> htmlStatus {
    println!("fn {} begin", "htmlElementStatusHere");
    let res: htmlStatus = unsafe {
        htmlElementStatusHere(parent, elt)
    };
    println!("fn {} ok", "htmlElementStatusHere");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlAttrAllowed_rust(mut elt: *const htmlElemDesc,
                                       mut attr: *const xmlChar,
                                       mut legacy: libc::c_int)
                                       -> htmlStatus {
    println!("fn {} begin", "htmlAttrAllowed");
    let res: htmlStatus = unsafe {
        htmlAttrAllowed(elt, attr, legacy)
    };
    println!("fn {} ok", "htmlAttrAllowed");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlNodeStatus_rust(node: htmlNodePtr,
                                      mut legacy: libc::c_int)
                                      -> htmlStatus {
    println!("fn {} begin", "htmlNodeStatus");
    let res: htmlStatus = unsafe {
        htmlNodeStatus(node, legacy)
    };
    println!("fn {} ok", "htmlNodeStatus");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlCtxtReset_rust(mut ctxt: htmlParserCtxtPtr) {
    println!("fn {} begin", "htmlCtxtReset");
    unsafe {
        htmlCtxtReset(ctxt);
    }
    println!("fn {} ok", "htmlCtxtReset");
}

#[no_mangle]
pub extern "C" fn htmlCtxtUseOptions_rust(mut ctxt: htmlParserCtxtPtr,
                                          mut options: libc::c_int)
                                          -> libc::c_int {
    println!("fn {} begin", "htmlCtxtUseOptions");
    let res: libc::c_int = unsafe {
        htmlCtxtUseOptions(ctxt, options)
    };
    println!("fn {} ok", "htmlCtxtUseOptions");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlReadDoc_rust(mut cur: *const xmlChar,
                                   mut URL: *const libc::c_char,
                                   mut encoding: *const libc::c_char,
                                   mut options: libc::c_int) -> htmlDocPtr {
    println!("fn {} begin", "htmlReadDoc");
    let res: htmlDocPtr = unsafe {
        htmlReadDoc(cur, URL, encoding, options)
    };
    println!("fn {} ok", "htmlReadDoc");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlReadFile_rust(mut filename: *const libc::c_char,
                                    mut encoding: *const libc::c_char,
                                    mut options: libc::c_int)
                                    -> htmlDocPtr {
    println!("fn {} begin", "htmlReadFile");
    let res: htmlDocPtr = unsafe {
        htmlReadFile(filename, encoding, options)
    };
    println!("fn {} ok", "htmlReadFile");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlReadMemory_rust(mut buffer: *const libc::c_char,
                                      mut size: libc::c_int,
                                      mut URL: *const libc::c_char,
                                      mut encoding: *const libc::c_char,
                                      mut options: libc::c_int)
                                      -> htmlDocPtr {
    println!("fn {} begin", "htmlReadMemory");
    let res: htmlDocPtr = unsafe {
        htmlReadMemory(buffer, size, URL, encoding, options)
    };
    println!("fn {} ok", "htmlReadMemory");
    return res;
}

#[no_mangle]
pub unsafe fn htmlReadFd_rust(mut fd: libc::c_int,
                              mut URL: *const libc::c_char,
                              mut encoding: *const libc::c_char,
                              mut options: libc::c_int) -> htmlDocPtr {
    println!("fn {} begin", "htmlReadFd");
    let res: htmlDocPtr = unsafe {
        htmlReadFd(fd, URL, encoding, options)
    };
    println!("fn {} ok", "htmlReadFd");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlReadIO_rust(mut ioread: xmlInputReadCallback,
                                  mut ioclose: xmlInputCloseCallback,
                                  mut ioctx: *mut libc::c_void,
                                  mut URL: *const libc::c_char,
                                  mut encoding: *const libc::c_char,
                                  mut options: libc::c_int) -> htmlDocPtr {
    println!("fn {} begin", "htmlReadIO");
    let res: htmlDocPtr = unsafe {
        htmlReadIO(ioread, ioclose, ioctx, URL, encoding, options)
    };
    println!("fn {} ok", "htmlReadIO");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlCtxtReadDoc_rust(mut ctxt: htmlParserCtxtPtr,
                                       mut cur: *const xmlChar,
                                       mut URL: *const libc::c_char,
                                       mut encoding: *const libc::c_char,
                                       mut options: libc::c_int)
                                       -> htmlDocPtr {
    println!("fn {} begin", "htmlCtxtReadDoc");
    let res: htmlDocPtr = unsafe {
        htmlCtxtReadDoc(ctxt, cur, URL, encoding, options)
    };
    println!("fn {} ok", "htmlCtxtReadDoc");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlCtxtReadFile_rust(mut ctxt: htmlParserCtxtPtr,
                                        mut filename: *const libc::c_char,
                                        mut encoding: *const libc::c_char,
                                        mut options: libc::c_int)
                                        -> htmlDocPtr {
    println!("fn {} begin", "htmlCtxtReadFile");
    let res: htmlDocPtr = unsafe {
        htmlCtxtReadFile(ctxt, filename, encoding, options)
    };
    println!("fn {} ok", "htmlCtxtReadFile");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlCtxtReadMemory_rust(mut ctxt: htmlParserCtxtPtr,
                                          mut buffer: *const libc::c_char,
                                          mut size: libc::c_int,
                                          mut URL: *const libc::c_char,
                                          mut encoding: *const libc::c_char,
                                          mut options: libc::c_int)
                                          -> htmlDocPtr {
    println!("fn {} begin", "htmlCtxtReadMemory");
    let res: htmlDocPtr = unsafe {
        htmlCtxtReadMemory(ctxt, buffer, size, URL, encoding, options)
    };
    println!("fn {} ok", "htmlCtxtReadMemory");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlCtxtReadFd_rust(mut ctxt: htmlParserCtxtPtr,
                                      mut fd: libc::c_int,
                                      mut URL: *const libc::c_char,
                                      mut encoding: *const libc::c_char,
                                      mut options: libc::c_int)
                                      -> htmlDocPtr {
    println!("fn {} begin", "htmlCtxtReadFd");
    let res: htmlDocPtr = unsafe {
        htmlCtxtReadFd(ctxt, fd, URL, encoding, options)
    };
    println!("fn {} ok", "htmlCtxtReadFd");
    return res;
}

#[no_mangle]
pub extern "C" fn htmlCtxtReadIO_rust(mut ctxt: htmlParserCtxtPtr,
                                      mut ioread: xmlInputReadCallback,
                                      mut ioclose: xmlInputCloseCallback,
                                      mut ioctx: *mut libc::c_void,
                                      mut URL: *const libc::c_char,
                                      mut encoding: *const libc::c_char,
                                      mut options: libc::c_int)
                                      -> htmlDocPtr {
    println!("fn {} begin", "htmlCtxtReadIO");
    let res: htmlDocPtr = unsafe {
        htmlCtxtReadIO(ctxt, ioread, ioclose, ioctx, URL, encoding, options)
    };
    println!("fn {} ok", "htmlCtxtReadIO");
    return res;
}


#[no_mangle]
pub extern "C" fn htmlParseTryOrFinish_rust(mut ctxt: htmlParserCtxtPtr,
                                            mut terminate: libc::c_int)
                                            -> libc::c_int {
    println!("fn {} used", "htmlParseTryOrFinish_rust");
    let mut res: libc::c_int = 0 as libc::c_int;

    match () {
        #[cfg(LIBXML_PUSH_ENABLED)]      _ => {
            res = unsafe {
                htmlParseTryOrFinish(ctxt, terminate)
            };
        }
        #[cfg(not(LIBXML_PUSH_ENABLED))]      _ => {}
    };
    return res;
}

#[no_mangle]
pub extern "C" fn htmlParseLookupSequence_rust(mut ctxt: htmlParserCtxtPtr,
                                               mut first: xmlChar,
                                               mut next: xmlChar,
                                               mut third: xmlChar,
                                               mut ignoreattrval: libc::c_int)
                                               -> libc::c_int {
    println!("fn {} used", "htmlParseLookupSequence_rust");
    let mut res: libc::c_int = 0 as libc::c_int;
    match () {
        #[cfg(LIBXML_PUSH_ENABLED)]      _ => {
            res = unsafe {
                htmlParseLookupSequence(ctxt, first, next, third, ignoreattrval)
            };
        }
        #[cfg(not(LIBXML_PUSH_ENABLED))]      _ => {}
    };
    return res;
}


