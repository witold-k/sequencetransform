// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::{ ConvertTo, ConvertToIdentity };

impl<A> Default for ConvertToIdentity<A>
{
    fn default() -> Self {
        Self { marker: Default::default() }
    }
}

impl<A> ConvertTo<A, A> for ConvertToIdentity<A>
where
    A: Clone
{
    #[inline(always)]
    fn convert_ref(&self, from: &A) -> A {
        from.clone()
    }

    #[inline(always)]
    fn convert(&self, from: A) -> A {
        from
    }
}

