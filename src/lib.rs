/*!
<a href="https://github.com/Nercury/typedef-rs">
    <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_darkblue_121621.png" alt="Fork me on GitHub">
</a>
<style>.sidebar { margin-top: 53px }</style>
*/

#![allow(unstable)]

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
//! assert_eq!(TypeDef::name_of::<i64>(), "i64");
//! ```
//!
//! Type can also serve as type identifier and name container:
//!
//! ```
//! use typedef::{ TypeDef };
//!
//! let typedef = TypeDef::of::<i64>();
//!
//! assert!(typedef.is::<i64>());
//! assert_eq!(typedef.get_str(), "i64");
//! ```
//!
//! More common usage would be in a generic method:
//!
//! ```
//! use std::fmt::{ Show };
//! use typedef::{ TypeDef };
//!
//! fn foo<T: 'static + Show>(value: T) -> String {
//!     format!(
//!         "the value of {:?} type is {:?}",
//!         TypeDef::of::<T>(),
//!         value
//!     )
//! }
//!
//! fn main() {
//!     assert_eq!(foo(15), "the value of i32 type is 15i32");
//! }
//! ```

use std::any::TypeId;
use std::intrinsics::get_tydesc;
use std::fmt;

/// Create a TypeDef structure to identify a type and to print its name.
///
/// ```
/// use typedef::{ TypeDef };
///
/// let typedef = TypeDef::of::<i64>();
///
/// assert!(typedef.is::<i64>());
/// assert!(typedef.get_str() == "i64");
/// ```
#[stable]
#[derive(Clone, Copy)]
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
    /// let typedef = TypeDef::of::<i64>();
    /// ```
    #[stable]
    pub fn of<T: 'static>() -> TypeDef {
        TypeDef {
            type_id: TypeId::of::<T>(),
            type_name: unsafe { (*get_tydesc::<T>()).name },
        }
    }

    /// Get `TypeId` for specified type directly.
    ///
    /// ```
    /// use std::any::{ TypeId };
    /// use typedef::{ TypeDef };
    ///
    /// assert!(TypeDef::id_of::<i64>() == TypeId::of::<i64>());
    /// ```
    #[stable]
    pub fn id_of<T: 'static>() -> TypeId {
        TypeId::of::<T>()
    }

    /// Get type name for specified type directly.
    ///
    /// ```
    /// use typedef::{ TypeDef };
    ///
    /// assert_eq!(TypeDef::name_of::<i64>(), "i64");
    /// ```
    #[stable]
    pub fn name_of<T: 'static>() -> &'static str {
        unsafe { (*get_tydesc::<T>()).name }
    }

    /// Check if typedef instance matches type.
    ///
    /// ```
    /// use typedef::{ TypeDef };
    ///
    /// let typedef = TypeDef::of::<i64>();
    ///
    /// assert!(typedef.is::<i64>());
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
    /// let typedef = TypeDef::of::<i64>();
    ///
    /// assert!(typedef.get_str() == "i64");
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
        assert_eq!(TypeDef::of::<i64>().get_str(), "i64");
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
