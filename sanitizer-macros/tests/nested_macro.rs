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

#[allow(dead_code)]
#[derive(Sanitize)]
enum FirstEnum {
    #[sanitize]
    Test(OtherEnum),
    None,
}

#[allow(dead_code)]
#[derive(Sanitize)]
enum OtherEnum {
    #[sanitize(trim)]
    Test(String),
    None,
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

#[test]
fn first_instance_enum() {
    let other_info = OtherEnum::Test(String::from(" hello"));
    let mut first_instance = FirstEnum::Test(other_info);
    first_instance.sanitize();
    match first_instance {
        FirstEnum::Test(x) => {
            if let OtherEnum::Test(y) = x {
                assert_eq!(y, String::from("hello"))
            }
        }
        _ => panic!(),
    }
}

#[test]
fn second_nesting_instance_enum() {
    let mut other_info = OtherEnum::Test(String::from(" hello"));

    other_info.sanitize();
    if let OtherEnum::Test(y) = other_info {
        assert_eq!(y, String::from("hello"))
    }
}
