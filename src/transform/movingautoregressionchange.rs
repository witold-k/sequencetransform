// SPDX-License-Identifier: Apache-2.0
//! Absolute difference between AR(1) OLS slopes on two adjacent half-windows.
use crate::transform::*;

impl<T> MovingAutoregressionChange<T> {
    pub fn new(count: usize) -> Self {
        Self { marker: Default::default(), count }
    }
}

fn slope(values: &[f64]) -> Option<f64> {
    let n = values.len() - 1;
    let x0 = values[0];
    let y0 = values[1];
    let mut sx = 0.0;
    let mut sy = 0.0;
    for i in 0..n {
        sx += values[i] - x0;
        sy += values[i + 1] - y0;
    }
    let mx = sx / n as f64;
    let my = sy / n as f64;
    let mut variance = 0.0;
    let mut covariance = 0.0;
    for i in 0..n {
        let x = (values[i] - x0) - mx;
        let y = (values[i + 1] - y0) - my;
        variance += x * x;
        covariance += x * y;
    }
    if variance <= 0.0 || !variance.is_finite() || !covariance.is_finite() {
        return None;
    }
    let result = covariance / variance;
    result.is_finite().then_some(result)
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingAutoregressionChange<ITEMOUT>
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
        // Two disjoint halves, each with at least three samples.
        if len != out.len() || self.count < 6 || !self.count.is_multiple_of(2) || self.count > len {
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
        let half = self.count / 2;
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            let start = end + 1 - self.count;
            if let (Some(old), Some(new)) = (
                slope(&values[start..start + half]),
                slope(&values[start + half..end + 1]),
            ) {
                let difference = (new - old).abs();
                let Some(value) = ITEMOUT::from_f64(difference) else {
                    return TransformResult::Error(2);
                };
                *output = value;
            }
        }
        TransformResult::Ok
    }
}
