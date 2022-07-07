//! Convert byte offsets into line numbers
const F_NEW: u32 = 1;
const F_RET: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location<'a> {
    pub file: &'a str,
    pub line: usize,
}

/// Find file name and line number that correspond to an offset in a preprocessed source.
///
/// If location was in an included file, second element of the returned tuple contains a list of
/// locations of `#include` directives, in the order they were procssed (top-level file first, then
/// all intermediate included files).
pub fn get_location_for_offset<'a>(src: &'a str, pos: usize) -> (Location<'a>, Vec<Location<'a>>) {
    let mut p = 0;
    let mut inc = Vec::new();
    let mut loc = Location { file: "", line: 1 };

    while p < pos {
        let n = p + src[p..].find("\n").unwrap_or(src[p..].len());
        if pos <= n {
            break;
        }

        if let Some((l, f)) = parse_line_directive(&src[p..n]) {
            if f & F_NEW == F_NEW {
                inc.push(loc);
            }
            if f & F_RET == F_RET {
                inc.pop();
            }
            loc = l;
        } else {
            loc.line += 1;
        }

        p = n + 1;
    }
    (loc, inc)
}

#[test]
fn test_get_location_for_offset() {
    fn t(src: &str, pos: usize, file: &str, line: usize, includes: &[(&str, usize)]) {
        let (loc, inc) = get_location_for_offset(src, pos);
        assert_eq!(
            loc,
            Location {
                file: file,
                line: line
            }
        );
        assert_eq!(inc.len(), includes.len());
        for (loc, &(f, l)) in inc.iter().zip(includes) {
            assert_eq!(loc, &Location { file: f, line: l });
        }
    }

    t("# 10 \"init\"", 0, "", 1, &[]);
    t("# 10 \"init\"", 1, "", 1, &[]);
    t("# 10 \"init\"\na\nb\n", 12, "init", 10, &[]);
    t("# 10 \"init\"\na\nb\n", 13, "init", 10, &[]);
    t("# 10 \"init\"\na\nb\n", 14, "init", 11, &[]);
    t("# 10 \"init\"\na\nb\n", 15, "init", 11, &[]);

    const T: &'static str = r#"
# 10 "foo"
...
# 1 "bar" 1 3 4
# 5 "bar" 3
# 11 "baz" 1
...
...
# 6 "bar" 2
...
# 15 "foo" 2
...
...
# 2 "ook" 1
"#;
    t(T, 12, "foo", 10, &[]);
    t(T, 32, "bar", 1, &[("foo", 11)]);
    t(T, 61, "baz", 12, &[("foo", 11), ("bar", 5)]);
    t(T, 77, "bar", 6, &[("foo", 11)]);
    t(T, 98, "foo", 16, &[]);
    t(T, 102, "foo", 17, &[]);
    t(T, 114, "ook", 2, &[("foo", 17)]);
}

macro_rules! otry {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return None,
        }
    };
}

fn strip_prefix<'a>(s: &'a str, p: &str) -> Option<&'a str> {
    if s.starts_with(p) {
        Some(&s[p.len()..])
    } else {
        None
    }
}

// https://gcc.gnu.org/onlinedocs/cpp/Preprocessor-Output.html
fn parse_line_directive(s: &str) -> Option<(Location, u32)> {
    let s = otry!(strip_prefix(s, "# "));
    let n = otry!(s.find(" "));
    let line = otry!(usize::from_str_radix(&s[..n], 10).ok());

    let s = otry!(strip_prefix(&s[n..], " \""));
    let mut n = 0;
    while n < s.len() {
        n += otry!(s[n..].find(&['"', '\\'][..]));
        if s[n..].starts_with('"') {
            break;
        }
        n += otry!(s[n..].char_indices().nth(2).map(|p| p.0));
    }
    let file = &s[..n];
    let s = otry!(strip_prefix(&s[n..], "\""));

    let flags = s.bytes().filter(|&c| c >= b'1' && c <= b'4');
    let flags = flags.fold(0, |a, f| a | 1 << (f - b'1'));

    Some((
        Location {
            file: file,
            line: line,
        },
        flags,
    ))
}

#[test]
fn test_line_directive() {
    assert_eq!(
        parse_line_directive(r#"# 14 "ab\"\000" 1 2 3 4"#),
        Some((
            Location {
                file: r#"ab\"\000"#,
                line: 14
            },
            15,
        ))
    );
    assert_eq!(parse_line_directive("#"), None);
    assert_eq!(parse_line_directive("# 1 "), None);
    assert_eq!(parse_line_directive("# 1 \""), None);
    assert_eq!(parse_line_directive("# -1 \"\""), None);
    assert_eq!(
        parse_line_directive("# 0 \"\""),
        Some((Location { file: "", line: 0 }, 0))
    );
    assert_eq!(parse_line_directive("# 0 \"# #\x0a\x0a\\"), None);
    assert_eq!(parse_line_directive("# 0 \"\\"), None);
    assert_eq!(parse_line_directive("# 0 \"\\…"), None);
}
