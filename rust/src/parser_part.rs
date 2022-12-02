unsafe fn xmlParserEntityCheck(mut ctxt: xmlParserCtxtPtr,
    mut size: size_t,
    mut ent: xmlEntityPtr,
    mut replacement: size_t)
    -> i32 {

let mut safe_ctxt  = unsafe{&mut *ctxt};
let mut safe_ent = unsafe{&mut *ent};

let mut consumed: size_t = 0 as i32 as size_t;
let mut i: i32 = 0;
if ctxt.is_null() || (safe_ctxt).options & XML_PARSE_HUGE as i32 != 0
{
return 0 as i32
}
if (safe_ctxt).lastError.code == XML_ERR_ENTITY_LOOP as i32 {
return 1 as i32
}
/*
* This may look absurd but is needed to detect
* entities problems
*/
if !ent.is_null() &&
(safe_ent).etype as u32 !=
XML_INTERNAL_PREDEFINED_ENTITY as i32 as u32
&& !(safe_ent).content.is_null() && (safe_ent).checked == 0 as i32
&& (safe_ctxt).errNo != XML_ERR_ENTITY_LOOP as i32 {
let mut oldnbent: u64 = (safe_ctxt).nbentities;
let mut diff: u64 = 0;
let mut rep: *mut xmlChar = 0 as *mut xmlChar;
(safe_ent).checked = 1 as i32;
(safe_ctxt).depth += 1;
unsafe {
rep =
xmlStringDecodeEntities(ctxt, (safe_ent).content, 1 as i32,
                    0 as i32 as xmlChar,
                    0 as i32 as xmlChar,
                    0 as i32 as xmlChar);
}
(safe_ctxt).depth -= 1;
if rep.is_null() ||
(safe_ctxt).errNo == XML_ERR_ENTITY_LOOP as i32 {
unsafe {
*(safe_ent).content.offset(0 as i32 as isize) =
0 as i32 as xmlChar;
}
}
diff =
(safe_ctxt).nbentities.wrapping_sub(oldnbent).wrapping_add(1 as
i32
as
u64);
if diff >
(2147483647 as i32 / 2 as i32) as u64
{
diff =
(2147483647 as i32 / 2 as i32) as
u64
}
(safe_ent).checked =
diff.wrapping_mul(2 as i32 as u64) as
i32;
if !rep.is_null() {
if !xmlStrchr_safe(rep, '<' as i32 as xmlChar).is_null() {
(safe_ent).checked |= 1 as i32
}
xmlFree_safe(rep as
*mut ());
rep = 0 as *mut xmlChar
}
}
/*
* Prevent entity exponential check, not just replacement while
* parsing the DTD
* The check is potentially costly so do that only once in a thousand
*/
if (safe_ctxt).instate as i32 == XML_PARSER_DTD as i32 &&
(safe_ctxt).nbentities > 10000 as i32 as u64 &&
(safe_ctxt).nbentities.wrapping_rem(1024 as i32 as
u64) ==
0 as i32 as u64 {
i = 0 as i32;
while i < (safe_ctxt).inputNr {
consumed = unsafe{
(consumed as
u64).wrapping_add((**(safe_ctxt).inputTab.offset(i
as
isize)).consumed.wrapping_add((**(safe_ctxt).inputTab.offset(i
as
isize)).cur.offset_from((**(safe_ctxt).inputTab.offset(i
as
isize)).base)
as
i64
as
u64))
as size_t as size_t};
i += 1
}
if (safe_ctxt).nbentities >
consumed.wrapping_mul(10 as i32 as u64) {
unsafe {
xmlFatalErr(ctxt, XML_ERR_ENTITY_LOOP, 0 as *const i8);
}
(safe_ctxt).instate = XML_PARSER_EOF;
return 1 as i32
}
consumed = 0 as i32 as size_t
}
if replacement != 0 as i32 as u64 {
if replacement < 10000000 as i32 as u64 {
return 0 as i32
}
/*
* If the volume of entity copy reaches 10 times the
* amount of parsed data and over the large text threshold
* then that's very likely to be an abuse.
*/
if !(safe_ctxt).input.is_null() {
consumed =
unsafe{(*(safe_ctxt).input).consumed.wrapping_add((*(safe_ctxt).input).cur.offset_from((*(safe_ctxt).input).base)
as i64 as u64)}
}
consumed =
(consumed as u64).wrapping_add((safe_ctxt).sizeentities) as
size_t as size_t;
if replacement <
(10 as i32 as u64).wrapping_mul(consumed) {
return 0 as i32
}
} else if size != 0 as i32 as u64 {
/*
* Do the check based on the replacement size of the entity
*/
if size < 1000 as i32 as u64 {
return 0 as i32
}
/*
* A limit on the amount of text data reasonably used
*/
if !(safe_ctxt).input.is_null() {
consumed =
unsafe{(*(safe_ctxt).input).consumed.wrapping_add((*(safe_ctxt).input).cur.offset_from((*(safe_ctxt).input).base)
as i64 as
u64)}
}
consumed =
(consumed as u64).wrapping_add((safe_ctxt).sizeentities) as
size_t as size_t;
if size < (10 as i32 as u64).wrapping_mul(consumed)
&&
(safe_ctxt).nbentities.wrapping_mul(3 as i32 as
u64) <
(10 as i32 as u64).wrapping_mul(consumed)
{
return 0 as i32
}
} else if !ent.is_null() {
/*
* use the number of parsed entities in the replacement
*/
size = ((safe_ent).checked / 2 as i32) as size_t;
/*
* The amount of data parsed counting entities size only once
*/
if !(safe_ctxt).input.is_null() {
consumed =
unsafe{(*(safe_ctxt).input).consumed.wrapping_add((*(safe_ctxt).input).cur.offset_from((*(safe_ctxt).input).base)
as i64 as
u64)}
}
consumed =
(consumed as u64).wrapping_add((safe_ctxt).sizeentities) as
size_t as size_t;
/*
* Check the density of entities for the amount of data
* knowing an entity reference will take at least 3 bytes
*/
if size.wrapping_mul(3 as i32 as u64) <
consumed.wrapping_mul(10 as i32 as u64) {
return 0 as i32
}
} else if (safe_ctxt).lastError.code !=
XML_ERR_UNDECLARED_ENTITY as i32 &&
(safe_ctxt).lastError.code !=
XML_WAR_UNDECLARED_ENTITY as i32 ||
(safe_ctxt).nbentities <= 10000 as i32 as u64
{
return 0 as i32
}
unsafe {
xmlFatalErr(ctxt, XML_ERR_ENTITY_LOOP, 0 as *const i8);
}
return 1 as i32;
}

