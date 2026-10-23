// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::cmp::Ordering;
use sequencetransform::OrdEntry;

#[test]
fn comparisons_use_wrapped_value() {
    let low = OrdEntry { value: 2.0_f64 };
    let high = OrdEntry { value: 5.0_f64 };
    let same = OrdEntry { value: 2.0_f64 };
    assert_eq!(low.partial_cmp(&high), Some(Ordering::Less));
    assert_eq!(high.cmp(&low), Ordering::Greater);
    assert_eq!(low.cmp(&same), Ordering::Equal);
    assert_eq!(low, same);
}

#[test]
#[should_panic]
fn unordered_floating_point_comparison_panics() {
    let nan = OrdEntry { value: f64::NAN };
    let finite = OrdEntry { value: 1.0_f64 };
    let _ = nan.cmp(&finite);
}
