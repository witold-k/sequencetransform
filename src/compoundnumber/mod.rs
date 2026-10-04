// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use serde::{Deserialize, Serialize};

pub mod n_uint_in_u64;
pub mod u8x4;
pub mod u8x8;

/**
 * param COUNT amount of numbers are stored in number
 */
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct NUintInU64<const COUNT: usize, const BITCOUNT: usize> {
    pub value: u64
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub struct U8x4 {
    pub index: [u8; 4]
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub struct U8x8 {
    pub index: [u8; 8]
}


