use crate::err::*;
use crate::iter::*;

pub type ParseRes<'a, V> = Result<(PIter<'a>, V, Option<PErr<'a>>), PErr<'a>>;

pub trait ResTrait<'a> {
    fn map_str(self, start: &PIter<'a>) -> ParseRes<'a, &'a str>;
    fn map_string(self, start: &PIter<'a>) -> ParseRes<'a, String>;
}

impl<'a, V> ResTrait<'a> for ParseRes<'a, V> {
    fn map_str(self, start: &PIter<'a>) -> ParseRes<'a, &'a str> {
        self.map(|(i2, _, e)| {
            let s = start.str_to(i2.index());
            (i2, s, e)
        })
    }
    fn map_string(self, start: &PIter<'a>) -> ParseRes<'a, String> {
        self.map(|(i2, _, e)| {
            let s = start.str_to(i2.index());
            (i2, s.to_string(), e)
        })
    }
}

pub trait Parser<'a> {
    type Out;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out>;

    fn parse_s(&self, s: &'a str) -> Result<Self::Out, PErr<'a>> {
        self.parse(&PIter::new(s)).map(|(_, v, _)| v)
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
