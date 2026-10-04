// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::selector::LastN;
use crate::selector:: { Selector, SelectReadIterator, SelectWriteIterator, SelectorResult };

impl LastN {
    #[inline(always)]
    pub fn new(count: usize) -> Self {
        LastN { count }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.count
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl Selector for LastN
{
    fn populate<'a, 'b, RI, WI, ITEM>(
        &self,
        read:  RI,
        write: WI
    ) -> SelectorResult
    where
        'a: 'b,
        RI: SelectReadIterator<'a, ITEM>,
        WI: SelectWriteIterator<'b, ITEM>,
        ITEM: 'a + Clone
    {
        if read.len() < self.count {
            return SelectorResult::Error((self.count - read.len()) as u32);
        }

        let ri = read.take(self.count);
        for (we, re) in write.zip(ri) {
            *we = (*re).clone();
        }

        SelectorResult::Ok
    }

    #[inline(always)]
    fn process_len(&self) -> usize {
        self.count
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.count
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.count == 0
    }
}

