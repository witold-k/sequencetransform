// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::sign;
use num_traits::cast::FromPrimitive;

use crate::ziptransform::*;

impl<ITEMOUT> Default for AtLastSignChange<ITEMOUT> {
    fn default() -> Self {
        AtLastSignChange { marker: Default::default() }
    }
}

impl<ITEMOUT> AtLastSignChange<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        mut ri1: RI,
        mut ri2: RI,
        out:     &mut [ITEMOUT],
        conv:    CTO
    ) -> Result<usize, TransformResult>
    where
        'a: 'b,
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        let len1 = ri1.len();
        if len1 != out.len() {
            return Err(TransformResult::Error(0));
        }
        let len1 = ri2.len();
        if len1 != out.len() {
            return Err(TransformResult::Error(0));
        }

        let mut wi = out.iter_mut();

        let re1 = match ri1.next() {
            Some(re1) => re1,
            None      => return Ok(0)
        };

        let re2 = match ri2.next() {
            Some(re2) => re2,
            None      => return Ok(0)
        };

        let we = wi.next().unwrap();
        let mut last1 = *re1;
        let mut last2 = sign::signum(*re2);
        *we = FromPrimitive::from_u32(0u32).unwrap();
        for ((re1, re2), we) in ri1.zip(ri2).zip(wi) {
            let current = sign::signum(*re2);
            if current != last2 {
                last1 = *re1;
            }

            *we  = conv.convert(last1);
            last2 = current;
        }

        Ok(len1)
    }

    pub fn transform_self<'a, RI, ITEMIN, CTO>(
        &self,
        ri1:         RI,
        ri2:         RI,
        out:         &mut [ITEMOUT],
        conv:        CTO
    ) -> Result<usize, TransformResult>
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri1, ri2, out, conv)
    }
}


impl<ITEMOUT> ZipSequenceTransform<ITEMOUT> for AtLastSignChange<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri1:         RI,
        ri2:         RI,
        out:         &mut [ITEMOUT],
        _default_in: ITEMIN,
        conv:        CTO
    ) -> Result<usize, TransformResult>
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri1, ri2, out, conv)
    }
}

