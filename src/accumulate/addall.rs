// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::*;
use crate::accumulate::*;

impl<ITEMOUT> Default for AddAll<ITEMOUT> {
    fn default() -> Self {
        AddAll { marker: Default::default() }
    }
}

impl<ITEMOUT> AddAll<ITEMOUT>
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
            for re in ri {
                let add: ITEMOUT = conv.convert_ref(re);
                accumulated += add;
            }

            Some(accumulated)
        }
        else {
            None
        }
    }
}

impl<ITEMOUT> SequenceAccumulate<ITEMOUT> for AddAll<ITEMOUT>
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

