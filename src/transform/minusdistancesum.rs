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
// Accumulates the signed distances computed by the look-ahead search below.
//
// Contract of the current implementation (see minusdistancesum_test.rs):
// - Read the input from front to back but write results from back to front.
// - For each input element, skip the immediate successor and search for the
//   next different value. Then search for a value crossing the current level
//   in the opposite direction; the remaining iterator lengths determine the
//   signed distance added to the running sum.
// - When fewer than two successors remain, write zero and stop. Earlier output
//   slots are not necessarily written; initialize the output if they matter.
// - This is NOT a conventional prefix/suffix sum of input values.
//
// Keep the reverse output alignment and early-stop behavior in mind before
// changing this algorithm. Its intended mathematical interpretation beyond
// these observable rules has not yet been independently specified.
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

        // Reverse alignment is intentional: the first processed input maps
        // to the last output slot.
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

            // The search deliberately starts after the immediate successor.
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

