// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::iterop::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_itertillup() {
        let in_vec: Vec::<f32> = vec!(0.0, 0.0, 0.0, 1.0, 1.0);
        let mut i = IterTillUp::new(in_vec.iter());
        let val = match i.next() {
            Some(val) => val,
            None => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(*val, 0.0);

        let val = match i.next() {
            Some(val) => val,
            None => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(*val, 0.0);

        _ = match i.next() {
            Some(_) => {
                assert_eq!(true, false);
                return;
            },
            None => 0.0
        };
    }
}
