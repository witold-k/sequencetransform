// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingAutoregressionChange, SequenceTransform};

fn run(data: &[f64], count: usize, default: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    assert_eq!(MovingAutoregressionChange::<f64>::new(count).transform(
        data.iter(), &mut out, default, ConvertToIdentity::default()), TransformResult::Ok);
    out
}

#[test]
fn same_dynamics_have_zero_change() {
    let data: Vec<f64> = (0..40).map(|i| 1e12 + i as f64 * 0.125).collect();
    let out = run(&data, 12, -1.0);
    assert!(out[..11].iter().all(|v| *v == -1.0));
    assert!(out[11..].iter().all(|v| v.abs() < 1e-12));
}

#[test]
fn different_half_window_slopes_are_detected() {
    // First half y=2x+1, second half y=-x+10.
    let data = [1.0, 3.0, 7.0, 15.0, 31.0, 63.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0];
    let out = run(&data, 12, -1.0);
    assert!((out[11] - 3.0).abs() < 1e-12, "{}", out[11]);
}

#[test]
fn constant_half_window_is_undefined() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 8.0, 8.0, 8.0, 8.0, 8.0];
    assert_eq!(run(&data, 12, -7.0)[11], -7.0);
}

#[test]
fn causal_prefix_and_invalid_inputs() {
    let data: Vec<f64> = (0..40).map(|i| ((i * 17) % 23) as f64).collect();
    let before = run(&data, 12, -1.0);
    let mut extended = data;
    extended.extend([1000.0, -1000.0]);
    let after = run(&extended, 12, -1.0);
    assert_eq!(before, after[..before.len()]);
    let input = [1.0; 6];
    let mut out = [0.0; 6];
    for count in [0, 5, 7, 8] {
        assert_ne!(MovingAutoregressionChange::<f64>::new(count).transform(
            input.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    }
    let invalid = [1.0, 2.0, 3.0, 4.0, f64::NAN, 6.0];
    assert_ne!(MovingAutoregressionChange::<f64>::new(6).transform(
        invalid.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
}
