// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::selector::{PartlyLastN2, Selector, SelectorResult};

#[test]
fn partly_last_n2_partitions_intervals_and_selects_values() {
    let selector = PartlyLastN2::new_taillen(vec![1, 2, 1], 2);
    assert_eq!(selector.front_len, 2);
    assert_eq!(selector.tail_len, 3);
    assert_eq!(selector.process_len(), 5);
    assert_eq!(selector.len(), 4);
    assert!(!selector.is_empty());

    let input = [2, 3, 4, 5, 6];
    let mut output = [0; 4];
    assert_eq!(selector.populate(input.iter(), output.iter_mut()), SelectorResult::Ok);
    assert_eq!(output, [2, 3, 5, 6]);
}

#[test]
fn partly_last_n2_rejects_short_input_and_output() {
    let selector = PartlyLastN2::new_taillen(vec![1, 2], 1);
    let input = [1, 2, 3];
    let mut output = [0; 3];
    assert_eq!(selector.populate(input.iter(), output.iter_mut()), SelectorResult::Error(1));

    let mut short_output = [0; 2];
    assert_eq!(selector.populate(input.iter(), short_output.iter_mut()), SelectorResult::Error(0));
}

#[test]
#[should_panic]
fn partly_last_n2_rejects_zero_delta() {
    let _ = PartlyLastN2::new_taillen(vec![1, 0], 1);
}
