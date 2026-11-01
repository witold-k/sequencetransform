// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

pub mod iter2eq;
pub mod iter2muteq;
pub mod iter2mutneq;
pub mod iter2neq;
pub mod itermatch;
pub mod iternmatch;
pub mod itersearchnextdown;
pub mod itersearchnextup;
pub mod itertilldown;
pub mod itertillup;

// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct IterSearchNextDown<ITER, ITEM>
{
    iter:      ITER,
    delta:     ITEM,
    pub count: usize
}

#[derive(Clone)]
pub struct IterSearchNextUp<ITER, ITEM> {
    pub iter:  ITER,
    delta:     ITEM,
    pub count: usize
}

#[derive(Clone)]
pub struct IterTillDown<ITER, ITEM> {
    pub iter: ITER,
    delta:    ITEM,
    last_val: ITEM
}

#[derive(Clone)]
pub struct IterTillUp<ITER, ITEM> {
    pub iter: ITER,
    delta:    ITEM,
    last_val: ITEM
}

#[derive(Clone)]
pub struct IterMatch<ITER, T>
{
    pub iter: ITER,
    select:   fn(&T) -> bool
}

#[derive(Clone)]
pub struct IterNMatch<ITER, T>
{
    pub iter: ITER,
    select:   fn(&T) -> bool
}

#[derive(Clone)]
pub struct Iter2Eq<ITER1, ITER2, T, U>
where
    U: PartialOrd<U> + PartialEq<U>
{
    select: fn(&T) -> &U,
    pub iter1: ITER1,
    pub iter2: ITER2
}

#[derive(Clone)]
pub struct Iter2MutEq<ITER1, ITER2, T, U>
where
    U: PartialOrd<U> + PartialEq<U>
{
    select: fn(&mut T) -> &mut U,
    pub iter1: ITER1,
    pub iter2: ITER2
}

#[derive(Clone)]
pub struct Iter2Neq<ITER1, ITER2, T, U>
where
    U: PartialOrd<U> + PartialEq<U>
{
    select: fn(&T) -> &U,
    pub iter1: ITER1,
    pub iter2: ITER2
}

#[derive(Clone)]
pub struct Iter2MutNeq<ITER1, ITER2, T, U>
where
    U: PartialOrd<U> + PartialEq<U>
{
    select: fn(&mut T) -> &mut U,
    pub iter1: ITER1,
    pub iter2: ITER2
}

