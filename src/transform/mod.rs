// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::marker::PhantomData;
use std::vec::Vec;
use num_traits::cast::FromPrimitive;
use crate::{ Number, TransformReadIterator, TransformResult, ConvertTo };

pub mod deltasignum;
pub mod deltasignumsum;
pub mod differentiate;
pub mod heavyside;
pub mod integrate;
mod minusdistance_core;
pub mod minusdistance;
pub mod minusdistancesum;
pub mod maxdirection;
pub mod movingaverage;
pub mod movingareamedian;
pub mod movingextremaconvergence;
pub mod movingextremabisectorslope;
pub mod movingaverageaverage;
pub mod movingmedian;
pub mod movingmedianaverage;
pub mod movingmedian2;
pub mod movingmedianaverage2;
pub mod movingpitch;
pub mod movingautoregression;
pub mod movingautoregressionchange;
pub mod movingexplainedvariance;
pub mod movingpermutationentropy;
pub mod movingordinalsurprise;
pub mod movinglempelzivcomplexity;
pub mod movingtimereversalasymmetry;
mod movingspectral_core;
mod movingfractal_core;
mod movinggeometry_core;
pub mod movingoptimaltransport;
pub mod movingconvexhulldeficit;
pub mod movingisotonicresidual;
pub mod movingmatrixeffectiverank;
pub mod movinghiguchidimension;
pub mod movingmultifractalcurvature;
pub mod movingrecurrencedeterminism;
pub mod movingspectralentropy;
pub mod movingspectralcentroid;
pub mod movingspectralchange;
pub mod onehotquantization;
pub mod order;
pub mod relativestrength;
pub mod relativestrengthweigth;
pub mod relativestrengthweigthsum;
pub mod signum;
pub mod sort;
pub mod signumsum;

// ---------------------------------------------------------------------------

/// Compare each sample with its predecessor and emit -1, 0, or +1.
/// The first output is zero; equal consecutive values emit zero.
#[derive(Copy, Clone)]
pub struct DeltaSignum<T> { marker: PhantomData<T> }

/// Cumulative sum of the direction of consecutive changes (-1, 0, +1).
/// Starts at zero and counts rises minus falls, not price differences.
#[derive(Copy, Clone)]
pub struct DeltaSignumSum<T> { marker: PhantomData<T> }

/// First discrete difference: output[i] = input[i] - input[i-1].
/// The first output is zero; subsequent differences are converted to T.
#[derive(Copy, Clone)]
pub struct Differentiate<T> { marker: PhantomData<T> }

/// Emit 1 when a sample is greater than or equal to its predecessor, else 0.
/// The first output is zero. Equality counts as a non-decrease.
#[derive(Copy, Clone)]
pub struct Heavyside<T> { marker: PhantomData<T> }

/// Discrete prefix sum of converted input samples.
/// The first output equals the first input; later outputs accumulate all preceding values.
#[derive(Copy, Clone)]
pub struct Integrate<T> { marker: PhantomData<T> }

/// Signed distance to a past level crossing after a deviation.
/// Skips the immediate predecessor and emits results in chronological order.
#[derive(Copy, Clone)]
pub struct MinusDistance<T> { marker: PhantomData<T> }

/// Chronological cumulative sum of historical signed level-crossing distances.
/// No future samples are consulted.
#[derive(Copy, Clone)]
pub struct MinusDistanceSum<T> { marker: PhantomData<T> }

/// Slope from the more recent of the trailing minimum and maximum to the current sample.
/// `count` is the number of preceding samples considered; initial prefixes use available history.
/// The first output is zero, and repeated extrema use their most recent occurrence.
#[derive(Copy, Clone)]
pub struct MaxDirection<T> { marker: PhantomData<T>, pub count: usize }

/// Trailing arithmetic mean over up to `count` samples.
/// Initial positions use the available prefix; `count` must be positive and fit the input.
#[derive(Copy, Clone)]
pub struct MovingAverage<T> { marker: PhantomData<T>, pub count: usize }

/// Normalized time at which half the area above the trailing minimum is reached.
/// Uses trapezoidal integration with interpolation; incomplete windows use default_out.
#[derive(Copy, Clone)]
pub struct MovingAreaMedian<T> { marker: PhantomData<T>, pub count: usize }

/// Signed normalized angle between lines through the two highest and two lowest samples.
/// Positive means converging at the current edge; negative means diverging.
#[derive(Copy, Clone)]
pub struct MovingExtremaConvergence<T> { marker: PhantomData<T>, pub count: usize }

