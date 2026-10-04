// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::transform::*;
use crate::select::*;

impl<ITEMOUT> MovingMedian<ITEMOUT> {
    pub fn new(count: usize) -> Self {
        MovingMedian { marker: Default::default(), count, border: count / 2 }
    }

    pub fn new_border(count: usize, border: usize) -> Self {
        MovingMedian { marker: Default::default(), count, border }
    }
}

impl<ITEMOUT> MovingMedian<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        ri:          RI,
        out:         &mut [ITEMOUT],
        default_out: ITEMOUT,
        conv:        CTO
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

        let count = self.count;
        let mut wi = out.iter_mut();

        for (i, we) in (1 .. count).zip(wi.by_ref()) {
            let median = Median::<ITEMOUT>::new_len(i, i / 2, default_out);
            let fi  = ri.clone().take(i);
            *we = median.select(fi, conv.clone()).item;
        }

        let median = Median::<ITEMOUT>::new_len(self.count, self.border, default_out);
        for (i, we) in (0 ..).zip(wi) {
            let fi  = ri.clone().skip(i).take(count);
            *we = median.select(fi, conv.clone()).item;
        }

        TransformResult::Ok
    }
}


impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingMedian<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri: RI,
        out:         &mut [ITEMOUT],
        default_out: ITEMOUT,
        conv:        CTO
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri, out, default_out, conv)
    }
}

