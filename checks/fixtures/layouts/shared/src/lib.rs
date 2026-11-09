//! Small shared dependency inherited by both workspace members.

/// Format a greeting without external I/O.
pub fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}
