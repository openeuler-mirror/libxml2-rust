use super::HTMLparser::*;
use rust_ffi::ffi_defination::defination::*;
use rust_ffi::ffi_extern_method::extern_method::*;
use rust_ffi::ffi_extern_method::extern_method_safe::*;

#[cfg(LIBXML_XPATH_ENABLED)]
fn toupper_xpath(mut __c: i32) -> i32 {
    return if __c >= -(128) && __c < 256 {
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
fn xmlXPathCmpNodesExt(mut node1: xmlNodePtr, mut node2: xmlNodePtr) -> i32 {
    let current_block: u64;
    let mut depth1: i32;
    let mut depth2: i32;
    let mut misc: i32 = 0;
    let mut precedence1: i32 = 0;
    let mut precedence2: i32 = 0;
    let mut miscNode1: xmlNodePtr = 0 as xmlNodePtr;
    let mut miscNode2: xmlNodePtr = 0 as xmlNodePtr;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut l1: ptrdiff_t;
    let mut l2: ptrdiff_t;
    if node1.is_null() || node2.is_null() {
        return -2;
    }
    if node1 == node2 {
        return 0;
    }
    let safe_node1 = unsafe { &mut *node1 };
    let safe_node2 = unsafe { &mut *node2 };
    /*
     * a couple of optimizations which will avoid computations in most cases
     */
    match safe_node1.type_0 as u32 {
        1 => {
            if safe_node2.type_0 as u32 == XML_ELEMENT_NODE as u32 {
                if 0 > safe_node1.content as ptrdiff_t
                    && 0 > safe_node2.content as ptrdiff_t
                    && safe_node1.doc == safe_node2.doc
                {
                    l1 = -(safe_node1.content as ptrdiff_t); /* element is owner */
                    l2 = -(safe_node2.content as ptrdiff_t);
                    if l1 < l2 {
                        return 1;
                    }
                    if l1 > l2 {
                        return -1;
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
            precedence1 = 1;
            miscNode1 = node1;
            node1 = safe_node1.parent;
            misc = 1 as i32;
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
                    if unsafe { (*node1).type_0 as u32 == XML_ELEMENT_NODE as u32 } {
                        precedence1 = 3 as i32; /* element is parent */
                        break;
                    } else {
                        if unsafe { !(*node1).prev.is_null() } {
                            continue;
                        }
                        precedence1 = 2 as i32;
                        /*
                         * URGENT TODO: Are there any cases, where the
                         * parent of such a node is not an element node?
                         */
                        node1 = unsafe { (*node1).parent }; /* element is parent */
                        break;
                    }
                }
            } else {
                precedence1 = 2 as i32;
                node1 = safe_node1.parent
            }
            if node1.is_null()
                || unsafe { (*node1).type_0 as u32 != XML_ELEMENT_NODE as u32 }
                || unsafe { 0 as i32 as i64 <= (*node1).content as ptrdiff_t }
            {
                /*
                 * Fallback for whatever case.
                 */
                node1 = miscNode1;
                precedence1 = 0 as i32
            } else {
                misc = 1 as i32
            }
            current_block = 721385680381463314;
        }
        18 => {
            /*
             * TODO: why do we return 1 for namespace nodes?
             */
            return 1 as i32;
        }
        _ => {
            current_block = 721385680381463314; /* element is owner */
        }
    } /* element in prev-sibl axis */
    match current_block {
        721385680381463314 => {
            match unsafe { (*node2).type_0 as u32 } {
                2 => {
                    precedence2 = 1 as i32; /* element is parent */
                    miscNode2 = node2; /* element is parent */
                    node2 = unsafe { (*node2).parent };
                    misc = 1 as i32
                }
                3 | 4 | 8 | 7 => {
                    miscNode2 = node2;
                    if unsafe { !(*node2).prev.is_null() } {
                        loop {
                            node2 = unsafe { (*node2).prev };
                            if unsafe { (*node2).type_0 as u32 == XML_ELEMENT_NODE as u32 } {
                                precedence2 = 3 as i32;
                                break;
                            } else {
                                if unsafe { !(*node2).prev.is_null() } {
                                    continue;
                                }
                                precedence2 = 2 as i32;
                                node2 = unsafe { (*node2).parent };
                                break;
                            }
                        }
                    } else {
                        precedence2 = 2 as i32;
                        node2 = unsafe { (*node2).parent }
                    }
                    if node2.is_null()
                        || unsafe { (*node2).type_0 as u32 != XML_ELEMENT_NODE as i32 as u32 }
                        || unsafe { 0 as i32 as i64 <= (*node2).content as ptrdiff_t }
                    {
                        node2 = miscNode2;
                        precedence2 = 0 as i32
                    } else {
                        misc = 1 as i32
                    }
                }
                18 => return 1 as i32,
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
                                return 1 as i32;
                            }
                            if unsafe { (*cur).type_0 as u32 == XML_ELEMENT_NODE as i32 as u32 } {
                                return -(1 as i32);
                            }
                            cur = unsafe { (*cur).prev }
                        }
                        return -(1 as i32);
                    } else if precedence1 < precedence2 {
                        return 1 as i32;
                    } else {
                        return -(1 as i32);
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
                if precedence2 == 3 as i32 && precedence1 > 1 as i32 {
                    cur = unsafe { (*node1).parent };
                    while !cur.is_null() {
                        if cur == node2 {
                            return 1 as i32;
                        }
                        cur = unsafe { (*cur).parent }
                    }
                }
                if precedence1 == 3 as i32 && precedence2 > 1 as i32 {
                    cur = unsafe { (*node2).parent };
                    while !cur.is_null() {
                        if cur == node1 {
                            return -(1 as i32);
                        }
                        cur = unsafe { (*cur).parent }
                    }
                }
            }
            /*
             * Speedup using document order if available.
             */
            if unsafe {
                (*node1).type_0 as u32 == XML_ELEMENT_NODE as u32
                    && (*node2).type_0 as u32 == XML_ELEMENT_NODE as u32
                    && 0 as i64 > (*node1).content as ptrdiff_t
                    && 0 as i64 > (*node2).content as ptrdiff_t
                    && (*node1).doc == (*node2).doc
            } {
                unsafe {
                    l1 = -((*node1).content as ptrdiff_t);
                    l2 = -((*node2).content as ptrdiff_t);
                };
                if l1 < l2 {
                    return 1 as i32;
                }
                if l1 > l2 {
                    return -(1 as i32);
                }
            }
        }
        _ => {}
    }

    // turtle_comparison: if node1 == unsafe{(*node2).prev} { return 1 as i32 }
    if node1 == unsafe { (*node2).next } {
        return -1;
    }
    /*
     * compute depth to root
     */
    depth2 = 0;
    cur = node2;
    while unsafe { !(*cur).parent.is_null() } {
        if unsafe { (*cur).parent == node1 } {
            return 1;
        }
        depth2 += 1;
        cur = unsafe { (*cur).parent }
    }
    root = cur;
    depth1 = 0;
    cur = node1;
    while unsafe { !(*cur).parent.is_null() } {
        if unsafe { (*cur).parent == node2 } {
            return -1;
        }
        depth1 += 1;
        cur = unsafe { (*cur).parent }
    }
    /*
     * Distinct document (or distinct entities :-( ) case.
     */
    if root != cur {
        return -2;
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
            return -2;
        }
    }
    /*
     * Find who's first.
     */
    if node1 == unsafe { (*node2).prev } {
        return 1;
    }
    if node1 == unsafe { (*node2).next } {
        return -1;
    }
    /*
     * Speedup using document order if available.
     */
    if unsafe {
        (*node1).type_0 as u32 == XML_ELEMENT_NODE as u32
            && (*node2).type_0 as u32 == XML_ELEMENT_NODE as u32
            && 0 as i64 > (*node1).content as ptrdiff_t
            && 0 as i64 > (*node2).content as ptrdiff_t
            && (*node1).doc == (*node2).doc
    } {
        unsafe {
            l1 = -((*node1).content as ptrdiff_t);
            l2 = -((*node2).content as ptrdiff_t);
        };
        if l1 < l2 {
            return 1;
        }
        if l1 > l2 {
            return -1;
        }
    }
    cur = unsafe { (*node1).next };
    while !cur.is_null() {
        if cur == node2 {
            return 1;
        }
        cur = unsafe { (*cur).next }
    }
    return -1;
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
fn wrap_cmp(x: xmlNodePtr, y: xmlNodePtr) -> i32 {
    let mut res: i32 = xmlXPathCmpNodesExt(x, y);
    return if res == -2 { res } else { -res };
}
#[cfg(not(XP_OPTIMIZED_NON_ELEM_COMPARISON))]
fn wrap_cmp(mut x: xmlNodePtr, mut y: xmlNodePtr) -> i32 {
    let mut res: i32 = unsafe { xmlXPathCmpNodes(x, y) };
    return if res == -2 { res } else { -res };
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
    dst: *mut xmlNodePtr,
    stack: *mut TIM_SORT_RUN_T,
    mut stack_curr: i32,
    store: *mut TEMP_STORAGE_T,
    size: size_t,
) -> i32 {
    loop {
        let mut A: size_t = 0;
        let mut B: size_t = 0;
        let mut C: size_t = 0;
        let mut D: size_t = 0;
        let mut ABC: i32 = 0;
        let mut BCD: i32 = 0;
        let mut CD: i32 = 0;
        let safe_stack = unsafe { &mut *stack };
        /* if the stack only has one thing on it, we are done with the collapse */
        if stack_curr <= 1 {
            break;
        }
        /* if this is the last merge, just do it */
        if stack_curr == 2
            && unsafe {
                (*stack.offset(0))
                    .length
                    .wrapping_add((*stack.offset(1)).length)
                    == size
            }
            || stack_curr == 2 && unsafe { (*stack.offset(0)).length <= (*stack.offset(1)).length }
        {
            libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store);
            let ref mut fresh0 = unsafe { (*stack.offset(0)).length };
            *fresh0 = (*fresh0 as u64).wrapping_add(unsafe { (*stack.offset(1)).length }) as size_t
                as size_t;
            stack_curr -= 1;
            break;
        } else {
            if stack_curr == 2 {
                break;
            }
            B = unsafe { (*stack.offset((stack_curr - 3) as isize)).length };
            C = unsafe { (*stack.offset((stack_curr - 2) as isize)).length };
            D = unsafe { (*stack.offset((stack_curr - 1) as isize)).length };
            if stack_curr >= 4 {
                A = unsafe { (*stack.offset((stack_curr - 4) as isize)).length };
                ABC = (A <= B.wrapping_add(C)) as i32
            } else {
                ABC = 0
            }
            BCD = (B <= C.wrapping_add(D) || ABC != 0) as i32;
            CD = (C <= D) as i32;
            /* check if the invariant is off for a stack of 2 elements */
            /* Both invariants are good */
            if BCD == 0 && CD == 0 {
                break;
            }
            /* left merge */
            if BCD != 0 && CD == 0 {
                libxml_domnode_tim_sort_merge(dst, stack, stack_curr - 1, store);
                let ref mut fresh2 = unsafe { (*stack.offset((stack_curr - 3) as isize)).length };
                *fresh2 = (*fresh2 as u64)
                    .wrapping_add(unsafe { (*stack.offset((stack_curr - 2) as isize)).length })
                    as size_t as size_t;
                unsafe {
                    *stack.offset((stack_curr - 2) as isize) =
                        *stack.offset((stack_curr - 1) as isize)
                };
                stack_curr -= 1
            } else {
                /* right merge */
                libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store);
                let ref mut fresh3 = unsafe { (*stack.offset((stack_curr - 2) as isize)).length };
                *fresh3 = (*fresh3 as u64)
                    .wrapping_add(unsafe { (*stack.offset((stack_curr - 1) as isize)).length })
                    as size_t as size_t;
                stack_curr -= 1
            }
        }
    }
    return stack_curr;
}

#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_tim_sort_merge(
    dst: *mut xmlNodePtr,
    stack: *const TIM_SORT_RUN_T,
    stack_curr: i32,
    store: *mut TEMP_STORAGE_T,
) {
    let A: size_t = unsafe { (*stack.offset((stack_curr - 2) as isize)).length };
    let B: size_t = unsafe { (*stack.offset((stack_curr - 1) as isize)).length };
    let curr: size_t = unsafe { (*stack.offset((stack_curr - 2) as isize)).start };
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
                storage as *mut (),
                &mut *dst.offset(curr as isize) as *mut xmlNodePtr as *const (),
                A.wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            )
        };
        i = 0 as size_t;
        j = curr.wrapping_add(A);
        k = curr;
        while k < curr.wrapping_add(A).wrapping_add(B) {
            if i < A && j < curr.wrapping_add(A).wrapping_add(B) {
                if unsafe { wrap_cmp(*storage.offset(i as isize), *dst.offset(j as isize)) <= 0 } {
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
                storage as *mut (),
                &mut *dst.offset(curr.wrapping_add(A) as isize) as *mut xmlNodePtr as *const (),
                B.wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            )
        };
        i = B;
        j = curr.wrapping_add(A);
        k = curr.wrapping_add(A).wrapping_add(B);
        while k > curr {
            k = k.wrapping_sub(1);
            if i > 0 && j > curr {
                if unsafe {
                    wrap_cmp(
                        *dst.offset(j.wrapping_sub(1) as isize),
                        *storage.offset(i.wrapping_sub(1) as isize),
                    ) > 0
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
                if !(i > 0) {
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
fn libxml_domnode_tim_sort_resize(store: *mut TEMP_STORAGE_T, new_size: size_t) {
    let safe_store = unsafe { &mut *store };
    if safe_store.alloc < new_size {
        let mut tempstore: *mut xmlNodePtr = unsafe {
            realloc(
                safe_store.storage as *mut (),
                new_size.wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            ) as *mut xmlNodePtr
        };
        if tempstore.is_null() {
            unsafe {
                fprintf(
                    stderr,
                    b"Error allocating temporary storage for tim sort: need %lu bytes\x00"
                        as *const u8 as *const i8,
                    (::std::mem::size_of::<xmlNodePtr>() as u64).wrapping_mul(new_size),
                );
                exit(1);
            };
        }
        safe_store.storage = tempstore;
        safe_store.alloc = new_size
    };
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_count_run(mut dst: *mut xmlNodePtr, start: size_t, size: size_t) -> size_t {
    let mut curr: size_t = 0;
    if size.wrapping_sub(start) == 1 {
        return 1 as size_t;
    }
    if start >= size.wrapping_sub(2) {
        if unsafe {
            wrap_cmp(
                *dst.offset(size.wrapping_sub(2) as isize),
                *dst.offset(size.wrapping_sub(1) as isize),
            ) > 0 as i32
        } {
            let mut __SORT_SWAP_t: xmlNodePtr =
                unsafe { *dst.offset(size.wrapping_sub(2) as isize) };
            let ref mut fresh13 = unsafe { *dst.offset(size.wrapping_sub(2) as isize) };
            *fresh13 = unsafe { *dst.offset(size.wrapping_sub(1) as isize) };
            let ref mut fresh14 = unsafe { *dst.offset(size.wrapping_sub(1) as isize) };
            *fresh14 = __SORT_SWAP_t
        }
        return 2;
    }
    curr = start.wrapping_add(2);
    if unsafe {
        wrap_cmp(
            *dst.offset(start as isize),
            *dst.offset(start.wrapping_add(1) as isize),
        ) <= 0 as i32
    } {
        while !(curr == size.wrapping_sub(1)) {
            if unsafe {
                wrap_cmp(
                    *dst.offset(curr.wrapping_sub(1) as isize),
                    *dst.offset(curr as isize),
                ) > 0
            } {
                break;
            }
            curr = curr.wrapping_add(1)
        }
        return curr.wrapping_sub(start);
    } else {
        while !(curr == size.wrapping_sub(1)) {
            if unsafe {
                wrap_cmp(
                    *dst.offset(curr.wrapping_sub(1) as isize),
                    *dst.offset(curr as isize),
                ) <= 0
            } {
                break;
            }
            curr = curr.wrapping_add(1)
        }
        libxml_domnode_reverse_elements(dst, start, curr.wrapping_sub(1));
        return curr.wrapping_sub(start);
    };
}
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn libxml_domnode_binary_insertion_sort(dst: *mut xmlNodePtr, size: size_t) {
    if size <= 1 {
        return;
    }
    libxml_domnode_binary_insertion_sort_start(dst, 1 as size_t, size);
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn compute_minrun(size: uint64_t) -> i32 {
    let top_bit: i32 = 64 - (size as u64).leading_zeros() as i32;
    let shift: i32 = (if top_bit > 6 { top_bit } else { 6 }) - 6;
    let minrun: i32 = (size >> shift) as i32;
    let mask: uint64_t = ((1 as u64) << shift).wrapping_sub(1 as u64) as uint64_t;
    if mask & size != 0 {
        return minrun + 1;
    }
    return minrun;
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_check_invariant(stack: *mut TIM_SORT_RUN_T, stack_curr: i32) -> i32 {
    let mut A: size_t = 0;
    let mut B: size_t = 0;
    let mut C: size_t = 0;
    if stack_curr < 2 {
        return 1;
    }
    if stack_curr == 2 {
        let A1: size_t = unsafe { (*stack.offset((stack_curr - 2) as isize)).length };
        let B1: size_t = unsafe { (*stack.offset((stack_curr - 1) as isize)).length };
        if A1 <= B1 {
            return 0;
        }
        return 1;
    }
    A = unsafe { (*stack.offset((stack_curr - 3) as isize)).length };
    B = unsafe { (*stack.offset((stack_curr - 2) as isize)).length };
    C = unsafe { (*stack.offset((stack_curr - 1) as isize)).length };
    if A <= B.wrapping_add(C) || B <= C {
        return 0;
    }
    return 1;
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_binary_insertion_find(
    dst: *mut xmlNodePtr,
    x: xmlNodePtr,
    size: size_t,
) -> size_t {
    let mut l: size_t = 0;
    let mut c: size_t = 0;
    let mut r: size_t = 0;
    let mut cx: xmlNodePtr = 0 as *mut xmlNode;
    l = 0;
    r = size.wrapping_sub(1);
    c = r >> 1;
    if unsafe { wrap_cmp(x, *dst.offset(0 as isize)) < 0 } {
        return 0 as size_t;
    } else {
        if unsafe { wrap_cmp(x, *dst.offset(r as isize)) > 0 } {
            return r;
        }
    }
    cx = unsafe { *dst.offset(c as isize) };
    loop {
        let val: i32 = wrap_cmp(x, cx);
        if val < 0 {
            if c.wrapping_sub(l) <= 1 {
                return c;
            }
            r = c
        } else {
            if r.wrapping_sub(c) <= 1 {
                return c.wrapping_add(1);
            }
            l = c
        }
        c = l.wrapping_add(r.wrapping_sub(l) >> 1);
        cx = unsafe { *dst.offset(c as isize) }
    }
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn libxml_domnode_binary_insertion_sort_start(dst: *mut xmlNodePtr, start: size_t, size: size_t) {
    let mut i: size_t = 0;
    i = start;
    while i < size {
        let mut j: size_t = 0;
        let mut x: xmlNodePtr = 0 as *mut xmlNode;
        let mut location: size_t = 0;
        if !(unsafe {
            wrap_cmp(
                *dst.offset(i.wrapping_sub(1) as isize),
                *dst.offset(i as isize),
            ) <= 0
        }) {
            unsafe {
                x = *dst.offset(i as isize);
                location = libxml_domnode_binary_insertion_find(dst, x, i);
            };
            j = i.wrapping_sub(1 as u64);
            while j >= location {
                unsafe {
                    let ref mut fresh15 = *dst.offset(j.wrapping_add(1) as isize);
                    *fresh15 = *dst.offset(j as isize);
                };
                if j == 0 {
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
fn libxml_domnode_reverse_elements(dst: *mut xmlNodePtr, mut start: size_t, mut end: size_t) {
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
    dst: *mut xmlNodePtr,
    size: size_t,
    store: *mut TEMP_STORAGE_T,
    minrun: size_t,
    run_stack: *mut TIM_SORT_RUN_T,
    stack_curr: *mut size_t,
    curr: *mut size_t,
) -> i32 {
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
        *curr = (*curr as u64).wrapping_add(len) as size_t;
    };
    if unsafe { *curr == size } {
        /* finish up */
        while unsafe { *stack_curr > 1 as u64 } {
            unsafe { libxml_domnode_tim_sort_merge(dst, run_stack, *stack_curr as i32, store) };
            unsafe {
                let ref mut fresh19 =
                    (*run_stack.offset((*stack_curr).wrapping_sub(2) as isize)).length;
                *fresh19 = (*fresh19 as u64).wrapping_add(
                    (*run_stack.offset((*stack_curr).wrapping_sub(1) as isize)).length,
                ) as size_t as size_t;
                *stack_curr = (*stack_curr).wrapping_sub(1)
            }
        }
        if !safe_store.storage.is_null() {
            unsafe { free(safe_store.storage as *mut ()) };
            safe_store.storage = 0 as *mut xmlNodePtr
        }
        return 0;
    }
    return 1;
}
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn libxml_domnode_tim_sort(dst: *mut xmlNodePtr, size: size_t) {
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
    let mut stack_curr: size_t = 0;
    let mut curr: size_t = 0;
    /* don't bother sorting an array of size 1 */
    if size <= 1 {
        return;
    }
    if size < 64 {
        libxml_domnode_binary_insertion_sort(dst, size);
        return;
    }
    /* compute the minimum run length */
    minrun = compute_minrun(size) as size_t;
    /* temporary storage for merges */
    store = &mut _store;
    let safe_store = unsafe { &mut *store };
    safe_store.alloc = 0 as size_t;
    safe_store.storage = 0 as *mut xmlNodePtr;
    if PUSH_NEXT(
        dst,
        size,
        store,
        minrun,
        run_stack.as_mut_ptr(),
        &mut stack_curr,
        &mut curr,
    ) == 0
    {
        return;
    }
    if PUSH_NEXT(
        dst,
        size,
        store,
        minrun,
        run_stack.as_mut_ptr(),
        &mut stack_curr,
        &mut curr,
    ) == 0
    {
        return;
    }
    if PUSH_NEXT(
        dst,
        size,
        store,
        minrun,
        run_stack.as_mut_ptr(),
        &mut stack_curr,
        &mut curr,
    ) == 0
    {
        return;
    }
    loop {
        if libxml_domnode_check_invariant(run_stack.as_mut_ptr(), stack_curr as i32) == 0 {
            stack_curr = libxml_domnode_tim_sort_collapse(
                dst,
                run_stack.as_mut_ptr(),
                stack_curr as i32,
                store,
                size,
            ) as size_t
        } else if PUSH_NEXT(
            dst,
            size,
            store,
            minrun,
            run_stack.as_mut_ptr(),
            &mut stack_curr,
            &mut curr,
        ) == 0
        {
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
    let zero: libc::c_double = 0.0f64;
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
pub fn xmlXPathIsNaN(val: libc::c_double) -> i32 {
    match () {
        #[cfg(ISNAN)]
        _ => {
            return if ::std::mem::size_of::<libc::c_double>() as u64
                == ::std::mem::size_of::<libc::c_float>() as u64
            {
                unsafe { __isnanf(val as libc::c_float) }
            } else if ::std::mem::size_of::<libc::c_double>() as u64
                == ::std::mem::size_of::<libc::c_double>() as u64
            {
                unsafe { __isnan(val) }
            } else {
                unsafe { __isnanl(val as libc::c_float) }
            };
        }
        #[cfg(not(ISNAN))]
        _ => return if val == val { 0 } else { 1 },
    };
}
/* *
 * xmlXPathIsInf: * @val: a double value
 *
 * Returns 1 if the value is +Infinite, -1 if -Infinite, 0 otherwise
 */
#[cfg(LIBXML_XPATH_ENABLED_OR_LIBXML_SCHEMAS_ENABLED)]
pub fn xmlXPathIsInf(val: libc::c_double) -> i32 {
    match () {
        #[cfg(ISINF)]
        _ => {
            return if if ::std::mem::size_of::<libc::c_double>() as u64
                == ::std::mem::size_of::<libc::c_float>() as u64
            {
                unsafe { __isinff(val as libc::c_float) }
            } else if ::std::mem::size_of::<libc::c_double>() as u64
                == ::std::mem::size_of::<libc::c_double>() as u64
            {
                unsafe { __isinf(val) }
            } else {
                unsafe { __isinfl(val as libc::c_float) }
            } != 0
            {
                if val > 0 as i32 as libc::c_double {
                    1
                } else {
                    -1
                }
            } else {
                0
            };
        }
        #[cfg(not(ISINF))]
        _ => {
            return if val >= xmlXPathPINF {
                1
            } else if val <= -xmlXPathPINF {
                -1
            } else {
                0
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
        href: b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as *const i8
            as *const xmlChar,
        prefix: b"xml\x00" as *const u8 as *const i8 as *mut xmlChar,
        _private: 0 as *const () as *mut (),
        context: 0 as *const _xmlDoc as *mut _xmlDoc,
    };
    init
};
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathXMLNamespace: xmlNsPtr =
    unsafe { &xmlXPathXMLNamespaceStruct as *const xmlNs as *mut xmlNs };

#[cfg(LIBXML_THREAD_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
static xmlXPathDisableOptimizer: i32 = 0;

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
static mut xmlXPathErrorMessages: [*const i8; 28] = [
    b"Ok\n\x00" as *const u8 as *const i8,
    b"Number encoding\n\x00" as *const u8 as *const i8,
    b"Unfinished literal\n\x00" as *const u8 as *const i8,
    b"Start of literal\n\x00" as *const u8 as *const i8,
    b"Expected $ for variable reference\n\x00" as *const u8 as *const i8,
    b"Undefined variable\n\x00" as *const u8 as *const i8,
    b"Invalid predicate\n\x00" as *const u8 as *const i8,
    b"Invalid expression\n\x00" as *const u8 as *const i8,
    b"Missing closing curly brace\n\x00" as *const u8 as *const i8,
    b"Unregistered function\n\x00" as *const u8 as *const i8,
    b"Invalid operand\n\x00" as *const u8 as *const i8,
    b"Invalid type\n\x00" as *const u8 as *const i8,
    b"Invalid number of arguments\n\x00" as *const u8 as *const i8,
    b"Invalid context size\n\x00" as *const u8 as *const i8,
    b"Invalid context position\n\x00" as *const u8 as *const i8,
    b"Memory allocation error\n\x00" as *const u8 as *const i8,
    b"Syntax error\n\x00" as *const u8 as *const i8,
    b"Resource error\n\x00" as *const u8 as *const i8,
    b"Sub resource error\n\x00" as *const u8 as *const i8,
    b"Undefined namespace prefix\n\x00" as *const u8 as *const i8,
    b"Encoding error\n\x00" as *const u8 as *const i8,
    b"Char out of XML range\n\x00" as *const u8 as *const i8,
    b"Invalid or incomplete context\n\x00" as *const u8 as *const i8,
    b"Stack usage error\n\x00" as *const u8 as *const i8,
    b"Forbidden variable\n\x00" as *const u8 as *const i8,
    b"Operation limit exceeded\n\x00" as *const u8 as *const i8,
    b"Recursion limit exceeded\n\x00" as *const u8 as *const i8,
    b"?? Unknown error ??\n\x00" as *const u8 as *const i8,
];
/* *
 * xmlXPathErrMemory: * @ctxt: an XPath context
 * @extra: extra information
 *
 * Handle a redefinition of attribute error
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathErrMemory(ctxt: xmlXPathContextPtr, extra: *const i8) {
    if !ctxt.is_null() {
        let safe_ctxt = unsafe { &mut *ctxt };
        unsafe { xmlResetError(&mut (*ctxt).lastError) };
        if !extra.is_null() {
            let mut buf: [xmlChar; 200] = [0; 200];
            unsafe {
                xmlStrPrintf(
                    buf.as_mut_ptr(),
                    200,
                    b"Memory allocation failed : %s\n\x00" as *const u8 as *const i8,
                    extra,
                )
            };
            safe_ctxt.lastError.message = unsafe { xmlStrdup(buf.as_mut_ptr()) as *mut i8 }
        } else {
            safe_ctxt.lastError.message = unsafe {
                xmlStrdup(
                    b"Memory allocation failed\n\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) as *mut i8
            }
        }
        safe_ctxt.lastError.domain = XML_FROM_XPATH as i32;
        safe_ctxt.lastError.code = XML_ERR_NO_MEMORY as i32;
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
                0 as *mut (),
                0 as *mut (),
                0 as *mut (),
                XML_FROM_XPATH as i32,
                XML_ERR_NO_MEMORY as i32,
                XML_ERR_FATAL,
                0 as *const i8,
                0,
                extra,
                0 as *const i8,
                0 as *const i8,
                0,
                0,
                b"Memory allocation failed : %s\n\x00" as *const u8 as *const i8,
                extra,
            )
        };
    } else {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut (),
                0 as *mut (),
                0 as *mut (),
                XML_FROM_XPATH as i32,
                XML_ERR_NO_MEMORY as i32,
                XML_ERR_FATAL,
                0 as *const i8,
                0,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0,
                0,
                b"Memory allocation failed\n\x00" as *const u8 as *const i8,
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
fn xmlXPathPErrMemory(ctxt: xmlXPathParserContextPtr, extra: *const i8) {
    if ctxt.is_null() {
        xmlXPathErrMemory(0 as xmlXPathContextPtr, extra);
    } else {
        let safe_ctxt = unsafe { &mut *ctxt };
        safe_ctxt.error = XPATH_MEMORY_ERROR as i32;
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
pub fn xmlXPathErr(mut ctxt: xmlXPathParserContextPtr, mut error: i32) {
    let safe_ctxt = unsafe { &mut *ctxt };
    if error < 0
        || error
            > (::std::mem::size_of::<[*const i8; 28]>() as u64)
                .wrapping_div(::std::mem::size_of::<*const i8>() as u64) as i32
                - 1
    {
        error = (::std::mem::size_of::<[*const i8; 28]>() as u64)
            .wrapping_div(::std::mem::size_of::<*const i8>() as u64) as i32
            - 1
    }
    if ctxt.is_null() {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut (),
                0 as *mut (),
                0 as *mut (),
                XML_FROM_XPATH as i32,
                error + XML_XPATH_EXPRESSION_OK as i32 - XPATH_EXPRESSION_OK as i32,
                XML_ERR_ERROR,
                0 as *const i8,
                0 as i32,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as i32,
                0 as i32,
                b"%s\x00" as *const u8 as *const i8,
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
                0 as *mut (),
                0 as *mut (),
                0 as *mut (),
                XML_FROM_XPATH as i32,
                error + XML_XPATH_EXPRESSION_OK as i32 - XPATH_EXPRESSION_OK as i32,
                XML_ERR_ERROR,
                0 as *const i8,
                0 as i32,
                (*ctxt).base as *const i8,
                0 as *const i8,
                0 as *const i8,
                (*ctxt).cur.offset_from((*ctxt).base) as i64 as i32,
                0 as i32,
                b"%s\x00" as *const u8 as *const i8,
                xmlXPathErrorMessages[error as usize],
            )
        };
        return;
    }
    /* cleanup current last error */
    unsafe {
        xmlResetError(&mut (*(*ctxt).context).lastError);
        (*safe_ctxt.context).lastError.domain = XML_FROM_XPATH as i32;
        (*safe_ctxt.context).lastError.code =
            error + XML_XPATH_EXPRESSION_OK as i32 - XPATH_EXPRESSION_OK as i32;
        (*safe_ctxt.context).lastError.level = XML_ERR_ERROR;
        (*safe_ctxt.context).lastError.str1 = xmlStrdup(safe_ctxt.base) as *mut i8;
        (*safe_ctxt.context).lastError.int1 =
            safe_ctxt.cur.offset_from(safe_ctxt.base) as i64 as i32;
        (*safe_ctxt.context).lastError.node = (*safe_ctxt.context).debugNode as *mut ();
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
                0 as *mut (),
                0 as *mut (),
                (*(*ctxt).context).debugNode as *mut (),
                XML_FROM_XPATH as i32,
                error + XML_XPATH_EXPRESSION_OK as i32 - XPATH_EXPRESSION_OK as i32,
                XML_ERR_ERROR,
                0 as *const i8,
                0 as i32,
                (*ctxt).base as *const i8,
                0 as *const i8,
                0 as *const i8,
                (*ctxt).cur.offset_from((*ctxt).base) as i64 as i32,
                0 as i32,
                b"%s\x00" as *const u8 as *const i8,
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
    mut file: *const i8,
    mut line: i32,
    mut no: i32,
) {
    xmlXPathErr(ctxt, no);
}
/* *
 * xmlXPathCheckOpLimit: * @ctxt: the XPath Parser context
 * @opCount: the number of operations to be added
 *
 * Adds opCount to the running total of operations and returns -1 if the
 * operation limit is exceeded. Returns 0 otherwise.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCheckOpLimit(ctxt: xmlXPathParserContextPtr, opCount: u64) -> i32 {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut xpctxt: xmlXPathContextPtr = safe_ctxt.context;
    let safe_xpctxt = unsafe { &mut *xpctxt };
    if opCount > safe_xpctxt.opLimit
        || safe_xpctxt.opCount > safe_xpctxt.opLimit.wrapping_sub(opCount)
    {
        safe_xpctxt.opCount = safe_xpctxt.opLimit;
        xmlXPathErr(ctxt, XPATH_OP_LIMIT_EXCEEDED as i32);
        return -1;
    }
    safe_xpctxt.opCount = safe_xpctxt.opCount.wrapping_add(opCount);
    return 0;
}
/*
* TODO: Since such a list-handling is used in xmlschemas.c and libxslt
* and here, we should make the functions public.
*/
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlPointerListAddSize(list: xmlPointerListPtr, item: *mut (), mut initialSize: i32) -> i32 {
    let safe_list = unsafe { &mut *list };

    if safe_list.items.is_null() {
        if initialSize <= 0 {
            initialSize = 1
        }
        safe_list.items = unsafe {
            xmlMalloc.expect("non-null function pointer")(
                (initialSize as u64).wrapping_mul(::std::mem::size_of::<*mut ()>() as u64),
            ) as *mut *mut ()
        };
        if safe_list.items.is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"xmlPointerListCreate: allocating item\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        safe_list.number = 0;
        safe_list.size = initialSize
    } else if safe_list.size <= safe_list.number {
        if safe_list.size > 50000000 {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"xmlPointerListAddSize: re-allocating item\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        safe_list.size *= 2 as i32;
        safe_list.items = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                safe_list.items as *mut (),
                (safe_list.size as u64).wrapping_mul(::std::mem::size_of::<*mut ()>() as u64),
            ) as *mut *mut ()
        };
        if safe_list.items.is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"xmlPointerListAddSize: re-allocating item\n\x00" as *const u8 as *const i8,
            );
            safe_list.size = 0;
            return -1;
        }
    }
    let fresh20 = safe_list.number;
    safe_list.number = safe_list.number + 1;
    unsafe {
        let ref mut fresh21 = *safe_list.items.offset(fresh20 as isize);
        *fresh21 = item;
    };
    return 0;
}
/* *
 * xsltPointerListCreate: *
 * Creates an xsltPointerList structure.
 *
 * Returns a xsltPointerList structure or NULL in case of an error.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlPointerListCreate(initialSize: i32) -> xmlPointerListPtr {
    let mut ret: xmlPointerListPtr = 0 as *mut xmlPointerList;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlPointerList>() as u64)
            as xmlPointerListPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"xmlPointerListCreate: allocating item\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlPointerListPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlPointerList>() as u64,
        )
    };
    if initialSize > 0 {
        xmlPointerListAddSize(ret, 0 as *mut (), initialSize);
        unsafe { (*ret).number = 0 }
    }
    return ret;
}
/* *
 * xsltPointerListFree: *
 * Frees the xsltPointerList structure. This does not free
 * the content of the list.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlPointerListFree(list: xmlPointerListPtr) {
    if list.is_null() {
        return;
    }
    let safe_list = unsafe { &mut *list };
    if !safe_list.items.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")((*list).items as *mut ()) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(list as *mut ()) };
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
            ::std::mem::size_of::<xmlXPathCompExpr>() as u64
        ) as xmlXPathCompExprPtr
    };
    let safe_cur = unsafe { &mut *cur };
    if cur.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"allocating component\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathCompExprPtr;
    }
    unsafe {
        memset(
            cur as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathCompExpr>() as u64,
        )
    };
    safe_cur.maxStep = 10;
    safe_cur.nbStep = 0;
    safe_cur.steps = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            (safe_cur.maxStep as u64).wrapping_mul(::std::mem::size_of::<xmlXPathStepOp>() as u64),
        ) as *mut xmlXPathStepOp
    };
    if safe_cur.steps.is_null() {
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"allocating steps\n\x00" as *const u8 as *const i8,
            );
            xmlFree.expect("non-null function pointer")(cur as *mut ());
        }
        return 0 as xmlXPathCompExprPtr;
    }
    unsafe {
        memset(
            safe_cur.steps as *mut (),
            0,
            (safe_cur.maxStep as u64).wrapping_mul(::std::mem::size_of::<xmlXPathStepOp>() as u64),
        )
    };
    safe_cur.last = -1;
    match () {
        #[cfg(DEBUG_EVAL_COUNTS)]
        _ => {
            safe_cur.nb = 0;
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
pub fn xmlXPathFreeCompExpr(comp: xmlXPathCompExprPtr) {
    let mut op: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
    let mut i: i32;
    if comp.is_null() {
        return;
    }
    let safe_comp = unsafe { &mut *comp };
    if safe_comp.dict.is_null() {
        i = 0;
        while i < safe_comp.nbStep {
            op = unsafe { &mut *(*comp).steps.offset(i as isize) as *mut xmlXPathStepOp };
            let safe_op = unsafe { &mut *op };
            if !safe_op.value4.is_null() {
                if safe_op.op as u32 == XPATH_OP_VALUE as u32 {
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
        i = 0;
        while i < safe_comp.nbStep {
            op = unsafe { &mut *(*comp).steps.offset(i as isize) as *mut xmlXPathStepOp };
            let safe_op = unsafe { &mut *op };
            if !safe_op.value4.is_null() {
                if safe_op.op as u32 == XPATH_OP_VALUE as u32 {
                    unsafe { xmlXPathFreeObject((*op).value4 as xmlXPathObjectPtr) };
                }
            }
            i += 1
        }
        unsafe { xmlDictFree((*comp).dict) };
    }
    if !safe_comp.steps.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")((*comp).steps as *mut ()) };
    }
    match () {
        #[cfg(DEBUG_EVAL_COUNTS)]
        _ => {
            if !safe_comp.string.is_null() {
                unsafe { xmlFree.expect("non-null function pointer")((*comp).string as *mut ()) };
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
        unsafe { xmlFree.expect("non-null function pointer")((*comp).expr as *mut ()) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(comp as *mut ()) };
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
    ctxt: xmlXPathParserContextPtr,
    ch1: i32,
    ch2: i32,
    op: xmlXPathOp,
    value: i32,
    value2: i32,
    value3: i32,
    value4: *mut (),
    value5: *mut (),
) -> i32 {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut comp: xmlXPathCompExprPtr = safe_ctxt.comp;
    let safe_comp = unsafe { &mut *comp };
    if safe_comp.nbStep >= safe_comp.maxStep {
        let mut real: *mut xmlXPathStepOp = 0 as *mut xmlXPathStepOp;
        if safe_comp.maxStep >= 1000000 {
            xmlXPathPErrMemory(ctxt, b"adding step\n\x00" as *const u8 as *const i8);
            return -1;
        }
        safe_comp.maxStep *= 2 as i32;
        real = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*comp).steps as *mut (),
                ((*comp).maxStep as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlXPathStepOp>() as u64),
            ) as *mut xmlXPathStepOp
        };
        if real.is_null() {
            safe_comp.maxStep /= 2;
            xmlXPathPErrMemory(ctxt, b"adding step\n\x00" as *const u8 as *const i8);
            return -1;
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
        && (op as u32 == XPATH_OP_FUNCTION as u32
            || op as u32 == XPATH_OP_VARIABLE as u32
            || op as u32 == XPATH_OP_COLLECT as u32)
    {
        if !value4.is_null() {
            unsafe {
                let ref mut fresh22 = (*(*comp).steps.offset((*comp).nbStep as isize)).value4;
                *fresh22 = xmlDictLookup((*comp).dict, value4 as *const xmlChar, -1) as *mut ()
                    as *mut xmlChar as *mut ();
                xmlFree.expect("non-null function pointer")(value4);
            }
        } else {
            unsafe {
                let ref mut fresh23 = (*(*comp).steps.offset((*comp).nbStep as isize)).value4;
                *fresh23 = 0 as *mut ()
            }
        }
        if !value5.is_null() {
            unsafe {
                let ref mut fresh24 = (*(*comp).steps.offset((*comp).nbStep as isize)).value5;
                *fresh24 = xmlDictLookup((*comp).dict, value5 as *const xmlChar, -1) as *mut ()
                    as *mut xmlChar as *mut ();
                xmlFree.expect("non-null function pointer")(value5);
            }
        } else {
            unsafe {
                let ref mut fresh25 = (*(*comp).steps.offset((*comp).nbStep as isize)).value5;
                *fresh25 = 0 as *mut ()
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
fn xmlXPathCompSwap(op: xmlXPathStepOpPtr) {
    let mut tmp: i32 = 0;
    match () {
        #[cfg(not(LIBXML_THREAD_ENABLED))]
        _ => {
            if xmlXPathDisableOptimizer == 1 {
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
fn xmlXPathDebugDumpNode(output: *mut FILE, cur: xmlNodePtr, depth: i32) {
    let safe_cur = unsafe { &mut *cur };
    let mut i: i32;
    let mut shift: [i8; 100] = [0; 100];
    i = 0 as i32;
    while i < depth && i < 25 {
        shift[(2 * i + 1) as usize] = ' ' as i8;
        shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
        i += 1
    }
    shift[(2 * i + 1) as usize] = 0 as i8;
    shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
    if cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const i8,
                shift.as_mut_ptr(),
            );
            fprintf(output, b"Node is NULL !\n\x00" as *const u8 as *const i8);
        };
        return;
    }
    if safe_cur.type_0 as u32 == XML_DOCUMENT_NODE as u32
        || safe_cur.type_0 as u32 == XML_HTML_DOCUMENT_NODE as u32
    {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const i8,
                shift.as_mut_ptr(),
            );
            fprintf(output, b" /\n\x00" as *const u8 as *const i8);
        }
    } else if safe_cur.type_0 as u32 == XML_ATTRIBUTE_NODE as u32 {
        unsafe { xmlDebugDumpAttr(output, cur as xmlAttrPtr, depth) };
    } else {
        unsafe { xmlDebugDumpOneNode(output, cur, depth) };
    };
}
#[cfg(LIBXML_DEBUG_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathDebugDumpNodeList(output: *mut FILE, mut cur: xmlNodePtr, depth: i32) {
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut i: i32;
    let mut shift: [i8; 100] = [0; 100];
    i = 0;
    while i < depth && i < 25 {
        shift[(2 * i + 1) as usize] = ' ' as i8;
        shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
        i += 1
    }
    shift[(2 * i + 1) as usize] = 0 as i8;
    shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
    if cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const i8,
                shift.as_mut_ptr(),
            );
            fprintf(output, b"Node is NULL !\n\x00" as *const u8 as *const i8);
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
fn xmlXPathDebugDumpNodeSet(output: *mut FILE, cur: xmlNodeSetPtr, depth: i32) {
    let mut i: i32 = 0;
    let mut shift: [i8; 100] = [0; 100];
    i = 0;
    while i < depth && i < 25 {
        shift[(2 * i + 1) as usize] = ' ' as i8;
        shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
        i += 1
    }
    shift[(2 * i + 1) as usize] = 0 as i8;
    shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
    if cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const i8,
                shift.as_mut_ptr(),
            );
            fprintf(output, b"NodeSet is NULL !\n\x00" as *const u8 as *const i8);
        };
        return;
    }
    if !cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"Set contains %d nodes:\n\x00" as *const u8 as *const i8,
                (*cur).nodeNr,
            )
        };
        i = 0;
        let safe_cur = unsafe { &mut *cur };
        while i < safe_cur.nodeNr {
            unsafe {
                fprintf(
                    output,
                    b"%s\x00" as *const u8 as *const i8,
                    shift.as_mut_ptr(),
                );
                fprintf(output, b"%d\x00" as *const u8 as *const i8, i + 1);
                xmlXPathDebugDumpNode(output, *(*cur).nodeTab.offset(i as isize), depth + 1);
            };
            i += 1
        }
    };
}
#[cfg(LIBXML_DEBUG_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathDebugDumpValueTree(output: *mut FILE, cur: xmlNodeSetPtr, depth: i32) {
    let mut i: i32 = 0;
    let mut shift: [i8; 100] = [0; 100];
    let safe_cur = unsafe { &mut *cur };
    i = 0;
    while i < depth && i < 25 {
        shift[(2 * i + 1) as usize] = ' ' as i8;
        shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
        i += 1
    }
    shift[(2 * i + 1) as usize] = 0 as i8;
    shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
    if unsafe { cur.is_null() || safe_cur.nodeNr == 0 || (*safe_cur.nodeTab.offset(0)).is_null() } {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const i8,
                shift.as_mut_ptr(),
            );
            fprintf(
                output,
                b"Value Tree is NULL !\n\x00" as *const u8 as *const i8,
            );
        };
        return;
    }
    unsafe {
        fprintf(
            output,
            b"%s\x00" as *const u8 as *const i8,
            shift.as_mut_ptr(),
        );
        fprintf(output, b"%d\x00" as *const u8 as *const i8, i + 1);
        xmlXPathDebugDumpNodeList(output, (**(*cur).nodeTab.offset(0)).children, depth + 1);
    }
}
#[cfg(LIBXML_XPTR_ENABLED)]
#[cfg(LIBXML_XPATH_ENABLED)]
#[cfg(LIBXML_DEBUG_ENABLED)]
fn xmlXPathDebugDumpLocationSet(output: *mut FILE, cur: xmlLocationSetPtr, depth: i32) {
    let mut i: i32;
    let mut shift: [i8; 100] = [0; 100];
    i = 0;
    while i < depth && i < 25 {
        shift[(2 * i + 1) as usize] = ' ' as i8;
        shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
        i += 1
    }
    shift[(2 * i + 1) as usize] = 0;
    shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
    if cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const i8,
                shift.as_mut_ptr(),
            );
            fprintf(
                output,
                b"LocationSet is NULL !\n\x00" as *const u8 as *const i8,
            );
        };
        return;
    }
    i = 0;
    let safe_cur = unsafe { &mut *cur };
    while i < safe_cur.locNr {
        unsafe {
            fprintf(
                output,
                b"%s\x00" as *const u8 as *const i8,
                shift.as_mut_ptr(),
            );
            fprintf(output, b"%d : \x00" as *const u8 as *const i8, i + 1);
            xmlXPathDebugDumpObject(output, *(*cur).locTab.offset(i as isize), depth + 1);
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
pub fn xmlXPathDebugDumpObject(mut output: *mut FILE, mut cur: xmlXPathObjectPtr, mut depth: i32) {
    let mut i: i32;
    let mut shift: [i8; 100] = [0; 100];
    if output.is_null() {
        return;
    }
    i = 0;
    while i < depth && i < 25 {
        shift[(2 * i + 1) as usize] = ' ' as i8;
        shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
        i += 1
    }
    shift[(2 * i + 1) as usize] = 0 as i8;
    shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
    unsafe {
        fprintf(
            output,
            b"%s\x00" as *const u8 as *const i8,
            shift.as_mut_ptr(),
        )
    };
    if cur.is_null() {
        unsafe {
            fprintf(
                output,
                b"Object is empty (NULL)\n\x00" as *const u8 as *const i8,
            )
        };
        return;
    }
    let safe_cur = unsafe { &mut *cur };
    match safe_cur.type_0 as u32 {
        XPATH_UNDEFINED => {
            unsafe {
                fprintf(
                    output,
                    b"Object is uninitialized\n\x00" as *const u8 as *const i8,
                )
            };
        }
        XPATH_NODESET => unsafe {
            fprintf(
                output,
                b"Object is a Node Set :\n\x00" as *const u8 as *const i8,
            );
            xmlXPathDebugDumpNodeSet(output, (*cur).nodesetval, depth);
        },
        XPATH_XSLT_TREE => unsafe {
            fprintf(
                output,
                b"Object is an XSLT value tree :\n\x00" as *const u8 as *const i8,
            );
            xmlXPathDebugDumpValueTree(output, (*cur).nodesetval, depth);
        },
        XPATH_BOOLEAN => {
            unsafe {
                fprintf(
                    output,
                    b"Object is a Boolean : \x00" as *const u8 as *const i8,
                );
            };
            if safe_cur.boolval != 0 {
                unsafe { fprintf(output, b"true\n\x00" as *const u8 as *const i8) };
            } else {
                unsafe { fprintf(output, b"false\n\x00" as *const u8 as *const i8) };
            }
        }
        XPATH_NUMBER => {
            match unsafe { xmlXPathIsInf((*cur).floatval) } {
                1 => unsafe {
                    fprintf(
                        output,
                        b"Object is a number : Infinity\n\x00" as *const u8 as *const i8,
                    );
                },
                -1 => unsafe {
                    fprintf(
                        output,
                        b"Object is a number : -Infinity\n\x00" as *const u8 as *const i8,
                    );
                },
                _ => {
                    if unsafe { xmlXPathIsNaN((*cur).floatval) != 0 } {
                        unsafe {
                            fprintf(
                                output,
                                b"Object is a number : NaN\n\x00" as *const u8 as *const i8,
                            )
                        };
                    } else if safe_cur.floatval == 0 as i32 as libc::c_double {
                        /* Omit sign for negative zero. */
                        unsafe {
                            fprintf(
                                output,
                                b"Object is a number : 0\n\x00" as *const u8 as *const i8,
                            )
                        };
                    } else {
                        unsafe {
                            fprintf(
                                output,
                                b"Object is a number : %0g\n\x00" as *const u8 as *const i8,
                                (*cur).floatval,
                            )
                        };
                    }
                }
            }
        }
        XPATH_STRING => unsafe {
            fprintf(
                output,
                b"Object is a string : \x00" as *const u8 as *const i8,
            );
            xmlDebugDumpString(output, (*cur).stringval);
            fprintf(output, b"\n\x00" as *const u8 as *const i8);
        },
        XPATH_POINT => {
            unsafe {
                fprintf(
                    output,
                    b"Object is a point : index %d in node\x00" as *const u8 as *const i8,
                    (*cur).index,
                );
                xmlXPathDebugDumpNode(output, (*cur).user as xmlNodePtr, depth + 1);
                fprintf(output, b"\n\x00" as *const u8 as *const i8);
            };
        }
        XPATH_RANGE => {
            if safe_cur.user2.is_null()
                || safe_cur.user2 == safe_cur.user && safe_cur.index == safe_cur.index2
            {
                unsafe {
                    fprintf(
                        output,
                        b"Object is a collapsed range :\n\x00" as *const u8 as *const i8,
                    );
                    fprintf(
                        output,
                        b"%s\x00" as *const u8 as *const i8,
                        shift.as_mut_ptr(),
                    );
                };
                if safe_cur.index >= 0 {
                    unsafe {
                        fprintf(
                            output,
                            b"index %d in \x00" as *const u8 as *const i8,
                            (*cur).index,
                        )
                    };
                }
                unsafe {
                    fprintf(output, b"node\n\x00" as *const u8 as *const i8);
                    xmlXPathDebugDumpNode(output, (*cur).user as xmlNodePtr, depth + 1);
                };
            } else {
                unsafe {
                    fprintf(
                        output,
                        b"Object is a range :\n\x00" as *const u8 as *const i8,
                    );
                    fprintf(
                        output,
                        b"%s\x00" as *const u8 as *const i8,
                        shift.as_mut_ptr(),
                    );
                    fprintf(output, b"From \x00" as *const u8 as *const i8);
                };
                if safe_cur.index >= 0 as i32 {
                    unsafe {
                        fprintf(
                            output,
                            b"index %d in \x00" as *const u8 as *const i8,
                            (*cur).index,
                        )
                    };
                }
                unsafe {
                    fprintf(output, b"node\n\x00" as *const u8 as *const i8);
                    xmlXPathDebugDumpNode(output, (*cur).user as xmlNodePtr, depth + 1 as i32);
                    fprintf(
                        output,
                        b"%s\x00" as *const u8 as *const i8,
                        shift.as_mut_ptr(),
                    );
                    fprintf(output, b"To \x00" as *const u8 as *const i8);
                };
                if safe_cur.index2 >= 0 as i32 {
                    unsafe {
                        fprintf(
                            output,
                            b"index %d in \x00" as *const u8 as *const i8,
                            (*cur).index2,
                        );
                    }
                }
                unsafe {
                    fprintf(output, b"node\n\x00" as *const u8 as *const i8);
                    xmlXPathDebugDumpNode(output, (*cur).user2 as xmlNodePtr, depth + 1 as i32);
                    fprintf(output, b"\n\x00" as *const u8 as *const i8);
                }
            }
        }
        XPATH_LOCATIONSET => {
            match () {
                #[cfg(LIBXML_XPTR_ENABLED)]
                _ => {
                    unsafe {
                        fprintf(
                            output,
                            b"Object is a Location Set:\n\x00" as *const u8 as *const i8,
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
        XPATH_USERS => {
            unsafe {
                fprintf(
                    output,
                    b"Object is user defined\n\x00" as *const u8 as *const i8,
                )
            };
        }
        _ => {}
    };
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathDebugDumpStepOp(
    output: *mut FILE,
    comp: xmlXPathCompExprPtr,
    op: xmlXPathStepOpPtr,
    depth: i32,
) {
    let safe_op = unsafe { &mut *op };
    let mut i: i32;
    let mut shift: [i8; 100] = [0; 100];
    i = 0;
    while i < depth && i < 25 {
        shift[(2 * i + 1) as usize] = ' ' as i8;
        shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
        i += 1
    }
    shift[(2 * i + 1) as usize] = 0 as i8;
    shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
    unsafe {
        fprintf(
            output,
            b"%s\x00" as *const u8 as *const i8,
            shift.as_mut_ptr(),
        )
    };
    if op.is_null() {
        unsafe { fprintf(output, b"Step is NULL\n\x00" as *const u8 as *const i8) };
        return;
    }
    match safe_op.op as u32 {
        XPATH_OP_END => {
            unsafe { fprintf(output, b"END\x00" as *const u8 as *const i8) };
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_AND => {
            unsafe { fprintf(output, b"AND\x00" as *const u8 as *const i8) };
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_OR => {
            unsafe { fprintf(output, b"OR\x00" as *const u8 as *const i8) };
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_EQUAL => {
            if safe_op.value != 0 {
                unsafe { fprintf(output, b"EQUAL =\x00" as *const u8 as *const i8) };
            } else {
                unsafe { fprintf(output, b"EQUAL !=\x00" as *const u8 as *const i8) };
            }
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_CMP => {
            if safe_op.value != 0 {
                unsafe { fprintf(output, b"CMP <\x00" as *const u8 as *const i8) };
            } else {
                unsafe { fprintf(output, b"CMP >\x00" as *const u8 as *const i8) };
            }
            if safe_op.value2 == 0 {
                unsafe { fprintf(output, b"=\x00" as *const u8 as *const i8) };
            }
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_PLUS => {
            if safe_op.value == 0 as i32 {
                unsafe { fprintf(output, b"PLUS -\x00" as *const u8 as *const i8) };
            } else if safe_op.value == 1 as i32 {
                unsafe { fprintf(output, b"PLUS +\x00" as *const u8 as *const i8) };
            } else if safe_op.value == 2 as i32 {
                unsafe { fprintf(output, b"PLUS unary -\x00" as *const u8 as *const i8) };
            } else if safe_op.value == 3 as i32 {
                unsafe { fprintf(output, b"PLUS unary - -\x00" as *const u8 as *const i8) };
            }
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_MULT => {
            if safe_op.value == 0 as i32 {
                unsafe { fprintf(output, b"MULT *\x00" as *const u8 as *const i8) };
            } else if safe_op.value == 1 as i32 {
                unsafe { fprintf(output, b"MULT div\x00" as *const u8 as *const i8) };
            } else {
                unsafe { fprintf(output, b"MULT mod\x00" as *const u8 as *const i8) };
            }
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_UNION => {
            unsafe { fprintf(output, b"UNION\x00" as *const u8 as *const i8) };
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_ROOT => {
            unsafe { fprintf(output, b"ROOT\x00" as *const u8 as *const i8) };
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_NODE => {
            unsafe { fprintf(output, b"NODE\x00" as *const u8 as *const i8) };
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_SORT => {
            unsafe { fprintf(output, b"SORT\x00" as *const u8 as *const i8) };
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_COLLECT => {
            let mut axis: xmlXPathAxisVal = safe_op.value as xmlXPathAxisVal;
            let mut test: xmlXPathTestVal = safe_op.value2 as xmlXPathTestVal;
            let mut type_0: xmlXPathTypeVal = safe_op.value3 as xmlXPathTypeVal;
            let mut prefix: *const xmlChar = safe_op.value4 as *const xmlChar;
            let mut name: *const xmlChar = safe_op.value5 as *const xmlChar;
            unsafe { fprintf(output, b"COLLECT \x00" as *const u8 as *const i8) };
            match axis as u32 {
                AXIS_ANCESTOR => {
                    unsafe { fprintf(output, b" \'ancestors\' \x00" as *const u8 as *const i8) };
                }
                AXIS_ANCESTOR_OR_SELF => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'ancestors-or-self\' \x00" as *const u8 as *const i8,
                        )
                    };
                }
                AXIS_ATTRIBUTE => {
                    unsafe { fprintf(output, b" \'attributes\' \x00" as *const u8 as *const i8) };
                }
                AXIS_CHILD => {
                    unsafe { fprintf(output, b" \'child\' \x00" as *const u8 as *const i8) };
                }
                AXIS_DESCENDANT => {
                    unsafe { fprintf(output, b" \'descendant\' \x00" as *const u8 as *const i8) };
                }
                AXIS_DESCENDANT_OR_SELF => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'descendant-or-self\' \x00" as *const u8 as *const i8,
                        )
                    };
                }
                AXIS_FOLLOWING => {
                    unsafe { fprintf(output, b" \'following\' \x00" as *const u8 as *const i8) };
                }
                AXIS_FOLLOWING_SIBLING => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'following-siblings\' \x00" as *const u8 as *const i8,
                        )
                    };
                }
                AXIS_NAMESPACE => {
                    unsafe { fprintf(output, b" \'namespace\' \x00" as *const u8 as *const i8) };
                }
                AXIS_PARENT => {
                    unsafe { fprintf(output, b" \'parent\' \x00" as *const u8 as *const i8) };
                }
                AXIS_PRECEDING => {
                    unsafe { fprintf(output, b" \'preceding\' \x00" as *const u8 as *const i8) };
                }
                AXIS_PRECEDING_SIBLING => {
                    unsafe {
                        fprintf(
                            output,
                            b" \'preceding-sibling\' \x00" as *const u8 as *const i8,
                        )
                    };
                }
                AXIS_SELF => {
                    unsafe { fprintf(output, b" \'self\' \x00" as *const u8 as *const i8) };
                }
                _ => {}
            }
            match test as u32 {
                NODE_TEST_NONE => {
                    unsafe { fprintf(output, b"\'none\' \x00" as *const u8 as *const i8) };
                }
                NODE_TEST_TYPE => {
                    unsafe { fprintf(output, b"\'type\' \x00" as *const u8 as *const i8) };
                }
                NODE_TEST_PI => {
                    unsafe { fprintf(output, b"\'PI\' \x00" as *const u8 as *const i8) };
                }
                NODE_TEST_ALL => {
                    unsafe { fprintf(output, b"\'all\' \x00" as *const u8 as *const i8) };
                }
                NODE_TEST_NS => {
                    unsafe { fprintf(output, b"\'namespace\' \x00" as *const u8 as *const i8) };
                }
                NODE_TEST_NAME => {
                    unsafe { fprintf(output, b"\'name\' \x00" as *const u8 as *const i8) };
                }
                _ => {}
            }
            match type_0 as u32 {
                NODE_TYPE_NODE => {
                    unsafe { fprintf(output, b"\'node\' \x00" as *const u8 as *const i8) };
                }
                NODE_TYPE_COMMENT => {
                    unsafe { fprintf(output, b"\'comment\' \x00" as *const u8 as *const i8) };
                }
                NODE_TYPE_TEXT => {
                    unsafe { fprintf(output, b"\'text\' \x00" as *const u8 as *const i8) };
                }
                NODE_TYPE_PI => {
                    unsafe { fprintf(output, b"\'PI\' \x00" as *const u8 as *const i8) };
                }
                _ => {}
            }
            if !prefix.is_null() {
                unsafe { fprintf(output, b"%s:\x00" as *const u8 as *const i8, prefix) };
            }
            if !name.is_null() {
                unsafe {
                    fprintf(
                        output,
                        b"%s\x00" as *const u8 as *const i8,
                        name as *const i8,
                    )
                };
            }
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_VALUE => {
            let mut object: xmlXPathObjectPtr = safe_op.value4 as xmlXPathObjectPtr;
            unsafe {
                fprintf(output, b"ELEM \x00" as *const u8 as *const i8);
                xmlXPathDebugDumpObject(output, object, 0 as i32);
            };
        }
        XPATH_OP_VARIABLE => {
            let mut prefix_0: *const xmlChar = safe_op.value5 as *const xmlChar;
            let mut name_0: *const xmlChar = safe_op.value4 as *const xmlChar;
            if !prefix_0.is_null() {
                unsafe {
                    fprintf(
                        output,
                        b"VARIABLE %s:%s\x00" as *const u8 as *const i8,
                        prefix_0,
                        name_0,
                    )
                };
            } else {
                unsafe { fprintf(output, b"VARIABLE %s\x00" as *const u8 as *const i8, name_0) };
            }
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_FUNCTION => {
            let mut nbargs: i32 = safe_op.value;
            let mut prefix_1: *const xmlChar = safe_op.value5 as *const xmlChar;
            let mut name_1: *const xmlChar = safe_op.value4 as *const xmlChar;
            if !prefix_1.is_null() {
                unsafe {
                    fprintf(
                        output,
                        b"FUNCTION %s:%s(%d args)\x00" as *const u8 as *const i8,
                        prefix_1,
                        name_1,
                        nbargs,
                    )
                };
            } else {
                unsafe {
                    fprintf(
                        output,
                        b"FUNCTION %s(%d args)\x00" as *const u8 as *const i8,
                        name_1,
                        nbargs,
                    )
                };
            }
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_ARG => {
            unsafe { fprintf(output, b"ARG\x00" as *const u8 as *const i8) };
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_PREDICATE => {
            unsafe { fprintf(output, b"PREDICATE\x00" as *const u8 as *const i8) };
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_FILTER => {
            unsafe { fprintf(output, b"FILTER\x00" as *const u8 as *const i8) };
            unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
        }
        XPATH_OP_RANGETO => {
            // LIBXML_XPTR_ENABLED
            match () {
                #[cfg(LIBXML_XPTR_ENABLED)]
                _ => {
                    unsafe { fprintf(output, b"RANGETO\x00" as *const u8 as *const i8) };
                    unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
                }
                #[cfg(not(LIBXML_XPTR_ENABLED))]
                _ => {}
            }
        }
        _ => {
            unsafe {
                fprintf(
                    output,
                    b"UNKNOWN %d\n\x00" as *const u8 as *const i8,
                    safe_op.op as u32,
                )
            };
            return;
        }
    }
    if safe_op.ch1 >= 0 {
        unsafe {
            xmlXPathDebugDumpStepOp(
                output,
                comp,
                &mut *(*comp).steps.offset(safe_op.ch1 as isize),
                depth + 1,
            )
        };
    }
    if safe_op.ch2 >= 0 {
        unsafe {
            xmlXPathDebugDumpStepOp(
                output,
                comp,
                &mut *(*comp).steps.offset(safe_op.ch2 as isize),
                depth + 1,
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
pub fn xmlXPathDebugDumpCompExpr(output: *mut FILE, comp: xmlXPathCompExprPtr, depth: i32) {
    let safe_comp = unsafe { &mut *comp };
    let mut i: i32;
    let mut shift: [i8; 100] = [0; 100];
    if output.is_null() || comp.is_null() {
        return;
    }
    i = 0;
    while i < depth && i < 25 {
        shift[(2 * i + 1) as usize] = ' ' as i8;
        shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
        i += 1
    }
    shift[(2 * i + 1) as usize] = 0 as i8;
    shift[(2 * i) as usize] = shift[(2 * i + 1) as usize];
    unsafe {
        fprintf(
            output,
            b"%s\x00" as *const u8 as *const i8,
            shift.as_mut_ptr(),
        )
    };

    match () {
        #[cfg(not(XPATH_STREAMING))]
        _ => {
            unsafe {
                fprintf(
                    output,
                    b"Compiled Expression : %d elements\n\x00" as *const u8 as *const i8,
                    (*comp).nbStep,
                )
            };
            i = safe_comp.last;
            unsafe {
                xmlXPathDebugDumpStepOp(
                    output,
                    comp,
                    &mut *(*comp).steps.offset(i as isize),
                    depth + 1,
                )
            };
        }
        #[cfg(XPATH_STREAMING)]
        _ => {
            if !safe_comp.stream.is_null() {
                unsafe {
                    fprintf(
                        output,
                        b"Streaming Expression\n\x00" as *const u8 as *const i8,
                    )
                };
            } else {
                unsafe {
                    fprintf(
                        output,
                        b"Compiled Expression : %d elements\n\x00" as *const u8 as *const i8,
                        (*comp).nbStep,
                    )
                };
                i = safe_comp.last;
                unsafe {
                    xmlXPathDebugDumpStepOp(
                        output,
                        comp,
                        &mut *(*comp).steps.offset(i as isize),
                        depth + 1,
                    )
                };
            };
        }
    }
}
/* XP_DEBUG_OBJ_USAGE */
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterUndefined: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterNodeset: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterBool: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterNumber: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterString: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterPoint: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterRange: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterLocset: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterUsers: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterXSLTTree: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjCounterAll: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalUndefined: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalNodeset: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalBool: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalNumber: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalString: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalPoint: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalRange: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalLocset: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalUsers: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalXSLTTree: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjTotalAll: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxUndefined: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxNodeset: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxBool: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxNumber: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxString: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxPoint: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxRange: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxLocset: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxUsers: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxXSLTTree: i32 = 0;
#[cfg(LIBXML_XPATH_ENABLED)]
static mut xmlXPathDebugObjMaxAll: i32 = 0;

#[cfg(XP_DEBUG_OBJ_USAGE)]
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathDebugObjUsageReset(ctxt: xmlXPathContextPtr) {
    if !ctxt.is_null() {
        let safe_ctxt = unsafe { &mut *ctxt };
        if !safe_ctxt.cache.is_null() {
            let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
            let safe_cache = unsafe { &mut *cache };
            safe_cache.dbgCachedAll = 0;
            safe_cache.dbgCachedNodeset = 0;
            safe_cache.dbgCachedString = 0;
            safe_cache.dbgCachedBool = 0;
            safe_cache.dbgCachedNumber = 0;
            safe_cache.dbgCachedPoint = 0;
            safe_cache.dbgCachedRange = 0;
            safe_cache.dbgCachedLocset = 0;
            safe_cache.dbgCachedUsers = 0;
            safe_cache.dbgCachedXSLTTree = 0;
            safe_cache.dbgCachedUndefined = 0;
            safe_cache.dbgReusedAll = 0;
            safe_cache.dbgReusedNodeset = 0;
            safe_cache.dbgReusedString = 0;
            safe_cache.dbgReusedBool = 0;
            safe_cache.dbgReusedNumber = 0;
            safe_cache.dbgReusedPoint = 0;
            safe_cache.dbgReusedRange = 0;
            safe_cache.dbgReusedLocset = 0;
            safe_cache.dbgReusedUsers = 0;
            safe_cache.dbgReusedXSLTTree = 0;
            safe_cache.dbgReusedUndefined = 0
        }
    }
    xmlXPathDebugObjCounterUndefined = 0;
    xmlXPathDebugObjCounterNodeset = 0;
    xmlXPathDebugObjCounterBool = 0;
    xmlXPathDebugObjCounterNumber = 0;
    xmlXPathDebugObjCounterString = 0;
    xmlXPathDebugObjCounterPoint = 0;
    xmlXPathDebugObjCounterRange = 0;
    xmlXPathDebugObjCounterLocset = 0;
    xmlXPathDebugObjCounterUsers = 0;
    xmlXPathDebugObjCounterXSLTTree = 0;
    xmlXPathDebugObjCounterAll = 0;
    xmlXPathDebugObjTotalUndefined = 0;
    xmlXPathDebugObjTotalNodeset = 0;
    xmlXPathDebugObjTotalBool = 0;
    xmlXPathDebugObjTotalNumber = 0;
    xmlXPathDebugObjTotalString = 0;
    xmlXPathDebugObjTotalPoint = 0;
    xmlXPathDebugObjTotalRange = 0;
    xmlXPathDebugObjTotalLocset = 0;
    xmlXPathDebugObjTotalUsers = 0;
    xmlXPathDebugObjTotalXSLTTree = 0;
    xmlXPathDebugObjTotalAll = 0;
    xmlXPathDebugObjMaxUndefined = 0;
    xmlXPathDebugObjMaxNodeset = 0;
    xmlXPathDebugObjMaxBool = 0;
    xmlXPathDebugObjMaxNumber = 0;
    xmlXPathDebugObjMaxString = 0;
    xmlXPathDebugObjMaxPoint = 0;
    xmlXPathDebugObjMaxRange = 0;
    xmlXPathDebugObjMaxLocset = 0;
    xmlXPathDebugObjMaxUsers = 0;
    xmlXPathDebugObjMaxXSLTTree = 0;
    xmlXPathDebugObjMaxAll = 0;
}

#[cfg(XP_DEBUG_OBJ_USAGE)]
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathDebugObjUsageRequested(
    ctxt: xmlXPathContextPtr,
    objType: xmlXPathObjectType,
) {
    let mut isCached: i32 = 0;
    if !ctxt.is_null() {
        let safe_ctxt = unsafe { &mut *ctxt };
        if !safe_ctxt.cache.is_null() {
            let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
            let safe_cache = unsafe { &mut *cache };
            isCached = 1;
            safe_cache.dbgReusedAll += 1;
            match objType as u32 {
                XPATH_UNDEFINED => safe_cache.dbgReusedUndefined += 1,
                XPATH_NODESET => safe_cache.dbgReusedNodeset += 1,
                XPATH_BOOLEAN => safe_cache.dbgReusedBool += 1,
                XPATH_NUMBER => safe_cache.dbgReusedNumber += 1,
                XPATH_STRING => safe_cache.dbgReusedString += 1,
                XPATH_POINT => safe_cache.dbgReusedPoint += 1,
                XPATH_RANGE => safe_cache.dbgReusedRange += 1,
                XPATH_LOCATIONSET => safe_cache.dbgReusedLocset += 1,
                XPATH_USERS => safe_cache.dbgReusedUsers += 1,
                XPATH_XSLT_TREE => safe_cache.dbgReusedXSLTTree += 1,
                _ => {}
            }
        }
    }
    match objType as u32 {
        XPATH_UNDEFINED => {
            if isCached == 0 {
                xmlXPathDebugObjTotalUndefined += 1
            }
            xmlXPathDebugObjCounterUndefined += 1;
            if xmlXPathDebugObjCounterUndefined > xmlXPathDebugObjMaxUndefined {
                xmlXPathDebugObjMaxUndefined = xmlXPathDebugObjCounterUndefined
            }
        }
        XPATH_NODESET => {
            if isCached == 0 {
                xmlXPathDebugObjTotalNodeset += 1
            }
            xmlXPathDebugObjCounterNodeset += 1;
            if xmlXPathDebugObjCounterNodeset > xmlXPathDebugObjMaxNodeset {
                xmlXPathDebugObjMaxNodeset = xmlXPathDebugObjCounterNodeset
            }
        }
        XPATH_BOOLEAN => {
            if isCached == 0 {
                xmlXPathDebugObjTotalBool += 1
            }
            xmlXPathDebugObjCounterBool += 1;
            if xmlXPathDebugObjCounterBool > xmlXPathDebugObjMaxBool {
                xmlXPathDebugObjMaxBool = xmlXPathDebugObjCounterBool
            }
        }
        XPATH_NUMBER => {
            if isCached == 0 {
                xmlXPathDebugObjTotalNumber += 1
            }
            xmlXPathDebugObjCounterNumber += 1;
            if xmlXPathDebugObjCounterNumber > xmlXPathDebugObjMaxNumber {
                xmlXPathDebugObjMaxNumber = xmlXPathDebugObjCounterNumber
            }
        }
        XPATH_STRING => {
            if isCached == 0 {
                xmlXPathDebugObjTotalString += 1
            }
            xmlXPathDebugObjCounterString += 1;
            if xmlXPathDebugObjCounterString > xmlXPathDebugObjMaxString {
                xmlXPathDebugObjMaxString = xmlXPathDebugObjCounterString
            }
        }
        XPATH_POINT => {
            if isCached == 0 {
                xmlXPathDebugObjTotalPoint += 1
            }
            xmlXPathDebugObjCounterPoint += 1;
            if xmlXPathDebugObjCounterPoint > xmlXPathDebugObjMaxPoint {
                xmlXPathDebugObjMaxPoint = xmlXPathDebugObjCounterPoint
            }
        }
        XPATH_RANGE => {
            if isCached == 0 {
                xmlXPathDebugObjTotalRange += 1
            }
            xmlXPathDebugObjCounterRange += 1;
            if xmlXPathDebugObjCounterRange > xmlXPathDebugObjMaxRange {
                xmlXPathDebugObjMaxRange = xmlXPathDebugObjCounterRange
            }
        }
        XPATH_LOCATIONSET => {
            if isCached == 0 {
                xmlXPathDebugObjTotalLocset += 1
            }
            xmlXPathDebugObjCounterLocset += 1;
            if xmlXPathDebugObjCounterLocset > xmlXPathDebugObjMaxLocset {
                xmlXPathDebugObjMaxLocset = xmlXPathDebugObjCounterLocset
            }
        }
        XPATH_USERS => {
            if isCached == 0 {
                xmlXPathDebugObjTotalUsers += 1
            }
            xmlXPathDebugObjCounterUsers += 1;
            if xmlXPathDebugObjCounterUsers > xmlXPathDebugObjMaxUsers {
                xmlXPathDebugObjMaxUsers = xmlXPathDebugObjCounterUsers
            }
        }
        XPATH_XSLT_TREE => {
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
extern "C" fn xmlXPathDebugObjUsageReleased(ctxt: xmlXPathContextPtr, objType: xmlXPathObjectType) {
    let mut isCached: i32 = 0;
    if !ctxt.is_null() {
        let safe_ctxt = unsafe { &mut *ctxt };
        if !safe_ctxt.cache.is_null() {
            let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
            let safe_cache = unsafe { &mut *cache };
            isCached = 1;
            safe_cache.dbgCachedAll += 1;
            match objType as u32 {
                XPATH_UNDEFINED => safe_cache.dbgCachedUndefined += 1,
                XPATH_NODESET => safe_cache.dbgCachedNodeset += 1,
                XPATH_BOOLEAN => safe_cache.dbgCachedBool += 1,
                XPATH_NUMBER => safe_cache.dbgCachedNumber += 1,
                XPATH_STRING => safe_cache.dbgCachedString += 1,
                XPATH_POINT => safe_cache.dbgCachedPoint += 1,
                XPATH_RANGE => safe_cache.dbgCachedRange += 1,
                XPATH_LOCATIONSET => safe_cache.dbgCachedLocset += 1,
                XPATH_USERS => safe_cache.dbgCachedUsers += 1,
                XPATH_XSLT_TREE => safe_cache.dbgCachedXSLTTree += 1,
                _ => {}
            }
        }
    }
    match objType as u32 {
        XPATH_UNDEFINED => xmlXPathDebugObjCounterUndefined -= 1,
        XPATH_NODESET => xmlXPathDebugObjCounterNodeset -= 1,
        XPATH_BOOLEAN => xmlXPathDebugObjCounterBool -= 1,
        XPATH_NUMBER => xmlXPathDebugObjCounterNumber -= 1,
        XPATH_STRING => xmlXPathDebugObjCounterString -= 1,
        XPATH_POINT => xmlXPathDebugObjCounterPoint -= 1,
        XPATH_RANGE => xmlXPathDebugObjCounterRange -= 1,
        XPATH_LOCATIONSET => xmlXPathDebugObjCounterLocset -= 1,
        XPATH_USERS => xmlXPathDebugObjCounterUsers -= 1,
        XPATH_XSLT_TREE => xmlXPathDebugObjCounterXSLTTree -= 1,
        _ => {}
    }
    xmlXPathDebugObjCounterAll -= 1;
}

#[cfg(XP_DEBUG_OBJ_USAGE)]
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathDebugObjUsageDisplay(ctxt: xmlXPathContextPtr) {
    let mut reqAll: i32 = 0;
    let mut reqNodeset: i32 = 0;
    let mut reqString: i32 = 0;
    let mut reqBool: i32 = 0;
    let mut reqNumber: i32 = 0;
    let mut reqXSLTTree: i32 = 0;
    let mut reqUndefined: i32 = 0;
    let mut caAll: i32 = 0;
    let mut caNodeset: i32 = 0;
    let mut caString: i32 = 0;
    let mut caBool: i32 = 0;
    let mut caNumber: i32 = 0;
    let mut caXSLTTree: i32 = 0;
    let mut caUndefined: i32 = 0;
    let mut reAll: i32 = 0;
    let mut reNodeset: i32 = 0;
    let mut reString: i32 = 0;
    let mut reBool: i32 = 0;
    let mut reNumber: i32 = 0;
    let mut reXSLTTree: i32 = 0;
    let mut reUndefined: i32 = 0;
    let mut leftObjs: i32 = xmlXPathDebugObjCounterAll;
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
            ::std::mem::size_of::<xmlXPathContextCache>() as u64
        ) as xmlXPathContextCachePtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating object cache\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathContextCachePtr;
    }
    let safe_ret = unsafe { &mut *ret };
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathContextCache>() as u64,
        )
    };
    safe_ret.maxNodeset = 100;
    safe_ret.maxString = 100;
    safe_ret.maxBoolean = 100;
    safe_ret.maxNumber = 100;
    safe_ret.maxMisc = 100;
    return ret;
}
#[cfg(LIBXML_XPATH_ENABLED)]
unsafe fn xmlXPathCacheFreeObjectList(list: xmlPointerListPtr) {
    let mut i: i32;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if list.is_null() {
        return;
    }
    let safe_list = unsafe { &mut *list };
    i = 0;
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
                        (*(*obj).nodesetval).nodeTab as *mut (),
                    )
                };
            }
            unsafe { xmlFree.expect("non-null function pointer")((*obj).nodesetval as *mut ()) };
        }
        unsafe { xmlFree.expect("non-null function pointer")(obj as *mut ()) };
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
    xmlPointerListFree(list);
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathFreeCache(cache: xmlXPathContextCachePtr) {
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
    unsafe { xmlFree.expect("non-null function pointer")(cache as *mut ()) };
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
    ctxt: xmlXPathContextPtr,
    active: i32,
    mut value: i32,
    options: i32,
) -> i32 {
    if ctxt.is_null() {
        return -1;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if active != 0 {
        let mut cache: xmlXPathContextCachePtr = 0 as *mut xmlXPathContextCache;
        if safe_ctxt.cache.is_null() {
            safe_ctxt.cache = xmlXPathNewCache() as *mut ();
            if safe_ctxt.cache.is_null() {
                return -1;
            }
        }
        cache = safe_ctxt.cache as xmlXPathContextCachePtr;
        let safe_cache = unsafe { &mut *cache };
        if options == 0 {
            if value < 0 {
                value = 100
            }
            safe_cache.maxNodeset = value;
            safe_cache.maxString = value;
            safe_cache.maxNumber = value;
            safe_cache.maxBoolean = value;
            safe_cache.maxMisc = value
        }
    } else if !safe_ctxt.cache.is_null() {
        unsafe { xmlXPathFreeCache((*ctxt).cache as xmlXPathContextCachePtr) };
        safe_ctxt.cache = 0 as *mut ()
    }
    return 0;
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
unsafe fn xmlXPathCacheWrapNodeSet(
    ctxt: xmlXPathContextPtr,
    val: xmlNodeSetPtr,
) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        let safe_cache = unsafe { &mut *cache };
        if unsafe { !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 } {
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
    return xmlXPathWrapNodeSet(val);
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
fn xmlXPathCacheWrapString(ctxt: xmlXPathContextPtr, val: *mut xmlChar) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        let safe_cache = unsafe { &mut *cache };
        if unsafe { !(*cache).stringObjs.is_null() && (*(*cache).stringObjs).number != 0 } {
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
            if unsafe { !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 } {
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
    return xmlXPathWrapString(val);
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
fn xmlXPathCacheNewNodeSet(ctxt: xmlXPathContextPtr, val: xmlNodePtr) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        let safe_cache = unsafe { &mut *cache };
        if unsafe { !(*cache).nodesetObjs.is_null() && (*(*cache).nodesetObjs).number != 0 } {
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
                (*ret).boolval = 0;
                if !val.is_null() {
                    if (*(*ret).nodesetval).nodeMax == 0
                        || (*val).type_0 as u32 == XML_NAMESPACE_DECL as u32
                    {
                        /* TODO: Check memory error. */
                        xmlXPathNodeSetAddUnique((*ret).nodesetval, val);
                    } else {
                        let ref mut fresh30 = *(*(*ret).nodesetval).nodeTab.offset(0);
                        *fresh30 = val;
                        (*(*ret).nodesetval).nodeNr = 1
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
            if unsafe { !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 } {
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
                    (*ret_0).boolval = 0;
                    (*ret_0).nodesetval = xmlXPathNodeSetCreate(val);
                    if (*ret_0).nodesetval.is_null() {
                        (*ctxt).lastError.domain = XML_FROM_XPATH as i32;
                        (*ctxt).lastError.code = XML_ERR_NO_MEMORY as i32;
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
    return xmlXPathNewNodeSet(val);
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
fn xmlXPathCacheNewCString(ctxt: xmlXPathContextPtr, val: *const i8) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        if unsafe { !(*cache).stringObjs.is_null() && (*(*cache).stringObjs).number != 0 } {
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
            if unsafe { !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 } {
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
    return xmlXPathNewCString(val);
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
fn xmlXPathCacheNewString(ctxt: xmlXPathContextPtr, val: *const xmlChar) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        if unsafe { !(*cache).stringObjs.is_null() && (*(*cache).stringObjs).number != 0 } {
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
                        xmlStrdup(b"\x00" as *const u8 as *const i8 as *const xmlChar)
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
            if unsafe { !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 } {
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
                            xmlStrdup(b"\x00" as *const u8 as *const i8 as *const xmlChar)
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
    return xmlXPathNewString(val);
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
fn xmlXPathCacheNewBoolean(ctxt: xmlXPathContextPtr, val: i32) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        if unsafe { !(*cache).booleanObjs.is_null() && (*(*cache).booleanObjs).number != 0 } {
            unsafe {
                let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                (*(*cache).booleanObjs).number -= 1;
                ret = *(*(*cache).booleanObjs)
                    .items
                    .offset((*(*cache).booleanObjs).number as isize)
                    as xmlXPathObjectPtr;
                (*ret).type_0 = XPATH_BOOLEAN;
                (*ret).boolval = (val != 0) as i32;
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
            if unsafe { !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 } {
                unsafe {
                    let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                    (*(*cache).miscObjs).number -= 1;
                    ret_0 = *(*(*cache).miscObjs)
                        .items
                        .offset((*(*cache).miscObjs).number as isize)
                        as xmlXPathObjectPtr;
                    (*ret_0).type_0 = XPATH_BOOLEAN;
                    (*ret_0).boolval = (val != 0) as i32;
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
    return xmlXPathNewBoolean(val);
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
fn xmlXPathCacheNewFloat(ctxt: xmlXPathContextPtr, val: libc::c_double) -> xmlXPathObjectPtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        if unsafe { !(*cache).numberObjs.is_null() && (*(*cache).numberObjs).number != 0 } {
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
            if unsafe { !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 } {
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
    return xmlXPathNewFloat(val);
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
    ctxt: xmlXPathContextPtr,
    val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut res: *mut xmlChar = 0 as *mut xmlChar;
    if val.is_null() {
        return xmlXPathCacheNewCString(ctxt, b"\x00" as *const u8 as *const i8);
    }
    let safe_val = unsafe { &mut *val };
    match safe_val.type_0 as u32 {
        XPATH_NODESET | XPATH_XSLT_TREE => {
            res = unsafe { xmlXPathCastNodeSetToString((*val).nodesetval) }
        }
        XPATH_STRING => return val,
        XPATH_BOOLEAN => res = unsafe { xmlXPathCastBooleanToString((*val).boolval) },
        XPATH_NUMBER => res = unsafe { xmlXPathCastNumberToString((*val).floatval) },
        XPATH_USERS | XPATH_POINT | XPATH_RANGE | XPATH_LOCATIONSET => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                    b"xpath.c\x00" as *const u8 as *const i8,
                    2712 as i32,
                )
            };
        }
        XPATH_UNDEFINED | _ => {
            match () {
                #[cfg(DEBUG_EXPR)]
                _ => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"STRING: undefined\n" as *const u8 as *const i8,
                        )
                    };
                }
                #[cfg(not(DEBUG_EXPR))]
                _ => {}
            };
        }
    }
    xmlXPathReleaseObject(ctxt, val);
    if res.is_null() {
        return xmlXPathCacheNewCString(ctxt, b"\x00" as *const u8 as *const i8);
    }
    return xmlXPathCacheWrapString(ctxt, res);
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
fn xmlXPathCacheObjectCopy(ctxt: xmlXPathContextPtr, val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    if val.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    let safe_val = unsafe { &mut *val };
    if !ctxt.is_null() && !safe_ctxt.cache.is_null() {
        match safe_val.type_0 as u32 {
            XPATH_NODESET => {
                return unsafe {
                    xmlXPathCacheWrapNodeSet(
                        ctxt,
                        xmlXPathNodeSetMerge(0 as xmlNodeSetPtr, (*val).nodesetval),
                    )
                }
            }
            XPATH_STRING => return unsafe { xmlXPathCacheNewString(ctxt, (*val).stringval) },
            XPATH_BOOLEAN => return unsafe { xmlXPathCacheNewBoolean(ctxt, (*val).boolval) },
            XPATH_NUMBER => return unsafe { xmlXPathCacheNewFloat(ctxt, (*val).floatval) },
            _ => {}
        }
    }
    return xmlXPathObjectCopy(val);
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
    ctxt: xmlXPathContextPtr,
    val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return xmlXPathCacheNewBoolean(ctxt, 0);
    }
    let safe_val = unsafe { &mut *val };
    if safe_val.type_0 as u32 == XPATH_BOOLEAN as u32 {
        return val;
    }
    ret = xmlXPathCacheNewBoolean(ctxt, xmlXPathCastToBoolean(val));
    xmlXPathReleaseObject(ctxt, val);
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
    ctxt: xmlXPathContextPtr,
    val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return xmlXPathCacheNewFloat(ctxt, 0.0f64);
    }
    let safe_val = unsafe { &mut *val };
    if safe_val.type_0 as u32 == XPATH_NUMBER as u32 {
        return val;
    }
    ret = xmlXPathCacheNewFloat(ctxt, xmlXPathCastToNumber(val));
    xmlXPathReleaseObject(ctxt, val);
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
fn xmlXPathSetFrame(ctxt: xmlXPathParserContextPtr) -> i32 {
    let mut ret: i32 = 0;
    if ctxt.is_null() {
        return 0;
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
fn xmlXPathPopFrame(ctxt: xmlXPathParserContextPtr, frame: i32) {
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame {
        xmlXPatherror(
            ctxt,
            b"xpath.c\x00" as *const u8 as *const i8,
            2840,
            XPATH_STACK_ERROR as i32,
        );
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
pub fn valuePop(ctxt: xmlXPathParserContextPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.valueNr <= 0 {
        return 0 as xmlXPathObjectPtr;
    }
    if safe_ctxt.valueNr <= safe_ctxt.valueFrame {
        xmlXPatherror(
            ctxt,
            b"xpath.c\x00" as *const u8 as *const i8,
            2862,
            XPATH_STACK_ERROR as i32,
        );
        return 0 as xmlXPathObjectPtr;
    }
    safe_ctxt.valueNr -= 1;
    if safe_ctxt.valueNr > 0 {
        safe_ctxt.value = unsafe { *(*ctxt).valueTab.offset(((*ctxt).valueNr - 1) as isize) }
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
pub fn valuePush(ctxt: xmlXPathParserContextPtr, value: xmlXPathObjectPtr) -> i32 {
    if ctxt.is_null() {
        return -1;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if value.is_null() {
        /*
         * A NULL value typically indicates that a memory allocation failed, * so we set ctxt->error here to propagate the error.
         */
        safe_ctxt.error = XPATH_MEMORY_ERROR as i32;
        return -1;
    }
    if safe_ctxt.valueNr >= safe_ctxt.valueMax {
        let mut tmp: *mut xmlXPathObjectPtr = 0 as *mut xmlXPathObjectPtr;
        if safe_ctxt.valueMax >= 1000000 {
            xmlXPathPErrMemory(
                ctxt,
                b"XPath stack depth limit reached\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        tmp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*ctxt).valueTab as *mut (),
                ((2 * (*ctxt).valueMax) as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>() as u64),
            ) as *mut xmlXPathObjectPtr
        };
        if tmp.is_null() {
            xmlXPathPErrMemory(ctxt, b"pushing value\n\x00" as *const u8 as *const i8);
            return -1;
        }
        safe_ctxt.valueMax *= 2;
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
pub fn xmlXPathPopBoolean(ctxt: xmlXPathParserContextPtr) -> i32 {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: i32;
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPatherror(
            ctxt,
            b"xpath.c\x00" as *const u8 as *const i8,
            2935,
            XPATH_INVALID_OPERAND as i32,
        );
        if !ctxt.is_null() {
            let safe_ctxt = unsafe { &mut *ctxt };
            safe_ctxt.error = XPATH_INVALID_OPERAND as i32
        }
        return 0;
    }
    let safe_obj = unsafe { &mut *obj };
    if safe_obj.type_0 as u32 != XPATH_BOOLEAN as u32 {
        ret = xmlXPathCastToBoolean(obj)
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
pub fn xmlXPathPopNumber(ctxt: xmlXPathParserContextPtr) -> libc::c_double {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: libc::c_double = 0.;
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPatherror(
            ctxt,
            b"xpath.c\x00" as *const u8 as *const i8,
            2962,
            XPATH_INVALID_OPERAND as i32,
        );
        if !ctxt.is_null() {
            let safe_ctxt = unsafe { &mut *ctxt };
            safe_ctxt.error = XPATH_INVALID_OPERAND as i32
        }
        return 0 as libc::c_double;
    }
    let safe_obj = unsafe { &mut *obj };
    if safe_obj.type_0 as u32 != XPATH_NUMBER as u32 {
        ret = xmlXPathCastToNumber(obj)
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
pub fn xmlXPathPopString(ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject; /* this does required strdup */
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPatherror(
            ctxt,
            b"xpath.c\x00" as *const u8 as *const i8,
            2989,
            XPATH_INVALID_OPERAND as i32,
        );
        if !ctxt.is_null() {
            let safe_ctxt = unsafe { &mut *ctxt };
            safe_ctxt.error = XPATH_INVALID_OPERAND as i32
        }
        return 0 as *mut xmlChar;
    }
    let safe_obj = unsafe { &mut *obj };
    ret = xmlXPathCastToString(obj);
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
        xmlXPatherror(
            ctxt,
            b"xpath.c\x00" as *const u8 as *const i8,
            3016,
            XPATH_INVALID_OPERAND as i32,
        );
        if !ctxt.is_null() {
            safe_ctxt.error = XPATH_INVALID_OPERAND as i32
        }
        return 0 as xmlNodeSetPtr;
    }
    if unsafe {
        !(!(*ctxt).value.is_null()
            && ((*(*ctxt).value).type_0 as u32 == XPATH_NODESET as u32
                || (*(*ctxt).value).type_0 as u32 == XPATH_XSLT_TREE as u32))
    } {
        xmlXPatherror(
            ctxt,
            b"xpath.c\x00" as *const u8 as *const i8,
            3020,
            XPATH_INVALID_TYPE as i32,
        );
        if !ctxt.is_null() {
            safe_ctxt.error = XPATH_INVALID_TYPE as i32
        }
        return 0 as xmlNodeSetPtr;
    }
    obj = valuePop(ctxt);
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
pub fn xmlXPathPopExternal(ctxt: xmlXPathParserContextPtr) -> *mut () {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: *mut () = 0 as *mut ();
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.value.is_null() {
        xmlXPatherror(
            ctxt,
            b"xpath.c\x00" as *const u8 as *const i8,
            3050,
            XPATH_INVALID_OPERAND as i32,
        );
        if !ctxt.is_null() {
            safe_ctxt.error = XPATH_INVALID_OPERAND as i32
        }
        return 0 as *mut ();
    }
    if unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_USERS as u32 } {
        xmlXPatherror(
            ctxt,
            b"xpath.c\x00" as *const u8 as *const i8,
            3054,
            XPATH_INVALID_TYPE as i32,
        );
        if !ctxt.is_null() {
            safe_ctxt.error = XPATH_INVALID_TYPE as i32
        }
        return 0 as *mut ();
    }
    obj = valuePop(ctxt);
    let safe_obj = unsafe { &mut *obj };
    ret = safe_obj.user;
    safe_obj.user = 0 as *mut ();
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
fn xmlXPathFormatNumber(number: libc::c_double, buffer: *mut i8, buffersize: i32) {
    match xmlXPathIsInf(number) {
        1 => {
            if buffersize > ::std::mem::size_of::<[i8; 9]>() as u64 as i32 {
                unsafe {
                    snprintf(
                        buffer,
                        buffersize as u64,
                        b"Infinity\x00" as *const u8 as *const i8,
                    )
                };
            }
        }
        -1 => {
            if buffersize > ::std::mem::size_of::<[i8; 10]>() as u64 as i32 {
                unsafe {
                    snprintf(
                        buffer,
                        buffersize as u64,
                        b"-Infinity\x00" as *const u8 as *const i8,
                    )
                };
            }
        }
        _ => {
            if xmlXPathIsNaN(number) != 0 {
                if buffersize > ::std::mem::size_of::<[i8; 4]>() as u64 as i32 {
                    unsafe {
                        snprintf(
                            buffer,
                            buffersize as u64,
                            b"NaN\x00" as *const u8 as *const i8,
                        )
                    };
                }
            } else if number == 0 as i32 as libc::c_double {
                /* Omit sign for negative zero. */
                unsafe {
                    snprintf(
                        buffer,
                        buffersize as u64,
                        b"0\x00" as *const u8 as *const i8,
                    )
                };
            } else if number > (-(2147483647 as i32) - 1 as i32) as libc::c_double
                && number < 2147483647 as i32 as libc::c_double
                && number == number as i32 as libc::c_double
            {
                let mut work: [i8; 30] = [0; 30];
                let mut ptr: *mut i8 = 0 as *mut i8;
                let mut cur: *mut i8 = 0 as *mut i8;
                let mut value: i32 = number as i32;
                ptr = unsafe { &mut *buffer.offset(0 as i32 as isize) as *mut i8 };
                if value == 0 as i32 {
                    let fresh34 = ptr;
                    unsafe {
                        ptr = ptr.offset(1);
                        *fresh34 = '0' as i32 as i8
                    }
                } else {
                    unsafe {
                        snprintf(
                            work.as_mut_ptr(),
                            29 as i32 as u64,
                            b"%d\x00" as *const u8 as *const i8,
                            value,
                        )
                    };
                    cur = unsafe { &mut *work.as_mut_ptr().offset(0 as i32 as isize) as *mut i8 };
                    while unsafe {
                        *cur as i32 != 0 && (ptr.offset_from(buffer) as i64) < buffersize as i64
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
                if unsafe { (ptr.offset_from(buffer) as i64) < buffersize as i64 } {
                    unsafe { *ptr = 0 as i32 as i8 }
                } else if buffersize > 0 as i32 {
                    unsafe {
                        ptr = ptr.offset(-1);
                        *ptr = 0 as i32 as i8
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
                let mut work_0: [i8; 28] = [0; 28];
                let mut integer_place: i32;
                let mut fraction_place: i32;
                let mut ptr_0: *mut i8 = 0 as *mut i8;
                let mut after_fraction: *mut i8 = 0 as *mut i8;
                let mut absolute_value: libc::c_double = 0.;
                let mut size: i32;
                absolute_value = unsafe { fabs(number) };
                /*
                 * First choose format - scientific or regular floating point.
                 * In either case, result is in work, and after_fraction points
                 * just past the fractional part.
                 */
                if (absolute_value > 1E9f64 || absolute_value < 1E-5f64) && absolute_value != 0.0f64
                {
                    /* Use scientific notation */
                    integer_place = 15 + (3 + 2) + 1;
                    fraction_place = 15 - 1;
                    size = unsafe {
                        snprintf(
                            work_0.as_mut_ptr(),
                            ::std::mem::size_of::<[i8; 28]>() as u64,
                            b"%*.*e\x00" as *const u8 as *const i8,
                            integer_place,
                            fraction_place,
                            number,
                        )
                    };
                    while size > 0 && work_0[size as usize] as i32 != 'e' as i32 {
                        size -= 1
                    }
                } else {
                    /* Use regular notation */
                    if absolute_value > 0.0f64 {
                        integer_place = unsafe { log10(absolute_value) } as i32;
                        if integer_place > 0 {
                            fraction_place = 15 - integer_place - 1
                        } else {
                            fraction_place = 15 - integer_place
                        }
                    } else {
                        fraction_place = 1
                    }
                    size = unsafe {
                        snprintf(
                            work_0.as_mut_ptr(),
                            ::std::mem::size_of::<[i8; 28]>() as u64,
                            b"%0.*f\x00" as *const u8 as *const i8,
                            fraction_place,
                            number,
                        )
                    }
                }
                /* Remove leading spaces sometimes inserted by snprintf */
                while work_0[0 as usize] as i32 == ' ' as i32 {
                    ptr_0 = unsafe { &mut *work_0.as_mut_ptr().offset(0 as isize) as *mut i8 };
                    loop {
                        unsafe {
                            let ref mut fresh37 = *ptr_0.offset(0 as isize);
                            *fresh37 = *ptr_0.offset(1 as isize);
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
                        if !(*ptr_0 as i32 == '0' as i32) {
                            break;
                        }
                    }
                }
                if unsafe { *ptr_0 as i32 != '.' as i32 } {
                    ptr_0 = unsafe { ptr_0.offset(1) }
                }
                loop {
                    unsafe {
                        let fresh38 = after_fraction;
                        after_fraction = after_fraction.offset(1);
                        let fresh39 = ptr_0;
                        ptr_0 = ptr_0.offset(1);
                        *fresh39 = *fresh38;
                        if !(*fresh39 as i32 != 0) {
                            break;
                        }
                    }
                }
                /* Finally copy result back to caller */
                size = unsafe { strlen(work_0.as_mut_ptr()).wrapping_add(1 as u64) as i32 };
                if size > buffersize {
                    work_0[(buffersize - 1) as usize] = 0;
                    size = buffersize
                }
                unsafe {
                    memmove(
                        buffer as *mut (),
                        work_0.as_mut_ptr() as *const (),
                        size as u64,
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
pub fn xmlXPathOrderDocElems(doc: xmlDocPtr) -> i64 {
    let mut count: ptrdiff_t = 0 as ptrdiff_t;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if doc.is_null() {
        return -1;
    }
    let safe_doc = unsafe { &mut *doc };
    cur = safe_doc.children;
    while !cur.is_null() {
        let safe_cur = unsafe { &mut *cur };
        if safe_cur.type_0 as u32 == XML_ELEMENT_NODE as u32 {
            count += 1;
            safe_cur.content = -count as *mut () as *mut xmlChar;
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
pub fn xmlXPathCmpNodes(mut node1: xmlNodePtr, mut node2: xmlNodePtr) -> i32 {
    let mut depth1: i32;
    let mut depth2: i32;
    let mut attr1: i32 = 0;
    let mut attr2: i32 = 0;
    let mut attrNode1: xmlNodePtr = 0 as xmlNodePtr;
    let mut attrNode2: xmlNodePtr = 0 as xmlNodePtr;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    if node1.is_null() || node2.is_null() {
        return -2;
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
        return 0;
    }
    if safe_node1.type_0 as u32 == XML_ATTRIBUTE_NODE as u32 {
        attr1 = 1;
        attrNode1 = node1;
        node1 = safe_node1.parent
    }
    if safe_node2.type_0 as u32 == XML_ATTRIBUTE_NODE as u32 {
        attr2 = 1;
        attrNode2 = node2;
        node2 = safe_node2.parent
    }
    if node1 == node2 {
        if attr1 == attr2 {
            /* not required, but we keep attributes in order */
            if attr1 != 0 {
                cur = safe_attrNode2.prev;

                while !cur.is_null() {
                    if cur == attrNode1 {
                        return 1 as i32;
                    }
                    unsafe { cur = (*cur).prev }
                }
                return -1;
            }
            return 0;
        }
        if attr2 == 1 {
            return 1;
        }
        return -1;
    }
    if unsafe {
        (*node1).type_0 as u32 == XML_NAMESPACE_DECL as u32
            || (*node2).type_0 as u32 == XML_NAMESPACE_DECL as u32
    } {
        return 1;
    }
    if unsafe { node1 == (*node2).prev } {
        return 1;
    }
    if unsafe { node1 == (*node2).next } {
        return -1;
    }
    /*
     * Speedup using document order if available.
     */
    if unsafe {
        (*node1).type_0 as u32 == XML_ELEMENT_NODE as u32
            && (*node2).type_0 as u32 == XML_ELEMENT_NODE as u32
            && 0 > (*node1).content as ptrdiff_t
            && 0 > (*node2).content as ptrdiff_t
            && (*node1).doc == (*node2).doc
    } {
        let mut l1: ptrdiff_t = 0;
        let mut l2: ptrdiff_t = 0;
        l1 = -(unsafe { (*node1).content as ptrdiff_t });
        l2 = -(unsafe { (*node2).content as ptrdiff_t });
        if l1 < l2 {
            return 1;
        }
        if l1 > l2 {
            return -1;
        }
    }
    /*
     * compute depth to root
     */
    depth2 = 0;
    cur = node2;
    while unsafe { !(*cur).parent.is_null() } {
        if unsafe { (*cur).parent == node1 } {
            return 1;
        }
        depth2 += 1;
        cur = unsafe { (*cur).parent }
    }
    root = cur;
    depth1 = 0;
    cur = node1;
    while unsafe { !(*cur).parent.is_null() } {
        if unsafe { (*cur).parent == node2 } {
            return -1;
        }
        depth1 += 1;
        cur = unsafe { (*cur).parent }
    }
    /*
     * Distinct document (or distinct entities :-( ) case.
     */
    if root != cur {
        return -2;
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
            return -2;
        }
    }
    /*
     * Find who's first.
     */
    if unsafe { node1 == (*node2).prev } {
        return 1;
    }
    if unsafe { node1 == (*node2).next } {
        return -1;
    }
    /*
     * Speedup using document order if available.
     */
    if unsafe {
        (*node1).type_0 as u32 == XML_ELEMENT_NODE as u32
            && (*node2).type_0 as u32 == XML_ELEMENT_NODE as u32
            && 0 as i64 > (*node1).content as ptrdiff_t
            && 0 as i64 > (*node2).content as ptrdiff_t
            && (*node1).doc == (*node2).doc
    } {
        let mut l1_0: ptrdiff_t = 0;
        let mut l2_0: ptrdiff_t = 0;
        l1_0 = -(unsafe { (*node1).content as ptrdiff_t });
        l2_0 = -(unsafe { (*node2).content as ptrdiff_t });
        if l1_0 < l2_0 {
            return 1;
        }
        if l1_0 > l2_0 {
            return -1;
        }
    }
    cur = unsafe { (*node1).next };
    while !cur.is_null() {
        if cur == node2 {
            return 1;
        }
        cur = unsafe { (*cur).next }
    }
    return -1;
    /* assume there is no sibling list corruption */
}
/* *
 * xmlXPathNodeSetSort: * @set: the node set
 *
 * Sort the node set in document order
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetSort(set: xmlNodeSetPtr) {
    // #ifndef WITH_TIM_SORT
    let safe_set = unsafe { &mut *set };
    match () {
        #[cfg(not(WITH_TIM_SORT))]
        _ => {
            let mut i: i32;
            let mut j: i32;
            let mut incr: i32;
            let mut len: i32;
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
            incr = len / 2;
            while incr > 0 {
                i = incr;
                while i < len {
                    j = i - incr;
                    while j >= 0 {
                        let mut XP_OPTIMIZED_NON_ELEM_COMPARISON_RES: i32 = 0;
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
                        if !(XP_OPTIMIZED_NON_ELEM_COMPARISON_RES == -1) {
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
                incr /= 2
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
fn xmlXPathNodeSetDupNs(node: xmlNodePtr, ns: xmlNsPtr) -> xmlNodePtr {
    let safe_node = unsafe { &mut *node };
    let safe_ns = unsafe { &mut *ns };
    let mut cur: xmlNsPtr = 0 as *mut xmlNs;
    let safe_cur = unsafe { &mut *cur };
    if ns.is_null() || safe_ns.type_0 as u32 != XML_NAMESPACE_DECL as u32 {
        return 0 as xmlNodePtr;
    }
    if node.is_null() || safe_node.type_0 as u32 == XML_NAMESPACE_DECL as u32 {
        return ns as xmlNodePtr;
    }
    /*
     * Allocate a new Namespace and fill the fields.
     */
    cur = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNs>() as u64)
            as xmlNsPtr
    };
    if cur.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"duplicating namespace\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlNodePtr;
    }
    unsafe {
        memset(cur as *mut (), 0, ::std::mem::size_of::<xmlNs>() as u64);
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
pub fn xmlXPathNodeSetFreeNs(ns: xmlNsPtr) {
    let safe_ns = unsafe { &mut *ns };
    if ns.is_null() || safe_ns.type_0 as u32 != XML_NAMESPACE_DECL as u32 {
        return;
    }
    if unsafe { !(*ns).next.is_null() && (*(*ns).next).type_0 as u32 != XML_NAMESPACE_DECL as u32 }
    {
        if !safe_ns.href.is_null() {
            unsafe {
                xmlFree.expect("non-null function pointer")((*ns).href as *mut xmlChar as *mut ())
            };
        }
        if !safe_ns.prefix.is_null() {
            unsafe {
                xmlFree.expect("non-null function pointer")((*ns).prefix as *mut xmlChar as *mut ())
            };
        }
        unsafe { xmlFree.expect("non-null function pointer")(ns as *mut ()) };
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
pub fn xmlXPathNodeSetCreate(val: xmlNodePtr) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNodeSet>() as u64)
            as xmlNodeSetPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating nodeset\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlNodeSetPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlNodeSet>() as u64,
        )
    };
    if !val.is_null() {
        let safe_val = unsafe { &mut *val };
        unsafe {
            (*ret).nodeTab = xmlMalloc.expect("non-null function pointer")(
                (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            ) as *mut xmlNodePtr
        };
        if unsafe { (*ret).nodeTab.is_null() } {
            unsafe {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"creating nodeset\n\x00" as *const u8 as *const i8,
                );
                xmlFree.expect("non-null function pointer")(ret as *mut ());
            };
            return 0 as xmlNodeSetPtr;
        }
        unsafe {
            memset(
                (*ret).nodeTab as *mut (),
                0,
                (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            );
            (*ret).nodeMax = 10;
        };
        if safe_val.type_0 as u32 == XML_NAMESPACE_DECL as u32 {
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
pub fn xmlXPathNodeSetContains(cur: xmlNodeSetPtr, val: xmlNodePtr) -> i32 {
    let mut i: i32;
    if cur.is_null() || val.is_null() {
        return 0;
    }
    let safe_val = unsafe { &mut *val };
    let safe_cur = unsafe { &mut *cur };
    if safe_val.type_0 as u32 == XML_NAMESPACE_DECL as u32 {
        i = 0;
        while i < safe_cur.nodeNr {
            if unsafe {
                (**(*cur).nodeTab.offset(i as isize)).type_0 as u32 == XML_NAMESPACE_DECL as u32
            } {
                let mut ns1: xmlNsPtr = 0 as *mut xmlNs;
                let mut ns2: xmlNsPtr = 0 as *mut xmlNs;
                ns1 = val as xmlNsPtr;
                ns2 = unsafe { *(*cur).nodeTab.offset(i as isize) as xmlNsPtr };
                if ns1 == ns2 {
                    return 1;
                }
                let safe_ns1 = unsafe { &mut *ns1 };
                let safe_ns2 = unsafe { &mut *ns2 };
                if !safe_ns1.next.is_null()
                    && safe_ns2.next == safe_ns1.next
                    && unsafe { xmlStrEqual((*ns1).prefix, (*ns2).prefix) } != 0
                {
                    return 1;
                }
            }
            i += 1
        }
    } else {
        i = 0;
        while i < safe_cur.nodeNr {
            if unsafe { *(*cur).nodeTab.offset(i as isize) == val } {
                return 1;
            }
            i += 1
        }
    }
    return 0;
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
pub fn xmlXPathNodeSetAddNs(cur: xmlNodeSetPtr, node: xmlNodePtr, ns: xmlNsPtr) -> i32 {
    let mut i: i32 = 0;
    let safe_ns = unsafe { &mut *ns };
    let safe_node = unsafe { &mut *node };
    let safe_cur = unsafe { &mut *cur };
    if cur.is_null()
        || ns.is_null()
        || node.is_null()
        || safe_ns.type_0 as u32 != XML_NAMESPACE_DECL as u32
        || safe_node.type_0 as u32 != XML_ELEMENT_NODE as u32
    {
        return -1;
    }
    /* @@ with_ns to check whether namespace nodes should be looked at @@ */
    /*
     * prevent duplicates
     */
    i = 0;
    while i < safe_cur.nodeNr {
        if unsafe {
            !(*(*cur).nodeTab.offset(i as isize)).is_null()
                && (**(*cur).nodeTab.offset(i as isize)).type_0 as u32 == XML_NAMESPACE_DECL as u32
                && (*(*(*cur).nodeTab.offset(i as isize) as xmlNsPtr)).next == node as xmlNsPtr
                && xmlStrEqual(
                    (*ns).prefix,
                    (*(*(*cur).nodeTab.offset(i as isize) as xmlNsPtr)).prefix,
                ) != 0
        } {
            return 0;
        }
        i += 1
    }
    /*
     * grow the nodeTab if needed
     */
    if safe_cur.nodeMax == 0 {
        unsafe {
            (*cur).nodeTab = xmlMalloc.expect("non-null function pointer")(
                (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            ) as *mut xmlNodePtr
        };
        if unsafe { (*cur).nodeTab.is_null() } {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        unsafe {
            memset(
                (*cur).nodeTab as *mut (),
                0,
                (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            )
        };
        unsafe { (*cur).nodeMax = 10 }
    } else if safe_cur.nodeNr == safe_cur.nodeMax {
        let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        if safe_cur.nodeMax >= 10000000 {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset hit limit\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        temp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*cur).nodeTab as *mut (),
                (((*cur).nodeMax * 2 as i32) as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            ) as *mut xmlNodePtr
        };
        if temp.is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        unsafe {
            (*cur).nodeMax *= 2;
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
    return 0;
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
pub fn xmlXPathNodeSetAdd(cur: xmlNodeSetPtr, val: xmlNodePtr) -> i32 {
    let mut i: i32;
    if cur.is_null() || val.is_null() {
        return -1;
    }
    /* @@ with_ns to check whether namespace nodes should be looked at @@ */
    /*
     * prevent duplicates
     */
    i = 0 as i32;
    let safe_cur = unsafe { &mut *cur };
    let safe_val = unsafe { &mut *val };
    while i < safe_cur.nodeNr {
        if unsafe { *(*cur).nodeTab.offset(i as isize) == val } {
            return 0;
        }
        i += 1
    }
    /*
     * grow the nodeTab if needed
     */
    if safe_cur.nodeMax == 0 {
        unsafe {
            (*cur).nodeTab = xmlMalloc.expect("non-null function pointer")(
                (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            ) as *mut xmlNodePtr
        };
        if safe_cur.nodeTab.is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        unsafe {
            memset(
                (*cur).nodeTab as *mut (),
                0,
                (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            )
        };
        unsafe { (*cur).nodeMax = 10 }
    } else if safe_cur.nodeNr == safe_cur.nodeMax {
        let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        if safe_cur.nodeMax >= 10000000 {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset hit limit\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        temp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*cur).nodeTab as *mut (),
                (((*cur).nodeMax * 2) as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            ) as *mut xmlNodePtr
        };
        if temp.is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        unsafe {
            (*cur).nodeMax *= 2;
            (*cur).nodeTab = temp
        }
    }
    if safe_val.type_0 as u32 == XML_NAMESPACE_DECL as u32 {
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
    return 0;
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
pub extern "C" fn xmlXPathNodeSetAddUnique(cur: xmlNodeSetPtr, val: xmlNodePtr) -> i32 {
    if cur.is_null() || val.is_null() {
        return -1;
    }
    /* @@ with_ns to check whether namespace nodes should be looked at @@ */
    /*
     * grow the nodeTab if needed
     */
    let safe_cur = unsafe { &mut *cur };
    let safe_val = unsafe { &mut *val };
    if safe_cur.nodeMax == 0 {
        unsafe {
            (*cur).nodeTab = xmlMalloc.expect("non-null function pointer")(
                (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            ) as *mut xmlNodePtr
        };
        if unsafe { (*cur).nodeTab.is_null() } {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        unsafe {
            memset(
                (*cur).nodeTab as *mut (),
                0,
                (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            )
        };
        unsafe { (*cur).nodeMax = 10 }
    } else if safe_cur.nodeNr == safe_cur.nodeMax {
        let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        if safe_cur.nodeMax >= 10000000 {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset hit limit\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        temp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*cur).nodeTab as *mut (),
                (((*cur).nodeMax * 2) as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            ) as *mut xmlNodePtr
        };
        if temp.is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\x00" as *const u8 as *const i8,
            );
            return -1;
        }
        unsafe {
            (*cur).nodeTab = temp;
            (*cur).nodeMax *= 2
        }
    }
    if safe_val.type_0 as u32 == XML_NAMESPACE_DECL as u32 {
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
    return 0;
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
pub fn xmlXPathNodeSetMerge(mut val1: xmlNodeSetPtr, val2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let mut i: i32;
    let mut j: i32;
    let mut initNr: i32;
    let mut skip: i32;
    let mut n1: xmlNodePtr = 0 as *mut xmlNode;
    let mut n2: xmlNodePtr = 0 as *mut xmlNode;
    if val2.is_null() {
        return val1;
    }
    if val1.is_null() {
        val1 = xmlXPathNodeSetCreate(0 as xmlNodePtr);
        if val1.is_null() {
            return 0 as xmlNodeSetPtr;
        }
    }
    /* @@ with_ns to check whether namespace nodes should be looked at @@ */
    let safe_val1 = unsafe { &mut *val1 };
    let safe_val2 = unsafe { &mut *val2 };
    initNr = safe_val1.nodeNr;
    i = 0;
    while i < safe_val2.nodeNr {
        n2 = unsafe { *(*val2).nodeTab.offset(i as isize) };
        /*
         * check against duplicates
         */
        skip = 0;
        j = 0;
        while j < initNr {
            n1 = unsafe { *(*val1).nodeTab.offset(j as isize) };
            let safe_n1 = unsafe { &mut *n1 };
            let safe_n2 = unsafe { &mut *n2 };
            if n1 == n2 {
                skip = 1;
                break;
            } else {
                if safe_n1.type_0 as u32 == XML_NAMESPACE_DECL as u32
                    && safe_n2.type_0 as u32 == XML_NAMESPACE_DECL as u32
                {
                    if unsafe {
                        (*(n1 as xmlNsPtr)).next == (*(n2 as xmlNsPtr)).next
                            && xmlStrEqual((*(n1 as xmlNsPtr)).prefix, (*(n2 as xmlNsPtr)).prefix)
                                != 0
                    } {
                        skip = 1;
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
            if safe_val1.nodeMax == 0 {
                unsafe {
                    (*val1).nodeTab = xmlMalloc.expect("non-null function pointer")(
                        (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
                    ) as *mut xmlNodePtr
                };
                if unsafe { (*val1).nodeTab.is_null() } {
                    xmlXPathErrMemory(
                        0 as xmlXPathContextPtr,
                        b"merging nodeset\n\x00" as *const u8 as *const i8,
                    );
                    return 0 as xmlNodeSetPtr;
                }
                unsafe {
                    memset(
                        (*val1).nodeTab as *mut (),
                        0,
                        (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
                    )
                };
                unsafe { (*val1).nodeMax = 10 }
            } else if safe_val1.nodeNr == safe_val1.nodeMax {
                let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
                if safe_val1.nodeMax >= 10000000 as i32 {
                    xmlXPathErrMemory(
                        0 as xmlXPathContextPtr,
                        b"merging nodeset hit limit\n\x00" as *const u8 as *const i8,
                    );
                    return 0 as xmlNodeSetPtr;
                }
                temp = unsafe {
                    xmlRealloc.expect("non-null function pointer")(
                        (*val1).nodeTab as *mut (),
                        (((*val1).nodeMax * 2) as u64)
                            .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
                    ) as *mut xmlNodePtr
                };
                if temp.is_null() {
                    xmlXPathErrMemory(
                        0 as xmlXPathContextPtr,
                        b"merging nodeset\n\x00" as *const u8 as *const i8,
                    );
                    return 0 as xmlNodeSetPtr;
                }
                unsafe {
                    (*val1).nodeTab = temp;
                    (*val1).nodeMax *= 2
                }
            }
            let safe_n1 = unsafe { &mut *n1 };
            let safe_n2 = unsafe { &mut *n2 };
            if safe_n2.type_0 as u32 == XML_NAMESPACE_DECL as u32 {
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
    set1: xmlNodeSetPtr,
    set2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let mut current_block: u64;
    let mut i: i32;
    let mut j: i32;
    let mut initNbSet1: i32 = 0;
    let mut n1: xmlNodePtr = 0 as *mut xmlNode;
    let mut n2: xmlNodePtr = 0 as *mut xmlNode;
    let safe_set1 = unsafe { &mut *set1 };
    let safe_set2 = unsafe { &mut *set2 };
    initNbSet1 = safe_set1.nodeNr;
    i = 0;
    while i < safe_set2.nodeNr {
        n2 = unsafe { *(*set2).nodeTab.offset(i as isize) };
        /*
         * Skip duplicates.
         */
        j = 0;
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
            if safe_n1.type_0 as u32 == XML_NAMESPACE_DECL as i32 as u32
                && safe_n2.type_0 as u32 == XML_NAMESPACE_DECL as i32 as u32
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
                if safe_set1.nodeMax == 0 as i32 {
                    unsafe {
                        (*set1).nodeTab = xmlMalloc.expect("non-null function pointer")(
                            (10 as i32 as u64)
                                .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
                        ) as *mut xmlNodePtr
                    };
                    if unsafe { (*set1).nodeTab.is_null() } {
                        xmlXPathErrMemory(
                            0 as xmlXPathContextPtr,
                            b"merging nodeset\n\x00" as *const u8 as *const i8,
                        );
                        return 0 as xmlNodeSetPtr;
                    }
                    unsafe {
                        memset(
                            (*set1).nodeTab as *mut (),
                            0 as i32,
                            (10 as i32 as u64)
                                .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
                        )
                    };
                    unsafe { (*set1).nodeMax = 10 as i32 }
                } else if safe_set1.nodeNr >= safe_set1.nodeMax {
                    let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
                    if safe_set1.nodeMax >= 10000000 as i32 {
                        xmlXPathErrMemory(
                            0 as xmlXPathContextPtr,
                            b"merging nodeset hit limit\n\x00" as *const u8 as *const i8,
                        );
                        return 0 as xmlNodeSetPtr;
                    }
                    temp = unsafe {
                        xmlRealloc.expect("non-null function pointer")(
                            (*set1).nodeTab as *mut (),
                            (((*set1).nodeMax * 2 as i32) as u64)
                                .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
                        ) as *mut xmlNodePtr
                    };
                    if temp.is_null() {
                        xmlXPathErrMemory(
                            0 as xmlXPathContextPtr,
                            b"merging nodeset\n\x00" as *const u8 as *const i8,
                        );
                        return 0 as xmlNodeSetPtr;
                    }
                    unsafe {
                        (*set1).nodeTab = temp;
                        (*set1).nodeMax *= 2 as i32
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
    unsafe { (*set2).nodeNr = 0 as i32 };
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
    set1: xmlNodeSetPtr,
    set2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let mut i: i32;
    let mut n2: xmlNodePtr = 0 as *mut xmlNode;
    let safe_set1 = unsafe { &mut *set1 };
    let safe_set2 = unsafe { &mut *set2 };
    i = 0;
    while i < safe_set2.nodeNr {
        n2 = unsafe { *(*set2).nodeTab.offset(i as isize) };
        if safe_set1.nodeMax == 0 {
            unsafe {
                (*set1).nodeTab = xmlMalloc.expect("non-null function pointer")(
                    (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
                ) as *mut xmlNodePtr
            };
            if unsafe { (*set1).nodeTab.is_null() } {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"merging nodeset\n\x00" as *const u8 as *const i8,
                );
                return 0 as xmlNodeSetPtr;
            }
            unsafe {
                memset(
                    (*set1).nodeTab as *mut (),
                    0,
                    (10 as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
                )
            };
            unsafe { (*set1).nodeMax = 10 }
        } else if safe_set1.nodeNr >= safe_set1.nodeMax {
            let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
            if safe_set1.nodeMax >= 10000000 as i32 {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"merging nodeset hit limit\n\x00" as *const u8 as *const i8,
                );
                return 0 as xmlNodeSetPtr;
            }
            unsafe {
                temp = xmlRealloc.expect("non-null function pointer")(
                    (*set1).nodeTab as *mut (),
                    (((*set1).nodeMax * 2 as i32) as u64)
                        .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
                ) as *mut xmlNodePtr
            };
            if temp.is_null() {
                xmlXPathErrMemory(
                    0 as xmlXPathContextPtr,
                    b"merging nodeset\n\x00" as *const u8 as *const i8,
                );
                return 0 as xmlNodeSetPtr;
            }
            unsafe {
                (*set1).nodeTab = temp;
                (*set1).nodeMax *= 2
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
    unsafe { (*set2).nodeNr = 0 };
    return set1;
}
/* *
 * xmlXPathNodeSetDel: * @cur: the initial node set
 * @val: an xmlNodePtr
 *
 * Removes an xmlNodePtr from an existing NodeSet
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathNodeSetDel(cur: xmlNodeSetPtr, val: xmlNodePtr) {
    let mut i: i32;
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
    i = 0;
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
                            as *const i8,
                        (*val).name as u32,
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
            && (**(*cur).nodeTab.offset(i as isize)).type_0 as u32 == XML_NAMESPACE_DECL as u32
    } {
        unsafe { xmlXPathNodeSetFreeNs(*(*cur).nodeTab.offset(i as isize) as xmlNsPtr) };
    }
    unsafe { (*cur).nodeNr -= 1 };
    while unsafe { i < (*cur).nodeNr } {
        unsafe {
            let ref mut fresh63 = *(*cur).nodeTab.offset(i as isize);
            *fresh63 = *(*cur).nodeTab.offset((i + 1) as isize);
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
pub fn xmlXPathNodeSetRemove(cur: xmlNodeSetPtr, mut val: i32) {
    if cur.is_null() {
        return;
    }
    let safe_cur = unsafe { &mut *cur };
    if val >= safe_cur.nodeNr {
        return;
    }
    if unsafe {
        !(*(*cur).nodeTab.offset(val as isize)).is_null()
            && (**(*cur).nodeTab.offset(val as isize)).type_0 as u32 == XML_NAMESPACE_DECL as u32
    } {
        unsafe { xmlXPathNodeSetFreeNs(*(*cur).nodeTab.offset(val as isize) as xmlNsPtr) };
    }
    unsafe { (*cur).nodeNr -= 1 };
    while unsafe { val < (*cur).nodeNr } {
        unsafe {
            let ref mut fresh65 = *(*cur).nodeTab.offset(val as isize);
            *fresh65 = *(*cur).nodeTab.offset((val + 1) as isize);
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
pub unsafe fn xmlXPathFreeNodeSet(obj: xmlNodeSetPtr) {
    if obj.is_null() {
        return;
    }
    let safe_obj = unsafe { &mut *obj };
    if !safe_obj.nodeTab.is_null() {
        let mut i: i32;
        /* @@ with_ns to check whether namespace nodes should be looked at @@ */
        i = 0;
        while i < safe_obj.nodeNr {
            if unsafe {
                !(*(*obj).nodeTab.offset(i as isize)).is_null()
                    && (**(*obj).nodeTab.offset(i as isize)).type_0 as u32
                        == XML_NAMESPACE_DECL as u32
            } {
                unsafe { xmlXPathNodeSetFreeNs(*(*obj).nodeTab.offset(i as isize) as xmlNsPtr) };
            }
            i += 1
        }
        unsafe { xmlFree.expect("non-null function pointer")((*obj).nodeTab as *mut ()) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(obj as *mut ()) };
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
fn xmlXPathNodeSetClearFromPos(set: xmlNodeSetPtr, pos: i32, hasNsNodes: i32) {
    let safe_set = unsafe { &mut *set };
    if set.is_null() || pos >= safe_set.nodeNr {
        return;
    } else {
        if hasNsNodes != 0 {
            let mut i: i32;
            let mut node: xmlNodePtr = 0 as *mut xmlNode;
            i = pos;
            while i < safe_set.nodeNr {
                node = unsafe { *(*set).nodeTab.offset(i as isize) };
                if unsafe { !node.is_null() && (*node).type_0 as u32 == XML_NAMESPACE_DECL as u32 }
                {
                    xmlXPathNodeSetFreeNs(node as xmlNsPtr);
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
fn xmlXPathNodeSetClear(set: xmlNodeSetPtr, hasNsNodes: i32) {
    xmlXPathNodeSetClearFromPos(set, 0, hasNsNodes);
}
/* *
 * xmlXPathNodeSetKeepLast: * @set: the node set to be cleared
 *
 * Move the last node to the first position and clear temporary XPath objects
 * (e.g. namespace nodes) from all other nodes. Sets the length of the list
 * to 1.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
unsafe fn xmlXPathNodeSetKeepLast(set: xmlNodeSetPtr) {
    let mut i: i32;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let safe_set = unsafe { &mut *set };
    if set.is_null() || safe_set.nodeNr <= 1 {
        return;
    }
    i = 0;
    while i < safe_set.nodeNr - 1 {
        node = unsafe { *(*set).nodeTab.offset(i as isize) };
        if !node.is_null() && unsafe { (*node).type_0 as u32 == XML_NAMESPACE_DECL as u32 } {
            xmlXPathNodeSetFreeNs(node as xmlNsPtr);
        }
        i += 1
    }
    unsafe {
        let ref mut fresh67 = *(*set).nodeTab.offset(0 as isize);
        *fresh67 = *(*set).nodeTab.offset(((*set).nodeNr - 1) as isize);
        (*set).nodeNr = 1;
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
fn xmlXPathFreeValueTree(obj: xmlNodeSetPtr) {
    let mut i: i32;
    if obj.is_null() {
        return;
    }
    let safe_obj = unsafe { &mut *obj };
    if !safe_obj.nodeTab.is_null() {
        i = 0;
        while i < safe_obj.nodeNr {
            if unsafe { !(*(*obj).nodeTab.offset(i as isize)).is_null() } {
                if unsafe {
                    (**(*obj).nodeTab.offset(i as isize)).type_0 as u32 == XML_NAMESPACE_DECL as u32
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
        unsafe { xmlFree.expect("non-null function pointer")((*obj).nodeTab as *mut ()) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(obj as *mut ()) };
}

/* *
 * xmlGenericErrorContextNodeSet: * @output: a FILE * for the output
 * @obj: the xmlNodeSetPtr to display
 *
 * Quick display of a NodeSet
 */
#[cfg(DEBUG_OR_DEBUG_STEP)]
#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlGenericErrorContextNodeSet(output: *mut FILE, obj: xmlNodeSetPtr) {
    let mut i: i32;
    let safe_obj = unsafe { &mut *obj };
    if output.is_null() {
        output = unsafe { *__xmlGenericErrorContext() as *mut FILE }
    }
    if obj.is_null() {
        unsafe { fprintf(output, b"NodeSet == NULL !\n\x00" as *const u8 as *const i8) };
        return;
    }
    if safe_obj.nodeNr == 0 {
        unsafe { fprintf(output, b"NodeSet is empty\n\x00" as *const u8 as *const i8) };
        return;
    }
    if safe_obj.nodeTab.is_null() {
        unsafe {
            fprintf(
                output,
                b" nodeTab == NULL !\n\x00" as *const u8 as *const i8,
            )
        };
        return;
    }
    i = 0;
    while i < safe_obj.nodeNr {
        if unsafe { (*(*obj).nodeTab.offset(i as isize)).is_null() } {
            unsafe { fprintf(output, b" NULL !\n\x00" as *const u8 as *const i8) };
            return;
        }
        if unsafe {
            (**(*obj).nodeTab.offset(i as isize)).type_0 as u32 == XML_DOCUMENT_NODE as u32
                || (**(*obj).nodeTab.offset(i as isize)).type_0 as u32
                    == XML_HTML_DOCUMENT_NODE as u32
        } {
            unsafe { fprintf(output, b" /\x00" as *const u8 as *const i8) };
        } else if unsafe { (**(*obj).nodeTab.offset(i as isize)).name.is_null() } {
            unsafe { fprintf(output, b" noname!\x00" as *const u8 as *const i8) };
        } else {
            unsafe {
                fprintf(
                    output,
                    b" %s\x00" as *const u8 as *const i8,
                    (**(*obj).nodeTab.offset(i as isize)).name,
                )
            };
        }
        i += 1
    }
    unsafe { fprintf(output, b"\n\x00" as *const u8 as *const i8) };
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
pub fn xmlXPathNewNodeSet(val: xmlNodePtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>() as u64)
            as xmlXPathObjectPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating nodeset\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathObject>() as u64,
        );
        (*ret).type_0 = XPATH_NODESET;
        (*ret).boolval = 0;
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
pub unsafe fn xmlXPathNewValueTree(val: xmlNodePtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>() as u64)
            as xmlXPathObjectPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating result value tree\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathObject>() as u64,
        );
        (*ret).type_0 = XPATH_XSLT_TREE;
        (*ret).boolval = 1;
        (*ret).user = val as *mut ();
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
pub unsafe fn xmlXPathNewNodeSetList(val: xmlNodeSetPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut i: i32;
    let safe_val = unsafe { &mut *val };
    if val.is_null() {
        ret = 0 as xmlXPathObjectPtr
    } else if safe_val.nodeTab.is_null() {
        ret = xmlXPathNewNodeSet(0 as xmlNodePtr)
    } else {
        ret = unsafe { xmlXPathNewNodeSet(*(*val).nodeTab.offset(0 as isize)) };
        if !ret.is_null() {
            i = 1;
            while i < safe_val.nodeNr {
                /* TODO: Propagate memory error. */
                if unsafe {
                    xmlXPathNodeSetAddUnique((*ret).nodesetval, *(*val).nodeTab.offset(i as isize))
                        < 0
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
pub fn xmlXPathWrapNodeSet(val: xmlNodeSetPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>() as u64)
            as xmlXPathObjectPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating node set object\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathObject>() as u64,
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
pub fn xmlXPathFreeNodeSetList(obj: xmlXPathObjectPtr) {
    if obj.is_null() {
        return;
    }
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe {
                xmlXPathDebugObjUsageRequested(0 as xmlXPathContextPtr, (*obj).type_0 as u32)
            };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    unsafe { xmlFree.expect("non-null function pointer")(obj as *mut ()) };
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
pub fn xmlXPathDifference(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut i: i32;
    let l1: i32;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 || safe_nodes2.nodeTab.is_null() {
        return nodes1;
    }
    /* TODO: Check memory error. */
    ret = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    if nodes1.is_null() || safe_nodes1.nodeNr == 0 || safe_nodes1.nodeTab.is_null() {
        return ret;
    }
    l1 = if !nodes1.is_null() {
        safe_nodes1.nodeNr
    } else {
        0
    };
    i = 0;
    while i < l1 {
        cur = if !nodes1.is_null() && i >= 0 && i < safe_nodes1.nodeNr {
            unsafe { *(*nodes1).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        if xmlXPathNodeSetContains(nodes2, cur) == 0 {
            /* TODO: Propagate memory error. */
            if xmlXPathNodeSetAddUnique(ret, cur) < 0 {
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
pub fn xmlXPathIntersection(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    let mut i: i32;
    let l1: i32;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if ret.is_null() {
        return ret;
    }
    if nodes1.is_null() || safe_nodes1.nodeNr == 0 || safe_nodes1.nodeTab.is_null() {
        return ret;
    }
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 || safe_nodes2.nodeTab.is_null() {
        return ret;
    }
    l1 = if !nodes1.is_null() {
        safe_nodes1.nodeNr
    } else {
        0
    };
    i = 0;
    while i < l1 {
        cur = if !nodes1.is_null() && i >= 0 && i < safe_nodes1.nodeNr {
            unsafe { *(*nodes1).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        if xmlXPathNodeSetContains(nodes2, cur) != 0 {
            /* TODO: Propagate memory error. */
            if xmlXPathNodeSetAddUnique(ret, cur) < 0 {
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
pub fn xmlXPathDistinctSorted(nodes: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut hash: xmlHashTablePtr = 0 as *mut xmlHashTable;
    let mut i: i32;
    let mut l: i32 = 0;
    let mut strval: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let safe_nodes = unsafe { &mut *nodes };
    if nodes.is_null() || safe_nodes.nodeNr == 0 || safe_nodes.nodeTab.is_null() {
        return nodes;
    }
    ret = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    if ret.is_null() {
        return ret;
    }
    l = if !nodes.is_null() {
        safe_nodes.nodeNr
    } else {
        0
    };
    hash = unsafe { xmlHashCreate(l) };
    i = 0;
    while i < l {
        cur = if !nodes.is_null() && i >= 0 && i < safe_nodes.nodeNr {
            unsafe { *(*nodes).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        strval = xmlXPathCastNodeToString(cur);
        if unsafe { xmlHashLookup(hash, strval).is_null() } {
            unsafe { xmlHashAddEntry(hash, strval, strval as *mut ()) };
            /* TODO: Propagate memory error. */
            if xmlXPathNodeSetAddUnique(ret, cur) < 0 {
                break;
            }
        } else {
            unsafe { xmlFree.expect("non-null function pointer")(strval as *mut ()) };
        }
        i += 1
    }
    unsafe {
        xmlHashFree(
            hash,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
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
pub fn xmlXPathDistinct(nodes: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let safe_nodes = unsafe { &mut *nodes };
    if nodes.is_null() || safe_nodes.nodeNr == 0 || safe_nodes.nodeTab.is_null() {
        return nodes;
    }
    xmlXPathNodeSetSort(nodes);
    return xmlXPathDistinctSorted(nodes);
}
/* *
 * xmlXPathHasSameNodes: * @nodes1: a node-set
 * @nodes2: a node-set
 *
 * Implements the EXSLT - Sets has-same-nodes function: *    boolean set:has-same-node(node-set, node-set) *
 * Returns true (1) if @nodes1 shares any node with @nodes2, false (0) *         otherwise
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathHasSameNodes(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> i32 {
    let mut i: i32;
    let mut l: i32 = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes1.is_null()
        || safe_nodes1.nodeNr == 0
        || safe_nodes1.nodeTab.is_null()
        || (nodes2.is_null() || safe_nodes2.nodeNr == 0 || safe_nodes2.nodeTab.is_null())
    {
        return 0;
    }
    l = if !nodes1.is_null() {
        safe_nodes1.nodeNr
    } else {
        0
    };
    i = 0;
    while i < l {
        cur = if !nodes1.is_null() && i >= 0 && i < safe_nodes1.nodeNr {
            unsafe { *(*nodes1).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        if xmlXPathNodeSetContains(nodes2, cur) != 0 {
            return 1;
        }
        i += 1
    }
    return 0;
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
pub fn xmlXPathNodeLeadingSorted(nodes: xmlNodeSetPtr, node: xmlNodePtr) -> xmlNodeSetPtr {
    let mut i: i32;
    let mut l: i32 = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if node.is_null() {
        return nodes;
    }
    let safe_nodes = unsafe { &mut *nodes };
    ret = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    if ret.is_null() {
        return ret;
    }
    if nodes.is_null()
        || safe_nodes.nodeNr == 0
        || safe_nodes.nodeTab.is_null()
        || xmlXPathNodeSetContains(nodes, node) == 0
    {
        return ret;
    }
    l = if !nodes.is_null() {
        safe_nodes.nodeNr
    } else {
        0
    };
    i = 0;
    while i < l {
        cur = if !nodes.is_null() && i >= 0 && i < safe_nodes.nodeNr {
            unsafe { *(*nodes).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        if cur == node {
            break;
        }
        /* TODO: Propagate memory error. */
        if xmlXPathNodeSetAddUnique(ret, cur) < 0 {
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
pub fn xmlXPathNodeLeading(nodes: xmlNodeSetPtr, node: xmlNodePtr) -> xmlNodeSetPtr {
    xmlXPathNodeSetSort(nodes);
    return xmlXPathNodeLeadingSorted(nodes, node);
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
pub fn xmlXPathLeadingSorted(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 as i32 || safe_nodes2.nodeTab.is_null() {
        return nodes1;
    }
    return unsafe {
        xmlXPathNodeLeadingSorted(
            nodes1,
            if !nodes2.is_null() && 1 >= 0 && (1) < (*nodes2).nodeNr {
                *(*nodes2).nodeTab.offset(1 as isize)
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
pub fn xmlXPathLeading(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 || safe_nodes2.nodeTab.is_null() {
        return nodes1;
    }
    if nodes1.is_null() || safe_nodes1.nodeNr == 0 || safe_nodes1.nodeTab.is_null() {
        return xmlXPathNodeSetCreate(0 as xmlNodePtr);
    }
    xmlXPathNodeSetSort(nodes1);
    xmlXPathNodeSetSort(nodes2);
    return unsafe {
        xmlXPathNodeLeadingSorted(
            nodes1,
            if !nodes2.is_null() && 1 >= 0 && (1) < (*nodes2).nodeNr {
                *(*nodes2).nodeTab.offset(1 as isize)
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
pub fn xmlXPathNodeTrailingSorted(nodes: xmlNodeSetPtr, node: xmlNodePtr) -> xmlNodeSetPtr {
    let mut i: i32;
    let mut l: i32 = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if node.is_null() {
        return nodes;
    }
    let safe_nodes = unsafe { &mut *nodes };
    ret = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    if ret.is_null() {
        return ret;
    }
    if nodes.is_null()
        || safe_nodes.nodeNr == 0
        || safe_nodes.nodeTab.is_null()
        || xmlXPathNodeSetContains(nodes, node) == 0
    {
        return ret;
    }
    l = if !nodes.is_null() {
        safe_nodes.nodeNr
    } else {
        0
    };
    i = l - 1;
    while i >= 0 {
        cur = if !nodes.is_null() && i >= 0 && i < safe_nodes.nodeNr {
            unsafe { *(*nodes).nodeTab.offset(i as isize) }
        } else {
            0 as xmlNodePtr
        };
        if cur == node {
            break;
        }
        /* TODO: Propagate memory error. */
        if xmlXPathNodeSetAddUnique(ret, cur) < 0 {
            break; /* bug 413451 */
        }
        i -= 1
    }
    xmlXPathNodeSetSort(ret);
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
pub fn xmlXPathNodeTrailing(nodes: xmlNodeSetPtr, node: xmlNodePtr) -> xmlNodeSetPtr {
    xmlXPathNodeSetSort(nodes);
    return xmlXPathNodeTrailingSorted(nodes, node);
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
pub fn xmlXPathTrailingSorted(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 || safe_nodes2.nodeTab.is_null() {
        return nodes1;
    }
    return unsafe {
        xmlXPathNodeTrailingSorted(
            nodes1,
            if !nodes2.is_null() && 2 > 1 && (0) < (*nodes2).nodeNr {
                *(*nodes2).nodeTab.offset(0 as isize)
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
pub unsafe fn xmlXPathTrailing(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr {
    let safe_nodes1 = unsafe { &mut *nodes1 };
    let safe_nodes2 = unsafe { &mut *nodes2 };
    if nodes2.is_null() || safe_nodes2.nodeNr == 0 || safe_nodes2.nodeTab.is_null() {
        return nodes1;
    }
    if nodes1.is_null() || safe_nodes1.nodeNr == 0 || safe_nodes1.nodeTab.is_null() {
        return xmlXPathNodeSetCreate(0 as xmlNodePtr);
    }
    xmlXPathNodeSetSort(nodes1);
    xmlXPathNodeSetSort(nodes2);
    return unsafe {
        xmlXPathNodeTrailingSorted(
            nodes1,
            if !nodes2.is_null() && 2 > 1 && (0) < (*nodes2).nodeNr {
                *(*nodes2).nodeTab.offset(0 as isize)
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
    ctxt: xmlXPathContextPtr,
    name: *const xmlChar,
    f: xmlXPathFunction,
) -> i32 {
    return xmlXPathRegisterFuncNS(ctxt, name, 0 as *const xmlChar, f);
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
    ctxt: xmlXPathContextPtr,
    name: *const xmlChar,
    ns_uri: *const xmlChar,
    f: xmlXPathFunction,
) -> i32 {
    if ctxt.is_null() {
        return -1;
    }
    if name.is_null() {
        return -1;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.funcHash.is_null() {
        unsafe { (*ctxt).funcHash = xmlHashCreate(0) }
    }
    if unsafe { (*ctxt).funcHash.is_null() } {
        return -1;
    }
    if f.is_none() {
        return unsafe { xmlHashRemoveEntry2((*ctxt).funcHash, name, ns_uri, None) };
    }
    return unsafe {
        xmlHashAddEntry2(
            (*ctxt).funcHash,
            name,
            ns_uri,
            ::std::mem::transmute::<xmlXPathFunction, *mut ()>(f),
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
    ctxt: xmlXPathContextPtr,
    f: xmlXPathFuncLookupFunc,
    funcCtxt: *mut (),
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
pub fn xmlXPathFunctionLookup(ctxt: xmlXPathContextPtr, name: *const xmlChar) -> xmlXPathFunction {
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
    return xmlXPathFunctionLookupNS(ctxt, name, 0 as *const xmlChar);
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
    ctxt: xmlXPathContextPtr,
    name: *const xmlChar,
    ns_uri: *const xmlChar,
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
        ::std::mem::transmute::<*mut (), xmlXPathFunction>(xmlHashLookup2(
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
pub extern "C" fn xmlXPathDebugDumpStepAxis(op: xmlXPathStepOpPtr, nbNodes: i32) {
    let safe_op = unsafe { &mut *op };
    unsafe {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"new step : \x00" as *const u8 as *const i8,
        )
    };
    match safe_op.value {
        AXIS_ANCESTOR => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'ancestors\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_ANCESTOR_OR_SELF => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'ancestors-or-self\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_ATTRIBUTE => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'attributes\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_CHILD => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'child\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_DESCENDANT => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'descendant\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_DESCENDANT_OR_SELF => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'descendant-or-self\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_FOLLOWING => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'following\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_FOLLOWING_SIBLING => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'following-siblings\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_NAMESPACE => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'namespace\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_PARENT => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'parent\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_PRECEDING => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'preceding\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_PRECEDING_SIBLING => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'preceding-sibling\' \x00" as *const u8 as *const i8,
                )
            };
        }
        AXIS_SELF => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"axis \'self\' \x00" as *const u8 as *const i8,
                )
            };
        }
        _ => {}
    }
    unsafe {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b" context contains %d nodes\n\x00" as *const u8 as *const i8,
            nbNodes,
        )
    };
    match safe_op.value2 {
        NODE_TEST_NONE => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for none !!!\n\x00" as *const u8 as *const i8,
                )
            };
        }
        NODE_TEST_TYPE => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for type %d\n\x00" as *const u8 as *const i8,
                    (*op).value3,
                )
            };
        }
        NODE_TEST_PI => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for PI !!!\n\x00" as *const u8 as *const i8,
                )
            };
        }
        NODE_TEST_ALL => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for *\n\x00" as *const u8 as *const i8,
                )
            };
        }
        NODE_TEST_NS => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for namespace %s\n\x00" as *const u8 as *const i8,
                    (*op).value5,
                )
            };
        }
        NODE_TEST_NAME => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"           searching for name %s\n\x00" as *const u8 as *const i8,
                    (*op).value5,
                )
            };
            if !safe_op.value4.is_null() {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"           with namespace %s\n\x00" as *const u8 as *const i8,
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
            b"Testing : \x00" as *const u8 as *const i8,
        )
    };
}

/* *
 * xmlXPathRegisteredFuncsCleanup: * @ctxt: the XPath context
 *
 * Cleanup the XPath context data associated to registered functions
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathRegisteredFuncsCleanup(ctxt: xmlXPathContextPtr) {
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
    ctxt: xmlXPathContextPtr,
    name: *const xmlChar,
    value: xmlXPathObjectPtr,
) -> i32 {
    return xmlXPathRegisterVariableNS(ctxt, name, 0 as *const xmlChar, value);
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
    ctxt: xmlXPathContextPtr,
    name: *const xmlChar,
    ns_uri: *const xmlChar,
    value: xmlXPathObjectPtr,
) -> i32 {
    if ctxt.is_null() {
        return -1;
    }
    if name.is_null() {
        return -1;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.varHash.is_null() {
        unsafe { (*ctxt).varHash = xmlHashCreate(0) }
    }
    if unsafe { (*ctxt).varHash.is_null() } {
        return -1;
    }
    if value.is_null() {
        return unsafe {
            xmlHashRemoveEntry2(
                (*ctxt).varHash,
                name,
                ns_uri,
                Some(
                    xmlXPathFreeObjectEntry
                        as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
                ),
            )
        };
    }
    return unsafe {
        xmlHashUpdateEntry2(
            (*ctxt).varHash,
            name,
            ns_uri,
            value as *mut (),
            Some(
                xmlXPathFreeObjectEntry
                    as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
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
    ctxt: xmlXPathContextPtr,
    f: xmlXPathVariableLookupFunc,
    data: *mut (),
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
pub fn xmlXPathVariableLookup(ctxt: xmlXPathContextPtr, name: *const xmlChar) -> xmlXPathObjectPtr {
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
    return xmlXPathVariableLookupNS(ctxt, name, 0 as *const xmlChar);
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
    ctxt: xmlXPathContextPtr,
    name: *const xmlChar,
    ns_uri: *const xmlChar,
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
                    as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
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
    ctxt: xmlXPathContextPtr,
    prefix: *const xmlChar,
    ns_uri: *const xmlChar,
) -> i32 {
    if ctxt.is_null() {
        return -1;
    }
    if prefix.is_null() {
        return -1;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if unsafe { *prefix.offset(0 as isize) as i32 == 0 } {
        return -1;
    }
    if safe_ctxt.nsHash.is_null() {
        unsafe { (*ctxt).nsHash = xmlHashCreate(10) }
    }
    if unsafe { (*ctxt).nsHash.is_null() } {
        return -1;
    }
    if ns_uri.is_null() {
        return unsafe {
            xmlHashRemoveEntry(
                (*ctxt).nsHash,
                prefix,
                Some(
                    xmlHashDefaultDeallocator
                        as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
                ),
            )
        };
    }
    return unsafe {
        xmlHashUpdateEntry(
            (*ctxt).nsHash,
            prefix,
            xmlStrdup(ns_uri) as *mut (),
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
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
pub fn xmlXPathNsLookup(ctxt: xmlXPathContextPtr, prefix: *const xmlChar) -> *const xmlChar {
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
                    b"xml\x00" as *const u8 as *const i8 as *const xmlChar,
                ) != 0
            } {
                return unsafe {
                    b"http://www.w3.org/XML/1998/namespace\x00" as *const u8 as *const i8
                        as *const xmlChar
                };
            }
        }
        #[cfg(not(XML_XML_NAMESPACE))]
        _ => {}
    };

    if !safe_ctxt.namespaces.is_null() {
        let mut i: i32;
        i = 0;
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
pub fn xmlXPathRegisteredNsCleanup(ctxt: xmlXPathContextPtr) {
    if ctxt.is_null() {
        return;
    }
    unsafe {
        xmlHashFree(
            (*ctxt).nsHash,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(_: *mut (), _: *const xmlChar) -> (),
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
pub fn xmlXPathNewFloat(val: libc::c_double) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>() as u64)
            as xmlXPathObjectPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating float object\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathObject>() as u64,
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
pub fn xmlXPathNewBoolean(val: i32) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>() as u64)
            as xmlXPathObjectPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating boolean object\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathObject>() as u64,
        );
        (*ret).type_0 = XPATH_BOOLEAN;
        (*ret).boolval = (val != 0) as i32;
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
pub fn xmlXPathNewString(val: *const xmlChar) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>() as u64)
            as xmlXPathObjectPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating string object\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathObject>() as u64,
        );
        (*ret).type_0 = XPATH_STRING;
    }
    if !val.is_null() {
        unsafe { (*ret).stringval = xmlStrdup(val) }
    } else {
        unsafe { (*ret).stringval = xmlStrdup(b"\x00" as *const u8 as *const i8 as *const xmlChar) }
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
pub fn xmlXPathWrapString(val: *mut xmlChar) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>() as u64)
            as xmlXPathObjectPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating string object\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathObject>() as u64,
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
pub fn xmlXPathNewCString(val: *const i8) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>() as u64)
            as xmlXPathObjectPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating string object\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathObject>() as u64,
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
pub fn xmlXPathWrapCString(val: *mut i8) -> xmlXPathObjectPtr {
    return xmlXPathWrapString(val as *mut xmlChar);
}
/* *
 * xmlXPathWrapExternal: * @val: the user data
 *
 * Wraps the @val data into an XPath object.
 *
 * Returns the newly created object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathWrapExternal(val: *mut ()) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>() as u64)
            as xmlXPathObjectPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating user object\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathObject>() as u64,
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
pub fn xmlXPathObjectCopy(val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    let safe_val = unsafe { &mut *val };
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>() as u64)
            as xmlXPathObjectPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"copying object\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathObjectPtr;
    }
    unsafe {
        memcpy(
            ret as *mut (),
            val as *const (),
            ::std::mem::size_of::<xmlXPathObject>() as u64,
        )
    };
    match () {
        #[cfg(XP_DEBUG_OBJ_USAGE)]
        _ => {
            unsafe {
                xmlXPathDebugObjUsageRequested(0 as xmlXPathContextPtr, (*val).type_0 as u32)
            };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    match safe_val.type_0 as u32 {
        XPATH_STRING => unsafe { (*ret).stringval = xmlStrdup((*val).stringval) },
        XPATH_XSLT_TREE | XPATH_NODESET => {
            unsafe {
                /* TODO: Check memory error. */
                (*ret).nodesetval = xmlXPathNodeSetMerge(0 as xmlNodeSetPtr, (*val).nodesetval);
                /* Do not deallocate the copied tree value */
                (*ret).boolval = 0 as i32
            }
        }
        XPATH_LOCATIONSET => {
            match () {
                #[cfg(LIBXML_XPTR_ENABLED)]
                _ => {
                    let mut loc: xmlLocationSetPtr = safe_val.user as xmlLocationSetPtr;
                    unsafe {
                        (*ret).user =
                            xmlXPtrLocationSetMerge(0 as xmlLocationSetPtr, loc) as *mut ()
                    }
                }
                #[cfg(not(LIBXML_XPTR_ENABLED))]
                _ => {}
            };
        }
        XPATH_USERS => unsafe { (*ret).user = (*val).user },
        XPATH_UNDEFINED => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlXPathObjectCopy: unsupported type %d\n\x00" as *const u8 as *const i8,
                    (*val).type_0 as u32,
                )
            };
        }
        XPATH_BOOLEAN | XPATH_NUMBER | XPATH_POINT | XPATH_RANGE | _ => {}
    }
    return ret;
}
/* *
 * xmlXPathFreeObject: * @obj: the object to free
 *
 * Free up an xmlXPathObjectPtr object.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathFreeObject(obj: xmlXPathObjectPtr) {
    if obj.is_null() {
        return;
    } /* TODO: Just for debugging. */
    let safe_obj = unsafe { &mut *obj };
    if safe_obj.type_0 as u32 == XPATH_NODESET as u32
        || safe_obj.type_0 as u32 == XPATH_XSLT_TREE as u32
    {
        if safe_obj.boolval != 0 {
            safe_obj.type_0 = XPATH_XSLT_TREE;
            if !safe_obj.nodesetval.is_null() {
                unsafe { xmlXPathFreeValueTree((*obj).nodesetval) };
            }
        } else if !safe_obj.nodesetval.is_null() {
            unsafe { xmlXPathFreeNodeSet((*obj).nodesetval) };
        }
    } else if safe_obj.type_0 as u32 == XPATH_STRING as u32 {
        if !safe_obj.stringval.is_null() {
            unsafe { xmlFree.expect("non-null function pointer")((*obj).stringval as *mut ()) };
        }
    }
    match () {
        #[cfg(LIBXML_XPTR_ENABLED)]
        _ => {
            if safe_obj.type_0 as u32 == XPATH_LOCATIONSET as u32 {
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
            unsafe { xmlXPathDebugObjUsageReleased(0 as xmlXPathContextPtr, (*obj).type_0 as u32) };
        }
        #[cfg(not(XP_DEBUG_OBJ_USAGE))]
        _ => {}
    };
    unsafe { xmlFree.expect("non-null function pointer")(obj as *mut ()) };
}
#[cfg(LIBXML_XPATH_ENABLED)]
extern "C" fn xmlXPathFreeObjectEntry(mut obj: *mut (), mut name: *const xmlChar) {
    xmlXPathFreeObject(obj as xmlXPathObjectPtr);
}
/* *
 * xmlXPathReleaseObject: * @obj: the xmlXPathObjectPtr to free or to cache
 *
 * Depending on the state of the cache this frees the given
 * XPath object or stores it in the cache.
 */
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathReleaseObject(ctxt: xmlXPathContextPtr, obj: xmlXPathObjectPtr) {
    let mut current_block: u64;
    if obj.is_null() {
        return;
    }
    let safe_obj = unsafe { &mut *obj };
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.cache.is_null() {
        xmlXPathFreeObject(obj);
    } else {
        let mut cache: xmlXPathContextCachePtr = safe_ctxt.cache as xmlXPathContextCachePtr;
        match safe_obj.type_0 as u32 {
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
                        (*(*obj).nodesetval).nodeMax <= 40
                            && ((*cache).nodesetObjs.is_null()
                                || (*(*cache).nodesetObjs).number < (*cache).maxNodeset)
                    } {
                        if unsafe { (*cache).nodesetObjs.is_null() } {
                            unsafe { (*cache).nodesetObjs = xmlPointerListCreate(10 as i32) };
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
                                        obj as *mut (),
                                        0 as i32,
                                    ) == -(1 as i32)
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
                        xmlFree.expect("non-null function pointer")((*obj).stringval as *mut ())
                    };
                }
                if unsafe {
                    (*cache).stringObjs.is_null()
                        || (*(*cache).stringObjs).number < (*cache).maxString
                } {
                    if unsafe { (*cache).stringObjs.is_null() } {
                        unsafe { (*cache).stringObjs = xmlPointerListCreate(10 as i32) };
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
                                xmlPointerListAddSize((*cache).stringObjs, obj as *mut (), 0 as i32)
                                    == -(1 as i32)
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
                        unsafe { (*cache).booleanObjs = xmlPointerListCreate(10 as i32) };
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
                                    obj as *mut (),
                                    0 as i32,
                                ) == -(1 as i32)
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
                        unsafe { (*cache).numberObjs = xmlPointerListCreate(10 as i32) };
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
                                xmlPointerListAddSize((*cache).numberObjs, obj as *mut (), 0 as i32)
                                    == -(1 as i32)
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
                        unsafe { (*cache).miscObjs = xmlPointerListCreate(10 as i32) };
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
                                xmlPointerListAddSize((*cache).miscObjs, obj as *mut (), 0 as i32)
                                    == -(1 as i32)
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
                unsafe { xmlFree.expect("non-null function pointer")(obj as *mut ()) };
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
                    if unsafe { (*tmpset).nodeNr > 1 as i32 } {
                        let mut i: i32 = 0;
                        let mut node: xmlNodePtr = 0 as *mut xmlNode;
                        i = 0 as i32;
                        while i < unsafe { (*tmpset).nodeNr } {
                            unsafe { node = *(*tmpset).nodeTab.offset(i as isize) };
                            if unsafe {
                                !node.is_null()
                                    && (*node).type_0 as u32 == XML_NAMESPACE_DECL as i32 as u32
                            } {
                                xmlXPathNodeSetFreeNs(node as xmlNsPtr);
                            }
                            i += 1
                        }
                    } else if unsafe { (*tmpset).nodeNr == 1 as i32 } {
                        if unsafe {
                            !(*(*tmpset).nodeTab.offset(0 as i32 as isize)).is_null()
                                && (**(*tmpset).nodeTab.offset(0 as i32 as isize)).type_0 as u32
                                    == XML_NAMESPACE_DECL as i32 as u32
                        } {
                            unsafe {
                                xmlXPathNodeSetFreeNs(
                                    *(*tmpset).nodeTab.offset(0 as i32 as isize) as xmlNsPtr
                                )
                            };
                        }
                    }
                    unsafe {
                        (*tmpset).nodeNr = 0 as i32;
                        memset(
                            obj as *mut (),
                            0 as i32,
                            ::std::mem::size_of::<xmlXPathObject>() as u64,
                        );
                        (*obj).nodesetval = tmpset
                    }
                } else {
                    unsafe {
                        memset(
                            obj as *mut (),
                            0 as i32,
                            ::std::mem::size_of::<xmlXPathObject>() as u64,
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
pub fn xmlXPathCastBooleanToString(val: i32) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if val != 0 {
        ret = unsafe { xmlStrdup(b"true\x00" as *const u8 as *const i8 as *const xmlChar) }
    } else {
        ret = unsafe { xmlStrdup(b"false\x00" as *const u8 as *const i8 as *const xmlChar) }
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
pub fn xmlXPathCastNumberToString(val: libc::c_double) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    match xmlXPathIsInf(val) {
        1 => {
            ret = unsafe { xmlStrdup(b"Infinity\x00" as *const u8 as *const i8 as *const xmlChar) }
        }
        -1 => {
            ret = unsafe { xmlStrdup(b"-Infinity\x00" as *const u8 as *const i8 as *const xmlChar) }
        }
        _ => {
            if xmlXPathIsNaN(val) != 0 {
                ret = unsafe { xmlStrdup(b"NaN\x00" as *const u8 as *const i8 as *const xmlChar) }
            } else if val == 0 as libc::c_double {
                /* Omit sign for negative zero. */
                ret = unsafe { xmlStrdup(b"0\x00" as *const u8 as *const i8 as *const xmlChar) }
            } else {
                /* could be improved */
                let mut buf: [i8; 100] = [0; 100];
                xmlXPathFormatNumber(val, buf.as_mut_ptr(), 99);
                buf[99] = 0;
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
pub fn xmlXPathCastNodeToString(node: xmlNodePtr) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    ret = unsafe { xmlNodeGetContent(node as *const xmlNode) };
    if ret.is_null() {
        ret = unsafe { xmlStrdup(b"\x00" as *const u8 as *const i8 as *const xmlChar) }
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
pub fn xmlXPathCastNodeSetToString(ns: xmlNodeSetPtr) -> *mut xmlChar {
    let safe_ns = unsafe { &mut *ns };
    if ns.is_null() || safe_ns.nodeNr == 0 || safe_ns.nodeTab.is_null() {
        return unsafe { xmlStrdup(b"\x00" as *const u8 as *const i8 as *const xmlChar) };
    }
    if safe_ns.nodeNr > 1 {
        xmlXPathNodeSetSort(ns);
    }
    return unsafe { xmlXPathCastNodeToString(*(*ns).nodeTab.offset(0 as isize)) };
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
pub fn xmlXPathCastToString(val: xmlXPathObjectPtr) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if val.is_null() {
        return unsafe { xmlStrdup(b"\x00" as *const u8 as *const i8 as *const xmlChar) };
    }
    let safe_val = unsafe { &mut *val };
    match safe_val.type_0 as u32 {
        XPATH_UNDEFINED => {
            match () {
                #[cfg(DEBUG_EXPR)]
                _ => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"String: undefined\n\x00" as *const u8 as *const i8,
                        )
                    };
                }
                #[cfg(not(DEBUG_EXPR))]
                _ => {}
            };
            ret = unsafe { xmlStrdup(b"\x00" as *const u8 as *const i8 as *const xmlChar) }
        }
        XPATH_NODESET | XPATH_XSLT_TREE => {
            ret = unsafe { xmlXPathCastNodeSetToString((*val).nodesetval) }
        }
        XPATH_STRING => return unsafe { xmlStrdup((*val).stringval) },
        XPATH_BOOLEAN => ret = unsafe { xmlXPathCastBooleanToString((*val).boolval) },
        XPATH_NUMBER => ret = unsafe { xmlXPathCastNumberToString((*val).floatval) },
        XPATH_USERS | XPATH_POINT | XPATH_RANGE | XPATH_LOCATIONSET => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                    b"xpath.c\x00" as *const u8 as *const i8,
                    5775,
                )
            };
            ret = unsafe { xmlStrdup(b"\x00" as *const u8 as *const i8 as *const xmlChar) }
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
pub fn xmlXPathConvertString(val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let mut res: *mut xmlChar = 0 as *mut xmlChar;
    if val.is_null() {
        return xmlXPathNewCString(b"\x00" as *const u8 as *const i8);
    }
    let safe_val = unsafe { &mut *val };
    match safe_val.type_0 as u32 {
        XPATH_NODESET | XPATH_XSLT_TREE => {
            res = unsafe { xmlXPathCastNodeSetToString((*val).nodesetval) }
        }
        XPATH_STRING => return val,
        XPATH_BOOLEAN => res = unsafe { xmlXPathCastBooleanToString((*val).boolval) },
        XPATH_NUMBER => res = unsafe { xmlXPathCastNumberToString((*val).floatval) },
        XPATH_USERS | XPATH_POINT | XPATH_RANGE | XPATH_LOCATIONSET => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                    b"xpath.c\x00" as *const u8 as *const i8,
                    5820,
                )
            };
        }
        XPATH_UNDEFINED | _ => {
            match () {
                #[cfg(DEBUG_EXPR)]
                _ => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"STRING: undefined\n\x00" as *const u8 as *const i8,
                        )
                    };
                }
                #[cfg(not(DEBUG_EXPR))]
                _ => {}
            };
        }
    }
    xmlXPathFreeObject(val);
    if res.is_null() {
        return xmlXPathNewCString(b"\x00" as *const u8 as *const i8);
    }
    return xmlXPathWrapString(res);
}
/* *
 * xmlXPathCastBooleanToNumber: * @val: a boolean
 *
 * Converts a boolean to its number value
 *
 * Returns the number value
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastBooleanToNumber(val: i32) -> libc::c_double {
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
pub fn xmlXPathCastStringToNumber(val: *const xmlChar) -> libc::c_double {
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
pub fn xmlXPathCastNodeToNumber(node: xmlNodePtr) -> libc::c_double {
    let mut strval: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: libc::c_double = 0.;
    if node.is_null() {
        return unsafe { xmlXPathNAN };
    }
    strval = xmlXPathCastNodeToString(node);
    if strval.is_null() {
        return unsafe { xmlXPathNAN };
    }
    ret = xmlXPathCastStringToNumber(strval);
    unsafe { xmlFree.expect("non-null function pointer")(strval as *mut ()) };
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
pub fn xmlXPathCastNodeSetToNumber(ns: xmlNodeSetPtr) -> libc::c_double {
    let mut str: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: libc::c_double = 0.;
    if ns.is_null() {
        return unsafe { xmlXPathNAN };
    }
    str = xmlXPathCastNodeSetToString(ns);
    ret = xmlXPathCastStringToNumber(str);
    unsafe { xmlFree.expect("non-null function pointer")(str as *mut ()) };
    return ret;
}
/* *
 * xmlXPathCastToNumber: * @val: an XPath object
 *
 * Converts an XPath object to its number value
 *
 * Returns the number value
 */

pub fn xmlXPathCastToNumber(val: xmlXPathObjectPtr) -> libc::c_double {
    let mut ret: libc::c_double = 0.0f64;
    if val.is_null() {
        return unsafe { xmlXPathNAN };
    }
    let safe_val = unsafe { &mut *val };
    match safe_val.type_0 as u32 {
        XPATH_UNDEFINED => {
            match () {
                #[cfg(DEBUG_EXPR)]
                _ => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"NUMBER: undefined\n\x00" as *const u8 as *const i8,
                        )
                    };
                }
                #[cfg(not(DEBUG_EXPR))]
                _ => {}
            };
            ret = unsafe { xmlXPathNAN }
        }
        XPATH_NODESET | XPATH_XSLT_TREE => {
            ret = unsafe { xmlXPathCastNodeSetToNumber((*val).nodesetval) }
        }
        XPATH_STRING => ret = unsafe { xmlXPathCastStringToNumber((*val).stringval) },
        XPATH_NUMBER => ret = safe_val.floatval,
        XPATH_BOOLEAN => ret = unsafe { xmlXPathCastBooleanToNumber((*val).boolval) },
        XPATH_USERS | XPATH_POINT | XPATH_RANGE | XPATH_LOCATIONSET => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                    b"xpath.c\x00" as *const u8 as *const i8,
                    5940,
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
pub fn xmlXPathConvertNumber(val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return xmlXPathNewFloat(0.0f64);
    }
    let safe_val = unsafe { &mut *val };
    if safe_val.type_0 as u32 == XPATH_NUMBER as u32 {
        return val;
    }
    ret = xmlXPathNewFloat(xmlXPathCastToNumber(val));
    xmlXPathFreeObject(val);
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
pub fn xmlXPathCastNumberToBoolean(val: libc::c_double) -> i32 {
    if xmlXPathIsNaN(val) != 0 || val == 0.0f64 {
        return 0;
    }
    return 1;
}
/* *
 * xmlXPathCastStringToBoolean: * @val: a string
 *
 * Converts a string to its boolean value
 *
 * Returns the boolean value
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastStringToBoolean(val: *const xmlChar) -> i32 {
    if val.is_null() || unsafe { xmlStrlen(val) } == 0 {
        return 0;
    }
    return 1;
}
/* *
 * xmlXPathCastNodeSetToBoolean: * @ns: a node-set
 *
 * Converts a node-set to its boolean value
 *
 * Returns the boolean value
 */

#[cfg(LIBXML_XPATH_ENABLED)]
pub fn xmlXPathCastNodeSetToBoolean(ns: xmlNodeSetPtr) -> i32 {
    let safe_ns = unsafe { &mut *ns };
    if ns.is_null() || safe_ns.nodeNr == 0 {
        return 0;
    }
    return 1;
}
/* *
 * xmlXPathCastToBoolean: * @val: an XPath object
 *
 * Converts an XPath object to its boolean value
 *
 * Returns the boolean value
 */

pub fn xmlXPathCastToBoolean(val: xmlXPathObjectPtr) -> i32 {
    let mut ret: i32 = 0;
    if val.is_null() {
        return 0;
    }
    let safe_val = unsafe { &mut *val };
    match safe_val.type_0 as u32 {
        XPATH_UNDEFINED => {
            match () {
                #[cfg(DEBUG_EXPR)]
                _ => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"BOOLEAN: undefined\n\x00" as *const u8 as *const i8,
                        )
                    };
                }
                #[cfg(not(DEBUG_EXPR))]
                _ => {}
            };
            ret = 0
        }
        XPATH_NODESET | XPATH_XSLT_TREE => {
            ret = unsafe { xmlXPathCastNodeSetToBoolean((*val).nodesetval) }
        }
        XPATH_STRING => ret = unsafe { xmlXPathCastStringToBoolean((*val).stringval) },
        XPATH_NUMBER => ret = unsafe { xmlXPathCastNumberToBoolean((*val).floatval) },
        XPATH_BOOLEAN => ret = safe_val.boolval,
        XPATH_USERS | XPATH_POINT | XPATH_RANGE | XPATH_LOCATIONSET => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                    b"xpath.c\x00" as *const u8 as *const i8,
                    6052,
                )
            };
            ret = 0
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
pub fn xmlXPathConvertBoolean(val: xmlXPathObjectPtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return xmlXPathNewBoolean(0);
    }
    let safe_val = unsafe { &mut *val };
    if safe_val.type_0 as u32 == XPATH_BOOLEAN as u32 {
        return val;
    }
    ret = xmlXPathNewBoolean(xmlXPathCastToBoolean(val));
    xmlXPathFreeObject(val);
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

pub fn xmlXPathNewContext(doc: xmlDocPtr) -> xmlXPathContextPtr {
    let mut ret: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathContext>() as u64
        ) as xmlXPathContextPtr
    };
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating context\n\x00" as *const u8 as *const i8,
        );
        return 0 as xmlXPathContextPtr;
    }
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathContext>() as u64,
        );
        (*ret).doc = doc;
        (*ret).node = 0 as xmlNodePtr;
        (*ret).varHash = 0 as xmlHashTablePtr;
        (*ret).nb_types = 0;
        (*ret).max_types = 0;
        (*ret).types = 0 as xmlXPathTypePtr;
        (*ret).funcHash = xmlHashCreate(0);
        (*ret).nb_axis = 0;
        (*ret).max_axis = 0;
        (*ret).axis = 0 as xmlXPathAxisPtr;
        (*ret).nsHash = 0 as xmlHashTablePtr;
        (*ret).user = 0 as *mut ();
        (*ret).contextSize = -1;
        (*ret).proximityPosition = -1;
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
pub fn xmlXPathFreeContext(ctxt: xmlXPathContextPtr) {
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
        xmlFree.expect("non-null function pointer")(ctxt as *mut ());
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
    str: *const xmlChar,
    ctxt: xmlXPathContextPtr,
) -> xmlXPathParserContextPtr {
    let mut ret: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathParserContext>() as u64,
        ) as xmlXPathParserContextPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                ctxt,
                b"creating parser context\n\x00" as *const u8 as *const i8,
            )
        };
        return 0 as xmlXPathParserContextPtr;
    }
    let safe_ret = unsafe { &mut *ret };
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathParserContext>() as u64,
        );
        (*ret).base = str;
        (*ret).cur = (*ret).base;
        (*ret).context = ctxt;
        (*ret).comp = xmlXPathNewCompExpr();
    }
    if safe_ret.comp.is_null() {
        unsafe {
            xmlFree.expect("non-null function pointer")((*ret).valueTab as *mut ());
            xmlFree.expect("non-null function pointer")(ret as *mut ());
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
    comp: xmlXPathCompExprPtr,
    ctxt: xmlXPathContextPtr,
) -> xmlXPathParserContextPtr {
    let mut ret: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    ret = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::std::mem::size_of::<xmlXPathParserContext>() as u64,
        ) as xmlXPathParserContextPtr
    };
    if ret.is_null() {
        unsafe {
            xmlXPathErrMemory(
                ctxt,
                b"creating evaluation context\n\x00" as *const u8 as *const i8,
            )
        };
        return 0 as xmlXPathParserContextPtr;
    }
    let safe_ret = unsafe { &mut *ret };
    unsafe {
        memset(
            ret as *mut (),
            0,
            ::std::mem::size_of::<xmlXPathParserContext>() as u64,
        );
        /* Allocate the value stack */
        (*ret).valueTab = xmlMalloc.expect("non-null function pointer")(
            (10 as u64).wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>() as u64),
        ) as *mut xmlXPathObjectPtr;
    }

    if safe_ret.valueTab.is_null() {
        unsafe {
            xmlFree.expect("non-null function pointer")(ret as *mut ());
            xmlXPathErrMemory(
                ctxt,
                b"creating evaluation context\n\x00" as *const u8 as *const i8,
            );
        }
        return 0 as xmlXPathParserContextPtr;
    }
    unsafe {
        (*ret).valueNr = 0;
        (*ret).valueMax = 10;
        (*ret).value = 0 as xmlXPathObjectPtr;
        (*ret).valueFrame = 0;
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
pub fn xmlXPathFreeParserContext(ctxt: xmlXPathParserContextPtr) {
    let mut i: i32;
    let safe_ctxt = unsafe { &mut *ctxt };
    if !safe_ctxt.valueTab.is_null() {
        i = 0;
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
        unsafe { xmlFree.expect("non-null function pointer")((*ctxt).valueTab as *mut ()) };
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
    unsafe { xmlFree.expect("non-null function pointer")(ctxt as *mut ()) };
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
fn xmlXPathNodeValHash(mut node: xmlNodePtr) -> u32 {
    let mut len: i32 = 2;
    let mut string: *const xmlChar = 0 as *const xmlChar;
    let mut tmp: xmlNodePtr = 0 as xmlNodePtr;
    let mut ret: u32 = 0;
    if node.is_null() {
        return 0;
    }
    let safe_node = unsafe { &mut *node };
    if safe_node.type_0 as u32 == XML_DOCUMENT_NODE as u32 {
        tmp = unsafe { xmlDocGetRootElement(node as xmlDocPtr as *const xmlDoc) };
        if tmp.is_null() {
            node = safe_node.children
        } else {
            node = tmp
        }
        if node.is_null() {
            return 0;
        }
    }
    match unsafe { (*node).type_0 as u32 } {
        8 | 7 | 4 | 3 => {
            string = unsafe { (*node).content };
            if string.is_null() {
                return 0;
            }
            if unsafe { *string.offset(0) as i32 == 0 } {
                return 0;
            }
            return unsafe {
                (*string.offset(0) as u32).wrapping_add((*string.offset(1) as u32) << 8)
            };
        }
        18 => {
            string = unsafe { (*(node as xmlNsPtr)).href };
            if string.is_null() {
                return 0;
            }
            if unsafe { *string.offset(0) as i32 == 0 } {
                return 0;
            }
            return unsafe {
                (*string.offset(0) as u32).wrapping_add((*string.offset(1) as u32) << 8)
            };
        }
        2 => tmp = unsafe { (*(node as xmlAttrPtr)).children },
        1 => tmp = unsafe { (*node).children },
        _ => return 0,
    }
    while !tmp.is_null() {
        match unsafe { (*tmp).type_0 as u32 } {
            4 | 3 => string = unsafe { (*tmp).content },
            _ => string = 0 as *const xmlChar,
        }
        if unsafe { !string.is_null() && *string.offset(0) as i32 != 0 } {
            if len == 1 {
                return unsafe { ret.wrapping_add((*string.offset(0) as u32) << 8) };
            }
            if unsafe { *string.offset(1) as i32 == 0 } {
                len = 1;
                ret = unsafe { *string.offset(0) as u32 }
            } else {
                return unsafe {
                    (*string.offset(0) as u32).wrapping_add((*string.offset(1) as u32) << 8)
                };
            }
        }
        /*
         * Skip to next node
         */
        if unsafe { !(*tmp).children.is_null() && (*tmp).type_0 as u32 != XML_DTD_NODE as u32 } {
            if unsafe { (*(*tmp).children).type_0 as u32 != XML_ENTITY_DECL as u32 } {
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
fn xmlXPathStringHash(string: *const xmlChar) -> u32 {
    if string.is_null() {
        return 0;
    }
    if unsafe { *string.offset(0) as i32 == 0 } {
        return 0;
    }
    return unsafe { (*string.offset(0) as u32).wrapping_add((*string.offset(1) as u32) << 8) };
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
    ctxt: xmlXPathParserContextPtr,
    inf: i32,
    strict: i32,
    arg: xmlXPathObjectPtr,
    f: xmlXPathObjectPtr,
) -> i32 {
    let mut i: i32;
    let mut ret: i32 = 0;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    let safe_arg = unsafe { &mut *arg };
    if f.is_null()
        || arg.is_null()
        || safe_arg.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg.type_0 as u32 != XPATH_XSLT_TREE as u32
    {
        unsafe {
            xmlXPathReleaseObject((*ctxt).context, arg);
            xmlXPathReleaseObject((*ctxt).context, f);
        };
        return 0;
    }
    ns = safe_arg.nodesetval;
    if !ns.is_null() {
        i = 0;
        while i < unsafe { (*ns).nodeNr } {
            str2 = unsafe { xmlXPathCastNodeToString(*(*ns).nodeTab.offset(i as isize)) };
            if !str2.is_null() {
                unsafe {
                    valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                    xmlFree.expect("non-null function pointer")(str2 as *mut ());
                    xmlXPathNumberFunction(ctxt, 1);
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
    ctxt: xmlXPathParserContextPtr,
    inf: i32,
    strict: i32,
    arg: xmlXPathObjectPtr,
    s: xmlXPathObjectPtr,
) -> i32 {
    let mut i: i32;
    let mut ret: i32 = 0;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    let safe_arg = unsafe { &mut *arg };
    if s.is_null()
        || arg.is_null()
        || safe_arg.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg.type_0 as u32 != XPATH_XSLT_TREE as u32
    {
        unsafe {
            xmlXPathReleaseObject((*ctxt).context, arg);
            xmlXPathReleaseObject((*ctxt).context, s);
        };
        return 0;
    }
    ns = safe_arg.nodesetval;
    if !ns.is_null() {
        i = 0;
        while i < unsafe { (*ns).nodeNr } {
            str2 = unsafe { xmlXPathCastNodeToString(*(*ns).nodeTab.offset(i as isize)) };
            if !str2.is_null() {
                unsafe {
                    valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                    xmlFree.expect("non-null function pointer")(str2 as *mut ());
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
    inf: i32,
    strict: i32,
    arg1: xmlXPathObjectPtr,
    arg2: xmlXPathObjectPtr,
) -> i32 {
    let mut i: i32;
    let mut j: i32;
    let mut init: i32 = 0;
    let mut val1: libc::c_double = 0.;
    let mut values2: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ret: i32 = 0;
    let mut ns1: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut ns2: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let safe_arg1 = unsafe { &mut *arg1 };
    let safe_arg2 = unsafe { &mut *arg2 };
    if arg1.is_null()
        || safe_arg1.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg1.type_0 as u32 != XPATH_XSLT_TREE as u32
    {
        unsafe { xmlXPathFreeObject(arg2) };
        return 0;
    }
    if arg2.is_null()
        || safe_arg2.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg2.type_0 as u32 != XPATH_XSLT_TREE as u32
    {
        unsafe {
            xmlXPathFreeObject(arg1);
            xmlXPathFreeObject(arg2);
        };
        return 0;
    }
    ns1 = safe_arg1.nodesetval;
    ns2 = safe_arg2.nodesetval;
    let safe_ns1 = unsafe { &mut *ns1 };
    let safe_ns2 = unsafe { &mut *ns2 };
    if ns1.is_null() || safe_ns1.nodeNr <= 0 {
        unsafe {
            xmlXPathFreeObject(arg1);
            xmlXPathFreeObject(arg2);
        };
        return 0;
    }
    if ns2.is_null() || safe_ns2.nodeNr <= 0 {
        unsafe {
            xmlXPathFreeObject(arg1);
            xmlXPathFreeObject(arg2);
        };
        return 0;
    }
    values2 = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ((*ns2).nodeNr as u64).wrapping_mul(::std::mem::size_of::<libc::c_double>() as u64),
        ) as *mut libc::c_double
    };
    if values2.is_null() {
        /* TODO: Propagate memory error. */
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"comparing nodesets\n\x00" as *const u8 as *const i8,
            );
            xmlXPathFreeObject(arg1);
            xmlXPathFreeObject(arg2);
        };
        return 0;
    }
    i = 0;
    while i < safe_ns1.nodeNr {
        val1 = unsafe { xmlXPathCastNodeToNumber(*(*ns1).nodeTab.offset(i as isize)) };
        if !(unsafe { xmlXPathIsNaN(val1) } != 0) {
            j = 0;
            while j < safe_ns2.nodeNr {
                if init == 0 {
                    unsafe {
                        *values2.offset(j as isize) =
                            xmlXPathCastNodeToNumber(*(*ns2).nodeTab.offset(j as isize))
                    }
                }
                if !(unsafe { xmlXPathIsNaN(*values2.offset(j as isize)) } != 0) {
                    if inf != 0 && strict != 0 {
                        ret = unsafe { (val1 < *values2.offset(j as isize)) } as i32
                    } else if inf != 0 && strict == 0 {
                        ret = unsafe { (val1 <= *values2.offset(j as isize)) } as i32
                    } else if inf == 0 && strict != 0 {
                        ret = unsafe { (val1 > *values2.offset(j as isize)) } as i32
                    } else if inf == 0 && strict == 0 {
                        ret = unsafe { (val1 >= *values2.offset(j as isize)) } as i32
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
            init = 1
        }
        i += 1
    }
    unsafe {
        xmlFree.expect("non-null function pointer")(values2 as *mut ());
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
    ctxt: xmlXPathParserContextPtr,
    inf: i32,
    strict: i32,
    arg: xmlXPathObjectPtr,
    val: xmlXPathObjectPtr,
) -> i32 {
    let safe_arg = unsafe { &mut *arg };
    let safe_val = unsafe { &mut *val };
    if val.is_null()
        || arg.is_null()
        || safe_arg.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg.type_0 as u32 != XPATH_XSLT_TREE as u32
    {
        return 0;
    }
    match safe_val.type_0 as u32 {
        3 => return unsafe { xmlXPathCompareNodeSetFloat(ctxt, inf, strict, arg, val) },
        1 | 9 => return unsafe { xmlXPathCompareNodeSets(inf, strict, arg, val) },
        4 => return unsafe { xmlXPathCompareNodeSetString(ctxt, inf, strict, arg, val) },
        2 => {
            unsafe {
                valuePush(ctxt, arg);
                xmlXPathBooleanFunction(ctxt, 1);
                valuePush(ctxt, val);
            };
            return unsafe { xmlXPathCompareValues(ctxt, inf, strict) };
        }
        _ => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(), b"xmlXPathCompareNodeSetValue: Can\'t compare node set and object of type %d\n\x00"
                                                                           as *const u8
                                                                           as *const i8, (*val).type_0
                                                                           as u32);
                xmlXPathReleaseObject((*ctxt).context, arg);
                xmlXPathReleaseObject((*ctxt).context, val);
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32)
            };
            return 0;
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
fn xmlXPathEqualNodeSetString(arg: xmlXPathObjectPtr, str: *const xmlChar, neq: i32) -> i32 {
    let mut i: i32;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    let mut hash: u32;
    let safe_arg = unsafe { &mut *arg };
    if str.is_null()
        || arg.is_null()
        || safe_arg.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg.type_0 as u32 != XPATH_XSLT_TREE as u32
    {
        return 0;
    }
    ns = safe_arg.nodesetval;
    /*
     * A NULL nodeset compared with a string is always false
     * (since there is no node equal, and no node not equal) */
    let safe_ns = unsafe { &mut *ns };
    if ns.is_null() || safe_ns.nodeNr <= 0 {
        return 0;
    }
    hash = unsafe { xmlXPathStringHash(str) };
    i = 0;
    while i < safe_ns.nodeNr {
        if unsafe { xmlXPathNodeValHash(*(*ns).nodeTab.offset(i as isize)) == hash } {
            str2 =
                unsafe { xmlNodeGetContent(*(*ns).nodeTab.offset(i as isize) as *const xmlNode) };
            if !str2.is_null() && unsafe { xmlStrEqual(str, str2) != 0 } {
                unsafe { xmlFree.expect("non-null function pointer")(str2 as *mut ()) };
                if !(neq != 0) {
                    return 1;
                }
            } else if str2.is_null()
                && unsafe {
                    xmlStrEqual(str, b"\x00" as *const u8 as *const i8 as *mut xmlChar) != 0
                }
            {
                if !(neq != 0) {
                    return 1;
                }
            } else {
                if neq != 0 {
                    if !str2.is_null() {
                        unsafe { xmlFree.expect("non-null function pointer")(str2 as *mut ()) };
                    }
                    return 1;
                }
                if !str2.is_null() {
                    unsafe { xmlFree.expect("non-null function pointer")(str2 as *mut ()) };
                }
            }
        } else if neq != 0 {
            return 1;
        }
        i += 1
    }
    return 0;
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
    ctxt: xmlXPathParserContextPtr,
    arg: xmlXPathObjectPtr,
    f: libc::c_double,
    neq: i32,
) -> i32 {
    let mut i: i32; /* NaN is unequal to any value */
    let mut ret: i32 = 0;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    let mut val: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut v: libc::c_double = 0.;
    let safe_arg = unsafe { &mut *arg };
    if arg.is_null()
        || safe_arg.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg.type_0 as u32 != XPATH_XSLT_TREE as u32
    {
        return 0;
    }
    ns = safe_arg.nodesetval;
    if !ns.is_null() {
        let safe_ns = unsafe { &mut *ns };
        i = 0;
        while i < safe_ns.nodeNr {
            str2 = unsafe { xmlXPathCastNodeToString(*(*ns).nodeTab.offset(i as isize)) };
            if !str2.is_null() {
                unsafe {
                    valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                    xmlFree.expect("non-null function pointer")(str2 as *mut ());
                    xmlXPathNumberFunction(ctxt, 1);
                    val = valuePop(ctxt);
                    v = (*val).floatval;
                    xmlXPathReleaseObject((*ctxt).context, val);
                }
                if unsafe { xmlXPathIsNaN(v) } == 0 {
                    if neq == 0 && v == f {
                        ret = 1;
                        break;
                    } else {
                        if neq != 0 && v != f {
                            ret = 1;
                            break;
                        }
                    }
                } else if neq != 0 {
                    ret = 1
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
fn xmlXPathEqualNodeSets(arg1: xmlXPathObjectPtr, arg2: xmlXPathObjectPtr, neq: i32) -> i32 {
    let mut i: i32;
    let mut j: i32;
    let mut hashs1: *mut u32 = 0 as *mut u32;
    let mut hashs2: *mut u32 = 0 as *mut u32;
    let mut values1: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut values2: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut ret: i32 = 0;
    let mut ns1: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut ns2: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let safe_arg1 = unsafe { &mut *arg1 };
    let safe_arg2 = unsafe { &mut *arg2 };
    if arg1.is_null()
        || safe_arg1.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg1.type_0 as u32 != XPATH_XSLT_TREE as u32
    {
        return 0;
    }
    if arg2.is_null()
        || safe_arg2.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg2.type_0 as u32 != XPATH_XSLT_TREE as u32
    {
        return 0;
    }
    ns1 = safe_arg1.nodesetval;
    ns2 = safe_arg2.nodesetval;
    let safe_ns1 = unsafe { &mut *ns1 };
    let safe_ns2 = unsafe { &mut *ns2 };
    if ns1.is_null() || safe_ns1.nodeNr <= 0 {
        return 0;
    }
    if ns2.is_null() || safe_ns2.nodeNr <= 0 {
        return 0;
    }
    /*
     * for equal, check if there is a node pertaining to both sets
     */
    if neq == 0 {
        i = 0;
        while i < safe_ns1.nodeNr {
            j = 0;
            while j < safe_ns2.nodeNr {
                if unsafe {
                    *(*ns1).nodeTab.offset(i as isize) == *(*ns2).nodeTab.offset(j as isize)
                } {
                    return 1;
                }
                j += 1
            }
            i += 1
        }
    }
    values1 = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ((*ns1).nodeNr as u64).wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as u64),
        ) as *mut *mut xmlChar
    };
    if values1.is_null() {
        /* TODO: Propagate memory error. */
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"comparing nodesets\n\x00" as *const u8 as *const i8,
            )
        };
        return 0;
    }
    hashs1 = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ((*ns1).nodeNr as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64),
        ) as *mut u32
    };
    if hashs1.is_null() {
        /* TODO: Propagate memory error. */
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"comparing nodesets\n\x00" as *const u8 as *const i8,
            );
            xmlFree.expect("non-null function pointer")(values1 as *mut ());
        };
        return 0;
    }
    unsafe {
        memset(
            values1 as *mut (),
            0,
            ((*ns1).nodeNr as u64).wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as u64),
        )
    };
    values2 = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ((*ns2).nodeNr as u64).wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as u64),
        ) as *mut *mut xmlChar
    };
    if values2.is_null() {
        /* TODO: Propagate memory error. */
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"comparing nodesets\n\x00" as *const u8 as *const i8,
            );
            xmlFree.expect("non-null function pointer")(hashs1 as *mut ());
            xmlFree.expect("non-null function pointer")(values1 as *mut ());
        }
        return 0;
    }
    hashs2 = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ((*ns2).nodeNr as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64),
        ) as *mut u32
    };
    if hashs2.is_null() {
        /* TODO: Propagate memory error. */
        unsafe {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"comparing nodesets\n\x00" as *const u8 as *const i8,
            );
            xmlFree.expect("non-null function pointer")(hashs1 as *mut ());
            xmlFree.expect("non-null function pointer")(values1 as *mut ());
            xmlFree.expect("non-null function pointer")(values2 as *mut ());
        }
        return 0;
    }
    unsafe {
        memset(
            values2 as *mut (),
            0,
            ((*ns2).nodeNr as u64).wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as u64),
        )
    };
    i = 0;
    while i < safe_ns1.nodeNr {
        unsafe {
            *hashs1.offset(i as isize) = xmlXPathNodeValHash(*(*ns1).nodeTab.offset(i as isize))
        };
        j = 0;
        while j < safe_ns2.nodeNr {
            if i == 0 {
                unsafe {
                    *hashs2.offset(j as isize) =
                        xmlXPathNodeValHash(*(*ns2).nodeTab.offset(j as isize))
                }
            }
            if unsafe { *hashs1.offset(i as isize) != *hashs2.offset(j as isize) } {
                if neq != 0 {
                    ret = 1;
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
    i = 0;
    while i < safe_ns1.nodeNr {
        if unsafe { !(*values1.offset(i as isize)).is_null() } {
            unsafe {
                xmlFree.expect("non-null function pointer")(*values1.offset(i as isize) as *mut ())
            };
        }
        i += 1
    }
    j = 0;
    while j < safe_ns2.nodeNr {
        if unsafe { !(*values2.offset(j as isize)).is_null() } {
            unsafe {
                xmlFree.expect("non-null function pointer")(*values2.offset(j as isize) as *mut ())
            };
        }
        j += 1
    }
    unsafe {
        xmlFree.expect("non-null function pointer")(values1 as *mut ());
        xmlFree.expect("non-null function pointer")(values2 as *mut ());
        xmlFree.expect("non-null function pointer")(hashs1 as *mut ());
        xmlFree.expect("non-null function pointer")(hashs2 as *mut ());
    }
    return ret;
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathEqualValuesCommon(
    ctxt: xmlXPathParserContextPtr,
    mut arg1: xmlXPathObjectPtr,
    mut arg2: xmlXPathObjectPtr,
) -> i32 {
    let mut ret: i32 = 0;
    /*
     *At this point we are assured neither arg1 nor arg2
     *is a nodeset, so we can just pick the appropriate routine.
     */
    let safe_arg1 = unsafe { &mut *arg1 };
    let safe_arg2 = unsafe { &mut *arg2 };
    match safe_arg1.type_0 as u32 {
        2 => match safe_arg2.type_0 as u32 {
            2 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Equal: undefined\n" as *const u8 as *const i8,
                    )
                };
                ret = (safe_arg1.boolval == safe_arg2.boolval) as i32
            }
            3 => {
                ret = (safe_arg1.boolval
                    == unsafe { xmlXPathCastNumberToBoolean((*arg2).floatval) })
                    as i32
            }
            4 => {
                if safe_arg2.stringval.is_null()
                    || unsafe { *(*arg2).stringval.offset(0) as i32 == 0 }
                {
                    ret = 0
                } else {
                    ret = 1
                }
                ret = (safe_arg1.boolval == ret) as i32
            }
            8 | 5 | 6 | 7 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                        b"xpath.c\x00" as *const u8 as *const i8,
                        6991,
                    )
                };
            }
            0 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Equal: undefined\n" as *const u8 as *const i8,
                    )
                };
            }
            1 | 9 | _ => {}
        },
        3 => {
            let mut current_block_37: u64;
            match safe_arg2.type_0 as u32 {
                2 => {
                    ret = unsafe {
                        ((*arg2).boolval == xmlXPathCastNumberToBoolean((*arg1).floatval))
                    } as i32;
                    current_block_37 = 14220266465818359136;
                }
                4 => {
                    unsafe {
                        valuePush(ctxt, arg2);
                        xmlXPathNumberFunction(ctxt, 1);
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
                            b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                            b"xpath.c\x00" as *const u8 as *const i8,
                            7048,
                        )
                    };
                    current_block_37 = 14220266465818359136;
                }
                0 => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Equal: undefined\n" as *const u8 as *const i8,
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
                        ret = 0
                    } else if unsafe { xmlXPathIsInf((*arg1).floatval) } == 1 {
                        if unsafe { xmlXPathIsInf((*arg2).floatval) } == 1 {
                            ret = 1
                        } else {
                            ret = 0
                        }
                    } else if unsafe { xmlXPathIsInf((*arg1).floatval) == -1 } {
                        if unsafe { xmlXPathIsInf((*arg2).floatval) == -1 } {
                            ret = 1
                        } else {
                            ret = 0
                        }
                    } else if unsafe { xmlXPathIsInf((*arg2).floatval) == 1 } {
                        if unsafe { xmlXPathIsInf((*arg1).floatval) == 1 } {
                            ret = 1
                        } else {
                            ret = 0
                        }
                    } else if unsafe { xmlXPathIsInf((*arg2).floatval) == -1 } {
                        if unsafe { xmlXPathIsInf((*arg1).floatval) == -1 } {
                            ret = 1
                        } else {
                            ret = 0
                        }
                    } else {
                        ret = (safe_arg1.floatval == safe_arg2.floatval) as i32
                    }
                }
                _ => {}
            }
        }
        4 => {
            match safe_arg2.type_0 as u32 {
                2 => {
                    if safe_arg1.stringval.is_null()
                        || unsafe { *(*arg1).stringval.offset(0) as i32 == 0 }
                    {
                        ret = 0
                    } else {
                        ret = 1
                    }
                    ret = (safe_arg2.boolval == ret) as i32
                }
                4 => ret = unsafe { xmlStrEqual((*arg1).stringval, (*arg2).stringval) },
                3 => {
                    unsafe {
                        valuePush(ctxt, arg1);
                        xmlXPathNumberFunction(ctxt, 1);
                        arg1 = valuePop(ctxt);
                    }
                    /* Hand check NaN and Infinity equalities */
                    if unsafe {
                        xmlXPathIsNaN((*arg1).floatval) != 0 || xmlXPathIsNaN((*arg2).floatval) != 0
                    } {
                        ret = 0
                    } else if unsafe { xmlXPathIsInf((*arg1).floatval) } == 1 {
                        if unsafe { xmlXPathIsInf((*arg2).floatval) } == 1 {
                            ret = 1
                        } else {
                            ret = 0
                        }
                    } else if unsafe { xmlXPathIsInf((*arg1).floatval) } == -1 {
                        if unsafe { xmlXPathIsInf((*arg2).floatval) } == -1 {
                            ret = 1
                        } else {
                            ret = 0
                        }
                    } else if unsafe { xmlXPathIsInf((*arg2).floatval) } == 1 {
                        if unsafe { xmlXPathIsInf((*arg1).floatval) } == 1 {
                            ret = 1
                        } else {
                            ret = 0
                        }
                    } else if unsafe { xmlXPathIsInf((*arg2).floatval) } == -1 {
                        if unsafe { xmlXPathIsInf((*arg1).floatval) } == -1 {
                            ret = 1
                        } else {
                            ret = 0
                        }
                    } else {
                        ret = (safe_arg1.floatval == safe_arg2.floatval) as i32
                    }
                }
                8 | 5 | 6 | 7 => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                            b"xpath.c\x00" as *const u8 as *const i8,
                            7109,
                        )
                    };
                }
                1 | 9 | _ => {}
                0 => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Equal: undefined\n" as *const u8 as *const i8,
                        )
                    };
                }
            }
        }
        8 | 5 | 6 | 7 => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                    b"xpath.c\x00" as *const u8 as *const i8,
                    7120,
                )
            };
        }
        0 | 1 | 9 | _ => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Equal: undefined\n" as *const u8 as *const i8,
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
pub fn xmlXPathEqualValues(ctxt: xmlXPathParserContextPtr) -> i32 {
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut argtmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: i32 = 0;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0;
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
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
        return 0;
    }
    if arg1 == arg2 {
        match () {
            #[cfg(DEBUG_EXPR)]
            _ => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"NotEqual: by pointer\n\x00" as *const u8 as *const i8,
                    )
                };
            }
            #[cfg(not(DEBUG_EXPR))]
            _ => {}
        };
        unsafe { xmlXPathFreeObject(arg1) };
        return 1;
    }
    /*
     *If either argument is a nodeset, it's a 'special case'
     */
    let safe_arg1 = unsafe { &mut *arg1 };
    let safe_arg2 = unsafe { &mut *arg2 };
    if safe_arg2.type_0 as u32 == XPATH_NODESET as u32
        || safe_arg2.type_0 as u32 == XPATH_XSLT_TREE as u32
        || safe_arg1.type_0 as u32 == XPATH_NODESET as u32
        || safe_arg1.type_0 as u32 == XPATH_XSLT_TREE as u32
    {
        /*
         *Hack it to assure arg1 is the nodeset
         */
        if safe_arg1.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg1.type_0 as u32 != XPATH_XSLT_TREE as u32
        {
            argtmp = arg2;
            arg2 = arg1;
            arg1 = argtmp
        }
        match safe_arg2.type_0 as u32 {
            1 | 9 => ret = unsafe { xmlXPathEqualNodeSets(arg1, arg2, 0) },
            2 => {
                if safe_arg1.nodesetval.is_null() || unsafe { (*(*arg1).nodesetval).nodeNr == 0 } {
                    ret = 0
                } else {
                    ret = 1
                }
                ret = (ret == safe_arg2.boolval) as i32
            }
            3 => ret = unsafe { xmlXPathEqualNodeSetFloat(ctxt, arg1, (*arg2).floatval, 0) },
            4 => ret = unsafe { xmlXPathEqualNodeSetString(arg1, (*arg2).stringval, 0) },
            8 | 5 | 6 | 7 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                        b"xpath.c\x00" as *const u8 as *const i8,
                        7205,
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
                                b"NotEqual: undefined\n\x00" as *const u8 as *const i8,
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
pub fn xmlXPathNotEqualValues(ctxt: xmlXPathParserContextPtr) -> i32 {
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut argtmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: i32 = 0;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0;
    }
    arg2 = unsafe { valuePop(ctxt) };
    arg1 = unsafe { valuePop(ctxt) };
    if arg1.is_null() || arg2.is_null() {
        if !arg1.is_null() {
            unsafe { xmlXPathReleaseObject((*ctxt).context, arg1) };
        } else {
            unsafe { xmlXPathReleaseObject((*ctxt).context, arg2) };
        }
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
        return 0;
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
                        b"NotEqual: by pointer\n\x00" as *const u8 as *const i8,
                    )
                };
            }
            #[cfg(not(DEBUG_EXPR))]
            _ => {}
        };
        unsafe { xmlXPathReleaseObject((*ctxt).context, arg1) };
        return 0;
    }
    /*
     *If either argument is a nodeset, it's a 'special case'
     */
    if safe_arg2.type_0 as u32 == XPATH_NODESET as u32
        || safe_arg2.type_0 as u32 == XPATH_XSLT_TREE as u32
        || safe_arg1.type_0 as u32 == XPATH_NODESET as u32
        || safe_arg1.type_0 as u32 == XPATH_XSLT_TREE as u32
    {
        /*
         *Hack it to assure arg1 is the nodeset
         */
        if safe_arg1.type_0 as u32 != XPATH_NODESET as u32
            && safe_arg1.type_0 as u32 != XPATH_XSLT_TREE as u32
        {
            argtmp = arg2;
            arg2 = arg1;
            arg1 = argtmp
        }
        match safe_arg2.type_0 as u32 {
            1 | 9 => ret = unsafe { xmlXPathEqualNodeSets(arg1, arg2, 1) },
            2 => {
                if safe_arg1.nodesetval.is_null() || unsafe { (*(*arg1).nodesetval).nodeNr == 0 } {
                    ret = 0
                } else {
                    ret = 1
                }
                ret = (ret != safe_arg2.boolval) as i32
            }
            3 => ret = unsafe { xmlXPathEqualNodeSetFloat(ctxt, arg1, (*arg2).floatval, 1) },
            4 => ret = unsafe { xmlXPathEqualNodeSetString(arg1, (*arg2).stringval, 1) },
            8 | 5 | 6 | 7 => {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                        b"xpath.c\x00" as *const u8 as *const i8,
                        7290,
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
                                b"NotEqual: undefined\n\x00" as *const u8 as *const i8,
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
    return unsafe { (xmlXPathEqualValuesCommon(ctxt, arg1, arg2) == 0) as i32 };
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
pub fn xmlXPathCompareValues(ctxt: xmlXPathParserContextPtr, inf: i32, strict: i32) -> i32 {
    let mut ret: i32 = 0;
    let mut arg1i: i32 = 0;
    let mut arg2i: i32 = 0;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0;
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
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
        return 0;
    }
    if safe_arg2.type_0 as u32 == XPATH_NODESET as u32
        || safe_arg2.type_0 as u32 == XPATH_XSLT_TREE as u32
        || safe_arg1.type_0 as u32 == XPATH_NODESET as u32
        || safe_arg1.type_0 as u32 == XPATH_XSLT_TREE as u32
    {
        /*
         * If either argument is a XPATH_NODESET or XPATH_XSLT_TREE the two arguments
         * are not freed from within this routine; they will be freed from the
         * called routine, e.g. xmlXPathCompareNodeSets or xmlXPathCompareNodeSetValue
         */
        if (safe_arg2.type_0 as u32 == XPATH_NODESET as u32
            || safe_arg2.type_0 as u32 == XPATH_XSLT_TREE as u32)
            && (safe_arg1.type_0 as u32 == XPATH_NODESET as u32
                || safe_arg1.type_0 as u32 == XPATH_XSLT_TREE as u32)
        {
            ret = unsafe { xmlXPathCompareNodeSets(inf, strict, arg1, arg2) }
        } else if safe_arg1.type_0 as u32 == XPATH_NODESET as u32
            || safe_arg1.type_0 as u32 == XPATH_XSLT_TREE as u32
        {
            ret = unsafe { xmlXPathCompareNodeSetValue(ctxt, inf, strict, arg1, arg2) }
        } else {
            ret =
                unsafe { xmlXPathCompareNodeSetValue(ctxt, (inf == 0) as i32, strict, arg2, arg1) }
        }
        return ret;
    }
    if safe_arg1.type_0 as u32 != XPATH_NUMBER as u32 {
        unsafe {
            valuePush(ctxt, arg1);
            xmlXPathNumberFunction(ctxt, 1);
            arg1 = valuePop(ctxt)
        }
    }
    if unsafe { (*arg1).type_0 as u32 != XPATH_NUMBER as u32 } {
        unsafe {
            xmlXPathFreeObject(arg1);
            xmlXPathFreeObject(arg2);
            xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32);
        }
        return 0;
    }
    if safe_arg2.type_0 as u32 != XPATH_NUMBER as u32 {
        unsafe {
            valuePush(ctxt, arg2);
            xmlXPathNumberFunction(ctxt, 1);
            arg2 = valuePop(ctxt)
        }
    }
    if unsafe { (*arg2).type_0 as u32 != XPATH_NUMBER as u32 } {
        unsafe {
            xmlXPathReleaseObject((*ctxt).context, arg1);
            xmlXPathReleaseObject((*ctxt).context, arg2);
            xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32);
        }
        return 0;
    }
    /*
     * Add tests for infinity and nan
     * => feedback on 3.4 for Inf and NaN
     */
    /* Hand check NaN and Infinity comparisons */
    if unsafe { xmlXPathIsNaN((*arg1).floatval) != 0 || xmlXPathIsNaN((*arg2).floatval) != 0 } {
        ret = 0
    } else {
        arg1i = unsafe { xmlXPathIsInf((*arg1).floatval) };
        arg2i = unsafe { xmlXPathIsInf((*arg2).floatval) };
        if inf != 0 && strict != 0 {
            if arg1i == -1 && arg2i != -1 || arg2i == 1 && arg1i != 1 {
                ret = 1
            } else if arg1i == 0 && arg2i == 0 {
                ret = unsafe { ((*arg1).floatval < (*arg2).floatval) as i32 }
            } else {
                ret = 0
            }
        } else if inf != 0 && strict == 0 {
            if arg1i == -1 || arg2i == 1 {
                ret = 1
            } else if arg1i == 0 && arg2i == 0 {
                ret = unsafe { ((*arg1).floatval <= (*arg2).floatval) as i32 }
            } else {
                ret = 0
            }
        } else if inf == 0 && strict != 0 {
            if arg1i == 1 && arg2i != 1 || arg2i == -1 && arg1i != -1 {
                ret = 1
            } else if arg1i == 0 && arg2i == 0 {
                ret = unsafe { ((*arg1).floatval > (*arg2).floatval) as i32 }
            } else {
                ret = 0
            }
        } else if inf == 0 && strict == 0 {
            if arg1i == 1 || arg2i == -1 {
                ret = 1
            } else if arg1i == 0 && arg2i == 0 {
                ret = unsafe { ((*arg1).floatval >= (*arg2).floatval) as i32 }
            } else {
                ret = 0
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
pub fn xmlXPathValueFlipSign(ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null() || unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
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
pub fn xmlXPathAddValues(ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = unsafe { valuePop(ctxt) };
    if arg.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
        return;
    }
    unsafe {
        val = xmlXPathCastToNumber(arg);
        xmlXPathReleaseObject((*ctxt).context, arg);
    };
    if !safe_ctxt.value.is_null()
        && unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null() || unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
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
pub fn xmlXPathSubValues(ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = unsafe { valuePop(ctxt) };
    if arg.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
        return;
    }
    unsafe {
        val = xmlXPathCastToNumber(arg);
        xmlXPathReleaseObject((*ctxt).context, arg);
    };
    if !safe_ctxt.value.is_null()
        && unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null() || unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
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
pub fn xmlXPathMultValues(ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = unsafe { valuePop(ctxt) };
    if arg.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
        return;
    }
    unsafe {
        val = xmlXPathCastToNumber(arg);
        xmlXPathReleaseObject((*ctxt).context, arg);
    };
    if !safe_ctxt.value.is_null()
        && unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null() || unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
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
pub fn xmlXPathDivValues(ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = unsafe { valuePop(ctxt) };
    if arg.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
        return;
    }
    unsafe {
        val = xmlXPathCastToNumber(arg);
        xmlXPathReleaseObject((*ctxt).context, arg);
    };
    if !safe_ctxt.value.is_null()
        && unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null() || unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
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
pub fn xmlXPathModValues(ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg1: libc::c_double = 0.;
    let mut arg2: libc::c_double = 0.;
    arg = unsafe { valuePop(ctxt) };
    if arg.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
        return;
    }
    unsafe {
        arg2 = xmlXPathCastToNumber(arg);
        xmlXPathReleaseObject((*ctxt).context, arg);
    };
    if !safe_ctxt.value.is_null()
        && unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null() || unsafe { (*(*ctxt).value).type_0 as u32 != XPATH_NUMBER as u32 }
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    arg1 = unsafe { (*(*ctxt).value).floatval };
    if arg2 == 0 as i32 as libc::c_double {
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
pub extern "C" fn xmlXPathNextSelf(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr {
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
pub extern "C" fn xmlXPathNextChild(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr {
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
            _ => match unsafe { (*(*(*ctxt).context).node).type_0 as u32 } {
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
            _ => match unsafe { (*(*(*ctxt).context).node).type_0 as u32 } {
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
    if safe_cur.type_0 as u32 == XML_DOCUMENT_NODE as u32
        || safe_cur.type_0 as u32 == XML_HTML_DOCUMENT_NODE as u32
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
    ctxt: xmlXPathParserContextPtr,
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
                match unsafe { (*cur).type_0 as u32 } {
                    1 | 11 | 5 | 6 => {
                        /* URGENT TODO: entify-refs as well? */
                        cur = unsafe { (*cur).children };
                        if !cur.is_null() {
                            if unsafe { (*cur).type_0 as u32 } == XML_ELEMENT_NODE as u32 {
                                return cur;
                            }
                            loop {
                                cur = unsafe { (*cur).next };
                                if !(!cur.is_null()
                                    && unsafe { (*cur).type_0 as u32 } != XML_ELEMENT_NODE as u32)
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
                match unsafe { (*cur).type_0 as u32 } {
                    1 | 11 | 5 | 6 => {
                        /* URGENT TODO: entify-refs as well? */
                        cur = unsafe { (*cur).children };
                        if !cur.is_null() {
                            if unsafe { (*cur).type_0 as u32 } == XML_ELEMENT_NODE as u32 {
                                return cur;
                            }
                            loop {
                                cur = unsafe { (*cur).next };
                                if !(!cur.is_null()
                                    && unsafe { (*cur).type_0 as u32 } != XML_ELEMENT_NODE as u32)
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
    match unsafe { (*cur).type_0 as u32 } {
        1 | 3 | 5 | 6 | 4 | 7 | 8 | 20 => {}
        _ => {
            /* case XML_DTD_NODE: */
            /* URGENT TODO: DTD-node as well? */
            return 0 as xmlNodePtr;
        }
    }
    if !unsafe { (*cur).next.is_null() } {
        if unsafe { (*(*cur).next).type_0 as u32 } == XML_ELEMENT_NODE as u32 {
            return unsafe { (*cur).next };
        }
        cur = unsafe { (*cur).next };
        loop {
            cur = unsafe { (*cur).next };
            if !(!cur.is_null() && unsafe { (*cur).type_0 as u32 } != XML_ELEMENT_NODE as u32) {
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
    ctxt: xmlXPathParserContextPtr,
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
        if unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } == XML_ATTRIBUTE_NODE as u32
            || unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } == XML_NAMESPACE_DECL as u32
        {
            return 0 as xmlNodePtr;
        }
        if unsafe { (*safe_ctxt.context).node == (*safe_ctxt.context).doc as xmlNodePtr } {
            return unsafe { (*(*safe_ctxt.context).doc).children };
        }
        return unsafe { (*(*safe_ctxt.context).node).children };
    }
    if unsafe { (*cur).type_0 as u32 } == XML_NAMESPACE_DECL as u32 {
        return 0 as xmlNodePtr;
    }
    if !unsafe { (*cur).children.is_null() } {
        /*
         * Do not descend on entities declarations
         */
        if unsafe { (*(*cur).children).type_0 as u32 } != XML_ENTITY_DECL as u32 {
            unsafe { cur = (*cur).children };
            /*
             * Skip DTDs
             */
            if unsafe { (*cur).type_0 as u32 } != XML_DTD_NODE as u32 {
                return cur;
            }
        }
    }
    if cur == unsafe { (*safe_ctxt.context).node } {
        return 0 as xmlNodePtr;
    }
    while !unsafe { (*cur).next.is_null() } {
        unsafe { cur = (*cur).next };
        if unsafe { (*cur).type_0 as u32 } != XML_ENTITY_DECL as u32
            && unsafe { (*cur).type_0 as u32 } != XML_DTD_NODE as u32
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
    ctxt: xmlXPathParserContextPtr,
    cur: xmlNodePtr,
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
    if unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } == XML_ATTRIBUTE_NODE as u32
        || unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } == XML_NAMESPACE_DECL as u32
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
    ctxt: xmlXPathParserContextPtr,
    cur: xmlNodePtr,
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
        match unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } {
            1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 15 | 16 | 19 | 20 | 17 => {
                if unsafe { (*(*safe_ctxt.context).node).parent.is_null() } {
                    return unsafe { (*safe_ctxt.context).doc as xmlNodePtr };
                }
                if unsafe {
                    (*(*(*safe_ctxt.context).node).parent).type_0 as u32 == XML_ELEMENT_NODE as u32
                } && unsafe {
                    (*(*(*(*safe_ctxt.context).node).parent).name.offset(0) as i32 == ' ' as i32
                        || xmlStrEqual(
                            (*(*(*safe_ctxt.context).node).parent).name,
                            b"fake node libxslt\x00" as *const u8 as *const i8 as *mut xmlChar,
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
                    && unsafe { (*(*ns).next).type_0 as u32 } != XML_NAMESPACE_DECL as u32
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
    ctxt: xmlXPathParserContextPtr,
    cur: xmlNodePtr,
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
        match unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } {
            1 | 3 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 16 | 17 | 12 | 19 | 20 => {
                if unsafe { (*(*safe_ctxt.context).node).parent.is_null() } {
                    return unsafe { (*safe_ctxt.context).doc as xmlNodePtr };
                }
                if unsafe {
                    (*(*(*safe_ctxt.context).node).parent).type_0 as u32 == XML_ELEMENT_NODE as u32
                } && unsafe {
                    (*(*(*(*safe_ctxt.context).node).parent).name.offset(0) as i32 == ' ' as i32
                        || xmlStrEqual(
                            (*(*(*safe_ctxt.context).node).parent).name,
                            b"fake node libxslt\x00" as *const u8 as *const i8 as *mut xmlChar,
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
                    && unsafe { (*safe_ns.next).type_0 as u32 } != XML_NAMESPACE_DECL as u32
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
    match unsafe { (*cur).type_0 as u32 } {
        1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 15 | 16 | 17 | 19 | 20 => {
            if unsafe { (*cur).parent.is_null() } {
                return 0 as xmlNodePtr;
            }
            if unsafe { (*(*cur).parent).type_0 as u32 } == XML_ELEMENT_NODE as u32
                && unsafe {
                    (*(*(*cur).parent).name.offset(0) as i32 == ' ' as i32
                        || xmlStrEqual(
                            (*(*cur).parent).name,
                            b"fake node libxslt\x00" as *const u8 as *const i8 as *mut xmlChar,
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
                && unsafe { (*(*ns_0).next).type_0 as u32 != XML_NAMESPACE_DECL as u32 }
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
    ctxt: xmlXPathParserContextPtr,
    cur: xmlNodePtr,
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
    ctxt: xmlXPathParserContextPtr,
    cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } == XML_ATTRIBUTE_NODE as u32
        || unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } == XML_NAMESPACE_DECL as u32
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
    ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } == XML_ATTRIBUTE_NODE as u32
        || unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } == XML_NAMESPACE_DECL as u32
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
        && unsafe { (*(*cur).prev).type_0 as u32 } == XML_DTD_NODE as u32
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
    ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    } /* ERROR */

    if !cur.is_null()
        && unsafe {
            (*cur).type_0 as u32 != XML_ATTRIBUTE_NODE as u32
                && (*cur).type_0 as u32 != XML_NAMESPACE_DECL as u32
                && !(*cur).children.is_null()
        }
    {
        return unsafe { (*cur).children };
    }
    if cur.is_null() {
        cur = unsafe { (*safe_ctxt.context).node };
        if unsafe { (*cur).type_0 as u32 } == XML_ATTRIBUTE_NODE as u32 {
            unsafe { cur = (*cur).parent }
        } else if unsafe { (*cur).type_0 as u32 } == XML_NAMESPACE_DECL as u32 {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if unsafe { (*ns).next.is_null() }
                || unsafe { (*(*ns).next).type_0 as u32 } == XML_NAMESPACE_DECL as u32
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
fn xmlXPathIsAncestor(ancestor: xmlNodePtr, mut node: xmlNodePtr) -> i32 {
    let safe_node = unsafe { &mut *node };
    if ancestor.is_null() || node.is_null() {
        return 0;
    }
    if safe_node.type_0 as u32 == XML_NAMESPACE_DECL as u32 {
        return 0;
    }
    if unsafe { (*ancestor).type_0 as u32 } == XML_NAMESPACE_DECL as u32 {
        return 0;
    }
    /* nodes need to be in the same document */
    if unsafe { (*ancestor).doc != safe_node.doc } {
        return 0;
    }
    /* avoid searching if ancestor or node is the root node */
    if ancestor == safe_node.doc as xmlNodePtr {
        return 1;
    }
    if node == unsafe { (*ancestor).doc as xmlNodePtr } {
        return 0;
    }
    while !safe_node.parent.is_null() {
        if safe_node.parent == ancestor {
            return 1;
        }
        node = safe_node.parent
    }
    return 0;
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
pub fn xmlXPathNextPreceding(ctxt: xmlXPathParserContextPtr, mut cur: xmlNodePtr) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        cur = unsafe { (*safe_ctxt.context).node };
        if unsafe { (*cur).type_0 as u32 } == XML_ATTRIBUTE_NODE as u32 {
            cur = unsafe { (*cur).parent }
        } else if unsafe { (*cur).type_0 as u32 } == XML_NAMESPACE_DECL as u32 {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if unsafe { (*ns).next.is_null() }
                || unsafe { (*(*ns).next).type_0 as u32 } == XML_NAMESPACE_DECL as u32
            {
                return 0 as xmlNodePtr;
            }
            cur = unsafe { (*ns).next as xmlNodePtr }
        }
    }
    if cur.is_null() || unsafe { (*cur).type_0 as u32 } == XML_NAMESPACE_DECL as u32 {
        return 0 as xmlNodePtr;
    }
    if !unsafe { (*cur).prev.is_null() }
        && unsafe { (*(*cur).prev).type_0 as u32 } == XML_DTD_NODE as u32
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
    ctxt: xmlXPathParserContextPtr,
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
        if unsafe { (*cur).type_0 as u32 } == XML_ATTRIBUTE_NODE as u32 {
            cur = unsafe { (*cur).parent }
        } else if unsafe { (*cur).type_0 as u32 } == XML_NAMESPACE_DECL as u32 {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if unsafe {
                (*ns).next.is_null() || (*(*ns).next).type_0 as u32 == XML_NAMESPACE_DECL as u32
            } {
                return 0 as xmlNodePtr;
            }
            cur = unsafe { (*ns).next as xmlNodePtr }
        }
        safe_ctxt.ancestor = unsafe { (*cur).parent }
    }
    if unsafe { (*cur).type_0 as u32 } == XML_NAMESPACE_DECL as u32 {
        return 0 as xmlNodePtr;
    }
    if !unsafe { (*cur).prev.is_null() }
        && unsafe { (*(*cur).prev).type_0 as u32 } == XML_DTD_NODE as u32
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
    ctxt: xmlXPathParserContextPtr,
    cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } != XML_ELEMENT_NODE as u32 {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if !unsafe { (*safe_ctxt.context).tmpNsList.is_null() } {
            unsafe {
                xmlFree.expect("non-null function pointer")(
                    (*safe_ctxt.context).tmpNsList as *mut (),
                )
            };
        }
        unsafe {
            (*safe_ctxt.context).tmpNsList = xmlGetNsList(
                (*safe_ctxt.context).doc as *const xmlDoc,
                (*safe_ctxt.context).node as *const xmlNode,
            )
        };
        unsafe { (*safe_ctxt.context).tmpNsNr = 0 };
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
    if unsafe { (*safe_ctxt.context).tmpNsNr > 0 } {
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
                    (*safe_ctxt.context).tmpNsList as *mut (),
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
    ctxt: xmlXPathParserContextPtr,
    cur: xmlNodePtr,
) -> xmlNodePtr {
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.context.is_null() {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*safe_ctxt.context).node.is_null() } {
        return 0 as xmlNodePtr;
    }
    if unsafe { (*(*safe_ctxt.context).node).type_0 as u32 } != XML_ELEMENT_NODE as u32 {
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
pub fn xmlXPathRoot(ctxt: xmlXPathParserContextPtr) {
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
pub extern "C" fn xmlXPathLastFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 0 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if unsafe { (*safe_ctxt.context).contextSize } >= 0 {
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
                        b"last() : %d\n\x00" as *const u8 as *const i8,
                        (*safe_ctxt.context).contextSize,
                    )
                };
            }
            #[cfg(not(DEBUG_EXPR))]
            _ => {}
        };
    } else {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_CTXT_SIZE as i32) };
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
pub extern "C" fn xmlXPathPositionFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 0 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if unsafe { (*safe_ctxt.context).proximityPosition } >= 0 {
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
                        b"position() : %d\n\x00" as *const u8 as *const i8,
                        (*safe_ctxt.context).proximityPosition,
                    )
                };
            }
            #[cfg(not(DEBUG_EXPR))]
            _ => {}
        };
    } else {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_CTXT_POSITION as i32) };
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
pub extern "C" fn xmlXPathCountFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NODESET as u32
            && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_XSLT_TREE as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    if cur.is_null() || unsafe { (*cur).nodesetval.is_null() } {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewFloat(safe_ctxt.context, 0 as i32 as libc::c_double),
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
fn xmlXPathGetElementsByIds(doc: xmlDocPtr, mut ids: *const xmlChar) -> xmlNodeSetPtr {
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
        *cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *cur as i32 && *cur as i32 <= 0xa as i32
            || *cur as i32 == 0xd as i32
    } {
        unsafe { cur = cur.offset(1) }
    }
    while unsafe { *cur as i32 != 0 } {
        while !unsafe {
            (*cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *cur as i32 && *cur as i32 <= 0xa as i32
                || *cur as i32 == 0xd as i32)
                && *cur as i32 != 0
        } {
            unsafe { cur = cur.offset(1) }
        }
        ID = unsafe { xmlStrndup(ids, cur.offset_from(ids) as i32) };
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
                if safe_attr.type_0 as u32 == XML_ATTRIBUTE_NODE as u32 {
                    elem = safe_attr.parent
                } else if safe_attr.type_0 as u32 == XML_ELEMENT_NODE as u32 {
                    elem = attr as xmlNodePtr
                } else {
                    elem = 0 as xmlNodePtr
                }
                /* TODO: Check memory error. */
                if !elem.is_null() {
                    unsafe { xmlXPathNodeSetAdd(ret, elem) };
                }
            }
            unsafe { xmlFree.expect("non-null function pointer")(ID as *mut ()) };
        }
        while unsafe {
            *cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *cur as i32 && *cur as i32 <= 0xa as i32
                || *cur as i32 == 0xd as i32
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
pub extern "C" fn xmlXPathIdFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut tokens: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    obj = unsafe { valuePop(ctxt) };
    if obj.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
        return;
    }
    let safe_obj = unsafe { &mut *obj };
    if safe_obj.type_0 as u32 == XPATH_NODESET as u32
        || safe_obj.type_0 as u32 == XPATH_XSLT_TREE as u32
    {
        let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
        let mut i: i32;
        /* TODO: Check memory error. */
        ret = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
        if !safe_obj.nodesetval.is_null() {
            i = 0;
            while i < unsafe { (*safe_obj.nodesetval).nodeNr } {
                tokens = unsafe {
                    xmlXPathCastNodeToString(*(*safe_obj.nodesetval).nodeTab.offset(i as isize))
                };
                ns = unsafe { xmlXPathGetElementsByIds((*safe_ctxt.context).doc, tokens) };
                /* TODO: Check memory error. */
                ret = unsafe { xmlXPathNodeSetMerge(ret, ns) };
                unsafe { xmlXPathFreeNodeSet(ns) };
                if !tokens.is_null() {
                    unsafe { xmlFree.expect("non-null function pointer")(tokens as *mut ()) };
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
pub extern "C" fn xmlXPathLocalNameFunction(ctxt: xmlXPathParserContextPtr, mut nargs: i32) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject; /* Should be first in document order !!!!! */
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
            )
        };
        nargs = 1
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NODESET as u32
            && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_XSLT_TREE as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    if unsafe { (*cur).nodesetval.is_null() } || unsafe { (*(*cur).nodesetval).nodeNr == 0 } {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewCString(safe_ctxt.context, b"\x00" as *const u8 as *const i8),
            )
        };
    } else {
        let mut i: i32 = 0;
        match unsafe { (**(*(*cur).nodesetval).nodeTab.offset(i as isize)).type_0 as u32 } {
            1 | 2 | 7 => {
                if unsafe {
                    *(**(*(*cur).nodesetval).nodeTab.offset(i as isize))
                        .name
                        .offset(0) as i32
                        == ' ' as i32
                } {
                    unsafe {
                        valuePush(
                            ctxt,
                            xmlXPathCacheNewCString(
                                safe_ctxt.context,
                                b"\x00" as *const u8 as *const i8,
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
                            b"\x00" as *const u8 as *const i8,
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
pub extern "C" fn xmlXPathNamespaceURIFunction(ctxt: xmlXPathParserContextPtr, mut nargs: i32) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject; /* Should be first in document order !!!!! */
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
            )
        };
        nargs = 1
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NODESET as u32
            && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_XSLT_TREE as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    let safe_cur = unsafe { &mut *cur };
    if safe_cur.nodesetval.is_null() || unsafe { (*safe_cur.nodesetval).nodeNr == 0 } {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewCString(safe_ctxt.context, b"\x00" as *const u8 as *const i8),
            )
        };
    } else {
        let mut i: i32 = 0;
        match unsafe { (**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).type_0 as u32 } {
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
                                b"\x00" as *const u8 as *const i8,
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
                            b"\x00" as *const u8 as *const i8,
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
extern "C" fn xmlXPathNameFunction(ctxt: xmlXPathParserContextPtr, mut nargs: i32) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject; /* Should be first in document order !!!!! */
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
            )
        };
        nargs = 1
    }

    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NODESET as u32
            && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_XSLT_TREE as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    let safe_cur = unsafe { &mut *cur };
    if safe_cur.nodesetval.is_null() || unsafe { (*safe_cur.nodesetval).nodeNr == 0 } {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewCString(safe_ctxt.context, b"\x00" as *const u8 as *const i8),
            )
        };
    } else {
        let mut i: i32 = 0;
        match unsafe { (**(*safe_cur.nodesetval).nodeTab.offset(i as isize)).type_0 as u32 } {
            1 | 2 => {
                if unsafe {
                    *(**(*safe_cur.nodesetval).nodeTab.offset(i as isize))
                        .name
                        .offset(0) as i32
                        == ' ' as i32
                } {
                    unsafe {
                        valuePush(
                            ctxt,
                            xmlXPathCacheNewCString(
                                safe_ctxt.context,
                                b"\x00" as *const u8 as *const i8,
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
                            0,
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
                        unsafe { xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as i32) };
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
                unsafe { xmlXPathLocalNameFunction(ctxt, 1) };
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
pub extern "C" fn xmlXPathStringFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 {
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
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    if cur.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
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
pub extern "C" fn xmlXPathStringLengthFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 {
        if ctxt.is_null() || safe_ctxt.context.is_null() {
            return;
        }
        if unsafe { (*safe_ctxt.context).node.is_null() } {
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewFloat(safe_ctxt.context, 0 as libc::c_double),
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
            unsafe { xmlFree.expect("non-null function pointer")(content as *mut ()) };
        }
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
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
pub extern "C" fn xmlXPathConcatFunction(ctxt: xmlXPathParserContextPtr, mut nargs: i32) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut newobj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs < 2 {
        if ctxt.is_null() {
            return;
        }
        if nargs != 2 {
            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
            return;
        }

        if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 {
            unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
            return;
        }
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    cur = unsafe { valuePop(ctxt) };
    if cur.is_null() || unsafe { (*cur).type_0 as u32 } != XPATH_STRING as u32 {
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, cur) };
        return;
    }
    nargs -= 1;
    while nargs > 0 {
        if !safe_ctxt.value.is_null()
            && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
        {
            unsafe { xmlXPathStringFunction(ctxt, 1) };
        }
        newobj = unsafe { valuePop(ctxt) };
        if newobj.is_null() || unsafe { (*newobj).type_0 as u32 } != XPATH_STRING as u32 {
            unsafe {
                xmlXPathReleaseObject(safe_ctxt.context, newobj);
                xmlXPathReleaseObject(safe_ctxt.context, cur);
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32);
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
pub extern "C" fn xmlXPathContainsFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut hay: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut needle: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    needle = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    hay = unsafe { valuePop(ctxt) };
    if hay.is_null() || unsafe { (*hay).type_0 as u32 } != XPATH_STRING as u32 {
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, hay) };
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, needle) };
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    if !unsafe { xmlStrstr((*hay).stringval, (*needle).stringval).is_null() } {
        unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, 1)) };
    } else {
        unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, 0)) };
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
pub extern "C" fn xmlXPathStartsWithFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut hay: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut needle: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let n: i32;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    needle = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    hay = unsafe { valuePop(ctxt) };
    if hay.is_null() || unsafe { (*hay).type_0 as u32 } != XPATH_STRING as u32 {
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, hay) };
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, needle) };
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    n = unsafe { xmlStrlen((*needle).stringval) };
    if unsafe { xmlStrncmp((*hay).stringval, (*needle).stringval, n) } != 0 {
        unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, 0)) };
    } else {
        unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, 1)) };
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
pub extern "C" fn xmlXPathSubstringFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut start: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut len: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut le: libc::c_double = 0 as i32 as libc::c_double;
    let mut in_0: libc::c_double = 0.;
    let mut i: i32 = 1;
    let mut j: i32 = 2147483647 as i32;
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs < 2 {
        if ctxt.is_null() {
            return;
        }
        if nargs != 2 {
            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
            return;
        }
        if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 {
            unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
            return;
        }
    }
    if nargs > 3 {
        if ctxt.is_null() {
            return;
        }
        if nargs != 3 {
            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
            return;
        }
        if safe_ctxt.valueNr < safe_ctxt.valueFrame + 3 {
            unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
            return;
        }
    }
    /*
     * take care of possible last (position) argument
     */
    if nargs == 3 {
        if !safe_ctxt.value.is_null()
            && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NUMBER as u32
        {
            unsafe { xmlXPathNumberFunction(ctxt, 1) };
        }
        if safe_ctxt.value.is_null()
            || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NUMBER as u32
        {
            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
            return;
        }
        len = unsafe { valuePop(ctxt) };
        let safe_len = unsafe { &mut *len };
        le = safe_len.floatval;
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, len) };
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NUMBER as u32
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NUMBER as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    start = unsafe { valuePop(ctxt) };
    let safe_start = unsafe { &mut *start };
    in_0 = safe_start.floatval;
    unsafe { xmlXPathReleaseObject(safe_ctxt.context, start) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    str = unsafe { valuePop(ctxt) };
    if !(in_0 < 2147483647 as libc::c_double) {
        /* Logical NOT to handle NaNs */
        i = 2147483647 as i32
    } else if in_0 >= 1.0f64 {
        i = in_0 as i32;
        if in_0 - unsafe { floor(in_0) } >= 0.5f64 {
            i += 1
        }
    }
    if nargs == 3 {
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
            j = 1
        } else if end < 2147483647 as libc::c_double {
            j = end as i32
        }
    }
    if i < j {
        let mut ret: *mut xmlChar = unsafe { xmlUTF8Strsub((*str).stringval, i - 1, j - i) };
        unsafe { valuePush(ctxt, xmlXPathCacheNewString(safe_ctxt.context, ret)) };
        unsafe { xmlFree.expect("non-null function pointer")(ret as *mut ()) };
    } else {
        unsafe {
            valuePush(
                ctxt,
                xmlXPathCacheNewCString(safe_ctxt.context, b"\x00" as *const u8 as *const i8),
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
pub extern "C" fn xmlXPathSubstringBeforeFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut find: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut point: *const xmlChar = 0 as *const xmlChar;
    let offset: i32;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs != 2 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    find = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    str = unsafe { valuePop(ctxt) };
    target = unsafe { xmlBufCreate() };
    if !target.is_null() {
        point = unsafe { xmlStrstr((*str).stringval, (*find).stringval) };
        if !point.is_null() {
            offset = unsafe { point.offset_from((*str).stringval) as i32 };
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
pub extern "C" fn xmlXPathSubstringAfterFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut find: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut point: *const xmlChar = 0 as *const xmlChar;
    let offset: i32;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs != 2 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    find = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    str = unsafe { valuePop(ctxt) };
    target = unsafe { xmlBufCreate() };
    if !target.is_null() {
        point = unsafe { xmlStrstr((*str).stringval, (*find).stringval) };
        if !point.is_null() {
            offset = unsafe {
                point.offset_from((*str).stringval) as i32 + xmlStrlen((*find).stringval)
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
pub extern "C" fn xmlXPathNormalizeFunction(ctxt: xmlXPathParserContextPtr, mut nargs: i32) {
    let mut obj: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut source: *mut xmlChar = 0 as *mut xmlChar;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut blank: xmlChar;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 {
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
        nargs = 1
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    obj = unsafe { valuePop(ctxt) };
    source = unsafe { (*obj).stringval };
    target = unsafe { xmlBufCreate() };
    if !target.is_null() && !source.is_null() {
        /* Skip leading whitespaces */
        while unsafe {
            *source as i32 == 0x20 as i32
                || 0x9 as i32 <= *source as i32 && *source as i32 <= 0xa as i32
                || *source as i32 == 0xd as i32
        } {
            unsafe { source = source.offset(1) }
        }
        /* Collapse intermediate whitespaces, and skip trailing whitespaces */
        blank = 0 as xmlChar;
        while unsafe { *source != 0 } {
            if unsafe {
                *source as i32 == 0x20 as i32
                    || 0x9 as i32 <= *source as i32 && *source as i32 <= 0xa as i32
                    || *source as i32 == 0xd as i32
            } {
                blank = 0x20 as xmlChar
            } else {
                if blank != 0 {
                    unsafe { xmlBufAdd(target, &mut blank, 1) };
                    blank = 0 as xmlChar
                }
                unsafe { xmlBufAdd(target, source, 1) };
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
pub extern "C" fn xmlXPathTranslateFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut from: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut to: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut offset: i32;
    let mut max: i32;
    let mut ch: xmlChar;
    let mut point: *const xmlChar = 0 as *const xmlChar;
    let mut cptr: *mut xmlChar = 0 as *mut xmlChar;
    if ctxt.is_null() {
        return;
    }
    if nargs != 3 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 3 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    to = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    from = unsafe { valuePop(ctxt) };
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
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
            if offset >= 0 {
                if offset < max {
                    point = unsafe { xmlUTF8Strpos((*to).stringval, offset) };
                    if !point.is_null() {
                        unsafe { xmlBufAdd(target, point, xmlUTF8Strsize(point, 1)) };
                    }
                }
            } else {
                unsafe { xmlBufAdd(target, cptr, xmlUTF8Strsize(cptr, 1)) };
            }
            /* Step to next character in input */
            unsafe {
                cptr = cptr.offset(1);
            }
            if !(ch as i32 & 0x80 as i32 != 0) {
                continue;
            }
            /* if not simple ascii, verify proper format */
            if ch as i32 & 0xc0 as i32 != 0xc0 as i32 {
                unsafe {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"xmlXPathTranslateFunction: Invalid UTF8 string\n\x00" as *const u8
                            as *const i8,
                    )
                };
                /* not asserting an XPath error is probably better */
                break;
            } else {
                loop
                /* then skip over remaining bytes for this char */
                {
                    ch = ((ch as i32) << 1) as xmlChar;
                    if !(ch as i32 & 0x80 as i32 != 0) {
                        break;
                    }
                    let fresh70 = cptr;
                    unsafe { cptr = cptr.offset(1) };
                    if !unsafe { (*fresh70 as i32 & 0xc0 as i32 != 0x80 as i32) } {
                        continue;
                    }
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"xmlXPathTranslateFunction: Invalid UTF8 string\n\x00" as *const u8
                                as *const i8,
                        )
                    };
                    break;
                }
                if ch as i32 & 0x80 as i32 != 0 {
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
pub extern "C" fn xmlXPathBooleanFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    if cur.is_null() {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
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
pub extern "C" fn xmlXPathNotFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_BOOLEAN as u32
    {
        unsafe { xmlXPathBooleanFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_BOOLEAN as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    unsafe { (*safe_ctxt.value).boolval = ((*safe_ctxt.value).boolval == 0) as i32 };
}
/* *
 * xmlXPathTrueFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the true() XPath function
 *    boolean true() */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathTrueFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 0 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, 1)) };
}
/* *
 * xmlXPathFalseFunction: * @ctxt: the XPath Parser context
 * @nargs: the number of arguments
 *
 * Implement the false() XPath function
 *    boolean false() */

#[cfg(LIBXML_XPATH_ENABLED)]
pub extern "C" fn xmlXPathFalseFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 0 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, 0)) };
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
pub extern "C" fn xmlXPathLangFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let current_block: u64;
    let mut val: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut theLang: *const xmlChar = 0 as *const xmlChar;
    let mut lang: *const xmlChar = 0 as *const xmlChar;
    let mut ret: i32 = 0;
    let mut i: i32;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_STRING as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    val = unsafe { valuePop(ctxt) };
    lang = unsafe { (*val).stringval };
    theLang = unsafe { xmlNodeGetLang((*safe_ctxt.context).node as *const xmlNode) };
    if !theLang.is_null() && !lang.is_null() {
        i = 0;
        loop {
            if !unsafe { (*lang.offset(i as isize) as i32 != 0) } {
                current_block = 2232869372362427478;
                break;
            }
            if ({
                let mut __res: i32 = 0;
                if ::std::mem::size_of::<xmlChar>() as u64 > 1 {
                    if 1 > 2 {
                        let mut __c: i32 = unsafe { *lang.offset(i as isize) as i32 };
                        __res = (if __c < -128 || __c > 255 {
                            __c
                        } else {
                            unsafe { *(*__ctype_toupper_loc()).offset(__c as isize) }
                        })
                    } else {
                        __res = unsafe { toupper(*lang.offset(i as isize) as i32) }
                    }
                } else {
                    __res = unsafe {
                        *(*__ctype_toupper_loc()).offset(*lang.offset(i as isize) as isize)
                    }
                }
                __res
            }) != ({
                let mut __res: i32 = 0;
                if ::std::mem::size_of::<xmlChar>() as u64 > 1 {
                    if 1 > 2 {
                        let mut __c: i32 = unsafe { *theLang.offset(i as isize) as i32 };
                        __res = (if __c < -128 || __c > 255 {
                            __c
                        } else {
                            unsafe { *(*__ctype_toupper_loc()).offset(__c as isize) }
                        })
                    } else {
                        __res = unsafe { toupper(*theLang.offset(i as isize) as i32) }
                    }
                } else {
                    __res = unsafe {
                        *(*__ctype_toupper_loc()).offset(*theLang.offset(i as isize) as isize)
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
                if unsafe { *theLang.offset(i as isize) as i32 } == 0
                    || unsafe { *theLang.offset(i as isize) as i32 } == '-' as i32
                {
                    ret = 1
                }
            }
        }
    }
    if !theLang.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")(theLang as *mut ()) };
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
pub extern "C" fn xmlXPathNumberFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut res: libc::c_double = 0.;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs == 0 {
        if unsafe { (*safe_ctxt.context).node.is_null() } {
            unsafe { valuePush(ctxt, xmlXPathCacheNewFloat(safe_ctxt.context, 0.0f64)) };
        } else {
            let mut content: *mut xmlChar =
                unsafe { xmlNodeGetContent((*safe_ctxt.context).node as *const xmlNode) };
            unsafe { res = xmlXPathStringEvalNumber(content) };
            unsafe { valuePush(ctxt, xmlXPathCacheNewFloat(safe_ctxt.context, res)) };
            unsafe { xmlFree.expect("non-null function pointer")(content as *mut ()) };
        }
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
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
pub extern "C" fn xmlXPathSumFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut i: i32;
    let mut res: libc::c_double = 0.0f64;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NODESET as u32
            && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_XSLT_TREE as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return;
    }
    cur = unsafe { valuePop(ctxt) };
    if !unsafe { (*cur).nodesetval.is_null() } && unsafe { (*(*cur).nodesetval).nodeNr != 0 } {
        i = 0;
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
pub extern "C" fn xmlXPathFloorFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NUMBER as u32
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NUMBER as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
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
pub extern "C" fn xmlXPathCeilingFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NUMBER as u32
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NUMBER as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
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
pub extern "C" fn xmlXPathRoundFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut f: libc::c_double = 0.;
    if ctxt.is_null() {
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if nargs != 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 1 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    if !safe_ctxt.value.is_null()
        && unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NUMBER as u32
    {
        unsafe { xmlXPathNumberFunction(ctxt, 1) };
    }
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NUMBER as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
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
fn xmlXPathCurrentChar(ctxt: xmlXPathParserContextPtr, len: *mut i32) -> i32 {
    let current_block: u64;
    let c: u8;
    let mut val: u32 = 0;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    if ctxt.is_null() {
        return 0;
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
    if c as i32 & 0x80 as i32 != 0 {
        if !unsafe { (*cur.offset(1) as i32 & 0xc0 as i32 != 0x80 as i32) } {
            if c as i32 & 0xe0 as i32 == 0xe0 as i32 {
                if unsafe { *cur.offset(2) as i32 } & 0xc0 as i32 != 0x80 as i32 {
                    current_block = 79219340423750421;
                } else if c as i32 & 0xf0 as i32 == 0xf0 as i32 {
                    if c as i32 & 0xf8 as i32 != 0xf0 as i32
                        || unsafe { *cur.offset(3) as i32 & 0xc0 as i32 } != 0x80 as i32
                    {
                        current_block = 79219340423750421;
                    } else {
                        /* 4-byte code */
                        unsafe { *len = 4 };
                        val = unsafe { ((*cur.offset(0) as i32 & 0x7 as i32) << 18) as u32 };
                        val |= unsafe { ((*cur.offset(1) as i32 & 0x3f as i32) << 12) as u32 };
                        val |= unsafe { ((*cur.offset(2) as i32 & 0x3f as i32) << 6) as u32 };
                        val |= unsafe { (*cur.offset(3) as i32 & 0x3f as i32) as u32 };
                        current_block = 10043043949733653460;
                    }
                } else {
                    /* 3-byte code */
                    unsafe { *len = 3 };
                    val = unsafe { ((*cur.offset(0) as i32 & 0xf as i32) << 12) as u32 };
                    val |= unsafe { ((*cur.offset(1) as i32 & 0x3f as i32) << 6) as u32 };
                    val |= unsafe { (*cur.offset(2) as i32 & 0x3f as i32) as u32 };
                    current_block = 10043043949733653460;
                }
            } else {
                /* 2-byte code */
                unsafe { *len = 2 };
                val = unsafe { ((*cur.offset(0) as i32 & 0x1f as i32) << 6) as u32 };
                val |= unsafe { (*cur.offset(1) as i32 & 0x3f as i32) as u32 };
                current_block = 10043043949733653460;
            }
            match current_block {
                79219340423750421 => {}
                _ => {
                    if if val < 0x100 as u32 {
                        (0x9 as u32 <= val && val <= 0xa as u32
                            || val == 0xd as u32
                            || 0x20 as u32 <= val) as i32
                    } else {
                        (0x100 as u32 <= val && val <= 0xd7ff as u32
                            || 0xe000 as u32 <= val && val <= 0xfffd as u32
                            || 0x10000 as u32 <= val && val <= 0x10ffff as u32)
                            as i32
                    } == 0
                    {
                        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_CHAR_ERROR as i32) };
                        return 0;
                    }
                    return val as i32;
                }
            }
        }
        /*
         * If we detect an UTF8 error that probably means that the
         * input encoding didn't get properly advertised in the
         * declaration header. Report the error and switch the encoding
         * to ISO-Latin-1 (if you don't like this policy, just declare the
         * encoding !) */
        unsafe { *len = 0 };
        unsafe { xmlXPathErr(ctxt, XPATH_ENCODING_ERROR as i32) };
        return 0;
    } else {
        /* 1-byte code */
        unsafe { *len = 1 };
        return unsafe { *cur as i32 };
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
pub fn xmlXPathParseNCName(ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut count: i32 = 0;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.cur.is_null() {
        return 0 as *mut xmlChar;
    }
    /*
     * Accelerator for simple ASCII names
     */
    in_0 = safe_ctxt.cur;
    if unsafe {
        *in_0 as i32 >= 0x61 && *in_0 as i32 <= 0x7a as i32
            || *in_0 as i32 >= 0x41 && *in_0 as i32 <= 0x5a as i32
            || *in_0 as i32 == '_' as i32
    } {
        unsafe { in_0 = in_0.offset(1) };
        while unsafe {
            *in_0 as i32 >= 0x61 && *in_0 as i32 <= 0x7a as i32
                || *in_0 as i32 >= 0x41 && *in_0 as i32 <= 0x5a as i32
                || *in_0 as i32 >= 0x30 as i32 && *in_0 as i32 <= 0x39 as i32
                || *in_0 as i32 == '_' as i32
                || *in_0 as i32 == '.' as i32
                || *in_0 as i32 == '-' as i32
        } {
            unsafe { in_0 = in_0.offset(1) }
        }
        if unsafe {
            *in_0 as i32 == ' ' as i32
                || *in_0 as i32 == '>' as i32
                || *in_0 as i32 == '/' as i32
                || *in_0 as i32 == '[' as i32
                || *in_0 as i32 == ']' as i32
                || *in_0 as i32 == ':' as i32
                || *in_0 as i32 == '@' as i32
                || *in_0 as i32 == '*' as i32
        } {
            count = unsafe { in_0.offset_from(safe_ctxt.cur) as i32 };
            if count == 0 {
                return 0 as *mut xmlChar;
            }
            ret = unsafe { xmlStrndup(safe_ctxt.cur, count) };
            safe_ctxt.cur = in_0;
            return ret;
        }
    }
    return unsafe { xmlXPathParseNameComplex(ctxt, 0) };
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
fn xmlXPathParseQName(ctxt: xmlXPathParserContextPtr, prefix: *mut *mut xmlChar) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    unsafe { *prefix = 0 as *mut xmlChar };
    ret = unsafe { xmlXPathParseNCName(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ret.is_null() && unsafe { *safe_ctxt.cur as i32 == ':' as i32 } {
        unsafe {
            *prefix = ret;
        }
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
pub fn xmlXPathParseName(ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut count: size_t = 0 as i32 as size_t;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.cur.is_null() {
        return 0 as *mut xmlChar;
    }
    /*
     * Accelerator for simple ASCII names
     */
    in_0 = safe_ctxt.cur;
    if unsafe {
        *in_0 as i32 >= 0x61 && *in_0 as i32 <= 0x7a as i32
            || *in_0 as i32 >= 0x41 && *in_0 as i32 <= 0x5a as i32
            || *in_0 as i32 == '_' as i32
            || *in_0 as i32 == ':' as i32
    } {
        unsafe { in_0 = in_0.offset(1) };
        while unsafe {
            *in_0 as i32 >= 0x61 && *in_0 as i32 <= 0x7a as i32
                || *in_0 as i32 >= 0x41 && *in_0 as i32 <= 0x5a as i32
                || *in_0 as i32 >= 0x30 as i32 && *in_0 as i32 <= 0x39 as i32
                || *in_0 as i32 == '_' as i32
                || *in_0 as i32 == '-' as i32
                || *in_0 as i32 == ':' as i32
                || *in_0 as i32 == '.' as i32
        } {
            unsafe { in_0 = in_0.offset(1) }
        }
        if unsafe { *in_0 as i32 > 0 && (*in_0 as i32) < 0x80 as i32 } {
            count = unsafe { in_0.offset_from(safe_ctxt.cur) as i64 as size_t };
            if count > 50000 as i32 as u64 {
                safe_ctxt.cur = in_0;
                unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
                return 0 as *mut xmlChar;
            }
            ret = unsafe { xmlStrndup(safe_ctxt.cur, count as i32) };
            safe_ctxt.cur = in_0;
            return ret;
        }
    }
    return unsafe { xmlXPathParseNameComplex(ctxt, 1) };
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathParseNameComplex(ctxt: xmlXPathParserContextPtr, qualified: i32) -> *mut xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut len: i32 = 0;
    let mut l: i32 = 0;
    let mut c: i32;
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
        || !((if c < 0x100 as i32 {
            (0x41 <= c && c <= 0x5a as i32
                || 0x61 <= c && c <= 0x7a as i32
                || 0xc0 as i32 <= c && c <= 0xd6 as i32
                || 0xd8 as i32 <= c && c <= 0xf6 as i32
                || 0xf8 as i32 <= c) as i32
        } else {
            unsafe { xmlCharInRange(c as u32, &xmlIsBaseCharGroup) }
        }) != 0
            || (if c < 0x100 as i32 {
                0
            } else {
                (0x4e00 as i32 <= c && c <= 0x9fa5 as i32
                    || c == 0x3007 as i32
                    || 0x3021 <= c && c <= 0x3029 as i32) as i32
            }) != 0)
            && c != '_' as i32
            && (qualified == 0 || c != ':' as i32)
    {
        return 0 as *mut xmlChar;
    }
    while c != ' ' as i32
        && c != '>' as i32
        && c != '/' as i32
        && ((if c < 0x100 as i32 {
            (0x41 <= c && c <= 0x5a as i32
                || 0x61 <= c && c <= 0x7a as i32
                || 0xc0 as i32 <= c && c <= 0xd6 as i32
                || 0xd8 as i32 <= c && c <= 0xf6 as i32
                || 0xf8 as i32 <= c) as i32
        } else {
            unsafe { xmlCharInRange(c as u32, &xmlIsBaseCharGroup) }
        }) != 0
            || (if c < 0x100 as i32 {
                0
            } else {
                (0x4e00 as i32 <= c && c <= 0x9fa5 as i32
                    || c == 0x3007 as i32
                    || 0x3021 <= c && c <= 0x3029 as i32) as i32
            }) != 0
            || (if c < 0x100 as i32 {
                (0x30 as i32 <= c && c <= 0x39 as i32) as i32
            } else {
                unsafe { xmlCharInRange(c as u32, &xmlIsDigitGroup) }
            }) != 0
            || c == '.' as i32
            || c == '-' as i32
            || c == '_' as i32
            || qualified != 0 && c == ':' as i32
            || (if c < 0x100 as i32 {
                0
            } else {
                unsafe { xmlCharInRange(c as u32, &xmlIsCombiningGroup) }
            }) != 0
            || (if c < 0x100 as i32 {
                (c == 0xb7 as i32) as i32
            } else {
                unsafe { xmlCharInRange(c as u32, &xmlIsExtenderGroup) }
            }) != 0)
    {
        if l == 1 {
            let fresh71 = len;
            len = len + 1;
            buf[fresh71 as usize] = c as xmlChar
        } else {
            len += unsafe { xmlCopyChar(l, &mut *buf.as_mut_ptr().offset(len as isize), c) }
        }
        unsafe { (*ctxt).cur = (*ctxt).cur.offset(l as isize) };
        c = unsafe { xmlXPathCurrentChar(ctxt, &mut l) };
        if len >= 100 {
            /*
             * Okay someone managed to make a huge name, so he's ready to pay
             * for the processing speed.
             */
            let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
            let mut max: i32 = len * 2 as i32;
            if len > 50000 {
                unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
                return 0 as *mut xmlChar;
            }
            buffer = unsafe {
                xmlMallocAtomic.expect("non-null function pointer")(
                    (max as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                ) as *mut xmlChar
            };
            if buffer.is_null() {
                unsafe { xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as i32) };
                return 0 as *mut xmlChar;
            }
            unsafe { memcpy(buffer as *mut (), buf.as_mut_ptr() as *const (), len as u64) };
            while (if c < 0x100 as i32 {
                (0x41 <= c && c <= 0x5a as i32
                    || 0x61 <= c && c <= 0x7a as i32
                    || 0xc0 as i32 <= c && c <= 0xd6 as i32
                    || 0xd8 as i32 <= c && c <= 0xf6 as i32
                    || 0xf8 as i32 <= c) as i32
            } else {
                unsafe { xmlCharInRange(c as u32, &xmlIsBaseCharGroup) }
            }) != 0
                || (if c < 0x100 as i32 {
                    0
                } else {
                    (0x4e00 as i32 <= c && c <= 0x9fa5 as i32
                        || c == 0x3007 as i32
                        || 0x3021 <= c && c <= 0x3029 as i32) as i32
                }) != 0
                || (if c < 0x100 as i32 {
                    (0x30 as i32 <= c && c <= 0x39 as i32) as i32
                } else {
                    unsafe { xmlCharInRange(c as u32, &xmlIsDigitGroup) }
                }) != 0
                || c == '.' as i32
                || c == '-' as i32
                || c == '_' as i32
                || qualified != 0 && c == ':' as i32
                || (if c < 0x100 as i32 {
                    0
                } else {
                    unsafe { xmlCharInRange(c as u32, &xmlIsCombiningGroup) }
                }) != 0
                || (if c < 0x100 as i32 {
                    (c == 0xb7 as i32) as i32
                } else {
                    unsafe { xmlCharInRange(c as u32, &xmlIsExtenderGroup) }
                }) != 0
            {
                if len + 10 > max {
                    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                    if max > 50000 {
                        unsafe { xmlFree.expect("non-null function pointer")(buffer as *mut ()) };
                        unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
                        return 0 as *mut xmlChar;
                    }
                    max *= 2;
                    tmp = unsafe {
                        xmlRealloc.expect("non-null function pointer")(
                            buffer as *mut (),
                            (max as u64).wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
                        ) as *mut xmlChar
                    };
                    if tmp.is_null() {
                        unsafe { xmlFree.expect("non-null function pointer")(buffer as *mut ()) };
                        unsafe { xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as i32) };
                        return 0 as *mut xmlChar;
                    }
                    buffer = tmp
                }
                if l == 1 {
                    let fresh72 = len;
                    len = len + 1;
                    unsafe { *buffer.offset(fresh72 as isize) = c as xmlChar }
                } else {
                    len += unsafe { xmlCopyChar(l, &mut *buffer.offset(len as isize), c) }
                }
                unsafe { (*ctxt).cur = (*ctxt).cur.offset(l as isize) };
                c = unsafe { xmlXPathCurrentChar(ctxt, &mut l) }
            }
            unsafe { *buffer.offset(len as isize) = 0 as xmlChar };
            return buffer;
        }
    }
    if len == 0 {
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

pub fn xmlXPathStringEvalNumber(str: *const xmlChar) -> libc::c_double {
    let mut cur: *const xmlChar = str;
    let mut ret: libc::c_double = 0.;
    let mut ok: i32 = 0;
    let mut isneg: i32 = 0;
    let mut exponent: i32 = 0;
    let mut is_exponent_negative: i32 = 0;
    let mut tmp: u64 = 0 as u64;
    let mut temp: libc::c_double = 0.;
    match () {
        #[cfg(GUNC)]
        _ => {
            tmp = 0 as u64;
            temp = 0.;
        }
        #[cfg(not(GUNC))]
        _ => {}
    };
    if cur.is_null() {
        return 0 as libc::c_double;
    }
    while unsafe {
        *cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *cur as i32 && *cur as i32 <= 0xa as i32
            || *cur as i32 == 0xd as i32
    } {
        unsafe { cur = cur.offset(1) }
    }
    if unsafe {
        *cur as i32 != '.' as i32
            && ((*cur as i32) < '0' as i32 || *cur as i32 > '9' as i32)
            && *cur as i32 != '-' as i32
    } {
        unsafe { return xmlXPathNAN }
    }
    if unsafe { *cur as i32 == '-' as i32 } {
        isneg = 1;
        unsafe { cur = cur.offset(1) }
    }
    /*
     * tmp/temp is a workaround against a gcc compiler bug
     * http://veillard.com/gcc.bug
     */

    match () {
        #[cfg(GUNC)]
        _ => {
            ret = 0 as libc::c_double;
            while unsafe { *cur as i32 >= '0' as i32 && *cur as i32 <= '9' as i32 } {
                ret = ret * 10 as libc::c_double;
                tmp = unsafe { (*cur as i32 - '0' as i32) as u64 };
                ok = 1;
                unsafe { cur = cur.offset(1) };
                temp = tmp as libc::c_double;
                ret = ret + temp
            }
        }
        #[cfg(not(GUNC))]
        _ => {
            ret = 0 as libc::c_double;
            while unsafe { *cur as i32 >= '0' as i32 && *cur as i32 <= '9' as i32 } {
                ret = ret * 10 as libc::c_double;
                tmp = unsafe { (*cur as i32 - '0' as i32) as u64 };
                ok = 1;
                unsafe { cur = cur.offset(1) };
                temp = tmp as libc::c_double;
                ret = ret + temp
            }
        }
    };
    if unsafe { *cur as i32 == '.' as i32 } {
        let mut v: i32;
        let mut frac: i32 = 0;
        let max: i32;
        let mut fraction: libc::c_double = 0 as libc::c_double;
        unsafe { cur = cur.offset(1) };
        if unsafe { ((*cur as i32) < '0' as i32 || *cur as i32 > '9' as i32) && ok == 0 } {
            unsafe { return xmlXPathNAN }
        }
        while unsafe { *cur as i32 == '0' as i32 } {
            frac = frac + 1;
            unsafe { cur = cur.offset(1) }
        }
        max = frac + 20;
        while unsafe { *cur as i32 >= '0' as i32 && *cur as i32 <= '9' as i32 && frac < max } {
            v = unsafe { *cur as i32 - '0' as i32 };
            fraction = fraction * 10 as libc::c_double + v as libc::c_double;
            frac = frac + 1;
            unsafe { cur = cur.offset(1) }
        }
        fraction /= unsafe { pow(10.0f64, frac as libc::c_double) };
        ret = ret + fraction;
        while unsafe { *cur as i32 >= '0' as i32 && *cur as i32 <= '9' as i32 } {
            unsafe { cur = cur.offset(1) }
        }
    }
    if unsafe { *cur as i32 == 'e' as i32 || *cur as i32 == 'E' as i32 } {
        unsafe { cur = cur.offset(1) };
        if unsafe { *cur as i32 == '-' as i32 } {
            is_exponent_negative = 1;
            unsafe { cur = cur.offset(1) }
        } else if unsafe { *cur as i32 == '+' as i32 } {
            unsafe { cur = cur.offset(1) }
        }
        while unsafe { *cur as i32 >= '0' as i32 && *cur as i32 <= '9' as i32 } {
            if exponent < 1000000 as i32 {
                exponent = exponent * 10 + unsafe { (*cur as i32 - '0' as i32) }
            }
            unsafe { cur = cur.offset(1) }
        }
    }
    while unsafe {
        *cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *cur as i32 && *cur as i32 <= 0xa as i32
            || *cur as i32 == 0xd as i32
    } {
        unsafe { cur = cur.offset(1) }
    }
    if unsafe { *cur as i32 != 0 } {
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
fn xmlXPathCompNumber(ctxt: xmlXPathParserContextPtr) {
    let mut ret: libc::c_double = 0.0f64;
    let mut ok: i32 = 0;
    let mut exponent: i32 = 0;
    let mut is_exponent_negative: i32 = 0;
    let mut num: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut tmp: u64 = 0 as u64;
    let mut temp: libc::c_double = 0.;
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    if unsafe {
        *safe_ctxt.cur as i32 != '.' as i32
            && ((*safe_ctxt.cur as i32) < '0' as i32 || *safe_ctxt.cur as i32 > '9' as i32)
    } {
        unsafe { xmlXPathErr(ctxt, XPATH_NUMBER_ERROR as i32) };
        return;
    }
    /*
     * tmp/temp is a workaround against a gcc compiler bug
     * http://veillard.com/gcc.bug
     */
    ret = 0 as libc::c_double;
    while unsafe { *safe_ctxt.cur as i32 >= '0' as i32 && *safe_ctxt.cur as i32 <= '9' as i32 } {
        ret = ret * 10 as libc::c_double;
        tmp = unsafe { (*safe_ctxt.cur as i32 - '0' as i32) as u64 };
        ok = 1;
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        temp = tmp as libc::c_double;
        ret = ret + temp
    }
    if unsafe { *safe_ctxt.cur as i32 == '.' as i32 } {
        let mut v: i32;
        let mut frac: i32 = 0;
        let max: i32;
        let mut fraction: libc::c_double = 0 as libc::c_double;
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        if unsafe {
            ((*safe_ctxt.cur as i32) < '0' as i32 || *safe_ctxt.cur as i32 > '9' as i32) && ok == 0
        } {
            unsafe { xmlXPathErr(ctxt, XPATH_NUMBER_ERROR as i32) };
            return;
        }
        while unsafe { *safe_ctxt.cur as i32 == '0' as i32 } {
            frac = frac + 1;
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        max = frac + 20;
        while unsafe {
            *safe_ctxt.cur as i32 >= '0' as i32 && *safe_ctxt.cur as i32 <= '9' as i32 && frac < max
        } {
            v = unsafe { *safe_ctxt.cur as i32 - '0' as i32 };
            fraction = fraction * 10 as libc::c_double + v as libc::c_double;
            frac = frac + 1;
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        fraction /= unsafe { pow(10.0f64, frac as libc::c_double) };
        ret = ret + fraction;
        while unsafe { *safe_ctxt.cur as i32 >= '0' as i32 && *safe_ctxt.cur as i32 <= '9' as i32 }
        {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
    if unsafe { *safe_ctxt.cur as i32 == 'e' as i32 || *safe_ctxt.cur as i32 == 'E' as i32 } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        if unsafe { *safe_ctxt.cur as i32 == '-' as i32 } {
            is_exponent_negative = 1;
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        } else if unsafe { *safe_ctxt.cur } as i32 == '+' as i32 {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        while unsafe { *safe_ctxt.cur as i32 >= '0' as i32 && *safe_ctxt.cur as i32 <= '9' as i32 }
        {
            if exponent < 1000000 as i32 {
                exponent = exponent * 10 + unsafe { (*safe_ctxt.cur as i32 - '0' as i32) }
            }
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
    } else if unsafe {
        xmlXPathCompExprAdd(
            ctxt,
            (*safe_ctxt.comp).last,
            -1,
            XPATH_OP_VALUE,
            XPATH_NUMBER as i32,
            0,
            0,
            num as *mut (),
            0 as *mut (),
        ) == -1
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
fn xmlXPathParseLiteral(ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let mut q: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let safe_ctxt = unsafe { &mut *ctxt };
    if unsafe { *safe_ctxt.cur as i32 == '\"' as i32 } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        q = safe_ctxt.cur;
        while unsafe {
            (0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
                || 0x20 as i32 <= *safe_ctxt.cur as i32)
                && *safe_ctxt.cur as i32 != '\"' as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if !unsafe {
            (0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
                || 0x20 as i32 <= *safe_ctxt.cur as i32)
        } {
            unsafe { xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as i32) };
            return 0 as *mut xmlChar;
        } else {
            ret = unsafe { xmlStrndup(q, safe_ctxt.cur.offset_from(q) as i32) };
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    } else if unsafe { *safe_ctxt.cur as i32 == '\'' as i32 } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        q = safe_ctxt.cur;
        while unsafe {
            (0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
                || 0x20 as i32 <= *safe_ctxt.cur as i32)
                && *safe_ctxt.cur as i32 != '\'' as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if !unsafe {
            (0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
                || 0x20 as i32 <= *safe_ctxt.cur as i32)
        } {
            unsafe { xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as i32) };
            return 0 as *mut xmlChar;
        } else {
            ret = unsafe { xmlStrndup(q, safe_ctxt.cur.offset_from(q) as i32) };
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    } else {
        unsafe { xmlXPathErr(ctxt, XPATH_START_LITERAL_ERROR as i32) };
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
fn xmlXPathCompLiteral(ctxt: xmlXPathParserContextPtr) {
    let mut q: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut lit: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if unsafe { *safe_ctxt.cur as i32 == '\"' as i32 } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        q = safe_ctxt.cur;
        while unsafe {
            (0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
                || 0x20 as i32 <= *safe_ctxt.cur as i32)
                && *safe_ctxt.cur as i32 != '\"' as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if !unsafe {
            (0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
                || 0x20 as i32 <= *safe_ctxt.cur as i32)
        } {
            unsafe { xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as i32) };
            return;
        } else {
            ret = unsafe { xmlStrndup(q, safe_ctxt.cur.offset_from(q) as i32) };
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    } else if unsafe { *safe_ctxt.cur as i32 == '\'' as i32 } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        q = safe_ctxt.cur;
        while unsafe {
            (0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
                || 0x20 as i32 <= *safe_ctxt.cur as i32)
                && *safe_ctxt.cur as i32 != '\'' as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if !unsafe {
            (0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
                || 0x20 as i32 <= *safe_ctxt.cur as i32)
        } {
            unsafe { xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as i32) };
            return;
        } else {
            ret = unsafe { xmlStrndup(q, safe_ctxt.cur.offset_from(q) as i32) };
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    } else {
        unsafe { xmlXPathErr(ctxt, XPATH_START_LITERAL_ERROR as i32) };
        return;
    }
    if ret.is_null() {
        return;
    }
    lit = unsafe { xmlXPathCacheNewString(safe_ctxt.context, ret) };
    if lit.is_null() {
        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
    } else if unsafe {
        xmlXPathCompExprAdd(
            ctxt,
            (*safe_ctxt.comp).last,
            -1,
            XPATH_OP_VALUE,
            XPATH_STRING as i32,
            0,
            0,
            lit as *mut (),
            0 as *mut (),
        ) == -1
    } {
        unsafe { xmlXPathReleaseObject(safe_ctxt.context, lit) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(ret as *mut ()) };
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
fn xmlXPathCompVariableReference(ctxt: xmlXPathParserContextPtr) {
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe { *safe_ctxt.cur as i32 != '$' as i32 } {
        unsafe { xmlXPathErr(ctxt, XPATH_VARIABLE_REF_ERROR as i32) };
        return;
    }
    if unsafe { *safe_ctxt.cur as i32 != 0 } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
    } else {
    };
    name = xmlXPathParseQName(ctxt, &mut prefix);
    if name.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut ()) };
        unsafe { xmlXPathErr(ctxt, XPATH_VARIABLE_REF_ERROR as i32) };
        return;
    }
    unsafe { (*safe_ctxt.comp).last = -1 };
    if unsafe {
        xmlXPathCompExprAdd(
            ctxt,
            (*safe_ctxt.comp).last,
            -1,
            XPATH_OP_VARIABLE,
            0,
            0,
            0,
            name as *mut (),
            prefix as *mut (),
        ) == -1
    } {
        unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut ()) };
        unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if !safe_ctxt.context.is_null() && unsafe { (*safe_ctxt.context).flags & (1) << 1 != 0 } {
        unsafe { xmlXPathErr(ctxt, XPATH_FORBID_VARIABLE_ERROR as i32) };
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
pub fn xmlXPathIsNodeType(name: *const xmlChar) -> i32 {
    if name.is_null() {
        return 0;
    }
    if unsafe { xmlStrEqual(name, b"node\x00" as *const u8 as *const i8 as *mut xmlChar) != 0 } {
        return 1;
    }
    if unsafe { xmlStrEqual(name, b"text\x00" as *const u8 as *const i8 as *mut xmlChar) != 0 } {
        return 1;
    }
    if unsafe {
        xmlStrEqual(
            name,
            b"comment\x00" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
    } {
        return 1;
    }
    if unsafe {
        xmlStrEqual(
            name,
            b"processing-instruction\x00" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
    } {
        return 1;
    }
    return 0;
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
fn xmlXPathCompFunctionCall(ctxt: xmlXPathParserContextPtr) {
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut nbargs: i32 = 0;
    let mut sort: i32 = 1;
    let safe_ctxt = unsafe { &mut *ctxt };
    name = unsafe { xmlXPathParseQName(ctxt, &mut prefix) };
    if name.is_null() {
        unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut ()) };
        unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
        return;
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    //DEBUG_EXPR
    if unsafe { *safe_ctxt.cur as i32 != '(' as i32 } {
        unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
        unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut ()) };
        unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
        return;
    }
    if unsafe { *safe_ctxt.cur as i32 != 0 } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
    } else {
    };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    /*
     * Optimization for count(): we don't need the node-set to be sorted.
     */
    if prefix.is_null()
        && unsafe {
            *name.offset(0) as i32 == 'c' as i32
                && xmlStrEqual(name, b"count\x00" as *const u8 as *const i8 as *mut xmlChar) != 0
        }
    {
        sort = 0
    }
    unsafe { (*safe_ctxt.comp).last = -1 };
    if unsafe { *safe_ctxt.cur as i32 != ')' as i32 } {
        while unsafe { *safe_ctxt.cur as i32 != 0 } {
            let mut op1: i32 = unsafe { (*safe_ctxt.comp).last };
            unsafe { (*safe_ctxt.comp).last = -(1) };
            unsafe { xmlXPathCompileExpr(ctxt, sort) };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
                unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut ()) };
                return;
            }
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    op1,
                    (*safe_ctxt.comp).last,
                    XPATH_OP_ARG,
                    0,
                    0,
                    0,
                    0 as *mut (),
                    0 as *mut (),
                )
            };
            nbargs += 1;
            if unsafe { *safe_ctxt.cur as i32 == ')' as i32 } {
                break;
            }
            if unsafe { *safe_ctxt.cur as i32 != ',' as i32 } {
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
                unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut ()) };
                unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
                return;
            }
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
            while unsafe {
                *safe_ctxt.cur as i32 == 0x20 as i32
                    || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                    || *safe_ctxt.cur as i32 == 0xd as i32
            } {
                if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
            -1,
            XPATH_OP_FUNCTION,
            nbargs,
            0,
            0,
            name as *mut (),
            prefix as *mut (),
        ) == -1
    } {
        unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut ()) };
        unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
    }
    if unsafe { *safe_ctxt.cur as i32 != 0 } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
    } else {
    };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathCompPrimaryExpr(ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe { *safe_ctxt.cur as i32 == '$' as i32 } {
        unsafe { xmlXPathCompVariableReference(ctxt) };
    } else if unsafe { *safe_ctxt.cur as i32 == '(' as i32 } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompileExpr(ctxt, 1) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
            return;
        }
        if unsafe { *safe_ctxt.cur as i32 != ')' as i32 } {
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
            return;
        }
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    } else if unsafe {
        0x30 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0x39 as i32
            || *safe_ctxt.cur as i32 == '.' as i32
                && (0x30 as i32 <= *safe_ctxt.cur.offset(1 as isize) as i32
                    && *safe_ctxt.cur.offset(1 as isize) as i32 <= 0x39 as i32)
    } {
        unsafe { xmlXPathCompNumber(ctxt) };
    } else if unsafe {
        *safe_ctxt.cur as i32 == '\'' as i32 || *safe_ctxt.cur as i32 == '\"' as i32
    } {
        unsafe { xmlXPathCompLiteral(ctxt) };
    } else {
        unsafe { xmlXPathCompFunctionCall(ctxt) };
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathCompFilterExpr(ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompPrimaryExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe { *safe_ctxt.cur as i32 == '[' as i32 } {
        unsafe { xmlXPathCompPredicate(ctxt, 1) };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathScanName(ctxt: xmlXPathParserContextPtr) -> *mut xmlChar {
    let mut len: i32 = 0;
    let mut l: i32 = 0;
    let mut c: i32;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let safe_ctxt = unsafe { &mut *ctxt };
    cur = safe_ctxt.cur;
    c = unsafe { xmlXPathCurrentChar(ctxt, &mut l) };
    if c == ' ' as i32
        || c == '>' as i32
        || c == '/' as i32
        || !((if c < 0x100 as i32 {
            (0x41 <= c && c <= 0x5a as i32
                || 0x61 <= c && c <= 0x7a as i32
                || 0xc0 as i32 <= c && c <= 0xd6 as i32
                || 0xd8 as i32 <= c && c <= 0xf6 as i32
                || 0xf8 as i32 <= c) as i32
        } else {
            unsafe { xmlCharInRange(c as u32, &xmlIsBaseCharGroup) }
        }) != 0
            || (if c < 0x100 as i32 {
                0
            } else {
                (0x4e00 as i32 <= c && c <= 0x9fa5 as i32
                    || c == 0x3007 as i32
                    || 0x3021 <= c && c <= 0x3029 as i32) as i32
            }) != 0)
            && c != '_' as i32
            && c != ':' as i32
    {
        return 0 as *mut xmlChar;
    }
    while c != ' ' as i32
        && c != '>' as i32
        && c != '/' as i32
        && ((if c < 0x100 as i32 {
            (0x41 <= c && c <= 0x5a as i32
                || 0x61 <= c && c <= 0x7a as i32
                || 0xc0 as i32 <= c && c <= 0xd6 as i32
                || 0xd8 as i32 <= c && c <= 0xf6 as i32
                || 0xf8 as i32 <= c) as i32
        } else {
            unsafe { xmlCharInRange(c as u32, &xmlIsBaseCharGroup) }
        }) != 0
            || (if c < 0x100 as i32 {
                0
            } else {
                (0x4e00 as i32 <= c && c <= 0x9fa5 as i32
                    || c == 0x3007 as i32
                    || 0x3021 <= c && c <= 0x3029 as i32) as i32
            }) != 0
            || (if c < 0x100 as i32 {
                (0x30 as i32 <= c && c <= 0x39 as i32) as i32
            } else {
                unsafe { xmlCharInRange(c as u32, &xmlIsDigitGroup) }
            }) != 0
            || c == '.' as i32
            || c == '-' as i32
            || c == '_' as i32
            || c == ':' as i32
            || (if c < 0x100 as i32 {
                0
            } else {
                unsafe { xmlCharInRange(c as u32, &xmlIsCombiningGroup) }
            }) != 0
            || (if c < 0x100 as i32 {
                (c == 0xb7 as i32) as i32
            } else {
                unsafe { xmlCharInRange(c as u32, &xmlIsExtenderGroup) }
            }) != 0)
    {
        len += l;
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(l as isize) };
        c = unsafe { xmlXPathCurrentChar(ctxt, &mut l) }
    }
    ret = unsafe { xmlStrndup(cur, safe_ctxt.cur.offset_from(cur) as i32) };
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
fn xmlXPathCompPathExpr(ctxt: xmlXPathParserContextPtr) {
    let mut lc: i32 = 1; /* Should we branch to LocationPath ?         */
    let mut name: *mut xmlChar = 0 as *mut xmlChar; /* we may have to preparse a name to find out */
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if (unsafe {
        *safe_ctxt.cur as i32 == '$' as i32
            || *safe_ctxt.cur as i32 == '(' as i32
            || 0x30 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0x39 as i32
            || *safe_ctxt.cur as i32 == '\'' as i32
            || *safe_ctxt.cur as i32 == '\"' as i32
            || *safe_ctxt.cur as i32 == '.' as i32
                && (0x30 as i32 <= *safe_ctxt.cur.offset(1 as isize) as i32
                    && *safe_ctxt.cur.offset(1 as isize) as i32 <= 0x39 as i32)
    }) {
        lc = 0
    } else if (unsafe { *safe_ctxt.cur as i32 == '*' as i32 })
        || (unsafe { *safe_ctxt.cur as i32 == '/' as i32 })
        || (unsafe { *safe_ctxt.cur as i32 == '@' as i32 })
        || (unsafe { *safe_ctxt.cur as i32 == '.' as i32 })
    {
        /* relative or absolute location path */
        lc = 1
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
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        name = unsafe { xmlXPathScanName(ctxt) };
        if !name.is_null()
            && !unsafe {
                xmlStrstr(name, b"::\x00" as *const u8 as *const i8 as *mut xmlChar).is_null()
            }
        {
            lc = 1;
            unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
        } else if !name.is_null() {
            let mut len: i32 = unsafe { xmlStrlen(name) };
            while unsafe { *safe_ctxt.cur.offset(len as isize) as i32 != 0 } {
                if unsafe { *safe_ctxt.cur.offset(len as isize) as i32 == '/' as i32 } {
                    /* element name */
                    lc = 1;
                    break;
                } else if unsafe {
                    *safe_ctxt.cur.offset(len as isize) as i32 == 0x20 as i32
                        || 0x9 as i32 <= *safe_ctxt.cur.offset(len as isize) as i32
                            && *safe_ctxt.cur.offset(len as isize) as i32 <= 0xa as i32
                        || *safe_ctxt.cur.offset(len as isize) as i32 == 0xd as i32
                } {
                    /* ignore blanks */
                    len += 1
                } else if unsafe { *safe_ctxt.cur.offset(len as isize) as i32 == ':' as i32 } {
                    lc = 1;
                    break;
                } else if unsafe { *safe_ctxt.cur.offset(len as isize) as i32 == '(' as i32 } {
                    /* Node Type or Function */
                    if (unsafe { xmlXPathIsNodeType(name) != 0 })
                        || (safe_ctxt.xptr != 0
                            && unsafe {
                                xmlStrEqual(
                                    name,
                                    b"range-to\x00" as *const u8 as *const i8 as *mut xmlChar,
                                ) != 0
                            })
                    {
                        lc = 1
                    } else {
                        lc = 0
                    }
                    break;
                } else {
                    lc = 1;
                    break;
                }
            }
            if unsafe { *safe_ctxt.cur.offset(len as isize) as i32 == 0 } {
                /* element name */
                lc = 1
            }
            unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
        } else {
            /* make sure all cases are covered explicitly */
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
            return;
        }
    }
    if lc != 0 {
        if unsafe { *safe_ctxt.cur as i32 == '/' as i32 } {
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    -1,
                    -1,
                    XPATH_OP_ROOT,
                    0,
                    0,
                    0,
                    0 as *mut (),
                    0 as *mut (),
                )
            };
        } else {
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    -1,
                    -1,
                    XPATH_OP_NODE,
                    0,
                    0,
                    0,
                    0 as *mut (),
                    0 as *mut (),
                )
            };
        }
        unsafe { xmlXPathCompLocationPath(ctxt) };
    } else {
        unsafe { xmlXPathCompFilterExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
            return;
        }
        if unsafe {
            *safe_ctxt.cur as i32 == '/' as i32
                && *safe_ctxt.cur.offset(1 as isize) as i32 == '/' as i32
        } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(2) };
            while unsafe {
                *safe_ctxt.cur as i32 == 0x20 as i32
                    || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                    || *safe_ctxt.cur as i32 == 0xd as i32
            } {
                if unsafe { *safe_ctxt.cur as i32 != 0 } {
                    unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                } else {
                };
            }
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    (*safe_ctxt.comp).last,
                    -1,
                    XPATH_OP_COLLECT,
                    AXIS_DESCENDANT_OR_SELF as i32,
                    NODE_TEST_TYPE as i32,
                    NODE_TYPE_NODE as i32,
                    0 as *mut (),
                    0 as *mut (),
                )
            };
            unsafe { xmlXPathCompRelativeLocationPath(ctxt) };
        } else if unsafe { *safe_ctxt.cur as i32 == '/' as i32 } {
            unsafe { xmlXPathCompRelativeLocationPath(ctxt) };
        }
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathCompUnionExpr(ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompPathExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe { *safe_ctxt.cur as i32 == '|' as i32 } {
        let mut op1: i32 = unsafe { (*safe_ctxt.comp).last };
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                -1,
                -1,
                XPATH_OP_NODE,
                0,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
                0,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathCompUnaryExpr(ctxt: xmlXPathParserContextPtr) {
    let mut minus: i32 = 0;
    let mut found: i32 = 0;
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe { *safe_ctxt.cur as i32 == '-' as i32 } {
        minus = 1 - minus;
        found = 1;
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
    unsafe { xmlXPathCompUnionExpr(ctxt) };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    if found != 0 {
        if minus != 0 {
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    (*safe_ctxt.comp).last,
                    -1,
                    XPATH_OP_PLUS,
                    2,
                    0,
                    0,
                    0 as *mut (),
                    0 as *mut (),
                )
            };
        } else {
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    (*safe_ctxt.comp).last,
                    -1,
                    XPATH_OP_PLUS,
                    3,
                    0,
                    0,
                    0 as *mut (),
                    0 as *mut (),
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
fn xmlXPathCompMultiplicativeExpr(ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompUnaryExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe {
        *safe_ctxt.cur as i32 == '*' as i32
            || *safe_ctxt.cur as i32 == 'd' as i32
                && *safe_ctxt.cur.offset(1) as i32 == 'i' as i32
                && *safe_ctxt.cur.offset(2) as i32 == 'v' as i32
            || *safe_ctxt.cur as i32 == 'm' as i32
                && *safe_ctxt.cur.offset(1) as i32 == 'o' as i32
                && *safe_ctxt.cur.offset(2) as i32 == 'd' as i32
    } {
        let mut op: i32 = -1;
        let mut op1: i32 = unsafe { (*safe_ctxt.comp).last };
        if unsafe { *safe_ctxt.cur as i32 == '*' as i32 } {
            op = 0;
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        } else if unsafe { *safe_ctxt.cur as i32 == 'd' as i32 } {
            op = 1;
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(3) }
        } else if unsafe { *safe_ctxt.cur as i32 == 'm' as i32 } {
            op = 2;
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(3) }
        }
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompUnaryExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_MULT,
                op,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathCompAdditiveExpr(ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompMultiplicativeExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe { *safe_ctxt.cur as i32 == '+' as i32 || *safe_ctxt.cur as i32 == '-' as i32 } {
        let mut plus: i32 = 0;
        let mut op1: i32 = unsafe { (*safe_ctxt.comp).last };
        if unsafe { *safe_ctxt.cur as i32 == '+' as i32 } {
            plus = 1
        } else {
            plus = 0
        }
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompMultiplicativeExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_PLUS,
                plus,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathCompRelationalExpr(ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompAdditiveExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe { *safe_ctxt.cur as i32 == '<' as i32 || *safe_ctxt.cur as i32 == '>' as i32 } {
        let mut inf: i32 = 0;
        let mut strict: i32 = 0;
        let mut op1: i32 = unsafe { (*safe_ctxt.comp).last };
        if unsafe { *safe_ctxt.cur as i32 == '<' as i32 } {
            inf = 1
        } else {
            inf = 0
        }
        if unsafe { *safe_ctxt.cur.offset(1) as i32 == '=' as i32 } {
            strict = 0
        } else {
            strict = 1
        }
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        if strict == 0 {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompAdditiveExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
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
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathCompEqualityExpr(ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompRelationalExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe {
        *safe_ctxt.cur as i32 == '=' as i32
            || *safe_ctxt.cur as i32 == '!' as i32 && *safe_ctxt.cur.offset(1) as i32 == '=' as i32
    } {
        let mut eq: i32 = 0;
        let mut op1: i32 = unsafe { (*safe_ctxt.comp).last };
        if unsafe { *safe_ctxt.cur as i32 == '=' as i32 } {
            eq = 1
        } else {
            eq = 0
        }
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        if eq == 0 {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompRelationalExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_EQUAL,
                eq,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathCompAndExpr(ctxt: xmlXPathParserContextPtr) {
    unsafe { xmlXPathCompEqualityExpr(ctxt) };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 'a' as i32
            && *safe_ctxt.cur.offset(1) as i32 == 'n' as i32
            && *safe_ctxt.cur.offset(2) as i32 == 'd' as i32
    } {
        let mut op1: i32 = unsafe { (*safe_ctxt.comp).last };
        unsafe {
            safe_ctxt.cur = safe_ctxt.cur.offset(3);
        }
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompEqualityExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_AND,
                0,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathCompileExpr(ctxt: xmlXPathParserContextPtr, mut sort: i32) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut xpctxt: xmlXPathContextPtr = safe_ctxt.context;
    let safe_xpctxt = unsafe { &mut *xpctxt };
    if !xpctxt.is_null() {
        if safe_xpctxt.depth >= 5000 {
            unsafe { xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as i32) };
            return;
        }
        /*
         * Parsing a single '(' pushes about 10 functions on the call stack
         * before recursing!
         */
        safe_xpctxt.depth += 10
    }
    unsafe { xmlXPathCompAndExpr(ctxt) };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 'o' as i32 && *safe_ctxt.cur.offset(1) as i32 == 'r' as i32
    } {
        let mut op1: i32 = unsafe { (*safe_ctxt.comp).last };
        unsafe {
            safe_ctxt.cur = safe_ctxt.cur.offset(2);
        }
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe { xmlXPathCompAndExpr(ctxt) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
            return;
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_OR,
                0,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
            .op as u32
        } != XPATH_OP_VALUE as u32
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
                -1,
                XPATH_OP_SORT,
                0,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
    }
    if !xpctxt.is_null() {
        safe_xpctxt.depth -= 1
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
fn xmlXPathCompPredicate(ctxt: xmlXPathParserContextPtr, filter: i32) {
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut op1: i32 = unsafe { (*safe_ctxt.comp).last };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe { *safe_ctxt.cur as i32 != '[' as i32 } {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_PREDICATE_ERROR as i32) };
        return;
    }
    if unsafe { *safe_ctxt.cur as i32 != 0 } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
    } else {
    };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    unsafe { (*safe_ctxt.comp).last = -1 };
    /*
     * This call to xmlXPathCompileExpr() will deactivate sorting
     * of the predicate result.
     * TODO: Sorting is still activated for filters, since I'm not
     *  sure if needed. Normally sorting should not be needed, since
     *  a filter can only diminish the number of items in a sequence, *  but won't change its order; so if the initial sequence is sorted, *  subsequent sorting is not needed.
     */
    if filter == 0 {
        unsafe { xmlXPathCompileExpr(ctxt, 0) };
    } else {
        unsafe { xmlXPathCompileExpr(ctxt, 1) };
    }
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    if unsafe { *safe_ctxt.cur as i32 != ']' as i32 } {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_PREDICATE_ERROR as i32) };
        return;
    }
    if filter != 0 {
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_FILTER,
                0,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
    } else {
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*safe_ctxt.comp).last,
                XPATH_OP_PREDICATE,
                0,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            )
        };
    }
    if unsafe { *safe_ctxt.cur as i32 != 0 } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
    } else {
    };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
    ctxt: xmlXPathParserContextPtr,
    test: *mut xmlXPathTestVal,
    type_0: *mut xmlXPathTypeVal,
    prefix: *mut *mut xmlChar,
    mut name: *mut xmlChar,
) -> *mut xmlChar {
    let mut blanks: i32 = 0;
    if test.is_null() || type_0.is_null() || prefix.is_null() {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Internal error at %s:%d\n\x00" as *const u8 as *const i8,
                b"xpath.c\x00" as *const u8 as *const i8,
                11067 as i32,
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
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if name.is_null() && unsafe { *safe_ctxt.cur as i32 == '*' as i32 } {
        /*
         * All elements
         */
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
        unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
        return 0 as *mut xmlChar;
    }
    blanks = unsafe {
        (*safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32) as i32
    };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe { *safe_ctxt.cur as i32 == '(' as i32 } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        /*
         * NodeType or PI search
         */
        if unsafe {
            xmlStrEqual(
                name,
                b"comment\x00" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
        } {
            unsafe { *type_0 = NODE_TYPE_COMMENT }
        } else if unsafe {
            xmlStrEqual(name, b"node\x00" as *const u8 as *const i8 as *mut xmlChar) != 0
        } {
            unsafe { *type_0 = NODE_TYPE_NODE }
        } else if unsafe {
            xmlStrEqual(
                name,
                b"processing-instruction\x00" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
        } {
            unsafe { *type_0 = NODE_TYPE_PI }
        } else if unsafe {
            xmlStrEqual(name, b"text\x00" as *const u8 as *const i8 as *mut xmlChar) != 0
        } {
            unsafe { *type_0 = NODE_TYPE_TEXT }
        } else {
            if !name.is_null() {
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
            }
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
            return 0 as *mut xmlChar;
        }
        unsafe {
            *test = NODE_TEST_TYPE;
        }
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        if unsafe { *type_0 as u32 == NODE_TYPE_PI as u32 } {
            /*
             * Specific case: search a PI by name.
             */
            if !name.is_null() {
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
            }
            name = 0 as *mut xmlChar;
            if unsafe { *safe_ctxt.cur as i32 != ')' as i32 } {
                name = unsafe { xmlXPathParseLiteral(ctxt) };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                    return 0 as *mut xmlChar;
                }
                unsafe {
                    *test = NODE_TEST_PI;
                }
                while unsafe {
                    *safe_ctxt.cur as i32 == 0x20 as i32
                        || 0x9 as i32 <= *safe_ctxt.cur as i32
                            && *safe_ctxt.cur as i32 <= 0xa as i32
                        || *safe_ctxt.cur as i32 == 0xd as i32
                } {
                    if unsafe { *safe_ctxt.cur as i32 != 0 } {
                        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                    } else {
                    };
                }
            }
        }
        if unsafe { *safe_ctxt.cur as i32 != ')' as i32 } {
            if !name.is_null() {
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
            }
            unsafe { xmlXPathErr(ctxt, XPATH_UNCLOSED_ERROR as i32) };
            return 0 as *mut xmlChar;
        }
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        return name;
    }
    unsafe {
        *test = NODE_TEST_NAME;
    }
    if blanks == 0 && unsafe { *safe_ctxt.cur as i32 == ':' as i32 } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
        if unsafe { *safe_ctxt.cur as i32 == '*' as i32 } {
            /*
             * All elements
             */
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
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
fn xmlXPathIsAxisName(name: *const xmlChar) -> xmlXPathAxisVal {
    let mut ret: xmlXPathAxisVal = 0 as xmlXPathAxisVal;
    match unsafe { *name.offset(0) as i32 } {
        97 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"ancestor\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_ANCESTOR
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"ancestor-or-self\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_ANCESTOR_OR_SELF
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"attribute\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_ATTRIBUTE
            }
        }
        99 => {
            if unsafe {
                xmlStrEqual(name, b"child\x00" as *const u8 as *const i8 as *mut xmlChar) != 0
            } {
                ret = AXIS_CHILD
            }
        }
        100 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"descendant\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_DESCENDANT
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"descendant-or-self\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_DESCENDANT_OR_SELF
            }
        }
        102 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"following\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_FOLLOWING
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"following-sibling\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_FOLLOWING_SIBLING
            }
        }
        110 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"namespace\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_NAMESPACE
            }
        }
        112 => {
            if unsafe {
                xmlStrEqual(
                    name,
                    b"parent\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_PARENT
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"preceding\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_PRECEDING
            }
            if unsafe {
                xmlStrEqual(
                    name,
                    b"preceding-sibling\x00" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            } {
                ret = AXIS_PRECEDING_SIBLING
            }
        }
        115 => {
            if unsafe {
                xmlStrEqual(name, b"self\x00" as *const u8 as *const i8 as *mut xmlChar) != 0
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
fn xmlXPathCompStep(ctxt: xmlXPathParserContextPtr) {
    match () {
        #[cfg(LIBXML_XPTR_ENABLED)]
        _ => {
            let mut rangeto: i32 = 0;
            let mut op2: i32 = -1;
        }
        #[cfg(not(LIBXML_XPTR_ENABLED))]
        _ => {}
    }
    let mut rangeto: i32 = 0;
    let mut op2: i32 = -1;
    while unsafe { *(*ctxt).cur } as i32 == 0x20 as i32
        || 0x9 as i32 <= unsafe { *(*ctxt).cur } as i32
            && unsafe { *(*ctxt).cur } as i32 <= 0xa as i32
        || unsafe { *(*ctxt).cur } as i32 == 0xd as i32
    {
        if unsafe { *(*ctxt).cur } as i32 != 0 {
            unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
        } else {
        };
    }
    if unsafe { *(*ctxt).cur } as i32 == '.' as i32
        && unsafe { *(*ctxt).cur.offset(1 as isize) } as i32 == '.' as i32
    {
        unsafe { (*ctxt).cur = (*ctxt).cur.offset(2) };
        while unsafe { *(*ctxt).cur } as i32 == 0x20 as i32
            || 0x9 as i32 <= unsafe { *(*ctxt).cur } as i32
                && unsafe { *(*ctxt).cur } as i32 <= 0xa as i32
            || unsafe { *(*ctxt).cur } as i32 == 0xd as i32
        {
            if unsafe { *(*ctxt).cur } as i32 != 0 {
                unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
            } else {
            };
        }
        xmlXPathCompExprAdd(
            ctxt,
            unsafe { (*(*ctxt).comp).last },
            -1,
            XPATH_OP_COLLECT,
            AXIS_PARENT as i32,
            NODE_TEST_TYPE as i32,
            NODE_TYPE_NODE as i32,
            0 as *mut (),
            0 as *mut (),
        );
    } else if unsafe { *(*ctxt).cur } as i32 == '.' as i32 {
        if unsafe { *(*ctxt).cur } as i32 != 0 {
            unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
        } else {
        };
        while unsafe { *(*ctxt).cur } as i32 == 0x20 as i32
            || 0x9 as i32 <= unsafe { *(*ctxt).cur } as i32
                && unsafe { *(*ctxt).cur } as i32 <= 0xa as i32
            || unsafe { *(*ctxt).cur } as i32 == 0xd as i32
        {
            if unsafe { *(*ctxt).cur } as i32 != 0 {
                unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
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
        let mut op1: i32;
        /*
         * The modification needed for XPointer change to the production
         */
        //  match(){
        //     #[cfg(LIBXML_XPTR_ENABLED)] _ => {

        //     }
        //     #[cfg(not(LIBXML_XPTR_ENABLED))] _ => {}
        // }
        if unsafe { (*ctxt).xptr } != 0 {
            name = xmlXPathParseNCName(ctxt);
            if !name.is_null()
                && unsafe {
                    xmlStrEqual(
                        name,
                        b"range-to\x00" as *const u8 as *const i8 as *mut xmlChar,
                    )
                } != 0
            {
                op2 = unsafe { (*(*ctxt).comp).last };
                unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
                while unsafe { *(*ctxt).cur } as i32 == 0x20 as i32
                    || 0x9 as i32 <= unsafe { *(*ctxt).cur } as i32
                        && unsafe { *(*ctxt).cur } as i32 <= 0xa as i32
                    || unsafe { *(*ctxt).cur } as i32 == 0xd as i32
                {
                    if unsafe { *(*ctxt).cur } as i32 != 0 {
                        unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
                    } else {
                    };
                }
                if unsafe { *(*ctxt).cur } as i32 != '(' as i32 {
                    unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
                    return;
                }
                if unsafe { *(*ctxt).cur } as i32 != 0 {
                    unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
                } else {
                };
                while unsafe { *(*ctxt).cur } as i32 == 0x20 as i32
                    || 0x9 as i32 <= unsafe { *(*ctxt).cur } as i32
                        && unsafe { *(*ctxt).cur } as i32 <= 0xa as i32
                    || unsafe { *(*ctxt).cur } as i32 == 0xd as i32
                {
                    if unsafe { *(*ctxt).cur } as i32 != 0 {
                        unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
                    } else {
                    };
                }
                xmlXPathCompileExpr(ctxt, 1);
                /* PUSH_BINARY_EXPR(XPATH_OP_RANGETO, op2, ctxt->comp->last, 0, 0); */
                if unsafe { (*ctxt).error } != XPATH_EXPRESSION_OK as i32 {
                    return;
                }
                while unsafe { *(*ctxt).cur } as i32 == 0x20 as i32
                    || 0x9 as i32 <= unsafe { *(*ctxt).cur } as i32
                        && unsafe { *(*ctxt).cur } as i32 <= 0xa as i32
                    || unsafe { *(*ctxt).cur } as i32 == 0xd as i32
                {
                    if unsafe { *(*ctxt).cur } as i32 != 0 {
                        unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
                    } else {
                    };
                }
                if unsafe { *(*ctxt).cur } as i32 != ')' as i32 {
                    unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
                    return;
                }
                if unsafe { *(*ctxt).cur } as i32 != 0 {
                    unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
                } else {
                };
                rangeto = 1;
                current_block_91 = 7936397428294447972;
            } else {
                current_block_91 = 3222590281903869779;
            }
        } else {
            current_block_91 = 3222590281903869779;
        }
        match current_block_91 {
            3222590281903869779 => {
                if unsafe { *(*ctxt).cur } as i32 == '*' as i32 {
                    axis = AXIS_CHILD
                } else {
                    if name.is_null() {
                        name = xmlXPathParseNCName(ctxt)
                    }
                    if !name.is_null() {
                        axis = xmlXPathIsAxisName(name);
                        if axis as u32 != 0 as u32 {
                            while unsafe { *(*ctxt).cur } as i32 == 0x20 as i32
                                || 0x9 as i32 <= unsafe { *(*ctxt).cur } as i32
                                    && unsafe { *(*ctxt).cur } as i32 <= 0xa as i32
                                || unsafe { *(*ctxt).cur } as i32 == 0xd as i32
                            {
                                if unsafe { *(*ctxt).cur } as i32 != 0 {
                                    unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
                                } else {
                                };
                            }
                            if unsafe { *(*ctxt).cur } as i32 == ':' as i32
                                && unsafe { *(*ctxt).cur.offset(1) } as i32 == ':' as i32
                            {
                                unsafe { (*ctxt).cur = (*ctxt).cur.offset(2) };
                                unsafe {
                                    xmlFree.expect("non-null function pointer")(name as *mut ())
                                };
                                name = 0 as *mut xmlChar
                            } else {
                                /* an element name can conflict with an axis one :-\ */
                                axis = AXIS_CHILD
                            }
                        } else {
                            axis = AXIS_CHILD
                        }
                    } else if unsafe { *(*ctxt).cur } as i32 == '@' as i32 {
                        if unsafe { *(*ctxt).cur } as i32 != 0 {
                            unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
                        } else {
                        };
                        axis = AXIS_ATTRIBUTE
                    } else {
                        axis = AXIS_CHILD
                    }
                }
                if unsafe { (*ctxt).error } != XPATH_EXPRESSION_OK as i32 {
                    unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
                    return;
                }
                name = xmlXPathCompNodeTest(ctxt, &mut test, &mut type_0, &mut prefix, name);
                if test as u32 == 0 {
                    return;
                }
                if !prefix.is_null()
                    && !unsafe { (*ctxt).context.is_null() }
                    && unsafe { (*(*ctxt).context).flags } & (1) << 0 != 0
                {
                    if unsafe { xmlXPathNsLookup((*ctxt).context, prefix).is_null() } {
                        unsafe { xmlXPathErr(ctxt, XPATH_UNDEF_PREFIX_ERROR as i32) };
                    }
                }
            }
            _ => {}
        }
        op1 = unsafe { (*(*ctxt).comp).last };
        unsafe { (*(*ctxt).comp).last = -(1) };
        while unsafe { *(*ctxt).cur } as i32 == 0x20 as i32
            || 0x9 as i32 <= unsafe { *(*ctxt).cur } as i32
                && unsafe { *(*ctxt).cur } as i32 <= 0xa as i32
            || unsafe { *(*ctxt).cur } as i32 == 0xd as i32
        {
            if unsafe { *(*ctxt).cur } as i32 != 0 {
                unsafe { (*ctxt).cur = (*ctxt).cur.offset(1) }
            } else {
            };
        }
        while 1 < 2 {
            if (!(unsafe { *(*ctxt).cur } as i32 == '[' as i32)) {
                break;
            }
            xmlXPathCompPredicate(ctxt, 0);
        }
        if rangeto != 0 {
            xmlXPathCompExprAdd(
                ctxt,
                op2,
                op1,
                XPATH_OP_RANGETO,
                0,
                0,
                0,
                0 as *mut (),
                0 as *mut (),
            );
        } else if xmlXPathCompExprAdd(
            ctxt,
            op1,
            unsafe { (*(*ctxt).comp).last },
            XPATH_OP_COLLECT,
            axis as i32,
            test as i32,
            type_0 as i32,
            prefix as *mut (),
            name as *mut (),
        ) == -1
        {
            unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut ()) };
            unsafe { xmlFree.expect("non-null function pointer")(name as *mut ()) };
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
fn xmlXPathCompRelativeLocationPath(ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe {
        *safe_ctxt.cur as i32 == '/' as i32 && *safe_ctxt.cur.offset(1) as i32 == '/' as i32
    } {
        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(2) };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
        unsafe {
            xmlXPathCompExprAdd(
                ctxt,
                (*safe_ctxt.comp).last,
                -(1),
                XPATH_OP_COLLECT,
                AXIS_DESCENDANT_OR_SELF as i32,
                NODE_TEST_TYPE as i32,
                NODE_TYPE_NODE as i32,
                0 as *mut (),
                0 as *mut (),
            )
        };
    } else if unsafe { *safe_ctxt.cur as i32 == '/' as i32 } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
        }
    }
    unsafe { xmlXPathCompStep(ctxt) };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return;
    }
    while unsafe {
        *safe_ctxt.cur as i32 == 0x20 as i32
            || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
            || *safe_ctxt.cur as i32 == 0xd as i32
    } {
        if unsafe { *safe_ctxt.cur as i32 != 0 } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    while unsafe { *safe_ctxt.cur as i32 == '/' as i32 } {
        if unsafe {
            *safe_ctxt.cur as i32 == '/' as i32 && *safe_ctxt.cur.offset(1) as i32 == '/' as i32
        } {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(2) };
            while unsafe {
                *safe_ctxt.cur as i32 == 0x20 as i32
                    || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                    || *safe_ctxt.cur as i32 == 0xd as i32
            } {
                if unsafe { *safe_ctxt.cur as i32 != 0 } {
                    unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                } else {
                };
            }
            unsafe {
                xmlXPathCompExprAdd(
                    ctxt,
                    (*safe_ctxt.comp).last,
                    -1,
                    XPATH_OP_COLLECT,
                    AXIS_DESCENDANT_OR_SELF as i32,
                    NODE_TEST_TYPE as i32,
                    NODE_TYPE_NODE as i32,
                    0 as *mut (),
                    0 as *mut (),
                )
            };
            unsafe { xmlXPathCompStep(ctxt) };
        } else if unsafe { *safe_ctxt.cur as i32 == '/' as i32 } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
            } else {
            };
            while unsafe {
                *safe_ctxt.cur as i32 == 0x20 as i32
                    || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                    || *safe_ctxt.cur as i32 == 0xd as i32
            } {
                if unsafe { *safe_ctxt.cur as i32 != 0 } {
                    unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                } else {
                };
            }
            unsafe { xmlXPathCompStep(ctxt) };
        }
        while unsafe {
            *safe_ctxt.cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *safe_ctxt.cur as i32 && *safe_ctxt.cur as i32 <= 0xa as i32
                || *safe_ctxt.cur as i32 == 0xd as i32
        } {
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
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
fn xmlXPathCompLocationPath(ctxt: xmlXPathParserContextPtr) {
    let safe_ctxt = unsafe { &mut *ctxt };
    while unsafe { *safe_ctxt.cur as i32 == 0x20 as i32 }
        || 0x9 as i32 <= unsafe { *safe_ctxt.cur as i32 }
            && unsafe { *safe_ctxt.cur as i32 <= 0xa as i32 }
        || unsafe { *safe_ctxt.cur as i32 == 0xd as i32 }
    {
        if unsafe { *safe_ctxt.cur as i32 } != 0 {
            unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
        } else {
        };
    }
    if unsafe { *safe_ctxt.cur as i32 != '/' as i32 } {
        unsafe { xmlXPathCompRelativeLocationPath(ctxt) };
    } else {
        while unsafe { *safe_ctxt.cur as i32 == '/' as i32 } {
            if unsafe { *safe_ctxt.cur as i32 == '/' as i32 }
                && unsafe { *safe_ctxt.cur.offset(1) as i32 == '/' as i32 }
            {
                unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(2) };
                while unsafe { *safe_ctxt.cur as i32 == 0x20 as i32 }
                    || 0x9 as i32 <= unsafe { *safe_ctxt.cur as i32 }
                        && unsafe { *safe_ctxt.cur as i32 } <= 0xa as i32
                    || unsafe { *safe_ctxt.cur as i32 == 0xd as i32 }
                {
                    if unsafe { *safe_ctxt.cur as i32 } != 0 {
                        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                    } else {
                    };
                }
                unsafe {
                    xmlXPathCompExprAdd(
                        ctxt,
                        (*safe_ctxt.comp).last,
                        -1,
                        XPATH_OP_COLLECT,
                        AXIS_DESCENDANT_OR_SELF as i32,
                        NODE_TEST_TYPE as i32,
                        NODE_TYPE_NODE as i32,
                        0 as *mut (),
                        0 as *mut (),
                    )
                };
                unsafe { xmlXPathCompRelativeLocationPath(ctxt) };
            } else if unsafe { *safe_ctxt.cur as i32 == '/' as i32 } {
                if unsafe { *safe_ctxt.cur as i32 } != 0 {
                    unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                } else {
                };
                while unsafe { *safe_ctxt.cur as i32 == 0x20 as i32 }
                    || 0x9 as i32 <= unsafe { *safe_ctxt.cur as i32 }
                        && unsafe { *safe_ctxt.cur as i32 } <= 0xa as i32
                    || unsafe { *safe_ctxt.cur as i32 == 0xd as i32 }
                {
                    if unsafe { *safe_ctxt.cur as i32 } != 0 {
                        unsafe { safe_ctxt.cur = safe_ctxt.cur.offset(1) }
                    } else {
                    };
                }
                if unsafe { *safe_ctxt.cur as i32 } != 0 as i32
                    && (0x41 <= unsafe { *safe_ctxt.cur as i32 }
                        && unsafe { *safe_ctxt.cur as i32 } <= 0x5a as i32
                        || 0x61 <= unsafe { *safe_ctxt.cur as i32 }
                            && unsafe { *safe_ctxt.cur as i32 } <= 0x7a as i32
                        || unsafe { *safe_ctxt.cur as i32 == '_' as i32 }
                        || unsafe { *safe_ctxt.cur as i32 == '.' as i32 }
                        || unsafe { *safe_ctxt.cur as i32 == '@' as i32 }
                        || unsafe { *safe_ctxt.cur as i32 == '*' as i32 })
                {
                    unsafe { xmlXPathCompRelativeLocationPath(ctxt) };
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
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
    ctxt: xmlXPathParserContextPtr,
    set: xmlNodeSetPtr,
    filterOpIndex: i32,
    minPos: i32,
    maxPos: i32,
    hasNsNodes: i32,
) {
    let mut xpctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    let mut oldnode: xmlNodePtr = 0 as *mut xmlNode;
    let mut olddoc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut filterOp: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
    let oldcs: i32;
    let oldpp: i32;
    let mut i: i32;
    let mut j: i32;
    let mut pos: i32;
    let safe_set = unsafe { &mut *set };

    if set.is_null() || safe_set.nodeNr == 0 {
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
    i = 0;
    j = 0;
    pos = 1;
    while i < safe_set.nodeNr {
        let mut node: xmlNodePtr = unsafe { *safe_set.nodeTab.offset(i as isize) };
        let mut res: i32;
        unsafe { (*xpctxt).node = node };
        unsafe { (*xpctxt).proximityPosition = i + 1 };
        /*
         * Also set the xpath document in case things like
         * key() are evaluated in the predicate.
         *
         * TODO: Get real doc for namespace nodes.
         */
        let safe_node = unsafe { &mut *node };
        if safe_node.type_0 as u32 != XML_NAMESPACE_DECL as u32 && !safe_node.doc.is_null() {
            safe_xpctxt.doc = safe_node.doc
        }
        res = unsafe { xmlXPathCompOpEvalToBoolean(ctxt, filterOp, 1) };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
            break;
        }
        if res < 0 {
            /* Shouldn't happen */
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
            break;
        } else {
            if res != 0 && (pos >= minPos && pos <= maxPos) {
                if i != j {
                    unsafe {
                        let ref mut fresh73 = *(*set).nodeTab.offset(j as isize);
                        *fresh73 = node;
                        let ref mut fresh74 = *(*set).nodeTab.offset(i as isize);
                        *fresh74 = 0 as xmlNodePtr
                    }
                }
                j += 1
            } else {
                /* Remove the entry from the initial node set. */
                unsafe {
                    let ref mut fresh75 = *(*set).nodeTab.offset(i as isize);
                    *fresh75 = 0 as xmlNodePtr;
                    if safe_node.type_0 as u32 == XML_NAMESPACE_DECL as u32 {
                        xmlXPathNodeSetFreeNs(node as xmlNsPtr);
                    }
                }
            }
            if res != 0 {
                if pos == maxPos {
                    i += 1;
                    break;
                } else {
                    pos += 1
                }
            }
            i += 1
        }
    }
    /* Free remaining nodes. */
    if hasNsNodes != 0 {
        while i < safe_set.nodeNr {
            let mut node_0: xmlNodePtr = unsafe { *(*set).nodeTab.offset(i as isize) };
            if !node_0.is_null() && unsafe { (*node_0).type_0 as u32 } == XML_NAMESPACE_DECL as u32
            {
                unsafe { xmlXPathNodeSetFreeNs(node_0 as xmlNsPtr) };
            }
            i += 1
        }
    }
    safe_set.nodeNr = j;
    /* If too many elements were removed, shrink table to preserve memory. */
    if safe_set.nodeMax > 10 && safe_set.nodeNr < safe_set.nodeMax / 2 {
        let mut tmp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        let mut nodeMax: i32 = safe_set.nodeNr;
        if nodeMax < 10 {
            nodeMax = 10
        }
        tmp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                safe_set.nodeTab as *mut (),
                (nodeMax as u64).wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
            ) as *mut xmlNodePtr
        };
        if tmp.is_null() {
            unsafe {
                xmlXPathPErrMemory(ctxt, b"shrinking nodeset\n\x00" as *const u8 as *const i8)
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
    ctxt: xmlXPathParserContextPtr,
    locset: xmlLocationSetPtr,
    filterOpIndex: i32,
    minPos: i32,
    maxPos: i32,
) {
    let mut xpctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    let mut oldnode: xmlNodePtr = 0 as *mut xmlNode;
    let mut olddoc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut filterOp: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
    let oldcs: i32;
    let oldpp: i32;
    let mut i: i32;
    let mut j: i32;
    let mut pos: i32;
    let safe_locset = unsafe { &mut *locset };
    if locset.is_null() || safe_locset.locNr == 0 || filterOpIndex == -1 {
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
    i = 0;
    j = 0;
    pos = 1;

    while i < safe_locset.locNr {
        let mut contextNode: xmlNodePtr =
            unsafe { (**(*locset).locTab.offset(i as isize)).user as xmlNodePtr };
        let mut res: i32;
        safe_xpctxt.node = contextNode;
        safe_xpctxt.proximityPosition = i + 1;

        /*
         * Also set the xpath document in case things like
         * key() are evaluated in the predicate.
         *
         * TODO: Get real doc for namespace nodes.
         */
        let safe_contextNode = unsafe { &mut *contextNode };
        if safe_contextNode.type_0 as u32 != XML_NAMESPACE_DECL as u32
            && !safe_contextNode.doc.is_null()
        {
            safe_xpctxt.doc = safe_contextNode.doc
        }

        res = unsafe { xmlXPathCompOpEvalToBoolean(ctxt, filterOp, 1) };

        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
            break;
        }

        if res < 0 {
            /* Shouldn't happen */
            unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
            break;
        } else {
            if res != 0 && (pos >= minPos && pos <= maxPos) {
                if i != j {
                    unsafe {
                        let ref mut fresh76 = *(*locset).locTab.offset(j as isize);
                        *fresh76 = *(*locset).locTab.offset(i as isize);
                        let ref mut fresh77 = *(*locset).locTab.offset(i as isize);
                        *fresh77 = 0 as xmlXPathObjectPtr
                    }
                }
                j += 1
            } else {
                /* Remove the entry from the initial location set. */
                unsafe { xmlXPathFreeObject(*(*locset).locTab.offset(i as isize)) };
                unsafe {
                    let ref mut fresh78 = *(*locset).locTab.offset(i as isize);
                    *fresh78 = 0 as xmlXPathObjectPtr
                }
            }
            if res != 0 {
                if pos == maxPos {
                    i += 1;
                    break;
                } else {
                    pos += 1
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
    if safe_locset.locMax > 10 as i32 && safe_locset.locNr < safe_locset.locMax / 2 {
        let mut tmp: *mut xmlXPathObjectPtr = 0 as *mut xmlXPathObjectPtr;
        let mut locMax: i32 = safe_locset.locNr;
        if locMax < 10 {
            locMax = 10
        }
        tmp = unsafe {
            xmlRealloc.expect("non-null function pointer")(
                (*locset).locTab as *mut (),
                (locMax as u64).wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>() as u64),
            ) as *mut xmlXPathObjectPtr
        };
        if tmp.is_null() {
            unsafe {
                xmlXPathPErrMemory(ctxt, b"shrinking locset\n\x00" as *const u8 as *const i8)
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
    ctxt: xmlXPathParserContextPtr,
    op: xmlXPathStepOpPtr,
    set: xmlNodeSetPtr,
    minPos: i32,
    maxPos: i32,
    hasNsNodes: i32,
) {
    let safe_op = unsafe { &mut *op };
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_op.ch1 != -1 {
        let mut comp: xmlXPathCompExprPtr = safe_ctxt.comp;
        /*
         * Process inner predicates first.
         */
        let safe_comp = unsafe { &mut *comp };
        if unsafe { (*safe_comp.steps.offset(safe_op.ch1 as isize)).op as u32 }
            != XPATH_OP_PREDICATE as u32
        {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlXPathCompOpEvalPredicate: Expected a predicate\n\x00" as *const u8
                        as *const i8,
                )
            };
            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
            return;
        }

        if unsafe { (*safe_ctxt.context).depth } >= 5000 {
            unsafe { xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as i32) };
            return;
        }
        unsafe { (*safe_ctxt.context).depth += 1 };
        unsafe {
            xmlXPathCompOpEvalPredicate(
                ctxt,
                &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                set,
                1,
                (*set).nodeNr,
                hasNsNodes,
            )
        };
        unsafe { (*safe_ctxt.context).depth -= 1 };
        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
            return;
        }
    }
    if safe_op.ch2 != -1 {
        unsafe { xmlXPathNodeSetFilter(ctxt, set, safe_op.ch2, minPos, maxPos, hasNsNodes) };
    };
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathIsPositionalPredicate(
    ctxt: xmlXPathParserContextPtr,
    op: xmlXPathStepOpPtr,
    maxPos: *mut i32,
) -> i32 {
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
    if safe_op.op as u32 != XPATH_OP_PREDICATE as u32 && safe_op.op as u32 != XPATH_OP_FILTER as u32
    {
        return 0;
    }
    if safe_op.ch2 != -1 {
        exprOp = unsafe {
            &mut *(*(*ctxt).comp).steps.offset(safe_op.ch2 as isize) as *mut xmlXPathStepOp
        }
    } else {
        return 0;
    }
    if !exprOp.is_null()
        && unsafe { (*exprOp).op as u32 } == XPATH_OP_VALUE as u32
        && !unsafe { (*exprOp).value4.is_null() }
        && unsafe { (*((*exprOp).value4 as xmlXPathObjectPtr)).type_0 as u32 }
            == XPATH_NUMBER as u32
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
        if floatval > (-(2147483647 as i32) - 1) as libc::c_double
            && floatval < 2147483647 as i32 as libc::c_double
        {
            unsafe { *maxPos = floatval as i32 };
            if unsafe { floatval == *maxPos as libc::c_double } {
                return 1;
            }
        }
    }
    return 0;
}
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathNodeCollectAndTest(
    ctxt: xmlXPathParserContextPtr,
    op: xmlXPathStepOpPtr,
    mut first: *mut xmlNodePtr,
    mut last: *mut xmlNodePtr,
    toBool: i32,
) -> i32 {
    let mut current_block: u64;
    let safe_op = unsafe { &mut *op };
    let mut axis: xmlXPathAxisVal = safe_op.value as xmlXPathAxisVal;
    let mut test: xmlXPathTestVal = safe_op.value2 as xmlXPathTestVal;
    let mut type_0: xmlXPathTypeVal = safe_op.value3 as xmlXPathTypeVal;
    let mut prefix: *const xmlChar = safe_op.value4 as *const xmlChar;
    let mut name: *const xmlChar = safe_op.value5 as *const xmlChar;
    let mut URI: *const xmlChar = 0 as *const xmlChar;
    let mut total: i32 = 0 as i32;
    let mut hasNsNodes: i32 = 0 as i32;
    /* The popped object holding the context nodes */
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    /* The set of context nodes for the node tests */
    let mut contextSeq: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut contextIdx: i32 = 0;
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
    let mut maxPos: i32 = 0;
    let mut hasPredicateRange: i32 = 0;
    let mut hasAxisRange: i32 = 0;
    let mut pos: i32 = 0;
    let mut breakOnFirstHit: i32 = 0;
    let mut next: xmlXPathTraversalFunction = None;
    let mut addNode: Option<unsafe extern "C" fn(_: xmlNodeSetPtr, _: xmlNodePtr) -> i32> = None;
    let mut mergeAndClear: xmlXPathNodeSetMergeFunction = None;
    let mut oldContextNode: xmlNodePtr = 0 as *mut xmlNode;
    let safe_ctxt = unsafe { &mut *ctxt };
    let mut xpctxt: xmlXPathContextPtr = safe_ctxt.context;
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NODESET as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return 0;
    }
    obj = unsafe { valuePop(ctxt) };
    /*
     * Setup namespaces.
     */
    if !prefix.is_null() {
        URI = unsafe { xmlXPathNsLookup(xpctxt, prefix) };
        if URI.is_null() {
            unsafe { xmlXPathReleaseObject(xpctxt, obj) };
            unsafe { xmlXPathErr(ctxt, XPATH_UNDEF_PREFIX_ERROR as i32) };
            return 0;
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

    match axis as u32 {
        AXIS_ANCESTOR => {
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
        AXIS_ANCESTOR_OR_SELF => {
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
        AXIS_ATTRIBUTE => {
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
        AXIS_CHILD => {
            last = 0 as *mut xmlNodePtr;
            if (test as u32 == NODE_TEST_NAME as u32 || test as u32 == NODE_TEST_ALL as u32)
                && type_0 as u32 == NODE_TYPE_NODE as u32
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
        AXIS_DESCENDANT => {
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
        AXIS_DESCENDANT_OR_SELF => {
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
        AXIS_FOLLOWING => {
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
        AXIS_FOLLOWING_SIBLING => {
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
        AXIS_NAMESPACE => {
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
        AXIS_PARENT => {
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
        AXIS_PRECEDING => {
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
        AXIS_PRECEDING_SIBLING => {
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
        AXIS_SELF => {
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
        return 0;
    }
    let safe_obj = unsafe { &mut *obj };
    contextSeq = safe_obj.nodesetval;
    let safe_contextSeq = unsafe { &mut *contextSeq };
    if contextSeq.is_null() || safe_contextSeq.nodeNr <= 0 {
        unsafe { xmlXPathReleaseObject(xpctxt, obj) };
        unsafe { valuePush(ctxt, xmlXPathCacheWrapNodeSet(xpctxt, 0 as xmlNodeSetPtr)) };
        return 0;
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
    maxPos = 0;
    predOp = 0 as xmlXPathStepOpPtr;
    hasPredicateRange = 0;
    hasAxisRange = 0;
    if safe_op.ch2 != -1 {
        /*
        	* There's at least one predicate. 16 == XPATH_OP_PREDICATE
        	*/
        predOp = unsafe {
            &mut *(*safe_ctxt.comp).steps.offset(safe_op.ch2 as isize) as *mut xmlXPathStepOp
        };
        if unsafe { xmlXPathIsPositionalPredicate(ctxt, predOp, &mut maxPos) != 0 } {
            if unsafe { (*predOp).ch1 != -1 } {
                /*
                	* Use the next inner predicate operator.
                	*/
                predOp = unsafe {
                    &mut *(*safe_ctxt.comp).steps.offset((*predOp).ch1 as isize)
                        as *mut xmlXPathStepOp
                };
                hasPredicateRange = 1
            } else {
                /*
                	* There's no other predicate than the [n] predicate.
                	*/
                predOp = 0 as xmlXPathStepOpPtr;
                hasAxisRange = 1
            }
        }
    }

    breakOnFirstHit = if toBool != 0 && predOp.is_null() {
        1
    } else {
        0
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
                as unsafe extern "C" fn(_: xmlNodeSetPtr, _: xmlNodePtr) -> i32,
        )
    };
    outSeq = 0 as xmlNodeSetPtr;
    seq = 0 as xmlNodeSetPtr;
    contextNode = 0 as xmlNodePtr;
    contextIdx = 0;
    's_486: while (contextIdx < safe_contextSeq.nodeNr || !contextNode.is_null())
        && safe_ctxt.error == XPATH_EXPRESSION_OK as i32
    {
        let fresh79 = contextIdx;
        contextIdx = contextIdx + 1;
        safe_xpctxt.node = unsafe { *safe_contextSeq.nodeTab.offset(fresh79 as isize) };
        if seq.is_null() {
            seq = unsafe { xmlXPathNodeSetCreate(0 as xmlNodePtr) };
            if seq.is_null() {
                /* TODO: Propagate memory error. */
                total = 0;
                break;
            }
        }
        /*
         * Traverse the axis and test the nodes.
         */
        pos = 0;
        cur = 0 as xmlNodePtr;
        hasNsNodes = 0;

        loop {
            if unsafe { (*safe_ctxt.context).opLimit != 0 && xmlXPathCheckOpLimit(ctxt, 1) < 0 } {
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
                        if total % 256 as i32 == 0 as i32
                            && unsafe { xmlXPathCmpNodesExt(*first, cur) } >= 0 as i32
                        {
                            current_block = 6300945584809257457;
                            break;
                        }
                    }
                    #[cfg(not(XP_OPTIMIZED_NON_ELEM_COMPARISON))]
                    _ => {
                        if total % 256 as i32 == 0 as i32
                            && unsafe { xmlXPathCmpNodes(*first, cur) } >= 0 as i32
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
                        if total % 256 as i32 == 0 as i32
                            && unsafe { xmlXPathCmpNodesExt(cur, *last) } >= 0 as i32
                        {
                            current_block = 6300945584809257457;
                            break;
                        }
                    }
                    #[cfg(not(XP_OPTIMIZED_NON_ELEM_COMPARISON))]
                    _ => {
                        if total % 256 as i32 == 0 as i32
                            && unsafe { xmlXPathCmpNodes(cur, *last) >= 0 } as i32
                        {
                            current_block = 6300945584809257457;
                            break;
                        }
                    }
                };
            }
            total += 1;
            match test as u32 {
                NODE_TEST_NONE => {
                    total = 0;
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Internal error at %s:%d\n\x00" as *const u8 as *const i8,
                            b"xpath.c\x00" as *const u8 as *const i8,
                            12269 as i32,
                        )
                    };
                    break 's_486;
                }
                NODE_TEST_TYPE => {
                    if type_0 as u32 == NODE_TYPE_NODE as u32 {
                        match unsafe { (*cur).type_0 as u32 } {
                            XML_DOCUMENT_NODE
                            | XML_HTML_DOCUMENT_NODE
                            | XML_DOCB_DOCUMENT_NODE
                            | XML_ELEMENT_NODE
                            | XML_ATTRIBUTE_NODE
                            | XML_PI_NODE
                            | XML_COMMENT_NODE
                            | XML_CDATA_SECTION_NODE
                            | XML_TEXT_NODE => {
                                current_block = 16677367482302331965;
                                match current_block {
                                    16677367482302331965 => {
                                        if hasAxisRange != 0 as i32 {
                                            pos += 1;
                                            if pos == maxPos {
                                                if unsafe {
                                                    addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    )
                                                } < 0 as i32
                                                {
                                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                                }
                                                current_block = 12278438173206364583;
                                                break;
                                            }
                                        } else {
                                            if unsafe {
                                                addNode.expect("non-null function pointer")(
                                                    seq, cur,
                                                )
                                            } < 0 as i32
                                            {
                                                safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                            }
                                            if breakOnFirstHit != 0 {
                                                current_block = 795179968803393002;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        if axis as u32 == AXIS_NAMESPACE as u32 {
                                            if hasAxisRange != 0 as i32 {
                                                pos += 1;
                                                if pos == maxPos {
                                                    hasNsNodes = 1;
                                                    if unsafe {
                                                        xmlXPathNodeSetAddNs(
                                                            seq,
                                                            safe_xpctxt.node,
                                                            cur as xmlNsPtr,
                                                        )
                                                    } < 0
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                                    }
                                                    current_block = 12278438173206364583;
                                                    break;
                                                }
                                            } else {
                                                hasNsNodes = 1;
                                                if unsafe {
                                                    xmlXPathNodeSetAddNs(
                                                        seq,
                                                        safe_xpctxt.node,
                                                        cur as xmlNsPtr,
                                                    )
                                                } < 0
                                                {
                                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 795179968803393002;
                                                    break;
                                                }
                                            }
                                        } else {
                                            hasNsNodes = 1;
                                            if hasAxisRange != 0 {
                                                pos += 1;
                                                if pos == maxPos {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                                    }
                                                    current_block = 12278438173206364583;
                                                    break;
                                                }
                                            } else {
                                                if unsafe {
                                                    addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    )
                                                } < 0
                                                {
                                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
                            XML_NAMESPACE_DECL => {
                                current_block = 16937825661756021828;
                                match current_block {
                                    16677367482302331965 => {
                                        if hasAxisRange != 0 {
                                            pos += 1;
                                            if pos == maxPos {
                                                if unsafe {
                                                    addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    )
                                                } < 0
                                                {
                                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                                }
                                                current_block = 12278438173206364583;
                                                break;
                                            }
                                        } else {
                                            if unsafe {
                                                addNode.expect("non-null function pointer")(
                                                    seq, cur,
                                                )
                                            } < 0
                                            {
                                                safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                            }
                                            if breakOnFirstHit != 0 {
                                                current_block = 795179968803393002;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        if axis as u32 == AXIS_NAMESPACE as u32 {
                                            if hasAxisRange != 0 {
                                                pos += 1;
                                                if pos == maxPos {
                                                    hasNsNodes = 1;
                                                    if unsafe {
                                                        xmlXPathNodeSetAddNs(
                                                            seq,
                                                            safe_xpctxt.node,
                                                            cur as xmlNsPtr,
                                                        )
                                                    } < 0
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                                    }
                                                    current_block = 12278438173206364583;
                                                    break;
                                                }
                                            } else {
                                                hasNsNodes = 1;
                                                if unsafe {
                                                    xmlXPathNodeSetAddNs(
                                                        seq,
                                                        safe_xpctxt.node,
                                                        cur as xmlNsPtr,
                                                    )
                                                } < 0
                                                {
                                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 795179968803393002;
                                                    break;
                                                }
                                            }
                                        } else {
                                            hasNsNodes = 1;
                                            if hasAxisRange != 0 {
                                                pos += 1;
                                                if pos == maxPos {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                                    }
                                                    current_block = 12278438173206364583;
                                                    break;
                                                }
                                            } else {
                                                if unsafe {
                                                    addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    )
                                                } < 0
                                                {
                                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
                    } else if unsafe { (*cur).type_0 as u32 } == type_0 as xmlElementType as u32 {
                        if unsafe { (*cur).type_0 as u32 } == XML_NAMESPACE_DECL as u32 {
                            if hasAxisRange != 0 {
                                pos += 1;
                                if pos == maxPos {
                                    hasNsNodes = 1;
                                    if unsafe {
                                        xmlXPathNodeSetAddNs(seq, safe_xpctxt.node, cur as xmlNsPtr)
                                    } < 0
                                    {
                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                    }
                                    current_block = 12278438173206364583;
                                    break;
                                }
                            } else {
                                hasNsNodes = 1;
                                if unsafe {
                                    xmlXPathNodeSetAddNs(seq, safe_xpctxt.node, cur as xmlNsPtr)
                                } < 0
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 795179968803393002;
                                    break;
                                }
                            }
                        } else if hasAxisRange != 0 {
                            pos += 1;
                            if pos == maxPos {
                                if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                    < 0
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                }
                                current_block = 12278438173206364583;
                                break;
                            }
                        } else {
                            if unsafe { addNode.expect("non-null function pointer")(seq, cur) } < 0
                            {
                                safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 795179968803393002;
                                break;
                            }
                        }
                    } else if type_0 as u32 == NODE_TYPE_TEXT as u32
                        && unsafe { (*cur).type_0 as u32 } == XML_CDATA_SECTION_NODE as u32
                    {
                        if hasAxisRange != 0 {
                            pos += 1;
                            if pos == maxPos {
                                if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                    < 0
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                }
                                current_block = 12278438173206364583;
                                break;
                            }
                        } else {
                            if unsafe { addNode.expect("non-null function pointer")(seq, cur) } < 0
                            {
                                safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 795179968803393002;
                                break;
                            }
                        }
                    }
                }
                NODE_TEST_PI => {
                    if unsafe { (*cur).type_0 as u32 } == XML_PI_NODE as u32
                        && (name.is_null() || unsafe { xmlStrEqual(name, (*cur).name) } != 0)
                    {
                        if hasAxisRange != 0 {
                            pos += 1;
                            if pos == maxPos {
                                if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                    < 0
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                }
                                current_block = 12278438173206364583;
                                break;
                            }
                        } else {
                            if unsafe { addNode.expect("non-null function pointer")(seq, cur) } < 0
                            {
                                safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 795179968803393002;
                                break;
                            }
                        }
                    }
                }
                NODE_TEST_ALL => {
                    if axis as u32 == AXIS_ATTRIBUTE as u32 {
                        if unsafe { (*cur).type_0 as u32 } == XML_ATTRIBUTE_NODE as u32 {
                            if (prefix.is_null())
                                || (!unsafe { (*cur).ns.is_null() }
                                    && unsafe { xmlStrEqual(URI, (*(*cur).ns).href) } != 0)
                            {
                                if hasAxisRange != 0 {
                                    pos += 1;
                                    if pos == maxPos {
                                        if unsafe {
                                            addNode.expect("non-null function pointer")(seq, cur)
                                        } < 0
                                        {
                                            safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                        }
                                        current_block = 12278438173206364583;
                                        break;
                                    }
                                } else {
                                    if unsafe {
                                        addNode.expect("non-null function pointer")(seq, cur)
                                    } < 0
                                    {
                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                    }
                                    if breakOnFirstHit != 0 {
                                        current_block = 795179968803393002;
                                        break;
                                    }
                                }
                            }
                            // else if !unsafe { (*cur).ns.is_null() }
                            //     && unsafe { xmlStrEqual(URI, (*(*cur).ns).href) } != 0
                            // {
                            //     if hasAxisRange != 0 as i32 {
                            //         pos += 1;
                            //         if pos == maxPos {
                            //             if unsafe {
                            //                 addNode.expect("non-null function pointer")(seq, cur)
                            //             } < 0 as i32
                            //             {
                            //                 safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                            //             }
                            //             current_block = 12278438173206364583;
                            //             break;
                            //         }
                            //     } else {
                            //         if unsafe {
                            //             addNode.expect("non-null function pointer")(seq, cur)
                            //         } < 0 as i32
                            //         {
                            //             safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                            //         }
                            //         if breakOnFirstHit != 0 {
                            //             current_block = 795179968803393002;
                            //             break;
                            //         }
                            //     }
                            // }
                        }
                    } else if axis as u32 == AXIS_NAMESPACE as u32 {
                        if unsafe { (*cur).type_0 as u32 } == XML_NAMESPACE_DECL as u32 {
                            if hasAxisRange != 0 as i32 {
                                pos += 1;
                                if pos == maxPos {
                                    hasNsNodes = 1;
                                    if unsafe {
                                        xmlXPathNodeSetAddNs(seq, safe_xpctxt.node, cur as xmlNsPtr)
                                    } < 0 as i32
                                    {
                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                    }
                                    current_block = 12278438173206364583;
                                    break;
                                }
                            } else {
                                hasNsNodes = 1;
                                if unsafe {
                                    xmlXPathNodeSetAddNs(seq, safe_xpctxt.node, cur as xmlNsPtr)
                                } < 0 as i32
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 795179968803393002;
                                    break;
                                }
                            }
                        }
                    } else if unsafe { (*cur).type_0 as u32 } == XML_ELEMENT_NODE as u32 {
                        if (prefix.is_null())
                            || (!unsafe { (*cur).ns.is_null() }
                                && unsafe { xmlStrEqual(URI, (*(*cur).ns).href) } != 0)
                        {
                            if hasAxisRange != 0 as i32 {
                                pos += 1;
                                if pos == maxPos {
                                    if unsafe {
                                        addNode.expect("non-null function pointer")(seq, cur)
                                    } < 0 as i32
                                    {
                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                    }
                                    current_block = 12278438173206364583;
                                    break;
                                }
                            } else {
                                if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                                    < 0 as i32
                                {
                                    safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 795179968803393002;
                                    break;
                                }
                            }
                        }
                        // else if !unsafe { (*cur).ns.is_null() }
                        //     && unsafe { xmlStrEqual(URI, (*(*cur).ns).href) } != 0
                        // {
                        //     if hasAxisRange != 0 as i32 {
                        //         pos += 1;
                        //         if pos == maxPos {
                        //             if unsafe {
                        //                 addNode.expect("non-null function pointer")(seq, cur)
                        //             } < 0 as i32
                        //             {
                        //                 safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                        //             }
                        //             current_block = 12278438173206364583;
                        //             break;
                        //         }
                        //     } else {
                        //         if unsafe { addNode.expect("non-null function pointer")(seq, cur) }
                        //             < 0 as i32
                        //         {
                        //             safe_ctxt.error = XPATH_MEMORY_ERROR as i32
                        //         }
                        //         if breakOnFirstHit != 0 {
                        //             current_block = 795179968803393002;
                        //             break;
                        //         }
                        //     }
                        // }
                    }
                }
                NODE_TEST_NS => {
                    unsafe {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Unimplemented block at %s:%d\n\x00" as *const u8 as *const i8,
                            b"xpath.c\x00" as *const u8 as *const i8,
                            12350 as i32,
                        )
                    };
                }
                NODE_TEST_NAME => {
                    if axis as u32 == AXIS_ATTRIBUTE as u32 {
                        if unsafe { (*cur).type_0 as u32 } != XML_ATTRIBUTE_NODE as u32 {
                            current_block = 2652804691515851435;
                        } else {
                            current_block = 8422527538794739384;
                        }
                    } else if axis as u32 == AXIS_NAMESPACE as u32 {
                        if unsafe { (*cur).type_0 as u32 } != XML_NAMESPACE_DECL as u32 {
                            current_block = 2652804691515851435;
                        } else {
                            current_block = 8422527538794739384;
                        }
                    } else if unsafe { (*cur).type_0 as u32 } != XML_ELEMENT_NODE as u32 {
                        current_block = 2652804691515851435;
                    } else {
                        current_block = 8422527538794739384;
                    }
                    match current_block {
                        2652804691515851435 => {}
                        _ => match unsafe { (*cur).type_0 as u32 } {
                            XML_ELEMENT_NODE => {
                                current_block = 8477846364420885724;
                                match current_block {
                                    8477846364420885724 => {
                                        if unsafe { xmlStrEqual(name, (*cur).name) } != 0 {
                                            if prefix.is_null() {
                                                if unsafe { (*cur).ns.is_null() } {
                                                    if hasAxisRange != 0 as i32 {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as i32
                                                            {
                                                                safe_ctxt.error =
                                                                    XPATH_MEMORY_ERROR as i32
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
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
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
                                                if hasAxisRange != 0 as i32 {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as i32
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
                                                    if hasAxisRange != 0 as i32 {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as i32
                                                            {
                                                                safe_ctxt.error =
                                                                    XPATH_MEMORY_ERROR as i32
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
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
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
                                                if hasAxisRange != 0 as i32 {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as i32
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
                                        if unsafe { (*cur).type_0 as u32 }
                                            == XML_NAMESPACE_DECL as u32
                                        {
                                            let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                            if !unsafe { (*ns).prefix.is_null() }
                                                && !name.is_null()
                                                && unsafe { xmlStrEqual((*ns).prefix, name) } != 0
                                            {
                                                if hasAxisRange != 0 as i32 {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        hasNsNodes = 1;
                                                        if unsafe {
                                                            xmlXPathNodeSetAddNs(
                                                                seq,
                                                                safe_xpctxt.node,
                                                                cur as xmlNsPtr,
                                                            )
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    hasNsNodes = 1;
                                                    if unsafe {
                                                        xmlXPathNodeSetAddNs(
                                                            seq,
                                                            safe_xpctxt.node,
                                                            cur as xmlNsPtr,
                                                        )
                                                    } < 0 as i32
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
                            XML_ATTRIBUTE_NODE => {
                                current_block = 15568404490809570198;
                                match current_block {
                                    8477846364420885724 => {
                                        if unsafe { xmlStrEqual(name, (*cur).name) } != 0 {
                                            if prefix.is_null() {
                                                if unsafe { (*cur).ns.is_null() } {
                                                    if hasAxisRange != 0 as i32 {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as i32
                                                            {
                                                                safe_ctxt.error =
                                                                    XPATH_MEMORY_ERROR as i32
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
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
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
                                                if hasAxisRange != 0 as i32 {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as i32
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
                                                    if hasAxisRange != 0 as i32 {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as i32
                                                            {
                                                                safe_ctxt.error =
                                                                    XPATH_MEMORY_ERROR as i32
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
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
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
                                                if hasAxisRange != 0 as i32 {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as i32
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
                                        if unsafe { (*cur).type_0 as u32 }
                                            == XML_NAMESPACE_DECL as u32
                                        {
                                            let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                            if !unsafe { (*ns).prefix.is_null() }
                                                && !name.is_null()
                                                && unsafe { xmlStrEqual((*ns).prefix, name) } != 0
                                            {
                                                if hasAxisRange != 0 as i32 {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        hasNsNodes = 1;
                                                        if unsafe {
                                                            xmlXPathNodeSetAddNs(
                                                                seq,
                                                                safe_xpctxt.node,
                                                                cur as xmlNsPtr,
                                                            )
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    hasNsNodes = 1;
                                                    if unsafe {
                                                        xmlXPathNodeSetAddNs(
                                                            seq,
                                                            safe_xpctxt.node,
                                                            cur as xmlNsPtr,
                                                        )
                                                    } < 0 as i32
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
                            XML_NAMESPACE_DECL => {
                                current_block = 8605932438663696392;
                                match current_block {
                                    8477846364420885724 => {
                                        if unsafe { xmlStrEqual(name, (*cur).name) } != 0 {
                                            if prefix.is_null() {
                                                if unsafe { (*cur).ns.is_null() } {
                                                    if hasAxisRange != 0 as i32 {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as i32
                                                            {
                                                                safe_ctxt.error =
                                                                    XPATH_MEMORY_ERROR as i32
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
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
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
                                                if hasAxisRange != 0 as i32 {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as i32
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
                                                    if hasAxisRange != 0 as i32 {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if unsafe {
                                                                addNode.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    seq, cur
                                                                )
                                                            } < 0 as i32
                                                            {
                                                                safe_ctxt.error =
                                                                    XPATH_MEMORY_ERROR as i32
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
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
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
                                                if hasAxisRange != 0 as i32 {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if unsafe {
                                                            addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            )
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    if unsafe {
                                                        addNode.expect("non-null function pointer")(
                                                            seq, cur,
                                                        )
                                                    } < 0 as i32
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
                                        if unsafe { (*cur).type_0 as u32 }
                                            == XML_NAMESPACE_DECL as u32
                                        {
                                            let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                            if !unsafe { (*ns).prefix.is_null() }
                                                && !name.is_null()
                                                && unsafe { xmlStrEqual((*ns).prefix, name) } != 0
                                            {
                                                if hasAxisRange != 0 as i32 {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        hasNsNodes = 1;
                                                        if unsafe {
                                                            xmlXPathNodeSetAddNs(
                                                                seq,
                                                                safe_xpctxt.node,
                                                                cur as xmlNsPtr,
                                                            )
                                                        } < 0 as i32
                                                        {
                                                            safe_ctxt.error =
                                                                XPATH_MEMORY_ERROR as i32
                                                        }
                                                        current_block = 12278438173206364583;
                                                        break;
                                                    }
                                                } else {
                                                    hasNsNodes = 1;
                                                    if unsafe {
                                                        xmlXPathNodeSetAddNs(
                                                            seq,
                                                            safe_xpctxt.node,
                                                            cur as xmlNsPtr,
                                                        )
                                                    } < 0 as i32
                                                    {
                                                        safe_ctxt.error = XPATH_MEMORY_ERROR as i32
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
            if !(!cur.is_null() && safe_ctxt.error == XPATH_EXPRESSION_OK as i32) {
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
                    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                        break;
                    }
                    /*
                     * Apply predicates.
                     */
                    if !predOp.is_null() && (*seq).nodeNr > 0 {
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
                        if hasPredicateRange != 0 {
                            xmlXPathCompOpEvalPredicate(
                                ctxt, predOp, seq, maxPos, maxPos, hasNsNodes,
                            );
                        } else {
                            xmlXPathCompOpEvalPredicate(
                                ctxt,
                                predOp,
                                seq,
                                1,
                                (*seq).nodeNr,
                                hasNsNodes,
                            );
                        }
                        if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                            total = 0;
                            break;
                        }
                    }
                    if !((*seq).nodeNr > 0) {
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
        unsafe { (*safe_ctxt.value).boolval = 1 };
        unsafe { (*safe_ctxt.value).user = safe_obj.user };
        safe_obj.user = 0 as *mut ();
        safe_obj.boolval = 0
    }
    unsafe { xmlXPathReleaseObject(xpctxt, obj) };
    /*
     * Ensure we return at least an empty set.
     */
    if outSeq.is_null() {
        if !seq.is_null() && unsafe { (*seq).nodeNr == 0 as i32 } {
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
        unsafe { xmlFree.expect("non-null function pointer")(safe_xpctxt.tmpNsList as *mut ()) };
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
    ctxt: xmlXPathParserContextPtr,
    op: xmlXPathStepOpPtr,
    first: *mut xmlNodePtr,
) -> i32 {
    let mut total: i32 = 0 as i32;
    let mut cur: i32;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return 0;
    }
    if unsafe { (*safe_ctxt.context).opLimit != 0 } && unsafe { xmlXPathCheckOpLimit(ctxt, 1) } < 0
    {
        return 0;
    }
    if unsafe { (*safe_ctxt.context).depth >= 5000 } {
        unsafe { xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as i32) };
        return 0;
    }
    unsafe { (*safe_ctxt.context).depth += 1 };
    comp = safe_ctxt.comp;
    let safe_op = unsafe { &mut *op };
    match safe_op.op as u32 {
        XPATH_OP_END => {}
        XPATH_OP_UNION => {
            total = unsafe {
                xmlXPathCompOpEvalFirst(
                    ctxt,
                    &mut *(*comp).steps.offset(safe_op.ch1 as isize),
                    first,
                )
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if !safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as u32 == XPATH_NODESET as u32
                        && !(*safe_ctxt.value).nodesetval.is_null()
                        && (*(*safe_ctxt.value).nodesetval).nodeNr >= 1
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
                if unsafe { (*(*safe_ctxt.value).nodesetval).nodeNr > 1 } {
                    unsafe { xmlXPathNodeSetSort((*safe_ctxt.value).nodesetval) };
                }
                unsafe { *first = *(*(*safe_ctxt.value).nodesetval).nodeTab.offset(0) }
            }
            cur = unsafe {
                xmlXPathCompOpEvalFirst(
                    ctxt,
                    &mut *(*comp).steps.offset(safe_op.ch2 as isize),
                    first,
                )
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            arg2 = unsafe { valuePop(ctxt) };
            arg1 = unsafe { valuePop(ctxt) };
            let safe_arg1 = unsafe { &mut *arg1 };
            let safe_arg2 = unsafe { &mut *arg2 };
            if arg1.is_null()
                || safe_arg1.type_0 as u32 != XPATH_NODESET as u32
                || arg2.is_null()
                || safe_arg2.type_0 as u32 != XPATH_NODESET as u32
            {
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
                unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
                return 0;
            }
            if unsafe { (*safe_ctxt.context).opLimit != 0 as i32 as u64 }
                && (!safe_arg1.nodesetval.is_null()
                    && unsafe { xmlXPathCheckOpLimit(ctxt, (*safe_arg1.nodesetval).nodeNr as u64) }
                        < 0
                    || !safe_arg2.nodesetval.is_null()
                        && unsafe {
                            xmlXPathCheckOpLimit(ctxt, (*safe_arg2.nodesetval).nodeNr as u64)
                        } < 0)
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
        XPATH_OP_ROOT => {
            unsafe { xmlXPathRoot(ctxt) };
        }
        XPATH_OP_NODE => {
            if safe_op.ch1 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *(*comp).steps.offset(safe_op.ch1 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if safe_op.ch2 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *(*comp).steps.offset(safe_op.ch2 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
                )
            };
        }
        XPATH_OP_COLLECT => {
            if !(safe_op.ch1 == -1) {
                total = unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *(*comp).steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                    return 0;
                }
                total += unsafe {
                    xmlXPathNodeCollectAndTest(ctxt, op, first, 0 as *mut xmlNodePtr, 0 as i32)
                }
            }
        }
        XPATH_OP_VALUE => {
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheObjectCopy(safe_ctxt.context, safe_op.value4 as xmlXPathObjectPtr),
                )
            };
        }
        XPATH_OP_SORT => {
            if safe_op.ch1 != -1 {
                total += unsafe {
                    xmlXPathCompOpEvalFirst(
                        ctxt,
                        &mut *(*comp).steps.offset(safe_op.ch1 as isize),
                        first,
                    )
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if !safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as u32 == XPATH_NODESET as u32
                        && !(*safe_ctxt.value).nodesetval.is_null()
                        && (*(*safe_ctxt.value).nodesetval).nodeNr > 1
                }
            {
                unsafe { xmlXPathNodeSetSort((*safe_ctxt.value).nodesetval) };
            }
        }

        XPATH_OP_FILTER => {
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
    unsafe { (*safe_ctxt.context).depth -= 1 };
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
    ctxt: xmlXPathParserContextPtr,
    op: xmlXPathStepOpPtr,
    last: *mut xmlNodePtr,
) -> i32 {
    let mut total: i32 = 0 as i32;
    let mut cur: i32 = 0;
    let mut comp: xmlXPathCompExprPtr;
    let mut arg1: xmlXPathObjectPtr;
    let mut arg2: xmlXPathObjectPtr;
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return 0 as i32;
    }
    if unsafe { (*safe_ctxt.context).opLimit != 0 }
        && unsafe { xmlXPathCheckOpLimit(ctxt, 1 as u64) } < 0 as i32
    {
        return 0 as i32;
    }
    if unsafe { (*safe_ctxt.context).depth >= 5000 as i32 } {
        unsafe { xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as i32) };
        return 0 as i32;
    }
    unsafe { (*safe_ctxt.context).depth += 1 };
    comp = safe_ctxt.comp;
    let safe_op = unsafe { &mut *op };
    let safe_comp = unsafe { &mut *comp };
    match safe_op.op as u32 {
        XPATH_OP_END => {}
        XPATH_OP_UNION => {
            total = unsafe {
                xmlXPathCompOpEvalLast(
                    ctxt,
                    &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                    last,
                )
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if !safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as u32 == XPATH_NODESET as u32
                        && !(*safe_ctxt.value).nodesetval.is_null()
                        && (*(*safe_ctxt.value).nodesetval).nodeNr >= 1
                }
            {
                /*
                 * limit tree traversing to first node in the result
                 */
                if unsafe { (*(*safe_ctxt.value).nodesetval).nodeNr > 1 } {
                    unsafe { xmlXPathNodeSetSort((*safe_ctxt.value).nodesetval) };
                }
                unsafe {
                    *last = *(*(*safe_ctxt.value).nodesetval)
                        .nodeTab
                        .offset(((*(*safe_ctxt.value).nodesetval).nodeNr - 1) as isize)
                }
            }
            cur = unsafe {
                xmlXPathCompOpEvalLast(
                    ctxt,
                    &mut *safe_comp.steps.offset(safe_op.ch2 as isize),
                    last,
                )
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            (!safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as u32 == XPATH_NODESET as u32
                        && !(*safe_ctxt.value).nodesetval.is_null()
                })
                && unsafe { (*(*safe_ctxt.value).nodesetval).nodeNr >= 1 };
            arg2 = unsafe { valuePop(ctxt) };
            arg1 = unsafe { valuePop(ctxt) };
            let safe_arg1 = unsafe { &mut *arg1 };
            let safe_arg2 = unsafe { &mut *arg2 };
            if arg1.is_null()
                || safe_arg1.type_0 as u32 != XPATH_NODESET as u32
                || arg2.is_null()
                || safe_arg1.type_0 as u32 != XPATH_NODESET as u32
            {
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
                unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
                return 0;
            }
            if unsafe { (*safe_ctxt.context).opLimit != 0 as i32 as u64 }
                && (!safe_arg1.nodesetval.is_null()
                    && unsafe { xmlXPathCheckOpLimit(ctxt, (*safe_arg1.nodesetval).nodeNr as u64) }
                        < 0 as i32
                    || !safe_arg1.nodesetval.is_null()
                        && unsafe {
                            xmlXPathCheckOpLimit(ctxt, (*safe_arg1.nodesetval).nodeNr as u64)
                        } < 0 as i32)
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
        XPATH_OP_ROOT => {
            unsafe { xmlXPathRoot(ctxt) };
        }
        XPATH_OP_NODE => {
            if safe_op.ch1 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if safe_op.ch2 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
                )
            };
        }
        XPATH_OP_COLLECT => {
            if !(safe_op.ch1 == -1) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                    return 0;
                }
                total += unsafe {
                    xmlXPathNodeCollectAndTest(ctxt, op, 0 as *mut xmlNodePtr, last, 0 as i32)
                }
            }
        }
        XPATH_OP_VALUE => {
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheObjectCopy(safe_ctxt.context, safe_op.value4 as xmlXPathObjectPtr),
                )
            };
        }
        XPATH_OP_SORT => {
            if safe_op.ch1 != -1 {
                total += unsafe {
                    xmlXPathCompOpEvalLast(
                        ctxt,
                        &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                        last,
                    )
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if !safe_ctxt.value.is_null()
                && unsafe {
                    (*safe_ctxt.value).type_0 as u32 == XPATH_NODESET as u32
                        && !(*safe_ctxt.value).nodesetval.is_null()
                        && (*(*safe_ctxt.value).nodesetval).nodeNr > 1
                }
            {
                unsafe { xmlXPathNodeSetSort((*safe_ctxt.value).nodesetval) };
            }
        }
        _ => total += unsafe { xmlXPathCompOpEval(ctxt, op) },
    }
    unsafe { (*safe_ctxt.context).depth -= 1 };
    return total;
}

#[cfg(XP_OPTIMIZED_FILTER_FIRST)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompOpEvalFilterFirst(
    ctxt: xmlXPathParserContextPtr,
    op: xmlXPathStepOpPtr,
    first: *mut xmlNodePtr,
) -> i32 {
    let mut total: i32 = 0 as i32;
    let comp: xmlXPathCompExprPtr;
    let set: xmlNodeSetPtr;
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return 0;
    }
    comp = safe_ctxt.comp;
    let safe_comp = unsafe { &mut *comp };
    let safe_op = unsafe { &mut *op };
    /*
     * Optimization for ()[last()] selection i.e. the last elem
     */
    if safe_op.ch1 != -1
        && safe_op.ch2 != -1
        && unsafe { (*safe_comp.steps.offset(safe_op.ch1 as isize)).op as u32 }
            == XPATH_OP_SORT as u32
        && unsafe { (*safe_comp.steps.offset(safe_op.ch2 as isize)).op as u32 }
            == XPATH_OP_SORT as u32
    {
        let mut f: i32 = unsafe { (*safe_comp.steps.offset(safe_op.ch2 as isize)).ch1 };
        if f != -1
            && unsafe { (*safe_comp.steps.offset(f as isize)).op as u32 }
                == XPATH_OP_FUNCTION as u32
            && unsafe { (*safe_comp.steps.offset(f as isize)).value5.is_null() }
            && unsafe { (*safe_comp.steps.offset(f as isize)).value == 0 as i32 }
            && !unsafe { (*safe_comp.steps.offset(f as isize)).value4.is_null() }
            && unsafe {
                xmlStrEqual(
                    (*safe_comp.steps.offset(f as isize)).value4 as *const xmlChar,
                    b"last\x00" as *const u8 as *const i8 as *mut xmlChar,
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

            if unsafe { safe_ctxt.error != XPATH_EXPRESSION_OK as i32 } {
                return 0;
            }
            /*
             * The nodeset should be in document order,
             * Keep only the last value
             */
            if !unsafe {
                safe_ctxt.value.is_null()
                    && (*safe_ctxt.value).type_0 as u32 == XPATH_NODESET as u32
                    && !(*safe_ctxt.value).nodesetval.is_null()
                    && !(*(*safe_ctxt.value).nodesetval).nodeTab.is_null()
                    && (*(*safe_ctxt.value).nodesetval).nodeNr > 1
            } {
                unsafe {
                    xmlXPathNodeSetKeepLast((*safe_ctxt.value).nodesetval);
                    *first = *(*(*safe_ctxt.value).nodesetval).nodeTab
                }
            }
            return total;
        }
    }
    if safe_op.ch1 != -1 {
        total +=
            unsafe { xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize)) }
    }
    if unsafe { safe_ctxt.error != XPATH_EXPRESSION_OK as i32 } {
        return 0;
    }
    if safe_op.ch2 == -1 {
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
            if unsafe { (*safe_ctxt.value).type_0 as u32 } == XPATH_LOCATIONSET as u32 {
                let mut locset: xmlLocationSetPtr =
                    unsafe { (*safe_ctxt.value).user as xmlLocationSetPtr };
                if !locset.is_null() {
                    unsafe { xmlXPathLocationSetFilter(ctxt, locset, safe_op.ch2, 1, 1) };
                    if unsafe { (*locset).locNr > 0 } {
                        unsafe { *first = (**(*locset).locTab.offset(0)).user as xmlNodePtr }
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
    if unsafe { (*safe_ctxt.value).type_0 as u32 } == XPATH_LOCATIONSET as u32 {
        let mut locset: xmlLocationSetPtr = unsafe { (*safe_ctxt.value).user as xmlLocationSetPtr };
        if !locset.is_null() {
            unsafe { xmlXPathLocationSetFilter(ctxt, locset, safe_op.ch2, 1, 1) };
            if unsafe { (*locset).locNr > 0 } {
                unsafe { *first = (**(*locset).locTab.offset(0)).user as xmlNodePtr }
            }
        }
        return total;
    }
    /* LIBXML_XPTR_ENABLED */
    if safe_ctxt.value.is_null()
        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NODESET as u32
    {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
        return 0;
    }
    unsafe { set = (*safe_ctxt.value).nodesetval };
    if !set.is_null() {
        unsafe { xmlXPathNodeSetFilter(ctxt, set, safe_op.ch2, 1, 1, 1) };
        if unsafe { (*set).nodeNr > 0 } {
            unsafe { *first = *(*set).nodeTab.offset(0) }
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
fn xmlXPathCompOpEval(ctxt: xmlXPathParserContextPtr, op: xmlXPathStepOpPtr) -> i32 {
    let mut current_block: u64;
    let mut total: i32 = 0 as i32;
    let mut equal: i32 = 0;
    let mut ret: i32 = 0;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
        return 0;
    }
    if unsafe { (*safe_ctxt.context).opLimit != 0 && xmlXPathCheckOpLimit(ctxt, 1) < 0 } {
        return 0;
    }
    if unsafe { (*safe_ctxt.context).depth >= 5000 } {
        unsafe { xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as i32) };
        return 0;
    }
    unsafe { (*safe_ctxt.context).depth += 1 };
    comp = safe_ctxt.comp;
    let safe_op = unsafe { &mut *op };
    let safe_comp = unsafe { &mut *comp };
    match safe_op.op as u32 {
        XPATH_OP_END => {}
        XPATH_OP_AND => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            unsafe { xmlXPathBooleanFunction(ctxt, 1) };
            if !(safe_ctxt.value.is_null() || unsafe { (*safe_ctxt.value).boolval == 0 as i32 }) {
                arg2 = unsafe { valuePop(ctxt) };
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                };
                if safe_ctxt.error != 0 {
                    unsafe { xmlXPathFreeObject(arg2) };
                } else {
                    unsafe { xmlXPathBooleanFunction(ctxt, 1) };
                    if !safe_ctxt.value.is_null() {
                        unsafe { (*safe_ctxt.value).boolval &= (*arg2).boolval }
                    }
                    unsafe { xmlXPathReleaseObject((*ctxt).context, arg2) };
                }
            }
        }
        XPATH_OP_OR => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            unsafe { xmlXPathBooleanFunction(ctxt, 1) };
            if !(safe_ctxt.value.is_null() || unsafe { (*safe_ctxt.value).boolval == 1 }) {
                arg2 = unsafe { valuePop(ctxt) };
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                };
                if safe_ctxt.error != 0 {
                    unsafe { xmlXPathFreeObject(arg2) };
                } else {
                    unsafe { xmlXPathBooleanFunction(ctxt, 1) };
                    if !safe_ctxt.value.is_null() {
                        unsafe { (*safe_ctxt.value).boolval |= (*arg2).boolval }
                    }
                    unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
                }
            }
        }
        XPATH_OP_EQUAL => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if safe_op.value != 0 {
                equal = unsafe { xmlXPathEqualValues(ctxt) }
            } else {
                equal = unsafe { xmlXPathNotEqualValues(ctxt) }
            }
            unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, equal)) };
        }
        XPATH_OP_CMP => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            ret = unsafe { xmlXPathCompareValues(ctxt, safe_op.value, safe_op.value2) };
            unsafe { valuePush(ctxt, xmlXPathCacheNewBoolean(safe_ctxt.context, ret)) };
        }
        XPATH_OP_PLUS => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if safe_op.ch2 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if safe_op.value == 0 {
                unsafe { xmlXPathSubValues(ctxt) };
            } else if safe_op.value == 1 {
                unsafe { xmlXPathAddValues(ctxt) };
            } else if safe_op.value == 2 {
                unsafe { xmlXPathValueFlipSign(ctxt) };
            } else if safe_op.value == 3 {
                if !safe_ctxt.value.is_null()
                    && unsafe { (*safe_ctxt.value).type_0 as u32 != XPATH_NUMBER as u32 }
                {
                    unsafe { xmlXPathNumberFunction(ctxt, 1) };
                }
                if safe_ctxt.value.is_null()
                    || unsafe { (*safe_ctxt.value).type_0 as u32 != XPATH_NUMBER as u32 }
                {
                    unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
                    return 0;
                }
            }
        }
        XPATH_OP_MULT => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if safe_op.value == 0 {
                unsafe { xmlXPathMultValues(ctxt) };
            } else if safe_op.value == 1 {
                unsafe { xmlXPathDivValues(ctxt) };
            } else if safe_op.value == 2 {
                unsafe { xmlXPathModValues(ctxt) };
            }
        }
        XPATH_OP_UNION => {
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            total += unsafe {
                xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
            };
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            arg2 = unsafe { valuePop(ctxt) };
            arg1 = unsafe { valuePop(ctxt) };
            let safe_arg1 = unsafe { &mut *arg1 };
            let safe_arg2 = unsafe { &mut *arg2 };
            if arg1.is_null()
                || safe_arg1.type_0 as u32 != XPATH_NODESET as u32
                || arg2.is_null()
                || safe_arg2.type_0 as u32 != XPATH_NODESET as u32
            {
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
                unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
                return 0 as i32;
            }
            if unsafe { (*safe_ctxt.context).opLimit != 0 as i32 as u64 }
                && (!safe_arg1.nodesetval.is_null()
                    && unsafe {
                        xmlXPathCheckOpLimit(ctxt, (*safe_arg1.nodesetval).nodeNr as u64) < 0 as i32
                    }
                    || !safe_arg2.nodesetval.is_null()
                        && unsafe {
                            xmlXPathCheckOpLimit(ctxt, (*safe_arg2.nodesetval).nodeNr as u64) < 0
                        })
            {
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
            } else {
                if safe_arg1.nodesetval.is_null()
                    || !safe_arg2.nodesetval.is_null()
                        && unsafe { (*safe_arg2.nodesetval).nodeNr != 0 as i32 }
                {
                    /* TODO: Check memory error. */
                    safe_arg1.nodesetval =
                        unsafe { xmlXPathNodeSetMerge(safe_arg1.nodesetval, safe_arg2.nodesetval) }
                }
                unsafe { valuePush(ctxt, arg1) };
                unsafe { xmlXPathReleaseObject(safe_ctxt.context, arg2) };
            }
        }
        XPATH_OP_ROOT => {
            unsafe { xmlXPathRoot(ctxt) };
        }
        XPATH_OP_NODE => {
            if safe_op.ch1 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if safe_op.ch2 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewNodeSet(safe_ctxt.context, (*safe_ctxt.context).node),
                )
            };
        }
        XPATH_OP_COLLECT => {
            if !(safe_op.ch1 == -1) {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                    return 0;
                }
                total += unsafe {
                    xmlXPathNodeCollectAndTest(
                        ctxt,
                        op,
                        0 as *mut xmlNodePtr,
                        0 as *mut xmlNodePtr,
                        0,
                    )
                }
            }
        }
        XPATH_OP_VALUE => {
            unsafe {
                valuePush(
                    ctxt,
                    xmlXPathCacheObjectCopy(safe_ctxt.context, safe_op.value4 as xmlXPathObjectPtr),
                )
            };
        }
        XPATH_OP_VARIABLE => {
            let mut val: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            if safe_op.ch1 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                }
            }
            if safe_op.value5.is_null() {
                val = unsafe {
                    xmlXPathVariableLookup(safe_ctxt.context, safe_op.value4 as *const xmlChar)
                };
                if val.is_null() {
                    unsafe { xmlXPathErr(ctxt, XPATH_UNDEF_VARIABLE_ERROR as i32) };
                    return 0;
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
                                as *const u8 as *const i8,
                            safe_op.value4 as *mut i8,
                            safe_op.value5 as *mut i8,
                        )
                    };
                    safe_ctxt.error = XPATH_UNDEF_PREFIX_ERROR as i32
                } else {
                    val = unsafe {
                        xmlXPathVariableLookupNS(
                            safe_ctxt.context,
                            safe_op.value4 as *const xmlChar,
                            URI,
                        )
                    };
                    if val.is_null() {
                        unsafe { xmlXPathErr(ctxt, XPATH_UNDEF_VARIABLE_ERROR as i32) };
                        return 0;
                    }
                    unsafe { valuePush(ctxt, val) };
                }
            }
        }
        XPATH_OP_FUNCTION => {
            let mut func: xmlXPathFunction = None;
            let mut oldFunc: *const xmlChar = 0 as *const xmlChar;
            let mut oldFuncURI: *const xmlChar = 0 as *const xmlChar;
            let mut i: i32 = 0;
            let mut frame: i32 = 0;
            frame = unsafe { xmlXPathSetFrame(ctxt) };
            if safe_op.ch1 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
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
                                    as *const i8,
                            )
                        };
                        safe_ctxt.error = XPATH_INVALID_OPERAND as i32;
                        unsafe { xmlXPathPopFrame(ctxt, frame) };
                    } else {
                        i = 0;
                        while i < safe_op.value {
                            if unsafe {
                                (*safe_ctxt
                                    .valueTab
                                    .offset((safe_ctxt.valueNr - 1 - i) as isize))
                                .is_null()
                            } {
                                unsafe {
                                    (*__xmlGenericError()).expect("non-null function pointer")(
                                        *__xmlGenericErrorContext(),
                                        b"xmlXPathCompOpEval: parameter error\n\x00" as *const u8
                                            as *const i8,
                                    )
                                };
                                safe_ctxt.error = XPATH_INVALID_OPERAND as i32;
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
                                                                                                   as *const i8, safe_op.value4
                                                                                                   as *mut i8, safe_op.value5
                                                                                                   as *mut i8)
                                    };
                                    unsafe { xmlXPathPopFrame(ctxt, frame) };
                                    safe_ctxt.error = XPATH_UNDEF_PREFIX_ERROR as i32;
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
                                                    as *const i8,
                                                safe_op.value4 as *mut i8,
                                            )
                                        };
                                        unsafe {
                                            xmlXPathErr(ctxt, XPATH_UNKNOWN_FUNC_ERROR as i32)
                                        };
                                        return 0 as i32;
                                    }
                                    safe_op.cache = func;
                                    safe_op.cacheURI = URI_0 as *mut ();
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
                                if safe_ctxt.error == XPATH_EXPRESSION_OK as i32
                                    && safe_ctxt.valueNr != safe_ctxt.valueFrame + 1
                                {
                                    unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
                                    return 0;
                                }
                                unsafe { xmlXPathPopFrame(ctxt, frame) };
                            }
                        }
                    }
                }
            }
        }
        XPATH_OP_ARG => {
            if safe_op.ch1 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                    return 0;
                }
            }
            if safe_op.ch2 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                    return 0;
                }
            }
        }
        XPATH_OP_PREDICATE | XPATH_OP_FILTER => {
            let mut set: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
            /*
             * Optimization for ()[1] selection i.e. the first elem
             */
            let mut XP_OPTIMIZED_FILTER_FIRST_FLAG: i32 = 0;
            // XP_OPTIMIZED_FILTER_FIRST
            match () {
                #[cfg(XP_OPTIMIZED_FILTER_FIRST)]
                _ => {
                    if unsafe {
                        ((*safe_comp.steps.offset(safe_op.ch1 as isize)).op as u32
                            == XPATH_OP_SORT as u32
                            || (*safe_comp.steps.offset(safe_op.ch1 as isize)).op as u32
                                == XPATH_OP_FILTER as u32)
                    } {
                        XP_OPTIMIZED_FILTER_FIRST_FLAG = 1;
                    }
                }
                // else
                #[cfg(not(XP_OPTIMIZED_FILTER_FIRST))]
                _ => {
                    if unsafe {
                        (*safe_comp.steps.offset(safe_op.ch1 as isize)).op as u32
                            == XPATH_OP_SORT as u32
                    } {
                        XP_OPTIMIZED_FILTER_FIRST_FLAG = 1;
                    }
                }
            }
            // endif XP_OPTIMIZED_FILTER_FIRST

            if safe_op.ch1 != -1
                && safe_op.ch2 != -1
                && (XP_OPTIMIZED_FILTER_FIRST_FLAG == (1))
                && unsafe {
                    (*safe_comp.steps.offset(safe_op.ch2 as isize)).op as u32
                        == XPATH_OP_VALUE as u32
                }
            {
                /* 12 */
                let mut val_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                val_0 = unsafe {
                    (*safe_comp.steps.offset(safe_op.ch2 as isize)).value4 as xmlXPathObjectPtr
                };
                let safe_val_0 = unsafe { &mut *val_0 };
                if !val_0.is_null()
                    && safe_val_0.type_0 as u32 == XPATH_NUMBER as u32
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
                    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                        return 0;
                    }
                    /*
                     * The nodeset should be in document order, * Keep only the first value
                     */
                    if !safe_ctxt.value.is_null()
                        && unsafe {
                            (*safe_ctxt.value).type_0 as u32 == XPATH_NODESET as u32
                                && !(*safe_ctxt.value).nodesetval.is_null()
                                && (*(*safe_ctxt.value).nodesetval).nodeNr > 1
                        }
                    {
                        unsafe { xmlXPathNodeSetClearFromPos((*safe_ctxt.value).nodesetval, 1, 1) };
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
                    if safe_op.ch1 != -1
                        && safe_op.ch2 != -1
                        && unsafe { (*safe_comp.steps.offset(safe_op.ch1 as isize)).op as u32 }
                            == XPATH_OP_SORT as u32
                        && unsafe { (*safe_comp.steps.offset(safe_op.ch2 as isize)).op as u32 }
                            == XPATH_OP_SORT as u32
                    {
                        let mut f: i32 =
                            unsafe { (*safe_comp.steps.offset(safe_op.ch2 as isize)).ch1 };
                        if f != -1
                            && unsafe { (*safe_comp.steps.offset(f as isize)).op as u32 }
                                == XPATH_OP_FUNCTION as u32
                            && unsafe { (*safe_comp.steps.offset(f as isize)).value5.is_null() }
                            && unsafe { (*safe_comp.steps.offset(f as isize)).value == 0 as i32 }
                            && !unsafe { (*safe_comp.steps.offset(f as isize)).value4.is_null() }
                            && unsafe {
                                xmlStrEqual(
                                    (*safe_comp.steps.offset(f as isize)).value4 as *const xmlChar,
                                    b"last\x00" as *const u8 as *const i8 as *mut xmlChar,
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
                            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                                return 0;
                            }
                            /*
                             * The nodeset should be in document order, * Keep only the last value
                             */
                            if !safe_ctxt.value.is_null()
                                && unsafe { (*safe_ctxt.value).type_0 as u32 }
                                    == XPATH_NODESET as u32
                                && !unsafe { (*safe_ctxt.value).nodesetval.is_null() }
                                && !unsafe { (*(*safe_ctxt.value).nodesetval).nodeTab.is_null() }
                                && unsafe { (*(*safe_ctxt.value).nodesetval).nodeNr } > 1
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
                            if safe_op.ch1 != -1 {
                                total += unsafe {
                                    xmlXPathCompOpEval(
                                        ctxt,
                                        &mut *safe_comp.steps.offset(safe_op.ch1 as isize),
                                    )
                                }
                            }
                            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                                return 0;
                            }
                            if !(safe_op.ch2 == -1) {
                                if !safe_ctxt.value.is_null() {
                                    /*
                                     * Hum are we filtering the result of an XPointer expression
                                     */
                                    if unsafe { (*safe_ctxt.value).type_0 as u32 }
                                        == XPATH_LOCATIONSET as u32
                                    {
                                        let mut locset: xmlLocationSetPtr =
                                            unsafe { (*safe_ctxt.value).user as xmlLocationSetPtr };
                                        unsafe {
                                            xmlXPathLocationSetFilter(
                                                ctxt,
                                                locset,
                                                safe_op.ch2,
                                                1,
                                                (*locset).locNr,
                                            )
                                        };
                                    } else {
                                        /* LIBXML_XPTR_ENABLED */
                                        if safe_ctxt.value.is_null()
                                            || unsafe { (*safe_ctxt.value).type_0 as u32 }
                                                != XPATH_NODESET as u32
                                        {
                                            unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) }; /* Not a location set */
                                            return 0;
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
                                                    1,
                                                    (*set).nodeNr,
                                                    1,
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
        XPATH_OP_SORT => {
            if safe_op.ch1 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                }
            }
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return 0;
            }
            if !safe_ctxt.value.is_null()
                && unsafe { (*safe_ctxt.value).type_0 as u32 == XPATH_NODESET as u32 }
                && !unsafe { (*safe_ctxt.value).nodesetval.is_null() }
                && unsafe { (*(*safe_ctxt.value).nodesetval).nodeNr > 1 }
            {
                unsafe { xmlXPathNodeSetSort((*safe_ctxt.value).nodesetval) };
            }
        }
        XPATH_OP_RANGETO => {
            let mut range: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut newlocset: xmlLocationSetPtr = 0 as xmlLocationSetPtr;
            let mut oldlocset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
            let mut oldset: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
            let mut oldnode: xmlNodePtr = unsafe { (*safe_ctxt.context).node };
            let mut oldcs: i32 = unsafe { (*safe_ctxt.context).contextSize };
            let mut oldpp: i32 = unsafe { (*safe_ctxt.context).proximityPosition };
            let mut i_0: i32 = 0;
            let mut j: i32 = 0;
            if safe_op.ch1 != -1 {
                total += unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
                };
                if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                    return 0;
                }
            }
            if safe_ctxt.value.is_null() {
                unsafe { xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as i32) };
                return 0;
            }
            if !(safe_op.ch2 == -1) {
                if unsafe { (*safe_ctxt.value).type_0 as u32 } == XPATH_LOCATIONSET as u32 {
                    /*
                     * Extract the old locset, and then evaluate the result of the
                     * expression for all the element in the locset. use it to grow
                     * up a new locset.
                     */
                    if safe_ctxt.value.is_null()
                        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_LOCATIONSET as u32
                    {
                        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
                        return 0;
                    }
                    if unsafe { (*safe_ctxt.value).user.is_null() }
                        || unsafe { (*((*safe_ctxt.value).user as xmlLocationSetPtr)).locNr } == 0
                    {
                        current_block = 16794583624483845564;
                    } else {
                        unsafe {
                            obj = valuePop(ctxt);
                            oldlocset = (*obj).user as xmlLocationSetPtr;
                            newlocset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr)
                        };
                        i_0 = 0;
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
                                (*safe_ctxt.context).proximityPosition = i_0 + 1;
                                tmp = xmlXPathCacheNewNodeSet(
                                    safe_ctxt.context,
                                    (*safe_ctxt.context).node,
                                );
                                valuePush(ctxt, tmp);
                            }
                            if safe_op.ch2 != -1 {
                                total += unsafe {
                                    xmlXPathCompOpEval(
                                        ctxt,
                                        &mut *safe_comp.steps.offset(safe_op.ch2 as isize),
                                    )
                                }
                            }
                            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                                unsafe { xmlXPtrFreeLocationSet(newlocset) };
                                current_block = 4361884637321370770;
                                break;
                            } else {
                                res = unsafe { valuePop(ctxt) };
                                let safe_res = unsafe { &mut *res };
                                if safe_res.type_0 as u32 == XPATH_LOCATIONSET as u32 {
                                    let mut rloc: xmlLocationSetPtr =
                                        safe_res.user as xmlLocationSetPtr;
                                    j = 0 as i32;
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
                        || unsafe { (*safe_ctxt.value).type_0 as u32 } != XPATH_NODESET as u32
                    {
                        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_TYPE as i32) };
                        return 0;
                    }
                    obj = unsafe { valuePop(ctxt) };
                    let safe_obj = unsafe { &mut *obj };
                    oldset = safe_obj.nodesetval;
                    newlocset = unsafe { xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr) };
                    let safe_oldset = unsafe { &mut *oldset };
                    if !oldset.is_null() {
                        i_0 = 0;
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
                            if safe_op.ch2 != -1 {
                                total += unsafe {
                                    xmlXPathCompOpEval(
                                        ctxt,
                                        &mut *safe_comp.steps.offset(safe_op.ch2 as isize),
                                    )
                                }
                            }
                            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
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
                    b"XPath: unknown precompiled operation %d\n\x00" as *const u8 as *const i8,
                    safe_op.op as u32,
                )
            };
            safe_ctxt.error = XPATH_INVALID_OPERAND as i32
        }
    }
    unsafe { (*safe_ctxt.context).depth -= 1 };
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
    ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    isPredicate: i32,
) -> i32 {
    let mut resObj: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    loop {
        if unsafe { (*(*ctxt).context).opLimit != 0 && xmlXPathCheckOpLimit(ctxt, 1) < 0 } {
            return 0;
        }
        /* comp = ctxt->comp; */
        match unsafe { (*op).op as u32 } {
            XPATH_OP_END => return 0,
            XPATH_OP_VALUE => {
                unsafe { resObj = (*op).value4 as xmlXPathObjectPtr };
                if isPredicate != 0 {
                    return unsafe { xmlXPathEvaluatePredicateResult(ctxt, resObj) };
                }
                return unsafe { xmlXPathCastToBoolean(resObj) };
            }
            XPATH_OP_SORT => {
                /*
                 * We don't need sorting for boolean results. Skip this one.
                 */
                if unsafe { (*op).ch1 != -1 } {
                    op = unsafe {
                        &mut *(*(*ctxt).comp).steps.offset((*op).ch1 as isize)
                            as *mut xmlXPathStepOp
                    }
                } else {
                    return 0;
                }
            }
            XPATH_OP_COLLECT => {
                if unsafe { (*op).ch1 == -1 } {
                    return 0;
                }
                unsafe {
                    xmlXPathCompOpEval(ctxt, &mut *(*(*ctxt).comp).steps.offset((*op).ch1 as isize))
                };
                if unsafe { (*ctxt).error != XPATH_EXPRESSION_OK as i32 } {
                    return -1;
                }
                unsafe {
                    xmlXPathNodeCollectAndTest(
                        ctxt,
                        op,
                        0 as *mut xmlNodePtr,
                        0 as *mut xmlNodePtr,
                        1,
                    )
                };
                if unsafe { (*ctxt).error != XPATH_EXPRESSION_OK as i32 } {
                    return -1;
                }
                resObj = unsafe { valuePop(ctxt) };
                if resObj.is_null() {
                    return -1;
                }
                break;
            }
            _ => {
                /*
                 * Fallback to call xmlXPathCompOpEval().
                 */
                unsafe { xmlXPathCompOpEval(ctxt, op) };
                if unsafe { (*ctxt).error != XPATH_EXPRESSION_OK as i32 } {
                    return -1;
                }
                resObj = unsafe { valuePop(ctxt) };
                if resObj.is_null() {
                    return -1;
                }
                break;
            }
        }
    }
    if !resObj.is_null() {
        let mut res: i32 = 0;
        let safe_resObj = unsafe { &mut *resObj };
        if safe_resObj.type_0 as u32 == XPATH_BOOLEAN as u32 {
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
    return 0;
}
/* *
 * xmlXPathRunStreamEval: * @ctxt: the XPath parser context with the compiled expression
 *
 * Evaluate the Precompiled Streamable XPath expression in the given context.
 */
#[cfg(XPATH_STREAMING)]
#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathRunStreamEval(
    ctxt: xmlXPathContextPtr,
    comp: xmlPatternPtr,
    resultSeq: *mut xmlXPathObjectPtr,
    toBool: i32,
) -> i32 {
    let mut current_block: u64;
    let mut max_depth;
    let mut min_depth;
    let mut from_root;
    let mut ret;
    let mut depth;
    let mut eval_all_nodes;
    let mut cur: xmlNodePtr = 0 as xmlNodePtr;
    let mut limit: xmlNodePtr = 0 as xmlNodePtr;
    let mut patstream: xmlStreamCtxtPtr = 0 as xmlStreamCtxtPtr;
    let mut nb_nodes: i32 = 0;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || comp.is_null() {
        return -1;
    }
    max_depth = unsafe { xmlPatternMaxDepth(comp) };
    if max_depth == -1 {
        return -1;
    }
    if max_depth == -2 {
        max_depth = 10000
    }
    min_depth = unsafe { xmlPatternMinDepth(comp) };
    if min_depth == -1 {
        return -1;
    }
    from_root = unsafe { xmlPatternFromRoot(comp) };
    if from_root < 0 {
        return -1;
    }
    if toBool == 0 {
        if resultSeq.is_null() {
            return -1;
        }
        unsafe {
            *resultSeq = xmlXPathCacheNewNodeSet(ctxt, 0 as xmlNodePtr);
            if (*resultSeq).is_null() {
                return -1;
            }
        }
    }
    /*
     * handle the special cases of "/" amd "." being matched
     */

    if min_depth == 0 as i32 {
        if from_root != 0 {
            /* Select "/" */
            if toBool != 0 {
                return 1;
            }
            /* TODO: Check memory error. */
            unsafe {
                xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, safe_ctxt.doc as xmlNodePtr)
            };
        } else {
            /* Select "self::node()" */
            if toBool != 0 {
                return 1;
            }
            /* TODO: Check memory error. */
            unsafe { xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, safe_ctxt.node) };
        }
    }
    if max_depth == 0 as i32 {
        return 0 as i32;
    }
    if from_root != 0 {
        cur = safe_ctxt.doc as xmlNodePtr
    } else if !safe_ctxt.node.is_null() {
        match unsafe { (*safe_ctxt.node).type_0 as u32 } {
            XML_ELEMENT_NODE
            | XML_DOCUMENT_NODE
            | XML_DOCUMENT_FRAG_NODE
            | XML_HTML_DOCUMENT_NODE
            | XML_DOCB_DOCUMENT_NODE => cur = safe_ctxt.node,
            XML_ATTRIBUTE_NODE
            | XML_TEXT_NODE
            | XML_CDATA_SECTION_NODE
            | XML_ENTITY_REF_NODE
            | XML_ENTITY_NODE
            | XML_PI_NODE
            | XML_COMMENT_NODE
            | XML_NOTATION_NODE
            | XML_DTD_NODE
            | XML_DOCUMENT_TYPE_NODE
            | XML_ELEMENT_DECL
            | XML_ATTRIBUTE_DECL
            | XML_ENTITY_DECL
            | XML_NAMESPACE_DECL
            | XML_XINCLUDE_START
            | XML_XINCLUDE_END
            | _ => {}
        }
        limit = cur
    }
    if cur.is_null() {
        return 0;
    }
    patstream = unsafe { xmlPatternGetStreamCtxt(comp) };
    if patstream.is_null() {
        /*
         * QUESTION TODO: Is this an error?
         */
        return 0;
    }
    eval_all_nodes = unsafe { xmlStreamWantsAnyNode(patstream) };

    if from_root != 0 {
        ret = unsafe { xmlStreamPush(patstream, 0 as *const xmlChar, 0 as *const xmlChar) };
        if ret < 0 {
            current_block = 7252614138838059896;
        } else if ret == 1 {
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
                depth = 0 as i32;
                'c_52613: loop {
                    if (*cur).type_0 as u32 == XML_NAMESPACE_DECL as u32 {
                        current_block = 16903048813113120619;
                        break;
                    }
                    if !(*cur).children.is_null() && depth < max_depth {
                        /*
                         * Do not descend on entities declarations
                         */
                        if (*(*cur).children).type_0 as u32 != XML_ENTITY_DECL as u32 {
                            cur = (*cur).children;
                            depth += 1;
                            /*
                             * Skip DTDs
                             */
                            if (*cur).type_0 as u32 != XML_DTD_NODE as u32 {
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
                                if (*cur).type_0 as u32 != XML_ENTITY_DECL as u32
                                    && (*cur).type_0 as u32 != XML_DTD_NODE as u32
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
                                            || (*cur).type_0 as u32 == XML_DOCUMENT_NODE as u32
                                        {
                                            current_block = 16903048813113120619;
                                            break 'c_52613;
                                        }
                                        if ((*cur).type_0 as u32 == XML_ELEMENT_NODE as u32)
                                            || (eval_all_nodes != 0
                                                && ((*cur).type_0 as u32 == XML_TEXT_NODE as u32
                                                    || (*cur).type_0 as u32
                                                        == XML_CDATA_SECTION_NODE as u32
                                                    || (*cur).type_0 as u32
                                                        == XML_COMMENT_NODE as u32
                                                    || (*cur).type_0 as u32 == XML_PI_NODE as u32))
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
                            if !(!cur.is_null() && depth >= 0 as i32) {
                                current_block = 16903048813113120619;
                                break;
                            }
                        }
                        _ => {}
                    }
                    's_317: loop {
                        if (*ctxt).opLimit != 0 as i32 as u64 {
                            if (*ctxt).opCount >= (*ctxt).opLimit {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"XPath operation limit exceeded\n\x00" as *const u8
                                        as *const i8,
                                );
                                xmlFreeStreamCtxt(patstream);
                                return -1;
                            }
                            (*ctxt).opCount = (*ctxt).opCount.wrapping_add(1)
                        }
                        nb_nodes += 1;
                        match (*cur).type_0 as u32 {
                            XML_ELEMENT_NODE
                            | XML_TEXT_NODE
                            | XML_CDATA_SECTION_NODE
                            | XML_COMMENT_NODE
                            | XML_PI_NODE => {}
                            _ => {
                                break;
                            }
                        }
                        if (*cur).type_0 as u32 == XML_ELEMENT_NODE as u32 {
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
                                (*cur).type_0 as i32,
                            )
                        }
                        if !(ret < 0 as i32) {
                            if ret == 1 {
                                if toBool != 0 {
                                    current_block = 13099311194446884672;
                                    break 'c_52613;
                                }
                                if xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, cur) < 0 {
                                    (*ctxt).lastError.domain = XML_FROM_XPATH as i32;
                                    (*ctxt).lastError.code = XML_ERR_NO_MEMORY as i32
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
                            if (*cur).type_0 as u32 != XML_ENTITY_DECL as u32
                                && (*cur).type_0 as u32 != XML_DTD_NODE as u32
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
                        return 0;
                    }
                }
            }
            _ => {}
        }
    }
    if !patstream.is_null() {
        unsafe { xmlFreeStreamCtxt(patstream) };
    }

    return 1;
}
/* XPATH_STREAMING */
/* *
 * xmlXPathRunEval: * @ctxt: the XPath parser context with the compiled expression
 * @toBool: evaluate to a boolean result
 *
 * Evaluate the Precompiled XPath expression in the given context.
 */

#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathRunEval(ctxt: xmlXPathParserContextPtr, toBool: i32) -> i32 {
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || safe_ctxt.comp.is_null() {
        return -1;
    }
    unsafe { (*safe_ctxt.context).depth = 0 };
    if safe_ctxt.valueTab.is_null() {
        /* Allocate the value stack */
        safe_ctxt.valueTab = unsafe {
            xmlMalloc.expect("non-null function pointer")(
                (10 as i32 as u64).wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>() as u64),
            ) as *mut xmlXPathObjectPtr
        };
        if safe_ctxt.valueTab.is_null() {
            unsafe {
                xmlXPathPErrMemory(
                    ctxt,
                    b"creating evaluation context\n\x00" as *const u8 as *const i8,
                )
            };
            unsafe { xmlFree.expect("non-null function pointer")(ctxt as *mut ()) };
        }
        safe_ctxt.valueNr = 0;
        safe_ctxt.valueMax = 10;
        safe_ctxt.value = 0 as xmlXPathObjectPtr;
        safe_ctxt.valueFrame = 0
    }
    // XPATH_STREAMING
    match () {
        #[cfg(XPATH_STREAMING)]
        _ => {
            if !unsafe { (*safe_ctxt.comp).stream.is_null() } {
                let mut res: i32 = 0;
                if toBool != 0 {
                    /*
                     * Evaluation to boolean result.
                     */
                    res = unsafe {
                        xmlXPathRunStreamEval(
                            safe_ctxt.context,
                            (*safe_ctxt.comp).stream,
                            0 as *mut xmlXPathObjectPtr,
                            1,
                        )
                    };
                    if res != -1 {
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
                            0,
                        )
                    };
                    if res != -1 && !resObj.is_null() {
                        unsafe { valuePush(ctxt, resObj) };
                        return 0;
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
    if safe_comp.last < 0 as i32 {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlXPathRunEval: last is less than zero\n\x00" as *const u8 as *const i8,
            )
        };
        return -1;
    }
    if toBool != 0 {
        return unsafe {
            xmlXPathCompOpEvalToBoolean(
                ctxt,
                &mut *safe_comp.steps.offset(safe_comp.last as isize),
                0,
            )
        };
    } else {
        unsafe { xmlXPathCompOpEval(ctxt, &mut *(*comp).steps.offset((*comp).last as isize)) };
    }
    return 0;
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
pub fn xmlXPathEvalPredicate(ctxt: xmlXPathContextPtr, res: xmlXPathObjectPtr) -> i32 {
    let safe_res = unsafe { &mut *res };
    let safe_ctxt = unsafe { &mut *ctxt };
    if ctxt.is_null() || res.is_null() {
        return 0;
    }
    match safe_res.type_0 as u32 {
        XPATH_BOOLEAN => return safe_res.boolval,
        XPATH_NUMBER => {
            return (safe_res.floatval == safe_ctxt.proximityPosition as libc::c_double) as i32
        }
        XPATH_NODESET | XPATH_XSLT_TREE => {
            if safe_res.nodesetval.is_null() {
                return 0;
            }
            return unsafe { ((*safe_res.nodesetval).nodeNr != 0 as i32) as i32 };
        }
        XPATH_STRING => {
            return (!safe_res.stringval.is_null()
                && unsafe { xmlStrlen(safe_res.stringval) != 0 as i32 }) as i32
        }
        _ => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Internal error at %s:%d\n\x00" as *const u8 as *const i8,
                    b"xpath.c\x00" as *const u8 as *const i8,
                    13998 as i32,
                )
            };
        }
    }
    return 0;
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
    ctxt: xmlXPathParserContextPtr,
    res: xmlXPathObjectPtr,
) -> i32 {
    let safe_res = unsafe { &mut *res };
    if ctxt.is_null() || res.is_null() {
        return 0 as i32;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    match safe_res.type_0 as u32 {
        XPATH_BOOLEAN => return safe_res.boolval,
        XPATH_NUMBER => {
            match () {
                #[cfg(BORLANDC_OR_MSC_VER_AND_MSC_VER)]
                _ => {
                    return (safe_res.floatval
                        == unsafe { (*safe_ctxt.context).proximityPosition as libc::c_double }
                        && xmlXPathIsNaN(safe_res.floatval) != 0) as i32
                }
                #[cfg(not(BORLANDC_OR_MSC_VER_AND_MSC_VER))]
                _ => {
                    return (safe_res.floatval
                        == unsafe { (*safe_ctxt.context).proximityPosition as libc::c_double })
                        as i32
                }
            };
        }
        XPATH_NODESET | XPATH_XSLT_TREE => {
            if safe_res.nodesetval.is_null() {
                return 0;
            }
            return unsafe { ((*safe_res.nodesetval).nodeNr != 0 as i32) as i32 };
        }
        XPATH_STRING => {
            return (!safe_res.stringval.is_null()
                && unsafe { *safe_res.stringval.offset(0) as i32 != 0 as i32 })
                as i32
        }
        XPATH_LOCATIONSET => {
            match () {
                #[cfg(XMLXPATHNODESETSORT)]
                _ => {
                    let mut ptr: xmlLocationSetPtr = safe_res.user as xmlLocationSetPtr;
                    let safe_ptr = unsafe { &mut *ptr };
                    if ptr.is_null() {
                        return 0 as i32;
                    }
                    return (safe_ptr.locNr != 0 as i32) as i32;
                }
                #[cfg(not(XMLXPATHNODESETSORT))]
                _ => {}
            };
        }
        _ => {
            unsafe {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Internal error at %s:%d\n\x00" as *const u8 as *const i8,
                    b"xpath.c\x00" as *const u8 as *const i8,
                    14049 as i32,
                )
            };
        }
    }
    return 0;
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
fn xmlXPathTryStreamCompile(ctxt: xmlXPathContextPtr, str: *const xmlChar) -> xmlXPathCompExprPtr {
    /*
     * Optimization: use streaming patterns when the XPath expression can
     * be compiled to a stream lookup
     */
    let mut stream: xmlPatternPtr;
    let mut comp: xmlXPathCompExprPtr;
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    let mut namespaces: *mut *const xmlChar = 0 as *mut *const xmlChar;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
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
                || safe_ctxt.nsNr == 0
                || unsafe { *tmp.offset(1) as i32 == ':' as i32 })
        {
            return 0 as xmlXPathCompExprPtr;
        }
        if !ctxt.is_null() {
            dict = safe_ctxt.dict;
            if safe_ctxt.nsNr > 0 {
                namespaces = unsafe {
                    xmlMalloc.expect("non-null function pointer")(
                        ((2 * (safe_ctxt.nsNr + 1)) as u64)
                            .wrapping_mul(::std::mem::size_of::<*mut xmlChar>() as u64),
                    ) as *mut *const xmlChar
                };
                if namespaces.is_null() {
                    unsafe {
                        xmlXPathErrMemory(
                            ctxt,
                            b"allocating namespaces array\n\x00" as *const u8 as *const i8,
                        )
                    };
                    return 0 as xmlXPathCompExprPtr;
                }

                let safe_namespaces = unsafe { &mut *namespaces };
                i = 0;
                j = 0;
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
        stream = unsafe { xmlPatterncompile(str, dict, XML_PATTERN_XPATH as i32, namespaces) };
        if !namespaces.is_null() {
            unsafe {
                xmlFree.expect("non-null function pointer")(
                    namespaces as *mut *mut xmlChar as *mut (),
                )
            };
        }
        if !stream.is_null() && unsafe { xmlPatternStreamable(stream) == 1 } {
            comp = unsafe { xmlXPathNewCompExpr() };
            if comp.is_null() {
                unsafe {
                    xmlXPathErrMemory(
                        ctxt,
                        b"allocating streamable expression\n\x00" as *const u8 as *const i8,
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
fn xmlXPathOptimizeExpression(pctxt: xmlXPathParserContextPtr, op: xmlXPathStepOpPtr) {
    let safe_pctxt = unsafe { &mut *pctxt };
    let mut comp: xmlXPathCompExprPtr = safe_pctxt.comp;
    let mut ctxt: xmlXPathContextPtr;
    /*
     * Try to rewrite "descendant-or-self::node()/foo" to an optimized
     * internal representation.
     */
    let safe_op = unsafe { &mut *op };
    let safe_comp = unsafe { &mut *comp };
    if safe_op.op as u32 == XPATH_OP_COLLECT as u32 && safe_op.ch1 != -1 && safe_op.ch2 == -1 {
        let mut prevop: xmlXPathStepOpPtr =
            unsafe { &mut *safe_comp.steps.offset(safe_op.ch1 as isize) as *mut xmlXPathStepOp };
        let safe_prevop = unsafe { &mut *prevop };
        if safe_prevop.op as u32 == XPATH_OP_COLLECT as u32
            && safe_prevop.value as xmlXPathAxisVal as u32 == AXIS_DESCENDANT_OR_SELF as u32
            && safe_prevop.ch2 == -1
            && safe_prevop.value2 as xmlXPathTestVal as u32 == NODE_TEST_TYPE as u32
            && safe_prevop.value3 as xmlXPathTypeVal as u32 == NODE_TYPE_NODE as u32
        {
            /*
             * This is a "descendant-or-self::node()" without predicates.
             * Try to eliminate it.
             */
            match safe_op.value as xmlXPathAxisVal as u32 {
                AXIS_CHILD | AXIS_DESCENDANT => {
                    /*
                     * Convert "descendant-or-self::node()/child::" or
                     * "descendant-or-self::node()/descendant::" to
                     * "descendant::"
                     */
                    safe_op.ch1 = safe_prevop.ch1;
                    safe_op.value = AXIS_DESCENDANT as i32
                }
                AXIS_SELF | AXIS_DESCENDANT_OR_SELF => {
                    /*
                     * Convert "descendant-or-self::node()/self::" or
                     * "descendant-or-self::node()/descendant-or-self::" to
                     * to "descendant-or-self::"
                     */
                    safe_op.ch1 = safe_prevop.ch1;
                    safe_op.value = AXIS_DESCENDANT_OR_SELF as i32
                }
                _ => {}
            }
        }
    }
    /* OP_VALUE has invalid ch1. */
    if safe_op.op as u32 == XPATH_OP_VALUE as u32 {
        return;
    }
    /* Recurse */
    ctxt = safe_pctxt.context;
    let safe_ctxt = unsafe { &mut *ctxt };
    if !ctxt.is_null() {
        if safe_ctxt.depth >= 5000 as i32 {
            return;
        }
        safe_ctxt.depth += 1
    }
    if safe_op.ch1 != -1 {
        unsafe {
            xmlXPathOptimizeExpression(pctxt, &mut *safe_comp.steps.offset(safe_op.ch1 as isize))
        };
    }
    if safe_op.ch2 != -1 {
        unsafe {
            xmlXPathOptimizeExpression(pctxt, &mut *safe_comp.steps.offset(safe_op.ch2 as isize))
        };
    }
    if !ctxt.is_null() {
        safe_ctxt.depth -= 1
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
pub fn xmlXPathCtxtCompile(ctxt: xmlXPathContextPtr, str: *const xmlChar) -> xmlXPathCompExprPtr {
    let mut pctxt: xmlXPathParserContextPtr;
    let mut comp: xmlXPathCompExprPtr;

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
        safe_ctxt.depth = 0 as i32
    }
    unsafe { xmlXPathCompileExpr(pctxt, 1) };
    if safe_pctxt.error != XPATH_EXPRESSION_OK as i32 {
        unsafe { xmlXPathFreeParserContext(pctxt) };
        return 0 as xmlXPathCompExprPtr;
    }
    if unsafe { *safe_pctxt.cur as i32 != 0 as i32 } {
        /*
         * aleksey: in some cases this line prints *second* error message
         * (see bug #78858) and probably this should be fixed.
         * However, we are not sure that all error messages are printed
         * out in other places. It's not critical so we leave it as-is for now
         */
        unsafe {
            xmlXPatherror(
                pctxt,
                b"xpath.c\x00" as *const u8 as *const i8,
                14254 as i32,
                XPATH_EXPR_ERROR as i32,
            )
        };
        comp = 0 as xmlXPathCompExprPtr
    } else {
        comp = safe_pctxt.comp;
        let safe_comp = unsafe { &mut *comp };
        if safe_comp.nbStep > 1 && safe_comp.last >= 0 {
            if !ctxt.is_null() {
                safe_ctxt.depth = 0
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
                safe_comp.nb = 0;
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
    return xmlXPathCtxtCompile(0 as xmlXPathContextPtr, str);
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
// static mut reentance: i32 = 0;

#[cfg(LIBXML_XPATH_ENABLED)]
fn xmlXPathCompiledEvalInternal(
    comp: xmlXPathCompExprPtr,
    ctxt: xmlXPathContextPtr,
    resObjPtr: *mut xmlXPathObjectPtr,
    toBool: i32,
) -> i32 {
    let mut pctxt: xmlXPathParserContextPtr;
    let mut resObj: xmlXPathObjectPtr;
    let mut reentance: i32;
    match () {
        #[cfg(LIBXML_THREAD_ENABLED)]
        _ => {}
        #[cfg(not(LIBXML_THREAD_ENABLED))]
        _ => {
            // 条件宏内的局部静态变量在rust中好像只局限在#cfg内部
            reentance = 0 as i32;
        }
    }
    let mut res: i32 = 0;
    if ctxt.is_null() {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut (),
                0 as *mut (),
                0 as *mut (),
                XML_FROM_XPATH as i32,
                XML_ERR_INTERNAL_ERROR as i32,
                XML_ERR_FATAL,
                b"xpath.c\x00" as *const u8 as *const i8,
                14317 as i32,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as i32,
                0 as i32,
                b"NULL context pointer\n\x00" as *const u8 as *const i8,
            )
        };
        return -1;
    }
    if comp.is_null() {
        return -1;
    }
    unsafe { xmlInitParser() };

    match () {
        #[cfg(not(LIBXML_THREAD_ENABLED))]
        _ => unsafe {
            reentance += 1;
            if reentance > 1 {
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
            if (!(*comp).string.is_null() && (*comp).nb > 100) {
                fprintf(
                    stderr,
                    b"100 x %s\n\x00" as *const u8 as *const i8,
                    (*comp).string,
                );
                (*comp).nb = 0;
            }
        }
        #[cfg(not(DEBUG_EVAL_COUNTS))]
        _ => {}
    }

    pctxt = unsafe { xmlXPathCompParserContext(comp, ctxt) };
    let safe_pctxt = unsafe { &mut *pctxt };
    res = unsafe { xmlXPathRunEval(pctxt, toBool) };
    if safe_pctxt.error != XPATH_EXPRESSION_OK as i32 {
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
                            as *const i8,
                    )
                };
            }
        } else if safe_pctxt.valueNr > 0 {
            unsafe {
                safe_xmlGenericError.expect("non-null function pointer")(
                    safe_xmlGenericErrorContext,
                    b"xmlXPathCompiledEval: %d object(s) left on the stack.\n\x00" as *const u8
                        as *const i8,
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
    comp: xmlXPathCompExprPtr,
    ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let mut res: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    unsafe { xmlXPathCompiledEvalInternal(comp, ctx, &mut res, 0) };
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
pub fn xmlXPathCompiledEvalToBoolean(comp: xmlXPathCompExprPtr, ctxt: xmlXPathContextPtr) -> i32 {
    return unsafe { xmlXPathCompiledEvalInternal(comp, ctxt, 0 as *mut xmlXPathObjectPtr, 1) };
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
                    unsafe { (*(*ctxt).context).depth = 0 }
                }

                xmlXPathCompileExpr(ctxt, 1);

                if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                    return;
                }
                /* Check for trailing characters. */
                if unsafe { *safe_ctxt.cur } as i32 != 0 {
                    unsafe { xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32) };
                    return;
                }
                if unsafe { (*safe_ctxt.comp).nbStep > 1 && (*safe_ctxt.comp).last >= 0 } {
                    if !safe_ctxt.context.is_null() {
                        unsafe { (*safe_ctxt.context).depth = 0 }
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
                unsafe { (*safe_ctxt.context).depth = 0 }
            }
            xmlXPathCompileExpr(ctxt, 1);
            if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
                return;
            }
            /* Check for trailing characters. */
            if unsafe { *safe_ctxt.cur as i32 != 0 } {
                xmlXPathErr(ctxt, XPATH_EXPR_ERROR as i32);
                return;
            }
            if unsafe { (*safe_ctxt.comp).nbStep > 1 && (*safe_ctxt.comp).last >= 0 as i32 } {
                if !safe_ctxt.context.is_null() {
                    unsafe { (*safe_ctxt.context).depth = 0 }
                }
                xmlXPathOptimizeExpression(ctxt, unsafe {
                    &mut *(*safe_ctxt.comp)
                        .steps
                        .offset((*safe_ctxt.comp).last as isize)
                });
            }
        }
    };
    xmlXPathRunEval(ctxt, 0);
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
pub fn xmlXPathEval(str: *const xmlChar, ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr {
    let mut ctxt: xmlXPathParserContextPtr;
    let mut res: xmlXPathObjectPtr;
    if ctx.is_null() {
        unsafe {
            __xmlRaiseError(
                None,
                None,
                0 as *mut (),
                0 as *mut (),
                0 as *mut (),
                XML_FROM_XPATH as i32,
                XML_ERR_INTERNAL_ERROR as i32,
                XML_ERR_FATAL,
                b"xpath.c\x00" as *const u8 as *const i8,
                14464 as i32,
                0 as *const i8,
                0 as *const i8,
                0 as *const i8,
                0 as i32,
                0 as i32,
                b"NULL context pointer\n\x00" as *const u8 as *const i8,
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
    if safe_ctxt.error != XPATH_EXPRESSION_OK as i32 {
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
                        as *const i8,
                )
            };
        } else if safe_ctxt.valueNr > 0 as i32 {
            unsafe {
                safe_xmlGenericError.expect("non-null function pointer")(
                    safe_xmlGenericErrorContext,
                    b"xmlXPathCompiledEval: %d object(s) left on the stack.\n\x00" as *const u8
                        as *const i8,
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
pub fn xmlXPathSetContextNode(node: xmlNodePtr, ctx: xmlXPathContextPtr) -> i32 {
    if node.is_null() || ctx.is_null() {
        return -1;
    }
    let safe_node = unsafe { &mut *node };
    let safe_ctx = unsafe { &mut *ctx };
    if safe_node.doc == safe_ctx.doc {
        safe_ctx.node = node;
        return 0;
    }
    return -1;
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
    node: xmlNodePtr,
    str: *const xmlChar,
    ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    if str.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    if xmlXPathSetContextNode(node, ctx) < 0 {
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
pub fn xmlXPathEvalExpression(str: *const xmlChar, ctxt: xmlXPathContextPtr) -> xmlXPathObjectPtr {
    return xmlXPathEval(str, ctxt);
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
extern "C" fn xmlXPathEscapeUriFunction(ctxt: xmlXPathParserContextPtr, nargs: i32) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut escape_reserved: i32 = 0;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut cptr: *mut xmlChar = 0 as *mut xmlChar;
    let mut escape: [xmlChar; 4] = [0; 4];
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as i32 {
        unsafe { xmlXPathErr(ctxt, XPATH_INVALID_ARITY as i32) };
        return;
    }
    let safe_ctxt = unsafe { &mut *ctxt };
    if safe_ctxt.valueNr < safe_ctxt.valueFrame + 2 {
        unsafe { xmlXPathErr(ctxt, XPATH_STACK_ERROR as i32) };
        return;
    }
    escape_reserved = unsafe { xmlXPathPopBoolean(ctxt) };

    let safe_ctxt2 = unsafe { &mut *safe_ctxt };
    let safe_ctxt_value = unsafe { &mut *(safe_ctxt2.value) };
    if !safe_ctxt.value.is_null() && safe_ctxt_value.type_0 as u32 != XPATH_STRING as u32 {
        unsafe { xmlXPathStringFunction(ctxt, 1) };
    }
    str = unsafe { valuePop(ctxt) };

    let safe_str = unsafe { &mut *str };
    target = unsafe { xmlBufCreate() };
    escape[0] = '%' as i32 as xmlChar;
    escape[3] = 0 as i32 as xmlChar;
    if !target.is_null() {
        cptr = safe_str.stringval;

        let safe_cptr = unsafe { *cptr } as i32;
        let safe_cptr_xmlChar = unsafe { *cptr } as *mut xmlChar;
        while 1 < 2 {
            if safe_cptr == 0 {
                break;
            }
            if safe_cptr as i32 >= 'A' as i32 && safe_cptr as i32 <= 'Z' as i32
                || safe_cptr as i32 >= 'a' as i32 && safe_cptr as i32 <= 'z' as i32
                || safe_cptr as i32 >= '0' as i32 && safe_cptr as i32 <= '9' as i32
                || safe_cptr as i32 == '-' as i32
                || safe_cptr as i32 == '_' as i32
                || safe_cptr as i32 == '.' as i32
                || safe_cptr as i32 == '!' as i32
                || safe_cptr as i32 == '~' as i32
                || safe_cptr as i32 == '*' as i32
                || safe_cptr as i32 == '\'' as i32
                || safe_cptr as i32 == '(' as i32
                || safe_cptr as i32 == ')' as i32
                || safe_cptr as i32 == '%' as i32
                    && (unsafe { safe_cptr_xmlChar.offset(1) } as i32 >= 'A' as i32
                        && unsafe { safe_cptr_xmlChar.offset(1) } as i32 <= 'F' as i32
                        || unsafe { safe_cptr_xmlChar.offset(1) } as i32 >= 'a' as i32
                            && unsafe { safe_cptr_xmlChar.offset(1) } as i32 <= 'f' as i32
                        || unsafe { safe_cptr_xmlChar.offset(1) } as i32 >= '0' as i32
                            && unsafe { safe_cptr_xmlChar.offset(1) } as i32 <= '9' as i32)
                    && (unsafe { safe_cptr_xmlChar.offset(2) } as i32 >= 'A' as i32
                        && unsafe { safe_cptr_xmlChar.offset(2) } as i32 <= 'F' as i32
                        || unsafe { safe_cptr_xmlChar.offset(2) } as i32 >= 'a' as i32
                            && unsafe { safe_cptr_xmlChar.offset(2) } as i32 <= 'f' as i32
                        || unsafe { safe_cptr_xmlChar.offset(2) } as i32 >= '0' as i32
                            && unsafe { safe_cptr_xmlChar.offset(2) } as i32 <= '9' as i32)
                || escape_reserved == 0
                    && (safe_cptr as i32 == ';' as i32
                        || safe_cptr as i32 == '/' as i32
                        || safe_cptr as i32 == '?' as i32
                        || safe_cptr as i32 == ':' as i32
                        || safe_cptr as i32 == '@' as i32
                        || safe_cptr as i32 == '&' as i32
                        || safe_cptr as i32 == '=' as i32
                        || safe_cptr as i32 == '+' as i32
                        || safe_cptr as i32 == '$' as i32
                        || safe_cptr as i32 == ',' as i32)
            {
                unsafe { xmlBufAdd(target, cptr, 1) };
            } else {
                if (safe_cptr as i32 >> 4) < 10 {
                    escape[1 as usize] = ('0' as i32 + (safe_cptr as i32 >> 4 as i32)) as xmlChar
                } else {
                    escape[1 as usize] =
                        ('A' as i32 - 10 as i32 + (safe_cptr as i32 >> 4 as i32)) as xmlChar
                }
                if (safe_cptr as i32 & 0xf as i32) < 10 as i32 {
                    escape[2 as i32 as usize] =
                        ('0' as i32 + (safe_cptr as i32 & 0xf as i32)) as xmlChar
                } else {
                    escape[2 as i32 as usize] =
                        ('A' as i32 - 10 as i32 + (safe_cptr as i32 & 0xf as i32)) as xmlChar
                }
                unsafe { xmlBufAdd(target, &mut *escape.as_mut_ptr().offset(0), 3 as i32) };
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
            b"boolean\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathBooleanFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"ceiling\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathCeilingFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"count\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathCountFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"concat\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathConcatFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"contains\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathContainsFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"id\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathIdFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"false\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathFalseFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"floor\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathFloorFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"last\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathLastFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"lang\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathLangFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"local-name\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathLocalNameFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"not\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathNotFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"name\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathNameFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"namespace-uri\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathNamespaceURIFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"normalize-space\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathNormalizeFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"number\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathNumberFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"position\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathPositionFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"round\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathRoundFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"string\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathStringFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"string-length\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathStringLengthFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"starts-with\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathStartsWithFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"substring\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathSubstringFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"substring-before\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathSubstringBeforeFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"substring-after\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathSubstringAfterFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"sum\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathSumFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"true\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathTrueFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFunc(
            ctxt,
            b"translate\x00" as *const u8 as *const i8 as *const xmlChar,
            Some(
                xmlXPathTranslateFunction
                    as unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
    unsafe {
        xmlXPathRegisterFuncNS(
            ctxt,
            b"escape-uri\x00" as *const u8 as *const i8 as *const xmlChar,
            b"http://www.w3.org/2002/08/xquery-functions\x00" as *const u8 as *const i8
                as *const xmlChar,
            Some(
                xmlXPathEscapeUriFunction
                    as extern "C" fn(_: xmlXPathParserContextPtr, _: i32) -> (),
            ),
        )
    };
}
/* LIBXML_XPATH_ENABLED */