pub static mut xmlParserMaxDepth: u32 =
256 as i32 as u32;
/*
* List of XML prefixed PI allowed by W3C specs
*/
static mut xmlW3CPIs: [*const i8; 3] =
[b"xml-stylesheet\x00" as *const u8 as *const i8,
b"xml-model\x00" as *const u8 as *const i8,
0 as *const i8];
/* ***********************************************************************
*									*
*		Some factorized error routines				*
*									*
************************************************************************/
/* *
* xmlErrAttributeDup:
* @ctxt:  an XML parser context
* @prefix:  the attribute prefix
* @localname:  the attribute localname
*
* Handle a redefinition of attribute error
*/
unsafe fn xmlErrAttributeDup(mut ctxt: xmlParserCtxtPtr,
  mut prefix: *const xmlChar,
  mut localname: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
if !ctxt.is_null() {
(safe_ctxt).errNo = XML_ERR_ATTRIBUTE_REDEFINED as i32
}
if prefix.is_null() {
unsafe {
__xmlRaiseError(None, None, 0 as *mut (),
        ctxt as *mut (), 0 as *mut (),
        XML_FROM_PARSER as i32,
        XML_ERR_ATTRIBUTE_REDEFINED as i32,
        XML_ERR_FATAL, 0 as *const i8,
        0 as i32, localname as *const i8,
        0 as *const i8, 0 as *const i8,
        0 as i32, 0 as i32,
        b"Attribute %s redefined\n\x00" as *const u8 as
            *const i8, localname);
}
} else {
unsafe {
__xmlRaiseError(None, None, 0 as *mut (),
        ctxt as *mut (), 0 as *mut (),
        XML_FROM_PARSER as i32,
        XML_ERR_ATTRIBUTE_REDEFINED as i32,
        XML_ERR_FATAL, 0 as *const i8,
        0 as i32, prefix as *const i8,
        localname as *const i8,
        0 as *const i8, 0 as i32,
        0 as i32,
        b"Attribute %s:%s redefined\n\x00" as *const u8 as
            *const i8, prefix, localname);
}
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as i32;
if (safe_ctxt).recovery == 0 as i32 {
(safe_ctxt).disableSAX = 1 as i32
}
};
}
/* *
* xmlFatalErr:
* @ctxt:  an XML parser context
* @error:  the error number
* @extra:  extra information string
*
* Handle a fatal parser error, i.e. violating Well-Formedness constraints
*/
unsafe fn xmlFatalErr(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut info: *const i8) {

let mut safe_ctxt  = unsafe{&mut *ctxt};

let mut errmsg: *const i8 = 0 as *const i8;
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
match error as u32 {
6 => {
errmsg =
b"CharRef: invalid hexadecimal value\x00" as *const u8 as
*const i8
}
7 => {
errmsg =
b"CharRef: invalid decimal value\x00" as *const u8 as
*const i8
}
8 => {
errmsg =
b"CharRef: invalid value\x00" as *const u8 as
*const i8
}
1 => {
errmsg = b"internal error\x00" as *const u8 as *const i8
}
18 => {
errmsg =
b"PEReference at end of document\x00" as *const u8 as
*const i8
}
19 => {
errmsg =
b"PEReference in prolog\x00" as *const u8 as
*const i8
}
20 => {
errmsg =
b"PEReference in epilog\x00" as *const u8 as
*const i8
}
24 => {
errmsg =
b"PEReference: no name\x00" as *const u8 as
*const i8
}
25 => {
errmsg =
b"PEReference: expecting \';\'\x00" as *const u8 as
*const i8
}
89 => {
errmsg =
b"Detected an entity reference loop\x00" as *const u8 as
*const i8
}
36 => {
errmsg =
b"EntityValue: \" or \' expected\x00" as *const u8 as
*const i8
}
88 => {
errmsg =
b"PEReferences forbidden in internal subset\x00" as *const u8
as *const i8
}
37 => {
errmsg =
b"EntityValue: \" or \' expected\x00" as *const u8 as
*const i8
}
39 => {
errmsg =
b"AttValue: \" or \' expected\x00" as *const u8 as
*const i8
}
38 => {
errmsg =
b"Unescaped \'<\' not allowed in attributes values\x00" as
*const u8 as *const i8
}
43 => {
errmsg =
b"SystemLiteral \" or \' expected\x00" as *const u8 as
*const i8
}
44 => {
errmsg =
b"Unfinished System or Public ID \" or \' expected\x00" as
*const u8 as *const i8
}
62 => {
errmsg =
b"Sequence \']]>\' not allowed in content\x00" as *const u8 as
*const i8
}
70 => {
errmsg =
b"SYSTEM or PUBLIC, the URI is missing\x00" as *const u8 as
*const i8
}
71 => {
errmsg =
b"PUBLIC, the Public Identifier is missing\x00" as *const u8
as *const i8
}
80 => {
errmsg =
b"Comment must not contain \'--\' (double-hyphen)\x00" as
*const u8 as *const i8
}
46 => {
errmsg =
b"xmlParsePI : no target name\x00" as *const u8 as
*const i8
}
64 => {
errmsg =
b"Invalid PI name\x00" as *const u8 as *const i8
}
48 => {
errmsg =
b"NOTATION: Name expected here\x00" as *const u8 as
*const i8
}
49 => {
errmsg =
b"\'>\' required to close NOTATION declaration\x00" as
*const u8 as *const i8
}
84 => {
errmsg =
b"Entity value required\x00" as *const u8 as
*const i8
}
92 => {
errmsg =
b"Fragment not allowed\x00" as *const u8 as
*const i8
}
50 => {
errmsg =
b"\'(\' required to start ATTLIST enumeration\x00" as
*const u8 as *const i8
}
67 => {
errmsg =
b"NmToken expected in ATTLIST enumeration\x00" as *const u8 as
*const i8
}
51 => {
errmsg =
b"\')\' required to finish ATTLIST enumeration\x00" as
*const u8 as *const i8
}
52 => {
errmsg =
b"MixedContentDecl : \'|\' or \')*\' expected\x00" as
*const u8 as *const i8
}
69 => {
errmsg =
b"MixedContentDecl : \'#PCDATA\' expected\x00" as *const u8 as
*const i8
}
54 => {
errmsg =
b"ContentDecl : Name or \'(\' expected\x00" as *const u8 as
*const i8
}
55 => {
errmsg =
b"ContentDecl : \',\' \'|\' or \')\' expected\x00" as
*const u8 as *const i8
}
21 => {
errmsg =
b"PEReference: forbidden within markup decl in internal subset\x00"
as *const u8 as *const i8
}
73 => {
errmsg = b"expected \'>\'\x00" as *const u8 as *const i8
}
83 => {
errmsg =
b"XML conditional section \'[\' expected\x00" as *const u8 as
*const i8
}
60 => {
errmsg =
b"Content error in the external subset\x00" as *const u8 as
*const i8
}
95 => {
errmsg =
b"conditional section INCLUDE or IGNORE keyword expected\x00"
as *const u8 as *const i8
}
59 => {
errmsg =
b"XML conditional section not closed\x00" as *const u8 as
*const i8
}
56 => {
errmsg =
b"Text declaration \'<?xml\' required\x00" as *const u8 as
*const i8
}
57 => {
errmsg =
b"parsing XML declaration: \'?>\' expected\x00" as *const u8
as *const i8
}
82 => {
errmsg =
b"external parsed entities cannot be standalone\x00" as
*const u8 as *const i8
}
23 => {
errmsg =
b"EntityRef: expecting \';\'\x00" as *const u8 as
*const i8
}
61 => {
errmsg =
b"DOCTYPE improperly terminated\x00" as *const u8 as
*const i8
}
74 => {
errmsg =
b"EndTag: \'</\' not found\x00" as *const u8 as
*const i8
}
75 => {
errmsg = b"expected \'=\'\x00" as *const u8 as *const i8
}
34 => {
errmsg =
b"String not closed expecting \" or \'\x00" as *const u8 as
*const i8
}
33 => {
errmsg =
b"String not started expecting \' or \"\x00" as *const u8 as
*const i8
}
79 => {
errmsg =
b"Invalid XML encoding name\x00" as *const u8 as
*const i8
}
78 => {
errmsg =
b"standalone accepts only \'yes\' or \'no\'\x00" as *const u8
as *const i8
}
4 => {
errmsg =
b"Document is empty\x00" as *const u8 as *const i8
}
5 => {
errmsg =
b"Extra content at the end of the document\x00" as *const u8
as *const i8
}
85 => {
errmsg =
b"chunk is not well balanced\x00" as *const u8 as
*const i8
}
86 => {
errmsg =
b"extra content at the end of well balanced chunk\x00" as
*const u8 as *const i8
}
96 => {
errmsg =
b"Malformed declaration expecting version\x00" as *const u8 as
*const i8
}
110 => {
errmsg =
b"Name too long use XML_PARSE_HUGE option\x00" as *const u8 as
*const i8
}
_ => {
errmsg =
b"Unregistered error message\x00" as *const u8 as
*const i8
}
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as i32 }
if info.is_null() {
unsafe{
__xmlRaiseError(None, None, 0 as *mut (),
        ctxt as *mut (), 0 as *mut (),
        XML_FROM_PARSER as i32, error as i32,
        XML_ERR_FATAL, 0 as *const i8,
        0 as i32, info, 0 as *const i8,
        0 as *const i8, 0 as i32,
        0 as i32,
        b"%s\n\x00" as *const u8 as *const i8,
        errmsg);
}
} else {
unsafe{
__xmlRaiseError(None, None, 0 as *mut (),
        ctxt as *mut (), 0 as *mut (),
        XML_FROM_PARSER as i32, error as i32,
        XML_ERR_FATAL, 0 as *const i8,
        0 as i32, info, 0 as *const i8,
        0 as *const i8, 0 as i32,
        0 as i32,
        b"%s: %s\n\x00" as *const u8 as *const i8,
        errmsg, info);
}
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as i32;
if (safe_ctxt).recovery == 0 as i32 {
(safe_ctxt).disableSAX = 1 as i32
}
};
}
/* *
* xmlFatalErrMsg:
* @ctxt:  an XML parser context
* @error:  the error number
* @msg:  the error message
*
* Handle a fatal parser error, i.e. violating Well-Formedness constraints
*/
unsafe fn xmlFatalErrMsg(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const i8) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as i32 }
unsafe{
__xmlRaiseError(None, None, 0 as *mut (),
    ctxt as *mut (), 0 as *mut (),
    XML_FROM_PARSER as i32, error as i32,
    XML_ERR_FATAL, 0 as *const i8, 0 as i32,
    0 as *const i8, 0 as *const i8,
    0 as *const i8, 0 as i32,
    0 as i32,
    b"%s\x00" as *const u8 as *const i8, msg);
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as i32;
if (safe_ctxt).recovery == 0 as i32 {
(safe_ctxt).disableSAX = 1 as i32
}
};
}
/* *
* xmlWarningMsg:
* @ctxt:  an XML parser context
* @error:  the error number
* @msg:  the error message
* @str1:  extra data
* @str2:  extra data
*
* Handle a warning.
*/
unsafe fn xmlWarningMsg(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const i8,
mut str1: *const xmlChar,
mut str2: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
let mut schannel: xmlStructuredErrorFunc = None;
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
if !ctxt.is_null() && !(safe_ctxt).sax.is_null() &&
unsafe{(*(safe_ctxt).sax).initialized == 0xdeedbeaf as u32} {
schannel = unsafe{(*(safe_ctxt).sax).serror};
}
if !ctxt.is_null() {
unsafe{
__xmlRaiseError(schannel,
        if !(safe_ctxt).sax.is_null() {
            (*(safe_ctxt).sax).warning
        } else { None }, (safe_ctxt).userData,
        ctxt as *mut (), 0 as *mut (),
        XML_FROM_PARSER as i32, error as i32,
        XML_ERR_WARNING, 0 as *const i8,
        0 as i32, str1 as *const i8,
        str2 as *const i8, 0 as *const i8,
        0 as i32, 0 as i32, msg,
        str1 as *const i8,
        str2 as *const i8);
}
} else {
unsafe{
__xmlRaiseError(schannel, None, 0 as *mut (),
        ctxt as *mut (), 0 as *mut (),
        XML_FROM_PARSER as i32, error as i32,
        XML_ERR_WARNING, 0 as *const i8,
        0 as i32, str1 as *const i8,
        str2 as *const i8, 0 as *const i8,
        0 as i32, 0 as i32, msg,
        str1 as *const i8,
        str2 as *const i8);
}
};
}
/* *
* xmlValidityError:
* @ctxt:  an XML parser context
* @error:  the error number
* @msg:  the error message
* @str1:  extra data
*
* Handle a validity error.
*/
unsafe fn xmlValidityError(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const i8,
mut str1: *const xmlChar,
mut str2: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
let mut schannel: xmlStructuredErrorFunc = None;
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
if !ctxt.is_null() {
(safe_ctxt).errNo = error as i32;
if !(safe_ctxt).sax.is_null() &&
unsafe{(*(safe_ctxt).sax).initialized == 0xdeedbeaf as u32} {
schannel = unsafe{(*(safe_ctxt).sax).serror};
}
}
if !ctxt.is_null() {
unsafe {
__xmlRaiseError(schannel, (safe_ctxt).vctxt.error, (safe_ctxt).vctxt.userData,
        ctxt as *mut (), 0 as *mut (),
        XML_FROM_DTD as i32, error as i32,
        XML_ERR_ERROR, 0 as *const i8,
        0 as i32, str1 as *const i8,
        str2 as *const i8, 0 as *const i8,
        0 as i32, 0 as i32, msg,
        str1 as *const i8,
        str2 as *const i8);
}
(safe_ctxt).valid = 0 as i32
} else {
unsafe{
__xmlRaiseError(schannel, None, 0 as *mut (),
        ctxt as *mut (), 0 as *mut (),
        XML_FROM_DTD as i32, error as i32,
        XML_ERR_ERROR, 0 as *const i8,
        0 as i32, str1 as *const i8,
        str2 as *const i8, 0 as *const i8,
        0 as i32, 0 as i32, msg,
        str1 as *const i8,
        str2 as *const i8);
}
};
}
/* *
* xmlFatalErrMsgInt:
* @ctxt:  an XML parser context
* @error:  the error number
* @msg:  the error message
* @val:  an integer value
*
* Handle a fatal parser error, i.e. violating Well-Formedness constraints
*/
unsafe fn xmlFatalErrMsgInt(mut ctxt: xmlParserCtxtPtr,
 mut error: xmlParserErrors,
 mut msg: *const i8,
 mut val: i32) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as i32 }
