// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MaxDirection, SequenceTransform};

fn run(data: &[f64], count: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    assert_eq!(
        MaxDirection::<f64>::new(count).transform(
            data.iter(), &mut out, 0.0, ConvertToIdentity::default(),
        ),
        TransformResult::Ok
    );
    out
}

#[test]
fn nearer_minimum_produces_upward_slope() {
    assert_eq!(run(&[10.0, 3.0, 7.0, 9.0], 3), vec![0.0, -7.0, 4.0, 3.0]);
}

#[test]
fn nearer_maximum_produces_downward_slope() {
    assert_eq!(run(&[1.0, 8.0, 6.0, 4.0], 3), vec![0.0, 7.0, -2.0, -2.0]);
}

#[test]
fn count_limits_lookback_and_prefixes_use_available_history() {
    assert_eq!(run(&[1.0, 10.0, 4.0, 6.0], 2), vec![0.0, 9.0, -6.0, 2.0]);
}

#[test]
fn repeated_extrema_select_latest_occurrence() {
    assert_eq!(run(&[5.0, 1.0, 5.0, 1.0, 3.0], 4),
               vec![0.0, -4.0, 4.0, -4.0, 2.0]);
}

#[test]
fn constant_series_and_singleton_have_zero_slope() {
    assert_eq!(run(&[2.0, 2.0, 2.0], 3), vec![0.0; 3]);
    assert_eq!(run(&[7.0], 1), vec![0.0]);
}

#[test]
fn invalid_parameters_leave_output_untouched() {
    let data = [1.0, 2.0];
    let mut out = [55.0, 55.0];
    assert_eq!(
        MaxDirection::<f64>::new(0).transform(
            data.iter(), &mut out, 0.0, ConvertToIdentity::default()
        ),
        TransformResult::Error(0)
    );
    assert_eq!(out, [55.0, 55.0]);

    let mut wrong_length = [55.0];
    assert_eq!(
        MaxDirection::<f64>::new(2).transform(
            data.iter(), &mut wrong_length, 0.0, ConvertToIdentity::default()
        ),
        TransformResult::Error(0)
    );
}

#[test]
fn optimized_matches_reference_for_various_windows() {
    fn reference(data: &[f64], count: usize) -> Vec<f64> {
        (0..data.len()).map(|i| {
            if i == 0 { return 0.0; }
            let start = i.saturating_sub(count);
            let mut min = start;
            let mut max = start;
            for j in start + 1..i {
                if data[j] <= data[min] { min = j; }
                if data[j] >= data[max] { max = j; }
            }
            let nearest = min.max(max);
            (data[i] - data[nearest]) / (i - nearest) as f64
        }).collect()
    }

    let mut data = Vec::new();
    let mut seed = 0x1234_5678_u64;
    for i in 0..1000 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        data.push(((seed >> 32) % 17) as f64 - 8.0 + (i % 3) as f64);
    }
    for count in [1, 2, 3, 7, 31, 128, 1000, 2000] {
        assert_eq!(run(&data, count), reference(&data, count), "window={count}");
    }
}

#[test]
fn monotonic_sequences_and_expired_extrema() {
    assert_eq!(run(&[1.0, 2.0, 3.0, 4.0, 5.0], 3), vec![0.0, 1.0, 1.0, 1.0, 1.0]);
    assert_eq!(run(&[5.0, 4.0, 3.0, 2.0, 1.0], 3), vec![0.0, -1.0, -1.0, -1.0, -1.0]);
    assert_eq!(run(&[100.0, 1.0, 2.0, 3.0, 4.0], 2), vec![0.0, -99.0, 1.0, 1.0, 1.0]);
}
