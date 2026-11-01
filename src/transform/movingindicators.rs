// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

//! Causal trailing-window indicators. Initial outputs use available prefixes.
//! Calculations use f64 internally; output is converted to ITEMOUT.

use std::marker::PhantomData;
use crate::{ConvertTo, Number, TransformReadIterator, TransformResult};
use super::SequenceTransform;

#[derive(Copy, Clone)]
pub struct MovingWeightedMedianPosition<T> { marker: PhantomData<T>, pub count: usize }
#[derive(Copy, Clone)]
pub struct MovingSquaredWeightedMedianPosition<T> { marker: PhantomData<T>, pub count: usize }
#[derive(Copy, Clone)]
pub struct MovingPositiveStrength<T> { marker: PhantomData<T>, pub count: usize }
#[derive(Copy, Clone)]
pub struct MovingDirection<T> { marker: PhantomData<T>, pub count: usize }
#[derive(Copy, Clone)]
pub struct MovingMaxCrossingLevel<T> { marker: PhantomData<T>, pub count: usize }

impl<T> MovingWeightedMedianPosition<T> { pub fn new(count: usize) -> Self { Self { marker: PhantomData, count } } }
impl<T> MovingSquaredWeightedMedianPosition<T> { pub fn new(count: usize) -> Self { Self { marker: PhantomData, count } } }
impl<T> MovingPositiveStrength<T> { pub fn new(count: usize) -> Self { Self { marker: PhantomData, count } } }
impl<T> MovingDirection<T> { pub fn new(count: usize) -> Self { Self { marker: PhantomData, count } } }
impl<T> MovingMaxCrossingLevel<T> { pub fn new(count: usize) -> Self { Self { marker: PhantomData, count } } }

fn weighted_position(values: &[f64], squared: bool) -> f64 {
    if values.len() <= 1 { return 0.0; }
    let weights: Vec<f64> = values.iter().map(|&x| if squared { x*x } else { x }).collect();
    if weights.iter().any(|&x| !x.is_finite() || x < 0.0) { return f64::NAN; }
    // Reproduce the original two-sided balancing rule, with safe indices.
    let (mut left, mut right, mut balance) = (0usize, weights.len()-1, 0.0f64);
    while left < right {
        if balance < 0.0 { balance += weights[left]; left += 1; }
        else if balance > 0.0 { balance -= weights[right]; right -= 1; }
        else { balance += weights[left] - weights[right]; left += 1; right -= 1; }
    }
    left as f64 / values.len() as f64
}

fn positive_strength(values: &[f64]) -> f64 {
    let (mut up, mut down) = (0.0, 0.0);
    for pair in values.windows(2) {
        let d = pair[1] - pair[0];
        if d > 0.0 { up += d; } else { down -= d; }
    }
    if up + down == 0.0 { 0.0 } else { up / (up + down) }
}

fn direction(values: &[f64]) -> f64 {
    let (mut up, mut down) = (0usize, 0usize);
    for pair in values.windows(2) {
        if pair[1] > pair[0] { up += 1; }
        if pair[1] < pair[0] { down += 1; }
    }
    if up + down == 0 { 0.0 } else { (up as f64 - down as f64) / (up + down) as f64 }
}

fn max_crossing_level(values: &[f64]) -> f64 {
    if values.is_empty() { return 0.0; }
    let low = values.iter().copied().fold(f64::INFINITY, f64::min);
    let high = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !low.is_finite() || !high.is_finite() { return f64::NAN; }
    if low == high { return low; }
    let steps = values.len();
    let step = (high - low) / steps as f64;
    let (mut best_level, mut best_crossings) = (low, 0usize);
    for k in 0..steps {
        let level = low + k as f64 * step;
        let (mut last_side, mut crossings) = (0i8, 0usize);
        for &v in values.iter().rev() {
            let side = if v > level { 1 } else if v < level { -1 } else { 0 };
            if side != 0 {
                if last_side != 0 && side != last_side { crossings += 1; }
                last_side = side;
            }
        }
        if crossings > best_crossings { best_crossings = crossings; best_level = level; }
    }
    best_level
}

fn run<'a, RI, ITEMIN, ITEMOUT, CTO>(
    ri: RI, out: &mut [ITEMOUT], count: usize, conv: CTO, operation: fn(&[f64])->f64
) -> TransformResult
where
    RI: TransformReadIterator<Item=&'a ITEMIN>,
    ITEMIN: 'a + Number<ITEMIN>,
    ITEMOUT: 'static + Number<ITEMOUT>,
    CTO: ConvertTo<ITEMIN, ITEMOUT>,
{
    if count == 0 || ri.len() != out.len() || count > ri.len() { return TransformResult::Error(0); }
    let values: Vec<f64> = ri.map(|x| {
        let y = conv.convert_ref(x);
        num_traits::cast::AsPrimitive::<f64>::as_(y)
    }).collect();
    for (i, target) in out.iter_mut().enumerate() {
        let start = (i+1).saturating_sub(count);
        let value = operation(&values[start..=i]);
        if !value.is_finite() { return TransformResult::Error(1); }
        match ITEMOUT::from_f64(value) {
            Some(v) => *target = v,
            None => return TransformResult::Error(0),
        }
    }
    TransformResult::Ok
}

fn linear_position(v:&[f64])->f64 { weighted_position(v,false) }
fn squared_position(v:&[f64])->f64 { weighted_position(v,true) }

macro_rules! impl_transform {
    ($name:ident, $op:ident) => {
        impl<ITEMOUT> SequenceTransform<ITEMOUT> for $name<ITEMOUT>
        where ITEMOUT: 'static + Number<ITEMOUT> {
            fn transform<'a, RI, ITEMIN, CTO>(
                &self, ri: RI, out: &mut [ITEMOUT], _default_out: ITEMOUT, conv: CTO
            ) -> TransformResult
            where
                RI: TransformReadIterator<Item=&'a ITEMIN>,
                ITEMIN: 'a + Number<ITEMIN>,
                CTO: ConvertTo<ITEMIN, ITEMOUT>,
            {
                run(ri, out, self.count, conv, $op)
            }
        }
    };
}
impl_transform!(MovingWeightedMedianPosition, linear_position);
impl_transform!(MovingSquaredWeightedMedianPosition, squared_position);
impl_transform!(MovingPositiveStrength, positive_strength);
impl_transform!(MovingDirection, direction);
impl_transform!(MovingMaxCrossingLevel, max_crossing_level);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strengths_and_direction() {
        assert_eq!(positive_strength(&[1.0, 3.0, 2.0]), 2.0/3.0);
        assert_eq!(direction(&[1.0, 3.0, 2.0]), 0.0);
        assert_eq!(positive_strength(&[4.0]), 0.0);
    }
    #[test]
    fn crossing_constant_and_position() {
        assert_eq!(max_crossing_level(&[3.0,3.0,3.0]), 3.0);
        assert_eq!(weighted_position(&[1.0,1.0,6.0,1.0,1.0],false), 2.0/5.0);
    }
}
