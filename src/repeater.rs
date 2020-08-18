//use crate::convert::*;
//use crate::err::*;
use crate::iter::*;
use crate::parser::*;
use crate::tuple::*;

#[derive(Clone)]
pub struct Exact<A> {
    n: usize,
    a: A,
}

impl<'a, A: Parser<'a>> Parser<'a> for Exact<A> {
    type Out = Vec<A::Out>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Vec<A::Out>> {
        do_rep(it, &self.a, self.n, true)
    }
}

pub struct Reflect<A, B, C> {
    a: A,
    b: B,
    c: C,
}
impl<'a, A, B, C> Parser<'a> for Reflect<A, B, C>
where
    A: Parser<'a>,
    B: Parser<'a>,
    C: Parser<'a>,
{
    type Out = (Vec<A::Out>, B::Out, Vec<C::Out>);
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (ni, (va, b), _) = do_repeat_until(it, 1, &self.a, &self.b)?;
        let (fi, vc, _) = do_rep(&ni, &self.c, va.len(), true)?;
        Ok((fi, (va, b, vc), None))
    }
}

/// A function for making sure number match on both sides of an equals
///
/// ```rust
/// use bogobble::*;
/// let p = reflect(ws_("("),Alpha.min_n(1),ws_(")"));
///
/// let (av,b,cv) =p.parse_s("(((help)))").unwrap();
///
/// assert_eq!(av,vec!["(","(","("]);
/// assert_eq!(b,"help".to_string());
/// assert_eq!(cv,vec![")",")",")"]);
///
/// let r2 = p.parse_s("(((no))");
/// assert!(r2.is_err());
/// ```
///
pub fn reflect<'a, A, B, C>(a: A, b: B, c: C) -> Reflect<A, B, C>
where
    A: Parser<'a>,
    B: Parser<'a>,
    C: Parser<'a>,
{
    Reflect { a, b, c }
}

/// Repeat an exact number of times
///
/// ```
/// use bogobble::*;
/// let p = exact(first(common::Int,","),5);
/// let v = p.parse_s("7,6,5,4,3,2,1").unwrap();
/// assert_eq!(v,vec![7,6,5,4,3]);
/// ```
pub fn exact<'a, A: Parser<'a>>(a: A, n: usize) -> Exact<A> {
    Exact { a, n }
}

fn do_sep<'a, A: Parser<'a>, B: Parser<'a>>(
    i: &PIter<'a>,
    a: &A,
    b: &B,
    min: usize,
    exact: bool,
) -> ParseRes<'a, Vec<A::Out>> {
    let mut res = Vec::new();
    let mut ri = i.clone();
    //TODO  consider wraping this error as parent
    loop {
        ri = match a.parse(&ri) {
            Ok((r, v, _)) => {
                res.push(v);
                r
            }
            Err(e) => {
                if res.len() == 0 && min == 0 {
                    return Ok((ri, res, Some(e)));
                }
                if res.len() == min && exact {
                    return Ok((ri, res, None));
                }
                return Err(e);
            }
        };
        //try sep if not found, return
        ri = match b.parse(&ri) {
            Ok((r, _, _)) => r,
            Err(e) => {
                if res.len() < min {
                    return Err(e);
                } else {
                    return Ok((ri, res, Some(e)));
                }
            }
        };
    }
}

#[derive(Clone)]
pub struct SepStar<A, B> {
    a: A,
    b: B,
}

impl<'a, A, B> Parser<'a> for SepStar<A, B>
where
    A: Parser<'a>,
    B: Parser<'a>,
{
    type Out = Vec<A::Out>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_sep(it, &self.a, &self.b, 0, false)
    }
}

pub fn sep_star<'a, A: Parser<'a>, B: Parser<'a>>(a: A, b: B) -> SepStar<A, B> {
    SepStar { a, b }
}
pub fn sep_plus<'a, A: Parser<'a>, B: Parser<'a>>(a: A, b: B) -> SepPlus<A, B> {
    SepPlus { a, b }
}

#[derive(Clone)]
pub struct SepPlus<A, B> {
    a: A,
    b: B,
}

impl<'a, A, B> Parser<'a> for SepPlus<A, B>
where
    A: Parser<'a>,
    B: Parser<'a>,
{
    type Out = Vec<A::Out>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_sep(it, &self.a, &self.b, 1, false)
    }
}

pub fn do_rep<'a, A: Parser<'a>>(
    i: &PIter<'a>,
    a: &A,
    min: usize,
    exact: bool,
) -> ParseRes<'a, Vec<A::Out>> {
    let mut it = i.clone();
    let mut res = Vec::new();

    loop {
        match a.parse(&it) {
            Ok((i2, v, _)) => {
                res.push(v);
                if it.lc() == i2.lc() && !exact {
                    return Err(it.err_s("To Consume some data"));
                }
                if res.len() == min && exact {
                    return Ok((i2, res, None));
                }
                it = i2;
            }
            Err(e) => {
                if res.len() >= min {
                    return Ok((it, res, Some(e)));
                }
                return Err(e);
            }
        }
    }
}

#[derive(Clone)]
pub struct RepStar<A> {
    a: A,
}

impl<'a, A: Parser<'a>> Parser<'a> for RepStar<A> {
    type Out = Vec<A::Out>;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_rep(i, &self.a, 0, false)
    }
}

