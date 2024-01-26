//! Used for getting keys for algebraic-type enums and referencing algebraic-type enum variants by their key.
//!
//! # Example
//!
//! A simple example of using the derive macro:
//! ```
//! use enum_key::{EnumKey, KeyableEnum};
//!
//! #[derive(KeyableEnum)]
//! pub enum MyEnum {
//!     A(u8),
//!     B {
//!         inside: u32
//!     },
//!     C,
//! }
//!
//! type Key = <MyEnum as KeyableEnum>::Key;
//!
//! assert_eq!(Key::VALUES, &[Key::A, Key::B, Key::C], "VALUES should contain entries for each variant");
//! assert_eq!(MyEnum::A(42).get_enum_key(), Key::A, "Variant A should have a key of A");
//! ```
//!
//! A simple example using the `key_enum` attribute:
//! ```
//! use enum_key::KeyableEnum;
//!
//! #[derive(KeyableEnum)]
//! #[keyable_enum(key_enum = MyKey)]
//! pub enum MyEnum {
//!     A(u8),
//!     B(u8),
//! }
//!
//! assert_eq!(MyEnum::A(42).get_enum_key(), MyKey::A, "Variant A should have a key A");
//! ```
//!
//! # Macro Attributes
//!
//! The macro has attributes:
//! * `key_enum` - An optional identifier used for the name of the enum used for keys of the original enum.

use std::fmt::Debug;
use std::hash::Hash;

pub use enum_key_derive::KeyableEnum;

/// An enum that has keys associated with each variant.
///
/// This trait is usually derived.
pub trait KeyableEnum {
    /// This enum's key type.
    type Key: EnumKey;

    /// Gets the key for a given variant.
    fn get_enum_key(&self) -> Self::Key;
}

/// For usually generated key types representing all the variants of an enum.
pub trait EnumKey:
    Debug + Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + 'static
{
    /// The enum type this key type is associated with.
    type Enum: KeyableEnum;

    /// An array of all key types.
    const VALUES: &'static [Self];
}
