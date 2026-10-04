// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::cmp;
use std::ops;
use std::fmt;
use num_traits::cast::FromPrimitive;
use itoa::Buffer;
use crate::distance::*;
use crate::compoundnumber::*;

impl<const COUNT: usize, const BITCOUNT: usize> FromPrimitive for NUintInU64<COUNT, BITCOUNT> {

    /// Converts an `isize` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_isize(n: isize) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    /// Converts an `i8` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    /// Converts an `i16` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    /// Converts an `i32` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    /// Converts an `i64` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    fn from_i64(n: i64) -> Option<Self>  {
        Some(Self { value: n as u64 })
    }

    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    /// Converts a `usize` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_usize(n: usize) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    /// Converts an `u8` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    /// Converts an `u16` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    /// Converts an `u32` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    /// Converts an `u64` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    fn from_u64(n: u64) -> Option<Self> {
        Some(Self { value: n })
    }

    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    /// Converts a `f32` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        Some(Self { value: n as u64 })
    }

    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        Some(Self { value: n as u64 })
    }
}


impl<const COUNT: usize, const BITCOUNT: usize> NUintInU64<COUNT, BITCOUNT>
{
    #[inline(always)]
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub fn get(self, value: usize) -> u64 {
        let mask: u64 = !(u64::MAX << BITCOUNT);
        (self.value >> (value * BITCOUNT)) & mask
    }

    #[inline(always)]
    pub fn distance_inline(self, other: NUintInU64<COUNT, BITCOUNT>) -> u32 {
        (self.value ^ other.value).count_ones()
    }

    /// treats n uints as function and
    /// differentaties it = f(i) - f(i - 1); f'(0) = 0
    #[inline(always)]
    pub fn delta_inline(self) -> Self {
        let mask: u64 = !(u64::MAX << BITCOUNT);
        let mut shift: usize = BITCOUNT;
        let mut prev: u64 = self.value & mask;
        let mut part: u64 = 0;
        for _ in 1 .. COUNT {
            let next = self.value >> shift;
            let part_res = next.wrapping_sub(prev) & mask;
            prev = next;
            part |= part_res << shift;
            shift += BITCOUNT;
        }
        Self { value: part }
    }

    #[inline(always)]
    pub fn to_n_f32_slice(self, slice: &mut [f32; COUNT]) {
        for (i, item) in slice.iter_mut().enumerate() {
            *item = self.get(i) as f32;
        }
    }

}

impl<const COUNT: usize, const BITCOUNT: usize> Distance<NUintInU64<COUNT, BITCOUNT>, u32>
for NUintInU64<COUNT, BITCOUNT>
{
    #[inline(always)]
    fn distance(self, other: NUintInU64<COUNT, BITCOUNT>) -> u32 {
        self.distance_inline(other)
    }
}

impl<const COUNT: usize, const BITCOUNT: usize> Delta<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    #[inline(always)]
    fn delta(self) -> NUintInU64<COUNT, BITCOUNT> {
        self.delta_inline()
    }
}

// ---------------------------------------------------------------------------

impl<const COUNT: usize, const BITCOUNT: usize> ops::Not
for NUintInU64<COUNT, BITCOUNT>
{
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self { value: !self.value }
    }
}

// ---------------------------------------------------------------------------

impl<const COUNT: usize, const BITCOUNT: usize> ops::BitAndAssign<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl<const COUNT: usize, const BITCOUNT: usize> ops::BitAnd<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    type Output = NUintInU64<COUNT, BITCOUNT>;

    #[inline(always)]
    fn bitand(self, other: Self) -> Self {
        Self { value: self.value & other.value }
    }
}

impl<const COUNT: usize, const BITCOUNT: usize> ops::BitOrAssign<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl<const COUNT: usize, const BITCOUNT: usize> ops::BitXor<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    type Output = NUintInU64<COUNT, BITCOUNT>;

    #[inline(always)]
    fn bitxor(self, other: Self) -> Self {
        Self { value: self.value ^ other.value }
    }
}

impl<const COUNT: usize, const BITCOUNT: usize> ops::BitXorAssign<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }
}

