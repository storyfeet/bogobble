use crate::err::*;
use crate::iter::*;
use crate::parser::*;

//use crate::reader::*;

pub trait CharBool: Sized {
    fn char_bool(&self, c: char) -> bool;
    fn expected(&self) -> Expected {
        Expected::Str(std::any::type_name::<Self>())
    }
    fn one(self) -> OneChar<Self> {
        OneChar { cb: self }
    }
    fn star(self) -> CharStar<Self> {
        CharStar { cb: self }
    }
    /// min_n not min to avoid ambiguity with std::cmp::Ord
    fn min_n(self, min: usize) -> CharMin<Self> {
        CharMin { cb: self, min }
    }

    fn plus(self) -> CharPlus<Self> {
        CharPlus { cb: self }
    }

    fn istar(self) -> ICharStar<Self> {
        ICharStar { cb: self }
    }
    fn iplus(self) -> ICharPlus<Self> {
        ICharPlus { cb: self }
    }

    fn iexact(self, n: usize) -> ICharExact<Self> {
        ICharExact { cb: self, n }
    }

    ///```rust
    /// use bogobble::*;
    /// assert_eq!(
    ///     Any.except("_").min_n(4).parse_s("asedf_wes"),
    ///     Ok("asedf")
    ///     );
    ///```
    fn except<E: CharBool>(self, e: E) -> CharsExcept<Self, E> {
        CharsExcept { a: self, e }
    }

    fn exact(self, n: usize) -> CharExact<Self> {
        CharExact { cb: self, n }
    }

    fn until<'a, P: Parser<'a>>(self, end: P) -> CharUntil<Self, P> {
        CharUntil { a: self, end }
    }
}

pub struct CharNot<C: CharBool> {
    c: C,
}

pub fn not<C: CharBool>(c: C) -> CharNot<C> {
    CharNot { c }
}

impl<C: CharBool> CharBool for CharNot<C> {
    fn char_bool(&self, c: char) -> bool {
        !self.c.char_bool(c)
    }
    fn expected(&self) -> Expected {
        Expected::Not(Box::new(self.c.expected()))
    }
}

pub fn is_alpha(c: char) -> bool {
    (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
}
char_bool!(Alpha, is_alpha);

pub fn is_num(c: char) -> bool {
    c >= '0' && c <= '9'
}
char_bool!(NumDigit, is_num);

char_bool!(Any, |_| true);

pub fn is_hex(c: char) -> bool {
    is_num(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}
char_bool!(HexDigit, is_hex);

char_bool!(WS, "\t ");
char_bool!(WSL, " \t\n\r");

impl CharBool for char {
    fn char_bool(&self, c: char) -> bool {
        *self == c
    }
    fn expected(&self) -> Expected {
        Expected::Char(*self)
    }
}

impl CharBool for &'static str {
    fn char_bool(&self, c: char) -> bool {
        self.contains(c)
    }
    fn expected(&self) -> Expected {
        Expected::CharIn(self)
    }
}

impl<F: Fn(char) -> bool> CharBool for F {
    fn char_bool(&self, c: char) -> bool {
        (self)(c)
    }
}

impl<A: CharBool, B: CharBool> CharBool for (A, B) {
    fn char_bool(&self, c: char) -> bool {
        self.0.char_bool(c) || self.1.char_bool(c)
    }
    fn expected(&self) -> Expected {
        Expected::OneOf(vec![self.0.expected(), self.1.expected()])
    }
}

impl<A: CharBool, B: CharBool, C: CharBool> CharBool for (A, B, C) {
    fn char_bool(&self, c: char) -> bool {
        self.0.char_bool(c) || self.1.char_bool(c) || self.2.char_bool(c)
    }
    fn expected(&self) -> Expected {
        Expected::OneOf(vec![
            self.0.expected(),
            self.1.expected(),
            self.2.expected(),
        ])
    }
}

impl<A, B, C, D> CharBool for (A, B, C, D)
where
    A: CharBool,
    B: CharBool,
    C: CharBool,
    D: CharBool,
{
    fn char_bool(&self, c: char) -> bool {
        self.0.char_bool(c) || self.1.char_bool(c) || self.2.char_bool(c) || self.3.char_bool(c)
    }
    fn expected(&self) -> Expected {
        Expected::OneOf(vec![
            self.0.expected(),
            self.1.expected(),
            self.2.expected(),
            self.3.expected(),
        ])
    }
}

impl<A, B, C, D, E> CharBool for (A, B, C, D, E)
where
    A: CharBool,
    B: CharBool,
    C: CharBool,
    D: CharBool,
    E: CharBool,
{
    fn char_bool(&self, c: char) -> bool {
        self.0.char_bool(c)
            || self.1.char_bool(c)
            || self.2.char_bool(c)
            || self.3.char_bool(c)
            || self.4.char_bool(c)
    }
    fn expected(&self) -> Expected {
        Expected::OneOf(vec![
            self.0.expected(),
            self.1.expected(),
            self.2.expected(),
            self.3.expected(),
            self.4.expected(),
        ])
    }
}

impl<A, B, C, D, E, F> CharBool for (A, B, C, D, E, F)
where
    A: CharBool,
    B: CharBool,
    C: CharBool,
    D: CharBool,
    E: CharBool,
    F: CharBool,
{
    fn char_bool(&self, c: char) -> bool {
        self.0.char_bool(c)
            || self.1.char_bool(c)
            || self.2.char_bool(c)
            || self.3.char_bool(c)
            || self.4.char_bool(c)
            || self.5.char_bool(c)
    }
    fn expected(&self) -> Expected {
        Expected::OneOf(vec![
            self.0.expected(),
            self.1.expected(),
            self.2.expected(),
            self.3.expected(),
            self.4.expected(),
            self.5.expected(),
        ])
    }
}
pub struct CharsExcept<A: CharBool, E: CharBool> {
    a: A,
    e: E,
}

impl<A: CharBool, E: CharBool> CharBool for CharsExcept<A, E> {
    fn char_bool(&self, c: char) -> bool {
        self.a.char_bool(c) && !self.e.char_bool(c)
    }
    fn expected(&self) -> Expected {
        self.a
            .expected()
            .join(Expected::Not(Box::new(self.e.expected())))
    }
}

pub fn do_one_char<'a, CB: CharBool>(i: &PIter<'a>, cb: &CB) -> ParseRes<'a, char> {
    let mut i2 = i.clone();
    let ic = i2.next().ok_or(i2.err(cb.expected()))?;
    if cb.char_bool(ic) {
        Ok((i2, ic, None))
    } else {
        i.err_r(cb.expected())
    }
}

