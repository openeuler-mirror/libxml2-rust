#[no_mangle]
pub unsafe extern "C" fn xmlXPathContextSetCache_rust(
    mut ctxt: xmlXPathContextPtr,
    mut active: i32,
    mut value: i32,
    mut options: i32,
) -> i32 {
    let res: i32 = unsafe { xmlXPathContextSetCache(ctxt, active, value, options) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn valuePop_rust(mut ctxt: xmlXPathParserContextPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { valuePop(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn valuePush_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut value: xmlXPathObjectPtr,
) -> i32 {
    let res: i32 = unsafe { valuePush(ctxt, value) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopBoolean_rust(mut ctxt: xmlXPathParserContextPtr) -> i32 {
    let res: i32 = unsafe { xmlXPathPopBoolean(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopNumber_rust(mut ctxt: xmlXPathParserContextPtr) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathPopNumber(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopString_rust(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathPopString(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopExternal_rust(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut () {
    let res: *mut () = unsafe { xmlXPathPopExternal(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathOrderDocElems_rust(mut doc: xmlDocPtr) -> i64 {
    let res: i64 = unsafe { xmlXPathOrderDocElems(doc) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCmpNodes_rust(
    mut node1: xmlNodePtr,
    mut node2: xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathCmpNodes(node1, node2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetSort_rust(mut set: xmlNodeSetPtr) {
    unsafe { xmlXPathNodeSetSort(set) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetFreeNs_rust(mut ns: xmlNsPtr) {
    unsafe { xmlXPathNodeSetFreeNs(ns) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetCreate_rust(mut val: xmlNodePtr) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeSetCreate(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetContains_rust(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathNodeSetContains(cur, val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetAddNs_rust(
    mut cur: xmlNodeSetPtr,
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathNodeSetAddNs(cur, node, ns) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetAdd_rust(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathNodeSetAdd(cur, val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetAddUnique_rust(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathNodeSetAddUnique(cur, val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetMerge_rust(
    mut val1: xmlNodeSetPtr,
    mut val2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeSetMerge(val1, val2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetDel_rust(mut cur: xmlNodeSetPtr, mut val: xmlNodePtr) {
    unsafe { xmlXPathNodeSetDel(cur, val) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetRemove_rust(mut cur: xmlNodeSetPtr, mut val: i32) {
    unsafe { xmlXPathNodeSetRemove(cur, val) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeNodeSet_rust(mut obj: xmlNodeSetPtr) {
    unsafe { xmlXPathFreeNodeSet(obj) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewNodeSetList_rust(mut val: xmlNodeSetPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewNodeSetList(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDifference_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathDifference(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathIntersection_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathIntersection(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDistinctSorted_rust(mut nodes: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathDistinctSorted(nodes) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDistinct_rust(mut nodes: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathDistinct(nodes) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathHasSameNodes_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathHasSameNodes(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeLeadingSorted_rust(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeLeadingSorted(nodes, node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeLeading_rust(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeLeading(nodes, node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathLeadingSorted_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathLeadingSorted(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathLeading_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathLeading(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeTrailingSorted_rust(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeTrailingSorted(nodes, node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeTrailing_rust(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathNodeTrailing(nodes, node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathTrailingSorted_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathTrailingSorted(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathTrailing_rust(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathTrailing(nodes1, nodes2) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterFunc_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut f: xmlXPathFunction,
) -> i32 {
    let res: i32 = unsafe { xmlXPathRegisterFunc(ctxt, name, f) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathErr_rust(mut ctxt: xmlXPathParserContextPtr, mut error: i32) {
    unsafe { xmlXPathErr(ctxt, error) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPatherror_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut file: *const i8,
    mut line: i32,
    mut no: i32,
) {
    unsafe { xmlXPatherror(ctxt, file, line, no) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopNodeSet_rust(mut ctxt: xmlXPathParserContextPtr) -> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe { xmlXPathPopNodeSet(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeCompExpr_rust(mut comp: xmlXPathCompExprPtr) {
    unsafe { xmlXPathFreeCompExpr(comp) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathIsNaN_rust(mut val: libc::c_double) -> i32 {
    let res: i32 = unsafe { xmlXPathIsNaN(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathIsInf_rust(mut val: libc::c_double) -> i32 {
    let res: i32 = unsafe { xmlXPathIsInf(val) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDebugDumpObject_rust(
    mut output: *mut FILE,
    mut cur: xmlXPathObjectPtr,
    mut depth: i32,
) {
    unsafe { xmlXPathDebugDumpObject(output, cur, depth) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathInit_rust() {
    unsafe { xmlXPathInit_xpath() };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeToNumber_rust(mut node: xmlNodePtr) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathCastNodeToNumber(node) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeSetToNumber_rust(mut ns: xmlNodeSetPtr) -> libc::c_double {

    let res: libc::c_double = unsafe { xmlXPathCastNodeSetToNumber(ns) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathStringEvalNumber_rust(mut str: *const xmlChar) -> libc::c_double {

    let res: libc::c_double = unsafe { xmlXPathStringEvalNumber(str) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastToNumber_rust(mut val: xmlXPathObjectPtr) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathCastToNumber(val) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastToBoolean_rust(mut val: xmlXPathObjectPtr) -> i32 {
    let res: i32 = unsafe { xmlXPathCastToBoolean(val) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDebugDumpCompExpr_rust(
    mut output: *mut FILE,
    mut comp: xmlXPathCompExprPtr,
    mut depth: i32,
) {
    unsafe { xmlXPathDebugDumpCompExpr(output, comp, depth) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextNamespace_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextNamespace(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterFuncLookup_rust(
    mut ctxt: xmlXPathContextPtr,
    mut f: xmlXPathFuncLookupFunc,
    mut funcCtxt: *mut (),
) {
    unsafe { xmlXPathRegisterFuncLookup(ctxt, f, funcCtxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFunctionLookup_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
) -> xmlXPathFunction {
    let res: xmlXPathFunction = unsafe { xmlXPathFunctionLookup(ctxt, name) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterFuncNS_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut f: xmlXPathFunction,
) -> i32 {
    let res: i32 = unsafe { xmlXPathRegisterFuncNS(ctxt, name, ns_uri, f) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisteredFuncsCleanup_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathRegisteredFuncsCleanup(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterVariableNS_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut value: xmlXPathObjectPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathRegisterVariableNS(ctxt, name, ns_uri, value) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterVariableLookup_rust(
    mut ctxt: xmlXPathContextPtr,
    mut f: xmlXPathVariableLookupFunc,
    mut data: *mut (),
) {
    unsafe { xmlXPathRegisterVariableLookup(ctxt, f, data) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathVariableLookup_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathVariableLookup(ctxt, name) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathVariableLookupNS_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathVariableLookupNS(ctxt, name, ns_uri) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisteredVariablesCleanup_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathRegisteredVariablesCleanup(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNsLookup_rust(
    mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
) -> *const xmlChar {
    let res: *const xmlChar = unsafe { xmlXPathNsLookup(ctxt, prefix) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterNs_rust(
    mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> i32 {
    let res: i32 = unsafe { xmlXPathRegisterNs(ctxt, prefix, ns_uri) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFunctionLookupNS_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathFunction {
    let res: xmlXPathFunction = unsafe { xmlXPathFunctionLookupNS(ctxt, name, ns_uri) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNumberToString_rust(mut val: libc::c_double) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathCastNumberToString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeContext_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathFreeContext(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathConvertBoolean_rust(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathConvertBoolean(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvalExpr_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathEvalExpr(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompiledEval_rust(
    mut comp: xmlXPathCompExprPtr,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathCompiledEval(comp, ctx) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompiledEvalToBoolean_rust(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathCompiledEvalToBoolean(comp, ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompile_rust(mut str: *const xmlChar) -> xmlXPathCompExprPtr {
    let res: xmlXPathCompExprPtr = unsafe { xmlXPathCompile(str) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCtxtCompile_rust(
    mut ctxt: xmlXPathContextPtr,
    mut str: *const xmlChar,
) -> xmlXPathCompExprPtr {
    let res: xmlXPathCompExprPtr = unsafe { xmlXPathCtxtCompile(ctxt, str) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewParserContext_rust(
    mut str: *const xmlChar,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathParserContextPtr {
    let res: xmlXPathParserContextPtr = unsafe { xmlXPathNewParserContext(str, ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeEval_rust(
    mut node: xmlNodePtr,
    mut str: *const xmlChar,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNodeEval(node, str, ctx) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSetContextNode_rust(
    mut node: xmlNodePtr,
    mut ctx: xmlXPathContextPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathSetContextNode(node, ctx) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvalPredicate_rust(
    mut ctxt: xmlXPathContextPtr,
    mut res: xmlXPathObjectPtr,
) -> i32 {
    let result: i32 = unsafe { xmlXPathEvalPredicate(ctxt, res) };
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvaluatePredicateResult_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut res: xmlXPathObjectPtr,
) -> i32 {
    let result: i32 = unsafe { xmlXPathEvaluatePredicateResult(ctxt, res) };
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathIsNodeType_rust(mut name: *const xmlChar) -> i32 {
    let result: i32 = unsafe { xmlXPathIsNodeType(name) };
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathParseNCName_rust(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathParseNCName(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathParseName_rust(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathParseName(ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRoundFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathRoundFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFloorFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathFloorFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCeilingFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathCeilingFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSumFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathSumFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextAncestor_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextAncestor(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextAncestorOrSelf_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextAncestorOrSelf(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextFollowingSibling_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextFollowingSibling(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextPrecedingSibling_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextPrecedingSibling(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextFollowing_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextFollowing(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextPreceding_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextPreceding(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextAttribute_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextAttribute(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRoot_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathRoot(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCountFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathCountFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathIdFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathIdFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathLocalNameFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathLocalNameFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNamespaceURIFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathNamespaceURIFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathStringFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathStringFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathStringLengthFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathStringLengthFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathConcatFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathConcatFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathContainsFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathContainsFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathStartsWithFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathStartsWithFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubstringFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathSubstringFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubstringBeforeFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathSubstringBeforeFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNormalizeFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathNormalizeFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathTranslateFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathTranslateFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathBooleanFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathBooleanFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNotFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathNotFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathTrueFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathTrueFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFalseFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathFalseFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathLangFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathLangFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubstringAfterFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathSubstringAfterFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNumberFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathNumberFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathEval_rust(
    mut str: *const xmlChar,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathEval(str, ctx) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvalExpression_rust(
    mut str: *const xmlChar,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathEvalExpression(str, ctxt) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterAllFunctions_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathRegisterAllFunctions(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastToString_rust(mut val: xmlXPathObjectPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathCastToString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathConvertString_rust(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathConvertString(val) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextDescendantOrSelf_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextDescendantOrSelf(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextDescendant_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextDescendant(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewContext_rust(mut doc: xmlDocPtr) -> xmlXPathContextPtr {
    let res: xmlXPathContextPtr = unsafe { xmlXPathNewContext(doc) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeParserContext_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathFreeParserContext(ctxt) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathEqualValues_rust(mut ctxt: xmlXPathParserContextPtr) -> i32 {
    let res: i32 = unsafe { xmlXPathEqualValues(ctxt) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNotEqualValues_rust(mut ctxt: xmlXPathParserContextPtr) -> i32 {
    let res: i32 = unsafe { xmlXPathNotEqualValues(ctxt) };

    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextChild_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextChild(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextParent_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextParent(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathLastFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathLastFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathPositionFunction_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: i32,
) {
    unsafe { xmlXPathPositionFunction(ctxt, nargs) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompareValues_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: i32,
    mut strict: i32,
) -> i32 {
    let res: i32 = unsafe { xmlXPathCompareValues(ctxt, inf, strict) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathAddValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathAddValues(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathSubValues(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathModValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathModValues(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathDivValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathDivValues(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathMultValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathMultValues(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathValueFlipSign_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathValueFlipSign(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextSelf_rust(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let res: xmlNodePtr = unsafe { xmlXPathNextSelf(ctxt, cur) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeSetToBoolean_rust(mut ns: xmlNodeSetPtr) -> i32 {
    let res: i32 = unsafe { xmlXPathCastNodeSetToBoolean(ns) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastStringToBoolean_rust(mut val: *const xmlChar) -> i32 {
    let res: i32 = unsafe { xmlXPathCastStringToBoolean(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNumberToBoolean_rust(mut val: libc::c_double) -> i32 {
    let res: i32 = unsafe { xmlXPathCastNumberToBoolean(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathConvertNumber_rust(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathConvertNumber(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeToString_rust(mut node: xmlNodePtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathCastNodeToString(node) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastBooleanToString_rust(mut val: i32) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathCastBooleanToString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisteredNsCleanup_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathRegisteredNsCleanup(ctxt) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeObject_rust(mut obj: xmlXPathObjectPtr) {
    unsafe { xmlXPathFreeObject(obj) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewFloat_rust(mut val: libc::c_double) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewFloat(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewBoolean_rust(mut val: i32) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewBoolean(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewString_rust(mut val: *const xmlChar) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapString_rust(mut val: *mut xmlChar) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathWrapString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewCString_rust(mut val: *const i8) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewCString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastStringToNumber_rust(mut val: *const xmlChar) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathCastStringToNumber(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastBooleanToNumber_rust(mut val: i32) -> libc::c_double {
    let res: libc::c_double = unsafe { xmlXPathCastBooleanToNumber(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeSetToString_rust(mut ns: xmlNodeSetPtr) -> *mut xmlChar {
    let res: *mut xmlChar = unsafe { xmlXPathCastNodeSetToString(ns) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapCString_rust(mut val: *mut i8) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathWrapCString(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapNodeSet_rust(mut val: xmlNodeSetPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathWrapNodeSet(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeNodeSetList_rust(mut obj: xmlXPathObjectPtr) {
    unsafe { xmlXPathFreeNodeSetList(obj) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewValueTree_rust(mut val: xmlNodePtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewValueTree(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewNodeSet_rust(mut val: xmlNodePtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathNewNodeSet(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapExternal_rust(mut val: *mut ()) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathWrapExternal(val) };
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathObjectCopy_rust(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe { xmlXPathObjectCopy(val) };
    return res;
}

#[no_mangle]
#[cfg(DEBUG_OR_DEBUG_STEP)]
pub unsafe extern "C" fn xmlGenericErrorContextNodeSet_rust(
    mut output: *mut FILE,
    mut obj: xmlNodeSetPtr,
) {
    unsafe { xmlGenericErrorContextNodeSet(output, obj) };
}

#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterVariable_rust(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut value: xmlXPathObjectPtr,
) -> i32 {
    let res: i32 = unsafe { xmlXPathRegisterVariable(ctxt, name, value) };
    return res;
}
