use crate::traits::*;
use crate::EOI;
pub mod charbool;
pub use traits::*;
pub mod mark_list;
pub mod p_repeat;
pub mod traits {
    pub use super::charbool::PartCharBool;
}
pub use p_repeat::*;

#[derive(Debug, Clone)]
pub struct PosTree<I> {
    start: Option<usize>,
    fin: Option<usize>,
    pub complete: bool,
    pub item: I,
    pub children: Vec<PosTree<I>>,
}

impl<I> PosTree<I> {
    pub fn new(start: Option<usize>, fin: Option<usize>, item: I) -> Self {
        PosTree {
            start,
            fin,
            item,
            complete: true,
            children: Vec::new(),
        }
    }

    fn merge(self, item: I, b: Self) -> Self {
        PosTree {
            start: self.start,
            fin: b.fin,
            complete: b.complete,
            item,
            children: vec![self, b],
        }
    }

    fn as_child(self, item: I) -> Self {
        PosTree {
            start: self.start,
            fin: self.fin,
            complete: self.complete,
            item,
            children: vec![self],
            ..self
        }
    }

    pub fn incomplete(mut self) -> Self {
        self.complete = false;
        self
    }

    pub fn push(mut self, b: Self) -> Self {
        self.fin = b.fin;
        self.children.push(b);
        self
    }

    ///Grab str from reference between points
    ///panics if str not long enough
    pub fn on_str<'a>(&self, s: &'a str) -> &'a str {
        match (self.start, self.fin) {
            (Some(a), Some(b)) => &s[a..b],
            (Some(a), _) => &s[a..],
            _ => "",
        }
    }

    pub fn str_len(&self, s: &str) -> usize {
        self.on_str(s).len()
    }
}

pub struct Merger<A, B, I> {
    a: A,
    b: B,
    i: I,
}

impl<'a, A, B, I> Parser<'a> for Merger<A, B, I>
where
    A: Parser<'a, Out = PosTree<I>>,
    B: Parser<'a, Out = PosTree<I>>,
    I: Clone,
{
    type Out = PosTree<I>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, PosTree<I>> {
        if it.eoi() {
            return Ok((
                it.clone(),
                PosTree::new(it.index(), it.index(), self.i.clone()),
                None,
            ));
        }
        let (i1, p1, _) = self.a.parse(it)?;

        match self.b.parse(&i1) {
            Ok((i2, p2, e2)) => Ok((i2, p1.merge(self.i.clone(), p2), e2)),
            Err(e2) => match i1.eoi() {
                true => Ok((i1, p1.as_child(self.i.clone()).incomplete(), None)),
                false => Err(e2),
            },
        }
    }
}

pub trait Mergable<I>: Sized {
    fn merge<'a, B: Parser<'a, Out = PosTree<I>>>(self, i: I, b: B) -> Merger<Self, B, I>;
}

impl<'b, A: Parser<'b, Out = PosTree<I>>, I: Clone> Mergable<I> for A {
    fn merge<'a, B: Parser<'a, Out = PosTree<I>>>(self, i: I, b: B) -> Merger<Self, B, I> {
        Merger { a: self, i, b }
    }
}

pub struct Pusher<A, B> {
    a: A,
    b: B,
}

impl<'a, A, B, I> Parser<'a> for Pusher<A, B>
where
    A: Parser<'a, Out = PosTree<I>>,
    B: Parser<'a, Out = PosTree<I>>,
    I: Clone,
{
    type Out = PosTree<I>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, PosTree<I>> {
        let (i1, p1, _) = self.a.parse(it)?;
        match self.b.parse(&i1) {
            Ok((i2, p2, e2)) => Ok((i2, p1.push(p2), e2)),
            Err(e2) => match EOI.parse(&i1) {
                Ok((i2, _, _)) => Ok((i2, p1.incomplete(), None)),
                Err(_) => Err(e2),
            },
        }
    }
}

pub trait Pushable<I>: Sized {
    fn push<'a, B: Parser<'a, Out = PosTree<I>>>(self, b: B) -> Pusher<Self, B>;
}

impl<'b, A: Parser<'b, Out = PosTree<I>>, I: Clone> Pushable<I> for A {
    fn push<'a, B: Parser<'a, Out = PosTree<I>>>(self, b: B) -> Pusher<Self, B> {
        Pusher { a: self, b }
    }
}

pub struct PosTreeParse<P, I> {
    p: P,
    item: I,
}

pub fn tpos<'a, P: Parser<'a>, I>(p: P, item: I) -> PosTreeParse<P, I> {
    PosTreeParse { p, item }
}

impl<'a, P: Parser<'a>, I: Clone> Parser<'a> for PosTreeParse<P, I> {
    type Out = PosTree<I>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, PosTree<I>> {
        let (i2, _, e) = self.p.parse(it)?;
        let fin = i2.index();
        let start = it.index();
        Ok((
            i2,
            PosTree {
                start,
                fin,
                item: self.item.clone(),
                children: Vec::new(),
                complete: true,
            },
            e,
        ))
    }
}

pub struct PMaybe<P, I> {
    p: P,
    i: I,
}

impl<'a, I: Clone, P: Parser<'a, Out = PosTree<I>>> Parser<'a> for PMaybe<P, I> {
    type Out = PosTree<I>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        match self.p.parse(it) {
            Ok(v) => Ok(v),
            Err(e) => Ok((
                it.clone(),
                PosTree {
                    start: it.index(),
                    fin: it.index(),
                    item: self.i.clone(),
                    complete: true,
                    children: Vec::new(),
                },
                Some(e),
            )),
        }
    }
}

pub fn pmaybe<'a, P: Parser<'a, Out = PosTree<I>>, I: Clone>(p: P, i: I) -> PMaybe<P, I> {
    PMaybe { p, i }
}

pub struct PosVecParse<P, I> {
    p: P,
    i: I,
}

pub fn vpos<'a, P: Parser<'a, Out = Vec<PosTree<I>>>, I: Clone>(p: P, i: I) -> PosVecParse<P, I> {
    PosVecParse { p, i }
}

impl<'a, P: Parser<'a, Out = Vec<PosTree<I>>>, I: Clone> Parser<'a> for PosVecParse<P, I> {
    type Out = PosTree<I>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        self.p
            .parse(it)
            .map(|(i2, vc, e)| {
                let complete = match vc.len() {
                    0 => false,
                    n => vc
                        .get(n - 1)
                        .map(|c| c.complete)
                        .unwrap_or_else(|| EOI.parse(&i2).is_ok()),
                };
                let res = PosTree {
                    start: it.index(),
                    fin: i2.index(),
                    item: self.i.clone(),
                    complete,
                    children: vc,
                };
                (i2, res, e)
            })
            .or_else(|e| {
                EOI.parse(it)
                    .map_v(|_| PosTree {
                        start: it.index(),
                        fin: it.index(),
                        item: self.i.clone(),
                        complete: false,
                        children: Vec::new(),
                    })
                    .map_err(|_| e)
            })
    }
}

#[macro_export]
macro_rules! p_list (
    (($it:expr)  $a:expr,$b:expr $(,$x:expr)* $(,)?) => ($a.merge($it,$b)$(.push($x))*)
);
