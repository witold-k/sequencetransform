// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::iterop::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter2muteq() {
        let mut in1_vec: Vec::<f32> = vec!(1.0, 4.0, 5.0, 7.0, 1.0);
        let mut in2_vec: Vec::<f32> = vec!(2.0, 3.0, 6.0, 7.0, 3.0);

        let mut i = Iter2MutEq::new(
            in1_vec.iter_mut(), in2_vec.iter_mut(), |elem: &mut f32| { elem }
        );

        let (val1, val2) = match i.next2() {
            (Some(val1), Some(val2)) => {
                (val1, val2)
            }
            (None, None) => {
                panic!("No difference found");
            }
            _ => {
                panic!("Unexpected case");
            }
        };

        assert_eq!(*val1, 7.0);
        assert_eq!(*val2, 7.0);

        match i.next2() {
            (Some(_), Some(_)) => {
                panic!("difference found");
            }
            (None, None) => {
                // should be end of stream
            }
            _ => {
                panic!("Unexpected case");
            }
        };
    }
}
