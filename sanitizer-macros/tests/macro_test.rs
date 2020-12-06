use sanitizer_macros::Sanitize;

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
    #[sanitize(trim, screaming_snake_case, numeric)]
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
        screaming_snake_case: String::from("helloWorld"),
        multiple_sanitizers: String::from("    helloWorld123  "),
    };
    instance.sanitize();
    assert_eq!(instance.trim, "test");
    assert_eq!(instance.numeric, "HelloWorld");
    assert_eq!(instance.alphanumeric, ",&&");
    assert_eq!(instance.lower_case, "hello, world");
    assert_eq!(instance.upper_case, "HELLO, WORLD");
    assert_eq!(instance.camel_case, "helloWorld");
    assert_eq!(instance.snake_case, "hello_world");
    assert_eq!(instance.screaming_snake_case, "HELLO_WORLD");
    // TODO: Check if this is actually true, the _ at the end seems fishy
    assert_eq!(instance.multiple_sanitizers, "HELLO_WORLD_");
}
