use crate::traits::*;

pub trait PartCharBool: CharBool + Sized {
    fn p_until(
}

impl<C: CharBool> PartCharBool for C {}


pub struct PUntil<C,P>{


}
