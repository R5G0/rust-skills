//! A small flat project with testable library logic and a thin binary.
//!
//! [`Config`] supplies input to [`run`]. The [`parse_port`] example demonstrates
//! hidden setup and error propagation in an executable doctest.

mod config;

pub use config::Config;

/// Run the application logic without reading process-global state.
///
/// # Examples
///
/// ```
/// use layout_flat_tool::{Config, run};
/// assert_eq!(run(Config::new("Ada")), "Hello, Ada!");
/// ```
pub fn run(config: Config) -> String {
    layout_shared::greeting(&config.name)
}

/// Parse a port number using [`str::parse`].
///
/// # Errors
///
/// Returns [`std::num::ParseIntError`] for non-numeric input or a value outside
/// the range of [`u16`].
///
/// # Examples
///
/// ```
/// # use layout_flat_tool::parse_port;
/// let port = parse_port("8080")?;
/// assert_eq!(port, 8080);
/// # Ok::<(), std::num::ParseIntError>(())
/// ```
///
/// Invalid input returns an error:
///
/// ```
/// use layout_flat_tool::parse_port;
/// assert!(parse_port("invalid").is_err());
/// ```
pub fn parse_port(input: &str) -> Result<u16, std::num::ParseIntError> {
    input.parse()
}

/// Divide integers, with a non-panicking alternative in [`i32::checked_div`].
///
/// # Panics
///
/// Panics if `divisor` is zero or if dividing [`i32::MIN`] by `-1` overflows.
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    dividend / divisor
}