unsafe{
__xmlRaiseError(None, None, 0 as *mut (),
    ctxt as *mut (), 0 as *mut (),
    XML_FROM_PARSER as i32, error as i32,
    XML_ERR_FATAL, 0 as *const i8, 0 as i32,
    0 as *const i8, 0 as *const i8,
    0 as *const i8, val, 0 as i32, msg,
    val);
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as i32;
if (safe_ctxt).recovery == 0 as i32 {
(safe_ctxt).disableSAX = 1 as i32
}
};
}
/* *
* xmlFatalErrMsgStrIntStr:
* @ctxt:  an XML parser context
* @error:  the error number
* @msg:  the error message
* @str1:  an string info
* @val:  an integer value
* @str2:  an string info
*
* Handle a fatal parser error, i.e. violating Well-Formedness constraints
*/
unsafe fn xmlFatalErrMsgStrIntStr(mut ctxt: xmlParserCtxtPtr,
       mut error: xmlParserErrors,
       mut msg: *const i8,
       mut str1: *const xmlChar,
       mut val: i32,
       mut str2: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as i32 }
unsafe{
__xmlRaiseError(None, None, 0 as *mut (),
    ctxt as *mut (), 0 as *mut (),
    XML_FROM_PARSER as i32, error as i32,
    XML_ERR_FATAL, 0 as *const i8, 0 as i32,
    str1 as *const i8, str2 as *const i8,
    0 as *const i8, val, 0 as i32, msg,
    str1, val, str2);
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as i32;
if (safe_ctxt).recovery == 0 as i32 {
(safe_ctxt).disableSAX = 1 as i32
}
};
}
/* *
* xmlFatalErrMsgStr:
* @ctxt:  an XML parser context
* @error:  the error number
* @msg:  the error message
* @val:  a string value
*
* Handle a fatal parser error, i.e. violating Well-Formedness constraints
*/
unsafe fn xmlFatalErrMsgStr(mut ctxt: xmlParserCtxtPtr,
 mut error: xmlParserErrors,
 mut msg: *const i8,
 mut val: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as i32 }
unsafe{
__xmlRaiseError(None, None, 0 as *mut (),
    ctxt as *mut (), 0 as *mut (),
    XML_FROM_PARSER as i32, error as i32,
    XML_ERR_FATAL, 0 as *const i8, 0 as i32,
    val as *const i8, 0 as *const i8,
    0 as *const i8, 0 as i32,
    0 as i32, msg, val);
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as i32;
if (safe_ctxt).recovery == 0 as i32 {
(safe_ctxt).disableSAX = 1 as i32
}
};
}
/* *
* xmlErrMsgStr:
* @ctxt:  an XML parser context
* @error:  the error number
* @msg:  the error message
* @val:  a string value
*
* Handle a non fatal parser error
*/
unsafe fn xmlErrMsgStr(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const i8,
mut val: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as i32 }
unsafe{
__xmlRaiseError(None, None, 0 as *mut (),
    ctxt as *mut (), 0 as *mut (),
    XML_FROM_PARSER as i32, error as i32,
    XML_ERR_ERROR, 0 as *const i8, 0 as i32,
    val as *const i8, 0 as *const i8,
    0 as *const i8, 0 as i32,
    0 as i32, msg, val);
}
}
/* *
* xmlNsErr:
* @ctxt:  an XML parser context
* @error:  the error number
* @msg:  the message
* @info1:  extra information string
* @info2:  extra information string
*
* Handle a fatal parser error, i.e. violating Well-Formedness constraints
*/
unsafe fn xmlNsErr(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const i8,
mut info1: *const xmlChar,
mut info2: *const xmlChar,
mut info3: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as i32 }
unsafe{
__xmlRaiseError(None, None, 0 as *mut (),
    ctxt as *mut (), 0 as *mut (),
    XML_FROM_NAMESPACE as i32, error as i32,
    XML_ERR_ERROR, 0 as *const i8, 0 as i32,
    info1 as *const i8,
    info2 as *const i8,
    info3 as *const i8, 0 as i32,
    0 as i32, msg, info1, info2, info3);
}
if !ctxt.is_null() { (safe_ctxt).nsWellFormed = 0 as i32 };
}
/* *
* xmlNsWarn
* @ctxt:  an XML parser context
* @error:  the error number
* @msg:  the message
* @info1:  extra information string
* @info2:  extra information string
*
* Handle a namespace warning error
*/
unsafe fn xmlNsWarn(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const i8,
mut info1: *const xmlChar,
mut info2: *const xmlChar,
mut info3: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as i32 &&
(safe_ctxt).instate as i32 == XML_PARSER_EOF as i32 {
return
}
unsafe{
__xmlRaiseError(None, None, 0 as *mut (),
    ctxt as *mut (), 0 as *mut (),
    XML_FROM_NAMESPACE as i32, error as i32,
    XML_ERR_WARNING, 0 as *const i8,
    0 as i32, info1 as *const i8,
    info2 as *const i8,
    info3 as *const i8, 0 as i32,
    0 as i32, msg, info1, info2, info3);
}
}
/* ***********************************************************************
*									*
*		Library wide options					*
*									*
************************************************************************/
/* *
* xmlHasFeature:
* @feature: the feature to be examined
*
* Examines if the library has been compiled with a given feature.
*
* Returns a non-zero value if the feature exist, otherwise zero.
* Returns zero (0) if the feature does not exist or an unknown
* unknown feature is requested, non-zero otherwise.
*/

