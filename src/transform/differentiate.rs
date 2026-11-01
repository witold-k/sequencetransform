// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::transform::*;

impl<ITEMOUT> Default for Differentiate<ITEMOUT> {
    fn default() -> Self {
        Differentiate { marker: Default::default() }
    }
}

impl<ITEMOUT> Differentiate<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        mut ri:       RI,
        out:          &mut [ITEMOUT],
        conv:         CTO
    ) -> TransformResult
    where
        'a: 'b,
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        let len = ri.len();
        if len != out.len() {
            return TransformResult::Error(0);
        }

        let mut wi = out.iter_mut();
        if let Some(re) = ri.next() {
            let we = wi.next().unwrap();
            let mut last = *re;
            *we = FromPrimitive::from_u32(0u32).unwrap();
            for (re, we) in ri.zip(wi) {
                let current = *re;
                *we = conv.convert(current - last);
                last = current;
            }
        }

        TransformResult::Ok
    }

    pub fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri:           RI,
        out:          &mut [ITEMOUT],
        _default_out: ITEMOUT,
        conv:         CTO
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri, out, conv)
    }
}


impl<ITEMOUT> SequenceTransform<ITEMOUT> for Differentiate<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri:           RI,
        out:          &mut [ITEMOUT],
        _default_out: ITEMOUT,
        conv:         CTO
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri, out, conv)
    }
}

