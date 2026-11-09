//! Application configuration kept in a single module.

/// Input accepted by [`crate::run`].
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    /// Name to greet.
    pub name: String,
}

impl Config {
    /// Construct a [`Config`] with an owned name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
