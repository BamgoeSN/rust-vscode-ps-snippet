#![no_main]

#[no_mangle]
fn main() -> i32 {
    // FastIO
    use fastio::*;
    let input_str = get_input();
    let mut sc = Tokenizer::new(input_str, |s| s.split_ascii_whitespace());
    use std::io::{stdout, BufWriter, Write};
    let stdout = stdout();
    let wr = &mut BufWriter::new(stdout.lock());

    // FastIO Macros
    macro_rules! out { ($($arg:tt)*) => { write!(wr, $($arg)*).ok(); }; }
    macro_rules! outln { ($($arg:tt)*) => { writeln!(wr, $($arg)*).ok(); }; }

    // Main

    wr.flush().unwrap();
    0
}

#[allow(unused)]
mod fastio {
    use std::{fmt, io, num::*, slice::*, str::*};

    #[link(name = "c")]
    extern "C" {}

    pub fn get_input() -> &'static str {
        use io::Read;
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf);
        Box::leak(buf.into_boxed_str())
    }

    pub enum InputError<'t> {
        InputExhaust,
        ParseError(&'t str),
    }
    use InputError::*;

    impl<'t> fmt::Debug for InputError<'t> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                InputExhaust => f.debug_struct("InputExhaust").finish(),
                ParseError(s) => f.debug_struct("ParseError").field("str", s).finish(),
            }
        }
    }

    pub trait Atom: Sized {
        fn parse_from(s: &str) -> Result<Self, InputError>;
    }

    pub trait IterParse: Sized {
        fn parse_from<'s, 't: 's, It>(it: &'s mut It) -> Result<Self, InputError<'t>>
        where
            It: Iterator<Item = &'t str>;
    }

    macro_rules! impl_trait_for_fromstr {
        ($($t:ty) *) => { $(
            impl Atom for $t { fn parse_from(s: &str) -> Result<Self, InputError> { s.parse().map_err(|_| ParseError(s)) } }
            impl IterParse for $t {
                fn parse_from<'s, 't: 's, It>(it: &'s mut It) -> Result<Self, InputError<'t>> where It: Iterator<Item = &'t str> {
                    it.next().map_or( Err(InputExhaust), <Self as Atom>::parse_from )
                }
            }
        )* };
    }

    impl_trait_for_fromstr!(bool char String);
    impl_trait_for_fromstr!(f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
    impl_trait_for_fromstr!(NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize);
    impl_trait_for_fromstr!(NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize);

    macro_rules! impl_iterparse_for_tuple {
        ($($t:ident) *) => {
            impl<$($t),*> IterParse for ($($t),*) where $($t: IterParse),* {
                fn parse_from<'s, 't: 's, It>(it: &'s mut It) -> Result<Self, InputError<'t>> where It: Iterator<Item = &'t str> {
                    Ok(( $($t::parse_from(it)?),* ))
                }
            }
        };
    }

    impl_iterparse_for_tuple!();
    impl_iterparse_for_tuple!(A B);
    impl_iterparse_for_tuple!(A B C);
    impl_iterparse_for_tuple!(A B C D);
    impl_iterparse_for_tuple!(A B C D E);
    impl_iterparse_for_tuple!(A B C D E F);
    impl_iterparse_for_tuple!(A B C D E F G);
    impl_iterparse_for_tuple!(A B C D E F G H);
    impl_iterparse_for_tuple!(A B C D E F G H I);
    impl_iterparse_for_tuple!(A B C D E F G H I J);
    impl_iterparse_for_tuple!(A B C D E F G H I J K);
    impl_iterparse_for_tuple!(A B C D E F G H I J K L);
    impl_iterparse_for_tuple!(A B C D E F G H I J K L M);

    pub struct Tokenizer<It> {
        it: It,
    }

    impl<'arg, 'str: 'arg, It> Tokenizer<It> {
        pub fn new(s: &'str str, split: impl FnOnce(&'arg str) -> It) -> Self {
            Self { it: split(s) }
        }
    }

    impl<'t, It> Tokenizer<It>
    where
        It: Iterator<Item = &'t str>,
    {
        pub fn next<T: IterParse>(&mut self) -> T {
            T::parse_from(&mut self.it).unwrap()
        }
        pub fn next_str(&mut self) -> &'t str {
            self.it.next().unwrap()
        }
        pub fn next_ok<T: IterParse>(&mut self) -> Result<T, InputError<'t>> {
            T::parse_from(&mut self.it)
        }
        pub fn next_str_ok(&mut self) -> Option<&'t str> {
            self.it.next()
        }
        pub fn next_iter<'s, T: IterParse>(&'s mut self) -> impl Iterator<Item = T> + '_
        where
            't: 's,
        {
            std::iter::repeat_with(move || self.next_ok())
                .take_while(|x| x.is_ok())
                .flatten()
        }
    }
}
