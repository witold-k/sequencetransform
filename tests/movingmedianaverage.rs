// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingAverage, MovingMedian, MovingMedianAverage, SequenceTransform};

fn check(data: &[f64], median_count: usize, average_count: usize) {
    let identity = ConvertToIdentity::<f64>::default();
    let mut medians = vec![0.0; data.len()];
    let mut expected = vec![0.0; data.len()];
    let mut actual = vec![0.0; data.len()];

    assert_eq!(
        MovingMedian::<f64>::new(median_count).transform(
            data.iter(), &mut medians, 0.0, identity.clone()
        ),
        TransformResult::Ok
    );
    assert_eq!(
        MovingAverage::<f64>::new(average_count).transform(
            medians.iter(), &mut expected, 0.0, identity.clone()
        ),
        TransformResult::Ok
    );
    assert_eq!(
        MovingMedianAverage::<f64>::new(median_count, average_count).transform(
            data.iter(), &mut actual, 0.0, identity
        ),
        TransformResult::Ok
    );

    for (index, (a, b)) in actual.iter().zip(expected.iter()).enumerate() {
        let tolerance = 1e-10 * (1.0 + b.abs());
        assert!(
            (a - b).abs() <= tolerance,
            "index {index}: fused={a}, composed={b}, tolerance={tolerance}"
        );
    }
}

#[test]
fn equivalent_to_separate_median_then_average() {
    let cases: &[&[f64]] = &[
        &[5.0, 1.0, 9.0, 2.0, 8.0, 3.0, 7.0, 4.0],
        &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        &[100.0, 100.02, 100.019, 100.04, 99.0, 101.0, 101.01, 100.0],
        &[-5.0, -2.0, 0.0, -1.0, 7.0, 7.0, -9.0, 2.0],
        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
    ];
    for data in cases {
        for median_count in 1..=data.len() {
            for average_count in 1..=data.len() {
                check(data, median_count, average_count);
            }
        }
    }
}

#[test]
fn equivalent_for_month_window_and_deterministic_data() {
    let data: Vec<f64> = (0..100)
        .map(|i| {
            let base = (i as f64) * 0.01;
            let jitter = ((i * 37 % 17) as f64 - 8.0) * 0.003;
            if i % 23 == 0 { base + 5.0 } else { base + jitter }
        })
        .collect();
    check(&data, 3, 20);
    check(&data, 5, 20);
}

#[test]
fn rejects_invalid_lengths_and_windows() {
    let data = [1.0, 2.0, 3.0];
    let identity = ConvertToIdentity::<f64>::default();
    for (m, a, out_len) in [(0, 2, 3), (2, 0, 3), (4, 2, 3), (2, 4, 3), (2, 2, 2)] {
        let mut output = vec![0.0; out_len];
        assert_eq!(
            MovingMedianAverage::<f64>::new(m, a).transform(
                data.iter(), &mut output, 0.0, identity.clone()
            ),
            TransformResult::Error(0)
        );
    }
}
