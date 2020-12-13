/// The IntSanitizer structure is a wrapper over a type T which is to
/// be sanitized, T can be anything that's `PartialOrd`
///
/// # Example
///
/// ```
/// use sanitizer::prelude::*;
///
/// let mut instance = IntSanitizer::new(5);
/// instance
/// 	.clamp(9, 15);
/// assert_eq!(instance.get(), 9);
/// ```
///
pub struct IntSanitizer<T: PartialOrd + Copy>(T);
// TODO: Remove Copy since its restrictive
impl<T: PartialOrd + Copy> IntSanitizer<T> {
    pub fn new(int: T) -> Self {
        Self(int)
    }
    pub fn get(self) -> T {
        self.0
    }
    /// Sets the int equal to the max value if it exceds the provided
    /// max value provided in the function argument
    pub fn clamp(&mut self, min: T, max: T) -> &mut Self {
        self.0 = num_traits::clamp(self.0, min, max);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_cap_min() {
        let int: u8 = 50;
        let mut instance = IntSanitizer::new(int);
        instance.clamp(99, 101);
        assert_eq!(99, instance.get());
    }

    #[test]
    fn basic_cap_max() {
        let int: u8 = 200;
        let mut instance = IntSanitizer::new(int);
        instance.clamp(99, 101);
        assert_eq!(101, instance.get());
    }
}
