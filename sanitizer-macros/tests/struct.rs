use sanitizer::prelude::*;

#[derive(Sanitize)]
struct SanitizerTest {
    #[sanitize(trim)]
    trim: String,
    #[sanitize(numeric)]
    numeric: String,
    #[sanitize(alphanumeric)]
    alphanumeric: String,
    #[sanitize(lower_case)]
    lower_case: String,
    #[sanitize(upper_case)]
    upper_case: String,
    #[sanitize(camel_case)]
    camel_case: String,
    #[sanitize(snake_case)]
    snake_case: String,
    #[sanitize(screaming_snake_case)]
    screaming_snake_case: String,
    #[sanitize(kebab_case)]
    kebab_case: String,
    #[sanitize(screaming_kebab_case)]
    screaming_kebab_case: String,
    #[sanitize(clamp(10))]
    clamp_str: String,
    #[sanitize(clamp(10, 50))]
    clamp_int: u8,
    #[sanitize(e164)]
    phone_number: String,
    #[sanitize(trim, screaming_snake_case)]
    multiple_sanitizers: String,
}

#[test]
fn sanitizer_check() {
    let mut instance = SanitizerTest {
        trim: String::from("    test     "),
        numeric: String::from("HelloWorld8130"),
        alphanumeric: String::from("Hello,藏World&&"),
        lower_case: String::from("HELLO, WORLD"),
        upper_case: String::from("hello, world"),
        camel_case: String::from("hello_world"),
        snake_case: String::from("helloWorld"),
        screaming_kebab_case: String::from("helloWorld"),
        kebab_case: String::from("hello-World"),
        screaming_snake_case: String::from("HELLO-WORLD"),
        clamp_str: String::from("Hello, World"),
        clamp_int: 9,
        phone_number: String::from("+1 (454)"),
        multiple_sanitizers: String::from("    helloWorld123  "),
    };
    instance.sanitize();
    assert_eq!(instance.trim, "test");
    assert_eq!(instance.numeric, "8130");
    assert_eq!(instance.alphanumeric, "Hello藏World");
    assert_eq!(instance.lower_case, "hello, world");
    assert_eq!(instance.upper_case, "HELLO, WORLD");
    assert_eq!(instance.camel_case, "helloWorld");
    assert_eq!(instance.snake_case, "hello_world");
    assert_eq!(instance.screaming_snake_case, "HELLO_WORLD");
    assert_eq!(instance.clamp_str, "Hello, Wor");

    assert_eq!(instance.kebab_case, "hello-world");
    assert_eq!(instance.clamp_str, "Hello, Wor");
    assert_eq!(instance.clamp_int, 10);
    assert_eq!(instance.phone_number, "+1454");
    assert_eq!(instance.multiple_sanitizers, "HELLO_WORLD123");
}
