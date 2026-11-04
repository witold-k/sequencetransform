// SPDX-License-Identifier: Apache-2.0
//! Finite-window estimators from transport, convex geometry and isotonic regression.
//! All functions only inspect the supplied window.
pub(crate) enum Feature { Transport, HullDeficit, IsotonicResidual }

fn transport(x: &[f64]) -> Option<f64> {
    if x.len() < 4 || !x.len().is_multiple_of(2) { return None; }
    let n = x.len()/2;
    let mut a = x[..n].to_vec();
    let mut b = x[n..].to_vec();
    a.sort_unstable_by(|u,v| u.total_cmp(v));
    b.sort_unstable_by(|u,v| u.total_cmp(v));
    Some(a.iter().zip(b.iter()).map(|(u,v)| (u-v).abs()).sum::<f64>()/n as f64)
}
fn cross(a:(f64,f64),b:(f64,f64),c:(f64,f64))->f64{
    (b.0-a.0)*(c.1-a.1)-(b.1-a.1)*(c.0-a.0)
}
fn hull_deficit(x: &[f64])-> Option<f64> {
    let n=x.len();
    if n<3 {return None;}
    let lo=x.iter().copied().fold(f64::INFINITY,f64::min);
    let hi=x.iter().copied().fold(f64::NEG_INFINITY,f64::max);
    let range=hi-lo;
    if range<=0.0 || !range.is_finite() {return None;}
    // Coordinates normalized to a unit-square bounding box.
    let points:Vec<(f64,f64)> = x.iter().enumerate()
        .map(|(i,&y)|(i as f64/(n-1) as f64,(y-lo)/range)).collect();
    let mut lower:Vec<(f64,f64)>=Vec::with_capacity(n);
    for &p in &points {
        while lower.len()>=2 && cross(lower[lower.len()-2],lower[lower.len()-1],p)<=0.0 {lower.pop();}
        lower.push(p);
    }
    let mut upper:Vec<(f64,f64)>=Vec::with_capacity(n);
    for &p in points.iter().rev() {
        while upper.len()>=2 && cross(upper[upper.len()-2],upper[upper.len()-1],p)<=0.0 {upper.pop();}
        upper.push(p);
    }
    lower.pop();upper.pop();lower.extend(upper);
    let area=lower.iter().zip(lower.iter().cycle().skip(1))
        .take(lower.len()).map(|(a,b)|a.0*b.1-b.0*a.1).sum::<f64>().abs()/2.0;
    Some((1.0-area).clamp(0.0,1.0))
}
fn pava_error(x:&[f64],increasing:bool)->f64 {
    // Each block stores (mean, number of samples). Adjacent violations merge.
    let mut blocks:Vec<(f64,usize)>=Vec::with_capacity(x.len());
    for &raw in x {
        let v=if increasing {raw} else {-raw};
        blocks.push((v,1));
        while blocks.len()>=2 {
            let n=blocks.len();
            if blocks[n-2].0<=blocks[n-1].0 {break;}
            let (m2,w2)=blocks.pop().unwrap();
            let (m1,w1)=blocks.pop().unwrap();
            let w=w1+w2;
            blocks.push(((m1*w1 as f64+m2*w2 as f64)/w as f64,w));
        }
    }
    let mut index=0; let mut error=0.0;
    for (mean,w) in blocks {
        let m=if increasing {mean} else {-mean};
        for &v in &x[index..index+w] {error+=(v-m)*(v-m);}
        index+=w;
    }
    error
}
fn isotonic(x:&[f64])->Option<f64>{
    if x.len()<3{return None;}
    let mean=x.iter().sum::<f64>()/x.len() as f64;
    let total=x.iter().map(|v|(v-mean).powi(2)).sum::<f64>();
    if total<=0.0 || !total.is_finite(){return None;}
    let residual=pava_error(x,true).min(pava_error(x,false));
    Some((residual/total).clamp(0.0,1.0))
}
pub(crate) fn feature(x:&[f64],kind:Feature)->Option<f64>{
    let result=match kind {
        Feature::Transport=>transport(x),
        Feature::HullDeficit=>hull_deficit(x),
        Feature::IsotonicResidual=>isotonic(x),
    }?;
    result.is_finite().then_some(result)
}
