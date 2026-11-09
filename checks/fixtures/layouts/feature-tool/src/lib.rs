//! Feature-based modules with private implementation files.

pub mod user;

/// Greet a user found by the [`user`] feature's public API.
pub fn greet_user(id: u32) -> Option<String> {
    user::find_user(id).map(|user| layout_shared::greeting(&user.name))
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_and_missing_users_follow_the_feature_contract() {
        assert_eq!(super::greet_user(1).as_deref(), Some("Hello, Ada!"));
        assert_eq!(super::greet_user(99), None);
    }
}
