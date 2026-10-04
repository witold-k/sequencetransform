// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;

use crate::transform::*;
use crate::accumulate::*;

impl<ITEMOUT> MovingPitch<ITEMOUT> {
    pub fn new(count: usize, skip_first: bool) -> Self {
        MovingPitch { marker: Default::default(), count, skip_first }
    }
}

/*
 * moving linear regression b(x):
 * f = a + b x
 *
 * get minimal error of yi - f(xi):
 * d/d b,a = sum( (yi - f(xi))^2 ) = 0
 *
 * =>
 *
 * d/db: sum( xi (yi - b xi - a) ) = 0
 * d/da: sum(    (yi - b xi - a) ) = 0
 *
 * =>
 *     n sum(xi yi) - sum(xi)sum(yi)
 * b = -----------------------------
 *       n sum(xi^2) - sum(xi)^2
 *
 * i: 0 .. n - 1 => xi = i
 *
 * sum(i) = 1/2 m (m + 1)
 * sum(i^2) = 1/6 m (m + 1) (2m + 1)
 * sum(xi)^2 - n sum(xi^2) = 1/12 n^2 (n - 1) (n + 1)
 *
 *     6 (2 sum(xi yi) - (n - 1) sum(yi))
 * b = --------------------------------------
 *            n (n - 1) (n + 1)
 *
 *          6     /  2 sum(xi yi)             \
 * b = ---------- | --------------- - sum(yi) |
 *      n (n + 1) \     (n - 1)               /
 */
impl<ITEMOUT> MovingPitch<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN, CTO>(
        &self,
        ri:           RI,
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

        let aa  = AddAll::<ITEMOUT>::default();
        let aai = AddAllInc::<ITEMOUT>::default();

        let n = self.count;
        let m = n - 1;
        let i2: ITEMOUT = FromPrimitive::from_u32(2u32).unwrap();
        let i6: ITEMOUT = FromPrimitive::from_u32(6u32).unwrap();

        let skip_first = if self.skip_first { m } else { 1 };
        let mut wi = out.iter_mut().skip(skip_first);

        if !self.skip_first {
            for (i, we) in (1 .. m).zip(wi.by_ref()) {
                let i = i + 1;
                let f1: ITEMOUT = i6 / FromPrimitive::from_usize(i * (i + 1)).unwrap();
                let f2: ITEMOUT = i2 / FromPrimitive::from_usize(i - 1).unwrap();
                let fi  = ri.clone().take(i);
                // ## FIXME OPTIMIZEME sum, sumi: do not calc each time all
                *we = f1 * (f2 * aai.accumulate(fi.clone(), conv.clone()).unwrap()
                    - aa.accumulate(fi, conv.clone()).unwrap());
            }
        }

        let f1: ITEMOUT = i6 / FromPrimitive::from_usize(n * (n + 1)).unwrap();
        let f2: ITEMOUT = i2 / FromPrimitive::from_usize(m).unwrap();
        for (i, we) in (0 .. ).zip(wi) {
            // because of possible nummeric instabilities
            // maybe sum, sumi needs to be calculated fully
            let fi  = ri.clone().skip(i).take(n);
            *we = f1 * (f2 * aai.accumulate(fi.clone(), conv.clone()).unwrap()
                - aa.accumulate(fi, conv.clone()).unwrap());
        }

        if self.skip_first {
            let valid = out[n];
            let wi = out.iter_mut().take(m);
            for we in wi {
                *we = valid;
            }
        }
        else {
            out[0] = out[1];
        }

        TransformResult::Ok
    }
}


impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingPitch<ITEMOUT>
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

