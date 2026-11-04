// SPDX-License-Identifier: Apache-2.0
//! Normalized Shannon entropy of ordinal patterns in trailing windows.
use crate::transform::*;

impl<T> MovingPermutationEntropy<T> {
    pub fn new(count: usize, order: usize) -> Self {
        Self { marker: Default::default(), count, order }
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingPermutationEntropy<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>,
{
    fn transform<'a, RI, ITEMIN, CTO>(
        &self, ri: RI, out: &mut [ITEMOUT], default_out: ITEMOUT, conv: CTO
    ) -> TransformResult
    where
        RI: TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO: ConvertTo<ITEMIN, ITEMOUT>,
    {
        let len = ri.len();
        // Cap factorial state space to avoid excessive allocations.
        if len != out.len() || self.order < 2 || self.order > 7
            || self.count < self.order || self.count > len {
            return TransformResult::Error(0);
        }
        let values: Vec<f64> = ri.map(|sample| {
            let value = conv.convert_ref(sample);
            value.as_()
        }).collect();
        if values.iter().any(|v| !v.is_finite()) {
            return TransformResult::Error(1);
        }
        let factorial = (2..=self.order).product::<usize>();
        let mut histogram = vec![0usize; factorial];
        let mut pattern_ids = Vec::with_capacity(len - self.order + 1);
        let mut indices: Vec<usize> = (0..self.order).collect();
        for start in 0..=len - self.order {
            indices.sort_unstable_by(|&a, &b| {
                values[start + a].total_cmp(&values[start + b]).then(a.cmp(&b))
            });
            // Lehmer rank of the permutation, in [0, order!).
            let mut id = 0;
            for i in 0..self.order {
                let smaller = indices[i + 1..].iter()
                    .filter(|&&v| v < indices[i]).count();
                id = id * (self.order - i) + smaller;
            }
            pattern_ids.push(id);
        }
        out.fill(default_out);
        let patterns_per_window = self.count - self.order + 1;
        for &id in &pattern_ids[..patterns_per_window] {
            histogram[id] += 1;
        }
        let norm = (factorial as f64).ln();
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            if end > self.count - 1 {
                histogram[pattern_ids[end - self.count]] -= 1;
                histogram[pattern_ids[end - self.order + 1]] += 1;
            }
            let n = patterns_per_window as f64;
            let entropy: f64 = histogram.iter().filter(|&&c| c > 0).map(|&c| {
                let p = c as f64 / n;
                -p * p.ln()
            }).sum();
            let score = (entropy / norm).clamp(0.0, 1.0);
            let Some(value) = ITEMOUT::from_f64(score) else {
                return TransformResult::Error(2);
            };
            *output = value;
        }
        TransformResult::Ok
    }
}
