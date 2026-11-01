# SVG requirements

This document specifies the requirements for the illustrated transform reference in `docs/transform/`. It applies to every `*-effect.svg` and the accompanying window/dependency diagrams.

## Purpose and fidelity

- Explain **both** the observable effect of a transform and **how its algorithm produces an output**. A generic highlighted window alone is not an algorithm explanation.
- Every construction (line, arrow, shaded area, threshold, selected rank, or reference point) must correspond to the **actual Rust implementation**, including prefix behavior, tie-breaking, indexing, signs, and special cases.
- **Omit an annotation rather than inventing a misleading one.** If a construction is only conceptual, label it explicitly as schematic; do not present it as a computed result.
- Do not change production algorithms to make a diagram appear correct. Fix the diagram or its test instead.

## Data and axes

- Plot **50 samples** of input and output; avoid four-point plots that conceal the transform's behavior.
- Use **independent vertical scales**: blue input on the **left** and orange output on the **right**. Each scale spans its own data minimum and maximum and is visibly labeled with numeric ticks. The same pixel height does not imply the same numeric value.
- For a constant series, draw a horizontal line at mid-height; record its actual constant min/max in metadata.
- Preserve the actual sequence order and sample indices. Use distinct stroke styles as well as colors (solid blue input, dashed orange output) for color-vision accessibility.
- When a transform has two inputs (MSE/RMSE), explicitly describe any derived series plotted in place of the original inputs. For one-hot output, distinguish the plotted bin index from the actual expanded vector. For whole-sequence transforms, do not imply causal time-series behavior.
- Keep the plot legible: labels, axes, highlighted constructions and legends must not overlap or obscure the main curves unnecessarily.

## Algorithm-specific overlays

- `Differentiate`: connect the actual consecutive input samples; show the rise/run triangle and discrete slope.
- `Integrate`: shade the signed, discrete summands relative to the **input-axis zero**; indicate the accumulated prefix and output point. Do not suggest continuous integration.
- `MinusDistance`: show the causal search and an arrow to the **actual reference index** determined by `minusdistance_core.rs`. Distinguish the first differing sample from the crossing-search result, including the no-crossing fallback. Validate the displayed signed distance.
- Moving means/medians: highlight the actual trailing window and the selected average or order statistic, including initial prefix handling.
- Direction/strength: distinguish upward and downward changes; respect counting versus magnitude weighting and any initial-sample behavior.
- Weighted positions: illustrate the two-sided balancing rule, not a generic median or center-of-mass formula.
- Crossing levels: show the selected candidate level and crossings according to the actual discrete search and tie handling.
- Other transforms: use a faithful construction tailored to their implementation (e.g. sign threshold, sorted rank, cumulative contribution, regression slope). If its correctness cannot be established, **leave the overlay out** and record the limitation.

## Numerical verification

- Embed the **50 input and 50 output values** in SVG metadata, plus separate `input_range` and `output_range` arrays and coordinate mapping.
- Reconstruct the plotted input and output values from SVG path coordinates; compare them to the embedded data within explicitly bounded pixel-rounding tolerance.
- Compare the embedded output against the **real Rust transform**, not merely an independent graphics formula. The regression test is `tests/transform/svg_roundtrip_test.rs` (`cargo test svg_effect_curves_match_actual_rust_transforms`).
- Verify algorithm overlays independently: their sample indices, coordinates, selected references, arithmetic and boundary cases must be testable or documented with the exact source rule.
- Run the test suite after edits. Do not claim the diagrams are fully algorithm-verified until the Rust regression test passes. Report unresolved discrepancies rather than hiding them.

## Documentation and workflow

- Keep explanations and SVG text **in English**.
- Each transform page should identify its parameters and explain what the highlighted construction means. Retain the separate window/dependency diagram where useful.
- Keep source changes, tests and documentation consistent. Do not add misleading visual decoration just to make every page look alike.
- Work on a feature branch; **do not create a PR, merge or commit to `master` without explicit authorization**.
