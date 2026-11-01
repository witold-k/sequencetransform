// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::ziptransform::{AtLastSignChangeToNeg, ZipSequenceTransform};

#[test]
fn atlastsignchangetoneg_tracks_last_matching_sign_transition() {
    let values = [10.0_f64, 20.0, 30.0, 40.0, 50.0];
    let triggers = [-1.0_f64, -1.0, 1.0, 1.0, -1.0];
    let mut output = [f64::NAN; 5];
    assert_eq!(
        AtLastSignChangeToNeg::<f64>::default().transform(
            values.iter(), triggers.iter(), &mut output, 0.0, ConvertToIdentity::default()
        ),
        Ok(5)
    );
    assert_eq!(output, [0.0, 10.0, 10.0, 10.0, 50.0]);
}

#[test]
fn atlastsignchangetoneg_handles_empty_input_and_rejects_length_mismatch() {
    let transform = AtLastSignChangeToNeg::<f32>::default();
    let empty: [f32; 0] = [];
    let mut empty_output: [f32; 0] = [];
    assert_eq!(
        transform.transform(
            empty.iter(), empty.iter(), &mut empty_output, 0.0, ConvertToIdentity::default()
        ),
        Ok(0)
    );
    let values = [1.0_f32, 2.0];
    let triggers = [1.0_f32, -1.0];
    let mut short_output = [0.0_f32];
    assert_eq!(
        transform.transform(
            values.iter(), triggers.iter(), &mut short_output, 0.0, ConvertToIdentity::default()
        ),
        Err(TransformResult::Error(0))
    );
    let mut output = [0.0_f32; 2];
    assert_eq!(
        transform.transform(
            values.iter(), triggers[..1].iter(), &mut output, 0.0, ConvertToIdentity::default()
        ),
        Err(TransformResult::Error(0))
    );
}
