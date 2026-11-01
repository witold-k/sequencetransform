// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate:: { TransformReadIterator, TransformWriteIterator };

impl<T> TransformReadIterator for T where T:
      core::iter::Iterator
    + core::iter::DoubleEndedIterator
    + core::iter::ExactSizeIterator
    + Clone
{
//    type Item = <Self as Iterator>::Item;
}

impl<T> TransformWriteIterator for T where T:
      core::iter::Iterator
    + core::iter::DoubleEndedIterator
    + core::iter::ExactSizeIterator
{
//    type Item = <Self as Iterator>::Item;
}

