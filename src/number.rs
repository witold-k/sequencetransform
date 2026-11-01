// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::fmt::Debug;
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, Div };
use num_traits::cast::{ AsPrimitive, FromPrimitive };
use num_traits::sign;

use crate::Number;
use crate::UNumber;

impl<ItemIn, ItemOut, T> Number<ItemIn, ItemOut> for T
where T:
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

impl<ItemIn, ItemOut, T> UNumber<ItemIn, ItemOut> for T
where T:
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

