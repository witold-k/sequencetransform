// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::marker::PhantomData;
use std::vec::Vec;
pub mod sortedtouint;

#[derive(Copy, Clone)]
pub struct SortedToUInt<T, const CAP_LN2: usize, const OVERLAP: usize, const CAP: usize> {
    marker:  PhantomData<T>
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