/// Slope of the right-facing angle bisector of the upper and lower extrema lines.
/// Time and values are normalized to the unit square; incomplete windows use default_out.
#[derive(Copy, Clone)]
pub struct MovingExtremaBisectorSlope<T> { marker: PhantomData<T>, pub count: usize }

/// Apply two trailing arithmetic means in sequence.
/// `first_count` and `second_count` are the respective window sizes.
/// Initial windows use available prefixes; floating-point rounding can differ from two passes.
#[derive(Copy, Clone)]
pub struct MovingAverageAverage<T> { marker: PhantomData<T>, pub first_count: usize, pub second_count: usize }

/// Select a zero-based order statistic in each trailing sorted window.
/// `count` is window length; `border` is the selected ascending rank (0 = minimum).
/// `new(count)` selects the median; initial windows use available prefixes.
#[derive(Copy, Clone)]
pub struct MovingMedian<T> { marker: PhantomData<T>, pub count: usize, pub border: usize }

/// Apply a trailing order-statistic selector, then a trailing arithmetic mean.
/// `median_count` controls the sorted window; `border` selects its zero-based rank.
/// `average_count` controls smoothing. `new()` retains median selection.
#[derive(Copy, Clone)]
pub struct MovingMedianAverage<T> { marker: PhantomData<T>, pub median_count: usize, pub average_count: usize, pub border: usize }

/// Average two selected order statistics from each trailing sorted window.
/// `count` is window size; `first_border` and `second_border` are zero-based ranks.
/// Initial windows clamp ranks to their last available position; integer output truncates.
#[derive(Copy, Clone)]
pub struct MovingMedian2<T> { marker: PhantomData<T>, pub count: usize, pub first_border: usize, pub second_border: usize }

/// Smooth the mean of two trailing-window order statistics.
/// `median_count` and the two zero-based borders define the selection stage;
/// `average_count` defines the subsequent trailing arithmetic mean.
#[derive(Copy, Clone)]
pub struct MovingMedianAverage2<T> { marker: PhantomData<T>, pub median_count: usize, pub average_count: usize, pub first_border: usize, pub second_border: usize }

/// Estimate the slope of a linear regression over each trailing window.
/// `count` sets the regression window; `skip_first` controls initial outputs.
/// With `skip_first = true`, the initial values are backfilled noncausally.
#[derive(Copy, Clone)]
pub struct MovingPitch<T> { marker: PhantomData<T>, pub count: usize, pub skip_first: bool }

/// Trailing AR(1) OLS coefficient with intercept. The first count-1 outputs
/// and windows with zero predictor variance retain default_out.
/// count >= 3; only past and current samples are consulted.
#[derive(Copy, Clone)]
pub struct MovingAutoregression<T> { marker: PhantomData<T>, pub count: usize }

/// Absolute difference of AR(1) slopes between disjoint older and newer half-windows.
/// Even count >= 6; undefined slopes and incomplete prefixes retain default_out.
#[derive(Copy, Clone)]
pub struct MovingAutoregressionChange<T> { marker: PhantomData<T>, pub count: usize }

/// Trailing AR(1) explained variance (squared Pearson correlation) with intercept.
/// Undefined constant windows and incomplete prefixes retain default_out.
#[derive(Copy, Clone)]
pub struct MovingExplainedVariance<T> { marker: PhantomData<T>, pub count: usize }

/// Normalized Shannon entropy of ordinal patterns in a trailing window.
/// Ties are broken by earlier sample position; order is restricted to 2..=7.
#[derive(Copy, Clone)]
pub struct MovingPermutationEntropy<T> { marker: PhantomData<T>, pub count: usize, pub order: usize }

/// Dimensionless normalized third moment of adjacent differences in trailing windows.
/// Odd under time reversal, invariant to positive affine scaling.
/// Incomplete or constant windows retain default_out; count >= 2.
#[derive(Copy, Clone)]
pub struct MovingTimeReversalAsymmetry<T> { marker: PhantomData<T>, pub count: usize }

/// Normalized Shannon entropy of the non-DC one-sided Hann-tapered power spectrum.
#[derive(Copy, Clone)]
pub struct MovingSpectralEntropy<T> { marker: PhantomData<T>, pub count: usize }

/// Normalized frequency-weighted centroid of the non-DC Hann power spectrum.
#[derive(Copy, Clone)]
pub struct MovingSpectralCentroid<T> { marker: PhantomData<T>, pub count: usize }

