#![allow(clippy::all)]
#![forbid(unsafe_code)]
//! The Sanitizer crate helps in sanitizing structured data
//! by providing macros.
//!
//! # Basic usage
//!
//! If you want your incoming data which is serialised to a
//! structure to be sanitized then first of all, you make a struct
//! and derive the Sanitize trait on it.
//! The macro will implement the trait for you all you have to do
//! now is to call the sanitize method on the trait
//!
//! ```
//! use sanitizer_macros::Sanitize;
//!
//! #[derive(Sanitize)]
//! struct User {
//! 	#[sanitize(trim, numeric)]
//! 	name: String,
//! 	#[sanitize(trim, lower_case)]
//! 	email: String
//! }
//!
//! fn main() {
//! 	let mut instance = User {
//! 		name: String::from("John Doe123"),
//! 		email: String::from(" JohnDoe123@email.com")
//! 	};
//! 	instance.sanitize();
//! 	assert_eq!(instance.name, "John Doe");
//! 	assert_eq!(instance.email, "johndoe123@email.com");
//! }
//! ```
//! To see a list of available sanitizers, check the [sanitizer-macros crate](../sanitizer_macros/derive.Sanitize.html)
mod sanitize;

pub use crate::sanitize::{Sanitize, Sanitizer};
