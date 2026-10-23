// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::sort_small::{sort2, sort3, Median3};

#[test]
fn sort2_handles_both_orders_and_equal_values() {
    assert_eq!(sort2(3, 1), (1, 3));
    assert_eq!(sort2(1, 3), (1, 3));
    assert_eq!(sort2(2, 2), (2, 2));
}

#[test]
fn sort3_handles_all_permutations() {
    for [a, b, c] in [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]] {
        assert_eq!(sort3(a, b, c), (1, 2, 3));
    }
    assert_eq!(sort3(2, 2, 1), (1, 2, 2));
}

#[test]
fn median3_keeps_last_three_samples() {
    let median = Median3::new(10);
    assert_eq!(median.median(), 10);
    let median = median.push(1);
    assert_eq!(median.median(), 10);
    let median = median.push(3);
    assert_eq!(median.median(), 3);
    let median = median.push(8);
    assert_eq!(median.median(), 3);
    let median = median.push(9);
    assert_eq!(median.median(), 8);
}
