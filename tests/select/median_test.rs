// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::select::{Median, ValueSelect};
use sequencetransform::ConvertToIdentity;

#[test]
fn median_selects_sorted_border_value() {
    let input = [9.0_f64, -2.0, 5.0, 1.0, 5.0];
    for (border, expected) in [(0, -2.0), (1, 1.0), (2, 5.0), (3, 5.0), (4, 9.0)] {
        let selector = Median::new_len(5, border, 0.0_f64);
        let result = selector.select(input.iter(), ConvertToIdentity::default());
        assert_eq!(result.item, expected);
        assert_eq!(result.index, border);
    }
}

#[test]
fn median_ignores_items_beyond_window() {
    let input = [4.0_f32, 2.0, 3.0, -100.0];
    let selector = Median::new_len(3, 1, 0.0_f32);
    assert_eq!(selector.select(input.iter(), ConvertToIdentity::default()).item, 3.0);
}

#[test]
#[should_panic(expected = "invalid median window")]
fn median_rejects_empty_window() {
    let _ = Median::new_len(0, 0, 0.0_f64);
}

#[test]
#[should_panic(expected = "invalid median window")]
fn median_rejects_border_outside_window() {
    let _ = Median::new_len(3, 3, 0.0_f64);
}

#[test]
#[should_panic(expected = "median input shorter than window")]
fn median_rejects_short_input() {
    let selector = Median::new_len(3, 1, 0.0_f64);
    let input = [1.0_f64, 2.0];
    let _ = selector.select(input.iter(), ConvertToIdentity::default());
}

#[test]
#[should_panic(expected = "unordered median value")]
fn median_rejects_nan() {
    let selector = Median::new_len(3, 1, 0.0_f64);
    let input = [1.0_f64, f64::NAN, 2.0];
    let _ = selector.select(input.iter(), ConvertToIdentity::default());
}
