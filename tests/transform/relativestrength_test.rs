// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::transform::*;
use sequencetransform::ConvertToIdentity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relativestrength() {
        // 1.0 2.0 3.0 4.0 1.0
        // window is 3:
        // seq: 1.0 2.0 4.0 => 3 - 0, 3 => 3 / 3  => 1
        //      2.0 4.0 2.0 => 2 - 2, 4 => 0 / 4  => 0
        //      3.0 2.0 1.0 => 0 - 2, 2 => -2 / 2 => -1
        let in_vec: Vec::<f32> = vec!(1.0, 2.0, 4.0, 2.0, 1.0);
        let mut out_vec = Vec::<f32>::with_capacity(5);
        out_vec.resize(5, 0.0);
        let order = RelativeStrength::new(3, true);
        order.transform(in_vec.iter(), out_vec.as_mut_slice(), 0f32, ConvertToIdentity::<f32>::default());
        assert_eq!(out_vec, vec!(0.0, 0.0, 1.0, 0.0, -1.0));
    }
}
