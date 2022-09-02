use std::{
    fs::{self, File},
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Serialize, Deserialize)]
struct Content {
    prefix: String,
    body: Vec<String>,
}

fn main() {
    let mut snippet = Map::new();

    let files = get_all_files("./snippets");

    for path in files.iter() {
        let name = path.rsplit_once("/").unwrap().1.strip_suffix(".txt");
        match name {
            None => continue,
            Some(name) => {
                let mut fp = File::open(path).unwrap();
                let mut code = String::new();
                fp.read_to_string(&mut code).unwrap();
                snippet.insert(name.to_string(), generate_snippet(name, &code));
            }
        }
    }

    let mut output = File::create("output.txt").unwrap();
    write!(output, "{}", serde_json::to_string(&snippet).unwrap()).unwrap();
}

fn generate_snippet(name: &str, code: &str) -> Value {
    let lines: Vec<String> = code
        .lines()
        .map(|s| {
            let mut chars = s.chars().peekable();
            let mut buf = String::new();
            while let Some(c) = chars.next() {
                match c {
                    '$' => chars
                        .peek()
                        .and_then(|&d| {
                            if d.is_alphanumeric() {
                                buf.push_str(r"\$");
                                Some(())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| buf.push('$')),
                    '\t' => buf.extend(c.escape_default()),
                    _ => buf.push(c),
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

fn get_all_files(path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    file_dfs(fs::read_dir(path).unwrap(), &mut files);
    files
}

fn file_dfs(entry: fs::ReadDir, files: &mut Vec<String>) {
    for path in entry.map(|p| p.unwrap()) {
        if path.file_type().unwrap().is_dir() {
            let s = path.path().to_str().unwrap().to_owned();
            file_dfs(fs::read_dir(&s).unwrap(), files);
        } else {
            files.push(path.path().to_str().unwrap().to_owned());
        }
    }
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
