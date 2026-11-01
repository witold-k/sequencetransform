// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;

use sequencetransform::set::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retain_property() {
        let mut in_vec1: Vec::<u32> = vec!(1, 2, 3, 4, 5);
        let out_vec: Vec::<u32> = vec!(2, 4);

        let ret = RetainProperty::new(
            |elem| { elem % 2 == 0 }
        );
        //let iter1 = &in_vec1.iter() as *const _;
        let len = ret.exe(in_vec1.as_mut_slice());
        assert_eq!(len, 2);
        assert_eq!(&in_vec1[0..len], out_vec.as_slice());
    }

}
