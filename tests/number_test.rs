// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{Number, UNumber};

fn require_signed_number<T: Number<T>>() {}
fn require_unsigned_number<T: UNumber<T>>() {}

#[test]
fn primitive_floating_point_types_implement_number() {
    require_signed_number::<f32>();
    require_signed_number::<f64>();
}

#[test]
fn primitive_unsigned_types_implement_unumber() {
    require_unsigned_number::<u8>();
    require_unsigned_number::<u16>();
    require_unsigned_number::<u32>();
}
