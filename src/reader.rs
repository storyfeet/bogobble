use crate::charbool::*;
use crate::combi::*;
use crate::err::*;
use crate::iter::*;
use crate::parser::*;
use crate::select::*;

pub type StrPos = Pos<()>;

#[derive(Debug, Clone, PartialEq)]
pub struct Pos<O> {
    pub line: usize,
    pub col: usize,
    pub start: Option<usize>,
    pub fin: Option<usize>,
    pub ob: O,
}

impl<O> Pos<O> {
    ///This version assumes that this is the string it came from
    pub fn on_str<'a>(&self, s: &'a str) -> &'a str {
        match (self.start, self.fin) {
            (Some(st), Some(f)) => &s[st..f],
            (Some(st), None) => &s[st..],
            _ => "",
        }
    }
}

pub struct PPos<P> {
    p: P,
}

impl<'a, P: Parser<'a>> Parser<'a> for PPos<P> {
    type Out = Pos<P::Out>;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let (line, col) = it.lc();
        let start = it.index();
        let (rit, r, pex) = self.p.parse(it)?;
        let fin = rit.index();
        Ok((
            rit,
            Pos {
                line,
                col,
                start,
                fin,
                ob: r,
            },
            pex,
        ))
    }
}

/// ```rust
/// use bogobble::*;
/// let s = " \n  hello   ".to_string();
/// let v = last("\n ".istar(),pos_ig(Alpha.istar())).parse_s(&s).unwrap();
/// assert_eq!(v,Pos{line:1,col:2,start:Some(4),fin:Some(9),ob:()});
/// assert_eq!(v.on_str(&s),"hello");
/// ```
pub fn pos_ig<'a, P: Parser<'a>>(p: P) -> PPos<Ig<P>> {
    PPos { p: p.ig() }
}

pub fn pos<'a, P: Parser<'a>>(p: P) -> PPos<P> {
    PPos { p }
}

pub fn ws__<P: OParser<V>, V>(p: P) -> impl OParser<V> {
    wrap(WS.istar(), p)
}

pub fn ws_<P: OParser<V>, V>(p: P) -> impl OParser<V> {
    last(WS.istar(), p)
}

pub fn do_keyword<'a, P: Parser<'a>>(it: &PIter<'a>, p: &P) -> ParseRes<'a, P::Out> {
    let (t2, r, _) = p.parse(it)?;
    match t2.clone().next() {
        Some(c) => {
            let al = (Alpha, NumDigit, '_');
            if al.char_bool(c) {
                t2.err_r(Expected::Keyword(Box::new(al.expected())))
            } else {
                Ok((t2, r, None))
            }
        }
        None => Ok((t2, r, None)),
    }
}

impl<'a, P: Parser<'a>> Parser<'a> for KeyWord<P> {
    type Out = P::Out;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, P::Out> {
        do_keyword(it, &self.p)
    }
}

pub struct KeyWord<P> {
    p: P,
}

///```rust
/// use bogobble::*;
/// assert_eq!(keyword("let").parse_s("let"), Ok("let"));
/// assert_eq!(keyword("let").parse_s("let "), Ok("let"));
/// assert_eq!(keyword("let").parse_s("let*"), Ok("let"));
/// assert!(keyword("let").parse_s("letl").is_err());
///```
pub fn keyword<'a, P: Parser<'a>>(p: P) -> KeyWord<P> {
    KeyWord { p }
}

parser! {(EOI->())
    eoi
}

pub fn eoi<'a>(i: &PIter<'a>) -> ParseRes<'a, ()> {
    let mut r = i.clone();
    if r.next() == None {
        return Ok((r, (), None));
    }
    i.err_r(Expected::EOI)
}

pub fn to_end() -> impl OParser<()> {
    (WS.star(), eoi).ig()
}

pub struct Peek<P> {
    p: P,
}

impl<'a, P: Parser<'a>> Parser<'a> for Peek<P> {
    type Out = P::Out;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, P::Out> {
        let (_, v, c) = self.p.parse(it)?;
        Ok((it.clone(), v, c))
    }
}

pub fn peek<'a, P: Parser<'a>>(p: P) -> Peek<P> {
    Peek { p }
}

pub struct CharsUntil<A, B> {
    a: A,
    b: B,
}

impl<'a, A: Parser<'a, Out = char>, B: Parser<'a>> Parser<'a> for CharsUntil<A, B> {
    type Out = (String, B::Out);
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
        let mut res = String::new();
        let mut it = it.clone();
        loop {
            //let it2 = it.clone();
            if let Ok((i, bv, c1)) = self.b.parse(&it) {
                return Ok((i, (res, bv), c1));
            }
            it = match self.a.parse(&it) {
                Ok((i, c, _)) => {
                    res.push(c);
                    i
                }
                Err(e) => return Err(e),
            };
        }
    }
}

pub fn chars_until<'a, A: Parser<'a, Out = char>, B: Parser<'a>>(a: A, b: B) -> CharsUntil<A, B> {
    CharsUntil { a, b }
}

pub struct StringRepeat<A> {
    a: A,
    min: usize,
}

impl<'a, A: Parser<'a, Out = AV>, AV: Into<String> + AsRef<str>> Parser<'a> for StringRepeat<A> {
    type Out = String;
    fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, String> {
        let (mut nit, mut res) = match self.a.parse(it) {
            Ok((it2, ss, _)) => (it2, ss.into()),
            Err(e) => {
                if self.min == 0 {
                    return Ok((it.clone(), String::new(), Some(e)));
                } else {
                    return Err(e);
                }
            }
        };
        let mut done = 1;
        loop {
            match self.a.parse(&nit) {
                Ok((it, r, _)) => {
                    res.push_str(r.as_ref());
                    nit = it;
                }
                Err(e) => {
                    if done < self.min {
                        return Err(e);
                    } else {
                        return Ok((nit, res, Some(e)));
                    }
                }
            }
            done += 1;
        }
    }
}

pub fn string_repeat<'a, A: Parser<'a, Out = AV>, AV: Into<String> + AsRef<str>>(
    a: A,
    min: usize,
) -> StringRepeat<A> {
    StringRepeat { a, min }
}
