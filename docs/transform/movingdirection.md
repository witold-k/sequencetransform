# MovingDirection

[← Transform index](README.md) · [Rust source](../../src/transform/movingindicators.rs)

## Algorithm

Compute (number of rises − number of falls)/(number of rises + number of falls).

## Window diagram

![movingdirection window visualization](assets/movingdirection-window.svg)

The highlighted region marks the samples considered for the final output of this algorithm.

## Input versus output

![movingdirection effect visualization](assets/movingdirection-effect.svg)

**50-sample effect plot:** count=10. The SVG contains 50 plotted values, with input/output data embedded in metadata. The Rust regression test `svg_effect_curves_match_actual_rust_transforms` decodes the orange curve and compares it with the implementation. The short numerical example below is separate.

**Numerical example:** input `[1,3,2,5]` → output `[0,1,0,0.3333333333333333]`. Directional balance of up/down steps, count=4.

## Interpretation and caveats

count=4; ties are excluded.

Blue solid: input (squared per-sample error for MSE/RMSE); orange dashed: output. Both are plotted on one numeric scale.
