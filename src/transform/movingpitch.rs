// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::transform::*;

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
        if len != out.len() || self.count < 2 || self.count > len {
            return TransformResult::Error(0);
        }

        // Center both axes before forming the covariance. Subtracting the
        // first sample avoids cancellation of large common offsets.
        let n = self.count;
        let m = n - 1;
        let Some(two) = ITEMOUT::from_u32(2) else { return TransformResult::Error(2); };
        let Some(twelve) = ITEMOUT::from_u32(12) else { return TransformResult::Error(2); };

        let calculate = |start: usize, width: usize| -> Option<ITEMOUT> {
            let count = ITEMOUT::from_usize(width)?;
            let last = ITEMOUT::from_usize(width - 1)?;
            let center = last / two;
            // Variance of 0..width-1 about its mean, without usize products.
            let denominator = count * (count - ITEMOUT::from_u32(1)?)
                * (count + ITEMOUT::from_u32(1)?) / twelve;
            if denominator == ITEMOUT::from_u32(0)? { return None; }
            let mut samples = ri.clone().skip(start).take(width);
            let base = conv.convert_ref(samples.next()?);
            let mut covariance = ITEMOUT::from_u32(0)?;
            for (index, sample) in samples.enumerate() {
                let x = ITEMOUT::from_usize(index + 1)? - center;
                covariance += x * (conv.convert_ref(sample) - base);
            }
            Some(covariance / denominator)
        };

        if !self.skip_first {
            for width in 2..n {
                let Some(value) = calculate(0, width) else { return TransformResult::Error(2); };
                out[width - 1] = value;
            }
        }
        for (end, output) in out.iter_mut().enumerate().skip(m) {
            let Some(value) = calculate(end + 1 - n, n) else { return TransformResult::Error(2); };
            *output = value;
        }

        if self.skip_first {
            let valid = out[m];
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

