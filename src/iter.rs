use crate::err::*;
use crate::parser::*;
use std::str::CharIndices;

#[derive(Debug, Clone)]
pub struct PIter<'a> {
    it: CharIndices<'a>,
    l: usize,
    c: usize,
}

impl<'a> PIter<'a> {
    pub fn new(s: &'a str) -> Self {
        PIter {
            it: s.char_indices(),
            l: 0,
            c: 0,
        }
    }
    pub fn as_str(&self) -> &'a str {
        self.it.as_str()
    }

    pub fn str_to(&self, id: Option<usize>) -> &'a str {
        match (self.index(), id) {
            (Some(s), Some(f)) => &self.it.as_str()[..(f - s)],
            _ => self.it.as_str(),
        }
    }

    pub fn err(&self, exp: Expected) -> PErr<'a> {
        PErr {
            exp,
            found: self.it.as_str(),
            index: self.index(),
            line: self.l,
            col: self.c,
            is_break: false,
            child: None,
        }
    }

    pub fn err_s(&self, s: &'static str) -> PErr<'a> {
        self.err(Expected::Str(s))
    }

    pub fn err_rs<V>(&self, s: &'static str) -> Result<V, PErr<'a>> {
        Err(self.err_s(s))
    }

    pub fn err_r<V>(&self, e: Expected) -> Result<V, PErr<'a>> {
        Err(self.err(e))
    }

    pub fn lc(&self) -> (usize, usize) {
        (self.l, self.c)
    }
    pub fn index(&self) -> Option<usize> {
        self.it.clone().next().map(|(i, _)| i)
    }

    pub fn next_i(&mut self) -> Option<(usize, char)> {
        self.it.next()
    }
}

impl<'a> Iterator for PIter<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        //println!("lc {} {} ", self.l, self.c);
        match self.it.next() {
            Some((_, '\n')) => {
                self.l += 1;
                self.c = 0;
                Some('\n')
            }
            Some((_, v)) => {
                self.c += 1;
                Some(v)
            }
            None => None,
        }
    }
}

pub fn index<'a>(it: &PIter<'a>) -> ParseRes<'a, Option<usize>> {
    return Ok((it.clone(), it.index(), None));
}

pub fn line_col<'a>(it: &PIter<'a>) -> ParseRes<'a, (usize, usize)> {
    return Ok((it.clone(), (it.l, it.c), None));
}
