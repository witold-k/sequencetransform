// SPDX-License-Identifier: Apache-2.0
//! Shared finite-window nonlinear-dynamics estimators.
use std::f64::consts::LN_2;

pub(crate) enum Feature { Higuchi, Multifractal, Recurrence }

fn slope(points: &[(f64,f64)]) -> Option<f64> {
    if points.len()<2 {return None;}
    let n=points.len() as f64;
    let mx=points.iter().map(|p|p.0).sum::<f64>()/n;
    let my=points.iter().map(|p|p.1).sum::<f64>()/n;
    let den=points.iter().map(|p|(p.0-mx).powi(2)).sum::<f64>();
    if den<=0.0 {return None;}
    Some(points.iter().map(|p|(p.0-mx)*(p.1-my)).sum::<f64>()/den)
}
fn higuchi(x:&[f64])->Option<f64>{
    let n=x.len();
    let kmax=(n/4).min(16);
    if kmax<2{return None;}
    let mut points=Vec::new();
    for k in 1..=kmax {
        let mut lengths=Vec::new();
        for m in 0..k {
            let steps=(n-1-m)/k;
            if steps==0 {continue;}
            let mut length=0.0;
            for j in 1..=steps {length+=(x[m+j*k]-x[m+(j-1)*k]).abs();}
            length*= (n-1) as f64 / (steps*k*k) as f64;
            lengths.push(length);
        }
        if lengths.is_empty(){continue;}
        let mean=lengths.iter().sum::<f64>()/lengths.len() as f64;
        if mean>0.0 && mean.is_finite(){points.push(((1.0/k as f64).ln(),mean.ln()));}
    }
    slope(&points)
}
fn multifractal(x:&[f64])->Option<f64>{
    let n=x.len();
    let max_scale=(n/4).min(32);
    if max_scale<4{return None;}
    let mut first=Vec::new();
    let mut fourth=Vec::new();
    for s in 1..=max_scale {
        let mut a=0.0;
        let mut b=0.0;
        for i in 0..n-s {
            let d=(x[i+s]-x[i]).abs();
            a+=d;
            b+=d.powi(4);
        }
        a/=(n-s) as f64;
        b/=(n-s) as f64;
        if a>0.0 && b>0.0 && a.is_finite() && b.is_finite() {
            first.push(((s as f64).ln(),a.ln()));
            fourth.push(((s as f64).ln(),b.ln()));
        }
    }
    Some(slope(&first)?-slope(&fourth)?/4.0)
}
fn recurrence(x:&[f64])->Option<f64>{
    // 2D delay vectors (x[t-1],x[t]); radius = 10% of the
    // Euclidean diameter of the input range. Exclude the main diagonal
    // and immediately adjacent pairs (Theiler exclusion = 1).
    let n=x.len();
    if n<8{return None;}
    let min=x.iter().copied().fold(f64::INFINITY,f64::min);
    let max=x.iter().copied().fold(f64::NEG_INFINITY,f64::max);
    let range=max-min;
    if range<=0.0 || !range.is_finite(){return None;}
    let radius2=2.0*(0.1*range).powi(2);
    let m=n-1;
    let mut grid=vec![false;m*m];
    for i in 0..m {
        for j in i+2..m {
            let d0=x[i]-x[j];
            let d1=x[i+1]-x[j+1];
            let recurrent=d0*d0+d1*d1<=radius2;
            grid[i*m+j]=recurrent;
            grid[j*m+i]=recurrent;
        }
    }
    let mut total=0usize;
    let mut diagonal=0usize;
    for i in 0..m {
        for j in i+2..m {
            if !grid[i*m+j]{continue;}
            total+=1;
            // A recurrence point is deterministic if it belongs to
            // a diagonal run of at least two consecutive points.
            let prev=i>0 && j>0 && grid[(i-1)*m+j-1];
            let next=i+1<m && j+1<m && grid[(i+1)*m+j+1];
            if prev || next {diagonal+=1;}
        }
    }
    if total==0 {None} else {Some(diagonal as f64/total as f64)}
}
pub(crate) fn feature(x:&[f64],kind:Feature)->Option<f64>{
    let _=LN_2; // no external dependencies
    let result=match kind {
        Feature::Higuchi=>higuchi(x),
        Feature::Multifractal=>multifractal(x),
        Feature::Recurrence=>recurrence(x),
    }?;
    result.is_finite().then_some(result)
}
