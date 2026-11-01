// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::transform::{sort::Sort, SequenceTransform};
use sequencetransform::{ConvertToIdentity, TransformResult};

#[test]
fn sorts_values_and_preserves_input() {
    let input = [4.0_f32, -2.0, 4.0, 1.0];
    let mut output = [0.0; 4];
    let result = Sort::<f32>::new().transform(
        input.iter(), &mut output, 0.0, ConvertToIdentity::<f32>::default()
    );
    assert_eq!(result, TransformResult::Ok);
    assert_eq!(output, [-2.0, 1.0, 4.0, 4.0]);
    assert_eq!(input, [4.0, -2.0, 4.0, 1.0]);
}

#[test]
fn empty_input_is_valid() {
    let input: [f32; 0] = [];
    let mut output: [f32; 0] = [];
    assert_eq!(
        Sort::<f32>::new().transform(input.iter(), &mut output, 0.0, ConvertToIdentity::<f32>::default()),
        TransformResult::Ok
    );
}

#[test]
fn rejects_mismatched_lengths_without_modifying_output() {
    let input = [1.0_f32, 2.0];
    let mut output = [99.0_f32];
    assert_eq!(
        Sort::<f32>::new().transform(input.iter(), &mut output, 0.0, ConvertToIdentity::<f32>::default()),
        TransformResult::Error(0)
    );
    assert_eq!(output, [99.0]);
}

#[test]
fn rejects_nan_without_modifying_output() {
    let input = [1.0_f32, f32::NAN, 2.0];
    let mut output = [99.0_f32; 3];
    assert_eq!(
        Sort::<f32>::new().transform(input.iter(), &mut output, 0.0, ConvertToIdentity::<f32>::default()),
        TransformResult::Error(1)
    );
    assert_eq!(output, [99.0; 3]);
}
