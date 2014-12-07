//! TypeDef is used to identify and compare types, as well as print their names.
//!
//! If you do not need readable type name, you should use `TypeId`. This
//! wrapper re-implements `TypeId`.
//!
//! To get a name of a type:
//!
//! ```
//! use typedef::{ TypeDef };
//!
//! fn main() {
//!     println!("type of variable is {}", TypeDef::of::<int>());
//! }
//! ```
//!
//! __>__ `type of variable is int`
//!
//! More common usage would be in a generic method:
//!
//! ```
//! use std::fmt::{ Show };
//! use typedef::{ TypeDef };
//!
//! fn foo<T: 'static + Show>(value: T) {
//!     println!(
//!         "the value of {} type is {}",
//!         TypeDef::of::<T>(),
//!         value
//!     );
//! }
//!
//! fn main() {
//!     foo(15i);
//! }
//! ```
//!
//! __>__ `the value of int type is 15`
//!
//! You can also compare type objects, as well as use `is` method to compare
//! with known type.

use std::intrinsics::TypeId;
use std::intrinsics::get_tydesc;
use std::fmt;

/// Create a TypeDef structure to identify a type and to print its name.
///
/// ```
/// use typedef::{ TypeDef };
///
/// let typedef = TypeDef::of::<int>();
///
/// assert!(typedef.is::<int>());
/// assert!(typedef.get_str() == "int");
/// ```
#[stable]
#[deriving(Clone)]
pub struct TypeDef {
    type_id: TypeId,
    type_name: &'static str,
}

#[stable]
impl TypeDef {
    /// Create a TypeDef structure from a type parameter.
    ///
    /// ```
    /// use typedef::{ TypeDef };
    ///
    /// let typedef = TypeDef::of::<int>();
    /// ```
    #[stable]
    pub fn of<T: 'static>() -> TypeDef {
        TypeDef {
            type_id: TypeId::of::<T>(),
            type_name: unsafe { (*get_tydesc::<T>()).name },
        }
    }

    /// Check if typedef instance matches type.
    ///
    /// ```
    /// use typedef::{ TypeDef };
    ///
    /// let typedef = TypeDef::of::<int>();
    ///
    /// assert!(typedef.is::<int>());
    /// ```
    #[stable]
    pub fn is<T: 'static>(&self) -> bool {
        self.type_id == TypeId::of::<T>()
    }

    /// Get the static `&str` for typedef instance.
    ///
    /// ```
    /// use typedef::{ TypeDef };
    ///
    /// let typedef = TypeDef::of::<int>();
    ///
    /// assert!(typedef.get_str() == "int");
    /// ```
    #[stable]
    pub fn get_str(&self) -> &'static str {
        self.type_name
    }
}

#[stable]
impl PartialEq for TypeDef {
    #[stable]
    fn eq(&self, other: &TypeDef) -> bool {
        self.type_id == other.type_id
    }
}

#[stable]
impl fmt::Show for TypeDef {
    #[stable]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_name)
    }
}

#[cfg(test)]
mod test {
    use super::{ TypeDef };

    #[test]
    fn should_match_type() {
        assert!(TypeDef::of::<i16>().is::<i16>());
    }

    #[test]
    #[should_fail]
    fn should_not_match_incorrect_type() {
        assert!(TypeDef::of::<i16>().is::<i32>());
    }

    #[test]
    fn should_return_type_name() {
        assert_eq!(TypeDef::of::<i16>().get_str(), "i16");
        assert_eq!(TypeDef::of::<int>().get_str(), "int");
    }

    #[test]
    fn should_be_equal_to_another_typedef_of_the_same_type() {
        assert_eq!(TypeDef::of::<i16>(), TypeDef::of::<i16>());
    }

    #[test]
    fn should_not_be_equal_to_another_typedef_of_different_type() {
        assert!(TypeDef::of::<i16>() != TypeDef::of::<i32>());
    }
}
