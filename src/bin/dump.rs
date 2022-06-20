//! Parse a C file and dump the AST.

extern crate lang_c;

use std::process::exit;

use lang_c::driver::{Config, Flavor};
use lang_c::visit::Visit;

fn main() {
    let mut config = Config::default();
    let mut source = None;
    let mut quiet = false;

    for opt in std::env::args().skip(1) {
        if opt == "-use-gcc" {
            config = Config::with_gcc();
        } else if opt == "-use-clang" {
            config = Config::with_clang();
        } else if opt == "-use-std" {
            config.flavor = Flavor::StdC11;
        } else if opt == "-q" {
            quiet = true;
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

    match lang_c::driver::parse(&config, &source) {
        Ok(parse) => {
            if !quiet {
                let mut buf = String::new();
                {
                    let mut printer = lang_c::print::Printer::new(&mut buf);
                    printer.visit_translation_unit(&parse.unit);
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
