//! Parse a C file and dump the AST.

extern crate lang_c;

use std::process::exit;

fn main() {
    let mut config = lang_c::driver::Config::default();
    let mut source = None;
    let mut quiet = false;

    for opt in std::env::args().skip(1) {
        if opt == "-q" {
            quiet = true;
        } else if opt.starts_with("-") {
            config.options.push(opt);
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
        Ok(parse) => if !quiet {
            println!("{:#?}", parse.unit);
        },
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }
}
