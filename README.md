# sequencetransform

One of my first attempts to learn Rust.

This is a helper library originally written for a larger stock-analysis experiment, more precisely for experiments around technical analysis.

The idea was not to implement a fixed set of well-known indicators. Instead, an indicator was treated as a composition of smaller operations on sequences. The price-dependent part (for example combinations of open, high, low and close values) lives outside this crate. This crate focuses on the time-axis: how values change, how windows are summarized, how events are selected and how several derived sequences can be combined into new signals.

## What it provides

The library contains small generic building blocks for numeric sequences:

- transforms such as differentiation, integration, sign, moving average, moving median, linear-regression slope and relative-strength variants;
- accumulators for sums, rises, falls and weighted variants;
- selectors such as min, max and median;
- helpers for retaining or matching values in sorted sequences;
- trigger-based transforms that combine two sequences;
- ordering and tokenization of sliding windows, including compact integer representations;
- utility types for packed small integers and iterator-based processing.

Most operations work on iterators and write into caller-provided output slices. Input and output numeric types can be separated through the `ConvertTo` abstraction.

## How it works

The central abstraction is `SequenceTransform`. A transform reads a cloneable, exact-size, double-ended iterator, applies one operation over the sequence and writes the result into an output slice.

More complex transforms are built from simpler components. For example, moving-window transforms reuse the accumulator and selector modules, while `ZipSequenceTransform` combines a data sequence with a second trigger sequence. `SortedToUInt` goes one step further: it takes overlapping windows, replaces the values by their relative order inside the window and packs that order into an integer token.

The result is a toolbox for constructing experimental signal-processing pipelines instead of one monolithic technical-analysis implementation.

## Status

The original stock-analysis experiment was not successful and the project is currently stalled. The library is kept because some of the sequence-processing ideas may become useful again, possibly in combination with later AI experiments.

Apache-2.0
