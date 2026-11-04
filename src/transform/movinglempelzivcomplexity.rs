// SPDX-License-Identifier: Apache-2.0
//! LZ76 exhaustive parsing complexity of a three-symbol difference sequence.
use crate::transform::*;

impl<T> MovingLempelZivComplexity<T> {
    pub fn new(count: usize) -> Self { Self { marker: Default::default(), count } }
}
fn lz76(symbols: &[u8]) -> usize {
    // Exhaustive LZ76 parsing: a phrase is the shortest substring not
    // occurring in the prefix ending immediately before that phrase.
    let mut pos = 0;
    let mut phrases = 0;
    while pos < symbols.len() {
        let mut length = 1;
        while pos + length <= symbols.len() {
            let candidate = &symbols[pos..pos + length];
            let known = (0..pos).any(|start| {
                start + length <= pos && &symbols[start..start + length] == candidate
            });
            if !known || pos + length == symbols.len() { break; }
            length += 1;
        }
        phrases += 1;
        pos += length;
    }
    phrases
}
impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingLempelZivComplexity<ITEMOUT>
where ITEMOUT: 'static + Number<ITEMOUT> {
    fn transform<'a, RI, ITEMIN, CTO>(
        &self, ri: RI, out: &mut [ITEMOUT], default_out: ITEMOUT, conv: CTO
    ) -> TransformResult
    where RI: TransformReadIterator<Item = &'a ITEMIN>,
          ITEMIN: 'a + Number<ITEMIN>, CTO: ConvertTo<ITEMIN, ITEMOUT> {
        let len = ri.len();
        if len != out.len() || self.count < 8 || self.count > len {
            return TransformResult::Error(0);
        }
        let values: Vec<f64> = ri.map(|x| { let v = conv.convert_ref(x); v.as_() }).collect();
        if values.iter().any(|v| !v.is_finite()) { return TransformResult::Error(1); }
        out.fill(default_out);
        for (end, output) in out.iter_mut().enumerate().skip(self.count - 1) {
            let start = end + 1 - self.count;
            let symbols: Vec<u8> = values[start..=end].windows(2)
                .map(|p| if p[1] > p[0] { 2 } else if p[1] < p[0] { 0 } else { 1 })
                .collect();
            let n = symbols.len();
            let complexity = lz76(&symbols) as f64;
            // Conventional asymptotic normalization; can exceed one at
            // finite N and is deliberately not clamped.
            let score = complexity * (n as f64).ln() / (n as f64 * 3.0_f64.ln());
            let Some(v) = ITEMOUT::from_f64(score) else {
                return TransformResult::Error(2);
            };
            *output = v;
        }
        TransformResult::Ok
    }
}
