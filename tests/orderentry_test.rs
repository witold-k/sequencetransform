// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::cmp::Ordering;
use sequencetransform::OrderEntry;

#[test]
fn comparisons_use_values_not_positions() {
    let low = OrderEntry { position: 99, value: 2.0_f64 };
    let high = OrderEntry { position: 0, value: 5.0_f64 };
    let same = OrderEntry { position: 1, value: 2.0_f64 };
    assert_eq!(low.partial_cmp(&high), Some(Ordering::Less));
    assert_eq!(high.cmp(&low), Ordering::Greater);
    assert_eq!(low.cmp(&same), Ordering::Equal);
    assert_eq!(low, same);
}

#[test]
#[should_panic]
fn unordered_floating_point_comparison_panics() {
    let nan = OrderEntry { position: 0, value: f64::NAN };
    let finite = OrderEntry { position: 1, value: 1.0_f64 };
    let _ = nan.cmp(&finite);
}
