use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Serialize, Deserialize)]
struct Content {
    prefix: String,
    body: Vec<String>,
}

fn main() -> io::Result<()> {
    let mut snippet = Map::new();
    let files = get_all_files("./snippets")?;

    for (name, mut fp) in files.into_iter() {
        let mut code = String::new();
        fp.read_to_string(&mut code).unwrap();
        let val = generate_snippet(&name, &code);
        snippet.insert(name, val);
    }

    let mut output = File::create("output.txt").unwrap();
    write!(output, "{}", serde_json::to_string(&snippet).unwrap()).unwrap();

    Ok(())
}

fn generate_snippet(name: &str, code: &str) -> Value {
    let lines: Vec<String> = code
        .lines()
        .map(|s| {
            let mut chars = s.chars().peekable();
            let mut buf = String::new();
            while let Some(c) = chars.next() {
                if c == '$' {
                    let check = chars.peek().map(|&d| d.is_alphabetic());
                    if check == Some(true) {
                        buf.push_str(r"\$");
                    } else {
                        buf.push('$');
                    }
                } else {
                    buf.push(c)
                }
            }
            buf
        })
        .collect();

    json!({
        "prefix": name,
        "body": lines,
    })
}

fn get_all_files(path: impl AsRef<path::Path>) -> io::Result<Vec<(String, File)>> {
    let mut files = vec![];
    let entry = fs::read_dir(path)?;
    file_dfs(entry, &mut files)?;
    Ok(files)
}

fn file_dfs(entry: fs::ReadDir, files: &mut Vec<(String, File)>) -> io::Result<()> {
    for path in entry {
        let path = path?;
        let pname = path.path();
        if path.file_type()?.is_dir() {
            file_dfs(
                fs::read_dir(pname.to_str().ok_or_else(invalid_encoding)?)?,
                files,
            )?;
        } else {
            let fname = pname.to_str().ok_or_else(invalid_encoding)?.to_owned();
            let file = File::open(fname.as_str())?;
            files.push((fname, file));
        }
    }
    Ok(())
}

fn invalid_encoding() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "Invalid encoding of file path")
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{Map, Result};

    #[test]
    pub fn generate_snippet_test() -> Result<()> {
        let code = r#"fn main() {
    // FastIO
    use {fastio::*, std::io::*};
    let input_str: &str = get_input();
    let mut scan: Scanner<_> = Scanner::tokenize(input_str);
    let stdout = stdout();
    let out = &mut BufWriter::new(stdout.lock());

    // FastIO Macros
    macro_rules! next {
        () => { scan.next() };
        (str) => { scan.next_str() };
        ($($t:ty) +) => { ($(scan.next::<$t>()),+) };
    }
    macro_rules! out { ($($arg:tt)*) => { write!(out, $($arg)*).ok(); }; }
    macro_rules! outln { ($($arg:tt)*) => { writeln!(out, $($arg)*).ok(); }; }

    // Main
}

mod fastio {
    extern "C" {
        fn mmap(addr: usize, len: usize, p: i32, f: i32, fd: i32, o: i64) -> *mut u8;
        fn fstat(fd: i32, st
            type Snippet = Map<String, Content>;

    pub fn get_input() -> &'static str {
        let mut stat = [0; 20];
        unsafe { fstat(0, (&mut stat).as_mut_ptr()) };
        let buffer = unsafe { mmap(0, stat[6], 1, 2, 0, 0) };
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(buffer, stat[6])) }
    }

    pub struct Scanner<'a, I: Iterator<Item = &'a str>> {
        it: I,
    }

    impl<'a> Scanner<'a, std::str::SplitAsciiWhitespace<'a>> {
        pub fn tokenize(s: &'a str) -> Self {
            Self {
                it: s.split_ascii_whitespace(),
            }
        }
    }

    impl<'a> Scanner<'a, std::str::Lines<'a>> {
        pub fn lines(s: &'a str) -> Self {
            Self { it: s.lines() }
        }
    }

    impl<'a, I: Iterator<Item = &'a str>> Scanner<'a, I> {
        #[inline(always)]
        pub fn next<T: std::str::FromStr>(&mut self) -> T {
            self.it.next().unwrap().parse().ok().unwrap()
        }
        #[inline(always)]
        pub fn next_str(&mut self) -> &'a str {
            self.it.next().unwrap()
        }
        #[inline(always)]
        pub fn next_option<T: std::str::FromStr>(&mut self) -> Option<T> {
            self.it.next().and_then(|s| s.parse().ok())
        }
        #[inline(always)]
        pub fn next_str_option(&mut self) -> Option<&'a str> {
            self.it.next()
        }
    }
}
"#;

        let name = "psbase";
        let base = generate_snippet(name, code);

        let mut snippet = Map::new();
        snippet.insert(name.to_string(), base);

        let x = serde_json::to_string(&snippet)?;
        println!("{}", x);
        assert_eq!(
            x,
            r##"{"psbase":{"body":["fn main() {","    // FastIO","    use {fastio::*, std::io::*};","    let input_str: &str = get_input();","    let mut scan: Scanner<_> = Scanner::tokenize(input_str);","    let stdout = stdout();","    let out = &mut BufWriter::new(stdout.lock());","","    // FastIO Macros","    macro_rules! next {","        () => { scan.next() };","        (str) => { scan.next_str() };","        ($(\\$t:ty) +) => { ($(scan.next::<\\$t>()),+) };","    }","    macro_rules! out { ($(\\$arg:tt)*) => { write!(out, $(\\$arg)*).ok(); }; }","    macro_rules! outln { ($(\\$arg:tt)*) => { writeln!(out, $(\\$arg)*).ok(); }; }","","    // Main","}","","mod fastio {","    extern \"C\" {","        fn mmap(addr: usize, len: usize, p: i32, f: i32, fd: i32, o: i64) -> *mut u8;","        fn fstat(fd: i32, st","            type Snippet = Map<String, Content>;","","    pub fn get_input() -> &'static str {","        let mut stat = [0; 20];","        unsafe { fstat(0, (&mut stat).as_mut_ptr()) };","        let buffer = unsafe { mmap(0, stat[6], 1, 2, 0, 0) };","        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(buffer, stat[6])) }","    }","","    pub struct Scanner<'a, I: Iterator<Item = &'a str>> {","        it: I,","    }","","    impl<'a> Scanner<'a, std::str::SplitAsciiWhitespace<'a>> {","        pub fn tokenize(s: &'a str) -> Self {","            Self {","                it: s.split_ascii_whitespace(),","            }","        }","    }","","    impl<'a> Scanner<'a, std::str::Lines<'a>> {","        pub fn lines(s: &'a str) -> Self {","            Self { it: s.lines() }","        }","    }","","    impl<'a, I: Iterator<Item = &'a str>> Scanner<'a, I> {","        #[inline(always)]","        pub fn next<T: std::str::FromStr>(&mut self) -> T {","            self.it.next().unwrap().parse().ok().unwrap()","        }","        #[inline(always)]","        pub fn next_str(&mut self) -> &'a str {","            self.it.next().unwrap()","        }","        #[inline(always)]","        pub fn next_option<T: std::str::FromStr>(&mut self) -> Option<T> {","            self.it.next().and_then(|s| s.parse().ok())","        }","        #[inline(always)]","        pub fn next_str_option(&mut self) -> Option<&'a str> {","            self.it.next()","        }","    }","}"],"prefix":"psbase"}}"##
        );

        Ok(())
    }
}
