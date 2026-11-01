// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::ziptransform::{SelectedAccumulationBorder, ZipSequenceTransform};

#[test]
fn selectedaccumulation_border_rejects_mismatched_input_lengths() {
    let transform = SelectedAccumulationBorder::<f64>::new(5.0_f64);
    let data = [2.0_f64, 7.0];
    let trigger = [0.0_f64];
    let mut output = [99.0_f64; 2];
    assert_eq!(
        transform.transform(data.iter(), trigger.iter(), &mut output, 0.0, ConvertToIdentity::default()),
        Err(TransformResult::Error(0))
    );
    assert_eq!(output, [99.0, 99.0]);
}

#[test]
fn selectedaccumulation_border_rejects_invalid_output_capacity() {
    let transform = SelectedAccumulationBorder::<f64>::new(5.0_f64);
    let data = [2.0_f64, 7.0, 3.0];
    let trigger = [0.0_f64, 1.0, 0.0];
    let mut output = [99.0_f64; 0];
    assert_eq!(
        transform.transform(data.iter(), trigger.iter(), &mut output, 0.0, ConvertToIdentity::default()),
        Err(TransformResult::Error(1))
    );
}

#[test]
fn selectedaccumulation_border_no_completed_trigger_window_produces_no_results() {
    let transform = SelectedAccumulationBorder::<f64>::new(5.0_f64);
    let data = [2.0_f64, 7.0, 3.0];
    let trigger = [0.0_f64, 0.0, 0.0];
    let mut output = [99.0_f64; 3];
    assert_eq!(
        transform.transform(data.iter(), trigger.iter(), &mut output, 0.0, ConvertToIdentity::default()),
        Ok(0)
    );
    assert_eq!(output, [99.0, 99.0, 99.0]);
}

#[test]
fn selectedaccumulation_border_classifies_completed_windows() {
    let data = [9.0_f64, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let trigger = [0.0_f64, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
    let transform = SelectedAccumulationBorder::new(5.0_f64);
    let mut output = [99.0_f64; 9];
    assert_eq!(
        transform.transform(data.iter(), trigger.iter(), &mut output, 0.0, ConvertToIdentity::default()),
        Ok(2)
    );
    assert_eq!(&output[..2], &[1.0, 0.0]);
}
