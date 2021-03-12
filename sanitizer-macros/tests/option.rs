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
}

#[test]
fn option_test() {
    let mut instance = First {
        name: Some(String::from("Test  ")),
        address: Some(String::from("Mars, i'm elon musk   ")),
        age: Some(0),
    };
    instance.sanitize();
    assert_eq!(instance.name, Some(String::from("Test")));
    assert_eq!(instance.address, Some(String::from("Mars, i'm elon musk")));
    assert_eq!(instance.age, Some(1));
}
