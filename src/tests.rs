use std::env;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::io::stdout;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::mem;
use std::path::PathBuf;

use env::Env;
use interner::DummyInterner;
use parser;
use print::Printer;
use span::Span;
use visit::Visit;

struct Case {
    path: PathBuf,
    name: String,
    kind: Kind,
    pragma: Vec<Pragma>,
    source: String,
    expect: String,
}

const OUTPUT_START: &'static str = "/*===";
const OUTPUT_END: &'static str = "===*/";

impl Case {
    fn from_path(entry: &DirEntry) -> io::Result<Case> {
        let name = entry.file_name();
        let name = name.to_str().expect("path to string");
        let kind = name.split("-").next().expect("case contains hyphen");
        let kind = Kind::from_str(&kind).expect("unknown test case kind");

        let file = BufReader::new(try!(File::open(entry.path())));

        let mut pragma = Vec::new();
        let mut expect = String::new();
        let mut source = String::new();
        let mut in_exp = false;

        for line in file.lines() {
            let target = if in_exp { &mut expect } else { &mut source };

            let line = try!(line);
            let line = line.trim_right();
            if line.is_empty() || line.starts_with("//") {
                continue;
            } else if line.starts_with("#pragma") {
                pragma.push(Pragma::from_str(line).expect("unknown pragma"));
            } else if line == OUTPUT_START {
                in_exp = true;
            } else if line == OUTPUT_END {
                in_exp = false;
            } else {
                target.push_str(line);
                target.push_str("\n");
            }
        }

        Ok(Case {
            path: entry.path(),
            name: name.to_owned(),
            kind: kind,
            pragma: pragma,
            source: source,
            expect: expect,
        })
    }

    fn run(&self) -> bool {
        let mut interner = DummyInterner::default();

        // As the Env will borrow the interner, we have only one shot to create the Env.
        enum EnvType {
            Gnu,
            Clang,
            None,
        }

        let mut env_type = EnvType::None;

        for pragma in &self.pragma {
            match *pragma {
                Pragma::Gnu => env_type = EnvType::Gnu,
                Pragma::Clang => env_type = EnvType::Clang,
                _ => {}
            }
        }

        let mut env = match env_type {
            EnvType::Gnu => Env::with_gnu(&mut interner),
            EnvType::Clang => Env::with_clang(&mut interner),
            EnvType::None => Env::with_core(&mut interner),
        };

        for pragma in &self.pragma {
            match *pragma {
                Pragma::Typedef(ref name) => env.add_typename(&name),
                _ => {}
            }
        }

        let (actual, error) = match self.kind.parse_and_print(&self.source, &mut env) {
            Ok(s) => (s, None),
            Err(e) => ("~ERROR\n".to_string(), Some(e)),
        };

        let pragma_fail = self
            .pragma
            .iter()
            .filter(|p| match **p {
                Pragma::IsTypename(ref name) => !env.is_typename(name),
                _ => false,
            })
            .collect::<Vec<_>>();

        let output_matches = actual == self.expect;
        let success = output_matches && pragma_fail.is_empty();

        if !success {
            writeln!(stdout(), "\n{}:", self.name).unwrap();
            writeln!(stdout(), "{}", self.source).unwrap();
            if let Some(e) = error {
                writeln!(stdout(), "ERROR:\n{}", e).unwrap();
            }
        }

        if !pragma_fail.is_empty() {
            writeln!(stdout(), "Failed checks: ").unwrap();
            for failed in &pragma_fail {
                writeln!(stdout(), "    {:?}", failed).unwrap();
            }
        }

        if !output_matches {
            let width = self.expect.lines().map(|s| s.len()).max().unwrap_or(25);
            let mut alines = Some(self.expect.lines());
            let mut blines = Some(actual.lines());
            while alines.is_some() || blines.is_some() {
                let a = match alines.as_mut().and_then(|i| i.next()) {
                    Some(l) => l,
                    None => {
                        alines = None;
                        ""
                    }
                };
                let b = match blines.as_mut().and_then(|i| i.next()) {
                    Some(l) => l,
                    None => {
                        blines = None;
                        ""
                    }
                };
                writeln!(stdout(), "{:w$} | {}", a, b, w = width).unwrap();
            }

            if env::var_os("TEST_UPDATE").is_some() {
                self.save(&actual).expect("failed to update test output");
            }
        }

        success
    }

