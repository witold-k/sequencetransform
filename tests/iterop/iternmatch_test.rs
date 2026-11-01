// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::iterop::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iternmatch() {
        let in_vec: Vec::<u32> = vec!(1, 2, 3, 4, 5);
        let mut i = IterNMatch::new(
            in_vec.iter(),
            |x| { x % 2 != 0 }
        );
        let val = match i.next() {
            Some(val) => val,
            None => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(*val, 2);

        let val = match i.next() {
            Some(val) => val,
            None => {
                assert_eq!(true, false);
                return;
            }
        };
        assert_eq!(*val, 4);

        _ = match i.next() {
            Some(_) => {
                assert_eq!(true, false);
                return;
            },
            None => 0
        };
    }
}
