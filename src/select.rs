use crate::iter::*;
use crate::parser::*;

pub struct Ig<A> {
    pub a: A,
}

impl<'a, A: Parser<'a>> Parser<'a> for Ig<A> {
    type Out = ();
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        self.a.parse(i).map_v(|_| ())
    }
}

pub struct First<A, B> {
    a: A,
    b: B,
}
impl<'a, A: Parser<'a>, B: Parser<'a>> Parser<'a> for First<A, B> {
    type Out = A::Out;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (it, v, c1) = self.a.parse(i)?;
        let (it, _, c2) = self.b.parse(&it).join_err_op(c1)?;
        Ok((it, v, c2))
    }
}

pub fn first<'a, A, B>(a: A, b: B) -> First<A, B>
where
    A: Parser<'a>,
    B: Parser<'a>,
{
    First { a, b }
}

pub struct Last<A, B> {
    a: A,
    b: B,
}
impl<'a, A: Parser<'a>, B: Parser<'a>> Parser<'a> for Last<A, B> {
    type Out = B::Out;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (it, _, c1) = self.a.parse(i)?;
        let (it, v, c2) = self.b.parse(&it).join_err_op(c1)?;
        Ok((it, v, c2))
    }
}

pub fn last<'a, A, B>(a: A, b: B) -> Last<A, B>
where
    A: Parser<'a>,
    B: Parser<'a>,
{
    Last { a, b }
}

pub struct Middle<A, B, C> {
    a: A,
    b: B,
    c: C,
}
impl<'a, A: Parser<'a>, B: Parser<'a>, C: Parser<'a>> Parser<'a> for Middle<A, B, C> {
    type Out = B::Out;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (it, _, c) = self.a.parse(i)?;
        let (it, v, c) = self.b.parse(&it).join_err_op(c)?;
        let (it, _, c) = self.c.parse(&it).join_err_op(c)?;
        Ok((it, v, c))
    }
}

pub fn middle<'a, A, B, C>(a: A, b: B, c: C) -> Middle<A, B, C>
where
    A: Parser<'a>,
    B: Parser<'a>,
    C: Parser<'a>,
{
    Middle { a, b, c }
}

pub fn or<'a, A, B, V>(a: A, b: B) -> Or<A, B>
where
    A: Parser<'a, Out = V>,
    B: Parser<'a, Out = V>,
{
    Or { a, b }
}

pub struct Or<A, B> {
    pub a: A,
    pub b: B,
}
impl<'a, A, B, V> Parser<'a> for Or<A, B>
where
    A: Parser<'a, Out = V>,
    B: Parser<'a, Out = V>,
{
    type Out = V;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, V> {
        match self.a.parse(i) {
            Ok((r, v, e)) => Ok((r, v, e)),
            Err(e) if e.is_break => Err(e),
            Err(e) => match self.b.parse(i) {
                Ok((r, v, ex)) => Ok((r, v, ex)),
                Err(e2) if e2.is_break => Err(e2),
                Err(e2) => Err(e.longer(e2)),
            },
        }
    }
}
