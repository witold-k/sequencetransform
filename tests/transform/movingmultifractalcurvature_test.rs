// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity,TransformResult};
use sequencetransform::transform::{SequenceTransform,MovingMultifractalCurvature};
fn run(x:&[f64],n:usize)->Vec<f64>{
 let mut out=vec![-99.0;x.len()];
 assert_eq!(MovingMultifractalCurvature::<f64>::new(n).transform(x.iter(),&mut out,-7.0,ConvertToIdentity::default()),TransformResult::Ok);
 out
}
#[test] fn causal_bounded_prefix(){
 let x:Vec<f64>=(0..80).map(|i|((i as f64)*0.41).sin()+0.2*((i as f64)*0.9).cos()).collect();
 let a=run(&x,32);
 assert!(a[..31].iter().all(|v|*v==-7.0));
 assert!(a[31..].iter().all(|v|v.is_finite()));
 let mut y=x;y.extend([100.0,-100.0]);
 let b=run(&y,32);assert_eq!(a,b[..a.len()]);
}
#[test] fn constant_is_undefined(){
 assert!(run(&[3.0;64],32).iter().all(|v|*v==-7.0));
}
#[test] fn invalid_window_and_nonfinite_input(){
 let x=[1.0,2.0,3.0];let mut out=[0.0;3];
 assert_ne!(MovingMultifractalCurvature::<f64>::new(0).transform(x.iter(),&mut out,0.0,ConvertToIdentity::default()),TransformResult::Ok);
 let mut invalid=vec![1.0;32];invalid[8]=f64::NAN;
 let mut output=vec![0.0;32];
 assert_ne!(MovingMultifractalCurvature::<f64>::new(32).transform(invalid.iter(),&mut output,0.0,ConvertToIdentity::default()),TransformResult::Ok);
}
