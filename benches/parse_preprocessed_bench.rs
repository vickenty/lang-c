extern crate lang_c;

use lang_c::driver;
use std::time::Instant;

fn main() {
    let mut _config = driver::Config::with_clang();
    #[cfg(target_os = "macos")]
    {
        _config.cpp_options.push("-fno-blocks".into());
    }
    let code = include_str!("yarpgen_autogen.c");

    let start = Instant::now();
    let _ast = match driver::parse_preprocessed(&_config, code.to_string()) {
        Err(err) => panic!(
            "Error in line {} col {} {:?}",
            err.line, err.column, err.expected
        ),
        Ok(ast) => ast,
    };
    println!("Took {} milliseconds.", start.elapsed().as_millis());
}
