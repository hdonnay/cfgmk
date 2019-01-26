#[macro_use]
extern crate nom;

use nom::*;

#[derive(Debug,PartialEq)]
pub enum Directive {
    Create,
    Append,
}

#[derive(Debug,PartialEq)]
pub struct Stmt {
    pub kind: Directive,
    pub path: String,
    pub from: From,
}

#[derive(Debug,PartialEq)]
pub enum From {
    File(String),
    Filter(String, String),
    Literal(String),
}

named!(stmt<Stmt>, do_parse!(
    kind: directive >>
    take_while1!(is_space) >>
    path: string >>
    take_while1!(is_space) >>
    from: alt!(fromFile | fromFilter | fromLiteral) >>
    char!('\n') >>
    (Stmt {
        kind: kind,
        path: path,
        from: from,
    })
));
named!(directive< Directive >, alt!(
    value!(Directive::Create, tag!("create")) |
    value!(Directive::Append, tag!("append"))
));
fn to_s(i: &[u8]) -> String {
    String::from_utf8_lossy(&i).into_owned().replace("''", "'")
}

named!(string< String >, map!(do_parse!(
    tag!("'") >>
    s : alt!(
        recognize!(is_not!("'")) |
        escaped!(is_not!("'"), '\'', one_of!("'"))
    ) >>
    tag!("'") >>
    (s)
), to_s));

named!(fromFile< From >, do_parse!(
    tag!("from") >>
    f: string >>
    (From::File(f))
));
named!(fromFilter< From >, do_parse!(
    tag!("filter") >>
    which: string >>
    f: string >>
    (From::Filter(which, f))
));
named!(fromLiteral<From>, do_parse!(
    tag!("literal") >>
    take_while1!(is_space) >>
    l: string >>
    (From::Literal(l))
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        assert_eq!(fromLiteral(&b"literal 'test'"[..]),
            Ok((&b""[..], From::Literal("test".to_string()))));
        assert_eq!(fromLiteral(&b"literal 'devil''s advocate'"[..]),
            Ok((&b""[..], From::Literal("devil's advocate".to_string()))));
    }
}
