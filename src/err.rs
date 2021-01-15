use std::cmp::Ordering;
use std::fmt;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub enum Expected {
    Nil,
    EOI,
    Not(Box<Expected>),
    Char(char),
    CharIn(&'static str),
    Str(&'static str),
    OneOf(Vec<Expected>),
    Keyword(Box<Expected>),
}
impl Expected {
    pub fn join(self, b: Self) -> Self {
        match (self, b) {
            (Expected::OneOf(mut ae), Expected::OneOf(be)) => {
                ae.extend(be);
                Expected::OneOf(ae)
            }
            (Expected::OneOf(mut ae), b) | (b, Expected::OneOf(mut ae)) => {
                if b != Expected::Nil {
                    ae.push(b);
                }
                Expected::OneOf(ae)
            }
            (Expected::Nil, a) => a,
            (a, Expected::Nil) => a,
            (a, b) => Expected::OneOf(vec![a, b]),
        }
    }

    pub fn first(a: Self, b: Self) -> Self {
        match a == Expected::Nil {
            true => b,
            false => a,
        }
    }
}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expected::Nil => write!(f, "NIL"),
            Expected::EOI => write!(f, "EOI"),
            Expected::Not(b) => write!(f, "Not({})", b),
            Expected::Char(c) => write!(f, "{}", c),
            Expected::CharIn(s) => write!(f, "Char In '{}'", s),
            Expected::Str(s) => write!(f, "{}", s),
            Expected::Keyword(s) => write!(f, "keyword {}", s),
            Expected::OneOf(v) => {
                write!(f, "One of [")?;
                let mut coma = "";
                for e in v {
                    write!(f, "{}{}", coma, e)?;
                    coma = " , ";
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PErr<'a> {
    pub exp: Expected,
    pub found: &'a str,
    pub line: usize,
    pub col: usize,
    pub index: Option<usize>,
    pub is_break: bool,
    pub child: Option<Box<Self>>,
}
fn compare_index(a: &Option<usize>, b: &Option<usize>) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => a.cmp(b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

fn join_children<'a>(a: Option<Box<PErr<'a>>>, b: Option<Box<PErr<'a>>>) -> Option<Box<PErr<'a>>> {
    match (a, b) {
        (Some(ac), Some(bc)) => Some(Box::new((*ac).join(*bc))),
        (None, b) => b,
        (a, None) => a,
    }
}

fn read_10<'a>(s: &'a str) -> &'a str {
    match s.char_indices().take(10).last() {
        Some((n, _)) => &s[..n],
        None => "EOI",
    }
}

impl<'a> std::error::Error for PErr<'a> {}
impl<'a> fmt::Display for PErr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match self.index {
            Some(n) => n.to_string(),
            None => "EOI".to_string(),
        };
        write!(
            f,
            "Expected '{}', Found '{}', at (i:{},l:{},c:{})",
            self.exp,
            read_10(self.found),
            id,
            self.line,
            self.col
        )?;
        if let Some(ref c) = self.child {
            write!(f, "\n\t{}", c)?;
        }
        Ok(())
    }
}
impl<'a> PErr<'a> {
    pub fn longer(mut self, b: Self) -> Self {
        match compare_index(&self.index, &b.index) {
            Ordering::Greater => self,
            Ordering::Less => b,
            _ => {
                self.child = join_children(self.child, b.child);
                self.exp = self.exp.join(b.exp);
                self
            }
        }
    }

    pub fn brk(mut self) -> Self {
        self.is_break = true;
        self
    }

    pub fn join(mut self, mut b: Self) -> Self {
        match compare_index(&self.index, &b.index) {
            Ordering::Greater => {
                self.child = join_children(self.child, Some(Box::new(b)));
                self
            }
            Ordering::Less => {
                b.child = join_children(b.child, Some(Box::new(self)));
                b
            }
            _ => {
                self.child = join_children(self.child, b.child);
                self.exp = self.exp.join(b.exp);
                self
            }
        }
    }
    pub fn strung(self) -> StrungError {
        StrungError {
            exp: self.exp,
            found: read_10(self.found).to_string(),
            line: self.line,
            col: self.col,
            index: self.index,
            is_break: self.is_break,
            child: self.child.map(|v| Box::new((*v).strung())),
        }
    }
}
//The StrungError has the String it was parsed from attached to it.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StrungError {
    pub exp: Expected,
    pub found: String,
    pub index: Option<usize>,
    pub line: usize,
    pub col: usize,
    pub is_break: bool,
    pub child: Option<Box<StrungError>>,
}
impl std::error::Error for StrungError {}

impl fmt::Debug for StrungError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i_str = match self.index {
            Some(n) => n.to_string(),
            None => "EOI".to_string(),
        };
        write!(
            f,
            "Expected '{}', Found '{}', at (i={},l={},c={})\n",
            self.exp, self.found, i_str, self.line, self.col
        )
    }
}
impl fmt::Display for StrungError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let i_str = match self.index {
            Some(n) => n.to_string(),
            None => "EOI".to_string(),
        };
        write!(
            f,
            "Expected '{}', Found '{}', at (i={},l={},c={})\n",
            self.exp, self.found, i_str, self.line, self.col
        )?;
        if let Some(ref c) = self.child {
            write!(f, "\t{}", c)?
        }
        Ok(())
    }
}
