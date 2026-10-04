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
        // then apply the algorithm
        let data_ptr = data.as_mut_ptr();
        let mut data_size = data.len();
        let rsl = RetainSortedLeft::<T, U>::new(self.select);
        let mut front;
        let vec;
        unsafe {
            vec = (self.select_vec)(&mut *data_ptr);
            front = vec.as_mut_slice();
        }

        let mut vec_size = front.len();
        let tail  = &mut data[1 .. data_size];
        data_size -= 1;
        for entry in tail.iter_mut().take(data_size) {
            let ivec = (self.select_vec)(entry);
            vec_size = rsl.exe(front, ivec.iter());
            front = &mut front[0..vec_size];
        }
        unsafe {
            let tmpvec = (self.select_vec)(&mut *data_ptr);
            tmpvec.truncate(vec_size);
        }

        for (i, entry) in tail.iter_mut().take(data_size).enumerate() {
            let ivec = (self.select_vec)(entry);
            let elem = ivec.as_mut_slice();
            vec_size = rsl.exe(elem, front.iter());
            unsafe {
                let tmpvec = (self.select_vec)(&mut *data_ptr.add(i + 1));
                tmpvec.truncate(vec_size);
            }
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
        // first remove all empty
        let mut len = data.len();
        if 0 == len { return 0 ; }

        for i in (0 .. len).rev() {
            if data[i].is_empty() { len = i; }
        }

        let tail = ((len as f32) * scale) as usize;
        self.exe_vec(&mut data[0 .. tail]);
        tail
    }
}
