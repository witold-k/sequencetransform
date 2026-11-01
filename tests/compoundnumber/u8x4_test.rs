// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski


#[cfg(test)]
mod correctness_regressions {
    use sequencetransform::compoundnumber::{U8x4, U8x8};

    #[test]
    fn reserved_slot_value_cannot_be_pushed() {
        let mut a = U8x4::default();
        let mut b = U8x8::default();
        assert!(!a.push(U8x4::FREE));
        assert!(!b.push(U8x8::FREE));
        assert!(a.empty());
        assert!(b.empty());
    }
}