    fn save(&self, actual: &str) -> io::Result<()> {
        let mut buf = String::new();
        let mut file = BufReader::new(try!(File::open(&self.path)));
        let mut content = Vec::new();
        while try!(file.read_line(&mut buf)) > 0 {
            content.push(mem::replace(&mut buf, String::new()));
        }

        let mut file = BufWriter::new(try!(File::create(&self.path)));
        let mut lines = content.into_iter();
        for line in &mut lines {
            try!(file.write_all(line.as_bytes()));
            if line.trim_right() == OUTPUT_START {
                break;
            }
        }
        try!(file.write_all(actual.as_bytes()));
        for line in &mut lines {
            if line.trim_right() == OUTPUT_END {
                try!(file.write_all(line.as_bytes()));
                break;
            }
        }
        for line in &mut lines {
            try!(file.write_all(line.as_bytes()));
        }

        Ok(())
    }
}

enum Kind {
    Constant,
    Declaration,
    Expression,
    Statement,
    TranslationUnit,
}

impl Kind {
    fn from_str(kind: &str) -> Option<Kind> {
        Some(match kind {
            "constant" => Kind::Constant,
            "declaration" => Kind::Declaration,
            "expression" => Kind::Expression,
            "statement" => Kind::Statement,
            "translation_unit" => Kind::TranslationUnit,
            _ => return None,
        })
    }

    fn parse_and_print(
        &self,
        source: &str,
        env: &mut Env<DummyInterner>,
    ) -> Result<String, parser::ParseError> {
        let source = source.trim_right();

        let mut s = "".to_string();
        {
            let mut p = Printer::new(&mut s);
            match *self {
                Kind::Constant => {
                    let n = try!(parser::constant(source, env));
                    p.visit_constant(&n, &Span::none());
                }
                Kind::Declaration => {
                    let n = match parser::declaration(source, env) {
                        Ok(n) => n,
                        Err(e) => return Err(e),
                    };
                    p.visit_declaration(&n.node, &n.span);
                }
                Kind::Statement => {
                    let n = match parser::statement(source, env) {
                        Ok(n) => n,
                        Err(e) => return Err(e),
                    };
                    p.visit_statement(&n.node, &n.span);
                }
                Kind::Expression => {
                    let n = match parser::expression(source, env) {
                        Ok(n) => n,
                        Err(e) => return Err(e),
                    };
                    p.visit_expression(&n.node, &n.span);
                }
                Kind::TranslationUnit => {
                    let n = match parser::translation_unit(source, env) {
                        Ok(n) => n,
                        Err(e) => return Err(e),
                    };
                    p.visit_translation_unit(&n);
                }
            }
        }

        Ok(s)
    }
}

#[derive(Debug)]
enum Pragma {
    /// Enable gnu extensions
    Gnu,
    // Enable clang extensions
    Clang,
    /// Define typename
    Typedef(String),
    /// Assert argument is a typename
    IsTypename(String),
}

impl Pragma {
    fn from_str(line: &str) -> Option<Pragma> {
        let mut line = line
            .split(" ")
            .skip(1)
            .map(|w| w.trim().to_owned())
            .collect::<Vec<_>>();
        Some(match line[0].trim() {
            "gnu" => Pragma::Gnu,
            "clang" => Pragma::Clang,
            "typedef" => Pragma::Typedef(match line.pop() {
                Some(v) => v,
                None => return None,
            }),
            "is_typename" => Pragma::IsTypename(match line.pop() {
                Some(v) => v,
                None => return None,
            }),
            _ => return None,
        })
    }
}

#[test]
fn reftest_main() {
    let mut cases = Vec::new();
    for entry in fs::read_dir("reftests").expect("listing reftests/") {
        let entry = entry.expect("failed to read reftests/ entry");
        let case = match Case::from_path(&entry) {
            Ok(case) => case,
            Err(err) => {
                panic!("{}: {}", entry.path().to_string_lossy(), err);
            }
        };
        cases.push(case);
    }

    let failed = cases.iter().filter(|c| !c.run()).count();
    if failed > 0 {
        panic!("{} cases failed", failed);
    }
}
