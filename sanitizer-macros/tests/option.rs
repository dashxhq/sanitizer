use sanitizer::prelude::*;

#[derive(Sanitize, Default)]
struct First {
    // basic option should work
    #[sanitize(trim)]
    name: Option<String>,
    #[sanitize(trim)]
    // even with random paths
    address: std::option::Option<std::string::String>,
    #[sanitize(clamp(1, 18))]
    // even with ints
    age: std::option::Option<u8>,
    // double option nested
    #[sanitize(trim)]
    name_last: Option<Option<String>>,
    #[sanitize(trim)]
    // None values should NOT be sanitized
    city: Option<String>,
    // Same with nested options
    country: Option<Option<String>>,
}

#[test]
fn option_test() {
    let mut instance = First {
        name: Some(String::from("Test  ")),
        address: Some(String::from("Mars, i'm elon musk   ")),
        age: Some(0),
        name_last: Some(Some(String::from(" William   "))),
        city: None,
        country: None,
    };
    instance.sanitize();
    assert_eq!(instance.name, Some(String::from("Test")));
    assert_eq!(instance.address, Some(String::from("Mars, i'm elon musk")));
    assert_eq!(instance.age, Some(1));
    assert_eq!(instance.name_last, Some(Some(String::from("William"))));
    assert_eq!(instance.city, None);
    assert_eq!(instance.country, None);
}
