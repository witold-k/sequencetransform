// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::compoundnumber::U8x8;

#[test]
fn default_slots_are_free() {
    let slots = U8x8::default();
    assert!(slots.empty());
    assert_eq!(slots.size(), 0);
    assert_eq!(slots.free_index(), Some(0));
    assert_eq!(slots.as_u64(), U8x8::FREE_ALL);
}

#[test]
fn push_fills_slots_in_order_and_rejects_overflow() {
    let mut slots = U8x8::default();
    for i in 0..8u8 {
        assert_eq!(slots.free_index(), Some(i));
        assert!(slots.push(i + 10));
        assert_eq!(slots.size(), (i + 1) as usize);
        assert_eq!(slots.find(i + 10), Some(i));
        assert!(slots.has(i + 10));
    }
    assert_eq!(slots.free_index(), None);
    assert!(!slots.empty());
    assert!(!slots.push(99));
    assert_eq!(slots.size(), 8);
    assert!(!slots.has(99));
}

#[test]
fn reserved_free_marker_is_not_insertable() {
    let mut slots = U8x8::default();
    assert!(!slots.push(U8x8::FREE));
    assert_eq!(slots.size(), 0);
    assert!(slots.empty());
}

#[test]
fn packed_representation_uses_big_endian_slot_order() {
    let slots = U8x8 { index: [1, 2, 3, 4, 5, 6, 7, 8] };
    assert_eq!(slots.as_u64(), 0x0102_0304_0506_0708);
    assert_eq!(slots.size(), 8);
}

#[test]
fn filled_slots_preserve_explicit_values() {
    let slots = U8x8::new_fill(7);
    assert_eq!(slots.size(), 8);
    assert_eq!(slots.find(7), Some(0));
    assert_eq!(slots.free_index(), None);
    assert_eq!(slots.as_u64(), 0x0707_0707_0707_0707);
}
