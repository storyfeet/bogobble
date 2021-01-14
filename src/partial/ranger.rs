use std::ops::{Bound, RangeBounds};

#[derive(Debug, Clone)]
pub enum Ranger {
    InEx(usize, usize),
    InOpen(usize),
}

impl Ranger {
    pub fn with_end(&self, n: usize) -> Self {
        match self {
            Ranger::InEx(a, _) => Ranger::InEx(*a, n),
            Ranger::InOpen(a) => Ranger::InEx(*a, n),
        }
    }
}

impl RangeBounds<usize> for Ranger {
    fn start_bound(&self) -> Bound<&usize> {
        match self {
            Ranger::InEx(n, _) => Bound::Included(n),
            Ranger::InOpen(n) => Bound::Included(n),
        }
    }

    fn end_bound(&self) -> Bound<&usize> {
        match self {
            Ranger::InEx(_, n) => Bound::Excluded(n),
            Ranger::InOpen(_) => Bound::Unbounded,
        }
    }
}
