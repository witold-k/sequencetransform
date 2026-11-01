// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::transform::*;

impl<ITEMOUT> Default for Signum<ITEMOUT> {
    fn default() -> Self {
        Signum { marker: Default::default() }
    }
}

impl<ITEMOUT> Signum<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN>(
        &self,
        ri:  RI,
        out: &mut [ITEMOUT],
    ) -> TransformResult
    where
        'a: 'b,
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
    {
        let len = ri.len();
        if len != out.len() {
            return TransformResult::Error(0);
        }

        let wi = out.iter_mut();
        for (re, we) in ri.zip(wi) {
            let current = *re;
            let out: i8 = if current > FromPrimitive::from_u32(0u32).unwrap() {
                1i8
            } else if current < FromPrimitive::from_u32(0u32).unwrap() { -1i8 } else { 0i8 };

            *we  = FromPrimitive::from_i8(out).unwrap();
        }

        TransformResult::Ok
    }

    pub fn transform_self<'a, RI, ITEMIN>(
        &self,
        ri:  RI,
        out: &mut [ITEMOUT],
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
    {
        self.transform_inline(ri, out)
    }
}


impl<ITEMOUT> SequenceTransform<ITEMOUT> for Signum<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri:           RI,
        out:          &mut [ITEMOUT],
        _default_out: ITEMOUT,
        _conv:        CTO
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri, out)
    }
}

