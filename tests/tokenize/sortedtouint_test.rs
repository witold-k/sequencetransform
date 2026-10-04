// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::*;
use sequencetransform::tokenize::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veci8_to_u64() {
        //  2  3  1  0
        // 10 11 01 00
        //     B     4
        assert_eq!(veci8_to_u64::<2>(vec!(0, 1, 3, 2)), 0xB4u64);
    }

    #[test]
    fn test_tokentize() {
        let s2i = SortedToUInt::<u64, 2, 2, 4>::default();

        let in_vec: Vec::<f32> = vec!(1.0, 2.0, 8.0, 2.5, 3.0, 7.0, 8.0, 5.0);
        let s2i_len = s2i.len(&in_vec.iter());

        let should_vec: Vec::<u64> = vec!(
            veci8_to_u64::<2>(vec!(0, 1, 3, 2)),
            veci8_to_u64::<2>(vec!(3, 0, 1, 2)),
            veci8_to_u64::<2>(vec!(0, 2, 3, 1))
        );
        assert_eq!(s2i_len, should_vec.len());

        let mut out_vec = vec![0; s2i_len];
        out_vec.resize(s2i_len, 0);

        let res = s2i.transform_self(in_vec.iter(), out_vec.as_mut_slice(), 0u64);
        assert_eq!(res, TransformResult::Ok);
        assert_eq!(out_vec, should_vec);
    }

}
