// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MinusDistanceSum, SequenceTransform};

#[test]
fn minus_distance_sum_nontrivial_sequence() {
    // Regression characterization of the current reverse-output algorithm.
    let input = [1.0_f32, 2.0, 3.0, 2.0, 1.0];
    let mut output = [0.0_f32; 5];
    let result = MinusDistanceSum::<f32>::default().transform(
        input.iter(), &mut output, 0.0, ConvertToIdentity::default()
    );
    assert_eq!(result, TransformResult::Ok);
    assert_eq!(output, [0.0, 0.0, 2.0, 5.0, 1.0]);
}

#[test]
fn minus_distance_sum_constant_sequence() {
    let input = [3.0_f64; 5];
    let mut output = [0.0_f64; 5];
    let result = MinusDistanceSum::<f64>::default().transform(
        input.iter(), &mut output, 0.0, ConvertToIdentity::default()
    );
    assert_eq!(result, TransformResult::Ok);
    assert_eq!(output, [0.0; 5]);
}

#[test]
fn cumulative_distances_are_chronological_and_causal() {
    let input = [1.0_f32, 2.0, 3.0, 2.0, 1.0, 4.0, 2.0, 1.0];
    let mut output = [99.0_f32; 8];
    let mut prefix = [99.0_f32; 5];
    assert_eq!(MinusDistanceSum::<f32>::default().transform(
        input.iter(), &mut output, 0.0, ConvertToIdentity::default()
    ), TransformResult::Ok);
    MinusDistanceSum::<f32>::default().transform(
        input[..5].iter(), &mut prefix, 0.0, ConvertToIdentity::default()
    );
    assert_eq!(output, [0.0, 0.0, 2.0, 5.0, 1.0, 6.0, 9.0, 6.0]);
    assert_eq!(prefix, output[..5]);
}
