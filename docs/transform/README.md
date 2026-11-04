# Transform algorithms

[← Main README](../../README.md)

Each public transform has its own documentation page explaining the algorithm, input/output behavior, numerical examples, and boundary conditions. The effect diagrams plot the numerical examples stated on each page. Window diagrams highlight sample dependencies, not additional measurements.

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
- [MovingTimeReversalAsymmetry](movingtimereversalasymmetry.md)
- [MovingSpectralEntropy](movingspectralentropy.md)
- [MovingSpectralCentroid](movingspectralcentroid.md)
- [MovingSpectralChange](movingspectralchange.md)
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

Every transform page includes a 50-sample input/output SVG. Its window sketch is separate. The SVG stores 50 input values, 50 output values and its coordinate scale in machine-readable metadata. The regression test `tests/transform/svg_roundtrip_test.rs` reads those values, reconstructs numbers from the plotted SVG Y coordinates, and compares both with the actual Rust implementation. Run `cargo test svg_effect_curves_match_actual_rust_transforms` after changing a plot. The figures are not considered verified until this test passes.

For MSE/RMSE, the blue curve is pointwise squared error between two signals. For OneHotQuantization, the orange curve is the chosen bin index, not the expanded one-hot output. `Order` and `Sort` operate on the whole sequence, not causally over time.

### Independent vertical scales

The blue input curve uses the **left Y-axis**, while the orange output curve uses the **right Y-axis**. Each axis is independently scaled from its own series minimum to maximum, so both curves use the full plot height even when their numerical magnitudes differ greatly. The two Y-axis values at the same screen height are **not numerically equal**. Constant series remain horizontal at mid-height because their minimum equals their maximum. Both plotted curves and both numeric axis ranges are checked by `svg_effect_curves_match_actual_rust_transforms`.
