// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::*;

#[test]
fn median_then_average_exact() {
    let data=[2.0_f64,8.,4.,6.];let mut out=[0.;4];
    let result=MovingMedianAverage::<f64>::new(3,2).transform(data.iter(),&mut out,0.,ConvertToIdentity::default());
    assert_eq!(result,TransformResult::Ok);
    // Prefix medians: 2,8,4,6. Then a trailing mean of length 2.
    assert_eq!(out,[2.,5.,6.,5.]);
}
#[test]
fn invalid_windows() {
    let data=[1.0_f64,2.];let mut out=[0.;2];
    assert_ne!(MovingMedianAverage::<f64>::new(0,2).transform(data.iter(),&mut out,0.,ConvertToIdentity::default()),TransformResult::Ok);
}
