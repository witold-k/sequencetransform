// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::accumulate::{AddRaiseWeigth, SequenceAccumulate};
use sequencetransform::ConvertToIdentity;

#[test]
fn addraiseweigth_finite_values_and_empty_input() {
    let data = [1.0_f64, 3.0, 2.0, 4.0];
    let acc = AddRaiseWeigth::<f64>::default();
    assert_eq!(acc.accumulate(data.iter(), ConvertToIdentity::default()), Some(6.0));
    assert_eq!(acc.accumulate([].iter(), ConvertToIdentity::<f64>::default()), None);
    assert_eq!(acc.accumulate([7.0_f64].iter(), ConvertToIdentity::default()), Some(0.0));
}
