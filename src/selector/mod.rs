// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use serde::{Deserialize, Serialize};

pub mod iter;
pub mod lastn;
pub mod partlylastn;
pub mod partlylastn2;
pub mod processor;

// ---------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub struct LastN {
    pub count: usize
}

#[derive(Clone, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub struct PartlyLastN {
    pub delta:        Vec<usize>,
    pub process_size: usize
}

#[derive(Clone, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub struct PartlyLastN2 {
    pub data:         PartlyLastN,
    pub front_len:    usize,
    pub tail_len:     usize
}

// ---------------------------------------------------------------------------

pub struct SimpleProcessor<RI, SELECT>
{
    pub read_iterator: RI,
    pub selector:      SELECT
}

pub struct SimpleRefProcessor<'a, RI, SELECT: 'a>
{
    pub read_iterator: RI,
    pub selector:      &'a SELECT
}

// ---------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub enum SelectorResult {
    Ok,
    Error(u32)
}

// ---------------------------------------------------------------------------

pub trait SelectReadIterator<'a, ITEM: 'a>:
      core::iter::DoubleEndedIterator<Item = &'a ITEM>
    + core::iter::ExactSizeIterator<Item = &'a ITEM>
    + Clone
{}

pub trait SelectWriteIterator<'a, ITEM: 'a>:
      core::iter::DoubleEndedIterator<Item = &'a mut ITEM>
    + core::iter::ExactSizeIterator<Item = &'a mut ITEM>
{}

// ---------------------------------------------------------------------------

pub trait Selector
{
    /**
     * return amout of iteration
     */
    fn populate<'a, 'b, RI, WI, ITEM>(
        &self,
        ri: RI,
        wi: WI
    ) -> SelectorResult
    where
        'a: 'b,
        RI: SelectReadIterator<'a, ITEM>,
        WI: SelectWriteIterator<'b, ITEM>,
        ITEM: 'a + Clone
    ;

    fn process_len(&self) -> usize;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}

pub trait Processor<'a, ITEM: 'a>
{
    /**
     * return amout of iteration
     */
    fn populate<'b, WI>(
        &mut self,
        wi: WI
    ) -> bool
    where
        'a: 'b,
        WI: 'b + SelectWriteIterator<'b, ITEM>,
        ITEM: Clone
    ;

    fn process_len(&self) -> usize;
}

