// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::selector:: { SelectReadIterator, SelectWriteIterator };

impl<'a, ITEM: 'a, T> SelectReadIterator<'a, ITEM> for T
where T:
      core::iter::DoubleEndedIterator<Item = &'a ITEM>
    + core::iter::ExactSizeIterator<Item = &'a ITEM>
    + Clone
{}

impl<'a, ITEM: 'a, T> SelectWriteIterator<'a, ITEM> for T where T:
      core::iter::DoubleEndedIterator<Item = &'a mut ITEM>
    + core::iter::ExactSizeIterator<Item = &'a mut ITEM>
{}

