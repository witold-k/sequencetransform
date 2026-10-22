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

#[cfg(test)]
mod invalid_window_regression {
    use sequencetransform::{ConvertToIdentity, TransformResult};
    use sequencetransform::transform::{MovingMedian, SequenceTransform};

    #[test]
    fn invalid_moving_window_returns_error() {
        let data = [1.0_f32, 2.0, 3.0];
        let mut out = [0.0_f32; 3];
        assert_ne!(MovingMedian::<f32>::new(4).transform(data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    }
}
