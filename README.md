# allwhat

Rust error aggregation tools.

The goal of this is to smooth out validation style errors, where one wants to know all the errors
that occurred rather than just the first.

## Groups

ErrorGroup is a wrapper around a simple vector of errors. There are macros to unwrap error results
into a single unified error.

## Batching

Aggregate errors into an accumulator.

## Splitting

Working with Vec iterables that filter the results into successes and failures.
