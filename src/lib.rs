#[macro_use]
pub mod macros;

pub mod charbool;
pub mod err;
pub mod iter;
pub mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
