// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::marker::PhantomData;
use std::vec::Vec;
pub mod sortedtouint;

/// Encode each full overlapping window as packed ascending ranks.
///
/// With a positive `threshold`, small consecutive declines are replaced only
/// for ranking by the next representable value above the preceding adjusted
/// sample. The threshold is an absolute difference in input units.
/// A zero threshold preserves the original exact-ranking behavior.
#[derive(Copy, Clone)]
pub struct SortedToUInt<T, const CAP_LN2: usize, const OVERLAP: usize, const CAP: usize> {
    marker:  PhantomData<T>,
    pub threshold: f64
}


#[inline(always)]
pub fn veci8_to_u64<const SHIFT: usize>(
    vec: Vec::<i8>
) -> u64 {
    // then convert ordered sequence into a token
    let mut token: u64 = 0;
    let mut shift: u64 = 0;
    for val in vec.iter() {
        let mask: u64 = (*val as u64) << shift;
        token |= mask;
        shift += SHIFT as u64;
    }
    token
}

