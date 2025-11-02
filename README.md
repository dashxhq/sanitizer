# sanitizer

Inspired by the [validator](https://github.com/Keats/validator) crate. The Sanitizer crate is a collection of
methods and a macro to sanitize struct fields, leveraging the macros of rust, it follows the elegant approach by
the validator crate.

# Overview

```rust
[dependencies]
sanitizer = { version = "0.1", features = ["derive"] }
```

Then to use the crate

```rust
use sanitizer::prelude::*;

#[derive(Debug, Sanitizer)]
struct SignupData {
    #[sanitizer(trim, lower_case)]
    mail: String,
    #[sanitizer(clamp(1, 60))]
    age: u8,
    #[sanitizer]
    user: User,
}

#[derive(Debug, Sanitizer)]
struct User {
    id: u64,
    #[sanitizer(trim, clamp(50))]
    name: String,
}

fn main() {
    let instance = SignupData::new();
    instance.sanitize();
}
```

If you do not want to use the derive macro, then the sanitizer crate provides structures and methods for sanitizing
ints

```rust
let int: u8 = 50;
let mut instance = IntSanitizer::from(int);
instance.clamp(99, 101);
assert_eq!(99, instance.get());
```

and strings

```rust
let mut sanitize = StringSanitizer::from("    some_string12 ");
sanitize.trim().numeric();
assert_eq!("12", sanitize.get());
```

# Sanitizers

### trim

Removes whitespace from ends.

### numeric

Removes any character that is not a numeric.

### alphanumeric

Removes any character that is not an alphanumeric.

### lower_case

Converts string input to lowercase.

### upper_case

Converts string input to UPPERCASE.

### camel_case

Converts string input to camelCase.

### snake_case

Converts string input to snake_case.

### screaming_snake_case

Converts string input to SCREAMING_SNAKE_CASE using the [Inflector](https://github.com/whatisinternet/Inflector) crate.

### e164

Converts string input to E164 International Phone Number format. This panics if the phone number is not a valid one.

### clamp(min, max)

Limit an valid integer field with the given min and max.

### clamp(max)

Limit a string input length to the following number

### custom(function)

Use a custom function to sanitize a field differently. For example

```rust

#[derive(Sanitizer)]
struct SanitizerTest {
    #[sanitizer(custom(func_string))]
    field_string: String,
}

fn func_string(field: &str) -> String {
    let mut sanitizer = StringSanitizer::from(field);
    sanitizer.trim();
    sanitizer.get()
}

#[test]
fn sanitizer_check_custom_functions() {
    let mut instance = SanitizerTest {
        field_string: String::from("Hello    "),
    };
    instance.sanitize();
    assert_eq!(instance.field_string, String::from("Hello"));
}
```

### nesting

```rust
#[derive(Sanitizer)]
struct First {
    #[sanitizer(trim)]
    name: String,
    #[sanitizer]
    info: OtherInfo,
}

#[derive(Sanitizer)]
struct OtherInfo {
    #[sanitizer(numeric)]
    id: String,
    #[sanitizer(lower_case, trim)]
    email: String,
}

```

The `sanitize` method of `First` will call the sanitizer method of `OtherInfo` automatically,
if you would like to individually snaitize `OtherInfo` then you can just call `snaitize` on one of its instance.

# LICENSE

dashxhq/sanitizer is licensed under the MIT License.
