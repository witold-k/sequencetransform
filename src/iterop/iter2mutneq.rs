// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::iter::Iterator;
use crate::iterop::Iter2MutNeq;

// ---------------------------------------------------------------------------

/**
 * iterate over all not equal elements, ignore equal elements
 */
impl<'a, ITER1, ITER2, T: 'a, U> Iter2MutNeq<ITER1, ITER2, T, U>
where
    ITER1: Iterator<Item = &'a mut T>,
    ITER2: Iterator<Item = &'a mut T>,
    U: PartialOrd<U> + PartialEq<U> + 'a
{
    #[inline(always)]
    pub fn new(iter1: ITER1, iter2: ITER2, select: fn(&mut T) -> &mut U) -> Self {
        Iter2MutNeq::<ITER1, ITER2, T, U> { select, iter1, iter2 }
    }

    #[inline(always)]
    pub fn next2(&mut self) -> (Option<&'a mut T>, Option<&'a mut T>)
    where
        U: 'a
    {
        while let (Some(obj1), Some(obj2)) = (self.iter1.next(), self.iter2.next()) {
            let neq = {
                let cobj1 = (self.select)(obj1); // &U
                let cobj2 = (self.select)(obj2); // &U
                *cobj1 != *cobj2
            };
            if neq {
                return (Some(obj1), Some(obj2));
            }
        }
        (None, None)
    }

}

