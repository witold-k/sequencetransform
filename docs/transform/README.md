# Transform algorithms

[← Main README](../../README.md)

Each public transform has its own documentation page explaining the algorithm, input/output behavior, and boundary conditions. Most effect diagrams plot numerical examples; the transport, convex-hull, isotonic and matrix-rank pages currently use explicitly labeled conceptual curves. Window diagrams highlight sample dependencies, not additional measurements.

## Individual transforms

- [DeltaSignum](deltasignum.md)
- [DeltaSignumSum](deltasignumsum.md)
- [Differentiate](differentiate.md)
- [Heavyside](heavyside.md)
- [Integrate](integrate.md)
- [MaxDirection](maxdirection.md)
- [MinusDistance](minusdistance.md)
- [MinusDistanceSum](minusdistancesum.md)
- [MovingAverage](movingaverage.md)
- [MovingAreaMedian](movingareamedian.md)
- [MovingExtremaConvergence](movingextremaconvergence.md)
- [MovingExtremaBisectorSlope](movingextremabisectorslope.md)
- [MovingAverageAverage](movingaverageaverage.md)
- [MovingMedian](movingmedian.md)
- [MovingMedian2](movingmedian2.md)
- [MovingMedianAverage](movingmedianaverage.md)
- [MovingMedianAverage2](movingmedianaverage2.md)
- [MovingPitch](movingpitch.md)
- [MovingAutoregression](movingautoregression.md)
- [MovingAutoregressionChange](movingautoregressionchange.md)
- [MovingExplainedVariance](movingexplainedvariance.md)
- [MovingPermutationEntropy](movingpermutationentropy.md)
- [MovingOrdinalSurprise](movingordinalsurprise.md)
- [MovingLempelZivComplexity](movinglempelzivcomplexity.md)
- [MovingOptimalTransport](movingoptimaltransport.md)
- [MovingConvexHullDeficit](movingconvexhulldeficit.md)
- [MovingIsotonicResidual](movingisotonicresidual.md)
- [MovingMatrixEffectiveRank](movingmatrixeffectiverank.md)
- [MovingTimeReversalAsymmetry](movingtimereversalasymmetry.md)
- [MovingSpectralEntropy](movingspectralentropy.md)
- [MovingSpectralCentroid](movingspectralcentroid.md)
- [MovingSpectralChange](movingspectralchange.md)
- [MovingHiguchiDimension](movinghiguchidimension.md)
- [MovingMultifractalCurvature](movingmultifractalcurvature.md)
- [MovingRecurrenceDeterminism](movingrecurrencedeterminism.md)
- [OneHotQuantization](onehotquantization.md)
- [Order](order.md)
- [RelativeStrength](relativestrength.md)
- [RelativeStrengthWeigth](relativestrengthweigth.md)
- [RelativeStrengthWeigthSum](relativestrengthweigthsum.md)
- [Signum](signum.md)
- [SignumSum](signumsum.md)
- [Sort](sort.md)
- [MovingWeightedMedianPosition](movingweightedmedianposition.md)
- [MovingSquaredWeightedMedianPosition](movingsquaredweightedmedianposition.md)
- [MovingPositiveStrength](movingpositivestrength.md)
- [MovingDirection](movingdirection.md)
- [MovingMaxCrossingLevel](movingmaxcrossinglevel.md)
- [MovingMeanSquareError](movingmeansquareerror.md)
- [MovingRootMeanSquareError](movingrootmeansquareerror.md)

## Architecture

- `SequenceTransform`: processes one input sequence and writes results into the caller-provided output slice.
- `ZipSequenceTransform`: processes two sequences; MSE and RMSE treat the second sequence as the reference signal.
- Moving windows generally use current and past values only. Exceptions, including noncausal initial padding, are documented per algorithm.
- `ConvertTo` allows different numeric input and output types where the transform supports it.

## Notes

These are experimental algorithms. Before using them as point-in-time ML features, verify prefix handling, numerical scaling, missing values, and causal alignment. Where an illustration differs from a directly tested fixture, it must be labeled as schematic.

## Fifty-sample plots and numeric verification

Most transform pages include a 50-sample input/output SVG with its window sketch separate. Numeric regression plots store 50 input values, 50 output values and coordinate scales in machine-readable metadata. The four new geometric/matrix transforms currently have conceptual effect diagrams without numerical metadata and are excluded from the roundtrip test. The regression test `tests/transform/svg_roundtrip_test.rs` reads those values, reconstructs numbers from the plotted SVG Y coordinates, and compares both with the actual Rust implementation. Run `cargo test svg_effect_curves_match_actual_rust_transforms` after changing a plot. The figures are not considered verified until this test passes.

For MSE/RMSE, the blue curve is pointwise squared error between two signals. For OneHotQuantization, the orange curve is the chosen bin index, not the expanded one-hot output. `Order` and `Sort` operate on the whole sequence, not causally over time.

### Independent vertical scales

The blue input curve uses the **left Y-axis**, while the orange output curve uses the **right Y-axis**. Each axis is independently scaled from its own series minimum to maximum, so both curves use the full plot height even when their numerical magnitudes differ greatly. The two Y-axis values at the same screen height are **not numerically equal**. Constant series remain horizontal at mid-height because their minimum equals their maximum. Both plotted curves and both numeric axis ranges are checked by `svg_effect_curves_match_actual_rust_transforms`.


## SequenceType: primary property classification

Every transformation in `transform` exposes a public associated constant
`SEQUENCE_TYPE: SequenceType`. The `SequenceClassification` trait exposes
the same constant for generic code. Classifications are declared centrally
in [src/transform/mod.rs](../../src/transform/mod.rs).

```rust
use sequencetransform::transform::{MovingPitch, SequenceType};
assert_eq!(MovingPitch::<f64>::SEQUENCE_TYPE, SequenceType::Trend);
```

The categories are: `Trend`, `Momentum`, `Volatility`, `Level`,
`TemporalPosition`, `Convergence`, `Complexity`, `Dependence`,
`Predictability`, `Frequency`, `RegimeChange`, `Geometry`,
`Recurrence`, `Asymmetry`, and `Processing`.

Each transform has **one primary category**, not an exhaustive list of
possible statistical relationships. `Processing` denotes transformations
that manipulate or encode a sequence rather than measuring a property.
`Volatility` is reserved for direct fluctuation-magnitude estimators;
none of the currently classified transforms is a direct volatility estimator.
The category is available for future transforms.

The classification is useful for restricting candidate feature combinations,
but does not guarantee low correlation or predictive value within or across
categories. Measure redundancy on training data before relying on it.
