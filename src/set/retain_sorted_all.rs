// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use crate::*;
use crate::set::*;

impl<'a, S, T: 'a, U> RetainSortedAll<S, T, U>
where
    T: Clone,
    U: PartialOrd<U> + PartialEq<U> + Clone
{

    #[inline(always)]
    pub fn new(
        select_vec:  fn(&mut S) -> &mut Vec::<T>,
        select:      fn(&T) -> &U
    ) -> Self {
        RetainSortedAll { select_vec, select }
    }

    pub fn exe_vec(&self, data: &'a mut [S])
    where
        U: 'a
    {
        if data.is_empty() { return; }
        // Work on disjoint entries without raw pointers or aliased mutable references.
        let (first, tail) = data.split_first_mut().unwrap();
        let first_vec = (self.select_vec)(first);
        let rsl = RetainSortedLeft::<T, U>::new(self.select);
        let mut front = first_vec.as_mut_slice();
        for entry in tail.iter_mut() {
            let other = (self.select_vec)(entry);
            let valid = rsl.exe(front, other.iter());
            front = &mut front[..valid];
        }
        let valid = front.len();
        first_vec.truncate(valid);
        let front = first_vec.as_slice();
        for entry in tail.iter_mut() {
            let other = (self.select_vec)(entry);
            let valid = rsl.exe(other.as_mut_slice(), front.iter());
            other.truncate(valid);
        }
    }

    pub fn normalize_exe_vec(
        &self,
        scale: f32,
        data: &'a mut [S]
    ) -> usize
    where
        U: 'a,
        S: ULength<usize>
    {
        assert!(scale.is_finite() && (0.0..=1.0).contains(&scale));
        let mut len = 0;
        for i in 0..data.len() {
            if !data[i].is_empty() { data.swap(len, i); len += 1; }
        }
        let selected = ((len as f32) * scale) as usize;
        if selected > 0 { self.exe_vec(&mut data[..selected]); }
        selected
    }
}
