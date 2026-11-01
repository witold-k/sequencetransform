// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MinusDistance, MinusDistanceSum, SequenceTransform};

fn reference(input: &[f64]) -> Vec<f64> {
    (0..input.len()).map(|i| {
        if i < 2 { return 0.0; }
        let Some(j) = (0..i-1).rev().find(|&j| input[j] != input[i]) else {
            return 0.0;
        };
        let upward = input[j] < input[i];
        let crossing = (0..j).rev().find(|&k| {
            if upward { input[k] >= input[i] } else { input[k] <= input[i] }
        });
        let remaining = crossing.unwrap_or(0);
        let distance = (i - remaining) as f64;
        if upward { distance } else { -distance }
    }).collect()
}

fn verify(input: &[f64]) {
    let mut distances = vec![f64::NAN; input.len()];
    let mut cumulative = vec![f64::NAN; input.len()];
    assert_eq!(MinusDistance::<f64>::default().transform(
        input.iter(), &mut distances, 0.0, ConvertToIdentity::default()
    ), TransformResult::Ok);
    assert_eq!(MinusDistanceSum::<f64>::default().transform(
        input.iter(), &mut cumulative, 0.0, ConvertToIdentity::default()
    ), TransformResult::Ok);
    let expected = reference(input);
    assert_eq!(distances, expected, "input={input:?}");
    let mut sum = 0.0;
    let expected_sum: Vec<_> = expected.into_iter().map(|v| { sum += v; sum }).collect();
    assert_eq!(cumulative, expected_sum, "input={input:?}");
}

#[test]
fn reference_comparison_exhaustive_short_sequences() {
    for len in 0..=8 {
        let combinations = 3_usize.pow(len);
        for mut code in 0..combinations {
            let mut input = Vec::with_capacity(len as usize);
            for _ in 0..len {
                input.push((code % 3) as f64 - 1.0);
                code /= 3;
            }
            verify(&input);
        }
    }
}

#[test]
fn reference_comparison_long_random_and_monotonic_sequences() {
    let mut seed = 0x1234_5678_u64;
    let mut data = Vec::new();
    for _ in 0..4096 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        data.push(((seed >> 32) % 11) as f64 - 5.0);
    }
    verify(&data);
    verify(&(0..4096).map(|x| x as f64).collect::<Vec<_>>());
    verify(&(0..4096).map(|x| -(x as f64)).collect::<Vec<_>>());
    verify(&vec![3.0; 4096]);
}

#[test]
fn mismatch_does_not_modify_output() {
    let input = [1.0, 2.0, 3.0];
    let mut out = [77.0; 2];
    assert_eq!(MinusDistance::<f64>::default().transform(
        input.iter(), &mut out, 0.0, ConvertToIdentity::default()
    ), TransformResult::Error(0));
    assert_eq!(out, [77.0; 2]);
    assert_eq!(MinusDistanceSum::<f64>::default().transform(
        input.iter(), &mut out, 0.0, ConvertToIdentity::default()
    ), TransformResult::Error(0));
    assert_eq!(out, [77.0; 2]);
}
