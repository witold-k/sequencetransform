// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::transform::*;

impl<ITEMOUT> Default for Integrate<ITEMOUT> {
    fn default() -> Self {
        Integrate { marker: Default::default() }
    }
}

impl<ITEMOUT> Integrate<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        mut ri:       RI,
        out:          &mut [ITEMOUT],
        _default_out: ITEMOUT,
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
            let mut sum: ITEMOUT = conv.convert_ref(re);
            *we = sum;
            for (re, we) in ri.zip(wi) {
                sum += conv.convert_ref(re);
                *we = sum;
            }
        }

        TransformResult::Ok
    }
}


impl<ITEMOUT> SequenceTransform<ITEMOUT> for Integrate<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri: RI,
        out:         &mut [ITEMOUT],
        default_out: ITEMOUT,
        conv:        CTO
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri, out, default_out, conv)
    }
}

