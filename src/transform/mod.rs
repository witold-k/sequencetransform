// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::marker::PhantomData;
use std::vec::Vec;
use num_traits::cast::FromPrimitive;
use crate::{ Number, TransformReadIterator, TransformResult, ConvertTo };

pub mod deltasignum;
pub mod deltasignumsum;
pub mod differentiate;
pub mod heavyside;
pub mod integrate;
pub mod minusdistance;
pub mod minusdistancesum;
pub mod movingaverage;
pub mod movingmedian;
pub mod movingpitch;
pub mod onehotquantization;
pub mod order;
pub mod relativestrength;
pub mod relativestrengthweigth;
pub mod relativestrengthweigthsum;
pub mod signum;
pub mod signumsum;

// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct DeltaSignum<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct DeltaSignumSum<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct Differentiate<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct Heavyside<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct Integrate<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct MinusDistance<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct MinusDistanceSum<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct MovingAverage<T> { marker: PhantomData<T>, pub count: usize }

#[derive(Copy, Clone)]
pub struct MovingMedian<T> { marker: PhantomData<T>, pub count: usize, pub border: usize }

#[derive(Copy, Clone)]
pub struct MovingPitch<T> { marker: PhantomData<T>, pub count: usize, pub skip_first: bool }

#[derive(Copy, Clone)]
pub struct OneHotQuantization<T> { marker: PhantomData<T>, pub factor: usize }

#[derive(Copy, Clone)]
pub struct Order<T>
where T : Number<T> { pub factor: T }

#[derive(Copy, Clone)]
pub struct RelativeStrength<T> { marker: PhantomData<T>, pub count: usize, pub skip_first: bool }

#[derive(Copy, Clone)]
pub struct RelativeStrengthWeigth<T> { marker: PhantomData<T>, pub count: usize, pub skip_first: bool }

#[derive(Copy, Clone)]
pub struct RelativeStrengthWeigthSum<T> { marker: PhantomData<T>, pub count: usize, pub skip_first: bool }

#[derive(Copy, Clone)]
pub struct Signum<T> { marker: PhantomData<T> }

#[derive(Copy, Clone)]
pub struct SignumSum<T> { marker: PhantomData<T> }

// ---------------------------------------------------------------------------

pub trait SequenceTransform<ITEMOUT>
where
    ITEMOUT: 'static + Copy + FromPrimitive
{
    /**
     * return amout of iteration
     */
    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri:          RI,
        out:         &mut [ITEMOUT],
        default_out: ITEMOUT,
        conv:        CTO
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    ;
}

