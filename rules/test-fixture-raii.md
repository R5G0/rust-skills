# test-fixture-raii

> Use RAII pattern (Drop trait) for automatic test cleanup

## Why It Matters

Tests often need setup and teardown, such as temporary files and local server resources. RAII (Resource Acquisition Is Initialization) ties cleanup to an owning guard. Drop runs on normal scope exit and during panic unwinding; process aborts do not run destructors. Prefer passing configuration directly or setting a child process environment instead of mutating process-global environment variables in concurrent tests.

## Bad

```rust
#[test]
fn test_with_temp_file() {
    let path = "/tmp/test_file.txt";
    std::fs::write(path, "test data").unwrap();
    
    let result = process_file(path);
    
    std::fs::remove_file(path).unwrap();  // Might not run if test panics!
    assert!(result.is_ok());
}

#[test]
fn test_with_env_var() {
    std::env::set_var("MY_VAR", "test_value");
    
    let result = read_config();
    
    std::env::remove_var("MY_VAR");  // Might not run if test panics!
    assert!(result.is_ok());
}
```

## Good

<!-- verify: compile -->
```rust
use std::path::Path;

fn process_file(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path).map(|text| text.trim().to_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn temporary_file_is_removed_when_its_owner_leaves_scope() {
        let path;
        {
            // Arrange: the unique file owns its cleanup operation.
            let file = NamedTempFile::new().expect("create temporary file");
            path = file.path().to_owned();
            std::fs::write(&path, "  test data\n").expect("write fixture");

            // Act
            let result = process_file(&path).expect("process fixture");

            // Assert
            assert_eq!(result, "TEST DATA");
            assert!(path.exists());
        }
        assert!(!path.exists());
    }

    #[test]
    fn temporary_directory_is_removed_during_panic_unwinding() {
        let path;
        let outcome;
        {
            let directory = TempDir::new().expect("create temporary directory");
            path = directory.path().to_owned();
            std::fs::write(path.join("data.txt"), "fixture").expect("write fixture");
            outcome = std::panic::catch_unwind(move || {
                // Moving the guard into the unwinding scope ensures Drop runs.
                let _directory = directory;
                panic!("injected failure after setup");
            });
        }
        assert!(outcome.is_err());
        assert!(!path.exists());
    }
}
```

## Common RAII Patterns

```rust
// Temporary directory
use tempfile::TempDir;

#[test]
fn test_with_temp_dir() {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.txt");
    std::fs::write(&file_path, "data").unwrap();
    
    // dir and all contents deleted on drop
}

// Server guard
struct TestServer {
    handle: std::thread::JoinHandle<()>,
    shutdown: std::sync::mpsc::Sender<()>,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.shutdown.send(());
        // Wait for server to stop
    }
}

// Database transaction rollback
struct TestTransaction<'a> {
    conn: &'a mut Connection,
}

impl Drop for TestTransaction<'_> {
    fn drop(&mut self) {
        self.conn.execute("ROLLBACK").unwrap();
    }
}
```

## scopeguard Crate

```rust
use scopeguard::defer;

#[test]
fn test_with_defer() {
    let path = "/tmp/test_file.txt";
    std::fs::write(path, "data").unwrap();
    
    defer! {
        std::fs::remove_file(path).ok();
    }
    
    // Test logic here
    // File removed when scope exits
}
```

## See Also

- [test-arrange-act-assert](./test-arrange-act-assert.md) - Test structure
- [test-tokio-async](./test-tokio-async.md) - Async test cleanup
- [test-mock-traits](./test-mock-traits.md) - Mocking with RAII
