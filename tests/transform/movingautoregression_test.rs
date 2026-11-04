// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingAutoregression, SequenceTransform};

fn coefficients(data: &[f64], count: usize, default: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    assert_eq!(MovingAutoregression::<f64>::new(count).transform(
        data.iter(), &mut out, default, ConvertToIdentity::default()), TransformResult::Ok);
    out
}

#[test]
fn exact_ar_one_with_intercept() {
    let mut data = vec![1.0_f64];
    for _ in 0..50 { data.push(3.0 + 0.75 * data.last().unwrap()); }
    let out = coefficients(&data, 12, -99.0);
    assert!(out[..11].iter().all(|v| *v == -99.0));
    for &value in &out[11..] {
        assert!((value - 0.75).abs() < 1e-8, "{value}");
    }
}

#[test]
fn constant_windows_use_default() {
    let out = coefficients(&[100.0; 20], 8, -7.0);
    assert!(out.iter().all(|v| *v == -7.0));
}

#[test]
fn large_offset_does_not_destroy_covariance() {
    let data: Vec<f64> = (0..30).map(|i| 1.0e12 + i as f64 * 0.125).collect();
    let out = coefficients(&data, 10, 0.0);
    assert!(out[9..].iter().all(|v| (*v - 1.0).abs() < 1e-10));
}

#[test]
fn prefix_is_causal() {
    let data: Vec<f64> = (0..30).map(|i| 100.0 + (i * i) as f64).collect();
    let before = coefficients(&data, 8, -9.0);
    let mut extended = data;
    extended.extend([10000.0, -10000.0, 50.0]);
    let after = coefficients(&extended, 8, -9.0);
    assert_eq!(before, after[..before.len()]);
}

#[test]
fn rejects_invalid_window_and_nonfinite_input() {
    let data = [1.0, 2.0, 3.0];
    let mut out = [0.0; 3];
    assert_ne!(MovingAutoregression::<f64>::new(2).transform(
        data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    let bad = [1.0, f64::NAN, 3.0];
    assert_ne!(MovingAutoregression::<f64>::new(3).transform(
        bad.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
}
