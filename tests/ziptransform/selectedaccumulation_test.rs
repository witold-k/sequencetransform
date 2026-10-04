// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::ziptransform::*;
use sequencetransform::ConvertToIdentity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selectedaccumulation_1() {
        let in_vec1: Vec::<f32> = vec!(9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let in_vec2: Vec::<f32> = vec!(0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0);
        let mut out_vec: Vec::<f32> = vec!(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let trans = SelectedAccumulation::default();

        let res = trans.transform(
            in_vec1.iter(), in_vec2.iter(),
            out_vec.as_mut_slice(), 0.0,
            ConvertToIdentity::<f32>::default()
        );

        let len = match res {
            Ok(len) => len,
            Err(err) => {
                println!("Error ocuured: {:?}", err);
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(len, 2);
        out_vec.resize(2, 0.0);

        let mut i = out_vec.iter();

        let val = match i.next() {
            Some(val) => val,
            None => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(*val, 8.0);

        let val = match i.next() {
            Some(val) => val,
            None => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(*val, 3.0);
    }


    #[test]
    fn test_selectedaccumulation_2() {
        let in_vec1: Vec::<f32> = vec!(9.0, 7.0, 8.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let in_vec2: Vec::<f32> = vec!(0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0);
        let mut out_vec: Vec::<f32> = vec!(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let trans = SelectedAccumulation::default();

        let res = trans.transform_self(
            in_vec1.iter(), in_vec2.iter(),
            out_vec.as_mut_slice(),
            ConvertToIdentity::<f32>::default()
        );

        let len = match res {
            Ok(len) => len,
            _ => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(len, 2);
        out_vec.resize(2, 0.0);

        let mut i = out_vec.iter();

        let val = match i.next() {
            Some(val) => val,
            None => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(*val, 8.0);

        let val = match i.next() {
            Some(val) => val,
            None => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(*val, 3.0);
    }

    #[test]
    fn test_selectedaccumulation_3() {
        let in_vec1: Vec::<f32> = vec!(9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 1.0, 2.0, 3.0);
        let in_vec2: Vec::<f32> = vec!(0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let mut out_vec: Vec::<f32> = vec!(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let trans = SelectedAccumulation::default();

        let res = trans.transform(
            in_vec1.iter(), in_vec2.iter(),
            out_vec.as_mut_slice(), 0.0,
            ConvertToIdentity::<f32>::default()
        );

        let len = match res {
            Ok(len) => len,
            _ => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(len, 2);
        out_vec.resize(2, 0.0);

        let mut i = out_vec.iter();

        let val = match i.next() {
            Some(val) => val,
            None => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(*val, 8.0);

        let val = match i.next() {
            Some(val) => val,
            None => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(*val, 3.0);
    }

}
