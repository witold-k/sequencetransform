// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingSpectralChange, SequenceTransform};

fn run(data: &[f64], count: usize, default: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    assert_eq!(MovingSpectralChange::<f64>::new(count).transform(
        data.iter(), &mut out, default, ConvertToIdentity::default()), TransformResult::Ok);
    out
}
#[test]
fn bounded_and_causal() {
    let data: Vec<f64> = (0..64).map(|i| (i as f64 * 0.35).sin() + 0.3 * (i as f64 * 1.2).cos()).collect();
    let before = run(&data, 16, -7.0);
    assert!(before[..15].iter().all(|v| *v == -7.0));
    assert!(before[15..].iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)));
    let mut extended = data;
    extended.extend([100.0, -100.0]);
    let after = run(&extended, 16, -7.0);
    assert_eq!(before, after[..before.len()]);
}
#[test]
fn constant_and_invalid_windows() {
    assert!(run(&[3.0; 32], 16, -9.0).iter().all(|v| *v == -9.0));
    let data = [1.0, 2.0, 3.0];
    let mut out = [0.0; 3];
    for count in [0, 1, 2, 3, 5] {
        assert_ne!(MovingSpectralChange::<f64>::new(count).transform(
            data.iter(), &mut out, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    }
    let invalid = [1.0, f64::NAN, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
    let mut output = [0.0; 16];
    assert_ne!(MovingSpectralChange::<f64>::new(16).transform(
        invalid.iter(), &mut output, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
}
#[test]
fn invariant_to_constant_offset() {
    let data: Vec<f64> = (0..32).map(|i| (i as f64 * 0.3).sin() + 0.2 * (i as f64 * 0.9).cos()).collect();
    let shifted: Vec<f64> = data.iter().map(|v| v + 100.0).collect();
    let a = run(&data, 16, -1.0);
    let b = run(&shifted, 16, -1.0);
    for i in 15..32 {
        assert!((a[i] - b[i]).abs() < 1e-10, "{}: {} {}", i, a[i], b[i]);
    }
}

#[test]
fn spectral_change_detects_frequency_shift() {
    let half = 32;
    let tone = |k: f64| -> Vec<f64> {
        (0..half).map(|i| (2.0 * std::f64::consts::PI * k * i as f64 / half as f64).sin()).collect()
    };
    let mut stable = tone(4.0);
    stable.extend(tone(4.0));
    let mut changed = tone(4.0);
    changed.extend(tone(12.0));
    assert!(run(&stable, 64, -1.0)[63] < 1e-10);
    assert!(run(&changed, 64, -1.0)[63] > 0.5);
}
