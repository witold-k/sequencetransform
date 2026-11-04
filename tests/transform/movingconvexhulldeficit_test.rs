// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{SequenceTransform, MovingConvexHullDeficit};
fn run(data:&[f64], count:usize)->Vec<f64> {
    let mut out=vec![0.0;data.len()];
    let result=MovingConvexHullDeficit::<f64>::new(32).transform(data.iter(),&mut out,-7.0,ConvertToIdentity::default());
    assert_eq!(result,TransformResult::Ok);
    assert_eq!(count,32);
    out
}
#[test] fn causal_and_valid(){
    let x:Vec<f64>=(0..80).map(|i|(i as f64*.31).sin()+.12*(i as f64*.7).cos()).collect();
    let original=run(&x,32);
    assert!(original[..31].iter().all(|v|*v==-7.0));
    assert!(original[31..].iter().all(|v|v.is_finite()));
    let mut y=x;y.extend([200.0,-300.0]);
    let extended=run(&y,32);
    assert_eq!(original,extended[..80]);
}
#[test] fn rejects_short_and_nonfinite(){
    let mut out=[0.0;3];
    assert_ne!(MovingConvexHullDeficit::<f64>::new(32).transform([1.0,2.0,3.0].iter(),&mut out,0.0,ConvertToIdentity::default()),TransformResult::Ok);
    let mut data=vec![1.0;32];data[4]=f64::NAN;
    let mut out=vec![0.0;32];
    assert_ne!(MovingConvexHullDeficit::<f64>::new(32).transform(data.iter(),&mut out,0.0,ConvertToIdentity::default()),TransformResult::Ok);
}
#[test] fn linear_graph_has_zero_area(){
    let x:Vec<f64>=(0..32).map(|i|i as f64).collect();
    assert!((run(&x,32)[31]-1.0).abs()<1e-12);
}
#[test] fn changes_of_scale_and_offset_are_invariant(){
    let x:Vec<f64>=(0..32).map(|i|((i as f64)*0.3).sin()).collect();
    let y:Vec<f64>=x.iter().map(|v|12.0+3.0*v).collect();
    assert!((run(&x,32)[31]-run(&y,32)[31]).abs()<1e-12);
}
