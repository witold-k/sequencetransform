// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::*;
use crate::select::*;
use crate::transform::*;

impl<ITEMOUT> MovingMedianAverage<ITEMOUT> {
    /// Trailing moving median followed by a trailing moving average.
    /// Both stages use the available prefix for their initial windows.
    pub fn new(median_count: usize, average_count: usize) -> Self {
        Self::new_border(median_count, average_count, median_count / 2)
    }

    pub fn new_border(median_count: usize, average_count: usize, border: usize) -> Self {
        Self { marker: Default::default(), median_count, average_count, border }
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingMedianAverage<ITEMOUT>
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
            || self.median_count == 0
            || self.average_count == 0
            || self.median_count > len
            || self.average_count > len
            || self.border >= self.median_count
        {
            return TransformResult::Error(0);
        }

        // Only the last average_count medians are needed. The median stage
        // retains MovingMedian's prefix-window and border semantics.
        let mut ring = vec![default_out; self.average_count];
        let mut sum: ITEMOUT = FromPrimitive::from_u32(0).unwrap();
        for (i, output) in out.iter_mut().enumerate() {
            let window_len = (i + 1).min(self.median_count);
            let border = if self.border == self.median_count / 2 && window_len < self.median_count {
                window_len / 2
            } else {
                self.border.min(window_len - 1)
            };
            let median = Median::<ITEMOUT>::new_len(window_len, border, default_out);
            let start = i + 1 - window_len;
            let value = median.select(ri.clone().skip(start).take(window_len), conv.clone()).item;

            let slot = i % self.average_count;
            if i >= self.average_count {
                sum -= ring[slot];
            }
            ring[slot] = value;
            sum += value;

            let divisor = (i + 1).min(self.average_count);
            *output = sum / FromPrimitive::from_usize(divisor).unwrap();
        }
        TransformResult::Ok
    }
}
