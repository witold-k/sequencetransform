// SPDX-License-Identifier: Apache-2.0
//! Signed angle between the two highest and two lowest points of a trailing window.
use crate::transform::*;

impl<T> MovingExtremaConvergence<T> {
    pub fn new(count: usize) -> Self { Self { marker: Default::default(), count } }
}

fn angle(window: &[f64]) -> Option<f64> {
    let n = window.len();
    if n < 4 || window.iter().any(|x| !x.is_finite()) { return None; }
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_unstable_by(|&i, &j| window[i].total_cmp(&window[j]).then(i.cmp(&j)));
    let low = (indices[0], indices[1]);
    let high = (indices[n-2], indices[n-1]);
    // A point cannot belong to both lines.
    if low.0 == high.0 || low.0 == high.1 || low.1 == high.0 || low.1 == high.1 {
        return None;
    }
    let min = window[indices[0]];
    let max = window[indices[n-1]];
    // Scaling before subtraction avoids overflow for extreme finite inputs.
    let scale = min.abs().max(max.abs());
    if scale == 0.0 { return None; }
    let scaled: Vec<f64> = window.iter().map(|&x| x / scale).collect();
    let range = scaled[indices[n-1]] - scaled[indices[0]];
    if range <= 0.0 || !range.is_finite() { return None; }
    let slope = |(i,j):(usize,usize)| {
        let dx = (j as f64 - i as f64) / (n-1) as f64;
        ((scaled[j] - scaled[i]) / range) / dx
    };
    let upper = slope(high);
    let lower = slope(low);
    if !upper.is_finite() || !lower.is_finite() { return None; }
    // atan2(cross,dot) computes the angle without subtracting close atan values.
    let unsigned = (lower - upper).abs().atan2(1.0 + lower * upper);
    // Evaluate separation at the current time. The sign follows its time derivative:
    // shrinking => positive (convergence), widening => negative (divergence).
    let extrapolate = |(i,j):(usize,usize), m:f64| {
        let y = (scaled[i] - scaled[indices[0]]) / range;
        y + m * (1.0 - i as f64 / (n-1) as f64)
    };
    let gap = extrapolate(high, upper) - extrapolate(low, lower);
    let change = upper - lower;
    if !gap.is_finite() { return None; }
    let signed = if gap == 0.0 || change == 0.0 { 0.0 }
        else if gap * change < 0.0 { unsigned }
        else { -unsigned };
    Some(signed / std::f64::consts::PI)
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingExtremaConvergence<ITEMOUT>
where ITEMOUT: 'static + Number<ITEMOUT> {
    fn transform<'a, RI, ITEMIN, CTO>(
        &self, ri: RI, out: &mut [ITEMOUT], default_out: ITEMOUT, conv: CTO
    ) -> TransformResult
    where RI: TransformReadIterator<Item = &'a ITEMIN>,
          ITEMIN: 'a + Number<ITEMIN>,
          CTO: ConvertTo<ITEMIN, ITEMOUT> {
        let len = ri.len();
        if len != out.len() || self.count < 4 || self.count > len {
            return TransformResult::Error(0);
        }
        let values: Vec<f64> = ri.map(|x| { let v = conv.convert_ref(x); v.as_() }).collect();
        if values.iter().any(|v| !v.is_finite()) { return TransformResult::Error(1); }
        out.fill(default_out);
        for (end, output) in out.iter_mut().enumerate().skip(self.count-1) {
            if let Some(value) = angle(&values[end+1-self.count..=end]) {
                let Some(converted) = ITEMOUT::from_f64(value) else { return TransformResult::Error(2); };
                *output = converted;
            }
        }
        TransformResult::Ok
    }
}
