// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::transform::*;

impl<ITEMOUT> Default for MinusDistanceSum<ITEMOUT> {
    fn default() -> Self {
        MinusDistanceSum { marker: Default::default() }
    }
}

/*
impl<ITEMOUT> MinusDistanceSum<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        mut ri:       RI,
        out:          &mut [ITEMOUT],
        _default_out: ITEMOUT,
        _conv:        CTO
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

        let mut wi = out.iter_mut().rev();
        let mut sum: ITEMOUT = FromPrimitive::from_u32(0u32).unwrap();

        loop {
            let re = match ri.next() {
                Some(re) => re,
                None     => break
            };
            let we = match wi.next() {
                Some(we) => we,
                None     => break
            };

            let mut ri_count = ri.clone();
            _ = match ri_count.next() {
                Some(curr) => curr,
                None => {
                    *we = FromPrimitive::from_u32(0u32).unwrap();
                    break
                }
            };
            let neo = ri_count.find(|&& x| x != *re);

            if let Some(ne) = neo {
                if ne < re {
                    _ = ri_count.find(|&& x| x >= *re);
                    sum += FromPrimitive::from_isize(ri.len() as isize - ri_count.len() as isize).unwrap();
                }
                else {
                    _ = ri_count.find(|&& x| x <= *re);
                    sum += FromPrimitive::from_isize(ri_count.len() as isize - ri.len() as isize).unwrap();
                }
            }
            *we = sum;
        }

        TransformResult::Ok
    }
}
*/

impl<ITEMOUT> MinusDistanceSum<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        mut ri:       RI,
        out:          &mut [ITEMOUT],
        _default_out: ITEMOUT,
        _conv:        CTO
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

        let mut wi = out.iter_mut().rev();
        let mut sum: ITEMOUT = FromPrimitive::from_u32(0u32).unwrap();

        #[allow(clippy::while_let_loop)]
        loop {
            let re = match ri.next() {
                Some(re) => re,
                None     => break
            };
            let we = match wi.next() {
                Some(we) => we,
                None     => break
            };

            let mut ri_count = ri.clone();
            _ = match ri_count.next() {
                Some(curr) => curr,
                None => {
                    *we = FromPrimitive::from_u32(0u32).unwrap();
                    break
                }
            };
            let neo = ri_count.find(|&& x| x != *re);

            if let Some(ne) = neo {
                if ne < re {
                    _ = ri_count.find(|&& x| x >= *re);
                    sum += FromPrimitive::from_isize(ri.len() as isize - ri_count.len() as isize).unwrap();
                }
                else {
                    _ = ri_count.find(|&& x| x <= *re);
                    sum += FromPrimitive::from_isize(ri_count.len() as isize - ri.len() as isize).unwrap();
                }
            }
            *we = sum;
        }

        TransformResult::Ok
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for MinusDistanceSum<ITEMOUT>
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

