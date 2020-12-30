use sanitizer::prelude::*;

#[derive(Sanitize)]
struct SanitizerTest {
    #[sanitize(custom(func_int))]
    field_int: u8,
    #[sanitize(custom(func_string))]
    field_string: String,
}

fn func_int(field: u8) -> u8 {
    let mut sanitizer = IntSanitizer::new(field);
    sanitizer.clamp(0, 5);
    sanitizer.get()
}

fn func_string(field: &str) -> String {
    let mut sanitizer = StringSanitizer::from(field);
    sanitizer.trim();
    sanitizer.get()
}

#[test]
fn sanitizer_check_custom_functions() {
    let mut instance = SanitizerTest {
        field_int: 10,
        field_string: String::from("Hello    "),
    };
    instance.sanitize();
    assert_eq!(instance.field_int, 5);
    assert_eq!(instance.field_string, String::from("Hello"));
}
