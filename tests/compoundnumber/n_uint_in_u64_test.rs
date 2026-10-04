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
