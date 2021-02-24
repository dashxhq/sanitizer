use std::convert::From;

macro_rules! impl_from {
    ( $type : tt ) => {
        impl From<$type> for IntSanitizer<$type> {
            fn from(content: $type) -> Self {
                Self::new(content)
            }
        }
    };
}
/// The IntSanitizer structure is a wrapper over a type T which is to
/// be sanitized, T can be anything that's `PartialOrd`
///
/// # Example
///
/// ```
/// use sanitizer::prelude::*;
///
/// let mut instance = IntSanitizer::from(5);
/// instance
/// 	.clamp(9, 15);
/// assert_eq!(instance.get(), 9);
/// ```
///
pub struct IntSanitizer<T: PartialOrd + Copy>(T);

// TODO: Remove Copy since its restrictive
impl<T: PartialOrd + Copy> IntSanitizer<T> {
    /// Make a new instance of the struct from the given T
    pub(crate) fn new(int: T) -> Self {
        Self(int)
    }
    /// Consume the struct and return T
    pub fn get(self) -> T {
        self.0
    }
    /// Sets the int equal to the max value if it exceds the provided
    /// max value provided in the function argument
    pub fn clamp(&mut self, min: T, max: T) -> &mut Self {
        self.0 = num_traits::clamp(self.0, min, max);
        self
    }
    /// Call a custom function for sanitizing the value of type T
    pub fn call<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(T) -> T,
    {
        self.0 = func(self.0);
        self
    }
}

impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);
impl_from!(usize);
impl_from!(isize);
impl_from!(i64);
impl_from!(i32);
impl_from!(i16);
impl_from!(i8);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_cap_min() {
        let int: u8 = 50;
        let mut instance = IntSanitizer::from(int);
        instance.clamp(99, 101);
        assert_eq!(99, instance.get());
    }

    #[test]
    fn basic_cap_max() {
        let int: u8 = 200;
        let mut instance = IntSanitizer::from(int);
        instance.clamp(99, 101);
        assert_eq!(101, instance.get());
    }
}
