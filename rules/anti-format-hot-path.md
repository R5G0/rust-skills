# anti-format-hot-path

> Don't use format! in hot paths

## Why It Matters

`format!()` allocates a new `String` every call. In hot paths (loops, frequently called functions), this creates allocation churn that impacts performance. Pre-allocate, reuse buffers, or use `write!()` to an existing buffer.

## Bad

```rust
// format! in loop - allocates every iteration
fn log_events(events: &[Event]) {
    for event in events {
        let message = format!("[{}] {}: {}", event.level, event.source, event.message);
        logger.log(&message);
    }
}

// format! for building parts
fn build_url(base: &str, path: &str, params: &[(&str, &str)]) -> String {
    let mut url = format!("{}{}", base, path);
    for (key, value) in params {
        url = format!("{}{}={}&", url, key, value);  // New allocation each time
    }
    url
}

// format! for simple concatenation
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)  // Fine for one-off, bad if called 1M times
}
```

## Good

<!-- verify: compile -->
```rust
use std::fmt::Write;

fn format_events(events: &[(&str, &str)], output: &mut String) -> std::fmt::Result {
    output.clear();
    for (level, message) in events {
        writeln!(output, "[{level}] {message}")?;
    }
    Ok(())
}

fn main() -> std::fmt::Result {
    let mut buffer = String::with_capacity(256);
    format_events(&[("INFO", "ready"), ("WARN", "retry")], &mut buffer)?;
    assert_eq!(buffer, "[INFO] ready\n[WARN] retry\n");
    format_events(&[("INFO", "done")], &mut buffer)?;
    assert_eq!(buffer, "[INFO] done\n");
    assert!(buffer.capacity() >= 256);
    Ok(())
}
```

## Comparison

| Approach | Allocations | Performance |
|----------|-------------|-------------|
| `format!()` in loop | N | Slow |
| `write!()` to reused buffer | 1 | Fast |
| `push_str()` + `push()` | 1 | Fastest |
| Pre-sized `String::with_capacity()` | 1 (no realloc) | Fast |

## When format! Is Fine

```rust
fn reject(input: &str) -> Result<(), String> {
    Err(format!("invalid input: {input}"))
}

fn main() {
    let home_dir = "/example";
    let config_path = format!("{home_dir}/config.toml");
    assert_eq!(config_path, "/example/config.toml");
    assert_eq!(reject("bad"), Err("invalid input: bad".into()));
}
```

## Pattern: Formatter Buffer Pool

```rust
use std::cell::RefCell;
use std::fmt::Write; // for write! into the String buffer

thread_local! {
    static BUFFER: RefCell<String> = RefCell::new(String::with_capacity(256));
}

fn format_event(event: &Event) -> String {
    BUFFER.with(|buf| {
        let mut buf = buf.borrow_mut();
        buf.clear();
        write!(buf, "[{}] {}", event.level, event.message).unwrap();
        buf.clone()  // Still one allocation per call, but no parsing
    })
}
```

## Pattern: Display Implementation

```rust
use std::fmt::Write; // for the caller's write! into a String

struct Event {
    level: Level,
    message: String,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.level, self.message)
    }
}

// Caller controls allocation
let mut buf = String::new();
write!(buf, "{}", event)?;
```

## Clippy Lint

```toml
[lints.clippy]
format_in_format_args = "warn"
```

## See Also

- [mem-avoid-format](./mem-avoid-format.md) - Avoiding format
- [mem-write-over-format](./mem-write-over-format.md) - Using write!
- [mem-reuse-collections](./mem-reuse-collections.md) - Buffer reuse
