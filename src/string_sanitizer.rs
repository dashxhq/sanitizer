use inflector::cases::{
    camelcase::to_camel_case, screamingsnakecase::to_screaming_snake_case, snakecase::to_snake_case,
};

use phonenumber::{parse, Mode};
use std::convert::From;

/// The Sanitizer structure is a wrapper over a String type which is to
/// be sanitized.
///
/// # Example
///
/// ```
/// use sanitizer::prelude::*;
///
/// let mut instance = StringSanitizer::from(" HELLO ");
/// instance
/// 	.trim()
/// 	.to_lowercase();
/// assert_eq!(instance.get(), "hello");
/// ```
pub struct StringSanitizer(String);

impl StringSanitizer {
    /// Create a new instance of the struct with the content
    /// as the string specified in the argument
    pub fn new(content: String) -> Self {
        Self(content)
    }
    /// Consume the struct and return the underlying string
    pub fn get(self) -> String {
        self.0
    }
    /// Trim the string
    pub fn trim(&mut self) -> &mut Self {
        self.0 = self.0.trim().to_string();
        self
    }
    /// Remove non numeric characters from the string
    pub fn numeric(&mut self) -> &mut Self {
        self.0 = self.0.chars().filter(|b| b.is_numeric()).collect();
        self
    }
    /// Remove non alphanumeric characters from the string
    pub fn alphanumeric(&mut self) -> &mut Self {
        self.0 = self.0.chars().filter(|b| b.is_alphanumeric()).collect();
        self
    }
    /// Convert string to lower case
    pub fn to_lowercase(&mut self) -> &mut Self {
        self.0 = self.0.to_lowercase();
        self
    }
    /// Convert string to upper case
    pub fn to_uppercase(&mut self) -> &mut Self {
        self.0 = self.0.to_uppercase();
        self
    }
    /// Convert string to camel case
    pub fn to_camelcase(&mut self) -> &mut Self {
        self.0 = to_camel_case(&self.0);
        self
    }
    /// Convert string to snake case
    pub fn to_snakecase(&mut self) -> &mut Self {
        self.0 = to_snake_case(&self.0);
        self
    }
    /// Convert string to screaming snake case
    pub fn to_screaming_snakecase(&mut self) -> &mut Self {
        self.0 = to_screaming_snake_case(&self.0);
        self
    }
    /// Set the maximum lenght of the content
    pub fn clamp_max(&mut self, limit: usize) -> &mut Self {
        self.0.truncate(limit);
        self
    }
    /// Convert the phone number to the E164 International Standard
    pub fn e164(&mut self) -> &mut Self {
        let phone_number = parse(None, &self.0);
        if let Ok(x) = phone_number {
            self.0 = x.format().mode(Mode::E164).to_string();
        } else {
            panic!("{:?}", "Not a valid phone number");
        }
        self
    }
    /// Truncate the string with the given amount
    pub fn cut(&mut self, amount: usize) -> &mut Self {
        self.0.truncate(amount);
        self
    }
    /// Call a custom function for sanitizing the string
    pub fn call<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(&str) -> String,
    {
        self.0 = func(&self.0);
        self
    }
}

impl From<String> for StringSanitizer {
    fn from(content: String) -> Self {
        Self::new(content)
    }
}

impl From<&str> for StringSanitizer {
    fn from(content: &str) -> Self {
        Self::new(content.to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[macro_use]
    macro_rules! string_test {
        ( $sanitizer : ident, $from : expr => $to : expr ) => {
            paste::paste! {
                #[test]
                fn [<$sanitizer>]() {
                    let mut sanitize = StringSanitizer::from($from);
                    sanitize.$sanitizer();
                    assert_eq!($to, sanitize.get());
                }
            }
        };
    }

    string_test!(trim, " Test   " => "Test");
    string_test!(numeric, "Test123445Test" => "123445");
    string_test!(alphanumeric, "Hello,藏World&&" => "Hello藏World");
    string_test!(to_lowercase, "HELLO" => "hello");
    string_test!(to_uppercase, "hello" => "HELLO");
    string_test!(to_camelcase, "some_string" => "someString");
    string_test!(to_snakecase, "someString" => "some_string");
    string_test!(to_screaming_snakecase, "someString" => "SOME_STRING");
    string_test!(e164, "+1 (555) 555-1234" => "+15555551234");

    #[test]
    fn clamp_max() {
        let mut sanitize = StringSanitizer::from("someString");
        sanitize.clamp_max(9);
        assert_eq!("someStrin", sanitize.get());
    }

    #[test]
    #[should_panic]
    fn wrong_phone_number() {
        let mut sanitize = StringSanitizer::from("Not a Phone Number");
        sanitize.e164();
    }

    #[test]
    fn multiple_lints() {
        let mut sanitize = StringSanitizer::from("    some_string12 ");
        sanitize.trim().numeric();
        assert_eq!("12", sanitize.get());
    }
}
