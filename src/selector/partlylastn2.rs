// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::selector::{ PartlyLastN, PartlyLastN2 };
use crate::selector:: { Selector, SelectReadIterator, SelectWriteIterator, SelectorResult };

impl PartlyLastN2 {
    pub fn new_taillen(
        delta: Vec<usize>, taillen: usize
    ) -> Self {
        let frontlen = delta.len() - taillen;
        let mut front = 1_usize;
        let mut tail  = 0_usize;
        for v in delta.iter().take(frontlen) { front += *v; }
        for v in delta.iter().skip(frontlen) { tail += *v; }

        PartlyLastN2 {
            data: PartlyLastN {
                delta,
                process_size: front + tail
            },
            front_len: front,
            tail_len:  tail
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.delta.len() + 1
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.delta.is_empty()
    }
}

impl Selector for PartlyLastN2
{
    #[inline(always)]
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
        self.data.populate(read, write)
    }

    #[inline(always)]
    fn process_len(&self) -> usize {
        self.data.process_size
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.data.delta.len() + 1
    }

    #[inline(always)]
    fn is_empty(&self) -> bool
    {
        self.data.delta.is_empty()
    }
}


