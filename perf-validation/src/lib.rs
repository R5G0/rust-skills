//! Executable experiments for the thirteen `perf-*` rules in `../rules`.
//!
//! Reference implementations remain available so behavior and measured costs
//! can be compared. A candidate's name does not imply that it will run faster.

pub mod buffered_io;
pub mod collections;
pub mod hashing;
pub mod iterators;
pub mod measurement;
