//! Parse a C file and dump the AST.

extern crate lang_c;

use std::process::exit;
use std::path::Path;

use lang_c::driver::Config;
use lang_c::visit::Visit;
use lang_c::interner::{Interner, DummyInterner, Rodeo};

fn main() {
    let mut config = Config::default();
    let mut source = None;
    let mut quiet = false;
    let mut intern = false;

    for opt in std::env::args().skip(1) {
        if opt == "-use-gcc" {
            config = Config::with_gcc();
        } else if opt == "-use-clang" {
            config = Config::with_clang();
        } else if opt == "-q" {
            quiet = true;
        } else if opt == "-use-intern" {
            intern = true;
        } else if opt.starts_with("-") {
            config.cpp_options.push(opt);
        } else {
            if source.is_none() {
                source = Some(opt);
            } else {
                println!("multiple input files given");
                exit(1);
            }
        }
    }

    let source = match source {
        Some(s) => s,
        None => {
            println!("input file required");
            exit(1);
        }
    };

    if intern {
        eprintln!("using hash interner");
        parse_ext(&mut Rodeo::new(), &config, &source, quiet);
    } else {
        eprintln!("using dummy interner");
        parse_ext(&mut DummyInterner, &config, &source, quiet);
    }
}

fn parse_ext<T: Interner>(interner: &mut T, config: &Config, source: &str, quiet: bool) {
    let res = lang_c::driver::parse_ext(interner, config, source);
    match res {
        Ok(parse) => {
            if !quiet {
                let mut buf = String::new();
                {
                    // let mut printer = lang_c::print::Printer::new(&mut buf);
                    // printer.visit_translation_unit(&parse.unit);
                }
                println!("{}", buf);
            }
        }
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }
}
