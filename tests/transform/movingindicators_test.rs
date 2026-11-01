// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::*;

fn run<T: SequenceTransform<f64>>(transform: T, input: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; input.len()];
    assert_eq!(transform.transform(input.iter(), &mut out, 0.0, ConvertToIdentity::<f64>::default()), TransformResult::Ok);
    out
}
fn close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (i, (&a, &b)) in actual.iter().zip(expected).enumerate() {
        assert!((a-b).abs() < 1e-9, "index {i}: {a} != {b}");
    }
}
#[test]
fn positive_strength_exact_and_prefix() {
    close(&run(MovingPositiveStrength::<f64>::new(3), &[1.,3.,2.,5.]), &[0.,1.,2./3.,3./4.]);
}
#[test]
fn direction_exact_and_prefix() {
    close(&run(MovingDirection::<f64>::new(3), &[1.,3.,2.,5.]), &[0.,1.,0.,0.]);
}
#[test]
fn weighted_position_known_case() {
    close(&run(MovingWeightedMedianPosition::<f64>::new(5), &[1.,1.,6.,1.,1.]), &[0.,0.5,1./3.,0.5,0.4]);
    close(&run(MovingSquaredWeightedMedianPosition::<f64>::new(5), &[1.,1.,6.,1.,1.]), &[0.,0.5,1./3.,0.5,0.4]);
}
#[test]
fn crossing_flat_and_oscillation() {
    close(&run(MovingMaxCrossingLevel::<f64>::new(3), &[3.,3.,3.]), &[3.,3.,3.]);
    let v = run(MovingMaxCrossingLevel::<f64>::new(4), &[1.,3.,1.,3.]);
    close(&v, &[1.,2.,5./3.,1.5]);
}
#[test]
fn documented_four_sample_examples() {
    close(&run(MovingPositiveStrength::<f64>::new(4), &[1.,3.,2.,5.]), &[0.,1.,2./3.,5./6.]);
    close(&run(MovingDirection::<f64>::new(4), &[1.,3.,2.,5.]), &[0.,1.,0.,1./3.]);
}
#[test]
fn invalid_windows_and_length_are_errors() {
    let data=[1.,2.,3.];
    let mut out=[0.;3];
    assert_ne!(MovingDirection::<f64>::new(0).transform(data.iter(),&mut out,0.,ConvertToIdentity::default()),TransformResult::Ok);
    assert_ne!(MovingPositiveStrength::<f64>::new(4).transform(data.iter(),&mut out,0.,ConvertToIdentity::default()),TransformResult::Ok);
    assert_ne!(MovingWeightedMedianPosition::<f64>::new(3).transform(data.iter(),&mut out[..2],0.,ConvertToIdentity::default()),TransformResult::Ok);
}
#[test]
fn causal_prefixes_unchanged_by_future_data() {
    let short=[1.,3.,2.,5.];
    let long=[1.,3.,2.,5.,100.,-100.];
    let a=run(MovingDirection::<f64>::new(3),&short);
    let b=run(MovingDirection::<f64>::new(3),&long);
    close(&a,&b[..short.len()]);
    let a=run(MovingPositiveStrength::<f64>::new(3),&short);
    let b=run(MovingPositiveStrength::<f64>::new(3),&long);
    close(&a,&b[..short.len()]);
}
