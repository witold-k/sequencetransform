// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::transform::*;
use sequencetransform::ConvertToIdentity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movingaverage() {
        let in_vec: Vec::<f32> = vec!(1.0, 2.0, 3.0, 4.0, 5.0);
        let mut out_vec = Vec::<f32>::with_capacity(5);
        out_vec.resize(5, 0.0);
        let order = MovingAverage::new(2);
        order.transform_self(in_vec.iter(), out_vec.as_mut_slice(), ConvertToIdentity::<f32>::default());
        assert_eq!(out_vec, vec!(1.0, 1.5, 2.5, 3.5, 4.5));
    }
}

#[cfg(test)]
mod invalid_window_regression {
    use sequencetransform::{ConvertToIdentity, TransformResult};
    use sequencetransform::transform::{MovingAverage};

    #[test]
    fn invalid_moving_window_returns_error() {
        let data = [1.0_f32, 2.0, 3.0];
        let mut out = [0.0_f32; 3];
        assert_ne!(MovingAverage::<f32>::new(0).transform_self(data.iter(), &mut out, ConvertToIdentity::default()), TransformResult::Ok);
    }
}
