# DeltaSignumSum

[← Transform index](README.md) · [Rust source](../../src/transform/deltasignumsum.rs)

## Algorithm

Accumulate the direction of consecutive changes.

## Window diagram

![deltasignumsum window visualization](assets/deltasignumsum-window.svg)

The highlighted region indicates the samples consulted for the final illustrated output. Prefix and history transforms use a different dependency pattern.

## Input versus output

![deltasignumsum effect visualization](assets/deltasignumsum-effect.svg)

**50-sample effect plot:** cumulative direction signs. The SVG contains 50 plotted values, with input/output data embedded in metadata. The Rust regression test `svg_effect_curves_match_actual_rust_transforms` decodes the orange curve and compares it with the implementation. The short numerical example below is separate.

**Numerical example:** input `[2,5,5,1]` → output `[0,1,1,0]`. Cumulative sum of consecutive direction signs.

## Boundary conditions and interpretation

Counts rises minus falls, not the total price change.

The blue line is the input and the dashed orange line is the transform output. Values are shown on a common vertical scale.
