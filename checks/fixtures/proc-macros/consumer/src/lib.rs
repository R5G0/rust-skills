//! A separate consumer crate exercises generated code across the crate boundary.

use checked_macros::{Greet, Hello, MyTrait};

#[derive(checked_macros::Greet, checked_macros::Hello, checked_macros::MyTrait)]
pub struct Borrowed<'a, T: Clone>
where
    T: 'a,
{
    pub value: &'a T,
}

/// Produce values through all three derives and an explicitly imported macro.
pub fn verify() {
    let value = 42;
    let borrowed = Borrowed { value: &value };
    assert_eq!(borrowed.greet(), "hello, Borrowed");
    assert_eq!(borrowed.hello(), "Borrowed");
    assert_eq!(Borrowed::<i32>::first_field_name(), "value");

    // The defining crate's helper is resolved through $crate, independent of
    // the caller's spelling and its identically named local helper module.
    use checked_macros as renamed;
    use renamed::greet;
    mod __private {
        pub fn format_greeting(_: &str) -> String {
            String::from("wrong helper")
        }
    }
    assert_eq!(greet!("world"), "hello, world");
    assert_eq!(__private::format_greeting("world"), "wrong helper");
}

#[cfg(test)]
mod tests {
    #[test]
    fn facade_derives_and_exported_macro_work_in_another_crate() {
        super::verify();
    }
}
