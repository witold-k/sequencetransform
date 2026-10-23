// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::selector::{SelectReadIterator, SelectWriteIterator};

fn require_read<'a, T: 'a, I: SelectReadIterator<'a, T>>(_iterator: I) {}
fn require_write<'a, T: 'a, I: SelectWriteIterator<'a, T>>(_iterator: I) {}

#[test]
fn slice_iterators_satisfy_selector_iterator_traits() {
    let mut values = [1.0_f64, 2.0, 3.0];
    require_read(values.iter());
    require_write(values.iter_mut());
}
