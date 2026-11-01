// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingMedian2, SequenceTransform};

#[test]
fn averages_two_smallest_in_trailing_window() {
    let input = [5.0_f64, 1.0, 7.0, 3.0, 9.0];
    let mut output = [0.0; 5];
    let transform = MovingMedian2::<f64>::new_border(3, 0, 1);
    assert_eq!(transform.transform(input.iter(), &mut output, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    assert_eq!(output, [5.0, 3.0, 3.0, 2.0, 5.0]);
}

#[test]
fn invalid_borders_are_rejected() {
    let input = [1.0_f64, 2.0, 3.0];
    let mut output = [0.0; 3];
    assert_ne!(MovingMedian2::<f64>::new_border(3, 0, 3).transform(input.iter(), &mut output, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
}

#[test]
fn even_window_default_averages_middle_two() {
    let input = [1.0_f64, 4.0, 2.0, 3.0];
    let mut output = [0.0; 4];
    assert_eq!(MovingMedian2::<f64>::new(4).transform(input.iter(), &mut output, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    assert_eq!(output[3], 2.5);
}
