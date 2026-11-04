// SPDX-License-Identifier: Apache-2.0
//! Laplace-smoothed conditional ordinal-pattern surprise, trained on past transitions.
use crate::transform::*;

impl<T> MovingOrdinalSurprise<T> {
    pub fn new(count: usize, order: usize) -> Self {
        Self { marker: Default::default(), count, order }
    }
}
impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingOrdinalSurprise<ITEMOUT>
where ITEMOUT: 'static + Number<ITEMOUT> {
    fn transform<'a, RI, ITEMIN, CTO>(
        &self, ri: RI, out: &mut [ITEMOUT], default_out: ITEMOUT, conv: CTO
    ) -> TransformResult
    where RI: TransformReadIterator<Item = &'a ITEMIN>,
          ITEMIN: 'a + Number<ITEMIN>, CTO: ConvertTo<ITEMIN, ITEMOUT> {
        let len = ri.len();
        if len != out.len() || !(2..=5).contains(&self.order)
            || self.count < self.order + 2 || self.count > len {
            return TransformResult::Error(0);
        }
        let values: Vec<f64> = ri.map(|x| { let v = conv.convert_ref(x); v.as_() }).collect();
        if values.iter().any(|v| !v.is_finite()) { return TransformResult::Error(1); }
        let states: usize = (2..=self.order).product();
        let mut ids = Vec::with_capacity(len - self.order + 1);
        let mut indices: Vec<usize> = (0..self.order).collect();
        for start in 0..=len - self.order {
            indices.sort_unstable_by(|&a, &b| {
                values[start + a].total_cmp(&values[start + b]).then(a.cmp(&b))
            });
            let mut id = 0;
            for i in 0..self.order {
                let smaller = indices[i + 1..].iter().filter(|&&v| v < indices[i]).count();
                id = id * (self.order - i) + smaller;
            }
            ids.push(id);
        }
        out.fill(default_out);
        // The observed last transition is excluded from its own training data.
        // The window contains count-order+1 patterns. The first transition
        // starts at the first pattern; the final transition is held out.
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            let first = end + 1 - self.count;
            let last = end + 1 - self.order;
            let previous = ids[last - 1];
            let current = ids[last];
            let mut total = 0usize;
            let mut hits = 0usize;
            for t in first + 1..last {
                if ids[t - 1] == previous {
                    total += 1;
                    if ids[t] == current { hits += 1; }
                }
            }
            // Add-one smoothing over all order! possible successor patterns.
            let probability = (hits + 1) as f64 / (total + states) as f64;
            let surprise = -probability.ln() / (states as f64).ln();
            let Some(v) = ITEMOUT::from_f64(surprise) else {
                return TransformResult::Error(2);
            };
            *output = v;
        }
        TransformResult::Ok
    }
}