pub unsafe fn xmlHasFeature(mut feature: xmlFeature)
 -> i32 {
match feature as u32 {
1 => { return 1 as i32 }
2 => { return 1 as i32 }
3 => { return 1 as i32 }
4 => { return 1 as i32 }
5 => { return 1 as i32 }
6 => { return 1 as i32 }
7 => { return 1 as i32 }
8 => { return 1 as i32 }
9 => { return 1 as i32 }
10 => { return 1 as i32 }
11 => { return 1 as i32 }
12 => { return 1 as i32 }
13 => { return 1 as i32 }
14 => { return 1 as i32 }
15 => { return 1 as i32 }
16 => { return 1 as i32 }
17 => { return 1 as i32 }
18 => { return 1 as i32 }
19 => { return 1 as i32 }
20 => { return 1 as i32 }
21 => { return 1 as i32 }
22 => { return 1 as i32 }
23 => { return 1 as i32 }
24 => { return 0 as i32 }
25 => { return 1 as i32 }
26 => { return 1 as i32 }
27 => { return 1 as i32 }
28 => { return 1 as i32 }
29 => { return 0 as i32 }
30 => { return 0 as i32 }
31 => { return 1 as i32 }
33 => { return 1 as i32 }
32 => { return 0 as i32 }
_ => { }
}
return 0 as i32;
}
/* ***********************************************************************
*									*
*		SAX2 defaulted attributes handling			*
*									*
************************************************************************/
/* *
* xmlDetectSAX2:
* @ctxt:  an XML parser context
*
* Do the SAX2 detection and specific initialization
*/
unsafe fn xmlDetectSAX2(mut ctxt: xmlParserCtxtPtr) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
let mut sax: xmlSAXHandlerPtr = 0 as *mut xmlSAXHandler;
if ctxt.is_null() { return }
sax = (safe_ctxt).sax;
let mut safe_sax  = unsafe{&mut *sax};
match () {
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]  _ => {
if !sax.is_null() && (safe_sax).initialized == 0xdeedbeaf as u32 &&
((safe_sax).startElementNs.is_some() || (safe_sax).endElementNs.is_some()
||
(safe_sax).startElement.is_none() && (safe_sax).endElement.is_none())
{
(safe_ctxt).sax2 = 1 as i32;
}
}
#[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))] _ => {
(safe_ctxt).sax2 = 1 as i32;
}
};

/* LIBXML_SAX1_ENABLED */
(safe_ctxt).str_xml =
xmlDictLookup_safe((safe_ctxt).dict,
       b"xml\x00" as *const u8 as *const i8 as
           *mut xmlChar, 3 as i32);
(safe_ctxt).str_xmlns =
xmlDictLookup_safe((safe_ctxt).dict,
       b"xmlns\x00" as *const u8 as *const i8 as
           *mut xmlChar, 5 as i32);
(safe_ctxt).str_xml_ns =
xmlDictLookup_safe((safe_ctxt).dict,
       b"http://www.w3.org/XML/1998/namespace\x00" as *const u8
           as *const i8 as *const xmlChar,
       36 as i32);
