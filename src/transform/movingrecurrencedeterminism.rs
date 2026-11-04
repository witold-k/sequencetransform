// SPDX-License-Identifier: Apache-2.0
//! Causal trailing-window MovingRecurrenceDeterminism estimator.
use crate::transform::*;
use super::movingfractal_core::{feature,Feature};
impl<T> MovingRecurrenceDeterminism<T> {
    pub fn new(count:usize)->Self {Self{marker:Default::default(),count}}
}
impl<ITEMOUT> SequenceTransform<ITEMOUT> for MovingRecurrenceDeterminism<ITEMOUT>
where ITEMOUT:'static+Number<ITEMOUT> {
    fn transform<'a,RI,ITEMIN,CTO>(&self,ri:RI,out:&mut [ITEMOUT],default_out:ITEMOUT,conv:CTO)->TransformResult
    where RI:TransformReadIterator<Item=&'a ITEMIN>, ITEMIN:'a+Number<ITEMIN>, CTO:ConvertTo<ITEMIN,ITEMOUT> {
        let len=ri.len();
        if len!=out.len() || self.count<8 || self.count>len {return TransformResult::Error(0);}
        let values:Vec<f64>=ri.map(|x|{let v=conv.convert_ref(x);v.as_()}).collect();
        if values.iter().any(|v|!v.is_finite()){return TransformResult::Error(1);}
        out.fill(default_out);
        for (end,target) in out.iter_mut().enumerate().skip(self.count-1){
            if let Some(v)=feature(&values[end+1-self.count..end+1],Feature::Recurrence){
                let Some(converted)=ITEMOUT::from_f64(v) else{return TransformResult::Error(2)};
                *target=converted;
            }
        }
        TransformResult::Ok
    }
}
