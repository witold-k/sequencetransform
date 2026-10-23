// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::ziptransform::{SelectedSyncAccumulation, ZipSequenceTransform};

#[test]
fn selectedsyncaccumulation_rejects_mismatched_input_lengths() {
    let transform = SelectedSyncAccumulation::<f64>::default();
    let data = [2.0_f64, 7.0];
    let trigger = [0.0_f64];
    let mut output = [99.0_f64; 2];
    assert_eq!(
        transform.transform(data.iter(), trigger.iter(), &mut output, 0.0, ConvertToIdentity::default()),
        Err(TransformResult::Error(1))
    );
    assert_eq!(output, [99.0, 99.0]);
}

#[test]
fn selectedsyncaccumulation_rejects_invalid_output_capacity() {
    let transform = SelectedSyncAccumulation::<f64>::default();
    let data = [2.0_f64, 7.0, 3.0];
    let trigger = [0.0_f64, 1.0, 0.0];
    let mut output = [99.0_f64; 2];
    assert_eq!(
        transform.transform(data.iter(), trigger.iter(), &mut output, 0.0, ConvertToIdentity::default()),
        Err(TransformResult::Error(2))
    );
}

#[test]
fn selectedsyncaccumulation_inactive_triggers_produce_zero_at_each_position() {
    let transform = SelectedSyncAccumulation::<f64>::default();
    let data = [2.0_f64, 7.0, 3.0];
    let trigger = [0.0_f64, 0.0, 0.0];
    let mut output = [99.0_f64; 3];
    assert_eq!(
        transform.transform(data.iter(), trigger.iter(), &mut output, 0.0, ConvertToIdentity::default()),
        Ok(3)
    );
    assert_eq!(output, [0.0, 0.0, 0.0]);
}

#[test]
fn selectedsyncaccumulation_maps_two_completed_trigger_intervals_to_output_positions() {
    let transform = SelectedSyncAccumulation::<f64>::default();
    let data = [9.0_f64, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let trigger = [0.0_f64, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
    let mut output = [99.0_f64; 9];
    assert_eq!(
        transform.transform(data.iter(), trigger.iter(), &mut output, 0.0, ConvertToIdentity::default()),
        Ok(9)
    );
    assert_eq!(output, [0.0, 8.0, 8.0, 8.0, 0.0, 0.0, 3.0, 3.0, 0.0]);
}
