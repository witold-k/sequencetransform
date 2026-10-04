// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::selector::*;
use sequencetransform::selector::LastN;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_n() {
        let data_in      = [2, 3, 4, 5];
        let mut data_out = vec![0; 3];
        data_out.resize(3, 0);

        let select = LastN { count: 3 };

        let res = select.populate(data_in.iter(), data_out.iter_mut());
        assert_eq!(res, SelectorResult::Ok);
        assert_eq!(data_out, vec!(2, 3, 4));

        let res = select.populate(data_in.iter().skip(1), data_out.iter_mut());
        assert_eq!(res, SelectorResult::Ok);
        assert_eq!(data_out, vec!(3, 4, 5));

        let res = select.populate(data_in.iter().skip(2), data_out.iter_mut());
        assert_eq!(res, SelectorResult::Error(1));
        assert_eq!(data_out, vec!(3, 4, 5));
}

}
