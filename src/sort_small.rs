// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use serde::{Deserialize, Serialize};

/// Sort two comparable values in ascending order.
#[inline(always)]
pub fn sort2<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a <= b { (a, b) } else { (b, a) }
}

/// Sort three comparable values in ascending order.
#[inline(always)]
pub fn sort3<T: PartialOrd>(a: T, b: T, c: T) -> (T, T, T) {
    let (a, b) = sort2(a, b);
    if b <= c { (a, b, c) }
    else if a <= c { (a, c, b) }
    else { (c, a, b) }
}

/// Rolling median of the last three values. Initially all slots contain `new`'s value.
#[derive(Copy, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Hash, Debug)]
pub struct Median3<T> {
    data: [T; 3],
}

impl<T: Copy + PartialOrd> Median3<T> {
    pub fn new(value: T) -> Self {
        Self { data: [value; 3] }
    }

    #[inline(always)]
    pub fn push(self, value: T) -> Self {
        Self { data: [value, self.data[0], self.data[1]] }
    }

    #[inline(always)]
    pub fn median(self) -> T {
        sort3(self.data[0], self.data[1], self.data[2]).1
    }
}
