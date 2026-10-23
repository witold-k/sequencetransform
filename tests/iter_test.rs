// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{TransformReadIterator, TransformWriteIterator};

fn require_read_iterator<T: TransformReadIterator>(_iterator: T) {}
fn require_write_iterator<T: TransformWriteIterator>(_iterator: T) {}

#[test]
fn standard_slice_iterators_implement_transform_iterator_traits() {
    let mut data = [1.0_f64, 2.0, 3.0];
    require_read_iterator(data.iter());
    require_write_iterator(data.iter_mut());
}

#[test]
fn cloned_read_iterator_has_independent_position() {
    let data = [1.0_f64, 2.0, 3.0];
    let mut first = data.iter();
    let mut second = first.clone();
    assert_eq!(first.next(), Some(&1.0));
    assert_eq!(second.next(), Some(&1.0));
    assert_eq!(second.next_back(), Some(&3.0));
    assert_eq!(first.len(), 2);
    assert_eq!(second.len(), 1);
}
