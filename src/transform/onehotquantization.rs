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
        let Some(out_len) = self.factor.checked_mul(len) else { return TransformResult::Error(0); };
        if self.factor == 0 || out_len != out.len() {
            return TransformResult::Error(0);
        }

        for (re, chunk) in ri.zip(out.chunks_exact_mut(facu)) {
            let re_f64: f64 = re.as_();
            if !re_f64.is_finite() { return TransformResult::Error(1); }
            let val = (re_f64 * fac64).round().clamp(0.0, (facu - 1) as f64);
            Self::push_one_hot(val as usize, facu, chunk);
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

