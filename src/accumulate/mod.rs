// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::marker::PhantomData;
use crate::ConvertTo;
use crate::Number;

pub mod addall;
pub mod addallinc;
pub mod addfall;
pub mod addfallweigth;
pub mod addraise;
pub mod addraiseweigth;

// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct AddAll<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct AddFall<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct AddFallWeigth<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct AddAllInc<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct AddRaise<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct AddRaiseWeigth<T> { marker: PhantomData<T> }

// ---------------------------------------------------------------------------

pub trait SequenceAccumulate<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{
    /**
     * return amout of iteration
     */
    fn accumulate<'a, RI, ITEMIN, CTO>(
        &self,
        ri:   RI,
        conv: CTO
    ) -> Option<ITEMOUT>
    where
        RI:     Iterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    ;
}

