# allwhat

Rust error aggregation tools.

The goal of this is to smooth out validation style errors, where one wants to know all the errors
that occurred rather than just the first.

## Releasing

I'm still working on the initial release, and am building out features in here as I need them. As
user acceptance, I'm going to finish Subpar's CSV reader and a rough stab at updating schemars with
update to ensure the API is complete and works as I intend.

Until I feel this is at a 1.0 level, all new non-breaking changes to the API (including new
features) will increment the hot-fix of the version version, while changes to existing functions
will garner a minor-version increase.

### Alpha Release TODO

These are the remaining items that need to be done before I will release this as a crate

- Finish implementing bulk_try proc_macro for all reasonable expression types
- Make current Subpar CSV Reader project
- Add Filtrate validation to schemars
- Clean up documentation (including/especially this README)
- Add integration tests for each function
- Remove dead code
- Review linting checklist

## Groups

ErrorGroup is a wrapper around a simple vector of errors. There are macros to unwrap error results
into a single unified error.

## Batching

Aggregate errors into an accumulator.

## Splitting

Working with Vec iterables that filter the results into successes and failures.

## Macros

Simplifying extracting errors from multiple sources into a single result.
