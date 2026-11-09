//! User lookup behavior grouped with its model and repository implementation.

mod model;
mod repository;

pub use model::User;

/// Find a [`User`] through the private repository layer.
pub fn find_user(id: u32) -> Option<User> {
    repository::lookup(id)
}
