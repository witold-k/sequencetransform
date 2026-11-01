// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::*;
use crate::transform::*;

impl<T> MovingMedianAverage2<T> {
    pub fn new_border(
        median_count: usize,
        average_count: usize,
        first_border: usize,
        second_border: usize,
    ) -> Self {
        Self {
            marker: Default::default(),
            median_count,
            average_count,
            first_border,
            second_border,
        }
    }

    pub fn new(median_count: usize, average_count: usize) -> Self {
        Self::new_border(
            median_count,
            average_count,
            median_count.saturating_sub(1) / 2,
            median_count / 2,
        )
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingMedianAverage2<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>,
{
    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri: RI,
        out: &mut [ITEMOUT],
        default_out: ITEMOUT,
        conv: CTO,
    ) -> TransformResult
    where
        RI: TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO: ConvertTo<ITEMIN, ITEMOUT>,
    {
        let len = ri.len();
        if out.len() != len
            || self.average_count == 0
            || self.average_count > len
        {
            return TransformResult::Error(0);
        }
        let median = MovingMedian2::<ITEMOUT>::new_border(
            self.median_count,
            self.first_border,
            self.second_border,
        );
        let mut intermediate = vec![default_out; len];
        let result = median.transform(ri, &mut intermediate, default_out, conv);
        if result != TransformResult::Ok {
            return result;
        }
        MovingAverage::<ITEMOUT>::new(self.average_count).transform(
            intermediate.iter(),
            out,
            default_out,
            ConvertToIdentity::<ITEMOUT>::default(),
        )
    }
}
