// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::iter::Iterator;
use crate::iterop::*;
use crate::set::*;

impl<'a, T: 'a, U> RetainSortedLeft<T, U>
where
    T: Clone,
    U: PartialOrd<U> + PartialEq<U> + Clone
{

    #[inline(always)]
    pub fn new(
        select: fn(&T) -> &U
    ) -> Self {
        RetainSortedLeft { select }
    }

    pub fn exe<J>(&self, data: &'a mut [T], iter2: J) -> usize
    where
        U: 'a,
        J: Iterator<Item = &'a T> + Clone
    {
        let init_len = data.len();

        let mut dataptr = data.as_mut_ptr();
        let beginptr = dataptr;

        let mut itneq = Iter2Neq::new(data.iter(), iter2.clone(), self.select);

        // find first not equal, skip equal
        if let (Some(neqobj1), Some(neqobj2)) = itneq.next2() {
            let write_pos = init_len - itneq.iter1.len() - 1;
            unsafe {
                dataptr = dataptr.add(write_pos);
            }
            let mut iteq = Iter2Eq::new(itneq.iter1, itneq.iter2, self.select);
            if let (Some(eqobj1), Some(_)) = iteq.step(neqobj1, neqobj2) {
                unsafe {
                    *dataptr = (*eqobj1).clone();
                    dataptr = dataptr.add(1);
                }
            }
            while let (Some(eqobj1), Some(_)) = iteq.next2() {
                unsafe {
                    *dataptr = (*eqobj1).clone();
                    dataptr = dataptr.add(1);
                }
            }
        }
        else {
            let mut iteq = Iter2Eq::new(data.iter(), iter2, self.select);
            while let (Some(eqobj1), Some(_)) = iteq.next2() {
                unsafe {
                    *dataptr = (*eqobj1).clone();
                    dataptr = dataptr.add(1);
                }
            }
        }

        unsafe { dataptr.offset_from(beginptr) as usize }
    }
}
