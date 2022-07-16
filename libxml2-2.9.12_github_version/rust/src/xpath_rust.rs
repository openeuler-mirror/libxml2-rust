#[no_mangle]
pub extern "C" fn xmlXPathContextSetCache_rust(mut ctxt: xmlXPathContextPtr,
    mut active: libc::c_int,
    mut value: libc::c_int,
    mut options: libc::c_int)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathContextSetCache(ctxt, active, value, options)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn valuePop_rust(mut ctxt: xmlXPathParserContextPtr)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        valuePop(ctxt)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn valuePush_rust(mut ctxt: xmlXPathParserContextPtr,
    mut value: xmlXPathObjectPtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        valuePush(ctxt, value)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathPopBoolean_rust(mut ctxt:
    xmlXPathParserContextPtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathPopBoolean(ctxt)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathPopNumber_rust(mut ctxt: xmlXPathParserContextPtr)
-> libc::c_double {
    let res: libc::c_double = unsafe {
        xmlXPathPopNumber(ctxt)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathPopString_rust(mut ctxt: xmlXPathParserContextPtr)
-> *mut xmlChar {
    let res: *mut xmlChar = unsafe {
        xmlXPathPopString(ctxt)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathPopExternal_rust(mut ctxt:
    xmlXPathParserContextPtr)
-> *mut libc::c_void {
    let res: *mut libc::c_void = unsafe {
        xmlXPathPopExternal(ctxt)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathOrderDocElems_rust(mut doc: xmlDocPtr)
-> libc::c_long {
    let res: libc::c_long = unsafe {
        xmlXPathOrderDocElems(doc)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCmpNodes_rust(mut node1: xmlNodePtr,
    mut node2: xmlNodePtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathCmpNodes(node1, node2)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeSetSort_rust(mut set: xmlNodeSetPtr) {
    unsafe {
        xmlXPathNodeSetSort(set)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeSetFreeNs_rust(mut ns: xmlNsPtr) {
    unsafe {
        xmlXPathNodeSetFreeNs(ns)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeSetCreate_rust(mut val: xmlNodePtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathNodeSetCreate(val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeSetContains_rust(mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathNodeSetContains(cur, val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeSetAddNs_rust(mut cur: xmlNodeSetPtr,
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathNodeSetAddNs(cur, node, ns)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeSetAdd_rust(mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathNodeSetAdd(cur, val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeSetAddUnique_rust(mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathNodeSetAddUnique(cur, val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeSetMerge_rust(mut val1: xmlNodeSetPtr,
    mut val2: xmlNodeSetPtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathNodeSetMerge(val1, val2)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeSetDel_rust(mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr) {
    unsafe {
        xmlXPathNodeSetDel(cur, val)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeSetRemove_rust(mut cur: xmlNodeSetPtr,
    mut val: libc::c_int) {
    unsafe {
        xmlXPathNodeSetRemove(cur, val)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathFreeNodeSet_rust(mut obj: xmlNodeSetPtr) {
    unsafe {
        xmlXPathFreeNodeSet(obj)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathNewNodeSetList_rust(mut val: xmlNodeSetPtr)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathNewNodeSetList(val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathDifference_rust(mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathDifference(nodes1, nodes2)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathIntersection_rust(mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathIntersection(nodes1, nodes2)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathDistinctSorted_rust(mut nodes: xmlNodeSetPtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathDistinctSorted(nodes)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathDistinct_rust(mut nodes: xmlNodeSetPtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathDistinct(nodes)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathHasSameNodes_rust(mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathHasSameNodes(nodes1, nodes2)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeLeadingSorted_rust(mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathNodeLeadingSorted(nodes, node)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeLeading_rust(mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathNodeLeading(nodes, node)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathLeadingSorted_rust(mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathLeadingSorted(nodes1, nodes2)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathLeading_rust(mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathLeading(nodes1, nodes2)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeTrailingSorted_rust(mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathNodeTrailingSorted(nodes, node)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeTrailing_rust(mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathNodeTrailing(nodes, node)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathTrailingSorted_rust(mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathTrailingSorted(nodes1, nodes2)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathTrailing_rust(mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathTrailing(nodes1, nodes2)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathRegisterFunc_rust(mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut f: xmlXPathFunction)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathRegisterFunc(ctxt, name, f)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathErr_rust(mut ctxt: xmlXPathParserContextPtr,
    mut error: libc::c_int) {
    unsafe {
        xmlXPathErr(ctxt, error)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPatherror_rust(mut ctxt: xmlXPathParserContextPtr,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut no: libc::c_int) {
    unsafe {
        xmlXPatherror(ctxt, file, line, no)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathPopNodeSet_rust(mut ctxt:
    xmlXPathParserContextPtr)
-> xmlNodeSetPtr {
    let res: xmlNodeSetPtr = unsafe {
        xmlXPathPopNodeSet(ctxt)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathFreeCompExpr_rust(mut comp: xmlXPathCompExprPtr) {
    unsafe {
        xmlXPathFreeCompExpr(comp)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathIsNaN_rust(mut val: libc::c_double)
-> libc::c_int {
    println!("xmlXPathIsNaN_rust pre");
    let res: libc::c_int = unsafe {
        xmlXPathIsNaN(val)
    };
    println!("xmlXPathIsNaN_rust next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathIsInf_rust(mut val: libc::c_double)
-> libc::c_int {
    println!("xmlXPathIsInf_rust pre");
    let res: libc::c_int = unsafe {
        xmlXPathIsInf(val)
    };
    println!("xmlXPathIsInf_rust next");

    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathDebugDumpObject_rust(mut output: *mut FILE,
    mut cur: xmlXPathObjectPtr,
    mut depth: libc::c_int) {
    unsafe {
        xmlXPathDebugDumpObject(output, cur, depth)
    };
}


#[no_mangle]
pub extern "C" fn xmlXPathInit_rust() {
    println!("xmlXPathInit before");
    unsafe {
        xmlXPathInit_xpath()
    };
    println!("xmlXPathInit after");
}

#[no_mangle]
pub extern "C" fn xmlXPathCastNodeToNumber_rust(mut node: xmlNodePtr)
-> libc::c_double {
    println!("xmlXPathCastNodeToNumber before");
    let res: libc::c_double = unsafe {
        xmlXPathCastNodeToNumber(node)
    };
    println!("xmlXPathCastNodeToNumber after");

    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastNodeSetToNumber_rust(mut ns: xmlNodeSetPtr)
-> libc::c_double {
    println!("xmlXPathCastNodeSetToNumber before");

    let res: libc::c_double = unsafe {
        xmlXPathCastNodeSetToNumber(ns)
    };
    println!("xmlXPathCastNodeSetToNumber after");

    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathStringEvalNumber_rust(mut str: *const xmlChar)
-> libc::c_double {
    println!("xmlXPathStringEvalNumber_rust before");

    let res: libc::c_double = unsafe {
        xmlXPathStringEvalNumber(str)
    };
    println!("xmlXPathStringEvalNumber_rust after");

    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastToNumber_rust(mut val: xmlXPathObjectPtr)
-> libc::c_double {
    println!("xmlXPathCastToNumber_rust pre");
    let res: libc::c_double = unsafe {
        xmlXPathCastToNumber(val)
    };
    println!("xmlXPathCastToNumber_rust next");

    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastToBoolean_rust(mut val: xmlXPathObjectPtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathCastToBoolean(val)
    };

    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathDebugDumpCompExpr_rust(mut output: *mut FILE,
    mut comp:
        xmlXPathCompExprPtr,
    mut depth: libc::c_int) {
    unsafe {
        xmlXPathDebugDumpCompExpr(output, comp, depth)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathNextNamespace_rust(mut ctxt:
    xmlXPathParserContextPtr,
mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextNamespace(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathRegisterFuncLookup_rust(mut ctxt:
    xmlXPathContextPtr,
mut f:
    xmlXPathFuncLookupFunc,
mut funcCtxt:
    *mut libc::c_void) {
    unsafe {
        xmlXPathRegisterFuncLookup(ctxt, f, funcCtxt)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathFunctionLookup_rust(mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar)
-> xmlXPathFunction {
    let res: xmlXPathFunction = unsafe {
        xmlXPathFunctionLookup(ctxt, name)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathRegisterFuncNS_rust(mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut f: xmlXPathFunction)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathRegisterFuncNS(ctxt, name, ns_uri, f)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathRegisteredFuncsCleanup_rust(mut ctxt:
    xmlXPathContextPtr) {
    unsafe {
        xmlXPathRegisteredFuncsCleanup(ctxt)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathRegisterVariableNS_rust(mut ctxt:
    xmlXPathContextPtr,
mut name: *const xmlChar,
mut ns_uri:
    *const xmlChar,
mut value:
    xmlXPathObjectPtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathRegisterVariableNS(ctxt, name, ns_uri, value)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathRegisterVariableLookup_rust(mut ctxt:
    xmlXPathContextPtr,
mut f:
    xmlXPathVariableLookupFunc,
mut data:
    *mut libc::c_void) {
    unsafe {
        xmlXPathRegisterVariableLookup(ctxt, f, data)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathVariableLookup_rust(mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathVariableLookup(ctxt, name)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathVariableLookupNS_rust(mut ctxt:
    xmlXPathContextPtr,
mut name: *const xmlChar,
mut ns_uri: *const xmlChar)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathVariableLookupNS(ctxt, name, ns_uri)
    };
    return res;
}


#[no_mangle]
pub extern "C" fn xmlXPathRegisteredVariablesCleanup_rust(mut ctxt:
    xmlXPathContextPtr) {
    unsafe {
        xmlXPathRegisteredVariablesCleanup(ctxt)
    };
}


#[no_mangle]
pub extern "C" fn xmlXPathNsLookup_rust(mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar)
-> *const xmlChar {
    let res: *const xmlChar = unsafe {
        xmlXPathNsLookup(ctxt, prefix)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathRegisterNs_rust(mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
    mut ns_uri: *const xmlChar)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathRegisterNs(ctxt, prefix, ns_uri)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathFunctionLookupNS_rust(mut ctxt:
    xmlXPathContextPtr,
mut name: *const xmlChar,
mut ns_uri: *const xmlChar)
-> xmlXPathFunction {
    let res: xmlXPathFunction = unsafe {
        xmlXPathFunctionLookupNS(ctxt, name, ns_uri)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastNumberToString_rust(mut val: libc::c_double)
-> *mut xmlChar {
    let res: *mut xmlChar = unsafe {
        xmlXPathCastNumberToString(val)
    };
    return res;
}


#[no_mangle]
pub extern "C" fn xmlXPathFreeContext_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe {
        xmlXPathFreeContext(ctxt)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathConvertBoolean_rust(mut val: xmlXPathObjectPtr)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathConvertBoolean(val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathEvalExpr_rust(mut ctxt:
    xmlXPathParserContextPtr) {
    unsafe {
        xmlXPathEvalExpr(ctxt)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathCompiledEval_rust(mut comp: xmlXPathCompExprPtr,
    mut ctx: xmlXPathContextPtr)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathCompiledEval(comp, ctx)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCompiledEvalToBoolean_rust(mut comp:
    xmlXPathCompExprPtr,
mut ctxt:
    xmlXPathContextPtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathCompiledEvalToBoolean(comp, ctxt)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCompile_rust(mut str: *const xmlChar)
-> xmlXPathCompExprPtr {
    let res: xmlXPathCompExprPtr = unsafe {
        xmlXPathCompile(str)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCtxtCompile_rust(mut ctxt: xmlXPathContextPtr,
    mut str: *const xmlChar)
-> xmlXPathCompExprPtr {
    let res: xmlXPathCompExprPtr = unsafe {
        xmlXPathCtxtCompile(ctxt, str)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNewParserContext_rust(mut str: *const xmlChar,
    mut ctxt:
        xmlXPathContextPtr)
-> xmlXPathParserContextPtr {
    let res: xmlXPathParserContextPtr = unsafe {
        xmlXPathNewParserContext(str, ctxt)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNodeEval_rust(mut node: xmlNodePtr,
    mut str: *const xmlChar, 
    mut ctx: xmlXPathContextPtr)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathNodeEval(node, str, ctx)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathSetContextNode_rust(mut node: xmlNodePtr,
    mut ctx: xmlXPathContextPtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathSetContextNode(node, ctx)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathEvalPredicate_rust(mut ctxt: xmlXPathContextPtr, 
    mut res: xmlXPathObjectPtr)
-> libc::c_int {
    let result: libc::c_int = unsafe {
        xmlXPathEvalPredicate(ctxt, res)
    };
    return result;
}

#[no_mangle]
pub extern "C" fn xmlXPathEvaluatePredicateResult_rust(mut ctxt: xmlXPathParserContextPtr, 
    mut res: xmlXPathObjectPtr)
-> libc::c_int {
    let result: libc::c_int = unsafe {
        xmlXPathEvaluatePredicateResult(ctxt, res)
    };
    return result;
}

#[no_mangle]
pub extern "C" fn xmlXPathIsNodeType_rust(mut name: *const xmlChar)
-> libc::c_int {
    let result: libc::c_int = unsafe {
        xmlXPathIsNodeType(name)
    };
    return result;
}

#[no_mangle]
pub extern "C" fn xmlXPathParseNCName_rust(mut ctxt: xmlXPathParserContextPtr)
-> *mut xmlChar {
    let res: *mut xmlChar = unsafe {
        xmlXPathParseNCName(ctxt)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathParseName_rust(mut ctxt: xmlXPathParserContextPtr)
-> *mut xmlChar {
    let res: *mut xmlChar = unsafe {
        xmlXPathParseName(ctxt)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathRoundFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathRoundFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathFloorFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathFloorFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathCeilingFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathCeilingFunction(ctxt, nargs) };

}

#[no_mangle]
pub extern "C" fn xmlXPathSumFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathSumFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathNextAncestor_rust(mut ctxt: xmlXPathParserContextPtr, 
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextAncestor(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNextAncestorOrSelf_rust(mut ctxt: xmlXPathParserContextPtr, 
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextAncestorOrSelf(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNextFollowingSibling_rust(mut ctxt: xmlXPathParserContextPtr, 
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextFollowingSibling(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNextPrecedingSibling_rust(mut ctxt: xmlXPathParserContextPtr, 
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextPrecedingSibling(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNextFollowing_rust(mut ctxt: xmlXPathParserContextPtr, 
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextFollowing(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNextPreceding_rust(mut ctxt: xmlXPathParserContextPtr, 
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextPreceding(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNextAttribute_rust(mut ctxt: xmlXPathParserContextPtr, 
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextAttribute(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathRoot_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathRoot(ctxt) };
}

#[no_mangle]
pub extern "C" fn xmlXPathCountFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathCountFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathIdFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathIdFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathLocalNameFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathLocalNameFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathNamespaceURIFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathNamespaceURIFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathStringFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathStringFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathStringLengthFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathStringLengthFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathConcatFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathConcatFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathContainsFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathContainsFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathStartsWithFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathStartsWithFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathSubstringFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathSubstringFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathSubstringBeforeFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathSubstringBeforeFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathNormalizeFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathNormalizeFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathTranslateFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathTranslateFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathBooleanFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathBooleanFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathNotFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathNotFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathTrueFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathTrueFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathFalseFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathFalseFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathLangFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathLangFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathSubstringAfterFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathSubstringAfterFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathNumberFunction_rust(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    unsafe { xmlXPathNumberFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathEval_rust(mut str: *const xmlChar, 
    mut ctx: xmlXPathContextPtr)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathEval(str, ctx)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathEvalExpression_rust(mut str: *const xmlChar, 
    mut ctxt: xmlXPathContextPtr)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathEvalExpression(str, ctxt)
    };
    return res;
}


#[no_mangle]
pub extern "C" fn xmlXPathRegisterAllFunctions_rust(mut ctxt: xmlXPathContextPtr) {
    unsafe { xmlXPathRegisterAllFunctions(ctxt) };
}

#[no_mangle]
pub extern "C" fn xmlXPathCastToString_rust(mut val: xmlXPathObjectPtr)
-> *mut xmlChar {
    println!("xmlXPathCastToString_rust pre");
    let res: *mut xmlChar = unsafe {
        xmlXPathCastToString(val)
    };
    println!("xmlXPathCastToString_rust next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathConvertString_rust(mut val: xmlXPathObjectPtr)
-> xmlXPathObjectPtr {
    println!("xmlXPathConvertString_rust pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathConvertString(val)
    };
    println!("xmlXPathConvertString_rust next");

    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNextDescendantOrSelf_rust(mut ctxt: xmlXPathParserContextPtr, 
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextDescendantOrSelf(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNextDescendant_rust(mut ctxt: xmlXPathParserContextPtr, 
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextDescendant(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNewContext_rust(mut doc: xmlDocPtr)
-> xmlXPathContextPtr {
    println!("xmlXPathNewContext_rust pre");
    let res: xmlXPathContextPtr = unsafe {
        xmlXPathNewContext(doc)
    };
    println!("xmlXPathNewContext_rust next");
    return res;
}


#[no_mangle]
pub extern "C" fn xmlXPathFreeParserContext_rust(mut ctxt:
    xmlXPathParserContextPtr) {
    println!("xmlXPathFreeParserContext_rust pre");
    unsafe { xmlXPathFreeParserContext(ctxt) };
    println!("xmlXPathFreeParserContext_rust next");

}
#[no_mangle]
pub extern "C" fn xmlXPathEqualValues_rust(mut ctxt:
    xmlXPathParserContextPtr)
-> libc::c_int {
    println!("xmlXPathEqualValues_rust pre");
    let res: libc::c_int = unsafe {
        xmlXPathEqualValues(ctxt)
    };
    println!("xmlXPathEqualValues_rust next");

    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNotEqualValues_rust(mut ctxt:
    xmlXPathParserContextPtr)
-> libc::c_int {
    println!("xmlXPathNotEqualValues_rust pre");
    let res: libc::c_int = unsafe {
        xmlXPathNotEqualValues(ctxt)
    };
    println!("xmlXPathNotEqualValues_rust next");

    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNextChild_rust(mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextChild(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNextParent_rust(mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextParent(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathLastFunction_rust(mut ctxt:
    xmlXPathParserContextPtr,
mut nargs: libc::c_int) {
    unsafe { xmlXPathLastFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathPositionFunction_rust(mut ctxt:
    xmlXPathParserContextPtr,
mut nargs: libc::c_int) {
    unsafe { xmlXPathPositionFunction(ctxt, nargs) };
}

#[no_mangle]
pub extern "C" fn xmlXPathCompareValues_rust(mut ctxt:
    xmlXPathParserContextPtr,
mut inf: libc::c_int,
mut strict: libc::c_int)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathCompareValues(ctxt, inf, strict)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathAddValues_rust(mut ctxt:
    xmlXPathParserContextPtr) {
    unsafe { xmlXPathAddValues(ctxt) };
}

#[no_mangle]
pub extern "C" fn xmlXPathSubValues_rust(mut ctxt:
    xmlXPathParserContextPtr) {
    unsafe { xmlXPathSubValues(ctxt) };
}

#[no_mangle]
pub extern "C" fn xmlXPathModValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe {
        xmlXPathModValues(ctxt)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathDivValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe {
        xmlXPathDivValues(ctxt)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathMultValues_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe {
        xmlXPathMultValues(ctxt)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathValueFlipSign_rust(mut ctxt: xmlXPathParserContextPtr) {
    unsafe {
        xmlXPathValueFlipSign(ctxt)
    };
}

#[no_mangle]
pub extern "C" fn xmlXPathNextSelf_rust(mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr)
-> xmlNodePtr {
    let res: xmlNodePtr = unsafe {
        xmlXPathNextSelf(ctxt, cur)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastNodeSetToBoolean_rust(mut ns: xmlNodeSetPtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathCastNodeSetToBoolean(ns)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastStringToBoolean_rust(mut val: *const xmlChar)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathCastStringToBoolean(val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastNumberToBoolean_rust(mut val: libc::c_double)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathCastNumberToBoolean(val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathConvertNumber_rust(mut val: xmlXPathObjectPtr)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathConvertNumber(val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastNodeToString_rust(mut node: xmlNodePtr)
-> *mut xmlChar {
    let res: *mut xmlChar = unsafe {
        xmlXPathCastNodeToString(node)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastBooleanToString_rust(mut val: libc::c_int)
-> *mut xmlChar {
    let res: *mut xmlChar = unsafe {
        xmlXPathCastBooleanToString(val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathRegisteredNsCleanup_rust(mut ctxt:
    xmlXPathContextPtr) {
    unsafe {
        xmlXPathRegisteredNsCleanup(ctxt)
    };
}


#[no_mangle]
pub extern "C" fn xmlXPathFreeObject_rust(mut obj: xmlXPathObjectPtr){
    println!("xmlXPathFreeObject_rust pre");
    unsafe {
        xmlXPathFreeObject(obj)
    };
    println!("xmlXPathFreeObject_rust next");
}

#[no_mangle]
pub extern "C" fn xmlXPathNewFloat_rust(mut val: libc::c_double)
-> xmlXPathObjectPtr {
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathNewFloat(val)
    };
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNewBoolean_rust(mut val: libc::c_int)
-> xmlXPathObjectPtr {
    println!("xmlXPathNewBoolean_rust pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathNewBoolean(val)
    };
    println!("xmlXPathNewBoolean_rust next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNewString_rust(mut val: *const xmlChar)
-> xmlXPathObjectPtr {
    println!("xmlXPathNewString pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathNewString(val)
    };
    println!("xmlXPathNewString next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathWrapString_rust(mut val: *mut xmlChar)
-> xmlXPathObjectPtr {
    println!("xmlXPathWrapString pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathWrapString(val)
    };
    println!("xmlXPathWrapString next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNewCString_rust(mut val: *const libc::c_char)
-> xmlXPathObjectPtr {
    println!("xmlXPathNewCString pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathNewCString(val)
    };
    println!("xmlXPathNewCString next");
    return res;
}


#[no_mangle]
pub extern "C" fn xmlXPathCastStringToNumber_rust(mut val: *const xmlChar)
-> libc::c_double {
    println!("xmlXPathCastStringToNumber pre");
    let res: libc::c_double = unsafe {
        xmlXPathCastStringToNumber(val)
    };
    println!("xmlXPathCastStringToNumber next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastBooleanToNumber_rust(mut val: libc::c_int)
-> libc::c_double {
    println!("xmlXPathCastBooleanToNumber pre");
    let res: libc::c_double = unsafe {
        xmlXPathCastBooleanToNumber(val)
    };
    println!("xmlXPathCastBooleanToNumber next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathCastNodeSetToString_rust(mut ns: xmlNodeSetPtr)
-> *mut xmlChar {
    println!("xmlXPathCastNodeSetToString pre");
    let res: *mut xmlChar = unsafe {
        xmlXPathCastNodeSetToString(ns)
    };
    println!("xmlXPathCastNodeSetToString next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathWrapCString_rust(mut val: *mut libc::c_char)
-> xmlXPathObjectPtr {
    println!("xmlXPathWrapCString pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathWrapCString(val)
    };
    println!("xmlXPathWrapCString next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathWrapNodeSet_rust(mut val: xmlNodeSetPtr)
-> xmlXPathObjectPtr {
    println!("xmlXPathWrapNodeSet pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathWrapNodeSet(val)
    };
    println!("xmlXPathWrapNodeSet next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathFreeNodeSetList_rust(mut obj: xmlXPathObjectPtr) {
    println!("xmlXPathFreeNodeSetList pre");
    unsafe {
        xmlXPathFreeNodeSetList(obj)
    };
    println!("xmlXPathFreeNodeSetList next");
}

#[no_mangle]
pub extern "C" fn xmlXPathNewValueTree_rust(mut val: xmlNodePtr)
-> xmlXPathObjectPtr {
    println!("xmlXPathNewValueTree pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathNewValueTree(val)
    };
    println!("xmlXPathNewValueTree next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathNewNodeSet_rust(mut val: xmlNodePtr)
-> xmlXPathObjectPtr {
    println!("xmlXPathNewNodeSet pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathNewNodeSet(val)
    };
    println!("xmlXPathNewNodeSet next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathWrapExternal_rust(mut val: *mut libc::c_void)
-> xmlXPathObjectPtr {
    println!("xmlXPathWrapExternal pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathWrapExternal(val)
    };
    println!("xmlXPathWrapExternal next");
    return res;
}

#[no_mangle]
pub extern "C" fn xmlXPathObjectCopy_rust(mut val: xmlXPathObjectPtr)
-> xmlXPathObjectPtr {
    println!("xmlXPathObjectCopy pre");
    let res: xmlXPathObjectPtr = unsafe {
        xmlXPathObjectCopy(val)
    };
    println!("xmlXPathObjectCopy next");
    return res;
}

#[no_mangle]
#[cfg(DEBUG_OR_DEBUG_STEP)]
pub extern "C" fn xmlGenericErrorContextNodeSet_rust(mut output: *mut FILE,
    mut obj:
        xmlNodeSetPtr) {
    println!("xmlGenericErrorContextNodeSet pre");
    unsafe {
        xmlGenericErrorContextNodeSet(output, obj)
    };
    println!("xmlGenericErrorContextNodeSet next");
}

#[no_mangle]
pub extern "C" fn xmlXPathRegisterVariable_rust(mut ctxt:
    xmlXPathContextPtr,
mut name: *const xmlChar,
mut value:
    xmlXPathObjectPtr)
-> libc::c_int {
    let res: libc::c_int = unsafe {
        xmlXPathRegisterVariable(ctxt, name, value)
    };
    return res;
}