// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;

use sequencetransform::set::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retain_sorted_left_1() {
        let mut in_vec1: Vec::<f32> = vec!(1.0, 2.2, 3.5, 4.1, 7.7);
        let in_vec2: Vec::<f32> = vec!(1.0, 3.5, 5.1, 7.7);
        let out_vec: Vec::<f32> = vec!(1.0, 3.5, 7.7);

        let ret = RetainSortedLeft::new(
            |elem: &f32| { elem }
        );
        //let iter1 = &in_vec1.iter() as *const _;
        let len = ret.exe(
            in_vec1.as_mut_slice(),
            in_vec2.iter()
        );
        assert_eq!(len, 3);
        assert_eq!(&in_vec1[0..len], out_vec.as_slice());
    }

    #[test]
    fn test_retain_sorted_left_2() {
        let mut in_vec1: Vec::<f32> = vec!(2.2, 3.5, 4.1, 7.7);
        let in_vec2: Vec::<f32> = vec!(1.0, 3.5, 5.1, 7.7);
        let out_vec: Vec::<f32> = vec!(3.5, 7.7);

        let ret = RetainSortedLeft::new(
            |elem: &f32| { elem }
        );
        //let iter1 = &in_vec1.iter() as *const _;
        let len = ret.exe(
            in_vec1.as_mut_slice(),
            in_vec2.iter()
        );
        assert_eq!(len, 2);
        assert_eq!(&in_vec1[0..len], out_vec.as_slice());
    }

    #[test]
    fn test_retain_sorted_left_3() {
        let mut in_vec1: Vec::<f32> = vec!(2.2, 3.5, 4.1, 7.7, 8.0, 9.0);
        let in_vec2: Vec::<f32> = vec!(1.0, 3.5, 5.1, 7.7, 8.0, 9.1);
        let out_vec: Vec::<f32> = vec!(3.5, 7.7, 8.0);

        let ret = RetainSortedLeft::new(
            |elem: &f32| { elem }
        );
        //let iter1 = &in_vec1.iter() as *const _;
        let len = ret.exe(
            in_vec1.as_mut_slice(),
            in_vec2.iter()
        );
        assert_eq!(len, 3);
        assert_eq!(&in_vec1[0..len], out_vec.as_slice());
    }

}
