#![allow(clippy::all)]
#![forbid(unsafe_code)]
//! The Sanitizer crate helps in sanitizing structured data
//! by providing [macros](https://docs.rs/sanitizer_macros/1.0.0/sanitizer_macros/derive.Sanitizer.html) and data structures to perform sanitization on
//! fields.
//!
//! Sanitizer uses [heck](https://docs.rs/heck/) to allow case conversions
//!
//! # Example
//!
//! If you want your incoming data which is serialised to a
//! structure to be sanitized then first of all, you make a struct
//! and derive the Sanitizer trait on it.
//! The macro will implement the trait for you all you have to do
//! now is to call the sanitize method on the trait
//!
//! ```
//! use sanitizer::prelude::*;
//!
//! #[derive(Sanitizer)]
//! struct User {
//! 	#[sanitizer(trim)]
//! 	name: String,
//! 	#[sanitizer(trim, lower_case)]
//! 	email: String
//! }
//!
//! fn main() {
//! 	let mut instance = User {
//! 		name: String::from("   John Doe123 "),
//! 		email: String::from(" JohnDoe123@email.com")
//! 	};
//! 	instance.sanitize();
//! 	assert_eq!(instance.name, "John Doe123");
//! 	assert_eq!(instance.email, "johndoe123@email.com");
//! }
//! ```
//! To see a list of available sanitizers, check the [sanitizer-macros crate](https://docs.rs/sanitizer_macros/0.1.0/sanitizer_macros/derive.Sanitize.html)
mod int_sanitizer;
mod string_sanitizer;
/// Bring all the sanitizers, the derive macro, and the Sanitizer trait in scope
pub mod prelude {
    pub use crate::Sanitizer;
    pub use crate::int_sanitizer::IntSanitizer;
    pub use crate::string_sanitizer::StringSanitizer;
    #[cfg(feature = "derive")]
    pub use sanitizer_macros::Sanitizer;
}
/// Sanitizer methods for ints
pub use crate::int_sanitizer::IntSanitizer;
/// Sanitizer methods for strings
pub use crate::string_sanitizer::StringSanitizer;
/// The Sanitizer trait generalises types that are to be sanitized.
pub trait Sanitizer {
    /// Call this associated method when sanitizing.
    fn sanitize(&mut self);
}

/// Generic `impl` for sanitizing values wrapped in an [Option]:
///
/// ```rust
/// use sanitizer::Sanitizer;
///
/// #[derive(Debug, PartialEq)]
/// struct MyValue(i32);
///
/// impl Sanitizer for MyValue {
///     /// If the inner value is `0`, change it to `1`.
///     fn sanitize(&mut self) {
///         if self.0 == 0 {
///             self.0 = 1;
///         }
///     }
/// }
///
/// let mut wrapped_value = Some(MyValue(0));
/// wrapped_value.sanitize();
/// assert_eq!(wrapped_value, Some(MyValue(1)));
/// ```
impl<T: Sanitizer> Sanitizer for Option<T> {
    fn sanitize(&mut self) {
        if let Some(inner) = self.as_mut() {
            inner.sanitize();
        }
    }
}

/// Generic `impl` for sanitizing values in a [Vec]:
///
/// ```rust
/// use sanitizer::Sanitizer;
///
/// #[derive(Debug, PartialEq)]
/// struct MyValue(i32);
///
/// impl Sanitizer for MyValue {
///     /// If the inner value is `0`, change it to `1`.
///     fn sanitize(&mut self) {
///         if self.0 == 0 {
///             self.0 = 1;
///         }
///     }
/// }
///
/// let mut values = vec![MyValue(0), MyValue(2)];
/// values.sanitize();
/// assert_eq!(values, vec![MyValue(1), MyValue(2)]);
/// ```
impl<T: Sanitizer> Sanitizer for Vec<T> {
    fn sanitize(&mut self) {
        for item in self.iter_mut() {
            item.sanitize()
        }
    }
}
