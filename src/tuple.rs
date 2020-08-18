use crate::iter::*;
use crate::parser::*;

impl<'a, A, B> Parser<'a> for (A, B)
where
    A: Parser<'a>,
    B: Parser<'a>,
{
    type Out = (A::Out, B::Out);
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (it2, av, c1) = self.0.parse(it)?;
        let (it3, bv, c2) = self.1.parse(&it2).join_err_op(c1)?;
        Ok((it3, (av, bv), c2))
    }
}

impl<'a, A, B, C> Parser<'a> for (A, B, C)
where
    A: Parser<'a>,
    B: Parser<'a>,
    C: Parser<'a>,
{
    type Out = (A::Out, B::Out, C::Out);
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (it2, av, c1) = self.0.parse(it)?;
        let (it3, bv, c2) = self.1.parse(&it2).join_err_op(c1)?;
        let (it4, cv, c3) = self.2.parse(&it3).join_err_op(c2)?;
        Ok((it4, (av, bv, cv), c3))
    }
}

impl<'a, A, B, C, D> Parser<'a> for (A, B, C, D)
where
    A: Parser<'a>,
    B: Parser<'a>,
    C: Parser<'a>,
    D: Parser<'a>,
{
    type Out = (A::Out, B::Out, C::Out, D::Out);
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (it2, av, c1) = self.0.parse(it)?;
        let (it3, bv, c2) = self.1.parse(&it2).join_err_op(c1)?;
        let (it4, cv, c3) = self.2.parse(&it3).join_err_op(c2)?;
        let (it5, dv, c4) = self.3.parse(&it4).join_err_op(c3)?;
        Ok((it5, (av, bv, cv, dv), c4))
    }
}
impl<'a, A, B, C, D, E> Parser<'a> for (A, B, C, D, E)
where
    A: Parser<'a>,
    B: Parser<'a>,
    C: Parser<'a>,
    D: Parser<'a>,
    E: Parser<'a>,
{
    type Out = (A::Out, B::Out, C::Out, D::Out, E::Out);
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (it, av, c) = self.0.parse(i)?;
        let (it, bv, c) = self.1.parse(&it).join_err_op(c)?;
        let (it, cv, c) = self.2.parse(&it).join_err_op(c)?;
        let (it, dv, c) = self.3.parse(&it).join_err_op(c)?;
        let (it, ev, c) = self.4.parse(&it).join_err_op(c)?;
        Ok((it, (av, bv, cv, dv, ev), c))
    }
}
impl<'a, A, B, C, D, E, F> Parser<'a> for (A, B, C, D, E, F)
where
    A: Parser<'a>,
    B: Parser<'a>,
    C: Parser<'a>,
    D: Parser<'a>,
    E: Parser<'a>,
    F: Parser<'a>,
{
    type Out = (A::Out, B::Out, C::Out, D::Out, E::Out, F::Out);
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (it2, av, c) = self.0.parse(it)?;
        let (it3, bv, c) = self.1.parse(&it2).join_err_op(c)?;
        let (it4, cv, c) = self.2.parse(&it3).join_err_op(c)?;
        let (it5, dv, c) = self.3.parse(&it4).join_err_op(c)?;
        let (it6, ev, c) = self.4.parse(&it5).join_err_op(c)?;
        let (it7, fv, c) = self.5.parse(&it6).join_err_op(c)?;
        Ok((it7, (av, bv, cv, dv, ev, fv), c))
    }
}

//Grabbing specific elements
pub trait PFirster: Sized {
    fn first(self) -> FirstRes<Self>;
}

impl<'a, A> PFirster for A
where
    A: Parser<'a>,
    A::Out: Firster,
{
    fn first(self) -> FirstRes<Self> {
        FirstRes { a: self }
    }
}

pub trait Firster {
    type Item;
    fn first(self) -> Self::Item;
}

pub fn first_res<'a, A: Parser<'a>>(a: A) -> FirstRes<A> {
    FirstRes { a }
}

pub struct FirstRes<A> {
    a: A,
}

impl<A, B> Firster for (A, B) {
    type Item = A;
    fn first(self) -> A {
        self.0
    }
}
impl<A, B, C> Firster for (A, B, C) {
    type Item = A;
    fn first(self) -> A {
        self.0
    }
}
impl<A, B, C, D> Firster for (A, B, C, D) {
    type Item = A;
    fn first(self) -> A {
        self.0
    }
}
impl<A, B, C, D, E> Firster for (A, B, C, D, E) {
    type Item = A;
    fn first(self) -> A {
        self.0
    }
}

impl<A, B, C, D, E, F> Firster for (A, B, C, D, E, F) {
    type Item = A;
    fn first(self) -> A {
        self.0
    }
}

impl<'a, A, R, O> Parser<'a> for FirstRes<A>
where
    A: Parser<'a, Out = R>,
    R: Firster<Item = O>,
{
    type Out = O;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        self.a.parse(i).map_v(|v| v.first())
    }
}
