// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::*;
use crate::select::*;

impl<ITEMOUT> Default for Max<ITEMOUT> {
    fn default() -> Self {
        Max { marker: Default::default() }
    }
}

impl<ITEMOUT> Max<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn select_inline<'a, 'b, RI, ITEMIN, CTO>(
        &mut self,
        mut ri: RI,
        conv:   CTO,
    ) -> ItemRefIndex<ITEMOUT>
    where
        'a: 'b,
        RI:     Iterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        let mut count: usize = 1;
        let mut index: usize = 0;
        if let Some(re) = ri.next() {
            let mut selected: ITEMOUT = conv.convert_ref(re);
            for re in ri {
                let val: ITEMOUT = conv.convert_ref(re);
                if selected < val {
                    selected = val;
                    index = count;
                }
                count += 1;
            }
            ItemRefIndex::<ITEMOUT> { item: selected, index }
        }
        else {
            ItemRefIndex::<ITEMOUT> { item: FromPrimitive::from_u32(0).unwrap(), index: 0usize }
        }
    }
}

impl<ITEMOUT> ValueSelect<ITEMOUT> for Max<ITEMOUT>
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
        let mut mut_self = *self;
        mut_self.select_inline(ri, conv)
    }
}

