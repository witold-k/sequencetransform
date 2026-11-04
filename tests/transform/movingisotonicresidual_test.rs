// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{SequenceTransform, MovingIsotonicResidual};
fn run(data:&[f64], count:usize)->Vec<f64> {
    let mut out=vec![0.0;data.len()];
    let result=MovingIsotonicResidual::<f64>::new(32).transform(data.iter(),&mut out,-7.0,ConvertToIdentity::default());
    assert_eq!(result,TransformResult::Ok);
    assert_eq!(count,32);
    out
}
#[test] fn causal_and_valid(){
    let x:Vec<f64>=(0..80).map(|i|(i as f64*0.31).sin()+0.12*(i as f64*0.7).cos()).collect();
    let original=run(&x,32);
    assert!(original[..31].iter().all(|v|*v==-7.0));
    assert!(original[31..].iter().all(|v|v.is_finite()));
    let mut y=x;y.extend([200.0,-300.0]);
    let extended=run(&y,32);
    assert_eq!(original,extended[..80]);
}
#[test] fn rejects_short_and_nonfinite(){
    let mut out=[0.0;3];
    assert_ne!(MovingIsotonicResidual::<f64>::new(32).transform([1.0,2.0,3.0].iter(),&mut out,0.0,ConvertToIdentity::default()),TransformResult::Ok);
    let mut data=vec![1.0;32];data[4]=f64::NAN;
    let mut out=vec![0.0;32];
    assert_ne!(MovingIsotonicResidual::<f64>::new(32).transform(data.iter(),&mut out,0.0,ConvertToIdentity::default()),TransformResult::Ok);
}
#[test] fn monotonic_is_zero_and_zigzag_is_positive() {
    let up:Vec<f64>=(0..32).map(|i|i as f64).collect();
    let down:Vec<f64>=up.iter().rev().copied().collect();
    assert!(run(&up,32)[31].abs()<1e-12);
    assert!(run(&down,32)[31].abs()<1e-12);
    let zig:Vec<f64>=(0..32).map(|i|if i%2==0 {1.0} else {-1.0}).collect();
    assert!(run(&zig,32)[31]>0.5);
}
