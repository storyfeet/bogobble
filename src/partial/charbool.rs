use super::PosTree;
use crate::traits::*;

pub struct S(pub &'static str);

impl<'a> Parser<'a> for S {
    type Out = &'static str;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let mut i2 = it.clone();
        for c in self.0.chars() {
            match i2.next() {
                None => return Ok((i2, self.0, None)),
                Some(nc) if nc == c => {}
                _ => return Err(i2.err_s(self.0)),
            }
        }
        Ok((i2, self.0, None))
    }
}

pub trait PartCharBool: CharBool + Sized {
    fn p_until<'a, P: Parser<'a, Out = PosTree<I>>, I: Clone>(
        self,
        p: P,
        i: I,
    ) -> PCUntil<Self, P, I> {
        PCUntil { c: self, p, i }
    }
}

impl<C: CharBool> PartCharBool for C {}

pub struct PCUntil<C, P, I> {
    c: C,
    p: P,
    i: I,
}

impl<'a, C: CharBool, P: Parser<'a, Out = PosTree<I>>, I: Clone> Parser<'a> for PCUntil<C, P, I> {
    type Out = (PosTree<I>, P::Out);
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let mut i2 = it.clone();
        loop {
            let p_err = match self.p.parse(&i2) {
                Ok((i3, r2, e_op)) => {
                    let fin = i3.index();
                    return Ok((
                        i3,
                        (PosTree::new(it.index(), fin, self.i.clone()), r2),
                        e_op,
                    ));
                }
                Err(e) => e,
            };
            match i2.next() {
                Some(c) if self.c.char_bool(c) => {}
                Some(_) => return Err(i2.err(self.c.expected())),
                None => return Err(p_err),
            }
        }
    }
}
