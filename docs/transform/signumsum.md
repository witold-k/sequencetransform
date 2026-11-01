# SignumSum

[← Transform index](README.md) · [Rust source](../../src/transform/signumsum.rs)

## Algorithm

Accumulate the signs of the input values.

## Window diagram

![signumsum window visualization](assets/signumsum-window.svg)

The highlight shows the samples needed for the final illustrated result. Full-sequence operations use the whole input; pointwise operations use a single value.

## Input versus output

![signumsum effect visualization](assets/signumsum-effect.svg)

**50-sample effect plot:** cumulative input signs. The SVG contains 50 plotted values, with input/output data embedded in metadata. The Rust regression test `svg_effect_curves_match_actual_rust_transforms` decodes the orange curve and compares it with the implementation. The short numerical example below is separate.

**Numerical example:** input `[-2,0,3,-1]` → output `[-1,-1,0,-1]`. Running sum of individual input signs.

## Boundary conditions and interpretation

Counts positive minus negative samples.

The blue line is the input and the dashed orange line is the transform output. Values are shown on a common vertical scale.
