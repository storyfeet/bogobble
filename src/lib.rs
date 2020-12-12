#[macro_use]
pub mod macros;
//Keep at top
pub use macros::*;

pub mod charbool;
pub mod combi;
pub mod common;
pub mod convert;
pub mod err;
pub mod iter;
pub mod parser;
pub mod reader;
pub mod repeater;
pub mod select;
pub mod strings;
pub mod strung;
pub mod traits;
pub mod tuple;

pub use charbool::*;
pub use combi::*;
pub use convert::*;
pub use err::*;
pub use iter::*;
pub use parser::*;
pub use reader::*;
pub use repeater::*;
pub use select::*;
pub use strings::*;
pub use strung::*;
pub use tuple::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
