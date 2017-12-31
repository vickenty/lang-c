//! Preprocess and parse C source file into an abstract syntax tree

use std::collections::HashSet;
use std::error;
use std::fmt;
use std::io;
use std::path::Path;
use std::process::Command;

use ast::TranslationUnit;
use env::Env;
use parser::translation_unit;

/// Parser configuration
#[derive(Clone, Debug, Default)]
pub struct Config {
    /// Options to pass to the preprocessor program
    pub options: Vec<String>,
    /// Language flavor to parse
    pub flavor: Flavor,
}

/// C language flavors
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Flavor {
    /// Strict standard C11
    StdC11,
    /// Standard C11 with GNU extensions
    GnuC11,
}

impl Default for Flavor {
    fn default() -> Self {
        Flavor::GnuC11
    }
}

/// Result of a successful parse
#[derive(Clone, Debug)]
pub struct Parse {
    /// Pre-processed source text
    pub source: String,
    /// Root of the abstract syntax tree
    pub unit: TranslationUnit,
}

#[derive(Debug)]
/// Error type returned from `parse`
pub enum Error {
    PreprocessorError(io::Error),
    SyntaxError(SyntaxError),
}

impl From<SyntaxError> for Error {
    fn from(e: SyntaxError) -> Error {
        Error::SyntaxError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::PreprocessorError(ref e) => write!(fmt, "preprocessor error: {}", e),
            &Error::SyntaxError(ref e) => write!(fmt, "syntax error: {}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::PreprocessorError(_) => "preprocessor error",
            &Error::SyntaxError(_) => "syntax error",
        }
    }
}

/// Syntax error during parsing
#[derive(Debug, Clone)]
pub struct SyntaxError {
    /// Pre-processed source text
    pub source: String,
    /// Line number in the preprocessed source
    pub line: usize,
    /// Column number in the preprocessed source
    pub column: usize,
    /// Byte position in the preproccessed source
    pub offset: usize,
    /// Tokens expected at the error location
    pub expected: HashSet<&'static str>,
}

impl SyntaxError {
    /// Quoted and comma-separated list of expected tokens
    pub fn format_expected(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut list = self.expected.iter().collect::<Vec<_>>();
        list.sort();
        for (i, t) in list.iter().enumerate() {
            if i > 0 {
                write!(fmt, ", ")?;
            }
            write!(fmt, "'{}'", t)?;
        }

        Ok(())
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "unexpected token at line {} column {}, expected ",
            self.line, self.column
        )?;
        self.format_expected(fmt)
    }
}

/// Parse a C file
pub fn parse<P: AsRef<Path>>(config: &Config, source: &P) -> Result<Parse, Error> {
    let processed = match preprocess(config, source.as_ref()) {
        Ok(s) => s,
        Err(e) => return Err(Error::PreprocessorError(e)),
    };

    Ok(parse_preprocessed(config, processed)?)
}

pub fn parse_preprocessed(config: &Config, source: String) -> Result<Parse, SyntaxError> {
    let mut env = match config.flavor {
        Flavor::StdC11 => Env::with_gnu(false),
        Flavor::GnuC11 => Env::with_gnu(true),
    };

    match translation_unit(&source, &mut env) {
        Ok(unit) => Ok(Parse {
            source: source,
            unit: unit,
        }),
        Err(err) => Err(SyntaxError {
            source: source,
            line: err.line,
            column: err.column,
            offset: err.offset,
            expected: err.expected,
        }),
    }
}

fn preprocess(config: &Config, source: &Path) -> io::Result<String> {
    let mut cmd = Command::new("cpp");

    for item in &config.options {
        cmd.arg(item);
    }

    cmd.arg(source);

    let output = cmd.output()?;

    if output.status.success() {
        match String::from_utf8(output.stdout) {
            Ok(s) => Ok(s),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    } else {
        match String::from_utf8(output.stderr) {
            Ok(s) => Err(io::Error::new(io::ErrorKind::Other, s)),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::Other,
                "cpp error contains invalid utf-8",
            )),
        }
    }
}
