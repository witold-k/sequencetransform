# Differentiate

[← Transform index](README.md) · [Rust source](../../src/transform/differentiate.rs)

## Algorithm

Compute first differences between consecutive samples.

## Window diagram

![differentiate window visualization](assets/differentiate-window.svg)

The highlighted region indicates the samples consulted for the final illustrated output. Prefix and history transforms use a different dependency pattern.

## Input versus output

![differentiate effect visualization](assets/differentiate-effect.svg)

**50-sample effect plot:** consecutive differences. The SVG contains 50 plotted values, with input/output data embedded in metadata. The Rust regression test `svg_effect_curves_match_actual_rust_transforms` decodes the orange curve and compares it with the implementation. The short numerical example below is separate.

**Numerical example:** input `[2,5,5,1]` → output `[0,3,0,-4]`. Consecutive differences.

## Boundary conditions and interpretation

The first output is zero.

The blue line is the input and the dashed orange line is the transform output. Values are shown on a common vertical scale.
