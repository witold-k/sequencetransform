// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::iter::Iterator;
use crate::iterop::IterNMatch;

// ---------------------------------------------------------------------------

impl<'a, ITER, T: 'a> IterNMatch<ITER, T>
where
    T: 'a,
    ITER: Iterator<Item = &'a T>
{
    #[inline(always)]
    pub fn new(iter: ITER, select: fn(&T) -> bool) -> Self {
        IterNMatch::<ITER, T> { iter, select }
    }
}

/**
 * iterate over all equal elements, ignore not equal elements
 */
impl<'a, ITER, T: 'a> Iterator for IterNMatch<ITER, T>
where
    T: 'a,
    ITER: Iterator<Item = &'a T>
{
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item>
    {
        if let Some(mut obj) = self.iter.next() {
            loop {
                if (self.select)(obj) {
                    obj = self.iter.next()?;
                }
                else {
                    // they are equal by value, not by reference: just return
                    return Some(obj);
                }
            }
        }
        else {
            None
        }
    }
}