/// Total-variation distance between normalized spectra of two disjoint halves.
#[derive(Copy, Clone)]
pub struct MovingSpectralChange<T> { marker: PhantomData<T>, pub count: usize }

/// Higuchi fractal dimension estimate over trailing windows (count >= 16).
#[derive(Copy, Clone)]
pub struct MovingHiguchiDimension<T> { marker: PhantomData<T>, pub count: usize }

/// Difference between q=1 and q=4 normalized structure-function slopes.
#[derive(Copy, Clone)]
pub struct MovingMultifractalCurvature<T> { marker: PhantomData<T>, pub count: usize }

/// Recurrence-plot determinism from two-dimensional delay vectors.
#[derive(Copy, Clone)]
pub struct MovingRecurrenceDeterminism<T> { marker: PhantomData<T>, pub count: usize }

/// Laplace-smoothed held-out ordinal transition surprise; count >= order+2.
#[derive(Copy, Clone)]
pub struct MovingOrdinalSurprise<T> { marker: PhantomData<T>, pub count: usize, pub order: usize }

/// Normalized LZ76 complexity of three-symbol consecutive differences.
#[derive(Copy, Clone)]
pub struct MovingLempelZivComplexity<T> { marker: PhantomData<T>, pub count: usize }

/// Wasserstein-1 distance between sorted disjoint halves of a trailing even window.
#[derive(Copy, Clone)]
pub struct MovingOptimalTransport<T> { marker: PhantomData<T>, pub count: usize }

/// One minus the normalized planar convex-hull area of the trailing graph.
#[derive(Copy, Clone)]
pub struct MovingConvexHullDeficit<T> { marker: PhantomData<T>, pub count: usize }

/// Minimum increasing/decreasing isotonic squared residual over total variance.
#[derive(Copy, Clone)]
pub struct MovingIsotonicResidual<T> { marker: PhantomData<T>, pub count: usize }

/// Entropy-based effective rank of a mean-centered trailing Hankel matrix.
/// The CPU backend of svdwrapper performs the reduced SVD.
#[derive(Copy, Clone)]
pub struct MovingMatrixEffectiveRank<T> { marker: PhantomData<T>, pub count: usize, pub columns: usize }

/// Quantize each sample to a one-hot vector with `factor` output bins.
/// Output length is input length times `factor`; values are scaled, rounded,
/// and clamped to valid bin indices. Nonfinite inputs are rejected.
#[derive(Copy, Clone)]
pub struct OneHotQuantization<T> { marker: PhantomData<T>, pub factor: usize }

/// Assign distinct ascending ranks at the original input positions.
/// Equal values still receive distinct ranks; `factor` optionally scales ranks.
/// Non-comparable input values are rejected.
#[derive(Copy, Clone)]
pub struct Order<T>
where T : Number<T> { pub factor: T }

/// Compute a windowed relative-strength ratio from rises and falls.
/// `count` controls the lookback; `skip_first` determines treatment of prefixes.
#[derive(Copy, Clone)]
pub struct RelativeStrength<T> { marker: PhantomData<T>, pub count: usize, pub skip_first: bool }

/// Windowed relative strength with weighted contributions from rises and falls.
/// `count` sets the lookback; `skip_first` controls prefix handling.
#[derive(Copy, Clone)]
pub struct RelativeStrengthWeigth<T> { marker: PhantomData<T>, pub count: usize, pub skip_first: bool }

/// Windowed accumulated weighted rise/fall strength.
/// `count` sets the lookback; `skip_first` controls prefix handling.
#[derive(Copy, Clone)]
pub struct RelativeStrengthWeigthSum<T> { marker: PhantomData<T>, pub count: usize, pub skip_first: bool }

/// Map each sample to -1, 0, or +1 according to its sign.
/// This is a pointwise transform; zero remains zero.
#[derive(Copy, Clone)]
pub struct Signum<T> { marker: PhantomData<T> }

/// Compute the cumulative sum of input signs (-1, 0, +1).
/// Counts positive minus negative samples, not the sum of magnitudes.
#[derive(Copy, Clone)]
pub struct SignumSum<T> { marker: PhantomData<T> }

// ---------------------------------------------------------------------------

pub trait SequenceTransform<ITEMOUT>
where
    ITEMOUT: 'static + Copy + FromPrimitive
{
    /**
     * return amout of iteration
     */
    fn transform<'a, RI, ITEMIN, CTO>(
        &self,
        ri:          RI,
        out:         &mut [ITEMOUT],
        default_out: ITEMOUT,
        conv:        CTO
    ) -> TransformResult
    where
        RI:     TransformReadIterator<Item = &'a ITEMIN>,
        ITEMIN: 'a + Number<ITEMIN>,
        CTO:    ConvertTo<ITEMIN, ITEMOUT>,
    ;
}


