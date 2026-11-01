// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::vec::Vec;
use sequencetransform::select::*;
use sequencetransform::ConvertToIdentity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let in_vec: Vec::<f32> = vec!(1.0, 3.0, 1.0, 7.0, 0.0);
        let sel = Max::<f32>::default();
        let max = sel.select(in_vec.iter(), ConvertToIdentity::<f32>::default());
        assert_eq!(max.item, 7.0);
        assert_eq!(max.index, 3);
    }
}
