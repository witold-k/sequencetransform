// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

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
        let Some(zero) = ITEMOUT::from_u32(0) else { return TransformResult::Error(2); };
        let Some(first_divisor) = ITEMOUT::from_usize(self.first_count) else { return TransformResult::Error(2); };
        let Some(second_divisor) = ITEMOUT::from_usize(self.second_count) else { return TransformResult::Error(2); };
        if first_divisor == zero || second_divisor == zero { return TransformResult::Error(2); }
        let mut first_ring = vec![zero; self.first_count];
        let mut second_ring = vec![zero; self.second_count];
        let mut first_sum = zero;
        let mut second_sum = zero;
        let mut first_compensation = zero;
        let mut second_compensation = zero;

        for (i, (input, output)) in ri.zip(out.iter_mut()).enumerate() {
            let first_slot = i % self.first_count;
            if i >= self.first_count {
                let delta = zero - first_ring[first_slot];
                let next = first_sum + delta;
                first_compensation += if first_sum.abs() >= delta.abs() {
                    (first_sum - next) + delta
                } else {
                    (delta - next) + first_sum
                };
                first_sum = next;
            }
            let value = conv.convert_ref(input);
            first_ring[first_slot] = value;
            let next = first_sum + value;
            first_compensation += if first_sum.abs() >= value.abs() {
                (first_sum - next) + value
            } else {
                (value - next) + first_sum
            };
            first_sum = next;

            let first_len = (i + 1).min(self.first_count);
            let Some(first_len_value) = ITEMOUT::from_usize(first_len) else { return TransformResult::Error(2); };
            if first_len_value == zero { return TransformResult::Error(2); }
            let first_average = if i + 1 < self.first_count {
                (first_sum + first_compensation) / first_len_value
            } else {
                (first_sum + first_compensation) / first_divisor
            };

            let second_slot = i % self.second_count;
            if i >= self.second_count {
                let delta = zero - second_ring[second_slot];
                let next = second_sum + delta;
                second_compensation += if second_sum.abs() >= delta.abs() {
                    (second_sum - next) + delta
                } else {
                    (delta - next) + second_sum
                };
                second_sum = next;
            }
            second_ring[second_slot] = first_average;
            let next = second_sum + first_average;
            second_compensation += if second_sum.abs() >= first_average.abs() {
                (second_sum - next) + first_average
            } else {
                (first_average - next) + second_sum
            };
            second_sum = next;

            let second_len = (i + 1).min(self.second_count);
            let Some(second_len_value) = ITEMOUT::from_usize(second_len) else { return TransformResult::Error(2); };
            if second_len_value == zero { return TransformResult::Error(2); }
            *output = if i + 1 < self.second_count {
                (second_sum + second_compensation) / second_len_value
            } else {
                (second_sum + second_compensation) / second_divisor
            };
        }
        TransformResult::Ok
    }
}
