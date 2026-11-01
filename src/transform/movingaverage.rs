// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

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
        if len != out.len() || self.count == 0 || self.count > len {
            return TransformResult::Error(0);
        }

        let aa = AddAll::<ITEMOUT>::default();

        let count = self.count;
        let mut wi = out.iter_mut();
        // Divide by the actual window length. Converting a fractional reciprocal
        // to an integer output type would truncate it to zero.
        let Some(full_divisor) = ITEMOUT::from_usize(count) else {
            return TransformResult::Error(2);
        };
        if full_divisor == ITEMOUT::from_u8(0).unwrap() {
            return TransformResult::Error(2);
        }

        for (i, we) in (1 .. count).zip(wi.by_ref()) {
            let fi  = ri.clone().take(i);
            let Some(divisor) = ITEMOUT::from_usize(i) else { return TransformResult::Error(2); };
            if divisor == ITEMOUT::from_u8(0).unwrap() { return TransformResult::Error(2); }
            let Some(sum) = aa.accumulate(fi, conv.clone()) else { return TransformResult::Error(2); };
            *we = sum / divisor;
        }
        for (i, we) in wi.enumerate() {
            let fi  = ri.clone().skip(i).take(count);
            let Some(sum) = aa.accumulate(fi, conv.clone()) else { return TransformResult::Error(2); };
            *we = sum / full_divisor;
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

