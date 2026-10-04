// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::transform::*;
use crate::accumulate::*;

impl<ITEMOUT> RelativeStrengthWeigthSum<ITEMOUT> {
    pub fn new(count: usize, skip_first: bool) -> Self {
        RelativeStrengthWeigthSum { marker: Default::default(), count, skip_first }
    }
}

impl<ITEMOUT> RelativeStrengthWeigthSum<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        ri:           RI,
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
        let count = self.count;

        if len != out.len() || len < count {
            return TransformResult::Error(0);
        }
        let pa = AddRaiseWeigth::<ITEMOUT>::default();
        let na = AddFallWeigth::<ITEMOUT>::default();

        let mut wi = out.iter_mut();
        if self.skip_first {
            for we in wi.by_ref().take(count - 1) {
                *we = FromPrimitive::from_u32(0u32).unwrap();
            }
        }
        else {
            for (i, we) in (1 .. count).zip(wi.by_ref()) {
                let j = i + 1;
                let p = pa.accumulate(ri.clone().take(j), conv.clone()).unwrap();
                let n = na.accumulate(ri.clone().take(j), conv.clone()).unwrap(); // n is negative
                let pn = p - n;
                *we = if pn == FromPrimitive::from_u32(0u32).unwrap() {
                    FromPrimitive::from_u32(0u32).unwrap()
                }
                else {
                    (p + n) / pn
                };
            }
        }

        for (i, we) in wi.enumerate() {
            let p = pa.accumulate(ri.clone().skip(i).take(count), conv.clone()).unwrap();
            let n = na.accumulate(ri.clone().skip(i).take(count), conv.clone()).unwrap();
            let pn = p - n;
            *we = if pn == FromPrimitive::from_u32(0u32).unwrap() {
                FromPrimitive::from_u32(0u32).unwrap()
            }
            else {
                (p + n) / pn
            };
        }

        TransformResult::Ok
    }

    pub fn transform_self<'a, RI, ITEMIN, CTO>(
        &self,
        ri: RI,
        out:          &mut [ITEMOUT],
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


impl<ITEMOUT> SequenceTransform<ITEMOUT> for RelativeStrengthWeigthSum<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri: RI,
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

