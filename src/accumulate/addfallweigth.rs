// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::*;
use crate::accumulate::*;

impl<ITEMOUT> Default for AddFallWeigth<ITEMOUT> {
    fn default() -> Self {
        AddFallWeigth { marker: Default::default() }
    }
}

impl<ITEMOUT> AddFallWeigth<ITEMOUT>
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
            let mut weigth = 1;
            let mut accumulated: ITEMOUT = FromPrimitive::from_u32(0u32).unwrap();
            for re in ri {
                if *re < t1 {
                    let ro: ITEMOUT = conv.convert_ref(re);
                    let to: ITEMOUT = conv.convert(t1);
                    accumulated += (ro - to) * FromPrimitive::from_u32(weigth).unwrap();
                    weigth += 1;
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

impl<ITEMOUT> SequenceAccumulate<ITEMOUT> for AddFallWeigth<ITEMOUT>
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

