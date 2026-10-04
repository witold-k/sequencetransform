// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::transform::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        let in_vec: Vec::<f32> = vec!(1.0, 3.5, 2.1);
        let mut out_vec = vec![0; 3];
        let order = Order { factor: 0i32 };
        order.transform_self(in_vec.iter(), out_vec.as_mut_slice());
        assert_eq!(out_vec, vec!(0, 2, 1));
    }
}
