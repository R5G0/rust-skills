# type-utf8-boundaries

> Distinguish byte offsets, Unicode scalar values, and grapheme clusters when slicing text

## Why It Matters

`str::len()` counts bytes, and a string range must start and end on UTF-8 boundaries. Treating a character count as a byte offset can panic on ordinary non-ASCII input. Choose byte, scalar-value, or grapheme semantics explicitly rather than letting ASCII-only examples define the contract.

## Bad

```rust
fn first_character(text: &str) -> &str {
    &text[..1]
}

fn main() {
    // The first Unicode scalar occupies two UTF-8 bytes.
    println!("{}", first_character("éclair")); // Panics.
}
```

## Good

```rust
// Return a prefix containing at most count Unicode scalar values.
// This borrows the original string and performs no allocation.
fn scalar_prefix(text: &str, count: usize) -> &str {
    let end = text
        .char_indices()
        .nth(count)
        .map_or(text.len(), |(offset, _)| offset);
    &text[..end]
}

fn main() {
    assert_eq!(scalar_prefix("éclair", 1), "é");
    assert_eq!(scalar_prefix("東京", 1), "東");
    assert_eq!(scalar_prefix("éclair", 0), "");
    assert_eq!(scalar_prefix("", 10), "");
    assert_eq!(scalar_prefix("abc", 10), "abc");

    // For an external byte offset, reject an invalid range without panicking.
    assert_eq!("éclair".get(..1), None);
    assert_eq!("éclair".get(..2), Some("é"));
}
```

## Pick the Text Contract

- Use bytes for binary protocols and ASCII grammars when that assumption is part
  of the format. Validate the assumption before treating arbitrary text as ASCII.
- Use `chars()` or `char_indices()` for Unicode scalar values. A Rust `char`
  is not necessarily a complete character as perceived by a reader.
- For user-visible cursor movement or truncation, consider Unicode grapheme
  segmentation; a combining mark or joined emoji can span several scalar values.
- Use `get(range)` for fallible byte slicing, or derive offsets from APIs that
  provide valid boundaries. Repeating `chars().nth(index)` in a loop rescans text;
  keep an iterator when processing it sequentially.

## Sources

- [Comprehensive Rust: Strings](https://google.github.io/comprehensive-rust/references/strings.html)
- [Comprehensive Rust: String](https://google.github.io/comprehensive-rust/std-types/string.html)
- [Standard library: str](https://doc.rust-lang.org/std/primitive.str.html)

## See Also

- [own-slice-over-vec](own-slice-over-vec.md) - accept borrowed string slices
- [api-extension-trait](api-extension-trait.md) - keep text extensions safe for their input domain
- [num-cast-try-from](num-cast-try-from.md) - validate representation changes at boundaries
