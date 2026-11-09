/// User data exposed by [`super::find_user`].
#[derive(Debug, PartialEq, Eq)]
pub struct User {
    /// User's stable numeric identifier.
    pub id: u32,
    /// Display name.
    pub name: String,
}
