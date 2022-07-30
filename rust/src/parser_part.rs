fn xmlParserEntityCheck(mut ctxt: xmlParserCtxtPtr,
    mut size: size_t,
    mut ent: xmlEntityPtr,
    mut replacement: size_t)
    -> libc::c_int {

let mut safe_ctxt  = unsafe{&mut *ctxt};
let mut safe_ent = unsafe{&mut *ent};

let mut consumed: size_t = 0 as libc::c_int as size_t;
let mut i: libc::c_int = 0;
if ctxt.is_null() || (safe_ctxt).options & XML_PARSE_HUGE as libc::c_int != 0
{
return 0 as libc::c_int
}
if (safe_ctxt).lastError.code == XML_ERR_ENTITY_LOOP as libc::c_int {
return 1 as libc::c_int
}
/*
* This may look absurd but is needed to detect
* entities problems
*/
if !ent.is_null() &&
(safe_ent).etype as libc::c_uint !=
XML_INTERNAL_PREDEFINED_ENTITY as libc::c_int as libc::c_uint
&& !(safe_ent).content.is_null() && (safe_ent).checked == 0 as libc::c_int
&& (safe_ctxt).errNo != XML_ERR_ENTITY_LOOP as libc::c_int {
let mut oldnbent: libc::c_ulong = (safe_ctxt).nbentities;
let mut diff: libc::c_ulong = 0;
let mut rep: *mut xmlChar = 0 as *mut xmlChar;
(safe_ent).checked = 1 as libc::c_int;
(safe_ctxt).depth += 1;
unsafe {
rep =
xmlStringDecodeEntities(ctxt, (safe_ent).content, 1 as libc::c_int,
                    0 as libc::c_int as xmlChar,
                    0 as libc::c_int as xmlChar,
                    0 as libc::c_int as xmlChar);
}
(safe_ctxt).depth -= 1;
if rep.is_null() ||
(safe_ctxt).errNo == XML_ERR_ENTITY_LOOP as libc::c_int {
unsafe {
*(safe_ent).content.offset(0 as libc::c_int as isize) =
0 as libc::c_int as xmlChar;
}
}
diff =
(safe_ctxt).nbentities.wrapping_sub(oldnbent).wrapping_add(1 as
libc::c_int
as
libc::c_ulong);
if diff >
(2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_ulong
{
diff =
(2147483647 as libc::c_int / 2 as libc::c_int) as
libc::c_ulong
}
(safe_ent).checked =
diff.wrapping_mul(2 as libc::c_int as libc::c_ulong) as
libc::c_int;
if !rep.is_null() {
if !xmlStrchr_safe(rep, '<' as i32 as xmlChar).is_null() {
(safe_ent).checked |= 1 as libc::c_int
}
xmlFree_safe(rep as
*mut libc::c_void);
rep = 0 as *mut xmlChar
}
}
/*
* Prevent entity exponential check, not just replacement while
* parsing the DTD
* The check is potentially costly so do that only once in a thousand
*/
if (safe_ctxt).instate as libc::c_int == XML_PARSER_DTD as libc::c_int &&
(safe_ctxt).nbentities > 10000 as libc::c_int as libc::c_ulong &&
(safe_ctxt).nbentities.wrapping_rem(1024 as libc::c_int as
libc::c_ulong) ==
0 as libc::c_int as libc::c_ulong {
i = 0 as libc::c_int;
while i < (safe_ctxt).inputNr {
consumed = unsafe{
(consumed as
libc::c_ulong).wrapping_add((**(safe_ctxt).inputTab.offset(i
as
isize)).consumed.wrapping_add((**(safe_ctxt).inputTab.offset(i
as
isize)).cur.offset_from((**(safe_ctxt).inputTab.offset(i
as
isize)).base)
as
libc::c_long
as
libc::c_ulong))
as size_t as size_t};
i += 1
}
if (safe_ctxt).nbentities >
consumed.wrapping_mul(10 as libc::c_int as libc::c_ulong) {
unsafe {
xmlFatalErr(ctxt, XML_ERR_ENTITY_LOOP, 0 as *const libc::c_char);
}
(safe_ctxt).instate = XML_PARSER_EOF;
return 1 as libc::c_int
}
consumed = 0 as libc::c_int as size_t
}
if replacement != 0 as libc::c_int as libc::c_ulong {
if replacement < 10000000 as libc::c_int as libc::c_ulong {
return 0 as libc::c_int
}
/*
* If the volume of entity copy reaches 10 times the
* amount of parsed data and over the large text threshold
* then that's very likely to be an abuse.
*/
if !(safe_ctxt).input.is_null() {
consumed =
unsafe{(*(safe_ctxt).input).consumed.wrapping_add((*(safe_ctxt).input).cur.offset_from((*(safe_ctxt).input).base)
as libc::c_long as libc::c_ulong)}
}
consumed =
(consumed as libc::c_ulong).wrapping_add((safe_ctxt).sizeentities) as
size_t as size_t;
if replacement <
(10 as libc::c_int as libc::c_ulong).wrapping_mul(consumed) {
return 0 as libc::c_int
}
} else if size != 0 as libc::c_int as libc::c_ulong {
/*
* Do the check based on the replacement size of the entity
*/
if size < 1000 as libc::c_int as libc::c_ulong {
return 0 as libc::c_int
}
/*
* A limit on the amount of text data reasonably used
*/
if !(safe_ctxt).input.is_null() {
consumed =
unsafe{(*(safe_ctxt).input).consumed.wrapping_add((*(safe_ctxt).input).cur.offset_from((*(safe_ctxt).input).base)
as libc::c_long as
libc::c_ulong)}
}
consumed =
(consumed as libc::c_ulong).wrapping_add((safe_ctxt).sizeentities) as
size_t as size_t;
if size < (10 as libc::c_int as libc::c_ulong).wrapping_mul(consumed)
&&
(safe_ctxt).nbentities.wrapping_mul(3 as libc::c_int as
libc::c_ulong) <
(10 as libc::c_int as libc::c_ulong).wrapping_mul(consumed)
{
return 0 as libc::c_int
}
} else if !ent.is_null() {
/*
* use the number of parsed entities in the replacement
*/
size = ((safe_ent).checked / 2 as libc::c_int) as size_t;
/*
* The amount of data parsed counting entities size only once
*/
if !(safe_ctxt).input.is_null() {
consumed =
unsafe{(*(safe_ctxt).input).consumed.wrapping_add((*(safe_ctxt).input).cur.offset_from((*(safe_ctxt).input).base)
as libc::c_long as
libc::c_ulong)}
}
consumed =
(consumed as libc::c_ulong).wrapping_add((safe_ctxt).sizeentities) as
size_t as size_t;
/*
* Check the density of entities for the amount of data
* knowing an entity reference will take at least 3 bytes
*/
if size.wrapping_mul(3 as libc::c_int as libc::c_ulong) <
consumed.wrapping_mul(10 as libc::c_int as libc::c_ulong) {
return 0 as libc::c_int
}
} else if (safe_ctxt).lastError.code !=
XML_ERR_UNDECLARED_ENTITY as libc::c_int &&
(safe_ctxt).lastError.code !=
XML_WAR_UNDECLARED_ENTITY as libc::c_int ||
(safe_ctxt).nbentities <= 10000 as libc::c_int as libc::c_ulong
{
return 0 as libc::c_int
}
unsafe {
xmlFatalErr(ctxt, XML_ERR_ENTITY_LOOP, 0 as *const libc::c_char);
}
return 1 as libc::c_int;
}

pub static mut xmlParserMaxDepth: libc::c_uint =
256 as libc::c_int as libc::c_uint;
/*
* List of XML prefixed PI allowed by W3C specs
*/
static mut xmlW3CPIs: [*const libc::c_char; 3] =
[b"xml-stylesheet\x00" as *const u8 as *const libc::c_char,
b"xml-model\x00" as *const u8 as *const libc::c_char,
0 as *const libc::c_char];
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
fn xmlErrAttributeDup(mut ctxt: xmlParserCtxtPtr,
  mut prefix: *const xmlChar,
  mut localname: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
if !ctxt.is_null() {
(safe_ctxt).errNo = XML_ERR_ATTRIBUTE_REDEFINED as libc::c_int
}
if prefix.is_null() {
unsafe {
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
        ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
        XML_FROM_PARSER as libc::c_int,
        XML_ERR_ATTRIBUTE_REDEFINED as libc::c_int,
        XML_ERR_FATAL, 0 as *const libc::c_char,
        0 as libc::c_int, localname as *const libc::c_char,
        0 as *const libc::c_char, 0 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int,
        b"Attribute %s redefined\n\x00" as *const u8 as
            *const libc::c_char, localname);
}
} else {
unsafe {
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
        ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
        XML_FROM_PARSER as libc::c_int,
        XML_ERR_ATTRIBUTE_REDEFINED as libc::c_int,
        XML_ERR_FATAL, 0 as *const libc::c_char,
        0 as libc::c_int, prefix as *const libc::c_char,
        localname as *const libc::c_char,
        0 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int,
        b"Attribute %s:%s redefined\n\x00" as *const u8 as
            *const libc::c_char, prefix, localname);
}
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as libc::c_int;
if (safe_ctxt).recovery == 0 as libc::c_int {
(safe_ctxt).disableSAX = 1 as libc::c_int
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
fn xmlFatalErr(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut info: *const libc::c_char) {

let mut safe_ctxt  = unsafe{&mut *ctxt};

let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
match error as libc::c_uint {
6 => {
errmsg =
b"CharRef: invalid hexadecimal value\x00" as *const u8 as
*const libc::c_char
}
7 => {
errmsg =
b"CharRef: invalid decimal value\x00" as *const u8 as
*const libc::c_char
}
8 => {
errmsg =
b"CharRef: invalid value\x00" as *const u8 as
*const libc::c_char
}
1 => {
errmsg = b"internal error\x00" as *const u8 as *const libc::c_char
}
18 => {
errmsg =
b"PEReference at end of document\x00" as *const u8 as
*const libc::c_char
}
19 => {
errmsg =
b"PEReference in prolog\x00" as *const u8 as
*const libc::c_char
}
20 => {
errmsg =
b"PEReference in epilog\x00" as *const u8 as
*const libc::c_char
}
24 => {
errmsg =
b"PEReference: no name\x00" as *const u8 as
*const libc::c_char
}
25 => {
errmsg =
b"PEReference: expecting \';\'\x00" as *const u8 as
*const libc::c_char
}
89 => {
errmsg =
b"Detected an entity reference loop\x00" as *const u8 as
*const libc::c_char
}
36 => {
errmsg =
b"EntityValue: \" or \' expected\x00" as *const u8 as
*const libc::c_char
}
88 => {
errmsg =
b"PEReferences forbidden in internal subset\x00" as *const u8
as *const libc::c_char
}
37 => {
errmsg =
b"EntityValue: \" or \' expected\x00" as *const u8 as
*const libc::c_char
}
39 => {
errmsg =
b"AttValue: \" or \' expected\x00" as *const u8 as
*const libc::c_char
}
38 => {
errmsg =
b"Unescaped \'<\' not allowed in attributes values\x00" as
*const u8 as *const libc::c_char
}
43 => {
errmsg =
b"SystemLiteral \" or \' expected\x00" as *const u8 as
*const libc::c_char
}
44 => {
errmsg =
b"Unfinished System or Public ID \" or \' expected\x00" as
*const u8 as *const libc::c_char
}
62 => {
errmsg =
b"Sequence \']]>\' not allowed in content\x00" as *const u8 as
*const libc::c_char
}
70 => {
errmsg =
b"SYSTEM or PUBLIC, the URI is missing\x00" as *const u8 as
*const libc::c_char
}
71 => {
errmsg =
b"PUBLIC, the Public Identifier is missing\x00" as *const u8
as *const libc::c_char
}
80 => {
errmsg =
b"Comment must not contain \'--\' (double-hyphen)\x00" as
*const u8 as *const libc::c_char
}
46 => {
errmsg =
b"xmlParsePI : no target name\x00" as *const u8 as
*const libc::c_char
}
64 => {
errmsg =
b"Invalid PI name\x00" as *const u8 as *const libc::c_char
}
48 => {
errmsg =
b"NOTATION: Name expected here\x00" as *const u8 as
*const libc::c_char
}
49 => {
errmsg =
b"\'>\' required to close NOTATION declaration\x00" as
*const u8 as *const libc::c_char
}
84 => {
errmsg =
b"Entity value required\x00" as *const u8 as
*const libc::c_char
}
92 => {
errmsg =
b"Fragment not allowed\x00" as *const u8 as
*const libc::c_char
}
50 => {
errmsg =
b"\'(\' required to start ATTLIST enumeration\x00" as
*const u8 as *const libc::c_char
}
67 => {
errmsg =
b"NmToken expected in ATTLIST enumeration\x00" as *const u8 as
*const libc::c_char
}
51 => {
errmsg =
b"\')\' required to finish ATTLIST enumeration\x00" as
*const u8 as *const libc::c_char
}
52 => {
errmsg =
b"MixedContentDecl : \'|\' or \')*\' expected\x00" as
*const u8 as *const libc::c_char
}
69 => {
errmsg =
b"MixedContentDecl : \'#PCDATA\' expected\x00" as *const u8 as
*const libc::c_char
}
54 => {
errmsg =
b"ContentDecl : Name or \'(\' expected\x00" as *const u8 as
*const libc::c_char
}
55 => {
errmsg =
b"ContentDecl : \',\' \'|\' or \')\' expected\x00" as
*const u8 as *const libc::c_char
}
21 => {
errmsg =
b"PEReference: forbidden within markup decl in internal subset\x00"
as *const u8 as *const libc::c_char
}
73 => {
errmsg = b"expected \'>\'\x00" as *const u8 as *const libc::c_char
}
83 => {
errmsg =
b"XML conditional section \'[\' expected\x00" as *const u8 as
*const libc::c_char
}
60 => {
errmsg =
b"Content error in the external subset\x00" as *const u8 as
*const libc::c_char
}
95 => {
errmsg =
b"conditional section INCLUDE or IGNORE keyword expected\x00"
as *const u8 as *const libc::c_char
}
59 => {
errmsg =
b"XML conditional section not closed\x00" as *const u8 as
*const libc::c_char
}
56 => {
errmsg =
b"Text declaration \'<?xml\' required\x00" as *const u8 as
*const libc::c_char
}
57 => {
errmsg =
b"parsing XML declaration: \'?>\' expected\x00" as *const u8
as *const libc::c_char
}
82 => {
errmsg =
b"external parsed entities cannot be standalone\x00" as
*const u8 as *const libc::c_char
}
23 => {
errmsg =
b"EntityRef: expecting \';\'\x00" as *const u8 as
*const libc::c_char
}
61 => {
errmsg =
b"DOCTYPE improperly terminated\x00" as *const u8 as
*const libc::c_char
}
74 => {
errmsg =
b"EndTag: \'</\' not found\x00" as *const u8 as
*const libc::c_char
}
75 => {
errmsg = b"expected \'=\'\x00" as *const u8 as *const libc::c_char
}
34 => {
errmsg =
b"String not closed expecting \" or \'\x00" as *const u8 as
*const libc::c_char
}
33 => {
errmsg =
b"String not started expecting \' or \"\x00" as *const u8 as
*const libc::c_char
}
79 => {
errmsg =
b"Invalid XML encoding name\x00" as *const u8 as
*const libc::c_char
}
78 => {
errmsg =
b"standalone accepts only \'yes\' or \'no\'\x00" as *const u8
as *const libc::c_char
}
4 => {
errmsg =
b"Document is empty\x00" as *const u8 as *const libc::c_char
}
5 => {
errmsg =
b"Extra content at the end of the document\x00" as *const u8
as *const libc::c_char
}
85 => {
errmsg =
b"chunk is not well balanced\x00" as *const u8 as
*const libc::c_char
}
86 => {
errmsg =
b"extra content at the end of well balanced chunk\x00" as
*const u8 as *const libc::c_char
}
96 => {
errmsg =
b"Malformed declaration expecting version\x00" as *const u8 as
*const libc::c_char
}
110 => {
errmsg =
b"Name too long use XML_PARSE_HUGE option\x00" as *const u8 as
*const libc::c_char
}
_ => {
errmsg =
b"Unregistered error message\x00" as *const u8 as
*const libc::c_char
}
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as libc::c_int }
if info.is_null() {
unsafe{
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
        ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
        XML_FROM_PARSER as libc::c_int, error as libc::c_int,
        XML_ERR_FATAL, 0 as *const libc::c_char,
        0 as libc::c_int, info, 0 as *const libc::c_char,
        0 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int,
        b"%s\n\x00" as *const u8 as *const libc::c_char,
        errmsg);
}
} else {
unsafe{
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
        ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
        XML_FROM_PARSER as libc::c_int, error as libc::c_int,
        XML_ERR_FATAL, 0 as *const libc::c_char,
        0 as libc::c_int, info, 0 as *const libc::c_char,
        0 as *const libc::c_char, 0 as libc::c_int,
        0 as libc::c_int,
        b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
        errmsg, info);
}
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as libc::c_int;
if (safe_ctxt).recovery == 0 as libc::c_int {
(safe_ctxt).disableSAX = 1 as libc::c_int
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
fn xmlFatalErrMsg(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const libc::c_char) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as libc::c_int }
unsafe{
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
    ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
    XML_FROM_PARSER as libc::c_int, error as libc::c_int,
    XML_ERR_FATAL, 0 as *const libc::c_char, 0 as libc::c_int,
    0 as *const libc::c_char, 0 as *const libc::c_char,
    0 as *const libc::c_char, 0 as libc::c_int,
    0 as libc::c_int,
    b"%s\x00" as *const u8 as *const libc::c_char, msg);
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as libc::c_int;
if (safe_ctxt).recovery == 0 as libc::c_int {
(safe_ctxt).disableSAX = 1 as libc::c_int
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
fn xmlWarningMsg(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const libc::c_char,
mut str1: *const xmlChar,
mut str2: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
let mut schannel: xmlStructuredErrorFunc = None;
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
if !ctxt.is_null() && !(safe_ctxt).sax.is_null() &&
unsafe{(*(safe_ctxt).sax).initialized == 0xdeedbeaf as libc::c_uint} {
schannel = unsafe{(*(safe_ctxt).sax).serror};
}
if !ctxt.is_null() {
unsafe{
__xmlRaiseError(schannel,
        if !(safe_ctxt).sax.is_null() {
            (*(safe_ctxt).sax).warning
        } else { None }, (safe_ctxt).userData,
        ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
        XML_FROM_PARSER as libc::c_int, error as libc::c_int,
        XML_ERR_WARNING, 0 as *const libc::c_char,
        0 as libc::c_int, str1 as *const libc::c_char,
        str2 as *const libc::c_char, 0 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int, msg,
        str1 as *const libc::c_char,
        str2 as *const libc::c_char);
}
} else {
unsafe{
__xmlRaiseError(schannel, None, 0 as *mut libc::c_void,
        ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
        XML_FROM_PARSER as libc::c_int, error as libc::c_int,
        XML_ERR_WARNING, 0 as *const libc::c_char,
        0 as libc::c_int, str1 as *const libc::c_char,
        str2 as *const libc::c_char, 0 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int, msg,
        str1 as *const libc::c_char,
        str2 as *const libc::c_char);
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
fn xmlValidityError(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const libc::c_char,
mut str1: *const xmlChar,
mut str2: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
let mut schannel: xmlStructuredErrorFunc = None;
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
if !ctxt.is_null() {
(safe_ctxt).errNo = error as libc::c_int;
if !(safe_ctxt).sax.is_null() &&
unsafe{(*(safe_ctxt).sax).initialized == 0xdeedbeaf as libc::c_uint} {
schannel = unsafe{(*(safe_ctxt).sax).serror};
}
}
if !ctxt.is_null() {
unsafe {
__xmlRaiseError(schannel, (safe_ctxt).vctxt.error, (safe_ctxt).vctxt.userData,
        ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
        XML_FROM_DTD as libc::c_int, error as libc::c_int,
        XML_ERR_ERROR, 0 as *const libc::c_char,
        0 as libc::c_int, str1 as *const libc::c_char,
        str2 as *const libc::c_char, 0 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int, msg,
        str1 as *const libc::c_char,
        str2 as *const libc::c_char);
}
(safe_ctxt).valid = 0 as libc::c_int
} else {
unsafe{
__xmlRaiseError(schannel, None, 0 as *mut libc::c_void,
        ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
        XML_FROM_DTD as libc::c_int, error as libc::c_int,
        XML_ERR_ERROR, 0 as *const libc::c_char,
        0 as libc::c_int, str1 as *const libc::c_char,
        str2 as *const libc::c_char, 0 as *const libc::c_char,
        0 as libc::c_int, 0 as libc::c_int, msg,
        str1 as *const libc::c_char,
        str2 as *const libc::c_char);
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
fn xmlFatalErrMsgInt(mut ctxt: xmlParserCtxtPtr,
 mut error: xmlParserErrors,
 mut msg: *const libc::c_char,
 mut val: libc::c_int) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as libc::c_int }
unsafe{
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
    ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
    XML_FROM_PARSER as libc::c_int, error as libc::c_int,
    XML_ERR_FATAL, 0 as *const libc::c_char, 0 as libc::c_int,
    0 as *const libc::c_char, 0 as *const libc::c_char,
    0 as *const libc::c_char, val, 0 as libc::c_int, msg,
    val);
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as libc::c_int;
if (safe_ctxt).recovery == 0 as libc::c_int {
(safe_ctxt).disableSAX = 1 as libc::c_int
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
fn xmlFatalErrMsgStrIntStr(mut ctxt: xmlParserCtxtPtr,
       mut error: xmlParserErrors,
       mut msg: *const libc::c_char,
       mut str1: *const xmlChar,
       mut val: libc::c_int,
       mut str2: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as libc::c_int }
unsafe{
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
    ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
    XML_FROM_PARSER as libc::c_int, error as libc::c_int,
    XML_ERR_FATAL, 0 as *const libc::c_char, 0 as libc::c_int,
    str1 as *const libc::c_char, str2 as *const libc::c_char,
    0 as *const libc::c_char, val, 0 as libc::c_int, msg,
    str1, val, str2);
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as libc::c_int;
if (safe_ctxt).recovery == 0 as libc::c_int {
(safe_ctxt).disableSAX = 1 as libc::c_int
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
fn xmlFatalErrMsgStr(mut ctxt: xmlParserCtxtPtr,
 mut error: xmlParserErrors,
 mut msg: *const libc::c_char,
 mut val: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as libc::c_int }
unsafe{
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
    ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
    XML_FROM_PARSER as libc::c_int, error as libc::c_int,
    XML_ERR_FATAL, 0 as *const libc::c_char, 0 as libc::c_int,
    val as *const libc::c_char, 0 as *const libc::c_char,
    0 as *const libc::c_char, 0 as libc::c_int,
    0 as libc::c_int, msg, val);
}
if !ctxt.is_null() {
(safe_ctxt).wellFormed = 0 as libc::c_int;
if (safe_ctxt).recovery == 0 as libc::c_int {
(safe_ctxt).disableSAX = 1 as libc::c_int
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
fn xmlErrMsgStr(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const libc::c_char,
mut val: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as libc::c_int }
unsafe{
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
    ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
    XML_FROM_PARSER as libc::c_int, error as libc::c_int,
    XML_ERR_ERROR, 0 as *const libc::c_char, 0 as libc::c_int,
    val as *const libc::c_char, 0 as *const libc::c_char,
    0 as *const libc::c_char, 0 as libc::c_int,
    0 as libc::c_int, msg, val);
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
fn xmlNsErr(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const libc::c_char,
mut info1: *const xmlChar,
mut info2: *const xmlChar,
mut info3: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
if !ctxt.is_null() { (safe_ctxt).errNo = error as libc::c_int }
unsafe{
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
    ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
    XML_FROM_NAMESPACE as libc::c_int, error as libc::c_int,
    XML_ERR_ERROR, 0 as *const libc::c_char, 0 as libc::c_int,
    info1 as *const libc::c_char,
    info2 as *const libc::c_char,
    info3 as *const libc::c_char, 0 as libc::c_int,
    0 as libc::c_int, msg, info1, info2, info3);
}
if !ctxt.is_null() { (safe_ctxt).nsWellFormed = 0 as libc::c_int };
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
fn xmlNsWarn(mut ctxt: xmlParserCtxtPtr,
mut error: xmlParserErrors,
mut msg: *const libc::c_char,
mut info1: *const xmlChar,
mut info2: *const xmlChar,
mut info3: *const xmlChar) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if !ctxt.is_null() && (safe_ctxt).disableSAX != 0 as libc::c_int &&
(safe_ctxt).instate as libc::c_int == XML_PARSER_EOF as libc::c_int {
return
}
unsafe{
__xmlRaiseError(None, None, 0 as *mut libc::c_void,
    ctxt as *mut libc::c_void, 0 as *mut libc::c_void,
    XML_FROM_NAMESPACE as libc::c_int, error as libc::c_int,
    XML_ERR_WARNING, 0 as *const libc::c_char,
    0 as libc::c_int, info1 as *const libc::c_char,
    info2 as *const libc::c_char,
    info3 as *const libc::c_char, 0 as libc::c_int,
    0 as libc::c_int, msg, info1, info2, info3);
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

pub fn xmlHasFeature(mut feature: xmlFeature)
 -> libc::c_int {
match feature as libc::c_uint {
1 => { return 1 as libc::c_int }
2 => { return 1 as libc::c_int }
3 => { return 1 as libc::c_int }
4 => { return 1 as libc::c_int }
5 => { return 1 as libc::c_int }
6 => { return 1 as libc::c_int }
7 => { return 1 as libc::c_int }
8 => { return 1 as libc::c_int }
9 => { return 1 as libc::c_int }
10 => { return 1 as libc::c_int }
11 => { return 1 as libc::c_int }
12 => { return 1 as libc::c_int }
13 => { return 1 as libc::c_int }
14 => { return 1 as libc::c_int }
15 => { return 1 as libc::c_int }
16 => { return 1 as libc::c_int }
17 => { return 1 as libc::c_int }
18 => { return 1 as libc::c_int }
19 => { return 1 as libc::c_int }
20 => { return 1 as libc::c_int }
21 => { return 1 as libc::c_int }
22 => { return 1 as libc::c_int }
23 => { return 1 as libc::c_int }
24 => { return 0 as libc::c_int }
25 => { return 1 as libc::c_int }
26 => { return 1 as libc::c_int }
27 => { return 1 as libc::c_int }
28 => { return 1 as libc::c_int }
29 => { return 0 as libc::c_int }
30 => { return 0 as libc::c_int }
31 => { return 1 as libc::c_int }
33 => { return 1 as libc::c_int }
32 => { return 0 as libc::c_int }
_ => { }
}
return 0 as libc::c_int;
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
fn xmlDetectSAX2(mut ctxt: xmlParserCtxtPtr) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
let mut sax: xmlSAXHandlerPtr = 0 as *mut xmlSAXHandler;
if ctxt.is_null() { return }
sax = (safe_ctxt).sax;
let mut safe_sax  = unsafe{&mut *sax};
match () {
#[cfg(HAVE_parser_LIBXML_SAX1_ENABLED)]  _ => {
if !sax.is_null() && (safe_sax).initialized == 0xdeedbeaf as libc::c_uint &&
((safe_sax).startElementNs.is_some() || (safe_sax).endElementNs.is_some()
||
(safe_sax).startElement.is_none() && (safe_sax).endElement.is_none())
{
(safe_ctxt).sax2 = 1 as libc::c_int;
}
}
#[cfg(not(HAVE_parser_LIBXML_SAX1_ENABLED))] _ => {
(safe_ctxt).sax2 = 1 as libc::c_int;
}
};

/* LIBXML_SAX1_ENABLED */
(safe_ctxt).str_xml =
xmlDictLookup_safe((safe_ctxt).dict,
       b"xml\x00" as *const u8 as *const libc::c_char as
           *mut xmlChar, 3 as libc::c_int);
(safe_ctxt).str_xmlns =
xmlDictLookup_safe((safe_ctxt).dict,
       b"xmlns\x00" as *const u8 as *const libc::c_char as
           *mut xmlChar, 5 as libc::c_int);
(safe_ctxt).str_xml_ns =
xmlDictLookup_safe((safe_ctxt).dict,
       b"http://www.w3.org/XML/1998/namespace\x00" as *const u8
           as *const libc::c_char as *const xmlChar,
       36 as libc::c_int);
if (safe_ctxt).str_xml.is_null() || (safe_ctxt).str_xmlns.is_null() ||
(safe_ctxt).str_xml_ns.is_null() {
unsafe {
unsafe{xmlErrMemory(ctxt, 0 as *const libc::c_char)};
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
fn xmlAttrNormalizeSpace(mut src: *const xmlChar,
     mut dst: *mut xmlChar)
     -> *mut xmlChar {
if src.is_null() || dst.is_null() { return 0 as *mut xmlChar }
while unsafe{*src} as libc::c_int == 0x20 as libc::c_int { src = unsafe{src.offset(1)} }
while unsafe{*src} as libc::c_int != 0 as libc::c_int {
if unsafe{*src} as libc::c_int == 0x20 as libc::c_int {
while unsafe{*src} as libc::c_int == 0x20 as libc::c_int {
src = unsafe{src.offset(1)}
}
if unsafe{*src} as libc::c_int != 0 as libc::c_int {
let fresh0 = dst;
dst = unsafe{dst.offset(1)};
unsafe{*fresh0 = 0x20 as libc::c_int as xmlChar};
}
} else {
let fresh1 = src;
src = unsafe{src.offset(1)};
let fresh2 = dst;
dst = unsafe{dst.offset(1)};
unsafe{*fresh2 = *fresh1};
}
}
unsafe{*dst = 0 as libc::c_int as xmlChar};
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
fn xmlAttrNormalizeSpace2(mut ctxt: xmlParserCtxtPtr,
      mut src: *mut xmlChar,
      mut len: *mut libc::c_int)
      -> *const xmlChar {
let mut i: libc::c_int = 0;
let mut remove_head: libc::c_int = 0 as libc::c_int;
let mut need_realloc: libc::c_int = 0 as libc::c_int;
let mut cur: *const xmlChar = 0 as *const xmlChar;
if ctxt.is_null() || src.is_null() || len.is_null() {
return 0 as *const xmlChar
}
i = unsafe{*len};
if i <= 0 as libc::c_int { return 0 as *const xmlChar }
cur = src;
while unsafe{*cur} as libc::c_int == 0x20 as libc::c_int {
cur = unsafe{cur.offset(1)};
remove_head += 1
}
while unsafe{*cur} as libc::c_int != 0 as libc::c_int {
if unsafe{*cur} as libc::c_int == 0x20 as libc::c_int {
cur = unsafe{cur.offset(1)};
if !(unsafe{*cur} as libc::c_int == 0x20 as libc::c_int ||
unsafe{*cur} as libc::c_int == 0 as libc::c_int) {
continue ;
}
need_realloc = 1 as libc::c_int;
break ;
} else { cur = unsafe{cur.offset(1)} }
}
if need_realloc != 0 {
let mut ret: *mut xmlChar = 0 as *mut xmlChar;
ret =
unsafe{xmlStrndup_safe(src.offset(remove_head as isize),
               i - remove_head + 1 as libc::c_int)};
if ret.is_null() {
unsafe{xmlErrMemory(ctxt, 0 as *const libc::c_char)};
return 0 as *const xmlChar
}
xmlAttrNormalizeSpace(ret, ret);
unsafe{*len = strlen_safe(ret as *const libc::c_char) as libc::c_int};
return ret
} else {
if remove_head != 0 {
unsafe{*len -= remove_head};
unsafe{
memmove_safe(src as *mut libc::c_void,
         src.offset(remove_head as isize) as *const libc::c_void,
         (1 as libc::c_int + *len) as libc::c_ulong);
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
fn xmlAddDefAttrs(mut ctxt: xmlParserCtxtPtr,
mut fullname: *const xmlChar,
mut fullattr: *const xmlChar,
mut value: *const xmlChar) {
let mut current_block: u64;
let mut defaults: xmlDefAttrsPtr = 0 as *mut xmlDefAttrs;
let mut len: libc::c_int = 0;
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
unsafe{(*ctxt).attsDefault = xmlHashCreateDict_safe(10 as libc::c_int,(*ctxt).dict)};
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
                  -(1 as libc::c_int));
prefix = 0 as *const xmlChar
} else {
name = xmlDictLookup_safe((*ctxt).dict, name, -(1 as libc::c_int));
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
        libc::c_ulong).wrapping_add(((4
        as
        libc::c_int
        *
        5
            as
            libc::c_int)
        as
        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
        as
        libc::c_ulong)))
        as xmlDefAttrsPtr;
if defaults.is_null() {
    current_block = 2968889880470072775;
} else {
    (*defaults).nbAttrs = 0 as libc::c_int;
    (*defaults).maxAttrs = 4 as libc::c_int;
    if xmlHashUpdateEntry2((*ctxt).attsDefault, name, prefix,
                           defaults as *mut libc::c_void,
                           None) < 0 as libc::c_int {
        xmlFree_safe(defaults
            as
            *mut libc::c_void);
        current_block = 2968889880470072775;
    } else { current_block = 8704759739624374314; }
}
} else if (*defaults).nbAttrs >= (*defaults).maxAttrs {
let mut temp: xmlDefAttrsPtr = 0 as *mut xmlDefAttrs;
temp =
    xmlRealloc_safe(defaults as
                                                       *mut libc::c_void,
                                                   (::std::mem::size_of::<xmlDefAttrs>()
                                                       as
                                                       libc::c_ulong).wrapping_add(((2
                                                       as
                                                       libc::c_int
                                                       *
                                                       (*defaults).maxAttrs
                                                       *
                                                       5
                                                           as
                                                           libc::c_int)
                                                       as
                                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
                                                       as
                                                       libc::c_ulong)))
        as xmlDefAttrsPtr;
if temp.is_null() {
    current_block = 2968889880470072775;
} else {
    defaults = temp;
    (*defaults).maxAttrs *= 2 as libc::c_int;
    if xmlHashUpdateEntry2((*ctxt).attsDefault, name, prefix,
                           defaults as *mut libc::c_void,
                           None) < 0 as libc::c_int {
        xmlFree_safe(defaults
            as
            *mut libc::c_void);
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
                          -(1 as libc::c_int));
        prefix = 0 as *const xmlChar
    } else {
        name =
            xmlDictLookup_safe((*ctxt).dict, name,
                          -(1 as libc::c_int));
        prefix = xmlDictLookup_safe((*ctxt).dict, fullattr, len)
    }
    let ref mut fresh3 =
        *(*defaults).values.as_mut_ptr().offset((5 as
            libc::c_int
            *
            (*defaults).nbAttrs)
            as isize);
    *fresh3 = name;
    let ref mut fresh4 =
        *(*defaults).values.as_mut_ptr().offset((5 as
            libc::c_int
            *
            (*defaults).nbAttrs
            +
            1 as
                libc::c_int)
            as isize);
    *fresh4 = prefix;
    /* intern the string and precompute the end */
    len = xmlStrlen_safe(value);
    value = xmlDictLookup_safe((*ctxt).dict, value, len);
    let ref mut fresh5 =
        *(*defaults).values.as_mut_ptr().offset((5 as
            libc::c_int
            *
            (*defaults).nbAttrs
            +
            2 as
                libc::c_int)
            as isize);
    *fresh5 = value;
    let ref mut fresh6 =
        *(*defaults).values.as_mut_ptr().offset((5 as
            libc::c_int
            *
            (*defaults).nbAttrs
            +
            3 as
                libc::c_int)
            as isize);
    *fresh6 = value.offset(len as isize);
    if (*ctxt).external != 0 {
        let ref mut fresh7 =
            *(*defaults).values.as_mut_ptr().offset((5 as
                libc::c_int
                *
                (*defaults).nbAttrs
                +
                4 as
                    libc::c_int)
                as
                isize);
        *fresh7 =
            b"external\x00" as *const u8 as
                *const libc::c_char as *mut xmlChar
    } else {
        let ref mut fresh8 =
            *(*defaults).values.as_mut_ptr().offset((5 as
                libc::c_int
                *
                (*defaults).nbAttrs
                +
                4 as
                    libc::c_int)
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
unsafe{xmlErrMemory(ctxt, 0 as *const libc::c_char)};
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
fn xmlAddSpecialAttr(mut ctxt: xmlParserCtxtPtr,
 mut fullname: *const xmlChar,
 mut fullattr: *const xmlChar,
 mut type_0: libc::c_int) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if (safe_ctxt).attsSpecial.is_null() {
(safe_ctxt).attsSpecial =
xmlHashCreateDict_safe(10 as libc::c_int, (safe_ctxt).dict);
if (safe_ctxt).attsSpecial.is_null() {
unsafe{
xmlErrMemory(ctxt, 0 as *const libc::c_char);
}
return
}
}
if !xmlHashLookup2_safe(unsafe{(*ctxt).attsSpecial}, fullname, fullattr).is_null() {
return
}
xmlHashAddEntry2_safe(unsafe{(*ctxt).attsSpecial}, fullname, fullattr,
      type_0 as ptrdiff_t as *mut libc::c_void);
}
/* *
* xmlCleanSpecialAttrCallback:
*
* Removes CDATA attributes from the special attribute table
*/
extern "C" fn xmlCleanSpecialAttrCallback(mut payload:
                      *mut libc::c_void,
                      mut data: *mut libc::c_void,
                      mut fullname: *const xmlChar,
                      mut fullattr: *const xmlChar,
                      mut unused: *const xmlChar) {
let mut ctxt: xmlParserCtxtPtr = data as xmlParserCtxtPtr;
let mut safe_ctxt  = unsafe{&mut *ctxt};
if payload as ptrdiff_t ==
XML_ATTRIBUTE_CDATA as libc::c_int as libc::c_long {
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
fn xmlCleanSpecialAttr(mut ctxt: xmlParserCtxtPtr) {
let mut safe_ctxt  = unsafe{&mut *ctxt};
if (safe_ctxt).attsSpecial.is_null() { return }
xmlHashScanFull_safe((safe_ctxt).attsSpecial,
Some(xmlCleanSpecialAttrCallback as
    extern "C" fn(_: *mut libc::c_void,
                  _: *mut libc::c_void,
                  _: *const xmlChar,
                  _: *const xmlChar,
                  _: *const xmlChar) -> ()),
ctxt as *mut libc::c_void);
if xmlHashSize_safe((safe_ctxt).attsSpecial) == 0 as libc::c_int {
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

pub fn xmlCheckLanguageID(mut lang: *const xmlChar)
             -> libc::c_int {
let mut current_block: u64;
let mut cur: *const xmlChar = lang;
let mut nxt: *const xmlChar = 0 as *const xmlChar;
if cur.is_null() { return 0 as libc::c_int }
if unsafe{*cur.offset(0 as libc::c_int as isize) as libc::c_int == 'i' as i32 &&
*cur.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
||
*cur.offset(0 as libc::c_int as isize) as libc::c_int == 'I' as i32
&&
*cur.offset(1 as libc::c_int as isize) as libc::c_int ==
'-' as i32 ||
*cur.offset(0 as libc::c_int as isize) as libc::c_int == 'x' as i32
&&
*cur.offset(1 as libc::c_int as isize) as libc::c_int ==
'-' as i32 ||
*cur.offset(0 as libc::c_int as isize) as libc::c_int == 'X' as i32
&&
*cur.offset(1 as libc::c_int as isize) as libc::c_int ==
'-' as i32} {
/*
* Still allow IANA code and user code which were coming
* from the previous version of the XML-1.0 specification
* it's deprecated but we should not fail
*/
cur = unsafe{cur.offset(2 as libc::c_int as isize)};
while unsafe{*cur.offset(0 as libc::c_int as isize) as libc::c_int >=
'A' as i32 &&
*cur.offset(0 as libc::c_int as isize) as libc::c_int <=
'Z' as i32 ||
*cur.offset(0 as libc::c_int as isize) as libc::c_int >=
'a' as i32 &&
*cur.offset(0 as libc::c_int as isize) as libc::c_int <=
'z' as i32} {
cur = unsafe{cur.offset(1)};
}
return unsafe{(*cur.offset(0 as libc::c_int as isize) as libc::c_int ==
0 as libc::c_int) as libc::c_int};
}
nxt = cur;
while unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int >= 'A' as i32
&&
*nxt.offset(0 as libc::c_int as isize) as libc::c_int <=
'Z' as i32 ||
*nxt.offset(0 as libc::c_int as isize) as libc::c_int >=
'a' as i32 &&
*nxt.offset(0 as libc::c_int as isize) as libc::c_int <=
'z' as i32} {
nxt = unsafe{nxt.offset(1)};
}
if unsafe{nxt.offset_from(cur) as libc::c_long >=
4 as libc::c_int as libc::c_long} {
/*
* Reserved
*/
if unsafe{nxt.offset_from(cur) as libc::c_long >
8 as libc::c_int as libc::c_long ||
*nxt.offset(0 as libc::c_int as isize) as libc::c_int !=
0 as libc::c_int} {
return 0 as libc::c_int
}
return 1 as libc::c_int
}
if unsafe{(nxt.offset_from(cur) as libc::c_long) <
2 as libc::c_int as libc::c_long} {
return 0 as libc::c_int
}
/* we got an ISO 639 code */
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int ==
0 as libc::c_int} {
return 1 as libc::c_int
}
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32} {
return 0 as libc::c_int
}
nxt = unsafe{nxt.offset(1)};
cur = nxt;
/* now we can have extlang or script or region or variant */
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32 &&
*nxt.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32}
{
current_block = 13163178004963364532;
} else {
while unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int >=
'A' as i32 &&
*nxt.offset(0 as libc::c_int as isize) as libc::c_int <=
'Z' as i32 ||
*nxt.offset(0 as libc::c_int as isize) as libc::c_int >=
'a' as i32 &&
*nxt.offset(0 as libc::c_int as isize) as libc::c_int <=
'z' as i32} {
nxt = unsafe{nxt.offset(1)};
}
if unsafe{nxt.offset_from(cur) as libc::c_long ==
4 as libc::c_int as libc::c_long} {
current_block = 14921549473310263854;
}
else if unsafe{nxt.offset_from(cur) as libc::c_long ==
2 as libc::c_int as libc::c_long} {
current_block = 15970415187932728765;
}
else if unsafe{nxt.offset_from(cur) as libc::c_long >=
5 as libc::c_int as libc::c_long &&
nxt.offset_from(cur) as libc::c_long <=
8 as libc::c_int as libc::c_long} {
current_block = 6166658882887268861;
} else {
if unsafe{nxt.offset_from(cur) as libc::c_long !=
3 as libc::c_int as libc::c_long} {
return 0 as libc::c_int
}
/* we parsed an extlang */
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int ==
0 as libc::c_int} {
return 1 as libc::c_int
}
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int !=
'-' as i32} {
return 0 as libc::c_int
}
nxt = unsafe{nxt.offset(1)};
cur = nxt;
/* now we can have script or region or variant */
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int >=
'0' as i32 &&
*nxt.offset(0 as libc::c_int as isize) as libc::c_int <=
'9' as i32} {
current_block = 13163178004963364532;
} else {
while unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int >=
'A' as i32 &&
*nxt.offset(0 as libc::c_int as isize) as
    libc::c_int <= 'Z' as i32 ||
*nxt.offset(0 as libc::c_int as isize) as
    libc::c_int >= 'a' as i32 &&
    *nxt.offset(0 as libc::c_int as isize) as
        libc::c_int <= 'z' as i32} {
nxt = unsafe{nxt.offset(1)};
}
if unsafe{nxt.offset_from(cur) as libc::c_long ==
2 as libc::c_int as libc::c_long} {
current_block = 15970415187932728765;
} else if unsafe{nxt.offset_from(cur) as libc::c_long >=
5 as libc::c_int as libc::c_long &&
nxt.offset_from(cur) as libc::c_long <=
    8 as libc::c_int as libc::c_long} {
current_block = 6166658882887268861;
} else {
if unsafe{nxt.offset_from(cur) as libc::c_long !=
    4 as libc::c_int as libc::c_long} {
    return 0 as libc::c_int
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
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int ==
    0 as libc::c_int} {
    return 1 as libc::c_int
}
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int !=
    '-' as i32} {
    return 0 as libc::c_int
}
nxt = unsafe{nxt.offset(1)};
cur = nxt;
/* now we can have region or variant */
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int >=
    '0' as i32 &&
    *nxt.offset(0 as libc::c_int as isize) as libc::c_int
        <= '9' as i32} {
    current_block = 13163178004963364532;
} else {
    while unsafe{*nxt.offset(0 as libc::c_int as isize) as
        libc::c_int >= 'A' as i32 &&
        *nxt.offset(0 as libc::c_int as isize) as
            libc::c_int <= 'Z' as i32 ||
        *nxt.offset(0 as libc::c_int as isize) as
            libc::c_int >= 'a' as i32 &&
            *nxt.offset(0 as libc::c_int as isize) as
                libc::c_int <= 'z' as i32} {
        nxt = unsafe{nxt.offset(1)};
    }
    if unsafe{nxt.offset_from(cur) as libc::c_long >=
        5 as libc::c_int as libc::c_long &&
        nxt.offset_from(cur) as libc::c_long <=
            8 as libc::c_int as libc::c_long} {
        current_block = 6166658882887268861;
    } else {
        if unsafe{nxt.offset_from(cur) as libc::c_long !=
            2 as libc::c_int as libc::c_long} {
            return 0 as libc::c_int
        }
        current_block = 15970415187932728765;
    }
}
}
}
}
match current_block {
13163178004963364532 => {
if unsafe{*nxt.offset(1 as libc::c_int as isize) as libc::c_int >=
'0' as i32 &&
*nxt.offset(1 as libc::c_int as isize) as libc::c_int <=
'9' as i32 &&
(*nxt.offset(2 as libc::c_int as isize) as libc::c_int >=
'0' as i32 &&
*nxt.offset(2 as libc::c_int as isize) as libc::c_int
    <= '9' as i32)} {
nxt = unsafe{nxt.offset(3 as libc::c_int as isize)}
} else { return 0 as libc::c_int }
current_block = 15970415187932728765;
}
_ => { }
}
match current_block {
15970415187932728765 =>
/* we parsed a region */
{
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int ==
0 as libc::c_int} {
return 1 as libc::c_int
}
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int !=
'-' as i32} {
return 0 as libc::c_int
}
nxt = unsafe{nxt.offset(1)};
cur = nxt;
/* now we can just have a variant */
while unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int >=
'A' as i32 &&
*nxt.offset(0 as libc::c_int as isize) as libc::c_int <=
    'Z' as i32 ||
*nxt.offset(0 as libc::c_int as isize) as libc::c_int >=
    'a' as i32 &&
    *nxt.offset(0 as libc::c_int as isize) as
        libc::c_int <= 'z' as i32} {
nxt = unsafe{nxt.offset(1)};
}
if unsafe{(nxt.offset_from(cur) as libc::c_long) <
5 as libc::c_int as libc::c_long ||
nxt.offset_from(cur) as libc::c_long >
    8 as libc::c_int as libc::c_long} {
return 0 as libc::c_int
}
}
_ => { }
}
/* we parsed a variant */
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int ==
0 as libc::c_int} {
return 1 as libc::c_int
}
if unsafe{*nxt.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32} {
return 0 as libc::c_int
}
/* extensions and private use subtags not checked */
return 1 as libc::c_int;
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
fn nsPush(mut ctxt: xmlParserCtxtPtr,
mut prefix: *const xmlChar,
mut URL: *const xmlChar) -> libc::c_int {
let mut safe_ctxt = unsafe { &mut *ctxt };
if (safe_ctxt).options & XML_PARSE_NSCLEAN as libc::c_int != 0 {
let mut i: libc::c_int = 0;
i = (safe_ctxt).nsNr - 2 as libc::c_int;
while i >= 0 as libc::c_int {
if unsafe { *(*ctxt).nsTab.offset(i as isize) == prefix } {
/* in scope */
if unsafe { *(*ctxt).nsTab.offset((i + 1 as libc::c_int) as isize) == URL } {
return -(2 as libc::c_int)
}
break;
} else { i -= 2 as libc::c_int }
}
}
if (safe_ctxt).nsMax == 0 as libc::c_int || (safe_ctxt).nsTab.is_null() {
(safe_ctxt).nsMax = 10 as libc::c_int;
(safe_ctxt).nsNr = 0 as libc::c_int;
(safe_ctxt).nsTab =
xmlMalloc_safe(((safe_ctxt).nsMax as
libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
as
libc::c_ulong))
as *mut *const xmlChar;
if (safe_ctxt).nsTab.is_null() {
unsafe {
xmlErrMemory(ctxt, 0 as *const libc::c_char);
}
(safe_ctxt).nsMax = 0 as libc::c_int;
return -(1 as libc::c_int)
}
} else if (safe_ctxt).nsNr >= (safe_ctxt).nsMax {
let mut tmp: *mut *const xmlChar = 0 as *mut *const xmlChar;
(safe_ctxt).nsMax *= 2 as libc::c_int;
tmp =
xmlRealloc_safe((safe_ctxt).nsTab as
            *mut libc::c_char
            as
            *mut libc::c_void,
        ((safe_ctxt).nsMax as
            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
            as
            libc::c_ulong))
as *mut *const xmlChar;
if tmp.is_null() {
unsafe {
xmlErrMemory(ctxt, 0 as *const libc::c_char);
}
(safe_ctxt).nsMax /= 2 as libc::c_int;
return -(1 as libc::c_int)
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
fn nsPop(mut ctxt: xmlParserCtxtPtr, mut nr: libc::c_int)
-> libc::c_int {
let mut safe_ctxt = unsafe { &mut *ctxt };
let mut i: libc::c_int = 0; /* allow for 10 attrs by default */
if (safe_ctxt).nsTab.is_null() { return 0 as libc::c_int }
if (safe_ctxt).nsNr < nr {
unsafe{
(*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                   b"Pbm popping %d NS\n\x00"
                                                       as
                                                       *const u8
                                                       as
                                                       *const libc::c_char,
                                                   nr);
}
nr = (safe_ctxt).nsNr
}
if (safe_ctxt).nsNr <= 0 as libc::c_int { return 0 as libc::c_int }
i = 0 as libc::c_int;
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
fn xmlCtxtGrowAttrs(mut ctxt: xmlParserCtxtPtr,
       mut nr: libc::c_int) -> libc::c_int {
let mut safe_ctxt = unsafe { &mut *ctxt };
let mut current_block: u64;
let mut atts: *mut *const xmlChar = 0 as *mut *const xmlChar;
let mut attallocs: *mut libc::c_int = 0 as *mut libc::c_int;
let mut maxatts: libc::c_int = 0;
if (safe_ctxt).atts.is_null() {
maxatts = 55 as libc::c_int;
atts =
xmlMalloc_safe((maxatts as
libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
as
libc::c_ulong))
as *mut *const xmlChar;
if atts.is_null() {
current_block = 1220566974040888119;
} else {
(safe_ctxt).atts = atts;
attallocs =
xmlMalloc_safe(((maxatts /
5 as
    libc::c_int)
as
libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
as
libc::c_ulong))
as *mut libc::c_int;
if attallocs.is_null() {
current_block = 1220566974040888119;
} else {
(safe_ctxt).attallocs = attallocs;
(safe_ctxt).maxatts = maxatts;
current_block = 13242334135786603907;
}
}
} else if nr + 5 as libc::c_int > (safe_ctxt).maxatts {
maxatts = (nr + 5 as libc::c_int) * 2 as libc::c_int;
atts =
xmlRealloc_safe((safe_ctxt).atts as
                                           *mut libc::c_void,
                                       (maxatts as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
                                           as
                                           libc::c_ulong))
as *mut *const xmlChar;
if atts.is_null() {
current_block = 1220566974040888119;
} else {
(safe_ctxt).atts = atts;
attallocs =
xmlRealloc_safe((safe_ctxt).attallocs
                                               as
                                               *mut libc::c_void,
                                           ((maxatts /
                                               5 as
                                                   libc::c_int)
                                               as
                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                               as
                                               libc::c_ulong))
as *mut libc::c_int;
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
xmlErrMemory(ctxt, 0 as *const libc::c_char);
}
return -(1 as libc::c_int)
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

pub fn inputPush_parser(mut ctxt: xmlParserCtxtPtr,
    mut value: xmlParserInputPtr)
    -> libc::c_int {
if ctxt.is_null() || value.is_null() { return -(1 as libc::c_int) }
let mut safe_ctxt = unsafe { &mut *ctxt };
if (safe_ctxt).inputNr >= (safe_ctxt).inputMax {
(safe_ctxt).inputMax *= 2 as libc::c_int;
(safe_ctxt).inputTab =
xmlRealloc_safe((safe_ctxt).inputTab as
                                           *mut libc::c_void,
                                       ((safe_ctxt).inputMax
                                           as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlParserInputPtr>()
                                           as
                                           libc::c_ulong))
as *mut xmlParserInputPtr;
if (safe_ctxt).inputTab.is_null() {
unsafe{
xmlErrMemory(ctxt, 0 as *const libc::c_char);
}
xmlFreeInputStream_safe(value);
(safe_ctxt).inputMax /= 2 as libc::c_int;
value = 0 as xmlParserInputPtr;
return -(1 as libc::c_int)
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

pub fn inputPop_parser(mut ctxt: xmlParserCtxtPtr)
   -> xmlParserInputPtr {
let mut ret: xmlParserInputPtr = 0 as *mut xmlParserInput;
if ctxt.is_null() { return 0 as xmlParserInputPtr }
let mut safe_ctxt = unsafe { &mut *ctxt };
if (safe_ctxt).inputNr <= 0 as libc::c_int { return 0 as xmlParserInputPtr }
(safe_ctxt).inputNr -= 1;
if (safe_ctxt).inputNr > 0 as libc::c_int {
(safe_ctxt).input = unsafe{*(*ctxt).inputTab.offset(((safe_ctxt).inputNr - 1 as libc::c_int) as isize)};
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

pub fn nodePush(mut ctxt: xmlParserCtxtPtr,
   mut value: xmlNodePtr) -> libc::c_int {
if ctxt.is_null() { return 0 as libc::c_int }
let mut safe_ctxt = unsafe { &mut *ctxt };
if (safe_ctxt).nodeNr >= (safe_ctxt).nodeMax {
let mut tmp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
tmp =
xmlRealloc_safe((safe_ctxt).nodeTab as
                                           *mut libc::c_void,
                                       (((safe_ctxt).nodeMax *
                                           2 as
                                               libc::c_int)
                                           as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                           as
                                           libc::c_ulong))
as *mut xmlNodePtr;
if tmp.is_null() {
unsafe{
xmlErrMemory(ctxt, 0 as *const libc::c_char);
}
return -(1 as libc::c_int)
}
(safe_ctxt).nodeTab = tmp;
(safe_ctxt).nodeMax *= 2 as libc::c_int
}
if (safe_ctxt).nodeNr as libc::c_uint > unsafe{xmlParserMaxDepth} &&
(safe_ctxt).options & XML_PARSE_HUGE as libc::c_int == 0 as libc::c_int
{
unsafe{
xmlFatalErrMsgInt(ctxt, XML_ERR_INTERNAL_ERROR,
          b"Excessive depth in document: %d use XML_PARSE_HUGE option\n\x00"
              as *const u8 as *const libc::c_char,
          xmlParserMaxDepth as libc::c_int);
xmlHaltParser(ctxt);
}
return -(1 as libc::c_int)
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

pub fn nodePop_parser(mut ctxt: xmlParserCtxtPtr) -> xmlNodePtr {
let mut ret: xmlNodePtr = 0 as *mut xmlNode;
if ctxt.is_null() { return 0 as xmlNodePtr }
let mut safe_ctxt = unsafe { &mut *ctxt };
if (safe_ctxt).nodeNr <= 0 as libc::c_int { return 0 as xmlNodePtr }
(safe_ctxt).nodeNr -= 1;
if (safe_ctxt).nodeNr > 0 as libc::c_int {
(safe_ctxt).node = unsafe{*(*ctxt).nodeTab.offset(((safe_ctxt).nodeNr - 1 as libc::c_int) as isize)};
} else { (safe_ctxt).node = 0 as xmlNodePtr }
unsafe{
ret = *(*ctxt).nodeTab.offset((safe_ctxt).nodeNr as isize);
let ref mut fresh19 = *(*ctxt).nodeTab.offset((safe_ctxt).nodeNr as isize);
*fresh19 = 0 as xmlNodePtr;
}
return ret;
}