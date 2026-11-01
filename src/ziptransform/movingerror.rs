// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

//! Trailing-window mean squared error and its square root (RMSE).

use std::marker::PhantomData;
use crate::{ConvertTo, Number, TransformReadIterator, TransformResult};
use super::ZipSequenceTransform;

#[derive(Copy, Clone)]
pub struct MovingMeanSquareError<T> { marker: PhantomData<T>, pub count: usize }
#[derive(Copy, Clone)]
pub struct MovingRootMeanSquareError<T> { marker: PhantomData<T>, pub count: usize }

impl<T> MovingMeanSquareError<T> {
    pub fn new(count: usize) -> Self { Self { marker: PhantomData, count } }
}
impl<T> MovingRootMeanSquareError<T> {
    pub fn new(count: usize) -> Self { Self { marker: PhantomData, count } }
}

fn calculate<'a, RI, ITEMIN, ITEMOUT, CTO>(
    ri1: RI, ri2: RI, out: &mut [ITEMOUT], count: usize, conv: CTO, root: bool
) -> Result<usize, TransformResult>
where
    RI: TransformReadIterator<Item=&'a ITEMIN>,
    ITEMIN: 'a + Number<ITEMIN>,
    ITEMOUT: 'static + Number<ITEMOUT>,
    CTO: ConvertTo<ITEMIN, ITEMOUT>,
{
    let len = ri1.len();
    if count == 0 || count > len || len != ri2.len() || len != out.len() {
        return Err(TransformResult::Error(0));
    }
    let mut squared = Vec::with_capacity(len);
    for (a,b) in ri1.zip(ri2) {
        let x: f64 = num_traits::cast::AsPrimitive::<f64>::as_(conv.convert_ref(a));
        let y: f64 = num_traits::cast::AsPrimitive::<f64>::as_(conv.convert_ref(b));
        let delta = x-y;
        squared.push(delta*delta);
    }
    let mut sum = 0.0f64;
    for i in 0..len {
        sum += squared[i];
        if i >= count { sum -= squared[i-count]; }
        let n = (i+1).min(count);
        let mse = (sum / n as f64).max(0.0);
        let value = if root { mse.sqrt() } else { mse };
        if !value.is_finite() { return Err(TransformResult::Error(1)); }
        out[i] = ITEMOUT::from_f64(value).ok_or(TransformResult::Error(0))?;
    }
    Ok(len)
}

macro_rules! impl_error {
    ($name:ident, $root:expr) => {
        impl<ITEMOUT> ZipSequenceTransform<ITEMOUT> for $name<ITEMOUT>
        where ITEMOUT: 'static + Number<ITEMOUT> {
            fn transform<'a, RI, ITEMIN, CTO>(
                &self, ri1: RI, ri_trigger: RI, out: &mut [ITEMOUT],
                _default_in: ITEMIN, conv: CTO
            ) -> Result<usize, TransformResult>
            where
                RI: TransformReadIterator<Item=&'a ITEMIN>,
                ITEMIN: 'a + Number<ITEMIN>,
                CTO: ConvertTo<ITEMIN, ITEMOUT>,
            {
                calculate(ri1, ri_trigger, out, self.count, conv, $root)
            }
        }
    };
}
impl_error!(MovingMeanSquareError, false);
impl_error!(MovingRootMeanSquareError, true);
