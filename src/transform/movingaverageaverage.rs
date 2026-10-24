// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::*;
use crate::transform::*;

/// Two trailing moving averages evaluated in one pass.
///
/// Equivalent (up to floating-point rounding) to
/// MovingAverage(first_count) followed by MovingAverage(second_count).
/// The initial windows in both stages use the available prefix.
impl<ITEMOUT> MovingAverageAverage<ITEMOUT> {
    pub fn new(first_count: usize, second_count: usize) -> Self {
        Self { marker: Default::default(), first_count, second_count }
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingAverageAverage<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>,
{
    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri: RI,
        out: &mut [ITEMOUT],
        _default_out: ITEMOUT,
        conv: CTO,
    ) -> TransformResult
    where
        RI: TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO: ConvertTo<ITEMIN, ITEMOUT>,
    {
        let len = ri.len();
        if out.len() != len
            || self.first_count == 0
            || self.second_count == 0
            || self.first_count > len
            || self.second_count > len
        {
            return TransformResult::Error(0);
        }

        // Two rolling sums and two ring buffers; no intermediate output array.
        let zero: ITEMOUT = FromPrimitive::from_u32(0).unwrap();
        let mut first_ring = vec![zero; self.first_count];
        let mut second_ring = vec![zero; self.second_count];
        let mut first_sum = zero;
        let mut second_sum = zero;

        for (i, (input, output)) in ri.zip(out.iter_mut()).enumerate() {
            let first_slot = i % self.first_count;
            if i >= self.first_count {
                first_sum -= first_ring[first_slot];
            }
            let value = conv.convert_ref(input);
            first_ring[first_slot] = value;
            first_sum += value;

            let first_len = (i + 1).min(self.first_count);
            let first_average = if i + 1 < self.first_count {
                first_sum / FromPrimitive::from_usize(first_len).unwrap()
            } else {
                let reciprocal = 1.0 / self.first_count as f64;
                first_sum * FromPrimitive::from_f64(reciprocal).unwrap()
            };

            let second_slot = i % self.second_count;
            if i >= self.second_count {
                second_sum -= second_ring[second_slot];
            }
            second_ring[second_slot] = first_average;
            second_sum += first_average;

            let second_len = (i + 1).min(self.second_count);
            *output = if i + 1 < self.second_count {
                second_sum / FromPrimitive::from_usize(second_len).unwrap()
            } else {
                let reciprocal = 1.0 / self.second_count as f64;
                second_sum * FromPrimitive::from_f64(reciprocal).unwrap()
            };
        }
        TransformResult::Ok
    }
}
