use crate::err::*;
use crate::iter::*;
use crate::parser::*;

pub fn map<'a, A: Parser<'a>, F: Fn(A::Out) -> V, V>(a: A, f: F) -> Map<A, F> {
    Map { a, f }
}

#[derive(Clone)]
pub struct Map<A, F> {
    a: A,
    f: F,
}

impl<'a, A: Parser<'a>, B, F: Fn(A::Out) -> B> Parser<'a> for Map<A, F> {
    type Out = B;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, B> {
        let (ri, v, ex) = self.a.parse(i)?;
        Ok((ri, (self.f)(v), ex))
    }
}

pub fn try_map<'a, A: Parser<'a>, F: Fn(A::Out) -> Result<V, Expected>, V>(
    a: A,
    f: F,
) -> TryMap<A, F> {
    TryMap { a, f }
}

#[derive(Clone)]
pub struct TryMap<A, F> {
    a: A,
    f: F,
}

impl<'a, A: Parser<'a>, B, F: Fn(A::Out) -> Result<B, Expected>> Parser<'a> for TryMap<A, F> {
    type Out = B;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, B> {
        let (ri, v, ct) = self.a.parse(i)?;
        match (self.f)(v) {
            Ok(v2) => Ok((ri, v2, ct)),
            Err(e) => ri.err_r(e),
        }
    }
}