impl<const COUNT: usize, const BITCOUNT: usize> ops::BitOr<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    type Output = NUintInU64<COUNT, BITCOUNT>;

    #[inline(always)]
    fn bitor(self, other: Self) -> Self {
        Self { value: self.value | other.value }
    }
}

// ---------------------------------------------------------------------------

impl<const COUNT: usize, const BITCOUNT: usize> ops::Neg
for NUintInU64<COUNT, BITCOUNT>
{
    type Output = NUintInU64<COUNT, BITCOUNT>;

    #[inline(always)]
    fn neg(self) -> Self {
        let mask: u64 = !(u64::MAX << BITCOUNT);
        let mut shift: usize = BITCOUNT;
        let a = self.value as i64;
        let mut part: u64 = (-a as u64) & mask;
        for _ in 1 .. COUNT {
            let a = (self.value >> shift) as i64;
            part |= ((-a as u64) & mask) << shift;
            shift += BITCOUNT;
        }
        Self { value: part }
    }
}

// ---------------------------------------------------------------------------

impl<const COUNT: usize, const BITCOUNT: usize> ops::Add<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    type Output = NUintInU64<COUNT, BITCOUNT>;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        let mask: u64 = !(u64::MAX << BITCOUNT);
        let mut shift: usize = BITCOUNT;
        let mut part: u64 = self.value.wrapping_add(other.value) & mask;
        for _ in 1 .. COUNT {
            let a = self.value >> shift;
            let b = other.value >> shift;
            let part_res = a.wrapping_add(b) & mask;
            part |= part_res << shift;
            shift += BITCOUNT;
        }
        Self { value: part }
    }
}

impl<const COUNT: usize, const BITCOUNT: usize> ops::AddAssign<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.value = (*self + other).value;
    }
}

// ---------------------------------------------------------------------------

impl<const COUNT: usize, const BITCOUNT: usize> ops::Sub<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    type Output = NUintInU64<COUNT, BITCOUNT>;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        let mask: u64 = !(u64::MAX << BITCOUNT);
        let mut shift: usize = BITCOUNT;
        let mut part: u64 = self.value.wrapping_sub(other.value) & mask;
        for _ in 1 .. COUNT {
            let a = self.value >> shift;
            let b = other.value >> shift;
            let part_res = a.wrapping_sub(b) & mask;
            part |= part_res << shift;
            shift += BITCOUNT;
        }
        Self { value: part }
    }
}

impl<const COUNT: usize, const BITCOUNT: usize> ops::SubAssign<NUintInU64<COUNT, BITCOUNT>>
for NUintInU64<COUNT, BITCOUNT>
{
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.value = (*self - other).value;
    }
}

// ---------------------------------------------------------------------------

impl<const COUNT: usize, const BITCOUNT: usize> cmp::PartialEq
for NUintInU64<COUNT, BITCOUNT>
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<const COUNT: usize, const BITCOUNT: usize> cmp::Eq
for NUintInU64<COUNT, BITCOUNT> {}

impl<const COUNT: usize, const BITCOUNT: usize> std::hash::Hash
for NUintInU64<COUNT, BITCOUNT>
{
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let v32 = (self.value << 32) ^ (self.value & 0xFFFFFFFFu64);
        state.write_u32(v32 as u32);
    }
}

// ---------------------------------------------------------------------------

impl<const COUNT: usize, const BITCOUNT: usize> std::fmt::Debug
for NUintInU64<COUNT, BITCOUNT>
{

    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut expr = f.debug_tuple("NUintInU64");
        for i in (0 .. COUNT).rev() {
            expr.field(&(*self).get(i));
        }
        expr.finish()
    }
}

impl<const COUNT: usize, const BITCOUNT: usize> fmt::Display
for NUintInU64<COUNT, BITCOUNT>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = Buffer::new();
        let mut s = buffer.format((*self).get(COUNT - 1));
        match f.write_str(s) {
            Ok(_) => {
                for i in (1 .. COUNT).rev() {
                    match f.write_str(", ") {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                    s = buffer.format((*self).get(i));
                    match f.write_str(s) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }

                }
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

