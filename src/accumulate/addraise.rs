// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::*;
use crate::accumulate::*;

impl<ITEMOUT> Default for AddRaise<ITEMOUT> {
    fn default() -> Self {
        AddRaise { marker: Default::default() }
    }
}

impl<ITEMOUT> AddRaise<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn accumulate_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        mut ri: RI,
        conv:   CTO
    ) -> Option<ITEMOUT>
    where
        'a: 'b,
        RI:     Iterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        if let Some(t0) = ri.next() {
            let mut t1 = *t0;
            let mut accumulated: ITEMOUT = FromPrimitive::from_u32(0u32).unwrap();
            for re in ri {
                if *re > t1 {
                    let rv: ITEMOUT = conv.convert_ref(re);
                    accumulated += rv - conv.convert(t1);
                }
                t1 = *re;
            }

            Some(accumulated)
        }
        else {
            None
        }
    }
}

impl<ITEMOUT> SequenceAccumulate<ITEMOUT> for AddRaise<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{
    fn accumulate<'a, RI, ITEMIN, CTO>(
        &self,
        ri:   RI,
        conv: CTO
    ) -> Option<ITEMOUT>
    where
        RI:     Iterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.accumulate_inline(ri, conv)
    }
}

