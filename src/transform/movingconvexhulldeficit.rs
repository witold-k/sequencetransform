// SPDX-License-Identifier: Apache-2.0
//! Causal trailing-window MovingConvexHullDeficit estimator.
use crate::transform::*;
use super::movinggeometry_core::{feature, Feature};
impl<T> MovingConvexHullDeficit<T> {
    pub fn new(count: usize) -> Self { Self { marker: Default::default(), count } }
}
impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingConvexHullDeficit<ITEMOUT>
where ITEMOUT: 'static + Number<ITEMOUT> {
    fn transform<'a, RI, ITEMIN, CTO>(
        &self, ri: RI, out: &mut [ITEMOUT], default_out: ITEMOUT, conv: CTO
    ) -> TransformResult
    where RI: TransformReadIterator<Item = &'a ITEMIN>,
          ITEMIN: 'a + Number<ITEMIN>,
          CTO: ConvertTo<ITEMIN, ITEMOUT> {
        let len = ri.len();
        if len != out.len() || self.count < 3 || self.count > len || false {
            return TransformResult::Error(0);
        }
        let values: Vec<f64> = ri.map(|x| { let v = conv.convert_ref(x); v.as_() }).collect();
        if values.iter().any(|v| !v.is_finite()) { return TransformResult::Error(1); }
        out.fill(default_out);
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            if let Some(score) = feature(&values[end + 1 - self.count..end + 1], Feature::HullDeficit) {
                let Some(v) = ITEMOUT::from_f64(score) else { return TransformResult::Error(2); };
                *output = v;
            }
        }
        TransformResult::Ok
    }
}
