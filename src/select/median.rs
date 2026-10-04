// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::*;
use crate::select::*;

impl<ITEMOUT> Median<ITEMOUT>
where
    ITEMOUT: Clone
{
    //pub fn new() -> Self {
    //    Median { marker: Default::default(), buffer: Vec::<OrdEntry::<ITEMOUT>>::new() }
    //}

    pub fn new_len(count: usize, border: usize, default_item: ITEMOUT) -> Self {
        let mut buffer = Vec::<OrdEntry::<ITEMOUT>>::with_capacity(count);
        let default_entry = OrdEntry::<ITEMOUT> { value: default_item };
        buffer.resize(count, default_entry);
        Median { marker: Default::default(), border, buffer }
    }
}

impl<ITEMOUT> Median<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn select_inline<'a, 'b, RI, ITEMIN, CTO>(
        &mut self,
        ri:   RI,
        conv: CTO,
    ) -> ItemRefIndex<ITEMOUT>
    where
        'a: 'b,
        RI:     Iterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        for (buf, re) in self.buffer.iter_mut().zip(ri) {
            *buf = OrdEntry::<ITEMOUT> { value: conv.convert_ref(re) };
        }
        self.buffer.sort();
        let item: ITEMOUT = self.buffer[self.border].value;
        ItemRefIndex::<ITEMOUT> { item, index: self.border }
    }
}

impl<ITEMOUT> ValueSelect<ITEMOUT> for Median<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn select<'a, RI, ITEMIN, CTO>(
        &self,
        ri:   RI,
        conv: CTO,
    ) -> ItemRefIndex<ITEMOUT>
    where
        RI:     Iterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        let mut mut_self = self.clone();
        mut_self.select_inline(ri, conv)
    }
}

