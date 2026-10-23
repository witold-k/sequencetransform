// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::TransformResult;

#[test]
fn logical_not_is_true_only_for_errors() {
    assert!(!(!TransformResult::Ok));
    assert!(!TransformResult::Error(0));
    assert!(!TransformResult::Error(42));
}

#[test]
fn result_variants_preserve_error_codes() {
    assert_eq!(TransformResult::Error(7), TransformResult::Error(7));
    assert_ne!(TransformResult::Error(7), TransformResult::Error(8));
    assert_ne!(TransformResult::Ok, TransformResult::Error(0));
}
