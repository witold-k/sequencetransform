// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingTimeReversalAsymmetry, SequenceTransform};

fn run(data: &[f64], count: usize, default: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    assert_eq!(MovingTimeReversalAsymmetry::<f64>::new(count).transform(
        data.iter(), &mut out, default, ConvertToIdentity::default()), TransformResult::Ok);
    out
}

#[test]
fn reversal_changes_sign() {
    let data = [0.0, 1.0, 3.0, 2.0, 8.0, 7.0, 11.0];
    let mut reversed = data;
    reversed.reverse();
    let forward = run(&data, data.len(), -7.0);
    let backward = run(&reversed, reversed.len(), -7.0);
    assert!((forward[6] + backward[6]).abs() < 1e-12);
    assert!(forward[6].abs() > 0.01);
}

#[test]
fn scale_translation_and_sign_invariance() {
    let data = [1.0, 2.0, 5.0, 3.0, 10.0, 6.0, 11.0, 9.0];
    let base = run(&data, 5, -7.0);
    let transformed: Vec<f64> = data.iter().map(|x| 1e8 + x * 1000.0).collect();
    let scaled = run(&transformed, 5, -7.0);
    for i in 0..data.len() {
        assert!((base[i] - scaled[i]).abs() < 1e-12);
    }
    let negative: Vec<f64> = data.iter().map(|x| -x).collect();
    let neg = run(&negative, 5, -7.0);
    for i in 4..data.len() {
        assert!((base[i] + neg[i]).abs() < 1e-12);
    }
}

#[test]
fn constant_windows_default_and_causal_prefix() {
    assert!(run(&[4.0; 20], 6, -9.0).iter().all(|x| *x == -9.0));
    let data: Vec<f64> = (0..40).map(|i| ((i * 17) % 23) as f64).collect();
    let before = run(&data, 9, -1.0);
    assert!(before[..8].iter().all(|x| *x == -1.0));
    assert!(before[8..].iter().all(|x| x.abs() <= 1.0));
    let mut extended = data;
    extended.extend([100.0, -100.0]);
    let after = run(&extended, 9, -1.0);
    assert_eq!(before, after[..before.len()]);
}

#[test]
fn handles_extreme_finite_increments_without_overflow() {
    let data = [0.0, 1e200, 0.0, 2e200, 1e200];
    let out = run(&data, 5, -7.0);
    assert!(out[4].is_finite());
    assert!(out[4].abs() <= 1.0);
}

#[test]
fn invalid_inputs_are_rejected() {
    let data = [1.0, 2.0, 3.0];
    let mut out = [0.0; 3];
    for count in [0, 1, 4] {
        assert_ne!(MovingTimeReversalAsymmetry::<f64>::new(count).transform(
            data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    }
    let invalid = [1.0, f64::INFINITY, 3.0];
    assert_ne!(MovingTimeReversalAsymmetry::<f64>::new(3).transform(
        invalid.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    let overflow = [-1e308, 1e308, 0.0];
    assert_ne!(MovingTimeReversalAsymmetry::<f64>::new(3).transform(
        overflow.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
}
