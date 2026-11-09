# num-bit-width

> Use named unsigned integer bit helpers when they clarify bit-width and set-bit operations

## Why It Matters

Rust 1.97 added `bit_width` to unsigned integers and named helpers for finding or isolating set bits. These express the intended operation and document its zero case. Manual formulas can be correct, but hardcoded widths and unchecked logarithms make boundary cases easier to miss.

## Bad

```rust
fn needed_bits(n: u32) -> u32 {
    n.ilog2() + 1 // Panics at zero.
}

fn highest_set_bit(n: u32) -> u32 {
    31 - n.leading_zeros() // Underflows at zero.
}
```

## Good

```rust
fn needed_bits(n: u32) -> u32 {
    n.bit_width()
}

fn highest_set_bit(n: u32) -> Option<u32> {
    n.highest_one()
}

fn lowest_set_bit(n: u32) -> Option<u32> {
    n.lowest_one()
}

fn lowest_power_of_two(n: u32) -> u32 {
    n.isolate_lowest_one()
}

fn highest_power_of_two(n: u32) -> u32 {
    n.isolate_highest_one()
}
```

For `0_u32`, these return `0`, `None`, `None`, `0`, and `0`, respectively. For `0b0110_0100_u32`, the bit width is `7`, the highest and lowest indexes are `6` and `2`, and the isolated values are `0b0100_0000` and `0b0000_0100`.

## Key Points

- `bit_width` is available on unsigned integers. For signed data, choose the intended magnitude or bit-pattern interpretation explicitly before converting.
- `highest_one` / `lowest_one` return `Option<u32>` indexes for primitive integers, and `u32` for `NonZero`.
- `isolate_highest_one` / `isolate_lowest_one` return the input type with only one bit retained; zero remains zero.
- Correct formulas such as `u32::BITS - n.leading_zeros()` already handle zero. Prefer the named method for clarity when the project's minimum Rust version permits it; there is no need to change an older MSRV for this shorthand alone.

## See Also

- [num-overflow-explicit](num-overflow-explicit.md) - name overflow intent with checked, saturating, or wrapping arithmetic
- [num-cast-try-from](num-cast-try-from.md) - avoid unchecked narrowing conversions
- [mem-smaller-integers](mem-smaller-integers.md) - choose an integer type that fits the domain

## Sources

- [Unsigned integer bit helpers](https://doc.rust-lang.org/std/primitive.u32.html#method.bit_width)
- [Rust 1.97 release notes](https://doc.rust-lang.org/releases.html#version-1970-2026-07-09)
