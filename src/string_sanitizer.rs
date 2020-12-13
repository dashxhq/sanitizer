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
    pub fn cut(&mut self, amount: usize) -> &mut Self {
        self.0.truncate(amount);
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
        Self::new(content.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trim() {
        let mut sanitize = StringSanitizer::from(" Test   ");
        sanitize.trim();
        assert_eq!("Test", sanitize.get());
    }

    #[test]
    fn numeric() {
        let mut sanitize = StringSanitizer::from("Test123445Test");
        sanitize.numeric();
        assert_eq!("123445", sanitize.get());
    }

    #[test]
    fn alphanumeric() {
        let mut sanitize = StringSanitizer::from("Hello,藏World&&");
        sanitize.alphanumeric();
        assert_eq!("Hello藏World", sanitize.get());
    }

    #[test]
    fn lowercase() {
        let mut sanitize = StringSanitizer::from("HELLO");
        sanitize.to_lowercase();
        assert_eq!("hello", sanitize.get());
    }

    #[test]
    fn uppercase() {
        let mut sanitize = StringSanitizer::from("hello");
        sanitize.to_uppercase();
        assert_eq!("HELLO", sanitize.get());
    }

    #[test]
    fn camelcase() {
        let mut sanitize = StringSanitizer::from("some_string");
        sanitize.to_camelcase();
        assert_eq!("someString", sanitize.get());
    }

    #[test]
    fn snakecase() {
        let mut sanitize = StringSanitizer::from("someString");
        sanitize.to_snakecase();
        assert_eq!("some_string", sanitize.get());
    }

    #[test]
    fn screaming_snakecase() {
        let mut sanitize = StringSanitizer::from("someString");
        sanitize.to_screaming_snakecase();
        assert_eq!("SOME_STRING", sanitize.get());
    }

    #[test]
    fn clamp_max() {
        let mut sanitize = StringSanitizer::from("someString");
        sanitize.clamp_max(9);
        assert_eq!("someStrin", sanitize.get());
    }

    #[test]
    fn phone_number() {
        let mut sanitize = StringSanitizer::from("+1 (555) 555-1234");
        sanitize.e164();
        assert_eq!("+15555551234", sanitize.get());
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