# sanitizer-macros

Derive macros for sanitizer crate

# Usage

```rust
use sanitizer::prelude::*;

#[derive(Sanitizer)]
struct User {
	#[sanitizer(trim)]
	name: String,
	#[sanitizer(trim, lower_case)]
	email: String
}
```
