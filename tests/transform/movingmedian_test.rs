// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::transform::*;
use sequencetransform::ConvertToIdentity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movingmedian() {
        let in_vec: Vec::<f32> = vec!(1.0, 2.0, 3.0, 4.0, 5.0);
        let mut out_vec = Vec::<f32>::with_capacity(5);
        out_vec.resize(5, 0.0);
        let order = MovingMedian::new(3);
        order.transform(in_vec.iter(), out_vec.as_mut_slice(), 0f32, ConvertToIdentity::<f32>::default());
        assert_eq!(out_vec, vec!(1.0, 2.0, 2.0, 3.0, 4.0));
    }
}
