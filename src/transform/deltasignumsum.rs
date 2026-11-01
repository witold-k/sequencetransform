// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::transform::*;

impl<ITEMOUT> Default for DeltaSignumSum<ITEMOUT> {
    fn default() -> Self {
        DeltaSignumSum { marker: Default::default() }
    }
}

impl<ITEMOUT> DeltaSignumSum<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN>(
        &self,
        mut ri:       RI,
        out:          &mut [ITEMOUT],
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

        let mut wi = out.iter_mut();
        if let Some(re) = ri.next() {
            let we = wi.next().unwrap();
            let mut last = *re;
            let mut sum: ITEMOUT = FromPrimitive::from_i8(0i8).unwrap();
            *we = FromPrimitive::from_u32(0u32).unwrap();
            for (re, we) in ri.zip(wi) {
                let current = *re;
                let out: i8 = if current > last {
                    1i8
                } else if current < last { -1i8 } else { 0i8 };
                sum += FromPrimitive::from_i8(out).unwrap();
                *we  = sum;
                last = current;
            }
        }

        TransformResult::Ok
    }

    pub fn transform_self<'a, RI, ITEMIN>(
        &self,
        ri:          RI,
        out:         &mut [ITEMOUT],
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
    {
        self.transform_inline(ri, out)
    }
}


impl<ITEMOUT> SequenceTransform<ITEMOUT> for DeltaSignumSum<ITEMOUT>
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

