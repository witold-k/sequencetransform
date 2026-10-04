// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::cast::FromPrimitive;
use arrayvec::ArrayVec;

use crate::*;
use crate::tokenize::*;
use crate::transform::*;

impl<ITEMOUT, const CAP_LN2: usize, const OVERLAP: usize, const CAP: usize> Default for SortedToUInt<ITEMOUT, CAP_LN2, OVERLAP, CAP> {
    fn default() -> Self {
        SortedToUInt {
            marker: Default::default()
        }
    }
}

/**
 *  1  2  3  4  5  6  7  8  9 10 11 -- size    = 11
 *  1  2  3  4  5                   -- cap     =  5
 *        1  2  3  4  5             -- overlap =  3 -> cap - overlap = 2
 *              1  2  3  4  5                       -> last = 6
 *                    1  2  3  4  5 -> len = 4
 */
impl<ITEMOUT, const CAP_LN2: usize, const OVERLAP: usize, const CAP: usize> SortedToUInt<ITEMOUT, CAP_LN2, OVERLAP, CAP>
where
    ITEMOUT: 'static + Copy + FromPrimitive
{
    #[inline(always)]
    pub fn len<'a, 'b, RI, ITEMIN: 'a>(
        &self,
        ri: &RI
    ) -> usize
    where
        'a: 'b,
        RI: TransformReadIterator<Item = &'a ITEMIN>,
    {
        let step = CAP - OVERLAP;
        let last = ri.len () - CAP;
        last / step + 1
    }

    /**
     * always:
     * calc_len = Self::len(ri)
     * for optimazation, len needs to be calculated before anyway,
     * passed as parameter
     */
    #[inline(always)]
    pub fn begin_read<'a, 'b, RI, ITEMIN: 'a>(
        &self,
        ri: &RI,
        calc_len: usize
    ) -> usize
    where
        'a: 'b,
        RI: TransformReadIterator<Item = &'a ITEMIN>,
    {
        let step = CAP - OVERLAP;
        ri.len() - (calc_len * step - CAP)
    }

    #[inline(always)]
    fn transform_inline<'a, 'b, RI, ITEMIN>(
        &self,
        mut ri:       RI,
        out:          &mut [ITEMOUT],
        default_out:  ITEMOUT,
    ) -> TransformResult
    where
        'a: 'b,
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
    {
        const { assert!(CAP <= 1 << CAP_LN2) }

        if self.len(&ri) != out.len() {
            return TransformResult::Error(0);
        }

        let init_i = out.iter_mut().take(CAP);
        for val in init_i {
            *val = default_out;
        }

        let mut temp_in    = ArrayVec::<ITEMIN,  CAP>::new();
        let mut temp_order = ArrayVec::<i8,      CAP>::new();
        unsafe {
            temp_in.set_len(CAP);
            temp_order.set_len(CAP);
        }

        let order = Order::<i8> { factor: 1 };
        let mut wi = out.iter_mut();

        // prime first window
        for i in 0..CAP {
            if let Some(x) = ri.next() {
                temp_in[i] = *x;
            } else {
                return TransformResult::Error(0);
            }
        }

        #[allow(clippy::while_let_loop)]
        loop {
            let wt = match wi.next() {
                Some(w) => w,
                None => break,
            };

            // order current window
            order.transform_self(temp_in.iter(), temp_order.as_mut_slice());

            // build token
            let mut token: u64 = 0;
            let mut shift: u64 = 0;
            for val in temp_order.iter() {
                let mask: u64 = (*val as u64) << shift;
                token |= mask;
                shift += CAP_LN2 as u64;
            }

            *wt = FromPrimitive::from_u64(token).unwrap();

            // slide window
            if CAP > OVERLAP {
                let keep = OVERLAP;
                let drop = CAP - keep;

                // shift left: keep last `keep` elements
                temp_in.as_mut_slice().copy_within(drop..CAP, 0);

                // refill tail
                let mut idx = keep;
                while idx < CAP {
                    if let Some(x) = ri.next() {
                        temp_in[idx] = *x;
                        idx += 1;
                    } else {
                        return TransformResult::Ok;
                    }
                }
            } else {
                break;
            }
        }

        TransformResult::Ok
    }

    pub fn transform_self<'a, RI, ITEMIN>(
        &self,
        ri: RI,
        out:         &mut [ITEMOUT],
        default_out: ITEMOUT,
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
    {
        self.transform_inline(ri, out, default_out)
    }
}


impl<ITEMOUT, const CAP_LN2: usize, const OVERLAP: usize, const CAP: usize> SequenceTransform<ITEMOUT>
for SortedToUInt<ITEMOUT, CAP_LN2, OVERLAP, CAP>
where
    ITEMOUT: 'static + Number<ITEMOUT>
{

    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri: RI,
        out:         &mut [ITEMOUT],
        default_out: ITEMOUT,
        _conv:       CTO
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    {
        self.transform_inline(ri, out, default_out)
    }
}

