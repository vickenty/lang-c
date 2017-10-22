//! C language parser and abstract syntax tree

pub mod ast;
pub mod driver;
pub mod span;

mod env;
mod parser;
mod astutil;
mod strings;

#[cfg(test)]
mod tests;
