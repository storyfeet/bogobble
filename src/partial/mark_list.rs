use super::PosTree;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter, Write};

#[derive(Clone, PartialEq, Debug)]
pub enum MarkErr {
    OutOfBounds,
}
impl Error for MarkErr {}

impl Display for MarkErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MarkErr::OutOfBounds => write!(f, "Index Outside of working String"),
        }
    }
}

pub fn mark_str<I: Clone + Display>(t: &PosTree<I>, s: &str) -> Result<String, MarkErr> {
    MarkList::new(t, None).mark_str(s)
}

struct MarkList<I>(BTreeMap<usize, I>);

impl<I: Clone + Display> MarkList<I> {
    pub fn new(t: &PosTree<I>, end: Option<&I>) -> Self {
        let mut res = MarkList(BTreeMap::new());
        res.set_marks(t, end);
        res
    }

    pub fn set_marks(&mut self, t: &PosTree<I>, end: Option<&I>) {
        if let Some(s) = t.start {
            self.0.insert(s, t.item.clone());
        }
        for c in &t.children {
            self.set_marks(c, Some(&t.item));
        }
        if let (Some(s), Some(e)) = (t.fin, end) {
            self.0.insert(s, e.clone());
        }
    }

    pub fn mark_str(&mut self, s: &str) -> Result<String, MarkErr> {
        let mut res = String::new();

        let mut last_i = 0;

        for (k, m) in &self.0 {
            write!(
                res,
                "{}{}",
                s.get(last_i..*k).ok_or(MarkErr::OutOfBounds)?,
                m
            )
            .ok();
            last_i = *k;
        }
        res.push_str(&s[last_i..]);
        Ok(res)
    }
}
