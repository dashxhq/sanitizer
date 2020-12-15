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

#[derive(Debug, Sanitize)]
struct SignupData {
    #[sanitize(trim, lower_case)]
    mail: String,
    #[sanitize(clamp(1, 60))]
    age: u8,
    #[sanitize]
    user: User,
}

#[derive(Debug, Sanitize)]
struct User {
    id: u64,
    #[sanitize(trim, clamp(50))]
    name: String,
}

fn main() {
    let instance = SignupData::new();
    instance.sanitize();
}
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

### nesting

```rust
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

```

The `sanitize` method of `First` will call the sanitizer method of `OtherInfo` automatically,
if you would like to individually snaitize `OtherInfo` then you can just call `snaitize` on one of its instance.

# LICENSE

MIT