if (safe_ctxt).str_xml.is_null() || (safe_ctxt).str_xmlns.is_null() ||
(safe_ctxt).str_xml_ns.is_null() {
unsafe {
unsafe{xmlErrMemory(ctxt, 0 as *const i8)};
}
};
}
/* array of localname/prefix/values/external */
/* *
* xmlAttrNormalizeSpace:
* @src: the source string
* @dst: the target string
*
* Normalize the space in non CDATA attribute values:
* If the attribute type is not CDATA, then the XML processor MUST further
* process the normalized attribute value by discarding any leading and
* trailing space (#x20) characters, and by replacing sequences of space
* (#x20) characters by a single space (#x20) character.
* Note that the size of dst need to be at least src, and if one doesn't need
* to preserve dst (and it doesn't come from a dictionary or read-only) then
* passing src as dst is just fine.
*
* Returns a pointer to the normalized value (dst) or NULL if no conversion
*         is needed.
*/
unsafe fn xmlAttrNormalizeSpace(mut src: *const xmlChar,
     mut dst: *mut xmlChar)
     -> *mut xmlChar {
if src.is_null() || dst.is_null() { return 0 as *mut xmlChar }
while unsafe{*src} as i32 == 0x20 as i32 { src = unsafe{src.offset(1)} }
while unsafe{*src} as i32 != 0 as i32 {
if unsafe{*src} as i32 == 0x20 as i32 {
while unsafe{*src} as i32 == 0x20 as i32 {
src = unsafe{src.offset(1)}
}
if unsafe{*src} as i32 != 0 as i32 {
let fresh0 = dst;
dst = unsafe{dst.offset(1)};
unsafe{*fresh0 = 0x20 as i32 as xmlChar};
}
} else {
let fresh1 = src;
src = unsafe{src.offset(1)};
let fresh2 = dst;
dst = unsafe{dst.offset(1)};
unsafe{*fresh2 = *fresh1};
}
}
unsafe{*dst = 0 as i32 as xmlChar};
if dst == src as *mut xmlChar { return 0 as *mut xmlChar }
return dst;
}
/* *
* xmlAttrNormalizeSpace2:
* @src: the source string
*
* Normalize the space in non CDATA attribute values, a slightly more complex
* front end to avoid allocation problems when running on attribute values
* coming from the input.
*
* Returns a pointer to the normalized value (dst) or NULL if no conversion
*         is needed.
*/
unsafe fn xmlAttrNormalizeSpace2(mut ctxt: xmlParserCtxtPtr,
      mut src: *mut xmlChar,
      mut len: *mut i32)
      -> *const xmlChar {
let mut i: i32 = 0;
let mut remove_head: i32 = 0 as i32;
let mut need_realloc: i32 = 0 as i32;
let mut cur: *const xmlChar = 0 as *const xmlChar;
if ctxt.is_null() || src.is_null() || len.is_null() {
return 0 as *const xmlChar
}
i = unsafe{*len};
if i <= 0 as i32 { return 0 as *const xmlChar }
cur = src;
while unsafe{*cur} as i32 == 0x20 as i32 {
cur = unsafe{cur.offset(1)};
remove_head += 1
}
while unsafe{*cur} as i32 != 0 as i32 {
if unsafe{*cur} as i32 == 0x20 as i32 {
cur = unsafe{cur.offset(1)};
if !(unsafe{*cur} as i32 == 0x20 as i32 ||
unsafe{*cur} as i32 == 0 as i32) {
continue ;
}
need_realloc = 1 as i32;
break ;
} else { cur = unsafe{cur.offset(1)} }
}
if need_realloc != 0 {
let mut ret: *mut xmlChar = 0 as *mut xmlChar;
ret =
unsafe{xmlStrndup_safe(src.offset(remove_head as isize),
               i - remove_head + 1 as i32)};
if ret.is_null() {
unsafe{xmlErrMemory(ctxt, 0 as *const i8)};
return 0 as *const xmlChar
}
xmlAttrNormalizeSpace(ret, ret);
unsafe{*len = strlen_safe(ret as *const i8) as i32};
return ret
} else {
if remove_head != 0 {
unsafe{*len -= remove_head};
unsafe{
memmove_safe(src as *mut (),
         src.offset(remove_head as isize) as *const (),
         (1 as i32 + *len) as u64);
}
return src
}
}
return 0 as *const xmlChar;
}
/* *
* xmlAddDefAttrs:
* @ctxt:  an XML parser context
* @fullname:  the element fullname
* @fullattr:  the attribute fullname
* @value:  the attribute value
*
* Add a defaulted attribute for an element
*/
unsafe fn xmlAddDefAttrs(mut ctxt: xmlParserCtxtPtr,
mut fullname: *const xmlChar,
mut fullattr: *const xmlChar,
mut value: *const xmlChar) {
let mut current_block: u64;
let mut defaults: xmlDefAttrsPtr = 0 as *mut xmlDefAttrs;
let mut len: i32 = 0;
let mut name: *const xmlChar = 0 as *const xmlChar;
let mut prefix: *const xmlChar = 0 as *const xmlChar;
/*
* Allows to detect attribute redefinitions
*/
if unsafe{!(*ctxt).attsSpecial.is_null()} {
if !xmlHashLookup2_safe(unsafe{(*ctxt).attsSpecial}, fullname, fullattr).is_null()
{
return
}
}
if unsafe{(*ctxt).attsDefault.is_null()} {
unsafe{(*ctxt).attsDefault = xmlHashCreateDict_safe(10 as i32,(*ctxt).dict)};
if unsafe{(*ctxt).attsDefault.is_null()} {
current_block = 2968889880470072775;
} else { current_block = 13183875560443969876; }
} else { current_block = 13183875560443969876; }
//@todo 削减unsafe范围
unsafe {
match current_block {
13183875560443969876 => {
/*
* split the element name into prefix:localname , the string found
* are within the DTD and then not associated to namespace names.
*/
name = xmlSplitQName3(fullname, &mut len);
if name.is_null() {
name =
    xmlDictLookup_safe((*ctxt).dict, fullname,
                  -(1 as i32));
prefix = 0 as *const xmlChar
} else {
name = xmlDictLookup_safe((*ctxt).dict, name, -(1 as i32));
prefix = xmlDictLookup_safe((*ctxt).dict, fullname, len)
}
/*
* make sure there is some storage
*/
defaults =
xmlHashLookup2_safe((*ctxt).attsDefault, name, prefix) as
    xmlDefAttrsPtr;
if defaults.is_null() {
defaults =
    xmlMalloc_safe((::std::mem::size_of::<xmlDefAttrs>()
        as
        u64).wrapping_add(((4
        as
        i32
        *
        5
            as
            i32)
        as
        u64).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
        as
        u64)))
        as xmlDefAttrsPtr;
if defaults.is_null() {
    current_block = 2968889880470072775;
} else {
    (*defaults).nbAttrs = 0 as i32;
    (*defaults).maxAttrs = 4 as i32;
    if xmlHashUpdateEntry2((*ctxt).attsDefault, name, prefix,
                           defaults as *mut (),
                           None) < 0 as i32 {
        xmlFree_safe(defaults
            as
            *mut ());
        current_block = 2968889880470072775;
    } else { current_block = 8704759739624374314; }
}
} else if (*defaults).nbAttrs >= (*defaults).maxAttrs {
let mut temp: xmlDefAttrsPtr = 0 as *mut xmlDefAttrs;
temp =
    xmlRealloc_safe(defaults as
                                                       *mut (),
                                                   (::std::mem::size_of::<xmlDefAttrs>()
                                                       as
                                                       u64).wrapping_add(((2
                                                       as
                                                       i32
                                                       *
                                                       (*defaults).maxAttrs
                                                       *
                                                       5
                                                           as
                                                           i32)
                                                       as
                                                       u64).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
                                                       as
                                                       u64)))
        as xmlDefAttrsPtr;
if temp.is_null() {
    current_block = 2968889880470072775;
} else {
    defaults = temp;
    (*defaults).maxAttrs *= 2 as i32;
    if xmlHashUpdateEntry2((*ctxt).attsDefault, name, prefix,
                           defaults as *mut (),
                           None) < 0 as i32 {
        xmlFree_safe(defaults
            as
            *mut ());
        current_block = 2968889880470072775;
    } else { current_block = 8704759739624374314; }
}
} else { current_block = 8704759739624374314; }
match current_block {
2968889880470072775 => { }
_ => {
    /*
* Split the element name into prefix:localname , the string found
* are within the DTD and hen not associated to namespace names.
*/
    name = xmlSplitQName3(fullattr, &mut len);
    if name.is_null() {
        name =
            xmlDictLookup_safe((*ctxt).dict, fullattr,
                          -(1 as i32));
        prefix = 0 as *const xmlChar
    } else {
        name =
            xmlDictLookup_safe((*ctxt).dict, name,
                          -(1 as i32));
        prefix = xmlDictLookup_safe((*ctxt).dict, fullattr, len)
    }
    let ref mut fresh3 =
        *(*defaults).values.as_mut_ptr().offset((5 as
            i32
            *
            (*defaults).nbAttrs)
            as isize);
    *fresh3 = name;
    let ref mut fresh4 =
        *(*defaults).values.as_mut_ptr().offset((5 as
            i32
            *
            (*defaults).nbAttrs
            +
            1 as
                i32)
            as isize);
    *fresh4 = prefix;
    /* intern the string and precompute the end */
    len = xmlStrlen_safe(value);
    value = xmlDictLookup_safe((*ctxt).dict, value, len);
    let ref mut fresh5 =
        *(*defaults).values.as_mut_ptr().offset((5 as
            i32
            *
            (*defaults).nbAttrs
            +
            2 as
                i32)
            as isize);
    *fresh5 = value;
    let ref mut fresh6 =
        *(*defaults).values.as_mut_ptr().offset((5 as
            i32
            *
            (*defaults).nbAttrs
            +
            3 as
                i32)
            as isize);
    *fresh6 = value.offset(len as isize);
    if (*ctxt).external != 0 {
        let ref mut fresh7 =
            *(*defaults).values.as_mut_ptr().offset((5 as
                i32
                *
                (*defaults).nbAttrs
                +
                4 as
                    i32)
                as
                isize);
        *fresh7 =
            b"external\x00" as *const u8 as
                *const i8 as *mut xmlChar
    } else {
        let ref mut fresh8 =
            *(*defaults).values.as_mut_ptr().offset((5 as
                i32
                *
                (*defaults).nbAttrs
                +
                4 as
                    i32)
                as
                isize);
        *fresh8 = 0 as *const xmlChar
    }
    (*defaults).nbAttrs += 1;
    return
}
}
}
_ => { }
}
}
unsafe{xmlErrMemory(ctxt, 0 as *const i8)};
}
/* *
* xmlAddSpecialAttr:
* @ctxt:  an XML parser context
* @fullname:  the element fullname
* @fullattr:  the attribute fullname
* @type:  the attribute type
*
* Register this attribute type
*/
unsafe fn xmlAddSpecialAttr(mut ctxt: xmlParserCtxtPtr,
 mut fullname: *const xmlChar,
 mut fullattr: *const xmlChar,
 mut type_0: i32) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if (safe_ctxt).attsSpecial.is_null() {
(safe_ctxt).attsSpecial =
xmlHashCreateDict_safe(10 as i32, (safe_ctxt).dict);
if (safe_ctxt).attsSpecial.is_null() {
unsafe{
xmlErrMemory(ctxt, 0 as *const i8);
}
return
}
}
if !xmlHashLookup2_safe(unsafe{(*ctxt).attsSpecial}, fullname, fullattr).is_null() {
return
}
xmlHashAddEntry2_safe(unsafe{(*ctxt).attsSpecial}, fullname, fullattr,
      type_0 as ptrdiff_t as *mut ());
}
/* *
* xmlCleanSpecialAttrCallback:
*
* Removes CDATA attributes from the special attribute table
*/
extern "C" fn xmlCleanSpecialAttrCallback(mut payload:
                      *mut (),
                      mut data: *mut (),
                      mut fullname: *const xmlChar,
                      mut fullattr: *const xmlChar,
                      mut unused: *const xmlChar) {
let mut ctxt: xmlParserCtxtPtr = data as xmlParserCtxtPtr;
let mut safe_ctxt  = unsafe{&mut *ctxt};
if payload as ptrdiff_t ==
XML_ATTRIBUTE_CDATA as i32 as i64 {
xmlHashRemoveEntry2_safe((safe_ctxt).attsSpecial, fullname, fullattr, None);
};
}
/* *
* xmlCleanSpecialAttr:
* @ctxt:  an XML parser context
*
* Trim the list of attributes defined to remove all those of type
* CDATA as they are not special. This call should be done when finishing
* to parse the DTD and before starting to parse the document root.
*/
unsafe fn xmlCleanSpecialAttr(mut ctxt: xmlParserCtxtPtr) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if (safe_ctxt).attsSpecial.is_null() { return }
xmlHashScanFull_safe((safe_ctxt).attsSpecial,
Some(xmlCleanSpecialAttrCallback as
    extern "C" fn(_: *mut (),
                  _: *mut (),
                  _: *const xmlChar,
                  _: *const xmlChar,
                  _: *const xmlChar) -> ()),
ctxt as *mut ());
if xmlHashSize_safe((safe_ctxt).attsSpecial) == 0 as i32 {
xmlHashFree_safe((safe_ctxt).attsSpecial, None);
(safe_ctxt).attsSpecial = 0 as xmlHashTablePtr;
};
}
/* *
* xmlCheckLanguageID:
* @lang:  pointer to the string value
*
* Checks that the value conforms to the LanguageID production:
*
* NOTE: this is somewhat deprecated, those productions were removed from
*       the XML Second edition.
*
* [33] LanguageID ::= Langcode ('-' Subcode)*
* [34] Langcode ::= ISO639Code |  IanaCode |  UserCode
* [35] ISO639Code ::= ([a-z] | [A-Z]) ([a-z] | [A-Z])
* [36] IanaCode ::= ('i' | 'I') '-' ([a-z] | [A-Z])+
* [37] UserCode ::= ('x' | 'X') '-' ([a-z] | [A-Z])+
* [38] Subcode ::= ([a-z] | [A-Z])+
*
* The current REC reference the successors of RFC 1766, currently 5646
*
* http://www.rfc-editor.org/rfc/rfc5646.txt
* langtag       = language
*                 ["-" script]
*                 ["-" region]
*                 *("-" variant)
*                 *("-" extension)
*                 ["-" privateuse]
* language      = 2*3ALPHA            ; shortest ISO 639 code
*                 ["-" extlang]       ; sometimes followed by
*                                     ; extended language subtags
*               / 4ALPHA              ; or reserved for future use
*               / 5*8ALPHA            ; or registered language subtag
*
* extlang       = 3ALPHA              ; selected ISO 639 codes
*                 *2("-" 3ALPHA)      ; permanently reserved
*
* script        = 4ALPHA              ; ISO 15924 code
*
* region        = 2ALPHA              ; ISO 3166-1 code
*               / 3DIGIT              ; UN M.49 code
*
* variant       = 5*8alphanum         ; registered variants
*               / (DIGIT 3alphanum)
*
* extension     = singleton 1*("-" (2*8alphanum))
*
*                                     ; Single alphanumerics
*                                     ; "x" reserved for private use
* singleton     = DIGIT               ; 0 - 9
*               / %x41-57             ; A - W
*               / %x59-5A             ; Y - Z
*               / %x61-77             ; a - w
*               / %x79-7A             ; y - z
*
* it sounds right to still allow Irregular i-xxx IANA and user codes too
* The parser below doesn't try to cope with extension or privateuse
* that could be added but that's not interoperable anyway
*
* Returns 1 if correct 0 otherwise
**/