pub mod movingindicators;
pub use movingindicators::{MovingWeightedMedianPosition, MovingSquaredWeightedMedianPosition, MovingPositiveStrength, MovingDirection, MovingMaxCrossingLevel};


/// Primary kind of information represented by a sequence transformation.
///
/// This is a coarse, mutually exclusive grouping intended for feature selection.
/// It does not assert predictive power or statistical independence; some
/// transformations also contain information belonging to other categories.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SequenceType {
    /// Direction or slope of the sequence.
    Trend,
    /// Relative upward/downward movement and directional impulses.
    Momentum,
    /// Magnitude or dispersion of fluctuations.
    Volatility,
    /// Central tendency, rank-based level or reference level.
    Level,
    /// Position of an event or mass within the observation window.
    TemporalPosition,
    /// Narrowing or widening of geometric boundaries.
    Convergence,
    /// Irregularity, entropy or effective dimensionality.
    Complexity,
    /// Temporal or autoregressive dependence.
    Dependence,
    /// How well a simple model or transition pattern explains the sequence.
    Predictability,
    /// Spectral location or frequency content.
    Frequency,
    /// Difference between successive portions of a window.
    RegimeChange,
    /// Shape of the time-value curve.
    Geometry,
    /// Repetition and recurrence structure.
    Recurrence,
    /// Time-direction asymmetry.
    Asymmetry,
    /// Encoding, sorting, accumulation or other data operations.
    Processing,
}

/// Type-level access to the primary sequence property.
pub trait SequenceClassification {
    const SEQUENCE_TYPE: SequenceType;
}

// Keep classification in one auditable table, rather than spreading it
// across implementation modules. Each transform also exposes an inherent
// `SEQUENCE_TYPE` constant for ergonomic access.
macro_rules! classify_sequence_transforms {
    ($($name:ident => $kind:ident),+ $(,)?) => {
        $(
            impl<T> SequenceClassification for $name<T> {
                const SEQUENCE_TYPE: SequenceType = SequenceType::$kind;
            }
            impl<T> $name<T> {
                pub const SEQUENCE_TYPE: SequenceType = SequenceType::$kind;
            }
        )+
    };
}
classify_sequence_transforms!(
    DeltaSignum => Momentum,
    DeltaSignumSum => Momentum,
    Differentiate => Momentum,
    Heavyside => Momentum,
    Integrate => Processing,
    MinusDistance => TemporalPosition,
    MinusDistanceSum => TemporalPosition,
    MaxDirection => Trend,
    MovingAverage => Level,
    MovingAreaMedian => TemporalPosition,
    MovingExtremaConvergence => Convergence,
    MovingExtremaBisectorSlope => Trend,
    MovingAverageAverage => Level,
    MovingMedian => Level,
    MovingMedianAverage => Level,
    MovingMedian2 => Level,
    MovingMedianAverage2 => Level,
    MovingPitch => Trend,
    MovingAutoregression => Dependence,
    MovingAutoregressionChange => RegimeChange,
    MovingExplainedVariance => Predictability,
    MovingPermutationEntropy => Complexity,
    MovingTimeReversalAsymmetry => Asymmetry,
    MovingSpectralEntropy => Complexity,
    MovingSpectralCentroid => Frequency,
    MovingSpectralChange => RegimeChange,
    MovingHiguchiDimension => Complexity,
    MovingMultifractalCurvature => Complexity,
    MovingRecurrenceDeterminism => Recurrence,
    MovingOrdinalSurprise => Predictability,
    MovingLempelZivComplexity => Complexity,
    MovingOptimalTransport => RegimeChange,
    MovingConvexHullDeficit => Geometry,
    MovingIsotonicResidual => Trend,
    MovingMatrixEffectiveRank => Complexity,
    OneHotQuantization => Processing,
    Order => Processing,
    RelativeStrength => Momentum,
    RelativeStrengthWeigth => Momentum,
    RelativeStrengthWeigthSum => Momentum,
    Signum => Processing,
    SignumSum => Momentum,
    MovingWeightedMedianPosition => TemporalPosition,
    MovingSquaredWeightedMedianPosition => TemporalPosition,
    MovingPositiveStrength => Momentum,
    MovingDirection => Trend,
    MovingMaxCrossingLevel => Level
);
