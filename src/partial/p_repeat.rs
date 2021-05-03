use crate::iter::*;
use crate::parser::*;
//use crate::tuple::*;

#[derive(Clone)]
pub struct PExact<A> {
    n: usize,
    a: A,
}

impl<'a, A: Parser<'a>> Parser<'a> for PExact<A> {
    type Out = Vec<A::Out>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Vec<A::Out>> {
        do_rep(it, &self.a, self.n, true)
    }
}

/// Repeat an exact number of times
///
/// ```
/// use bogobble::*;
/// let p = exact(first(common::Int,","),5);
/// let v = p.parse_s("7,6,5,4,3,2,1").unwrap();
/// assert_eq!(v,vec![7,6,5,4,3]);
/// ```
pub fn p_exact<'a, A: Parser<'a>>(a: A, n: usize) -> PExact<A> {
    PExact { a, n }
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
    loop {
        ri = match a.parse(&ri) {
            Ok((r, v, _)) => {
                res.push(v);
                r
            }
            Err(e) => {
                if ri.eoi() {
                    return Ok((ri, res, None));
                }
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
                if ri.eoi() {
                    return Ok((ri, res, None));
                }
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
pub struct PSepStar<A, B> {
    a: A,
    b: B,
}

impl<'a, A, B> Parser<'a> for PSepStar<A, B>
where
    A: Parser<'a>,
    B: Parser<'a>,
{
    type Out = Vec<A::Out>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_sep(it, &self.a, &self.b, 0, false)
    }
}

pub fn p_sep_star<'a, A: Parser<'a>, B: Parser<'a>>(a: A, b: B) -> PSepStar<A, B> {
    PSepStar { a, b }
}
pub fn p_sep_plus<'a, A: Parser<'a>, B: Parser<'a>>(a: A, b: B) -> PSepPlus<A, B> {
    PSepPlus { a, b }
}

#[derive(Clone)]
pub struct PSepPlus<A, B> {
    a: A,
    b: B,
}

impl<'a, A, B> Parser<'a> for PSepPlus<A, B>
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
                if i2.eoi() {
                    return Ok((i2, res, None));
                }
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
pub struct PStar<A>(pub A);

impl<'a, A: Parser<'a>> Parser<'a> for PStar<A> {
    type Out = Vec<A::Out>;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_rep(i, &self.0, 0, false)
    }
}

pub fn p_star<'a, A: Parser<'a>>(a: A) -> PStar<A> {
    PStar(a)
}

#[derive(Clone)]
pub struct PPlus<A>(pub A);

impl<'a, A: Parser<'a>> Parser<'a> for PPlus<A> {
    type Out = Vec<A::Out>;
    fn parse(&self, i: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_rep(i, &self.0, 1, false)
    }
}

pub fn p_plus<'a, A: Parser<'a>>(a: A) -> PPlus<A> {
    PPlus(a)
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
