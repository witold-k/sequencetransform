// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingAverage, MovingAverageAverage, SequenceTransform};

fn check(data: &[f64], first_count: usize, second_count: usize) {
    let identity = ConvertToIdentity::<f64>::default();
    let mut intermediate = vec![0.0; data.len()];
    let mut expected = vec![0.0; data.len()];
    let mut actual = vec![0.0; data.len()];

    assert_eq!(
        MovingAverage::<f64>::new(first_count).transform(
            data.iter(), &mut intermediate, 0.0, identity.clone()
        ),
        TransformResult::Ok
    );
    assert_eq!(
        MovingAverage::<f64>::new(second_count).transform(
            intermediate.iter(), &mut expected, 0.0, identity.clone()
        ),
        TransformResult::Ok
    );
    assert_eq!(
        MovingAverageAverage::<f64>::new(first_count, second_count).transform(
            data.iter(), &mut actual, 0.0, identity
        ),
        TransformResult::Ok
    );

    for (index, (a, b)) in actual.iter().zip(expected.iter()).enumerate() {
        let tolerance = 1e-10 * (1.0 + b.abs());
        assert!(
            (a - b).abs() <= tolerance,
            "index {index}, windows ({first_count}, {second_count}): fused={a}, composed={b}"
        );
    }
}

#[test]
fn matches_two_separate_moving_averages_for_all_small_windows() {
    let cases: &[&[f64]] = &[
        &[1.0],
        &[1.0, 2.0],
        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
        &[8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0],
        &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        &[-8.0, 2.0, -3.0, 7.0, 0.0, 5.0, -4.0, 1.0],
        &[100.0, 100.02, 100.019, 100.04, 99.0, 101.0, 101.01, 100.0],
        &[1.0, 1.0, 1.0, 100.0, 1.0, 1.0, 1.0, 1.0],
    ];
    for data in cases {
        for first in 1..=data.len() {
            for second in 1..=data.len() {
                check(data, first, second);
            }
        }
    }
}

#[test]
fn matches_separate_pipeline_for_long_series_and_asymmetric_windows() {
    let data: Vec<f64> = (0..1000)
        .map(|i| {
            let trend = i as f64 * 0.003;
            let noise = ((i * 37 % 19) as f64 - 9.0) * 0.01;
            if i % 73 == 0 { trend + 4.0 } else { trend + noise }
        })
        .collect();
    for (first, second) in [(3, 20), (20, 3), (20, 20), (1, 100), (100, 1), (7, 31)] {
        check(&data, first, second);
    }
}

#[test]
fn f32_matches_separate_pipeline() {
    let data: Vec<f32> = (0..128)
        .map(|i| (i as f32 * 0.01) + (i * 13 % 11) as f32 * 0.005)
        .collect();
    let identity = ConvertToIdentity::<f32>::default();
    let mut intermediate = vec![0.0_f32; data.len()];
    let mut expected = vec![0.0_f32; data.len()];
    let mut actual = vec![0.0_f32; data.len()];
    assert_eq!(MovingAverage::<f32>::new(3).transform(data.iter(), &mut intermediate, 0.0, identity.clone()), TransformResult::Ok);
    assert_eq!(MovingAverage::<f32>::new(20).transform(intermediate.iter(), &mut expected, 0.0, identity.clone()), TransformResult::Ok);
    assert_eq!(MovingAverageAverage::<f32>::new(3, 20).transform(data.iter(), &mut actual, 0.0, identity), TransformResult::Ok);
    for (a, b) in actual.iter().zip(expected.iter()) {
        assert!((a - b).abs() <= 1e-4 * (1.0 + b.abs()), "fused={a}, composed={b}");
    }
}

#[test]
fn rejects_invalid_windows_and_lengths() {
    let data = [1.0, 2.0, 3.0];
    let identity = ConvertToIdentity::<f64>::default();
    for (first, second, output_len) in [(0, 1, 3), (1, 0, 3), (4, 1, 3), (1, 4, 3), (2, 2, 2)] {
        let mut output = vec![0.0; output_len];
        assert_eq!(
            MovingAverageAverage::<f64>::new(first, second).transform(
                data.iter(), &mut output, 0.0, identity.clone()
            ),
            TransformResult::Error(0)
        );
    }
}
