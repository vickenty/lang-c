//! Preprocess and parse C source file into an abstract syntax tree

use std::collections::HashSet;
use std::error;
use std::fmt;
use std::io;
use std::path::Path;
use std::process::Command;

use ast::Name;
use ast::TranslationUnit;
use env::Env;
use parser::translation_unit;

/// Parser configuration
#[derive(Clone, Debug)]
pub struct Config {
    /// Command used to invoke C preprocessor
    pub cpp_command: String,
    /// Options to pass to the preprocessor program
    pub cpp_options: Vec<String>,
    /// Language flavor to parse
    pub flavor: Flavor,
}

impl Config {
    /// Use `gcc` as a pre-processor and enable gcc extensions
    pub fn with_gcc() -> Config {
        Config {
            cpp_command: "gcc".into(),
            cpp_options: vec!["-E".into()],
            flavor: Flavor::GnuC11,
        }
    }

    /// Use `clang` as a pre-processor and enable Clang extensions
    pub fn with_clang() -> Config {
        Config {
            cpp_command: "clang".into(),
            cpp_options: vec!["-E".into()],
            flavor: Flavor::ClangC11,
        }
    }
}

impl Default for Config {
    #[cfg(target_os = "macos")]
    fn default() -> Config {
        Self::with_clang()
    }

    #[cfg(not(target_os = "macos"))]
    fn default() -> Config {
        Self::with_gcc()
    }
}

/// C language flavors
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Flavor {
    /// Strict standard C11
    StdC11,
    /// Standard C11 with GNU extensions
    GnuC11,
    /// Standard C11 with Clang extensions
    ClangC11,
}

/// Result of a successful parse
#[derive(Clone, Debug)]
pub struct Parse {
    /// Pre-processed source text
    pub source: String,
    /// Root of the abstract syntax tree
    pub unit: TranslationUnit<String>,
}

/// Result of a successful parse for generic identifier type
#[derive(Clone, Debug)]
pub struct ParseExt<T: Name> {
    /// Pre-processed source text
    pub source: String,
    /// Root of the abstract syntax tree
    pub unit: TranslationUnit<T>,
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
                try!(write!(fmt, ", "));
            }
            try!(write!(fmt, "'{}'", t));
        }

        Ok(())
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(
            fmt,
            "unexpected token at line {} column {}, expected ",
            self.line, self.column
        ));
        self.format_expected(fmt)
    }
}

/// Parse a C file
pub fn parse<P: AsRef<Path>>(config: &Config, source: P) -> Result<Parse, Error> {
    let processed = match preprocess(config, source.as_ref()) {
        Ok(s) => s,
        Err(e) => return Err(Error::PreprocessorError(e)),
    };

    Ok(try!(parse_preprocessed(config, processed)))
}

pub fn parse_preprocessed(config: &Config, source: String) -> Result<Parse, SyntaxError> {
    let mut env = match config.flavor {
        Flavor::StdC11 => Env::with_core(),
        Flavor::GnuC11 => Env::with_gnu(),
        Flavor::ClangC11 => Env::with_clang(),
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

/// Parse a C file using a custom string type extended setup
pub fn parse_ext<P: AsRef<Path>, T: Name>(config: &Config, source: P) -> Result<ParseExt<T>, Error> {
    let processed = match preprocess(config, source.as_ref()) {
        Ok(s) => s,
        Err(e) => return Err(Error::PreprocessorError(e)),
    };

    Ok(try!(parse_preprocessed_ext::<P,T>(config, processed)))
}

pub fn parse_preprocessed_ext<P: AsRef<Path>, T: Name>(config: &Config, source: String) -> Result<ParseExt<T>, SyntaxError> {
    let mut env = match config.flavor {
        Flavor::StdC11 => Env::with_core(),
        Flavor::GnuC11 => Env::with_gnu(),
        Flavor::ClangC11 => Env::with_clang(),
    };

    match translation_unit(&source, &mut env) {
        Ok(unit) => Ok(ParseExt::<T> {
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
    let mut cmd = Command::new(&config.cpp_command);

    for item in &config.cpp_options {
        cmd.arg(item);
    }

    cmd.arg(source);

    let output = try!(cmd.output());

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
