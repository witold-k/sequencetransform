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

#[cfg(test)]
mod correctness_regressions {
    use sequencetransform::TransformResult;
    use sequencetransform::transform::{Order};

    #[test]
    fn order_rejects_nan_instead_of_panicking() {
        let data = [1.0_f32, f32::NAN];
        let mut out = [0i32; 2];
        assert_ne!(Order { factor: 1i32 }.transform_self(data.iter(), &mut out), TransformResult::Ok);
    }
}

#[cfg(test)]
mod floating_factor_regressions {
    use sequencetransform::transform::Order;
    use sequencetransform::TransformResult;

    #[test]
    fn zero_nan_and_infinite_factors_mean_unscaled_ranks() {
        let input = [1.0_f32, 3.5, 2.1];
        for factor in [0.0_f32, 1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut output = [f32::NAN; 3];
            assert_eq!(
                Order { factor }.transform_self(input.iter(), &mut output),
                TransformResult::Ok
            );
            assert_eq!(output, [0.0, 2.0, 1.0]);
        }
    }

    #[test]
    fn finite_nonidentity_factor_scales_ranks() {
        let input = [1.0_f64, 3.5, 2.1];
        let mut output = [0.0_f64; 3];
        assert_eq!(
            Order { factor: 2.0_f64 }.transform_self(input.iter(), &mut output),
            TransformResult::Ok
        );
        assert_eq!(output, [0.0, 4.0, 2.0]);
    }
}
