// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use sequencetransform::{ConvertToIdentity, TransformResult};
use sequencetransform::transform::{MovingMedianAverage, MovingMedianAverage2, SequenceTransform};

#[test]
fn averages_two_lowest_ranks_then_moving_average() {
    let input = [5.0_f64, 1.0, 7.0, 3.0, 9.0];
    let mut output = [0.0; 5];
    let transform = MovingMedianAverage2::<f64>::new_border(3, 2, 0, 1);
    assert_eq!(transform.transform(input.iter(), &mut output, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    assert_eq!(output, [5.0, 4.0, 3.0, 2.5, 3.5]);
}

#[test]
fn old_median_average_constructor_remains_median() {
    let input = [5.0_f64, 1.0, 7.0, 3.0, 9.0];
    let mut old = [0.0; 5];
    let mut explicit = [0.0; 5];
    let a = MovingMedianAverage::<f64>::new(3, 2);
    let b = MovingMedianAverage::<f64>::new_border(3, 2, 1);
    assert_eq!(a.transform(input.iter(), &mut old, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    assert_eq!(b.transform(input.iter(), &mut explicit, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    assert_eq!(old, explicit);
}

#[test]
fn first_rank_can_be_selected_in_median_average() {
    let input = [5.0_f64, 1.0, 7.0, 3.0, 9.0];
    let mut output = [0.0; 5];
    let transform = MovingMedianAverage::<f64>::new_border(3, 2, 0);
    assert_eq!(transform.transform(input.iter(), &mut output, 0.0, ConvertToIdentity::default()), TransformResult::Ok);
    assert_eq!(output, [5.0, 3.0, 1.0, 1.0, 2.0]);
}
