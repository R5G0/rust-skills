# api-range-bounds

> Use standard range types for interval APIs and validate endpoint conversions

## Why It Matters

A range type makes inclusive, exclusive, and open-ended interval semantics visible at the call site. `RangeBounds<T>` can accept several range forms when the algorithm supports them. Normalizing an excluded lower bound or included upper bound may require addition; unchecked arithmetic can overflow before the final bounds check.

## Bad

```rust
// Callers cannot see whether high is included.
fn select(values: &[u8], low: usize, high: usize) -> &[u8] {
    &values[low..high + 1] // Overflow and invalid ranges can panic.
}
```

## Good

```rust
use std::ops::{Bound, RangeBounds};

fn select<T>(values: &[T], range: impl RangeBounds<usize>) -> Option<&[T]> {
    let start = match range.start_bound() {
        Bound::Included(&index) => index,
        Bound::Excluded(&index) => index.checked_add(1)?,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(&index) => index.checked_add(1)?,
        Bound::Excluded(&index) => index,
        Bound::Unbounded => values.len(),
    };
    values.get(start..end)
}

fn main() {
    let values = [10, 20, 30, 40];
    assert_eq!(select(&values, 1..=2), Some(&values[1..3]));
    assert_eq!(select(&values, 2..), Some(&values[2..]));
    assert_eq!(select(&values, ..), Some(&values[..]));
    assert_eq!(select(&values, 4..4), Some(&values[4..4]));
    assert_eq!(select(&values, 3..1), None);
    assert_eq!(select(&values, ..=usize::MAX), None);
    assert_eq!(
        select(&values, (Bound::Excluded(usize::MAX), Bound::Unbounded)),
        None
    );
}
```

## Choose the Supported Forms

Accept `Range<usize>` when a half-open interval is the intended contract.
Use `RangeBounds<usize>` when inclusive and unbounded forms are useful and their
meaning is defined. Do not generalize point parameters or unrelated pairs into
ranges merely for consistency.

Validate ordering, domain limits, and checked endpoint arithmetic before slicing
or allocating. For text, byte bounds must additionally respect UTF-8 boundaries;
a valid numeric range alone does not make it a valid string range.

## Sources

- [Microsoft: M-IMPL-RANGEBOUNDS](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-IMPL-RANGEBOUNDS)
- [Standard library: RangeBounds](https://doc.rust-lang.org/std/ops/trait.RangeBounds.html)

## See Also

- [num-overflow-explicit](num-overflow-explicit.md) - handle arithmetic boundaries deliberately
- [type-utf8-boundaries](type-utf8-boundaries.md) - validate text slicing boundaries
- [own-slice-over-vec](own-slice-over-vec.md) - accept the borrowed sequence an algorithm needs
