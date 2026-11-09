//! Tests compiled as a separate crate against only the public library API.

use layout_flat_tool::{Config, divide, parse_port, run};

#[test]
fn run_greets_the_configured_name() {
    assert_eq!(run(Config::new("Ada")), "Hello, Ada!");
}

#[test]
fn parse_port_rejects_empty_and_out_of_range_input() {
    assert!(parse_port("").is_err());
    assert!(parse_port("65536").is_err());
    assert_eq!(parse_port("65535"), Ok(u16::MAX));
}

#[test]
#[should_panic]
fn divide_panics_for_zero_divisor() {
    divide(1, 0);
}

#[test]
#[should_panic]
fn divide_panics_for_signed_overflow() {
    divide(i32::MIN, -1);
}
