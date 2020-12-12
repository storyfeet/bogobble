use crate::traits::*;
//use std::convert::Into;

pub fn strings_plus_until<A: OParser<C>, B: OParser<D>, C: AsRef<str>, D>(
    a: A,
    b: B,
) -> StringsPlusUntil<A, B> {
    StringsPlusUntil { a, b }
}

pub struct StringsPlusUntil<A, B> {
    a: A,
    b: B,
}

impl<'a, A: Parser<'a, Out = String>, B: Parser<'a>> Parser<'a> for StringsPlusUntil<A, B> {
    type Out = (String, B::Out);
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_strings_until(it, &self.a, &self.b, 1)
    }
}
pub fn strings_star_until<A: OParser<C>, B: OParser<D>, C: AsRef<str>, D>(
    a: A,
    b: B,
) -> StringsPlusUntil<A, B> {
    StringsPlusUntil { a, b }
}

pub struct StringsStarUntil<A, B> {
    a: A,
    b: B,
}
impl<'a, A: Parser<'a, Out = String>, B: Parser<'a>> Parser<'a> for StringsStarUntil<A, B> {
    type Out = (String, B::Out);
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        do_strings_until(it, &self.a, &self.b, 0)
    }
}

pub fn do_strings_until<'a, A: Parser<'a, Out = C>, B: Parser<'a>, C: AsRef<str>>(
    it: &PIter<'a>,
    a: &A,
    b: &B,
    min: usize,
) -> ParseRes<'a, (String, B::Out)> {
    let mut res = String::new();
    let mut it = it.clone();
    let mut done = 0;
    loop {
        let b_err = if done >= min {
            match b.parse(&it) {
                Ok((nit, v, e)) => return Ok((nit, (res, v), e)),
                Err(e) => Some(e),
            }
        } else {
            None
        };
        match a.parse(&it) {
            Ok((nit, v, _e)) => {
                res.push_str(v.as_ref());
                it = nit;
                done += 1;
            }
            Err(e) => {
                if let Some(berr) = b_err {
                    return Err(e.join(berr));
                }
            }
        }
    }
}

pub fn strings_plus<A: OParser<I>, I: Into<String>>(a: A) -> StringsPlus<A> {
    StringsPlus { a }
}

pub struct StringsPlus<A> {
    a: A,
}

impl<'a, A: Parser<'a>> Parser<'a> for StringsPlus<A>
where
    A::Out: AsRef<str>,
{
    type Out = String;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, String> {
        let mut res = String::new();
        let mut it = it.clone();
        loop {
            match self.a.parse(&it) {
                Ok((i, v, _)) => {
                    res.push_str(v.as_ref());
                    it = i;
                }
                Err(e) => {
                    if res.len() > 0 {
                        return Ok((it, res, Some(e)));
                    }
                    return Err(e);
                }
            }
        }
    }
}
