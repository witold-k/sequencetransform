// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::*;
use crate::accumulate::*;

impl<ITEMOUT> Default for AddAllInc<ITEMOUT> {
    fn default() -> Self {
        AddAllInc { marker: Default::default() }
    }
}

impl<ITEMOUT> AddAllInc<ITEMOUT>
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
        if let Some(re) = ri.next() {
            let mut accumulated: ITEMOUT = conv.convert_ref(re);
            for (re, fac) in ri.zip(2 .. ) {
                let add1: ITEMOUT = conv.convert_ref(re);
                let add2: ITEMOUT = FromPrimitive::from_i32(fac).unwrap();
                accumulated += add1 * add2;
            }

            Some(accumulated)
        }
        else {
            None
        }
    }
}

impl<ITEMOUT> SequenceAccumulate<ITEMOUT> for AddAllInc<ITEMOUT>
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

