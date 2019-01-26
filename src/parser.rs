pub use nom::{IResult, Needed};

use nom::is_space;

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

named!(pub parse_statement<&[u8], Stmt>, do_parse!(
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

named!(sq, dbg!(delimited!(tag!("'"), take_until!("'"), tag!("'"))));
named!(string< String >, map!(
    dbg!(fold_many1!(sq, Vec::new(), |mut acc: Vec<String>, item| {
        acc.push(String::from_utf8_lossy(item).to_string());
        acc
    })),
    |v: Vec<String>| {
        v.join("'")
    }
));

named!(fromFile< From >, do_parse!(
    tag!("from") >>
    take_while1!(is_space) >>
    f: string >>
    (From::File(f))
));
named!(fromFilter<From>, do_parse!(
    tag!("filter") >>
    take_while1!(is_space) >>
    which: string >>
    take_while1!(is_space) >>
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
    fn singlequoting() {
        assert_eq!(sq(&b"'test'"[..]), Ok((&b""[..], &b"test"[..])));
        assert_eq!(sq(&b"'one tick: '''"[..]), Ok((&b"''"[..], &b"one tick: "[..])));
    }

    #[test]
    fn literal() {
        assert_eq!(fromLiteral(&b"literal 'test'\n"[..]),
            Ok((&b"\n"[..], From::Literal("test".to_string()))));
        assert_eq!(fromLiteral(&b"literal 'devil''s advocate'\n"[..]),
            Ok((&b"\n"[..], From::Literal("devil's advocate".to_string()))));
    }

    #[test]
    fn file() {
        assert_eq!(fromFile(&b"from 'test'\n"[..]),
            Ok((&b"\n"[..], From::File("test".to_string()))));
        assert_eq!(fromFile(&b"from '/dev/null'\n"[..]),
            Ok((&b"\n"[..], From::File("/dev/null".to_string()))));
        assert_eq!(fromFile(&b"from 'odd path'\n"[..]),
            Ok((&b"\n"[..], From::File("odd path".to_string()))));
    }

    #[test]
    fn filter() {
        assert_eq!(fromFilter(&b"filter 'cat' '/dev/null'\n"[..]),
            Ok((&b"\n"[..], From::Filter("cat".to_string(), "/dev/null".to_string()))));
        assert_eq!(fromFilter(&b"filter 'simple' '/dev/null'\n"[..]),
            Ok((&b"\n"[..], From::Filter("simple".to_string(), "/dev/null".to_string()))));
        assert_eq!(fromFilter(&b"filter 'other filter' 'odd path'\n"[..]),
            Ok((&b"\n"[..], From::Filter("other filter".to_string(), "odd path".to_string()))));
    }

    #[test]
    fn statement() {
        assert_eq!(parse_statement(&b"create 'test' from 'test'\n"[..]),
            Ok((&b""[..], Stmt{
                kind: Directive::Create,
                path: "test".to_string(),
                from: From::File("test".to_string())})));
        assert_eq!(parse_statement(&b"append 'test' literal 'test\n'\n"[..]),
            Ok((&b""[..], Stmt{
                kind: Directive::Append,
                path: "test".to_string(),
                from: From::Literal("test\n".to_string())})));
    }
}
