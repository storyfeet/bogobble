use crate::convert::*;
use crate::err::*;
use crate::iter::*;
use crate::select::*;

pub type ParseRes<'a, V> = Result<(PIter<'a>, V, Option<PErr<'a>>), PErr<'a>>;

pub trait ResTrait<'a>: Sized {
    type Val;
    fn map_v<F: Fn(Self::Val) -> R, R>(self, f: F) -> ParseRes<'a, R>;
    fn map_str(self, start: &PIter<'a>) -> ParseRes<'a, &'a str>;
    fn map_string(self, start: &PIter<'a>) -> ParseRes<'a, String> {
        self.map_str(start).map(|(i, v, e)| (i, v.to_string(), e))
    }
    fn join_err(self, e: PErr<'a>) -> Self;
    fn join_err_op(self, e: Option<PErr<'a>>) -> Self {
        match e {
            Some(e) => self.join_err(e),
            None => self,
        }
    }
}

impl<'a, V> ResTrait<'a> for ParseRes<'a, V> {
    type Val = V;
    fn map_v<F: Fn(Self::Val) -> R, R>(self, f: F) -> ParseRes<'a, R> {
        self.map(|(i, v, e)| (i, f(v), e))
    }
    fn map_str(self, start: &PIter<'a>) -> ParseRes<'a, &'a str> {
        self.map(|(i2, _, e)| {
            let s = start.str_to(i2.index());
            (i2, s, e)
        })
    }
    fn join_err(self, e2: PErr<'a>) -> Self {
        self.map_err(|e| e.join(e2))
    }
}

pub trait OParser<Out>: for<'text> Parser<'text, Out = Out> {}
impl<Out, P: ?Sized> OParser<Out> for P where P: for<'text> Parser<'text, Out = Out> {}

pub trait Parser<'a>: Sized {
    type Out;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out>;

    fn parse_s(&self, s: &'a str) -> Result<Self::Out, PErr<'a>> {
        self.parse(&PIter::new(s)).map(|(_, v, _)| v)
    }

    fn or<B: Parser<'a, Out = Self::Out>>(self, b: B) -> Or<Self, B> {
        or(self, b)
    }

    fn map<B, F: Fn(Self::Out) -> B>(self, f: F) -> Map<Self, F> {
        map(self, f)
    }

    fn try_map<B, F: Fn(Self::Out) -> Result<B, Expected>>(self, f: F) -> TryMap<Self, F> {
        try_map(self, f)
    }
    fn ig(self) -> Ig<Self> {
        Ig { a: self }
    }
}

impl<'a, F, V> Parser<'a> for F
where
    F: Fn(&PIter<'a>) -> ParseRes<'a, V>,
{
    type Out = V;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, V> {
        self(it)
    }
}

impl<'a> Parser<'a> for &'static str {
    type Out = &'static str;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let mut it = i.clone();
        for c in self.chars() {
            match it.next() {
                Some(ic) if ic == c => {}
                _ => return Err(i.err_s(self)),
            }
        }
        Ok((it, self, None))
    }
}

impl<'a> Parser<'a> for char {
    type Out = char;

    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let mut it = i.clone();
        match it.next() {
            Some(ic) if ic == *self => Ok((it, ic, None)),
            _ => return Err(i.err(Expected::Char(*self))),
        }
    }
}
