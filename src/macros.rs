//! The macros primarily exist to make creating zero size parsers easier.
//! Without putting them in macros "&'static str" and "chars" can act as parsers,
//! but they have a size, and when combined they can become bigger.
//! If however all the parsers you combine have zero size, then the final resulting parser
//! will also be zero size and therefor much easier to construct
//!

/// Makes zero sized parsers based on the expression given and potentially the return type given.

/// ```rust
/// use bogobble::*;
/// parser!{
///     (Cat->&'a str),
///     "cat".plus(),
/// }
/// assert_eq!(Cat.parse_s("ctar"),Ok("cta"));
/// ```
#[macro_export]
macro_rules! parser {
    ($id:ident,$x:expr) => {
        parser!(($id->&'static str) $x);
    };
    ($($doc:literal $(,)?)? ($id:ident -> $ot:ty) $(,)? $x:expr $(,)?) => {
        parser!($($doc)? ($id->$ot) $x, stringify!($id));
    };
    ($id:ident,$x:expr,$exp:expr) => {
        parser!(($id->&'static str) $x, $exp);
    };
    ($($doc:literal $(,)?)? ($id:ident -> $ot:ty) $(,)? $x:expr,$exp:expr $(,)?) => {
        $(#[doc=$doc])?
        #[derive(Copy, Clone)]
        pub struct $id;
        impl <'a> Parser<'a> for $id {
            type Out = $ot;
            ///Parse run the main parser
            fn parse(&self, it: &PIter<'a>) -> ParseRes<'a, Self::Out> {
                let name_e = it.err_s($exp);
                match (&$x).parse(it){
                    Ok(v)=> Ok(v),
                    Err(e)=> match (e.index,name_e.index) {
                        (Some(ei),Some(ii)) if (ii == ei) => Err(it.err_s($exp)),
                        _=>Err(e.join(name_e)),
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! parser_as {
    (($ot:ty),(($id:ident->$res:expr) $(,)? $main:expr,$exp:expr $(,)?) ) => {
        parser! {($id->$ot) ,$main.map(|_|$res),$exp}
    };
    (($ot:ty),(($id:ident->$res:expr) $(,)? $main:expr $(,)?) ) => {
        parser! {($id->$ot) ,$main.map(|_|$res)}
    };
    (($ot:ty),($id:ident, $main:expr)) => {
        parser! { ($id->$ot) $main}
    };
}

#[macro_export]
macro_rules! as_id {
    ((($id:ident->$_x:expr) $($_t:tt)*) ) => {
        $id
    };
    (($id:ident $($_t:tt)*) ) => {
        $id
    };
}

#[macro_export]
macro_rules! char_bool {
    ($id:ident,$x:expr) => {
        char_bool!($id, $x, Expected::CharIn(stringify!($id)));
    };
    ($id:ident,$x:expr,$s:literal) => {
        char_bool!($id, $x, Expected::Char($s));
    };
    ($id:ident,$x:expr,$exp:expr) => {
        #[derive(Copy, Clone)]
        pub struct $id;
        impl CharBool for $id {
            fn char_bool(&self, c: char) -> bool {
                (&$x).char_bool(c)
            }
            fn expected(&self) -> Expected {
                $exp
            }
        }
    };
}

#[macro_export]
macro_rules! char_bools {
    ( $( ($id:ident,$x:expr) ),*) => {$(char_bool!($id,$x);)*};
}

/// a macro replacement for numbered or statements.
/// ```rust
/// use bogobble::*;
/// assert_eq!(or!("cat","dog","car",).parse_s("catdogman "),Ok("cat"));
/// ```
#[macro_export]
macro_rules! or{
    ($s:expr,$($x:expr),* $(,)?) => { $s$(.or($x))*;};
}

#[macro_export]
macro_rules! or_ig{
    ($s:expr,$($x:expr),* $(,)?) => { $s.ig()$(.or($x.ig()))*;};
}

#[cfg(test)]
mod test {
    use super::*;
    fn size_of<T: Sized>(_t: &T) -> usize {
        std::mem::size_of::<T>()
    }

    use crate::*;
    parser!(DOG, "dog");
    parser!(CAR, "car");
    parser!(CAT, "cat");

    parser!((GROW->Vec<&'static str>) star(or(CAT, DOG)));

    #[test]
    pub fn parser_makes_parser() {
        assert_eq!(DOG.parse_s("dog   "), Ok("dog"));
        assert_eq!(CAT.parse_s("cat    "), Ok("cat"));
        assert_eq!(
            GROW.parse_s("catdogcatcatno"),
            Ok(vec!["cat", "dog", "cat", "cat"])
        );
    }

    char_bool!(HOT, "hot");
    char_bool!(MNUM, |c| c >= '0' && c <= '9');

    #[test]
    pub fn charbool_macro_makes_parser() {
        use err::Expected::*;
        let p = (HOT, MNUM);
        assert_eq!(std::mem::size_of::<(HOT, MNUM)>(), 0);
        assert_eq!(p.plus().parse_s("09h3f"), Ok("09h3"));
        assert_eq!(p.expected(), OneOf(vec![CharIn("HOT"), CharIn("MNUM")]));
        assert_eq!(size_of(&p), 0);
    }
}
