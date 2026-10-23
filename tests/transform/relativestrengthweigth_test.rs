// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{RelativeStrengthWeigth, SequenceTransform};

#[test]
fn relativestrengthweigth_constant_and_monotonic_windows() {
    let transform = RelativeStrengthWeigth::<f64>::new(3, false);
    for (input, expected) in [
        ([2.0_f64, 2.0, 2.0, 2.0], [0.0, 0.0, 0.0, 0.0]),
        ([1.0_f64, 2.0, 3.0, 4.0], [0.0, 1.0, 1.0, 1.0]),
        ([4.0_f64, 3.0, 2.0, 1.0], [0.0, -1.0, -1.0, -1.0]),
    ] {
        let mut output = [f64::NAN; 4];
        assert_eq!(
            transform.transform(input.iter(), &mut output, 0.0, ConvertToIdentity::default()),
            TransformResult::Ok
        );
        assert_eq!(output, expected);
    }
}

#[test]
fn relativestrengthweigth_rejects_zero_window() {
    let input = [1.0_f32, 2.0, 3.0];
    let mut output = [0.0_f32; 3];
    assert_ne!(
        RelativeStrengthWeigth::<f32>::new(0, false).transform(
            input.iter(), &mut output, 0.0, ConvertToIdentity::default()
        ),
        TransformResult::Ok
    );
}
