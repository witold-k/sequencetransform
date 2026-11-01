// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::collections::VecDeque;
use crate::transform::*;

impl<ITEMOUT> MaxDirection<ITEMOUT> {
    pub fn new(count: usize) -> Self {
        Self { marker: Default::default(), count }
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MaxDirection<ITEMOUT>
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
        if self.count == 0 || ri.len() != out.len() {
            return TransformResult::Error(0);
        }

        let values: Vec<ITEMOUT> = ri.map(|value| conv.convert_ref(value)).collect();
        // Monotonic deques store indices; newer equal extrema replace older ones.
        let mut minima = VecDeque::<usize>::new();
        let mut maxima = VecDeque::<usize>::new();
        let zero = ITEMOUT::from_u32(0).unwrap();

        for i in 0..values.len() {
            let start = i.saturating_sub(self.count);
            while minima.front().is_some_and(|&j| j < start) {
                minima.pop_front();
            }
            while maxima.front().is_some_and(|&j| j < start) {
                maxima.pop_front();
            }

            out[i] = if let (Some(&min_index), Some(&max_index)) =
                (minima.front(), maxima.front())
            {
                let nearest = min_index.max(max_index);
                let span = ITEMOUT::from_usize(i - nearest).unwrap();
                (values[i] - values[nearest]) / span
            } else {
                zero
            };

            while minima.back().is_some_and(|&j| values[j] >= values[i]) {
                minima.pop_back();
            }
            minima.push_back(i);
            while maxima.back().is_some_and(|&j| values[j] <= values[i]) {
                maxima.pop_back();
            }
            maxima.push_back(i);
        }

        TransformResult::Ok
    }
}
