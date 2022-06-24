//! # Partial Functional types for Rust
//!
//! I have tried taking some functional programming types into Rust types.
//! You will find types that are in the group of [Semigroup] and [Monoid]s as well as [Functor](functor::Functor).
//!
//! It is meant to bring some more functional programming tricks in to rust.
//!
//! ## Examples
//! ```
//! use partial_functional::prelude::*;
//!
//! #[derive(Debug, PartialEq)]
//! struct OrderLine {
//!     product_code: String,
//!     quantity: Sum<u32>,
//!     price: Sum<f32>,
//! }
//!
//! impl Semigroup for OrderLine {
//!     fn combine(self, other: Self) -> Self {
//!         Self {
//!             product_code: String::from("TOTAL"),
//!             quantity: self.quantity.combine(other.quantity),
//!             price: self.price.combine(other.price),
//!         }
//!     }
//! }
//!
//! impl Monoid for OrderLine {
//!     fn empty() -> Self {
//!         Self { product_code: String::from(""), quantity: Sum::empty(), price: Sum::empty() }
//!     }
//! }
//!
//! let order_lines = vec![
//!     OrderLine { product_code: String::from("AAA"), quantity: Sum(2), price: Sum(19.98) },
//!     OrderLine { product_code: String::from("BBB"), quantity: Sum(1), price: Sum(1.99) },
//!     OrderLine { product_code: String::from("CCC"), quantity: Sum(3), price: Sum(3.99) },
//! ];
//!
//! let total = order_lines
//!     .into_iter()
//!     .fold(OrderLine::empty(), |acc, item| acc.combine(item));
//!
//! let expected = OrderLine { product_code: "TOTAL".into(), quantity: 6.into(), price: 25.96.into() };
//! assert_eq!(expected, total);
//! ```

pub mod functor;
pub mod hkt;
pub mod monoid;
pub mod semigroup;

pub use hkt::*;
pub use monoid::{All, Any, First, Last, Monoid, Product, Sum};
pub use semigroup::Semigroup;

pub mod prelude {
    pub use crate::{
        monoid::{All, Any, First, Last, Monoid, Product, Sum},
        semigroup::Semigroup,
    };
}
