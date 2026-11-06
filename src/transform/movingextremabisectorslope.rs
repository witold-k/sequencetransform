// SPDX-License-Identifier: Apache-2.0
//! Slope of the right-facing angular bisector of lines through window extrema.
use crate::transform::*;

impl<T> MovingExtremaBisectorSlope<T> {
    pub fn new(count: usize) -> Self { Self { marker: Default::default(), count } }
}

fn bisector_slope(window: &[f64]) -> Option<f64> {
    let n = window.len();
    if n < 4 || window.iter().any(|x| !x.is_finite()) { return None; }

    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_unstable_by(|&i, &j| window[i].total_cmp(&window[j]).then(i.cmp(&j)));
    let (lo0, lo1) = (indices[0], indices[1]);
    let (hi0, hi1) = (indices[n-2], indices[n-1]);

    let scale = window[indices[0]].abs().max(window[indices[n-1]].abs());
    if scale == 0.0 { return None; }
    // Scale before subtracting to avoid overflow in the range.
    let values: Vec<f64> = window.iter().map(|&v| v / scale).collect();
    let range = values[indices[n-1]] - values[indices[0]];
    if !range.is_finite() || range <= 0.0 { return None; }

    let slope = |a: usize, b: usize| {
        let dx = (b as f64 - a as f64) / (n-1) as f64;
        ((values[b] - values[a]) / range) / dx
    };
    let upper = slope(hi0, hi1);
    let lower = slope(lo0, lo1);
    if !upper.is_finite() || !lower.is_finite() { return None; }

    // Average the angles, not the slopes. atan(m) is the direction of
    // the right-facing ray (dx > 0), so this selects the desired bisector.
    let half_angle = 0.5 * (upper.atan() + lower.atan());
    let result = half_angle.tan();
    result.is_finite().then_some(result)
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingExtremaBisectorSlope<ITEMOUT>
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
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            if let Some(slope) = bisector_slope(&values[end + 1 - self.count..=end]) {
                let Some(converted) = ITEMOUT::from_f64(slope) else {
                    return TransformResult::Error(2);
                };
                *output = converted;
            }
        }
        TransformResult::Ok
    }
}
