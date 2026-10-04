// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::ops;
use crate::*;

impl ops::Not for TransformResult {
    type Output = bool;

    #[inline(always)]
    fn not(self) -> bool {
        match self {
            TransformResult::Ok          => false,
            TransformResult::Error(_u32) => true
        }
    }
}

