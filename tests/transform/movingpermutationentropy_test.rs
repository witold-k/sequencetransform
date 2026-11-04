// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingPermutationEntropy, SequenceTransform};

fn run(data: &[f64], count: usize, order: usize, default: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    assert_eq!(MovingPermutationEntropy::<f64>::new(count, order).transform(
        data.iter(), &mut out, default, ConvertToIdentity::default()), TransformResult::Ok);
    out
}

#[test]
fn monotonic_and_constant_sequences_have_zero_entropy() {
    let increasing: Vec<f64> = (0..40).map(f64::from).collect();
    for data in [&increasing[..], &[5.0; 40][..]] {
        let out = run(data, 12, 3, -1.0);
        assert!(out[..11].iter().all(|v| *v == -1.0));
        assert!(out[11..].iter().all(|v| v.abs() < 1e-12));
    }
}

#[test]
fn alternating_sequence_has_two_patterns() {
    let data: Vec<f64> = (0..40).map(|i| if i % 2 == 0 { 0.0 } else { 1.0 }).collect();
    let out = run(&data, 12, 3, -1.0);
    let expected = (2.0_f64).ln() / (6.0_f64).ln();
    assert!((out[11] - expected).abs() < 1e-12);
}

#[test]
fn causal_and_invariant_to_monotonic_transform() {
    let data: Vec<f64> = (0..50).map(|i| ((i * 37) % 23) as f64).collect();
    let before = run(&data, 14, 3, -1.0);
    let scaled: Vec<f64> = data.iter().map(|x| x * 7.0 + 1000.0).collect();
    assert_eq!(before, run(&scaled, 14, 3, -1.0));
    let mut extended = data;
    extended.extend([100.0, -100.0]);
    let after = run(&extended, 14, 3, -1.0);
    assert_eq!(before, after[..before.len()]);
}

#[test]
fn rejects_invalid_parameters_and_nonfinite_samples() {
    let data = [1.0, 2.0, 3.0];
    let mut out = [0.0; 3];
    for (count, order) in [(2, 3), (3, 1), (3, 8), (4, 3)] {
        assert_ne!(MovingPermutationEntropy::<f64>::new(count, order).transform(
            data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    }
    let bad = [1.0, f64::NAN, 3.0];
    assert_ne!(MovingPermutationEntropy::<f64>::new(3, 3).transform(
        bad.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
}
