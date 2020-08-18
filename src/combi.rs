use crate::err::*;
use crate::iter::*;
use crate::parser::*;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Maybe<A> {
    p: A,
}

impl<'a, A> Parser<'a> for Maybe<A>
where
    A: Parser<'a>,
{
    type Out = Option<A::Out>;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        match self.p.parse(i) {
            Ok((ir, v, ex)) => Ok((ir, Some(v), ex)),
            Err(e) => Ok((i.clone(), None, Some(e))),
        }
    }
}

/// returns an option on whether this item was found A common use would be
/// looking for a minus on the front of a number
///
/// ```rust
/// use bogobble::*;
/// use std::str::FromStr;
/// let p = (maybe("-"),(NumDigit.min_n(1))).try_map(|(m,n)|{
///     let res:i32 = n.parse().map_err(|e|Expected::Str("[1..9]+"))?;
///     if m.is_some() {
///         return Ok(-res )
///     }
///     Ok(res)
/// });
/// let s = p.parse_s("-34").unwrap();
/// assert_eq!(s,-34);
/// let s = p.parse_s("34").unwrap();
/// assert_eq!(s,34);
/// ```
pub fn maybe<'a, P: Parser<'a>>(p: P) -> Maybe<P> {
    Maybe { p }
}

pub struct Exists<P> {
    p: P,
}

impl<'a, P: Parser<'a>> Parser<'a> for Exists<P> {
    type Out = bool;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, bool> {
        match self.p.parse(it) {
            Ok((nit, _, e)) => Ok((nit, true, e)),
            Err(e) => Ok((it.clone(), false, Some(e))),
        }
    }
}

pub fn exists<'a, P: Parser<'a>>(p: P) -> Exists<P> {
    Exists { p }
}

#[derive(Clone)]
pub struct Wrap<A, B> {
    a: A,
    b: B,
}

impl<'a, A, B> Parser<'a> for Wrap<A, B>
where
    A: Parser<'a>,
    B: Parser<'a>,
{
    type Out = B::Out;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (i, _, c1) = self.a.parse(i)?;
        let (i, res, c2) = self.b.parse(&i).join_err_op(c1)?;
        let (n, _, c3) = self.a.parse(&i).join_err_op(c2)?;
        Ok((n, res, c3))
    }
}

pub fn wrap<'a, A, B>(a: A, b: B) -> Wrap<A, B>
where
    A: Parser<'a>,
    B: Parser<'a>,
{
    Wrap { a, b }
}

impl<'a, P: Parser<'a, Out = V>, V: Debug> Parser<'a> for FailOn<P> {
    type Out = ();
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, ()> {
        match self.p.parse(it) {
            Ok((_, _, _)) => it.err_r(Expected::Str("Failon Succeeded")),
            Err(_) => Ok((it.clone(), (), None)),
        }
    }
}

pub struct FailOn<P> {
    p: P,
}

pub fn fail_on<'a, P: Parser<'a>>(p: P) -> FailOn<P> {
    FailOn { p }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PDebugger<P> {
    p: P,
    s: &'static str,
}

pub fn debug<'a, P: Parser<'a>>(p: P, s: &'static str) -> PDebugger<P> {
    PDebugger { p, s }
}

impl<'a, P: Parser<'a>> Parser<'a> for PDebugger<P> {
    type Out = P::Out;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        println!("DebuggerPre - {}", self.s);
        let r = self.p.parse(it);
        match &r {
            Ok((nit, _, _)) => {
                let s = match (it.index(), nit.index()) {
                    (Some(st), Some(f)) => &it.as_str()[..f - st],
                    _ => it.as_str(),
                };
                println!("Success - {}, \"{}\"", self.s, s);
            }
            Err(_) => println!("Fail - {}", self.s),
        };
        r
    }
}
