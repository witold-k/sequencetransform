// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::transform::*;

/// Sort input values in ascending order without changing the input sequence.
/// Returns Error(1) if any input value is not comparable (e.g. NaN).
#[derive(Copy, Clone, Default)]
pub struct Sort<T> {
    marker: std::marker::PhantomData<T>,
}

impl<T> Sort<T> {
    pub fn new() -> Self {
        Self { marker: std::marker::PhantomData }
    }
}

impl<ITEMOUT> SequenceTransform<ITEMOUT> for Sort<ITEMOUT>
where
    ITEMOUT: 'static + Number<ITEMOUT>,
{
    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri: RI,
        out: &mut [ITEMOUT],
        _default_out: ITEMOUT,
        conv: CTO,
    ) -> TransformResult
    where
        RI: TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO: ConvertTo<ITEMIN, ITEMOUT>,
    {
        if ri.len() != out.len() {
            return TransformResult::Error(0);
        }
        let mut values: Vec<ITEMOUT> = ri.map(|v| conv.convert_ref(v)).collect();
        if values.iter().any(|v| v.partial_cmp(v).is_none()) {
            return TransformResult::Error(1);
        }
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        out.copy_from_slice(&values);
        TransformResult::Ok
    }
}
