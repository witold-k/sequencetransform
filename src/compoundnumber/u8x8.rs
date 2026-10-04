// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::option::Option;
use crate::compoundnumber::*;

impl Default for U8x8 {
    #[inline(always)]
    fn default() -> U8x8 {
        U8x8 { index: [
            Self::FREE, Self::FREE, Self::FREE, Self::FREE,
            Self::FREE, Self::FREE, Self::FREE, Self::FREE
        ] }
    }
}

impl U8x8 {
    pub const FREE:     u8  = 0xFF;
    pub const FREE_ALL: u64 = 0xFFFFFFFF_FFFFFFFF;

    #[inline(always)]
    pub fn new_fill(val: u8) -> U8x8 {
        U8x8 { index: [
            val, val, val, val,
            val, val, val, val
        ] }
    }

    #[inline(always)]
    pub fn free_index(&self) -> Option<u8> {
        if self.index[0] == Self::FREE { return Some(0); }
        if self.index[1] == Self::FREE { return Some(1); }
        if self.index[2] == Self::FREE { return Some(2); }
        if self.index[3] == Self::FREE { return Some(3); }
        if self.index[4] == Self::FREE { return Some(4); }
        if self.index[5] == Self::FREE { return Some(5); }
        if self.index[6] == Self::FREE { return Some(6); }
        if self.index[7] == Self::FREE { return Some(7); }
        None
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        if self.index[0] == Self::FREE { return 0; }
        if self.index[1] == Self::FREE { return 1; }
        if self.index[2] == Self::FREE { return 2; }
        if self.index[3] == Self::FREE { return 3; }
        if self.index[4] == Self::FREE { return 4; }
        if self.index[5] == Self::FREE { return 5; }
        if self.index[6] == Self::FREE { return 6; }
        if self.index[7] == Self::FREE { return 7; }
        8
    }

    #[inline(always)]
    pub fn as_u64(&self) -> u64 {
        ((self.index[0] as u64) << 56) |
        ((self.index[1] as u64) << 48) |
        ((self.index[2] as u64) << 40) |
        ((self.index[3] as u64) << 32) |
        ((self.index[4] as u64) << 24) |
        ((self.index[5] as u64) << 16) |
        ((self.index[6] as u64) <<  8) |
        (self.index[7] as u64)
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        Self::FREE_ALL == self.as_u64()
    }

    #[inline(always)]
    pub fn find(&self, val: u8) -> Option<u8> {
        if self.index[0] == val { return Some(0); }
        if self.index[1] == val { return Some(1); }
        if self.index[2] == val { return Some(2); }
        if self.index[3] == val { return Some(3); }
        if self.index[4] == val { return Some(4); }
        if self.index[5] == val { return Some(5); }
        if self.index[6] == val { return Some(6); }
        if self.index[7] == val { return Some(7); }
        None
    }

    #[inline(always)]
    pub fn has(&self, val: u8) -> bool {
        if self.index[0] == val { return true; }
        if self.index[1] == val { return true; }
        if self.index[2] == val { return true; }
        if self.index[3] == val { return true; }
        if self.index[4] == val { return true; }
        if self.index[5] == val { return true; }
        if self.index[6] == val { return true; }
        if self.index[7] == val { return true; }
        false
    }

    #[inline(always)]
    pub fn push(&mut self, val: u8) -> bool {
        if let Some(index) = self.free_index() {
            self.index[index as usize] = val;
            true
        }
        else {
            false
        }
    }

}

