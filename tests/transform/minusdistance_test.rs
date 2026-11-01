// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MinusDistance, SequenceTransform};

#[test]
fn constant_sequence_has_zero_signed_distances() {
    let input = [3.0_f64; 5];
    let mut output = [0.0_f64; 5];
    assert_eq!(
        MinusDistance::<f64>::default().transform(
            input.iter(), &mut output, 0.0, ConvertToIdentity::default()
        ),
        TransformResult::Ok
    );
    assert_eq!(output, [0.0; 5]);
}

#[test]
fn output_length_must_match_input_length() {
    let input = [1.0_f32, 2.0];
    let mut output = [0.0_f32; 1];
    assert_ne!(
        MinusDistance::<f32>::default().transform(
            input.iter(), &mut output, 0.0, ConvertToIdentity::default()
        ),
        TransformResult::Ok
    );
}

#[test]
fn distances_search_only_past_and_keep_chronological_alignment() {
    let input = [1.0_f32, 2.0, 3.0, 2.0, 1.0];
    let mut output = [99.0_f32; 5];
    assert_eq!(MinusDistance::<f32>::default().transform(
        input.iter(), &mut output, 0.0, ConvertToIdentity::default()
    ), TransformResult::Ok);
    assert_eq!(output, [0.0, 0.0, 2.0, 3.0, -4.0]);
}

#[test]
fn changing_future_does_not_change_past_distances() {
    let input = [5.0_f32, 6.0, 8.0, 7.0, 5.0, 4.0];
    let mut full = [99.0_f32; 6];
    let mut prefix = [99.0_f32; 4];
    MinusDistance::<f32>::default().transform(input.iter(), &mut full, 0.0, ConvertToIdentity::default());
    MinusDistance::<f32>::default().transform(input[..4].iter(), &mut prefix, 0.0, ConvertToIdentity::default());
    assert_eq!(full, [0.0, 0.0, 2.0, 3.0, -4.0, -5.0]);
    assert_eq!(prefix, full[..4]);
}
