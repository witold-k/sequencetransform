// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertTo, ConvertToIdentity};

#[test]
fn identity_conversion_preserves_floating_point_values() {
    let converter = ConvertToIdentity::<f64>::default();
    for value in [-12.5_f64, -0.0, 0.0, 3.25, f64::INFINITY] {
        assert_eq!(converter.convert(value), value);
        assert_eq!(converter.convert_ref(&value), value);
    }
}

#[test]
fn identity_reference_conversion_clones_non_copy_values() {
    let converter = ConvertToIdentity::<String>::default();
    let original = String::from("hello");
    let converted = converter.convert_ref(&original);
    assert_eq!(converted, original);
    assert_eq!(original, "hello");
    assert_eq!(converter.convert(String::from("world")), "world");
}
