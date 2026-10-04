// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::marker::PhantomData;
use crate::{ ConvertTo, Number, TransformReadIterator, TransformResult };

pub mod atlastsignchange;
pub mod atlastsignchangetopos;
pub mod atlastsignchangetoneg;
pub mod selectedaccumulation_border;
pub mod selectedaccumulation;
pub mod selectedsyncaccumulation;
pub mod selectedsyncaccumulation_border;

// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct AtLastSignChange<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct AtLastSignChangeToPos<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct AtLastSignChangeToNeg<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct SelectedAccumulation<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct SelectedAccumulationBorder<T> { border: T }

#[derive(Copy, Clone)]
pub struct SelectedSyncAccumulation<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct SelectedSyncAccumulationBorder<T> { border: T }


// ---------------------------------------------------------------------------

pub trait ZipSequenceTransform<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{
    /**
     * return amout of iteration
     */
    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri1:        RI,
        ri_trigger: RI,
        out:        &mut [ITEMOUT],
        default_in: ITEMIN,
        conv:       CTO
    ) -> Result<usize, TransformResult>
    where
        RI: TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    ;
}

