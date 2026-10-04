// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::iter::Iterator;
use crate::iterop::Iter2MutEq;

// ---------------------------------------------------------------------------

/**
 * iterate over all equal elements, ignore not equal elements
 */
impl<'a, ITER1, ITER2, T: 'a, U> Iter2MutEq<ITER1, ITER2, T, U>
where
    ITER1: Iterator<Item = &'a mut T>,
    ITER2: Iterator<Item = &'a mut T>,
    U: PartialOrd<U> + PartialEq<U> + 'a
{
    #[inline(always)]
    pub fn new(iter1: ITER1, iter2: ITER2, select: fn(&mut T) -> &mut U) -> Self {
        Iter2MutEq::<ITER1, ITER2, T, U> { select, iter1, iter2 }
    }

    #[inline(always)]
    pub fn next2(&mut self) -> (Option<&'a mut T>, Option<&'a mut T>) {
        if let (Some(mut obj1), Some(mut obj2)) = (self.iter1.next(), self.iter2.next()) {
            let mut cobj1 = (self.select)(obj1); // &U
            let mut cobj2 = (self.select)(obj2); // &U

            loop {
                if *cobj1 < *cobj2 {
                    obj1 = match self.iter1.next() {
                        Some(obj1) => obj1,
                        None       => return (None, None)
                    };
                    cobj1 = (self.select)(obj1);
                }
                else if *cobj2 < *cobj1 {
                    obj2 = match self.iter2.next() {
                        Some(obj2) => obj2,
                        None       => return (None, None)
                    };
                    cobj2 = (self.select)(obj2);
                }
                else {
                    // they are equal by value, not by reference: just return
                    return (Some(obj1), Some(obj2));
                }
            }
        }
        else {
            (None, None)
        }
    }

}

