// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::set::RetainSortedLeft;

impl<'a, T: 'a, U> RetainSortedLeft<T, U>
where
    T: Clone,
    U: PartialOrd<U> + PartialEq<U> + Clone,
{
    pub fn new(select: fn(&T) -> &U) -> Self {
        RetainSortedLeft { select }
    }

    /// Retain elements whose key occurs in the second sorted sequence.
    /// Inputs must be sorted ascending by the selected key.
    /// Duplicate left elements are retained when their key matches.
    pub fn exe<J>(&self, data: &'a mut [T], iter2: J) -> usize
    where
        U: 'a,
        J: Iterator<Item = &'a T> + Clone,
    {
        let mut right = iter2.peekable();
        let mut retained = Vec::new();
        for left in data.iter() {
            let left_key = (self.select)(left);
            while let Some(other) = right.peek() {
                if (self.select)(other) < left_key {
                    right.next();
                } else {
                    break;
                }
            }
            if let Some(other) = right.peek() {
                if (self.select)(other) == left_key {
                    retained.push(left.clone());
                }
            }
        }
        let len = retained.len();
        data[..len].clone_from_slice(&retained);
        len
    }
}
