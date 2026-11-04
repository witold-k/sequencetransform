// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{SequenceTransform, MovingMatrixEffectiveRank};
fn run(data:&[f64], count:usize)->Vec<f64> {
    let mut out=vec![0.0;data.len()];
    let result=MovingMatrixEffectiveRank::<f64>::new(32,8).transform(data.iter(),&mut out,-7.0,ConvertToIdentity::default());
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
    assert_ne!(MovingMatrixEffectiveRank::<f64>::new(32,8).transform([1.0,2.0,3.0].iter(),&mut out,0.0,ConvertToIdentity::default()),TransformResult::Ok);
    let mut data=vec![1.0;32];data[4]=f64::NAN;
    let mut out=vec![0.0;32];
    assert_ne!(MovingMatrixEffectiveRank::<f64>::new(32,8).transform(data.iter(),&mut out,0.0,ConvertToIdentity::default()),TransformResult::Ok);
}
#[test] fn periodic_rank_and_constant_undefined(){
    let x:Vec<f64>=(0..64).map(|i|(i as f64 * std::f64::consts::PI/2.0).sin()).collect();
    let rank=run(&x,32)[31];
    assert!(rank.is_finite()&&rank>1.0&&rank<3.0, "{rank}");
    assert!(run(&[4.0;64],32).iter().all(|v|*v==-7.0));
}
