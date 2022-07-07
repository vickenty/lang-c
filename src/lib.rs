//! C language parser and abstract syntax tree
//!
//! ```
//! use lang_c::driver::{Config, parse};
//!
//! fn main() {
//!     let config = Config::default();
//!     println!("{:?}", parse(&config, "example.c"));
//! }
//! ```

#![allow(deprecated)]
#![allow(ellipsis_inclusive_range_patterns)]

pub mod ast;
pub mod driver;
pub mod loc;
pub mod print;
pub mod span;
pub mod visit;

mod astutil;
mod env;
mod parser;
mod strings;

#[cfg(test)]
mod tests;
