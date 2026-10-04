// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::transform::*;
use crate::accumulate::*;

impl<ITEMOUT> MovingAverage<ITEMOUT> {
    pub fn new(count: usize) -> Self {
        MovingAverage { marker: Default::default(), count }
    }
}

impl<ITEMOUT> MovingAverage<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        ri:   RI,
        out:  &mut [ITEMOUT],
        conv: CTO
    ) -> TransformResult
    where
        'a: 'b,
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        let len = ri.len();
        if len != out.len() {
            return TransformResult::Error(0);
        }

        let aa = AddAll::<ITEMOUT>::default();

        let count = self.count;
        let mut wi = out.iter_mut();
        let rez_count: f64 = 1. / (count as f64);

        for (i, we) in (1 .. count).zip(wi.by_ref()) {
            let fi  = ri.clone().take(i);
            *we = aa.accumulate(fi, conv.clone()).unwrap() / FromPrimitive::from_usize(i).unwrap();
        }
        for (i, we) in wi.enumerate() {
            let fi  = ri.clone().skip(i).take(count);
            *we = aa.accumulate(fi, conv.clone()).unwrap() * FromPrimitive::from_f64(rez_count).unwrap() ;
        }

        TransformResult::Ok
    }

    pub fn transform_self<'a, RI, ITEMIN, CTO>(
        &self,
        ri:   RI,
        out:  &mut [ITEMOUT],
        conv: CTO
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri, out, conv)
    }

}


impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingAverage<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri:           RI,
        out:          &mut [ITEMOUT],
        _default_out: ITEMOUT,
        conv:         CTO
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri, out, conv)
    }
}

