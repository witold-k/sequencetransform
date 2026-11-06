// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingExtremaBisectorSlope, SequenceTransform};

fn run(data: &[f64], count: usize) -> (TransformResult, Vec<f64>) {
    let mut out = vec![-7.0; data.len()];
    let status = MovingExtremaBisectorSlope::<f64>::new(count)
        .transform(data.iter(), &mut out, -7.0, ConvertToIdentity::default());
    (status, out)
}
fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
}

#[test]
fn symmetric_lines_have_horizontal_bisector() {
    // Upper line falls, lower line rises with equal normalized slope.
    let (status, output) = run(&[10.0, 0.0, 9.0, 1.0], 4);
    assert_eq!(status, TransformResult::Ok);
    assert_eq!(&output[..3], &[-7.0; 3]);
    close(output[3], 0.0);
}

#[test]
fn asymmetric_lines_bisect_angles_not_slopes() {
    // Top pair: (0,10), (2,9); bottom pair: (1,0), (3,2).
    // Normalized slopes: -0.15 and +0.6.
    let (_, output) = run(&[10.0, 0.0, 9.0, 2.0], 4);
    let expected = ((-0.15_f64).atan() + (0.6_f64).atan()).mul_add(0.5, 0.0).tan();
    close(output[3], expected);
    assert!(output[3] > 0.0);
}

#[test]
fn invariant_under_positive_affine_transforms_and_causal() {
    let x = [10.0, 0.0, 9.0, 2.0, 8.0, 3.0];
    let y = x.map(|v| v * 3.0 + 100.0);
    let (_, a) = run(&x, 4);
    let (_, b) = run(&y, 4);
    for (u, v) in a[3..].iter().zip(&b[3..]) { close(*u, *v); }
    let mut extended = x.to_vec();
    extended.extend([1000.0, -1000.0]);
    assert_eq!(a, run(&extended, 4).1[..x.len()]);
}

#[test]
fn constant_and_invalid_windows() {
    assert_eq!(run(&[2.0; 4], 4).1[3], -7.0);
    assert_eq!(run(&[1.0, 2.0, 3.0], 4).0, TransformResult::Error(0));
    assert_eq!(run(&[1.0, f64::NAN, 2.0, 3.0], 4).0, TransformResult::Error(1));
}
