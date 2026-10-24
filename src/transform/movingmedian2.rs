// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::*;
use crate::transform::*;

impl<T> MovingMedian2<T> {
    /// Select two zero-based ranks from each sorted trailing window and average them.
    pub fn new_border(count: usize, first_border: usize, second_border: usize) -> Self {
        Self { marker: Default::default(), count, first_border, second_border }
    }

    /// Select the two central ranks (the same rank twice for odd windows).
    pub fn new(count: usize) -> Self {
        Self::new_border(count, (count.saturating_sub(1)) / 2, count / 2)
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingMedian2<ITEMOUT>
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
        if len != out.len()
            || self.count == 0
            || self.count > len
            || self.first_border >= self.count
            || self.second_border >= self.count
        {
            return TransformResult::Error(0);
        }

        let two = FromPrimitive::from_u32(2).unwrap();
        let mut window = Vec::<ITEMOUT>::with_capacity(self.count);
        for (i, output) in out.iter_mut().enumerate() {
            let window_len = (i + 1).min(self.count);
            let start = i + 1 - window_len;
            window.clear();
            window.extend(
                ri.clone().skip(start).take(window_len).map(|x| conv.convert_ref(x)),
            );
            if window.iter().any(|x| x.partial_cmp(x).is_none()) {
                return TransformResult::Error(1);
            }
            window.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let first = window[self.first_border.min(window_len - 1)];
            let second = window[self.second_border.min(window_len - 1)];
            *output = (first + second) / two;
        }
        TransformResult::Ok
    }
}
