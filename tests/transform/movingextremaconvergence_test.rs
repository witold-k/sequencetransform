// SPDX-License-Identifier: Apache-2.0
use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingExtremaConvergence, SequenceTransform};

fn run(data: &[f64], count: usize) -> (TransformResult, Vec<f64>) {
    let mut out = vec![-7.0; data.len()];
    let result = MovingExtremaConvergence::<f64>::new(count)
        .transform(data.iter(), &mut out, -7.0, ConvertToIdentity::default());
    (result, out)
}
#[test]
fn converging_and_diverging_lines() {
    // Highest points at indices 0,2; lowest at 1,3.
    let convergence = [10.0,0.0,9.0,1.0];
    let divergence = [9.0,1.0,10.0,0.0];
    let (status,a) = run(&convergence,4);
    assert_eq!(status,TransformResult::Ok);
    assert!(a[3] > 0.0, "expected convergence: {}",a[3]);
    let (_,b) = run(&divergence,4);
    assert!(b[3] < 0.0, "expected divergence: {}",b[3]);
}
#[test]
fn invariance_and_prefix() {
    let x = [10.0,0.0,9.0,1.0,8.0,2.0];
    let y = x.map(|v| v*3.0+100.0);
    let (_,a) = run(&x,4);
    let (_,b) = run(&y,4);
    assert_eq!(&a[..3],&[-7.0;3]);
    for (u,v) in a[3..].iter().zip(&b[3..]) {
        assert!((u-v).abs() < 1e-12);
        assert!(u.abs() <= 1.0);
    }
}
#[test]
fn causal_and_degenerate() {
    let x = [10.0,0.0,9.0,1.0,8.0,2.0];
    let (_,a) = run(&x,4);
    let mut extended = x.to_vec(); extended.push(200.0);
    assert_eq!(a,run(&extended,4).1[..x.len()]);
    assert_eq!(run(&[2.0;4],4).1[3],-7.0);
}
#[test]
fn invalid_input() {
    assert_eq!(run(&[1.0,2.0,3.0],4).0,TransformResult::Error(0));
    assert_eq!(run(&[1.0,f64::NAN,2.0,3.0],4).0,TransformResult::Error(1));
}
