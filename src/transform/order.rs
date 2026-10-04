// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::*;
use crate::transform::*;

impl<ITEMOUT> Order<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN>(
        &self,
        ri:           RI,
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
        let mut temp = Vec::<OrderEntry<ITEMIN>>::with_capacity(len);
        temp.reserve(len);
        for (index, val) in ri.enumerate() {
            temp.push(OrderEntry::<ITEMIN> { position: index as u32, value: *val });
        }
        temp.sort();
        let tri = temp.iter();
        if  self.factor != FromPrimitive::from_u32(0u32).unwrap()
         && self.factor != FromPrimitive::from_u32(1u32).unwrap()
        {
            for (r, index) in tri.zip(0 .. len) {
                let val: ITEMOUT = FromPrimitive::from_u32(index as u32).unwrap();
                let fac: ITEMOUT = val * self.factor;
                out[r.position as usize] = fac;
            }
        }
        else {
            for (r, index) in tri.zip(0 .. len) {
                out[r.position as usize] = FromPrimitive::from_u32(index as u32).unwrap();
            }
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

impl<ITEMOUT> SequenceTransform<ITEMOUT> for Order<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri: RI,
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

