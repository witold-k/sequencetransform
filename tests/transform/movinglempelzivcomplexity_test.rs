// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity,TransformResult};
use sequencetransform::transform::{SequenceTransform,MovingLempelZivComplexity};
#[test] fn causal_and_finite() {
 let x:Vec<f64>=(0..72).map(|i| (i as f64*0.27).sin()+(i as f64*0.83).cos()).collect();
 let mut a=vec![-1.0;72];
 assert_eq!(MovingLempelZivComplexity::<f64>::new(24).transform(x.iter(),&mut a,-7.0,ConvertToIdentity::default()),TransformResult::Ok);
 assert!(a[..23].iter().all(|v|*v==-7.0));
 assert!(a[23..].iter().all(|v|v.is_finite()&&*v>=0.0));
 let mut y=x;y.extend([12.0,-8.0]);let mut z=vec![0.0;y.len()];
 assert_eq!(MovingLempelZivComplexity::<f64>::new(24).transform(y.iter(),&mut z,-7.0,ConvertToIdentity::default()),TransformResult::Ok);
 assert_eq!(a,z[..72]);
}
#[test] fn rejects_invalid_and_nonfinite(){
 let x=[1.0,2.0,3.0];let mut out=[0.0;3];
 assert_ne!(MovingLempelZivComplexity::<f64>::new(24).transform(x.iter(),&mut out,0.0,ConvertToIdentity::default()),TransformResult::Ok);
 let mut x=vec![1.0;24];x[5]=f64::NAN;let mut out=vec![0.0;24];
 assert_ne!(MovingLempelZivComplexity::<f64>::new(24).transform(x.iter(),&mut out,0.0,ConvertToIdentity::default()),TransformResult::Ok);
}
