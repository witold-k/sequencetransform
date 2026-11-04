// SPDX-License-Identifier: Apache-2.0
//! Hankel-matrix effective rank using svdwrapper's CPU reduced SVD.
use crate::transform::*;
use ndarray::Array2;
use svdwrapper::{Backend, Svd, SvdMode};

impl<T> MovingMatrixEffectiveRank<T> {
    pub fn new(count: usize, columns: usize) -> Self {
        Self { marker: Default::default(), count, columns }
    }
}
impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingMatrixEffectiveRank<ITEMOUT>
where ITEMOUT: 'static + Number<ITEMOUT> {
    fn transform<'a, RI, ITEMIN, CTO>(
        &self, ri: RI, out: &mut [ITEMOUT], default_out: ITEMOUT, conv: CTO
    ) -> TransformResult
    where RI: TransformReadIterator<Item = &'a ITEMIN>,
          ITEMIN: 'a + Number<ITEMIN>,
          CTO: ConvertTo<ITEMIN, ITEMOUT> {
        let len = ri.len();
        if len != out.len() || self.count < 4 || self.count > len
            || self.columns < 2 || self.columns > self.count - 1 {
            return TransformResult::Error(0);
        }
        let values: Vec<f64> = ri.map(|x| { let v = conv.convert_ref(x); v.as_() }).collect();
        if values.iter().any(|v| !v.is_finite()) { return TransformResult::Error(1); }
        let Ok(svd) = Svd::<f64>::new(Backend::Cpu) else { return TransformResult::Error(2); };
        let rows = self.count - self.columns + 1;
        let mut matrix = Array2::<f64>::zeros((rows, self.columns));
        out.fill(default_out);
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            let start = end + 1 - self.count;
            let window = &values[start..=end];
            // Subtract the window mean to remove a pure DC component.
            let mean = window.iter().sum::<f64>() / self.count as f64;
            for i in 0..rows {
                for j in 0..self.columns {
                    matrix[(i,j)] = window[i+j] - mean;
                }
            }
            // A zero matrix has undefined singular-value probabilities.
            let amplitude = matrix.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
            if !amplitude.is_finite() || amplitude <= 1e-12 { continue; }
            matrix.mapv_inplace(|v| v / amplitude);
            let Ok(decomposition) = svd.compute(&matrix, SvdMode::Reduced) else {
                return TransformResult::Error(2);
            };
            let maximum = decomposition.s.iter().copied().fold(0.0_f64, f64::max);
            let total: f64 = decomposition.s.iter().copied().filter(|&s| s > maximum * 1e-12).sum();
            if !total.is_finite() || total<=0.0 { continue; }
            let entropy: f64 = decomposition.s.iter()
                .filter(|&&s| s > maximum * 1e-12)
                .map(|&s| {
                    let p=s/total;
                    -p*p.ln()
                }).sum();
            let rank=entropy.exp();
            let Some(v) = ITEMOUT::from_f64(rank) else { return TransformResult::Error(2); };
            *output=v;
        }
        TransformResult::Ok
    }
}