pub unsafe fn xmlCheckLanguageID(mut lang: *const xmlChar)
             -> i32 {
let mut current_block: u64;
let mut cur: *const xmlChar = lang;
let mut nxt: *const xmlChar = 0 as *const xmlChar;
if cur.is_null() { return 0 as i32 }
if unsafe{*cur.offset(0 as i32 as isize) as i32 == 'i' as i32 &&
*cur.offset(1 as i32 as isize) as i32 == '-' as i32
||
*cur.offset(0 as i32 as isize) as i32 == 'I' as i32
&&
*cur.offset(1 as i32 as isize) as i32 ==
'-' as i32 ||
*cur.offset(0 as i32 as isize) as i32 == 'x' as i32
&&
*cur.offset(1 as i32 as isize) as i32 ==
'-' as i32 ||
*cur.offset(0 as i32 as isize) as i32 == 'X' as i32
&&
*cur.offset(1 as i32 as isize) as i32 ==
'-' as i32} {
/*
* Still allow IANA code and user code which were coming
* from the previous version of the XML-1.0 specification
* it's deprecated but we should not fail
*/
cur = unsafe{cur.offset(2 as i32 as isize)};
while unsafe{*cur.offset(0 as i32 as isize) as i32 >=
'A' as i32 &&
*cur.offset(0 as i32 as isize) as i32 <=
'Z' as i32 ||
*cur.offset(0 as i32 as isize) as i32 >=
'a' as i32 &&
*cur.offset(0 as i32 as isize) as i32 <=
'z' as i32} {
cur = unsafe{cur.offset(1)};
}
return unsafe{(*cur.offset(0 as i32 as isize) as i32 ==
0 as i32) as i32};
}
nxt = cur;
while unsafe{*nxt.offset(0 as i32 as isize) as i32 >= 'A' as i32
&&
*nxt.offset(0 as i32 as isize) as i32 <=
'Z' as i32 ||
*nxt.offset(0 as i32 as isize) as i32 >=
'a' as i32 &&
*nxt.offset(0 as i32 as isize) as i32 <=
'z' as i32} {
nxt = unsafe{nxt.offset(1)};
}
if unsafe{nxt.offset_from(cur) as i64 >=
4 as i32 as i64} {
/*
* Reserved
*/
if unsafe{nxt.offset_from(cur) as i64 >
8 as i32 as i64 ||
*nxt.offset(0 as i32 as isize) as i32 !=
0 as i32} {
return 0 as i32
}
return 1 as i32
}
if unsafe{(nxt.offset_from(cur) as i64) <
2 as i32 as i64} {
return 0 as i32
}
/* we got an ISO 639 code */
if unsafe{*nxt.offset(0 as i32 as isize) as i32 ==
0 as i32} {
return 1 as i32
}
if unsafe{*nxt.offset(0 as i32 as isize) as i32 != '-' as i32} {
return 0 as i32
}
nxt = unsafe{nxt.offset(1)};
cur = nxt;
/* now we can have extlang or script or region or variant */
if unsafe{*nxt.offset(0 as i32 as isize) as i32 >= '0' as i32 &&
*nxt.offset(0 as i32 as isize) as i32 <= '9' as i32}
{
current_block = 13163178004963364532;
} else {
while unsafe{*nxt.offset(0 as i32 as isize) as i32 >=
'A' as i32 &&
*nxt.offset(0 as i32 as isize) as i32 <=
'Z' as i32 ||
*nxt.offset(0 as i32 as isize) as i32 >=
'a' as i32 &&
*nxt.offset(0 as i32 as isize) as i32 <=
'z' as i32} {
nxt = unsafe{nxt.offset(1)};
}
if unsafe{nxt.offset_from(cur) as i64 ==
4 as i32 as i64} {
current_block = 14921549473310263854;
}
else if unsafe{nxt.offset_from(cur) as i64 ==
2 as i32 as i64} {
current_block = 15970415187932728765;
}
else if unsafe{nxt.offset_from(cur) as i64 >=
5 as i32 as i64 &&
nxt.offset_from(cur) as i64 <=
8 as i32 as i64} {
current_block = 6166658882887268861;
} else {
if unsafe{nxt.offset_from(cur) as i64 !=
3 as i32 as i64} {
return 0 as i32
}
/* we parsed an extlang */
if unsafe{*nxt.offset(0 as i32 as isize) as i32 ==
0 as i32} {
return 1 as i32
}
if unsafe{*nxt.offset(0 as i32 as isize) as i32 !=
'-' as i32} {
return 0 as i32
}
nxt = unsafe{nxt.offset(1)};
cur = nxt;
/* now we can have script or region or variant */
if unsafe{*nxt.offset(0 as i32 as isize) as i32 >=
'0' as i32 &&
*nxt.offset(0 as i32 as isize) as i32 <=
'9' as i32} {
current_block = 13163178004963364532;
} else {
while unsafe{*nxt.offset(0 as i32 as isize) as i32 >=
'A' as i32 &&
*nxt.offset(0 as i32 as isize) as
    i32 <= 'Z' as i32 ||
*nxt.offset(0 as i32 as isize) as
    i32 >= 'a' as i32 &&
    *nxt.offset(0 as i32 as isize) as
        i32 <= 'z' as i32} {
nxt = unsafe{nxt.offset(1)};
}
if unsafe{nxt.offset_from(cur) as i64 ==
2 as i32 as i64} {
current_block = 15970415187932728765;
} else if unsafe{nxt.offset_from(cur) as i64 >=
5 as i32 as i64 &&
nxt.offset_from(cur) as i64 <=
    8 as i32 as i64} {
current_block = 6166658882887268861;
} else {
if unsafe{nxt.offset_from(cur) as i64 !=
    4 as i32 as i64} {
    return 0 as i32
}
current_block = 14921549473310263854;
}
}
}
match current_block {
15970415187932728765 => { }
6166658882887268861 => { }
13163178004963364532 => { }
_ =>
/* we parsed a script */
{
if unsafe{*nxt.offset(0 as i32 as isize) as i32 ==
    0 as i32} {
    return 1 as i32
}
if unsafe{*nxt.offset(0 as i32 as isize) as i32 !=
    '-' as i32} {
    return 0 as i32
}
nxt = unsafe{nxt.offset(1)};
cur = nxt;
/* now we can have region or variant */
if unsafe{*nxt.offset(0 as i32 as isize) as i32 >=
    '0' as i32 &&
    *nxt.offset(0 as i32 as isize) as i32
        <= '9' as i32} {
    current_block = 13163178004963364532;
} else {
    while unsafe{*nxt.offset(0 as i32 as isize) as
        i32 >= 'A' as i32 &&
        *nxt.offset(0 as i32 as isize) as
            i32 <= 'Z' as i32 ||
        *nxt.offset(0 as i32 as isize) as
            i32 >= 'a' as i32 &&
            *nxt.offset(0 as i32 as isize) as
                i32 <= 'z' as i32} {
        nxt = unsafe{nxt.offset(1)};
    }
    if unsafe{nxt.offset_from(cur) as i64 >=
        5 as i32 as i64 &&
        nxt.offset_from(cur) as i64 <=
            8 as i32 as i64} {
        current_block = 6166658882887268861;
    } else {
        if unsafe{nxt.offset_from(cur) as i64 !=
            2 as i32 as i64} {
            return 0 as i32
        }
        current_block = 15970415187932728765;
    }
}
}
}
}
match current_block {
13163178004963364532 => {
if unsafe{*nxt.offset(1 as i32 as isize) as i32 >=
'0' as i32 &&
*nxt.offset(1 as i32 as isize) as i32 <=
'9' as i32 &&
(*nxt.offset(2 as i32 as isize) as i32 >=
'0' as i32 &&
*nxt.offset(2 as i32 as isize) as i32
    <= '9' as i32)} {
nxt = unsafe{nxt.offset(3 as i32 as isize)}
} else { return 0 as i32 }
current_block = 15970415187932728765;
}
_ => { }
}
match current_block {
15970415187932728765 =>
/* we parsed a region */
{
if unsafe{*nxt.offset(0 as i32 as isize) as i32 ==
0 as i32} {
return 1 as i32
}
if unsafe{*nxt.offset(0 as i32 as isize) as i32 !=
'-' as i32} {
return 0 as i32
}
nxt = unsafe{nxt.offset(1)};
cur = nxt;
/* now we can just have a variant */
while unsafe{*nxt.offset(0 as i32 as isize) as i32 >=
'A' as i32 &&
*nxt.offset(0 as i32 as isize) as i32 <=
    'Z' as i32 ||
*nxt.offset(0 as i32 as isize) as i32 >=
    'a' as i32 &&
    *nxt.offset(0 as i32 as isize) as
        i32 <= 'z' as i32} {
nxt = unsafe{nxt.offset(1)};
}
if unsafe{(nxt.offset_from(cur) as i64) <
5 as i32 as i64 ||
nxt.offset_from(cur) as i64 >
    8 as i32 as i64} {
return 0 as i32
}
}
_ => { }
}
/* we parsed a variant */
if unsafe{*nxt.offset(0 as i32 as isize) as i32 ==
0 as i32} {
return 1 as i32
}
if unsafe{*nxt.offset(0 as i32 as isize) as i32 != '-' as i32} {
return 0 as i32
}
/* extensions and private use subtags not checked */
return 1 as i32;
}
/* *
* nsPush:
* @ctxt:  an XML parser context
* @prefix:  the namespace prefix or NULL
* @URL:  the namespace name
*
* Pushes a new parser namespace on top of the ns stack
*
* Returns -1 in case of error, -2 if the namespace should be discarded
*	   and the index in the stack otherwise.
*/
#[cfg(HAVE_parser_SAX2)]
unsafe fn nsPush(mut ctxt: xmlParserCtxtPtr,
mut prefix: *const xmlChar,
mut URL: *const xmlChar) -> i32 {
let mut safe_ctxt = unsafe { &mut *ctxt };
if (safe_ctxt).options & XML_PARSE_NSCLEAN as i32 != 0 {
let mut i: i32 = 0;
i = (safe_ctxt).nsNr - 2 as i32;
while i >= 0 as i32 {
if unsafe { *(*ctxt).nsTab.offset(i as isize) == prefix } {
/* in scope */
if unsafe { *(*ctxt).nsTab.offset((i + 1 as i32) as isize) == URL } {
return -(2 as i32)
}
break;
} else { i -= 2 as i32 }
}
}
if (safe_ctxt).nsMax == 0 as i32 || (safe_ctxt).nsTab.is_null() {
(safe_ctxt).nsMax = 10 as i32;
(safe_ctxt).nsNr = 0 as i32;
(safe_ctxt).nsTab =
xmlMalloc_safe(((safe_ctxt).nsMax as
u64).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
as
u64))
as *mut *const xmlChar;
if (safe_ctxt).nsTab.is_null() {
unsafe {
xmlErrMemory(ctxt, 0 as *const i8);
}
(safe_ctxt).nsMax = 0 as i32;
return -(1 as i32)
}
} else if (safe_ctxt).nsNr >= (safe_ctxt).nsMax {
let mut tmp: *mut *const xmlChar = 0 as *mut *const xmlChar;
(safe_ctxt).nsMax *= 2 as i32;
tmp =
xmlRealloc_safe((safe_ctxt).nsTab as
            *mut i8
            as
            *mut (),
        ((safe_ctxt).nsMax as
            u64).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
            as
            u64))
