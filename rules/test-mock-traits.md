# test-mock-traits

> Make external effects controllable with appropriate inputs, I/O traits, or narrow dependency interfaces

## Why It Matters

Clocks, entropy, filesystem state, network behavior, and environment configuration can make failure cases difficult to reproduce. Put a controllable boundary around the effects that matter to the behavior under test. A supplied value, closure, standard I/O trait, concrete fake, or narrow custom trait can each provide that boundary without requiring every dependency to become generic.

## Bad

```rust
// Configuration parsing cannot be exercised without manipulating process state.
fn retry_limit() -> u32 {
    std::env::var("RETRY_LIMIT").unwrap().parse().unwrap()
}
```

## Good

```rust
// The application reads its environment once, then supplies the input.
fn retry_limit(text: &str) -> Result<u32, std::num::ParseIntError> {
    text.parse()
}

fn main() {
    assert_eq!(retry_limit("3"), Ok(3));
    assert!(retry_limit("many").is_err());
}
```

A reusable reader can accept the existing I/O trait instead of requiring a
particular file or a new application-specific interface:

```rust
use std::io::{self, Read};

fn read_version(mut reader: impl Read) -> io::Result<u8> {
    let mut byte = [0];
    reader.read_exact(&mut byte)?;
    Ok(byte[0])
}

struct Unavailable;

impl Read for Unavailable {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "access denied"))
    }
}

fn main() -> io::Result<()> {
    assert_eq!(read_version(&[7_u8][..])?, 7);
    assert_eq!(
        read_version(&[][..]).expect_err("missing header").kind(),
        io::ErrorKind::UnexpectedEof
    );
    assert_eq!(
        read_version(Unavailable).expect_err("unavailable reader").kind(),
        io::ErrorKind::PermissionDenied
    );
    Ok(())
}
```

## Select the Smallest Useful Boundary

- Pass sampled time, configuration, or seeds as values when the algorithm only
  needs one observation. Use a closure or clock abstraction when repeated,
  controllable observations are part of the behavior.
- Use standard `Read`/`Write` traits for synchronous stream processing when
  they express the required contract. Use runtime-specific I/O traits when the
  library intentionally targets that runtime.
- Introduce a custom trait when callers need substitution at a meaningful
  boundary. A single implementation does not invalidate a genuine boundary,
  and testability alone does not require a trait for every concrete type.
- Keep fakes faithful to the observable contract: partial I/O, ordering, failure
  modes, and concurrency may matter. Retain real integration checks for adapter
  behavior that the fake cannot establish.
- Keep diagnostic access and safety-check bypasses out of production APIs.
  A clearly named optional testing feature can expose shared test utilities;
  feature unification means it is not a security boundary by itself.
- Avoid tests that require unsafe process-wide environment mutation. Supplying
  parsed configuration usually makes those tests simpler and more isolated.

## Sources

- [Microsoft: M-MOCKABLE-SYSCALLS and M-TEST-UTIL](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-MOCKABLE-SYSCALLS)
- [Microsoft: M-IMPL-IO](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-IMPL-IO)

## See Also

- [anti-over-abstraction](anti-over-abstraction.md) - introduce a real boundary without generic scaffolding
- [test-mockall-mocking](test-mockall-mocking.md) - generate mocks when a trait interface is appropriate
- [test-fixture-raii](test-fixture-raii.md) - manage external resources when they are needed
