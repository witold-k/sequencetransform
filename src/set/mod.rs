// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

pub mod retain_property;
pub mod retain_sorted_left;
pub mod retain_sorted_all;

use std::cmp::{ PartialOrd, PartialEq };

/**
 * keeps (retain) only elements that matches a property,
 * removes all elements of two sorted sequences
 * represented as iterators that do not match a property
 * prepersented as select function
 */
pub struct RetainProperty<T>
{
    select: fn(&T) -> bool
}

/**
 * keeps (retain) only equal elements,
 * removes all elements of two sorted sequences
 * represented as iterators that are not inside
 * both sequences. before the comparision the elemenrs
 * is selected or transformed by the select function
 */
pub struct RetainSortedLeft<T, U>
where
    T: Clone,
    U: PartialOrd<U> + PartialEq<U>
{
    select:     fn(&T) -> &U,
}

/**
 * S is a datastructure that provides a possibility
 * to obtain &mut S -> &mut Vec<T>
 * &T must provide to get &S oz of T
 */
pub struct RetainSortedAll<S, T, U>
where
    T: Clone,
    U: PartialOrd<U> + PartialEq<U>
{
    select_vec: fn(&mut S) -> &mut Vec::<T>,
    select:     fn(&T) -> &U,
}
