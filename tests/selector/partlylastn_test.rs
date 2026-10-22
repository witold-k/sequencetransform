// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::selector::*;
use sequencetransform::selector::PartlyLastN;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partly_last_n() {
        let data_in      = [2, 3, 4, 5, 6, 7];
        let mut data_out = vec![0; 4];
        data_out.resize(4, 0);

        let select = PartlyLastN::new(vec!(1, 2, 1));
        assert_eq!(select.process_size, 5);

        let res = select.populate(data_in.iter(), data_out.iter_mut());
        assert_eq!(res, SelectorResult::Ok);
        assert_eq!(data_out, vec!(2, 3, 5, 6));

        let res = select.populate(data_in.iter().skip(1), data_out.iter_mut());
        assert_eq!(res, SelectorResult::Ok);
        assert_eq!(data_out, vec!(3, 4, 6, 7));

        let res = select.populate(data_in.iter().skip(2), data_out.iter_mut());
        assert_eq!(res, SelectorResult::Error(1));
        assert_eq!(data_out, vec!(3, 4, 6, 7));

        let select = PartlyLastN::new(vec!(1, 2, 2));
        assert_eq!(select.process_size, 6);

        let res = select.populate(data_in.iter(), data_out.iter_mut());
        assert_eq!(res, SelectorResult::Ok);
        assert_eq!(data_out, vec!(2, 3, 5, 7));
    }

}

#[cfg(test)]
mod correctness_regressions {
    use sequencetransform::selector::{PartlyLastN, PartlyLastN2, Selector, SelectorResult};

    #[test]
    fn selector_rejects_short_output() {
        let sel = PartlyLastN::new(vec![1, 2, 1]);
        let data = [1, 2, 3, 4, 5];
        let mut out = [0];
        assert_ne!(sel.populate(data.iter(), out.iter_mut()), SelectorResult::Ok);
    }

    #[test]
    #[should_panic]
    fn selector_rejects_zero_interval() {
        PartlyLastN::new(vec![1, 0, 2]);
    }

    #[test]
    #[should_panic]
    fn selector_rejects_oversized_tail() {
        PartlyLastN2::new_taillen(vec![1, 2], 3);
    }
}
