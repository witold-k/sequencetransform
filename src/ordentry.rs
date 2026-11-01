// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::cmp::Ordering;
use std::cmp::{ PartialOrd, PartialEq };

use crate::*;

impl<ITEM: Clone + PartialEq> PartialEq for OrdEntry<ITEM> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<ITEM: Clone + PartialOrd> PartialOrd for OrdEntry<ITEM>
{
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<ITEM: Clone + PartialOrd> Ord for OrdEntry<ITEM> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl<ITEM: Clone + PartialEq> Eq for OrdEntry<ITEM> {}

