// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::marker::PhantomData;
use std::fmt::Debug;
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, Div, Range };
use serde::{Deserialize, Serialize};
use num_traits::sign;
use num_traits::cast::{ AsPrimitive, FromPrimitive };

pub mod accumulate;
pub mod compoundnumber;
pub mod iterop;
pub mod select;
pub mod selector;
pub mod set;
pub mod tokenize;
pub mod transform;
pub mod ziptransform;
pub mod matrixtransform;

//

pub mod convertto;
pub mod distance;
pub mod iter;
pub mod number;
pub mod ordentry;
pub mod orderentry;
pub mod transformresult;

// ---------------------------------------------------------------------------

#[derive(Copy, Clone, PartialOrd, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub enum TransformResult {
    Ok,
    Error(u32)
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub enum TransformZipResult {
    Ok,
    Error(u32)
}

#[derive(Clone)]
pub struct ConvertToIdentity<T> {
    marker: PhantomData<T>,
}

// ---------------------------------------------------------------------------


/**
 * converts a object of Type A to an object of type B
 */
pub trait ConvertTo<A, B>:
    Clone
{
    fn convert_ref(&self, a: &A) -> B;

    fn convert(&self, a: A) -> B;
}

// ---------------------------------------------------------------------------

pub trait TransformReadIterator:
      core::iter::Iterator
    + core::iter::DoubleEndedIterator
    + core::iter::ExactSizeIterator
    + Clone
{
//    type Item;
}

pub trait TransformWriteIterator:
      core::iter::Iterator
    + core::iter::DoubleEndedIterator
    + core::iter::ExactSizeIterator
{
//    fn fill(&mut self, obj: Self::Item)
//    where
//        Self::Item: Clone;
}

// ---------------------------------------------------------------------------

pub trait Number<ItemIn, ItemOut = ItemIn>:
    Copy + Clone +
    PartialOrd<ItemIn> + PartialEq<ItemIn> +
    FromPrimitive +
    AsPrimitive<usize> + AsPrimitive<isize> +
    AsPrimitive<u32> + AsPrimitive<i32> +
    AsPrimitive<u16> + AsPrimitive<i16> +
    AsPrimitive<u8> + AsPrimitive<i8> +
    AsPrimitive<f32> + AsPrimitive<f64> +
    AddAssign<ItemIn> + SubAssign<ItemIn> +
    sign::Signed +
    Add<ItemIn, Output = ItemOut> + Sub<ItemIn, Output = ItemOut> +
    Mul<ItemIn, Output = ItemOut> + Div<ItemIn, Output = ItemOut> +
    Debug
{}

pub trait UNumber<ItemIn, ItemOut = ItemIn>:
    Copy + Clone +
    PartialOrd<ItemIn> + PartialEq<ItemIn> +
    FromPrimitive +
    AsPrimitive<usize> + AsPrimitive<isize> +
    AsPrimitive<u32> + AsPrimitive<i32> +
    AsPrimitive<u16> + AsPrimitive<i16> +
    AsPrimitive<u8> + AsPrimitive<i8> +
    AsPrimitive<f32> + AsPrimitive<f64> +
    AddAssign<ItemIn> + SubAssign<ItemIn> +
    Add<ItemIn, Output = ItemOut> + Sub<ItemIn, Output = ItemOut> +
    Mul<ItemIn, Output = ItemOut> + Div<ItemIn, Output = ItemOut> +
    Debug
{}

// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ItemRefIndex<ITEM>
where
    ITEM: Clone + Debug
{
    pub item:  ITEM,
    pub index: usize
}

#[derive(Clone, Debug)]
pub struct OrdEntry<ITEM>
where
    ITEM: Clone
{
    pub value:    ITEM
}

#[derive(Clone, Debug)]
pub struct OrderEntry<ITEM>
where
    ITEM: Clone
{
    pub position: u32,
    pub value:    ITEM
}

// ---------------------------------------------------------------------------

pub trait ULength<Idx>
{
    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;
}

impl<T> ULength<usize> for Vec<T> {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> ULength<usize> for [T] {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl ULength<Range<usize>> for str {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.len()
    }
}

