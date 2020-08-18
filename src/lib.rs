#[macro_use]
pub mod macros;

pub mod charbool;
pub mod err;
pub mod iter;
pub mod parser;
pub mod strung;

pub use charbool::*;
pub use err::*;
pub use iter::*;
pub use macros::*;
pub use parser::*;
pub use strung::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
