// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::accumulate::rise_max8::rise_max8;

#[test]
fn collects_row_indices_per_column() {
    let result = rise_max8([[1.0, 0.0, 1.0], [0.0, 1.0, 1.0], [1.0, 0.0, 0.0]]);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].size(), 2);
    assert!(result[0].has(0) && result[0].has(2));
    assert_eq!(result[1].size(), 1);
    assert!(result[1].has(1));
    assert_eq!(result[2].size(), 2);
    assert!(result[2].has(0) && result[2].has(1));
}

#[test]
fn empty_rows_and_empty_input() {
    assert!(rise_max8(Vec::<Vec<f32>>::new()).is_empty());
    assert_eq!(rise_max8([vec![0.0, 0.0]]).len(), 2);
    assert!(rise_max8([vec![0.0, 0.0]]).iter().all(|x| x.empty()));
}

#[test]
#[should_panic(expected = "inconsistent row width")]
fn rejects_ragged_rows() {
    rise_max8([vec![1.0, 0.0], vec![1.0]]);
}

#[test]
#[should_panic(expected = "more than eight matches")]
fn rejects_more_than_eight_matches() {
    rise_max8(vec![vec![1.0]; 9]);
}

#[test]
#[should_panic(expected = "row index 255 is reserved")]
fn rejects_reserved_row_index() {
    rise_max8(vec![vec![0.0]; 256]);
}
