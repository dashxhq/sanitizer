#![allow(clippy::all)]
#![forbid(unsafe_code)]
//! The Sanitizer crate helps in sanitizing structured data
//! by providing [macros](https://docs.rs/sanitizer_macros/0.1.0/sanitizer_macros/derive.Sanitize.html) and data structures to perform sanitization on
//! fields.
//!
//! Sanitizer uses [heck](https://docs.rs/heck/) to allow case conversions
//!
//! # Example
//!
//! If you want your incoming data which is serialised to a
//! structure to be sanitized then first of all, you make a struct
//! and derive the Sanitize trait on it.
//! The macro will implement the trait for you all you have to do
//! now is to call the sanitize method on the trait
//!
//! ```
//! use sanitizer::prelude::*;
//!
//! #[derive(Sanitize)]
//! struct User {
//! 	#[sanitize(trim)]
//! 	name: String,
//! 	#[sanitize(trim, lower_case)]
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
/// Bring all the sanitizers, the derive macro, and the Sanitize trait in scope
pub mod prelude {
    pub use crate::int_sanitizer::IntSanitizer;
    pub use crate::string_sanitizer::StringSanitizer;
    pub use crate::Sanitize;
    #[cfg(feature = "derive")]
    pub use sanitizer_macros::Sanitize;
}
/// Error to throw when phone number parsing fails
pub type NumberParseError = ParseError;
/// The Sanitize trait generalises types that are to be sanitized.
pub trait Sanitize {
    /// Call this associated method when sanitizing.
    fn sanitize(&mut self) -> Result<(), NumberParseError>;
}
/// Sanitizer methods for ints
pub use crate::int_sanitizer::IntSanitizer;
/// Sanitizer methods for strings
pub use crate::string_sanitizer::StringSanitizer;
use phonenumber::ParseError;
