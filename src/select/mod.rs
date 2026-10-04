// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::marker::PhantomData;
use std::vec::Vec;
use crate::{ ConvertTo, ItemRefIndex, Number, OrdEntry };

pub mod max;
pub mod min;
pub mod median;

// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Max<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct Min<T> { marker: PhantomData<T> }

// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct Median<T>
where
    T: Clone
{
    marker: PhantomData<T>,
    border: usize,
    buffer: Vec<OrdEntry<T>>
}

// ---------------------------------------------------------------------------

pub trait ValueSelect<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{
    /**
     * return amout of iteration
     */
    fn select<'a, RI, ITEMIN, CTO>(
        &self,
        ri:   RI,
        conv: CTO,
    ) -> ItemRefIndex<ITEMOUT>
    where
        RI:     Iterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    ;
}

pub trait ValueSelectIn<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{
    /**
     * return amout of iteration
     */
    fn select<'a, RI, ITEMIN, CTO>(
        &self,
        ri1:  RI,
        ri2:  RI,
        conv: CTO,
    ) -> ItemRefIndex<ITEMOUT>
    where
        RI:     Iterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    ;
}