pub struct OneChar<CB: CharBool> {
    cb: CB,
}

impl<'a, CB: CharBool> Parser<'a> for OneChar<CB> {
    type Out = char;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, char> {
        do_one_char(it, &self.cb)
    }
}

pub fn one_char<C: CharBool>(cb: C) -> OneChar<C> {
    OneChar { cb }
}

pub fn do_chars<'a, CB: CharBool>(
    i: &PIter<'a>,
    cb: &CB,
    min: usize,
    exact: bool,
) -> ParseRes<'a, ()> {
    let mut it = i.clone();
    let mut done = 0;
    loop {
        let it2 = it.clone();
        match it.next() {
            Some(c) if cb.char_bool(c) => {
                println!("do_chars CHAR = {}", c);
                done += 1;
                if done == min && exact {
                    return Ok((it, (), None));
                }
            }
            Some(_) | None => {
                if done >= min {
                    let eo = Some(it2.err(cb.expected()));
                    return Ok((it2, (), eo));
                } else {
                    return it2.err_r(cb.expected());
                }
            }
        }
    }
}

pub struct ICharStar<C: CharBool> {
    cb: C,
}
impl<'a, CB: CharBool> Parser<'a> for ICharStar<CB> {
    type Out = ();
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_chars(i, &self.cb, 0, false)
    }
}

pub struct ICharPlus<C: CharBool> {
    cb: C,
}
impl<'a, CB: CharBool> Parser<'a> for ICharPlus<CB> {
    type Out = ();
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_chars(i, &self.cb, 1, false)
    }
}

pub struct ICharExact<C: CharBool> {
    cb: C,
    n: usize,
}
impl<'a, CB: CharBool> Parser<'a> for ICharExact<CB> {
    type Out = ();
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_chars(i, &self.cb, self.n, true)
    }
}
#[derive(Clone)]
pub struct CharStar<C: CharBool> {
    cb: C,
}

impl<'a, CB: CharBool> Parser<'a> for CharStar<CB> {
    type Out = &'a str;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_chars(i, &self.cb, 0, false).map_str(i)
    }
}

#[derive(Clone)]
pub struct CharPlus<C: CharBool> {
    cb: C,
}

impl<'a, CB: CharBool> Parser<'a> for CharPlus<CB> {
    type Out = &'a str;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, &'a str> {
        do_chars(i, &self.cb, 1, false).map_str(i)
    }
}

#[derive(Clone)]
pub struct CharExact<CB: CharBool> {
    cb: CB,
    n: usize,
}

impl<'a, A: CharBool> Parser<'a> for CharExact<A> {
    type Out = &'a str;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_chars(i, &self.cb, self.n, true).map_str(i)
    }
}

#[derive(Clone)]
pub struct CharMin<A: CharBool> {
    cb: A,
    min: usize,
}

impl<'a, A: CharBool> Parser<'a> for CharMin<A> {
    type Out = &'a str;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_chars(i, &self.cb, self.min, false).map_str(i)
    }
}

#[derive(Clone)]
pub struct CharUntil<A: CharBool, E> {
    a: A,
    end: E,
}

impl<'a, A: CharBool, E: Parser<'a>> Parser<'a> for CharUntil<A, E> {
    type Out = (&'a str, E::Out);
    fn parse(&self, i_start: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let mut it = i_start.clone();
        let mut a_stop = it.index();
        loop {
            match self.end.parse(&it) {
                Ok((i, v, e)) => return Ok((i, (i_start.str_to(a_stop), v), e)),
                Err(e1) => match it.next() {
                    Some(c) if self.a.char_bool(c) => a_stop = it.index(),
                    _ => return Err(e1.join(it.err(self.a.expected()))),
                },
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    pub fn test_alpha_works_as_struct() {
        assert_eq!(Alpha.char_bool('a'), true)
    }
}
