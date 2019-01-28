use std::fs;

use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "parser/rulesfile.pest"]
struct RulesParser;

pub fn rulesfile(file: &str) -> Result<Vec<Stmt>, Error<Rule>> {
    let rp = RulesParser::parse(Rule::file, file)?;

    let mut stmts = Vec::new();
    for pair in rp {
        match pair.as_rule() {
            Rule::statement => {
                let mut s = pair.into_inner();
                let kind = s.next().unwrap();
                let path = s.next().unwrap();
                let from = s.next().unwrap().into_inner().next().unwrap();
                stmts.push(Stmt{
                    kind: match kind.as_str() {
                        "create" => Directive::Create,
                        "append" => Directive::Append,
                        _ => panic!("wat")
                    },
                    path: path.as_str().to_string(),
                    from: match from.as_rule() {
                        Rule::fromFile => {
                            let n = from.into_inner().as_str();
                            From::File(n.to_string())
                        },
                        Rule::fromLiteral => unimplemented!(),
                        Rule::fromFilter => unimplemented!(),
                        _ => panic!("wat"),
                    },
                });
            },
            Rule::EOI => break,
            _ => unimplemented!(),
        }
        //stmts.push(Stmt{ });
    }

    Ok(stmts)
}

fn unescape(s: &str) -> String {
    unimplemented!()
}

pub use nom::{IResult, Needed};
use nom::{is_space, newline};

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

pub fn statements(i: &[u8]) -> IResult<&[u8], Vec<Stmt>> {
    do_parse!(i, s: many0!(parse_statement) >> eof!() >> (s))
}

named!(parse_statement<Stmt>, do_parse!(
    kind: directive >>
    take_while1!(is_space) >>
    path: string >>
    take_while1!(is_space) >>
    from: alt!(fromFile | fromFilter | fromLiteral) >>
    call!(newline) >>
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

named!(sq, delimited!(tag!("'"), take_until!("'"), tag!("'")));
named!(run_sq< Vec<String> >, many1!(map!(sq, |item| {String::from_utf8_lossy(item).to_string()})));
named!(string< String >, map!(run_sq, |v| { v.join("'") }));

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
mod test {
    use super::*;
    static NIL: &[u8] = b"";
    static LF: &[u8] = b"\n";

    #[test]
    fn singlequoting() {
        assert_eq!(sq(&b"'test'"[..]), Ok((NIL, &b"test"[..])));
        let extra = &b"''"[..];
        assert_eq!(sq(&b"'one tick: '''"[..]), Ok((extra, &b"one tick: "[..])));
    }

    #[test]
    fn string_input() {
        let input = Vec::from("'test'\n");
        assert_eq!(string(&input), Ok((LF, String::from("test"))));
    }

    #[test]
    fn literal() {
        assert_eq!(fromLiteral(&b"literal 'test'\n"[..]),
            Ok((LF, From::Literal("test".to_string()))));
        assert_eq!(fromLiteral(&b"literal 'devil''s advocate'\n"[..]),
            Ok((LF, From::Literal("devil's advocate".to_string()))));
    }

    #[test]
    fn file() {
        assert_eq!(fromFile(&b"from 'test'\n"[..]),
            Ok((LF, From::File("test".to_string()))));
        assert_eq!(fromFile(&b"from '/dev/null'\n"[..]),
            Ok((LF, From::File("/dev/null".to_string()))));
        assert_eq!(fromFile(&b"from 'odd path'\n"[..]),
            Ok((LF, From::File("odd path".to_string()))));
    }

    #[test]
    fn filter() {
        assert_eq!(fromFilter(&b"filter 'cat' '/dev/null'\n"[..]),
            Ok((LF, From::Filter("cat".to_string(), "/dev/null".to_string()))));
        assert_eq!(fromFilter(&b"filter 'simple' '/dev/null'\n"[..]),
            Ok((LF, From::Filter("simple".to_string(), "/dev/null".to_string()))));
        assert_eq!(fromFilter(&b"filter 'other filter' 'odd path'\n"[..]),
            Ok((LF, From::Filter("other filter".to_string(), "odd path".to_string()))));
    }

    #[test]
    fn statement() {
        assert_eq!(parse_statement(&b"create 'test' from 'test'\n"[..]),
            Ok((NIL, Stmt{
                kind: Directive::Create,
                path: "test".to_string(),
                from: From::File("test".to_string())})));
        assert_eq!(parse_statement(&b"append 'test' literal 'test\n'\n"[..]),
            Ok((NIL, Stmt{
                kind: Directive::Append,
                path: "test".to_string(),
                from: From::Literal("test\n".to_string())})));
    }

    #[test]
    fn pest() {
        let rf = fs::read_to_string("tests/walk/Rules").unwrap();
        match rulesfile(&rf) {
            Err(e) => println!("{:?}", e),
            Ok(x) => println!("{:?}", x),
        }
    }
}
