// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::{ AsPrimitive, FromPrimitive };

use crate::transform::*;

impl<ITEMOUT> OneHotQuantization<ITEMOUT> {
    pub fn new(factor: usize) -> Self {
        OneHotQuantization { marker: Default::default(), factor }
    }
}

impl<ITEMOUT> OneHotQuantization<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn push_one_hot(
        index: usize,
        len:   usize,
        out: &mut [ITEMOUT]
    ) {
        for item in out.iter_mut().take(index) {
            *item = FromPrimitive::from_u32(0u32).unwrap();
        }
        out[index] = FromPrimitive::from_u32(1u32).unwrap();
        for item in out.iter_mut().take(len).skip(index + 1) {
            *item = FromPrimitive::from_u32(0u32).unwrap();
        }
    }

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN>(
        &self,
        ri: RI,
        out: &mut [ITEMOUT],
    ) -> TransformResult
    where
        'a: 'b,
        RI: TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>
    {
        let len = ri.len();
        let fac64: f64 = self.factor.as_();
        let facu       = self.factor;
        let out_len = (fac64 * len as f64) as usize;
        if out_len != out.len() {
            return TransformResult::Error(0);
        }

        for re in ri {
            let re_f64: f64 = re.as_();
            let val: f64 = re_f64 * fac64;
            Self::push_one_hot(val.round() as usize, facu, out);
        }

        TransformResult::Ok
    }

    pub fn transform_self<'a, RI, ITEMIN>(
        &self,
        ri:           RI,
        out:          &mut [ITEMOUT],
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
    {
        self.transform_inline(ri, out)
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for OneHotQuantization<ITEMOUT>
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

