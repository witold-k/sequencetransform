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
