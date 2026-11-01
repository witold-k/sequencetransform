// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::ConvertToIdentity;
use sequencetransform::ziptransform::*;

fn check<T: ZipSequenceTransform<f64>>(t: T, a: &[f64], b: &[f64], expected: &[f64]) {
    let mut out=vec![0.0; a.len()];
    let res=t.transform(a.iter(),b.iter(),&mut out,0.0,ConvertToIdentity::<f64>::default());
    assert_eq!(res,Ok(a.len()));
    for (i, (&x,&y)) in out.iter().zip(expected).enumerate() {
        assert!((x-y).abs()<1e-9,"index {i}: {x} != {y}");
    }
}
#[test]
fn mse_prefix_and_sliding_window() {
    check(MovingMeanSquareError::<f64>::new(2), &[1.,3.,2.], &[2.,1.,3.], &[1.,2.5,2.5]);
}
#[test]
fn rmse_is_sqrt_of_mse() {
    check(MovingRootMeanSquareError::<f64>::new(2), &[1.,3.,2.], &[2.,1.,3.], &[1.,2.5_f64.sqrt(),2.5_f64.sqrt()]);
}
#[test]
fn zero_error_is_zero() {
    check(MovingMeanSquareError::<f64>::new(2), &[1.,3.], &[1.,3.], &[0.,0.]);
    check(MovingRootMeanSquareError::<f64>::new(2), &[1.,3.], &[1.,3.], &[0.,0.]);
}
#[test]
fn invalid_windows_and_mismatched_lengths() {
    let a=[1.,2.]; let b=[1.,2.]; let mut out=[0.;2];
    assert_ne!(MovingMeanSquareError::<f64>::new(0).transform(a.iter(),b.iter(),&mut out,0.,ConvertToIdentity::default()),Ok(2));
    assert_ne!(MovingRootMeanSquareError::<f64>::new(3).transform(a.iter(),b.iter(),&mut out,0.,ConvertToIdentity::default()),Ok(2));
    assert_ne!(MovingMeanSquareError::<f64>::new(2).transform(a.iter(),b[..1].iter(),&mut out,0.,ConvertToIdentity::default()),Ok(2));
}
#[test]
fn future_samples_do_not_change_previous_errors() {
    let a=[1.,3.,2.]; let b=[2.,1.,3.];
    let mut first=[0.;3];
    MovingMeanSquareError::<f64>::new(2).transform(a.iter(),b.iter(),&mut first,0.,ConvertToIdentity::default()).unwrap();
    let mut longer=[0.;4];
    MovingMeanSquareError::<f64>::new(2).transform([1.,3.,2.,100.].iter(),[2.,1.,3.,0.].iter(),&mut longer,0.,ConvertToIdentity::default()).unwrap();
    assert_eq!(&longer[..3],&first);
}
