// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;
use crate::transform::*;

struct ExtremaTree<T> {
    size: usize,
    min: Vec<Option<T>>,
    max: Vec<Option<T>>,
}
impl<T: Number<T>> ExtremaTree<T> {
    fn new(values: &[T]) -> Self {
        let size = values.len().max(1).next_power_of_two();
        let mut min = vec![None; size * 2];
        let mut max = vec![None; size * 2];
        for (i, &v) in values.iter().enumerate() {
            min[size + i] = Some(v);
            max[size + i] = Some(v);
        }
        for i in (1..size).rev() {
            min[i] = match (min[2*i], min[2*i+1]) {
                (Some(a), Some(b)) => Some(if a <= b { a } else { b }),
                (a,b) => a.or(b),
            };
            max[i] = match (max[2*i], max[2*i+1]) {
                (Some(a), Some(b)) => Some(if a >= b { a } else { b }),
                (a,b) => a.or(b),
            };
        }
        Self { size, min, max }
    }

    fn rightmost(&self, end: usize, threshold: T, greater: bool) -> Option<usize> {
        self.search(1, 0, self.size, end, threshold, greater)
    }

    fn search(&self, node: usize, lo: usize, hi: usize, end: usize, threshold: T, greater: bool) -> Option<usize> {
        if lo >= end { return None; }
        let extreme = if greater { self.max[node] } else { self.min[node] };
        if !extreme.is_some_and(|v| if greater { v >= threshold } else { v <= threshold }) {
            return None;
        }
        if hi - lo == 1 { return Some(lo); }
        let mid = (lo + hi) / 2;
        self.search(node * 2 + 1, mid, hi, end, threshold, greater)
            .or_else(|| self.search(node * 2, lo, mid, end, threshold, greater))
    }
}

// Shared causal search; reproduces the original iterator-consumption distances.
pub(super) fn signed_distances<'a, RI, ITEMIN, ITEMOUT>(ri: RI, out: &mut [ITEMOUT]) -> TransformResult
where
    RI: TransformReadIterator<Item = &'a ITEMIN>,
    ITEMIN: 'a + Number<ITEMIN>,
    ITEMOUT: Number<ITEMOUT>,
{
    if ri.len() != out.len() { return TransformResult::Error(0); }
    let values: Vec<ITEMIN> = ri.copied().collect();
    let tree = ExtremaTree::new(&values);
    let zero = ITEMOUT::from_u32(0).unwrap();
    let mut prev_different = vec![None; values.len()];
    for i in 1..values.len() {
        prev_different[i] = if values[i-1] != values[i] {
            Some(i-1)
        } else {
            prev_different[i-1]
        };
    }
    for i in 0..values.len() {
        if i < 2 { out[i] = zero; continue; }
        let j = if values[i-2] != values[i] {
            Some(i-2)
        } else {
            prev_different[i-2]
        };
        let Some(j) = j else { out[i] = zero; continue; };
        let greater = values[j] < values[i];
        // The first different value at j was consumed by the old iterator.
        // A crossing at k consumes k too, leaving k elements.
        let remaining = tree.rightmost(j, values[i], greater).map_or(0, |k| k);
        let distance = i - remaining;
        let signed = if greater { distance as isize } else { -(distance as isize) };
        out[i] = FromPrimitive::from_isize(signed).unwrap();
    }
    TransformResult::Ok
}
