// SPDX-License-Identifier: Apache-2.0
//! Trailing AR(1) ordinary-least-squares coefficient with an intercept.
//! For each full window x[0..count], regress x[1..count] on x[0..count-1].
use crate::transform::*;

impl<T> MovingAutoregression<T> {
    pub fn new(count: usize) -> Self {
        Self { marker: Default::default(), count }
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingAutoregression<ITEMOUT>
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
        if len != out.len() || self.count < 3 || self.count > len {
            return TransformResult::Error(0);
        }
        out.fill(default_out);
        // The conversion is applied once per input sample, not per window.
        let values: Vec<f64> = ri.map(|value| {
            let converted = conv.convert_ref(value);
            converted.as_()
        }).collect();
        if values.iter().any(|value| !value.is_finite()) {
            return TransformResult::Error(1);
        }
        let n = self.count - 1;
        for end in self.count - 1..len {
            let start = end + 1 - self.count;
            // Shift both series before summation to avoid cancellation on
            // large common offsets. Means are estimated separately.
            let x0 = values[start];
            let y0 = values[start + 1];
            let mut sx = 0.0;
            let mut sy = 0.0;
            for i in 0..n {
                sx += values[start + i] - x0;
                sy += values[start + i + 1] - y0;
            }
            let mx = sx / n as f64;
            let my = sy / n as f64;
            let mut variance = 0.0;
            let mut covariance = 0.0;
            for i in 0..n {
                let x = values[start + i] - x0 - mx;
                let y = values[start + i + 1] - y0 - my;
                variance += x * x;
                covariance += x * y;
            }
            if variance > 0.0 && variance.is_finite() && covariance.is_finite() {
                let coefficient = covariance / variance;
                if coefficient.is_finite() {
                    let Some(value) = ITEMOUT::from_f64(coefficient) else {
                        return TransformResult::Error(2);
                    };
                    out[end] = value;
                }
            }
            // Undefined (constant predictor) -> default_out.
        }
        TransformResult::Ok
    }
}
