//! This is a test project for racer.
//!
//! # Example:
//! Basic Usage.
//!
//! ```
//! extern crate fixtures;
//! use fixtures::foo;
//! fn main {
//!     println!("Racer")
//! }
//! ```
//!
//! ## Notes:
//! - We should check racer can parse rust doc style comments
//! - and some comments...

#[path = "submod/bar.rs"]
pub mod bar;

pub mod foo;

pub use test_crate2::useless_func;

pub type UsizeVec = Vec<usize>;
