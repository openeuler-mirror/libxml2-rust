
 #[no_mangle]
 pub unsafe extern "C" fn xmlHasFeature_rust(mut feature: xmlFeature) -> libc::c_int{
     let res: libc::c_int = unsafe{
         xmlHasFeature(feature)
     };
     return res;
 }
 
 #[no_mangle]
 pub extern "C" fn xmlCheckLanguageID_rust(mut lang: *const xmlChar) -> libc::c_int{
     let res: libc::c_int = unsafe{
         xmlCheckLanguageID(lang)
     };
     return res;
 }
 
 #[no_mangle]
 pub extern "C" fn inputPush_rust(mut ctxt: xmlParserCtxtPtr, mut value: xmlParserInputPtr)  -> libc::c_int{
     let res: libc::c_int = unsafe{
         inputPush_parser(ctxt, value)
     };
     return res;
 }
 
 #[no_mangle]
 pub extern "C" fn inputPop_rust(mut ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr{
     let res: xmlParserInputPtr = unsafe{
         inputPop_parser(ctxt)
     };
     return res;
 }
 
 #[no_mangle]
 pub extern "C" fn nodePush_rust(mut ctxt: xmlParserCtxtPtr, mut value: xmlNodePtr) -> libc::c_int {
     let res: libc::c_int = unsafe{
         nodePush(ctxt, value)
     };
     return res;
 }
 
 #[no_mangle]
 pub extern "C" fn nodePop_rust(mut ctxt: xmlParserCtxtPtr) -> xmlNodePtr{
     let res: xmlNodePtr = unsafe{
         nodePop_parser(ctxt)
     };
     return res;
 }
 
 #[no_mangle]
 pub extern "C" fn namePush_rust(mut ctxt: xmlParserCtxtPtr, mut value: * const xmlChar) -> libc::c_int {
     let res: libc::c_int = unsafe{
         namePush(ctxt, value)
     };
     return res;
 }
 
 #[no_mangle]
 pub extern "C" fn namePop_rust(mut ctxt: xmlParserCtxtPtr) -> * const xmlChar {
     let res: * const xmlChar = unsafe{
         namePop(ctxt)
     };
     return res;
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSkipBlankChars_rust(mut ctxt: xmlParserCtxtPtr) -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlSkipBlankChars(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlPopInput_rust(mut ctxt: xmlParserCtxtPtr) -> xmlChar {
     let res: xmlChar  = unsafe{
         xmlPopInput_parser(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlPushInput_rust(mut ctxt: xmlParserCtxtPtr, mut input: xmlParserInputPtr) -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlPushInput(ctxt, input)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseCharRef_rust(mut ctxt: xmlParserCtxtPtr) -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseCharRef(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParserHandlePEReference_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParserHandlePEReference(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlStringLenDecodeEntities_rust(mut ctxt:
                                                     xmlParserCtxtPtr,
                                                     mut str: *const xmlChar,
                                                     mut len: libc::c_int,
                                                     mut what: libc::c_int,
                                                     mut end: xmlChar,
                                                     mut end2: xmlChar,
                                                     mut end3: xmlChar)
                                                     -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlStringLenDecodeEntities(ctxt,
                                    str,
                                    len,
                                    what,
                                    end,
                                    end2,
                                    end3)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlStringDecodeEntities_rust(mut ctxt: xmlParserCtxtPtr,
                                                  mut str: *const xmlChar,
                                                  mut what: libc::c_int,
                                                  mut end: xmlChar,
                                                  mut end2: xmlChar,
                                                  mut end3: xmlChar)
                                                  -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlStringDecodeEntities(ctxt,
                                                   str,
                                                   what,
                                                   end,
                                                   end2,
                                                   end3)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSplitQName_rust(mut ctxt: xmlParserCtxtPtr,
                                        mut name: *const xmlChar,
                                        mut prefix: *mut *mut xmlChar)
                                        -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlSplitQName(ctxt,
                                        name,
                                        prefix)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseName_rust(mut ctxt: xmlParserCtxtPtr)
                                       -> *const xmlChar {
     let res: *const xmlChar  = unsafe{
         xmlParseName(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseNmtoken_rust(mut ctxt: xmlParserCtxtPtr)
                                          -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlParseNmtoken(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseEntityValue_rust(mut ctxt: xmlParserCtxtPtr,
                                              mut orig: *mut *mut xmlChar)
                                              -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlParseEntityValue(ctxt, orig)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseAttValue_rust(mut ctxt: xmlParserCtxtPtr)
                                           -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlParseAttValue(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseSystemLiteral_rust(mut ctxt: xmlParserCtxtPtr)
                                                -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlParseSystemLiteral(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParsePubidLiteral_rust(mut ctxt: xmlParserCtxtPtr)
                                               -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlParsePubidLiteral(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseCharData_rust(mut ctxt: xmlParserCtxtPtr, mut cdata: libc::c_int) {
     unsafe{
         xmlParseCharData(ctxt, cdata)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseExternalID_rust(mut ctxt: xmlParserCtxtPtr,
                                             mut publicID: *mut *mut xmlChar,
                                             mut strict: libc::c_int)
                                             -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlParseExternalID(ctxt, publicID, strict)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseComment_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseComment(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParsePITarget_rust(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
     let res: *const xmlChar  = unsafe{
         xmlParsePITarget(ctxt)
     };
     return res;
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParsePI_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParsePI(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseNotationDecl_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseNotationDecl(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseEntityDecl_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseEntityDecl(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseDefaultDecl_rust(mut ctxt: xmlParserCtxtPtr, mut value: *mut *mut xmlChar)
                                              -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseDefaultDecl(ctxt, value)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseNotationType_rust(mut ctxt: xmlParserCtxtPtr) -> xmlEnumerationPtr {
     let res: xmlEnumerationPtr  = unsafe{
         xmlParseNotationType(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseEnumerationType_rust(mut ctxt: xmlParserCtxtPtr) -> xmlEnumerationPtr {
     let res: xmlEnumerationPtr  = unsafe{
         xmlParseEnumerationType(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseEnumeratedType_rust(mut ctxt: xmlParserCtxtPtr,
                                                 mut tree:
                                                 *mut xmlEnumerationPtr)
                                                 -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseEnumeratedType(ctxt, tree)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseAttributeType_rust(mut ctxt: xmlParserCtxtPtr, mut tree: *mut xmlEnumerationPtr) -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseAttributeType(ctxt, tree)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseAttributeListDecl_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseAttributeListDecl(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseElementMixedContentDecl_rust(mut ctxt:
                                                          xmlParserCtxtPtr,
                                                          mut inputchk:
                                                          libc::c_int)
                                                          -> xmlElementContentPtr {
     let res: xmlElementContentPtr  = unsafe{
         xmlParseElementMixedContentDecl(ctxt, inputchk)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseElementChildrenContentDecl_rust(mut ctxt:
                                                             xmlParserCtxtPtr,
                                                             mut inputchk:
                                                             libc::c_int)
                                                             -> xmlElementContentPtr {
     let res: xmlElementContentPtr  = unsafe{
         xmlParseElementChildrenContentDecl(ctxt, inputchk)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseElementContentDecl_rust(mut ctxt:
                                                     xmlParserCtxtPtr,
                                                     mut name: *const xmlChar,
                                                     mut result:
                                                     *mut xmlElementContentPtr)
                                                     -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseElementContentDecl(ctxt, name, result)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseElementDecl_rust(mut ctxt: xmlParserCtxtPtr) -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseElementDecl(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseMarkupDecl_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseMarkupDecl(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseTextDecl_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseTextDecl(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseExternalSubset_rust(mut ctxt: xmlParserCtxtPtr,
                                                 mut ExternalID:
                                                 *const xmlChar,
                                                 mut SystemID:
                                                 *const xmlChar) {
     unsafe{
         xmlParseExternalSubset(ctxt, ExternalID, SystemID)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseReference_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseReference(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseEntityRef_rust(mut ctxt: xmlParserCtxtPtr) -> xmlEntityPtr {
     let res: xmlEntityPtr  = unsafe{
         xmlParseEntityRef(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParsePEReference_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParsePEReference(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseDocTypeDecl_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseDocTypeDecl(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseAttribute_rust(mut ctxt: xmlParserCtxtPtr, mut value: *mut *mut xmlChar) -> *const xmlChar {
     let res: *const xmlChar  = unsafe{
         xmlParseAttribute(ctxt, value)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseStartTag_rust(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
     let res: *const xmlChar  = unsafe{
         xmlParseStartTag(ctxt)
     };
     return res;
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseEndTag_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseEndTag(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseCDSect_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseCDSect(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseContent_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseContent(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseElement_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseElement(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseVersionNum_rust(mut ctxt: xmlParserCtxtPtr)
                                             -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlParseVersionNum(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseVersionInfo_rust(mut ctxt: xmlParserCtxtPtr)
                                              -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlParseVersionInfo(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseEncName_rust(mut ctxt: xmlParserCtxtPtr)
                                          -> *mut xmlChar {
     let res: *mut xmlChar  = unsafe{
         xmlParseEncName(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseEncodingDecl_rust(mut ctxt: xmlParserCtxtPtr)
                                               -> *const xmlChar {
     let res: *const xmlChar  = unsafe{
         xmlParseEncodingDecl(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseSDDecl_rust(mut ctxt: xmlParserCtxtPtr)
                                         -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseSDDecl(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseXMLDecl_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseXMLDecl(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseMisc_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlParseMisc(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseDocument_rust(mut ctxt: xmlParserCtxtPtr)
                                           -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseDocument(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseExtParsedEnt_rust(mut ctxt: xmlParserCtxtPtr)
                                               -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseExtParsedEnt(ctxt)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseChunk_rust(mut ctxt: xmlParserCtxtPtr,
                                        mut chunk: *const libc::c_char,
                                        mut size: libc::c_int,
                                        mut terminate: libc::c_int)
                                        -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseChunk(ctxt,
                                         chunk,
                                         size,
                                         terminate)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCreatePushParserCtxt_rust(mut sax: xmlSAXHandlerPtr,
                                                  mut user_data:
                                                  *mut libc::c_void,
                                                  mut chunk:
                                                  *const libc::c_char,
                                                  mut size: libc::c_int,
                                                  mut filename:
                                                  *const libc::c_char)
                                                  -> xmlParserCtxtPtr {
     let res: xmlParserCtxtPtr  = unsafe{
         xmlCreatePushParserCtxt(sax,
                                                   user_data
                                                  ,
                                                   chunk
                                                  ,
                                                   size,
                                                   filename
                                                  )
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlStopParser_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlStopParser_parser(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCreateIOParserCtxt_rust(mut sax: xmlSAXHandlerPtr,
                                                mut user_data:
                                                *mut libc::c_void,
                                                mut ioread:
                                                xmlInputReadCallback,
                                                mut ioclose:
                                                xmlInputCloseCallback,
                                                mut ioctx: *mut libc::c_void,
                                                mut enc: xmlCharEncoding)
                                                -> xmlParserCtxtPtr {
     let res: xmlParserCtxtPtr  = unsafe{
         xmlCreateIOParserCtxt(sax, user_data, ioread, ioclose, ioctx, enc)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlIOParseDTD_rust(mut sax: xmlSAXHandlerPtr,
                                        mut input: xmlParserInputBufferPtr,
                                        mut enc: xmlCharEncoding)
                                        -> xmlDtdPtr {
     let res: xmlDtdPtr  = unsafe{
         xmlIOParseDTD(sax,
                                         input,
                                         enc)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSAXParseDTD_rust(mut sax: xmlSAXHandlerPtr,
                                         mut ExternalID: *const xmlChar,
                                         mut SystemID: *const xmlChar)
                                         -> xmlDtdPtr {
     let res: xmlDtdPtr  = unsafe{
         xmlSAXParseDTD(sax,
                                          ExternalID,
                                          SystemID)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseDTD_rust(mut ExternalID: *const xmlChar,
                                      mut SystemID: *const xmlChar)
                                      -> xmlDtdPtr {
     let res: xmlDtdPtr  = unsafe{
         xmlParseDTD(ExternalID,
                                       SystemID)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseCtxtExternalEntity_rust(mut ctx: xmlParserCtxtPtr,
                                                     mut URL: *const xmlChar,
                                                     mut ID: *const xmlChar,
                                                     mut lst: *mut xmlNodePtr)
                                                     -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseCtxtExternalEntity(ctx,
                                                      URL,
                                                      ID,
                                                      lst)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseExternalEntity_rust(mut doc: xmlDocPtr,
                                                 mut sax: xmlSAXHandlerPtr,
                                                 mut user_data:
                                                 *mut libc::c_void,
                                                 mut depth: libc::c_int,
                                                 mut URL: *const xmlChar,
                                                 mut ID: *const xmlChar,
                                                 mut lst: *mut xmlNodePtr)
                                                 -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseExternalEntity(doc,
                                                  sax,
                                                  user_data,
                                                  depth,
                                                  URL,
                                                  ID,
                                                  lst)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseBalancedChunkMemory_rust(mut doc: xmlDocPtr,
                                                      mut sax:
                                                      xmlSAXHandlerPtr,
                                                      mut user_data:
                                                      *mut libc::c_void,
                                                      mut depth: libc::c_int,
                                                      mut string:
                                                      *const xmlChar,
                                                      mut lst: *mut xmlNodePtr)
                                                      -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseBalancedChunkMemory(doc,
                                                       sax
                                                      ,
                                                       user_data,
                                                       depth,
                                                       string,
                                                       lst)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseInNodeContext_rust(mut node: xmlNodePtr,
                                                mut data: *const libc::c_char,
                                                mut datalen: libc::c_int,
                                                mut options: libc::c_int,
                                                mut lst: *mut xmlNodePtr)
                                                -> xmlParserErrors {
     let res: xmlParserErrors  = unsafe{
         xmlParseInNodeContext(node,
                                                 data,
                                                 datalen,
                                                 options,
                                                 lst)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseBalancedChunkMemoryRecover_rust(mut doc:
                                                             xmlDocPtr,
                                                             mut sax:
                                                             xmlSAXHandlerPtr,
                                                             mut user_data:
                                                             *mut libc::c_void,
                                                             mut depth:
                                                             libc::c_int,
                                                             mut string:
                                                             *const xmlChar,
                                                             mut lst:
                                                             *mut xmlNodePtr,
                                                             mut recover:
                                                             libc::c_int)
                                                             -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlParseBalancedChunkMemoryRecover(doc,
                                                              sax,
                                                              user_data,
                                                              depth,
                                                              string,
                                                              lst,
                                                              recover)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSAXParseEntity_rust(mut sax: xmlSAXHandlerPtr,
                                            mut filename: *const libc::c_char)
                                            -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlSAXParseEntity(sax,filename)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseEntity_rust(mut filename: *const libc::c_char)
                                         -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlParseEntity(filename)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCreateEntityParserCtxt_rust(mut URL: *const xmlChar,
                                                    mut ID: *const xmlChar,
                                                    mut base: *const xmlChar)
                                                    -> xmlParserCtxtPtr {
     let res: xmlParserCtxtPtr  = unsafe{
         xmlCreateEntityParserCtxt(URL,
                                                     ID,
                                                     base)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCreateURLParserCtxt_rust(mut filename:
                                                 *const libc::c_char,
                                                 mut options: libc::c_int)
                                                 -> xmlParserCtxtPtr {
     let res: xmlParserCtxtPtr  = unsafe{
         xmlCreateURLParserCtxt(filename,options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCreateFileParserCtxt_rust(mut filename: *const libc::c_char) -> xmlParserCtxtPtr {
     let res: xmlParserCtxtPtr  = unsafe{
         xmlCreateFileParserCtxt(filename)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSAXParseFileWithData_rust(mut sax: xmlSAXHandlerPtr,
                                                  mut filename:
                                                  *const libc::c_char,
                                                  mut recovery: libc::c_int,
                                                  mut data: *mut libc::c_void)
                                                  -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlSAXParseFileWithData(sax,
                                                   filename,
                                                   recovery,
                                                   data)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSAXParseFile_rust(mut sax: xmlSAXHandlerPtr,
                                          mut filename: *const libc::c_char,
                                          mut recovery: libc::c_int)
                                          -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlSAXParseFile(sax,
                                           filename,
                                           recovery)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlRecoverDoc_rust(mut cur: *const xmlChar) -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlRecoverDoc(cur)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseFile_rust(mut filename: *const libc::c_char)
                                       -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlParseFile(filename)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlRecoverFile_rust(mut filename: *const libc::c_char)
                                         -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlRecoverFile(filename)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSetupParserForBuffer_rust(mut ctxt: xmlParserCtxtPtr,
                                                  mut buffer: *const xmlChar,
                                                  mut filename:
                                                  *const libc::c_char) {
     unsafe{
         xmlSetupParserForBuffer(ctxt, buffer, filename)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSAXUserParseFile_rust(mut sax: xmlSAXHandlerPtr,
                                              mut user_data: *mut libc::c_void,
                                              mut filename:
                                              *const libc::c_char)
                                              -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlSAXUserParseFile(sax, user_data, filename)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCreateMemoryParserCtxt_rust(mut buffer:
                                                    *const libc::c_char,
                                                    mut size: libc::c_int)
                                                    -> xmlParserCtxtPtr {
     let res: xmlParserCtxtPtr  = unsafe{
         xmlCreateMemoryParserCtxt_parser(buffer, size)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSAXParseMemoryWithData_rust(mut sax: xmlSAXHandlerPtr,
                                                    mut buffer:
                                                    *const libc::c_char,
                                                    mut size: libc::c_int,
                                                    mut recovery: libc::c_int,
                                                    mut data:
                                                    *mut libc::c_void)
                                                    -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlSAXParseMemoryWithData(sax,
                                                     buffer,
                                                     size,
                                                     recovery,
                                                     data)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSAXParseMemory_rust(mut sax: xmlSAXHandlerPtr,
                                            mut buffer: *const libc::c_char,
                                            mut size: libc::c_int,
                                            mut recovery: libc::c_int)
                                            -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlSAXParseMemory(sax,
                                             buffer,
                                             size,
                                             recovery)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseMemory_rust(mut buffer: *const libc::c_char,
                                         mut size: libc::c_int) -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlParseMemory(buffer,size)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlRecoverMemory_rust(mut buffer: *const libc::c_char,
                                           mut size: libc::c_int)
                                           -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlRecoverMemory(buffer,size)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSAXUserParseMemory_rust(mut sax: xmlSAXHandlerPtr,
                                                mut user_data:
                                                *mut libc::c_void,
                                                mut buffer:
                                                *const libc::c_char,
                                                mut size: libc::c_int)
                                                -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlSAXUserParseMemory(sax,
                                                 user_data,
                                                 buffer,
                                                 size)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCreateDocParserCtxt_rust(mut cur: *const xmlChar) -> xmlParserCtxtPtr {
     let res: xmlParserCtxtPtr  = unsafe{
         xmlCreateDocParserCtxt(cur)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSAXParseDoc_rust(mut sax: xmlSAXHandlerPtr,
                                         mut cur: *const xmlChar,
                                         mut recovery: libc::c_int)
                                         -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlSAXParseDoc(sax,
                                          cur,
                                          recovery)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlParseDoc_rust(mut cur: *const xmlChar) -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlParseDoc(cur)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlSetEntityReferenceFunc_rust(mut func: xmlEntityReferenceFunc) {
     unsafe{
         xmlSetEntityReferenceFunc(func)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlInitParser_rust() {
     unsafe{
         xmlInitParser_parser()
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCleanupParser_rust() {
     unsafe{
         xmlCleanupParser()
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCtxtReset_rust(mut ctxt: xmlParserCtxtPtr) {
     unsafe{
         xmlCtxtReset_parser(ctxt)
     };
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCtxtResetPush_rust(mut ctxt: xmlParserCtxtPtr,
                                           mut chunk: *const libc::c_char,
                                           mut size: libc::c_int,
                                           mut filename: *const libc::c_char,
                                           mut encoding: *const libc::c_char)
                                           -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlCtxtResetPush(ctxt, chunk, size, filename, encoding)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCtxtUseOptions_rust(mut ctxt: xmlParserCtxtPtr,
                                            mut options: libc::c_int)
                                            -> libc::c_int {
     let res: libc::c_int  = unsafe{
         xmlCtxtUseOptions(ctxt,
                                             options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlReadDoc_rust(mut cur: *const xmlChar,
                                     mut URL: *const libc::c_char,
                                     mut encoding: *const libc::c_char,
                                     mut options: libc::c_int) -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlReadDoc(cur,
                                      URL,
                                      encoding,
                                      options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlReadFile_rust(mut filename: *const libc::c_char,
                                      mut encoding: *const libc::c_char,
                                      mut options: libc::c_int) -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlReadFile(filename,
                                       encoding,
                                       options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlReadMemory_rust(mut buffer: *const libc::c_char,
                                        mut size: libc::c_int,
                                        mut URL: *const libc::c_char,
                                        mut encoding: *const libc::c_char,
                                        mut options: libc::c_int)
                                        -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlReadMemory(buffer,
                                         size,
                                         URL,
                                         encoding,
                                         options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlReadFd_rust(mut fd: libc::c_int,
                                    mut URL: *const libc::c_char,
                                    mut encoding: *const libc::c_char,
                                    mut options: libc::c_int) -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlReadFd(fd,
                                     URL,
                                     encoding,
                                     options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlReadIO_rust(mut ioread: xmlInputReadCallback,
                                    mut ioclose: xmlInputCloseCallback,
                                    mut ioctx: *mut libc::c_void,
                                    mut URL: *const libc::c_char,
                                    mut encoding: *const libc::c_char,
                                    mut options: libc::c_int) -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlReadIO(ioread,
                                     ioclose,
                                     ioctx,
                                     URL,
                                     encoding,
                                     options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCtxtReadDoc_rust(mut ctxt: xmlParserCtxtPtr,
                                         mut cur: *const xmlChar,
                                         mut URL: *const libc::c_char,
                                         mut encoding: *const libc::c_char,
                                         mut options: libc::c_int)
                                         -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlCtxtReadDoc(ctxt,
                                          cur,
                                          URL,
                                          encoding,
                                          options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCtxtReadFile_rust(mut ctxt: xmlParserCtxtPtr,
                                          mut filename: *const libc::c_char,
                                          mut encoding: *const libc::c_char,
                                          mut options: libc::c_int)
                                          -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlCtxtReadFile(ctxt,
                                           filename,
                                           encoding,
                                           options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCtxtReadMemory_rust(mut ctxt: xmlParserCtxtPtr,
                                            mut buffer: *const libc::c_char,
                                            mut size: libc::c_int,
                                            mut URL: *const libc::c_char,
                                            mut encoding: *const libc::c_char,
                                            mut options: libc::c_int)
                                            -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlCtxtReadMemory(ctxt,
                                             buffer,
                                             size,
                                             URL,
                                             encoding,
                                             options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCtxtReadFd_rust(mut ctxt: xmlParserCtxtPtr,
                                        mut fd: libc::c_int,
                                        mut URL: *const libc::c_char,
                                        mut encoding: *const libc::c_char,
                                        mut options: libc::c_int)
                                        -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlCtxtReadFd(ctxt,
                                         fd,
                                         URL,
                                         encoding,
                                         options)
     };
     return res;
 
 }
 
 #[no_mangle]
 pub unsafe extern "C" fn xmlCtxtReadIO_rust(mut ctxt: xmlParserCtxtPtr,
                                        mut ioread: xmlInputReadCallback,
                                        mut ioclose: xmlInputCloseCallback,
                                        mut ioctx: *mut libc::c_void,
                                        mut URL: *const libc::c_char,
                                        mut encoding: *const libc::c_char,
                                        mut options: libc::c_int)
                                        -> xmlDocPtr {
     let res: xmlDocPtr  = unsafe{
         xmlCtxtReadIO(ctxt,
                                         ioread,
                                         ioclose,
                                         ioctx,
                                         URL,
                                         encoding,
                                         options)
     };
     return res;
 
 }