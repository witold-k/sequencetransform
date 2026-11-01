// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{DeltaSignumSum, SequenceTransform};

#[test]
fn deltasignumsum_finite_sequence_and_empty_input() {
    let input = [-1.0_f64, 1.0_f64, 1.0_f64, -3.0_f64, 0.0_f64];
    let mut output = [f64::NAN; 5];
    let transform = DeltaSignumSum::<f64>::default();
    assert_eq!(
        transform.transform(input.iter(), &mut output, 0.0, ConvertToIdentity::default()),
        TransformResult::Ok
    );
    assert_eq!(output, [0.0, 1.0, 1.0, 0.0, 1.0]);

    let empty: [f64; 0] = [];
    let mut empty_out: [f64; 0] = [];
    assert_eq!(
        transform.transform(empty.iter(), &mut empty_out, 0.0, ConvertToIdentity::default()),
        TransformResult::Ok
    );
}

#[test]
fn deltasignumsum_rejects_output_length_mismatch() {
    let input = [1.0_f32, 2.0];
    let mut output = [0.0_f32; 1];
    assert_ne!(
        DeltaSignumSum::<f32>::default().transform(
            input.iter(), &mut output, 0.0, ConvertToIdentity::default()
        ),
        TransformResult::Ok
    );
}
