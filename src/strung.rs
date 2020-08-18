use crate::iter::*;
use crate::parser::*;

/// ```rust
/// use bogobble::*;
/// let p = str_range(("abc".plus(),"def".plus()));
/// assert_eq!(p.parse_s("aacfc_gp"),Ok("aacf"));
/// ```
pub fn str_range<'a, P: Parser<'a>>(p: P) -> StrRange<P> {
    StrRange { p }
}

pub struct StrRange<P> {
    p: P,
}

impl<'a, P: Parser<'a>> Parser<'a> for StrRange<P> {
    type Out = &'a str;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        self.p.parse(i).map_str(i)
    }
}

pub fn string<'a, P: Parser<'a>>(p: P) -> StringRange<P> {
    StringRange { p }
}

pub struct StringRange<P> {
    p: P,
}

impl<'a, P: Parser<'a>> Parser<'a> for StringRange<P> {
    type Out = String;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        self.p.parse(i).map_string(i)
    }
}
