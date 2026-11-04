// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingExplainedVariance, SequenceTransform};

fn run(data: &[f64], count: usize, default: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    assert_eq!(MovingExplainedVariance::<f64>::new(count).transform(
        data.iter(), &mut out, default, ConvertToIdentity::default()), TransformResult::Ok);
    out
}

#[test]
fn exact_linear_relationship_has_unit_score() {
    let mut data = vec![1.0];
    for _ in 0..30 { data.push(3.0 + 0.75 * data.last().unwrap()); }
    let out = run(&data, 10, -1.0);
    assert!(out[..9].iter().all(|v| *v == -1.0));
    assert!(out[9..].iter().all(|v| (*v - 1.0).abs() < 1e-8), "{out:?}");
}

#[test]
fn zero_correlation_and_constant_windows() {
    // pairs: (0,1), (1,0), (0,-1) => x=[0,1,0], y=[1,0,-1]; covariance zero.
    let out = run(&[0.0, 1.0, 0.0, -1.0], 4, -2.0);
    assert!(out[3].abs() < 1e-12);
    assert!(run(&[8.0; 12], 5, -2.0).iter().all(|v| *v == -2.0));
}

#[test]
fn offset_and_future_independence() {
    let data: Vec<f64> = (0..40).map(|i| 1e12 + i as f64 * 0.125).collect();
    let before = run(&data, 9, -1.0);
    assert!(before[8..].iter().all(|v| (*v - 1.0).abs() < 1e-12));
    let mut extended = data;
    extended.extend([0.0, 1e14]);
    let after = run(&extended, 9, -1.0);
    assert_eq!(before, after[..before.len()]);
}

#[test]
fn invalid_inputs_rejected() {
    let data = [1.0, 2.0, 3.0];
    let mut out = [0.0; 3];
    assert_ne!(MovingExplainedVariance::<f64>::new(2).transform(
        data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    let bad = [1.0, f64::INFINITY, 3.0];
    assert_ne!(MovingExplainedVariance::<f64>::new(3).transform(
        bad.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
}
