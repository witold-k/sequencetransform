# DeltaSignum

[← Transform index](README.md) · [Rust source](../../src/transform/deltasignum.rs)

## Algorithm

Compare consecutive values and emit the direction of change.

## Window diagram

![deltasignum window visualization](assets/deltasignum-window.svg)

The highlighted region indicates the samples consulted for the final illustrated output. Prefix and history transforms use a different dependency pattern.

## Input versus output

![deltasignum effect visualization](assets/deltasignum-effect.svg)

**50-sample effect plot:** pointwise consecutive comparison. The SVG contains 50 plotted values, with input/output data embedded in metadata. The Rust regression test `svg_effect_curves_match_actual_rust_transforms` decodes the orange curve and compares it with the implementation. The short numerical example below is separate.

**Numerical example:** input `[2,5,5,1]` → output `[0,1,0,-1]`. Sign of consecutive differences.

## Boundary conditions and interpretation

The first output is zero. Equal consecutive values produce zero.

The blue line is the input and the dashed orange line is the transform output. Values are shown on a common vertical scale.
