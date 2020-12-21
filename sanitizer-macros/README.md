# sanitizer-macros

Derive macros for sanitizer crate

# Usage

```rust
use sanitizer::prelude::*;

#[derive(Sanitize)]
struct User {
	#[sanitize(trim)]
	name: String,
	#[sanitize(trim, lower_case)]
	email: String
}
```
