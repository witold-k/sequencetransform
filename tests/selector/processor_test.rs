// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::selector::*;
use sequencetransform::selector::LastN;

#[cfg(test)]
mod tests {
    use super::*;

    fn new_vec() -> Vec<i32> {
        let mut data_out = vec![0; 3];
        data_out.resize(3, 0);
        data_out
    }

    #[test]
    fn test_processor() {
        let data_in      = [2, 3, 4, 5, 6];

        let select = LastN { count: 3 };
        let mut proc = SimpleProcessor {
            read_iterator: data_in.iter(),
            selector: select
        };
        let mut data_out = new_vec();

        assert!(proc.populate(data_out.iter_mut()));
        assert_eq!(data_out, vec!(2, 3, 4));

        assert!(proc.populate(data_out.iter_mut()));
        assert_eq!(data_out, vec!(3, 4, 5));

        assert!(proc.populate(data_out.iter_mut()));
        assert_eq!(data_out, vec!(4, 5, 6));

        assert!(!proc.populate(data_out.iter_mut()));
        assert_eq!(data_out, vec!(4, 5, 6));
    }

}
