// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::selector::PartlyLastN;
use crate::selector:: { Selector, SelectReadIterator, SelectWriteIterator, SelectorResult };

impl PartlyLastN {
    pub fn new(delta: Vec<usize>) -> Self {
        let mut sum = 1_usize;
        for v in delta.iter() { sum += *v; }

        PartlyLastN { delta, process_size: sum }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.delta.len() + 1
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        0 == self.delta.len()
    }
}

impl Selector for PartlyLastN
{
    fn populate<'a, 'b, RI, WI, ITEM>(
        &self,
        mut read:  RI,
        mut write: WI
    ) -> SelectorResult
    where
        'a: 'b,
        RI: SelectReadIterator<'a, ITEM>,
        WI: SelectWriteIterator<'b, ITEM>,
        ITEM: 'a + Clone
    {
        if read.len() < self.process_size {
            return SelectorResult::Error((self.process_size - read.len()) as u32);
        }

        let re = match read.next() {
            Some(re) => re,
            None     => return SelectorResult::Ok
        };
        let we = match write.next() {
            Some(we) => we,
            None     => return SelectorResult::Ok
        };
        *we = (*re).clone();
        for (we, de) in write.zip(self.delta.iter()) {
            let delta = *de;
            if delta > 1 { read.nth(delta - 2); } // should read.advance_by later on
            let re = match read.next() {
                Some(re) => re,
                None     => break
            };
            *we = (*re).clone();
        }

        SelectorResult::Ok
    }

    #[inline(always)]
    fn process_len(&self) -> usize {
        self.process_size
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.delta.len() + 1
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.delta.is_empty()
    }
}


