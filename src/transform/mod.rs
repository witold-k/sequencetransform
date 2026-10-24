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
pub mod minusdistance;
pub mod minusdistancesum;
pub mod movingaverage;
pub mod movingaverageaverage;
pub mod movingmedian;
pub mod movingmedianaverage;
pub mod movingmedian2;
pub mod movingmedianaverage2;
pub mod movingpitch;
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

/// Look-ahead signed distance to a future level crossing.
/// Reads forward but writes output in reverse order; this is not an adjacent-sample delta.
/// The implementation may leave some output slots untouched on early termination.
#[derive(Copy, Clone)]
pub struct MinusDistance<T> { marker: PhantomData<T> }

/// Accumulate signed distances from the look-ahead level-crossing search.
/// Writes in reverse order and can stop early, leaving some output slots unchanged.
/// This is not a conventional cumulative sum of the input.
#[derive(Copy, Clone)]
pub struct MinusDistanceSum<T> { marker: PhantomData<T> }

/// Trailing arithmetic mean over up to `count` samples.
/// Initial positions use the available prefix; `count` must be positive and fit the input.
#[derive(Copy, Clone)]
pub struct MovingAverage<T> { marker: PhantomData<T>, pub count: usize }

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

