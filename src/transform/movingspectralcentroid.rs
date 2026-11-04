// SPDX-License-Identifier: Apache-2.0
//! Trailing spectral centroid using a shared direct DFT.
use crate::transform::*;
use super::movingspectral_core::{feature, Feature};

impl<T> MovingSpectralCentroid<T> {
    pub fn new(count: usize) -> Self {
        Self { marker: Default::default(), count }
    }
}
impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingSpectralCentroid<ITEMOUT>
where ITEMOUT: 'static + Number<ITEMOUT> {
    fn transform<'a, RI, ITEMIN, CTO>(
        &self, ri: RI, out: &mut [ITEMOUT], default_out: ITEMOUT, conv: CTO
    ) -> TransformResult
    where
        RI: TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO: ConvertTo<ITEMIN, ITEMOUT>,
    {
        let len = ri.len();
        if len != out.len() || self.count < 4 || self.count > len {
            return TransformResult::Error(0);
        }
        let values: Vec<f64> = ri.map(|v| {
            let converted = conv.convert_ref(v);
            converted.as_()
        }).collect();
        if values.iter().any(|v| !v.is_finite()) {
            return TransformResult::Error(1);
        }
        out.fill(default_out);
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            let window = &values[end + 1 - self.count..end + 1];
            if let Some(score) = feature(window, Feature::Centroid) {
                let Some(value) = ITEMOUT::from_f64(score) else {
                    return TransformResult::Error(2);
                };
                *output = value;
            }
        }
        TransformResult::Ok
    }
}
