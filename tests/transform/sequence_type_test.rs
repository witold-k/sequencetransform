// SPDX-License-Identifier: Apache-2.0
use sequencetransform::transform::*;

fn category_of<T: SequenceClassification>() -> SequenceType {
    T::SEQUENCE_TYPE
}

#[test]
fn representative_categories_are_exposed_as_constants() {
    assert_eq!(MovingPitch::<f64>::SEQUENCE_TYPE, SequenceType::Trend);
    assert_eq!(RelativeStrength::<f64>::SEQUENCE_TYPE, SequenceType::Momentum);
    assert_eq!(MovingAreaMedian::<f64>::SEQUENCE_TYPE, SequenceType::TemporalPosition);
    assert_eq!(MovingExtremaConvergence::<f64>::SEQUENCE_TYPE, SequenceType::Convergence);
    assert_eq!(MovingExtremaBisectorSlope::<f64>::SEQUENCE_TYPE, SequenceType::Trend);
    assert_eq!(MovingPermutationEntropy::<f64>::SEQUENCE_TYPE, SequenceType::Complexity);
    assert_eq!(MovingSpectralCentroid::<f64>::SEQUENCE_TYPE, SequenceType::Frequency);
    assert_eq!(MovingOptimalTransport::<f64>::SEQUENCE_TYPE, SequenceType::RegimeChange);
    assert_eq!(MovingMatrixEffectiveRank::<f64>::SEQUENCE_TYPE, SequenceType::Complexity);
    assert_eq!(Order::<f64>::SEQUENCE_TYPE, SequenceType::Processing);
    assert_eq!(MovingDirection::<f64>::SEQUENCE_TYPE, SequenceType::Trend);
    assert_eq!(category_of::<MovingDirection<f64>>(), MovingDirection::<f64>::SEQUENCE_TYPE);
}

#[test]
fn category_is_independent_of_numeric_type() {
    assert_eq!(MovingAverage::<f32>::SEQUENCE_TYPE, MovingAverage::<f64>::SEQUENCE_TYPE);
    assert_eq!(category_of::<MovingAverage<f32>>(), SequenceType::Level);
}
