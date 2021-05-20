use sanitizer::prelude::*;
#[derive(Sanitize)]
enum EnumTest {
    #[sanitize(clamp(10, 50))]
    Number(u8),
    #[sanitize(trim)]
    String(String),
}

#[test]
fn sanitizer_check_enum() {
    let mut instance = EnumTest::Number(9);
    let mut string_instance = EnumTest::String(String::from(" hello"));
    instance.sanitize();
    string_instance.sanitize();

    match instance {
        EnumTest::Number(x) => assert_eq!(x, 10),
        _ => panic!(),
    }

    match string_instance {
        EnumTest::String(x) => assert_eq!(x, String::from("hello")),
        _ => panic!(),
    }
}
