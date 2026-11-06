// SPDX-License-Identifier: Apache-2.0
//! Position at which half the area above the trailing window minimum is reached.
use crate::transform::*;

impl<T> MovingAreaMedian<T> {
    pub fn new(count: usize) -> Self {
        Self { marker: Default::default(), count }
    }
}

#[derive(Default)]
struct Sum { sum: f64, correction: f64 }
impl Sum {
    fn add(&mut self, value: f64) {
        let next = self.sum + value;
        self.correction += if self.sum.abs() >= value.abs() {
            (self.sum - next) + value
        } else {
            (value - next) + self.sum
        };
        self.sum = next;
    }
    fn value(&self) -> f64 { self.sum + self.correction }
}

fn position(window: &[f64]) -> Option<f64> {
    if window.len() < 2 { return None; }
    let low = window.iter().copied().fold(f64::INFINITY, f64::min);
    let high = window.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !low.is_finite() || !high.is_finite() { return None; }
    if low == high { return Some(0.5); }

    // Divide first to avoid overflow in high-low or in the trapezoid sum.
    let scale = low.abs().max(high.abs());
    let base = low / scale;
    let heights: Vec<f64> = window.iter().map(|&v| (v / scale - base).max(0.0)).collect();
    let max_height = heights.iter().copied().fold(0.0_f64, f64::max);
    if max_height == 0.0 { return Some(0.5); }
    let heights: Vec<f64> = heights.into_iter().map(|v| v / max_height).collect();

    let mut total = Sum::default();
    for pair in heights.windows(2) { total.add((pair[0] + pair[1]) * 0.5); }
    let target = total.value() * 0.5;
    if target <= 0.0 || !target.is_finite() { return None; }

    let mut cumulative = Sum::default();
    for (i, pair) in heights.windows(2).enumerate() {
        let (a, b) = (pair[0], pair[1]);
        let area = (a + b) * 0.5;
        let before = cumulative.value();
        cumulative.add(area);
        if cumulative.value() >= target || i + 2 == window.len() {
            let remaining = (target - before).clamp(0.0, area);
            // When a plateau carries zero area at the halfway mark,
            // select the center of the entire zero-area plateau.
            if area == 0.0 {
                let mut end = i + 1;
                while end + 1 < heights.len() && heights[end] == 0.0 && heights[end + 1] == 0.0 {
                    end += 1;
                }
                return Some(((i as f64 + end as f64) * 0.5) / (window.len() - 1) as f64);
            }
            // Integral of linear interpolation: a*u + (b-a)*u*u/2.
            // Rationalized quadratic root avoids cancellation for b < a.
            let discriminant = (a*a + 2.0*(b-a)*remaining).max(0.0);
            let denom = a + discriminant.sqrt();
            let u = if denom > 0.0 { 2.0*remaining / denom } else { 0.0 };
            return Some(((i as f64 + u.clamp(0.0, 1.0)) / (window.len()-1) as f64).clamp(0.0, 1.0));
        }
    }
    None
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingAreaMedian<ITEMOUT>
where ITEMOUT: 'static + Number<ITEMOUT> {
    fn transform<'a, RI, ITEMIN, CTO>(
        &self, ri: RI, out: &mut [ITEMOUT], default_out: ITEMOUT, conv: CTO
    ) -> TransformResult
    where RI: TransformReadIterator<Item=&'a ITEMIN>,
          ITEMIN: 'a + Number<ITEMIN>,
          CTO: ConvertTo<ITEMIN, ITEMOUT> {
        let len = ri.len();
        if len != out.len() || self.count < 2 || self.count > len {
            return TransformResult::Error(0);
        }
        let values: Vec<f64> = ri.map(|x| { let v = conv.convert_ref(x); v.as_() }).collect();
        if values.iter().any(|v| !v.is_finite()) { return TransformResult::Error(1); }
        out.fill(default_out);
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            let Some(value) = position(&values[end + 1 - self.count..=end]) else {
                return TransformResult::Error(2);
            };
            let Some(converted) = ITEMOUT::from_f64(value) else {
                return TransformResult::Error(2);
            };
            *output = converted;
        }
        TransformResult::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::position;
    fn close(actual: f64, expected: f64) {
        assert!((actual-expected).abs() < 1e-12, "{actual} != {expected}");
    }
    #[test]
    fn constant_and_linear() {
        close(position(&[7.0,7.0,7.0]).unwrap(), 0.5);
        close(position(&[0.0,1.0,2.0]).unwrap(), 2.0_f64.sqrt()/2.0);
        close(position(&[2.0,1.0,0.0]).unwrap(), 1.0-2.0_f64.sqrt()/2.0);
    }
    #[test]
    fn scale_and_offset() {
        let base = position(&[1.0,2.0,5.0,3.0,2.0]).unwrap();
        close(position(&[101.0,102.0,105.0,103.0,102.0]).unwrap(),base);
        close(position(&[10.0,20.0,50.0,30.0,20.0]).unwrap(),base);
    }
    #[test]
    fn zero_area_plateau() {
        close(position(&[1.0,0.0,0.0,0.0,1.0]).unwrap(),0.5);
    }
    #[test]
    fn rejects_nonfinite() {
        assert!(position(&[1.0,f64::NAN]).is_none());
    }
}
