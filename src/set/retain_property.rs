// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::iterop::*;
use crate::set::*;

impl<T> RetainProperty<T>
where
    T: Clone
{

    #[inline(always)]
    pub fn new(
        select: fn(&T) -> bool
    ) -> Self {
        RetainProperty { select }
    }

    pub fn exe(&self, data: &mut [T]) -> usize
    {
        let init_len = data.len();

        let mut dataptr = data.as_mut_ptr();
        let beginptr = dataptr;

        let mut itneq = IterNMatch::new(data.iter(), self.select);

        // find first not equal, skip equal
        if let Some(neqobj) = itneq.next() {
            let write_pos = init_len - itneq.iter.len() - 1;
            unsafe {
                dataptr = dataptr.add(write_pos);
            }
            let mut iteq = IterMatch::new(itneq.iter, self.select);
            if let Some(eqobj) = iteq.step(neqobj) {
                unsafe {
                    *dataptr = (*eqobj).clone();
                    dataptr = dataptr.add(1);
                }
            }
            for eqobj1 in iteq {
                unsafe {
                    *dataptr = (*eqobj1).clone();
                    dataptr = dataptr.add(1);
                }
            }
        }
        else {
            let iteq = IterMatch::new(data.iter(), self.select);
            for eqobj1 in iteq {
                unsafe {
                    *dataptr = (*eqobj1).clone();
                    dataptr = dataptr.add(1);
                }
            }
        }

        unsafe { dataptr.offset_from(beginptr) as usize }
    }
}