as *mut *const xmlChar;
if tmp.is_null() {
unsafe {
xmlErrMemory(ctxt, 0 as *const i8);
}
(safe_ctxt).nsMax /= 2 as i32;
return -(1 as i32)
}
(safe_ctxt).nsTab = tmp
}
let fresh9 = (safe_ctxt).nsNr;
(safe_ctxt).nsNr = (safe_ctxt).nsNr + 1;
unsafe {
let ref mut fresh10 = *(*ctxt).nsTab.offset(fresh9 as isize);
*fresh10 = prefix;
let fresh11 = (safe_ctxt).nsNr;
(safe_ctxt).nsNr = (safe_ctxt).nsNr + 1;
let ref mut fresh12 = *(*ctxt).nsTab.offset(fresh11 as isize);
*fresh12 = URL;
};
return (safe_ctxt).nsNr;
}
/* *
* nsPop:
* @ctxt: an XML parser context
* @nr:  the number to pop
*
* Pops the top @nr parser prefix/namespace from the ns stack
*
* Returns the number of namespaces removed
*/
#[cfg(HAVE_parser_SAX2)]
unsafe fn nsPop(mut ctxt: xmlParserCtxtPtr, mut nr: i32)
-> i32 {
let mut safe_ctxt = unsafe { &mut *ctxt };
let mut i: i32 = 0; /* allow for 10 attrs by default */
if (safe_ctxt).nsTab.is_null() { return 0 as i32 }
if (safe_ctxt).nsNr < nr {
unsafe{
(*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                   b"Pbm popping %d NS\n\x00"
                                                       as
                                                       *const u8
                                                       as
                                                       *const i8,
                                                   nr);
}
nr = (safe_ctxt).nsNr
}
if (safe_ctxt).nsNr <= 0 as i32 { return 0 as i32 }
i = 0 as i32;
while i < nr {
(safe_ctxt).nsNr -= 1;
unsafe{
let ref mut fresh13 = *(*ctxt).nsTab.offset((safe_ctxt).nsNr as isize);
*fresh13 = 0 as *const xmlChar;
}
i += 1
}
return nr;
}
unsafe fn xmlCtxtGrowAttrs(mut ctxt: xmlParserCtxtPtr,
       mut nr: i32) -> i32 {
let mut safe_ctxt = unsafe { &mut *ctxt };
let mut current_block: u64;
let mut atts: *mut *const xmlChar = 0 as *mut *const xmlChar;
let mut attallocs: *mut i32 = 0 as *mut i32;
let mut maxatts: i32 = 0;
if (safe_ctxt).atts.is_null() {
maxatts = 55 as i32;
atts =
xmlMalloc_safe((maxatts as
u64).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
as
u64))
as *mut *const xmlChar;
if atts.is_null() {
current_block = 1220566974040888119;
} else {
(safe_ctxt).atts = atts;
attallocs =
xmlMalloc_safe(((maxatts /
5 as
    i32)
as
u64).wrapping_mul(::std::mem::size_of::<i32>()
as
u64))
as *mut i32;
if attallocs.is_null() {
current_block = 1220566974040888119;
} else {
(safe_ctxt).attallocs = attallocs;
(safe_ctxt).maxatts = maxatts;
current_block = 13242334135786603907;
}
}
} else if nr + 5 as i32 > (safe_ctxt).maxatts {
maxatts = (nr + 5 as i32) * 2 as i32;
atts =
xmlRealloc_safe((safe_ctxt).atts as
                                           *mut (),
                                       (maxatts as
                                           u64).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
                                           as
                                           u64))
