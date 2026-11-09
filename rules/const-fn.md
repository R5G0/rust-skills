# const-fn

> Use `const fn` for useful compile-time operations and preserve public const-callability

## Why It Matters

`const fn` allows a function to be called in const contexts as well as at runtime. Once callers depend on a public function in array lengths or constant initializers, removing `const` breaks their code. Choose it when compile-time use is useful and the implementation can retain that promise on the supported Rust version.

## Bad

```rust
fn header_len() -> usize {
    4
}

// Cannot call a non-const function in an array length.
const HEADER: [u8; header_len()] = [0; header_len()];
```

```rust
// Previous public API:
// pub const fn header_len() -> usize { 4 }

// Removing const is a breaking change even though the runtime signature matches.
pub fn header_len() -> usize {
    4
}
```

## Good

```rust
pub const fn header_len() -> usize {
    4
}

const HEADER: [u8; header_len()] = [0; header_len()];

const fn checked_align_up(value: usize, alignment: usize) -> Option<usize> {
    if !alignment.is_power_of_two() {
        return None;
    }
    match value.checked_add(alignment - 1) {
        Some(rounded) => Some(rounded & !(alignment - 1)),
        None => None,
    }
}

const ALIGNED: Option<usize> = checked_align_up(13, 8);

fn main() {
    assert_eq!(HEADER.len(), 4);
    assert_eq!(ALIGNED, Some(16));
    assert_eq!(checked_align_up(1, 0), None);
    assert_eq!(checked_align_up(usize::MAX, 8), None);
}
```

## API and Toolchain Considerations

Adding `const` is generally backwards-compatible, but removing it from a public
API is not. Ordinary calls to `const fn` are not guaranteed to execute at compile
time; use a const context when evaluation during compilation is required. Check
that every operation in the function is const-stable on the project's MSRV.

## Sources

- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) — responsible public const APIs
- [Cargo SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html) — removing const

## See Also

- [const-block](const-block.md) - require compile-time evaluation inline
- [const-generics](const-generics.md) - parameterize types with compile-time values
- [proj-msrv-declare](proj-msrv-declare.md) - respect the supported compiler version
