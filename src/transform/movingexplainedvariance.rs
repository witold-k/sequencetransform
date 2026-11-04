// SPDX-License-Identifier: Apache-2.0
//! Trailing coefficient of determination for AR(1) with an intercept.
use crate::transform::*;

impl<T> MovingExplainedVariance<T> {
    pub fn new(count: usize) -> Self {
        Self { marker: Default::default(), count }
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingExplainedVariance<ITEMOUT>
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
        if len != out.len() || self.count < 3 || self.count > len {
            return TransformResult::Error(0);
        }
        let values: Vec<f64> = ri.map(|sample| {
            let value = conv.convert_ref(sample);
            value.as_()
        }).collect();
        if values.iter().any(|value| !value.is_finite()) {
            return TransformResult::Error(1);
        }
        out.fill(default_out);
        let n = self.count - 1;
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            let start = end + 1 - self.count;
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
            let mut vx = 0.0;
            let mut vy = 0.0;
            let mut cov = 0.0;
            for i in 0..n {
                let x = (values[start + i] - x0) - mx;
                let y = (values[start + i + 1] - y0) - my;
                vx += x * x;
                vy += y * y;
                cov += x * y;
            }
            if vx > 0.0 && vy > 0.0 && vx.is_finite() && vy.is_finite() && cov.is_finite() {
                let correlation = (cov / vx.sqrt()) / vy.sqrt();
                let score = (correlation * correlation).clamp(0.0, 1.0);
                if score.is_finite() {
                    let Some(value) = ITEMOUT::from_f64(score) else {
                        return TransformResult::Error(2);
                    };
                    *output = value;
                }
            }
        }
        TransformResult::Ok
    }
}
