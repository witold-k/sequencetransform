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
