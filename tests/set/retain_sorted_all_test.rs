// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;

use sequencetransform::set::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retain_sorted_all_1() {
        let in_vec1: Vec::<f32> = vec!(1.0, 2.2, 3.5, 4.1, 7.7);
        let in_vec2: Vec::<f32> = vec!(1.0, 3.5, 5.1, 7.7);
        let in_vec3: Vec::<f32> = vec!(1.0, 1.1, 3.3, 3.5, 5.1, 7.7);
        let mut in_matrix: Vec::<Vec::<f32>> = vec!(in_vec1, in_vec2, in_vec3);

        let out_vec: Vec::<f32> = vec!(1.0, 3.5, 7.7);

        let ret = RetainSortedAll::new(
            |elem: &mut Vec::<f32>| { elem },
            |elem: &f32| { elem }
        );
        ret.exe_vec(in_matrix.as_mut_slice());

        for element in in_matrix.iter() {
            assert_eq!(*element, out_vec);
        }
    }
}

#[cfg(test)]
mod correctness_regressions {
    use sequencetransform::set::RetainSortedAll;

    #[test]
    fn empty_intersection_is_safe() {
        let intersection = RetainSortedAll::new(|v: &mut Vec<f32>| v, |v: &f32| v);
        let mut data: Vec<Vec<f32>> = vec![];
        intersection.exe_vec(&mut data);
        assert!(data.is_empty());
    }

    #[test]
    fn normalize_skips_empty_series() {
        let intersection = RetainSortedAll::new(|v: &mut Vec<f32>| v, |v: &f32| v);
        let mut data = vec![vec![], vec![1.0, 2.0, 3.0], vec![2.0, 3.0]];
        assert_eq!(intersection.normalize_exe_vec(1.0, &mut data), 2);
        assert_eq!(data[0], vec![2.0, 3.0]);
        assert_eq!(data[1], vec![2.0, 3.0]);
    }

    #[test]
    fn sorted_intersection_matches_reference_for_small_inputs() {
        let intersection = RetainSortedAll::new(|v: &mut Vec<i32>| v, |v: &i32| v);
        for left_mask in 0u32..32 {
            for right_mask in 0u32..32 {
                let left: Vec<i32> = (0..5).filter(|i| (left_mask & (1 << i)) != 0).collect();
                let right: Vec<i32> = (0..5).filter(|i| (right_mask & (1 << i)) != 0).collect();
                let expected: Vec<i32> = left.iter().copied().filter(|x| right.contains(x)).collect();
                let mut inputs = vec![left, right];
                intersection.exe_vec(&mut inputs);
                assert_eq!(inputs[0], expected, "left={left_mask}, right={right_mask}");
                assert_eq!(inputs[1], expected, "left={left_mask}, right={right_mask}");
            }
        }
    }
}