as *mut *const xmlChar;
if atts.is_null() {
current_block = 1220566974040888119;
} else {
(safe_ctxt).atts = atts;
attallocs =
xmlRealloc_safe((safe_ctxt).attallocs
                                               as
                                               *mut (),
                                           ((maxatts /
                                               5 as
                                                   i32)
                                               as
                                               u64).wrapping_mul(::std::mem::size_of::<i32>()
                                               as
                                               u64))
as *mut i32;
if attallocs.is_null() {
current_block = 1220566974040888119;
} else {
(safe_ctxt).attallocs = attallocs;
(safe_ctxt).maxatts = maxatts;
current_block = 13242334135786603907;
}
}
} else { current_block = 13242334135786603907; }
match current_block {
13242334135786603907 => { return (safe_ctxt).maxatts }
_ => {
unsafe{
xmlErrMemory(ctxt, 0 as *const i8);
}
return -(1 as i32)
}
};
}
/* *
* inputPush:
* @ctxt:  an XML parser context
* @value:  the parser input
*
* Pushes a new parser input on top of the input stack
*
* Returns -1 in case of error, the index in the stack otherwise
*/

pub unsafe fn inputPush_parser(mut ctxt: xmlParserCtxtPtr,
    mut value: xmlParserInputPtr)
    -> i32 {
if ctxt.is_null() || value.is_null() { return -(1 as i32) }
let mut safe_ctxt = unsafe { &mut *ctxt };
if (safe_ctxt).inputNr >= (safe_ctxt).inputMax {
(safe_ctxt).inputMax *= 2 as i32;
(safe_ctxt).inputTab =
xmlRealloc_safe((safe_ctxt).inputTab as
                                           *mut (),
                                       ((safe_ctxt).inputMax
                                           as
                                           u64).wrapping_mul(::std::mem::size_of::<xmlParserInputPtr>()
                                           as
                                           u64))
as *mut xmlParserInputPtr;
if (safe_ctxt).inputTab.is_null() {
unsafe{
xmlErrMemory(ctxt, 0 as *const i8);
}
xmlFreeInputStream_safe(value);
(safe_ctxt).inputMax /= 2 as i32;
value = 0 as xmlParserInputPtr;
return -(1 as i32)
}
}
unsafe{
let ref mut fresh14 = *(*ctxt).inputTab.offset((safe_ctxt).inputNr as isize);
*fresh14 = value;
}
(safe_ctxt).input = value;
let fresh15 = (safe_ctxt).inputNr;
(safe_ctxt).inputNr = (safe_ctxt).inputNr + 1;
return fresh15;
}
/* *
* inputPop:
* @ctxt: an XML parser context
*
* Pops the top parser input from the input stack
*
* Returns the input just removed
*/

pub unsafe fn inputPop_parser(mut ctxt: xmlParserCtxtPtr)
   -> xmlParserInputPtr {
let mut ret: xmlParserInputPtr = 0 as *mut xmlParserInput;
if ctxt.is_null() { return 0 as xmlParserInputPtr }
let mut safe_ctxt = unsafe { &mut *ctxt };
if (safe_ctxt).inputNr <= 0 as i32 { return 0 as xmlParserInputPtr }
(safe_ctxt).inputNr -= 1;
if (safe_ctxt).inputNr > 0 as i32 {
(safe_ctxt).input = unsafe{*(*ctxt).inputTab.offset(((safe_ctxt).inputNr - 1 as i32) as isize)};
} else { (safe_ctxt).input = 0 as xmlParserInputPtr }
unsafe{
ret = *(*ctxt).inputTab.offset((safe_ctxt).inputNr as isize);
let ref mut fresh16 = *(*ctxt).inputTab.offset((safe_ctxt).inputNr as isize);
*fresh16 = 0 as xmlParserInputPtr;
}
return ret;
}
/* *
* nodePush:
* @ctxt:  an XML parser context
* @value:  the element node
*
* Pushes a new element node on top of the node stack
*
* Returns -1 in case of error, the index in the stack otherwise
*/

pub unsafe fn nodePush(mut ctxt: xmlParserCtxtPtr,
   mut value: xmlNodePtr) -> i32 {
if ctxt.is_null() { return 0 as i32 }
let mut safe_ctxt = unsafe { &mut *ctxt };
if (safe_ctxt).nodeNr >= (safe_ctxt).nodeMax {
let mut tmp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
tmp =
xmlRealloc_safe((safe_ctxt).nodeTab as
                                           *mut (),
                                       (((safe_ctxt).nodeMax *
                                           2 as
                                               i32)
                                           as
                                           u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                           as
                                           u64))
as *mut xmlNodePtr;
if tmp.is_null() {
unsafe{
xmlErrMemory(ctxt, 0 as *const i8);
}
return -(1 as i32)
}
(safe_ctxt).nodeTab = tmp;
(safe_ctxt).nodeMax *= 2 as i32
}
if (safe_ctxt).nodeNr as u32 > unsafe{xmlParserMaxDepth} &&
(safe_ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
{
unsafe{
xmlFatalErrMsgInt(ctxt, XML_ERR_INTERNAL_ERROR,
          b"Excessive depth in document: %d use XML_PARSE_HUGE option\n\x00"
              as *const u8 as *const i8,
          xmlParserMaxDepth as i32);
xmlHaltParser(ctxt);
}
return -(1 as i32)
}
unsafe{
let ref mut fresh17 = *(*ctxt).nodeTab.offset((safe_ctxt).nodeNr as isize);
*fresh17 = value;
}
(safe_ctxt).node = value;
let fresh18 = (safe_ctxt).nodeNr;
(safe_ctxt).nodeNr = (safe_ctxt).nodeNr + 1;
return fresh18;
}
/* *
* nodePop:
* @ctxt: an XML parser context
*
* Pops the top element node from the node stack
*
* Returns the node just removed
*/

pub unsafe fn nodePop_parser(mut ctxt: xmlParserCtxtPtr) -> xmlNodePtr {
let mut ret: xmlNodePtr = 0 as *mut xmlNode;
if ctxt.is_null() { return 0 as xmlNodePtr }
let mut safe_ctxt = unsafe { &mut *ctxt };
if (safe_ctxt).nodeNr <= 0 as i32 { return 0 as xmlNodePtr }
(safe_ctxt).nodeNr -= 1;
if (safe_ctxt).nodeNr > 0 as i32 {
(safe_ctxt).node = unsafe{*(*ctxt).nodeTab.offset(((safe_ctxt).nodeNr - 1 as i32) as isize)};
} else { (safe_ctxt).node = 0 as xmlNodePtr }
unsafe{
ret = *(*ctxt).nodeTab.offset((safe_ctxt).nodeNr as isize);
let ref mut fresh19 = *(*ctxt).nodeTab.offset((safe_ctxt).nodeNr as isize);
*fresh19 = 0 as xmlNodePtr;
}
return ret;
}
