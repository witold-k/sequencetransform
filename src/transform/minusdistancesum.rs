// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::transform::*;

impl<ITEMOUT> Default for MinusDistanceSum<ITEMOUT> {
    fn default() -> Self { Self { marker: Default::default() } }
}
impl<ITEMOUT> SequenceTransform<ITEMOUT> for MinusDistanceSum<ITEMOUT>
where ITEMOUT: 'static + Number<ITEMOUT> {
    fn transform<'a, RI, ITEMIN, CTO>(
        &self, ri: RI, out: &mut [ITEMOUT], _default_out: ITEMOUT, _conv: CTO
    ) -> TransformResult
    where
        RI: TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO: ConvertTo<ITEMIN, ITEMOUT>,
    {
        let result = super::minusdistance_core::signed_distances(ri, out);
        if result != TransformResult::Ok { return result; }
        let mut sum = ITEMOUT::from_u32(0).unwrap();
        for value in out.iter_mut() {
            sum += *value;
            *value = sum;
        }
        TransformResult::Ok
    }
}
