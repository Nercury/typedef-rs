//! TypeDef is used to identify and compare types, as well as print their names.
//!
//! If you do not need readable type name, you should use `TypeId`. This
//! wrapper re-implements `TypeId`.
//!
//! Since Rust 1.0, this library can only work on nightly Rust. To activate the nice names instead
//! of gobbledygook, include this library with `features = ["nightly"]` configuration parameter.
//! On stable rust, it falls back to gobbledygook (type identifier) instead of a nice name.
//!
//! To get a name of a type:
//!
//! ``` ignore
//! use typedef::{ TypeDef };
//!
//! assert_eq!(TypeDef::name_of::<i64>(), "i64");
//! ```
//!
//! Type can also serve as type identifier and name container:
//!
//! ``` ignore
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
//! ``` ignore
//! use std::any::Any;
//! use std::fmt;
//! use typedef::TypeDef;
//!
//! fn foo<T: Any + fmt::Debug>(value: T) -> String {
//!     format!(
//!         "the value of {} type is {:?}",
//!         TypeDef::of::<T>(),
//!         value
//!     )
//! }
//!
//! fn main() {
//!     assert_eq!(foo(15), "the value of i32 type is 15");
//! }
//! ```

#![cfg_attr(feature = "nightly", feature(core_intrinsics))]

use std::any::{Any, TypeId};
use std::fmt;
use std::borrow::Cow;

/// Create a TypeDef structure to identify a type and to print its name.
///
/// ``` ignore
/// use typedef::{ TypeDef };
///
/// let typedef = TypeDef::of::<i64>();
///
/// assert!(typedef.is::<i64>());
/// assert!(typedef.get_str() == "i64");
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg(feature = "nightly")]
pub struct TypeDef {
    id: TypeId,
    name: &'static str,
}

#[derive(Debug, Clone, Copy)]
#[cfg(not(feature = "nightly"))]
pub struct TypeDef {
    id: TypeId,
}

impl TypeDef {
    /// Create a TypeDef structure from a type parameter.
    ///
    /// ```
    /// use typedef::{ TypeDef };
    ///
    /// let typedef = TypeDef::of::<i64>();
    /// ```
    #[cfg(feature = "nightly")]
    pub fn of<T: Any>() -> TypeDef {
        use std::intrinsics::type_name;
        TypeDef {
            id: TypeId::of::<T>(),
            name: unsafe { type_name::<T>() },
        }
    }

    /// Create a TypeDef structure from a type parameter.
    ///
    /// ```
    /// use typedef::{ TypeDef };
    ///
    /// let typedef = TypeDef::of::<i64>();
    /// ```
    #[cfg(not(feature = "nightly"))]
    pub fn of<T: Any>() -> TypeDef {
        TypeDef {
            id: TypeId::of::<T>(),
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
    pub fn id_of<T: Any>() -> TypeId {
        TypeId::of::<T>()
    }

    /// Get type name for specified type directly.
    ///
    /// This only works if this crate is compiled with `features = ["nightly"]`
    ///
    /// ``` ignore
    /// use typedef::{ TypeDef };
    ///
    /// assert_eq!(TypeDef::name_of::<i64>(), "i64");
    /// ```
    #[cfg(feature = "nightly")]
    pub fn name_of<T: Any>() -> Cow<'static, str> {
        use std::intrinsics::type_name;
        Cow::Borrowed(unsafe { type_name::<T>() })
    }

    /// Get type name for specified type directly.
    ///
    /// This only works if this crate is compiled with `features = ["nightly"]`
    ///
    /// ``` ignore
    /// use typedef::{ TypeDef };
    ///
    /// assert_eq!(TypeDef::name_of::<i64>(), "i64");
    /// ```
    #[cfg(not(feature = "nightly"))]
    pub fn name_of<T: Any>() -> Cow<'static, str> {
        Cow::Owned(format!("{}", unsafe { ::std::mem::transmute_copy::<TypeId, u64>(&TypeId::of::<T>()) }))
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
    pub fn is<T: Any>(&self) -> bool {
        self.id == TypeId::of::<T>()
    }

    /// Get the static `&str` for typedef instance.
    ///
    /// ``` ignore
    /// use typedef::{ TypeDef };
    ///
    /// let typedef = TypeDef::of::<i64>();
    ///
    /// assert!(typedef.get_str() == "i64");
    /// ```
    #[cfg(feature = "nightly")]
    pub fn get_str(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.name)
    }

    /// Get the static `&str` for typedef instance.
    ///
    /// This only works if this crate is compiled with `features = ["nightly"]`
    ///
    /// ``` ignore
    /// use typedef::{ TypeDef };
    ///
    /// let typedef = TypeDef::of::<i64>();
    ///
    /// assert!(typedef.get_str() == "i64");
    /// ```
    #[cfg(not(feature = "nightly"))]
    pub fn get_str(&self) -> Cow<'static, str> {
        Cow::Owned(format!("{}", unsafe { ::std::mem::transmute_copy::<TypeId, u64>(&self.id) }))
    }
}

impl PartialEq for TypeDef {
    fn eq(&self, other: &TypeDef) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for TypeDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.get_str())
    }
}

#[cfg(test)]
mod test {
    use super::TypeDef;

    #[test]
    fn should_match_type() {
        assert!(TypeDef::of::<i16>().is::<i16>());
    }

    #[test]
    fn should_not_match_incorrect_type() {
        assert!(!TypeDef::of::<i16>().is::<i32>());
    }

    #[test]
    #[cfg(not(feature = "nightly"))]
    fn should_return_type_name() {
        assert_eq!(TypeDef::of::<i16>().get_str().into_owned(), format!("{:?}", type_id_fallback::<i16>()));
        assert_eq!(TypeDef::of::<i64>().get_str().into_owned(), format!("{:?}", type_id_fallback::<i64>()));
    }

    #[test]
    #[cfg(feature = "nightly")]
    fn should_return_type_name() {
        assert_eq!(&TypeDef::of::<i16>().get_str(), "i16");
        assert_eq!(&TypeDef::of::<i64>().get_str(), "i64");
    }

    #[test]
    fn should_be_equal_to_another_typedef_of_the_same_type() {
        assert_eq!(TypeDef::of::<i16>(), TypeDef::of::<i16>());
    }

    #[test]
    fn should_not_be_equal_to_another_typedef_of_different_type() {
        assert!(TypeDef::of::<i16>() != TypeDef::of::<i32>());
    }

    #[cfg(not(feature = "nightly"))]
    fn type_id_fallback<T: 'static>() -> u64 {
        use std::any::TypeId;
        unsafe { ::std::mem::transmute_copy::<TypeId, u64>(&TypeId::of::<T>()) }
    }
}
