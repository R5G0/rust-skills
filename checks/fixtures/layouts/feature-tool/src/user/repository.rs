use super::User;

// pub(super) here exposes lookup to user and its descendants, not crate root.
pub(super) fn lookup(id: u32) -> Option<User> {
    (id == 1).then(|| User {
        id,
        name: "Ada".to_owned(),
    })
}
