use sanitizer::prelude::*;

#[derive(Sanitize)]
struct First {
    #[sanitize(trim)]
    name: String,
    #[sanitize]
    info: OtherInfo,
}

#[derive(Sanitize)]
struct OtherInfo {
    #[sanitize(numeric)]
    id: String,
    #[sanitize(lower_case, trim)]
    email: String,
}

impl OtherInfo {
    pub fn new() -> Self {
        Self {
            id: String::from("123984T"),
            email: String::from("Test@gmail.com   "),
        }
    }
}

#[test]
fn first_instance() {
    let other_info = OtherInfo::new();
    let mut first_instance = First {
        name: String::from(" John Doe "),
        info: other_info,
    };
    first_instance.sanitize();
    assert_eq!(first_instance.name, "John Doe");
    assert_eq!(first_instance.info.id, "123984");
    assert_eq!(first_instance.info.email, "test@gmail.com");
}

#[test]
fn second_nesting_instance() {
    let mut other_info = OtherInfo::new();
    other_info.sanitize();
    assert_eq!(other_info.id, "123984");
    assert_eq!(other_info.email, "test@gmail.com");
}
