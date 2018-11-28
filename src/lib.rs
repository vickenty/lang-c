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

pub mod ast;
pub mod driver;
pub mod span;

mod env;
mod parser;
mod astutil;
mod strings;

#[cfg(test)]
mod tests;
