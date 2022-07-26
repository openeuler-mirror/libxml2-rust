#[cfg(LIBXML_XPATH_ENABLED)]
fn toupper_xpath(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        unsafe { *(*__ctype_toupper_loc()).offset(__c as isize) }
    } else {
        __c
    };
}

/*
 * TODO: * There are a few spots where some tests are done which depend upon ascii
 * data.  These should be enhanced for full UTF8 support (see particularly
 * any use of the macros IS_ASCII_CHARACTER and IS_ASCII_DIGIT) */

/* *
 * xmlXPathCmpNodesExt: * @node1: the first node
 * @node2: the second node
 *
 * Compare two nodes w.r.t document order.
 * This one is optimized for handling of non-element nodes.
 *
 * Returns -2 in case of error 1 if first point < second point, 0 if
 *         it's the same node, -1 otherwise
 */
#[cfg(XP_OPTIMIZED_NON_ELEM_COMPARISON)]
fn xmlXPathCmpNodesExt(mut node1: xmlNodePtr, mut node2: xmlNodePtr) -> libc::c_int {
    let mut current_block: u64;
    let mut depth1: libc::c_int = 0;
    let mut depth2: libc::c_int = 0;
    let mut misc: libc::c_int = 0 as libc::c_int;
    let mut precedence1: libc::c_int = 0 as libc::c_int;
    let mut precedence2: libc::c_int = 0 as libc::c_int;
    let mut miscNode1: xmlNodePtr = 0 as xmlNodePtr;
    let mut miscNode2: xmlNodePtr = 0 as xmlNodePtr;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut l1: ptrdiff_t = 0;
    let mut l2: ptrdiff_t = 0;
    if node1.is_null() || node2.is_null() {
        return -(2 as libc::c_int);
    }
    if node1 == node2 {
        return 0 as libc::c_int;
    }
    let safe_node1 = unsafe { &mut *node1 };
    let safe_node2 = unsafe { &mut *node2 };
    /*
     * a couple of optimizations which will avoid computations in most cases
     */
    match safe_node1.type_0 as libc::c_uint {
        1 => {
            if safe_node2.type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            {
                if 0 as libc::c_int as libc::c_long > safe_node1.content as ptrdiff_t
                    && 0 as libc::c_int as libc::c_long > safe_node2.content as ptrdiff_t
                    && safe_node1.doc == safe_node2.doc
                {
                    l1 = -(safe_node1.content as ptrdiff_t); /* element is owner */
                    l2 = -(safe_node2.content as ptrdiff_t);
                    if l1 < l2 {
                        return 1 as libc::c_int;
                    }
                    if l1 > l2 {
                        return -(1 as libc::c_int);
                    }
                    current_block = 721385680381463314;
                } else {
                    current_block = 14549389285293717636;
                }
            } else {
                current_block = 721385680381463314;
            }
        }
        2 => {
            precedence1 = 1 as libc::c_int;
            miscNode1 = node1;
            node1 = safe_node1.parent;
            misc = 1 as libc::c_int;
            current_block = 721385680381463314;
        }
        3 | 4 | 8 | 7 => {
            miscNode1 = node1;
            /*
             * Find nearest element node.
             */
            if !safe_node1.prev.is_null() {
                loop {
                    node1 = safe_node1.prev; /* element in prev-sibl axis */
                    if unsafe {
                        (*node1).type_0 as libc::c_uint
                            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    } {
                        precedence1 = 3 as libc::c_int; /* element is parent */
                        break;
                    } else {
                        if unsafe { !(*node1).prev.is_null() } {
                            continue;
                        }
                        precedence1 = 2 as libc::c_int;
                        /*
                         * URGENT TODO: Are there any cases, where the
                         * parent of such a node is not an element node?
                         */
                        node1 = unsafe { (*node1).parent }; /* element is parent */
                        break;
                    }
                }
            } else {
                precedence1 = 2 as libc::c_int;
                node1 = safe_node1.parent
            }
            if node1.is_null()
                || unsafe {
                    (*node1).type_0 as libc::c_uint
                        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                }
                || unsafe { 0 as libc::c_int as libc::c_long <= (*node1).content as ptrdiff_t }
            {
                /*
                 * Fallback for whatever case.
                 */
                node1 = miscNode1;
                precedence1 = 0 as libc::c_int
            } else {
                misc = 1 as libc::c_int
            }
            current_block = 721385680381463314;
        }
        18 => {
            /*
             * TODO: why do we return 1 for namespace nodes?
             */
            return 1 as libc::c_int;
        }
        _ => {
            current_block = 721385680381463314; /* element is owner */
        }
    } /* element in prev-sibl axis */
    match current_block {
        721385680381463314 => {
            match unsafe { (*node2).type_0 as libc::c_uint } {
                2 => {
                    precedence2 = 1 as libc::c_int; /* element is parent */
                    miscNode2 = node2; /* element is parent */
                    node2 = unsafe { (*node2).parent };
                    misc = 1 as libc::c_int
                }
                3 | 4 | 8 | 7 => {
                    miscNode2 = node2;
                    if unsafe { !(*node2).prev.is_null() } {
                        loop {
                            node2 = unsafe { (*node2).prev };
                            if unsafe {
                                (*node2).type_0 as libc::c_uint
                                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                            } {
                                precedence2 = 3 as libc::c_int;
                                break;
                            } else {
                                if unsafe { !(*node2).prev.is_null() } {
                                    continue;
                                }
                                precedence2 = 2 as libc::c_int;
                                node2 = unsafe { (*node2).parent };
                                break;
                            }
                        }
                    } else {
                        precedence2 = 2 as libc::c_int;
                        node2 = unsafe { (*node2).parent }
                    }
                    if node2.is_null()
                        || unsafe {
                            (*node2).type_0 as libc::c_uint
                                != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                        }
                        || unsafe {
                            0 as libc::c_int as libc::c_long <= (*node2).content as ptrdiff_t
                        }
                    {
                        node2 = miscNode2;
                        precedence2 = 0 as libc::c_int
                    } else {
                        misc = 1 as libc::c_int
                    }
                }
                18 => return 1 as libc::c_int,
                1 | _ => {}
            }
            if misc != 0 {
                if node1 == node2 {
                    if precedence1 == precedence2 {
                        /*
                         * The ugly case; but normally there aren't many
                         * adjacent non-element nodes around.
                         */
                        cur = unsafe { (*miscNode2).prev };
                        while !cur.is_null() {
                            if cur == miscNode1 {
                                return 1 as libc::c_int;
                            }
                            if unsafe {
                                (*cur).type_0 as libc::c_uint
                                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                            } {
                                return -(1 as libc::c_int);
                            }
                            cur = unsafe { (*cur).prev }
                        }
                        return -(1 as libc::c_int);
                    } else if precedence1 < precedence2 {
                        return 1 as libc::c_int;
                    } else {
                        return -(1 as libc::c_int);
                    }
                }
                /*
                 * Evaluate based on higher precedence wrt to the element.
                 * TODO: This assumes attributes are sorted before content.
                 *   Is this 100% correct?
                 */
                /*
                 * Special case: One of the helper-elements is contained by the other.
                 * <foo>
                 *   <node2>
                 *     <node1>Text-1(precedence1 == 2)</node1>
                 *   </node2>
                 *   Text-6(precedence2 == 3) * </foo>
                 */
                if precedence2 == 3 as libc::c_int && precedence1 > 1 as libc::c_int {
                    cur = unsafe { (*node1).parent };
                    while !cur.is_null() {
                        if cur == node2 {
                            return 1 as libc::c_int;
                        }
                        cur = unsafe { (*cur).parent }
                    }
                }
                if precedence1 == 3 as libc::c_int && precedence2 > 1 as libc::c_int {
                    cur = unsafe { (*node2).parent };
                    while !cur.is_null() {
                        if cur == node1 {
                            return -(1 as libc::c_int);
                        }
                        cur = unsafe { (*cur).parent }
                    }
                }
            }
            /*
             * Speedup using document order if available.
             */
            if unsafe {
                (*node1).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    && (*node2).type_0 as libc::c_uint
                        == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    && 0 as libc::c_int as libc::c_long > (*node1).content as ptrdiff_t
                    && 0 as libc::c_int as libc::c_long > (*node2).content as ptrdiff_t
                    && (*node1).doc == (*node2).doc
            } {
                unsafe {
                    l1 = -((*node1).content as ptrdiff_t);
                    l2 = -((*node2).content as ptrdiff_t);
                };
                if l1 < l2 {
                    return 1 as libc::c_int;
                }
                if l1 > l2 {
                    return -(1 as libc::c_int);
                }
            }
        }
        _ => {}
    }

    // turtle_comparison: if node1 == unsafe{(*node2).prev} { return 1 as libc::c_int }
    if node1 == unsafe { (*node2).next } {
        return -(1 as libc::c_int);
    }
    /*
     * compute depth to root
     */
    depth2 = 0 as libc::c_int;
    cur = node2;
    while unsafe { !(*cur).parent.is_null() } {
        if unsafe { (*cur).parent == node1 } {
            return 1 as libc::c_int;
        }
        depth2 += 1;
        cur = unsafe { (*cur).parent }
    }
    root = cur;
    depth1 = 0 as libc::c_int;
    cur = node1;
    while unsafe { !(*cur).parent.is_null() } {
        if unsafe { (*cur).parent == node2 } {
            return -(1 as libc::c_int);
        }
        depth1 += 1;
        cur = unsafe { (*cur).parent }
    }
    /*
     * Distinct document (or distinct entities :-( ) case.
     */
    if root != cur {
        return -(2 as libc::c_int);
    }
    /*
     * get the nearest common ancestor.
     */
    while depth1 > depth2 {
        depth1 -= 1;
        node1 = unsafe { (*node1).parent }
    }
    while depth2 > depth1 {
        depth2 -= 1;
        node2 = unsafe { (*node2).parent }
    }
    while unsafe { (*node1).parent != (*node2).parent } {
        unsafe {
            node1 = (*node1).parent;
            node2 = (*node2).parent;
        };
        /* should not happen but just in case ... */
        if node1.is_null() || node2.is_null() {
            return -(2 as libc::c_int);
        }
    }
    /*
     * Find who's first.
     */
    if node1 == unsafe { (*node2).prev } {
        return 1 as libc::c_int;
    }
    if node1 == unsafe { (*node2).next } {
        return -(1 as libc::c_int);
    }
    /*
     * Speedup using document order if available.
     */
    if unsafe {
        (*node1).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            && (*node2).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            && 0 as libc::c_int as libc::c_long > (*node1).content as ptrdiff_t
            && 0 as libc::c_int as libc::c_long > (*node2).content as ptrdiff_t
            && (*node1).doc == (*node2).doc
    } {
        unsafe {
            l1 = -((*node1).content as ptrdiff_t);
            l2 = -((*node2).content as ptrdiff_t);
        };
        if l1 < l2 {
            return 1 as libc::c_int;
        }
        if l1 > l2 {
            return -(1 as libc::c_int);
        }
    }
    cur = unsafe { (*node1).next };
    while !cur.is_null() {
        if cur == node2 {
            return 1 as libc::c_int;
        }
        cur = unsafe { (*cur).next }
    }
    return -(1 as libc::c_int);
    /* assume there is no sibling list corruption */
}
/* XP_OPTIMIZED_NON_ELEM_COMPARISON */

/*
 * Wrapper for the Timsort algorithm from timsort.h
 */
#[cfg(WITH_TIM_SORT)]
/* *
 * wrap_cmp: * @x: a node
 * @y: another node
 *
 * Comparison function for the Timsort implementation
 *
 * Returns -2 in case of error -1 if first point < second point, 0 if
 *         it's the same node, +1 otherwise
 */
#[cfg(XP_OPTIMIZED_NON_ELEM_COMPARISON)]
fn wrap_cmp(mut x: xmlNodePtr, mut y: xmlNodePtr) -> libc::c_int {
    let mut res: libc::c_int = unsafe { xmlXPathCmpNodesExt(x, y) };
    return if res == -(2 as libc::c_int) {
        res
    } else {
        -res
    };
}
#[cfg(not(XP_OPTIMIZED_NON_ELEM_COMPARISON))]
fn wrap_cmp(mut x: xmlNodePtr, mut y: xmlNodePtr) -> libc::c_int {
    let mut res: libc::c_int = unsafe { xmlXPathCmpNodes(x, y) };
    return if res == -(2 as libc::c_int) {
        res
    } else {
        -res
    };
}
/*
 * Taken from https://github.com/swenson/sort
 * Revision: 05fd77bfec049ce8b7c408c4d3dd2d51ee061a15
 * Removed all code unrelated to Timsort and made minor adjustments for
 * cross-platform compatibility.
 */
/*
 * The MIT License (MIT) *
 * Copyright (c) 2010-2017 Christopher Swenson.
 * Copyright (c) 2012 Vojtech Fried.
 * Copyright (c) 2012 Google Inc. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"), * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense, * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions: *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */
/* Common, type-agnostic functions and constants that we don't want to declare twice. */
/* SORT_COMMON_H */
/* Function used to do a binary search for binary insertion sort */
/* check for out of bounds at the beginning. */
/* allow = for stability. The binary search favors the right. */
/* Binary insertion sort, but knowing that the first "start" entries are sorted.  Used in timsort. */
/* If this entry is already correct, just move along */
/* Else we need to find the right place, shift everything over, and squeeze in */
/* check edge case because j is unsigned */
/* Binary insertion sort */
/* don't bother sorting an array of size <= 1 */
/* timsort implementation, based on timsort.txt */
/* increasing run */
/* decreasing run */
/* reverse in-place */
/* left merge */
/* right merge */
#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_tim_sort_collapse(
    mut dst: *mut xmlNodePtr,
    mut stack: *mut TIM_SORT_RUN_T,
    mut stack_curr: libc::c_int,
    mut store: *mut TEMP_STORAGE_T,
    size: size_t,
) -> libc::c_int {
    loop {
        let mut A: size_t = 0;
        let mut B: size_t = 0;
        let mut C: size_t = 0;
        let mut D: size_t = 0;
        let mut ABC: libc::c_int = 0;
        let mut BCD: libc::c_int = 0;
        let mut CD: libc::c_int = 0;
        let safe_stack = unsafe { &mut *stack };
        /* if the stack only has one thing on it, we are done with the collapse */
        if stack_curr <= 1 as libc::c_int {
            break;
        }
        /* if this is the last merge, just do it */
        if stack_curr == 2 as libc::c_int
            && unsafe {
                (*stack.offset(0 as libc::c_int as isize))
                    .length
                    .wrapping_add((*stack.offset(1 as libc::c_int as isize)).length)
                    == size
            }
        {
            unsafe { libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store) };
            let ref mut fresh0 = unsafe { (*stack.offset(0 as libc::c_int as isize)).length };
            *fresh0 = (*fresh0 as libc::c_ulong)
                .wrapping_add(unsafe { (*stack.offset(1 as libc::c_int as isize)).length })
                as size_t as size_t;
            stack_curr -= 1;
            break;
        } else if stack_curr == 2 as libc::c_int
            && unsafe {
                (*stack.offset(0 as libc::c_int as isize)).length
                    <= (*stack.offset(1 as libc::c_int as isize)).length
            }
        {
            unsafe { libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store) };
            let ref mut fresh1 = unsafe { (*stack.offset(0 as libc::c_int as isize)).length };
            *fresh1 = (*fresh1 as libc::c_ulong)
                .wrapping_add(unsafe { (*stack.offset(1 as libc::c_int as isize)).length })
                as size_t as size_t;
            stack_curr -= 1;
            break;
        } else {
            if stack_curr == 2 as libc::c_int {
                break;
            }
            B = unsafe { (*stack.offset((stack_curr - 3 as libc::c_int) as isize)).length };
            C = unsafe { (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).length };
            D = unsafe { (*stack.offset((stack_curr - 1 as libc::c_int) as isize)).length };
            if stack_curr >= 4 as libc::c_int {
                A = unsafe { (*stack.offset((stack_curr - 4 as libc::c_int) as isize)).length };
                ABC = (A <= B.wrapping_add(C)) as libc::c_int
            } else {
                ABC = 0 as libc::c_int
            }
            BCD = (B <= C.wrapping_add(D) || ABC != 0) as libc::c_int;
            CD = (C <= D) as libc::c_int;
            /* check if the invariant is off for a stack of 2 elements */
            /* Both invariants are good */
            if BCD == 0 && CD == 0 {
                break;
            }
            /* left merge */
            if BCD != 0 && CD == 0 {
                unsafe {
                    libxml_domnode_tim_sort_merge(dst, stack, stack_curr - 1 as libc::c_int, store)
                };
                let ref mut fresh2 =
                    unsafe { (*stack.offset((stack_curr - 3 as libc::c_int) as isize)).length };
                *fresh2 = (*fresh2 as libc::c_ulong).wrapping_add(unsafe {
                    (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).length
                }) as size_t as size_t;
                unsafe {
                    *stack.offset((stack_curr - 2 as libc::c_int) as isize) =
                        *stack.offset((stack_curr - 1 as libc::c_int) as isize)
                };
                stack_curr -= 1
            } else {
                /* right merge */
                unsafe { libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store) };
                let ref mut fresh3 =
                    unsafe { (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).length };
                *fresh3 = (*fresh3 as libc::c_ulong).wrapping_add(unsafe {
                    (*stack.offset((stack_curr - 1 as libc::c_int) as isize)).length
                }) as size_t as size_t;
                stack_curr -= 1
            }
        }
    }
    return stack_curr;
}

#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_tim_sort_merge(
    mut dst: *mut xmlNodePtr,
    mut stack: *const TIM_SORT_RUN_T,
    stack_curr: libc::c_int,
    mut store: *mut TEMP_STORAGE_T,
) {
    let A: size_t = unsafe { (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).length };
    let B: size_t = unsafe { (*stack.offset((stack_curr - 1 as libc::c_int) as isize)).length };
    let curr: size_t = unsafe { (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).start };
    let mut storage: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    unsafe {
        libxml_domnode_tim_sort_resize(store, if A < B { A } else { B });
        storage = (*store).storage;
    };
    if A < B {
        unsafe {
            memcpy(
                storage as *mut libc::c_void,
                &mut *dst.offset(curr as isize) as *mut xmlNodePtr as *const libc::c_void,
                A.wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            )
        };
        i = 0 as libc::c_int as size_t;
        j = curr.wrapping_add(A);
        k = curr;
        while k < curr.wrapping_add(A).wrapping_add(B) {
            if i < A && j < curr.wrapping_add(A).wrapping_add(B) {
                if unsafe {
                    wrap_cmp(*storage.offset(i as isize), *dst.offset(j as isize))
                        <= 0 as libc::c_int
                } {
                    let fresh4 = i;
                    i = i.wrapping_add(1);
                    let ref mut fresh5 = unsafe { *dst.offset(k as isize) };
                    *fresh5 = unsafe { *storage.offset(fresh4 as isize) }
                } else {
                    let fresh6 = j;
                    j = j.wrapping_add(1);
                    let ref mut fresh7 = unsafe { *dst.offset(k as isize) };
                    *fresh7 = unsafe { *dst.offset(fresh6 as isize) }
                }
            } else {
                if !(i < A) {
                    break;
                }
                let fresh8 = i;
                i = i.wrapping_add(1);
                let ref mut fresh9 = unsafe { *dst.offset(k as isize) };
                *fresh9 = unsafe { *storage.offset(fresh8 as isize) }
            }
            k = k.wrapping_add(1)
        }
    } else {
        unsafe {
            memcpy(
                storage as *mut libc::c_void,
                &mut *dst.offset(curr.wrapping_add(A) as isize) as *mut xmlNodePtr
                    as *const libc::c_void,
                B.wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            )
        };
        i = B;
        j = curr.wrapping_add(A);
        k = curr.wrapping_add(A).wrapping_add(B);
        while k > curr {
            k = k.wrapping_sub(1);
            if i > 0 as libc::c_int as libc::c_ulong && j > curr {
                if unsafe {
                    wrap_cmp(
                        *dst.offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
                        *storage.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
                    ) > 0 as libc::c_int
                } {
                    j = j.wrapping_sub(1);
                    let ref mut fresh10 = unsafe { *dst.offset(k as isize) };
                    *fresh10 = unsafe { *dst.offset(j as isize) }
                } else {
                    i = i.wrapping_sub(1);
                    let ref mut fresh11 = unsafe { *dst.offset(k as isize) };
                    *fresh11 = unsafe { *storage.offset(i as isize) }
                }
            } else {
                if !(i > 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
                i = i.wrapping_sub(1);
                let ref mut fresh12 = unsafe { *dst.offset(k as isize) };
                *fresh12 = unsafe { *storage.offset(i as isize) }
            }
        }
    };
}

#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_tim_sort_resize(mut store: *mut TEMP_STORAGE_T, new_size: size_t) {
    let safe_store = unsafe { &mut *store };
    if safe_store.alloc < new_size {
        let mut tempstore: *mut xmlNodePtr = unsafe {
            realloc(
                safe_store.storage as *mut libc::c_void,
                new_size.wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            ) as *mut xmlNodePtr
        };
        if tempstore.is_null() {
            unsafe {
                fprintf(
                    stderr,
                    b"Error allocating temporary storage for tim sort: need %lu bytes\x00"
                        as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong).wrapping_mul(new_size),
                );
                exit(1 as libc::c_int);
            };
        }
        safe_store.storage = tempstore;
        safe_store.alloc = new_size
    };
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_count_run(mut dst: *mut xmlNodePtr, start: size_t, size: size_t) -> size_t {
    let mut curr: size_t = 0;
    if size.wrapping_sub(start) == 1 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int as size_t;
    }
    if start >= size.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
        if unsafe {
            wrap_cmp(
                *dst.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize),
                *dst.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
            ) > 0 as libc::c_int
        } {
            let mut __SORT_SWAP_t: xmlNodePtr = unsafe {
                *dst.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            };
            let ref mut fresh13 = unsafe {
                *dst.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            };
            *fresh13 = unsafe {
                *dst.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            };
            let ref mut fresh14 = unsafe {
                *dst.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            };
            *fresh14 = __SORT_SWAP_t
        }
        return 2 as libc::c_int as size_t;
    }
    curr = start.wrapping_add(2 as libc::c_int as libc::c_ulong);
    if unsafe {
        wrap_cmp(
            *dst.offset(start as isize),
            *dst.offset(start.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize),
        ) <= 0 as libc::c_int
    } {
        while !(curr == size.wrapping_sub(1 as libc::c_int as libc::c_ulong)) {
            if unsafe {
                wrap_cmp(
                    *dst.offset(curr.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
                    *dst.offset(curr as isize),
                ) > 0 as libc::c_int
            } {
                break;
            }
            curr = curr.wrapping_add(1)
        }
        return curr.wrapping_sub(start);
    } else {
        while !(curr == size.wrapping_sub(1 as libc::c_int as libc::c_ulong)) {
            if unsafe {
                wrap_cmp(
                    *dst.offset(curr.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
                    *dst.offset(curr as isize),
                ) <= 0 as libc::c_int
            } {
                break;
            }
            curr = curr.wrapping_add(1)
        }
        unsafe {
            libxml_domnode_reverse_elements(
                dst,
                start,
                curr.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
        };
        return curr.wrapping_sub(start);
    };
}
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn libxml_domnode_binary_insertion_sort(mut dst: *mut xmlNodePtr, size: size_t) {
    if size <= 1 as libc::c_int as libc::c_ulong {
        return;
    }
    unsafe { libxml_domnode_binary_insertion_sort_start(dst, 1 as libc::c_int as size_t, size) };
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn compute_minrun(size: uint64_t) -> libc::c_int {
    let top_bit: libc::c_int =
        64 as libc::c_int - (size as libc::c_ulonglong).leading_zeros() as i32;
    let shift: libc::c_int = (if top_bit > 6 as libc::c_int {
        top_bit
    } else {
        6 as libc::c_int
    }) - 6 as libc::c_int;
    let minrun: libc::c_int = (size >> shift) as libc::c_int;
    let mask: uint64_t = ((1 as libc::c_ulonglong) << shift)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    if mask & size != 0 {
        return minrun + 1 as libc::c_int;
    }
    return minrun;
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_check_invariant(
    mut stack: *mut TIM_SORT_RUN_T,
    stack_curr: libc::c_int,
) -> libc::c_int {
    let mut A: size_t = 0;
    let mut B: size_t = 0;
    let mut C: size_t = 0;
    if stack_curr < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if stack_curr == 2 as libc::c_int {
        let A1: size_t =
            unsafe { (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).length };
        let B1: size_t =
            unsafe { (*stack.offset((stack_curr - 1 as libc::c_int) as isize)).length };
        if A1 <= B1 {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    A = unsafe { (*stack.offset((stack_curr - 3 as libc::c_int) as isize)).length };
    B = unsafe { (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).length };
    C = unsafe { (*stack.offset((stack_curr - 1 as libc::c_int) as isize)).length };
    if A <= B.wrapping_add(C) || B <= C {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_binary_insertion_find(
    mut dst: *mut xmlNodePtr,
    x: xmlNodePtr,
    size: size_t,
) -> size_t {
    let mut l: size_t = 0;
    let mut c: size_t = 0;
    let mut r: size_t = 0;
    let mut cx: xmlNodePtr = 0 as *mut xmlNode;
    l = 0 as libc::c_int as size_t;
    r = size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    c = r >> 1 as libc::c_int;
    if unsafe { wrap_cmp(x, *dst.offset(0 as libc::c_int as isize)) < 0 as libc::c_int } {
        return 0 as libc::c_int as size_t;
    } else {
        if unsafe { wrap_cmp(x, *dst.offset(r as isize)) > 0 as libc::c_int } {
            return r;
        }
    }
    cx = unsafe { *dst.offset(c as isize) };
    loop {
        let val: libc::c_int = unsafe { wrap_cmp(x, cx) };
        if val < 0 as libc::c_int {
            if c.wrapping_sub(l) <= 1 as libc::c_int as libc::c_ulong {
                return c;
            }
            r = c
        } else {
            if r.wrapping_sub(c) <= 1 as libc::c_int as libc::c_ulong {
                return c.wrapping_add(1 as libc::c_int as libc::c_ulong);
            }
            l = c
        }
        c = l.wrapping_add(r.wrapping_sub(l) >> 1 as libc::c_int);
        cx = unsafe { *dst.offset(c as isize) }
    }
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_binary_insertion_sort_start(
    mut dst: *mut xmlNodePtr,
    start: size_t,
    size: size_t,
) {
    let mut i: size_t = 0;
    i = start;
    while i < size {
        let mut j: size_t = 0;
        let mut x: xmlNodePtr = 0 as *mut xmlNode;
        let mut location: size_t = 0;
        if !(unsafe {
            wrap_cmp(
                *dst.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
                *dst.offset(i as isize),
            ) <= 0 as libc::c_int
        }) {
            unsafe {
                x = *dst.offset(i as isize);
                location = libxml_domnode_binary_insertion_find(dst, x, i);
            };
            j = i.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            while j >= location {
                unsafe {
                    let ref mut fresh15 =
                        *dst.offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                    *fresh15 = *dst.offset(j as isize);
                };
                if j == 0 as libc::c_int as libc::c_ulong {
                    break;
                }
                j = j.wrapping_sub(1)
            }
            unsafe {
                let ref mut fresh16 = *dst.offset(location as isize);
                *fresh16 = x
            }
        }
        i = i.wrapping_add(1)
    }
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_reverse_elements(mut dst: *mut xmlNodePtr, mut start: size_t, mut end: size_t) {
    loop {
        if start >= end {
            return;
        }
        unsafe {
            let mut __SORT_SWAP_t: xmlNodePtr = *dst.offset(start as isize);
            let ref mut fresh17 = *dst.offset(start as isize);
            *fresh17 = *dst.offset(end as isize);
            let ref mut fresh18 = *dst.offset(end as isize);
            *fresh18 = __SORT_SWAP_t;
        };
        start = start.wrapping_add(1);
        end = end.wrapping_sub(1)
    }
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn PUSH_NEXT(
    mut dst: *mut xmlNodePtr,
    size: size_t,
    mut store: *mut TEMP_STORAGE_T,
    minrun: size_t,
    mut run_stack: *mut TIM_SORT_RUN_T,
    mut stack_curr: *mut size_t,
    mut curr: *mut size_t,
) -> libc::c_int {
    let mut len: size_t = unsafe { libxml_domnode_count_run(dst, *curr, size) };
    let mut run: size_t = minrun;
    let safe_store = unsafe { &mut *store };
    if unsafe { run > size.wrapping_sub(*curr) } {
        unsafe { run = size.wrapping_sub(*curr) }
    }
    if run > len {
        unsafe {
            libxml_domnode_binary_insertion_sort_start(&mut *dst.offset(*curr as isize), len, run)
        };
        len = run
    }
    unsafe {
        (*run_stack.offset(*stack_curr as isize)).start = *curr;
        (*run_stack.offset(*stack_curr as isize)).length = len;
        *stack_curr = (*stack_curr).wrapping_add(1);
        *curr = (*curr as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    };
    if unsafe { *curr == size } {
        /* finish up */
        while unsafe { *stack_curr > 1 as libc::c_int as libc::c_ulong } {
            unsafe {
                libxml_domnode_tim_sort_merge(dst, run_stack, *stack_curr as libc::c_int, store)
            };
            unsafe {
                let ref mut fresh19 = (*run_stack.offset(
                    (*stack_curr).wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                ))
                .length;
                *fresh19 = (*fresh19 as libc::c_ulong).wrapping_add(
                    (*run_stack.offset(
                        (*stack_curr).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ))
                    .length,
                ) as size_t as size_t;
                *stack_curr = (*stack_curr).wrapping_sub(1)
            }
        }
        if !safe_store.storage.is_null() {
            unsafe { free(safe_store.storage as *mut libc::c_void) };
            safe_store.storage = 0 as *mut xmlNodePtr
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn libxml_domnode_tim_sort(mut dst: *mut xmlNodePtr, size: size_t) {
    let mut minrun: size_t = 0;
    let mut _store: TEMP_STORAGE_T = TEMP_STORAGE_T {
        alloc: 0,
        storage: 0 as *mut xmlNodePtr,
    };
    let mut store: *mut TEMP_STORAGE_T = 0 as *mut TEMP_STORAGE_T;
    let mut run_stack: [TIM_SORT_RUN_T; 128] = [TIM_SORT_RUN_T {
        start: 0,
        length: 0,
    }; 128];
    let mut stack_curr: size_t = 0 as libc::c_int as size_t;
    let mut curr: size_t = 0 as libc::c_int as size_t;
    /* don't bother sorting an array of size 1 */
    if size <= 1 as libc::c_int as libc::c_ulong {
        return;
    }
    if size < 64 as libc::c_int as libc::c_ulong {
        unsafe { libxml_domnode_binary_insertion_sort(dst, size) };
        return;
    }
    /* compute the minimum run length */
    minrun = unsafe { compute_minrun(size) as size_t };
    /* temporary storage for merges */
    store = &mut _store;
    let safe_store = unsafe { &mut *store };
    safe_store.alloc = 0 as libc::c_int as size_t;
    safe_store.storage = 0 as *mut xmlNodePtr;
    if unsafe {
        PUSH_NEXT(
            dst,
            size,
            store,
            minrun,
            run_stack.as_mut_ptr(),
            &mut stack_curr,
            &mut curr,
        ) == 0
    } {
        return;
    }
    if unsafe {
        PUSH_NEXT(
            dst,
            size,
            store,
            minrun,
            run_stack.as_mut_ptr(),
            &mut stack_curr,
            &mut curr,
        ) == 0
    } {
        return;
    }
    if unsafe {
        PUSH_NEXT(
            dst,
            size,
            store,
            minrun,
            run_stack.as_mut_ptr(),
            &mut stack_curr,
            &mut curr,
        ) == 0
    } {
        return;
    }
    loop {
        if unsafe {
            libxml_domnode_check_invariant(run_stack.as_mut_ptr(), stack_curr as libc::c_int) == 0
        } {
            stack_curr = unsafe {
                libxml_domnode_tim_sort_collapse(
                    dst,
                    run_stack.as_mut_ptr(),
                    stack_curr as libc::c_int,
                    store,
                    size,
                ) as size_t
            }
        } else if unsafe {
            PUSH_NEXT(
                dst,
                size,
                store,
                minrun,
                run_stack.as_mut_ptr(),
                &mut stack_curr,
                &mut curr,
            ) == 0
        } {
            return;
        }
    }
}
/* WITH_TIM_SORT */
/* ***********************************************************************
 *									*
 *			Floating point stuff				*
 *									*
 ************************************************************************/

#[cfg(LIBXML_XPATH_ENABLED_OR_LIBXML_SCHEMAS_ENABLED)]
pub static mut xmlXPathNAN: libc::c_double = 0.;
#[cfg(LIBXML_XPATH_ENABLED_OR_LIBXML_SCHEMAS_ENABLED)]
pub static mut xmlXPathPINF: libc::c_double = 0.;
#[cfg(LIBXML_XPATH_ENABLED_OR_LIBXML_SCHEMAS_ENABLED)]
pub static mut xmlXPathNINF: libc::c_double = 0.;

/* LIBXML_XPATH_ENABLED */
/* *
 * xmlXPathInit: *
 * Initialize the XPath environment
 */
#[cfg(LIBXML_XPATH_ENABLED_OR_LIBXML_SCHEMAS_ENABLED)]
pub fn xmlXPathInit_xpath() {
    /* MSVC doesn't allow division by zero in constant expressions. */
    let mut zero: libc::c_double = 0.0f64;
    unsafe {
        xmlXPathNAN = 0.0f64 / zero;
        xmlXPathPINF = 1.0f64 / zero;
        xmlXPathNINF = -xmlXPathPINF;
    };
}
/* *
 * xmlXPathIsNaN: * @val: a double value
 *
 * Returns 1 if the value is a NaN, 0 otherwise
 */
#[cfg(LIBXML_XPATH_ENABLED_OR_LIBXML_SCHEMAS_ENABLED)]
pub fn xmlXPathIsNaN(mut val: libc::c_double) -> libc::c_int {
    match () {
        #[cfg(ISNAN)]
        _ => {
            return if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
            {
                unsafe { __isnanf(val as libc::c_float) }
            } else if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
            {
                unsafe { __isnan(val) }
            } else {
                unsafe { __isnanl(val as libc::c_float) }
            };
        }
        #[cfg(not(ISNAN))]
        _ => {
            return if val == val {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            }
        }
    };
}
/* *
 * xmlXPathIsInf: * @val: a double value
 *
 * Returns 1 if the value is +Infinite, -1 if -Infinite, 0 otherwise
 */
#[cfg(LIBXML_XPATH_ENABLED_OR_LIBXML_SCHEMAS_ENABLED)]
pub fn xmlXPathIsInf(mut val: libc::c_double) -> libc::c_int {
    match () {
        #[cfg(ISINF)]
        _ => {
            return if if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
            {
                unsafe { __isinff(val as libc::c_float) }
            } else if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
            {
                unsafe { __isinf(val) }
            } else {
                unsafe { __isinfl(val as libc::c_float) }
            } != 0
            {
                if val > 0 as libc::c_int as libc::c_double {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                }
            } else {
                0 as libc::c_int
            };
        }
        #[cfg(not(ISINF))]
        _ => {
            return if val >= xmlXPathPINF {
                1 as libc::c_int
            } else if val <= -xmlXPathPINF {
                -1 as libc::c_int
            } else {
                0 as libc::c_int
            }
        }
    };
}
/* SCHEMAS or XPATH */
/*
 * TODO: when compatibility allows remove all "fake node libxslt" strings
 *       the test should just be name[0] = ' '
 */
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathXMLNamespaceStruct: xmlNs = {
    let mut init = _xmlNs {
        next: 0 as *const _xmlNs as *mut _xmlNs,
        type_0: XML_NAMESPACE_DECL,
        href: b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as *const libc::c_char
            as *const xmlChar,
        prefix: b"xml\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
        _private: 0 as *const libc::c_void as *mut libc::c_void,
        context: 0 as *const _xmlDoc as *mut _xmlDoc,
    };
    init
};
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathXMLNamespace: xmlNsPtr =
    unsafe { &xmlXPathXMLNamespaceStruct as *const xmlNs as *mut xmlNs };

#[cfg(LIBXML_THREAD_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDisableOptimizer: libc::c_int = 0;

/* ***********************************************************************
 *									*
 *			Error handling routines				*
 *									*
 ************************************************************************/
/* *
 * XP_ERRORNULL: * @X: the error code
 *
 * Macro to raise an XPath error and return NULL.
 */
/*
 * The array xmlXPathErrorMessages corresponds to the enum xmlXPathError
 */
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathErrorMessages: [*const libc::c_char; 28] = [
    b"Ok\n\x00" as *const u8 as *const libc::c_char,
    b"Number encoding\n\x00" as *const u8 as *const libc::c_char,
    b"Unfinished literal\n\x00" as *const u8 as *const libc::c_char,
    b"Start of literal\n\x00" as *const u8 as *const libc::c_char,
    b"Expected $ for variable reference\n\x00" as *const u8 as *const libc::c_char,
    b"Undefined variable\n\x00" as *const u8 as *const libc::c_char,
    b"Invalid predicate\n\x00" as *const u8 as *const libc::c_char,
    b"Invalid expression\n\x00" as *const u8 as *const libc::c_char,
    b"Missing closing curly brace\n\x00" as *const u8 as *const libc::c_char,
    b"Unregistered function\n\x00" as *const u8 as *const libc::c_char,
    b"Invalid operand\n\x00" as *const u8 as *const libc::c_char,
    b"Invalid type\n\x00" as *const u8 as *const libc::c_char,
    b"Invalid number of arguments\n\x00" as *const u8 as *const libc::c_char,
    b"Invalid context size\n\x00" as *const u8 as *const libc::c_char,
    b"Invalid context position\n\x00" as *const u8 as *const libc::c_char,
    b"Memory allocation error\n\x00" as *const u8 as *const libc::c_char,
    b"Syntax error\n\x00" as *const u8 as *const libc::c_char,
    b"Resource error\n\x00" as *const u8 as *const libc::c_char,
    b"Sub resource error\n\x00" as *const u8 as *const libc::c_char,
    b"Undefined namespace prefix\n\x00" as *const u8 as *const libc::c_char,
    b"Encoding error\n\x00" as *const u8 as *const libc::c_char,
    b"Char out of XML range\n\x00" as *const u8 as *const libc::c_char,
    b"Invalid or incomplete context\n\x00" as *const u8 as *const libc::c_char,
    b"Stack usage error\n\x00" as *const u8 as *const libc::c_char,
    b"Forbidden variable\n\x00" as *const u8 as *const libc::c_char,
    b"Operation limit exceeded\n\x00" as *const u8 as *const libc::c_char,
    b"Recursion limit exceeded\n\x00" as *const u8 as *const libc::c_char,
    b"?? Unknown error ??\n\x00" as *const u8 as *const libc::c_char,
];
/* *
 * xmlXPathErrMemory: * @ctxt: an XPath context
 * @extra: extra information
 *
 * Handle a redefinition of attribute error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathErrMemory(mut ctxt: xmlXPathContextPtr, mut extra: *const libc::c_char) {
    if !ctxt.is_null() {
        let safe_ctxt = unsafe { &mut *ctxt };
        unsafe { xmlResetError(&mut (*ctxt).lastError) };
        if !extra.is_null() {
            let mut buf: [xmlChar; 200] = [0; 200];
            unsafe {
                xmlStrPrintf(
                    buf.as_mut_ptr(),
                    200 as libc::c_int,
                    b"Memory allocation failed : %s\n\x00" as *const u8 as *const libc::c_char,
                    extra,
                )
            };
            safe_ctxt.lastError.message =
                unsafe { xmlStrdup(buf.as_mut_ptr()) as *mut libc::c_char }
        } else {
            safe_ctxt.lastError.message = unsafe {
                xmlStrdup(
                    b"Memory allocation failed\n\x00" as *const u8 as *const libc::c_char
                        as *mut xmlChar,
                ) as *mut libc::c_char
            }
        }
        safe_ctxt.lastError.domain = XML_FROM_XPATH as libc::c_int;
        safe_ctxt.lastError.code = XML_ERR_NO_MEMORY as libc::c_int;
        if safe_ctxt.error.is_some() {
            unsafe {
                safe_ctxt.error.expect("non-null function pointer")(
                    safe_ctxt.userData,
                    &mut safe_ctxt.lastError,
                )
            };
        }
    } else if !extra.is_null() {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                XML_FROM_XPATH as libc::c_int,
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
            )
        };
    } else {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                XML_FROM_XPATH as libc::c_int,
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
            )
        };
    };
}
/* *
 * xmlXPathPErrMemory: * @ctxt: an XPath parser context
 * @extra: extra information
 *
 * Handle a redefinition of attribute error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathPErrMemory(mut ctxt: xmlXPathParserContextPtr, mut extra: *const libc::c_char) {
    if ctxt.is_null() {
        unsafe { xmlXPathErrMemory(0 as xmlXPathContextPtr, extra) };
    } else {
        let safe_ctxt = unsafe { &mut *ctxt };
        safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int;
        unsafe { xmlXPathErrMemory((*ctxt).context, extra) };
    };
}
/* *
 * xmlXPathErr: * @ctxt: a XPath parser context
 * @error: the error code
 *
 * Handle an XPath error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathErr(mut ctxt: xmlXPathParserContextPtr, mut error: libc::c_int) {
    let safe_ctxt = unsafe { &mut *ctxt };
    if error < 0 as libc::c_int
        || error
            > (::std::mem::size_of::<[*const libc::c_char; 28]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                as libc::c_int
                - 1 as libc::c_int
    {
        error = (::std::mem::size_of::<[*const libc::c_char; 28]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int
            - 1 as libc::c_int
    }
    if ctxt.is_null() {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                XML_FROM_XPATH as libc::c_int,
                error + XML_XPATH_EXPRESSION_OK as libc::c_int - XPATH_EXPRESSION_OK as libc::c_int,
                XML_ERR_ERROR,
                0 as *const libc::c_char,
                0 as libc::c_int,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char,
                xmlXPathErrorMessages[error as usize],
            )
        };
        return;
    }
    safe_ctxt.error = error;
    if safe_ctxt.context.is_null() {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                XML_FROM_XPATH as libc::c_int,
                error + XML_XPATH_EXPRESSION_OK as libc::c_int - XPATH_EXPRESSION_OK as libc::c_int,
                XML_ERR_ERROR,
                0 as *const libc::c_char,
                0 as libc::c_int,
                (*ctxt).base as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                (*ctxt).cur.offset_from((*ctxt).base) as libc::c_long as libc::c_int,
                0 as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char,
                xmlXPathErrorMessages[error as usize],
            )
        };
        return;
    }
    /* cleanup current last error */
    unsafe {
        xmlResetError(&mut (*(*ctxt).context).lastError);
        (*safe_ctxt.context).lastError.domain = XML_FROM_XPATH as libc::c_int;
        (*safe_ctxt.context).lastError.code =
            error + XML_XPATH_EXPRESSION_OK as libc::c_int - XPATH_EXPRESSION_OK as libc::c_int;
        (*safe_ctxt.context).lastError.level = XML_ERR_ERROR;
        (*safe_ctxt.context).lastError.str1 = xmlStrdup(safe_ctxt.base) as *mut libc::c_char;
        (*safe_ctxt.context).lastError.int1 =
            safe_ctxt.cur.offset_from(safe_ctxt.base) as libc::c_long as libc::c_int;
        (*safe_ctxt.context).lastError.node = (*safe_ctxt.context).debugNode as *mut libc::c_void;
    };
    if unsafe { (*safe_ctxt.context).error.is_some() } {
        unsafe {
            (*safe_ctxt.context)
                .error
                .expect("non-null function pointer")(
                (*safe_ctxt.context).userData,
                &mut (*safe_ctxt.context).lastError,
            )
        };
    } else {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                (*(*ctxt).context).debugNode as *mut libc::c_void,
                XML_FROM_XPATH as libc::c_int,
                error + XML_XPATH_EXPRESSION_OK as libc::c_int - XPATH_EXPRESSION_OK as libc::c_int,
                XML_ERR_ERROR,
                0 as *const libc::c_char,
                0 as libc::c_int,
                (*ctxt).base as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                (*ctxt).cur.offset_from((*ctxt).base) as libc::c_long as libc::c_int,
                0 as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char,
                xmlXPathErrorMessages[error as usize],
            )
        };
    };
}
/*
 * Error reporting.
 */
/* *
 * xmlXPatherror: * @ctxt: the XPath Parser context
 * @file: the file name
 * @line: the line number
 * @no: the error number
 *
 * Formats an error message.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPatherror(
    mut ctxt: xmlXPathParserContextPtr,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut no: libc::c_int,
) {
    unsafe { xmlXPathErr(ctxt, no) };
}
/* *
 * xmlXPathCheckOpLimit: * @ctxt: the XPath Parser context
 * @opCount: the number of operations to be added
 *
 * Adds opCount to the running total of operations and returns -1 if the
 * operation limit is exceeded. Returns 0 otherwise.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCheckOpLimit(
    mut ctxt: xmlXPathParserContextPtr,
    mut opCount: libc::c_ulong,
) -> libc::c_int {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut xpctxt: xmlXPathContextPtr = safe_ctxt.context;
    let safe_xpctxt = unsafe { &mut *xpctxt };
    if opCount > safe_xpctxt.opLimit
        || safe_xpctxt.opCount > safe_xpctxt.opLimit.wrapping_sub(opCount)
    {
        safe_xpctxt.opCount = safe_xpctxt.opLimit;
        unsafe { xmlXPathErr(ctxt, XPATH_OP_LIMIT_EXCEEDED as libc::c_int) };
        return -(1 as libc::c_int);
    }
    safe_xpctxt.opCount = safe_xpctxt.opCount.wrapping_add(opCount);
    return 0 as libc::c_int;
}
/*
* TODO: Since such a list-handling is used in xmlschemas.c and libxslt
* and here, we should make the functions public.
*/
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlPointerListAddSize(
    mut list: xmlPointerListPtr,
    mut item: *mut libc::c_void,
    mut initialSize: libc::c_int,
) -> libc::c_int {
    let safe_list = unsafe { &mut *list };

    if safe_list.items.is_null() {
        if initialSize <= 0 as libc::c_int {
            initialSize = 1 as libc::c_int
        }
        safe_list.items = unsafe {
            xmlMalloc.expect("non-null function pointer")(
                (initialSize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
            ) as *mut *mut libc::c_void
        };
        if safe_list.items.is_null() {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"xmlPointerListCreate: allocating item\n\x00" as *const u8
                        as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        safe_list.number = 0 as libc::c_int;
        safe_list.size = initialSize
    } else if safe_list.size <= safe_list.number {
        if safe_list.size > 50000000 as libc::c_int {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"xmlPointerListAddSize: re-allocating item\n\x00" as *const u8
                        as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        safe_list.size *= 2 as libc::c_int;
        safe_list.items = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                safe_list.items as *mut libc::c_void,
                (safe_list.size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
            ) as *mut *mut libc::c_void
        };
        if safe_list.items.is_null() {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"xmlPointerListAddSize: re-allocating item\n\x00" as *const u8
                        as *const libc::c_char,
                )
            };
            safe_list.size = 0 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    let fresh20 = safe_list.number;
    safe_list.number = safe_list.number + 1;
    unsafe {
        let ref mut fresh21 = *safe_list.items.offset(fresh20 as isize);
        *fresh21 = item;
    };
    return 0 as libc::c_int;
}
/* *
 * xsltPointerListCreate: *
 * Creates an xsltPointerList structure.
 *
 * Returns a xsltPointerList structure or NULL in case of an error.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlPointerListCreate(mut initialSize: libc::c_int) -> xmlPointerListPtr {
    let mut ret: xmlPointerListPtr = 0 as *mut xmlPointerList;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlPointerList>() as libc::c_ulong
        ) as xmlPointerListPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"xmlPointerListCreate: allocating item\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlPointerListPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlPointerList>() as libc::c_ulong,
        )
    };
    if initialSize > 0 as libc::c_int {
        unsafe { xmlPointerListAddSize(ret, 0 as *mut libc::c_void, initialSize) };
        unsafe { (*ret).number = 0 as libc::c_int }
    }
    return ret;
}
/* *
 * xsltPointerListFree: *
 * Frees the xsltPointerList structure. This does not free
 * the content of the list.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlPointerListFree(mut list: xmlPointerListPtr) {
    if list.is_null() {
        return;
    }
    let safe_list = unsafe { &mut *list };
    if !safe_list.items.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")((*list).items as *mut libc::c_void) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(list as *mut libc::c_void) };
}
/* ***********************************************************************
 *									*
 *			Parser Type functions				*
 *									*
 ************************************************************************/
/* *
 * xmlXPathNewCompExpr: *
 * Create a new Xpath component
 *
 * Returns the newly allocated xmlXPathCompExprPtr or NULL in case of error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathNewCompExpr() -> xmlXPathCompExprPtr {
    let mut cur: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    cur = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathCompExpr>() as libc::c_ulong
        ) as xmlXPathCompExprPtr
    };
    let safe_cur = unsafe { &mut *cur };
    if cur.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"allocating component\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathCompExprPtr;
    }
    unsafe {
        memset(
            cur as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathCompExpr>() as libc::c_ulong,
        )
    };
    safe_cur.maxStep = 10 as libc::c_int;
    safe_cur.nbStep = 0 as libc::c_int;
    safe_cur.steps = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            (safe_cur.maxStep as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlXPathStepOp>() as libc::c_ulong),
        ) as *mut xmlXPathStepOp
    };
    if safe_cur.steps.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"allocating steps\n\x00" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void);
        }
        return 0 as xmlXPathCompExprPtr;
    }
    unsafe {
        memset(
            safe_cur.steps as *mut libc::c_void,
            0 as libc::c_int,
            (safe_cur.maxStep as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlXPathStepOp>() as libc::c_ulong),
        )
    };
    safe_cur.last = -(1 as libc::c_int);
    match () {
        #[cfg(DEBUG_EVAL_COUNTS)]
        _ => {
            safe_cur.nb = (0 as libc::c_int);
        }
        #[cfg(not(DEBUG_EVAL_COUNTS))]
        _ => {}
    };
    return cur;
}
/* *
 * xmlXPathFreeCompExpr: * @comp: an XPATH comp
 *
 * Free up the memory allocated by @comp
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathFreeCompExpr(mut comp: xmlXPathCompExprPtr) {
    let mut op: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
    let mut i: libc::c_int = 0;
    if comp.is_null() {
        return;
    }
    let safe_comp = unsafe { &mut *comp };
    if safe_comp.dict.is_null() {
        i = 0 as libc::c_int;
        while i < safe_comp.nbStep {
            op = unsafe { &mut *(*comp).steps.offset(i as isize) as *mut xmlXPathStepOp };
            let safe_op = unsafe { &mut *op };
            if !safe_op.value4.is_null() {
                if safe_op.op as libc::c_uint == XPATH_OP_VALUE as libc::c_int as libc::c_uint {
                    unsafe { xmlXPathFreeObject((*op).value4 as xmlXPathObjectPtr) };
                } else {
                    unsafe { xmlFree.expect("non-null function pointer")((*op).value4) };
                }
            }
            if !safe_op.value5.is_null() {
                unsafe { xmlFree.expect("non-null function pointer")((*op).value5) };
            }
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < safe_comp.nbStep {
            op = unsafe { &mut *(*comp).steps.offset(i as isize) as *mut xmlXPathStepOp };
            let safe_op = unsafe { &mut *op };
            if !safe_op.value4.is_null() {
                if safe_op.op as libc::c_uint == XPATH_OP_VALUE as libc::c_int as libc::c_uint {
                    unsafe { xmlXPathFreeObject((*op).value4 as xmlXPathObjectPtr) };
                }
            }
            i += 1
        }
        unsafe { xmlDictFree((*comp).dict) };
    }
    if !safe_comp.steps.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")((*comp).steps as *mut libc::c_void) };
    }
    match () {
        #[cfg(DEBUG_EVAL_COUNTS)]
        _ => {
            if !safe_comp.string.is_null() {
                unsafe {
                    xmlFree.expect("non-null function pointer")((*comp).string as *mut libc::c_void)
                };
            }
        }
        #[cfg(not(DEBUG_EVAL_COUNTS))]
        _ => {}
    };
    match () {
        #[cfg(XPATH_STREAMING)]
        _ => {
            if !safe_comp.stream.is_null() {
                unsafe { xmlFreePatternList((*comp).stream) };
            }
        }
        #[cfg(not(XPATH_STREAMING))]
        _ => {}
    };
    if !safe_comp.expr.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")((*comp).expr as *mut libc::c_void) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(comp as *mut libc::c_void) };
}
/* *
 * xmlXPathCompExprAdd: * @comp: the compiled expression
 * @ch1: first child index
 * @ch2: second child index
 * @op: an op
 * @value: the first int value
 * @value2: the second int value
 * @value3: the third int value
 * @value4: the first string value
 * @value5: the second string value
 *
 * Add a step to an XPath Compiled Expression
 *
 * Returns -1 in case of failure, the index otherwise
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompExprAdd(
    mut ctxt: xmlXPathParserContextPtr,
    mut ch1: libc::c_int,
    mut ch2: libc::c_int,
    mut op: xmlXPathOp,
    mut value: libc::c_int,
    mut value2: libc::c_int,
    mut value3: libc::c_int,
    mut value4: *mut libc::c_void,
    mut value5: *mut libc::c_void,
) -> libc::c_int {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut comp: xmlXPathCompExprPtr = safe_ctxt.comp;
    let safe_comp = unsafe { &mut *comp };
    if safe_comp.nbStep >= safe_comp.maxStep {
        let mut real: *mut xmlXPathStepOp = 0 as *mut xmlXPathStepOp;
        if safe_comp.maxStep >= 1000000 as libc::c_int {
            unsafe {
                xmlXPathPErrMemory(
                    ctxt,
                    b"adding step\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        safe_comp.maxStep *= 2 as libc::c_int;
        real = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*comp).steps as *mut libc::c_void,
                ((*comp).maxStep as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlXPathStepOp>() as libc::c_ulong),
            ) as *mut xmlXPathStepOp
        };
        if real.is_null() {
            safe_comp.maxStep /= 2 as libc::c_int;
            unsafe {
                xmlXPathPErrMemory(
                    ctxt,
                    b"adding step\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        safe_comp.steps = real
    }
    unsafe {
        (*comp).last = (*comp).nbStep;
        (*(*comp).steps.offset((*comp).nbStep as isize)).ch1 = ch1;
        (*(*comp).steps.offset((*comp).nbStep as isize)).ch2 = ch2;
        (*(*comp).steps.offset((*comp).nbStep as isize)).op = op;
        (*(*comp).steps.offset((*comp).nbStep as isize)).value = value;
        (*(*comp).steps.offset((*comp).nbStep as isize)).value2 = value2;
        (*(*comp).steps.offset((*comp).nbStep as isize)).value3 = value3;
    };
    if !safe_comp.dict.is_null()
        && (op as libc::c_uint == XPATH_OP_FUNCTION as libc::c_int as libc::c_uint
            || op as libc::c_uint == XPATH_OP_VARIABLE as libc::c_int as libc::c_uint
            || op as libc::c_uint == XPATH_OP_COLLECT as libc::c_int as libc::c_uint)
    {
        if !value4.is_null() {
            unsafe {
                let ref mut fresh22 = (*(*comp).steps.offset((*comp).nbStep as isize)).value4;
                *fresh22 =
                    xmlDictLookup((*comp).dict, value4 as *const xmlChar, -(1 as libc::c_int))
                        as *mut libc::c_void as *mut xmlChar
                        as *mut libc::c_void;
                xmlFree.expect("non-null function pointer")(value4);
            }
        } else {
            unsafe {
                let ref mut fresh23 = (*(*comp).steps.offset((*comp).nbStep as isize)).value4;
                *fresh23 = 0 as *mut libc::c_void
            }
        }
        if !value5.is_null() {
            unsafe {
                let ref mut fresh24 = (*(*comp).steps.offset((*comp).nbStep as isize)).value5;
                *fresh24 =
                    xmlDictLookup((*comp).dict, value5 as *const xmlChar, -(1 as libc::c_int))
                        as *mut libc::c_void as *mut xmlChar
                        as *mut libc::c_void;
                xmlFree.expect("non-null function pointer")(value5);
            }
        } else {
            unsafe {
                let ref mut fresh25 = (*(*comp).steps.offset((*comp).nbStep as isize)).value5;
                *fresh25 = 0 as *mut libc::c_void
            }
        }
    } else {
        unsafe {
            let ref mut fresh26 = (*(*comp).steps.offset((*comp).nbStep as isize)).value4;
            *fresh26 = value4;
            let ref mut fresh27 = (*(*comp).steps.offset((*comp).nbStep as isize)).value5;
            *fresh27 = value5
        }
    }
    unsafe {
        let ref mut fresh28 = (*(*comp).steps.offset((*comp).nbStep as isize)).cache;
        *fresh28 = None;
    }
    let fresh29 = safe_comp.nbStep;
    safe_comp.nbStep = safe_comp.nbStep + 1;
    return fresh29;
}
/* *
 * xmlXPathCompSwap: * @comp: the compiled expression
 * @op: operation index
 *
 * Swaps 2 operations in the compiled expression
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompSwap(mut op: xmlXPathStepOpPtr) {
    let mut tmp: libc::c_int = 0;
    match () {
        #[cfg(not(LIBXML_THREAD_ENABLED))]
        _ => {
            if xmlXPathDisableOptimizer == 1 as libc::c_int {
                return;
            }
        }
        #[cfg(LIBXML_THREAD_ENABLED)]
        _ => {}
    }
    let safe_op = unsafe { &mut *op };
    tmp = safe_op.ch1;
    safe_op.ch1 = safe_op.ch2;
    safe_op.ch2 = tmp;
}
/* ***********************************************************************
 *									*
 *		Debugging related functions				*
 *									*
 ************************************************************************/
#[cfg(LIBXML_DEBUG_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathDebugDumpNode(mut output: *mut FILE, mut cur: xmlNodePtr, mut depth: libc::c_int) {
    let safe_cur = unsafe { &mut *cur };
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i) as usize] =
            shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i) as usize] =
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    if cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const libc::c_char,
                shift.as_mut_ptr(),
            );
            fprintf(
                output,
                b"Node is NULL !\n\x00" as *const u8 as *const libc::c_char,
            );
        };
        return;
    }
    if safe_cur.type_0 as libc::c_uint == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        || safe_cur.type_0 as libc::c_uint == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const libc::c_char,
                shift.as_mut_ptr(),
            );
            fprintf(output, b" /\n\x00" as *const u8 as *const libc::c_char);
        }
    } else if safe_cur.type_0 as libc::c_uint == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint {
        unsafe { xmlDebugDumpAttr(output, cur as xmlAttrPtr, depth) };
    } else {
        unsafe { xmlDebugDumpOneNode(output, cur, depth) };
    };
}
#[cfg(LIBXML_DEBUG_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathDebugDumpNodeList(mut output: *mut FILE, mut cur: xmlNodePtr, mut depth: libc::c_int) {
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i) as usize] =
            shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i) as usize] =
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    if cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const libc::c_char,
                shift.as_mut_ptr(),
            );
            fprintf(
                output,
                b"Node is NULL !\n\x00" as *const u8 as *const libc::c_char,
            );
        };
        return;
    }
    while !cur.is_null() {
        unsafe {
            tmp = cur;
            cur = (*cur).next;
            xmlDebugDumpOneNode(output, tmp, depth);
        }
    }
}
#[cfg(LIBXML_DEBUG_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathDebugDumpNodeSet(mut output: *mut FILE, mut cur: xmlNodeSetPtr, mut depth: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i) as usize] =
            shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i) as usize] =
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    if cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const libc::c_char,
                shift.as_mut_ptr(),
            );
            fprintf(
                output,
                b"NodeSet is NULL !\n\x00" as *const u8 as *const libc::c_char,
            );
        };
        return;
    }
    if !cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"Set contains %d nodes:\n\x00" as *const u8 as *const libc::c_char,
                (*cur).nodeNr,
            )
        };
        i = 0 as libc::c_int;
        let safe_cur = unsafe { &mut *cur };
        while i < safe_cur.nodeNr {
            unsafe {
                fprintf(
                    output,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    shift.as_mut_ptr(),
                );
                fprintf(
                    output,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    i + 1 as libc::c_int,
                );
                xmlXPathDebugDumpNode(
                    output,
                    *(*cur).nodeTab.offset(i as isize),
                    depth + 1 as libc::c_int,
                );
            };
            i += 1
        }
    };
}
#[cfg(LIBXML_DEBUG_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathDebugDumpValueTree(
    mut output: *mut FILE,
    mut cur: xmlNodeSetPtr,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    let safe_cur = unsafe { &mut *cur };
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i) as usize] =
            shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i) as usize] =
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    if unsafe {
        cur.is_null()
            || safe_cur.nodeNr == 0 as libc::c_int
            || (*safe_cur.nodeTab.offset(0 as libc::c_int as isize)).is_null()
    } {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const libc::c_char,
                shift.as_mut_ptr(),
            );
            fprintf(
                output,
                b"Value Tree is NULL !\n\x00" as *const u8 as *const libc::c_char,
            );
        };
        return;
    }
    unsafe {
        fprintf(
            output,
            b"%s\x00" as *const u8 as *const libc::c_char,
            shift.as_mut_ptr(),
        );
        fprintf(
            output,
            b"%d\x00" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        xmlXPathDebugDumpNodeList(
            output,
            (**(*cur).nodeTab.offset(0 as libc::c_int as isize)).children,
            depth + 1 as libc::c_int,
        );
    }
}
#[cfg(LIBXML_XPTR_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
#[cfg(LIBXML_DEBUG_ENABLED)]
fn xmlXPathDebugDumpLocationSet(
    mut output: *mut FILE,
    mut cur: xmlLocationSetPtr,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i) as usize] =
            shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i) as usize] =
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    if cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const libc::c_char,
                shift.as_mut_ptr(),
            );
            fprintf(
                output,
                b"LocationSet is NULL !\n\x00" as *const u8 as *const libc::c_char,
            );
        };
        return;
    }
    i = 0 as libc::c_int;
    let safe_cur = unsafe { &mut *cur };
    while i < safe_cur.locNr {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const libc::c_char,
                shift.as_mut_ptr(),
            );
            fprintf(
                output,
                b"%d : \x00" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
            );
            xmlXPathDebugDumpObject(
                output,
                *(*cur).locTab.offset(i as isize),
                depth + 1 as libc::c_int,
            );
        };
        i += 1
    }
}
/* LIBXML_XPTR_ENABLED */
/* *
 * xmlXPathDebugDumpObject: * @output: the FILE * to dump the output
 * @cur: the object to inspect
 * @depth: indentation level
 *
 * Dump the content of the object for debugging purposes
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathDebugDumpObject(
    mut output: *mut FILE,
    mut cur: xmlXPathObjectPtr,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    if output.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i) as usize] =
            shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i) as usize] =
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    unsafe {
        fprintf(
            output,
            b"%s\x00" as *const u8 as *const libc::c_char,
            shift.as_mut_ptr(),
        )
    };
    if cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"Object is empty (NULL)\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return;
    }
    let safe_cur = unsafe { &mut *cur };
    match safe_cur.type_0 as libc::c_uint {
        0 => {
            unsafe {
                fprintf(
                    output,
                    b"Object is uninitialized\n\x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        1 => unsafe {
            fprintf(
                output,
                b"Object is a Node Set :\n\x00" as *const u8 as *const libc::c_char,
            );
            xmlXPathDebugDumpNodeSet(output, (*cur).nodesetval, depth);
        },
        9 => unsafe {
            fprintf(
                output,
                b"Object is an XSLT value tree :\n\x00" as *const u8 as *const libc::c_char,
            );
            xmlXPathDebugDumpValueTree(output, (*cur).nodesetval, depth);
        },
        2 => {
            unsafe {
                fprintf(
                    output,
                    b"Object is a Boolean : \x00" as *const u8 as *const libc::c_char,
                );
            };
            if safe_cur.boolval != 0 {
                unsafe { fprintf(output, b"true\n\x00" as *const u8 as *const libc::c_char) };
            } else {
                unsafe { fprintf(output, b"false\n\x00" as *const u8 as *const libc::c_char) };
            }
        }
        3 => {
            match unsafe { xmlXPathIsInf((*cur).floatval) } {
                1 => unsafe {
                    fprintf(
                        output,
                        b"Object is a number : Infinity\n\x00" as *const u8 as *const libc::c_char,
                    );
                },
                -1 => unsafe {
                    fprintf(
                        output,
                        b"Object is a number : -Infinity\n\x00" as *const u8 as *const libc::c_char,
                    );
                },
                _ => {
                    if unsafe { xmlXPathIsNaN((*cur).floatval) != 0 } {
                        unsafe {
                            fprintf(
                                output,
                                b"Object is a number : NaN\n\x00" as *const u8
                                    as *const libc::c_char,
                            )
                        };
                    } else if safe_cur.floatval == 0 as libc::c_int as libc::c_double {
                        /* Omit sign for negative zero. */
                        unsafe {
                            fprintf(
                                output,
                                b"Object is a number : 0\n\x00" as *const u8 as *const libc::c_char,
                            )
                        };
                    } else {
                        unsafe {
                            fprintf(
                                output,
                                b"Object is a number : %0g\n\x00" as *const u8
                                    as *const libc::c_char,
                                (*cur).floatval,
                            )
                        };
                    }
                }
            }
        }
        4 => unsafe {
            fprintf(
                output,
                b"Object is a string : \x00" as *const u8 as *const libc::c_char,
            );
            xmlDebugDumpString(output, (*cur).stringval);
            fprintf(output, b"\n\x00" as *const u8 as *const libc::c_char);
        },
        5 => {
            unsafe {
                fprintf(
                    output,
                    b"Object is a point : index %d in node\x00" as *const u8 as *const libc::c_char,
                    (*cur).index,
                );
                xmlXPathDebugDumpNode(output, (*cur).user as xmlNodePtr, depth + 1 as libc::c_int);
                fprintf(output, b"\n\x00" as *const u8 as *const libc::c_char);
            };
        }
        6 => {
            if safe_cur.user2.is_null()
                || safe_cur.user2 == safe_cur.user && safe_cur.index == safe_cur.index2
            {
                unsafe {
                    fprintf(
                        output,
                        b"Object is a collapsed range :\n\x00" as *const u8 as *const libc::c_char,
                    );
                    fprintf(
                        output,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        shift.as_mut_ptr(),
                    );
                };
                if safe_cur.index >= 0 as libc::c_int {
                    unsafe {
                        fprintf(
                            output,
                            b"index %d in \x00" as *const u8 as *const libc::c_char,
                            (*cur).index,
                        )
                    };
                }
                unsafe {
                    fprintf(output, b"node\n\x00" as *const u8 as *const libc::c_char);
                    xmlXPathDebugDumpNode(
                        output,
                        (*cur).user as xmlNodePtr,
                        depth + 1 as libc::c_int,
                    );
                };
            } else {
                unsafe {
                    fprintf(
                        output,
                        b"Object is a range :\n\x00" as *const u8 as *const libc::c_char,
                    );
                    fprintf(
                        output,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        shift.as_mut_ptr(),
                    );
                    fprintf(output, b"From \x00" as *const u8 as *const libc::c_char);
                };
                if safe_cur.index >= 0 as libc::c_int {
                    unsafe {
                        fprintf(
                            output,
                            b"index %d in \x00" as *const u8 as *const libc::c_char,
                            (*cur).index,
                        )
                    };
                }
                unsafe {
                    fprintf(output, b"node\n\x00" as *const u8 as *const libc::c_char);
                    xmlXPathDebugDumpNode(
                        output,
                        (*cur).user as xmlNodePtr,
                        depth + 1 as libc::c_int,
                    );
                    fprintf(
                        output,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        shift.as_mut_ptr(),
                    );
                    fprintf(output, b"To \x00" as *const u8 as *const libc::c_char);
                };
                if safe_cur.index2 >= 0 as libc::c_int {
                    unsafe {
                        fprintf(
                            output,
                            b"index %d in \x00" as *const u8 as *const libc::c_char,
                            (*cur).index2,
                        );
                    }
                }
                unsafe {
                    fprintf(output, b"node\n\x00" as *const u8 as *const libc::c_char);
                    xmlXPathDebugDumpNode(
                        output,
                        (*cur).user2 as xmlNodePtr,
                        depth + 1 as libc::c_int,
                    );
                    fprintf(output, b"\n\x00" as *const u8 as *const libc::c_char);
                }
            }
        }
        7 => {
            match () {
                #[cfg(LIBXML_XPTR_ENABLED)]
                _ => {
                    unsafe {
                        fprintf(
                            output,
                            b"Object is a Location Set:\n\x00" as *const u8 as *const libc::c_char,
                        );
                        xmlXPathDebugDumpLocationSet(
                            output,
                            (*cur).user as xmlLocationSetPtr,
                            depth,
                        );
                    };
                }
                #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                _ => {}
            };
        }
        8 => {
            unsafe {
                fprintf(
                    output,
                    b"Object is user defined\n\x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        _ => {}
    };
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathDebugDumpStepOp(
    mut output: *mut FILE,
    mut comp: xmlXPathCompExprPtr,
    mut op: xmlXPathStepOpPtr,
    mut depth: libc::c_int,
) {
    let mut current_block: u64;
    let safe_op = unsafe { &mut *op };
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i) as usize] =
            shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i) as usize] =
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    unsafe {
        fprintf(
            output,
            b"%s\x00" as *const u8 as *const libc::c_char,
            shift.as_mut_ptr(),
        )
    };
    if op.is_null() {
        unsafe {
            fprintf(
                output,
                b"Step is NULL\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return;
    }
    match safe_op.op as libc::c_uint {
        0 => {
            unsafe { fprintf(output, b"END\x00" as *const u8 as *const libc::c_char) };
            current_block = 14612007084265645573;
        }
        1 => {
            unsafe { fprintf(output, b"AND\x00" as *const u8 as *const libc::c_char) };
            current_block = 14612007084265645573;
        }
        2 => {
            unsafe { fprintf(output, b"OR\x00" as *const u8 as *const libc::c_char) };
            current_block = 14612007084265645573;
        }
        3 => {
            if safe_op.value != 0 {
                unsafe { fprintf(output, b"EQUAL =\x00" as *const u8 as *const libc::c_char) };
            } else {
                unsafe { fprintf(output, b"EQUAL !=\x00" as *const u8 as *const libc::c_char) };
            }
            current_block = 14612007084265645573;
        }
        4 => {
            if safe_op.value != 0 {
                unsafe { fprintf(output, b"CMP <\x00" as *const u8 as *const libc::c_char) };
            } else {
                unsafe { fprintf(output, b"CMP >\x00" as *const u8 as *const libc::c_char) };
            }
            if safe_op.value2 == 0 {
                unsafe { fprintf(output, b"=\x00" as *const u8 as *const libc::c_char) };
            }
            current_block = 14612007084265645573;
        }
        5 => {
            if safe_op.value == 0 as libc::c_int {
                unsafe { fprintf(output, b"PLUS -\x00" as *const u8 as *const libc::c_char) };
            } else if safe_op.value == 1 as libc::c_int {
                unsafe { fprintf(output, b"PLUS +\x00" as *const u8 as *const libc::c_char) };
            } else if safe_op.value == 2 as libc::c_int {
                unsafe {
                    fprintf(
                        output,
                        b"PLUS unary -\x00" as *const u8 as *const libc::c_char,
                    )
                };
            } else if safe_op.value == 3 as libc::c_int {
                unsafe {
                    fprintf(
                        output,
                        b"PLUS unary - -\x00" as *const u8 as *const libc::c_char,
                    )
                };
            }
            current_block = 14612007084265645573;
        }
        6 => {
            if safe_op.value == 0 as libc::c_int {
                unsafe { fprintf(output, b"MULT *\x00" as *const u8 as *const libc::c_char) };
            } else if safe_op.value == 1 as libc::c_int {
                unsafe { fprintf(output, b"MULT div\x00" as *const u8 as *const libc::c_char) };
            } else {
                unsafe { fprintf(output, b"MULT mod\x00" as *const u8 as *const libc::c_char) };
            }
            current_block = 14612007084265645573;
        }
        7 => {
            unsafe { fprintf(output, b"UNION\x00" as *const u8 as *const libc::c_char) };
            current_block = 14612007084265645573;
        }
        8 => {
            unsafe { fprintf(output, b"ROOT\x00" as *const u8 as *const libc::c_char) };
            current_block = 14612007084265645573;
        }
        9 => {
            unsafe { fprintf(output, b"NODE\x00" as *const u8 as *const libc::c_char) };
            current_block = 14612007084265645573;
        }
        17 => {
            unsafe { fprintf(output, b"SORT\x00" as *const u8 as *const libc::c_char) };
            current_block = 14612007084265645573;
        }
        10 => {
            let mut axis: xmlXPathAxisVal = safe_op.value as xmlXPathAxisVal;
            let mut test: xmlXPathTestVal = safe_op.value2 as xmlXPathTestVal;
            let mut type_0: xmlXPathTypeVal = safe_op.value3 as xmlXPathTypeVal;
            let mut prefix: *const xmlChar = safe_op.value4 as *const xmlChar;
            let mut name: *const xmlChar = safe_op.value5 as *const xmlChar;
            unsafe { fprintf(output, b"COLLECT \x00" as *const u8 as *const libc::c_char) };
            match axis as libc::c_uint {
                1 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'ancestors\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                2 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'ancestors-or-self\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                3 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'attributes\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                4 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'child\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                5 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'descendant\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                6 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'descendant-or-self\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                7 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'following\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                8 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'following-siblings\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                9 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'namespace\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                10 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'parent\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                11 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'preceding\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                12 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'preceding-sibling\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                13 => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'self\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                _ => {}
            }
            match test as libc::c_uint {
                0 => {
                    unsafe {
                        fprintf(output, b"\'none\' \x00" as *const u8 as *const libc::c_char)
                    };
                }
                1 => {
                    unsafe {
                        fprintf(output, b"\'type\' \x00" as *const u8 as *const libc::c_char)
                    };
                }
                2 => {
                    unsafe { fprintf(output, b"\'PI\' \x00" as *const u8 as *const libc::c_char) };
                }
                3 => {
                    unsafe { fprintf(output, b"\'all\' \x00" as *const u8 as *const libc::c_char) };
                }
                4 => {
                    unsafe {
                        fprintf(
                            output,
                            b"\'namespace\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                5 => {
                    unsafe {
                        fprintf(output, b"\'name\' \x00" as *const u8 as *const libc::c_char)
                    };
                }
                _ => {}
            }
            match type_0 as libc::c_uint {
                0 => {
                    unsafe {
                        fprintf(output, b"\'node\' \x00" as *const u8 as *const libc::c_char)
                    };
                }
                8 => {
                    unsafe {
                        fprintf(
                            output,
                            b"\'comment\' \x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                3 => {
                    unsafe {
                        fprintf(output, b"\'text\' \x00" as *const u8 as *const libc::c_char)
                    };
                }
                7 => {
                    unsafe { fprintf(output, b"\'PI\' \x00" as *const u8 as *const libc::c_char) };
                }
                _ => {}
            }
            if !prefix.is_null() {
                unsafe {
                    fprintf(
                        output,
                        b"%s:\x00" as *const u8 as *const libc::c_char,
                        prefix,
                    )
                };
            }
            if !name.is_null() {
                unsafe {
                    fprintf(
                        output,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        name as *const libc::c_char,
                    )
                };
            }
            current_block = 14612007084265645573;
        }
        11 => {
            let mut object: xmlXPathObjectPtr = safe_op.value4 as xmlXPathObjectPtr;
            unsafe {
                fprintf(output, b"ELEM \x00" as *const u8 as *const libc::c_char);
                xmlXPathDebugDumpObject(output, object, 0 as libc::c_int);
            };
            current_block = 8842003632612379093;
        }
        12 => {
            let mut prefix_0: *const xmlChar = safe_op.value5 as *const xmlChar;
            let mut name_0: *const xmlChar = safe_op.value4 as *const xmlChar;
            if !prefix_0.is_null() {
                unsafe {
                    fprintf(
                        output,
                        b"VARIABLE %s:%s\x00" as *const u8 as *const libc::c_char,
                        prefix_0,
                        name_0,
                    )
                };
            } else {
                unsafe {
                    fprintf(
                        output,
                        b"VARIABLE %s\x00" as *const u8 as *const libc::c_char,
                        name_0,
                    )
                };
            }
            current_block = 14612007084265645573;
        }
        13 => {
            let mut nbargs: libc::c_int = safe_op.value;
            let mut prefix_1: *const xmlChar = safe_op.value5 as *const xmlChar;
            let mut name_1: *const xmlChar = safe_op.value4 as *const xmlChar;
            if !prefix_1.is_null() {
                unsafe {
                    fprintf(
                        output,
                        b"FUNCTION %s:%s(%d args)\x00" as *const u8 as *const libc::c_char,
                        prefix_1,
                        name_1,
                        nbargs,
                    )
                };
            } else {
                unsafe {
                    fprintf(
                        output,
                        b"FUNCTION %s(%d args)\x00" as *const u8 as *const libc::c_char,
                        name_1,
                        nbargs,
                    )
                };
            }
            current_block = 14612007084265645573;
        }
        14 => {
            unsafe { fprintf(output, b"ARG\x00" as *const u8 as *const libc::c_char) };
            current_block = 14612007084265645573;
        }
        15 => {
            unsafe { fprintf(output, b"PREDICATE\x00" as *const u8 as *const libc::c_char) };
            current_block = 14612007084265645573;
        }
        16 => {
            unsafe { fprintf(output, b"FILTER\x00" as *const u8 as *const libc::c_char) };
            current_block = 14612007084265645573;
        }
        18 => {
            // LIBXML_XPTR_ENABLED
            match () {
                #[cfg(LIBXML_XPTR_ENABLED)]
                _ => {
                    unsafe { fprintf(output, b"RANGETO\x00" as *const u8 as *const libc::c_char) };
                    current_block = 14612007084265645573;
                }
                #[cfg(not(LIBXML_XPTR_ENABLED))]
                _ => {}
            }
        }
        _ => {
            unsafe {
                fprintf(
                    output,
                    b"UNKNOWN %d\n\x00" as *const u8 as *const libc::c_char,
                    safe_op.op as libc::c_uint,
                )
            };
            return;
        }
    }
    match current_block {
        14612007084265645573 => {
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const libc::c_char) };
        }
        _ => {}
    }
    if safe_op.ch1 >= 0 as libc::c_int {
        unsafe {
            xmlXPathDebugDumpStepOp(
                output,
                comp,
                &mut *(*comp).steps.offset(safe_op.ch1 as isize),
                depth + 1 as libc::c_int,
            )
        };
    }
    if safe_op.ch2 >= 0 as libc::c_int {
        unsafe {
            xmlXPathDebugDumpStepOp(
                output,
                comp,
                &mut *(*comp).steps.offset(safe_op.ch2 as isize),
                depth + 1 as libc::c_int,
            )
        };
    };
}
/* *
 * xmlXPathDebugDumpCompExpr: * @output: the FILE * for the output
 * @comp: the precompiled XPath expression
 * @depth: the indentation level.
 *
 * Dumps the tree of the compiled XPath expression.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathDebugDumpCompExpr(
    mut output: *mut FILE,
    mut comp: xmlXPathCompExprPtr,
    mut depth: libc::c_int,
) {
    let safe_comp = unsafe { &mut *comp };
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    if output.is_null() || comp.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i) as usize] =
            shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i) as usize] =
        shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    unsafe {
        fprintf(
            output,
            b"%s\x00" as *const u8 as *const libc::c_char,
            shift.as_mut_ptr(),
        )
    };

    match () {
        #[cfg(not(XPATH_STREAMING))]
        _ => {
            unsafe {
                fprintf(
                    output,
                    b"Compiled Expression : %d elements\n\x00" as *const u8 as *const libc::c_char,
                    (*comp).nbStep,
                )
            };
            i = safe_comp.last;
            unsafe {
                xmlXPathDebugDumpStepOp(
                    output,
                    comp,
                    &mut *(*comp).steps.offset(i as isize),
                    depth + 1 as libc::c_int,
                )
            };
        }
        #[cfg(XPATH_STREAMING)]
        _ => {
            if !safe_comp.stream.is_null() {
                unsafe {
                    fprintf(
                        output,
                        b"Streaming Expression\n\x00" as *const u8 as *const libc::c_char,
                    )
                };
            } else {
                unsafe {
                    fprintf(
                        output,
                        b"Compiled Expression : %d elements\n\x00" as *const u8
                            as *const libc::c_char,
                        (*comp).nbStep,
                    )
                };
                i = safe_comp.last;
                unsafe {
                    xmlXPathDebugDumpStepOp(
                        output,
                        comp,
                        &mut *(*comp).steps.offset(i as isize),
                        depth + 1 as libc::c_int,
                    )
                };
            };
        }
    }
}
/* XP_DEBUG_OBJ_USAGE */
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterUndefined: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterNodeset: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterBool: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterNumber: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterString: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterPoint: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterRange: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterLocset: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterUsers: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterXSLTTree: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterAll: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalUndefined: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalNodeset: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalBool: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalNumber: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalString: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalPoint: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalRange: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalLocset: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalUsers: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalXSLTTree: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalAll: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxUndefined: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxNodeset: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxBool: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxNumber: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxString: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxPoint: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxRange: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxLocset: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxUsers: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxXSLTTree: libc::c_int = 0 as libc::c_int;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxAll: libc::c_int = 0 as libc::c_int;

#[cfg(XP_DEBUG_OBJ_USAGE)]
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathDebugObjUsageReset(mut ctxt: xmlXPathContextPtr) {
    if !ctxt.is_null() {
        let safe_ctxt = unsafe { &mut *ctxt };
        if !safe_ctxt.cache.is_null() {
            let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
            let safe_cache = unsafe { &mut *cache };
            safe_cache.dbgCachedAll = 0 as libc::c_int;
            safe_cache.dbgCachedNodeset = 0 as libc::c_int;
            safe_cache.dbgCachedString = 0 as libc::c_int;
            safe_cache.dbgCachedBool = 0 as libc::c_int;
            safe_cache.dbgCachedNumber = 0 as libc::c_int;
            safe_cache.dbgCachedPoint = 0 as libc::c_int;
            safe_cache.dbgCachedRange = 0 as libc::c_int;
            safe_cache.dbgCachedLocset = 0 as libc::c_int;
            safe_cache.dbgCachedUsers = 0 as libc::c_int;
            safe_cache.dbgCachedXSLTTree = 0 as libc::c_int;
            safe_cache.dbgCachedUndefined = 0 as libc::c_int;
            safe_cache.dbgReusedAll = 0 as libc::c_int;
            safe_cache.dbgReusedNodeset = 0 as libc::c_int;
            safe_cache.dbgReusedString = 0 as libc::c_int;
            safe_cache.dbgReusedBool = 0 as libc::c_int;
            safe_cache.dbgReusedNumber = 0 as libc::c_int;
            safe_cache.dbgReusedPoint = 0 as libc::c_int;
            safe_cache.dbgReusedRange = 0 as libc::c_int;
            safe_cache.dbgReusedLocset = 0 as libc::c_int;
            safe_cache.dbgReusedUsers = 0 as libc::c_int;
            safe_cache.dbgReusedXSLTTree = 0 as libc::c_int;
            safe_cache.dbgReusedUndefined = 0 as libc::c_int
        }
    }
    xmlXPathDebugObjCounterUndefined = 0 as libc::c_int;
    xmlXPathDebugObjCounterNodeset = 0 as libc::c_int;
    xmlXPathDebugObjCounterBool = 0 as libc::c_int;
    xmlXPathDebugObjCounterNumber = 0 as libc::c_int;
    xmlXPathDebugObjCounterString = 0 as libc::c_int;
    xmlXPathDebugObjCounterPoint = 0 as libc::c_int;
    xmlXPathDebugObjCounterRange = 0 as libc::c_int;
    xmlXPathDebugObjCounterLocset = 0 as libc::c_int;
    xmlXPathDebugObjCounterUsers = 0 as libc::c_int;
    xmlXPathDebugObjCounterXSLTTree = 0 as libc::c_int;
    xmlXPathDebugObjCounterAll = 0 as libc::c_int;
    xmlXPathDebugObjTotalUndefined = 0 as libc::c_int;
    xmlXPathDebugObjTotalNodeset = 0 as libc::c_int;
    xmlXPathDebugObjTotalBool = 0 as libc::c_int;
    xmlXPathDebugObjTotalNumber = 0 as libc::c_int;
    xmlXPathDebugObjTotalString = 0 as libc::c_int;
    xmlXPathDebugObjTotalPoint = 0 as libc::c_int;
    xmlXPathDebugObjTotalRange = 0 as libc::c_int;
    xmlXPathDebugObjTotalLocset = 0 as libc::c_int;
    xmlXPathDebugObjTotalUsers = 0 as libc::c_int;
    xmlXPathDebugObjTotalXSLTTree = 0 as libc::c_int;
    xmlXPathDebugObjTotalAll = 0 as libc::c_int;
    xmlXPathDebugObjMaxUndefined = 0 as libc::c_int;
    xmlXPathDebugObjMaxNodeset = 0 as libc::c_int;
    xmlXPathDebugObjMaxBool = 0 as libc::c_int;
    xmlXPathDebugObjMaxNumber = 0 as libc::c_int;
    xmlXPathDebugObjMaxString = 0 as libc::c_int;
    xmlXPathDebugObjMaxPoint = 0 as libc::c_int;
    xmlXPathDebugObjMaxRange = 0 as libc::c_int;
    xmlXPathDebugObjMaxLocset = 0 as libc::c_int;
    xmlXPathDebugObjMaxUsers = 0 as libc::c_int;
    xmlXPathDebugObjMaxXSLTTree = 0 as libc::c_int;
    xmlXPathDebugObjMaxAll = 0 as libc::c_int;
}

#[cfg(XP_DEBUG_OBJ_USAGE)]
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathDebugObjUsageRequested(
    mut ctxt: xmlXPathContextPtr,
    mut objType: xmlXPathObjectType,
) {
    let mut isCached: libc::c_int = 0 as libc::c_int;
    if !ctxt.is_null() {
        let safe_ctxt = unsafe { &mut *ctxt };
        if !safe_ctxt.cache.is_null() {
            let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
            let safe_cache = unsafe { &mut *cache };
            isCached = 1 as libc::c_int;
            safe_cache.dbgReusedAll += 1;
            match objType as libc::c_uint {
                0 => safe_cache.dbgReusedUndefined += 1,
                1 => safe_cache.dbgReusedNodeset += 1,
                2 => safe_cache.dbgReusedBool += 1,
                3 => safe_cache.dbgReusedNumber += 1,
                4 => safe_cache.dbgReusedString += 1,
                5 => safe_cache.dbgReusedPoint += 1,
                6 => safe_cache.dbgReusedRange += 1,
                7 => safe_cache.dbgReusedLocset += 1,
                8 => safe_cache.dbgReusedUsers += 1,
                9 => safe_cache.dbgReusedXSLTTree += 1,
                _ => {}
            }
        }
    }
    match objType as libc::c_uint {
        0 => {
            if isCached == 0 {
                xmlXPathDebugObjTotalUndefined += 1
            }
            xmlXPathDebugObjCounterUndefined += 1;
            if xmlXPathDebugObjCounterUndefined > xmlXPathDebugObjMaxUndefined {
                xmlXPathDebugObjMaxUndefined = xmlXPathDebugObjCounterUndefined
            }
        }
        1 => {
            if isCached == 0 {
                xmlXPathDebugObjTotalNodeset += 1
            }
            xmlXPathDebugObjCounterNodeset += 1;
            if xmlXPathDebugObjCounterNodeset > xmlXPathDebugObjMaxNodeset {
                xmlXPathDebugObjMaxNodeset = xmlXPathDebugObjCounterNodeset
            }
        }
        2 => {
            if isCached == 0 {
                xmlXPathDebugObjTotalBool += 1
            }
            xmlXPathDebugObjCounterBool += 1;
            if xmlXPathDebugObjCounterBool > xmlXPathDebugObjMaxBool {
                xmlXPathDebugObjMaxBool = xmlXPathDebugObjCounterBool
            }
        }
        3 => {
            if isCached == 0 {
                xmlXPathDebugObjTotalNumber += 1
            }
            xmlXPathDebugObjCounterNumber += 1;
            if xmlXPathDebugObjCounterNumber > xmlXPathDebugObjMaxNumber {
                xmlXPathDebugObjMaxNumber = xmlXPathDebugObjCounterNumber
            }
        }
        4 => {
            if isCached == 0 {
                xmlXPathDebugObjTotalString += 1
            }
            xmlXPathDebugObjCounterString += 1;
            if xmlXPathDebugObjCounterString > xmlXPathDebugObjMaxString {
                xmlXPathDebugObjMaxString = xmlXPathDebugObjCounterString
            }
        }
        5 => {
            if isCached == 0 {
                xmlXPathDebugObjTotalPoint += 1
            }
            xmlXPathDebugObjCounterPoint += 1;
            if xmlXPathDebugObjCounterPoint > xmlXPathDebugObjMaxPoint {
                xmlXPathDebugObjMaxPoint = xmlXPathDebugObjCounterPoint
            }
        }
        6 => {
            if isCached == 0 {
                xmlXPathDebugObjTotalRange += 1
            }
            xmlXPathDebugObjCounterRange += 1;
            if xmlXPathDebugObjCounterRange > xmlXPathDebugObjMaxRange {
                xmlXPathDebugObjMaxRange = xmlXPathDebugObjCounterRange
            }
        }
        7 => {
            if isCached == 0 {
                xmlXPathDebugObjTotalLocset += 1
            }
            xmlXPathDebugObjCounterLocset += 1;
            if xmlXPathDebugObjCounterLocset > xmlXPathDebugObjMaxLocset {
                xmlXPathDebugObjMaxLocset = xmlXPathDebugObjCounterLocset
            }
        }
        8 => {
            if isCached == 0 {
                xmlXPathDebugObjTotalUsers += 1
            }
            xmlXPathDebugObjCounterUsers += 1;
            if xmlXPathDebugObjCounterUsers > xmlXPathDebugObjMaxUsers {
                xmlXPathDebugObjMaxUsers = xmlXPathDebugObjCounterUsers
            }
        }
        9 => {
            if isCached == 0 {
                xmlXPathDebugObjTotalXSLTTree += 1
            }
            xmlXPathDebugObjCounterXSLTTree += 1;
            if xmlXPathDebugObjCounterXSLTTree > xmlXPathDebugObjMaxXSLTTree {
                xmlXPathDebugObjMaxXSLTTree = xmlXPathDebugObjCounterXSLTTree
            }
        }
        _ => {}
    }
    if isCached == 0 {
        xmlXPathDebugObjTotalAll += 1
    }
    xmlXPathDebugObjCounterAll += 1;
    if xmlXPathDebugObjCounterAll > xmlXPathDebugObjMaxAll {
        xmlXPathDebugObjMaxAll = xmlXPathDebugObjCounterAll
    };
}

#[cfg(XP_DEBUG_OBJ_USAGE)]
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathDebugObjUsageReleased(
    mut ctxt: xmlXPathContextPtr,
    mut objType: xmlXPathObjectType,
) {
    let mut isCached: libc::c_int = 0 as libc::c_int;
    if !ctxt.is_null() {
        let safe_ctxt = unsafe { &mut *ctxt };
        if !safe_ctxt.cache.is_null() {
            let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
            let safe_cache = unsafe { &mut *cache };
            isCached = 1 as libc::c_int;
            safe_cache.dbgCachedAll += 1;
            match objType as libc::c_uint {
                0 => safe_cache.dbgCachedUndefined += 1,
                1 => safe_cache.dbgCachedNodeset += 1,
                2 => safe_cache.dbgCachedBool += 1,
                3 => safe_cache.dbgCachedNumber += 1,
                4 => safe_cache.dbgCachedString += 1,
                5 => safe_cache.dbgCachedPoint += 1,
                6 => safe_cache.dbgCachedRange += 1,
                7 => safe_cache.dbgCachedLocset += 1,
                8 => safe_cache.dbgCachedUsers += 1,
                9 => safe_cache.dbgCachedXSLTTree += 1,
                _ => {}
            }
        }
    }
    match objType as libc::c_uint {
        0 => xmlXPathDebugObjCounterUndefined -= 1,
        1 => xmlXPathDebugObjCounterNodeset -= 1,
        2 => xmlXPathDebugObjCounterBool -= 1,
        3 => xmlXPathDebugObjCounterNumber -= 1,
        4 => xmlXPathDebugObjCounterString -= 1,
        5 => xmlXPathDebugObjCounterPoint -= 1,
        6 => xmlXPathDebugObjCounterRange -= 1,
        7 => xmlXPathDebugObjCounterLocset -= 1,
        8 => xmlXPathDebugObjCounterUsers -= 1,
        9 => xmlXPathDebugObjCounterXSLTTree -= 1,
        _ => {}
    }
    xmlXPathDebugObjCounterAll -= 1;
}

#[cfg(XP_DEBUG_OBJ_USAGE)]
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathDebugObjUsageDisplay(mut ctxt: xmlXPathContextPtr) {
    let mut reqAll: libc::c_int = 0;
    let mut reqNodeset: libc::c_int = 0;
    let mut reqString: libc::c_int = 0;
    let mut reqBool: libc::c_int = 0;
    let mut reqNumber: libc::c_int = 0;
    let mut reqXSLTTree: libc::c_int = 0;
    let mut reqUndefined: libc::c_int = 0;
    let mut caAll: libc::c_int = 0 as libc::c_int;
    let mut caNodeset: libc::c_int = 0 as libc::c_int;
    let mut caString: libc::c_int = 0 as libc::c_int;
    let mut caBool: libc::c_int = 0 as libc::c_int;
    let mut caNumber: libc::c_int = 0 as libc::c_int;
    let mut caXSLTTree: libc::c_int = 0 as libc::c_int;
    let mut caUndefined: libc::c_int = 0 as libc::c_int;
    let mut reAll: libc::c_int = 0 as libc::c_int;
    let mut reNodeset: libc::c_int = 0 as libc::c_int;
    let mut reString: libc::c_int = 0 as libc::c_int;
    let mut reBool: libc::c_int = 0 as libc::c_int;
    let mut reNumber: libc::c_int = 0 as libc::c_int;
    let mut reXSLTTree: libc::c_int = 0 as libc::c_int;
    let mut reUndefined: libc::c_int = 0 as libc::c_int;
    let mut leftObjs: libc::c_int = xmlXPathDebugObjCounterAll;
    reqAll = xmlXPathDebugObjTotalAll;
    reqNodeset = xmlXPathDebugObjTotalNodeset;
    reqString = xmlXPathDebugObjTotalString;
    reqBool = xmlXPathDebugObjTotalBool;
    reqNumber = xmlXPathDebugObjTotalNumber;
    reqXSLTTree = xmlXPathDebugObjTotalXSLTTree;
    reqUndefined = xmlXPathDebugObjTotalUndefined;
    print!("# XPath object usage:\n\x00");
    if !ctxt.is_null() {
        let safe_ctxt = unsafe { &mut *ctxt };
        if !safe_ctxt.cache.is_null() {
            let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
            let safe_cache = unsafe { &mut *cache };
            reAll = safe_cache.dbgReusedAll;
            reqAll += reAll;
            reNodeset = safe_cache.dbgReusedNodeset;
            reqNodeset += reNodeset;
            reString = safe_cache.dbgReusedString;
            reqString += reString;
            reBool = safe_cache.dbgReusedBool;
            reqBool += reBool;
            reNumber = safe_cache.dbgReusedNumber;
            reqNumber += reNumber;
            reXSLTTree = safe_cache.dbgReusedXSLTTree;
            reqXSLTTree += reXSLTTree;
            reUndefined = safe_cache.dbgReusedUndefined;
            reqUndefined += reUndefined;
            caAll = safe_cache.dbgCachedAll;
            caBool = safe_cache.dbgCachedBool;
            caNodeset = safe_cache.dbgCachedNodeset;
            caString = safe_cache.dbgCachedString;
            caNumber = safe_cache.dbgCachedNumber;
            caXSLTTree = safe_cache.dbgCachedXSLTTree;
            caUndefined = safe_cache.dbgCachedUndefined;
            if !safe_cache.nodesetObjs.is_null() {
                leftObjs -= (*safe_cache.nodesetObjs).number
            }
            if !safe_cache.stringObjs.is_null() {
                leftObjs -= (*safe_cache.stringObjs).number
            }
            if !safe_cache.booleanObjs.is_null() {
                leftObjs -= (*safe_cache.booleanObjs).number
            }
            if !safe_cache.numberObjs.is_null() {
                leftObjs -= (*safe_cache.numberObjs).number
            }
            if !safe_cache.miscObjs.is_null() {
                leftObjs -= (*safe_cache.miscObjs).number
            }
        }
    }
    print!("# all\n\x00");
    print!("#   total  : {}\n\x00", reqAll);
    print!("#   left  : {}\n\x00", leftObjs);
    print!("#   created: {}\n\x00", xmlXPathDebugObjTotalAll);
    print!("#   reused : {}\n\x00", reAll);
    print!("#   max    : {}\n\x00", xmlXPathDebugObjMaxAll);
    print!("# node-sets\n\x00");
    print!("#   total  : {}\n\x00", reqNodeset);
    print!("#   created: {}\n\x00", xmlXPathDebugObjTotalNodeset);
    print!("#   reused : {}\n\x00", reNodeset);
    print!("#   max    : {}\n\x00", xmlXPathDebugObjMaxNodeset);
    print!("# strings\n\x00");
    print!("#   total  : {}\n\x00", reqString);
    print!("#   created: {}\n\x00", xmlXPathDebugObjTotalString);
    print!("#   reused : {}\n\x00", reString);
    print!("#   max    : {}\n\x00", xmlXPathDebugObjMaxString);
    print!("# booleans\n\x00");
    print!("#   total  : {}\n\x00", reqBool);
    print!("#   created: {}\n\x00", xmlXPathDebugObjTotalBool);
    print!("#   reused : {}\n\x00", reBool);
    print!("#   max    : {}\n\x00", xmlXPathDebugObjMaxBool);
    print!("# numbers\n\x00");
    print!("#   total  : {}\n\x00", reqNumber);
    print!("#   created: {}\n\x00", xmlXPathDebugObjTotalNumber);
    print!("#   reused : {}\n\x00", reNumber);
    print!("#   max    : {}\n\x00", xmlXPathDebugObjMaxNumber);
    print!("# XSLT result tree fragments\n\x00");
    print!("#   total  : {}\n\x00", reqXSLTTree);
    print!("#   created: {}\n\x00", xmlXPathDebugObjTotalXSLTTree);
    print!("#   reused : {}\n\x00", reXSLTTree);
    print!("#   max    : {}\n\x00", xmlXPathDebugObjMaxXSLTTree);
    print!("# undefined\n\x00");
    print!("#   total  : {}\n\x00", reqUndefined);
    print!("#   created: {}\n\x00", xmlXPathDebugObjTotalUndefined);
    print!("#   reused : {}\n\x00", reUndefined);
    print!("#   max    : {}\n\x00", xmlXPathDebugObjMaxUndefined);
}
/* LIBXML_DEBUG_ENABLED */
/* ***********************************************************************
 *									*
 *			XPath object caching				*
 *									*
 ************************************************************************/
/* *
 * xmlXPathNewCache: *
 * Create a new object cache
 *
 * Returns the xmlXPathCache just allocated.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathNewCache() -> xmlXPathContextCachePtr {
    let mut ret: xmlXPathContextCachePtr = 0 as *mut xmlXPathContextCache;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathContextCache>() as libc::c_ulong
        ) as xmlXPathContextCachePtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating object cache\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathContextCachePtr;
    }
    let safe_ret = unsafe { &mut *ret };
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathContextCache>() as libc::c_ulong,
        )
    };
    safe_ret.maxNodeset = 100 as libc::c_int;
    safe_ret.maxString = 100 as libc::c_int;
    safe_ret.maxBoolean = 100 as libc::c_int;
    safe_ret.maxNumber = 100 as libc::c_int;
    safe_ret.maxMisc = 100 as libc::c_int;
    return ret;
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheFreeObjectList(mut list: xmlPointerListPtr) {
    let mut i: libc::c_int = 0;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if list.is_null() {
        return;
    }
    let safe_list = unsafe { &mut *list };
    i = 0 as libc::c_int;
    while i < safe_list.number {
        obj = unsafe { *(*list).items.offset(i as isize) as xmlXPathObjectPtr };
        let safe_obj = unsafe { &mut *obj };
        /*
        	* Note that it is already assured that we don't need to
        	* look out for namespace nodes in the node-set.
        	*/
        if !safe_obj.nodesetval.is_null() {
            if unsafe { !(*(*obj).nodesetval).nodeTab.is_null() } {
                unsafe {
                    xmlFree.expect("non-null function pointer")(
                        (*(*obj).nodesetval).nodeTab as *mut libc::c_void,
                    )
                };
            }
            unsafe {
                xmlFree.expect("non-null function pointer")((*obj).nodesetval as *mut libc::c_void)
            };
        }
        unsafe { xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void) };
        i += 1;
        match () {
            #[cfg(XP_DEBUG_OBJ_USAGE)]
            _ => {
                xmlXPathDebugObjCounterAll -= 1;
            }
            #[cfg(not(XP_DEBUG_OBJ_USAGE))]
            _ => {}
        };
    }
    unsafe { xmlPointerListFree(list) };
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathFreeCache(mut cache: xmlXPathContextCachePtr) {
    if cache.is_null() {
        return;
    }
    let safe_cache = unsafe { &mut *cache };
    if !safe_cache.nodesetObjs.is_null() {
        unsafe { xmlXPathCacheFreeObjectList((*cache).nodesetObjs) };
    }
    if !safe_cache.stringObjs.is_null() {
        unsafe { xmlXPathCacheFreeObjectList((*cache).stringObjs) };
    }
    if !safe_cache.booleanObjs.is_null() {
        unsafe { xmlXPathCacheFreeObjectList((*cache).booleanObjs) };
    }
    if !safe_cache.numberObjs.is_null() {
        unsafe { xmlXPathCacheFreeObjectList((*cache).numberObjs) };
    }
    if !safe_cache.miscObjs.is_null() {
        unsafe { xmlXPathCacheFreeObjectList((*cache).miscObjs) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(cache as *mut libc::c_void) };
}
/* *
 * xmlXPathContextSetCache: *
 * @ctxt: the XPath context
 * @active: enables/disables (creates/frees) the cache
 * @value: a value with semantics dependent on @options
 * @options: options (currently only the value 0 is used) *
 * Creates/frees an object cache on the XPath context.
 * If activates XPath objects (xmlXPathObject) will be cached internally
 * to be reused.
 * @options: *   0: This will set the XPath object caching: *      @value: *        This will set the maximum number of XPath objects
 *        to be cached per slot
 *        There are 5 slots for: node-set, string, number, boolean, and
 *        misc objects. Use <0 for the default number (100).
 *   Other values for @options have currently no effect.
 *
 * Returns 0 if the setting succeeded, and -1 on API or internal errors.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathContextSetCache(
    mut ctxt: xmlXPathContextPtr,
    mut active: libc::c_int,
    mut value: libc::c_int,
    mut options: libc::c_int,
) -> libc::c_int {
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if active != 0 {
        let mut cache: xmlXPathContextCachePtr = 0 as *mut xmlXPathContextCache;
        if safe_ctxt.cache.is_null() {
            safe_ctxt.cache = xmlXPathNewCache() as *mut libc::c_void;
            if safe_ctxt.cache.is_null() {
                return -(1 as libc::c_int);
            }
        }
        cache = safe_ctxt.cache as xmlXPathContextCachePtr;
        let safe_cache = unsafe { &mut *cache };
        if options == 0 as libc::c_int {
            if value < 0 as libc::c_int {
                value = 100 as libc::c_int
            }
            safe_cache.maxNodeset = value;
            safe_cache.maxString = value;
            safe_cache.maxNumber = value;
            safe_cache.maxBoolean = value;
            safe_cache.maxMisc = value
        }
    } else if !safe_ctxt.cache.is_null() {
        unsafe { xmlXPathFreeCache((*ctxt).cache as xmlXPathContextCachePtr) };
        safe_ctxt.cache = 0 as *mut libc::c_void
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathCacheWrapNodeSet: * @ctxt: the XPath context
 * @val: the NodePtr value
 *
 * This is the cached version of xmlXPathWrapNodeSet().
 * Wrap the Nodeset @val in a new xmlXPathObjectPtr
 *
 * Returns the created or reused object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheWrapNodeSet(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlNodeSetPtr,
) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        let safe_cache = unsafe { &mut *cache };
        if unsafe {
            !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 as libc::c_int
        } {
            unsafe {
                let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                (*(*cache).miscObjs).number -= 1;
                ret = *(*(*cache).miscObjs)
                    .items
                    .offset((*(*cache).miscObjs).number as isize)
                    as xmlXPathObjectPtr;
                (*ret).type_0 = XPATH_NODESET;
                (*ret).nodesetval = val;
                match () {
                    #[cfg(XP_DEBUG_OBJ_USAGE)]
                    _ => {
                        unsafe { xmlXPathDebugObjUsageRequested(ctxt, XPATH_NODESET) };
                    }
                    #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                    _ => {}
                };
                return ret;
            }
        }
    }
    return unsafe { xmlXPathWrapNodeSet(val) };
}
/* *
 * xmlXPathCacheWrapString: * @ctxt: the XPath context
 * @val: the xmlChar * value
 *
 * This is the cached version of xmlXPathWrapString().
 * Wraps the @val string into an XPath object.
 *
 * Returns the created or reused object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheWrapString(
    mut ctxt: xmlXPathContextPtr,
    mut val: *mut xmlChar,
) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        let safe_cache = unsafe { &mut *cache };
        if unsafe {
            !(*cache).stringObjs.is_null() && (*(*cache).stringObjs).number != 0 as libc::c_int
        } {
            unsafe {
                let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                (*(*cache).stringObjs).number -= 1;
                ret = *(*(*cache).stringObjs)
                    .items
                    .offset((*(*cache).stringObjs).number as isize)
                    as xmlXPathObjectPtr;
                (*ret).type_0 = XPATH_STRING;
                (*ret).stringval = val;
                match () {
                    #[cfg(XP_DEBUG_OBJ_USAGE)]
                    _ => {
                        xmlXPathDebugObjUsageRequested(ctxt, XPATH_STRING);
                    }
                    #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                    _ => {}
                };
                return ret;
            }
        } else {
            if unsafe {
                !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 as libc::c_int
            } {
                unsafe {
                    let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                    /*
                     * Fallback to misc-cache.
                     */
                    (*(*cache).miscObjs).number -= 1;
                    ret_0 = *(*(*cache).miscObjs)
                        .items
                        .offset((*(*cache).miscObjs).number as isize)
                        as xmlXPathObjectPtr;
                    (*ret_0).type_0 = XPATH_STRING;
                    (*ret_0).stringval = val;
                    match () {
                        #[cfg(XP_DEBUG_OBJ_USAGE)]
                        _ => {
                            xmlXPathDebugObjUsageRequested(ctxt, XPATH_STRING);
                        }
                        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                        _ => {}
                    };
                    return ret_0;
                }
            }
        }
    }
    return unsafe { xmlXPathWrapString(val) };
}

/* *
 * xmlXPathCacheNewNodeSet: * @ctxt: the XPath context
 * @val: the NodePtr value
 *
 * This is the cached version of xmlXPathNewNodeSet().
 * Acquire an xmlXPathObjectPtr of type NodeSet and initialize
 * it with the single Node @val
 *
 * Returns the created or reused object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheNewNodeSet(mut ctxt: xmlXPathContextPtr, mut val: xmlNodePtr) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        let safe_cache = unsafe { &mut *cache };
        if unsafe {
            !(*cache).nodesetObjs.is_null() && (*(*cache).nodesetObjs).number != 0 as libc::c_int
        } {
            unsafe {
                let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                /*
                 * Use the nodeset-cache.
                 */
                (*(*cache).nodesetObjs).number -= 1;
                ret = *(*(*cache).nodesetObjs)
                    .items
                    .offset((*(*cache).nodesetObjs).number as isize)
                    as xmlXPathObjectPtr;
                (*ret).type_0 = XPATH_NODESET;
                (*ret).boolval = 0 as libc::c_int;
                if !val.is_null() {
                    if (*(*ret).nodesetval).nodeMax == 0 as libc::c_int
                        || (*val).type_0 as libc::c_uint
                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                    {
                        /* TODO: Check memory error. */
                        xmlXPathNodeSetAddUnique((*ret).nodesetval, val);
                    } else {
                        let ref mut fresh30 = *(*(*ret).nodesetval)
                            .nodeTab
                            .offset(0 as libc::c_int as isize);
                        *fresh30 = val;
                        (*(*ret).nodesetval).nodeNr = 1 as libc::c_int
                    }
                }
                match () {
                    #[cfg(XP_DEBUG_OBJ_USAGE)]
                    _ => {
                        xmlXPathDebugObjUsageRequested(ctxt, XPATH_NODESET);
                    }
                    #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                    _ => {}
                };
                return ret;
            }
        } else {
            if unsafe {
                !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 as libc::c_int
            } {
                unsafe {
                    let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                    /*
                     * Fallback to misc-cache.
                     */
                    (*(*cache).miscObjs).number -= 1;
                    ret_0 = *(*(*cache).miscObjs)
                        .items
                        .offset((*(*cache).miscObjs).number as isize)
                        as xmlXPathObjectPtr;
                    (*ret_0).type_0 = XPATH_NODESET;
                    (*ret_0).boolval = 0 as libc::c_int;
                    (*ret_0).nodesetval = xmlXPathNodeSetCreate(val);
                    if (*ret_0).nodesetval.is_null() {
                        (*ctxt).lastError.domain = XML_FROM_XPATH as libc::c_int;
                        (*ctxt).lastError.code = XML_ERR_NO_MEMORY as libc::c_int;
                        return 0 as xmlXPathObjectPtr;
                    }
                    match () {
                        #[cfg(XP_DEBUG_OBJ_USAGE)]
                        _ => {
                            xmlXPathDebugObjUsageRequested(ctxt, XPATH_NODESET);
                        }
                        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                        _ => {}
                    };
                    return ret_0;
                }
            }
        }
    }
    return unsafe { xmlXPathNewNodeSet(val) };
}

/* *
 * xmlXPathCacheNewCString: * @ctxt: the XPath context
 * @val: the char * value
 *
 * This is the cached version of xmlXPathNewCString().
 * Acquire an xmlXPathObjectPtr of type string and of value @val
 *
 * Returns the created or reused object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheNewCString(
    mut ctxt: xmlXPathContextPtr,
    mut val: *const libc::c_char,
) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        if unsafe {
            !(*cache).stringObjs.is_null() && (*(*cache).stringObjs).number != 0 as libc::c_int
        } {
            unsafe {
                let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                (*(*cache).stringObjs).number -= 1;
                ret = *(*(*cache).stringObjs)
                    .items
                    .offset((*(*cache).stringObjs).number as isize)
                    as xmlXPathObjectPtr;
                (*ret).type_0 = XPATH_STRING;
                (*ret).stringval = xmlStrdup(val as *mut xmlChar);
                match () {
                    #[cfg(XP_DEBUG_OBJ_USAGE)]
                    _ => {
                        xmlXPathDebugObjUsageRequested(ctxt, XPATH_STRING);
                    }
                    #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                    _ => {}
                };
                return ret;
            }
        } else {
            if unsafe {
                !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 as libc::c_int
            } {
                unsafe {
                    let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                    (*(*cache).miscObjs).number -= 1;
                    ret_0 = *(*(*cache).miscObjs)
                        .items
                        .offset((*(*cache).miscObjs).number as isize)
                        as xmlXPathObjectPtr;
                    (*ret_0).type_0 = XPATH_STRING;
                    (*ret_0).stringval = xmlStrdup(val as *mut xmlChar);
                    match () {
                        #[cfg(XP_DEBUG_OBJ_USAGE)]
                        _ => {
                            xmlXPathDebugObjUsageRequested(ctxt, XPATH_STRING);
                        }
                        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                        _ => {}
                    };
                    return ret_0;
                }
            }
        }
    }
    return unsafe { xmlXPathNewCString(val) };
}
/* *
 * xmlXPathCacheNewString: * @ctxt: the XPath context
 * @val: the xmlChar * value
 *
 * This is the cached version of xmlXPathNewString().
 * Acquire an xmlXPathObjectPtr of type string and of value @val
 *
 * Returns the created or reused object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheNewString(
    mut ctxt: xmlXPathContextPtr,
    mut val: *const xmlChar,
) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        if unsafe {
            !(*cache).stringObjs.is_null() && (*(*cache).stringObjs).number != 0 as libc::c_int
        } {
            unsafe {
                let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                (*(*cache).stringObjs).number -= 1;
                ret = *(*(*cache).stringObjs)
                    .items
                    .offset((*(*cache).stringObjs).number as isize)
                    as xmlXPathObjectPtr;
                (*ret).type_0 = XPATH_STRING;
                if !val.is_null() {
                    (*ret).stringval = xmlStrdup(val)
                } else {
                    (*ret).stringval =
                        xmlStrdup(b"\x00" as *const u8 as *const libc::c_char as *const xmlChar)
                }
                match () {
                    #[cfg(XP_DEBUG_OBJ_USAGE)]
                    _ => {
                        xmlXPathDebugObjUsageRequested(ctxt, XPATH_STRING);
                    }
                    #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                    _ => {}
                };
                return ret;
            }
        } else {
            if unsafe {
                !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 as libc::c_int
            } {
                unsafe {
                    let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                    (*(*cache).miscObjs).number -= 1;
                    ret_0 = *(*(*cache).miscObjs)
                        .items
                        .offset((*(*cache).miscObjs).number as isize)
                        as xmlXPathObjectPtr;
                    (*ret_0).type_0 = XPATH_STRING;
                    if !val.is_null() {
                        (*ret_0).stringval = xmlStrdup(val)
                    } else {
                        (*ret_0).stringval =
                            xmlStrdup(b"\x00" as *const u8 as *const libc::c_char as *const xmlChar)
                    }
                    match () {
                        #[cfg(XP_DEBUG_OBJ_USAGE)]
                        _ => {
                            xmlXPathDebugObjUsageRequested(ctxt, XPATH_STRING);
                        }
                        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                        _ => {}
                    };
                    return ret_0;
                }
            }
        }
    }
    return unsafe { xmlXPathNewString(val) };
}
/* *
 * xmlXPathCacheNewBoolean: * @ctxt: the XPath context
 * @val: the boolean value
 *
 * This is the cached version of xmlXPathNewBoolean().
 * Acquires an xmlXPathObjectPtr of type boolean and of value @val
 *
 * Returns the created or reused object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheNewBoolean(
    mut ctxt: xmlXPathContextPtr,
    mut val: libc::c_int,
) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        if unsafe {
            !(*cache).booleanObjs.is_null() && (*(*cache).booleanObjs).number != 0 as libc::c_int
        } {
            unsafe {
                let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                (*(*cache).booleanObjs).number -= 1;
                ret = *(*(*cache).booleanObjs)
                    .items
                    .offset((*(*cache).booleanObjs).number as isize)
                    as xmlXPathObjectPtr;
                (*ret).type_0 = XPATH_BOOLEAN;
                (*ret).boolval = (val != 0 as libc::c_int) as libc::c_int;
                match () {
                    #[cfg(XP_DEBUG_OBJ_USAGE)]
                    _ => {
                        xmlXPathDebugObjUsageRequested(ctxt, XPATH_BOOLEAN);
                    }
                    #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                    _ => {}
                };
                return ret;
            }
        } else {
            if unsafe {
                !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 as libc::c_int
            } {
                unsafe {
                    let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                    (*(*cache).miscObjs).number -= 1;
                    ret_0 = *(*(*cache).miscObjs)
                        .items
                        .offset((*(*cache).miscObjs).number as isize)
                        as xmlXPathObjectPtr;
                    (*ret_0).type_0 = XPATH_BOOLEAN;
                    (*ret_0).boolval = (val != 0 as libc::c_int) as libc::c_int;
                    match () {
                        #[cfg(XP_DEBUG_OBJ_USAGE)]
                        _ => {
                            xmlXPathDebugObjUsageRequested(ctxt, XPATH_BOOLEAN);
                        }
                        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                        _ => {}
                    };
                    return ret_0;
                }
            }
        }
    }
    return unsafe { xmlXPathNewBoolean(val) };
}
/* *
 * xmlXPathCacheNewFloat: * @ctxt: the XPath context
 * @val: the double value
 *
 * This is the cached version of xmlXPathNewFloat().
 * Acquires an xmlXPathObjectPtr of type double and of value @val
 *
 * Returns the created or reused object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheNewFloat(
    mut ctxt: xmlXPathContextPtr,
    mut val: libc::c_double,
) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        if unsafe {
            !(*cache).numberObjs.is_null() && (*(*cache).numberObjs).number != 0 as libc::c_int
        } {
            unsafe {
                let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                (*(*cache).numberObjs).number -= 1;
                ret = *(*(*cache).numberObjs)
                    .items
                    .offset((*(*cache).numberObjs).number as isize)
                    as xmlXPathObjectPtr;
                (*ret).type_0 = XPATH_NUMBER;
                (*ret).floatval = val;
                match () {
                    #[cfg(XP_DEBUG_OBJ_USAGE)]
                    _ => {
                        xmlXPathDebugObjUsageRequested(ctxt, XPATH_NUMBER);
                    }
                    #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                    _ => {}
                };
                return ret;
            }
        } else {
            if unsafe {
                !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 as libc::c_int
            } {
                unsafe {
                    let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                    (*(*cache).miscObjs).number -= 1;
                    ret_0 = *(*(*cache).miscObjs)
                        .items
                        .offset((*(*cache).miscObjs).number as isize)
                        as xmlXPathObjectPtr;
                    (*ret_0).type_0 = XPATH_NUMBER;
                    (*ret_0).floatval = val;
                    match () {
                        #[cfg(XP_DEBUG_OBJ_USAGE)]
                        _ => {
                            xmlXPathDebugObjUsageRequested(ctxt, XPATH_NUMBER);
                        }
                        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
                        _ => {}
                    };
                    return ret_0;
                }
            }
        }
    }
    return unsafe { xmlXPathNewFloat(val) };
}
/* *
 * xmlXPathCacheConvertString: * @ctxt: the XPath context
 * @val: an XPath object
 *
 * This is the cached version of xmlXPathConvertString().
 * Converts an existing object to its string() equivalent
 *
 * Returns a created or reused object, the old one is freed (cached) *         (or the operation is done directly on @val) */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheConvertString(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut res: *mut xmlChar = 0 as *mut xmlChar;
    if val.is_null() {
        return unsafe {
            xmlXPathCacheNewCString(ctxt, b"\x00" as *const u8 as *const libc::c_char)
        };
    }
    let safe_val = unsafe { &mut *val };
    match safe_val.type_0 as libc::c_uint {
        1 | 9 => res = unsafe { xmlXPathCastNodeSetToString((*val).nodesetval) },
        4 => return val,
        2 => res = unsafe { xmlXPathCastBooleanToString((*val).boolval) },
        3 => res = unsafe { xmlXPathCastNumberToString((*val).floatval) },
        8 | 5 | 6 | 7 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                    b"xpath.c\x00" as *const u8 as *const libc::c_char,
                    2712 as libc::c_int,
                )
            };
        }
        0 | _ => {
            match () {
                #[cfg(DEBUG_EXPR)]
                _ => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"STRING: undefined\n" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                #[cfg(not(DEBUG_EXPR))]
                _ => {}
            };
        }
    }
    unsafe { xmlXPathReleaseObject(ctxt, val) };
    if res.is_null() {
        return unsafe {
            xmlXPathCacheNewCString(ctxt, b"\x00" as *const u8 as *const libc::c_char)
        };
    }
    return unsafe { xmlXPathCacheWrapString(ctxt, res) };
}
/* *
 * xmlXPathCacheObjectCopy: * @ctxt: the XPath context
 * @val: the original object
 *
 * This is the cached version of xmlXPathObjectCopy().
 * Acquire a copy of a given object
 *
 * Returns a created or reused created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheObjectCopy(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    if val.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    let safe_val = unsafe { &mut *val };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        match safe_val.type_0 as libc::c_uint {
            1 => {
                return unsafe {
                    xmlXPathCacheWrapNodeSet(
                        ctxt,
                        xmlXPathNodeSetMerge(0 as xmlNodeSetPtr, (*val).nodesetval),
                    )
                }
            }
            4 => return unsafe { xmlXPathCacheNewString(ctxt, (*val).stringval) },
            2 => return unsafe { xmlXPathCacheNewBoolean(ctxt, (*val).boolval) },
            3 => return unsafe { xmlXPathCacheNewFloat(ctxt, (*val).floatval) },
            _ => {}
        }
    }
    return unsafe { xmlXPathObjectCopy(val) };
}
/* *
 * xmlXPathCacheConvertBoolean: * @ctxt: the XPath context
 * @val: an XPath object
 *
 * This is the cached version of xmlXPathConvertBoolean().
 * Converts an existing object to its boolean() equivalent
 *
 * Returns a created or reused object, the old one is freed (or the operation
 *         is done directly on @val) */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheConvertBoolean(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return unsafe { xmlXPathCacheNewBoolean(ctxt, 0 as libc::c_int) };
    }
    let safe_val = unsafe { &mut *val };
    if safe_val.type_0 as libc::c_uint == XPATH_BOOLEAN as libc::c_int as libc::c_uint {
        return val;
    }
    ret = unsafe { xmlXPathCacheNewBoolean(ctxt, xmlXPathCastToBoolean(val)) };
    unsafe { xmlXPathReleaseObject(ctxt, val) };
    return ret;
}
/* *
 * xmlXPathCacheConvertNumber: * @ctxt: the XPath context
 * @val: an XPath object
 *
 * This is the cached version of xmlXPathConvertNumber().
 * Converts an existing object to its number() equivalent
 *
 * Returns a created or reused object, the old one is freed (or the operation
 *         is done directly on @val) */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCacheConvertNumber(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return unsafe { xmlXPathCacheNewFloat(ctxt, 0.0f64) };
    }
    let safe_val = unsafe { &mut *val };
    if safe_val.type_0 as libc::c_uint == XPATH_NUMBER as libc::c_int as libc::c_uint {
        return val;
    }
    ret = unsafe { xmlXPathCacheNewFloat(ctxt, xmlXPathCastToNumber(val)) };
    unsafe { xmlXPathReleaseObject(ctxt, val) };
    return ret;
}
/* ***********************************************************************
 *									*
 *		Parser stacks related functions and macros		*
 *									*
 ************************************************************************/
/* *
 * xmlXPathSetFrame: * @ctxt: an XPath parser context
 *
 * Set the callee evaluation frame
 *
 * Returns the previous frame value to be restored once done
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathSetFrame(mut ctxt: xmlXPathParserContextPtr) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if ctxt.is_null() {
        return 0 as libc::c_int;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    ret = safe_ctxt.valueFrame;
    safe_ctxt.valueFrame = safe_ctxt.valueNr;
    return ret;
}
/* *
 * xmlXPathPopFrame: * @ctxt: an XPath parser context
 * @frame: the previous frame value
 *
 * Remove the callee evaluation frame
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathPopFrame(mut ctxt: xmlXPathParserContextPtr, mut frame: libc::c_int) {
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame {
        unsafe {
            xmlXPatherror(
                ctxt,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                2840 as libc::c_int,
                XPATH_STACK_ERROR as libc::c_int,
            )
        };
    }
    safe_ctxt.valueFrame = frame;
}
/* TODO: remap to xmlXPathValuePop and Push. */
/* *
 * valuePop: * @ctxt: an XPath evaluation context
 *
 * Pops the top XPath object from the value stack
 *
 * Returns the XPath object just removed
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn valuePop(mut ctxt: xmlXPathParserContextPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.valueNr <= 0 as libc::c_int {
        return 0 as xmlXPathObjectPtr;
    }
    if safe_ctxt.valueNr <= safe_ctxt.valueFrame {
        unsafe {
            xmlXPatherror(
                ctxt,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                2862 as libc::c_int,
                XPATH_STACK_ERROR as libc::c_int,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    safe_ctxt.valueNr -= 1;
    if safe_ctxt.valueNr > 0 as libc::c_int {
        safe_ctxt.value = unsafe {
            *(*ctxt)
                .valueTab
                .offset(((*ctxt).valueNr - 1 as libc::c_int) as isize)
        }
    } else {
        safe_ctxt.value = 0 as xmlXPathObjectPtr
    }
    unsafe {
        ret = *(*ctxt).valueTab.offset((*ctxt).valueNr as isize);
        let ref mut fresh31 = *(*ctxt).valueTab.offset((*ctxt).valueNr as isize);
        *fresh31 = 0 as xmlXPathObjectPtr;
        return ret;
    }
}
/* *
 * valuePush: * @ctxt: an XPath evaluation context
 * @value: the XPath object
 *
 * Pushes a new XPath object on top of the value stack. If value is NULL, * a memory error is recorded in the parser context.
 *
 * Returns the number of items on the value stack, or -1 in case of error.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn valuePush(mut ctxt: xmlXPathParserContextPtr, mut value: xmlXPathObjectPtr) -> libc::c_int {
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if value.is_null() {
        /*
         * A NULL value typically indicates that a memory allocation failed, * so we set ctxt->error here to propagate the error.
         */
        safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int;
        return -(1 as libc::c_int);
    }
    if safe_ctxt.valueNr >= safe_ctxt.valueMax {
        let mut tmp: *mut xmlXPathObjectPtr = 0 as *mut xmlXPathObjectPtr;
        if safe_ctxt.valueMax >= 1000000 as libc::c_int {
            unsafe {
                xmlXPathPErrMemory(
                    ctxt,
                    b"XPath stack depth limit reached\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        tmp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*ctxt).valueTab as *mut libc::c_void,
                ((2 as libc::c_int * (*ctxt).valueMax) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>() as libc::c_ulong),
            ) as *mut xmlXPathObjectPtr
        };
        if tmp.is_null() {
            unsafe {
                xmlXPathPErrMemory(
                    ctxt,
                    b"pushing value\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        safe_ctxt.valueMax *= 2 as libc::c_int;
        safe_ctxt.valueTab = tmp
    }
    unsafe {
        let ref mut fresh32 = *(*ctxt).valueTab.offset((*ctxt).valueNr as isize);
        *fresh32 = value;
        (*ctxt).value = value;
        let fresh33 = (*ctxt).valueNr;
        (*ctxt).valueNr = (*ctxt).valueNr + 1;
        return fresh33;
    }
}
/*
 * Summary: internal interfaces for XML Path Language implementation
 * Description: internal interfaces for XML Path Language implementation
 *              used to build new modules on top of XPath like XPointer and
 *              XSLT
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* ***********************************************************************
 *									*
 *			Helpers						*
 *									*
 ************************************************************************/
/*
 * Many of these macros may later turn into functions. They
 * shouldn't be used in #ifdef's preprocessor instructions.
 */
/* *
 * xmlXPathSetError: * @ctxt: an XPath parser context
 * @err: an xmlXPathError code
 *
 * Raises an error.
 */
/* *
 * xmlXPathSetArityError: * @ctxt: an XPath parser context
 *
 * Raises an XPATH_INVALID_ARITY error.
 */
/* *
 * xmlXPathSetTypeError: * @ctxt: an XPath parser context
 *
 * Raises an XPATH_INVALID_TYPE error.
 */
/* *
 * xmlXPathGetError: * @ctxt: an XPath parser context
 *
 * Get the error code of an XPath context.
 *
 * Returns the context error.
 */
/* *
 * xmlXPathCheckError: * @ctxt: an XPath parser context
 *
 * Check if an XPath error was raised.
 *
 * Returns true if an error has been raised, false otherwise.
 */
/* *
 * xmlXPathGetDocument: * @ctxt: an XPath parser context
 *
 * Get the document of an XPath context.
 *
 * Returns the context document.
 */
/* *
 * xmlXPathGetContextNode: * @ctxt: an XPath parser context
 *
 * Get the context node of an XPath context.
 *
 * Returns the context node.
 */
/* *
 * xmlXPathPopBoolean: * @ctxt: an XPath parser context
 *
 * Pops a boolean from the stack, handling conversion if needed.
 * Check error with #xmlXPathCheckError.
 *
 * Returns the boolean
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathPopBoolean(mut ctxt: xmlXPathParserContextPtr) -> libc::c_int {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: libc::c_int = 0;
    obj = valuePop(ctxt);
    if obj.is_null() {
        unsafe {
            xmlXPatherror(
                ctxt,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                2935 as libc::c_int,
                XPATH_INVALID_OPERAND as libc::c_int,
            )
        };
        if !ctxt.is_null() {
            let safe_ctxt = unsafe { &mut *ctxt };
            safe_ctxt.error = XPATH_INVALID_OPERAND as libc::c_int
        }
        return 0 as libc::c_int;
    }
    let safe_obj = unsafe { &mut *obj };
    if safe_obj.type_0 as libc::c_uint != XPATH_BOOLEAN as libc::c_int as libc::c_uint {
        ret = unsafe { xmlXPathCastToBoolean(obj) }
    } else {
        ret = safe_obj.boolval
    }
    unsafe { xmlXPathReleaseObject((*ctxt).context, obj) };
    return ret;
}
/* *
 * xmlXPathPopNumber: * @ctxt: an XPath parser context
 *
 * Pops a number from the stack, handling conversion if needed.
 * Check error with #xmlXPathCheckError.
 *
 * Returns the number
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathPopNumber(mut ctxt: xmlXPathParserContextPtr) -> libc::c_double {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: libc::c_double = 0.;
    obj = unsafe { valuePop(ctxt) };
    if obj.is_null() {
        unsafe {
            xmlXPatherror(
                ctxt,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                2962 as libc::c_int,
                XPATH_INVALID_OPERAND as libc::c_int,
            )
        };
        if !ctxt.is_null() {
            let safe_ctxt = unsafe { &mut *ctxt };
            safe_ctxt.error = XPATH_INVALID_OPERAND as libc::c_int
        }
        return 0 as libc::c_int as libc::c_double;
    }
    let safe_obj = unsafe { &mut *obj };
    if safe_obj.type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint {
        ret = unsafe { xmlXPathCastToNumber(obj) }
    } else {
        ret = safe_obj.floatval
    }
    unsafe { xmlXPathReleaseObject((*ctxt).context, obj) };
    return ret;
}
/* *
 * xmlXPathPopString: * @ctxt: an XPath parser context
 *
 * Pops a string from the stack, handling conversion if needed.
 * Check error with #xmlXPathCheckError.
 *
 * Returns the string
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathPopString(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject; /* this does required strdup */
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    obj = valuePop(ctxt);
    if obj.is_null() {
        unsafe {
            xmlXPatherror(
                ctxt,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                2989 as libc::c_int,
                XPATH_INVALID_OPERAND as libc::c_int,
            )
        };
        if !ctxt.is_null() {
            let safe_ctxt = unsafe { &mut *ctxt };
            safe_ctxt.error = XPATH_INVALID_OPERAND as libc::c_int
        }
        return 0 as *mut xmlChar;
    }
    let safe_obj = unsafe { &mut *obj };
    ret = unsafe { xmlXPathCastToString(obj) };
    /* TODO: needs refactoring somewhere else */
    if safe_obj.stringval == ret {
        safe_obj.stringval = 0 as *mut xmlChar
    }
    unsafe { xmlXPathReleaseObject((*ctxt).context, obj) };
    return ret;
}
/* *
 * xmlXPathPopNodeSet: * @ctxt: an XPath parser context
 *
 * Pops a node-set from the stack, handling conversion if needed.
 * Check error with #xmlXPathCheckError.
 *
 * Returns the node-set
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathPopNodeSet(mut ctxt: xmlXPathParserContextPtr) -> xmlNodeSetPtr {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if ctxt.is_null() {
        return 0 as xmlNodeSetPtr;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.value.is_null() {
        unsafe {
            xmlXPatherror(
                ctxt,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                3016 as libc::c_int,
                XPATH_INVALID_OPERAND as libc::c_int,
            )
        };
        if !ctxt.is_null() {
            safe_ctxt.error = XPATH_INVALID_OPERAND as libc::c_int
        }
        return 0 as xmlNodeSetPtr;
    }
    if unsafe {
        !(!(*ctxt).value.is_null()
            && ((*(*ctxt).value).type_0 as libc::c_uint
                == XPATH_NODESET as libc::c_int as libc::c_uint
                || (*(*ctxt).value).type_0 as libc::c_uint
                    == XPATH_XSLT_TREE as libc::c_int as libc::c_uint))
    } {
        unsafe {
            xmlXPatherror(
                ctxt,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                3020 as libc::c_int,
                XPATH_INVALID_TYPE as libc::c_int,
            )
        };
        if !ctxt.is_null() {
            safe_ctxt.error = XPATH_INVALID_TYPE as libc::c_int
        }
        return 0 as xmlNodeSetPtr;
    }
    obj = unsafe { valuePop(ctxt) };
    let safe_obj = unsafe { &mut *obj };
    ret = safe_obj.nodesetval;
    safe_obj.nodesetval = 0 as xmlNodeSetPtr;
    unsafe { xmlXPathReleaseObject((*ctxt).context, obj) };
    return ret;
}
/* *
 * xmlXPathPopExternal: * @ctxt: an XPath parser context
 *
 * Pops an external object from the stack, handling conversion if needed.
 * Check error with #xmlXPathCheckError.
 *
 * Returns the object
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathPopExternal(mut ctxt: xmlXPathParserContextPtr) -> *mut libc::c_void {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.value.is_null() {
        unsafe {
            xmlXPatherror(
                ctxt,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                3050 as libc::c_int,
                XPATH_INVALID_OPERAND as libc::c_int,
            )
        };
        if !ctxt.is_null() {
            safe_ctxt.error = XPATH_INVALID_OPERAND as libc::c_int
        }
        return 0 as *mut libc::c_void;
    }
    if unsafe {
        (*(*ctxt).value).type_0 as libc::c_uint != XPATH_USERS as libc::c_int as libc::c_uint
    } {
        unsafe {
            xmlXPatherror(
                ctxt,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                3054 as libc::c_int,
                XPATH_INVALID_TYPE as libc::c_int,
            )
        };
        if !ctxt.is_null() {
            safe_ctxt.error = XPATH_INVALID_TYPE as libc::c_int
        }
        return 0 as *mut libc::c_void;
    }
    obj = unsafe { valuePop(ctxt) };
    let safe_obj = unsafe { &mut *obj };
    ret = safe_obj.user;
    safe_obj.user = 0 as *mut libc::c_void;
    unsafe { xmlXPathReleaseObject((*ctxt).context, obj) };
    return ret;
}
/* *
 * xmlXPathFormatNumber: * @number: number to format
 * @buffer: output buffer
 * @buffersize: size of output buffer
 *
 * Convert the number into a string representation.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathFormatNumber(
    mut number: libc::c_double,
    mut buffer: *mut libc::c_char,
    mut buffersize: libc::c_int,
) {
    match unsafe { xmlXPathIsInf(number) } {
        1 => {
            if buffersize
                > ::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as libc::c_int
            {
                unsafe {
                    snprintf(
                        buffer,
                        buffersize as libc::c_ulong,
                        b"Infinity\x00" as *const u8 as *const libc::c_char,
                    )
                };
            }
        }
        -1 => {
            if buffersize
                > ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as libc::c_int
            {
                unsafe {
                    snprintf(
                        buffer,
                        buffersize as libc::c_ulong,
                        b"-Infinity\x00" as *const u8 as *const libc::c_char,
                    )
                };
            }
        }
        _ => {
            if unsafe { xmlXPathIsNaN(number) } != 0 {
                if buffersize
                    > ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as libc::c_int
                {
                    unsafe {
                        snprintf(
                            buffer,
                            buffersize as libc::c_ulong,
                            b"NaN\x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
            } else if number == 0 as libc::c_int as libc::c_double {
                /* Omit sign for negative zero. */
                unsafe {
                    snprintf(
                        buffer,
                        buffersize as libc::c_ulong,
                        b"0\x00" as *const u8 as *const libc::c_char,
                    )
                };
            } else if number > (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
                && number < 2147483647 as libc::c_int as libc::c_double
                && number == number as libc::c_int as libc::c_double
            {
                let mut work: [libc::c_char; 30] = [0; 30];
                let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut value: libc::c_int = number as libc::c_int;
                ptr =
                    unsafe { &mut *buffer.offset(0 as libc::c_int as isize) as *mut libc::c_char };
                if value == 0 as libc::c_int {
                    let fresh34 = ptr;
                    unsafe {
                        ptr = ptr.offset(1);
                        *fresh34 = '0' as i32 as libc::c_char
                    }
                } else {
                    unsafe {
                        snprintf(
                            work.as_mut_ptr(),
                            29 as libc::c_int as libc::c_ulong,
                            b"%d\x00" as *const u8 as *const libc::c_char,
                            value,
                        )
                    };
                    cur = unsafe {
                        &mut *work.as_mut_ptr().offset(0 as libc::c_int as isize)
                            as *mut libc::c_char
                    };
                    while unsafe {
                        *cur as libc::c_int != 0
                            && (ptr.offset_from(buffer) as libc::c_long)
                                < buffersize as libc::c_long
                    } {
                        unsafe {
                            let fresh35 = cur;
                            cur = cur.offset(1);
                            let fresh36 = ptr;
                            ptr = ptr.offset(1);
                            *fresh36 = *fresh35
                        }
                    }
                }
                if unsafe { (ptr.offset_from(buffer) as libc::c_long) < buffersize as libc::c_long }
                {
                    unsafe { *ptr = 0 as libc::c_int as libc::c_char }
                } else if buffersize > 0 as libc::c_int {
                    unsafe {
                        ptr = ptr.offset(-1);
                        *ptr = 0 as libc::c_int as libc::c_char
                    }
                }
            } else {
                /*
                  For the dimension of work,
                      DBL_DIG is number of significant digits
                  EXPONENT is only needed for "scientific notation"
                      3 is sign, decimal point, and terminating zero
                  LOWER_DOUBLE_EXP is max number of leading zeroes in fraction
                  Note that this dimension is slightly (a few characters)
                  larger than actually necessary.
                */
                let mut work_0: [libc::c_char; 28] = [0; 28];
                let mut integer_place: libc::c_int = 0;
                let mut fraction_place: libc::c_int = 0;
                let mut ptr_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut after_fraction: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut absolute_value: libc::c_double = 0.;
                let mut size: libc::c_int = 0;
                absolute_value = unsafe { fabs(number) };
                /*
                 * First choose format - scientific or regular floating point.
                 * In either case, result is in work, and after_fraction points
                 * just past the fractional part.
                 */
                if (absolute_value > 1E9f64 || absolute_value < 1E-5f64) && absolute_value != 0.0f64
                {
                    /* Use scientific notation */
                    integer_place = 15 as libc::c_int
                        + (3 as libc::c_int + 2 as libc::c_int)
                        + 1 as libc::c_int;
                    fraction_place = 15 as libc::c_int - 1 as libc::c_int;
                    size = unsafe {
                        snprintf(
                            work_0.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong,
                            b"%*.*e\x00" as *const u8 as *const libc::c_char,
                            integer_place,
                            fraction_place,
                            number,
                        )
                    };
                    while size > 0 as libc::c_int
                        && work_0[size as usize] as libc::c_int != 'e' as i32
                    {
                        size -= 1
                    }
                } else {
                    /* Use regular notation */
                    if absolute_value > 0.0f64 {
                        integer_place = unsafe { log10(absolute_value) } as libc::c_int;
                        if integer_place > 0 as libc::c_int {
                            fraction_place = 15 as libc::c_int - integer_place - 1 as libc::c_int
                        } else {
                            fraction_place = 15 as libc::c_int - integer_place
                        }
                    } else {
                        fraction_place = 1 as libc::c_int
                    }
                    size = unsafe {
                        snprintf(
                            work_0.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong,
                            b"%0.*f\x00" as *const u8 as *const libc::c_char,
                            fraction_place,
                            number,
                        )
                    }
                }
                /* Remove leading spaces sometimes inserted by snprintf */
                while work_0[0 as libc::c_int as usize] as libc::c_int == ' ' as i32 {
                    ptr_0 = unsafe {
                        &mut *work_0.as_mut_ptr().offset(0 as libc::c_int as isize)
                            as *mut libc::c_char
                    };
                    loop {
                        unsafe {
                            let ref mut fresh37 = *ptr_0.offset(0 as libc::c_int as isize);
                            *fresh37 = *ptr_0.offset(1 as libc::c_int as isize);
                            if !(*fresh37 != 0) {
                                break;
                            }
                            ptr_0 = ptr_0.offset(1)
                        }
                    }
                    size -= 1
                }
                /* Remove fractional trailing zeroes */
                after_fraction = unsafe { work_0.as_mut_ptr().offset(size as isize) };
                ptr_0 = after_fraction;
                loop {
                    unsafe {
                        ptr_0 = ptr_0.offset(-1);
                        if !(*ptr_0 as libc::c_int == '0' as i32) {
                            break;
                        }
                    }
                }
                if unsafe { *ptr_0 as libc::c_int != '.' as i32 } {
                    ptr_0 = unsafe { ptr_0.offset(1) }
                }
                loop {
                    unsafe {
                        let fresh38 = after_fraction;
                        after_fraction = after_fraction.offset(1);
                        let fresh39 = ptr_0;
                        ptr_0 = ptr_0.offset(1);
                        *fresh39 = *fresh38;
                        if !(*fresh39 as libc::c_int != 0 as libc::c_int) {
                            break;
                        }
                    }
                }
                /* Finally copy result back to caller */
                size = unsafe {
                    strlen(work_0.as_mut_ptr()).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as libc::c_int
                };
                if size > buffersize {
                    work_0[(buffersize - 1 as libc::c_int) as usize] =
                        0 as libc::c_int as libc::c_char;
                    size = buffersize
                }
                unsafe {
                    memmove(
                        buffer as *mut libc::c_void,
                        work_0.as_mut_ptr() as *const libc::c_void,
                        size as libc::c_ulong,
                    )
                };
            }
        }
    };
}
/* *
 * Evaluation functions.
 */
/* ***********************************************************************
 *									*
 *			Routines to handle NodeSets			*
 *									*
 ************************************************************************/
/* *
 * xmlXPathOrderDocElems: * @doc: an input document
 *
 * Call this routine to speed up XPath computation on static documents.
 * This stamps all the element nodes with the document order
 * Like for line information, the order is kept in the element->content
 * field, the value stored is actually - the node number (starting at -1) * to be able to differentiate from line numbers.
 *
 * Returns the number of elements found in the document or -1 in case
 *    of error.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathOrderDocElems(mut doc: xmlDocPtr) -> libc::c_long {
    let mut count: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if doc.is_null() {
        return -(1 as libc::c_int) as libc::c_long;
    }
    let safe_doc = unsafe { &mut *doc };
    cur = safe_doc.children;
    while !cur.is_null() {
        let safe_cur = unsafe { &mut *cur };
        if safe_cur.type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint {
            count += 1;
            safe_cur.content = -count as *mut libc::c_void as *mut xmlChar;
            if !safe_cur.children.is_null() {
                cur = safe_cur.children;
                continue;
            }
        }
        if !safe_cur.next.is_null() {
            cur = safe_cur.next;
        } else {
            loop {
                cur = safe_cur.parent;
                if cur.is_null() {
                    break;
                }
                if cur == doc as xmlNodePtr {
                    cur = 0 as xmlNodePtr;
                    break;
                } else if !safe_cur.next.is_null() {
                    cur = safe_cur.next;
                    break;
                } else if cur.is_null() {
                    break;
                }
            }
        }
    }
    return count;
}
/* *
 * xmlXPathCmpNodes: * @node1: the first node
 * @node2: the second node
 *
 * Compare two nodes w.r.t document order
 *
 * Returns -2 in case of error 1 if first point < second point, 0 if
 *         it's the same node, -1 otherwise
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCmpNodes(mut node1: xmlNodePtr, mut node2: xmlNodePtr) -> libc::c_int {
    let mut depth1: libc::c_int = 0;
    let mut depth2: libc::c_int = 0;
    let mut attr1: libc::c_int = 0 as libc::c_int;
    let mut attr2: libc::c_int = 0 as libc::c_int;
    let mut attrNode1: xmlNodePtr = 0 as xmlNodePtr;
    let mut attrNode2: xmlNodePtr = 0 as xmlNodePtr;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    if node1.is_null() || node2.is_null() {
        return -(2 as libc::c_int);
    }
    /*
     * a couple of optimizations which will avoid computations in most cases
     */
    let safe_node1 = unsafe { &mut *node1 };
    let safe_node2 = unsafe { &mut *node2 };
    let safe_attrNode1 = unsafe { &mut *attrNode1 };
    let safe_attrNode2 = unsafe { &mut *attrNode2 };
    let safe_cur = unsafe { &mut *cur };
    if node1 == node2 {
        /* trivial case */
        return 0 as libc::c_int;
    }
    if safe_node1.type_0 as libc::c_uint == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint {
        attr1 = 1 as libc::c_int;
        attrNode1 = node1;
        node1 = safe_node1.parent
    }
    if safe_node2.type_0 as libc::c_uint == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint {
        attr2 = 1 as libc::c_int;
        attrNode2 = node2;
        node2 = safe_node2.parent
    }
    if node1 == node2 {
        if attr1 == attr2 {
            /* not required, but we keep attributes in order */
            if attr1 != 0 as libc::c_int {
                cur = safe_attrNode2.prev;

                while !cur.is_null() {
                    if cur == attrNode1 {
                        return 1 as libc::c_int;
                    }
                    unsafe { cur = (*cur).prev }
                }
                return -(1 as libc::c_int);
            }
            return 0 as libc::c_int;
        }
        if attr2 == 1 as libc::c_int {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if unsafe {
        (*node1).type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            || (*node2).type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    } {
        return 1 as libc::c_int;
    }
    if unsafe { node1 == (*node2).prev } {
        return 1 as libc::c_int;
    }
    if unsafe { node1 == (*node2).next } {
        return -(1 as libc::c_int);
    }
    /*
     * Speedup using document order if available.
     */
    if unsafe {
        (*node1).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            && (*node2).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            && 0 as libc::c_int as libc::c_long > (*node1).content as ptrdiff_t
            && 0 as libc::c_int as libc::c_long > (*node2).content as ptrdiff_t
            && (*node1).doc == (*node2).doc
    } {
        let mut l1: ptrdiff_t = 0;
        let mut l2: ptrdiff_t = 0;
        l1 = -(unsafe { (*node1).content as ptrdiff_t });
        l2 = -(unsafe { (*node2).content as ptrdiff_t });
        if l1 < l2 {
            return 1 as libc::c_int;
        }
        if l1 > l2 {
            return -(1 as libc::c_int);
        }
    }
    /*
     * compute depth to root
     */
    depth2 = 0 as libc::c_int;
    cur = node2;
    while unsafe { !(*cur).parent.is_null() } {
        if unsafe { (*cur).parent == node1 } {
            return 1 as libc::c_int;
        }
        depth2 += 1;
        cur = unsafe { (*cur).parent }
    }
    root = cur;
    depth1 = 0 as libc::c_int;
    cur = node1;
    while unsafe { !(*cur).parent.is_null() } {
        if unsafe { (*cur).parent == node2 } {
            return -(1 as libc::c_int);
        }
        depth1 += 1;
        cur = unsafe { (*cur).parent }
    }
    /*
     * Distinct document (or distinct entities :-( ) case.
     */
    if root != cur {
        return -(2 as libc::c_int);
    }
    /*
     * get the nearest common ancestor.
     */
    while depth1 > depth2 {
        depth1 -= 1;
        node1 = unsafe { (*node1).parent }
    }
    while depth2 > depth1 {
        depth2 -= 1;
        node2 = unsafe { (*node2).parent }
    }
    while unsafe { (*node1).parent != (*node2).parent } {
        unsafe {
            node1 = (*node1).parent;
            node2 = (*node2).parent;
        }
        /* should not happen but just in case ... */
        if node1.is_null() || node2.is_null() {
            return -(2 as libc::c_int);
        }
    }
    /*
     * Find who's first.
     */
    if unsafe { node1 == (*node2).prev } {
        return 1 as libc::c_int;
    }
    if unsafe { node1 == (*node2).next } {
        return -(1 as libc::c_int);
    }
    /*
     * Speedup using document order if available.
     */
    if unsafe {
        (*node1).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            && (*node2).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            && 0 as libc::c_int as libc::c_long > (*node1).content as ptrdiff_t
            && 0 as libc::c_int as libc::c_long > (*node2).content as ptrdiff_t
            && (*node1).doc == (*node2).doc
    } {
        let mut l1_0: ptrdiff_t = 0;
        let mut l2_0: ptrdiff_t = 0;
        l1_0 = -(unsafe { (*node1).content as ptrdiff_t });
        l2_0 = -(unsafe { (*node2).content as ptrdiff_t });
        if l1_0 < l2_0 {
            return 1 as libc::c_int;
        }
        if l1_0 > l2_0 {
            return -(1 as libc::c_int);
        }
    }
    cur = unsafe { (*node1).next };
    while !cur.is_null() {
        if cur == node2 {
            return 1 as libc::c_int;
        }
        cur = unsafe { (*cur).next }
    }
    return -(1 as libc::c_int);
    /* assume there is no sibling list corruption */
}
/* *
 * xmlXPathNodeSetSort: * @set: the node set
 *
 * Sort the node set in document order
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetSort(mut set: xmlNodeSetPtr) {
    // #ifndef WITH_TIM_SORT
    let safe_set = unsafe { &mut *set };
    match () {
        #[cfg(not(WITH_TIM_SORT))]
        _ => {
            let mut i: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            let mut incr: libc::c_int = 0;
            let mut len: libc::c_int = 0;
            let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
        }
        #[cfg(WITH_TIM_SORT)]
        _ => {}
    }
    // #endif
    if set.is_null() {
        return;
    }

    // #ifndef WITH_TIM_SORT
    match () {
        #[cfg(not(WITH_TIM_SORT))]
        _ => {
            /*
             * Use the old Shell's sort implementation to sort the node-set
             * Timsort ought to be quite faster
             */
            len = safe_set.nodeNr;
            incr = len / 2 as libc::c_int;
            while incr > 0 as libc::c_int {
                i = incr;
                while i < len {
                    j = i - incr;
                    while j >= 0 as libc::c_int {
                        let mut XP_OPTIMIZED_NON_ELEM_COMPARISON_RES: libc::c_int = 0;
                        match () {
                            #[cfg(XP_OPTIMIZED_NON_ELEM_COMPARISON)]
                            _ => {
                                XP_OPTIMIZED_NON_ELEM_COMPARISON_RES = unsafe {
                                    xmlXPathCmpNodesExt(
                                        *(*set).nodeTab.offset(j as isize),
                                        *(*set).nodeTab.offset((j + incr) as isize),
                                    )
                                };
                            }
                            #[cfg(not(XP_OPTIMIZED_NON_ELEM_COMPARISON))]
                            _ => {
                                XP_OPTIMIZED_NON_ELEM_COMPARISON_RES = unsafe {
                                    xmlXPathCmpNodes(
                                        *(*set).nodeTab.offset(j as isize),
                                        *(*set).nodeTab.offset((j + incr) as isize),
                                    )
                                };
                            }
                        }
                        if !(XP_OPTIMIZED_NON_ELEM_COMPARISON_RES == -(1 as libc::c_int)) {
                            break;
                        }
                        unsafe {
                            tmp = *(*set).nodeTab.offset(j as isize);
                            let ref mut fresh40 = *(*set).nodeTab.offset(j as isize);
                            *fresh40 = *(*set).nodeTab.offset((j + incr) as isize);
                            let ref mut fresh41 = *(*set).nodeTab.offset((j + incr) as isize);
                            *fresh41 = tmp;
                        }
                        j -= incr
                    }
                    i += 1
                }
                incr /= 2 as libc::c_int
            }
        }
        // #else /* WITH_TIM_SORT */
        #[cfg(WITH_TIM_SORT)]
        _ => {
            unsafe { libxml_domnode_tim_sort((*set).nodeTab, (*set).nodeNr as size_t) };
        }
    }
    // #endif /* WITH_TIM_SORT */
}
/* *
 * xmlXPathNodeSetDupNs: * @node: the parent node of the namespace XPath node
 * @ns: the libxml namespace declaration node.
 *
 * Namespace node in libxml don't match the XPath semantic. In a node set
 * the namespace nodes are duplicated and the next pointer is set to the
 * parent node in the XPath semantic.
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathNodeSetDupNs(mut node: xmlNodePtr, mut ns: xmlNsPtr) -> xmlNodePtr {
    let safe_node = unsafe { &mut *node };
    let safe_ns = unsafe { &mut *ns };
    let mut cur: xmlNsPtr = 0 as *mut xmlNs;
    let safe_cur = unsafe { &mut *cur };
    if ns.is_null()
        || safe_ns.type_0 as libc::c_uint != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if node.is_null()
        || safe_node.type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return ns as xmlNodePtr;
    }
    /*
     * Allocate a new Namespace and fill the fields.
     */
    cur = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlNs>() as libc::c_ulong
        ) as xmlNsPtr
    };
    if cur.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"duplicating namespace\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlNodePtr;
    }
    unsafe {
        memset(
            cur as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlNs>() as libc::c_ulong,
        );
        (*cur).type_0 = XML_NAMESPACE_DECL;
    }
    if !safe_ns.href.is_null() {
        unsafe { (*cur).href = xmlStrdup((*ns).href) }
    }
    if !safe_ns.prefix.is_null() {
        unsafe { (*cur).prefix = xmlStrdup((*ns).prefix) }
    }
    unsafe {
        (*cur).next = node as xmlNsPtr;
        return cur as xmlNodePtr;
    }
}
/* *
 * Really internal functions
 */
/* *
 * xmlXPathNodeSetFreeNs: * @ns: the XPath namespace node found in a nodeset.
 *
 * Namespace nodes in libxml don't match the XPath semantic. In a node set
 * the namespace nodes are duplicated and the next pointer is set to the
 * parent node in the XPath semantic. Check if such a node needs to be freed
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetFreeNs(mut ns: xmlNsPtr) {
    let safe_ns = unsafe { &mut *ns };
    if ns.is_null()
        || safe_ns.type_0 as libc::c_uint != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return;
    }
    if unsafe {
        !(*ns).next.is_null()
            && (*(*ns).next).type_0 as libc::c_uint
                != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    } {
        if !safe_ns.href.is_null() {
            unsafe {
                xmlFree.expect("non-null function pointer")(
                    (*ns).href as *mut xmlChar as *mut libc::c_void,
                )
            };
        }
        if !safe_ns.prefix.is_null() {
            unsafe {
                xmlFree.expect("non-null function pointer")(
                    (*ns).prefix as *mut xmlChar as *mut libc::c_void,
                )
            };
        }
        unsafe { xmlFree.expect("non-null function pointer")(ns as *mut libc::c_void) };
    };
}
/* *
 * xmlXPathNodeSetCreate: * @val: an initial xmlNodePtr, or NULL
 *
 * Create a new xmlNodeSetPtr of type double and of value @val
 *
 * Returns the newly created object.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetCreate(mut val: xmlNodePtr) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlNodeSet>() as libc::c_ulong
        ) as xmlNodeSetPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating nodeset\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlNodeSetPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlNodeSet>() as libc::c_ulong,
        )
    };
    if !val.is_null() {
        let safe_val = unsafe { &mut *val };
        unsafe {
            (*ret).nodeTab = xmlMalloc.expect("non-null function pointer")(
                (10 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            ) as *mut xmlNodePtr
        };
        if unsafe { (*ret).nodeTab.is_null() } {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"creating nodeset\n\x00" as *const u8 as *const libc::c_char,
                );
                xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            };
            return 0 as xmlNodeSetPtr;
        }
        unsafe {
            memset(
                (*ret).nodeTab as *mut libc::c_void,
                0 as libc::c_int,
                (10 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            );
            (*ret).nodeMax = 10 as libc::c_int;
        };
        if safe_val.type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint {
            let mut ns: xmlNsPtr = val as xmlNsPtr;
            /* TODO: Check memory error. */
            unsafe {
                let fresh40 = (*ret).nodeNr;
                (*ret).nodeNr = (*ret).nodeNr + 1;
                let ref mut fresh41 = *(*ret).nodeTab.offset(fresh40 as isize);
                *fresh41 = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns)
            }
        } else {
            unsafe {
                let fresh42 = (*ret).nodeNr;
                (*ret).nodeNr = (*ret).nodeNr + 1;
                let ref mut fresh43 = *(*ret).nodeTab.offset(fresh42 as isize);
                *fresh43 = val
            }
        }
    }
    return ret;
}
/* *
 * NodeSet handling.
 */
/* *
 * xmlXPathNodeSetContains: * @cur: the node-set
 * @val: the node
 *
 * checks whether @cur contains @val
 *
 * Returns true (1) if @cur contains @val, false (0) otherwise
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetContains(mut cur: xmlNodeSetPtr, mut val: xmlNodePtr) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if cur.is_null() || val.is_null() {
        return 0 as libc::c_int;
    }
    let safe_val = unsafe { &mut *val };
    let safe_cur = unsafe { &mut *cur };
    if safe_val.type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < safe_cur.nodeNr {
            if unsafe {
                (**(*cur).nodeTab.offset(i as isize)).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            } {
                let mut ns1: xmlNsPtr = 0 as *mut xmlNs;
                let mut ns2: xmlNsPtr = 0 as *mut xmlNs;
                ns1 = val as xmlNsPtr;
                ns2 = unsafe { *(*cur).nodeTab.offset(i as isize) as xmlNsPtr };
                if ns1 == ns2 {
                    return 1 as libc::c_int;
                }
                let safe_ns1 = unsafe { &mut *ns1 };
                let safe_ns2 = unsafe { &mut *ns2 };
                if !safe_ns1.next.is_null()
                    && safe_ns2.next == safe_ns1.next
                    && unsafe { xmlStrEqual((*ns1).prefix, (*ns2).prefix) } != 0
                {
                    return 1 as libc::c_int;
                }
            }
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < safe_cur.nodeNr {
            if unsafe { *(*cur).nodeTab.offset(i as isize) == val } {
                return 1 as libc::c_int;
            }
            i += 1
        }
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathNodeSetAddNs: * @cur: the initial node set
 * @node: the hosting node
 * @ns: a the namespace node
 *
 * add a new namespace node to an existing NodeSet
 *
 * Returns 0 in case of success and -1 in case of error
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetAddNs(
    mut cur: xmlNodeSetPtr,
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let safe_ns = unsafe { &mut *ns };
    let safe_node = unsafe { &mut *node };
    let safe_cur = unsafe { &mut *cur };
    if cur.is_null()
        || ns.is_null()
        || node.is_null()
        || safe_ns.type_0 as libc::c_uint != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        || safe_node.type_0 as libc::c_uint != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    /* @@ with_ns to check whether namespace nodes should be looked at @@ */
    /*
     * prevent duplicates
     */
    i = 0 as libc::c_int;
    while i < safe_cur.nodeNr {
        if unsafe {
            !(*(*cur).nodeTab.offset(i as isize)).is_null()
                && (**(*cur).nodeTab.offset(i as isize)).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                && (*(*(*cur).nodeTab.offset(i as isize) as xmlNsPtr)).next == node as xmlNsPtr
                && xmlStrEqual(
                    (*ns).prefix,
                    (*(*(*cur).nodeTab.offset(i as isize) as xmlNsPtr)).prefix,
                ) != 0
        } {
            return 0 as libc::c_int;
        }
        i += 1
    }
    /*
     * grow the nodeTab if needed
     */
    if safe_cur.nodeMax == 0 as libc::c_int {
        unsafe {
            (*cur).nodeTab = xmlMalloc.expect("non-null function pointer")(
                (10 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            ) as *mut xmlNodePtr
        };
        if unsafe { (*cur).nodeTab.is_null() } {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"growing nodeset\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        unsafe {
            memset(
                (*cur).nodeTab as *mut libc::c_void,
                0 as libc::c_int,
                (10 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            )
        };
        unsafe { (*cur).nodeMax = 10 as libc::c_int }
    } else if safe_cur.nodeNr == safe_cur.nodeMax {
        let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        if safe_cur.nodeMax >= 10000000 as libc::c_int {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"growing nodeset hit limit\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        temp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*cur).nodeTab as *mut libc::c_void,
                (((*cur).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            ) as *mut xmlNodePtr
        };
        if temp.is_null() {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"growing nodeset\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        unsafe {
            (*cur).nodeMax *= 2 as libc::c_int;
            (*cur).nodeTab = temp
        }
    }
    /* TODO: Check memory error. */
    unsafe {
        let fresh44 = (*cur).nodeNr;
        (*cur).nodeNr = (*cur).nodeNr + 1;
        let ref mut fresh45 = *(*cur).nodeTab.offset(fresh44 as isize);
        *fresh45 = xmlXPathNodeSetDupNs(node, ns);
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathNodeSetAdd: * @cur: the initial node set
 * @val: a new xmlNodePtr
 *
 * add a new xmlNodePtr to an existing NodeSet
 *
 * Returns 0 in case of success, and -1 in case of error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetAdd(mut cur: xmlNodeSetPtr, mut val: xmlNodePtr) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if cur.is_null() || val.is_null() {
        return -(1 as libc::c_int);
    }
    /* @@ with_ns to check whether namespace nodes should be looked at @@ */
    /*
     * prevent duplicates
     */
    i = 0 as libc::c_int;
    let safe_cur = unsafe { &mut *cur };
    let safe_val = unsafe { &mut *val };
    while i < safe_cur.nodeNr {
        if unsafe { *(*cur).nodeTab.offset(i as isize) == val } {
            return 0 as libc::c_int;
        }
        i += 1
    }
    /*
     * grow the nodeTab if needed
     */
    if safe_cur.nodeMax == 0 as libc::c_int {
        unsafe {
            (*cur).nodeTab = xmlMalloc.expect("non-null function pointer")(
                (10 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            ) as *mut xmlNodePtr
        };
        if safe_cur.nodeTab.is_null() {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"growing nodeset\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        unsafe {
            memset(
                (*cur).nodeTab as *mut libc::c_void,
                0 as libc::c_int,
                (10 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            )
        };
        unsafe { (*cur).nodeMax = 10 as libc::c_int }
    } else if safe_cur.nodeNr == safe_cur.nodeMax {
        let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        if safe_cur.nodeMax >= 10000000 as libc::c_int {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"growing nodeset hit limit\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        temp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*cur).nodeTab as *mut libc::c_void,
                (((*cur).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            ) as *mut xmlNodePtr
        };
        if temp.is_null() {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"growing nodeset\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        unsafe {
            (*cur).nodeMax *= 2 as libc::c_int;
            (*cur).nodeTab = temp
        }
    }
    if safe_val.type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint {
        let mut ns: xmlNsPtr = val as xmlNsPtr;
        /* TODO: Check memory error. */
        unsafe {
            let fresh46 = (*cur).nodeNr;
            (*cur).nodeNr = (*cur).nodeNr + 1;
            let ref mut fresh47 = *(*cur).nodeTab.offset(fresh46 as isize);
            *fresh47 = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns)
        }
    } else {
        unsafe {
            let fresh48 = (*cur).nodeNr;
            (*cur).nodeNr = (*cur).nodeNr + 1;
            let ref mut fresh49 = *(*cur).nodeTab.offset(fresh48 as isize);
            *fresh49 = val
        }
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathNodeSetAddUnique: * @cur: the initial node set
 * @val: a new xmlNodePtr
 *
 * add a new xmlNodePtr to an existing NodeSet, optimized version
 * when we are sure the node is not already in the set.
 *
 * Returns 0 in case of success and -1 in case of failure
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNodeSetAddUnique(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> libc::c_int {
    if cur.is_null() || val.is_null() {
        return -(1 as libc::c_int);
    }
    /* @@ with_ns to check whether namespace nodes should be looked at @@ */
    /*
     * grow the nodeTab if needed
     */
    let safe_cur = unsafe { &mut *cur };
    let safe_val = unsafe { &mut *val };
    if safe_cur.nodeMax == 0 as libc::c_int {
        unsafe {
            (*cur).nodeTab = xmlMalloc.expect("non-null function pointer")(
                (10 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            ) as *mut xmlNodePtr
        };
        if unsafe { (*cur).nodeTab.is_null() } {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"growing nodeset\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        unsafe {
            memset(
                (*cur).nodeTab as *mut libc::c_void,
                0 as libc::c_int,
                (10 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            )
        };
        unsafe { (*cur).nodeMax = 10 as libc::c_int }
    } else if safe_cur.nodeNr == safe_cur.nodeMax {
        let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        if safe_cur.nodeMax >= 10000000 as libc::c_int {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"growing nodeset hit limit\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        temp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*cur).nodeTab as *mut libc::c_void,
                (((*cur).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            ) as *mut xmlNodePtr
        };
        if temp.is_null() {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"growing nodeset\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            return -(1 as libc::c_int);
        }
        unsafe {
            (*cur).nodeTab = temp;
            (*cur).nodeMax *= 2 as libc::c_int
        }
    }
    if safe_val.type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint {
        let mut ns: xmlNsPtr = val as xmlNsPtr;
        /* TODO: Check memory error. */
        unsafe {
            let fresh50 = (*cur).nodeNr;
            (*cur).nodeNr = (*cur).nodeNr + 1;
            let ref mut fresh51 = *(*cur).nodeTab.offset(fresh50 as isize);
            *fresh51 = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns)
        }
    } else {
        unsafe {
            let fresh52 = (*cur).nodeNr;
            (*cur).nodeNr = (*cur).nodeNr + 1;
            let ref mut fresh53 = *(*cur).nodeTab.offset(fresh52 as isize);
            *fresh53 = val
        }
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathNodeSetMerge: * @val1: the first NodeSet or NULL
 * @val2: the second NodeSet
 *
 * Merges two nodesets, all nodes from @val2 are added to @val1
 * if @val1 is NULL, a new set is created and copied from @val2
 *
 * Returns @val1 once extended or NULL in case of error.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetMerge(mut val1: xmlNodeSetPtr, mut val2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut initNr: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut n1: xmlNodePtr = 0 as *mut xmlNode;
    let mut n2: xmlNodePtr = 0 as *mut xmlNode;
    if val2.is_null() {
        return val1;
    }
    if val1.is_null() {
        val1 = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
        if val1.is_null() {
            return 0 as xmlNodeSetPtr;
        }
    }
    /* @@ with_ns to check whether namespace nodes should be looked at @@ */
    let safe_val1 = unsafe { &mut *val1 };
    let safe_val2 = unsafe { &mut *val2 };
    initNr = safe_val1.nodeNr;
    i = 0 as libc::c_int;
    while i < safe_val2.nodeNr {
        n2 = unsafe { *(*val2).nodeTab.offset(i as isize) };
        /*
         * check against duplicates
         */
        skip = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < initNr {
            n1 = unsafe { *(*val1).nodeTab.offset(j as isize) };
            let safe_n1 = unsafe { &mut *n1 };
            let safe_n2 = unsafe { &mut *n2 };
            if n1 == n2 {
                skip = 1 as libc::c_int;
                break;
            } else {
                if safe_n1.type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                    && safe_n2.type_0 as libc::c_uint
                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    if unsafe {
                        (*(n1 as xmlNsPtr)).next == (*(n2 as xmlNsPtr)).next
                            && xmlStrEqual((*(n1 as xmlNsPtr)).prefix, (*(n2 as xmlNsPtr)).prefix)
                                != 0
                    } {
                        skip = 1 as libc::c_int;
                        break;
                    }
                }
                j += 1
            }
        }
        if !(skip != 0) {
            /*
             * grow the nodeTab if needed
             */
            if safe_val1.nodeMax == 0 as libc::c_int {
                unsafe {
                    (*val1).nodeTab =
                        xmlMalloc.expect("non-null function pointer")(
                            (10 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
                        ) as *mut xmlNodePtr
                };
                if unsafe { (*val1).nodeTab.is_null() } {
                    unsafe {
                        xmlXPathErrMemory(
                            0 as xmlXPathContextPtr,
                            b"merging nodeset\n\x00" as *const u8 as *const libc::c_char,
                        )
                    };
                    return 0 as xmlNodeSetPtr;
                }
                unsafe {
                    memset(
                        (*val1).nodeTab as *mut libc::c_void,
                        0 as libc::c_int,
                        (10 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                            xmlNodePtr,
                        >(
                        )
                            as libc::c_ulong),
                    )
                };
                unsafe { (*val1).nodeMax = 10 as libc::c_int }
            } else if safe_val1.nodeNr == safe_val1.nodeMax {
                let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
                if safe_val1.nodeMax >= 10000000 as libc::c_int {
                    unsafe {
                        xmlXPathErrMemory(
                            0 as xmlXPathContextPtr,
                            b"merging nodeset hit limit\n\x00" as *const u8 as *const libc::c_char,
                        )
                    };
                    return 0 as xmlNodeSetPtr;
                }
                temp = unsafe {
                    xmlRealloc.expect("non-null function pointer")(
                        (*val1).nodeTab as *mut libc::c_void,
                        (((*val1).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
                    ) as *mut xmlNodePtr
                };
                if temp.is_null() {
                    unsafe {
                        xmlXPathErrMemory(
                            0 as xmlXPathContextPtr,
                            b"merging nodeset\n\x00" as *const u8 as *const libc::c_char,
                        )
                    };
                    return 0 as xmlNodeSetPtr;
                }
                unsafe {
                    (*val1).nodeTab = temp;
                    (*val1).nodeMax *= 2 as libc::c_int
                }
            }
            let safe_n1 = unsafe { &mut *n1 };
            let safe_n2 = unsafe { &mut *n2 };
            if safe_n2.type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint {
                let mut ns: xmlNsPtr = n2 as xmlNsPtr;
                /* TODO: Check memory error. */
                unsafe {
                    let fresh54 = (*val1).nodeNr;
                    (*val1).nodeNr = (*val1).nodeNr + 1;
                    let ref mut fresh55 = *(*val1).nodeTab.offset(fresh54 as isize);
                    *fresh55 = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns)
                }
            } else {
                unsafe {
                    let fresh56 = (*val1).nodeNr;
                    (*val1).nodeNr = (*val1).nodeNr + 1;
                    let ref mut fresh57 = *(*val1).nodeTab.offset(fresh56 as isize);
                    *fresh57 = n2
                }
            }
        }
        i += 1
    }
    return val1;
}
/* *
 * xmlXPathNodeSetMergeAndClear: * @set1: the first NodeSet or NULL
 * @set2: the second NodeSet
 *
 * Merges two nodesets, all nodes from @set2 are added to @set1.
 * Checks for duplicate nodes. Clears set2.
 *
 * Returns @set1 once extended or NULL in case of error.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathNodeSetMergeAndClear(
    mut set1: xmlNodeSetPtr,
    mut set2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut initNbSet1: libc::c_int = 0;
    let mut n1: xmlNodePtr = 0 as *mut xmlNode;
    let mut n2: xmlNodePtr = 0 as *mut xmlNode;
    let safe_set1 = unsafe { &mut *set1 };
    let safe_set2 = unsafe { &mut *set2 };
    initNbSet1 = safe_set1.nodeNr;
    i = 0 as libc::c_int;
    while i < safe_set2.nodeNr {
        n2 = unsafe { *(*set2).nodeTab.offset(i as isize) };
        /*
         * Skip duplicates.
         */
        j = 0 as libc::c_int;
        loop {
            if !(j < initNbSet1) {
                current_block = 9606288038608642794;
                break;
            }
            n1 = unsafe { *(*set1).nodeTab.offset(j as isize) };
            if n1 == n2 {
                current_block = 12675440807659640239;
                break;
            }
            let safe_n1 = unsafe { &mut *n1 };
            let safe_n2 = unsafe { &mut *n2 };
            if safe_n1.type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                && safe_n2.type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                if unsafe {
                    (*(n1 as xmlNsPtr)).next == (*(n2 as xmlNsPtr)).next
                        && xmlStrEqual((*(n1 as xmlNsPtr)).prefix, (*(n2 as xmlNsPtr)).prefix) != 0
                } {
                    /*
                    	* Free the namespace node.
                    	*/
                    unsafe {
                        let ref mut fresh58 = *(*set2).nodeTab.offset(i as isize);
                        *fresh58 = 0 as xmlNodePtr;
                        xmlXPathNodeSetFreeNs(n2 as xmlNsPtr);
                    }
                    current_block = 12675440807659640239;
                    break;
                }
            }
            j += 1
        }
        match current_block {
            9606288038608642794 => {
                /*
                 * grow the nodeTab if needed
                 */
                if safe_set1.nodeMax == 0 as libc::c_int {
                    unsafe {
                        (*set1).nodeTab = xmlMalloc.expect("non-null function pointer")(
                            (10 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
                        ) as *mut xmlNodePtr
                    };
                    if unsafe { (*set1).nodeTab.is_null() } {
                        unsafe {
                            xmlXPathErrMemory(
                                0 as xmlXPathContextPtr,
                                b"merging nodeset\n\x00" as *const u8 as *const libc::c_char,
                            )
                        };
                        return 0 as xmlNodeSetPtr;
                    }
                    unsafe {
                        memset(
                            (*set1).nodeTab as *mut libc::c_void,
                            0 as libc::c_int,
                            (10 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
                        )
                    };
                    unsafe { (*set1).nodeMax = 10 as libc::c_int }
                } else if safe_set1.nodeNr >= safe_set1.nodeMax {
                    let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
                    if safe_set1.nodeMax >= 10000000 as libc::c_int {
                        unsafe {
                            xmlXPathErrMemory(
                                0 as xmlXPathContextPtr,
                                b"merging nodeset hit limit\n\x00" as *const u8
                                    as *const libc::c_char,
                            )
                        };
                        return 0 as xmlNodeSetPtr;
                    }
                    temp = unsafe {
                        xmlRealloc.expect("non-null function pointer")(
                            (*set1).nodeTab as *mut libc::c_void,
                            (((*set1).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
                        ) as *mut xmlNodePtr
                    };
                    if temp.is_null() {
                        unsafe {
                            xmlXPathErrMemory(
                                0 as xmlXPathContextPtr,
                                b"merging nodeset\n\x00" as *const u8 as *const libc::c_char,
                            )
                        };
                        return 0 as xmlNodeSetPtr;
                    }
                    unsafe {
                        (*set1).nodeTab = temp;
                        (*set1).nodeMax *= 2 as libc::c_int
                    }
                }
                unsafe {
                    let fresh59 = (*set1).nodeNr;
                    (*set1).nodeNr = (*set1).nodeNr + 1;
                    let ref mut fresh60 = *(*set1).nodeTab.offset(fresh59 as isize);
                    *fresh60 = n2
                }
            }
            _ => {}
        }
        i += 1
    }
    unsafe { (*set2).nodeNr = 0 as libc::c_int };
    return set1;
}
/* *
 * xmlXPathNodeSetMergeAndClearNoDupls: * @set1: the first NodeSet or NULL
 * @set2: the second NodeSet
 *
 * Merges two nodesets, all nodes from @set2 are added to @set1.
 * Doesn't check for duplicate nodes. Clears set2.
 *
 * Returns @set1 once extended or NULL in case of error.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathNodeSetMergeAndClearNoDupls(
    mut set1: xmlNodeSetPtr,
    mut set2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let mut i: libc::c_int = 0;
    let mut n2: xmlNodePtr = 0 as *mut xmlNode;
    let safe_set1 = unsafe { &mut *set1 };
    let safe_set2 = unsafe { &mut *set2 };
    i = 0 as libc::c_int;
    while i < safe_set2.nodeNr {
        n2 = unsafe { *(*set2).nodeTab.offset(i as isize) };
        if safe_set1.nodeMax == 0 as libc::c_int {
            unsafe {
                (*set1).nodeTab = xmlMalloc.expect("non-null function pointer")(
                    (10 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
                ) as *mut xmlNodePtr
            };
            if unsafe { (*set1).nodeTab.is_null() } {
                unsafe {
                    xmlXPathErrMemory(
                        0 as xmlXPathContextPtr,
                        b"merging nodeset\n\x00" as *const u8 as *const libc::c_char,
                    )
                };
                return 0 as xmlNodeSetPtr;
            }
            unsafe {
                memset(
                    (*set1).nodeTab as *mut libc::c_void,
                    0 as libc::c_int,
                    (10 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
                )
            };
            unsafe { (*set1).nodeMax = 10 as libc::c_int }
        } else if safe_set1.nodeNr >= safe_set1.nodeMax {
            let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
            if safe_set1.nodeMax >= 10000000 as libc::c_int {
                unsafe {
                    xmlXPathErrMemory(
                        0 as xmlXPathContextPtr,
                        b"merging nodeset hit limit\n\x00" as *const u8 as *const libc::c_char,
                    )
                };
                return 0 as xmlNodeSetPtr;
            }
            unsafe {
                temp = xmlRealloc.expect("non-null function pointer")(
                    (*set1).nodeTab as *mut libc::c_void,
                    (((*set1).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
                ) as *mut xmlNodePtr
            };
            if temp.is_null() {
                unsafe {
                    xmlXPathErrMemory(
                        0 as xmlXPathContextPtr,
                        b"merging nodeset\n\x00" as *const u8 as *const libc::c_char,
                    )
                };
                return 0 as xmlNodeSetPtr;
            }
            unsafe {
                (*set1).nodeTab = temp;
                (*set1).nodeMax *= 2 as libc::c_int
            }
        }
        unsafe {
            let fresh61 = (*set1).nodeNr;
            (*set1).nodeNr = (*set1).nodeNr + 1;
            let ref mut fresh62 = *(*set1).nodeTab.offset(fresh61 as isize);
            *fresh62 = n2;
        }
        i += 1
    }
    unsafe { (*set2).nodeNr = 0 as libc::c_int };
    return set1;
}
/* *
 * xmlXPathNodeSetDel: * @cur: the initial node set
 * @val: an xmlNodePtr
 *
 * Removes an xmlNodePtr from an existing NodeSet
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetDel(mut cur: xmlNodeSetPtr, mut val: xmlNodePtr) {
    let mut i: libc::c_int = 0;
    if cur.is_null() {
        return;
    }
    if val.is_null() {
        return;
    }
    let safe_cur = unsafe { &mut *cur };
    let safe_val = unsafe { &mut *val };
    /*
     * find node in nodeTab
     */
    i = 0 as libc::c_int;
    while i < safe_cur.nodeNr {
        if unsafe { *(*cur).nodeTab.offset(i as isize) == val } {
            break;
        }
        i += 1
    }
    if i >= safe_cur.nodeNr {
        /* not found */
        match () {
            #[cfg(DEBUG)]
            _ => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"xmlXPathNodeSetDel: Node %s wasn't found in NodeList\n" as *const u8
                            as *const libc::c_char,
                        (*val).name as libc::c_uint,
                    )
                };
            }
            #[cfg(not(DEBUG))]
            _ => {}
        };
        return;
    }
    if unsafe {
        !(*(*cur).nodeTab.offset(i as isize)).is_null()
            && (**(*cur).nodeTab.offset(i as isize)).type_0 as libc::c_uint
                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    } {
        unsafe { xmlXPathNodeSetFreeNs(*(*cur).nodeTab.offset(i as isize) as xmlNsPtr) };
    }
    unsafe { (*cur).nodeNr -= 1 };
    while unsafe { i < (*cur).nodeNr } {
        unsafe {
            let ref mut fresh63 = *(*cur).nodeTab.offset(i as isize);
            *fresh63 = *(*cur).nodeTab.offset((i + 1 as libc::c_int) as isize);
        };
        i += 1
    }
    unsafe {
        let ref mut fresh64 = *(*cur).nodeTab.offset((*cur).nodeNr as isize);
        *fresh64 = 0 as xmlNodePtr;
    }
}
/* *
 * xmlXPathNodeSetRemove: * @cur: the initial node set
 * @val: the index to remove
 *
 * Removes an entry from an existing NodeSet list.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetRemove(mut cur: xmlNodeSetPtr, mut val: libc::c_int) {
    if cur.is_null() {
        return;
    }
    let safe_cur = unsafe { &mut *cur };
    if val >= safe_cur.nodeNr {
        return;
    }
    if unsafe {
        !(*(*cur).nodeTab.offset(val as isize)).is_null()
            && (**(*cur).nodeTab.offset(val as isize)).type_0 as libc::c_uint
                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    } {
        unsafe { xmlXPathNodeSetFreeNs(*(*cur).nodeTab.offset(val as isize) as xmlNsPtr) };
    }
    unsafe { (*cur).nodeNr -= 1 };
    while unsafe { val < (*cur).nodeNr } {
        unsafe {
            let ref mut fresh65 = *(*cur).nodeTab.offset(val as isize);
            *fresh65 = *(*cur).nodeTab.offset((val + 1 as libc::c_int) as isize);
        }
        val += 1
    }
    unsafe {
        let ref mut fresh66 = *(*cur).nodeTab.offset((*cur).nodeNr as isize);
        *fresh66 = 0 as xmlNodePtr;
    }
}
/* *
 * xmlXPathFreeNodeSet: * @obj: the xmlNodeSetPtr to free
 *
 * Free the NodeSet compound (not the actual nodes !).
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathFreeNodeSet(mut obj: xmlNodeSetPtr) {
    if obj.is_null() {
        return;
    }
    let safe_obj = unsafe { &mut *obj };
    if !safe_obj.nodeTab.is_null() {
        let mut i: libc::c_int = 0;
        /* @@ with_ns to check whether namespace nodes should be looked at @@ */
        i = 0 as libc::c_int;
        while i < safe_obj.nodeNr {
            if unsafe {
                !(*(*obj).nodeTab.offset(i as isize)).is_null()
                    && (**(*obj).nodeTab.offset(i as isize)).type_0 as libc::c_uint
                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            } {
                unsafe { xmlXPathNodeSetFreeNs(*(*obj).nodeTab.offset(i as isize) as xmlNsPtr) };
            }
            i += 1
        }
        unsafe { xmlFree.expect("non-null function pointer")((*obj).nodeTab as *mut libc::c_void) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void) };
}
/* *
 * xmlXPathNodeSetClearFromPos: * @set: the node set to be cleared
 * @pos: the start position to clear from
 *
 * Clears the list from temporary XPath objects (e.g. namespace nodes
 * are feed) starting with the entry at @pos, but does *not* free the list
 * itself. Sets the length of the list to @pos.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathNodeSetClearFromPos(
    mut set: xmlNodeSetPtr,
    mut pos: libc::c_int,
    mut hasNsNodes: libc::c_int,
) {
    let safe_set = unsafe { &mut *set };
    if set.is_null() || pos >= safe_set.nodeNr {
        return;
    } else {
        if hasNsNodes != 0 {
            let mut i: libc::c_int = 0;
            let mut node: xmlNodePtr = 0 as *mut xmlNode;
            i = pos;
            while i < safe_set.nodeNr {
                node = unsafe { *(*set).nodeTab.offset(i as isize) };
                if unsafe {
                    !node.is_null()
                        && (*node).type_0 as libc::c_uint
                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                } {
                    unsafe { xmlXPathNodeSetFreeNs(node as xmlNsPtr) };
                }
                i += 1
            }
        }
    }
    unsafe { (*set).nodeNr = pos };
}
/* *
 * xmlXPathNodeSetClear: * @set: the node set to clear
 *
 * Clears the list from all temporary XPath objects (e.g. namespace nodes
 * are feed), but does *not* free the list itself. Sets the length of the
 * list to 0.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathNodeSetClear(mut set: xmlNodeSetPtr, mut hasNsNodes: libc::c_int) {
    unsafe { xmlXPathNodeSetClearFromPos(set, 0 as libc::c_int, hasNsNodes) };
}
/* *
 * xmlXPathNodeSetKeepLast: * @set: the node set to be cleared
 *
 * Move the last node to the first position and clear temporary XPath objects
 * (e.g. namespace nodes) from all other nodes. Sets the length of the list
 * to 1.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathNodeSetKeepLast(mut set: xmlNodeSetPtr) {
    let mut i: libc::c_int = 0;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let safe_set = unsafe { &mut *set };
    if set.is_null() || safe_set.nodeNr <= 1 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < safe_set.nodeNr - 1 as libc::c_int {
        node = unsafe { *(*set).nodeTab.offset(i as isize) };
        if !node.is_null()
            && unsafe {
                (*node).type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            }
        {
            unsafe { xmlXPathNodeSetFreeNs(node as xmlNsPtr) };
        }
        i += 1
    }
    unsafe {
        let ref mut fresh67 = *(*set).nodeTab.offset(0 as libc::c_int as isize);
        *fresh67 = *(*set)
            .nodeTab
            .offset(((*set).nodeNr - 1 as libc::c_int) as isize);
        (*set).nodeNr = 1 as libc::c_int;
    }
}
/* ***********************************************************************
 *									*
 *			Forward declarations				*
 *									*
 ************************************************************************/
/* *
 * xmlXPathFreeValueTree: * @obj: the xmlNodeSetPtr to free
 *
 * Free the NodeSet compound and the actual tree, this is different
 * from xmlXPathFreeNodeSet() */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathFreeValueTree(mut obj: xmlNodeSetPtr) {
    let mut i: libc::c_int = 0;
    if obj.is_null() {
        return;
    }
    let safe_obj = unsafe { &mut *obj };
    if !safe_obj.nodeTab.is_null() {
        i = 0 as libc::c_int;
        while i < safe_obj.nodeNr {
            if unsafe { !(*(*obj).nodeTab.offset(i as isize)).is_null() } {
                if unsafe {
                    (**(*obj).nodeTab.offset(i as isize)).type_0 as libc::c_uint
                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                } {
                    unsafe {
                        xmlXPathNodeSetFreeNs(*(*obj).nodeTab.offset(i as isize) as xmlNsPtr)
                    };
                } else {
                    unsafe { xmlFreeNodeList(*(*obj).nodeTab.offset(i as isize)) };
                }
            }
            i += 1
        }
        unsafe { xmlFree.expect("non-null function pointer")((*obj).nodeTab as *mut libc::c_void) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void) };
}

/* *
 * xmlGenericErrorContextNodeSet: * @output: a FILE * for the output
 * @obj: the xmlNodeSetPtr to display
 *
 * Quick display of a NodeSet
 */
#[cfg(DEBUG_OR_DEBUG_STEP)]
#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlGenericErrorContextNodeSet(mut output: *mut FILE, mut obj: xmlNodeSetPtr) {
    let mut i: libc::c_int = 0;
    let safe_obj = unsafe { &mut *obj };
    if output.is_null() {
        output = unsafe { *__xmlGenericErrorContext() as *mut FILE }
    }
    if obj.is_null() {
        unsafe {
            fprintf(
                output,
                b"NodeSet == NULL !\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return;
    }
    if safe_obj.nodeNr == 0 as libc::c_int {
        unsafe {
            fprintf(
                output,
                b"NodeSet is empty\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return;
    }
    if safe_obj.nodeTab.is_null() {
        unsafe {
            fprintf(
                output,
                b" nodeTab == NULL !\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return;
    }
    i = 0 as libc::c_int;
    while i < safe_obj.nodeNr {
        if unsafe { (*(*obj).nodeTab.offset(i as isize)).is_null() } {
            unsafe { fprintf(output, b" NULL !\n\x00" as *const u8 as *const libc::c_char) };
            return;
        }
        if unsafe {
            (**(*obj).nodeTab.offset(i as isize)).type_0 as libc::c_uint
                == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
                || (**(*obj).nodeTab.offset(i as isize)).type_0 as libc::c_uint
                    == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        } {
            unsafe { fprintf(output, b" /\x00" as *const u8 as *const libc::c_char) };
        } else if unsafe { (**(*obj).nodeTab.offset(i as isize)).name.is_null() } {
            unsafe { fprintf(output, b" noname!\x00" as *const u8 as *const libc::c_char) };
        } else {
            unsafe {
                fprintf(
                    output,
                    b" %s\x00" as *const u8 as *const libc::c_char,
                    (**(*obj).nodeTab.offset(i as isize)).name,
                )
            };
        }
        i += 1
    }
    unsafe { fprintf(output, b"\n\x00" as *const u8 as *const libc::c_char) };
}

/* *
 * xmlXPathNewNodeSet: * @val: the NodePtr value
 *
 * Create a new xmlXPathObjectPtr of type NodeSet and initialize
 * it with the single Node @val
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNewNodeSet(mut val: xmlNodePtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong
        ) as xmlXPathObjectPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating nodeset\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
        );
        (*ret).type_0 = XPATH_NODESET;
        (*ret).boolval = 0 as libc::c_int;
        /* TODO: Check memory error. */
        (*ret).nodesetval = xmlXPathNodeSetCreate(val);
    }

    /* @@ with_ns to check whether namespace nodes should be looked at @@ */
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe { xmlXPathDebugObjUsageRequested(0 as xmlXPathContextPtr, XPATH_NODESET) };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };

    return ret;
}
/* *
 * xmlXPathNewValueTree: * @val: the NodePtr value
 *
 * Create a new xmlXPathObjectPtr of type Value Tree (XSLT) and initialize
 * it with the tree root @val
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNewValueTree(mut val: xmlNodePtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong
        ) as xmlXPathObjectPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating result value tree\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
        );
        (*ret).type_0 = XPATH_XSLT_TREE;
        (*ret).boolval = 1 as libc::c_int;
        (*ret).user = val as *mut libc::c_void;
        (*ret).nodesetval = xmlXPathNodeSetCreate(val);
    }

    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe { xmlXPathDebugObjUsageRequested(0 as xmlXPathContextPtr, XPATH_XSLT_TREE) };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    return ret;
}
/* *
 * xmlXPathNewNodeSetList: * @val: an existing NodeSet
 *
 * Create a new xmlXPathObjectPtr of type NodeSet and initialize
 * it with the Nodeset @val
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNewNodeSetList(mut val: xmlNodeSetPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut i: libc::c_int = 0;
    let safe_val = unsafe { &mut *val };
    if val.is_null() {
        ret = 0 as xmlXPathObjectPtr
    } else if safe_val.nodeTab.is_null() {
        ret = unsafe { xmlXPathNewNodeSet(0 as xmlNodePtr) }
    } else {
        ret = unsafe { xmlXPathNewNodeSet(*(*val).nodeTab.offset(0 as libc::c_int as isize)) };
        if !ret.is_null() {
            i = 1 as libc::c_int;
            while i < safe_val.nodeNr {
                /* TODO: Propagate memory error. */
                if unsafe {
                    xmlXPathNodeSetAddUnique((*ret).nodesetval, *(*val).nodeTab.offset(i as isize))
                        < 0 as libc::c_int
                } {
                    break;
                }
                i += 1
            }
        }
    }
    return ret;
}
/* *
 * xmlXPathWrapNodeSet: * @val: the NodePtr value
 *
 * Wrap the Nodeset @val in a new xmlXPathObjectPtr
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathWrapNodeSet(mut val: xmlNodeSetPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong
        ) as xmlXPathObjectPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating node set object\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
        );
        (*ret).type_0 = XPATH_NODESET;
        (*ret).nodesetval = val;
    };
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe { xmlXPathDebugObjUsageRequested(ctxt, XPATH_NODESET) };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    return ret;
}
/* *
 * xmlXPathFreeNodeSetList: * @obj: an existing NodeSetList object
 *
 * Free up the xmlXPathObjectPtr @obj but don't deallocate the objects in
 * the list contrary to xmlXPathFreeObject().
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathFreeNodeSetList(mut obj: xmlXPathObjectPtr) {
    if obj.is_null() {
        return;
    }
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe {
                xmlXPathDebugObjUsageRequested(
                    0 as xmlXPathContextPtr,
                    (*obj).type_0 as libc::c_uint,
                )
            };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    unsafe { xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void) };
}
/* *
 * xmlXPathDifference: * @nodes1: a node-set
 * @nodes2: a node-set
 *
 * Implements the EXSLT - Sets difference() function: *    node-set set:difference (node-set, node-set) *
 * Returns the difference between the two node sets, or nodes1 if
 *         nodes2 is empty
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathDifference(mut nodes1: xmlNodeSetPtr, mut nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut i: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 as libc::c_int || safe_nodes2.nodeTab.is_null() {
        return nodes1;
    }
    /* TODO: Check memory error. */
    ret = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
    if nodes1.is_null() || safe_nodes1.nodeNr == 0 as libc::c_int || safe_nodes1.nodeTab.is_null() {
        return ret;
    }
    l1 = if !nodes1.is_null() {
        safe_nodes1.nodeNr
    } else {
        0 as libc::c_int
    };
    i = 0 as libc::c_int;
    while i < l1 {
        cur = if !nodes1.is_null() && i >= 0 as libc::c_int && i < safe_nodes1.nodeNr {
            unsafe { *(*nodes1).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        if unsafe { xmlXPathNodeSetContains(nodes2, cur) == 0 } {
            /* TODO: Propagate memory error. */
            if unsafe { xmlXPathNodeSetAddUnique(ret, cur) < 0 as libc::c_int } {
                break;
            }
        }
        i += 1
    }
    return ret;
}
/* *
 * xmlXPathIntersection: * @nodes1: a node-set
 * @nodes2: a node-set
 *
 * Implements the EXSLT - Sets intersection() function: *    node-set set:intersection (node-set, node-set) *
 * Returns a node set comprising the nodes that are within both the
 *         node sets passed as arguments
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathIntersection(mut nodes1: xmlNodeSetPtr, mut nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
    let mut i: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if ret.is_null() {
        return ret;
    }
    if nodes1.is_null() || safe_nodes1.nodeNr == 0 as libc::c_int || safe_nodes1.nodeTab.is_null() {
        return ret;
    }
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 as libc::c_int || safe_nodes2.nodeTab.is_null() {
        return ret;
    }
    l1 = if !nodes1.is_null() {
        safe_nodes1.nodeNr
    } else {
        0 as libc::c_int
    };
    i = 0 as libc::c_int;
    while i < l1 {
        cur = if !nodes1.is_null() && i >= 0 as libc::c_int && i < safe_nodes1.nodeNr {
            unsafe { *(*nodes1).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        if unsafe { xmlXPathNodeSetContains(nodes2, cur) } != 0 {
            /* TODO: Propagate memory error. */
            if unsafe { xmlXPathNodeSetAddUnique(ret, cur) } < 0 as libc::c_int {
                break;
            }
        }
        i += 1
    }
    return ret;
}
/* *
 * xmlXPathDistinctSorted: * @nodes: a node-set, sorted by document order
 *
 * Implements the EXSLT - Sets distinct() function: *    node-set set:distinct (node-set) *
 * Returns a subset of the nodes contained in @nodes, or @nodes if
 *         it is empty
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathDistinctSorted(mut nodes: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut hash: xmlHashTablePtr = 0 as *mut xmlHashTable;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut strval: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let safe_nodes = unsafe { &mut *nodes };
    if nodes.is_null() || safe_nodes.nodeNr == 0 as libc::c_int || safe_nodes.nodeTab.is_null() {
        return nodes;
    }
    ret = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
    if ret.is_null() {
        return ret;
    }
    l = if !nodes.is_null() {
        safe_nodes.nodeNr
    } else {
        0 as libc::c_int
    };
    hash = unsafe { xmlHashCreate(l) };
    i = 0 as libc::c_int;
    while i < l {
        cur = if !nodes.is_null() && i >= 0 as libc::c_int && i < safe_nodes.nodeNr {
            unsafe { *(*nodes).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        strval = unsafe { xmlXPathCastNodeToString(cur) };
        if unsafe { xmlHashLookup(hash, strval).is_null() } {
            unsafe { xmlHashAddEntry(hash, strval, strval as *mut libc::c_void) };
            /* TODO: Propagate memory error. */
            if unsafe { xmlXPathNodeSetAddUnique(ret, cur) } < 0 as libc::c_int {
                break;
            }
        } else {
            unsafe { xmlFree.expect("non-null function pointer")(strval as *mut libc::c_void) };
        }
        i += 1
    }
    unsafe {
        xmlHashFree(
            hash,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: *const xmlChar) -> (),
            ),
        )
    };
    return ret;
}
/* *
 * xmlXPathDistinct: * @nodes: a node-set
 *
 * Implements the EXSLT - Sets distinct() function: *    node-set set:distinct (node-set) * @nodes is sorted by document order, then #exslSetsDistinctSorted
 * is called with the sorted node-set
 *
 * Returns a subset of the nodes contained in @nodes, or @nodes if
 *         it is empty
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathDistinct(mut nodes: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let safe_nodes = unsafe { &mut *nodes };
    if nodes.is_null() || safe_nodes.nodeNr == 0 as libc::c_int || safe_nodes.nodeTab.is_null() {
        return nodes;
    }
    unsafe { xmlXPathNodeSetSort(nodes) };
    return unsafe { xmlXPathDistinctSorted(nodes) };
}
/* *
 * xmlXPathHasSameNodes: * @nodes1: a node-set
 * @nodes2: a node-set
 *
 * Implements the EXSLT - Sets has-same-nodes function: *    boolean set:has-same-node(node-set, node-set) *
 * Returns true (1) if @nodes1 shares any node with @nodes2, false (0) *         otherwise
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathHasSameNodes(mut nodes1: xmlNodeSetPtr, mut nodes2: xmlNodeSetPtr) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes1.is_null()
        || safe_nodes1.nodeNr == 0 as libc::c_int
        || safe_nodes1.nodeTab.is_null()
        || (nodes2.is_null()
            || safe_nodes2.nodeNr == 0 as libc::c_int
            || safe_nodes2.nodeTab.is_null())
    {
        return 0 as libc::c_int;
    }
    l = if !nodes1.is_null() {
        safe_nodes1.nodeNr
    } else {
        0 as libc::c_int
    };
    i = 0 as libc::c_int;
    while i < l {
        cur = if !nodes1.is_null() && i >= 0 as libc::c_int && i < safe_nodes1.nodeNr {
            unsafe { *(*nodes1).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        if unsafe { xmlXPathNodeSetContains(nodes2, cur) } != 0 {
            return 1 as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathNodeLeadingSorted: * @nodes: a node-set, sorted by document order
 * @node: a node
 *
 * Implements the EXSLT - Sets leading() function: *    node-set set:leading (node-set, node-set) *
 * Returns the nodes in @nodes that precede @node in document order, *         @nodes if @node is NULL or an empty node-set if @nodes
 *         doesn't contain @node
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeLeadingSorted(mut nodes: xmlNodeSetPtr, mut node: xmlNodePtr) -> xmlNodeSetPtr {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if node.is_null() {
        return nodes;
    }
    let safe_nodes = unsafe { &mut *nodes };
    ret = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
    if ret.is_null() {
        return ret;
    }
    if nodes.is_null()
        || safe_nodes.nodeNr == 0 as libc::c_int
        || safe_nodes.nodeTab.is_null()
        || unsafe { xmlXPathNodeSetContains(nodes, node) } == 0
    {
        return ret;
    }
    l = if !nodes.is_null() {
        safe_nodes.nodeNr
    } else {
        0 as libc::c_int
    };
    i = 0 as libc::c_int;
    while i < l {
        cur = if !nodes.is_null() && i >= 0 as libc::c_int && i < safe_nodes.nodeNr {
            unsafe { *(*nodes).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        if cur == node {
            break;
        }
        /* TODO: Propagate memory error. */
        if unsafe { xmlXPathNodeSetAddUnique(ret, cur) } < 0 as libc::c_int {
            break;
        }
        i += 1
    }
    return ret;
}
/* *
 * xmlXPathNodeLeading: * @nodes: a node-set
 * @node: a node
 *
 * Implements the EXSLT - Sets leading() function: *    node-set set:leading (node-set, node-set) * @nodes is sorted by document order, then #exslSetsNodeLeadingSorted
 * is called.
 *
 * Returns the nodes in @nodes that precede @node in document order, *         @nodes if @node is NULL or an empty node-set if @nodes
 *         doesn't contain @node
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeLeading(mut nodes: xmlNodeSetPtr, mut node: xmlNodePtr) -> xmlNodeSetPtr {
    unsafe { xmlXPathNodeSetSort(nodes) };
    return unsafe { xmlXPathNodeLeadingSorted(nodes, node) };
}
/* *
 * xmlXPathLeadingSorted: * @nodes1: a node-set, sorted by document order
 * @nodes2: a node-set, sorted by document order
 *
 * Implements the EXSLT - Sets leading() function: *    node-set set:leading (node-set, node-set) *
 * Returns the nodes in @nodes1 that precede the first node in @nodes2
 *         in document order, @nodes1 if @nodes2 is NULL or empty or
 *         an empty node-set if @nodes1 doesn't contain @nodes2
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathLeadingSorted(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 as libc::c_int || safe_nodes2.nodeTab.is_null() {
        return nodes1;
    }
    return unsafe {
        xmlXPathNodeLeadingSorted(
            nodes1,
            if !nodes2.is_null()
                && 1 as libc::c_int >= 0 as libc::c_int
                && (1 as libc::c_int) < (*nodes2).nodeNr
            {
                *(*nodes2).nodeTab.offset(1 as libc::c_int as isize)
            } else {
                0 as xmlNodePtr
            },
        )
    };
}
/* *
 * xmlXPathLeading: * @nodes1: a node-set
 * @nodes2: a node-set
 *
 * Implements the EXSLT - Sets leading() function: *    node-set set:leading (node-set, node-set) * @nodes1 and @nodes2 are sorted by document order, then
 * #exslSetsLeadingSorted is called.
 *
 * Returns the nodes in @nodes1 that precede the first node in @nodes2
 *         in document order, @nodes1 if @nodes2 is NULL or empty or
 *         an empty node-set if @nodes1 doesn't contain @nodes2
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathLeading(mut nodes1: xmlNodeSetPtr, mut nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 as libc::c_int || safe_nodes2.nodeTab.is_null() {
        return nodes1;
    }
    if nodes1.is_null() || safe_nodes1.nodeNr == 0 as libc::c_int || safe_nodes1.nodeTab.is_null() {
        return unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
    }
    unsafe { xmlXPathNodeSetSort(nodes1) };
    unsafe { xmlXPathNodeSetSort(nodes2) };
    return unsafe {
        xmlXPathNodeLeadingSorted(
            nodes1,
            if !nodes2.is_null()
                && 1 as libc::c_int >= 0 as libc::c_int
                && (1 as libc::c_int) < (*nodes2).nodeNr
            {
                *(*nodes2).nodeTab.offset(1 as libc::c_int as isize)
            } else {
                0 as xmlNodePtr
            },
        )
    };
}
/* *
 * xmlXPathNodeTrailingSorted: * @nodes: a node-set, sorted by document order
 * @node: a node
 *
 * Implements the EXSLT - Sets trailing() function: *    node-set set:trailing (node-set, node-set) *
 * Returns the nodes in @nodes that follow @node in document order, *         @nodes if @node is NULL or an empty node-set if @nodes
 *         doesn't contain @node
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeTrailingSorted(mut nodes: xmlNodeSetPtr, mut node: xmlNodePtr) -> xmlNodeSetPtr {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if node.is_null() {
        return nodes;
    }
    let safe_nodes = unsafe { &mut *nodes };
    ret = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
    if ret.is_null() {
        return ret;
    }
    if nodes.is_null()
        || safe_nodes.nodeNr == 0 as libc::c_int
        || safe_nodes.nodeTab.is_null()
        || unsafe { xmlXPathNodeSetContains(nodes, node) } == 0
    {
        return ret;
    }
    l = if !nodes.is_null() {
        safe_nodes.nodeNr
    } else {
        0 as libc::c_int
    };
    i = l - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        cur = if !nodes.is_null() && i >= 0 as libc::c_int && i < safe_nodes.nodeNr {
            unsafe { *(*nodes).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        if cur == node {
            break;
        }
        /* TODO: Propagate memory error. */
        if unsafe { xmlXPathNodeSetAddUnique(ret, cur) } < 0 as libc::c_int {
            break; /* bug 413451 */
        }
        i -= 1
    }
    unsafe { xmlXPathNodeSetSort(ret) };
    return ret;
}
/* *
 * xmlXPathNodeTrailing: * @nodes: a node-set
 * @node: a node
 *
 * Implements the EXSLT - Sets trailing() function: *    node-set set:trailing (node-set, node-set) * @nodes is sorted by document order, then #xmlXPathNodeTrailingSorted
 * is called.
 *
 * Returns the nodes in @nodes that follow @node in document order, *         @nodes if @node is NULL or an empty node-set if @nodes
 *         doesn't contain @node
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeTrailing(mut nodes: xmlNodeSetPtr, mut node: xmlNodePtr) -> xmlNodeSetPtr {
    unsafe { xmlXPathNodeSetSort(nodes) };
    return unsafe { xmlXPathNodeTrailingSorted(nodes, node) };
}
/* *
 * xmlXPathTrailingSorted: * @nodes1: a node-set, sorted by document order
 * @nodes2: a node-set, sorted by document order
 *
 * Implements the EXSLT - Sets trailing() function: *    node-set set:trailing (node-set, node-set) *
 * Returns the nodes in @nodes1 that follow the first node in @nodes2
 *         in document order, @nodes1 if @nodes2 is NULL or empty or
 *         an empty node-set if @nodes1 doesn't contain @nodes2
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathTrailingSorted(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 as libc::c_int || safe_nodes2.nodeTab.is_null() {
        return nodes1;
    }
    return unsafe {
        xmlXPathNodeTrailingSorted(
            nodes1,
            if !nodes2.is_null()
                && 0 as libc::c_int >= 0 as libc::c_int
                && (0 as libc::c_int) < (*nodes2).nodeNr
            {
                *(*nodes2).nodeTab.offset(0 as libc::c_int as isize)
            } else {
                0 as xmlNodePtr
            },
        )
    };
}
/* *
 * xmlXPathTrailing: * @nodes1: a node-set
 * @nodes2: a node-set
 *
 * Implements the EXSLT - Sets trailing() function: *    node-set set:trailing (node-set, node-set) * @nodes1 and @nodes2 are sorted by document order, then
 * #xmlXPathTrailingSorted is called.
 *
 * Returns the nodes in @nodes1 that follow the first node in @nodes2
 *         in document order, @nodes1 if @nodes2 is NULL or empty or
 *         an empty node-set if @nodes1 doesn't contain @nodes2
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathTrailing(mut nodes1: xmlNodeSetPtr, mut nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 as libc::c_int || safe_nodes2.nodeTab.is_null() {
        return nodes1;
    }
    if nodes1.is_null() || safe_nodes1.nodeNr == 0 as libc::c_int || safe_nodes1.nodeTab.is_null() {
        return unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
    }
    unsafe { xmlXPathNodeSetSort(nodes1) };
    unsafe { xmlXPathNodeSetSort(nodes2) };
    return unsafe {
        xmlXPathNodeTrailingSorted(
            nodes1,
            if !nodes2.is_null()
                && 0 as libc::c_int >= 0 as libc::c_int
                && (0 as libc::c_int) < (*nodes2).nodeNr
            {
                *(*nodes2).nodeTab.offset(0 as libc::c_int as isize)
            } else {
                0 as xmlNodePtr
            },
        )
    };
}
/* ***********************************************************************
 *									*
 *		Routines to handle extra functions			*
 *									*
 ************************************************************************/
/* *
 * xmlXPathRegisterFunc: * @ctxt: the XPath context
 * @name: the function name
 * @f: the function implementation or NULL
 *
 * Register a new function. If @f is NULL it unregisters the function
 *
 * Returns 0 in case of success, -1 in case of error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisterFunc(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut f: xmlXPathFunction,
) -> libc::c_int {
    return unsafe { xmlXPathRegisterFuncNS(ctxt, name, 0 as *const xmlChar, f) };
}
/* *
 * xmlXPathRegisterFuncNS: * @ctxt: the XPath context
 * @name: the function name
 * @ns_uri: the function namespace URI
 * @f: the function implementation or NULL
 *
 * Register a new function. If @f is NULL it unregisters the function
 *
 * Returns 0 in case of success, -1 in case of error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisterFuncNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut f: xmlXPathFunction,
) -> libc::c_int {
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if name.is_null() {
        return -(1 as libc::c_int);
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.funcHash.is_null() {
        unsafe { (*ctxt).funcHash = xmlHashCreate(0 as libc::c_int) }
    }
    if unsafe { (*ctxt).funcHash.is_null() } {
        return -(1 as libc::c_int);
    }
    if f.is_none() {
        return unsafe { xmlHashRemoveEntry2((*ctxt).funcHash, name, ns_uri, None) };
    }
    return unsafe {
        xmlHashAddEntry2(
            (*ctxt).funcHash,
            name,
            ns_uri,
            ::std::mem::transmute::<xmlXPathFunction, *mut libc::c_void>(f),
        )
    };
}
/*
 * Function Lookup forwarding.
 */
/* *
 * xmlXPathRegisterFuncLookup: * @ctxt: the XPath context
 * @f: the lookup function
 * @funcCtxt: the lookup data
 *
 * Registers an external mechanism to do function lookup.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisterFuncLookup(
    mut ctxt: xmlXPathContextPtr,
    mut f: xmlXPathFuncLookupFunc,
    mut funcCtxt: *mut libc::c_void,
) {
    if ctxt.is_null() {
        return;
    }
    unsafe {
        (*ctxt).funcLookupFunc = f;
        (*ctxt).funcLookupData = funcCtxt;
    }
}
/* *
 * xmlXPathFunctionLookup: * @ctxt: the XPath context
 * @name: the function name
 *
 * Search in the Function array of the context for the given
 * function.
 *
 * Returns the xmlXPathFunction or NULL if not found
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathFunctionLookup(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
) -> xmlXPathFunction {
    if ctxt.is_null() {
        return None;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.funcLookupFunc.is_some() {
        let mut ret: xmlXPathFunction = None;
        let mut f: xmlXPathFuncLookupFunc = None;
        f = safe_ctxt.funcLookupFunc;
        ret = unsafe {
            f.expect("non-null function pointer")((*ctxt).funcLookupData, name, 0 as *const xmlChar)
        };
        if ret.is_some() {
            return ret;
        }
    }
    return unsafe { xmlXPathFunctionLookupNS(ctxt, name, 0 as *const xmlChar) };
}
/* *
 * xmlXPathFunctionLookupNS: * @ctxt: the XPath context
 * @name: the function name
 * @ns_uri: the function namespace URI
 *
 * Search in the Function array of the context for the given
 * function.
 *
 * Returns the xmlXPathFunction or NULL if not found
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathFunctionLookupNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathFunction {
    let mut ret: xmlXPathFunction = None;
    if ctxt.is_null() {
        return None;
    }
    if name.is_null() {
        return None;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.funcLookupFunc.is_some() {
        let mut f: xmlXPathFuncLookupFunc = None;
        f = safe_ctxt.funcLookupFunc;
        ret =
            unsafe { f.expect("non-null function pointer")((*ctxt).funcLookupData, name, ns_uri) };
        if ret.is_some() {
            return ret;
        }
    }
    if safe_ctxt.funcHash.is_null() {
        return None;
    }
    //XML_IGNORE_PEDANTIC_WARNINGS
    ret = unsafe {
        ::std::mem::transmute::<*mut libc::c_void, xmlXPathFunction>(xmlHashLookup2(
            (*ctxt).funcHash,
            name,
            ns_uri,
        ))
    };
    //XML_POP_WARNINGS
    return ret;
}

#[cfg(LIBXML_XPATH_ENABLED)]
#[cfg(DEBUG_STEP)]
pub extern "C" fn xmlXPathDebugDumpStepAxis(mut op: xmlXPathStepOpPtr, mut nbNodes: libc::c_int) {
    let safe_op = unsafe { &mut *op };
    unsafe {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"new step : \x00" as *const u8 as *const libc::c_char,
        )
    };
    match safe_op.value {
        1 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'ancestors\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        2 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'ancestors-or-self\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        3 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'attributes\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        4 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'child\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        5 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'descendant\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        6 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'descendant-or-self\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        7 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'following\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        8 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'following-siblings\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        9 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'namespace\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        10 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'parent\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        11 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'preceding\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        12 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'preceding-sibling\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        13 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'self\' \x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        _ => {}
    }
    unsafe {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b" context contains %d nodes\n\x00" as *const u8 as *const libc::c_char,
            nbNodes,
        )
    };
    match safe_op.value2 {
        0 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for none !!!\n\x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        1 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for type %d\n\x00" as *const u8 as *const libc::c_char,
                    (*op).value3,
                )
            };
        }
        2 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for PI !!!\n\x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        3 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for *\n\x00" as *const u8 as *const libc::c_char,
                )
            };
        }
        4 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for namespace %s\n\x00" as *const u8
                        as *const libc::c_char,
                    (*op).value5,
                )
            };
        }
        5 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for name %s\n\x00" as *const u8 as *const libc::c_char,
                    (*op).value5,
                )
            };
            if !safe_op.value4.is_null() {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"           with namespace %s\n\x00" as *const u8 as *const libc::c_char,
                        (*op).value4,
                    )
                };
            }
        }
        _ => {}
    }
    unsafe {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Testing : \x00" as *const u8 as *const libc::c_char,
        )
    };
}

/* *
 * xmlXPathRegisteredFuncsCleanup: * @ctxt: the XPath context
 *
 * Cleanup the XPath context data associated to registered functions
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisteredFuncsCleanup(mut ctxt: xmlXPathContextPtr) {
    if ctxt.is_null() {
        return;
    }
    unsafe {
        xmlHashFree((*ctxt).funcHash, None);
        (*ctxt).funcHash = 0 as xmlHashTablePtr;
    }
}
/* ***********************************************************************
 *									*
 *			Routines to handle Variables			*
 *									*
 ************************************************************************/
/* *
 * xmlXPathRegisterVariable: * @ctxt: the XPath context
 * @name: the variable name
 * @value: the variable value or NULL
 *
 * Register a new variable value. If @value is NULL it unregisters
 * the variable
 *
 * Returns 0 in case of success, -1 in case of error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisterVariable(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut value: xmlXPathObjectPtr,
) -> libc::c_int {
    return unsafe { xmlXPathRegisterVariableNS(ctxt, name, 0 as *const xmlChar, value) };
}
/* *
 * xmlXPathRegisterVariableNS: * @ctxt: the XPath context
 * @name: the variable name
 * @ns_uri: the variable namespace URI
 * @value: the variable value or NULL
 *
 * Register a new variable value. If @value is NULL it unregisters
 * the variable
 *
 * Returns 0 in case of success, -1 in case of error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisterVariableNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut value: xmlXPathObjectPtr,
) -> libc::c_int {
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if name.is_null() {
        return -(1 as libc::c_int);
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.varHash.is_null() {
        unsafe { (*ctxt).varHash = xmlHashCreate(0 as libc::c_int) }
    }
    if unsafe { (*ctxt).varHash.is_null() } {
        return -(1 as libc::c_int);
    }
    if value.is_null() {
        return unsafe {
            xmlHashRemoveEntry2(
                (*ctxt).varHash,
                name,
                ns_uri,
                Some(
                    xmlXPathFreeObjectEntry
                        as unsafe extern "C" fn(_: *mut libc::c_void, _: *const xmlChar) -> (),
                ),
            )
        };
    }
    return unsafe {
        xmlHashUpdateEntry2(
            (*ctxt).varHash,
            name,
            ns_uri,
            value as *mut libc::c_void,
            Some(
                xmlXPathFreeObjectEntry
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: *const xmlChar) -> (),
            ),
        )
    };
}
/* *
 * xmlXPathStackIsExternal: * @ctxt: an XPath parser context
 *
 * Checks if the current value on the XPath stack is an external
 * object.
 *
 * Returns true if the current object on the stack is an external
 * object.
 */
/* *
 * xmlXPathEmptyNodeSet: * @ns: a node-set
 *
 * Empties a node-set.
 */
/* *
 * CHECK_ERROR: *
 * Macro to return from the function if an XPath error was detected.
 */
/* *
 * CHECK_ERROR0: *
 * Macro to return 0 from the function if an XPath error was detected.
 */
/* *
 * XP_ERROR: * @X: the error code
 *
 * Macro to raise an XPath error and return.
 */
/* *
 * XP_ERROR0: * @X: the error code
 *
 * Macro to raise an XPath error and return 0.
 */
/* *
 * CHECK_TYPE: * @typeval: the XPath type
 *
 * Macro to check that the value on top of the XPath stack is of a given
 * type.
 */
/* *
 * CHECK_TYPE0: * @typeval: the XPath type
 *
 * Macro to check that the value on top of the XPath stack is of a given
 * type. Return(0) in case of failure
 */
/* *
 * CHECK_ARITY: * @x: the number of expected args
 *
 * Macro to check that the number of args passed to an XPath function matches.
 */
/* *
 * CAST_TO_STRING: *
 * Macro to try to cast the value on the top of the XPath stack to a string.
 */
/* *
 * CAST_TO_NUMBER: *
 * Macro to try to cast the value on the top of the XPath stack to a number.
 */
/* *
 * CAST_TO_BOOLEAN: *
 * Macro to try to cast the value on the top of the XPath stack to a boolean.
 */
/*
 * Variable Lookup forwarding.
 */
/* *
 * xmlXPathRegisterVariableLookup: * @ctxt: the XPath context
 * @f: the lookup function
 * @data: the lookup data
 *
 * register an external mechanism to do variable lookup
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisterVariableLookup(
    mut ctxt: xmlXPathContextPtr,
    mut f: xmlXPathVariableLookupFunc,
    mut data: *mut libc::c_void,
) {
    if ctxt.is_null() {
        return;
    }
    unsafe {
        (*ctxt).varLookupFunc = f;
        (*ctxt).varLookupData = data;
    }
}
/* *
 * xmlXPathVariableLookup: * @ctxt: the XPath context
 * @name: the variable name
 *
 * Search in the Variable array of the context for the given
 * variable value.
 *
 * Returns a copy of the value or NULL if not found
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathVariableLookup(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
) -> xmlXPathObjectPtr {
    if ctxt.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.varLookupFunc.is_some() {
        let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
        ret = unsafe {
            (*ctxt).varLookupFunc.expect("non-null function pointer")(
                (*ctxt).varLookupData,
                name,
                0 as *const xmlChar,
            )
        };
        return ret;
    }
    return unsafe { xmlXPathVariableLookupNS(ctxt, name, 0 as *const xmlChar) };
}
/* *
 * xmlXPathVariableLookupNS: * @ctxt: the XPath context
 * @name: the variable name
 * @ns_uri: the variable namespace URI
 *
 * Search in the Variable array of the context for the given
 * variable value.
 *
 * Returns the a copy of the value or NULL if not found
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathVariableLookupNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathObjectPtr {
    if ctxt.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.varLookupFunc.is_some() {
        let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
        ret = unsafe {
            (*ctxt).varLookupFunc.expect("non-null function pointer")(
                (*ctxt).varLookupData,
                name,
                ns_uri,
            )
        };
        if !ret.is_null() {
            return ret;
        }
    }
    if safe_ctxt.varHash.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    if name.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    return unsafe {
        xmlXPathCacheObjectCopy(
            ctxt,
            xmlHashLookup2((*ctxt).varHash, name, ns_uri) as xmlXPathObjectPtr,
        )
    };
}
/* *
 * xmlXPathRegisteredVariablesCleanup: * @ctxt: the XPath context
 *
 * Cleanup the XPath context data associated to registered variables
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisteredVariablesCleanup(mut ctxt: xmlXPathContextPtr) {
    if ctxt.is_null() {
        return;
    }
    unsafe {
        xmlHashFree(
            (*ctxt).varHash,
            Some(
                xmlXPathFreeObjectEntry
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: *const xmlChar) -> (),
            ),
        );
        (*ctxt).varHash = 0 as xmlHashTablePtr;
    }
}
/* *
 * Extending a context.
 */
/* *
 * xmlXPathRegisterNs: * @ctxt: the XPath context
 * @prefix: the namespace prefix cannot be NULL or empty string
 * @ns_uri: the namespace name
 *
 * Register a new namespace. If @ns_uri is NULL it unregisters
 * the namespace
 *
 * Returns 0 in case of success, -1 in case of error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisterNs(
    mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> libc::c_int {
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if prefix.is_null() {
        return -(1 as libc::c_int);
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if unsafe { *prefix.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int } {
        return -(1 as libc::c_int);
    }
    if safe_ctxt.nsHash.is_null() {
        unsafe { (*ctxt).nsHash = xmlHashCreate(10 as libc::c_int) }
    }
    if unsafe { (*ctxt).nsHash.is_null() } {
        return -(1 as libc::c_int);
    }
    if ns_uri.is_null() {
        return unsafe {
            xmlHashRemoveEntry(
                (*ctxt).nsHash,
                prefix,
                Some(
                    xmlHashDefaultDeallocator
                        as unsafe extern "C" fn(_: *mut libc::c_void, _: *const xmlChar) -> (),
                ),
            )
        };
    }
    return unsafe {
        xmlHashUpdateEntry(
            (*ctxt).nsHash,
            prefix,
            xmlStrdup(ns_uri) as *mut libc::c_void,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: *const xmlChar) -> (),
            ),
        )
    };
}
/* *
 * xmlXPathNsLookup: * @ctxt: the XPath context
 * @prefix: the namespace prefix value
 *
 * Search in the namespace declaration array of the context for the given
 * namespace name associated to the given prefix
 *
 * Returns the value or NULL if not found
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNsLookup(
    mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
) -> *const xmlChar {
    if ctxt.is_null() {
        return 0 as *const xmlChar;
    }
    if prefix.is_null() {
        return 0 as *const xmlChar;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    match () {
        #[cfg(XML_XML_NAMESPACE)]
        _ => {
            if unsafe {
                xmlStrEqual(
                    prefix,
                    b"xml\x00" as *const u8 as *const libc::c_char as *const xmlChar,
                ) != 0
            } {
                return unsafe {
                    b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as *const libc::c_char
                        as *const xmlChar
                };
            }
        }
        #[cfg(not(XML_XML_NAMESPACE))]
        _ => {}
    };

    if !safe_ctxt.namespaces.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < safe_ctxt.nsNr {
            if unsafe {
                !(*(*ctxt).namespaces.offset(i as isize)).is_null()
                    && xmlStrEqual((**(*ctxt).namespaces.offset(i as isize)).prefix, prefix) != 0
            } {
                return unsafe { (**(*ctxt).namespaces.offset(i as isize)).href };
            }
            i += 1
        }
    }
    return unsafe { xmlHashLookup((*ctxt).nsHash, prefix) as *const xmlChar };
}
/* *
 * xmlXPathRegisteredNsCleanup: * @ctxt: the XPath context
 *
 * Cleanup the XPath context data associated to registered variables
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisteredNsCleanup(mut ctxt: xmlXPathContextPtr) {
    if ctxt.is_null() {
        return;
    }
    unsafe {
        xmlHashFree(
            (*ctxt).nsHash,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: *const xmlChar) -> (),
            ),
        );
        (*ctxt).nsHash = 0 as xmlHashTablePtr;
    }
}
/* ***********************************************************************
 *									*
 *			Routines to handle Values			*
 *									*
 ************************************************************************/
/* Allocations are terrible, one needs to optimize all this !!! */
/* *
 * xmlXPathNewFloat: * @val: the double value
 *
 * Create a new xmlXPathObjectPtr of type double and of value @val
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNewFloat(mut val: libc::c_double) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong
        ) as xmlXPathObjectPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating float object\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
        );
        (*ret).type_0 = XPATH_NUMBER;
        (*ret).floatval = val;
    }
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe { xmlXPathDebugObjUsageRequested(0 as xmlXPathContextPtr, XPATH_NUMBER) };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };

    return ret;
}
/* *
 * xmlXPathNewBoolean: * @val: the boolean value
 *
 * Create a new xmlXPathObjectPtr of type boolean and of value @val
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNewBoolean(mut val: libc::c_int) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong
        ) as xmlXPathObjectPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating boolean object\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
        );
        (*ret).type_0 = XPATH_BOOLEAN;
        (*ret).boolval = (val != 0 as libc::c_int) as libc::c_int;
    }
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe { xmlXPathDebugObjUsageRequested(0 as xmlXPathContextPtr, XPATH_BOOLEAN) };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    return ret;
}
/* *
 * xmlXPathNewString: * @val: the xmlChar * value
 *
 * Create a new xmlXPathObjectPtr of type string and of value @val
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNewString(mut val: *const xmlChar) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong
        ) as xmlXPathObjectPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating string object\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
        );
        (*ret).type_0 = XPATH_STRING;
    }
    if !val.is_null() {
        unsafe { (*ret).stringval = xmlStrdup(val) }
    } else {
        unsafe {
            (*ret).stringval =
                xmlStrdup(b"\x00" as *const u8 as *const libc::c_char as *const xmlChar)
        }
    }

    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe { xmlXPathDebugObjUsageRequested(0 as xmlXPathContextPtr, XPATH_STRING) };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    return ret;
}
/* *
 * xmlXPathWrapString: * @val: the xmlChar * value
 *
 * Wraps the @val string into an XPath object.
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathWrapString(mut val: *mut xmlChar) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong
        ) as xmlXPathObjectPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating string object\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
        );
        (*ret).type_0 = XPATH_STRING;
        (*ret).stringval = val;
    }
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe { xmlXPathDebugObjUsageRequested(0 as xmlXPathContextPtr, XPATH_STRING) };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    return ret;
}
/* *
 * xmlXPathNewCString: * @val: the char * value
 *
 * Create a new xmlXPathObjectPtr of type string and of value @val
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNewCString(mut val: *const libc::c_char) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong
        ) as xmlXPathObjectPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating string object\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
        );
        (*ret).type_0 = XPATH_STRING;
        (*ret).stringval = xmlStrdup(val as *mut xmlChar);
    }
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe { xmlXPathDebugObjUsageRequested(0 as xmlXPathContextPtr, XPATH_STRING) };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    return ret;
}
/* *
 * xmlXPathWrapCString: * @val: the char * value
 *
 * Wraps a string into an XPath object.
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathWrapCString(mut val: *mut libc::c_char) -> xmlXPathObjectPtr {
    return unsafe { xmlXPathWrapString(val as *mut xmlChar) };
}
/* *
 * xmlXPathWrapExternal: * @val: the user data
 *
 * Wraps the @val data into an XPath object.
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathWrapExternal(mut val: *mut libc::c_void) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong
        ) as xmlXPathObjectPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating user object\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
        );
        (*ret).type_0 = XPATH_USERS;
        (*ret).user = val;
    }
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe { xmlXPathDebugObjUsageRequested(0 as xmlXPathContextPtr, XPATH_USERS) };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    return ret;
}
/* *
 * xmlXPathObjectCopy: * @val: the original object
 *
 * allocate a new copy of a given object
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathObjectCopy(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    let safe_val = unsafe { &mut *val };
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong
        ) as xmlXPathObjectPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"copying object\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memcpy(
            ret as *mut libc::c_void,
            val as *const libc::c_void,
            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
        )
    };
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe {
                xmlXPathDebugObjUsageRequested(
                    0 as xmlXPathContextPtr,
                    (*val).type_0 as libc::c_uint,
                )
            };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    match safe_val.type_0 as libc::c_uint {
        4 => unsafe { (*ret).stringval = xmlStrdup((*val).stringval) },
        9 | 1 => {
            unsafe {
                /* TODO: Check memory error. */
                (*ret).nodesetval = xmlXPathNodeSetMerge(0 as xmlNodeSetPtr, (*val).nodesetval);
                /* Do not deallocate the copied tree value */
                (*ret).boolval = 0 as libc::c_int
            }
        }
        7 => {
            match () {
                #[cfg(LIBXML_XPTR_ENABLED)]
                _ => {
                    let mut loc: xmlLocationSetPtr = safe_val.user as xmlLocationSetPtr;
                    unsafe {
                        (*ret).user = xmlXPtrLocationSetMerge(0 as xmlLocationSetPtr, loc)
                            as *mut libc::c_void
                    }
                }
                #[cfg(not(LIBXML_XPTR_ENABLED))]
                _ => {}
            };
        }
        8 => unsafe { (*ret).user = (*val).user },
        0 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlXPathObjectCopy: unsupported type %d\n\x00" as *const u8
                        as *const libc::c_char,
                    (*val).type_0 as libc::c_uint,
                )
            };
        }
        2 | 3 | 5 | 6 | _ => {}
    }
    return ret;
}
/* *
 * xmlXPathFreeObject: * @obj: the object to free
 *
 * Free up an xmlXPathObjectPtr object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathFreeObject(mut obj: xmlXPathObjectPtr) {
    if obj.is_null() {
        return;
    } /* TODO: Just for debugging. */
    let safe_obj = unsafe { &mut *obj };
    if safe_obj.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || safe_obj.type_0 as libc::c_uint == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        if safe_obj.boolval != 0 {
            safe_obj.type_0 = XPATH_XSLT_TREE;
            if !safe_obj.nodesetval.is_null() {
                unsafe { xmlXPathFreeValueTree((*obj).nodesetval) };
            }
        } else if !safe_obj.nodesetval.is_null() {
            unsafe { xmlXPathFreeNodeSet((*obj).nodesetval) };
        }
    } else if safe_obj.type_0 as libc::c_uint == XPATH_STRING as libc::c_int as libc::c_uint {
        if !safe_obj.stringval.is_null() {
            unsafe {
                xmlFree.expect("non-null function pointer")((*obj).stringval as *mut libc::c_void)
            };
        }
    }
    match () {
        #[cfg(LIBXML_XPTR_ENABLED)]
        _ => {
            if safe_obj.type_0 as libc::c_uint == XPATH_LOCATIONSET as libc::c_int as libc::c_uint {
                if !safe_obj.user.is_null() {
                    unsafe { xmlXPtrFreeLocationSet((*obj).user as xmlLocationSetPtr) };
                }
            }
        }
        #[cfg(not(LIBXML_XPTR_ENABLED))]
        _ => {}
    }
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe {
                xmlXPathDebugObjUsageReleased(
                    0 as xmlXPathContextPtr,
                    (*obj).type_0 as libc::c_uint,
                )
            };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    unsafe { xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void) };
}
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathFreeObjectEntry(mut obj: *mut libc::c_void, mut name: *const xmlChar) {
    unsafe { xmlXPathFreeObject(obj as xmlXPathObjectPtr) };
}
/* *
 * xmlXPathReleaseObject: * @obj: the xmlXPathObjectPtr to free or to cache
 *
 * Depending on the state of the cache this frees the given
 * XPath object or stores it in the cache.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathReleaseObject(mut ctxt: xmlXPathContextPtr, mut obj: xmlXPathObjectPtr) {
    let mut current_block: u64;
    if obj.is_null() {
        return;
    }
    let safe_obj = unsafe { &mut *obj };
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.cache.is_null() {
        unsafe { xmlXPathFreeObject(obj) };
    } else {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        match safe_obj.type_0 as libc::c_uint {
            1 | 9 => {
                if !safe_obj.nodesetval.is_null() {
                    if safe_obj.boolval != 0 {
                        /*
                        	* It looks like the @boolval is used for
                        	* evaluation if this an XSLT Result Tree Fragment.
                        	* TODO: Check if this assumption is correct.
                        	*/
                        unsafe {
                            (*obj).type_0 = XPATH_XSLT_TREE; /* just for debugging */
                            xmlXPathFreeValueTree((*obj).nodesetval);
                            (*obj).nodesetval = 0 as xmlNodeSetPtr;
                        }
                        current_block = 2290177392965769716;
                    } else if unsafe {
                        (*(*obj).nodesetval).nodeMax <= 40 as libc::c_int
                            && ((*cache).nodesetObjs.is_null()
                                || (*(*cache).nodesetObjs).number < (*cache).maxNodeset)
                    } {
                        if unsafe { (*cache).nodesetObjs.is_null() } {
                            unsafe {
                                (*cache).nodesetObjs = xmlPointerListCreate(10 as libc::c_int)
                            };
                            if unsafe { (*cache).nodesetObjs.is_null() } {
                                current_block = 4144075667789361827;
                            } else {
                                current_block = 10048703153582371463;
                            }
                        } else {
                            current_block = 10048703153582371463;
                        }
                        match current_block {
                            4144075667789361827 => {}
                            _ => {
                                if unsafe {
                                    xmlPointerListAddSize(
                                        (*cache).nodesetObjs,
                                        obj as *mut libc::c_void,
                                        0 as libc::c_int,
                                    ) == -(1 as libc::c_int)
                                } {
                                    current_block = 4144075667789361827;
                                } else {
                                    current_block = 10134510186868088070;
                                }
                            }
                        }
                    } else {
                        unsafe {
                            xmlXPathFreeNodeSet((*obj).nodesetval);
                            (*obj).nodesetval = 0 as xmlNodeSetPtr;
                        }
                        current_block = 2290177392965769716;
                    }
                } else {
                    current_block = 2290177392965769716;
                }
            }
            4 => {
                if !safe_obj.stringval.is_null() {
                    unsafe {
                        xmlFree.expect("non-null function pointer")(
                            (*obj).stringval as *mut libc::c_void,
                        )
                    };
                }
                if unsafe {
                    (*cache).stringObjs.is_null()
                        || (*(*cache).stringObjs).number < (*cache).maxString
                } {
                    if unsafe { (*cache).stringObjs.is_null() } {
                        unsafe { (*cache).stringObjs = xmlPointerListCreate(10 as libc::c_int) };
                        if unsafe { (*cache).stringObjs.is_null() } {
                            current_block = 4144075667789361827;
                        } else {
                            current_block = 16924917904204750491;
                        }
                    } else {
                        current_block = 16924917904204750491;
                    }
                    match current_block {
                        4144075667789361827 => {}
                        _ => {
                            if unsafe {
                                xmlPointerListAddSize(
                                    (*cache).stringObjs,
                                    obj as *mut libc::c_void,
                                    0 as libc::c_int,
                                ) == -(1 as libc::c_int)
                            } {
                                current_block = 4144075667789361827;
                            } else {
                                current_block = 10134510186868088070;
                            }
                        }
                    }
                } else {
                    current_block = 2290177392965769716;
                }
            }
            2 => {
                if unsafe {
                    (*cache).booleanObjs.is_null()
                        || (*(*cache).booleanObjs).number < (*cache).maxBoolean
                } {
                    if unsafe { (*cache).booleanObjs.is_null() } {
                        unsafe { (*cache).booleanObjs = xmlPointerListCreate(10 as libc::c_int) };
                        if unsafe { (*cache).booleanObjs.is_null() } {
                            current_block = 4144075667789361827;
                        } else {
                            current_block = 11048769245176032998;
                        }
                    } else {
                        current_block = 11048769245176032998;
                    }
                    match current_block {
                        4144075667789361827 => {}
                        _ => {
                            if unsafe {
                                xmlPointerListAddSize(
                                    (*cache).booleanObjs,
                                    obj as *mut libc::c_void,
                                    0 as libc::c_int,
                                ) == -(1 as libc::c_int)
                            } {
                                current_block = 4144075667789361827;
                            } else {
                                current_block = 10134510186868088070;
                            }
                        }
                    }
                } else {
                    current_block = 2290177392965769716;
                }
            }
            3 => {
                if unsafe {
                    (*cache).numberObjs.is_null()
                        || (*(*cache).numberObjs).number < (*cache).maxNumber
                } {
                    if unsafe { (*cache).numberObjs.is_null() } {
                        unsafe { (*cache).numberObjs = xmlPointerListCreate(10 as libc::c_int) };
                        if unsafe { (*cache).numberObjs.is_null() } {
                            current_block = 4144075667789361827;
                        } else {
                            current_block = 9441801433784995173;
                        }
                    } else {
                        current_block = 9441801433784995173;
                    }
                    match current_block {
                        4144075667789361827 => {}
                        _ => {
                            if unsafe {
                                xmlPointerListAddSize(
                                    (*cache).numberObjs,
                                    obj as *mut libc::c_void,
                                    0 as libc::c_int,
                                ) == -(1 as libc::c_int)
                            } {
                                current_block = 4144075667789361827;
                            } else {
                                current_block = 10134510186868088070;
                            }
                        }
                    }
                } else {
                    current_block = 2290177392965769716;
                }
            }
            7 => {
                if !safe_obj.user.is_null() {
                    unsafe { xmlXPtrFreeLocationSet((*obj).user as xmlLocationSetPtr) };
                }
                current_block = 4144075667789361827;
            }
            _ => {
                current_block = 4144075667789361827;
            }
        }
        match current_block {
            2290177392965769716 =>
            /*
            	* Fallback to adding to the misc-objects slot.
            	*/
            {
                if unsafe {
                    (*cache).miscObjs.is_null() || (*(*cache).miscObjs).number < (*cache).maxMisc
                } {
                    if unsafe { (*cache).miscObjs.is_null() } {
                        unsafe { (*cache).miscObjs = xmlPointerListCreate(10 as libc::c_int) };
                        if unsafe { (*cache).miscObjs.is_null() } {
                            current_block = 4144075667789361827;
                        } else {
                            current_block = 9241535491006583629;
                        }
                    } else {
                        current_block = 9241535491006583629;
                    }
                    match current_block {
                        4144075667789361827 => {}
                        _ => {
                            if unsafe {
                                xmlPointerListAddSize(
                                    (*cache).miscObjs,
                                    obj as *mut libc::c_void,
                                    0 as libc::c_int,
                                ) == -(1 as libc::c_int)
                            } {
                                current_block = 4144075667789361827;
                            } else {
                                current_block = 10134510186868088070;
                            }
                        }
                    }
                } else {
                    current_block = 4144075667789361827;
                }
            }
            _ => {}
        }
        match current_block {
            4144075667789361827 => {
                /*
                	* Cache is full; free the object.
                	*/
                if !safe_obj.nodesetval.is_null() {
                    unsafe { xmlXPathFreeNodeSet((*obj).nodesetval) };
                }
                unsafe { xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void) };
            }
            _ => {
                if !safe_obj.nodesetval.is_null() {
                    let mut tmpset: xmlNodeSetPtr = safe_obj.nodesetval;
                    /*
                     * TODO: Due to those nasty ns-nodes, we need to traverse
                     *  the list and free the ns-nodes.
                     * URGENT TODO: Check if it's actually slowing things down.
                     *  Maybe we shouldn't try to preserve the list.
                     */
                    if unsafe { (*tmpset).nodeNr > 1 as libc::c_int } {
                        let mut i: libc::c_int = 0;
                        let mut node: xmlNodePtr = 0 as *mut xmlNode;
                        i = 0 as libc::c_int;
                        while i < unsafe { (*tmpset).nodeNr } {
                            unsafe { node = *(*tmpset).nodeTab.offset(i as isize) };
                            if unsafe {
                                !node.is_null()
                                    && (*node).type_0 as libc::c_uint
                                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                            } {
                                unsafe { xmlXPathNodeSetFreeNs(node as xmlNsPtr) };
                            }
                            i += 1
                        }
                    } else if unsafe { (*tmpset).nodeNr == 1 as libc::c_int } {
                        if unsafe {
                            !(*(*tmpset).nodeTab.offset(0 as libc::c_int as isize)).is_null()
                                && (**(*tmpset).nodeTab.offset(0 as libc::c_int as isize)).type_0
                                    as libc::c_uint
                                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        } {
                            unsafe {
                                xmlXPathNodeSetFreeNs(
                                    *(*tmpset).nodeTab.offset(0 as libc::c_int as isize)
                                        as xmlNsPtr,
                                )
                            };
                        }
                    }
                    unsafe {
                        (*tmpset).nodeNr = 0 as libc::c_int;
                        memset(
                            obj as *mut libc::c_void,
                            0 as libc::c_int,
                            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
                        );
                        (*obj).nodesetval = tmpset
                    }
                } else {
                    unsafe {
                        memset(
                            obj as *mut libc::c_void,
                            0 as libc::c_int,
                            ::std::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
                        )
                    };
                }
                return;
            }
        }
    };
}
/* ***********************************************************************
 *									*
 *			Type Casting Routines				*
 *									*
 ************************************************************************/
/* *
 * xmlXPathCastBooleanToString: * @val: a boolean
 *
 * Converts a boolean to its string value.
 *
 * Returns a newly allocated string.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastBooleanToString(mut val: libc::c_int) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if val != 0 {
        ret =
            unsafe { xmlStrdup(b"true\x00" as *const u8 as *const libc::c_char as *const xmlChar) }
    } else {
        ret =
            unsafe { xmlStrdup(b"false\x00" as *const u8 as *const libc::c_char as *const xmlChar) }
    }
    return ret;
}
/* *
 * xmlXPathCastNumberToString: * @val: a number
 *
 * Converts a number to its string value.
 *
 * Returns a newly allocated string.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastNumberToString(mut val: libc::c_double) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    match unsafe { xmlXPathIsInf(val) } {
        1 => {
            ret = unsafe {
                xmlStrdup(b"Infinity\x00" as *const u8 as *const libc::c_char as *const xmlChar)
            }
        }
        -1 => {
            ret = unsafe {
                xmlStrdup(b"-Infinity\x00" as *const u8 as *const libc::c_char as *const xmlChar)
            }
        }
        _ => {
            if unsafe { xmlXPathIsNaN(val) } != 0 {
                ret = unsafe {
                    xmlStrdup(b"NaN\x00" as *const u8 as *const libc::c_char as *const xmlChar)
                }
            } else if val == 0 as libc::c_int as libc::c_double {
                /* Omit sign for negative zero. */
                ret = unsafe {
                    xmlStrdup(b"0\x00" as *const u8 as *const libc::c_char as *const xmlChar)
                }
            } else {
                /* could be improved */
                let mut buf: [libc::c_char; 100] = [0; 100];
                unsafe { xmlXPathFormatNumber(val, buf.as_mut_ptr(), 99 as libc::c_int) };
                buf[99 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                ret = unsafe { xmlStrdup(buf.as_mut_ptr() as *const xmlChar) }
            }
        }
    }
    return ret;
}
/* *
 * xmlXPathCastNodeToString: * @node: a node
 *
 * Converts a node to its string value.
 *
 * Returns a newly allocated string.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastNodeToString(mut node: xmlNodePtr) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    ret = unsafe { xmlNodeGetContent(node as *const xmlNode) };
    if ret.is_null() {
        ret = unsafe { xmlStrdup(b"\x00" as *const u8 as *const libc::c_char as *const xmlChar) }
    }
    return ret;
}
/* *
 * xmlXPathCastNodeSetToString: * @ns: a node-set
 *
 * Converts a node-set to its string value.
 *
 * Returns a newly allocated string.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastNodeSetToString(mut ns: xmlNodeSetPtr) -> *mut xmlChar {
    let safe_ns = unsafe { &mut *ns };
    if ns.is_null() || safe_ns.nodeNr == 0 as libc::c_int || safe_ns.nodeTab.is_null() {
        return unsafe { xmlStrdup(b"\x00" as *const u8 as *const libc::c_char as *const xmlChar) };
    }
    if safe_ns.nodeNr > 1 as libc::c_int {
        unsafe { xmlXPathNodeSetSort(ns) };
    }
    return unsafe { xmlXPathCastNodeToString(*(*ns).nodeTab.offset(0 as libc::c_int as isize)) };
}
/* *
 * xmlXPathCastToString: * @val: an XPath object
 *
 * Converts an existing object to its string() equivalent
 *
 * Returns the allocated string value of the object, NULL in case of error.
 *         It's up to the caller to free the string memory with xmlFree().
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastToString(mut val: xmlXPathObjectPtr) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if val.is_null() {
        return unsafe { xmlStrdup(b"\x00" as *const u8 as *const libc::c_char as *const xmlChar) };
    }
    let safe_val = unsafe { &mut *val };
    match safe_val.type_0 as libc::c_uint {
        0 => {
            match () {
                #[cfg(DEBUG_EXPR)]
                _ => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"String: undefined\n\x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                #[cfg(not(DEBUG_EXPR))]
                _ => {}
            };
            ret =
                unsafe { xmlStrdup(b"\x00" as *const u8 as *const libc::c_char as *const xmlChar) }
        }
        1 | 9 => ret = unsafe { xmlXPathCastNodeSetToString((*val).nodesetval) },
        4 => return unsafe { xmlStrdup((*val).stringval) },
        2 => ret = unsafe { xmlXPathCastBooleanToString((*val).boolval) },
        3 => ret = unsafe { xmlXPathCastNumberToString((*val).floatval) },
        8 | 5 | 6 | 7 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                    b"xpath.c\x00" as *const u8 as *const libc::c_char,
                    5775 as libc::c_int,
                )
            };
            ret =
                unsafe { xmlStrdup(b"\x00" as *const u8 as *const libc::c_char as *const xmlChar) }
        }
        _ => {}
    }
    return ret;
}
/* *
 * xmlXPathConvertString: * @val: an XPath object
 *
 * Converts an existing object to its string() equivalent
 *
 * Returns the new object, the old one is freed (or the operation
 *         is done directly on @val) */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathConvertString(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let mut res: *mut xmlChar = 0 as *mut xmlChar;
    if val.is_null() {
        return unsafe { xmlXPathNewCString(b"\x00" as *const u8 as *const libc::c_char) };
    }
    let safe_val = unsafe { &mut *val };
    match safe_val.type_0 as libc::c_uint {
        1 | 9 => res = unsafe { xmlXPathCastNodeSetToString((*val).nodesetval) },
        4 => return val,
        2 => res = unsafe { xmlXPathCastBooleanToString((*val).boolval) },
        3 => res = unsafe { xmlXPathCastNumberToString((*val).floatval) },
        8 | 5 | 6 | 7 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                    b"xpath.c\x00" as *const u8 as *const libc::c_char,
                    5820 as libc::c_int,
                )
            };
        }
        0 | _ => {
            match () {
                #[cfg(DEBUG_EXPR)]
                _ => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"STRING: undefined\n\x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                #[cfg(not(DEBUG_EXPR))]
                _ => {}
            };
        }
    }
    unsafe { xmlXPathFreeObject(val) };
    if res.is_null() {
        return unsafe { xmlXPathNewCString(b"\x00" as *const u8 as *const libc::c_char) };
    }
    return unsafe { xmlXPathWrapString(res) };
}
/* *
 * xmlXPathCastBooleanToNumber: * @val: a boolean
 *
 * Converts a boolean to its number value
 *
 * Returns the number value
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastBooleanToNumber(mut val: libc::c_int) -> libc::c_double {
    if val != 0 {
        return 1.0f64;
    }
    return 0.0f64;
}
/* *
 * xmlXPathCastStringToNumber: * @val: a string
 *
 * Converts a string to its number value
 *
 * Returns the number value
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastStringToNumber(mut val: *const xmlChar) -> libc::c_double {
    return unsafe { xmlXPathStringEvalNumber(val) };
}
/* *
 * xmlXPathCastNodeToNumber: * @node: a node
 *
 * Converts a node to its number value
 *
 * Returns the number value
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastNodeToNumber(mut node: xmlNodePtr) -> libc::c_double {
    let mut strval: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: libc::c_double = 0.;
    if node.is_null() {
        return unsafe { xmlXPathNAN };
    }
    strval = unsafe { xmlXPathCastNodeToString(node) };
    if strval.is_null() {
        return unsafe { xmlXPathNAN };
    }
    ret = unsafe { xmlXPathCastStringToNumber(strval) };
    unsafe { xmlFree.expect("non-null function pointer")(strval as *mut libc::c_void) };
    return ret;
}
/* *
 * xmlXPathCastNodeSetToNumber: * @ns: a node-set
 *
 * Converts a node-set to its number value
 *
 * Returns the number value
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastNodeSetToNumber(mut ns: xmlNodeSetPtr) -> libc::c_double {
    let mut str: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: libc::c_double = 0.;
    if ns.is_null() {
        return unsafe { xmlXPathNAN };
    }
    str = unsafe { xmlXPathCastNodeSetToString(ns) };
    ret = unsafe { xmlXPathCastStringToNumber(str) };
    unsafe { xmlFree.expect("non-null function pointer")(str as *mut libc::c_void) };
    return ret;
}
/* *
 * xmlXPathCastToNumber: * @val: an XPath object
 *
 * Converts an XPath object to its number value
 *
 * Returns the number value
 */

pub fn xmlXPathCastToNumber(mut val: xmlXPathObjectPtr) -> libc::c_double {
    let mut ret: libc::c_double = 0.0f64;
    if val.is_null() {
        return unsafe { xmlXPathNAN };
    }
    let safe_val = unsafe { &mut *val };
    match safe_val.type_0 as libc::c_uint {
        0 => {
            match () {
                #[cfg(DEBUG_EXPR)]
                _ => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"NUMBER: undefined\n\x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                #[cfg(not(DEBUG_EXPR))]
                _ => {}
            };
            ret = unsafe { xmlXPathNAN }
        }
        1 | 9 => ret = unsafe { xmlXPathCastNodeSetToNumber((*val).nodesetval) },
        4 => ret = unsafe { xmlXPathCastStringToNumber((*val).stringval) },
        3 => ret = safe_val.floatval,
        2 => ret = unsafe { xmlXPathCastBooleanToNumber((*val).boolval) },
        8 | 5 | 6 | 7 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                    b"xpath.c\x00" as *const u8 as *const libc::c_char,
                    5940 as libc::c_int,
                )
            };
            ret = unsafe { xmlXPathNAN }
        }
        _ => {}
    }
    return ret;
}
/* *
 * xmlXPathConvertNumber: * @val: an XPath object
 *
 * Converts an existing object to its number() equivalent
 *
 * Returns the new object, the old one is freed (or the operation
 *         is done directly on @val) */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathConvertNumber(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return xmlXPathNewFloat(0.0f64);
    }
    let safe_val = unsafe { &mut *val };
    if safe_val.type_0 as libc::c_uint == XPATH_NUMBER as libc::c_int as libc::c_uint {
        return val;
    }
    ret = unsafe { xmlXPathNewFloat(xmlXPathCastToNumber(val)) };
    unsafe { xmlXPathFreeObject(val) };
    return ret;
}
/* *
 * Conversion functions to basic types.
 */
/* *
 * xmlXPathCastNumberToBoolean: * @val: a number
 *
 * Converts a number to its boolean value
 *
 * Returns the boolean value
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastNumberToBoolean(mut val: libc::c_double) -> libc::c_int {
    if unsafe { xmlXPathIsNaN(val) } != 0 || val == 0.0f64 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* *
 * xmlXPathCastStringToBoolean: * @val: a string
 *
 * Converts a string to its boolean value
 *
 * Returns the boolean value
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastStringToBoolean(mut val: *const xmlChar) -> libc::c_int {
    if val.is_null() || unsafe { xmlStrlen(val) } == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* *
 * xmlXPathCastNodeSetToBoolean: * @ns: a node-set
 *
 * Converts a node-set to its boolean value
 *
 * Returns the boolean value
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastNodeSetToBoolean(mut ns: xmlNodeSetPtr) -> libc::c_int {
    let safe_ns = unsafe { &mut *ns };
    if ns.is_null() || safe_ns.nodeNr == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* *
 * xmlXPathCastToBoolean: * @val: an XPath object
 *
 * Converts an XPath object to its boolean value
 *
 * Returns the boolean value
 */

pub fn xmlXPathCastToBoolean(mut val: xmlXPathObjectPtr) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if val.is_null() {
        return 0 as libc::c_int;
    }
    let safe_val = unsafe { &mut *val };
    match safe_val.type_0 as libc::c_uint {
        0 => {
            match () {
                #[cfg(DEBUG_EXPR)]
                _ => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"BOOLEAN: undefined\n\x00" as *const u8 as *const libc::c_char,
                        )
                    };
                }
                #[cfg(not(DEBUG_EXPR))]
                _ => {}
            };
            ret = 0 as libc::c_int
        }
        1 | 9 => ret = unsafe { xmlXPathCastNodeSetToBoolean((*val).nodesetval) },
        4 => ret = unsafe { xmlXPathCastStringToBoolean((*val).stringval) },
        3 => ret = unsafe { xmlXPathCastNumberToBoolean((*val).floatval) },
        2 => ret = safe_val.boolval,
        8 | 5 | 6 | 7 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                    b"xpath.c\x00" as *const u8 as *const libc::c_char,
                    6052 as libc::c_int,
                )
            };
            ret = 0 as libc::c_int
        }
        _ => {}
    }
    return ret;
}
/* *
 * xmlXPathConvertBoolean: * @val: an XPath object
 *
 * Converts an existing object to its boolean() equivalent
 *
 * Returns the new object, the old one is freed (or the operation
 *         is done directly on @val) */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathConvertBoolean(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return unsafe { xmlXPathNewBoolean(0 as libc::c_int) };
    }
    let safe_val = unsafe { &mut *val };
    if safe_val.type_0 as libc::c_uint == XPATH_BOOLEAN as libc::c_int as libc::c_uint {
        return val;
    }
    ret = unsafe { xmlXPathNewBoolean(xmlXPathCastToBoolean(val)) };
    unsafe { xmlXPathFreeObject(val) };
    return ret;
}
/* *
 * Context handling.
 */
/* ***********************************************************************
 *									*
 *		Routines to handle XPath contexts			*
 *									*
 ************************************************************************/
/* *
 * xmlXPathNewContext: * @doc: the XML document
 *
 * Create a new xmlXPathContext
 *
 * Returns the xmlXPathContext just allocated. The caller will need to free it.
 */

pub fn xmlXPathNewContext(mut doc: xmlDocPtr) -> xmlXPathContextPtr {
    let mut ret: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathContext>() as libc::c_ulong
        ) as xmlXPathContextPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating context\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathContextPtr;
    }
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathContext>() as libc::c_ulong,
        );
        (*ret).doc = doc;
        (*ret).node = 0 as xmlNodePtr;
        (*ret).varHash = 0 as xmlHashTablePtr;
        (*ret).nb_types = 0 as libc::c_int;
        (*ret).max_types = 0 as libc::c_int;
        (*ret).types = 0 as xmlXPathTypePtr;
        (*ret).funcHash = xmlHashCreate(0 as libc::c_int);
        (*ret).nb_axis = 0 as libc::c_int;
        (*ret).max_axis = 0 as libc::c_int;
        (*ret).axis = 0 as xmlXPathAxisPtr;
        (*ret).nsHash = 0 as xmlHashTablePtr;
        (*ret).user = 0 as *mut libc::c_void;
        (*ret).contextSize = -(1 as libc::c_int);
        (*ret).proximityPosition = -(1 as libc::c_int);
    }
    match () {
        #[cfg(XP_DEFAULT_CACHE_ON)]
        _ => {
            if unsafe { xmlXPathContextSetCache(ret, 1, -1, 0) } == -1 {
                unsafe { xmlXPathFreeContext(ret) };
                return 0 as *mut xmlXPathContext;
            }
        }
        #[cfg(not(XP_DEFAULT_CACHE_ON))]
        _ => {}
    };
    unsafe { xmlXPathRegisterAllFunctions(ret) };
    return ret;
}
/* *
 * xmlXPathFreeContext: * @ctxt: the context to free
 *
 * Free up an xmlXPathContext
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathFreeContext(mut ctxt: xmlXPathContextPtr) {
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if !safe_ctxt.cache.is_null() {
        unsafe { xmlXPathFreeCache((*ctxt).cache as xmlXPathContextCachePtr) };
    }
    unsafe {
        xmlXPathRegisteredNsCleanup(ctxt);
        xmlXPathRegisteredFuncsCleanup(ctxt);
        xmlXPathRegisteredVariablesCleanup(ctxt);
        xmlResetError(&mut (*ctxt).lastError);
        xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
    }
}
/* *
 * Utilities to extend XPath.
 */
/* ***********************************************************************
 *									*
 *		Routines to handle XPath parser contexts		*
 *									*
 ************************************************************************/
/* *
 * xmlXPathNewParserContext: * @str: the XPath expression
 * @ctxt: the XPath context
 *
 * Create a new xmlXPathParserContext
 *
 * Returns the xmlXPathParserContext just allocated.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNewParserContext(
    mut str: *const xmlChar,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathParserContextPtr {
    let mut ret: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathParserContext>() as libc::c_ulong,
        ) as xmlXPathParserContextPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                ctxt,
                b"creating parser context\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathParserContextPtr;
    }
    let safe_ret = unsafe { &mut *ret };
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathParserContext>() as libc::c_ulong,
        );
        (*ret).base = str;
        (*ret).cur = (*ret).base;
        (*ret).context = ctxt;
        (*ret).comp = xmlXPathNewCompExpr();
    }
    if safe_ret.comp.is_null() {
        unsafe {
            xmlFree.expect("non-null function pointer")((*ret).valueTab as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
        }
        return 0 as xmlXPathParserContextPtr;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.dict.is_null() {
        unsafe {
            (*(*ret).comp).dict = (*ctxt).dict;
            xmlDictReference((*(*ret).comp).dict);
        }
    }
    return ret;
}
/* *
 * xmlXPathCompParserContext: * @comp: the XPath compiled expression
 * @ctxt: the XPath context
 *
 * Create a new xmlXPathParserContext when processing a compiled expression
 *
 * Returns the xmlXPathParserContext just allocated.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompParserContext(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathParserContextPtr {
    let mut ret: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathParserContext>() as libc::c_ulong,
        ) as xmlXPathParserContextPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                ctxt,
                b"creating evaluation context\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as xmlXPathParserContextPtr;
    }
    let safe_ret = unsafe { &mut *ret };
    unsafe {
        memset(
            ret as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlXPathParserContext>() as libc::c_ulong,
        );
        /* Allocate the value stack */
        (*ret).valueTab = xmlMalloc.expect("non-null function pointer")(
            (10 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>() as libc::c_ulong),
        ) as *mut xmlXPathObjectPtr;
    }

    if safe_ret.valueTab.is_null() {
        unsafe {
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            xmlXPathErrMemory(
                ctxt,
                b"creating evaluation context\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as xmlXPathParserContextPtr;
    }
    unsafe {
        (*ret).valueNr = 0 as libc::c_int;
        (*ret).valueMax = 10 as libc::c_int;
        (*ret).value = 0 as xmlXPathObjectPtr;
        (*ret).valueFrame = 0 as libc::c_int;
        (*ret).context = ctxt;
        (*ret).comp = comp;
    }
    return ret;
}
/* *
 * xmlXPathFreeParserContext: * @ctxt: the context to free
 *
 * Free up an xmlXPathParserContext
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathFreeParserContext(mut ctxt: xmlXPathParserContextPtr) {
    let mut i: libc::c_int = 0;
    let safe_ctxt = unsafe { &mut *ctxt };
    if !safe_ctxt.valueTab.is_null() {
        i = 0 as libc::c_int;
        while i < safe_ctxt.valueNr {
            if !safe_ctxt.context.is_null() {
                unsafe {
                    xmlXPathReleaseObject((*ctxt).context, *(*ctxt).valueTab.offset(i as isize))
                };
            } else {
                unsafe { xmlXPathFreeObject(*(*ctxt).valueTab.offset(i as isize)) };
            }
            i += 1
        }
        unsafe {
            xmlFree.expect("non-null function pointer")((*ctxt).valueTab as *mut libc::c_void)
        };
    }
    if !safe_ctxt.comp.is_null() {
        match () {
            #[cfg(XPATH_STREAMING)]
            _ => {
                if unsafe { !(*(*ctxt).comp).stream.is_null() } {
                    unsafe {
                        xmlFreePatternList((*(*ctxt).comp).stream);
                        (*(*ctxt).comp).stream = 0 as xmlPatternPtr
                    }
                }
            }
            #[cfg(not(XPATH_STREAMING))]
            _ => {}
        };
        unsafe { xmlXPathFreeCompExpr((*ctxt).comp) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void) };
}
/* ***********************************************************************
 *									*
 *		The implicit core function library			*
 *									*
 ************************************************************************/
/* *
 * xmlXPathNodeValHash: * @node: a node pointer
 *
 * Function computing the beginning of the string value of the node, * used to speed up comparisons
 *
 * Returns an int usable as a hash
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathNodeValHash(mut node: xmlNodePtr) -> libc::c_uint {
    let mut len: libc::c_int = 2 as libc::c_int;
    let mut string: *const xmlChar = 0 as *const xmlChar;
    let mut tmp: xmlNodePtr = 0 as xmlNodePtr;
    let mut ret: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if node.is_null() {
        return 0 as libc::c_int as libc::c_uint;
    }
    let safe_node = unsafe { &mut *node };
    if safe_node.type_0 as libc::c_uint == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint {
        tmp = unsafe { xmlDocGetRootElement(node as xmlDocPtr as *const xmlDoc) };
        if tmp.is_null() {
            node = safe_node.children
        } else {
            node = tmp
        }
        if node.is_null() {
            return 0 as libc::c_int as libc::c_uint;
        }
    }
    match unsafe { (*node).type_0 as libc::c_uint } {
        8 | 7 | 4 | 3 => {
            string = unsafe { (*node).content };
            if string.is_null() {
                return 0 as libc::c_int as libc::c_uint;
            }
            if unsafe {
                *string.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            } {
                return 0 as libc::c_int as libc::c_uint;
            }
            return unsafe {
                (*string.offset(0 as libc::c_int as isize) as libc::c_uint).wrapping_add(
                    (*string.offset(1 as libc::c_int as isize) as libc::c_uint) << 8 as libc::c_int,
                )
            };
        }
        18 => {
            string = unsafe { (*(node as xmlNsPtr)).href };
            if string.is_null() {
                return 0 as libc::c_int as libc::c_uint;
            }
            if unsafe {
                *string.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            } {
                return 0 as libc::c_int as libc::c_uint;
            }
            return unsafe {
                (*string.offset(0 as libc::c_int as isize) as libc::c_uint).wrapping_add(
                    (*string.offset(1 as libc::c_int as isize) as libc::c_uint) << 8 as libc::c_int,
                )
            };
        }
        2 => tmp = unsafe { (*(node as xmlAttrPtr)).children },
        1 => tmp = unsafe { (*node).children },
        _ => return 0 as libc::c_int as libc::c_uint,
    }
    while !tmp.is_null() {
        match unsafe { (*tmp).type_0 as libc::c_uint } {
            4 | 3 => string = unsafe { (*tmp).content },
            _ => string = 0 as *const xmlChar,
        }
        if unsafe {
            !string.is_null()
                && *string.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
        } {
            if len == 1 as libc::c_int {
                return unsafe {
                    ret.wrapping_add(
                        (*string.offset(0 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                };
            }
            if unsafe {
                *string.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            } {
                len = 1 as libc::c_int;
                ret = unsafe { *string.offset(0 as libc::c_int as isize) as libc::c_uint }
            } else {
                return unsafe {
                    (*string.offset(0 as libc::c_int as isize) as libc::c_uint).wrapping_add(
                        (*string.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                };
            }
        }
        /*
         * Skip to next node
         */
        if unsafe {
            !(*tmp).children.is_null()
                && (*tmp).type_0 as libc::c_uint != XML_DTD_NODE as libc::c_int as libc::c_uint
        } {
            if unsafe {
                (*(*tmp).children).type_0 as libc::c_uint
                    != XML_ENTITY_DECL as libc::c_int as libc::c_uint
            } {
                tmp = unsafe { (*tmp).children };
                continue;
            }
        }
        if tmp == node {
            break;
        }
        if unsafe { !(*tmp).next.is_null() } {
            tmp = unsafe { (*tmp).next }
        } else {
            loop {
                tmp = unsafe { (*tmp).parent };
                if tmp.is_null() {
                    break;
                }
                if tmp == node {
                    tmp = 0 as xmlNodePtr;
                    break;
                } else if unsafe { !(*tmp).next.is_null() } {
                    tmp = unsafe { (*tmp).next };
                    break;
                } else if tmp.is_null() {
                    break;
                }
            }
        }
    }
    return ret;
}
/* *
 * xmlXPathStringHash: * @string: a string
 *
 * Function computing the beginning of the string value of the node, * used to speed up comparisons
 *
 * Returns an int usable as a hash
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathStringHash(mut string: *const xmlChar) -> libc::c_uint {
    if string.is_null() {
        return 0 as libc::c_int as libc::c_uint;
    }
    if unsafe { *string.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int } {
        return 0 as libc::c_int as libc::c_uint;
    }
    return unsafe {
        (*string.offset(0 as libc::c_int as isize) as libc::c_uint).wrapping_add(
            (*string.offset(1 as libc::c_int as isize) as libc::c_uint) << 8 as libc::c_int,
        )
    };
}
/* *
 * xmlXPathCompareNodeSetFloat: * @ctxt: the XPath Parser context
 * @inf: less than (1) or greater than (0) * @strict: is the comparison strict
 * @arg: the node set
 * @f: the value
 *
 * Implement the compare operation between a nodeset and a number
 *     @ns < @val    (1, 1, ...
 *     @ns <= @val   (1, 0, ...
 *     @ns > @val    (0, 1, ...
 *     @ns >= @val   (0, 0, ...
 *
 * If one object to be compared is a node-set and the other is a number, * then the comparison will be true if and only if there is a node in the
 * node-set such that the result of performing the comparison on the number
 * to be compared and on the result of converting the string-value of that
 * node to a number using the number function is true.
 *
 * Returns 0 or 1 depending on the results of the test.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompareNodeSetFloat(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: libc::c_int,
    mut strict: libc::c_int,
    mut arg: xmlXPathObjectPtr,
    mut f: xmlXPathObjectPtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    let safe_arg = unsafe { &mut *arg };
    if f.is_null()
        || arg.is_null()
        || safe_arg.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        unsafe {
            xmlXPathReleaseObject((*ctxt).context, arg);
            xmlXPathReleaseObject((*ctxt).context, f);
        };
        return 0 as libc::c_int;
    }
    ns = safe_arg.nodesetval;
    if !ns.is_null() {
        i = 0 as libc::c_int;
        while i < unsafe { (*ns).nodeNr } {
            str2 = unsafe { xmlXPathCastNodeToString(*(*ns).nodeTab.offset(i as isize)) };
            if !str2.is_null() {
                unsafe {
                    valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                    xmlFree.expect("non-null function pointer")(str2 as *mut libc::c_void);
                    xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
                    valuePush(ctxt, xmlXPathCacheObjectCopy((*ctxt).context, f));
                    ret = xmlXPathCompareValues(ctxt, inf, strict);
                };
                if ret != 0 {
                    break;
                }
            }
            i += 1
        }
    }
    unsafe {
        xmlXPathReleaseObject((*ctxt).context, arg);
        xmlXPathReleaseObject((*ctxt).context, f);
    };
    return ret;
}
/* *
 * xmlXPathCompareNodeSetString: * @ctxt: the XPath Parser context
 * @inf: less than (1) or greater than (0) * @strict: is the comparison strict
 * @arg: the node set
 * @s: the value
 *
 * Implement the compare operation between a nodeset and a string
 *     @ns < @val    (1, 1, ...
 *     @ns <= @val   (1, 0, ...
 *     @ns > @val    (0, 1, ...
 *     @ns >= @val   (0, 0, ...
 *
 * If one object to be compared is a node-set and the other is a string, * then the comparison will be true if and only if there is a node in
 * the node-set such that the result of performing the comparison on the
 * string-value of the node and the other string is true.
 *
 * Returns 0 or 1 depending on the results of the test.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompareNodeSetString(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: libc::c_int,
    mut strict: libc::c_int,
    mut arg: xmlXPathObjectPtr,
    mut s: xmlXPathObjectPtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    let safe_arg = unsafe { &mut *arg };
    if s.is_null()
        || arg.is_null()
        || safe_arg.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        unsafe {
            xmlXPathReleaseObject((*ctxt).context, arg);
            xmlXPathReleaseObject((*ctxt).context, s);
        };
        return 0 as libc::c_int;
    }
    ns = safe_arg.nodesetval;
    if !ns.is_null() {
        i = 0 as libc::c_int;
        while i < unsafe { (*ns).nodeNr } {
            str2 = unsafe { xmlXPathCastNodeToString(*(*ns).nodeTab.offset(i as isize)) };
            if !str2.is_null() {
                unsafe {
                    valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                    xmlFree.expect("non-null function pointer")(str2 as *mut libc::c_void);
                    valuePush(ctxt, xmlXPathCacheObjectCopy((*ctxt).context, s));
                    ret = xmlXPathCompareValues(ctxt, inf, strict);
                };
                if ret != 0 {
                    break;
                }
            }
            i += 1
        }
    }
    unsafe {
        xmlXPathReleaseObject((*ctxt).context, arg);
        xmlXPathReleaseObject((*ctxt).context, s);
    };
    return ret;
}
/* *
 * xmlXPathCompareNodeSets: * @inf: less than (1) or greater than (0) * @strict: is the comparison strict
 * @arg1: the first node set object
 * @arg2: the second node set object
 *
 * Implement the compare operation on nodesets: *
 * If both objects to be compared are node-sets, then the comparison
 * will be true if and only if there is a node in the first node-set
 * and a node in the second node-set such that the result of performing
 * the comparison on the string-values of the two nodes is true.
 * ....
 * When neither object to be compared is a node-set and the operator
 * is <=, <, >= or >, then the objects are compared by converting both
 * objects to numbers and comparing the numbers according to IEEE 754.
 * ....
 * The number function converts its argument to a number as follows: *  - a string that consists of optional whitespace followed by an
 *    optional minus sign followed by a Number followed by whitespace
 *    is converted to the IEEE 754 number that is nearest (according
 *    to the IEEE 754 round-to-nearest rule) to the mathematical value
 *    represented by the string; any other string is converted to NaN
 *
 * Conclusion all nodes need to be converted first to their string value
 * and then the comparison must be done when possible
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompareNodeSets(
    mut inf: libc::c_int,
    mut strict: libc::c_int,
    mut arg1: xmlXPathObjectPtr,
    mut arg2: xmlXPathObjectPtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut init: libc::c_int = 0 as libc::c_int;
    let mut val1: libc::c_double = 0.;
    let mut values2: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ns1: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut ns2: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let safe_arg1 = unsafe { &mut *arg1 };
    let safe_arg2 = unsafe { &mut *arg2 };
    if arg1.is_null()
        || safe_arg1.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg1.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathFreeObject(arg2) };
        return 0 as libc::c_int;
    }
    if arg2.is_null()
        || safe_arg2.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg2.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        unsafe {
            xmlXPathFreeObject(arg1);
            xmlXPathFreeObject(arg2);
        };
        return 0 as libc::c_int;
    }
    ns1 = safe_arg1.nodesetval;
    ns2 = safe_arg2.nodesetval;
    let safe_ns1 = unsafe { &mut *ns1 };
    let safe_ns2 = unsafe { &mut *ns2 };
    if ns1.is_null() || safe_ns1.nodeNr <= 0 as libc::c_int {
        unsafe {
            xmlXPathFreeObject(arg1);
            xmlXPathFreeObject(arg2);
        };
        return 0 as libc::c_int;
    }
    if ns2.is_null() || safe_ns2.nodeNr <= 0 as libc::c_int {
        unsafe {
            xmlXPathFreeObject(arg1);
            xmlXPathFreeObject(arg2);
        };
        return 0 as libc::c_int;
    }
    values2 = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ((*ns2).nodeNr as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double
    };
    if values2.is_null() {
        /* TODO: Propagate memory error. */
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"comparing nodesets\n\x00" as *const u8 as *const libc::c_char,
            );
            xmlXPathFreeObject(arg1);
            xmlXPathFreeObject(arg2);
        };
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < safe_ns1.nodeNr {
        val1 = unsafe { xmlXPathCastNodeToNumber(*(*ns1).nodeTab.offset(i as isize)) };
        if !(unsafe { xmlXPathIsNaN(val1) } != 0) {
            j = 0 as libc::c_int;
            while j < safe_ns2.nodeNr {
                if init == 0 as libc::c_int {
                    unsafe {
                        *values2.offset(j as isize) =
                            xmlXPathCastNodeToNumber(*(*ns2).nodeTab.offset(j as isize))
                    }
                }
                if !(unsafe { xmlXPathIsNaN(*values2.offset(j as isize)) } != 0) {
                    if inf != 0 && strict != 0 {
                        ret = unsafe { (val1 < *values2.offset(j as isize)) } as libc::c_int
                    } else if inf != 0 && strict == 0 {
                        ret = unsafe { (val1 <= *values2.offset(j as isize)) } as libc::c_int
                    } else if inf == 0 && strict != 0 {
                        ret = unsafe { (val1 > *values2.offset(j as isize)) } as libc::c_int
                    } else if inf == 0 && strict == 0 {
                        ret = unsafe { (val1 >= *values2.offset(j as isize)) } as libc::c_int
                    }
                    if ret != 0 {
                        break;
                    }
                }
                j += 1
            }
            if ret != 0 {
                break;
            }
            init = 1 as libc::c_int
        }
        i += 1
    }
    unsafe {
        xmlFree.expect("non-null function pointer")(values2 as *mut libc::c_void);
        xmlXPathFreeObject(arg1);
        xmlXPathFreeObject(arg2);
    };
    return ret;
}
/* *
 * xmlXPathCompareNodeSetValue: * @ctxt: the XPath Parser context
 * @inf: less than (1) or greater than (0) * @strict: is the comparison strict
 * @arg: the node set
 * @val: the value
 *
 * Implement the compare operation between a nodeset and a value
 *     @ns < @val    (1, 1, ...
 *     @ns <= @val   (1, 0, ...
 *     @ns > @val    (0, 1, ...
 *     @ns >= @val   (0, 0, ...
 *
 * If one object to be compared is a node-set and the other is a boolean, * then the comparison will be true if and only if the result of performing
 * the comparison on the boolean and on the result of converting
 * the node-set to a boolean using the boolean function is true.
 *
 * Returns 0 or 1 depending on the results of the test.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompareNodeSetValue(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: libc::c_int,
    mut strict: libc::c_int,
    mut arg: xmlXPathObjectPtr,
    mut val: xmlXPathObjectPtr,
) -> libc::c_int {
    let safe_arg = unsafe { &mut *arg };
    let safe_val = unsafe { &mut *val };
    if val.is_null()
        || arg.is_null()
        || safe_arg.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    match safe_val.type_0 as libc::c_uint {
        3 => return unsafe { xmlXPathCompareNodeSetFloat(ctxt, inf, strict, arg, val) },
        1 | 9 => return unsafe { xmlXPathCompareNodeSets(inf, strict, arg, val) },
        4 => return unsafe { xmlXPathCompareNodeSetString(ctxt, inf, strict, arg, val) },
        2 => {
            unsafe {
                valuePush(ctxt, arg);
                xmlXPathBooleanFunction(ctxt, 1 as libc::c_int);
                valuePush(ctxt, val);
            };
            return unsafe { xmlXPathCompareValues(ctxt, inf, strict) };
        }
        _ => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(), b"xmlXPathCompareNodeSetValue: Can\'t compare node set and object of type %d\n\x00"
                                                                           as *const u8
                                                                           as *const libc::c_char, (*val).type_0
                                                                           as libc::c_uint);
                xmlXPathReleaseObject((*ctxt).context, arg);
                xmlXPathReleaseObject((*ctxt).context, val);
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int)
            };
            return 0 as libc::c_int;
        }
    };
}
/* *
 * xmlXPathEqualNodeSetString: * @arg: the nodeset object argument
 * @str: the string to compare to.
 * @neq: flag to show whether for '=' (0) or '!=' (1) *
 * Implement the equal operation on XPath objects content: @arg1 == @arg2
 * If one object to be compared is a node-set and the other is a string, * then the comparison will be true if and only if there is a node in
 * the node-set such that the result of performing the comparison on the
 * string-value of the node and the other string is true.
 *
 * Returns 0 or 1 depending on the results of the test.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathEqualNodeSetString(
    mut arg: xmlXPathObjectPtr,
    mut str: *const xmlChar,
    mut neq: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    let mut hash: libc::c_uint = 0;
    let safe_arg = unsafe { &mut *arg };
    if str.is_null()
        || arg.is_null()
        || safe_arg.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    ns = safe_arg.nodesetval;
    /*
     * A NULL nodeset compared with a string is always false
     * (since there is no node equal, and no node not equal) */
    let safe_ns = unsafe { &mut *ns };
    if ns.is_null() || safe_ns.nodeNr <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    hash = unsafe { xmlXPathStringHash(str) };
    i = 0 as libc::c_int;
    while i < safe_ns.nodeNr {
        if unsafe { xmlXPathNodeValHash(*(*ns).nodeTab.offset(i as isize)) == hash } {
            str2 =
                unsafe { xmlNodeGetContent(*(*ns).nodeTab.offset(i as isize) as *const xmlNode) };
            if !str2.is_null() && unsafe { xmlStrEqual(str, str2) != 0 } {
                unsafe { xmlFree.expect("non-null function pointer")(str2 as *mut libc::c_void) };
                if !(neq != 0) {
                    return 1 as libc::c_int;
                }
            } else if str2.is_null()
                && unsafe {
                    xmlStrEqual(
                        str,
                        b"\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) != 0
                }
            {
                if !(neq != 0) {
                    return 1 as libc::c_int;
                }
            } else {
                if neq != 0 {
                    if !str2.is_null() {
                        unsafe {
                            xmlFree.expect("non-null function pointer")(str2 as *mut libc::c_void)
                        };
                    }
                    return 1 as libc::c_int;
                }
                if !str2.is_null() {
                    unsafe {
                        xmlFree.expect("non-null function pointer")(str2 as *mut libc::c_void)
                    };
                }
            }
        } else if neq != 0 {
            return 1 as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathEqualNodeSetFloat: * @arg: the nodeset object argument
 * @f: the float to compare to
 * @neq: flag to show whether to compare '=' (0) or '!=' (1) *
 * Implement the equal operation on XPath objects content: @arg1 == @arg2
 * If one object to be compared is a node-set and the other is a number, * then the comparison will be true if and only if there is a node in
 * the node-set such that the result of performing the comparison on the
 * number to be compared and on the result of converting the string-value
 * of that node to a number using the number function is true.
 *
 * Returns 0 or 1 depending on the results of the test.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathEqualNodeSetFloat(
    mut ctxt: xmlXPathParserContextPtr,
    mut arg: xmlXPathObjectPtr,
    mut f: libc::c_double,
    mut neq: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0; /* NaN is unequal to any value */
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    let mut val: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut v: libc::c_double = 0.;
    let safe_arg = unsafe { &mut *arg };
    if arg.is_null()
        || safe_arg.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    ns = safe_arg.nodesetval;
    if !ns.is_null() {
        let safe_ns = unsafe { &mut *ns };
        i = 0 as libc::c_int;
        while i < safe_ns.nodeNr {
            str2 = unsafe { xmlXPathCastNodeToString(*(*ns).nodeTab.offset(i as isize)) };
            if !str2.is_null() {
                unsafe {
                    valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                    xmlFree.expect("non-null function pointer")(str2 as *mut libc::c_void);
                    xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
                    val = valuePop(ctxt);
                    v = (*val).floatval;
                    xmlXPathReleaseObject((*ctxt).context, val);
                }
                if unsafe { xmlXPathIsNaN(v) } == 0 {
                    if neq == 0 && v == f {
                        ret = 1 as libc::c_int;
                        break;
                    } else if neq != 0 && v != f {
                        ret = 1 as libc::c_int;
                        break;
                    }
                } else if neq != 0 {
                    ret = 1 as libc::c_int
                }
            }
            i += 1
        }
    }
    return ret;
}
/* *
 * xmlXPathEqualNodeSets: * @arg1: first nodeset object argument
 * @arg2: second nodeset object argument
 * @neq: flag to show whether to test '=' (0) or '!=' (1) *
 * Implement the equal / not equal operation on XPath nodesets: * @arg1 == @arg2  or  @arg1 != @arg2
 * If both objects to be compared are node-sets, then the comparison
 * will be true if and only if there is a node in the first node-set and
 * a node in the second node-set such that the result of performing the
 * comparison on the string-values of the two nodes is true.
 *
 * (needless to say, this is a costly operation) *
 * Returns 0 or 1 depending on the results of the test.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathEqualNodeSets(
    mut arg1: xmlXPathObjectPtr,
    mut arg2: xmlXPathObjectPtr,
    mut neq: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hashs1: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut hashs2: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut values1: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut values2: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ns1: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut ns2: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let safe_arg1 = unsafe { &mut *arg1 };
    let safe_arg2 = unsafe { &mut *arg2 };
    if arg1.is_null()
        || safe_arg1.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg1.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if arg2.is_null()
        || safe_arg2.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg2.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    ns1 = safe_arg1.nodesetval;
    ns2 = safe_arg2.nodesetval;
    let safe_ns1 = unsafe { &mut *ns1 };
    let safe_ns2 = unsafe { &mut *ns2 };
    if ns1.is_null() || safe_ns1.nodeNr <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if ns2.is_null() || safe_ns2.nodeNr <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    /*
     * for equal, check if there is a node pertaining to both sets
     */
    if neq == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < safe_ns1.nodeNr {
            j = 0 as libc::c_int;
            while j < safe_ns2.nodeNr {
                if unsafe {
                    *(*ns1).nodeTab.offset(i as isize) == *(*ns2).nodeTab.offset(j as isize)
                } {
                    return 1 as libc::c_int;
                }
                j += 1
            }
            i += 1
        }
    }
    values1 = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ((*ns1).nodeNr as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
        ) as *mut *mut xmlChar
    };
    if values1.is_null() {
        /* TODO: Propagate memory error. */
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"comparing nodesets\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return 0 as libc::c_int;
    }
    hashs1 = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ((*ns1).nodeNr as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        ) as *mut libc::c_uint
    };
    if hashs1.is_null() {
        /* TODO: Propagate memory error. */
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"comparing nodesets\n\x00" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(values1 as *mut libc::c_void);
        };
        return 0 as libc::c_int;
    }
    unsafe {
        memset(
            values1 as *mut libc::c_void,
            0 as libc::c_int,
            ((*ns1).nodeNr as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
        )
    };
    values2 = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ((*ns2).nodeNr as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
        ) as *mut *mut xmlChar
    };
    if values2.is_null() {
        /* TODO: Propagate memory error. */
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"comparing nodesets\n\x00" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(hashs1 as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(values1 as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    }
    hashs2 = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ((*ns2).nodeNr as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        ) as *mut libc::c_uint
    };
    if hashs2.is_null() {
        /* TODO: Propagate memory error. */
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"comparing nodesets\n\x00" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(hashs1 as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(values1 as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(values2 as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    }
    unsafe {
        memset(
            values2 as *mut libc::c_void,
            0 as libc::c_int,
            ((*ns2).nodeNr as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
        )
    };
    i = 0 as libc::c_int;
    while i < safe_ns1.nodeNr {
        unsafe {
            *hashs1.offset(i as isize) = xmlXPathNodeValHash(*(*ns1).nodeTab.offset(i as isize))
        };
        j = 0 as libc::c_int;
        while j < safe_ns2.nodeNr {
            if i == 0 as libc::c_int {
                unsafe {
                    *hashs2.offset(j as isize) =
                        xmlXPathNodeValHash(*(*ns2).nodeTab.offset(j as isize))
                }
            }
            if unsafe { *hashs1.offset(i as isize) != *hashs2.offset(j as isize) } {
                if neq != 0 {
                    ret = 1 as libc::c_int;
                    break;
                }
            } else {
                if unsafe { (*values1.offset(i as isize)).is_null() } {
                    unsafe {
                        let ref mut fresh68 = *values1.offset(i as isize);
                        *fresh68 =
                            xmlNodeGetContent(*(*ns1).nodeTab.offset(i as isize) as *const xmlNode)
                    }
                }
                if unsafe { (*values2.offset(j as isize)).is_null() } {
                    unsafe {
                        let ref mut fresh69 = *values2.offset(j as isize);
                        *fresh69 =
                            xmlNodeGetContent(*(*ns2).nodeTab.offset(j as isize) as *const xmlNode)
                    }
                }
                ret = unsafe {
                    xmlStrEqual(*values1.offset(i as isize), *values2.offset(j as isize)) ^ neq
                };
                if ret != 0 {
                    break;
                }
            }
            j += 1
        }
        if ret != 0 {
            break;
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < safe_ns1.nodeNr {
        if unsafe { !(*values1.offset(i as isize)).is_null() } {
            unsafe {
                xmlFree.expect("non-null function pointer")(
                    *values1.offset(i as isize) as *mut libc::c_void
                )
            };
        }
        i += 1
    }
    j = 0 as libc::c_int;
    while j < safe_ns2.nodeNr {
        if unsafe { !(*values2.offset(j as isize)).is_null() } {
            unsafe {
                xmlFree.expect("non-null function pointer")(
                    *values2.offset(j as isize) as *mut libc::c_void
                )
            };
        }
        j += 1
    }
    unsafe {
        xmlFree.expect("non-null function pointer")(values1 as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(values2 as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(hashs1 as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(hashs2 as *mut libc::c_void);
    }
    return ret;
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathEqualValuesCommon(
    mut ctxt: xmlXPathParserContextPtr,
    mut arg1: xmlXPathObjectPtr,
    mut arg2: xmlXPathObjectPtr,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    /*
     *At this point we are assured neither arg1 nor arg2
     *is a nodeset, so we can just pick the appropriate routine.
     */
    let safe_arg1 = unsafe { &mut *arg1 };
    let safe_arg2 = unsafe { &mut *arg2 };
    match safe_arg1.type_0 as libc::c_uint {
        2 => match safe_arg2.type_0 as libc::c_uint {
            2 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Equal: undefined\n" as *const u8 as *const libc::c_char,
                    )
                };
                ret = (safe_arg1.boolval == safe_arg2.boolval) as libc::c_int
            }
            3 => {
                ret = (safe_arg1.boolval
                    == unsafe { xmlXPathCastNumberToBoolean((*arg2).floatval) })
                    as libc::c_int
            }
            4 => {
                if safe_arg2.stringval.is_null()
                    || unsafe {
                        *(*arg2).stringval.offset(0 as libc::c_int as isize) as libc::c_int
                            == 0 as libc::c_int
                    }
                {
                    ret = 0 as libc::c_int
                } else {
                    ret = 1 as libc::c_int
                }
                ret = (safe_arg1.boolval == ret) as libc::c_int
            }
            8 | 5 | 6 | 7 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                        b"xpath.c\x00" as *const u8 as *const libc::c_char,
                        6991 as libc::c_int,
                    )
                };
            }
            0 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Equal: undefined\n" as *const u8 as *const libc::c_char,
                    )
                };
            }
            1 | 9 | _ => {}
        },
        3 => {
            let mut current_block_37: u64;
            match safe_arg2.type_0 as libc::c_uint {
                2 => {
                    ret = unsafe {
                        ((*arg2).boolval == xmlXPathCastNumberToBoolean((*arg1).floatval))
                    } as libc::c_int;
                    current_block_37 = 14220266465818359136;
                }
                4 => {
                    unsafe {
                        valuePush(ctxt, arg2);
                        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
                        arg2 = valuePop(ctxt);
                    }
                    current_block_37 = 1647386430803289519;
                }
                3 => {
                    current_block_37 = 1647386430803289519;
                }
                8 | 5 | 6 | 7 => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Unimplemented block at %s:%d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"xpath.c\x00" as *const u8 as *const libc::c_char,
                            7048 as libc::c_int,
                        )
                    };
                    current_block_37 = 14220266465818359136;
                }
                0 => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Equal: undefined\n" as *const u8 as *const libc::c_char,
                        )
                    };
                    current_block_37 = 14220266465818359136;
                }
                1 | 9 | _ => {
                    current_block_37 = 14220266465818359136;
                }
            }
            match current_block_37 {
                1647386430803289519 =>
                /* Falls through. */
                /* Hand check NaN and Infinity equalities */
                {
                    if unsafe {
                        xmlXPathIsNaN((*arg1).floatval) != 0 || xmlXPathIsNaN((*arg2).floatval) != 0
                    } {
                        ret = 0 as libc::c_int
                    } else if unsafe { xmlXPathIsInf((*arg1).floatval) } == 1 as libc::c_int {
                        if unsafe { xmlXPathIsInf((*arg2).floatval) } == 1 as libc::c_int {
                            ret = 1 as libc::c_int
                        } else {
                            ret = 0 as libc::c_int
                        }
                    } else if unsafe { xmlXPathIsInf((*arg1).floatval) == -(1 as libc::c_int) } {
                        if unsafe { xmlXPathIsInf((*arg2).floatval) == -(1 as libc::c_int) } {
                            ret = 1 as libc::c_int
                        } else {
                            ret = 0 as libc::c_int
                        }
                    } else if unsafe { xmlXPathIsInf((*arg2).floatval) == 1 as libc::c_int } {
                        if unsafe { xmlXPathIsInf((*arg1).floatval) == 1 as libc::c_int } {
                            ret = 1 as libc::c_int
                        } else {
                            ret = 0 as libc::c_int
                        }
                    } else if unsafe { xmlXPathIsInf((*arg2).floatval) == -(1 as libc::c_int) } {
                        if unsafe { xmlXPathIsInf((*arg1).floatval) == -(1 as libc::c_int) } {
                            ret = 1 as libc::c_int
                        } else {
                            ret = 0 as libc::c_int
                        }
                    } else {
                        ret = (safe_arg1.floatval == safe_arg2.floatval) as libc::c_int
                    }
                }
                _ => {}
            }
        }
        4 => {
            match safe_arg2.type_0 as libc::c_uint {
                2 => {
                    if safe_arg1.stringval.is_null()
                        || unsafe {
                            *(*arg1).stringval.offset(0 as libc::c_int as isize) as libc::c_int
                                == 0 as libc::c_int
                        }
                    {
                        ret = 0 as libc::c_int
                    } else {
                        ret = 1 as libc::c_int
                    }
                    ret = (safe_arg2.boolval == ret) as libc::c_int
                }
                4 => ret = unsafe { xmlStrEqual((*arg1).stringval, (*arg2).stringval) },
                3 => {
                    unsafe {
                        valuePush(ctxt, arg1);
                        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
                        arg1 = valuePop(ctxt);
                    }
                    /* Hand check NaN and Infinity equalities */
                    if unsafe {
                        xmlXPathIsNaN((*arg1).floatval) != 0 || xmlXPathIsNaN((*arg2).floatval) != 0
                    } {
                        ret = 0 as libc::c_int
                    } else if unsafe { xmlXPathIsInf((*arg1).floatval) } == 1 as libc::c_int {
                        if unsafe { xmlXPathIsInf((*arg2).floatval) } == 1 as libc::c_int {
                            ret = 1 as libc::c_int
                        } else {
                            ret = 0 as libc::c_int
                        }
                    } else if unsafe { xmlXPathIsInf((*arg1).floatval) } == -(1 as libc::c_int) {
                        if unsafe { xmlXPathIsInf((*arg2).floatval) } == -(1 as libc::c_int) {
                            ret = 1 as libc::c_int
                        } else {
                            ret = 0 as libc::c_int
                        }
                    } else if unsafe { xmlXPathIsInf((*arg2).floatval) } == 1 as libc::c_int {
                        if unsafe { xmlXPathIsInf((*arg1).floatval) } == 1 as libc::c_int {
                            ret = 1 as libc::c_int
                        } else {
                            ret = 0 as libc::c_int
                        }
                    } else if unsafe { xmlXPathIsInf((*arg2).floatval) } == -(1 as libc::c_int) {
                        if unsafe { xmlXPathIsInf((*arg1).floatval) } == -(1 as libc::c_int) {
                            ret = 1 as libc::c_int
                        } else {
                            ret = 0 as libc::c_int
                        }
                    } else {
                        ret = (safe_arg1.floatval == safe_arg2.floatval) as libc::c_int
                    }
                }
                8 | 5 | 6 | 7 => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Unimplemented block at %s:%d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"xpath.c\x00" as *const u8 as *const libc::c_char,
                            7109 as libc::c_int,
                        )
                    };
                }
                1 | 9 | _ => {}
                0 => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Equal: undefined\n" as *const u8 as *const libc::c_char,
                        )
                    };
                }
            }
        }
        8 | 5 | 6 | 7 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                    b"xpath.c\x00" as *const u8 as *const libc::c_char,
                    7120 as libc::c_int,
                )
            };
        }
        0 | 1 | 9 | _ => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Equal: undefined\n" as *const u8 as *const libc::c_char,
                )
            };
        }
    }
    unsafe {
        xmlXPathReleaseObject((*ctxt).context, arg1);
        xmlXPathReleaseObject((*ctxt).context, arg2);
    }
    return ret;
}
/* *
 * xmlXPathEqualValues: * @ctxt: the XPath Parser context
 *
 * Implement the equal operation on XPath objects content: @arg1 == @arg2
 *
 * Returns 0 or 1 depending on the results of the test.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathEqualValues(mut ctxt: xmlXPathParserContextPtr) -> libc::c_int {
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut argtmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as libc::c_int;
    }
    unsafe {
        arg2 = valuePop(ctxt);
        arg1 = valuePop(ctxt);
    }
    if arg1.is_null() || arg2.is_null() {
        if !arg1.is_null() {
            unsafe { xmlXPathReleaseObject((*ctxt).context, arg1) };
        } else {
            unsafe { xmlXPathReleaseObject((*ctxt).context, arg2) };
        }
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return 0 as libc::c_int;
    }
    if arg1 == arg2 {
        match () {
            #[cfg(DEBUG_EXPR)]
            _ => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"NotEqual: by pointer\n\x00" as *const u8 as *const libc::c_char,
                    )
                };
            }
            #[cfg(not(DEBUG_EXPR))]
            _ => {}
        };
        unsafe { xmlXPathFreeObject(arg1) };
        return 1 as libc::c_int;
    }
    /*
     *If either argument is a nodeset, it's a 'special case'
     */
    let safe_arg1 = unsafe { &mut *arg1 };
    let safe_arg2 = unsafe { &mut *arg2 };
    if safe_arg2.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || safe_arg2.type_0 as libc::c_uint == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        || safe_arg1.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || safe_arg1.type_0 as libc::c_uint == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        /*
         *Hack it to assure arg1 is the nodeset
         */
        if safe_arg1.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg1.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        {
            argtmp = arg2;
            arg2 = arg1;
            arg1 = argtmp
        }
        match safe_arg2.type_0 as libc::c_uint {
            1 | 9 => ret = unsafe { xmlXPathEqualNodeSets(arg1, arg2, 0 as libc::c_int) },
            2 => {
                if safe_arg1.nodesetval.is_null()
                    || unsafe { (*(*arg1).nodesetval).nodeNr == 0 as libc::c_int }
                {
                    ret = 0 as libc::c_int
                } else {
                    ret = 1 as libc::c_int
                }
                ret = (ret == safe_arg2.boolval) as libc::c_int
            }
            3 => {
                ret = unsafe {
                    xmlXPathEqualNodeSetFloat(ctxt, arg1, (*arg2).floatval, 0 as libc::c_int)
                }
            }
            4 => {
                ret =
                    unsafe { xmlXPathEqualNodeSetString(arg1, (*arg2).stringval, 0 as libc::c_int) }
            }
            8 | 5 | 6 | 7 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                        b"xpath.c\x00" as *const u8 as *const libc::c_char,
                        7205 as libc::c_int,
                    )
                };
            }
            0 | _ => {
                match () {
                    #[cfg(DEBUG_EXPR)]
                    _ => {
                        unsafe {
                            (*__xmlGenericError()).expect("non-null function pointer")(
                                *__xmlGenericErrorContext(),
                                b"NotEqual: undefined\n\x00" as *const u8 as *const libc::c_char,
                            )
                        };
                    }
                    #[cfg(not(DEBUG_EXPR))]
                    _ => {}
                };
            }
        }
        unsafe {
            xmlXPathReleaseObject((*ctxt).context, arg1);
            xmlXPathReleaseObject((*ctxt).context, arg2);
        }
        return ret;
    }
    return unsafe { xmlXPathEqualValuesCommon(ctxt, arg1, arg2) };
}
/* *
 * xmlXPathNotEqualValues: * @ctxt: the XPath Parser context
 *
 * Implement the equal operation on XPath objects content: @arg1 == @arg2
 *
 * Returns 0 or 1 depending on the results of the test.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNotEqualValues(mut ctxt: xmlXPathParserContextPtr) -> libc::c_int {
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut argtmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as libc::c_int;
    }
    arg2 = unsafe { valuePop(ctxt) };
    arg1 = unsafe { valuePop(ctxt) };
    if arg1.is_null() || arg2.is_null() {
        if !arg1.is_null() {
            unsafe { xmlXPathReleaseObject((*ctxt).context, arg1) };
        } else {
            unsafe { xmlXPathReleaseObject((*ctxt).context, arg2) };
        }
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return 0 as libc::c_int;
    }
    let safe_arg1 = unsafe { &mut *arg1 };
    let safe_arg2 = unsafe { &mut *arg2 };
    if arg1 == arg2 {
        match () {
            #[cfg(DEBUG_EXPR)]
            _ => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"NotEqual: by pointer\n\x00" as *const u8 as *const libc::c_char,
                    )
                };
            }
            #[cfg(not(DEBUG_EXPR))]
            _ => {}
        };
        unsafe { xmlXPathReleaseObject((*ctxt).context, arg1) };
        return 0 as libc::c_int;
    }
    /*
     *If either argument is a nodeset, it's a 'special case'
     */
    if safe_arg2.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || safe_arg2.type_0 as libc::c_uint == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        || safe_arg1.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || safe_arg1.type_0 as libc::c_uint == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        /*
         *Hack it to assure arg1 is the nodeset
         */
        if safe_arg1.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && safe_arg1.type_0 as libc::c_uint != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        {
            argtmp = arg2;
            arg2 = arg1;
            arg1 = argtmp
        }
        match safe_arg2.type_0 as libc::c_uint {
            1 | 9 => ret = unsafe { xmlXPathEqualNodeSets(arg1, arg2, 1 as libc::c_int) },
            2 => {
                if safe_arg1.nodesetval.is_null()
                    || unsafe { (*(*arg1).nodesetval).nodeNr == 0 as libc::c_int }
                {
                    ret = 0 as libc::c_int
                } else {
                    ret = 1 as libc::c_int
                }
                ret = (ret != safe_arg2.boolval) as libc::c_int
            }
            3 => {
                ret = unsafe {
                    xmlXPathEqualNodeSetFloat(ctxt, arg1, (*arg2).floatval, 1 as libc::c_int)
                }
            }
            4 => {
                ret =
                    unsafe { xmlXPathEqualNodeSetString(arg1, (*arg2).stringval, 1 as libc::c_int) }
            }
            8 | 5 | 6 | 7 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                        b"xpath.c\x00" as *const u8 as *const libc::c_char,
                        7290 as libc::c_int,
                    )
                };
            }
            0 | _ => {
                match () {
                    #[cfg(DEBUG_EXPR)]
                    _ => {
                        unsafe {
                            (*__xmlGenericError()).expect("non-null function pointer")(
                                *__xmlGenericErrorContext(),
                                b"NotEqual: undefined\n\x00" as *const u8 as *const libc::c_char,
                            )
                        };
                    }
                    #[cfg(not(DEBUG_EXPR))]
                    _ => {}
                };
            }
        }
        unsafe {
            xmlXPathReleaseObject((*ctxt).context, arg1);
            xmlXPathReleaseObject((*ctxt).context, arg2);
        }

        return ret;
    }
    return unsafe { (xmlXPathEqualValuesCommon(ctxt, arg1, arg2) == 0) as libc::c_int };
}
/* *
 * xmlXPathCompareValues: * @ctxt: the XPath Parser context
 * @inf: less than (1) or greater than (0) * @strict: is the comparison strict
 *
 * Implement the compare operation on XPath objects: *     @arg1 < @arg2    (1, 1, ...
 *     @arg1 <= @arg2   (1, 0, ...
 *     @arg1 > @arg2    (0, 1, ...
 *     @arg1 >= @arg2   (0, 0, ...
 *
 * When neither object to be compared is a node-set and the operator is
 * <=, <, >=, >, then the objects are compared by converted both objects
 * to numbers and comparing the numbers according to IEEE 754. The <
 * comparison will be true if and only if the first number is less than the
 * second number. The <= comparison will be true if and only if the first
 * number is less than or equal to the second number. The > comparison
 * will be true if and only if the first number is greater than the second
 * number. The >= comparison will be true if and only if the first number
 * is greater than or equal to the second number.
 *
 * Returns 1 if the comparison succeeded, 0 if it failed
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCompareValues(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: libc::c_int,
    mut strict: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut arg1i: libc::c_int = 0 as libc::c_int;
    let mut arg2i: libc::c_int = 0 as libc::c_int;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as libc::c_int;
    }
    arg2 = unsafe { valuePop(ctxt) };
    arg1 = unsafe { valuePop(ctxt) };
    let safe_arg1 = unsafe { &mut *arg1 };
    let safe_arg2 = unsafe { &mut *arg2 };
    if arg1.is_null() || arg2.is_null() {
        if !arg1.is_null() {
            unsafe { xmlXPathReleaseObject((*ctxt).context, arg1) };
        } else {
            unsafe { xmlXPathReleaseObject((*ctxt).context, arg2) };
        }
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return 0 as libc::c_int;
    }
    if safe_arg2.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || safe_arg2.type_0 as libc::c_uint == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        || safe_arg1.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || safe_arg1.type_0 as libc::c_uint == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        /*
         * If either argument is a XPATH_NODESET or XPATH_XSLT_TREE the two arguments
         * are not freed from within this routine; they will be freed from the
         * called routine, e.g. xmlXPathCompareNodeSets or xmlXPathCompareNodeSetValue
         */
        if (safe_arg2.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
            || safe_arg2.type_0 as libc::c_uint == XPATH_XSLT_TREE as libc::c_int as libc::c_uint)
            && (safe_arg1.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
                || safe_arg1.type_0 as libc::c_uint
                    == XPATH_XSLT_TREE as libc::c_int as libc::c_uint)
        {
            ret = unsafe { xmlXPathCompareNodeSets(inf, strict, arg1, arg2) }
        } else if safe_arg1.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
            || safe_arg1.type_0 as libc::c_uint == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        {
            ret = unsafe { xmlXPathCompareNodeSetValue(ctxt, inf, strict, arg1, arg2) }
        } else {
            ret = unsafe {
                xmlXPathCompareNodeSetValue(ctxt, (inf == 0) as libc::c_int, strict, arg2, arg1)
            }
        }
        return ret;
    }
    if safe_arg1.type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint {
        unsafe {
            valuePush(ctxt, arg1);
            xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
            arg1 = valuePop(ctxt)
        }
    }
    if unsafe { (*arg1).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint } {
        unsafe {
            xmlXPathFreeObject(arg1);
            xmlXPathFreeObject(arg2);
            xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if safe_arg2.type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint {
        unsafe {
            valuePush(ctxt, arg2);
            xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
            arg2 = valuePop(ctxt)
        }
    }
    if unsafe { (*arg2).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint } {
        unsafe {
            xmlXPathReleaseObject((*ctxt).context, arg1);
            xmlXPathReleaseObject((*ctxt).context, arg2);
            xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    /*
     * Add tests for infinity and nan
     * => feedback on 3.4 for Inf and NaN
     */
    /* Hand check NaN and Infinity comparisons */
    if unsafe { xmlXPathIsNaN((*arg1).floatval) != 0 || xmlXPathIsNaN((*arg2).floatval) != 0 } {
        ret = 0 as libc::c_int
    } else {
        arg1i = unsafe { xmlXPathIsInf((*arg1).floatval) };
        arg2i = unsafe { xmlXPathIsInf((*arg2).floatval) };
        if inf != 0 && strict != 0 {
            if arg1i == -(1 as libc::c_int) && arg2i != -(1 as libc::c_int)
                || arg2i == 1 as libc::c_int && arg1i != 1 as libc::c_int
            {
                ret = 1 as libc::c_int
            } else if arg1i == 0 as libc::c_int && arg2i == 0 as libc::c_int {
                ret = unsafe { ((*arg1).floatval < (*arg2).floatval) as libc::c_int }
            } else {
                ret = 0 as libc::c_int
            }
        } else if inf != 0 && strict == 0 {
            if arg1i == -(1 as libc::c_int) || arg2i == 1 as libc::c_int {
                ret = 1 as libc::c_int
            } else if arg1i == 0 as libc::c_int && arg2i == 0 as libc::c_int {
                ret = unsafe { ((*arg1).floatval <= (*arg2).floatval) as libc::c_int }
            } else {
                ret = 0 as libc::c_int
            }
        } else if inf == 0 && strict != 0 {
            if arg1i == 1 as libc::c_int && arg2i != 1 as libc::c_int
                || arg2i == -(1 as libc::c_int) && arg1i != -(1 as libc::c_int)
            {
                ret = 1 as libc::c_int
            } else if arg1i == 0 as libc::c_int && arg2i == 0 as libc::c_int {
                ret = unsafe { ((*arg1).floatval > (*arg2).floatval) as libc::c_int }
            } else {
                ret = 0 as libc::c_int
            }
        } else if inf == 0 && strict == 0 {
            if arg1i == 1 as libc::c_int || arg2i == -(1 as libc::c_int) {
                ret = 1 as libc::c_int
            } else if arg1i == 0 as libc::c_int && arg2i == 0 as libc::c_int {
                ret = unsafe { ((*arg1).floatval >= (*arg2).floatval) as libc::c_int }
            } else {
                ret = 0 as libc::c_int
            }
        }
    }
    unsafe {
        xmlXPathReleaseObject((*ctxt).context, arg1);
        xmlXPathReleaseObject((*ctxt).context, arg2);
    }
    return ret;
}
/* *
 * xmlXPathValueFlipSign: * @ctxt: the XPath Parser context
 *
 * Implement the unary - operation on an XPath object
 * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathValueFlipSign(mut ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    unsafe { (*(*ctxt).value).floatval = -(*(*ctxt).value).floatval };
}
/* *
 * xmlXPathAddValues: * @ctxt: the XPath Parser context
 *
 * Implement the add operation on XPath objects: * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathAddValues(mut ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = unsafe { valuePop(ctxt) };
    if arg.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return;
    }
    unsafe {
        val = xmlXPathCastToNumber(arg);
        xmlXPathReleaseObject((*ctxt).context, arg);
    };
    if !safe_ctxt.value.is_null()
        && unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    unsafe { (*(*ctxt).value).floatval += val };
}
/* *
 * xmlXPathSubValues: * @ctxt: the XPath Parser context
 *
 * Implement the subtraction operation on XPath objects: * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathSubValues(mut ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = unsafe { valuePop(ctxt) };
    if arg.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return;
    }
    unsafe {
        val = xmlXPathCastToNumber(arg);
        xmlXPathReleaseObject((*ctxt).context, arg);
    };
    if !safe_ctxt.value.is_null()
        && unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    unsafe { (*(*ctxt).value).floatval -= val };
}
/* *
 * xmlXPathMultValues: * @ctxt: the XPath Parser context
 *
 * Implement the multiply operation on XPath objects: * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathMultValues(mut ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = unsafe { valuePop(ctxt) };
    if arg.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return;
    }
    unsafe {
        val = xmlXPathCastToNumber(arg);
        xmlXPathReleaseObject((*ctxt).context, arg);
    };
    if !safe_ctxt.value.is_null()
        && unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    unsafe { (*(*ctxt).value).floatval *= val };
}
/* *
 * xmlXPathDivValues: * @ctxt: the XPath Parser context
 *
 * Implement the div operation on XPath objects @arg1 / @arg2: * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathDivValues(mut ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = unsafe { valuePop(ctxt) };
    if arg.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return;
    }
    unsafe {
        val = xmlXPathCastToNumber(arg);
        xmlXPathReleaseObject((*ctxt).context, arg);
    };
    if !safe_ctxt.value.is_null()
        && unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    unsafe { (*(*ctxt).value).floatval /= val };
}
/* *
 * xmlXPathModValues: * @ctxt: the XPath Parser context
 *
 * Implement the mod operation on XPath objects: @arg1 / @arg2
 * The numeric operators convert their operands to numbers as if
 * by calling the number function.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathModValues(mut ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg1: libc::c_double = 0.;
    let mut arg2: libc::c_double = 0.;
    arg = unsafe { valuePop(ctxt) };
    if arg.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return;
    }
    unsafe {
        arg2 = xmlXPathCastToNumber(arg);
        xmlXPathReleaseObject((*ctxt).context, arg);
    };
    if !safe_ctxt.value.is_null()
        && unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe {
            (*(*ctxt).value).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint
        }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    arg1 = unsafe { (*(*ctxt).value).floatval };
    if arg2 == 0 as libc::c_int as libc::c_double {
        unsafe { (*(*ctxt).value).floatval = xmlXPathNAN }
    } else {
        unsafe { (*(*ctxt).value).floatval = fmod(arg1, arg2) }
    };
}
/*
 * Some of the axis navigation routines.
 */
/* *
 * xmlXPathNextSelf: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "self" direction
 * The self axis contains just the context node itself
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextSelf(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        return unsafe { (*(*ctxt).context).node };
    }
    return 0 as xmlNodePtr;
}
/* *
 * xmlXPathNextChild: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "child" direction
 * The child axis contains the children of the context node in document order.
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextChild(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if unsafe { (*(*ctxt).context).node.is_null() } {
            return 0 as xmlNodePtr;
        }
        match () {
            #[cfg(LIBXML_DOCB_ENABLED)]
            _ => match unsafe { (*(*(*ctxt).context).node).type_0 as libc::c_uint } {
                1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 => {
                    return unsafe { (*(*(*ctxt).context).node).children }
                }
                9 | 10 | 11 | 13 => {
                    return unsafe { (*((*(*ctxt).context).node as xmlDocPtr)).children }
                }
                15 | 16 | 17 | 2 | 18 | 19 | 20 => return 0 as xmlNodePtr,
                _ => {}
            },
            #[cfg(not(LIBXML_DOCB_ENABLED))]
            _ => match unsafe { (*(*(*ctxt).context).node).type_0 as libc::c_uint } {
                1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 => {
                    return unsafe { (*(*(*ctxt).context).node).children }
                }
                9 | 10 | 11 | 13 | 21 => {
                    return unsafe { (*((*(*ctxt).context).node as xmlDocPtr)).children }
                }
                15 | 16 | 17 | 2 | 18 | 19 | 20 => return 0 as xmlNodePtr,
                _ => {}
            },
        };
        return 0 as xmlNodePtr;
    }
    let safe_cur = unsafe { &mut *cur };
    if safe_cur.type_0 as libc::c_uint == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        || safe_cur.type_0 as libc::c_uint == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    return safe_cur.next;
}
/* *
 * xmlXPathNextChildElement: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "child" direction and nodes of type element.
 * The child axis contains the children of the context node in document order.
 *
 * Returns the next element following that axis
 */
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathNextChildElement(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        cur = unsafe { (*safe_ctxt.context).node };
        if cur.is_null() {
            return 0 as xmlNodePtr;
        }
        /*
        	* Get the first element child.
        	*/
        match () {
            #[cfg(LIBXML_DOCB_ENABLED)]
            _ => {
                match unsafe { (*cur).type_0 as libc::c_uint } {
                    1 | 11 | 5 | 6 => {
                        /* URGENT TODO: entify-refs as well? */
                        cur = unsafe { (*cur).children };
                        if !cur.is_null() {
                            if unsafe { (*cur).type_0 as libc::c_uint }
                                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                            {
                                return cur;
                            }
                            loop {
                                cur = unsafe { (*cur).next };
                                if !(!cur.is_null()
                                    && unsafe { (*cur).type_0 as libc::c_uint }
                                        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint)
                                {
                                    break;
                                }
                            }
                            return cur;
                        }
                        return 0 as xmlNodePtr;
                    }
                    9 | 13 | 21 => {
                        return unsafe { xmlDocGetRootElement(cur as xmlDocPtr as *const xmlDoc) }
                    }
                    _ => return 0 as xmlNodePtr,
                }
            }
            #[cfg(not(LIBXML_DOCB_ENABLED))]
            _ => {
                match unsafe { (*cur).type_0 as libc::c_uint } {
                    1 | 11 | 5 | 6 => {
                        /* URGENT TODO: entify-refs as well? */
                        cur = unsafe { (*cur).children };
                        if !cur.is_null() {
                            if unsafe { (*cur).type_0 as libc::c_uint }
                                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                            {
                                return cur;
                            }
                            loop {
                                cur = unsafe { (*cur).next };
                                if !(!cur.is_null()
                                    && unsafe { (*cur).type_0 as libc::c_uint }
                                        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint)
                                {
                                    break;
                                }
                            }
                            return cur;
                        }
                        return 0 as xmlNodePtr;
                    }
                    9 | 13 => {
                        return unsafe { xmlDocGetRootElement(cur as xmlDocPtr as *const xmlDoc) }
                    }
                    _ => return 0 as xmlNodePtr,
                }
            }
        };
    }
    /*
     * Get the next sibling element node.
     */
    match unsafe { (*cur).type_0 as libc::c_uint } {
        1 | 3 | 5 | 6 | 4 | 7 | 8 | 20 => {}
        _ => {
            /* case XML_DTD_NODE: */
            /* URGENT TODO: DTD-node as well? */
            return 0 as xmlNodePtr;
        }
    }
    if !unsafe { (*cur).next.is_null() } {
        if unsafe { (*(*cur).next).type_0 as libc::c_uint }
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        {
            return unsafe { (*cur).next };
        }
        cur = unsafe { (*cur).next };
        loop {
            cur = unsafe { (*cur).next };
            if !(!cur.is_null()
                && unsafe { (*cur).type_0 as libc::c_uint }
                    != XML_ELEMENT_NODE as libc::c_int as libc::c_uint)
            {
                break;
            }
        }
        return cur;
    }
    return 0 as xmlNodePtr;
}
/* *
 * xmlXPathNextDescendant: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "descendant" direction
 * the descendant axis contains the descendants of the context node in document
 * order; a descendant is a child or a child of a child and so on.
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextDescendant(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if unsafe { (*safe_ctxt.context).node.is_null() } {
            return 0 as xmlNodePtr;
        }
        if unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint }
            == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
            || unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint }
                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            return 0 as xmlNodePtr;
        }
        if unsafe { (*safe_ctxt.context).node == (*safe_ctxt.context).doc as xmlNodePtr } {
            return unsafe { (*(*safe_ctxt.context).doc).children };
        }
        return unsafe { (*(*safe_ctxt.context).node).children };
    }
    if unsafe { (*cur).type_0 as libc::c_uint } == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if !unsafe { (*cur).children.is_null() } {
        /*
         * Do not descend on entities declarations
         */
        if unsafe { (*(*cur).children).type_0 as libc::c_uint }
            != XML_ENTITY_DECL as libc::c_int as libc::c_uint
        {
            unsafe { cur = (*cur).children };
            /*
             * Skip DTDs
             */
            if unsafe { (*cur).type_0 as libc::c_uint }
                != XML_DTD_NODE as libc::c_int as libc::c_uint
            {
                return cur;
            }
        }
    }
    if cur == unsafe { (*safe_ctxt.context).node } {
        return 0 as xmlNodePtr;
    }
    while !unsafe { (*cur).next.is_null() } {
        unsafe { cur = (*cur).next };
        if unsafe { (*cur).type_0 as libc::c_uint }
            != XML_ENTITY_DECL as libc::c_int as libc::c_uint
            && unsafe { (*cur).type_0 as libc::c_uint }
                != XML_DTD_NODE as libc::c_int as libc::c_uint
        {
            return cur;
        }
    }
    loop {
        cur = unsafe { (*cur).parent };
        if cur.is_null() {
            break;
        }
        if cur == unsafe { (*safe_ctxt.context).node } {
            return 0 as xmlNodePtr;
        }
        if !unsafe { (*cur).next.is_null() } {
            cur = unsafe { (*cur).next };
            return cur;
        }
        if cur.is_null() {
            break;
        }
    }
    return cur;
}
/* *
 * xmlXPathNextDescendantOrSelf: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "descendant-or-self" direction
 * the descendant-or-self axis contains the context node and the descendants
 * of the context node in document order; thus the context node is the first
 * node on the axis, and the first child of the context node is the second node
 * on the axis
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextDescendantOrSelf(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        return unsafe { (*safe_ctxt.context).node };
    }
    if unsafe { (*safe_ctxt.context).node.is_null() } {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint }
        == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        || unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint }
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    return unsafe { xmlXPathNextDescendant(ctxt, cur) };
}
/* *
 * xmlXPathNextParent: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "parent" direction
 * The parent axis contains the parent of the context node, if there is one.
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextParent(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    /*
    * the parent of an attribute or namespace node is the element
    * to which the attribute or namespace node is attached
    * Namespace handling !!!
    */
    if cur.is_null() {
        if unsafe { (*safe_ctxt.context).node.is_null() } {
            return 0 as xmlNodePtr;
        }
        match unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint } {
            1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 15 | 16 | 19 | 20 | 17 => {
                if unsafe { (*(*safe_ctxt.context).node).parent.is_null() } {
                    return unsafe { (*safe_ctxt.context).doc as xmlNodePtr };
                }
                if unsafe {
                    (*(*(*safe_ctxt.context).node).parent).type_0 as libc::c_uint
                        == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                } && unsafe {
                    (*(*(*(*safe_ctxt.context).node).parent)
                        .name
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        == ' ' as i32
                        || xmlStrEqual(
                            (*(*(*safe_ctxt.context).node).parent).name,
                            b"fake node libxslt\x00" as *const u8 as *const libc::c_char
                                as *mut xmlChar,
                        ) != 0)
                } {
                    return 0 as xmlNodePtr;
                }
                return unsafe { (*(*safe_ctxt.context).node).parent };
            }
            2 => {
                let mut att: xmlAttrPtr = unsafe { (*safe_ctxt.context).node as xmlAttrPtr };
                return unsafe { (*att).parent };
            }
            9 | 10 | 11 | 13 => return 0 as xmlNodePtr,
            18 => {
                let mut ns: xmlNsPtr = unsafe { (*safe_ctxt.context).node as xmlNsPtr };
                if !unsafe { (*ns).next.is_null() }
                    && unsafe { (*(*ns).next).type_0 as libc::c_uint }
                        != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    return unsafe { (*ns).next as xmlNodePtr };
                }
                return 0 as xmlNodePtr;
            }
            _ => {}
        }
    }
    return 0 as xmlNodePtr;
}
/* *
 * xmlXPathNextAncestor: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "ancestor" direction
 * the ancestor axis contains the ancestors of the context node; the ancestors
 * of the context node consist of the parent of context node and the parent's
 * parent and so on; the nodes are ordered in reverse document order; thus the
 * parent is the first node on the axis, and the parent's parent is the second
 * node on the axis
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextAncestor(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    /*
     * the parent of an attribute or namespace node is the element
     * to which the attribute or namespace node is attached
     * !!!!!!!!!!!!!
     */
    if cur.is_null() {
        if unsafe { (*safe_ctxt.context).node.is_null() } {
            return 0 as xmlNodePtr;
        }
        match unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint } {
            1 | 3 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 16 | 17 | 12 | 19 | 20 => {
                if unsafe { (*(*safe_ctxt.context).node).parent.is_null() } {
                    return unsafe { (*safe_ctxt.context).doc as xmlNodePtr };
                }
                if unsafe {
                    (*(*(*safe_ctxt.context).node).parent).type_0 as libc::c_uint
                        == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                } && unsafe {
                    (*(*(*(*safe_ctxt.context).node).parent)
                        .name
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        == ' ' as i32
                        || xmlStrEqual(
                            (*(*(*safe_ctxt.context).node).parent).name,
                            b"fake node libxslt\x00" as *const u8 as *const libc::c_char
                                as *mut xmlChar,
                        ) != 0)
                } {
                    return 0 as xmlNodePtr;
                }
                return unsafe { (*(*safe_ctxt.context).node).parent };
            }
            2 => {
                let mut tmp: xmlAttrPtr = unsafe { (*safe_ctxt.context).node as xmlAttrPtr };
                return unsafe { (*tmp).parent };
            }
            9 | 10 | 11 | 13 => return 0 as xmlNodePtr,
            18 => {
                let mut ns: xmlNsPtr = unsafe { (*safe_ctxt.context).node as xmlNsPtr };
                let safe_ns = unsafe { &mut *ns };
                if !safe_ns.next.is_null()
                    && unsafe { (*safe_ns.next).type_0 as libc::c_uint }
                        != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    return safe_ns.next as xmlNodePtr;
                }
                /* Bad, how did that namespace end up here ? */
                return 0 as xmlNodePtr;
            }
            _ => {}
        }
        return 0 as xmlNodePtr;
    }
    if cur == unsafe { (*(*safe_ctxt.context).doc).children } {
        return unsafe { (*safe_ctxt.context).doc as xmlNodePtr };
    }
    if cur == unsafe { (*safe_ctxt.context).doc as xmlNodePtr } {
        return 0 as xmlNodePtr;
    }
    match unsafe { (*cur).type_0 as libc::c_uint } {
        1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 15 | 16 | 17 | 19 | 20 => {
            if unsafe { (*cur).parent.is_null() } {
                return 0 as xmlNodePtr;
            }
            if unsafe { (*(*cur).parent).type_0 as libc::c_uint }
                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                && unsafe {
                    (*(*(*cur).parent).name.offset(0 as libc::c_int as isize) as libc::c_int
                        == ' ' as i32
                        || xmlStrEqual(
                            (*(*cur).parent).name,
                            b"fake node libxslt\x00" as *const u8 as *const libc::c_char
                                as *mut xmlChar,
                        ) != 0)
                }
            {
                return 0 as xmlNodePtr;
            }
            return unsafe { (*cur).parent };
        }
        2 => {
            let mut att: xmlAttrPtr = cur as xmlAttrPtr;
            return unsafe { (*att).parent };
        }
        18 => {
            let mut ns_0: xmlNsPtr = cur as xmlNsPtr;
            if !unsafe { (*ns_0).next.is_null() }
                && unsafe {
                    (*(*ns_0).next).type_0 as libc::c_uint
                        != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                }
            {
                return unsafe { (*ns_0).next as xmlNodePtr };
            }
            /* Bad, how did that namespace end up here ? */
            return 0 as xmlNodePtr;
        }
        9 | 10 | 11 | 13 => return 0 as xmlNodePtr,
        _ => {}
    }
    return 0 as xmlNodePtr;
}
/* *
 * xmlXPathNextAncestorOrSelf: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "ancestor-or-self" direction
 * he ancestor-or-self axis contains the context node and ancestors of
 * the context node in reverse document order; thus the context node is
 * the first node on the axis, and the context node's parent the second;
 * parent here is defined the same as with the parent axis.
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextAncestorOrSelf(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        return unsafe { (*safe_ctxt.context).node };
    }
    return unsafe { xmlXPathNextAncestor(ctxt, cur) };
}
/* *
 * xmlXPathNextFollowingSibling: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "following-sibling" direction
 * The following-sibling axis contains the following siblings of the context
 * node in document order.
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextFollowingSibling(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint }
        == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        || unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint }
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if cur == unsafe { (*safe_ctxt.context).doc as xmlNodePtr } {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        return unsafe { (*(*safe_ctxt.context).node).next };
    }
    return unsafe { (*cur).next };
}
/* *
 * xmlXPathNextPrecedingSibling: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "preceding-sibling" direction
 * The preceding-sibling axis contains the preceding siblings of the context
 * node in reverse document order; the first preceding sibling is first on the
 * axis; the sibling preceding that node is the second on the axis and so on.
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextPrecedingSibling(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint }
        == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        || unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint }
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if cur == unsafe { (*safe_ctxt.context).doc as xmlNodePtr } {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        return unsafe { (*(*safe_ctxt.context).node).prev };
    }
    if !unsafe { (*cur).prev.is_null() }
        && unsafe { (*(*cur).prev).type_0 as libc::c_uint }
            == XML_DTD_NODE as libc::c_int as libc::c_uint
    {
        unsafe { cur = (*cur).prev };
        if cur.is_null() {
            return unsafe { (*(*safe_ctxt.context).node).prev };
        }
    }
    return unsafe { (*cur).prev };
}
/* *
 * xmlXPathNextFollowing: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "following" direction
 * The following axis contains all nodes in the same document as the context
 * node that are after the context node in document order, excluding any
 * descendants and excluding attribute nodes and namespace nodes; the nodes
 * are ordered in document order
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextFollowing(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    } /* ERROR */

    if !cur.is_null()
        && unsafe {
            (*cur).type_0 as libc::c_uint != XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
                && (*cur).type_0 as libc::c_uint
                    != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                && !(*cur).children.is_null()
        }
    {
        return unsafe { (*cur).children };
    }
    if cur.is_null() {
        cur = unsafe { (*safe_ctxt.context).node };
        if unsafe { (*cur).type_0 as libc::c_uint }
            == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        {
            unsafe { cur = (*cur).parent }
        } else if unsafe { (*cur).type_0 as libc::c_uint }
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if unsafe { (*ns).next.is_null() }
                || unsafe { (*(*ns).next).type_0 as libc::c_uint }
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                return 0 as xmlNodePtr;
            }
            cur = unsafe { (*ns).next as xmlNodePtr }
        }
    }
    if cur.is_null() {
        return 0 as xmlNodePtr;
    }
    if !unsafe { (*cur).next.is_null() } {
        return unsafe { (*cur).next };
    }
    loop {
        cur = unsafe { (*cur).parent };
        if cur.is_null() {
            break;
        }
        if cur == unsafe { (*safe_ctxt.context).doc as xmlNodePtr } {
            return 0 as xmlNodePtr;
        }
        if !unsafe { (*cur).next.is_null() } {
            return unsafe { (*cur).next };
        }
        if cur.is_null() {
            break;
        }
    }
    return cur;
}
/*
 * xmlXPathIsAncestor: * @ancestor: the ancestor node
 * @node: the current node
 *
 * Check that @ancestor is a @node's ancestor
 *
 * returns 1 if @ancestor is a @node's ancestor, 0 otherwise.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathIsAncestor(mut ancestor: xmlNodePtr, mut node: xmlNodePtr) -> libc::c_int {
    let safe_node = unsafe { &mut *node };
    if ancestor.is_null() || node.is_null() {
        return 0 as libc::c_int;
    }
    if safe_node.type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if unsafe { (*ancestor).type_0 as libc::c_uint }
        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    /* nodes need to be in the same document */
    if unsafe { (*ancestor).doc != safe_node.doc } {
        return 0 as libc::c_int;
    }
    /* avoid searching if ancestor or node is the root node */
    if ancestor == safe_node.doc as xmlNodePtr {
        return 1 as libc::c_int;
    }
    if node == unsafe { (*ancestor).doc as xmlNodePtr } {
        return 0 as libc::c_int;
    }
    while !safe_node.parent.is_null() {
        if safe_node.parent == ancestor {
            return 1 as libc::c_int;
        }
        node = safe_node.parent
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathNextPreceding: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "preceding" direction
 * the preceding axis contains all nodes in the same document as the context
 * node that are before the context node in document order, excluding any
 * ancestors and excluding attribute nodes and namespace nodes; the nodes are
 * ordered in reverse document order
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNextPreceding(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        cur = unsafe { (*safe_ctxt.context).node };
        if unsafe { (*cur).type_0 as libc::c_uint }
            == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        {
            cur = unsafe { (*cur).parent }
        } else if unsafe { (*cur).type_0 as libc::c_uint }
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if unsafe { (*ns).next.is_null() }
                || unsafe { (*(*ns).next).type_0 as libc::c_uint }
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                return 0 as xmlNodePtr;
            }
            cur = unsafe { (*ns).next as xmlNodePtr }
        }
    }
    if cur.is_null()
        || unsafe { (*cur).type_0 as libc::c_uint }
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if !unsafe { (*cur).prev.is_null() }
        && unsafe { (*(*cur).prev).type_0 as libc::c_uint }
            == XML_DTD_NODE as libc::c_int as libc::c_uint
    {
        unsafe { cur = (*cur).prev }
    }
    loop {
        if !unsafe { (*cur).prev.is_null() } {
            cur = unsafe { (*cur).prev };
            while !unsafe { (*cur).last.is_null() } {
                cur = unsafe { (*cur).last }
            }
            return cur;
        }
        cur = unsafe { (*cur).parent };
        if cur.is_null() {
            return 0 as xmlNodePtr;
        }
        if cur == unsafe { (*(*safe_ctxt.context).doc).children } {
            return 0 as xmlNodePtr;
        }
        if !unsafe { (xmlXPathIsAncestor(cur, (*safe_ctxt.context).node) != 0) } {
            break;
        }
    }
    return cur;
}
/* *
 * xmlXPathNextPrecedingInternal: * @ctxt: the XPath Parser context
 * @cur: the current node in the traversal
 *
 * Traversal function for the "preceding" direction
 * the preceding axis contains all nodes in the same document as the context
 * node that are before the context node in document order, excluding any
 * ancestors and excluding attribute nodes and namespace nodes; the nodes are
 * ordered in reverse document order
 * This is a faster implementation but internal only since it requires a
 * state kept in the parser context: ctxt->ancestor.
 *
 * Returns the next element following that axis
 */
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathNextPrecedingInternal(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }

    if cur.is_null() {
        cur = unsafe { (*safe_ctxt.context).node };
        if cur.is_null() {
            return 0 as xmlNodePtr;
        }
        if unsafe { (*cur).type_0 as libc::c_uint }
            == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        {
            cur = unsafe { (*cur).parent }
        } else if unsafe { (*cur).type_0 as libc::c_uint }
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if unsafe {
                (*ns).next.is_null()
                    || (*(*ns).next).type_0 as libc::c_uint
                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            } {
                return 0 as xmlNodePtr;
            }
            cur = unsafe { (*ns).next as xmlNodePtr }
        }
        safe_ctxt.ancestor = unsafe { (*cur).parent }
    }
    if unsafe { (*cur).type_0 as libc::c_uint } == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if !unsafe { (*cur).prev.is_null() }
        && unsafe { (*(*cur).prev).type_0 as libc::c_uint }
            == XML_DTD_NODE as libc::c_int as libc::c_uint
    {
        cur = unsafe { (*cur).prev }
    }
    while unsafe { (*cur).prev.is_null() } {
        cur = unsafe { (*cur).parent };
        if cur.is_null() {
            return 0 as xmlNodePtr;
        }
        if cur == unsafe { (*(*safe_ctxt.context).doc).children } {
            return 0 as xmlNodePtr;
        }
        if cur != safe_ctxt.ancestor {
            return cur;
        }
        safe_ctxt.ancestor = unsafe { (*cur).parent }
    }
    cur = unsafe { (*cur).prev };
    while !unsafe { (*cur).last.is_null() } {
        cur = unsafe { (*cur).last }
    }
    return cur;
}
/* *
 * xmlXPathNextNamespace: * @ctxt: the XPath Parser context
 * @cur: the current attribute in the traversal
 *
 * Traversal function for the "namespace" direction
 * the namespace axis contains the namespace nodes of the context node;
 * the order of nodes on this axis is implementation-defined; the axis will
 * be empty unless the context node is an element
 *
 * We keep the XML namespace node at the end of the list.
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextNamespace(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint }
        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if !unsafe { (*safe_ctxt.context).tmpNsList.is_null() } {
            unsafe {
                xmlFree.expect("non-null function pointer")(
                    (*safe_ctxt.context).tmpNsList as *mut libc::c_void,
                )
            };
        }
        unsafe {
            (*safe_ctxt.context).tmpNsList = xmlGetNsList(
                (*safe_ctxt.context).doc as *const xmlDoc,
                (*safe_ctxt.context).node as *const xmlNode,
            )
        };
        unsafe { (*safe_ctxt.context).tmpNsNr = 0 as libc::c_int };
        if !unsafe { (*safe_ctxt.context).tmpNsList.is_null() } {
            while !unsafe {
                (*(*safe_ctxt.context)
                    .tmpNsList
                    .offset((*safe_ctxt.context).tmpNsNr as isize))
                .is_null()
            } {
                unsafe { (*safe_ctxt.context).tmpNsNr += 1 }
            }
        }
        return unsafe { xmlXPathXMLNamespace as xmlNodePtr };
    }
    if unsafe { (*safe_ctxt.context).tmpNsNr > 0 as libc::c_int } {
        unsafe { (*safe_ctxt.context).tmpNsNr -= 1 };
        return unsafe {
            *(*safe_ctxt.context)
                .tmpNsList
                .offset((*safe_ctxt.context).tmpNsNr as isize) as xmlNodePtr
        };
    } else {
        if !unsafe { (*safe_ctxt.context).tmpNsList.is_null() } {
            unsafe {
                xmlFree.expect("non-null function pointer")(
                    (*safe_ctxt.context).tmpNsList as *mut libc::c_void,
                )
            };
        }
        unsafe { (*safe_ctxt.context).tmpNsList = 0 as *mut xmlNsPtr };
        return 0 as xmlNodePtr;
    };
}
/* *
 * xmlXPathNextAttribute: * @ctxt: the XPath Parser context
 * @cur: the current attribute in the traversal
 *
 * Traversal function for the "attribute" direction
 * TODO: support DTD inherited default attributes
 *
 * Returns the next element following that axis
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNextAttribute(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*safe_ctxt.context).node.is_null() } {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*(*safe_ctxt.context).node).type_0 as libc::c_uint }
        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if unsafe { (*safe_ctxt.context).node == (*safe_ctxt.context).doc as xmlNodePtr } {
            return 0 as xmlNodePtr;
        }
        return unsafe { (*(*safe_ctxt.context).node).properties as xmlNodePtr };
    }
    let safe_cur = unsafe { &mut *cur };
    return safe_cur.next as xmlNodePtr;
}
/* ***********************************************************************
 *									*
 *		NodeTest Functions					*
 *									*
 ************************************************************************/
/* ***********************************************************************
 *									*
 *		Implicit tree core function library			*
 *									*
 ************************************************************************/
/* *
 * xmlXPathRoot: * @ctxt: the XPath Parser context
 *
 * Initialize the context to the root of the document
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRoot(mut ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return;
    }

    unsafe {
        valuePush(
            ctxt,
            xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).doc as xmlNodePtr),
        )
    };
}
/*
 * The official core of XPath functions.
 */
/* ***********************************************************************
 *									*
 *		The explicit core function library			*
 *http://www.w3.org/Style/XSL/Group/1999/07/xpath-19990705.html#corelib	*
 *									*
 ************************************************************************/
/* *
 * xmlXPathLastFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the last() XPath function
 *    number last() * The last function returns the number of nodes in the context node list.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathLastFunction(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 0 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if unsafe { (*safe_ctxt.context).contextSize } >= 0 as libc::c_int {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewFloat(
                    safe_ctxt.context,
                    (*safe_ctxt.context).contextSize as libc::c_double,
                ),
            )
        };
        match () {
            #[cfg(DEBUG_EXPR)]
            _ => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"last() : %d\n\x00" as *const u8 as *const libc::c_char,
                        (*safe_ctxt.context).contextSize,
                    )
                };
            }
            #[cfg(not(DEBUG_EXPR))]
            _ => {}
        };
    } else {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_CTXT_SIZE as libc::c_int) };
        return;
    };
}
/* *
 * xmlXPathPositionFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the position() XPath function
 *    number position() * The position function returns the position of the context node in the
 * context node list. The first position is 1, and so the last position
 * will be equal to last().
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathPositionFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 0 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if unsafe { (*safe_ctxt.context).proximityPosition } >= 0 as libc::c_int {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewFloat(
                    safe_ctxt.context,
                    (*safe_ctxt.context).proximityPosition as libc::c_double,
                ),
            )
        };
        match () {
            #[cfg(DEBUG_EXPR)]
            _ => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"position() : %d\n\x00" as *const u8 as *const libc::c_char,
                        (*safe_ctxt.context).proximityPosition,
                    )
                };
            }
            #[cfg(not(DEBUG_EXPR))]
            _ => {}
        };
    } else {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_CTXT_POSITION as libc::c_int) };
        return;
    };
}
/* *
 * xmlXPathCountFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the count() XPath function
 *    number count(node-set) */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathCountFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NODESET as libc::c_int as libc::c_uint
            && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    if cur.is_null() || unsafe { (*cur).nodesetval.is_null() } {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewFloat(safe_ctxt.context, 0 as libc::c_int as libc::c_double),
            )
        };
    } else {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewFloat(
                    safe_ctxt.context,
                    (*(*cur).nodesetval).nodeNr as libc::c_double,
                ),
            )
        };
    }
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, cur) };
}
/* *
 * xmlXPathGetElementsByIds: * @doc: the document
 * @ids: a whitespace separated list of IDs
 *
 * Selects elements by their unique ID.
 *
 * Returns a node-set of selected elements.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathGetElementsByIds(mut doc: xmlDocPtr, mut ids: *const xmlChar) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut cur: *const xmlChar = ids;
    let mut ID: *mut xmlChar = 0 as *mut xmlChar;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut elem: xmlNodePtr = 0 as xmlNodePtr;
    if ids.is_null() {
        return 0 as xmlNodeSetPtr;
    }
    ret = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
    if ret.is_null() {
        return ret;
    }
    while unsafe {
        *cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *cur as libc::c_int
                && *cur as libc::c_int <= 0xa as libc::c_int
            || *cur as libc::c_int == 0xd as libc::c_int
    } {
        unsafe { cur = cur.offset(1) }
    }
    while unsafe { *cur as libc::c_int != 0 as libc::c_int } {
        while !unsafe {
            (*cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *cur as libc::c_int
                    && *cur as libc::c_int <= 0xa as libc::c_int
                || *cur as libc::c_int == 0xd as libc::c_int)
                && *cur as libc::c_int != 0 as libc::c_int
        } {
            unsafe { cur = cur.offset(1) }
        }
        ID = unsafe { xmlStrndup(ids, cur.offset_from(ids) as libc::c_long as libc::c_int) };
        if !ID.is_null() {
            /*
             * We used to check the fact that the value passed
             * was an NCName, but this generated much troubles for
             * me and Aleksey Sanin, people blatantly violated that
             * constraint, like Visa3D spec.
             * if (xmlValidateNCName(ID, 1) == 0)
             */
            attr = unsafe { xmlGetID(doc, ID) };
            let safe_attr = unsafe { &mut *attr };
            if !attr.is_null() {
                if safe_attr.type_0 as libc::c_uint
                    == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
                {
                    elem = safe_attr.parent
                } else if safe_attr.type_0 as libc::c_uint
                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                {
                    elem = attr as xmlNodePtr
                } else {
                    elem = 0 as xmlNodePtr
                }
                /* TODO: Check memory error. */
                if !elem.is_null() {
                    unsafe { xmlXPathNodeSetAdd(ret, elem) };
                }
            }
            unsafe { xmlFree.expect("non-null function pointer")(ID as *mut libc::c_void) };
        }
        while unsafe {
            *cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *cur as libc::c_int
                    && *cur as libc::c_int <= 0xa as libc::c_int
                || *cur as libc::c_int == 0xd as libc::c_int
        } {
            unsafe { cur = cur.offset(1) }
        }
        ids = cur
    }
    return ret;
}
/* *
 * xmlXPathIdFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the id() XPath function
 *    node-set id(object) * The id function selects elements by their unique ID
 * (see [5.2.1 Unique IDs]). When the argument to id is of type node-set, * then the result is the union of the result of applying id to the
 * string value of each of the nodes in the argument node-set. When the
 * argument to id is of any other type, the argument is converted to a
 * string as if by a call to the string function; the string is split
 * into a whitespace-separated list of tokens (whitespace is any sequence
 * of characters matching the production S); the result is a node-set
 * containing the elements in the same document as the context node that
 * have a unique ID equal to any of the tokens in the list.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathIdFunction(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    let mut tokens: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    obj = unsafe { valuePop(ctxt) };
    if obj.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return;
    }
    let safe_obj = unsafe { &mut *obj };
    if safe_obj.type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || safe_obj.type_0 as libc::c_uint == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
        let mut i: libc::c_int = 0;
        /* TODO: Check memory error. */
        ret = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
        if !safe_obj.nodesetval.is_null() {
            i = 0 as libc::c_int;
            while i < unsafe { (*safe_obj.nodesetval).nodeNr } {
                tokens = unsafe {
                    xmlXPathCastNodeToString(*(*safe_obj.nodesetval).nodeTab.offset(i as isize))
                };
                ns = unsafe { xmlXPathGetElementsByIds((*safe_ctxt.context).doc, tokens) };
                /* TODO: Check memory error. */
                ret = unsafe { xmlXPathNodeSetMerge(ret, ns) };
                unsafe { xmlXPathFreeNodeSet(ns) };
                if !tokens.is_null() {
                    unsafe {
                        xmlFree.expect("non-null function pointer")(tokens as *mut libc::c_void)
                    };
                }
                i += 1
            }
        }
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, obj) };
        unsafe { valuePush(ctxt, xmlXPathCacheWrapNodeSet(safe_ctxt.context, ret)) };
        return;
    }
    obj = unsafe { xmlXPathCacheConvertString(safe_ctxt.context, obj) };
    if obj.is_null() {
        return;
    }
    ret = unsafe { xmlXPathGetElementsByIds((*safe_ctxt.context).doc, safe_obj.stringval) };
    unsafe { valuePush(ctxt, xmlXPathCacheWrapNodeSet(safe_ctxt.context, ret)) };
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, obj) };
}
/* *
 * xmlXPathLocalNameFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the local-name() XPath function
 *    string local-name(node-set?) * The local-name function returns a string containing the local part
 * of the name of the node in the argument node-set that is first in
 * document order. If the node-set is empty or the first node has no
 * name, an empty string is returned. If the argument is omitted it
 * defaults to the context node.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathLocalNameFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject; /* Should be first in document order !!!!! */
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 as libc::c_int {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
            )
        };
        nargs = 1 as libc::c_int
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NODESET as libc::c_int as libc::c_uint
            && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    if unsafe { (*cur).nodesetval.is_null() }
        || unsafe { (*(*cur).nodesetval).nodeNr == 0 as libc::c_int }
    {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewCString(
                    safe_ctxt.context,
                    b"\x00" as *const u8 as *const libc::c_char,
                ),
            )
        };
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        match unsafe { (**(*(*cur).nodesetval).nodeTab.offset(i as isize)).type_0 as libc::c_uint }
        {
            1 | 2 | 7 => {
                if unsafe {
                    *(**(*(*cur).nodesetval).nodeTab.offset(i as isize))
                        .name
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        == ' ' as i32
                } {
                    unsafe {
                        valuePush(
                            ctxt,
                            xmlXPathCacheNewCString(
                                safe_ctxt.context,
                                b"\x00" as *const u8 as *const libc::c_char,
                            ),
                        )
                    };
                } else {
                    unsafe {
                        valuePush(
                            ctxt,
                            xmlXPathCacheNewString(
                                safe_ctxt.context,
                                (**(*(*cur).nodesetval).nodeTab.offset(i as isize)).name,
                            ),
                        )
                    };
                }
            }
            18 => {
                unsafe {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewString(
                            safe_ctxt.context,
                            (*(*(*(*cur).nodesetval).nodeTab.offset(i as isize) as xmlNsPtr))
                                .prefix,
                        ),
                    )
                };
            }
            _ => {
                unsafe {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewCString(
                            safe_ctxt.context,
                            b"\x00" as *const u8 as *const libc::c_char,
                        ),
                    )
                };
            }
        }
    }
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, cur) };
}
/* *
 * xmlXPathNamespaceURIFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the namespace-uri() XPath function
 *    string namespace-uri(node-set?) * The namespace-uri function returns a string containing the
 * namespace URI of the expanded name of the node in the argument
 * node-set that is first in document order. If the node-set is empty, * the first node has no name, or the expanded name has no namespace
 * URI, an empty string is returned. If the argument is omitted it
 * defaults to the context node.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNamespaceURIFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject; /* Should be first in document order !!!!! */
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as libc::c_int {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
            )
        };
        nargs = 1 as libc::c_int
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NODESET as libc::c_int as libc::c_uint
            && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    let safe_cur = unsafe { &mut *cur };
    if safe_cur.nodesetval.is_null() || unsafe { (*safe_cur.nodesetval).nodeNr == 0 as libc::c_int }
    {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewCString(
                    safe_ctxt.context,
                    b"\x00" as *const u8 as *const libc::c_char,
                ),
            )
        };
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        match unsafe {
            (**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).type_0 as libc::c_uint
        } {
            1 | 2 => {
                if unsafe {
                    (**(*safe_cur.nodesetval).nodeTab.offset(i as isize))
                        .ns
                        .is_null()
                } {
                    unsafe {
                        valuePush(
                            ctxt,
                            xmlXPathCacheNewCString(
                                safe_ctxt.context,
                                b"\x00" as *const u8 as *const libc::c_char,
                            ),
                        )
                    };
                } else {
                    unsafe {
                        valuePush(
                            ctxt,
                            xmlXPathCacheNewString(
                                safe_ctxt.context,
                                (*(**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).ns).href,
                            ),
                        )
                    };
                }
            }
            _ => {
                unsafe {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewCString(
                            safe_ctxt.context,
                            b"\x00" as *const u8 as *const libc::c_char,
                        ),
                    )
                };
            }
        }
    }
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, cur) };
}
/* *
 * xmlXPathNameFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the name() XPath function
 *    string name(node-set?) * The name function returns a string containing a QName representing
 * the name of the node in the argument node-set that is first in document
 * order. The QName must represent the name with respect to the namespace
 * declarations in effect on the node whose name is being represented.
 * Typically, this will be the form in which the name occurred in the XML
 * source. This need not be the case if there are namespace declarations
 * in effect on the node that associate multiple prefixes with the same
 * namespace. However, an implementation may include information about
 * the original prefix in its representation of nodes; in this case, an
 * implementation can ensure that the returned string is always the same
 * as the QName used in the XML source. If the argument it omitted it
 * defaults to the context node.
 * Libxml keep the original prefix so the "real qualified name" used is
 * returned.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathNameFunction(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject; /* Should be first in document order !!!!! */
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 as libc::c_int {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
            )
        };
        nargs = 1 as libc::c_int
    }

    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NODESET as libc::c_int as libc::c_uint
            && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    let safe_cur = unsafe { &mut *cur };
    if safe_cur.nodesetval.is_null() || unsafe { (*safe_cur.nodesetval).nodeNr == 0 as libc::c_int }
    {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewCString(
                    safe_ctxt.context,
                    b"\x00" as *const u8 as *const libc::c_char,
                ),
            )
        };
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        match unsafe {
            (**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).type_0 as libc::c_uint
        } {
            1 | 2 => {
                if unsafe {
                    *(**(*safe_cur.nodesetval).nodeTab.offset(i as isize))
                        .name
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        == ' ' as i32
                } {
                    unsafe {
                        valuePush(
                            ctxt,
                            xmlXPathCacheNewCString(
                                safe_ctxt.context,
                                b"\x00" as *const u8 as *const libc::c_char,
                            ),
                        )
                    };
                } else if unsafe {
                    (**(*safe_cur.nodesetval).nodeTab.offset(i as isize))
                        .ns
                        .is_null()
                        || (*(**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).ns)
                            .prefix
                            .is_null()
                } {
                    unsafe {
                        valuePush(
                            ctxt,
                            xmlXPathCacheNewString(
                                safe_ctxt.context,
                                (**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).name,
                            ),
                        )
                    };
                } else {
                    let mut fullname: *mut xmlChar = 0 as *mut xmlChar;
                    fullname = unsafe {
                        xmlBuildQName(
                            (**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).name,
                            (*(**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).ns).prefix,
                            0 as *mut xmlChar,
                            0 as libc::c_int,
                        )
                    };
                    if fullname
                        == unsafe {
                            (**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).name
                                as *mut xmlChar
                        }
                    {
                        fullname = unsafe {
                            xmlStrdup((**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).name)
                        }
                    }
                    if fullname.is_null() {
                        unsafe { xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as libc::c_int) };
                        return;
                    }
                    unsafe {
                        valuePush(ctxt, xmlXPathCacheWrapString(safe_ctxt.context, fullname))
                    };
                }
            }
            _ => {
                unsafe {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewNodeSet(
                            safe_ctxt.context,
                            *(*safe_cur.nodesetval).nodeTab.offset(i as isize),
                        ),
                    )
                };
                unsafe { xmlXPathLocalNameFunction(ctxt, 1 as libc::c_int) };
            }
        }
    }
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, cur) };
}
/* *
 * xmlXPathStringFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the string() XPath function
 *    string string(object?) * The string function converts an object to a string as follows: *    - A node-set is converted to a string by returning the value of
 *      the node in the node-set that is first in document order.
 *      If the node-set is empty, an empty string is returned.
 *    - A number is converted to a string as follows
 *      + NaN is converted to the string NaN
 *      + positive zero is converted to the string 0
 *      + negative zero is converted to the string 0
 *      + positive infinity is converted to the string Infinity
 *      + negative infinity is converted to the string -Infinity
 *      + if the number is an integer, the number is represented in
 *        decimal form as a Number with no decimal point and no leading
 *        zeros, preceded by a minus sign (-) if the number is negative
 *      + otherwise, the number is represented in decimal form as a
 *        Number including a decimal point with at least one digit
 *        before the decimal point and at least one digit after the
 *        decimal point, preceded by a minus sign (-) if the number
 *        is negative; there must be no leading zeros before the decimal
 *        point apart possibly from the one required digit immediately
 *        before the decimal point; beyond the one required digit
 *        after the decimal point there must be as many, but only as *        many, more digits as are needed to uniquely distinguish the
 *        number from all other IEEE 754 numeric values.
 *    - The boolean false value is converted to the string false.
 *      The boolean true value is converted to the string true.
 *
 * If the argument is omitted, it defaults to a node-set with the
 * context node as its only member.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathStringFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 as libc::c_int {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheWrapString(
                    safe_ctxt.context,
                    xmlXPathCastNodeToString((*safe_ctxt.context).node),
                ),
            )
        };
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    if cur.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return;
    }
    unsafe { valuePush(ctxt, xmlXPathCacheConvertString(safe_ctxt.context, cur)) };
}
/* *
 * xmlXPathStringLengthFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the string-length() XPath function
 *    number string-length(string?) * The string-length returns the number of characters in the string
 * (see [3.6 Strings]). If the argument is omitted, it defaults to
 * the context node converted to a string, in other words the value
 * of the context node.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathStringLengthFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 as libc::c_int {
        if ctxt.is_null() || safe_ctxt.context.is_null() {
            return;
        }
        if unsafe { (*safe_ctxt.context).node.is_null() } {
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewFloat(safe_ctxt.context, 0 as libc::c_int as libc::c_double),
                )
            };
        } else {
            let mut content: *mut xmlChar = 0 as *mut xmlChar;
            content = unsafe { xmlXPathCastNodeToString((*safe_ctxt.context).node) };
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewFloat(
                        safe_ctxt.context,
                        xmlUTF8Strlen(content) as libc::c_double,
                    ),
                )
            };
            unsafe { xmlFree.expect("non-null function pointer")(content as *mut libc::c_void) };
        }
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    unsafe {
        valuePush(
            ctxt,
            xmlXPathCacheNewFloat(
                safe_ctxt.context,
                xmlUTF8Strlen((*cur).stringval) as libc::c_double,
            ),
        )
    };
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, cur) };
}
/* *
 * xmlXPathConcatFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the concat() XPath function
 *    string concat(string, string, string*) * The concat function returns the concatenation of its arguments.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathConcatFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut newobj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs < 2 as libc::c_int {
        if ctxt.is_null() {
            return;
        }
        if nargs != 2 as libc::c_int {
            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
            return;
        }

        if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 as libc::c_int {
            unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
            return;
        }
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    cur = unsafe { valuePop(ctxt) };
    if cur.is_null()
        || unsafe { (*cur).type_0 as libc::c_uint } != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, cur) };
        return;
    }
    nargs -= 1;
    while nargs > 0 as libc::c_int {
        if !safe_ctxt.value.is_null()
            && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                != XPATH_STRING as libc::c_int as libc::c_uint
        {
            unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
        }
        newobj = unsafe { valuePop(ctxt) };
        if newobj.is_null()
            || unsafe { (*newobj).type_0 as libc::c_uint }
                != XPATH_STRING as libc::c_int as libc::c_uint
        {
            unsafe {
                xmlXPathReleaseObject(safe_ctxt.context, newobj);
                xmlXPathReleaseObject(safe_ctxt.context, cur);
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
            }
            return;
        }
        tmp = unsafe { xmlStrcat((*newobj).stringval, (*cur).stringval) };
        unsafe {
            (*newobj).stringval = (*cur).stringval;
            (*cur).stringval = tmp;
            xmlXPathReleaseObject(safe_ctxt.context, newobj);
        }
        nargs -= 1
    }
    unsafe {
        valuePush(ctxt, cur);
    }
}
/* *
 * xmlXPathContainsFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the contains() XPath function
 *    boolean contains(string, string) * The contains function returns true if the first argument string
 * contains the second argument string, and otherwise returns false.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathContainsFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut hay: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut needle: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    needle = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    hay = unsafe { valuePop(ctxt) };
    if hay.is_null()
        || unsafe { (*hay).type_0 as libc::c_uint } != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, hay) };
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, needle) };
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    if !unsafe { xmlStrstr((*hay).stringval, (*needle).stringval).is_null() } {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewBoolean(safe_ctxt.context, 1 as libc::c_int),
            )
        };
    } else {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewBoolean(safe_ctxt.context, 0 as libc::c_int),
            )
        };
    }
    unsafe {
        xmlXPathReleaseObject(safe_ctxt.context, hay);
        xmlXPathReleaseObject(safe_ctxt.context, needle);
    }
}
/* *
 * xmlXPathStartsWithFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the starts-with() XPath function
 *    boolean starts-with(string, string) * The starts-with function returns true if the first argument string
 * starts with the second argument string, and otherwise returns false.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathStartsWithFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut hay: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut needle: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut n: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    needle = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    hay = unsafe { valuePop(ctxt) };
    if hay.is_null()
        || unsafe { (*hay).type_0 as libc::c_uint } != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, hay) };
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, needle) };
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    n = unsafe { xmlStrlen((*needle).stringval) };
    if unsafe { xmlStrncmp((*hay).stringval, (*needle).stringval, n) } != 0 {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewBoolean(safe_ctxt.context, 0 as libc::c_int),
            )
        };
    } else {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewBoolean(safe_ctxt.context, 1 as libc::c_int),
            )
        };
    }
    unsafe {
        xmlXPathReleaseObject(safe_ctxt.context, hay);
        xmlXPathReleaseObject(safe_ctxt.context, needle);
    }
}
/* *
 * xmlXPathSubstringFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the substring() XPath function
 *    string substring(string, number, number?) * The substring function returns the substring of the first argument
 * starting at the position specified in the second argument with
 * length specified in the third argument. For example, * substring("12345",2,3) returns "234". If the third argument is not
 * specified, it returns the substring starting at the position specified
 * in the second argument and continuing to the end of the string. For
 * example, substring("12345",2) returns "2345".  More precisely, each
 * character in the string (see [3.6 Strings]) is considered to have a
 * numeric position: the position of the first character is 1, the position
 * of the second character is 2 and so on. The returned substring contains
 * those characters for which the position of the character is greater than
 * or equal to the second argument and, if the third argument is specified, * less than the sum of the second and third arguments; the comparisons
 * and addition used for the above follow the standard IEEE 754 rules. Thus: *  - substring("12345", 1.5, 2.6) returns "234"
 *  - substring("12345", 0, 3) returns "12"
 *  - substring("12345", 0 div 0, 3) returns ""
 *  - substring("12345", 1, 0 div 0) returns ""
 *  - substring("12345", -42, 1 div 0) returns "12345"
 *  - substring("12345", -1 div 0, 1 div 0) returns ""
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathSubstringFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut start: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut len: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut le: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut in_0: libc::c_double = 0.;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut j: libc::c_int = 2147483647 as libc::c_int;
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs < 2 as libc::c_int {
        if ctxt.is_null() {
            return;
        }
        if nargs != 2 as libc::c_int {
            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
            return;
        }
        if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 as libc::c_int {
            unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
            return;
        }
    }
    if nargs > 3 as libc::c_int {
        if ctxt.is_null() {
            return;
        }
        if nargs != 3 as libc::c_int {
            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
            return;
        }
        if safe_ctxt.valueNr < safe_ctxt.valueFrame + 3 as libc::c_int {
            unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
            return;
        }
    }
    /*
     * take care of possible last (position) argument
     */
    if nargs == 3 as libc::c_int {
        if !safe_ctxt.value.is_null()
            && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                != XPATH_NUMBER as libc::c_int as libc::c_uint
        {
            unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
        }
        if safe_ctxt.value.is_null()
            || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                != XPATH_NUMBER as libc::c_int as libc::c_uint
        {
            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
            return;
        }
        len = unsafe { valuePop(ctxt) };
        let safe_len = unsafe { &mut *len };
        le = safe_len.floatval;
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, len) };
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    start = unsafe { valuePop(ctxt) };
    let safe_start = unsafe { &mut *start };
    in_0 = safe_start.floatval;
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, start) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    str = unsafe { valuePop(ctxt) };
    if !(in_0 < 2147483647 as libc::c_int as libc::c_double) {
        /* Logical NOT to handle NaNs */
        i = 2147483647 as libc::c_int
    } else if in_0 >= 1.0f64 {
        i = in_0 as libc::c_int;
        if in_0 - unsafe { floor(in_0) } >= 0.5f64 {
            i += 1 as libc::c_int
        }
    }
    if nargs == 3 as libc::c_int {
        let mut rin: libc::c_double = 0.;
        let mut rle: libc::c_double = 0.;
        let mut end: libc::c_double = 0.;
        rin = unsafe { floor(in_0) };
        if in_0 - rin >= 0.5f64 {
            rin += 1.0f64
        }
        rle = unsafe { floor(le) };
        if le - rle >= 0.5f64 {
            rle += 1.0f64
        }
        end = rin + rle;
        if !(end >= 1.0f64) {
            /* Logical NOT to handle NaNs */
            j = 1 as libc::c_int
        } else if end < 2147483647 as libc::c_int as libc::c_double {
            j = end as libc::c_int
        }
    }
    if i < j {
        let mut ret: *mut xmlChar =
            unsafe { xmlUTF8Strsub((*str).stringval, i - 1 as libc::c_int, j - i) };
        unsafe { valuePush(ctxt, xmlXPathCacheNewString(safe_ctxt.context, ret)) };
        unsafe { xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void) };
    } else {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewCString(
                    safe_ctxt.context,
                    b"\x00" as *const u8 as *const libc::c_char,
                ),
            )
        };
    }
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, str) };
}
/* *
 * xmlXPathSubstringBeforeFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the substring-before() XPath function
 *    string substring-before(string, string) * The substring-before function returns the substring of the first
 * argument string that precedes the first occurrence of the second
 * argument string in the first argument string, or the empty string
 * if the first argument string does not contain the second argument
 * string. For example, substring-before("1999/04/01","/") returns 1999.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathSubstringBeforeFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut find: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut point: *const xmlChar = 0 as *const xmlChar;
    let mut offset: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs != 2 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    find = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    str = unsafe { valuePop(ctxt) };
    target = unsafe { xmlBufCreate() };
    if !target.is_null() {
        point = unsafe { xmlStrstr((*str).stringval, (*find).stringval) };
        if !point.is_null() {
            offset = unsafe { point.offset_from((*str).stringval) as libc::c_long as libc::c_int };
            unsafe { xmlBufAdd(target, (*str).stringval, offset) };
        }
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewString(safe_ctxt.context, xmlBufContent(target as *const xmlBuf)),
            )
        };
        unsafe { xmlBufFree(target) };
    }
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, str) };
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, find) };
}
/* *
 * xmlXPathSubstringAfterFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the substring-after() XPath function
 *    string substring-after(string, string) * The substring-after function returns the substring of the first
 * argument string that follows the first occurrence of the second
 * argument string in the first argument string, or the empty stringi
 * if the first argument string does not contain the second argument
 * string. For example, substring-after("1999/04/01","/") returns 04/01, * and substring-after("1999/04/01","19") returns 99/04/01.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathSubstringAfterFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut find: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut point: *const xmlChar = 0 as *const xmlChar;
    let mut offset: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs != 2 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    find = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    str = unsafe { valuePop(ctxt) };
    target = unsafe { xmlBufCreate() };
    if !target.is_null() {
        point = unsafe { xmlStrstr((*str).stringval, (*find).stringval) };
        if !point.is_null() {
            offset = unsafe {
                point.offset_from((*str).stringval) as libc::c_long as libc::c_int
                    + xmlStrlen((*find).stringval)
            };
            unsafe {
                xmlBufAdd(
                    target,
                    &mut *(*str).stringval.offset(offset as isize),
                    xmlStrlen((*str).stringval) - offset,
                )
            };
        }
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewString(safe_ctxt.context, xmlBufContent(target as *const xmlBuf)),
            )
        };
        unsafe { xmlBufFree(target) };
    }
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, str) };
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, find) };
}
/* *
 * xmlXPathNormalizeFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the normalize-space() XPath function
 *    string normalize-space(string?) * The normalize-space function returns the argument string with white
 * space normalized by stripping leading and trailing whitespace
 * and replacing sequences of whitespace characters by a single
 * space. Whitespace characters are the same allowed by the S production
 * in XML. If the argument is omitted, it defaults to the context
 * node converted to a string, in other words the value of the context node.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNormalizeFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut obj: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut source: *mut xmlChar = 0 as *mut xmlChar;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut blank: xmlChar = 0;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 as libc::c_int {
        /* Use current context node */
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheWrapString(
                    safe_ctxt.context,
                    xmlXPathCastNodeToString((*safe_ctxt.context).node),
                ),
            )
        };
        nargs = 1 as libc::c_int
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    obj = unsafe { valuePop(ctxt) };
    source = unsafe { (*obj).stringval };
    target = unsafe { xmlBufCreate() };
    if !target.is_null() && !source.is_null() {
        /* Skip leading whitespaces */
        while unsafe {
            *source as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *source as libc::c_int
                    && *source as libc::c_int <= 0xa as libc::c_int
                || *source as libc::c_int == 0xd as libc::c_int
        } {
            unsafe { source = source.offset(1) }
        }
        /* Collapse intermediate whitespaces, and skip trailing whitespaces */
        blank = 0 as libc::c_int as xmlChar;
        while unsafe { *source != 0 } {
            if unsafe {
                *source as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *source as libc::c_int
                        && *source as libc::c_int <= 0xa as libc::c_int
                    || *source as libc::c_int == 0xd as libc::c_int
            } {
                blank = 0x20 as libc::c_int as xmlChar
            } else {
                if blank != 0 {
                    unsafe { xmlBufAdd(target, &mut blank, 1 as libc::c_int) };
                    blank = 0 as libc::c_int as xmlChar
                }
                unsafe { xmlBufAdd(target, source, 1 as libc::c_int) };
            }
            unsafe { source = source.offset(1) }
        }
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewString(safe_ctxt.context, xmlBufContent(target as *const xmlBuf)),
            )
        };
        unsafe { xmlBufFree(target) };
    }
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, obj) };
}
/* *
 * xmlXPathTranslateFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the translate() XPath function
 *    string translate(string, string, string) * The translate function returns the first argument string with
 * occurrences of characters in the second argument string replaced
 * by the character at the corresponding position in the third argument
 * string. For example, translate("bar","abc","ABC") returns the string
 * BAr. If there is a character in the second argument string with no
 * character at a corresponding position in the third argument string
 * (because the second argument string is longer than the third argument
 * string), then occurrences of that character in the first argument
 * string are removed. For example, translate("--aaa--","abc-","ABC") * returns "AAA". If a character occurs more than once in second
 * argument string, then the first occurrence determines the replacement
 * character. If the third argument string is longer than the second
 * argument string, then excess characters are ignored.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathTranslateFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut from: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut to: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut offset: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut ch: xmlChar = 0;
    let mut point: *const xmlChar = 0 as *const xmlChar;
    let mut cptr: *mut xmlChar = 0 as *mut xmlChar;
    if ctxt.is_null() {
        return;
    }
    if nargs != 3 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 3 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    to = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    from = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    str = unsafe { valuePop(ctxt) };
    target = unsafe { xmlBufCreate() };
    if !target.is_null() {
        max = unsafe { xmlUTF8Strlen((*to).stringval) };
        cptr = unsafe { (*str).stringval };
        loop {
            ch = unsafe { *cptr };
            if !(ch != 0) {
                break;
            }
            offset = unsafe { xmlUTF8Strloc((*from).stringval, cptr) };
            if offset >= 0 as libc::c_int {
                if offset < max {
                    point = unsafe { xmlUTF8Strpos((*to).stringval, offset) };
                    if !point.is_null() {
                        unsafe {
                            xmlBufAdd(target, point, xmlUTF8Strsize(point, 1 as libc::c_int))
                        };
                    }
                }
            } else {
                unsafe { xmlBufAdd(target, cptr, xmlUTF8Strsize(cptr, 1 as libc::c_int)) };
            }
            /* Step to next character in input */
            unsafe {
                cptr = cptr.offset(1);
            }
            if !(ch as libc::c_int & 0x80 as libc::c_int != 0) {
                continue;
            }
            /* if not simple ascii, verify proper format */
            if ch as libc::c_int & 0xc0 as libc::c_int != 0xc0 as libc::c_int {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"xmlXPathTranslateFunction: Invalid UTF8 string\n\x00" as *const u8
                            as *const libc::c_char,
                    )
                };
                /* not asserting an XPath error is probably better */
                break;
            } else {
                loop
                /* then skip over remaining bytes for this char */
                {
                    ch = ((ch as libc::c_int) << 1 as libc::c_int) as xmlChar;
                    if !(ch as libc::c_int & 0x80 as libc::c_int != 0) {
                        break;
                    }
                    let fresh70 = cptr;
                    unsafe { cptr = cptr.offset(1) };
                    if !unsafe {
                        (*fresh70 as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int)
                    } {
                        continue;
                    }
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"xmlXPathTranslateFunction: Invalid UTF8 string\n\x00" as *const u8
                                as *const libc::c_char,
                        )
                    };
                    break;
                }
                if ch as libc::c_int & 0x80 as libc::c_int != 0 {
                    break;
                }
            }
        }
    }
    unsafe {
        valuePush(
            ctxt,
            xmlXPathCacheNewString(safe_ctxt.context, xmlBufContent(target as *const xmlBuf)),
        )
    };
    unsafe { xmlBufFree(target) };
    unsafe {
        xmlXPathReleaseObject(safe_ctxt.context, str);
        xmlXPathReleaseObject(safe_ctxt.context, from);
        xmlXPathReleaseObject(safe_ctxt.context, to);
    }
}
/* *
 * xmlXPathBooleanFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the boolean() XPath function
 *    boolean boolean(object) * The boolean function converts its argument to a boolean as follows: *    - a number is true if and only if it is neither positive or
 *      negative zero nor NaN
 *    - a node-set is true if and only if it is non-empty
 *    - a string is true if and only if its length is non-zero
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathBooleanFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    if cur.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
        return;
    }
    cur = unsafe { xmlXPathCacheConvertBoolean(safe_ctxt.context, cur) };
    unsafe { valuePush(ctxt, cur) };
}
/* *
 * xmlXPathNotFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the not() XPath function
 *    boolean not(boolean) * The not function returns true if its argument is false, * and false otherwise.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNotFunction(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_BOOLEAN as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathBooleanFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_BOOLEAN as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    unsafe { (*safe_ctxt.value).boolval = ((*safe_ctxt.value).boolval == 0) as libc::c_int };
}
/* *
 * xmlXPathTrueFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the true() XPath function
 *    boolean true() */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathTrueFunction(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 0 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    unsafe {
        valuePush(
            ctxt,
            xmlXPathCacheNewBoolean(safe_ctxt.context, 1 as libc::c_int),
        )
    };
}
/* *
 * xmlXPathFalseFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the false() XPath function
 *    boolean false() */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathFalseFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 0 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    unsafe {
        valuePush(
            ctxt,
            xmlXPathCacheNewBoolean(safe_ctxt.context, 0 as libc::c_int),
        )
    };
}
/* *
 * xmlXPathLangFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the lang() XPath function
 *    boolean lang(string) * The lang function returns true or false depending on whether the
 * language of the context node as specified by xml:lang attributes
 * is the same as or is a sublanguage of the language specified by
 * the argument string. The language of the context node is determined
 * by the value of the xml:lang attribute on the context node, or, if
 * the context node has no xml:lang attribute, by the value of the
 * xml:lang attribute on the nearest ancestor of the context node that
 * has an xml:lang attribute. If there is no such attribute, then lang
 * returns false. If there is such an attribute, then lang returns
 * true if the attribute value is equal to the argument ignoring case, * or if there is some suffix starting with - such that the attribute
 * value is equal to the argument ignoring that suffix of the attribute
 * value and ignoring case.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathLangFunction(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    let mut current_block: u64;
    let mut val: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut theLang: *const xmlChar = 0 as *const xmlChar;
    let mut lang: *const xmlChar = 0 as *const xmlChar;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    val = unsafe { valuePop(ctxt) };
    lang = unsafe { (*val).stringval };
    theLang = unsafe { xmlNodeGetLang((*safe_ctxt.context).node as *const xmlNode) };
    if !theLang.is_null() && !lang.is_null() {
        i = 0 as libc::c_int;
        loop {
            if !unsafe { (*lang.offset(i as isize) as libc::c_int != 0 as libc::c_int) } {
                current_block = 2232869372362427478;
                break;
            }
            if ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<xmlChar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int =
                            unsafe { *lang.offset(i as isize) as libc::c_int };
                        __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            unsafe { *(*__ctype_toupper_loc()).offset(__c as isize) }
                        })
                    } else {
                        __res = unsafe { toupper(*lang.offset(i as isize) as libc::c_int) }
                    }
                } else {
                    __res = unsafe {
                        *(*__ctype_toupper_loc())
                            .offset(*lang.offset(i as isize) as libc::c_int as isize)
                    }
                }
                __res
            }) != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<xmlChar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int =
                            unsafe { *theLang.offset(i as isize) as libc::c_int };
                        __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            unsafe { *(*__ctype_toupper_loc()).offset(__c as isize) }
                        })
                    } else {
                        __res = unsafe { toupper(*theLang.offset(i as isize) as libc::c_int) }
                    }
                } else {
                    __res = unsafe {
                        *(*__ctype_toupper_loc())
                            .offset(*theLang.offset(i as isize) as libc::c_int as isize)
                    }
                }
                __res
            }) {
                current_block = 14069797209531859851;
                break;
            }
            i += 1
        }
        match current_block {
            14069797209531859851 => {}
            _ => {
                if unsafe { *theLang.offset(i as isize) as libc::c_int } == 0 as libc::c_int
                    || unsafe { *theLang.offset(i as isize) as libc::c_int } == '-' as i32
                {
                    ret = 1 as libc::c_int
                }
            }
        }
    }
    if !theLang.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")(theLang as *mut libc::c_void) };
    }
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, val) };
    unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, ret)) };
}
/* *
 * xmlXPathNumberFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the number() XPath function
 *    number number(object?) */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathNumberFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut res: libc::c_double = 0.;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 as libc::c_int {
        if unsafe { (*safe_ctxt.context).node.is_null() } {
            unsafe { valuePush(ctxt, xmlXPathCacheNewFloat(safe_ctxt.context, 0.0f64)) };
        } else {
            let mut content: *mut xmlChar =
                unsafe { xmlNodeGetContent((*safe_ctxt.context).node as *const xmlNode) };
            res = xmlXPathStringEvalNumber(content);
            unsafe { valuePush(ctxt, xmlXPathCacheNewFloat(safe_ctxt.context, res)) };
            unsafe { xmlFree.expect("non-null function pointer")(content as *mut libc::c_void) };
        }
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    unsafe { valuePush(ctxt, xmlXPathCacheConvertNumber(safe_ctxt.context, cur)) };
}
/* *
 * xmlXPathSumFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the sum() XPath function
 *    number sum(node-set) * The sum function returns the sum of the values of the nodes in
 * the argument node-set.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathSumFunction(mut ctxt: xmlXPathParserContextPtr, mut nargs: libc::c_int) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut i: libc::c_int = 0;
    let mut res: libc::c_double = 0.0f64;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NODESET as libc::c_int as libc::c_uint
            && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    if !unsafe { (*cur).nodesetval.is_null() }
        && unsafe { (*(*cur).nodesetval).nodeNr != 0 as libc::c_int }
    {
        i = 0 as libc::c_int;
        while i < unsafe { (*(*cur).nodesetval).nodeNr } {
            res += unsafe {
                xmlXPathCastNodeToNumber(*(*(*cur).nodesetval).nodeTab.offset(i as isize))
            };
            i += 1
        }
    }
    unsafe { valuePush(ctxt, xmlXPathCacheNewFloat(safe_ctxt.context, res)) };
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, cur) };
}
/* *
 * xmlXPathFloorFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the floor() XPath function
 *    number floor(number) * The floor function returns the largest (closest to positive infinity) * number that is not greater than the argument and that is an integer.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathFloorFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    unsafe {
        (*safe_ctxt.value).floatval = floor((*safe_ctxt.value).floatval);
    }
}
/* *
 * xmlXPathCeilingFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the ceiling() XPath function
 *    number ceiling(number) * The ceiling function returns the smallest (closest to negative infinity) * number that is not less than the argument and that is an integer.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathCeilingFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    let f = unsafe { ceil((*safe_ctxt.value).floatval) };
    match () {
        #[cfg(AIX)]
        _ => unsafe {
            (*safe_ctxt.value).floatval = f.copysign((*safe_ctxt.value).floatval);
        },
        #[cfg(not(AIX))]
        _ => unsafe {
            (*safe_ctxt.value).floatval = f;
        },
    };
}
/* *
 * xmlXPathRoundFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the round() XPath function
 *    number round(number) * The round function returns the number that is closest to the
 * argument and that is an integer. If there are two such numbers, * then the one that is closest to positive infinity is returned.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathRoundFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut f: libc::c_double = 0.;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs != 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return;
    }
    f = unsafe { (*safe_ctxt.value).floatval };
    if f >= -0.5f64 && f < 0.5f64 {
        /* Handles negative zero. */
        unsafe { (*safe_ctxt.value).floatval *= 0.0f64 }
    } else {
        let mut rounded: libc::c_double = unsafe { floor(f) };
        if f - rounded >= 0.5f64 {
            rounded += 1.0f64
        }
        unsafe { (*safe_ctxt.value).floatval = rounded }
    };
}
/* *
 * xmlXPathCurrentChar: * @ctxt: the XPath parser context
 * @cur: pointer to the beginning of the char
 * @len: pointer to the length of the char read
 *
 * The current char value, if using UTF-8 this may actually span multiple
 * bytes in the input buffer.
 *
 * Returns the current char value and its length
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCurrentChar(
    mut ctxt: xmlXPathParserContextPtr,
    mut len: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_uchar = 0;
    let mut val: libc::c_uint = 0;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    if ctxt.is_null() {
        return 0 as libc::c_int;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    cur = safe_ctxt.cur;
    /*
     * We are supposed to handle UTF8, check it's valid
     * From rfc2044: encoding of the Unicode values on UTF-8: *
     * UCS-4 range (hex.) UTF-8 octet sequence (binary) * 0000 0000-0000 007F   0xxxxxxx
     * 0000 0080-0000 07FF   110xxxxx 10xxxxxx
     * 0000 0800-0000 FFFF   1110xxxx 10xxxxxx 10xxxxxx
     *
     * Check for the 0x110000 limit too
     */
    c = unsafe { *cur };
    if c as libc::c_int & 0x80 as libc::c_int != 0 {
        if !unsafe {
            (*cur.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                != 0x80 as libc::c_int)
        } {
            if c as libc::c_int & 0xe0 as libc::c_int == 0xe0 as libc::c_int {
                if unsafe { *cur.offset(2 as libc::c_int as isize) as libc::c_int }
                    & 0xc0 as libc::c_int
                    != 0x80 as libc::c_int
                {
                    current_block = 79219340423750421;
                } else if c as libc::c_int & 0xf0 as libc::c_int == 0xf0 as libc::c_int {
                    if c as libc::c_int & 0xf8 as libc::c_int != 0xf0 as libc::c_int
                        || unsafe {
                            *cur.offset(3 as libc::c_int as isize) as libc::c_int
                                & 0xc0 as libc::c_int
                        } != 0x80 as libc::c_int
                    {
                        current_block = 79219340423750421;
                    } else {
                        /* 4-byte code */
                        unsafe { *len = 4 as libc::c_int };
                        val = unsafe {
                            ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                                & 0x7 as libc::c_int)
                                << 18 as libc::c_int) as libc::c_uint
                        };
                        val |= unsafe {
                            ((*cur.offset(1 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int)
                                << 12 as libc::c_int) as libc::c_uint
                        };
                        val |= unsafe {
                            ((*cur.offset(2 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int)
                                << 6 as libc::c_int) as libc::c_uint
                        };
                        val |= unsafe {
                            (*cur.offset(3 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int) as libc::c_uint
                        };
                        current_block = 10043043949733653460;
                    }
                } else {
                    /* 3-byte code */
                    unsafe { *len = 3 as libc::c_int };
                    val = unsafe {
                        ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                            & 0xf as libc::c_int)
                            << 12 as libc::c_int) as libc::c_uint
                    };
                    val |= unsafe {
                        ((*cur.offset(1 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int)
                            << 6 as libc::c_int) as libc::c_uint
                    };
                    val |= unsafe {
                        (*cur.offset(2 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int) as libc::c_uint
                    };
                    current_block = 10043043949733653460;
                }
            } else {
                /* 2-byte code */
                unsafe { *len = 2 as libc::c_int };
                val = unsafe {
                    ((*cur.offset(0 as libc::c_int as isize) as libc::c_int & 0x1f as libc::c_int)
                        << 6 as libc::c_int) as libc::c_uint
                };
                val |= unsafe {
                    (*cur.offset(1 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int)
                        as libc::c_uint
                };
                current_block = 10043043949733653460;
            }
            match current_block {
                79219340423750421 => {}
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
                        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_CHAR_ERROR as libc::c_int) };
                        return 0 as libc::c_int;
                    }
                    return val as libc::c_int;
                }
            }
        }
        /*
         * If we detect an UTF8 error that probably means that the
         * input encoding didn't get properly advertised in the
         * declaration header. Report the error and switch the encoding
         * to ISO-Latin-1 (if you don't like this policy, just declare the
         * encoding !) */
        unsafe { *len = 0 as libc::c_int };
        unsafe { xmlXPathErr(ctxt, XPATH_ENCODING_ERROR as libc::c_int) };
        return 0 as libc::c_int;
    } else {
        /* 1-byte code */
        unsafe { *len = 1 as libc::c_int };
        return unsafe { *cur as libc::c_int };
    };
}
/* *
 * xmlXPathParseNCName: * @ctxt: the XPath Parser context
 *
 * parse an XML namespace non qualified name.
 *
 * [NS 3] NCName ::= (Letter | '_') (NCNameChar)*
 *
 * [NS 4] NCNameChar ::= Letter | Digit | '.' | '-' | '_' |
 *                       CombiningChar | Extender
 *
 * Returns the namespace name or NULL
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathParseNCName(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut count: libc::c_int = 0 as libc::c_int;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.cur.is_null() {
        return 0 as *mut xmlChar;
    }
    /*
     * Accelerator for simple ASCII names
     */
    in_0 = safe_ctxt.cur;
    if unsafe {
        *in_0 as libc::c_int >= 0x61 as libc::c_int && *in_0 as libc::c_int <= 0x7a as libc::c_int
            || *in_0 as libc::c_int >= 0x41 as libc::c_int
                && *in_0 as libc::c_int <= 0x5a as libc::c_int
            || *in_0 as libc::c_int == '_' as i32
    } {
        unsafe { in_0 = in_0.offset(1) };
        while unsafe {
            *in_0 as libc::c_int >= 0x61 as libc::c_int
                && *in_0 as libc::c_int <= 0x7a as libc::c_int
                || *in_0 as libc::c_int >= 0x41 as libc::c_int
                    && *in_0 as libc::c_int <= 0x5a as libc::c_int
                || *in_0 as libc::c_int >= 0x30 as libc::c_int
                    && *in_0 as libc::c_int <= 0x39 as libc::c_int
                || *in_0 as libc::c_int == '_' as i32
                || *in_0 as libc::c_int == '.' as i32
                || *in_0 as libc::c_int == '-' as i32
        } {
            unsafe { in_0 = in_0.offset(1) }
        }
        if unsafe {
            *in_0 as libc::c_int == ' ' as i32
                || *in_0 as libc::c_int == '>' as i32
                || *in_0 as libc::c_int == '/' as i32
                || *in_0 as libc::c_int == '[' as i32
                || *in_0 as libc::c_int == ']' as i32
                || *in_0 as libc::c_int == ':' as i32
                || *in_0 as libc::c_int == '@' as i32
                || *in_0 as libc::c_int == '*' as i32
        } {
            count = unsafe { in_0.offset_from(safe_ctxt.cur) as libc::c_long as libc::c_int };
            if count == 0 as libc::c_int {
                return 0 as *mut xmlChar;
            }
            ret = unsafe { xmlStrndup(safe_ctxt.cur, count) };
            safe_ctxt.cur = in_0;
            return ret;
        }
    }
    return xmlXPathParseNameComplex(ctxt, 0 as libc::c_int);
}
/* *
 * xmlXPathParseQName: * @ctxt: the XPath Parser context
 * @prefix: a xmlChar **
 *
 * parse an XML qualified name
 *
 * [NS 5] QName ::= (Prefix ':')? LocalPart
 *
 * [NS 6] Prefix ::= NCName
 *
 * [NS 7] LocalPart ::= NCName
 *
 * Returns the function returns the local part, and prefix is updated
 *   to get the Prefix if any.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathParseQName(
    mut ctxt: xmlXPathParserContextPtr,
    mut prefix: *mut *mut xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    unsafe { *prefix = 0 as *mut xmlChar };
    ret = unsafe { xmlXPathParseNCName(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ret.is_null() && unsafe { *safe_ctxt.cur as libc::c_int == ':' as i32 } {
        unsafe {
            *prefix = ret;
        }
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        ret = unsafe { xmlXPathParseNCName(ctxt) }
    }
    return ret;
}
/* *
 * xmlXPathParseName: * @ctxt: the XPath Parser context
 *
 * parse an XML name
 *
 * [4] NameChar ::= Letter | Digit | '.' | '-' | '_' | ':' |
 *                  CombiningChar | Extender
 *
 * [5] Name ::= (Letter | '_' | ':') (NameChar)*
 *
 * Returns the namespace name or NULL
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathParseName(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut count: size_t = 0 as libc::c_int as size_t;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.cur.is_null() {
        return 0 as *mut xmlChar;
    }
    /*
     * Accelerator for simple ASCII names
     */
    in_0 = safe_ctxt.cur;
    if unsafe {
        *in_0 as libc::c_int >= 0x61 as libc::c_int && *in_0 as libc::c_int <= 0x7a as libc::c_int
            || *in_0 as libc::c_int >= 0x41 as libc::c_int
                && *in_0 as libc::c_int <= 0x5a as libc::c_int
            || *in_0 as libc::c_int == '_' as i32
            || *in_0 as libc::c_int == ':' as i32
    } {
        unsafe { in_0 = in_0.offset(1) };
        while unsafe {
            *in_0 as libc::c_int >= 0x61 as libc::c_int
                && *in_0 as libc::c_int <= 0x7a as libc::c_int
                || *in_0 as libc::c_int >= 0x41 as libc::c_int
                    && *in_0 as libc::c_int <= 0x5a as libc::c_int
                || *in_0 as libc::c_int >= 0x30 as libc::c_int
                    && *in_0 as libc::c_int <= 0x39 as libc::c_int
                || *in_0 as libc::c_int == '_' as i32
                || *in_0 as libc::c_int == '-' as i32
                || *in_0 as libc::c_int == ':' as i32
                || *in_0 as libc::c_int == '.' as i32
        } {
            unsafe { in_0 = in_0.offset(1) }
        }
        if unsafe {
            *in_0 as libc::c_int > 0 as libc::c_int && (*in_0 as libc::c_int) < 0x80 as libc::c_int
        } {
            count = unsafe { in_0.offset_from(safe_ctxt.cur) as libc::c_long as size_t };
            if count > 50000 as libc::c_int as libc::c_ulong {
                safe_ctxt.cur = in_0;
                unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
                return 0 as *mut xmlChar;
            }
            ret = unsafe { xmlStrndup(safe_ctxt.cur, count as libc::c_int) };
            safe_ctxt.cur = in_0;
            return ret;
        }
    }
    return unsafe { xmlXPathParseNameComplex(ctxt, 1 as libc::c_int) };
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathParseNameComplex(
    mut ctxt: xmlXPathParserContextPtr,
    mut qualified: libc::c_int,
) -> *mut xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    /*
     * Handler for more complex cases
     */
    c = unsafe { xmlXPathCurrentChar(ctxt, &mut l) };
    if c == ' ' as i32
        || c == '>' as i32
        || c == '/' as i32
        || c == '[' as i32
        || c == ']' as i32
        || c == '@' as i32
        || c == '*' as i32
        || !((if c < 0x100 as libc::c_int {
            (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
                || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
                || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
                || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
                || 0xf8 as libc::c_int <= c) as libc::c_int
        } else {
            unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup) }
        }) != 0
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                    || c == 0x3007 as libc::c_int
                    || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                    as libc::c_int
            }) != 0)
            && c != '_' as i32
            && (qualified == 0 || c != ':' as i32)
    {
        return 0 as *mut xmlChar;
    }
    while c != ' ' as i32
        && c != '>' as i32
        && c != '/' as i32
        && ((if c < 0x100 as libc::c_int {
            (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
                || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
                || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
                || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
                || 0xf8 as libc::c_int <= c) as libc::c_int
        } else {
            unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup) }
        }) != 0
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                    || c == 0x3007 as libc::c_int
                    || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                    as libc::c_int
            }) != 0
            || (if c < 0x100 as libc::c_int {
                (0x30 as libc::c_int <= c && c <= 0x39 as libc::c_int) as libc::c_int
            } else {
                unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsDigitGroup) }
            }) != 0
            || c == '.' as i32
            || c == '-' as i32
            || c == '_' as i32
            || qualified != 0 && c == ':' as i32
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsCombiningGroup) }
            }) != 0
            || (if c < 0x100 as libc::c_int {
                (c == 0xb7 as libc::c_int) as libc::c_int
            } else {
                unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsExtenderGroup) }
            }) != 0)
    {
        if l == 1 as libc::c_int {
            let fresh71 = len;
            len = len + 1;
            buf[fresh71 as usize] = c as xmlChar
        } else {
            len += unsafe { xmlCopyChar(l, &mut *buf.as_mut_ptr().offset(len as isize), c) }
        }
        unsafe { (*ctxt).cur = (*ctxt).cur.offset(l as isize) };
        c = unsafe { xmlXPathCurrentChar(ctxt, &mut l) };
        if len >= 100 as libc::c_int {
            /*
             * Okay someone managed to make a huge name, so he's ready to pay
             * for the processing speed.
             */
            let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
            let mut max: libc::c_int = len * 2 as libc::c_int;
            if len > 50000 as libc::c_int {
                unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
                return 0 as *mut xmlChar;
            }
            buffer = unsafe {
                xmlMallocAtomic.expect("non-null function pointer")(
                    (max as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<xmlChar>() as libc::c_ulong),
                ) as *mut xmlChar
            };
            if buffer.is_null() {
                unsafe { xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as libc::c_int) };
                return 0 as *mut xmlChar;
            }
            unsafe {
                memcpy(
                    buffer as *mut libc::c_void,
                    buf.as_mut_ptr() as *const libc::c_void,
                    len as libc::c_ulong,
                )
            };
            while (if c < 0x100 as libc::c_int {
                (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
                    || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
                    || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
                    || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
                    || 0xf8 as libc::c_int <= c) as libc::c_int
            } else {
                unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup) }
            }) != 0
                || (if c < 0x100 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                        || c == 0x3007 as libc::c_int
                        || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                        as libc::c_int
                }) != 0
                || (if c < 0x100 as libc::c_int {
                    (0x30 as libc::c_int <= c && c <= 0x39 as libc::c_int) as libc::c_int
                } else {
                    unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsDigitGroup) }
                }) != 0
                || c == '.' as i32
                || c == '-' as i32
                || c == '_' as i32
                || qualified != 0 && c == ':' as i32
                || (if c < 0x100 as libc::c_int {
                    0 as libc::c_int
                } else {
                    unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsCombiningGroup) }
                }) != 0
                || (if c < 0x100 as libc::c_int {
                    (c == 0xb7 as libc::c_int) as libc::c_int
                } else {
                    unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsExtenderGroup) }
                }) != 0
            {
                if len + 10 as libc::c_int > max {
                    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                    if max > 50000 as libc::c_int {
                        unsafe {
                            xmlFree.expect("non-null function pointer")(buffer as *mut libc::c_void)
                        };
                        unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
                        return 0 as *mut xmlChar;
                    }
                    max *= 2 as libc::c_int;
                    tmp = unsafe {
                        xmlRealloc.expect("non-null function pointer")(
                            buffer as *mut libc::c_void,
                            (max as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<xmlChar>() as libc::c_ulong),
                        ) as *mut xmlChar
                    };
                    if tmp.is_null() {
                        unsafe {
                            xmlFree.expect("non-null function pointer")(buffer as *mut libc::c_void)
                        };
                        unsafe { xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as libc::c_int) };
                        return 0 as *mut xmlChar;
                    }
                    buffer = tmp
                }
                if l == 1 as libc::c_int {
                    let fresh72 = len;
                    len = len + 1;
                    unsafe { *buffer.offset(fresh72 as isize) = c as xmlChar }
                } else {
                    len += unsafe { xmlCopyChar(l, &mut *buffer.offset(len as isize), c) }
                }
                unsafe { (*ctxt).cur = (*ctxt).cur.offset(l as isize) };
                c = unsafe { xmlXPathCurrentChar(ctxt, &mut l) }
            }
            unsafe { *buffer.offset(len as isize) = 0 as libc::c_int as xmlChar };
            return buffer;
        }
    }
    if len == 0 as libc::c_int {
        return 0 as *mut xmlChar;
    }
    return unsafe { xmlStrndup(buf.as_mut_ptr(), len) };
}
/*
 * Existing functions.
 */
/* *
 * xmlXPathStringEvalNumber: * @str: A string to scan
 *
 *  [30a]  Float  ::= Number ('e' Digits?)?
 *
 *  [30]   Number ::= Digits ('.' Digits?)?
 *                    | '.' Digits
 *  [31]   Digits ::= [0-9]+ *
 * Compile a Number in the string
 * In complement of the Number expression, this function also handles
 * negative values : '-' Number.
 *
 * Returns the double value.
 */

pub fn xmlXPathStringEvalNumber(mut str: *const xmlChar) -> libc::c_double {
    let mut cur: *const xmlChar = str;
    let mut ret: libc::c_double = 0.;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut isneg: libc::c_int = 0 as libc::c_int;
    let mut exponent: libc::c_int = 0 as libc::c_int;
    let mut is_exponent_negative: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut temp: libc::c_double = 0.;
    match () {
        #[cfg(GUNC)]
        _ => {
            tmp = 0 as libc::c_int as libc::c_ulong;
            temp = 0.;
        }
        #[cfg(not(GUNC))]
        _ => {}
    };
    if cur.is_null() {
        return 0 as libc::c_int as libc::c_double;
    }
    while unsafe {
        *cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *cur as libc::c_int
                && *cur as libc::c_int <= 0xa as libc::c_int
            || *cur as libc::c_int == 0xd as libc::c_int
    } {
        unsafe { cur = cur.offset(1) }
    }
    if unsafe {
        *cur as libc::c_int != '.' as i32
            && ((*cur as libc::c_int) < '0' as i32 || *cur as libc::c_int > '9' as i32)
            && *cur as libc::c_int != '-' as i32
    } {
        unsafe { return xmlXPathNAN }
    }
    if unsafe { *cur as libc::c_int == '-' as i32 } {
        isneg = 1 as libc::c_int;
        unsafe { cur = cur.offset(1) }
    }
    /*
     * tmp/temp is a workaround against a gcc compiler bug
     * http://veillard.com/gcc.bug
     */

    match () {
        #[cfg(GUNC)]
        _ => {
            ret = 0 as libc::c_int as libc::c_double;
            while unsafe { *cur as libc::c_int >= '0' as i32 && *cur as libc::c_int <= '9' as i32 }
            {
                ret = ret * 10 as libc::c_int as libc::c_double;
                tmp = unsafe { (*cur as libc::c_int - '0' as i32) as libc::c_ulong };
                ok = 1 as libc::c_int;
                unsafe { cur = cur.offset(1) };
                temp = tmp as libc::c_double;
                ret = ret + temp
            }
        }
        #[cfg(not(GUNC))]
        _ => {
            ret = 0 as libc::c_int as libc::c_double;
            while unsafe { *cur as libc::c_int >= '0' as i32 && *cur as libc::c_int <= '9' as i32 }
            {
                ret = ret * 10 as libc::c_int as libc::c_double;
                tmp = unsafe { (*cur as libc::c_int - '0' as i32) as libc::c_ulong };
                ok = 1 as libc::c_int;
                unsafe { cur = cur.offset(1) };
                temp = tmp as libc::c_double;
                ret = ret + temp
            }
        }
    };
    if unsafe { *cur as libc::c_int == '.' as i32 } {
        let mut v: libc::c_int = 0;
        let mut frac: libc::c_int = 0 as libc::c_int;
        let mut max: libc::c_int = 0;
        let mut fraction: libc::c_double = 0 as libc::c_int as libc::c_double;
        unsafe { cur = cur.offset(1) };
        if unsafe {
            ((*cur as libc::c_int) < '0' as i32 || *cur as libc::c_int > '9' as i32) && ok == 0
        } {
            unsafe { return xmlXPathNAN }
        }
        while unsafe { *cur as libc::c_int == '0' as i32 } {
            frac = frac + 1 as libc::c_int;
            unsafe { cur = cur.offset(1) }
        }
        max = frac + 20 as libc::c_int;
        while unsafe {
            *cur as libc::c_int >= '0' as i32 && *cur as libc::c_int <= '9' as i32 && frac < max
        } {
            v = unsafe { *cur as libc::c_int - '0' as i32 };
            fraction = fraction * 10 as libc::c_int as libc::c_double + v as libc::c_double;
            frac = frac + 1 as libc::c_int;
            unsafe { cur = cur.offset(1) }
        }
        fraction /= unsafe { pow(10.0f64, frac as libc::c_double) };
        ret = ret + fraction;
        while unsafe { *cur as libc::c_int >= '0' as i32 && *cur as libc::c_int <= '9' as i32 } {
            unsafe { cur = cur.offset(1) }
        }
    }
    if unsafe { *cur as libc::c_int == 'e' as i32 || *cur as libc::c_int == 'E' as i32 } {
        unsafe { cur = cur.offset(1) };
        if unsafe { *cur as libc::c_int == '-' as i32 } {
            is_exponent_negative = 1 as libc::c_int;
            unsafe { cur = cur.offset(1) }
        } else if unsafe { *cur as libc::c_int == '+' as i32 } {
            unsafe { cur = cur.offset(1) }
        }
        while unsafe { *cur as libc::c_int >= '0' as i32 && *cur as libc::c_int <= '9' as i32 } {
            if exponent < 1000000 as libc::c_int {
                exponent =
                    exponent * 10 as libc::c_int + unsafe { (*cur as libc::c_int - '0' as i32) }
            }
            unsafe { cur = cur.offset(1) }
        }
    }
    while unsafe {
        *cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *cur as libc::c_int
                && *cur as libc::c_int <= 0xa as libc::c_int
            || *cur as libc::c_int == 0xd as libc::c_int
    } {
        unsafe { cur = cur.offset(1) }
    }
    if unsafe { *cur as libc::c_int != 0 as libc::c_int } {
        unsafe { return xmlXPathNAN }
    }
    if isneg != 0 {
        ret = -ret
    }
    if is_exponent_negative != 0 {
        exponent = -exponent
    }
    ret *= unsafe { pow(10.0f64, exponent as libc::c_double) };
    return ret;
}
/* *
 * xmlXPathCompNumber: * @ctxt: the XPath Parser context
 *
 *  [30]   Number ::= Digits ('.' Digits?)?
 *                    | '.' Digits
 *  [31]   Digits ::= [0-9]+ *
 * Compile a Number, then push it on the stack
 *
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompNumber(mut ctxt: xmlXPathParserContextPtr) {
    let mut ret: libc::c_double = 0.0f64;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut exponent: libc::c_int = 0 as libc::c_int;
    let mut is_exponent_negative: libc::c_int = 0 as libc::c_int;
    let mut num: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut tmp: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut temp: libc::c_double = 0.;
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    if unsafe {
        *safe_ctxt.cur as libc::c_int != '.' as i32
            && ((*safe_ctxt.cur as libc::c_int) < '0' as i32
                || *safe_ctxt.cur as libc::c_int > '9' as i32)
    } {
        unsafe { xmlXPathErr(ctxt, XPATH_NUMBER_ERROR as libc::c_int) };
        return;
    }
    /*
     * tmp/temp is a workaround against a gcc compiler bug
     * http://veillard.com/gcc.bug
     */
    ret = 0 as libc::c_int as libc::c_double;
    while unsafe {
        *safe_ctxt.cur as libc::c_int >= '0' as i32 && *safe_ctxt.cur as libc::c_int <= '9' as i32
    } {
        ret = ret * 10 as libc::c_int as libc::c_double;
        tmp = unsafe { (*safe_ctxt.cur as libc::c_int - '0' as i32) as libc::c_ulong };
        ok = 1 as libc::c_int;
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        temp = tmp as libc::c_double;
        ret = ret + temp
    }
    if unsafe { *safe_ctxt.cur as libc::c_int == '.' as i32 } {
        let mut v: libc::c_int = 0;
        let mut frac: libc::c_int = 0 as libc::c_int;
        let mut max: libc::c_int = 0;
        let mut fraction: libc::c_double = 0 as libc::c_int as libc::c_double;
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        if unsafe {
            ((*safe_ctxt.cur as libc::c_int) < '0' as i32
                || *safe_ctxt.cur as libc::c_int > '9' as i32)
                && ok == 0
        } {
            unsafe { xmlXPathErr(ctxt, XPATH_NUMBER_ERROR as libc::c_int) };
            return;
        }
        while unsafe { *safe_ctxt.cur as libc::c_int == '0' as i32 } {
            frac = frac + 1 as libc::c_int;
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        max = frac + 20 as libc::c_int;
        while unsafe {
            *safe_ctxt.cur as libc::c_int >= '0' as i32
                && *safe_ctxt.cur as libc::c_int <= '9' as i32
                && frac < max
        } {
            v = unsafe { *safe_ctxt.cur as libc::c_int - '0' as i32 };
            fraction = fraction * 10 as libc::c_int as libc::c_double + v as libc::c_double;
            frac = frac + 1 as libc::c_int;
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        fraction /= unsafe { pow(10.0f64, frac as libc::c_double) };
        ret = ret + fraction;
        while unsafe {
            *safe_ctxt.cur as libc::c_int >= '0' as i32
                && *safe_ctxt.cur as libc::c_int <= '9' as i32
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
    if unsafe {
        *safe_ctxt.cur as libc::c_int == 'e' as i32 || *safe_ctxt.cur as libc::c_int == 'E' as i32
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        if unsafe { *safe_ctxt.cur as libc::c_int == '-' as i32 } {
            is_exponent_negative = 1 as libc::c_int;
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        } else if unsafe { *safe_ctxt.cur } as libc::c_int == '+' as i32 {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        while unsafe {
            *safe_ctxt.cur as libc::c_int >= '0' as i32
                && *safe_ctxt.cur as libc::c_int <= '9' as i32
        } {
            if exponent < 1000000 as libc::c_int {
                exponent = exponent * 10 as libc::c_int
                    + unsafe { (*safe_ctxt.cur as libc::c_int - '0' as i32) }
            }
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if is_exponent_negative != 0 {
            exponent = -exponent
        }
        ret *= unsafe { pow(10.0f64, exponent as libc::c_double) }
    }
    num = unsafe { xmlXPathCacheNewFloat(safe_ctxt.context, ret) };
    if num.is_null() {
        safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
    } else if unsafe {
        xmlXPathCompExprAdd(
            ctxt,
            (*safe_ctxt.comp).last,
            -(1 as libc::c_int),
            XPATH_OP_VALUE,
            XPATH_NUMBER as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            num as *mut libc::c_void,
            0 as *mut libc::c_void,
        ) == -(1 as libc::c_int)
    } {
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, num) };
    };
}
/* *
 * xmlXPathParseLiteral: * @ctxt: the XPath Parser context
 *
 * Parse a Literal
 *
 *  [29]   Literal ::= '"' [^"]* '"'
 *                    | "'" [^']* "'"
 *
 * Returns the value found or NULL in case of error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathParseLiteral(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let mut q: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let safe_ctxt = unsafe { &mut *ctxt };
    if unsafe { *safe_ctxt.cur as libc::c_int == '\"' as i32 } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        q = safe_ctxt.cur;
        while unsafe {
            (0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
                || 0x20 as libc::c_int <= *safe_ctxt.cur as libc::c_int)
                && *safe_ctxt.cur as libc::c_int != '\"' as i32
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if !unsafe {
            (0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
                || 0x20 as libc::c_int <= *safe_ctxt.cur as libc::c_int)
        } {
            unsafe { xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as libc::c_int) };
            return 0 as *mut xmlChar;
        } else {
            ret = unsafe {
                xmlStrndup(
                    q,
                    safe_ctxt.cur.offset_from(q) as libc::c_long as libc::c_int,
                )
            };
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    } else if unsafe { *safe_ctxt.cur as libc::c_int == '\'' as i32 } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        q = safe_ctxt.cur;
        while unsafe {
            (0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
                || 0x20 as libc::c_int <= *safe_ctxt.cur as libc::c_int)
                && *safe_ctxt.cur as libc::c_int != '\'' as i32
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if !unsafe {
            (0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
                || 0x20 as libc::c_int <= *safe_ctxt.cur as libc::c_int)
        } {
            unsafe { xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as libc::c_int) };
            return 0 as *mut xmlChar;
        } else {
            ret = unsafe {
                xmlStrndup(
                    q,
                    safe_ctxt.cur.offset_from(q) as libc::c_long as libc::c_int,
                )
            };
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    } else {
        unsafe { xmlXPathErr(ctxt, XPATH_START_LITERAL_ERROR as libc::c_int) };
        return 0 as *mut xmlChar;
    }
    return ret;
}
/* *
 * xmlXPathCompLiteral: * @ctxt: the XPath Parser context
 *
 * Parse a Literal and push it on the stack.
 *
 *  [29]   Literal ::= '"' [^"]* '"'
 *                    | "'" [^']* "'"
 *
 * TODO: xmlXPathCompLiteral memory allocation could be improved.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompLiteral(mut ctxt: xmlXPathParserContextPtr) {
    let mut q: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut lit: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if unsafe { *safe_ctxt.cur as libc::c_int == '\"' as i32 } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        q = safe_ctxt.cur;
        while unsafe {
            (0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
                || 0x20 as libc::c_int <= *safe_ctxt.cur as libc::c_int)
                && *safe_ctxt.cur as libc::c_int != '\"' as i32
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if !unsafe {
            (0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
                || 0x20 as libc::c_int <= *safe_ctxt.cur as libc::c_int)
        } {
            unsafe { xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as libc::c_int) };
            return;
        } else {
            ret = unsafe {
                xmlStrndup(
                    q,
                    safe_ctxt.cur.offset_from(q) as libc::c_long as libc::c_int,
                )
            };
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    } else if unsafe { *safe_ctxt.cur as libc::c_int == '\'' as i32 } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        q = safe_ctxt.cur;
        while unsafe {
            (0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
                || 0x20 as libc::c_int <= *safe_ctxt.cur as libc::c_int)
                && *safe_ctxt.cur as libc::c_int != '\'' as i32
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if !unsafe {
            (0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
                || 0x20 as libc::c_int <= *safe_ctxt.cur as libc::c_int)
        } {
            unsafe { xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as libc::c_int) };
            return;
        } else {
            ret = unsafe {
                xmlStrndup(
                    q,
                    safe_ctxt.cur.offset_from(q) as libc::c_long as libc::c_int,
                )
            };
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    } else {
        unsafe { xmlXPathErr(ctxt, XPATH_START_LITERAL_ERROR as libc::c_int) };
        return;
    }
    if ret.is_null() {
        return;
    }
    lit = unsafe { xmlXPathCacheNewString(safe_ctxt.context, ret) };
    if lit.is_null() {
        safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
    } else if unsafe {
        xmlXPathCompExprAdd(
            ctxt,
            (*safe_ctxt.comp).last,
            -(1 as libc::c_int),
            XPATH_OP_VALUE,
            XPATH_STRING as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            lit as *mut libc::c_void,
            0 as *mut libc::c_void,
        ) == -(1 as libc::c_int)
    } {
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, lit) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void) };
}
/* *
 * xmlXPathCompVariableReference: * @ctxt: the XPath Parser context
 *
 * Parse a VariableReference, evaluate it and push it on the stack.
 *
 * The variable bindings consist of a mapping from variable names
 * to variable values. The value of a variable is an object, which can be
 * of any of the types that are possible for the value of an expression, * and may also be of additional types not specified here.
 *
 * Early evaluation is possible since: * The variable bindings [...] used to evaluate a subexpression are
 * always the same as those used to evaluate the containing expression.
 *
 *  [36]   VariableReference ::= '$' QName
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompVariableReference(mut ctxt: xmlXPathParserContextPtr) {
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe { *safe_ctxt.cur as libc::c_int != '$' as i32 } {
        unsafe { xmlXPathErr(ctxt, XPATH_VARIABLE_REF_ERROR as libc::c_int) };
        return;
    }
    if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
    } else {
    };
    name = xmlXPathParseQName(ctxt, &mut prefix);
    if name.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void) };
        unsafe { xmlXPathErr(ctxt, XPATH_VARIABLE_REF_ERROR as libc::c_int) };
        return;
    }
    unsafe { (*safe_ctxt.comp).last = -(1 as libc::c_int) };
    if unsafe {
        xmlXPathCompExprAdd(
            ctxt,
            (*safe_ctxt.comp).last,
            -(1 as libc::c_int),
            XPATH_OP_VARIABLE,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            name as *mut libc::c_void,
            prefix as *mut libc::c_void,
        ) == -(1 as libc::c_int)
    } {
        unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void) };
        unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) };
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if !safe_ctxt.context.is_null()
        && unsafe { (*safe_ctxt.context).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_FORBID_VARIABLE_ERROR as libc::c_int) };
        return;
    };
}
/* *
 * xmlXPathIsNodeType: * @name: a name string
 *
 * Is the name given a NodeType one.
 *
 *  [38]   NodeType ::= 'comment'
 *                    | 'text'
 *                    | 'processing-instruction'
 *                    | 'node'
 *
 * Returns 1 if true 0 otherwise
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathIsNodeType(mut name: *const xmlChar) -> libc::c_int {
    if name.is_null() {
        return 0 as libc::c_int;
    }
    if unsafe {
        xmlStrEqual(
            name,
            b"node\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
        ) != 0
    } {
        return 1 as libc::c_int;
    }
    if unsafe {
        xmlStrEqual(
            name,
            b"text\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
        ) != 0
    } {
        return 1 as libc::c_int;
    }
    if unsafe {
        xmlStrEqual(
            name,
            b"comment\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
        ) != 0
    } {
        return 1 as libc::c_int;
    }
    if unsafe {
        xmlStrEqual(
            name,
            b"processing-instruction\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
        ) != 0
    } {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathCompFunctionCall: * @ctxt: the XPath Parser context
 *
 *  [16]   FunctionCall ::= FunctionName '(' ( Argument ( ',' Argument)*)? ')'
 *  [17]   Argument ::= Expr
 *
 * Compile a function call, the evaluation of all arguments are
 * pushed on the stack
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompFunctionCall(mut ctxt: xmlXPathParserContextPtr) {
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut nbargs: libc::c_int = 0 as libc::c_int;
    let mut sort: libc::c_int = 1 as libc::c_int;
    let safe_ctxt = unsafe { &mut *ctxt };
    name = unsafe { xmlXPathParseQName(ctxt, &mut prefix) };
    if name.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void) };
        unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
        return;
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    //DEBUG_EXPR
    if unsafe { *safe_ctxt.cur as libc::c_int != '(' as i32 } {
        unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) };
        unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void) };
        unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
        return;
    }
    if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
    } else {
    };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    /*
     * Optimization for count(): we don't need the node-set to be sorted.
     */
    if prefix.is_null()
        && unsafe {
            *name.offset(0 as libc::c_int as isize) as libc::c_int == 'c' as i32
                && xmlStrEqual(
                    name,
                    b"count\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
        }
    {
        sort = 0 as libc::c_int
    }
    unsafe { (*safe_ctxt.comp).last = -(1 as libc::c_int) };
    if unsafe { *safe_ctxt.cur as libc::c_int != ')' as i32 } {
        while unsafe { *safe_ctxt.cur as libc::c_int != 0 as libc::c_int } {
            let mut op1: libc::c_int = unsafe { (*safe_ctxt.comp).last };
            unsafe { (*safe_ctxt.comp).last = -(1 as libc::c_int) };
            unsafe { xmlXPathCompileExpr(ctxt, sort) };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) };
                unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void) };
                return;
            }
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    op1,
                    (*safe_ctxt.comp).last,
                    XPATH_OP_ARG,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                )
            };
            nbargs += 1;
            if unsafe { *safe_ctxt.cur as libc::c_int == ')' as i32 } {
                break;
            }
            if unsafe { *safe_ctxt.cur as libc::c_int != ',' as i32 } {
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) };
                unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void) };
                unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
                return;
            }
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
            while unsafe {
                *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                        && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                    || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
            } {
                if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                    unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                } else {
                };
            }
        }
    }
    if unsafe {
        xmlXPathCompExprAdd(
            ctxt,
            (*safe_ctxt.comp).last,
            -(1 as libc::c_int),
            XPATH_OP_FUNCTION,
            nbargs,
            0 as libc::c_int,
            0 as libc::c_int,
            name as *mut libc::c_void,
            prefix as *mut libc::c_void,
        ) == -(1 as libc::c_int)
    } {
        unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void) };
        unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) };
    }
    if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
    } else {
    };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
}
/* *
 * xmlXPathCompPrimaryExpr: * @ctxt: the XPath Parser context
 *
 *  [15]   PrimaryExpr ::= VariableReference
 *                | '(' Expr ')'
 *                | Literal
 *                | Number
 *                | FunctionCall
 *
 * Compile a primary expression.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompPrimaryExpr(mut ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe { *safe_ctxt.cur as libc::c_int == '$' as i32 } {
        unsafe { xmlXPathCompVariableReference(ctxt) };
    } else if unsafe { *safe_ctxt.cur as libc::c_int == '(' as i32 } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompileExpr(ctxt, 1 as libc::c_int) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        if unsafe { *safe_ctxt.cur as libc::c_int != ')' as i32 } {
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
            return;
        }
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    } else if unsafe {
        0x30 as libc::c_int <= *safe_ctxt.cur as libc::c_int
            && *safe_ctxt.cur as libc::c_int <= 0x39 as libc::c_int
            || *safe_ctxt.cur as libc::c_int == '.' as i32
                && (0x30 as libc::c_int
                    <= *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int
                    && *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int
                        <= 0x39 as libc::c_int)
    } {
        unsafe { xmlXPathCompNumber(ctxt) };
    } else if unsafe {
        *safe_ctxt.cur as libc::c_int == '\'' as i32 || *safe_ctxt.cur as libc::c_int == '\"' as i32
    } {
        unsafe { xmlXPathCompLiteral(ctxt) };
    } else {
        unsafe { xmlXPathCompFunctionCall(ctxt) };
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
}
/* *
 * xmlXPathCompFilterExpr: * @ctxt: the XPath Parser context
 *
 *  [20]   FilterExpr ::= PrimaryExpr
 *               | FilterExpr Predicate
 *
 * Compile a filter expression.
 * Square brackets are used to filter expressions in the same way that
 * they are used in location paths. It is an error if the expression to
 * be filtered does not evaluate to a node-set. The context node list
 * used for evaluating the expression in square brackets is the node-set
 * to be filtered listed in document order.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompFilterExpr(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompPrimaryExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe { *safe_ctxt.cur as libc::c_int == '[' as i32 } {
        unsafe { xmlXPathCompPredicate(ctxt, 1 as libc::c_int) };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
}
/* *
 * xmlXPathScanName: * @ctxt: the XPath Parser context
 *
 * Trickery: parse an XML name but without consuming the input flow
 * Needed to avoid insanity in the parser state.
 *
 * [4] NameChar ::= Letter | Digit | '.' | '-' | '_' | ':' |
 *                  CombiningChar | Extender
 *
 * [5] Name ::= (Letter | '_' | ':') (NameChar)*
 *
 * [6] Names ::= Name (S Name)*
 *
 * Returns the Name parsed or NULL
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathScanName(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let safe_ctxt = unsafe { &mut *ctxt };
    cur = safe_ctxt.cur;
    c = unsafe { xmlXPathCurrentChar(ctxt, &mut l) };
    if c == ' ' as i32
        || c == '>' as i32
        || c == '/' as i32
        || !((if c < 0x100 as libc::c_int {
            (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
                || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
                || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
                || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
                || 0xf8 as libc::c_int <= c) as libc::c_int
        } else {
            unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup) }
        }) != 0
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                    || c == 0x3007 as libc::c_int
                    || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                    as libc::c_int
            }) != 0)
            && c != '_' as i32
            && c != ':' as i32
    {
        return 0 as *mut xmlChar;
    }
    while c != ' ' as i32
        && c != '>' as i32
        && c != '/' as i32
        && ((if c < 0x100 as libc::c_int {
            (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
                || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
                || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
                || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
                || 0xf8 as libc::c_int <= c) as libc::c_int
        } else {
            unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup) }
        }) != 0
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                    || c == 0x3007 as libc::c_int
                    || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                    as libc::c_int
            }) != 0
            || (if c < 0x100 as libc::c_int {
                (0x30 as libc::c_int <= c && c <= 0x39 as libc::c_int) as libc::c_int
            } else {
                unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsDigitGroup) }
            }) != 0
            || c == '.' as i32
            || c == '-' as i32
            || c == '_' as i32
            || c == ':' as i32
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsCombiningGroup) }
            }) != 0
            || (if c < 0x100 as libc::c_int {
                (c == 0xb7 as libc::c_int) as libc::c_int
            } else {
                unsafe { xmlCharInRange(c as libc::c_uint, &xmlIsExtenderGroup) }
            }) != 0)
    {
        len += l;
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(l as isize) };
        c = unsafe { xmlXPathCurrentChar(ctxt, &mut l) }
    }
    ret = unsafe {
        xmlStrndup(
            cur,
            safe_ctxt.cur.offset_from(cur) as libc::c_long as libc::c_int,
        )
    };
    safe_ctxt.cur = cur;
    return ret;
}
/* *
 * xmlXPathCompPathExpr: * @ctxt: the XPath Parser context
 *
 *  [19]   PathExpr ::= LocationPath
 *               | FilterExpr
 *               | FilterExpr '/' RelativeLocationPath
 *               | FilterExpr '//' RelativeLocationPath
 *
 * Compile a path expression.
 * The / operator and // operators combine an arbitrary expression
 * and a relative location path. It is an error if the expression
 * does not evaluate to a node-set.
 * The / operator does composition in the same way as when / is
 * used in a location path. As in location paths, // is short for
 * /descendant-or-self::node()/.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompPathExpr(mut ctxt: xmlXPathParserContextPtr) {
    let mut lc: libc::c_int = 1 as libc::c_int; /* Should we branch to LocationPath ?         */
    let mut name: *mut xmlChar = 0 as *mut xmlChar; /* we may have to preparse a name to find out */
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe {
        *safe_ctxt.cur as libc::c_int == '$' as i32
            || *safe_ctxt.cur as libc::c_int == '(' as i32
            || 0x30 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0x39 as libc::c_int
            || *safe_ctxt.cur as libc::c_int == '\'' as i32
            || *safe_ctxt.cur as libc::c_int == '\"' as i32
            || *safe_ctxt.cur as libc::c_int == '.' as i32
                && (0x30 as libc::c_int
                    <= *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int
                    && *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int
                        <= 0x39 as libc::c_int)
    } {
        lc = 0 as libc::c_int
    } else if unsafe { *safe_ctxt.cur as libc::c_int == '*' as i32 } {
        /* relative or absolute location path */
        lc = 1 as libc::c_int
    } else if unsafe { *safe_ctxt.cur as libc::c_int == '/' as i32 } {
        /* relative or absolute location path */
        lc = 1 as libc::c_int
    } else if unsafe { *safe_ctxt.cur as libc::c_int == '@' as i32 } {
        /* relative abbreviated attribute location path */
        lc = 1 as libc::c_int
    } else if unsafe { *safe_ctxt.cur as libc::c_int == '.' as i32 } {
        /* relative abbreviated attribute location path */
        lc = 1 as libc::c_int
    } else {
        /*
         * Problem is finding if we have a name here whether it's:
         *   - a nodetype
         *   - a function call in which case it's followed by '('
         *   - an axis in which case it's followed by ':'
         *   - a element name
         * We do an a priori analysis here rather than having to
         * maintain parsed token content through the recursive function
         * calls. This looks uglier but makes the code easier to
         * read/write/debug.
         */
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        name = unsafe { xmlXPathScanName(ctxt) };
        if !name.is_null()
            && !unsafe {
                xmlStrstr(
                    name,
                    b"::\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                )
                .is_null()
            }
        {
            lc = 1 as libc::c_int;
            unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) };
        } else if !name.is_null() {
            let mut len: libc::c_int = unsafe { xmlStrlen(name) };
            while unsafe { *safe_ctxt.cur.offset(len as isize) as libc::c_int != 0 as libc::c_int }
            {
                if unsafe { *safe_ctxt.cur.offset(len as isize) as libc::c_int == '/' as i32 } {
                    /* element name */
                    lc = 1 as libc::c_int;
                    break;
                } else if unsafe {
                    *safe_ctxt.cur.offset(len as isize) as libc::c_int == 0x20 as libc::c_int
                        || 0x9 as libc::c_int <= *safe_ctxt.cur.offset(len as isize) as libc::c_int
                            && *safe_ctxt.cur.offset(len as isize) as libc::c_int
                                <= 0xa as libc::c_int
                        || *safe_ctxt.cur.offset(len as isize) as libc::c_int == 0xd as libc::c_int
                } {
                    /* ignore blanks */
                    len += 1
                } else if unsafe {
                    *safe_ctxt.cur.offset(len as isize) as libc::c_int == ':' as i32
                } {
                    lc = 1 as libc::c_int;
                    break;
                } else if unsafe {
                    *safe_ctxt.cur.offset(len as isize) as libc::c_int == '(' as i32
                } {
                    /* Node Type or Function */
                    if unsafe { xmlXPathIsNodeType(name) != 0 } {
                        lc = 1 as libc::c_int
                    } else if safe_ctxt.xptr != 0
                        && unsafe {
                            xmlStrEqual(
                                name,
                                b"range-to\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                            ) != 0
                        }
                    {
                        lc = 1 as libc::c_int
                    } else {
                        lc = 0 as libc::c_int
                    }
                    break;
                } else if unsafe {
                    *safe_ctxt.cur.offset(len as isize) as libc::c_int == '[' as i32
                } {
                    /* element name */
                    lc = 1 as libc::c_int;
                    break;
                } else if unsafe {
                    *safe_ctxt.cur.offset(len as isize) as libc::c_int == '<' as i32
                        || *safe_ctxt.cur.offset(len as isize) as libc::c_int == '>' as i32
                        || *safe_ctxt.cur.offset(len as isize) as libc::c_int == '=' as i32
                } {
                    lc = 1 as libc::c_int;
                    break;
                } else {
                    lc = 1 as libc::c_int;
                    break;
                }
            }
            if unsafe { *safe_ctxt.cur.offset(len as isize) as libc::c_int == 0 as libc::c_int } {
                /* element name */
                lc = 1 as libc::c_int
            }
            unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) };
        } else {
            /* make sure all cases are covered explicitly */
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
            return;
        }
    }
    if lc != 0 {
        if unsafe { *safe_ctxt.cur as libc::c_int == '/' as i32 } {
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    -(1 as libc::c_int),
                    -(1 as libc::c_int),
                    XPATH_OP_ROOT,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                )
            };
        } else {
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    -(1 as libc::c_int),
                    -(1 as libc::c_int),
                    XPATH_OP_NODE,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                )
            };
        }
        unsafe { xmlXPathCompLocationPath(ctxt) };
    } else {
        unsafe { xmlXPathCompFilterExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        if unsafe {
            *safe_ctxt.cur as libc::c_int == '/' as i32
                && *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(2 as libc::c_int as isize) };
            while unsafe {
                *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                        && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                    || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
            } {
                if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                    unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                } else {
                };
            }
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    (*safe_ctxt.comp).last,
                    -(1 as libc::c_int),
                    XPATH_OP_COLLECT,
                    AXIS_DESCENDANT_OR_SELF as libc::c_int,
                    NODE_TEST_TYPE as libc::c_int,
                    NODE_TYPE_NODE as libc::c_int,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                )
            };
            unsafe { xmlXPathCompRelativeLocationPath(ctxt) };
        } else if unsafe { *safe_ctxt.cur as libc::c_int == '/' as i32 } {
            unsafe { xmlXPathCompRelativeLocationPath(ctxt) };
        }
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
}
/* *
 * xmlXPathCompUnionExpr: * @ctxt: the XPath Parser context
 *
 *  [18]   UnionExpr ::= PathExpr
 *               | UnionExpr '|' PathExpr
 *
 * Compile an union expression.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompUnionExpr(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompPathExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe { *safe_ctxt.cur as libc::c_int == '|' as i32 } {
        let mut op1: libc::c_int = unsafe { (*safe_ctxt.comp).last };
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                XPATH_OP_NODE,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompPathExpr(ctxt) };
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_UNION,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
}
/* *
 * xmlXPathCompUnaryExpr: * @ctxt: the XPath Parser context
 *
 *  [27]   UnaryExpr ::= UnionExpr
 *                   | '-' UnaryExpr
 *
 * Compile an unary expression.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompUnaryExpr(mut ctxt: xmlXPathParserContextPtr) {
    let mut minus: libc::c_int = 0 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe { *safe_ctxt.cur as libc::c_int == '-' as i32 } {
        minus = 1 as libc::c_int - minus;
        found = 1 as libc::c_int;
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
    unsafe { xmlXPathCompUnionExpr(ctxt) };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    if found != 0 {
        if minus != 0 {
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    (*safe_ctxt.comp).last,
                    -(1 as libc::c_int),
                    XPATH_OP_PLUS,
                    2 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                )
            };
        } else {
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    (*safe_ctxt.comp).last,
                    -(1 as libc::c_int),
                    XPATH_OP_PLUS,
                    3 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                )
            };
        }
    };
}
/* *
 * xmlXPathCompMultiplicativeExpr: * @ctxt: the XPath Parser context
 *
 *  [26]   MultiplicativeExpr ::= UnaryExpr
 *                   | MultiplicativeExpr MultiplyOperator UnaryExpr
 *                   | MultiplicativeExpr 'div' UnaryExpr
 *                   | MultiplicativeExpr 'mod' UnaryExpr
 *  [34]   MultiplyOperator ::= '*'
 *
 * Compile an Additive expression.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompMultiplicativeExpr(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompUnaryExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == '*' as i32
            || *safe_ctxt.cur as libc::c_int == 'd' as i32
                && *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int == 'i' as i32
                && *safe_ctxt.cur.offset(2 as libc::c_int as isize) as libc::c_int == 'v' as i32
            || *safe_ctxt.cur as libc::c_int == 'm' as i32
                && *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int == 'o' as i32
                && *safe_ctxt.cur.offset(2 as libc::c_int as isize) as libc::c_int == 'd' as i32
    } {
        let mut op: libc::c_int = -(1 as libc::c_int);
        let mut op1: libc::c_int = unsafe { (*safe_ctxt.comp).last };
        if unsafe { *safe_ctxt.cur as libc::c_int == '*' as i32 } {
            op = 0 as libc::c_int;
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        } else if unsafe { *safe_ctxt.cur as libc::c_int == 'd' as i32 } {
            op = 1 as libc::c_int;
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(3 as libc::c_int as isize) }
        } else if unsafe { *safe_ctxt.cur as libc::c_int == 'm' as i32 } {
            op = 2 as libc::c_int;
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(3 as libc::c_int as isize) }
        }
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompUnaryExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_MULT,
                op,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
}
/* *
 * xmlXPathCompAdditiveExpr: * @ctxt: the XPath Parser context
 *
 *  [25]   AdditiveExpr ::= MultiplicativeExpr
 *                   | AdditiveExpr '+' MultiplicativeExpr
 *                   | AdditiveExpr '-' MultiplicativeExpr
 *
 * Compile an Additive expression.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompAdditiveExpr(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompMultiplicativeExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == '+' as i32 || *safe_ctxt.cur as libc::c_int == '-' as i32
    } {
        let mut plus: libc::c_int = 0;
        let mut op1: libc::c_int = unsafe { (*safe_ctxt.comp).last };
        if unsafe { *safe_ctxt.cur as libc::c_int == '+' as i32 } {
            plus = 1 as libc::c_int
        } else {
            plus = 0 as libc::c_int
        }
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompMultiplicativeExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_PLUS,
                plus,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
}
/* *
 * xmlXPathCompRelationalExpr: * @ctxt: the XPath Parser context
 *
 *  [24]   RelationalExpr ::= AdditiveExpr
 *                 | RelationalExpr '<' AdditiveExpr
 *                 | RelationalExpr '>' AdditiveExpr
 *                 | RelationalExpr '<=' AdditiveExpr
 *                 | RelationalExpr '>=' AdditiveExpr
 *
 *  A <= B > C is allowed ? Answer from James, yes with
 *  (AdditiveExpr <= AdditiveExpr) > AdditiveExpr
 *  which is basically what got implemented.
 *
 * Compile a Relational expression, then push the result
 * on the stack
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompRelationalExpr(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompAdditiveExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == '<' as i32 || *safe_ctxt.cur as libc::c_int == '>' as i32
    } {
        let mut inf: libc::c_int = 0;
        let mut strict: libc::c_int = 0;
        let mut op1: libc::c_int = unsafe { (*safe_ctxt.comp).last };
        if unsafe { *safe_ctxt.cur as libc::c_int == '<' as i32 } {
            inf = 1 as libc::c_int
        } else {
            inf = 0 as libc::c_int
        }
        if unsafe { *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32 }
        {
            strict = 0 as libc::c_int
        } else {
            strict = 1 as libc::c_int
        }
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        if strict == 0 {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompAdditiveExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_CMP,
                inf,
                strict,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
}
/* *
 * xmlXPathCompEqualityExpr: * @ctxt: the XPath Parser context
 *
 *  [23]   EqualityExpr ::= RelationalExpr
 *                 | EqualityExpr '=' RelationalExpr
 *                 | EqualityExpr '!=' RelationalExpr
 *
 *  A != B != C is allowed ? Answer from James, yes with
 *  (RelationalExpr = RelationalExpr) = RelationalExpr
 *  (RelationalExpr != RelationalExpr) != RelationalExpr
 *  which is basically what got implemented.
 *
 * Compile an Equality expression.
 *
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompEqualityExpr(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompRelationalExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == '=' as i32
            || *safe_ctxt.cur as libc::c_int == '!' as i32
                && *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
    } {
        let mut eq: libc::c_int = 0;
        let mut op1: libc::c_int = unsafe { (*safe_ctxt.comp).last };
        if unsafe { *safe_ctxt.cur as libc::c_int == '=' as i32 } {
            eq = 1 as libc::c_int
        } else {
            eq = 0 as libc::c_int
        }
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        if eq == 0 {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompRelationalExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_EQUAL,
                eq,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
}
/* *
 * xmlXPathCompAndExpr: * @ctxt: the XPath Parser context
 *
 *  [22]   AndExpr ::= EqualityExpr
 *                 | AndExpr 'and' EqualityExpr
 *
 * Compile an AND expression.
 *
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompAndExpr(mut ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompEqualityExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 'a' as i32
            && *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32
            && *safe_ctxt.cur.offset(2 as libc::c_int as isize) as libc::c_int == 'd' as i32
    } {
        let mut op1: libc::c_int = unsafe { (*safe_ctxt.comp).last };
        unsafe {
            safe_ctxt.cur = safe_ctxt.cur.offset(3 as libc::c_int as isize);
        }
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompEqualityExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_AND,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
}
/* ***********************************************************************
 *									*
 *			The Parser					*
 *									*
 ************************************************************************/
/*
 * a few forward declarations since we use a recursive call based
 * implementation.
 */
/* *
 * xmlXPathCompileExpr: * @ctxt: the XPath Parser context
 *
 *  [14]   Expr ::= OrExpr
 *  [21]   OrExpr ::= AndExpr
 *                 | OrExpr 'or' AndExpr
 *
 * Parse and compile an expression
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompileExpr(mut ctxt: xmlXPathParserContextPtr, mut sort: libc::c_int) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut xpctxt: xmlXPathContextPtr = safe_ctxt.context;
    let safe_xpctxt = unsafe { &mut *xpctxt };
    if !xpctxt.is_null() {
        if safe_xpctxt.depth >= 5000 as libc::c_int {
            unsafe { xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as libc::c_int) };
            return;
        }
        /*
         * Parsing a single '(' pushes about 10 functions on the call stack
         * before recursing!
         */
        safe_xpctxt.depth += 10 as libc::c_int
    }
    unsafe { xmlXPathCompAndExpr(ctxt) };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 'o' as i32
            && *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int == 'r' as i32
    } {
        let mut op1: libc::c_int = unsafe { (*safe_ctxt.comp).last };
        unsafe {
            safe_ctxt.cur = safe_ctxt.cur.offset(2 as libc::c_int as isize);
        }
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompAndExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_OR,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
    if sort != 0
        && unsafe {
            (*(*safe_ctxt.comp)
                .steps
                .offset((*safe_ctxt.comp).last as isize))
            .op as libc::c_uint
        } != XPATH_OP_VALUE as libc::c_int as libc::c_uint
    {
        /* more ops could be optimized too */
        /*
        	* This is the main place to eliminate sorting for
        	* operations which don't require a sorted node-set.
        	* E.g. count().
        	*/
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                (*safe_ctxt.comp).last,
                -(1 as libc::c_int),
                XPATH_OP_SORT,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
    }
    if !xpctxt.is_null() {
        safe_xpctxt.depth -= 1 as libc::c_int
    };
}
/* *
 * xmlXPathCompPredicate: * @ctxt: the XPath Parser context
 * @filter: act as a filter
 *
 *  [8]   Predicate ::= '[' PredicateExpr ']'
 *  [9]   PredicateExpr ::= Expr
 *
 * Compile a predicate expression
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompPredicate(mut ctxt: xmlXPathParserContextPtr, mut filter: libc::c_int) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut op1: libc::c_int = unsafe { (*safe_ctxt.comp).last };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe { *safe_ctxt.cur as libc::c_int != '[' as i32 } {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_PREDICATE_ERROR as libc::c_int) };
        return;
    }
    if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
    } else {
    };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    unsafe { (*safe_ctxt.comp).last = -(1 as libc::c_int) };
    /*
     * This call to xmlXPathCompileExpr() will deactivate sorting
     * of the predicate result.
     * TODO: Sorting is still activated for filters, since I'm not
     *  sure if needed. Normally sorting should not be needed, since
     *  a filter can only diminish the number of items in a sequence, *  but won't change its order; so if the initial sequence is sorted, *  subsequent sorting is not needed.
     */
    if filter == 0 {
        unsafe { xmlXPathCompileExpr(ctxt, 0 as libc::c_int) };
    } else {
        unsafe { xmlXPathCompileExpr(ctxt, 1 as libc::c_int) };
    }
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    if unsafe { *safe_ctxt.cur as libc::c_int != ']' as i32 } {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_PREDICATE_ERROR as libc::c_int) };
        return;
    }
    if filter != 0 {
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_FILTER,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
    } else {
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_PREDICATE,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
    }
    if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
    } else {
    };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
}
/* *
 * xmlXPathCompNodeTest: * @ctxt: the XPath Parser context
 * @test: pointer to a xmlXPathTestVal
 * @type: pointer to a xmlXPathTypeVal
 * @prefix: placeholder for a possible name prefix
 *
 * [7] NodeTest ::= NameTest
 *		    | NodeType '(' ')'
 *		    | 'processing-instruction' '(' Literal ')'
 *
 * [37] NameTest ::= '*'
 *		    | NCName ':' '*'
 *		    | QName
 * [38] NodeType ::= 'comment'
 *		   | 'text'
 *		   | 'processing-instruction'
 *		   | 'node'
 *
 * Returns the name found and updates @test, @type and @prefix appropriately
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompNodeTest(
    mut ctxt: xmlXPathParserContextPtr,
    mut test: *mut xmlXPathTestVal,
    mut type_0: *mut xmlXPathTypeVal,
    mut prefix: *mut *mut xmlChar,
    mut name: *mut xmlChar,
) -> *mut xmlChar {
    let mut blanks: libc::c_int = 0;
    if test.is_null() || type_0.is_null() || prefix.is_null() {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Internal error at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                11067 as libc::c_int,
            )
        };
        return 0 as *mut xmlChar;
    }
    unsafe {
        *type_0 = NODE_TYPE_NODE;
        *test = NODE_TEST_NONE;
        *prefix = 0 as *mut xmlChar;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if name.is_null() && unsafe { *safe_ctxt.cur as libc::c_int == '*' as i32 } {
        /*
         * All elements
         */
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        unsafe {
            *test = NODE_TEST_ALL;
        }
        return 0 as *mut xmlChar;
    }
    if name.is_null() {
        name = unsafe { xmlXPathParseNCName(ctxt) }
    }
    if name.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
        return 0 as *mut xmlChar;
    }
    blanks = unsafe {
        (*safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int) as libc::c_int
    };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe { *safe_ctxt.cur as libc::c_int == '(' as i32 } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        /*
         * NodeType or PI search
         */
        if unsafe {
            xmlStrEqual(
                name,
                b"comment\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
        } {
            unsafe { *type_0 = NODE_TYPE_COMMENT }
        } else if unsafe {
            xmlStrEqual(
                name,
                b"node\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
        } {
            unsafe { *type_0 = NODE_TYPE_NODE }
        } else if unsafe {
            xmlStrEqual(
                name,
                b"processing-instruction\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
        } {
            unsafe { *type_0 = NODE_TYPE_PI }
        } else if unsafe {
            xmlStrEqual(
                name,
                b"text\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
        } {
            unsafe { *type_0 = NODE_TYPE_TEXT }
        } else {
            if !name.is_null() {
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) };
            }
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
            return 0 as *mut xmlChar;
        }
        unsafe {
            *test = NODE_TEST_TYPE;
        }
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if unsafe { *type_0 as libc::c_uint == NODE_TYPE_PI as libc::c_int as libc::c_uint } {
            /*
             * Specific case: search a PI by name.
             */
            if !name.is_null() {
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) };
            }
            name = 0 as *mut xmlChar;
            if unsafe { *safe_ctxt.cur as libc::c_int != ')' as i32 } {
                name = unsafe { xmlXPathParseLiteral(ctxt) };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as *mut xmlChar;
                }
                unsafe {
                    *test = NODE_TEST_PI;
                }
                while unsafe {
                    *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                        || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                            && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                        || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
                } {
                    if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                    } else {
                    };
                }
            }
        }
        if unsafe { *safe_ctxt.cur as libc::c_int != ')' as i32 } {
            if !name.is_null() {
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) };
            }
            unsafe { xmlXPathErr(ctxt, XPATH_UNCLOSED_ERROR as libc::c_int) };
            return 0 as *mut xmlChar;
        }
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        return name;
    }
    unsafe {
        *test = NODE_TEST_NAME;
    }
    if blanks == 0 && unsafe { *safe_ctxt.cur as libc::c_int == ':' as i32 } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        /*
         * Since currently the parser context don't have a
         * namespace list associated:
         * The namespace name for this prefix can be computed
         * only at evaluation time. The compilation is done
         * outside of any context.
         */
        unsafe {
            *prefix = name;
        }
        if unsafe { *safe_ctxt.cur as libc::c_int == '*' as i32 } {
            /*
             * All elements
             */
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
            unsafe {
                *test = NODE_TEST_ALL;
            }
            return 0 as *mut xmlChar;
        }
        name = unsafe { xmlXPathParseNCName(ctxt) };
        if name.is_null() {
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
            return 0 as *mut xmlChar;
        }
    }
    return name;
}
/* *
 * xmlXPathIsAxisName: * @name: a preparsed name token
 *
 * [6] AxisName ::= 'ancestor'
 *                  | 'ancestor-or-self'
 *                  | 'attribute'
 *                  | 'child'
 *                  | 'descendant'
 *                  | 'descendant-or-self'
 *                  | 'following'
 *                  | 'following-sibling'
 *                  | 'namespace'
 *                  | 'parent'
 *                  | 'preceding'
 *                  | 'preceding-sibling'
 *                  | 'self'
 *
 * Returns the axis or 0
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathIsAxisName(mut name: *const xmlChar) -> xmlXPathAxisVal {
    let mut ret: xmlXPathAxisVal = 0 as xmlXPathAxisVal;
    match unsafe { *name.offset(0 as libc::c_int as isize) as libc::c_int } {
        97 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"ancestor\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_ANCESTOR
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"ancestor-or-self\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_ANCESTOR_OR_SELF
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"attribute\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_ATTRIBUTE
            }
        }
        99 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"child\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_CHILD
            }
        }
        100 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"descendant\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_DESCENDANT
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"descendant-or-self\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_DESCENDANT_OR_SELF
            }
        }
        102 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"following\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_FOLLOWING
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"following-sibling\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_FOLLOWING_SIBLING
            }
        }
        110 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"namespace\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_NAMESPACE
            }
        }
        112 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"parent\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_PARENT
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"preceding\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_PRECEDING
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"preceding-sibling\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_PRECEDING_SIBLING
            }
        }
        115 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"self\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_SELF
            }
        }
        _ => {}
    }
    return ret;
}
/* *
 * xmlXPathCompStep: * @ctxt: the XPath Parser context
 *
 * [4] Step ::= AxisSpecifier NodeTest Predicate*
 *                  | AbbreviatedStep
 *
 * [12] AbbreviatedStep ::= '.' | '..'
 *
 * [5] AxisSpecifier ::= AxisName '::'
 *                  | AbbreviatedAxisSpecifier
 *
 * [13] AbbreviatedAxisSpecifier ::= '@'?
 *
 * Modified for XPtr range support as: *
 *  [4xptr] Step ::= AxisSpecifier NodeTest Predicate*
 *                     | AbbreviatedStep
 *                     | 'range-to' '(' Expr ')' Predicate*
 *
 * Compile one step in a Location Path
 * A location step of . is short for self::node(). This is
 * particularly useful in conjunction with //. For example, the
 * location path .//para is short for
 * self::node()/descendant-or-self::node()/child::para
 * and so will select all para descendant elements of the context
 * node.
 * Similarly, a location step of .. is short for parent::node().
 * For example, ../title is short for parent::node()/child::title
 * and so will select the title children of the parent of the context
 * node.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
unsafe fn xmlXPathCompStep(mut ctxt: xmlXPathParserContextPtr) {
    match () {
        #[cfg(LIBXML_XPTR_ENABLED)]
        _ => {
            let mut rangeto: libc::c_int = 0 as libc::c_int;
            let mut op2: libc::c_int = -(1 as libc::c_int);
        }
        #[cfg(not(LIBXML_XPTR_ENABLED))]
        _ => {}
    }
    let mut rangeto: libc::c_int = 0 as libc::c_int;
    let mut op2: libc::c_int = -(1 as libc::c_int);
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1)
        } else {
        };
    }
    if *(*ctxt).cur as libc::c_int == '.' as i32
        && *(*ctxt).cur.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        (*ctxt).cur = (*ctxt).cur.offset(2 as libc::c_int as isize);
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1)
            } else {
            };
        }
        xmlXPathCompExprAdd(
            ctxt,
            (*(*ctxt).comp).last,
            -(1 as libc::c_int),
            XPATH_OP_COLLECT,
            AXIS_PARENT as libc::c_int,
            NODE_TEST_TYPE as libc::c_int,
            NODE_TYPE_NODE as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
    } else if *(*ctxt).cur as libc::c_int == '.' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1)
        } else {
        };
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1)
            } else {
            };
        }
    } else {
        let mut current_block_91: u64;
        let mut name: *mut xmlChar = 0 as *mut xmlChar;
        let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
        let mut test: xmlXPathTestVal = NODE_TEST_NONE;
        let mut axis: xmlXPathAxisVal = 0 as xmlXPathAxisVal;
        let mut type_0: xmlXPathTypeVal = NODE_TYPE_NODE;
        let mut op1: libc::c_int = 0;
        /*
         * The modification needed for XPointer change to the production
         */
        //  match(){
        //     #[cfg(LIBXML_XPTR_ENABLED)] _ => {

        //     }
        //     #[cfg(not(LIBXML_XPTR_ENABLED))] _ => {}
        // }
        if (*ctxt).xptr != 0 {
            name = xmlXPathParseNCName(ctxt);
            if !name.is_null()
                && xmlStrEqual(
                    name,
                    b"range-to\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            {
                op2 = (*(*ctxt).comp).last;
                xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
                while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                        && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                    || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = (*ctxt).cur.offset(1)
                    } else {
                    };
                }
                if *(*ctxt).cur as libc::c_int != '(' as i32 {
                    xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
                    return;
                }
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1)
                } else {
                };
                while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                        && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                    || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = (*ctxt).cur.offset(1)
                    } else {
                    };
                }
                xmlXPathCompileExpr(ctxt, 1 as libc::c_int);
                /* PUSH_BINARY_EXPR(XPATH_OP_RANGETO, op2, ctxt->comp->last, 0, 0); */
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    return;
                }
                while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                        && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                    || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = (*ctxt).cur.offset(1)
                    } else {
                    };
                }
                if *(*ctxt).cur as libc::c_int != ')' as i32 {
                    xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
                    return;
                }
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1)
                } else {
                };
                rangeto = 1 as libc::c_int;
                current_block_91 = 7936397428294447972;
            } else {
                current_block_91 = 3222590281903869779;
            }
        } else {
            current_block_91 = 3222590281903869779;
        }
        match current_block_91 {
            3222590281903869779 => {
                if *(*ctxt).cur as libc::c_int == '*' as i32 {
                    axis = AXIS_CHILD
                } else {
                    if name.is_null() {
                        name = xmlXPathParseNCName(ctxt)
                    }
                    if !name.is_null() {
                        axis = xmlXPathIsAxisName(name);
                        if axis as libc::c_uint != 0 as libc::c_int as libc::c_uint {
                            while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                                || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                                    && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                                || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                            {
                                if *(*ctxt).cur as libc::c_int != 0 {
                                    (*ctxt).cur = (*ctxt).cur.offset(1)
                                } else {
                                };
                            }
                            if *(*ctxt).cur as libc::c_int == ':' as i32
                                && *(*ctxt).cur.offset(1 as libc::c_int as isize) as libc::c_int
                                    == ':' as i32
                            {
                                (*ctxt).cur = (*ctxt).cur.offset(2 as libc::c_int as isize);
                                xmlFree.expect("non-null function pointer")(
                                    name as *mut libc::c_void,
                                );
                                name = 0 as *mut xmlChar
                            } else {
                                /* an element name can conflict with an axis one :-\ */
                                axis = AXIS_CHILD
                            }
                        } else {
                            axis = AXIS_CHILD
                        }
                    } else if *(*ctxt).cur as libc::c_int == '@' as i32 {
                        if *(*ctxt).cur as libc::c_int != 0 {
                            (*ctxt).cur = (*ctxt).cur.offset(1)
                        } else {
                        };
                        axis = AXIS_ATTRIBUTE
                    } else {
                        axis = AXIS_CHILD
                    }
                }
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
                    return;
                }
                name = xmlXPathCompNodeTest(ctxt, &mut test, &mut type_0, &mut prefix, name);
                if test as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                    return;
                }
                if !prefix.is_null()
                    && !(*ctxt).context.is_null()
                    && (*(*ctxt).context).flags & (1 as libc::c_int) << 0 as libc::c_int != 0
                {
                    if xmlXPathNsLookup((*ctxt).context, prefix).is_null() {
                        xmlXPathErr(ctxt, XPATH_UNDEF_PREFIX_ERROR as libc::c_int);
                    }
                }
            }
            _ => {}
        }
        op1 = (*(*ctxt).comp).last;
        (*(*ctxt).comp).last = -(1 as libc::c_int);
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1)
            } else {
            };
        }
        while *(*ctxt).cur as libc::c_int == '[' as i32 {
            xmlXPathCompPredicate(ctxt, 0 as libc::c_int);
        }
        if rangeto != 0 {
            xmlXPathCompExprAdd(
                ctxt,
                op2,
                op1,
                XPATH_OP_RANGETO,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
        } else if xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_COLLECT,
            axis as libc::c_int,
            test as libc::c_int,
            type_0 as libc::c_int,
            prefix as *mut libc::c_void,
            name as *mut libc::c_void,
        ) == -(1 as libc::c_int)
        {
            xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
        }
    };
}
/* *
 * xmlXPathCompRelativeLocationPath: * @ctxt: the XPath Parser context
 *
 *  [3]   RelativeLocationPath ::= Step
 *                     | RelativeLocationPath '/' Step
 *                     | AbbreviatedRelativeLocationPath
 *  [11]  AbbreviatedRelativeLocationPath ::= RelativeLocationPath '//' Step
 *
 * Compile a relative location path.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompRelativeLocationPath(mut ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe {
        *safe_ctxt.cur as libc::c_int == '/' as i32
            && *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(2 as libc::c_int as isize) };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                (*safe_ctxt.comp).last,
                -(1 as libc::c_int),
                XPATH_OP_COLLECT,
                AXIS_DESCENDANT_OR_SELF as libc::c_int,
                NODE_TEST_TYPE as libc::c_int,
                NODE_TYPE_NODE as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            )
        };
    } else if unsafe { *safe_ctxt.cur as libc::c_int == '/' as i32 } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
    unsafe { xmlXPathCompStep(ctxt) };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
            || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
    } {
        if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe { *safe_ctxt.cur as libc::c_int == '/' as i32 } {
        if unsafe {
            *safe_ctxt.cur as libc::c_int == '/' as i32
                && *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(2 as libc::c_int as isize) };
            while unsafe {
                *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                        && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                    || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
            } {
                if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                    unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                } else {
                };
            }
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    (*safe_ctxt.comp).last,
                    -(1 as libc::c_int),
                    XPATH_OP_COLLECT,
                    AXIS_DESCENDANT_OR_SELF as libc::c_int,
                    NODE_TEST_TYPE as libc::c_int,
                    NODE_TYPE_NODE as libc::c_int,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                )
            };
            unsafe { xmlXPathCompStep(ctxt) };
        } else if unsafe { *safe_ctxt.cur as libc::c_int == '/' as i32 } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
            while unsafe {
                *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                        && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                    || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
            } {
                if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                    unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                } else {
                };
            }
            unsafe { xmlXPathCompStep(ctxt) };
        }
        while unsafe {
            *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *safe_ctxt.cur as libc::c_int
                    && *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int
                || *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int
        } {
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
}
/* *
 * xmlXPathCompLocationPath: * @ctxt: the XPath Parser context
 *
 *  [1]   LocationPath ::= RelativeLocationPath
 *                     | AbsoluteLocationPath
 *  [2]   AbsoluteLocationPath ::= '/' RelativeLocationPath?
 *                     | AbbreviatedAbsoluteLocationPath
 *  [10]   AbbreviatedAbsoluteLocationPath ::= *                           '//' RelativeLocationPath
 *
 * Compile a location path
 *
 * // is short for /descendant-or-self::node()/. For example, * //para is short for /descendant-or-self::node()/child::para and
 * so will select any para element in the document (even a para element
 * that is a document element will be selected by //para since the
 * document element node is a child of the root node); div//para is
 * short for div/descendant-or-self::node()/child::para and so will
 * select all para descendants of div children.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompLocationPath(mut ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe { *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int }
        || 0x9 as libc::c_int <= unsafe { *safe_ctxt.cur as libc::c_int }
            && unsafe { *safe_ctxt.cur as libc::c_int <= 0xa as libc::c_int }
        || unsafe { *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int }
    {
        if unsafe { *safe_ctxt.cur as libc::c_int } != 0 {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe { *safe_ctxt.cur as libc::c_int != '/' as i32 } {
        unsafe { xmlXPathCompRelativeLocationPath(ctxt) };
    } else {
        while unsafe { *safe_ctxt.cur as libc::c_int == '/' as i32 } {
            if unsafe { *safe_ctxt.cur as libc::c_int == '/' as i32 }
                && unsafe {
                    *safe_ctxt.cur.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                }
            {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(2 as libc::c_int as isize) };
                while unsafe { *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int }
                    || 0x9 as libc::c_int <= unsafe { *safe_ctxt.cur as libc::c_int }
                        && unsafe { *safe_ctxt.cur as libc::c_int } <= 0xa as libc::c_int
                    || unsafe { *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int }
                {
                    if unsafe { *safe_ctxt.cur as libc::c_int } != 0 {
                        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                    } else {
                    };
                }
                unsafe {
                    xmlXPathCompExprAdd(
                        ctxt,
                        (*safe_ctxt.comp).last,
                        -(1 as libc::c_int),
                        XPATH_OP_COLLECT,
                        AXIS_DESCENDANT_OR_SELF as libc::c_int,
                        NODE_TEST_TYPE as libc::c_int,
                        NODE_TYPE_NODE as libc::c_int,
                        0 as *mut libc::c_void,
                        0 as *mut libc::c_void,
                    )
                };
                unsafe { xmlXPathCompRelativeLocationPath(ctxt) };
            } else if unsafe { *safe_ctxt.cur as libc::c_int == '/' as i32 } {
                if unsafe { *safe_ctxt.cur as libc::c_int } != 0 {
                    unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                } else {
                };
                while unsafe { *safe_ctxt.cur as libc::c_int == 0x20 as libc::c_int }
                    || 0x9 as libc::c_int <= unsafe { *safe_ctxt.cur as libc::c_int }
                        && unsafe { *safe_ctxt.cur as libc::c_int } <= 0xa as libc::c_int
                    || unsafe { *safe_ctxt.cur as libc::c_int == 0xd as libc::c_int }
                {
                    if unsafe { *safe_ctxt.cur as libc::c_int } != 0 {
                        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                    } else {
                    };
                }
                if unsafe { *safe_ctxt.cur as libc::c_int } != 0 as libc::c_int
                    && (0x41 as libc::c_int <= unsafe { *safe_ctxt.cur as libc::c_int }
                        && unsafe { *safe_ctxt.cur as libc::c_int } <= 0x5a as libc::c_int
                        || 0x61 as libc::c_int <= unsafe { *safe_ctxt.cur as libc::c_int }
                            && unsafe { *safe_ctxt.cur as libc::c_int } <= 0x7a as libc::c_int
                        || unsafe { *safe_ctxt.cur as libc::c_int == '_' as i32 }
                        || unsafe { *safe_ctxt.cur as libc::c_int == '.' as i32 }
                        || unsafe { *safe_ctxt.cur as libc::c_int == '@' as i32 }
                        || unsafe { *safe_ctxt.cur as libc::c_int == '*' as i32 })
                {
                    unsafe { xmlXPathCompRelativeLocationPath(ctxt) };
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return;
            }
        }
    };
}
/* DEBUG_STEP */
/* *
 * xmlXPathNodeSetFilter: * @ctxt: the XPath Parser context
 * @set: the node set to filter
 * @filterOpIndex: the index of the predicate/filter op
 * @minPos: minimum position in the filtered set (1-based) * @maxPos: maximum position in the filtered set (1-based) * @hasNsNodes: true if the node set may contain namespace nodes
 *
 * Filter a node set, keeping only nodes for which the predicate expression
 * matches. Afterwards, keep only nodes between minPos and maxPos in the
 * filtered result.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathNodeSetFilter(
    mut ctxt: xmlXPathParserContextPtr,
    mut set: xmlNodeSetPtr,
    mut filterOpIndex: libc::c_int,
    mut minPos: libc::c_int,
    mut maxPos: libc::c_int,
    mut hasNsNodes: libc::c_int,
) {
    let mut xpctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    let mut oldnode: xmlNodePtr = 0 as *mut xmlNode;
    let mut olddoc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut filterOp: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
    let mut oldcs: libc::c_int = 0;
    let mut oldpp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let safe_set = unsafe { &mut *set };

    if set.is_null() || safe_set.nodeNr == 0 as libc::c_int {
        return;
    }
    /*
     * Check if the node set contains a sufficient number of nodes for
     * the requested range.
     */
    if safe_set.nodeNr < minPos {
        unsafe { xmlXPathNodeSetClear(set, hasNsNodes) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    xpctxt = safe_ctxt.context;
    let safe_xpctxt = unsafe { &mut *xpctxt };
    oldnode = safe_xpctxt.node;
    olddoc = safe_xpctxt.doc;
    oldcs = safe_xpctxt.contextSize;
    oldpp = safe_xpctxt.proximityPosition;
    filterOp = unsafe {
        &mut *(*(*ctxt).comp).steps.offset(filterOpIndex as isize) as *mut xmlXPathStepOp
    };
    safe_xpctxt.contextSize = safe_set.nodeNr;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    pos = 1 as libc::c_int;
    while i < safe_set.nodeNr {
        let mut node: xmlNodePtr = unsafe { *safe_set.nodeTab.offset(i as isize) };
        let mut res: libc::c_int = 0;
        unsafe { (*xpctxt).node = node };
        unsafe { (*xpctxt).proximityPosition = i + 1 as libc::c_int };
        /*
         * Also set the xpath document in case things like
         * key() are evaluated in the predicate.
         *
         * TODO: Get real doc for namespace nodes.
         */
        let safe_node = unsafe { &mut *node };
        if safe_node.type_0 as libc::c_uint != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            && !safe_node.doc.is_null()
        {
            safe_xpctxt.doc = safe_node.doc
        }
        res = unsafe { xmlXPathCompOpEvalToBoolean(ctxt, filterOp, 1 as libc::c_int) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            break;
        }
        if res < 0 as libc::c_int {
            /* Shouldn't happen */
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
            break;
        } else {
            if res != 0 as libc::c_int && (pos >= minPos && pos <= maxPos) {
                if i != j {
                    unsafe {
                        let ref mut fresh73 = *(*set).nodeTab.offset(j as isize);
                        *fresh73 = node;
                        let ref mut fresh74 = *(*set).nodeTab.offset(i as isize);
                        *fresh74 = 0 as xmlNodePtr
                    }
                }
                j += 1 as libc::c_int
            } else {
                /* Remove the entry from the initial node set. */
                unsafe {
                    let ref mut fresh75 = *(*set).nodeTab.offset(i as isize);
                    *fresh75 = 0 as xmlNodePtr;
                    if safe_node.type_0 as libc::c_uint
                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                    {
                        xmlXPathNodeSetFreeNs(node as xmlNsPtr);
                    }
                }
            }
            if res != 0 as libc::c_int {
                if pos == maxPos {
                    i += 1 as libc::c_int;
                    break;
                } else {
                    pos += 1 as libc::c_int
                }
            }
            i += 1
        }
    }
    /* Free remaining nodes. */
    if hasNsNodes != 0 {
        while i < safe_set.nodeNr {
            let mut node_0: xmlNodePtr = unsafe { *(*set).nodeTab.offset(i as isize) };
            if !node_0.is_null()
                && unsafe { (*node_0).type_0 as libc::c_uint }
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                unsafe { xmlXPathNodeSetFreeNs(node_0 as xmlNsPtr) };
            }
            i += 1
        }
    }
    safe_set.nodeNr = j;
    /* If too many elements were removed, shrink table to preserve memory. */
    if safe_set.nodeMax > 10 as libc::c_int && safe_set.nodeNr < safe_set.nodeMax / 2 as libc::c_int
    {
        let mut tmp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        let mut nodeMax: libc::c_int = safe_set.nodeNr;
        if nodeMax < 10 as libc::c_int {
            nodeMax = 10 as libc::c_int
        }
        tmp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                safe_set.nodeTab as *mut libc::c_void,
                (nodeMax as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            ) as *mut xmlNodePtr
        };
        if tmp.is_null() {
            unsafe {
                xmlXPathPErrMemory(
                    ctxt,
                    b"shrinking nodeset\n\x00" as *const u8 as *const libc::c_char,
                )
            };
        } else {
            safe_set.nodeTab = tmp;
            safe_set.nodeMax = nodeMax
        }
    }
    safe_xpctxt.node = oldnode;
    safe_xpctxt.doc = olddoc;
    safe_xpctxt.contextSize = oldcs;
    safe_xpctxt.proximityPosition = oldpp;
}
/* *
 * xmlXPathLocationSetFilter: * @ctxt: the XPath Parser context
 * @locset: the location set to filter
 * @filterOpIndex: the index of the predicate/filter op
 * @minPos: minimum position in the filtered set (1-based) * @maxPos: maximum position in the filtered set (1-based) *
 * Filter a location set, keeping only nodes for which the predicate
 * expression matches. Afterwards, keep only nodes between minPos and maxPos
 * in the filtered result.
 */
#[cfg(LIBXML_XPTR_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathLocationSetFilter(
    mut ctxt: xmlXPathParserContextPtr,
    mut locset: xmlLocationSetPtr,
    mut filterOpIndex: libc::c_int,
    mut minPos: libc::c_int,
    mut maxPos: libc::c_int,
) {
    let mut xpctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    let mut oldnode: xmlNodePtr = 0 as *mut xmlNode;
    let mut olddoc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut filterOp: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
    let mut oldcs: libc::c_int = 0;
    let mut oldpp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let safe_locset = unsafe { &mut *locset };
    if locset.is_null()
        || safe_locset.locNr == 0 as libc::c_int
        || filterOpIndex == -(1 as libc::c_int)
    {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    xpctxt = safe_ctxt.context;
    let safe_xpctxt = unsafe { &mut *xpctxt };
    oldnode = safe_xpctxt.node;
    olddoc = safe_xpctxt.doc;
    oldcs = safe_xpctxt.contextSize;
    oldpp = safe_xpctxt.proximityPosition;

    filterOp = unsafe {
        &mut *(*(*ctxt).comp).steps.offset(filterOpIndex as isize) as *mut xmlXPathStepOp
    };
    safe_xpctxt.contextSize = safe_locset.locNr;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    pos = 1 as libc::c_int;

    while i < safe_locset.locNr {
        let mut contextNode: xmlNodePtr =
            unsafe { (**(*locset).locTab.offset(i as isize)).user as xmlNodePtr };
        let mut res: libc::c_int = 0;
        safe_xpctxt.node = contextNode;
        safe_xpctxt.proximityPosition = i + 1 as libc::c_int;

        /*
         * Also set the xpath document in case things like
         * key() are evaluated in the predicate.
         *
         * TODO: Get real doc for namespace nodes.
         */
        let safe_contextNode = unsafe { &mut *contextNode };
        if safe_contextNode.type_0 as libc::c_uint
            != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            && !safe_contextNode.doc.is_null()
        {
            safe_xpctxt.doc = safe_contextNode.doc
        }

        res = unsafe { xmlXPathCompOpEvalToBoolean(ctxt, filterOp, 1 as libc::c_int) };

        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            break;
        }

        if res < 0 as libc::c_int {
            /* Shouldn't happen */
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
            break;
        } else {
            if res != 0 as libc::c_int && (pos >= minPos && pos <= maxPos) {
                if i != j {
                    unsafe {
                        let ref mut fresh76 = *(*locset).locTab.offset(j as isize);
                        *fresh76 = *(*locset).locTab.offset(i as isize);
                        let ref mut fresh77 = *(*locset).locTab.offset(i as isize);
                        *fresh77 = 0 as xmlXPathObjectPtr
                    }
                }
                j += 1 as libc::c_int
            } else {
                /* Remove the entry from the initial location set. */
                unsafe { xmlXPathFreeObject(*(*locset).locTab.offset(i as isize)) };
                unsafe {
                    let ref mut fresh78 = *(*locset).locTab.offset(i as isize);
                    *fresh78 = 0 as xmlXPathObjectPtr
                }
            }
            if res != 0 as libc::c_int {
                if pos == maxPos {
                    i += 1 as libc::c_int;
                    break;
                } else {
                    pos += 1 as libc::c_int
                }
            }
            i += 1
        }
    }
    /* Free remaining nodes. */
    while i < safe_locset.locNr {
        unsafe { xmlXPathFreeObject(*(*locset).locTab.offset(i as isize)) };
        i += 1
    }

    safe_locset.locNr = j;
    /* If too many elements were removed, shrink table to preserve memory. */
    if safe_locset.locMax > 10 as libc::c_int
        && safe_locset.locNr < safe_locset.locMax / 2 as libc::c_int
    {
        let mut tmp: *mut xmlXPathObjectPtr = 0 as *mut xmlXPathObjectPtr;
        let mut locMax: libc::c_int = safe_locset.locNr;
        if locMax < 10 as libc::c_int {
            locMax = 10 as libc::c_int
        }
        tmp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*locset).locTab as *mut libc::c_void,
                (locMax as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>() as libc::c_ulong),
            ) as *mut xmlXPathObjectPtr
        };
        if tmp.is_null() {
            unsafe {
                xmlXPathPErrMemory(
                    ctxt,
                    b"shrinking locset\n\x00" as *const u8 as *const libc::c_char,
                )
            };
        } else {
            safe_locset.locTab = tmp;
            safe_locset.locMax = locMax
        }
    }
    safe_xpctxt.node = oldnode;
    safe_xpctxt.doc = olddoc;
    safe_xpctxt.contextSize = oldcs;
    safe_xpctxt.proximityPosition = oldpp;
}
/* LIBXML_XPTR_ENABLED */
/* *
 * xmlXPathCompOpEvalPredicate: * @ctxt: the XPath Parser context
 * @op: the predicate op
 * @set: the node set to filter
 * @minPos: minimum position in the filtered set (1-based) * @maxPos: maximum position in the filtered set (1-based) * @hasNsNodes: true if the node set may contain namespace nodes
 *
 * Filter a node set, keeping only nodes for which the sequence of predicate
 * expressions matches. Afterwards, keep only nodes between minPos and maxPos
 * in the filtered result.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompOpEvalPredicate(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut set: xmlNodeSetPtr,
    mut minPos: libc::c_int,
    mut maxPos: libc::c_int,
    mut hasNsNodes: libc::c_int,
) {
    let safe_op = unsafe { &mut *op };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_op.ch1 != -(1 as libc::c_int) {
        let mut comp: xmlXPathCompExprPtr = safe_ctxt.comp;
        /*
         * Process inner predicates first.
         */
        let safe_comp = unsafe { &mut *comp };
        if unsafe { (*safe_comp.steps.offset(safe_op.ch1 as isize)).op as libc::c_uint }
            != XPATH_OP_PREDICATE as libc::c_int as libc::c_uint
        {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlXPathCompOpEvalPredicate: Expected a predicate\n\x00" as *const u8
                        as *const libc::c_char,
                )
            };
            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
            return;
        }

        if unsafe { (*safe_ctxt.context).depth } >= 5000 as libc::c_int {
            unsafe { xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as libc::c_int) };
            return;
        }
        unsafe { (*safe_ctxt.context).depth += 1 as libc::c_int };
        unsafe {
            xmlXPathCompOpEvalPredicate(
                ctxt,
                &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                set,
                1 as libc::c_int,
                (*set).nodeNr,
                hasNsNodes,
            )
        };
        unsafe { (*safe_ctxt.context).depth -= 1 as libc::c_int };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
    }
    if safe_op.ch2 != -(1 as libc::c_int) {
        unsafe { xmlXPathNodeSetFilter(ctxt, set, safe_op.ch2, minPos, maxPos, hasNsNodes) };
    };
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathIsPositionalPredicate(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut maxPos: *mut libc::c_int,
) -> libc::c_int {
    let mut exprOp: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
    /*
     * BIG NOTE: This is not intended for XPATH_OP_FILTER yet!
     */
    /*
     * If not -1, then ch1 will point to: * 1) For predicates (XPATH_OP_PREDICATE): *    - an inner predicate operator
     * 2) For filters (XPATH_OP_FILTER): *    - an inner filter operator OR
     *    - an expression selecting the node set.
     *      E.g. "key('a', 'b')" or "(//foo | //bar)".
     */
    let safe_op = unsafe { &mut *op };
    if safe_op.op as libc::c_uint != XPATH_OP_PREDICATE as libc::c_int as libc::c_uint
        && safe_op.op as libc::c_uint != XPATH_OP_FILTER as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if safe_op.ch2 != -(1 as libc::c_int) {
        exprOp = unsafe {
            &mut *(*(*ctxt).comp).steps.offset(safe_op.ch2 as isize) as *mut xmlXPathStepOp
        }
    } else {
        return 0 as libc::c_int;
    }
    if !exprOp.is_null()
        && unsafe { (*exprOp).op as libc::c_uint } == XPATH_OP_VALUE as libc::c_int as libc::c_uint
        && !unsafe { (*exprOp).value4.is_null() }
        && unsafe { (*((*exprOp).value4 as xmlXPathObjectPtr)).type_0 as libc::c_uint }
            == XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        let mut floatval: libc::c_double =
            unsafe { (*((*exprOp).value4 as xmlXPathObjectPtr)).floatval };
        /*
        	* We have a "[n]" predicate here.
        	* TODO: Unfortunately this simplistic test here is not
        	* able to detect a position() predicate in compound
        	* expressions like "[@attr = 'a" and position() = 1],
        	* and even not the usage of position() in
        	* "[position() = 1]"; thus - obviously - a position-range,
        	* like it "[position() < 5]", is also not detected.
        	* Maybe we could rewrite the AST to ease the optimization.
        	*/
        if floatval > (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
            && floatval < 2147483647 as libc::c_int as libc::c_double
        {
            unsafe { *maxPos = floatval as libc::c_int };
            if unsafe { floatval == *maxPos as libc::c_double } {
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathNodeCollectAndTest(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut first: *mut xmlNodePtr,
    mut last: *mut xmlNodePtr,
    mut toBool: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let safe_op = unsafe { &mut *op };
    let mut axis: xmlXPathAxisVal = safe_op.value as xmlXPathAxisVal;
    let mut test: xmlXPathTestVal = safe_op.value2 as xmlXPathTestVal;
    let mut type_0: xmlXPathTypeVal = safe_op.value3 as xmlXPathTypeVal;
    let mut prefix: *const xmlChar = safe_op.value4 as *const xmlChar;
    let mut name: *const xmlChar = safe_op.value5 as *const xmlChar;
    let mut URI: *const xmlChar = 0 as *const xmlChar;
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut hasNsNodes: libc::c_int = 0 as libc::c_int;
    /* The popped object holding the context nodes */
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    /* The set of context nodes for the node tests */
    let mut contextSeq: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut contextIdx: libc::c_int = 0;
    let mut contextNode: xmlNodePtr = 0 as *mut xmlNode;
    /* The final resulting node set wrt to all context nodes */
    let mut outSeq: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    /*
     * The temporary resulting node set wrt 1 context node.
     * Used to feed predicate evaluation.
     */
    let mut seq: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    /* First predicate operator */
    let mut predOp: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp; /* The requested position() (when a "[n]" predicate) */
    let mut maxPos: libc::c_int = 0;
    let mut hasPredicateRange: libc::c_int = 0;
    let mut hasAxisRange: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut breakOnFirstHit: libc::c_int = 0;
    let mut next: xmlXPathTraversalFunction = None;
    let mut addNode: Option<unsafe extern "C" fn(_: xmlNodeSetPtr, _: xmlNodePtr) -> libc::c_int> =
        None;
    let mut mergeAndClear: xmlXPathNodeSetMergeFunction = None;
    let mut oldContextNode: xmlNodePtr = 0 as *mut xmlNode;
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut xpctxt: xmlXPathContextPtr = safe_ctxt.context;
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NODESET as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return 0 as libc::c_int;
    }
    obj = unsafe { valuePop(ctxt) };
    /*
     * Setup namespaces.
     */
    if !prefix.is_null() {
        URI = unsafe { xmlXPathNsLookup(xpctxt, prefix) };
        if URI.is_null() {
            unsafe { xmlXPathReleaseObject(xpctxt, obj) };
            unsafe { xmlXPathErr(ctxt, XPATH_UNDEF_PREFIX_ERROR as libc::c_int) };
            return 0 as libc::c_int;
        }
    }
    /*
     * Setup axis.
     *
     * MAYBE FUTURE TODO: merging optimizations: * - If the nodes to be traversed wrt to the initial nodes and
     *   the current axis cannot overlap, then we could avoid searching
     *   for duplicates during the merge.
     *   But the question is how/when to evaluate if they cannot overlap.
     *   Example: if we know that for two initial nodes, the one is
     *   not in the ancestor-or-self axis of the other, then we could safely
     *   avoid a duplicate-aware merge, if the axis to be traversed is e.g.
     *   the descendant-or-self axis.
     */
    mergeAndClear = unsafe {
        Some(
            xmlXPathNodeSetMergeAndClear
                as unsafe extern "C" fn(_: xmlNodeSetPtr, _: xmlNodeSetPtr) -> xmlNodeSetPtr,
        )
    };

    match axis as libc::c_uint {
        1 => {
            first = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextAncestor
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            }
        }
        2 => {
            first = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextAncestorOrSelf
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            }
        }
        3 => {
            first = 0 as *mut xmlNodePtr;
            last = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextAttribute
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            };
            mergeAndClear = unsafe {
                Some(
                    xmlXPathNodeSetMergeAndClearNoDupls
                        as unsafe extern "C" fn(
                            _: xmlNodeSetPtr,
                            _: xmlNodeSetPtr,
                        ) -> xmlNodeSetPtr,
                )
            }
        }
        4 => {
            last = 0 as *mut xmlNodePtr;
            if (test as libc::c_uint == NODE_TEST_NAME as libc::c_int as libc::c_uint
                || test as libc::c_uint == NODE_TEST_ALL as libc::c_int as libc::c_uint)
                && type_0 as libc::c_uint == NODE_TYPE_NODE as libc::c_int as libc::c_uint
            {
                /*
                	* Optimization if an element node type is 'element'.
                	*/
                next = unsafe {
                    Some(
                        xmlXPathNextChildElement
                            as unsafe extern "C" fn(
                                _: xmlXPathParserContextPtr,
                                _: xmlNodePtr,
                            ) -> xmlNodePtr,
                    )
                }
            } else {
                next = unsafe {
                    Some(
                        xmlXPathNextChild
                            as unsafe extern "C" fn(
                                _: xmlXPathParserContextPtr,
                                _: xmlNodePtr,
                            ) -> xmlNodePtr,
                    )
                }
            }
            mergeAndClear = unsafe {
                Some(
                    xmlXPathNodeSetMergeAndClearNoDupls
                        as unsafe extern "C" fn(
                            _: xmlNodeSetPtr,
                            _: xmlNodeSetPtr,
                        ) -> xmlNodeSetPtr,
                )
            }
        }
        5 => {
            last = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextDescendant
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            }
        }
        6 => {
            last = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextDescendantOrSelf
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            }
        }
        7 => {
            last = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextFollowing
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            }
        }
        8 => {
            last = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextFollowingSibling
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            }
        }
        9 => {
            first = 0 as *mut xmlNodePtr;
            last = 0 as *mut xmlNodePtr;
            next = unsafe {
                ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                    >,
                    xmlXPathTraversalFunction,
                >(Some(
                    xmlXPathNextNamespace
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                ))
            };
            mergeAndClear = unsafe {
                Some(
                    xmlXPathNodeSetMergeAndClearNoDupls
                        as unsafe extern "C" fn(
                            _: xmlNodeSetPtr,
                            _: xmlNodeSetPtr,
                        ) -> xmlNodeSetPtr,
                )
            }
        }
        10 => {
            first = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextParent
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            }
        }
        11 => {
            first = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextPrecedingInternal
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            }
        }
        12 => {
            first = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextPrecedingSibling
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            }
        }
        13 => {
            first = 0 as *mut xmlNodePtr;
            last = 0 as *mut xmlNodePtr;
            next = unsafe {
                Some(
                    xmlXPathNextSelf
                        as unsafe extern "C" fn(
                            _: xmlXPathParserContextPtr,
                            _: xmlNodePtr,
                        ) -> xmlNodePtr,
                )
            };
            mergeAndClear = unsafe {
                Some(
                    xmlXPathNodeSetMergeAndClearNoDupls
                        as unsafe extern "C" fn(
                            _: xmlNodeSetPtr,
                            _: xmlNodeSetPtr,
                        ) -> xmlNodeSetPtr,
                )
            }
        }
        _ => {}
    }

    if next.is_none() {
        unsafe { xmlXPathReleaseObject(xpctxt, obj) };
        return 0 as libc::c_int;
    }
    let safe_obj = unsafe { &mut *obj };
    contextSeq = safe_obj.nodesetval;
    let safe_contextSeq = unsafe { &mut *contextSeq };
    if contextSeq.is_null() || safe_contextSeq.nodeNr <= 0 as libc::c_int {
        unsafe { xmlXPathReleaseObject(xpctxt, obj) };
        unsafe { valuePush(ctxt, xmlXPathCacheWrapNodeSet(xpctxt, 0 as xmlNodeSetPtr)) };
        return 0 as libc::c_int;
    }
    /*
     * Predicate optimization --------------------------------------------- * If this step has a last predicate, which contains a position(), * then we'll optimize (although not exactly "position()", but only
     * the  short-hand form, i.e., "[n]".
     *
     * Example - expression "/foo[parent::bar][1]": *
     * COLLECT 'child' 'name' 'node' foo    -- op (we are here) *   ROOT                               -- op->ch1
     *   PREDICATE                          -- op->ch2 (predOp) *     PREDICATE                          -- predOp->ch1 = [parent::bar]
     *       SORT
     *         COLLECT  'parent' 'name' 'node' bar
     *           NODE
     *     ELEM Object is a number : 1        -- predOp->ch2 = [1]
     *
     */
    maxPos = 0 as libc::c_int;
    predOp = 0 as xmlXPathStepOpPtr;
    hasPredicateRange = 0 as libc::c_int;
    hasAxisRange = 0 as libc::c_int;
    if safe_op.ch2 != -(1 as libc::c_int) {
        /*
        	* There's at least one predicate. 16 == XPATH_OP_PREDICATE
        	*/
        predOp = unsafe {
            &mut *(*safe_ctxt.comp).steps.offset(safe_op.ch2 as isize) as *mut xmlXPathStepOp
        };
        if unsafe { xmlXPathIsPositionalPredicate(ctxt, predOp, &mut maxPos) != 0 } {
            if unsafe { (*predOp).ch1 != -(1 as libc::c_int) } {
                /*
                	* Use the next inner predicate operator.
                	*/
                predOp = unsafe {
                    &mut *(*safe_ctxt.comp).steps.offset((*predOp).ch1 as isize)
                        as *mut xmlXPathStepOp
                };
                hasPredicateRange = 1 as libc::c_int
            } else {
                /*
                	* There's no other predicate than the [n] predicate.
                	*/
                predOp = 0 as xmlXPathStepOpPtr;
                hasAxisRange = 1 as libc::c_int
            }
        }
    }

    breakOnFirstHit = if toBool != 0 && predOp.is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    /*
     * Axis traversal ----------------------------------------------------- */
    /*
     * 2.3 Node Tests
     *  - For the attribute axis, the principal node type is attribute.
     *  - For the namespace axis, the principal node type is namespace.
     *  - For other axes, the principal node type is element.
     *
     * A node test * is true for any node of the
     * principal node type. For example, child::* will
     * select all element children of the context node
     */
    let safe_xpctxt = unsafe { &mut *xpctxt };
    oldContextNode = safe_xpctxt.node;
    addNode = unsafe {
        Some(
            xmlXPathNodeSetAddUnique
                as unsafe extern "C" fn(_: xmlNodeSetPtr, _: xmlNodePtr) -> libc::c_int,
        )
    };
    outSeq = 0 as xmlNodeSetPtr;
    seq = 0 as xmlNodeSetPtr;
    contextNode = 0 as xmlNodePtr;
    contextIdx = 0 as libc::c_int;
    's_486: while (contextIdx < safe_contextSeq.nodeNr || !contextNode.is_null())
        && safe_ctxt.error == XPATH_EXPRESSION_OK as libc::c_int
    {
        let fresh79 = contextIdx;
        contextIdx = contextIdx + 1;
        safe_xpctxt.node = unsafe { *safe_contextSeq.nodeTab.offset(fresh79 as isize) };
        if seq.is_null() {
            seq = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
            if seq.is_null() {
                /* TODO: Propagate memory error. */
                total = 0 as libc::c_int;
                break;
            }
        }
        /*
         * Traverse the axis and test the nodes.
         */
        pos = 0 as libc::c_int;
        cur = 0 as xmlNodePtr;
        hasNsNodes = 0 as libc::c_int;

        loop {
            if unsafe {
                (*safe_ctxt.context).opLimit != 0 as libc::c_int as libc::c_ulong
                    && xmlXPathCheckOpLimit(ctxt, 1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int
            } {
                break 's_486;
                /* switch(test) */
            }
            cur = unsafe { next.expect("non-null function pointer")(ctxt, cur) };
            if cur.is_null() {
                current_block = 6300945584809257457;
                break;
            }
            /*
             * QUESTION TODO: What does the "first" and "last" stuff do?
             */

            if !first.is_null() && !unsafe { (*first).is_null() } {
                if unsafe { *first == cur } {
                    current_block = 6300945584809257457;
                    break;
                }
                match () {
                    #[cfg(XP_OPTIMIZED_NON_ELEM_COMPARISON)]
                    _ => {
                        if total % 256 as libc::c_int == 0 as libc::c_int
                            && unsafe { xmlXPathCmpNodesExt(*first, cur) } >= 0 as libc::c_int
                        {
                            current_block = 6300945584809257457;
                            break;
                        }
                    }
                    #[cfg(not(XP_OPTIMIZED_NON_ELEM_COMPARISON))]
                    _ => {
                        if total % 256 as libc::c_int == 0 as libc::c_int
                            && unsafe { xmlXPathCmpNodes(*first, cur) } >= 0 as libc::c_int
                        {
                            current_block = 6300945584809257457;
                            break;
                        }
                    }
                };
            }
            if !last.is_null() && !unsafe { (*last).is_null() } {
                if unsafe { *last == cur } {
                    current_block = 6300945584809257457;
                    break;
                }
                match () {
                    #[cfg(XP_OPTIMIZED_NON_ELEM_COMPARISON)]
                    _ => {
                        if total % 256 as libc::c_int == 0 as libc::c_int
                            && unsafe { xmlXPathCmpNodesExt(cur, *last) } >= 0 as libc::c_int
                        {
                            current_block = 6300945584809257457;
                            break;
                        }
                    }
                    #[cfg(not(XP_OPTIMIZED_NON_ELEM_COMPARISON))]
                    _ => {
                        if total % 256 as libc::c_int == 0 as libc::c_int
                            && unsafe { xmlXPathCmpNodes(cur, *last) >= 0 } as libc::c_int
                        {
                            current_block = 6300945584809257457;
                            break;
                        }
                    }
                };
            }
            total += 1;
            match test as libc::c_uint {
                0 => {
                    total = 0 as libc::c_int;
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Internal error at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                            b"xpath.c\x00" as *const u8 as *const libc::c_char,
                            12269 as libc::c_int,
                        )
                    };
                    break 's_486;
                }
                1 => {
                    if type_0 as libc::c_uint == NODE_TYPE_NODE as libc::c_int as libc::c_uint {
                        match unsafe { (*cur).type_0 as libc::c_uint } {
                            9 | 13 | 21 | 1 | 2 | 7 | 8 | 4 | 3 => {
                                current_block = 16677367482302331965;
                                match current_block {
                                    16677367482302331965 => {
                                        if hasAxisRange != 0 as libc::c_int {
                                            pos += 1;
                                            if pos == maxPos {
                                                if unsafe {
                                                    addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    )
                                                } < 0 as libc::c_int
                                                {
                                                    safe_ctxt.error =
                                                        XPATH_MEMORY_ERROR as libc::c_int
                                                }
                                                current_block = 12278438173206364583;
                                                break;
                                            }
                                        } else {
                                            if unsafe {
                                                addNode.expect("non-null function pointer")(
                                                    seq, cur,
                                                )
                                            } < 0 as libc::c_int
                                            {
                                                safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                            }
                                            if breakOnFirstHit != 0 {
                                                current_block = 795179968803393002;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        if axis as libc::c_uint
                                            == AXIS_NAMESPACE as libc::c_int as libc::c_uint
                                        {
                                            if hasAxisRange != 0 as libc::c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    hasNsNodes = 1 as libc::c_int;
                                                    if unsafe {
                                                        xmlXPathNodeSetAddNs(
                                                            seq,
                                                            safe_xpctxt.node,
                                                            cur as xmlNsPtr,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    current_block = 12278438173206364583;
                                                    break;
                                                }
                                            } else {
                                                hasNsNodes = 1 as libc::c_int;
                                                if unsafe {
                                                    xmlXPathNodeSetAddNs(
                                                        seq,
                                                        safe_xpctxt.node,
                                                        cur as xmlNsPtr,
                                                    )
                                                } < 0 as libc::c_int
                                                {
                                                    safe_ctxt.error =
                                                        XPATH_MEMORY_ERROR as libc::c_int
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 795179968803393002;
                                                    break;
                                                }
                                            }
                                        } else {
                                            hasNsNodes = 1 as libc::c_int;
                                            if hasAxisRange != 0 as libc::c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    current_block = 12278438173206364583;
                                                    break;
                                                }
                                            } else {
                                                if unsafe {
                                                    addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    )
                                                } < 0 as libc::c_int
                                                {
                                                    safe_ctxt.error =
                                                        XPATH_MEMORY_ERROR as libc::c_int
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 795179968803393002;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            18 => {
                                current_block = 16937825661756021828;
                                match current_block {
                                    16677367482302331965 => {
                                        if hasAxisRange != 0 as libc::c_int {
                                            pos += 1;
                                            if pos == maxPos {
                                                if unsafe {
                                                    addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    )
                                                } < 0 as libc::c_int
                                                {
                                                    safe_ctxt.error =
                                                        XPATH_MEMORY_ERROR as libc::c_int
                                                }
                                                current_block = 12278438173206364583;
                                                break;
                                            }
                                        } else {
                                            if unsafe {
                                                addNode.expect("non-null function pointer")(
                                                    seq, cur,
                                                )
                                            } < 0 as libc::c_int
                                            {
                                                safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                            }
                                            if breakOnFirstHit != 0 {
                                                current_block = 795179968803393002;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        if axis as libc::c_uint
                                            == AXIS_NAMESPACE as libc::c_int as libc::c_uint
                                        {
                                            if hasAxisRange != 0 as libc::c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    hasNsNodes = 1 as libc::c_int;
                                                    if unsafe {
                                                        xmlXPathNodeSetAddNs(
                                                            seq,
                                                            safe_xpctxt.node,
                                                            cur as xmlNsPtr,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    current_block = 12278438173206364583;
                                                    break;
                                                }
                                            } else {
                                                hasNsNodes = 1 as libc::c_int;
                                                if unsafe {
                                                    xmlXPathNodeSetAddNs(
                                                        seq,
                                                        safe_xpctxt.node,
                                                        cur as xmlNsPtr,
                                                    )
                                                } < 0 as libc::c_int
                                                {
                                                    safe_ctxt.error =
                                                        XPATH_MEMORY_ERROR as libc::c_int
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 795179968803393002;
                                                    break;
                                                }
                                            }
                                        } else {
                                            hasNsNodes = 1 as libc::c_int;
                                            if hasAxisRange != 0 as libc::c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    current_block = 12278438173206364583;
                                                    break;
                                                }
                                            } else {
                                                if unsafe {
                                                    addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    )
                                                } < 0 as libc::c_int
                                                {
                                                    safe_ctxt.error =
                                                        XPATH_MEMORY_ERROR as libc::c_int
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 795179968803393002;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else if unsafe { (*cur).type_0 as libc::c_uint }
                        == type_0 as xmlElementType as libc::c_uint
                    {
                        if unsafe { (*cur).type_0 as libc::c_uint }
                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        {
                            if hasAxisRange != 0 as libc::c_int {
                                pos += 1;
                                if pos == maxPos {
                                    hasNsNodes = 1 as libc::c_int;
                                    if unsafe {
                                        xmlXPathNodeSetAddNs(seq, safe_xpctxt.node, cur as xmlNsPtr)
                                    } < 0 as libc::c_int
                                    {
                                        safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                    }
                                    current_block = 12278438173206364583;
                                    break;
                                }
                            } else {
                                hasNsNodes = 1 as libc::c_int;
                                if unsafe {
                                    xmlXPathNodeSetAddNs(seq, safe_xpctxt.node, cur as xmlNsPtr)
                                } < 0 as libc::c_int
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 795179968803393002;
                                    break;
                                }
                            }
                        } else if hasAxisRange != 0 as libc::c_int {
                            pos += 1;
                            if pos == maxPos {
                                if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                    < 0 as libc::c_int
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                }
                                current_block = 12278438173206364583;
                                break;
                            }
                        } else {
                            if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                < 0 as libc::c_int
                            {
                                safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 795179968803393002;
                                break;
                            }
                        }
                    } else if type_0 as libc::c_uint
                        == NODE_TYPE_TEXT as libc::c_int as libc::c_uint
                        && unsafe { (*cur).type_0 as libc::c_uint }
                            == XML_CDATA_SECTION_NODE as libc::c_int as libc::c_uint
                    {
                        if hasAxisRange != 0 as libc::c_int {
                            pos += 1;
                            if pos == maxPos {
                                if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                    < 0 as libc::c_int
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                }
                                current_block = 12278438173206364583;
                                break;
                            }
                        } else {
                            if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                < 0 as libc::c_int
                            {
                                safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 795179968803393002;
                                break;
                            }
                        }
                    }
                }
                2 => {
                    if unsafe { (*cur).type_0 as libc::c_uint }
                        == XML_PI_NODE as libc::c_int as libc::c_uint
                        && (name.is_null() || unsafe { xmlStrEqual(name, (*cur).name) } != 0)
                    {
                        if hasAxisRange != 0 as libc::c_int {
                            pos += 1;
                            if pos == maxPos {
                                if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                    < 0 as libc::c_int
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                }
                                current_block = 12278438173206364583;
                                break;
                            }
                        } else {
                            if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                < 0 as libc::c_int
                            {
                                safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 795179968803393002;
                                break;
                            }
                        }
                    }
                }
                3 => {
                    if axis as libc::c_uint == AXIS_ATTRIBUTE as libc::c_int as libc::c_uint {
                        if unsafe { (*cur).type_0 as libc::c_uint }
                            == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
                        {
                            if prefix.is_null() {
                                if hasAxisRange != 0 as libc::c_int {
                                    pos += 1;
                                    if pos == maxPos {
                                        if unsafe {
                                            addNode.expect("non-null function pointer")(seq, cur)
                                        } < 0 as libc::c_int
                                        {
                                            safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                        }
                                        current_block = 12278438173206364583;
                                        break;
                                    }
                                } else {
                                    if unsafe {
                                        addNode.expect("non-null function pointer")(seq, cur)
                                    } < 0 as libc::c_int
                                    {
                                        safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                    }
                                    if breakOnFirstHit != 0 {
                                        current_block = 795179968803393002;
                                        break;
                                    }
                                }
                            } else if !unsafe { (*cur).ns.is_null() }
                                && unsafe { xmlStrEqual(URI, (*(*cur).ns).href) } != 0
                            {
                                if hasAxisRange != 0 as libc::c_int {
                                    pos += 1;
                                    if pos == maxPos {
                                        if unsafe {
                                            addNode.expect("non-null function pointer")(seq, cur)
                                        } < 0 as libc::c_int
                                        {
                                            safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                        }
                                        current_block = 12278438173206364583;
                                        break;
                                    }
                                } else {
                                    if unsafe {
                                        addNode.expect("non-null function pointer")(seq, cur)
                                    } < 0 as libc::c_int
                                    {
                                        safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                    }
                                    if breakOnFirstHit != 0 {
                                        current_block = 795179968803393002;
                                        break;
                                    }
                                }
                            }
                        }
                    } else if axis as libc::c_uint == AXIS_NAMESPACE as libc::c_int as libc::c_uint
                    {
                        if unsafe { (*cur).type_0 as libc::c_uint }
                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        {
                            if hasAxisRange != 0 as libc::c_int {
                                pos += 1;
                                if pos == maxPos {
                                    hasNsNodes = 1 as libc::c_int;
                                    if unsafe {
                                        xmlXPathNodeSetAddNs(seq, safe_xpctxt.node, cur as xmlNsPtr)
                                    } < 0 as libc::c_int
                                    {
                                        safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                    }
                                    current_block = 12278438173206364583;
                                    break;
                                }
                            } else {
                                hasNsNodes = 1 as libc::c_int;
                                if unsafe {
                                    xmlXPathNodeSetAddNs(seq, safe_xpctxt.node, cur as xmlNsPtr)
                                } < 0 as libc::c_int
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 795179968803393002;
                                    break;
                                }
                            }
                        }
                    } else if unsafe { (*cur).type_0 as libc::c_uint }
                        == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        if prefix.is_null() {
                            if hasAxisRange != 0 as libc::c_int {
                                pos += 1;
                                if pos == maxPos {
                                    if unsafe {
                                        addNode.expect("non-null function pointer")(seq, cur)
                                    } < 0 as libc::c_int
                                    {
                                        safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                    }
                                    current_block = 12278438173206364583;
                                    break;
                                }
                            } else {
                                if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                    < 0 as libc::c_int
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 795179968803393002;
                                    break;
                                }
                            }
                        } else if !unsafe { (*cur).ns.is_null() }
                            && unsafe { xmlStrEqual(URI, (*(*cur).ns).href) } != 0
                        {
                            if hasAxisRange != 0 as libc::c_int {
                                pos += 1;
                                if pos == maxPos {
                                    if unsafe {
                                        addNode.expect("non-null function pointer")(seq, cur)
                                    } < 0 as libc::c_int
                                    {
                                        safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                    }
                                    current_block = 12278438173206364583;
                                    break;
                                }
                            } else {
                                if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                    < 0 as libc::c_int
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as libc::c_int
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 795179968803393002;
                                    break;
                                }
                            }
                        }
                    }
                }
                4 => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Unimplemented block at %s:%d\n\x00" as *const u8
                                as *const libc::c_char,
                            b"xpath.c\x00" as *const u8 as *const libc::c_char,
                            12350 as libc::c_int,
                        )
                    };
                }
                5 => {
                    if axis as libc::c_uint == AXIS_ATTRIBUTE as libc::c_int as libc::c_uint {
                        if unsafe { (*cur).type_0 as libc::c_uint }
                            != XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
                        {
                            current_block = 2652804691515851435;
                        } else {
                            current_block = 8422527538794739384;
                        }
                    } else if axis as libc::c_uint == AXIS_NAMESPACE as libc::c_int as libc::c_uint
                    {
                        if unsafe { (*cur).type_0 as libc::c_uint }
                            != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        {
                            current_block = 2652804691515851435;
                        } else {
                            current_block = 8422527538794739384;
                        }
                    } else if unsafe { (*cur).type_0 as libc::c_uint }
                        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        current_block = 2652804691515851435;
                    } else {
                        current_block = 8422527538794739384;
                    }
                    match current_block {
                        2652804691515851435 => {}
                        _ => match unsafe { (*cur).type_0 as libc::c_uint } {
                            1 => {
                                current_block = 8477846364420885724;
                                match current_block {
                                    8477846364420885724 => {
                                        if unsafe { xmlStrEqual(name, (*cur).name) } != 0 {
                                            if prefix.is_null() {
                                                if unsafe { (*cur).ns.is_null() } {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as libc::c_int
                                                            {
                                                                safe_ctxt.error = XPATH_MEMORY_ERROR
                                                                    as libc::c_int
                                                            }
                                                            current_block = 12278438173206364583;
                                                            break;
                                                        }
                                                    } else {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 795179968803393002;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !unsafe { (*cur).ns.is_null() }
                                                && unsafe { xmlStrEqual(URI, (*(*cur).ns).href) }
                                                    != 0
                                            {
                                                if hasAxisRange != 0 as libc::c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 795179968803393002;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    15568404490809570198 => {
                                        let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
                                        if unsafe { xmlStrEqual(name, (*attr).name) } != 0 {
                                            if prefix.is_null() {
                                                if unsafe { (*attr).ns.is_null() }
                                                    || unsafe { (*(*attr).ns).prefix.is_null() }
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as libc::c_int
                                                            {
                                                                safe_ctxt.error = XPATH_MEMORY_ERROR
                                                                    as libc::c_int
                                                            }
                                                            current_block = 12278438173206364583;
                                                            break;
                                                        }
                                                    } else {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 795179968803393002;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !unsafe { (*attr).ns.is_null() }
                                                && unsafe {
                                                    xmlStrEqual(URI, (*(*attr).ns).href) != 0
                                                }
                                            {
                                                if hasAxisRange != 0 as libc::c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 795179968803393002;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        if unsafe { (*cur).type_0 as libc::c_uint }
                                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                                        {
                                            let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                            if !unsafe { (*ns).prefix.is_null() }
                                                && !name.is_null()
                                                && unsafe { xmlStrEqual((*ns).prefix, name) } != 0
                                            {
                                                if hasAxisRange != 0 as libc::c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        hasNsNodes = 1 as libc::c_int;
                                                        if unsafe {
                                                            xmlXPathNodeSetAddNs(
                                                                seq,
                                                                safe_xpctxt.node,
                                                                cur as xmlNsPtr,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    hasNsNodes = 1 as libc::c_int;
                                                    if unsafe {
                                                        xmlXPathNodeSetAddNs(
                                                            seq,
                                                            safe_xpctxt.node,
                                                            cur as xmlNsPtr,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 795179968803393002;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            2 => {
                                current_block = 15568404490809570198;
                                match current_block {
                                    8477846364420885724 => {
                                        if unsafe { xmlStrEqual(name, (*cur).name) } != 0 {
                                            if prefix.is_null() {
                                                if unsafe { (*cur).ns.is_null() } {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as libc::c_int
                                                            {
                                                                safe_ctxt.error = XPATH_MEMORY_ERROR
                                                                    as libc::c_int
                                                            }
                                                            current_block = 12278438173206364583;
                                                            break;
                                                        }
                                                    } else {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 795179968803393002;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !unsafe { (*cur).ns.is_null() }
                                                && unsafe { xmlStrEqual(URI, (*(*cur).ns).href) }
                                                    != 0
                                            {
                                                if hasAxisRange != 0 as libc::c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 795179968803393002;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    15568404490809570198 => {
                                        let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
                                        if unsafe { xmlStrEqual(name, (*attr).name) } != 0 {
                                            if prefix.is_null() {
                                                if unsafe { (*attr).ns.is_null() }
                                                    || unsafe { (*(*attr).ns).prefix.is_null() }
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as libc::c_int
                                                            {
                                                                safe_ctxt.error = XPATH_MEMORY_ERROR
                                                                    as libc::c_int
                                                            }
                                                            current_block = 12278438173206364583;
                                                            break;
                                                        }
                                                    } else {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 795179968803393002;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !unsafe { (*attr).ns.is_null() }
                                                && unsafe { xmlStrEqual(URI, (*(*attr).ns).href) }
                                                    != 0
                                            {
                                                if hasAxisRange != 0 as libc::c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 795179968803393002;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        if unsafe { (*cur).type_0 as libc::c_uint }
                                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                                        {
                                            let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                            if !unsafe { (*ns).prefix.is_null() }
                                                && !name.is_null()
                                                && unsafe { xmlStrEqual((*ns).prefix, name) } != 0
                                            {
                                                if hasAxisRange != 0 as libc::c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        hasNsNodes = 1 as libc::c_int;
                                                        if unsafe {
                                                            xmlXPathNodeSetAddNs(
                                                                seq,
                                                                safe_xpctxt.node,
                                                                cur as xmlNsPtr,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    hasNsNodes = 1 as libc::c_int;
                                                    if unsafe {
                                                        xmlXPathNodeSetAddNs(
                                                            seq,
                                                            safe_xpctxt.node,
                                                            cur as xmlNsPtr,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 795179968803393002;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            18 => {
                                current_block = 8605932438663696392;
                                match current_block {
                                    8477846364420885724 => {
                                        if unsafe { xmlStrEqual(name, (*cur).name) } != 0 {
                                            if prefix.is_null() {
                                                if unsafe { (*cur).ns.is_null() } {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as libc::c_int
                                                            {
                                                                safe_ctxt.error = XPATH_MEMORY_ERROR
                                                                    as libc::c_int
                                                            }
                                                            current_block = 12278438173206364583;
                                                            break;
                                                        }
                                                    } else {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 795179968803393002;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !unsafe { (*cur).ns.is_null() }
                                                && unsafe { xmlStrEqual(URI, (*(*cur).ns).href) }
                                                    != 0
                                            {
                                                if hasAxisRange != 0 as libc::c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 795179968803393002;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    15568404490809570198 => {
                                        let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
                                        if unsafe { xmlStrEqual(name, (*attr).name) } != 0 {
                                            if prefix.is_null() {
                                                if unsafe { (*attr).ns.is_null() }
                                                    || unsafe { (*(*attr).ns).prefix.is_null() }
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as libc::c_int
                                                            {
                                                                safe_ctxt.error = XPATH_MEMORY_ERROR
                                                                    as libc::c_int
                                                            }
                                                            current_block = 12278438173206364583;
                                                            break;
                                                        }
                                                    } else {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 795179968803393002;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !unsafe { (*attr).ns.is_null() }
                                                && unsafe { xmlStrEqual(URI, (*(*attr).ns).href) }
                                                    != 0
                                            {
                                                if hasAxisRange != 0 as libc::c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 795179968803393002;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        if unsafe { (*cur).type_0 as libc::c_uint }
                                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                                        {
                                            let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                            if !unsafe { (*ns).prefix.is_null() }
                                                && !name.is_null()
                                                && unsafe { xmlStrEqual((*ns).prefix, name) } != 0
                                            {
                                                if hasAxisRange != 0 as libc::c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        hasNsNodes = 1 as libc::c_int;
                                                        if unsafe {
                                                            xmlXPathNodeSetAddNs(
                                                                seq,
                                                                safe_xpctxt.node,
                                                                cur as xmlNsPtr,
                                                            )
                                                        } < 0 as libc::c_int
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as libc::c_int
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    hasNsNodes = 1 as libc::c_int;
                                                    if unsafe {
                                                        xmlXPathNodeSetAddNs(
                                                            seq,
                                                            safe_xpctxt.node,
                                                            cur as xmlNsPtr,
                                                        )
                                                    } < 0 as libc::c_int
                                                    {
                                                        safe_ctxt.error =
                                                            XPATH_MEMORY_ERROR as libc::c_int
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 795179968803393002;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },
                    }
                }
                _ => {}
            }
            if !(!cur.is_null() && safe_ctxt.error == XPATH_EXPRESSION_OK as libc::c_int) {
                current_block = 6300945584809257457;
                break;
            }
        }

        unsafe {
            match current_block {
                795179968803393002 => {
                    /* ---------------------------------------------------------- */
                    /*
                     * Break if only a true/false result was requested and
                     * no predicates existed and a node test succeeded.
                     */
                    if outSeq.is_null() {
                        outSeq = seq;
                        seq = 0 as xmlNodeSetPtr
                    } else {
                        /* TODO: Check memory error. */
                        outSeq = mergeAndClear.expect("non-null function pointer")(outSeq, seq)
                    }
                    break;
                }
                6300945584809257457 =>
                /* --------------------------------------------------- */
                {
                    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                        break;
                    }
                    /*
                     * Apply predicates.
                     */
                    if !predOp.is_null() && (*seq).nodeNr > 0 as libc::c_int {
                        /*
                         * E.g. when we have a "/foo[some expression][n]".
                         */
                        /*
                         * QUESTION TODO: The old predicate evaluation took into
                         *  account location-sets.
                         *  (E.g. ctxt->value->type == XPATH_LOCATIONSET) *  Do we expect such a set here?
                         *  All what I learned now from the evaluation semantics
                         *  does not indicate that a location-set will be processed
                         *  here, so this looks OK.
                         */
                        /*
                         * Iterate over all predicates, starting with the outermost
                         * predicate.
                         * TODO: Problem: we cannot execute the inner predicates first
                         *  since we cannot go back *up* the operator tree!
                         *  Options we have: *  1) Use of recursive functions (like is it currently done
                         *     via xmlXPathCompOpEval()) *  2) Add a predicate evaluation information stack to the
                         *     context struct
                         *  3) Change the way the operators are linked; we need a
                         *     "parent" field on xmlXPathStepOp
                         *
                         * For the moment, I'll try to solve this with a recursive
                         * function: xmlXPathCompOpEvalPredicate().
                         */
                        if hasPredicateRange != 0 as libc::c_int {
                            xmlXPathCompOpEvalPredicate(
                                ctxt, predOp, seq, maxPos, maxPos, hasNsNodes,
                            );
                        } else {
                            xmlXPathCompOpEvalPredicate(
                                ctxt,
                                predOp,
                                seq,
                                1 as libc::c_int,
                                (*seq).nodeNr,
                                hasNsNodes,
                            );
                        }
                        if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                            total = 0 as libc::c_int;
                            break;
                        }
                    }
                    if !((*seq).nodeNr > 0 as libc::c_int) {
                        continue;
                    }
                    /*
                     * Add to result set.
                     */
                    if outSeq.is_null() {
                        outSeq = seq;
                        seq = 0 as xmlNodeSetPtr
                    } else {
                        /* TODO: Check memory error. */
                        outSeq = mergeAndClear.expect("non-null function pointer")(outSeq, seq)
                    }
                    if toBool != 0 {
                        break;
                    }
                }
                _ => {
                    /* ----------------------------------------------------- */
                    /*
                     * We have a "/foo[n]", and position() = n was reached.
                     * Note that we can have as well "/foo/::parent::foo[1]", so
                     * a duplicate-aware merge is still needed.
                     * Merge with the result.
                     */
                    if outSeq.is_null() {
                        outSeq = seq;
                        seq = 0 as xmlNodeSetPtr
                    } else {
                        /* TODO: Check memory error. */
                        outSeq = mergeAndClear.expect("non-null function pointer")(outSeq, seq)
                    }
                    /*
                     * Break if only a true/false result was requested.
                     */
                    if toBool != 0 {
                        break;
                    }
                }
            }
        }
    }
    if safe_obj.boolval != 0 && !safe_obj.user.is_null() {
        /*
        	* QUESTION TODO: What does this do and why?
        	* TODO: Do we have to do this also for the "error"
        	* cleanup further down?
        	*/
        unsafe { (*safe_ctxt.value).boolval = 1 as libc::c_int };
        unsafe { (*safe_ctxt.value).user = safe_obj.user };
        safe_obj.user = 0 as *mut libc::c_void;
        safe_obj.boolval = 0 as libc::c_int
    }
    unsafe { xmlXPathReleaseObject(xpctxt, obj) };
    /*
     * Ensure we return at least an empty set.
     */
    if outSeq.is_null() {
        if !seq.is_null() && unsafe { (*seq).nodeNr == 0 as libc::c_int } {
            outSeq = seq
        } else {
            /* TODO: Check memory error. */
            outSeq = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) }
        }
    }
    if !seq.is_null() && seq != outSeq {
        unsafe { xmlXPathFreeNodeSet(seq) };
    }
    /*
     * Hand over the result. Better to push the set also in
     * case of errors.
     */
    unsafe { valuePush(ctxt, xmlXPathCacheWrapNodeSet(xpctxt, outSeq)) };
    /*
     * Reset the context node.
     */
    safe_xpctxt.node = oldContextNode;
    /*
     * When traversing the namespace axis in "toBool" mode, it's
     * possible that tmpNsList wasn't freed.
     */
    if !safe_xpctxt.tmpNsList.is_null() {
        unsafe {
            xmlFree.expect("non-null function pointer")(safe_xpctxt.tmpNsList as *mut libc::c_void)
        };
        safe_xpctxt.tmpNsList = 0 as *mut xmlNsPtr
    }
    return total;
}
/* *
 * xmlXPathCompOpEvalFirst: * @ctxt: the XPath parser context with the compiled expression
 * @op: an XPath compiled operation
 * @first: the first elem found so far
 *
 * Evaluate the Precompiled XPath operation searching only the first
 * element in document order
 *
 * Returns the number of examined objects.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompOpEvalFirst(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut first: *mut xmlNodePtr,
) -> libc::c_int {
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut cur: libc::c_int = 0;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return 0 as libc::c_int;
    }
    if unsafe { (*safe_ctxt.context).opLimit != 0 as libc::c_int as libc::c_ulong }
        && unsafe { xmlXPathCheckOpLimit(ctxt, 1 as libc::c_int as libc::c_ulong) }
            < 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if unsafe { (*safe_ctxt.context).depth >= 5000 as libc::c_int } {
        unsafe { xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as libc::c_int) };
        return 0 as libc::c_int;
    }
    unsafe { (*safe_ctxt.context).depth += 1 as libc::c_int };
    comp = safe_ctxt.comp;
    let safe_op = unsafe { &mut *op };
    match safe_op.op as libc::c_uint {
        0 => {}
        7 => {
            total = unsafe {
                xmlXPathCompOpEvalFirst(
                    ctxt,
                    &mut *(*comp).steps.offset(safe_op.ch1 as isize),
                    first,
                )
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as libc::c_uint
                        == XPATH_NODESET as libc::c_int as libc::c_uint
                        && !(*safe_ctxt.value).nodesetval.is_null()
                        && (*(*safe_ctxt.value).nodesetval).nodeNr >= 1 as libc::c_int
                }
            {
                /*
                 * limit tree traversing to first node in the result
                 */
                /*
                	* OPTIMIZE TODO: This implicitly sorts
                	*  the result, even if not needed. E.g. if the argument
                	*  of the count() function, no sorting is needed.
                	* OPTIMIZE TODO: How do we know if the node-list wasn't
                	*  already sorted?
                	*/
                if unsafe { (*(*safe_ctxt.value).nodesetval).nodeNr > 1 as libc::c_int } {
                    unsafe { xmlXPathNodeSetSort((*safe_ctxt.value).nodesetval) };
                }
                unsafe {
                    *first = *(*(*safe_ctxt.value).nodesetval)
                        .nodeTab
                        .offset(0 as libc::c_int as isize)
                }
            }
            cur = unsafe {
                xmlXPathCompOpEvalFirst(
                    ctxt,
                    &mut *(*comp).steps.offset(safe_op.ch2 as isize),
                    first,
                )
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            arg2 = unsafe { valuePop(ctxt) };
            arg1 = unsafe { valuePop(ctxt) };
            let safe_arg1 = unsafe { &mut *arg1 };
            let safe_arg2 = unsafe { &mut *arg2 };
            if arg1.is_null()
                || safe_arg1.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
                || arg2.is_null()
                || safe_arg2.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            {
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
                unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
                return 0 as libc::c_int;
            }
            if unsafe { (*safe_ctxt.context).opLimit != 0 as libc::c_int as libc::c_ulong }
                && (!safe_arg1.nodesetval.is_null()
                    && unsafe {
                        xmlXPathCheckOpLimit(ctxt, (*safe_arg1.nodesetval).nodeNr as libc::c_ulong)
                    } < 0 as libc::c_int
                    || !safe_arg2.nodesetval.is_null()
                        && unsafe {
                            xmlXPathCheckOpLimit(
                                ctxt,
                                (*safe_arg2.nodesetval).nodeNr as libc::c_ulong,
                            )
                        } < 0 as libc::c_int)
            {
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
            } else {
                /* TODO: Check memory error. */
                safe_arg1.nodesetval =
                    unsafe { xmlXPathNodeSetMerge(safe_arg1.nodesetval, safe_arg2.nodesetval) };
                unsafe { valuePush(ctxt, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
                /* optimizer */
                if total > cur {
                    unsafe { xmlXPathCompSwap(op) };
                }
                total += cur
            }
        }
        8 => {
            unsafe { xmlXPathRoot(ctxt) };
        }
        9 => {
            if safe_op.ch1 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *(*comp).steps.offset(safe_op.ch1 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if safe_op.ch2 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *(*comp).steps.offset(safe_op.ch2 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
                )
            };
        }
        10 => {
            if !(safe_op.ch1 == -(1 as libc::c_int)) {
                total = unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *(*comp).steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as libc::c_int;
                }
                total += unsafe {
                    xmlXPathNodeCollectAndTest(
                        ctxt,
                        op,
                        first,
                        0 as *mut xmlNodePtr,
                        0 as libc::c_int,
                    )
                }
            }
        }
        11 => {
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheObjectCopy(safe_ctxt.context, safe_op.value4 as xmlXPathObjectPtr),
                )
            };
        }
        17 => {
            if safe_op.ch1 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEvalFirst(
                        ctxt,
                        &mut *(*comp).steps.offset(safe_op.ch1 as isize),
                        first,
                    )
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as libc::c_uint
                        == XPATH_NODESET as libc::c_int as libc::c_uint
                        && !(*safe_ctxt.value).nodesetval.is_null()
                        && (*(*safe_ctxt.value).nodesetval).nodeNr > 1 as libc::c_int
                }
            {
                unsafe { xmlXPathNodeSetSort((*safe_ctxt.value).nodesetval) };
            }
        }

        16 => {
            match () {
                #[cfg(XP_OPTIMIZED_FILTER_FIRST)]
                _ => total += unsafe { xmlXPathCompOpEvalFilterFirst(ctxt, op, first) },
                #[cfg(not(XP_OPTIMIZED_FILTER_FIRST))]
                _ => {}
            };
        }
        // total += xmlXPathCompOpEvalFilterFirst(ctxt, op, first) }
        _ => total += xmlXPathCompOpEval(ctxt, op),
    }
    unsafe { (*safe_ctxt.context).depth -= 1 as libc::c_int };
    return total;
}
/* *
 * xmlXPathCompOpEvalLast: * @ctxt: the XPath parser context with the compiled expression
 * @op: an XPath compiled operation
 * @last: the last elem found so far
 *
 * Evaluate the Precompiled XPath operation searching only the last
 * element in document order
 *
 * Returns the number of nodes traversed
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompOpEvalLast(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut last: *mut xmlNodePtr,
) -> libc::c_int {
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut cur: libc::c_int = 0;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return 0 as libc::c_int;
    }
    if unsafe { (*safe_ctxt.context).opLimit != 0 as libc::c_int as libc::c_ulong }
        && unsafe { xmlXPathCheckOpLimit(ctxt, 1 as libc::c_int as libc::c_ulong) }
            < 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if unsafe { (*safe_ctxt.context).depth >= 5000 as libc::c_int } {
        unsafe { xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as libc::c_int) };
        return 0 as libc::c_int;
    }
    unsafe { (*safe_ctxt.context).depth += 1 as libc::c_int };
    comp = safe_ctxt.comp;
    let safe_op = unsafe { &mut *op };
    let safe_comp = unsafe { &mut *comp };
    match safe_op.op as libc::c_uint {
        0 => {}
        7 => {
            total = unsafe {
                xmlXPathCompOpEvalLast(
                    ctxt,
                    &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                    last,
                )
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as libc::c_uint
                        == XPATH_NODESET as libc::c_int as libc::c_uint
                        && !(*safe_ctxt.value).nodesetval.is_null()
                        && (*(*safe_ctxt.value).nodesetval).nodeNr >= 1 as libc::c_int
                }
            {
                /*
                 * limit tree traversing to first node in the result
                 */
                if unsafe { (*(*safe_ctxt.value).nodesetval).nodeNr > 1 as libc::c_int } {
                    unsafe { xmlXPathNodeSetSort((*safe_ctxt.value).nodesetval) };
                }
                unsafe {
                    *last = *(*(*safe_ctxt.value).nodesetval).nodeTab.offset(
                        ((*(*safe_ctxt.value).nodesetval).nodeNr - 1 as libc::c_int) as isize,
                    )
                }
            }
            cur = unsafe {
                xmlXPathCompOpEvalLast(
                    ctxt,
                    &mut *safe_comp.steps.offset(safe_op.ch2 as isize),
                    last,
                )
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            (!safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as libc::c_uint
                        == XPATH_NODESET as libc::c_int as libc::c_uint
                        && !(*safe_ctxt.value).nodesetval.is_null()
                })
                && unsafe { (*(*safe_ctxt.value).nodesetval).nodeNr >= 1 as libc::c_int };
            arg2 = unsafe { valuePop(ctxt) };
            arg1 = unsafe { valuePop(ctxt) };
            let safe_arg1 = unsafe { &mut *arg1 };
            let safe_arg2 = unsafe { &mut *arg2 };
            if arg1.is_null()
                || safe_arg1.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
                || arg2.is_null()
                || safe_arg1.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            {
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
                unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
                return 0 as libc::c_int;
            }
            if unsafe { (*safe_ctxt.context).opLimit != 0 as libc::c_int as libc::c_ulong }
                && (!safe_arg1.nodesetval.is_null()
                    && unsafe {
                        xmlXPathCheckOpLimit(ctxt, (*safe_arg1.nodesetval).nodeNr as libc::c_ulong)
                    } < 0 as libc::c_int
                    || !safe_arg1.nodesetval.is_null()
                        && unsafe {
                            xmlXPathCheckOpLimit(
                                ctxt,
                                (*safe_arg1.nodesetval).nodeNr as libc::c_ulong,
                            )
                        } < 0 as libc::c_int)
            {
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
            } else {
                /* TODO: Check memory error. */
                safe_arg1.nodesetval =
                    unsafe { xmlXPathNodeSetMerge((*arg1).nodesetval, safe_arg1.nodesetval) };
                unsafe { valuePush(ctxt, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
                /* optimizer */
                if total > cur {
                    unsafe { xmlXPathCompSwap(op) };
                }
                total += cur
            }
        }
        8 => {
            unsafe { xmlXPathRoot(ctxt) };
        }
        9 => {
            if safe_op.ch1 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if safe_op.ch2 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
                )
            };
        }
        10 => {
            if !(safe_op.ch1 == -(1 as libc::c_int)) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as libc::c_int;
                }
                total += unsafe {
                    xmlXPathNodeCollectAndTest(
                        ctxt,
                        op,
                        0 as *mut xmlNodePtr,
                        last,
                        0 as libc::c_int,
                    )
                }
            }
        }
        11 => {
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheObjectCopy(safe_ctxt.context, safe_op.value4 as xmlXPathObjectPtr),
                )
            };
        }
        17 => {
            if safe_op.ch1 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEvalLast(
                        ctxt,
                        &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                        last,
                    )
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as libc::c_uint
                        == XPATH_NODESET as libc::c_int as libc::c_uint
                        && !(*safe_ctxt.value).nodesetval.is_null()
                        && (*(*safe_ctxt.value).nodesetval).nodeNr > 1 as libc::c_int
                }
            {
                unsafe { xmlXPathNodeSetSort((*safe_ctxt.value).nodesetval) };
            }
        }
        _ => total += unsafe { xmlXPathCompOpEval(ctxt, op) },
    }
    unsafe { (*safe_ctxt.context).depth -= 1 as libc::c_int };
    return total;
}

#[cfg(XP_OPTIMIZED_FILTER_FIRST)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompOpEvalFilterFirst(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut first: *mut xmlNodePtr,
) -> libc::c_int {
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut set: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return 0 as libc::c_int;
    }
    comp = safe_ctxt.comp;
    let safe_comp = unsafe { &mut *comp };
    let safe_op = unsafe { &mut *op };
    /*
     * Optimization for ()[last()] selection i.e. the last elem
     */
    if safe_op.ch1 != -(1 as libc::c_int)
        && safe_op.ch2 != -(1 as libc::c_int)
        && unsafe { (*safe_comp.steps.offset(safe_op.ch1 as isize)).op as libc::c_uint }
            == XPATH_OP_SORT as libc::c_int as libc::c_uint
        && unsafe { (*safe_comp.steps.offset(safe_op.ch2 as isize)).op as libc::c_uint }
            == XPATH_OP_SORT as libc::c_int as libc::c_uint
    {
        let mut f: libc::c_int = unsafe { (*safe_comp.steps.offset(safe_op.ch2 as isize)).ch1 };
        if f != -(1 as libc::c_int)
            && unsafe { (*safe_comp.steps.offset(f as isize)).op as libc::c_uint }
                == XPATH_OP_FUNCTION as libc::c_int as libc::c_uint
            && unsafe { (*safe_comp.steps.offset(f as isize)).value5.is_null() }
            && unsafe { (*safe_comp.steps.offset(f as isize)).value == 0 as libc::c_int }
            && !unsafe { (*safe_comp.steps.offset(f as isize)).value4.is_null() }
            && unsafe {
                xmlStrEqual(
                    (*safe_comp.steps.offset(f as isize)).value4 as *const xmlChar,
                    b"last\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            }
        {
            let mut last: xmlNodePtr = 0 as xmlNodePtr;
            total += unsafe {
                xmlXPathCompOpEvalLast(
                    ctxt,
                    &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                    &mut last,
                )
            };

            if unsafe { safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int } {
                return 0 as libc::c_int;
            }
            /*
             * The nodeset should be in document order,
             * Keep only the last value
             */
            if !unsafe {
                safe_ctxt.value.is_null()
                    && (*safe_ctxt.value).type_0 as libc::c_uint
                        == XPATH_NODESET as libc::c_int as libc::c_uint
                    && !(*safe_ctxt.value).nodesetval.is_null()
                    && !(*(*safe_ctxt.value).nodesetval).nodeTab.is_null()
                    && (*(*safe_ctxt.value).nodesetval).nodeNr > 1 as libc::c_int
            } {
                unsafe {
                    xmlXPathNodeSetKeepLast((*safe_ctxt.value).nodesetval);
                    *first = *(*(*safe_ctxt.value).nodesetval).nodeTab
                }
            }
            return total;
        }
    }
    if safe_op.ch1 != -(1 as libc::c_int) {
        total +=
            unsafe { xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize)) }
    }
    if unsafe { safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int } {
        return 0 as libc::c_int;
    }
    if safe_op.ch2 == -(1 as libc::c_int) {
        return total;
    }
    if unsafe { safe_ctxt.value.is_null() } {
        return total;
    }
    match () {
        #[cfg(LIBXML_XPTR_ENABLED)]
        _ => {
            /*
             * Hum are we filtering the result of an XPointer expression
             */
            if unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                == XPATH_LOCATIONSET as libc::c_int as libc::c_uint
            {
                let mut locset: xmlLocationSetPtr =
                    unsafe { (*safe_ctxt.value).user as xmlLocationSetPtr };
                if !locset.is_null() {
                    unsafe {
                        xmlXPathLocationSetFilter(
                            ctxt,
                            locset,
                            safe_op.ch2,
                            1 as libc::c_int,
                            1 as libc::c_int,
                        )
                    };
                    if unsafe { (*locset).locNr > 0 as libc::c_int } {
                        unsafe {
                            *first = (**(*locset).locTab.offset(0 as libc::c_int as isize)).user
                                as xmlNodePtr
                        }
                    }
                }
                return total;
            }
            /* LIBXML_XPTR_ENABLED */
        }
        #[cfg(not(LIBXML_XPTR_ENABLED))]
        _ => {}
    }
    /*
     * Hum are we filtering the result of an XPointer expression
     */
    if unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
        == XPATH_LOCATIONSET as libc::c_int as libc::c_uint
    {
        let mut locset: xmlLocationSetPtr = unsafe { (*safe_ctxt.value).user as xmlLocationSetPtr };
        if !locset.is_null() {
            unsafe {
                xmlXPathLocationSetFilter(
                    ctxt,
                    locset,
                    safe_op.ch2,
                    1 as libc::c_int,
                    1 as libc::c_int,
                )
            };
            if unsafe { (*locset).locNr > 0 as libc::c_int } {
                unsafe {
                    *first =
                        (**(*locset).locTab.offset(0 as libc::c_int as isize)).user as xmlNodePtr
                }
            }
        }
        return total;
    }
    /* LIBXML_XPTR_ENABLED */
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
            != XPATH_NODESET as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
        return 0 as libc::c_int;
    }
    unsafe { set = (*safe_ctxt.value).nodesetval };
    if !set.is_null() {
        unsafe {
            xmlXPathNodeSetFilter(
                ctxt,
                set,
                safe_op.ch2,
                1 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
            )
        };
        if unsafe { (*set).nodeNr > 0 as libc::c_int } {
            unsafe { *first = *(*set).nodeTab.offset(0 as libc::c_int as isize) }
        }
    }
    return total;
}
/* XP_OPTIMIZED_FILTER_FIRST */

/* ***********************************************************************
 *									*
 *		XPath precompiled expression evaluation			*
 *									*
 ************************************************************************/

/* *
 * xmlXPathCompOpEval: * @ctxt: the XPath parser context with the compiled expression
 * @op: an XPath compiled operation
 *
 * Evaluate the Precompiled XPath operation
 * Returns the number of nodes traversed
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompOpEval(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
) -> libc::c_int {
    let mut current_block: u64;
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut equal: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        return 0 as libc::c_int;
    }
    if unsafe {
        (*safe_ctxt.context).opLimit != 0 as libc::c_int as libc::c_ulong
            && xmlXPathCheckOpLimit(ctxt, 1 as libc::c_int as libc::c_ulong) < 0 as libc::c_int
    } {
        return 0 as libc::c_int;
    }
    if unsafe { (*safe_ctxt.context).depth >= 5000 as libc::c_int } {
        unsafe { xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as libc::c_int) };
        return 0 as libc::c_int;
    }
    unsafe { (*safe_ctxt.context).depth += 1 as libc::c_int };
    comp = safe_ctxt.comp;
    let safe_op = unsafe { &mut *op };
    let safe_comp = unsafe { &mut *comp };
    match safe_op.op as libc::c_uint {
        0 => {}
        1 => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            unsafe { xmlXPathBooleanFunction(ctxt, 1 as libc::c_int) };
            if !(safe_ctxt.value.is_null()
                || unsafe { (*safe_ctxt.value).boolval == 0 as libc::c_int })
            {
                arg2 = unsafe { valuePop(ctxt) };
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                };
                if safe_ctxt.error != 0 {
                    unsafe { xmlXPathFreeObject(arg2) };
                } else {
                    unsafe { xmlXPathBooleanFunction(ctxt, 1 as libc::c_int) };
                    if !safe_ctxt.value.is_null() {
                        unsafe { (*safe_ctxt.value).boolval &= (*arg2).boolval }
                    }
                    unsafe { xmlXPathReleaseObject((*ctxt).context, arg2) };
                }
            }
        }
        2 => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            unsafe { xmlXPathBooleanFunction(ctxt, 1 as libc::c_int) };
            if !(safe_ctxt.value.is_null()
                || unsafe { (*safe_ctxt.value).boolval == 1 as libc::c_int })
            {
                arg2 = unsafe { valuePop(ctxt) };
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                };
                if safe_ctxt.error != 0 {
                    unsafe { xmlXPathFreeObject(arg2) };
                } else {
                    unsafe { xmlXPathBooleanFunction(ctxt, 1 as libc::c_int) };
                    if !safe_ctxt.value.is_null() {
                        unsafe { (*safe_ctxt.value).boolval |= (*arg2).boolval }
                    }
                    unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
                }
            }
        }
        3 => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if safe_op.value != 0 {
                equal = unsafe { xmlXPathEqualValues(ctxt) }
            } else {
                equal = unsafe { xmlXPathNotEqualValues(ctxt) }
            }
            unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, equal)) };
        }
        4 => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            ret = unsafe { xmlXPathCompareValues(ctxt, safe_op.value, safe_op.value2) };
            unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, ret)) };
        }
        5 => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if safe_op.ch2 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if safe_op.value == 0 as libc::c_int {
                unsafe { xmlXPathSubValues(ctxt) };
            } else if safe_op.value == 1 as libc::c_int {
                unsafe { xmlXPathAddValues(ctxt) };
            } else if safe_op.value == 2 as libc::c_int {
                unsafe { xmlXPathValueFlipSign(ctxt) };
            } else if safe_op.value == 3 as libc::c_int {
                if !safe_ctxt.value.is_null()
                    && unsafe {
                        (*safe_ctxt.value).type_0 as libc::c_uint
                            != XPATH_NUMBER as libc::c_int as libc::c_uint
                    }
                {
                    unsafe { xmlXPathNumberFunction(ctxt, 1 as libc::c_int) };
                }
                if safe_ctxt.value.is_null()
                    || unsafe {
                        (*safe_ctxt.value).type_0 as libc::c_uint
                            != XPATH_NUMBER as libc::c_int as libc::c_uint
                    }
                {
                    unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
                    return 0 as libc::c_int;
                }
            }
        }
        6 => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if safe_op.value == 0 as libc::c_int {
                unsafe { xmlXPathMultValues(ctxt) };
            } else if safe_op.value == 1 as libc::c_int {
                unsafe { xmlXPathDivValues(ctxt) };
            } else if safe_op.value == 2 as libc::c_int {
                unsafe { xmlXPathModValues(ctxt) };
            }
        }
        7 => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            arg2 = unsafe { valuePop(ctxt) };
            arg1 = unsafe { valuePop(ctxt) };
            let safe_arg1 = unsafe { &mut *arg1 };
            let safe_arg2 = unsafe { &mut *arg2 };
            if arg1.is_null()
                || safe_arg1.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
                || arg2.is_null()
                || safe_arg2.type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            {
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
                unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
                return 0 as libc::c_int;
            }
            if unsafe { (*safe_ctxt.context).opLimit != 0 as libc::c_int as libc::c_ulong }
                && (!safe_arg1.nodesetval.is_null()
                    && unsafe {
                        xmlXPathCheckOpLimit(ctxt, (*safe_arg1.nodesetval).nodeNr as libc::c_ulong)
                            < 0 as libc::c_int
                    }
                    || !safe_arg2.nodesetval.is_null()
                        && unsafe {
                            xmlXPathCheckOpLimit(
                                ctxt,
                                (*safe_arg2.nodesetval).nodeNr as libc::c_ulong,
                            ) < 0 as libc::c_int
                        })
            {
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
            } else {
                if safe_arg1.nodesetval.is_null()
                    || !safe_arg2.nodesetval.is_null()
                        && unsafe { (*safe_arg2.nodesetval).nodeNr != 0 as libc::c_int }
                {
                    /* TODO: Check memory error. */
                    safe_arg1.nodesetval =
                        unsafe { xmlXPathNodeSetMerge(safe_arg1.nodesetval, safe_arg2.nodesetval) }
                }
                unsafe { valuePush(ctxt, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
            }
        }
        8 => {
            unsafe { xmlXPathRoot(ctxt) };
        }
        9 => {
            if safe_op.ch1 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if safe_op.ch2 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
                )
            };
        }
        10 => {
            if !(safe_op.ch1 == -(1 as libc::c_int)) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as libc::c_int;
                }
                total += unsafe {
                    xmlXPathNodeCollectAndTest(
                        ctxt,
                        op,
                        0 as *mut xmlNodePtr,
                        0 as *mut xmlNodePtr,
                        0 as libc::c_int,
                    )
                }
            }
        }
        11 => {
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheObjectCopy(safe_ctxt.context, safe_op.value4 as xmlXPathObjectPtr),
                )
            };
        }
        12 => {
            let mut val: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            if safe_op.ch1 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                }
            }
            if safe_op.value5.is_null() {
                val = unsafe {
                    xmlXPathVariableLookup(safe_ctxt.context, safe_op.value4 as *const xmlChar)
                };
                if val.is_null() {
                    unsafe { xmlXPathErr(ctxt, XPATH_UNDEF_VARIABLE_ERROR as libc::c_int) };
                    return 0 as libc::c_int;
                }
                unsafe { valuePush(ctxt, val) };
            } else {
                let mut URI: *const xmlChar = 0 as *const xmlChar;
                URI = unsafe {
                    xmlXPathNsLookup(safe_ctxt.context, safe_op.value5 as *const xmlChar)
                };
                if URI.is_null() {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"xmlXPathCompOpEval: variable %s bound to undefined prefix %s\n\x00"
                                as *const u8 as *const libc::c_char,
                            safe_op.value4 as *mut libc::c_char,
                            safe_op.value5 as *mut libc::c_char,
                        )
                    };
                    safe_ctxt.error = XPATH_UNDEF_PREFIX_ERROR as libc::c_int
                } else {
                    val = unsafe {
                        xmlXPathVariableLookupNS(
                            safe_ctxt.context,
                            safe_op.value4 as *const xmlChar,
                            URI,
                        )
                    };
                    if val.is_null() {
                        unsafe { xmlXPathErr(ctxt, XPATH_UNDEF_VARIABLE_ERROR as libc::c_int) };
                        return 0 as libc::c_int;
                    }
                    unsafe { valuePush(ctxt, val) };
                }
            }
        }
        13 => {
            let mut func: xmlXPathFunction = None;
            let mut oldFunc: *const xmlChar = 0 as *const xmlChar;
            let mut oldFuncURI: *const xmlChar = 0 as *const xmlChar;
            let mut i: libc::c_int = 0;
            let mut frame: libc::c_int = 0;
            frame = unsafe { xmlXPathSetFrame(ctxt) };
            if safe_op.ch1 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                    unsafe { xmlXPathPopFrame(ctxt, frame) };
                    current_block = 16794583624483845564;
                } else {
                    current_block = 6535105651042291885;
                }
            } else {
                current_block = 6535105651042291885;
            }
            match current_block {
                16794583624483845564 => {}
                _ => {
                    if safe_ctxt.valueNr < safe_ctxt.valueFrame + safe_op.value {
                        unsafe {
                            (*__xmlGenericError()).expect("non-null function pointer")(
                                *__xmlGenericErrorContext(),
                                b"xmlXPathCompOpEval: parameter error\n\x00" as *const u8
                                    as *const libc::c_char,
                            )
                        };
                        safe_ctxt.error = XPATH_INVALID_OPERAND as libc::c_int;
                        unsafe { xmlXPathPopFrame(ctxt, frame) };
                    } else {
                        i = 0 as libc::c_int;
                        while i < safe_op.value {
                            if unsafe {
                                (*safe_ctxt
                                    .valueTab
                                    .offset((safe_ctxt.valueNr - 1 as libc::c_int - i) as isize))
                                .is_null()
                            } {
                                unsafe {
                                    (*__xmlGenericError()).expect("non-null function pointer")(
                                        *__xmlGenericErrorContext(),
                                        b"xmlXPathCompOpEval: parameter error\n\x00" as *const u8
                                            as *const libc::c_char,
                                    )
                                };
                                safe_ctxt.error = XPATH_INVALID_OPERAND as libc::c_int;
                                unsafe { xmlXPathPopFrame(ctxt, frame) };
                                break;
                            } else {
                                i += 1
                            }
                        }
                        if safe_op.cache.is_some() {
                            func = safe_op.cache;
                            current_block = 14187386403465544025;
                        } else {
                            let mut URI_0: *const xmlChar = 0 as *const xmlChar;
                            if safe_op.value5.is_null() {
                                func = unsafe {
                                    xmlXPathFunctionLookup(
                                        safe_ctxt.context,
                                        safe_op.value4 as *const xmlChar,
                                    )
                                };
                                current_block = 13718575627189773797;
                            } else {
                                URI_0 = unsafe {
                                    xmlXPathNsLookup(
                                        safe_ctxt.context,
                                        safe_op.value5 as *const xmlChar,
                                    )
                                };
                                if URI_0.is_null() {
                                    unsafe {
                                        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(), b"xmlXPathCompOpEval: function %s bound to undefined prefix %s\n\x00"
                                                                                                   as *const u8
                                                                                                   as *const libc::c_char, safe_op.value4
                                                                                                   as *mut libc::c_char, safe_op.value5
                                                                                                   as *mut libc::c_char)
                                    };
                                    unsafe { xmlXPathPopFrame(ctxt, frame) };
                                    safe_ctxt.error = XPATH_UNDEF_PREFIX_ERROR as libc::c_int;
                                    current_block = 16794583624483845564;
                                } else {
                                    func = unsafe {
                                        xmlXPathFunctionLookupNS(
                                            safe_ctxt.context,
                                            safe_op.value4 as *const xmlChar,
                                            URI_0,
                                        )
                                    };
                                    current_block = 13718575627189773797;
                                }
                            }
                            match current_block {
                                16794583624483845564 => {}
                                _ => {
                                    if func.is_none() {
                                        unsafe {
                                            (*__xmlGenericError())
                                                .expect("non-null function pointer")(
                                                *__xmlGenericErrorContext(),
                                                b"xmlXPathCompOpEval: function %s not found\n\x00"
                                                    as *const u8
                                                    as *const libc::c_char,
                                                safe_op.value4 as *mut libc::c_char,
                                            )
                                        };
                                        unsafe {
                                            xmlXPathErr(
                                                ctxt,
                                                XPATH_UNKNOWN_FUNC_ERROR as libc::c_int,
                                            )
                                        };
                                        return 0 as libc::c_int;
                                    }
                                    safe_op.cache = func;
                                    safe_op.cacheURI = URI_0 as *mut libc::c_void;
                                    current_block = 14187386403465544025;
                                }
                            }
                        }
                        match current_block {
                            16794583624483845564 => {}
                            _ => {
                                unsafe {
                                    oldFunc = (*safe_ctxt.context).function;
                                    oldFuncURI = (*safe_ctxt.context).functionURI;
                                    (*safe_ctxt.context).function =
                                        safe_op.value4 as *const xmlChar;
                                    (*safe_ctxt.context).functionURI =
                                        safe_op.cacheURI as *const xmlChar;
                                    func.expect("non-null function pointer")(ctxt, safe_op.value);
                                    (*safe_ctxt.context).function = oldFunc;
                                    (*safe_ctxt.context).functionURI = oldFuncURI;
                                }
                                if safe_ctxt.error == XPATH_EXPRESSION_OK as libc::c_int
                                    && safe_ctxt.valueNr != safe_ctxt.valueFrame + 1 as libc::c_int
                                {
                                    unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
                                    return 0 as libc::c_int;
                                }
                                unsafe { xmlXPathPopFrame(ctxt, frame) };
                            }
                        }
                    }
                }
            }
        }
        14 => {
            if safe_op.ch1 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            if safe_op.ch2 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
        }
        15 | 16 => {
            let mut set: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
            /*
             * Optimization for ()[1] selection i.e. the first elem
             */
            let mut XP_OPTIMIZED_FILTER_FIRST_FLAG: libc::c_int = 0;
            // XP_OPTIMIZED_FILTER_FIRST
            match () {
                #[cfg(XP_OPTIMIZED_FILTER_FIRST)]
                _ => {
                    if unsafe {
                        ((*safe_comp.steps.offset(safe_op.ch1 as isize)).op as libc::c_uint
                            == XPATH_OP_SORT as libc::c_int as libc::c_uint
                            || (*safe_comp.steps.offset(safe_op.ch1 as isize)).op as libc::c_uint
                                == XPATH_OP_FILTER as libc::c_int as libc::c_uint)
                    } {
                        XP_OPTIMIZED_FILTER_FIRST_FLAG = 1;
                    }
                }
                // else
                #[cfg(not(XP_OPTIMIZED_FILTER_FIRST))]
                _ => {
                    if unsafe {
                        (*safe_comp.steps.offset(safe_op.ch1 as isize)).op as libc::c_uint
                            == XPATH_OP_SORT as libc::c_int as libc::c_uint
                    } {
                        XP_OPTIMIZED_FILTER_FIRST_FLAG = 1;
                    }
                }
            }
            // endif XP_OPTIMIZED_FILTER_FIRST

            if safe_op.ch1 != -(1 as libc::c_int)
                && safe_op.ch2 != -(1 as libc::c_int)
                && (XP_OPTIMIZED_FILTER_FIRST_FLAG == (1 as libc::c_int))
                && unsafe {
                    (*safe_comp.steps.offset(safe_op.ch2 as isize)).op as libc::c_uint
                        == XPATH_OP_VALUE as libc::c_int as libc::c_uint
                }
            {
                /* 12 */
                let mut val_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                val_0 = unsafe {
                    (*safe_comp.steps.offset(safe_op.ch2 as isize)).value4 as xmlXPathObjectPtr
                };
                let safe_val_0 = unsafe { &mut *val_0 };
                if !val_0.is_null()
                    && safe_val_0.type_0 as libc::c_uint
                        == XPATH_NUMBER as libc::c_int as libc::c_uint
                    && safe_val_0.floatval == 1.0f64
                {
                    let mut first: xmlNodePtr = 0 as xmlNodePtr;
                    total += unsafe {
                        xmlXPathCompOpEvalFirst(
                            ctxt,
                            &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                            &mut first,
                        )
                    };
                    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                        return 0 as libc::c_int;
                    }
                    /*
                     * The nodeset should be in document order, * Keep only the first value
                     */
                    if !safe_ctxt.value.is_null()
                        && unsafe {
                            (*safe_ctxt.value).type_0 as libc::c_uint
                                == XPATH_NODESET as libc::c_int as libc::c_uint
                                && !(*safe_ctxt.value).nodesetval.is_null()
                                && (*(*safe_ctxt.value).nodesetval).nodeNr > 1 as libc::c_int
                        }
                    {
                        unsafe {
                            xmlXPathNodeSetClearFromPos(
                                (*safe_ctxt.value).nodesetval,
                                1 as libc::c_int,
                                1 as libc::c_int,
                            )
                        };
                    }
                    current_block = 16794583624483845564;
                } else {
                    current_block = 2640716771647493481;
                }
            } else {
                current_block = 2640716771647493481;
            }
            match current_block {
                16794583624483845564 => {}
                _ =>
                /*
                 * Optimization for ()[last()] selection i.e. the last elem
                 */
                {
                    if safe_op.ch1 != -(1 as libc::c_int)
                        && safe_op.ch2 != -(1 as libc::c_int)
                        && unsafe {
                            (*safe_comp.steps.offset(safe_op.ch1 as isize)).op as libc::c_uint
                        } == XPATH_OP_SORT as libc::c_int as libc::c_uint
                        && unsafe {
                            (*safe_comp.steps.offset(safe_op.ch2 as isize)).op as libc::c_uint
                        } == XPATH_OP_SORT as libc::c_int as libc::c_uint
                    {
                        let mut f: libc::c_int =
                            unsafe { (*safe_comp.steps.offset(safe_op.ch2 as isize)).ch1 };
                        if f != -(1 as libc::c_int)
                            && unsafe { (*safe_comp.steps.offset(f as isize)).op as libc::c_uint }
                                == XPATH_OP_FUNCTION as libc::c_int as libc::c_uint
                            && unsafe { (*safe_comp.steps.offset(f as isize)).value5.is_null() }
                            && unsafe {
                                (*safe_comp.steps.offset(f as isize)).value == 0 as libc::c_int
                            }
                            && !unsafe { (*safe_comp.steps.offset(f as isize)).value4.is_null() }
                            && unsafe {
                                xmlStrEqual(
                                    (*safe_comp.steps.offset(f as isize)).value4 as *const xmlChar,
                                    b"last\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                                ) != 0
                            }
                        {
                            let mut last: xmlNodePtr = 0 as xmlNodePtr;
                            total += unsafe {
                                xmlXPathCompOpEvalLast(
                                    ctxt,
                                    &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                                    &mut last,
                                )
                            };
                            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                                return 0 as libc::c_int;
                            }
                            /*
                             * The nodeset should be in document order, * Keep only the last value
                             */
                            if !safe_ctxt.value.is_null()
                                && unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                                    == XPATH_NODESET as libc::c_int as libc::c_uint
                                && !unsafe { (*safe_ctxt.value).nodesetval.is_null() }
                                && !unsafe { (*(*safe_ctxt.value).nodesetval).nodeTab.is_null() }
                                && unsafe { (*(*safe_ctxt.value).nodesetval).nodeNr }
                                    > 1 as libc::c_int
                            {
                                unsafe { xmlXPathNodeSetKeepLast((*safe_ctxt.value).nodesetval) };
                            }
                            current_block = 16794583624483845564;
                        } else {
                            current_block = 15696916892398440870;
                        }
                    } else {
                        current_block = 15696916892398440870;
                    }
                    match current_block {
                        16794583624483845564 => {}
                        _ => {
                            /*
                            	* Process inner predicates first.
                            	* Example "index[parent::book][1]":
                            	* ...
                            	*   PREDICATE   <-- we are here "[1]"
                            	*     PREDICATE <-- process "[parent::book]" first
                            	*       SORT
                            	*         COLLECT  'parent' 'name' 'node' book
                            	*           NODE
                            	*     ELEM Object is a number : 1
                            	*/
                            if safe_op.ch1 != -(1 as libc::c_int) {
                                total += unsafe {
                                    xmlXPathCompOpEval(
                                        ctxt,
                                        &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                                    )
                                }
                            }
                            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                                return 0 as libc::c_int;
                            }
                            if !(safe_op.ch2 == -(1 as libc::c_int)) {
                                if !safe_ctxt.value.is_null() {
                                    /*
                                     * Hum are we filtering the result of an XPointer expression
                                     */
                                    if unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                                        == XPATH_LOCATIONSET as libc::c_int as libc::c_uint
                                    {
                                        let mut locset: xmlLocationSetPtr =
                                            unsafe { (*safe_ctxt.value).user as xmlLocationSetPtr };
                                        unsafe {
                                            xmlXPathLocationSetFilter(
                                                ctxt,
                                                locset,
                                                safe_op.ch2,
                                                1 as libc::c_int,
                                                (*locset).locNr,
                                            )
                                        };
                                    } else {
                                        /* LIBXML_XPTR_ENABLED */
                                        if safe_ctxt.value.is_null()
                                            || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                                                != XPATH_NODESET as libc::c_int as libc::c_uint
                                        {
                                            unsafe {
                                                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int)
                                            }; /* Not a location set */
                                            return 0 as libc::c_int;
                                        }
                                        unsafe {
                                            set = (*safe_ctxt.value).nodesetval;
                                        }
                                        if !set.is_null() {
                                            unsafe {
                                                xmlXPathNodeSetFilter(
                                                    ctxt,
                                                    set,
                                                    safe_op.ch2,
                                                    1 as libc::c_int,
                                                    (*set).nodeNr,
                                                    1 as libc::c_int,
                                                )
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        17 => {
            if safe_op.ch1 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as libc::c_uint
                        == XPATH_NODESET as libc::c_int as libc::c_uint
                }
                && !unsafe { (*safe_ctxt.value).nodesetval.is_null() }
                && unsafe { (*(*safe_ctxt.value).nodesetval).nodeNr > 1 as libc::c_int }
            {
                unsafe { xmlXPathNodeSetSort((*safe_ctxt.value).nodesetval) };
            }
        }
        18 => {
            let mut range: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut newlocset: xmlLocationSetPtr = 0 as xmlLocationSetPtr;
            let mut oldlocset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
            let mut oldset: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
            let mut oldnode: xmlNodePtr = unsafe { (*safe_ctxt.context).node };
            let mut oldcs: libc::c_int = unsafe { (*safe_ctxt.context).contextSize };
            let mut oldpp: libc::c_int = unsafe { (*safe_ctxt.context).proximityPosition };
            let mut i_0: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            if safe_op.ch1 != -(1 as libc::c_int) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            if safe_ctxt.value.is_null() {
                unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int) };
                return 0 as libc::c_int;
            }
            if !(safe_op.ch2 == -(1 as libc::c_int)) {
                if unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                    == XPATH_LOCATIONSET as libc::c_int as libc::c_uint
                {
                    /*
                     * Extract the old locset, and then evaluate the result of the
                     * expression for all the element in the locset. use it to grow
                     * up a new locset.
                     */
                    if safe_ctxt.value.is_null()
                        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                            != XPATH_LOCATIONSET as libc::c_int as libc::c_uint
                    {
                        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
                        return 0 as libc::c_int;
                    }
                    if unsafe { (*safe_ctxt.value).user.is_null() }
                        || unsafe { (*((*safe_ctxt.value).user as xmlLocationSetPtr)).locNr }
                            == 0 as libc::c_int
                    {
                        current_block = 16794583624483845564;
                    } else {
                        unsafe {
                            obj = valuePop(ctxt);
                            oldlocset = (*obj).user as xmlLocationSetPtr;
                            newlocset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr)
                        };
                        i_0 = 0 as libc::c_int;
                        let safe_oldlocset = unsafe { &mut *oldlocset };
                        loop {
                            if !(i_0 < safe_oldlocset.locNr) {
                                current_block = 6668037405377178701;
                                break;
                            }
                            /*
                             * Run the evaluation with a node list made of a
                             * single item in the nodelocset.
                             */
                            unsafe {
                                (*safe_ctxt.context).node =
                                    (**(*oldlocset).locTab.offset(i_0 as isize)).user as xmlNodePtr;
                                (*safe_ctxt.context).contextSize = (*oldlocset).locNr;
                                (*safe_ctxt.context).proximityPosition = i_0 + 1 as libc::c_int;
                                tmp = xmlXPathCacheNewNodeSet(
                                    safe_ctxt.context,
                                    (*safe_ctxt.context).node,
                                );
                                valuePush(ctxt, tmp);
                            }
                            if safe_op.ch2 != -(1 as libc::c_int) {
                                total += unsafe {
                                    xmlXPathCompOpEval(
                                        ctxt,
                                        &mut *safe_comp.steps.offset(safe_op.ch2 as isize),
                                    )
                                }
                            }
                            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                                unsafe { xmlXPtrFreeLocationSet(newlocset) };
                                current_block = 4361884637321370770;
                                break;
                            } else {
                                res = unsafe { valuePop(ctxt) };
                                let safe_res = unsafe { &mut *res };
                                if safe_res.type_0 as libc::c_uint
                                    == XPATH_LOCATIONSET as libc::c_int as libc::c_uint
                                {
                                    let mut rloc: xmlLocationSetPtr =
                                        safe_res.user as xmlLocationSetPtr;
                                    j = 0 as libc::c_int;
                                    let safe_rloc = unsafe { &mut *rloc };
                                    while j < safe_rloc.locNr {
                                        range = unsafe {
                                            xmlXPtrNewRange(
                                                (**(*oldlocset).locTab.offset(i_0 as isize)).user
                                                    as xmlNodePtr,
                                                (**(*oldlocset).locTab.offset(i_0 as isize)).index,
                                                (**safe_rloc.locTab.offset(j as isize)).user2
                                                    as xmlNodePtr,
                                                (**safe_rloc.locTab.offset(j as isize)).index2,
                                            )
                                        };
                                        if !range.is_null() {
                                            unsafe { xmlXPtrLocationSetAdd(newlocset, range) };
                                        }
                                        j += 1
                                    }
                                } else {
                                    range = unsafe {
                                        xmlXPtrNewRangeNodeObject(
                                            (**(*oldlocset).locTab.offset(i_0 as isize)).user
                                                as xmlNodePtr,
                                            res,
                                        )
                                    };
                                    if !range.is_null() {
                                        unsafe { xmlXPtrLocationSetAdd(newlocset, range) };
                                    }
                                }
                                /*
                                 * Cleanup
                                 */
                                if !res.is_null() {
                                    unsafe { xmlXPathReleaseObject(safe_ctxt.context, res) };
                                }
                                if safe_ctxt.value == tmp {
                                    res = unsafe { valuePop(ctxt) };
                                    unsafe { xmlXPathReleaseObject(safe_ctxt.context, res) };
                                }
                                i_0 += 1
                            }
                        }
                    }
                } else {
                    if safe_ctxt.value.is_null()
                        || unsafe { (*safe_ctxt.value).type_0 as libc::c_uint }
                            != XPATH_NODESET as libc::c_int as libc::c_uint
                    {
                        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int) };
                        return 0 as libc::c_int;
                    }
                    obj = unsafe { valuePop(ctxt) };
                    let safe_obj = unsafe { &mut *obj };
                    oldset = safe_obj.nodesetval;
                    newlocset = unsafe { xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr) };
                    let safe_oldset = unsafe { &mut *oldset };
                    if !oldset.is_null() {
                        i_0 = 0 as libc::c_int;
                        loop {
                            if !(i_0 < safe_oldset.nodeNr) {
                                current_block = 6668037405377178701;
                                break;
                            }
                            /*
                             * Run the evaluation with a node list made of a single item
                             * in the nodeset.
                             */
                            unsafe {
                                (*safe_ctxt.context).node =
                                    *safe_oldset.nodeTab.offset(i_0 as isize)
                            };
                            /*
                             * OPTIMIZE TODO: Avoid recreation for every iteration.
                             */
                            tmp = unsafe {
                                xmlXPathCacheNewNodeSet(
                                    safe_ctxt.context,
                                    (*safe_ctxt.context).node,
                                )
                            };
                            unsafe { valuePush(ctxt, tmp) };
                            if safe_op.ch2 != -(1 as libc::c_int) {
                                total += unsafe {
                                    xmlXPathCompOpEval(
                                        ctxt,
                                        &mut *safe_comp.steps.offset(safe_op.ch2 as isize),
                                    )
                                }
                            }
                            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                                unsafe { xmlXPtrFreeLocationSet(newlocset) };
                                current_block = 4361884637321370770;
                                break;
                            } else {
                                res = unsafe { valuePop(ctxt) };
                                range = unsafe {
                                    xmlXPtrNewRangeNodeObject(
                                        *safe_oldset.nodeTab.offset(i_0 as isize),
                                        res,
                                    )
                                };
                                if !range.is_null() {
                                    unsafe { xmlXPtrLocationSetAdd(newlocset, range) };
                                }
                                /*
                                 * Cleanup
                                 */
                                if !res.is_null() {
                                    unsafe { xmlXPathReleaseObject(safe_ctxt.context, res) };
                                }
                                if safe_ctxt.value == tmp {
                                    res = unsafe { valuePop(ctxt) };
                                    unsafe { xmlXPathReleaseObject(safe_ctxt.context, res) };
                                }
                                i_0 += 1
                            }
                        }
                    } else {
                        current_block = 6668037405377178701;
                    }
                }
                match current_block {
                    16794583624483845564 => {}
                    _ => {
                        match current_block {
                            6668037405377178701 => {
                                /*
                                 * The result is used as the new evaluation set.
                                 */
                                unsafe { valuePush(ctxt, xmlXPtrWrapLocationSet(newlocset)) };
                            }
                            _ => {}
                        }
                        unsafe {
                            xmlXPathReleaseObject(safe_ctxt.context, obj);
                            (*safe_ctxt.context).node = oldnode;
                            (*safe_ctxt.context).contextSize = oldcs;
                            (*safe_ctxt.context).proximityPosition = oldpp
                        }
                    }
                }
            }
        }
        _ => {
            /* LIBXML_XPTR_ENABLED */
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"XPath: unknown precompiled operation %d\n\x00" as *const u8
                        as *const libc::c_char,
                    safe_op.op as libc::c_uint,
                )
            };
            safe_ctxt.error = XPATH_INVALID_OPERAND as libc::c_int
        }
    }
    unsafe { (*safe_ctxt.context).depth -= 1 as libc::c_int };
    return total;
}
/* *
 * xmlXPathCompOpEvalToBoolean: * @ctxt: the XPath parser context
 *
 * Evaluates if the expression evaluates to true.
 *
 * Returns 1 if true, 0 if false and -1 on API or internal errors.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompOpEvalToBoolean(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut isPredicate: libc::c_int,
) -> libc::c_int {
    let mut resObj: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    loop {
        if unsafe {
            (*(*ctxt).context).opLimit != 0 as libc::c_int as libc::c_ulong
                && xmlXPathCheckOpLimit(ctxt, 1 as libc::c_int as libc::c_ulong) < 0 as libc::c_int
        } {
            return 0 as libc::c_int;
        }
        /* comp = ctxt->comp; */
        match unsafe { (*op).op as libc::c_uint } {
            0 => return 0 as libc::c_int,
            11 => {
                unsafe { resObj = (*op).value4 as xmlXPathObjectPtr };
                if isPredicate != 0 {
                    return unsafe { xmlXPathEvaluatePredicateResult(ctxt, resObj) };
                }
                return unsafe { xmlXPathCastToBoolean(resObj) };
            }
            17 => {
                /*
                 * We don't need sorting for boolean results. Skip this one.
                 */
                if unsafe { (*op).ch1 != -(1 as libc::c_int) } {
                    op = unsafe {
                        &mut *(*(*ctxt).comp).steps.offset((*op).ch1 as isize)
                            as *mut xmlXPathStepOp
                    }
                } else {
                    return 0 as libc::c_int;
                }
            }
            10 => {
                if unsafe { (*op).ch1 == -(1 as libc::c_int) } {
                    return 0 as libc::c_int;
                }
                unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *(*(*ctxt).comp).steps.offset((*op).ch1 as isize))
                };
                if unsafe { (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int } {
                    return -(1 as libc::c_int);
                }
                unsafe {
                    xmlXPathNodeCollectAndTest(
                        ctxt,
                        op,
                        0 as *mut xmlNodePtr,
                        0 as *mut xmlNodePtr,
                        1 as libc::c_int,
                    )
                };
                if unsafe { (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int } {
                    return -(1 as libc::c_int);
                }
                resObj = unsafe { valuePop(ctxt) };
                if resObj.is_null() {
                    return -(1 as libc::c_int);
                }
                break;
            }
            _ => {
                /*
                 * Fallback to call xmlXPathCompOpEval().
                 */
                unsafe { xmlXPathCompOpEval(ctxt, op) };
                if unsafe { (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int } {
                    return -(1 as libc::c_int);
                }
                resObj = unsafe { valuePop(ctxt) };
                if resObj.is_null() {
                    return -(1 as libc::c_int);
                }
                break;
            }
        }
    }
    if !resObj.is_null() {
        let mut res: libc::c_int = 0;
        let safe_resObj = unsafe { &mut *resObj };
        if safe_resObj.type_0 as libc::c_uint == XPATH_BOOLEAN as libc::c_int as libc::c_uint {
            res = safe_resObj.boolval
        } else if isPredicate != 0 {
            /*
             * For predicates a result of type "number" is handled
             * differently:
             * SPEC XPath 1.0:
             * "If the result is a number, the result will be converted
             *  to true if the number is equal to the context position
             *  and will be converted to false otherwise;"
             */
            res = unsafe { xmlXPathEvaluatePredicateResult(ctxt, resObj) }
        } else {
            res = unsafe { xmlXPathCastToBoolean(resObj) }
        }
        unsafe { xmlXPathReleaseObject((*ctxt).context, resObj) };
        return res;
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathRunStreamEval: * @ctxt: the XPath parser context with the compiled expression
 *
 * Evaluate the Precompiled Streamable XPath expression in the given context.
 */
#[cfg(XPATH_STREAMING)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathRunStreamEval(
    mut ctxt: xmlXPathContextPtr,
    mut comp: xmlPatternPtr,
    mut resultSeq: *mut xmlXPathObjectPtr,
    mut toBool: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut max_depth: libc::c_int = 0;
    let mut min_depth: libc::c_int = 0;
    let mut from_root: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut eval_all_nodes: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as xmlNodePtr;
    let mut limit: xmlNodePtr = 0 as xmlNodePtr;
    let mut patstream: xmlStreamCtxtPtr = 0 as xmlStreamCtxtPtr;
    let mut nb_nodes: libc::c_int = 0 as libc::c_int;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || comp.is_null() {
        return -(1 as libc::c_int);
    }
    max_depth = unsafe { xmlPatternMaxDepth(comp) };
    if max_depth == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if max_depth == -(2 as libc::c_int) {
        max_depth = 10000 as libc::c_int
    }
    min_depth = unsafe { xmlPatternMinDepth(comp) };
    if min_depth == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    from_root = unsafe { xmlPatternFromRoot(comp) };
    if from_root < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if toBool == 0 {
        if resultSeq.is_null() {
            return -(1 as libc::c_int);
        }
        unsafe {
            *resultSeq = xmlXPathCacheNewNodeSet(ctxt, 0 as xmlNodePtr);
            if (*resultSeq).is_null() {
                return -(1 as libc::c_int);
            }
        }
    }
    /*
     * handle the special cases of "/" amd "." being matched
     */

    if min_depth == 0 as libc::c_int {
        if from_root != 0 {
            /* Select "/" */
            if toBool != 0 {
                return 1 as libc::c_int;
            }
            /* TODO: Check memory error. */
            unsafe {
                xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, safe_ctxt.doc as xmlNodePtr)
            };
        } else {
            /* Select "self::node()" */
            if toBool != 0 {
                return 1 as libc::c_int;
            }
            /* TODO: Check memory error. */
            unsafe { xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, safe_ctxt.node) };
        }
    }
    if max_depth == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if from_root != 0 {
        cur = safe_ctxt.doc as xmlNodePtr
    } else if !safe_ctxt.node.is_null() {
        match unsafe { (*safe_ctxt.node).type_0 as libc::c_uint } {
            1 | 9 | 11 | 13 | 21 => cur = safe_ctxt.node,
            2 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 10 | 15 | 16 | 17 | 18 | 19 | 20 | _ => {}
        }
        limit = cur
    }
    if cur.is_null() {
        return 0 as libc::c_int;
    }
    patstream = unsafe { xmlPatternGetStreamCtxt(comp) };
    if patstream.is_null() {
        /*
         * QUESTION TODO: Is this an error?
         */
        return 0 as libc::c_int;
    }
    eval_all_nodes = unsafe { xmlStreamWantsAnyNode(patstream) };

    if from_root != 0 {
        ret = unsafe { xmlStreamPush(patstream, 0 as *const xmlChar, 0 as *const xmlChar) };
        if ret < 0 as libc::c_int {
            current_block = 7252614138838059896;
        } else if ret == 1 as libc::c_int {
            if toBool != 0 {
                current_block = 13099311194446884672;
            } else {
                /* TODO: Check memory error. */
                unsafe { xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, cur) };
                current_block = 7252614138838059896;
            }
        } else {
            current_block = 7252614138838059896;
        }
    } else {
        current_block = 7252614138838059896;
    }
    let safe_cur = unsafe { &mut *cur };
    unsafe {
        match current_block {
            7252614138838059896 => {
                depth = 0 as libc::c_int;
                'c_52613: loop {
                    if (*cur).type_0 as libc::c_uint
                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                    {
                        current_block = 16903048813113120619;
                        break;
                    }
                    if !(*cur).children.is_null() && depth < max_depth {
                        /*
                         * Do not descend on entities declarations
                         */
                        if (*(*cur).children).type_0 as libc::c_uint
                            != XML_ENTITY_DECL as libc::c_int as libc::c_uint
                        {
                            cur = (*cur).children;
                            depth += 1;
                            /*
                             * Skip DTDs
                             */
                            if (*cur).type_0 as libc::c_uint
                                != XML_DTD_NODE as libc::c_int as libc::c_uint
                            {
                                current_block = 12930649117290160518;
                            } else {
                                current_block = 5700653730392116747;
                            }
                        } else {
                            current_block = 5700653730392116747;
                        }
                    } else {
                        current_block = 5700653730392116747;
                    }
                    match current_block {
                        5700653730392116747 => {
                            if cur == limit {
                                current_block = 16903048813113120619;
                                break;
                            }
                            loop {
                                if (*cur).next.is_null() {
                                    current_block = 11064061988481400464;
                                    break;
                                }
                                cur = (*cur).next;
                                if (*cur).type_0 as libc::c_uint
                                    != XML_ENTITY_DECL as libc::c_int as libc::c_uint
                                    && (*cur).type_0 as libc::c_uint
                                        != XML_DTD_NODE as libc::c_int as libc::c_uint
                                {
                                    current_block = 5807581744382915773;
                                    break;
                                }
                            }
                            match current_block {
                                5807581744382915773 => {}
                                _ => {
                                    loop {
                                        cur = (*cur).parent;
                                        depth -= 1;
                                        if cur.is_null()
                                            || cur == limit
                                            || (*cur).type_0 as libc::c_uint
                                                == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
                                        {
                                            current_block = 16903048813113120619;
                                            break 'c_52613;
                                        }
                                        if (*cur).type_0 as libc::c_uint
                                            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                                        {
                                            ret = xmlStreamPop(patstream)
                                        } else if eval_all_nodes != 0
                                            && ((*cur).type_0 as libc::c_uint
                                                == XML_TEXT_NODE as libc::c_int as libc::c_uint
                                                || (*cur).type_0 as libc::c_uint
                                                    == XML_CDATA_SECTION_NODE as libc::c_int
                                                        as libc::c_uint
                                                || (*cur).type_0 as libc::c_uint
                                                    == XML_COMMENT_NODE as libc::c_int
                                                        as libc::c_uint
                                                || (*cur).type_0 as libc::c_uint
                                                    == XML_PI_NODE as libc::c_int as libc::c_uint)
                                        {
                                            ret = xmlStreamPop(patstream)
                                        }
                                        if !(*cur).next.is_null() {
                                            cur = (*cur).next;
                                            break;
                                        } else if cur.is_null() {
                                            break;
                                        }
                                    }
                                    current_block = 12930649117290160518;
                                }
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        12930649117290160518 => {
                            if !(!cur.is_null() && depth >= 0 as libc::c_int) {
                                current_block = 16903048813113120619;
                                break;
                            }
                        }
                        _ => {}
                    }
                    's_317: loop {
                        if (*ctxt).opLimit != 0 as libc::c_int as libc::c_ulong {
                            if (*ctxt).opCount >= (*ctxt).opLimit {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"XPath operation limit exceeded\n\x00" as *const u8
                                        as *const libc::c_char,
                                );
                                xmlFreeStreamCtxt(patstream);
                                return -(1 as libc::c_int);
                            }
                            (*ctxt).opCount = (*ctxt).opCount.wrapping_add(1)
                        }
                        nb_nodes += 1;
                        match (*cur).type_0 as libc::c_uint {
                            1 | 3 | 4 | 8 | 7 => {}
                            _ => {
                                break;
                            }
                        }
                        if (*cur).type_0 as libc::c_uint
                            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                        {
                            ret = xmlStreamPush(
                                patstream,
                                (*cur).name,
                                if !(*cur).ns.is_null() {
                                    (*(*cur).ns).href
                                } else {
                                    0 as *const xmlChar
                                },
                            )
                        } else {
                            if !(eval_all_nodes != 0) {
                                break;
                            }
                            ret = xmlStreamPushNode(
                                patstream,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                                (*cur).type_0 as libc::c_int,
                            )
                        }
                        if !(ret < 0 as libc::c_int) {
                            if ret == 1 as libc::c_int {
                                if toBool != 0 {
                                    current_block = 13099311194446884672;
                                    break 'c_52613;
                                }
                                if xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, cur)
                                    < 0 as libc::c_int
                                {
                                    (*ctxt).lastError.domain = XML_FROM_XPATH as libc::c_int;
                                    (*ctxt).lastError.code = XML_ERR_NO_MEMORY as libc::c_int
                                }
                            }
                        }
                        /* NOP. */
                        if !((*cur).children.is_null() || depth >= max_depth) {
                            break;
                        }
                        ret = xmlStreamPop(patstream);
                        loop {
                            if (*cur).next.is_null() {
                                break 's_317;
                            }
                            cur = (*cur).next;
                            if (*cur).type_0 as libc::c_uint
                                != XML_ENTITY_DECL as libc::c_int as libc::c_uint
                                && (*cur).type_0 as libc::c_uint
                                    != XML_DTD_NODE as libc::c_int as libc::c_uint
                            {
                                break;
                            }
                        }
                    }
                }
                match current_block {
                    13099311194446884672 => {}
                    _ => {
                        if !patstream.is_null() {
                            xmlFreeStreamCtxt(patstream);
                        }
                        return 0 as libc::c_int;
                    }
                }
            }
            _ => {}
        }
    }
    if !patstream.is_null() {
        unsafe { xmlFreeStreamCtxt(patstream) };
    }

    return 1 as libc::c_int;
}
/* XPATH_STREAMING */
/* *
 * xmlXPathRunEval: * @ctxt: the XPath parser context with the compiled expression
 * @toBool: evaluate to a boolean result
 *
 * Evaluate the Precompiled XPath expression in the given context.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathRunEval(mut ctxt: xmlXPathParserContextPtr, mut toBool: libc::c_int) -> libc::c_int {
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.comp.is_null() {
        return -(1 as libc::c_int);
    }
    unsafe { (*safe_ctxt.context).depth = 0 as libc::c_int };
    if safe_ctxt.valueTab.is_null() {
        /* Allocate the value stack */
        safe_ctxt.valueTab = unsafe {
            xmlMalloc.expect("non-null function pointer")(
                (10 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>() as libc::c_ulong),
            ) as *mut xmlXPathObjectPtr
        };
        if safe_ctxt.valueTab.is_null() {
            unsafe {
                xmlXPathPErrMemory(
                    ctxt,
                    b"creating evaluation context\n\x00" as *const u8 as *const libc::c_char,
                )
            };
            unsafe { xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void) };
        }
        safe_ctxt.valueNr = 0 as libc::c_int;
        safe_ctxt.valueMax = 10 as libc::c_int;
        safe_ctxt.value = 0 as xmlXPathObjectPtr;
        safe_ctxt.valueFrame = 0 as libc::c_int
    }
    // XPATH_STREAMING
    match () {
        #[cfg(XPATH_STREAMING)]
        _ => {
            if !unsafe { (*safe_ctxt.comp).stream.is_null() } {
                let mut res: libc::c_int = 0;
                if toBool != 0 {
                    /*
                     * Evaluation to boolean result.
                     */
                    res = unsafe {
                        xmlXPathRunStreamEval(
                            safe_ctxt.context,
                            (*safe_ctxt.comp).stream,
                            0 as *mut xmlXPathObjectPtr,
                            1 as libc::c_int,
                        )
                    };
                    if res != -(1 as libc::c_int) {
                        return res;
                    }
                } else {
                    let mut resObj: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
                    /*
                     * Evaluation to a sequence.
                     */
                    res = unsafe {
                        xmlXPathRunStreamEval(
                            safe_ctxt.context,
                            (*safe_ctxt.comp).stream,
                            &mut resObj,
                            0 as libc::c_int,
                        )
                    };
                    if res != -(1 as libc::c_int) && !resObj.is_null() {
                        unsafe { valuePush(ctxt, resObj) };
                        return 0 as libc::c_int;
                    }
                    if !resObj.is_null() {
                        unsafe { xmlXPathReleaseObject(safe_ctxt.context, resObj) };
                    }
                }
                /*
                 * QUESTION TODO: This falls back to normal XPath evaluation
                 * if res == -1. Is this intended?
                 */
            }
        }
        // endif XPATH_STREAMING
        #[cfg(not(XPATH_STREAMING))]
        _ => {}
    }

    comp = safe_ctxt.comp;
    let safe_comp = unsafe { &mut *comp };
    if safe_comp.last < 0 as libc::c_int {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlXPathRunEval: last is less than zero\n\x00" as *const u8
                    as *const libc::c_char,
            )
        };
        return -(1 as libc::c_int);
    }
    if toBool != 0 {
        return unsafe {
            xmlXPathCompOpEvalToBoolean(
                ctxt,
                &mut *safe_comp.steps.offset(safe_comp.last as isize),
                0 as libc::c_int,
            )
        };
    } else {
        unsafe { xmlXPathCompOpEval(ctxt, &mut *(*comp).steps.offset((*comp).last as isize)) };
    }
    return 0 as libc::c_int;
}
/* ***********************************************************************
 *									*
 *			Public interfaces				*
 *									*
 ************************************************************************/
/* *
 * xmlXPathEvalPredicate: * @ctxt: the XPath context
 * @res: the Predicate Expression evaluation result
 *
 * Evaluate a predicate result for the current node.
 * A PredicateExpr is evaluated by evaluating the Expr and converting
 * the result to a boolean. If the result is a number, the result will
 * be converted to true if the number is equal to the position of the
 * context node in the context node list (as returned by the position
 * function) and will be converted to false otherwise; if the result
 * is not a number, then the result will be converted as if by a call
 * to the boolean function.
 *
 * Returns 1 if predicate is true, 0 otherwise
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathEvalPredicate(
    mut ctxt: xmlXPathContextPtr,
    mut res: xmlXPathObjectPtr,
) -> libc::c_int {
    let safe_res = unsafe { &mut *res };
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || res.is_null() {
        return 0 as libc::c_int;
    }
    match safe_res.type_0 as libc::c_uint {
        2 => return safe_res.boolval,
        3 => {
            return (safe_res.floatval == safe_ctxt.proximityPosition as libc::c_double)
                as libc::c_int
        }
        1 | 9 => {
            if safe_res.nodesetval.is_null() {
                return 0 as libc::c_int;
            }
            return unsafe { ((*safe_res.nodesetval).nodeNr != 0 as libc::c_int) as libc::c_int };
        }
        4 => {
            return (!safe_res.stringval.is_null()
                && unsafe { xmlStrlen(safe_res.stringval) != 0 as libc::c_int })
                as libc::c_int
        }
        _ => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Internal error at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                    b"xpath.c\x00" as *const u8 as *const libc::c_char,
                    13998 as libc::c_int,
                )
            };
        }
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathEvaluatePredicateResult: * @ctxt: the XPath Parser context
 * @res: the Predicate Expression evaluation result
 *
 * Evaluate a predicate result for the current node.
 * A PredicateExpr is evaluated by evaluating the Expr and converting
 * the result to a boolean. If the result is a number, the result will
 * be converted to true if the number is equal to the position of the
 * context node in the context node list (as returned by the position
 * function) and will be converted to false otherwise; if the result
 * is not a number, then the result will be converted as if by a call
 * to the boolean function.
 *
 * Returns 1 if predicate is true, 0 otherwise
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathEvaluatePredicateResult(
    mut ctxt: xmlXPathParserContextPtr,
    mut res: xmlXPathObjectPtr,
) -> libc::c_int {
    let safe_res = unsafe { &mut *res };
    if ctxt.is_null() || res.is_null() {
        return 0 as libc::c_int;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    match safe_res.type_0 as libc::c_uint {
        2 => return safe_res.boolval,
        3 => {
            match () {
                #[cfg(BORLANDC_OR_MSC_VER_AND_MSC_VER)]
                _ => {
                    return (safe_res.floatval
                        == unsafe { (*safe_ctxt.context).proximityPosition as libc::c_double }
                        && xmlXPathIsNaN(safe_res.floatval) != 0)
                        as libc::c_int
                }
                #[cfg(not(BORLANDC_OR_MSC_VER_AND_MSC_VER))]
                _ => {
                    return (safe_res.floatval
                        == unsafe { (*safe_ctxt.context).proximityPosition as libc::c_double })
                        as libc::c_int
                }
            };
        }
        1 | 9 => {
            if safe_res.nodesetval.is_null() {
                return 0 as libc::c_int;
            }
            return unsafe { ((*safe_res.nodesetval).nodeNr != 0 as libc::c_int) as libc::c_int };
        }
        4 => {
            return (!safe_res.stringval.is_null()
                && unsafe {
                    *safe_res.stringval.offset(0 as libc::c_int as isize) as libc::c_int
                        != 0 as libc::c_int
                }) as libc::c_int
        }
        7 => {
            match () {
                #[cfg(XMLXPATHNODESETSORT)]
                _ => {
                    let mut ptr: xmlLocationSetPtr = safe_res.user as xmlLocationSetPtr;
                    let safe_ptr = unsafe { &mut *ptr };
                    if ptr.is_null() {
                        return 0 as libc::c_int;
                    }
                    return (safe_ptr.locNr != 0 as libc::c_int) as libc::c_int;
                }
                #[cfg(not(XMLXPATHNODESETSORT))]
                _ => {}
            };
        }
        _ => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Internal error at %s:%d\n\x00" as *const u8 as *const libc::c_char,
                    b"xpath.c\x00" as *const u8 as *const libc::c_char,
                    14049 as libc::c_int,
                )
            };
        }
    }
    return 0 as libc::c_int;
}
/* *
 * xmlXPathTryStreamCompile: * @ctxt: an XPath context
 * @str: the XPath expression
 *
 * Try to compile the XPath expression as a streamable subset.
 *
 * Returns the compiled expression or NULL if failed to compile.
 */
#[cfg(XPATH_STREAMING)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathTryStreamCompile(
    mut ctxt: xmlXPathContextPtr,
    mut str: *const xmlChar,
) -> xmlXPathCompExprPtr {
    /*
     * Optimization: use streaming patterns when the XPath expression can
     * be compiled to a stream lookup
     */
    let mut stream: xmlPatternPtr = 0 as *mut xmlPattern;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    let mut namespaces: *mut *const xmlChar = 0 as *mut *const xmlChar;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if unsafe {
        xmlStrchr(str, '[' as i32 as xmlChar).is_null()
            && xmlStrchr(str, '(' as i32 as xmlChar).is_null()
            && xmlStrchr(str, '@' as i32 as xmlChar).is_null()
    } {
        let mut tmp: *const xmlChar = 0 as *const xmlChar;
        /*
         * We don't try to handle expressions using the verbose axis
         * specifiers ("::"), just the simplified form at this point.
         * Additionally, if there is no list of namespaces available and
         *  there's a ":" in the expression, indicating a prefixed QName,
         *  then we won't try to compile either. xmlPatterncompile() needs
         *  to have a list of namespaces at compilation time in order to
         *  compile prefixed name tests.
         */
        tmp = unsafe { xmlStrchr(str, ':' as i32 as xmlChar) };
        let safe_ctxt = unsafe { &mut *ctxt };
        if !tmp.is_null()
            && (ctxt.is_null()
                || safe_ctxt.nsNr == 0 as libc::c_int
                || unsafe { *tmp.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 })
        {
            return 0 as xmlXPathCompExprPtr;
        }
        if !ctxt.is_null() {
            dict = safe_ctxt.dict;
            if safe_ctxt.nsNr > 0 as libc::c_int {
                namespaces = unsafe {
                    xmlMalloc.expect("non-null function pointer")(
                        ((2 as libc::c_int * (safe_ctxt.nsNr + 1 as libc::c_int)) as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
                    ) as *mut *const xmlChar
                };
                if namespaces.is_null() {
                    unsafe {
                        xmlXPathErrMemory(
                            ctxt,
                            b"allocating namespaces array\n\x00" as *const u8
                                as *const libc::c_char,
                        )
                    };
                    return 0 as xmlXPathCompExprPtr;
                }

                let safe_namespaces = unsafe { &mut *namespaces };
                i = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < safe_ctxt.nsNr {
                    ns = unsafe { *safe_ctxt.namespaces.offset(j as isize) };
                    let fresh80 = i;
                    i = i + 1;
                    let ref mut fresh81 = unsafe { safe_namespaces.offset(fresh80 as isize) };
                    let safe_ns = unsafe { &mut *ns };
                    unsafe {
                        *fresh81 = safe_ns.href;
                    }
                    let fresh82 = i;
                    i = i + 1;
                    unsafe {
                        let ref mut fresh83 = *namespaces.offset(fresh82 as isize);
                        *fresh83 = safe_ns.prefix;
                    }
                    j += 1
                }
                let fresh84 = i;
                i = i + 1;
                unsafe {
                    let ref mut fresh85 = *namespaces.offset(fresh84 as isize);
                    *fresh85 = 0 as *const xmlChar;
                    let ref mut fresh86 = *namespaces.offset(i as isize);
                    *fresh86 = 0 as *const xmlChar
                }
            }
        }
        stream =
            unsafe { xmlPatterncompile(str, dict, XML_PATTERN_XPATH as libc::c_int, namespaces) };
        if !namespaces.is_null() {
            unsafe {
                xmlFree.expect("non-null function pointer")(
                    namespaces as *mut *mut xmlChar as *mut libc::c_void,
                )
            };
        }
        if !stream.is_null() && unsafe { xmlPatternStreamable(stream) == 1 as libc::c_int } {
            comp = unsafe { xmlXPathNewCompExpr() };
            if comp.is_null() {
                unsafe {
                    xmlXPathErrMemory(
                        ctxt,
                        b"allocating streamable expression\n\x00" as *const u8
                            as *const libc::c_char,
                    )
                };
                return 0 as xmlXPathCompExprPtr;
            }
            let safe_comp = unsafe { &mut *comp };
            safe_comp.stream = stream;
            safe_comp.dict = dict;
            if !safe_comp.dict.is_null() {
                unsafe { xmlDictReference(safe_comp.dict) };
            }
            return comp;
        }
        unsafe { xmlFreePattern(stream) };
    }
    return 0 as xmlXPathCompExprPtr;
}
/* XPATH_STREAMING */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathOptimizeExpression(mut pctxt: xmlXPathParserContextPtr, mut op: xmlXPathStepOpPtr) {
    let safe_pctxt = unsafe { &mut *pctxt };
    let mut comp: xmlXPathCompExprPtr = safe_pctxt.comp;
    let mut ctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    /*
     * Try to rewrite "descendant-or-self::node()/foo" to an optimized
     * internal representation.
     */
    let safe_op = unsafe { &mut *op };
    let safe_comp = unsafe { &mut *comp };
    if safe_op.op as libc::c_uint == XPATH_OP_COLLECT as libc::c_int as libc::c_uint
        && safe_op.ch1 != -(1 as libc::c_int)
        && safe_op.ch2 == -(1 as libc::c_int)
    {
        let mut prevop: xmlXPathStepOpPtr =
            unsafe { &mut *safe_comp.steps.offset(safe_op.ch1 as isize) as *mut xmlXPathStepOp };
        let safe_prevop = unsafe { &mut *prevop };
        if safe_prevop.op as libc::c_uint == XPATH_OP_COLLECT as libc::c_int as libc::c_uint
            && safe_prevop.value as xmlXPathAxisVal as libc::c_uint
                == AXIS_DESCENDANT_OR_SELF as libc::c_int as libc::c_uint
            && safe_prevop.ch2 == -(1 as libc::c_int)
            && safe_prevop.value2 as xmlXPathTestVal as libc::c_uint
                == NODE_TEST_TYPE as libc::c_int as libc::c_uint
            && safe_prevop.value3 as xmlXPathTypeVal as libc::c_uint
                == NODE_TYPE_NODE as libc::c_int as libc::c_uint
        {
            /*
             * This is a "descendant-or-self::node()" without predicates.
             * Try to eliminate it.
             */
            match safe_op.value as xmlXPathAxisVal as libc::c_uint {
                4 | 5 => {
                    /*
                     * Convert "descendant-or-self::node()/child::" or
                     * "descendant-or-self::node()/descendant::" to
                     * "descendant::"
                     */
                    safe_op.ch1 = safe_prevop.ch1;
                    safe_op.value = AXIS_DESCENDANT as libc::c_int
                }
                13 | 6 => {
                    /*
                     * Convert "descendant-or-self::node()/self::" or
                     * "descendant-or-self::node()/descendant-or-self::" to
                     * to "descendant-or-self::"
                     */
                    safe_op.ch1 = safe_prevop.ch1;
                    safe_op.value = AXIS_DESCENDANT_OR_SELF as libc::c_int
                }
                _ => {}
            }
        }
    }
    /* OP_VALUE has invalid ch1. */
    if safe_op.op as libc::c_uint == XPATH_OP_VALUE as libc::c_int as libc::c_uint {
        return;
    }
    /* Recurse */
    ctxt = safe_pctxt.context;
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() {
        if safe_ctxt.depth >= 5000 as libc::c_int {
            return;
        }
        safe_ctxt.depth += 1 as libc::c_int
    }
    if safe_op.ch1 != -(1 as libc::c_int) {
        unsafe {
            xmlXPathOptimizeExpression(pctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
        };
    }
    if safe_op.ch2 != -(1 as libc::c_int) {
        unsafe {
            xmlXPathOptimizeExpression(pctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
        };
    }
    if !ctxt.is_null() {
        safe_ctxt.depth -= 1 as libc::c_int
    };
}
/* *
 * xmlXPathCtxtCompile: * @ctxt: an XPath context
 * @str: the XPath expression
 *
 * Compile an XPath expression
 *
 * Returns the xmlXPathCompExprPtr resulting from the compilation or NULL.
 *         the caller has to free the object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCtxtCompile(
    mut ctxt: xmlXPathContextPtr,
    mut str: *const xmlChar,
) -> xmlXPathCompExprPtr {
    let mut pctxt: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;

    match () {
        #[cfg(XPATH_STREAMING)]
        _ => {
            comp = unsafe { xmlXPathTryStreamCompile(ctxt, str) };
            if !comp.is_null() {
                return comp;
            }
        }
        #[cfg(not(XPATH_STREAMING))]
        _ => {}
    }
    unsafe { xmlInitParser() };
    pctxt = unsafe { xmlXPathNewParserContext(str, ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    let safe_pctxt = unsafe { &mut *pctxt };
    if pctxt.is_null() {
        return 0 as xmlXPathCompExprPtr;
    }
    if !ctxt.is_null() {
        safe_ctxt.depth = 0 as libc::c_int
    }
    unsafe { xmlXPathCompileExpr(pctxt, 1 as libc::c_int) };
    if safe_pctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        unsafe { xmlXPathFreeParserContext(pctxt) };
        return 0 as xmlXPathCompExprPtr;
    }
    if unsafe { *safe_pctxt.cur as libc::c_int != 0 as libc::c_int } {
        /*
         * aleksey: in some cases this line prints *second* error message
         * (see bug #78858) and probably this should be fixed.
         * However, we are not sure that all error messages are printed
         * out in other places. It's not critical so we leave it as-is for now
         */
        unsafe {
            xmlXPatherror(
                pctxt,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                14254 as libc::c_int,
                XPATH_EXPR_ERROR as libc::c_int,
            )
        };
        comp = 0 as xmlXPathCompExprPtr
    } else {
        comp = safe_pctxt.comp;
        let safe_comp = unsafe { &mut *comp };
        if safe_comp.nbStep > 1 as libc::c_int && safe_comp.last >= 0 as libc::c_int {
            if !ctxt.is_null() {
                safe_ctxt.depth = 0 as libc::c_int
            }
            unsafe {
                xmlXPathOptimizeExpression(
                    pctxt,
                    &mut *safe_comp.steps.offset(safe_comp.last as isize),
                )
            };
        }
        safe_pctxt.comp = 0 as xmlXPathCompExprPtr
    }
    unsafe { xmlXPathFreeParserContext(pctxt) };
    if !comp.is_null() {
        let safe_comp = unsafe { &mut *comp };
        safe_comp.expr = unsafe { xmlStrdup(str) };

        match () {
            #[cfg(DEBUG_EVAL_COUNTS)]
            _ => {
                safe_comp.string = unsafe { xmlStrdup(str) };
                safe_comp.nb = 0 as libc::c_int;
            }
            #[cfg(not(DEBUG_EVAL_COUNTS))]
            _ => {}
        }
    }
    return comp;
}
/* *
 * Separate compilation/evaluation entry points.
 */
/* *
 * xmlXPathCompile: * @str: the XPath expression
 *
 * Compile an XPath expression
 *
 * Returns the xmlXPathCompExprPtr resulting from the compilation or NULL.
 *         the caller has to free the object.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCompile(mut str: *const xmlChar) -> xmlXPathCompExprPtr {
    return unsafe { xmlXPathCtxtCompile(0 as xmlXPathContextPtr, str) };
}
/* *
 * xmlXPathCompiledEvalInternal: * @comp: the compiled XPath expression
 * @ctxt: the XPath context
 * @resObj: the resulting XPath object or NULL
 * @toBool: 1 if only a boolean result is requested
 *
 * Evaluate the Precompiled XPath expression in the given context.
 * The caller has to free @resObj.
 *
 * Returns the xmlXPathObjectPtr resulting from the evaluation or NULL.
 *         the caller has to free the object.
 */

// xmlXPathCompiledEvalInternal中在LIBXML_THREAD_ENABLED宏中定义的局部静态变量
// #[cfg(not(LIBXML_THREAD_ENABLED))]
// static mut reentance: libc::c_int = 0;

#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompiledEvalInternal(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
    mut resObjPtr: *mut xmlXPathObjectPtr,
    mut toBool: libc::c_int,
) -> libc::c_int {
    let mut pctxt: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    let mut resObj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut reentance: libc::c_int;
    match () {
        #[cfg(LIBXML_THREAD_ENABLED)]
        _ => {}
        #[cfg(not(LIBXML_THREAD_ENABLED))]
        _ => {
            // 条件宏内的局部静态变量在rust中好像只局限在#cfg内部
            reentance = 0 as libc::c_int;
        }
    }
    let mut res: libc::c_int = 0;
    if ctxt.is_null() {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                XML_FROM_XPATH as libc::c_int,
                XML_ERR_INTERNAL_ERROR as libc::c_int,
                XML_ERR_FATAL,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                14317 as libc::c_int,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                b"NULL context pointer\n\x00" as *const u8 as *const libc::c_char,
            )
        };
        return -(1 as libc::c_int);
    }
    if comp.is_null() {
        return -(1 as libc::c_int);
    }
    unsafe { xmlInitParser() };

    match () {
        #[cfg(not(LIBXML_THREAD_ENABLED))]
        _ => unsafe {
            reentance += 1;
            if reentance > 1 as libc::c_int {
                xmlXPathDisableOptimizer = 1;
            }
        },
        #[cfg(LIBXML_THREAD_ENABLED)]
        _ => {}
    }
    match () {
        #[cfg(DEBUG_EVAL_COUNTS)]
        _ => {
            (*comp).nb += 1;
            if (!(*comp).string.is_null() && (*comp).nb > 100 as libc::c_int) {
                fprintf(
                    stderr,
                    b"100 x %s\n\x00" as *const u8 as *const libc::c_char,
                    (*comp).string,
                );
                (*comp).nb = 0 as libc::c_int;
            }
        }
        #[cfg(not(DEBUG_EVAL_COUNTS))]
        _ => {}
    }

    pctxt = unsafe { xmlXPathCompParserContext(comp, ctxt) };
    let safe_pctxt = unsafe { &mut *pctxt };
    res = unsafe { xmlXPathRunEval(pctxt, toBool) };
    if safe_pctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        resObj = 0 as xmlXPathObjectPtr
    } else {
        resObj = unsafe { valuePop(pctxt) };

        let safe_xmlGenericError = unsafe { &mut *__xmlGenericError() };
        let safe_xmlGenericErrorContext = unsafe { *__xmlGenericErrorContext() };
        if resObj.is_null() {
            if toBool == 0 {
                unsafe {
                    safe_xmlGenericError.expect("non-null function pointer")(
                        safe_xmlGenericErrorContext,
                        b"xmlXPathCompiledEval: No result on the stack.\n\x00" as *const u8
                            as *const libc::c_char,
                    )
                };
            }
        } else if safe_pctxt.valueNr > 0 as libc::c_int {
            unsafe {
                safe_xmlGenericError.expect("non-null function pointer")(
                    safe_xmlGenericErrorContext,
                    b"xmlXPathCompiledEval: %d object(s) left on the stack.\n\x00" as *const u8
                        as *const libc::c_char,
                    (*pctxt).valueNr,
                )
            };
        }
    }
    if !resObjPtr.is_null() {
        unsafe { *resObjPtr = resObj }
    } else {
        unsafe { xmlXPathReleaseObject(ctxt, resObj) };
    }
    safe_pctxt.comp = 0 as xmlXPathCompExprPtr;
    unsafe { xmlXPathFreeParserContext(pctxt) };
    match () {
        #[cfg(not(LIBXML_THREAD_ENABLED))]
        _ => unsafe {
            reentance -= 1;
        },
        #[cfg(LIBXML_THREAD_ENABLED)]
        _ => {}
    }
    return res;
}

/* *
 * xmlXPathCompiledEval: * @comp: the compiled XPath expression
 * @ctx: the XPath context
 *
 * Evaluate the Precompiled XPath expression in the given context.
 *
 * Returns the xmlXPathObjectPtr resulting from the evaluation or NULL.
 *         the caller has to free the object.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCompiledEval(
    mut comp: xmlXPathCompExprPtr,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let mut res: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    unsafe { xmlXPathCompiledEvalInternal(comp, ctx, &mut res, 0 as libc::c_int) };
    return res;
}
/* *
 * xmlXPathCompiledEvalToBoolean: * @comp: the compiled XPath expression
 * @ctxt: the XPath context
 *
 * Applies the XPath boolean() function on the result of the given
 * compiled expression.
 *
 * Returns 1 if the expression evaluated to true, 0 if to false and
 *         -1 in API and internal errors.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCompiledEvalToBoolean(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
) -> libc::c_int {
    return unsafe {
        xmlXPathCompiledEvalInternal(comp, ctxt, 0 as *mut xmlXPathObjectPtr, 1 as libc::c_int)
    };
}
/* *
 * xmlXPathEvalExpr: * @ctxt: the XPath Parser context
 *
 * Parse and evaluate an XPath expression in the given context, * then push the result on the context stack
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathEvalExpr(mut ctxt: xmlXPathParserContextPtr) {
    let mut comp: xmlXPathCompExprPtr;
    match () {
        #[cfg(XPATH_STREAMING)]
        _ => {
            comp = 0 as *mut xmlXPathCompExpr;
        }
        #[cfg(not(XPATH_STREAMING))]
        _ => {}
    };

    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    match () {
        #[cfg(XPATH_STREAMING)]
        _ => {
            comp = xmlXPathTryStreamCompile(safe_ctxt.context, safe_ctxt.base);
            if !comp.is_null() {
                if !safe_ctxt.comp.is_null() {
                    unsafe { xmlXPathFreeCompExpr(safe_ctxt.comp) };
                }
                safe_ctxt.comp = comp
            } else {
                if !safe_ctxt.context.is_null() {
                    unsafe { (*(*ctxt).context).depth = 0 as libc::c_int }
                }

                xmlXPathCompileExpr(ctxt, 1 as libc::c_int);

                if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                    return;
                }
                /* Check for trailing characters. */
                if unsafe { *safe_ctxt.cur } as libc::c_int != 0 as libc::c_int {
                    unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int) };
                    return;
                }
                if unsafe {
                    (*safe_ctxt.comp).nbStep > 1 as libc::c_int
                        && (*safe_ctxt.comp).last >= 0 as libc::c_int
                } {
                    if !safe_ctxt.context.is_null() {
                        unsafe { (*safe_ctxt.context).depth = 0 as libc::c_int }
                    }
                    xmlXPathOptimizeExpression(ctxt, unsafe {
                        &mut *(*safe_ctxt.comp)
                            .steps
                            .offset((*safe_ctxt.comp).last as isize)
                    });
                }
            }
        }

        #[cfg(not(XPATH_STREAMING))]
        _ => {
            if !safe_ctxt.context.is_null() {
                unsafe { (*safe_ctxt.context).depth = 0 as libc::c_int }
            }
            xmlXPathCompileExpr(ctxt, 1 as libc::c_int);
            if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
                return;
            }
            /* Check for trailing characters. */
            if unsafe { *safe_ctxt.cur as libc::c_int != 0 as libc::c_int } {
                xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
                return;
            }
            if unsafe {
                (*safe_ctxt.comp).nbStep > 1 as libc::c_int
                    && (*safe_ctxt.comp).last >= 0 as libc::c_int
            } {
                if !safe_ctxt.context.is_null() {
                    unsafe { (*safe_ctxt.context).depth = 0 as libc::c_int }
                }
                xmlXPathOptimizeExpression(ctxt, unsafe {
                    &mut *(*safe_ctxt.comp)
                        .steps
                        .offset((*safe_ctxt.comp).last as isize)
                });
            }
        }
    };
    xmlXPathRunEval(ctxt, 0 as libc::c_int);
}
/* *
 * xmlXPathEval: * @str: the XPath expression
 * @ctx: the XPath context
 *
 * Evaluate the XPath Location Path in the given context.
 *
 * Returns the xmlXPathObjectPtr resulting from the evaluation or NULL.
 *         the caller has to free the object.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathEval(mut str: *const xmlChar, mut ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr {
    let mut ctxt: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctx.is_null() {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
                XML_FROM_XPATH as libc::c_int,
                XML_ERR_INTERNAL_ERROR as libc::c_int,
                XML_ERR_FATAL,
                b"xpath.c\x00" as *const u8 as *const libc::c_char,
                14464 as libc::c_int,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                b"NULL context pointer\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as xmlXPathObjectPtr;
    }
    unsafe { xmlInitParser() };
    ctxt = unsafe { xmlXPathNewParserContext(str, ctx) };

    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    unsafe { xmlXPathEvalExpr(ctxt) };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as libc::c_int {
        res = 0 as xmlXPathObjectPtr
    } else {
        let safe_xmlGenericError = unsafe { &mut *__xmlGenericError() };
        let safe_xmlGenericErrorContext = unsafe { *__xmlGenericErrorContext() };

        res = unsafe { valuePop(ctxt) };
        if res.is_null() {
            unsafe {
                safe_xmlGenericError.expect("non-null function pointer")(
                    safe_xmlGenericErrorContext,
                    b"xmlXPathCompiledEval: No result on the stack.\n\x00" as *const u8
                        as *const libc::c_char,
                )
            };
        } else if safe_ctxt.valueNr > 0 as libc::c_int {
            unsafe {
                safe_xmlGenericError.expect("non-null function pointer")(
                    safe_xmlGenericErrorContext,
                    b"xmlXPathCompiledEval: %d object(s) left on the stack.\n\x00" as *const u8
                        as *const libc::c_char,
                    safe_ctxt.valueNr,
                )
            };
        }
    }
    unsafe { xmlXPathFreeParserContext(ctxt) };
    return res;
}
/* *
 * xmlXPathSetContextNode: * @node: the node to to use as the context node
 * @ctx: the XPath context
 *
 * Sets 'node' as the context node. The node must be in the same
 * document as that associated with the context.
 *
 * Returns -1 in case of error or 0 if successful
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathSetContextNode(mut node: xmlNodePtr, mut ctx: xmlXPathContextPtr) -> libc::c_int {
    if node.is_null() || ctx.is_null() {
        return -(1 as libc::c_int);
    }
    let safe_node = unsafe { &mut *node };
    let safe_ctx = unsafe { &mut *ctx };
    if safe_node.doc == safe_ctx.doc {
        safe_ctx.node = node;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
/* *
 * xmlXPathNodeEval: * @node: the node to to use as the context node
 * @str: the XPath expression
 * @ctx: the XPath context
 *
 * Evaluate the XPath Location Path in the given context. The node 'node'
 * is set as the context node. The context node is not restored.
 *
 * Returns the xmlXPathObjectPtr resulting from the evaluation or NULL.
 *         the caller has to free the object.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeEval(
    mut node: xmlNodePtr,
    mut str: *const xmlChar,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    if str.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    if xmlXPathSetContextNode(node, ctx) < 0 as libc::c_int {
        return 0 as xmlXPathObjectPtr;
    }
    return xmlXPathEval(str, ctx);
}
/* *
 * xmlXPathEvalExpression: * @str: the XPath expression
 * @ctxt: the XPath context
 *
 * Alias for xmlXPathEval().
 *
 * Returns the xmlXPathObjectPtr resulting from the evaluation or NULL.
 *         the caller has to free the object.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathEvalExpression(
    mut str: *const xmlChar,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    return unsafe { xmlXPathEval(str, ctxt) };
}
/* ***********************************************************************
 *									*
 *	Extra functions not pertaining to the XPath spec		*
 *									*
 ************************************************************************/
/* *
 * xmlXPathEscapeUriFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the escape-uri() XPath function
 *    string escape-uri(string $str, bool $escape-reserved) *
 * This function applies the URI escaping rules defined in section 2 of [RFC
 * 2396] to the string supplied as $uri-part, which typically represents all
 * or part of a URI. The effect of the function is to replace any special
 * character in the string by an escape sequence of the form %xx%yy..., * where xxyy... is the hexadecimal representation of the octets used to
 * represent the character in UTF-8.
 *
 * The set of characters that are escaped depends on the setting of the
 * boolean argument $escape-reserved.
 *
 * If $escape-reserved is true, all characters are escaped other than lower
 * case letters a-z, upper case letters A-Z, digits 0-9, and the characters
 * referred to in [RFC 2396] as "marks": specifically, "-" | "_" | "." | "!"
 * | "~" | "*" | "'" | "(" | ")". The "%" character itself is escaped only
 * if it is not followed by two hexadecimal digits (that is, 0-9, a-f, and
 * A-F).
 *
 * If $escape-reserved is false, the behavior differs in that characters
 * referred to in [RFC 2396] as reserved characters are not escaped. These
 * characters are ";" | "/" | "?" | ":" | "@" | "&" | "=" | "+" | "$" | ",".
 *
 * [RFC 2396] does not define whether escaped URIs should use lower case or
 * upper case for hexadecimal digits. To ensure that escaped URIs can be
 * compared using string comparison functions, this function must always use
 * the upper-case letters A-F.
 *
 * Generally, $escape-reserved should be set to true when escaping a string
 * that is to form a single part of a URI, and to false when escaping an
 * entire URI or URI reference.
 *
 * In the case of non-ascii characters, the string is encoded according to
 * utf-8 and then converted according to RFC 2396.
 *
 * Examples
 *  xf:escape-uri ("gopher://spinaltap.micro.umn.edu/00/Weather/California/Los%20Angeles#ocean"), true()) *  returns "gopher%3A%2F%2Fspinaltap.micro.umn.edu%2F00%2FWeather%2FCalifornia%2FLos%20Angeles%23ocean"
 *  xf:escape-uri ("gopher://spinaltap.micro.umn.edu/00/Weather/California/Los%20Angeles#ocean"), false()) *  returns "gopher://spinaltap.micro.umn.edu/00/Weather/California/Los%20Angeles%23ocean"
 *
 */
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathEscapeUriFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut escape_reserved: libc::c_int = 0;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut cptr: *mut xmlChar = 0 as *mut xmlChar;
    let mut escape: [xmlChar; 4] = [0; 4];
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 as libc::c_int {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int) };
        return;
    }
    escape_reserved = unsafe { xmlXPathPopBoolean(ctxt) };

    let safe_ctxt2 = unsafe { &mut *safe_ctxt };
    let safe_ctxt_value = unsafe { &mut *(safe_ctxt2.value) };
    if !safe_ctxt.value.is_null()
        && safe_ctxt_value.type_0 as libc::c_uint != XPATH_STRING as libc::c_int as libc::c_uint
    {
        unsafe { xmlXPathStringFunction(ctxt, 1 as libc::c_int) };
    }
    str = unsafe { valuePop(ctxt) };

    let safe_str = unsafe { &mut *str };
    target = unsafe { xmlBufCreate() };
    escape[0 as libc::c_int as usize] = '%' as i32 as xmlChar;
    escape[3 as libc::c_int as usize] = 0 as libc::c_int as xmlChar;
    if !target.is_null() {
        cptr = safe_str.stringval;

        let safe_cptr = unsafe { *cptr } as libc::c_int;
        let safe_cptr_xmlChar = unsafe { *cptr } as *mut xmlChar;
        while safe_cptr != 0 {
            if safe_cptr as libc::c_int >= 'A' as i32 && safe_cptr as libc::c_int <= 'Z' as i32
                || safe_cptr as libc::c_int >= 'a' as i32 && safe_cptr as libc::c_int <= 'z' as i32
                || safe_cptr as libc::c_int >= '0' as i32 && safe_cptr as libc::c_int <= '9' as i32
                || safe_cptr as libc::c_int == '-' as i32
                || safe_cptr as libc::c_int == '_' as i32
                || safe_cptr as libc::c_int == '.' as i32
                || safe_cptr as libc::c_int == '!' as i32
                || safe_cptr as libc::c_int == '~' as i32
                || safe_cptr as libc::c_int == '*' as i32
                || safe_cptr as libc::c_int == '\'' as i32
                || safe_cptr as libc::c_int == '(' as i32
                || safe_cptr as libc::c_int == ')' as i32
                || safe_cptr as libc::c_int == '%' as i32
                    && (unsafe { safe_cptr_xmlChar.offset(1 as libc::c_int as isize) }
                        as libc::c_int
                        >= 'A' as i32
                        && unsafe { safe_cptr_xmlChar.offset(1 as libc::c_int as isize) }
                            as libc::c_int
                            <= 'F' as i32
                        || unsafe { safe_cptr_xmlChar.offset(1 as libc::c_int as isize) }
                            as libc::c_int
                            >= 'a' as i32
                            && unsafe { safe_cptr_xmlChar.offset(1 as libc::c_int as isize) }
                                as libc::c_int
                                <= 'f' as i32
                        || unsafe { safe_cptr_xmlChar.offset(1 as libc::c_int as isize) }
                            as libc::c_int
                            >= '0' as i32
                            && unsafe { safe_cptr_xmlChar.offset(1 as libc::c_int as isize) }
                                as libc::c_int
                                <= '9' as i32)
                    && (unsafe { safe_cptr_xmlChar.offset(2 as libc::c_int as isize) }
                        as libc::c_int
                        >= 'A' as i32
                        && unsafe { safe_cptr_xmlChar.offset(2 as libc::c_int as isize) }
                            as libc::c_int
                            <= 'F' as i32
                        || unsafe { safe_cptr_xmlChar.offset(2 as libc::c_int as isize) }
                            as libc::c_int
                            >= 'a' as i32
                            && unsafe { safe_cptr_xmlChar.offset(2 as libc::c_int as isize) }
                                as libc::c_int
                                <= 'f' as i32
                        || unsafe { safe_cptr_xmlChar.offset(2 as libc::c_int as isize) }
                            as libc::c_int
                            >= '0' as i32
                            && unsafe { safe_cptr_xmlChar.offset(2 as libc::c_int as isize) }
                                as libc::c_int
                                <= '9' as i32)
                || escape_reserved == 0
                    && (safe_cptr as libc::c_int == ';' as i32
                        || safe_cptr as libc::c_int == '/' as i32
                        || safe_cptr as libc::c_int == '?' as i32
                        || safe_cptr as libc::c_int == ':' as i32
                        || safe_cptr as libc::c_int == '@' as i32
                        || safe_cptr as libc::c_int == '&' as i32
                        || safe_cptr as libc::c_int == '=' as i32
                        || safe_cptr as libc::c_int == '+' as i32
                        || safe_cptr as libc::c_int == '$' as i32
                        || safe_cptr as libc::c_int == ',' as i32)
            {
                unsafe { xmlBufAdd(target, cptr, 1 as libc::c_int) };
            } else {
                if (safe_cptr as libc::c_int >> 4 as libc::c_int) < 10 as libc::c_int {
                    escape[1 as libc::c_int as usize] =
                        ('0' as i32 + (safe_cptr as libc::c_int >> 4 as libc::c_int)) as xmlChar
                } else {
                    escape[1 as libc::c_int as usize] = ('A' as i32 - 10 as libc::c_int
                        + (safe_cptr as libc::c_int >> 4 as libc::c_int))
                        as xmlChar
                }
                if (safe_cptr as libc::c_int & 0xf as libc::c_int) < 10 as libc::c_int {
                    escape[2 as libc::c_int as usize] =
                        ('0' as i32 + (safe_cptr as libc::c_int & 0xf as libc::c_int)) as xmlChar
                } else {
                    escape[2 as libc::c_int as usize] = ('A' as i32 - 10 as libc::c_int
                        + (safe_cptr as libc::c_int & 0xf as libc::c_int))
                        as xmlChar
                }
                unsafe {
                    xmlBufAdd(
                        target,
                        &mut *escape.as_mut_ptr().offset(0 as libc::c_int as isize),
                        3 as libc::c_int,
                    )
                };
            }
            unsafe { cptr = cptr.offset(1) }
        }
    }
    unsafe {
        valuePush(
            ctxt,
            xmlXPathCacheNewString(safe_ctxt.context, xmlBufContent(target as *const xmlBuf)),
        )
    };
    unsafe { xmlBufFree(target) };
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, str) };
}

/* *
 * xmlXPathRegisterAllFunctions: * @ctxt: the XPath context
 *
 * Registers all default XPath functions in this context
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisterAllFunctions(mut ctxt: xmlXPathContextPtr) {
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"boolean\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathBooleanFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"ceiling\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathCeilingFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"count\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathCountFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"concat\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathConcatFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"contains\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathContainsFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"id\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathIdFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"false\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathFalseFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"floor\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathFloorFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"last\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathLastFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"lang\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathLangFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"local-name\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathLocalNameFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"not\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathNotFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"name\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathNameFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"namespace-uri\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathNamespaceURIFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"normalize-space\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathNormalizeFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"number\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathNumberFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"position\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathPositionFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"round\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathRoundFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"string\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathStringFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"string-length\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathStringLengthFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"starts-with\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathStartsWithFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"substring\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathSubstringFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"substring-before\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathSubstringBeforeFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"substring-after\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathSubstringAfterFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"sum\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathSumFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"true\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathTrueFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"translate\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            Some(
                xmlXPathTranslateFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFuncNS(
            ctxt,
            b"escape-uri\x00" as *const u8 as *const libc::c_char as *const xmlChar,
            b"http://www.w3.org/2002/08/xquery-functions\x00" as *const u8 as *const libc::c_char
                as *const xmlChar,
            Some(
                xmlXPathEscapeUriFunction
                    as extern "C" fn(_: xmlXPathParserContextPtr, _: libc::c_int) -> (),
            ),
        )
    };
}
/* LIBXML_XPATH_ENABLED */
