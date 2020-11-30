# sanitizer

Inspired by the [validator](https://github.com/Keats/validator) crate.

## Overview

```rust
[dependencies]
sanitizer = { version = "0.1", features = ["derive"] }
```

Full example with different options:

```rust
use serde::Deserialize;

use sanitizer::{Sanitize, SanitizeError};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate, Sanitize)]
struct SignupData {
    // Idea #1
    #[sanitize(trim, lower_case)]
    #[validate(email)]
    mail: String,
    
    // Idea #2
    #[validate(phone)]
    #[post_validate(phone)]
    phone: String,
    
    #[pre_validate(trim, lower_case)]
    #[validate(url)]
    site: String,
}

fn process_signup(data: SignupData) -> Result<(), dyn std::error::Error> {
    // Idea #1
    data.sanitize()?;
    data.validate()?;
    
    // Idea #2A
    data.pre_validate()?;
    data.validate()?;
    data.post_validate()?;
    
    // Idea #2B
    data.sanitize()?; // Single method that calls pre_validate(), validate(), post_validate()
}
```

## Sanitizers

### trim

Removes whitespace from ends.

### numeric

Removes any character that is not a numeric.

### alphanumeric

Removes any character that is not an alphanumeric.

### lower_case

Converts input to lowercase using the [Inflector](https://github.com/whatisinternet/Inflector) crate.

### upper_case

Converts input to UPPERCASE using the [Inflector](https://github.com/whatisinternet/Inflector) crate.

### camel_case

Converts input to camelCase using the [Inflector](https://github.com/whatisinternet/Inflector) crate.

### snake_case

Converts input to snake_case using the [Inflector](https://github.com/whatisinternet/Inflector) crate.

### screaming_snake_case

Converts input to SCREAMING_SNAKE_CASE using the [Inflector](https://github.com/whatisinternet/Inflector) crate.

### e164

Converts input to E164 International Phone Number format. Ideally used only post-validation.

### cap_min(N)

Caps the input number to `max(input, cap_min)`. (Question: Do we have a use case for this?)

### cap_max(N)

Caps the input number to `min(input, cap_max)`. (Question: Do we have a use case for this?)

### nested

Handles nesting similar to the [validator](https://github.com/Keats/validator) crate.

## Struct level Sanitization

```
#[derive(Debug, Validate, Deserialize)]
#[sanitize(schema(function = "sanitize_category"))]
struct CategoryData {
    category: String,
    name: String,
}
```

Question: Should we instead have pre_validate & post_validate here as well?

## Messages

Question: Do we need custom messages?

## Crate Direction

Do we want to have this separate from the `validator` crate? Or, do we want a more deeply integrated `validator-hooks` crate?
