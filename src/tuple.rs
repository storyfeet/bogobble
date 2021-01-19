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
        ((self.0.br(), self.1.br()), self.2.br())
            .parse(it)
            .map_v(|((a, b), c)| (a, b, c))
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
        ((self.0.br(), self.1.br()), (self.2.br(), self.3.br()))
            .parse(it)
            .map_v(|((a, b), (c, d))| (a, b, c, d))
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
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        (
            (self.0.br(), self.1.br(), self.2.br()),
            (self.3.br(), self.4.br()),
        )
            .parse(it)
            .map_v(|((a, b, c), (d, e))| (a, b, c, d, e))
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
        (
            (self.0.br(), self.1.br(), self.2.br()),
            (self.3.br(), self.4.br(), self.5.br()),
        )
            .parse(it)
            .map_v(|((a, b, c), (d, e, f))| (a, b, c, d, e, f))
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
    /// ```rust
    /// use bogobble::*;
    /// let p = ("abc".plus(),"def".plus()).first().parse_s("aced").unwrap();
    /// assert_eq!(p,"ac");
    /// let p = ("abc".plus(),"def".plus()).last().parse_s("aced").unwrap();
    /// assert_eq!(p,"ed");
    ///
    /// ```
    fn get_first(self) -> Self::Item;
}

pub fn first_res<'a, A: Parser<'a>>(a: A) -> FirstRes<A> {
    FirstRes { a }
}

pub struct FirstRes<A> {
    a: A,
}

impl<A, B> Firster for (A, B) {
    type Item = A;
    fn get_first(self) -> A {
        self.0
    }
}
impl<A, B, C> Firster for (A, B, C) {
    type Item = A;
    fn get_first(self) -> A {
        self.0
    }
}
impl<A, B, C, D> Firster for (A, B, C, D) {
    type Item = A;
    fn get_first(self) -> A {
        self.0
    }
}
impl<A, B, C, D, E> Firster for (A, B, C, D, E) {
    type Item = A;
    fn get_first(self) -> A {
        self.0
    }
}

impl<A, B, C, D, E, F> Firster for (A, B, C, D, E, F) {
    type Item = A;
    fn get_first(self) -> A {
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
        self.a.parse(i).map_v(|v| v.get_first())
    }
}

//LAST
pub trait PLaster: Sized {
    fn last(self) -> LastRes<Self>;
}

impl<'a, A> PLaster for A
where
    A: Parser<'a>,
    A::Out: Laster,
{
    fn last(self) -> LastRes<Self> {
        LastRes { a: self }
    }
}

pub trait Laster {
    type Item;
    fn get_last(self) -> Self::Item;
}

pub fn last_res<'a, A: Parser<'a>>(a: A) -> LastRes<A> {
    LastRes { a }
}

pub struct LastRes<A> {
    a: A,
}

impl<A, L> Laster for (A, L) {
    type Item = L;
    fn get_last(self) -> L {
        self.1
    }
}
impl<A, B, L> Laster for (A, B, L) {
    type Item = L;
    fn get_last(self) -> L {
        self.2
    }
}
impl<A, B, C, L> Laster for (A, B, C, L) {
    type Item = L;
    fn get_last(self) -> L {
        self.3
    }
}
impl<A, B, C, D, L> Laster for (A, B, C, D, L) {
    type Item = L;
    fn get_last(self) -> L {
        self.4
    }
}

impl<A, B, C, D, E, L> Laster for (A, B, C, D, E, L) {
    type Item = L;
    fn get_last(self) -> L {
        self.5
    }
}

impl<'a, A, R, O> Parser<'a> for LastRes<A>
where
    A: Parser<'a, Out = R>,
    R: Laster<Item = O>,
{
    type Out = O;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        self.a.parse(i).map_v(|v| v.get_last())
    }
}
