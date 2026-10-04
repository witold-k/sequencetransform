// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::iter::Iterator;
use crate::iterop::IterMatch;

// ---------------------------------------------------------------------------

impl<'a, ITER, T: 'a> IterMatch<ITER, T>
where
    T: 'a,
    ITER: Iterator<Item = &'a T>
{
    #[inline(always)]
    pub fn new(iter: ITER, select: fn(&T) -> bool) -> Self {
        IterMatch::<ITER, T> { iter, select }
    }

    #[inline(always)]
    pub fn step(&mut self, mut obj: &'a T) -> Option<&'a T>
    {
        loop {
            if !(self.select)(obj) {
                obj = self.iter.next()?;
            }
            else {
                // match: just return
                return Some(obj);
            }
        }
    }
}

/**
 * iterate over all equal elements, ignore not equal elements
 */
impl<'a, ITER, T: 'a> Iterator for IterMatch<ITER, T>
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
                if !(self.select)(obj) {
                    obj = self.iter.next()?
                }
                else {
                    // match: just return
                    return Some(obj);
                }
            }
        }
        else {
            None
        }
    }

}