pub fn star<'a, A: Parser<'a>>(a: A) -> RepStar<A> {
    RepStar { a }
}

#[derive(Clone)]
pub struct RepPlus<A> {
    a: A,
}

impl<'a, A: Parser<'a>> Parser<'a> for RepPlus<A> {
    type Out = Vec<A::Out>;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_rep(i, &self.a, 1, false)
    }
}

pub fn plus<'a, A: Parser<'a>>(a: A) -> RepPlus<A> {
    RepPlus { a }
}

fn do_repeat_until<'a, A: Parser<'a>, B: Parser<'a>>(
    it: &PIter<'a>,
    min: i32,
    a: &A,
    b: &B,
) -> ParseRes<'a, (Vec<A::Out>, B::Out)> {
    let mut ri = it.clone();
    let mut res = Vec::new();
    let mut done = 0;
    loop {
        let b_err = match done >= min {
            true => match b.parse(&ri) {
                Ok((r, v, _)) => return Ok((r, (res, v), None)),
                Err(e) => Some(e),
            },
            false => None,
        };
        ri = match a.parse(&ri) {
            Ok((r, v, _)) => {
                if r.lc() == ri.lc() {
                    return Err(r.err_s("To Consume some Data"));
                }
                res.push(v);
                r
            }
            Err(e) => {
                return match b_err {
                    Some(b_err) => Err(e.join(b_err)),
                    None => Err(e),
                }
            }
        };
        done += 1;
    }
}

pub struct StarUntil<A, B> {
    a: A,
    b: B,
}

impl<'a, A: Parser<'a>, B: Parser<'a>> Parser<'a> for StarUntil<A, B> {
    type Out = (Vec<A::Out>, B::Out);
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_repeat_until(i, 0, &self.a, &self.b)
    }
}

pub struct PlusUntil<A, B> {
    a: A,
    b: B,
}

impl<'a, A: Parser<'a>, B: Parser<'a>> Parser<'a> for PlusUntil<A, B> {
    type Out = (Vec<A::Out>, B::Out);
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_repeat_until(i, 1, &self.a, &self.b)
    }
}

pub fn star_until<'a, A: Parser<'a>, B: Parser<'a>>(a: A, b: B) -> StarUntil<A, B> {
    StarUntil { a, b }
}
pub fn plus_until<'a, A: Parser<'a>, B: Parser<'a>>(a: A, b: B) -> PlusUntil<A, B> {
    PlusUntil { a, b }
}

pub fn star_until_ig<'a, A: Parser<'a>, B: Parser<'a>, F: Fn((Vec<A::Out>, B::Out)) -> A::Out>(
    a: A,
    b: B,
) -> FirstRes<StarUntil<A, B>> {
    first_res(star_until(a, b))
}
pub fn plus_until_ig<'a, A: Parser<'a>, B: Parser<'a>>(a: A, b: B) -> FirstRes<PlusUntil<A, B>> {
    first_res(plus_until(a, b))
}

pub struct SepUntil<A, B, C> {
    a: A,
    b: B,
    c: C,
}

impl<'a, A, B, C> Parser<'a> for SepUntil<A, B, C>
where
    A: Parser<'a>,
    B: Parser<'a>,
    C: Parser<'a>,
{
    type Out = (Vec<A::Out>, C::Out);
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let mut ri = i.clone();
        let mut res = Vec::new();
        match self.c.parse(&ri) {
            Ok((r, v, _)) => return Ok((r, (res, v), None)),
            Err(_) => {}
        }
        loop {
            ri = match self.a.parse(&ri) {
                Ok((r, v, _)) => {
                    res.push(v);
                    r
                }
                Err(e) => return Err(e),
            };
            let c_err = match self.c.parse(&ri) {
                Ok((r, v, _)) => return Ok((r, (res, v), None)),
                Err(e) => e,
            };
            ri = match self.b.parse(&ri) {
                Ok((r, _, _)) => r,
                Err(e) => return Err(e.join(c_err)),
            }
        }
    }
}

///Allows for better errors looping until a specific finish. It does not return the close or the
///seperators the
///close is expected to be some kind of closer like '}'
///If you need the close you will have to use sep(..).then(..) though the errors will be less
///nice Recent changes mean that this now returns the ending result aswel, if you wish to ignore
///that use sep_until_ig
pub fn sep_until<'a, A, B, C>(a: A, b: B, c: C) -> SepUntil<A, B, C>
where
    A: Parser<'a>,
    B: Parser<'a>,
    C: Parser<'a>,
{
    SepUntil { a, b, c }
}

pub fn sep_until_ig<'a, A, B, C>(a: A, b: B, c: C) -> FirstRes<SepUntil<A, B, C>>
where
    A: Parser<'a>,
    B: Parser<'a>,
    C: Parser<'a>,
{
    first_res(sep_until(a, b, c))
}

#[cfg(test)]
pub mod test {
    use super::*;
    //use crate::ptrait::*;
    use crate::*;
    #[test]
    pub fn test_reflecter() {
        let (av, b, cv) = reflect(ws__("("), (Alpha, NumDigit).plus(), ws__(")"))
            .parse_s("(((help)))")
            .unwrap();

        assert_eq!(av, vec!["(", "(", "("]);
        assert_eq!(b, "help".to_string());
        assert_eq!(cv, vec![")", ")", ")"]);
    }
}
