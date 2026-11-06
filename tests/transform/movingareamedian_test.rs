// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingAreaMedian, SequenceTransform};

fn run(input: &[f64], count: usize, default: f64) -> (TransformResult, Vec<f64>) {
    let mut output = vec![default; input.len()];
    let status = MovingAreaMedian::<f64>::new(count).transform(
        input.iter(), &mut output, default, ConvertToIdentity::default()
    );
    (status, output)
}
fn close(a: f64, b: f64) { assert!((a-b).abs() < 1e-12, "{a} != {b}"); }

#[test]
fn constant_and_linear_windows() {
    for (data, expected) in [
        (vec![7.0,7.0,7.0],0.5),
        (vec![0.0,1.0,2.0],2.0_f64.sqrt()/2.0),
        (vec![2.0,1.0,0.0],1.0-2.0_f64.sqrt()/2.0),
    ] {
        let (status, out)=run(&data,3,-7.0);
        assert_eq!(status,TransformResult::Ok);
        assert_eq!(out[..2],[-7.0,-7.0]);
        close(out[2],expected);
    }
}
#[test]
fn scale_offset_and_plateau() {
    let base=run(&[1.0,2.0,5.0,3.0,2.0],5,-1.0).1[4];
    close(base,run(&[101.0,102.0,105.0,103.0,102.0],5,-1.0).1[4]);
    close(base,run(&[10.0,20.0,50.0,30.0,20.0],5,-1.0).1[4]);
    close(run(&[1.0,0.0,0.0,0.0,1.0],5,-1.0).1[4],0.5);
}
#[test]
fn causal_and_bounded() {
    let x:Vec<f64>=(0..80).map(|i|(i as f64*0.31).sin()).collect();
    let (status,original)=run(&x,16,-7.0);
    assert_eq!(status,TransformResult::Ok);
    assert!(original[..15].iter().all(|v|*v==-7.0));
    assert!(original[15..].iter().all(|v| (0.0..=1.0).contains(v)));
    let mut extended=x; extended.extend([100.0,-100.0]);
    assert_eq!(original,run(&extended,16,-7.0).1[..80]);
}
#[test]
fn invalid_input() {
    assert_eq!(run(&[1.0,2.0,3.0],4,0.0).0,TransformResult::Error(0));
    assert_eq!(run(&[1.0,f64::NAN,3.0],3,0.0).0,TransformResult::Error(1));
}
