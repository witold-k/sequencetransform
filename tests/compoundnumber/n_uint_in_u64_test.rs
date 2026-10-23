// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::distance::*;
use sequencetransform::compoundnumber::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let value: u64 = (6 << 6) | (1 << 3) | 3;
        let u33 = NUintInU64::<3, 3>::new(value);
        assert_eq!(u33.get(0), 3);
        assert_eq!(u33.get(1), 1);
        assert_eq!(u33.get(2), 6);
    }

    #[test]
    fn test_delta() {
        const MASK: u64 = 7;
        let value: u64 = (6 << 6) | (1 << 3) | 3;
        let u33_1 = NUintInU64::<3, 3>::new(value);
        let dvalue = ((6 - 1) << 6) | ((1u64.wrapping_sub(3) & MASK) << 3);
        let u33_2 = NUintInU64::<3, 3>::new(dvalue);
        assert_eq!(u33_1.delta(), u33_2);
    }

    #[test]
    fn test_not() {
        let value: u64 = (6 << 6) | (1 << 3) | 3;
        let u33_1 = NUintInU64::<3, 3>::new(value);
        let u33_2 = !u33_1;
        assert_eq!(u33_2.value, !value);
    }

    #[test]
    fn test_and() {
        let value_1: u64 = (6 << 6) | (1 << 3) | 3;
        let value_2: u64 = (5 << 6) | (2 << 3) | 5;
        let u33_1 = NUintInU64::<3, 3>::new(value_1);
        let mut u33_2 = NUintInU64::<3, 3>::new(value_2);
        let u33_3 = u33_1 & u33_2;
        assert_eq!(u33_3.value, value_1 & value_2);
        u33_2 &= u33_1;
        assert_eq!(u33_3, u33_2);
    }

    #[test]
    fn test_or() {
        let value_1: u64 = (6 << 6) | (1 << 3) | 3;
        let value_2: u64 = (5 << 6) | (2 << 3) | 5;
        let u33_1 = NUintInU64::<3, 3>::new(value_1);
        let mut u33_2 = NUintInU64::<3, 3>::new(value_2);
        let u33_3 = u33_1 | u33_2;
        assert_eq!(u33_3.value, value_1 | value_2);
        u33_2 |= u33_1;
        assert_eq!(u33_3, u33_2);
    }

    #[test]
    fn test_distance() {
        let value_1: u64 = (6 << 6) | (1 << 3) | 3;
        let value_2: u64 = (5 << 6) | (2 << 3) | 5;
        let u33_1 = NUintInU64::<3, 3>::new(value_1);
        let mut u33_2 = NUintInU64::<3, 3>::new(value_2);
        let u33_3 = u33_1 | u33_2;
        assert_eq!(u33_3.value, value_1 | value_2);
        u33_2 |= u33_1;
        assert_eq!(u33_3, u33_2);
    }

    #[test]
    fn test_sub() {
        const MASK: u64 = 7;
        let value_1: u64 = (6 << 6) | (1 << 3) | 3;
        let value_2: u64 = (5 << 6) | (2 << 3) | 5;
        let value_3: u64 = ((5u64.wrapping_sub(6) & MASK) << 6) | (((2 - 1) & MASK) << 3) | (5 - 3);
        let u33_1 = NUintInU64::<3, 3>::new(value_1);
        let mut u33_2 = NUintInU64::<3, 3>::new(value_2);
        let u33_3 = NUintInU64::<3, 3>::new(value_3);
        let u33_4 = u33_2 - u33_1;
        assert_eq!(u33_3, u33_4);
        u33_2 -= u33_1;
        assert_eq!(u33_3, u33_2);
    }

    #[test]
    fn test_add() {
        const MASK: u64 = 7;
        let value_1: u64 = (6 << 6) | (1 << 3) | 3;
        let value_2: u64 = (5 << 6) | (2 << 3) | 5;
        let value_3: u64 = ((5u64.wrapping_add(6) & MASK) << 6) | (((2 + 1) & MASK) << 3) | (5u64.wrapping_add(3) & MASK);
        let u33_1 = NUintInU64::<3, 3>::new(value_1);
        let mut u33_2 = NUintInU64::<3, 3>::new(value_2);
        let u33_3 = NUintInU64::<3, 3>::new(value_3);
        let u33_4 = u33_2 + u33_1;
        assert_eq!(u33_3, u33_4);
        u33_2 += u33_1;
        assert_eq!(u33_3, u33_2);
    }
}

#[cfg(test)]
mod correctness_regressions {
    use num_traits::FromPrimitive;
    use sequencetransform::compoundnumber::NUintInU64;

    #[test]
    fn packed_number_conversion_rejects_invalid_values() {
        type Packed = NUintInU64<4, 8>;
        assert!(Packed::from_i32(-1).is_none());
        assert!(Packed::from_u128(u128::MAX).is_none());
        assert!(Packed::from_f64(f64::NAN).is_none());
        assert!(Packed::from_f64(f64::INFINITY).is_none());
        assert_eq!(Packed::from_u32(42).unwrap().value, 42);
    }
}

#[cfg(test)]
mod additional_regressions {
    use num_traits::FromPrimitive;
    use sequencetransform::compoundnumber::NUintInU64;

    #[test]
    fn invalid_layout_is_rejected_by_conversions() {
        type Invalid = NUintInU64<2, 64>;
        assert!(Invalid::from_u64(1).is_none());
        assert!(Invalid::from_i32(1).is_none());
        assert!(Invalid::from_f64(1.0).is_none());
        type Valid = NUintInU64<1, 64>;
        assert_eq!(Valid::from_u64(u64::MAX).unwrap().get(0), u64::MAX);
    }
}

#[cfg(test)]
mod mathematical_invariants {
    use sequencetransform::compoundnumber::NUintInU64;
    use sequencetransform::distance::{Delta, Distance};

    #[test]
    fn hamming_distance_is_symmetric_and_zero_for_equal_values() {
        type Packed = NUintInU64<4, 8>;
        let a = Packed::new(0x0102_0304);
        let b = Packed::new(0x0102_03ff);
        assert_eq!(a.distance(a), 0);
        assert_eq!(a.distance(b), (a.value ^ b.value).count_ones());
        assert_eq!(a.distance(b), b.distance(a));
    }

    #[test]
    fn per_lane_addition_wraps_without_carrying_into_adjacent_lanes() {
        type Packed = NUintInU64<4, 8>;
        let a = Packed::new(0x00ff_00ff);
        let b = Packed::new(0x0001_0001);
        let result = a + b;
        assert_eq!(result.value, 0);
        for lane in 0..4 {
            assert_eq!(result.get(lane), 0);
        }
    }

    #[test]
    fn delta_of_constant_sequence_is_zero() {
        type Packed = NUintInU64<4, 8>;
        let constant = Packed::new(0x0707_0707);
        assert_eq!(constant.delta().value, 0);
    }

    #[test]
    fn delta_of_ascending_sequence_is_one_after_first_lane() {
        type Packed = NUintInU64<4, 8>;
        let ascending = Packed::new(0x0403_0201);
        assert_eq!(ascending.delta().value, 0x0101_0100);
    }
}
