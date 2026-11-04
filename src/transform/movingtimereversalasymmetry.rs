// SPDX-License-Identifier: Apache-2.0
//! Normalized odd third moment of adjacent increments in trailing windows.
use crate::transform::*;

impl<T> MovingTimeReversalAsymmetry<T> {
    pub fn new(count: usize) -> Self {
        Self { marker: Default::default(), count }
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingTimeReversalAsymmetry<ITEMOUT>
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
        if len != out.len() || self.count < 2 || self.count > len {
            return TransformResult::Error(0);
        }
        let values: Vec<f64> = ri.map(|sample| {
            let value = conv.convert_ref(sample);
            value.as_()
        }).collect();
        if values.iter().any(|value| !value.is_finite()) {
            return TransformResult::Error(1);
        }
        // Subtract before scaling, then use max-absolute normalization per
        // window to avoid cubing large increments or underflowing tiny ones.
        let differences: Vec<f64> = values.windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        if differences.iter().any(|value| !value.is_finite()) {
            return TransformResult::Error(1);
        }
        out.fill(default_out);
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            let increments = &differences[end + 1 - self.count..end];
            let max_abs = increments.iter().map(|d| d.abs()).fold(0.0_f64, f64::max);
            if max_abs == 0.0 {
                continue;
            }
            let mut sum2 = 0.0;
            let mut sum3 = 0.0;
            for &d in increments {
                let scaled = d / max_abs;
                sum2 += scaled * scaled;
                sum3 += scaled * scaled * scaled;
            }
            let score = (sum3 / (sum2 * sum2.sqrt())).clamp(-1.0, 1.0);
            if !score.is_finite() {
                return TransformResult::Error(2);
            }
            let Some(value) = ITEMOUT::from_f64(score) else {
                return TransformResult::Error(2);
            };
            *output = value;
        }
        TransformResult::Ok
    }
}
