use std::fs::File;
use std::io::Read;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

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
                let kind = s.next();
                let path = s.next().unwrap();
                let from = s.next().unwrap().into_inner().next();
                stmts.push(Stmt {
                    kind: parse_kind(kind).unwrap(),
                    path: unescape(path.as_str()),
                    from: parse_from(from).unwrap(),
                });
            }
            Rule::EOI => break,
            _ => unimplemented!(),
        }
    }

    Ok(stmts)
}

fn unescape(s: &str) -> String {
    let l = s.len();
    if l < 2 {
        panic!("string too short");
    }
    let s = &s[1..l - 1];
    s.to_string().replace("''", "'")
}

fn parse_kind(pair: Option<Pair<Rule>>) -> Option<Directive> {
    match pair {
        None => None,
        Some(p) => Some(match p.as_str() {
            "create" => Directive::Create,
            "append" => Directive::Append,
            _ => unreachable!(),
        }),
    }
}

fn parse_from(pair: Option<Pair<Rule>>) -> Option<From> {
    match pair {
        None => None,
        Some(p) => Some(match p.as_rule() {
            Rule::fromFile => {
                let n = p.into_inner().as_str();
                From::File(unescape(n))
            }
            Rule::fromLiteral => {
                let n = p.into_inner().as_str();
                From::Literal(unescape(n))
            }
            Rule::fromFilter => {
                let mut n = p.into_inner();
                let which = unescape(n.next().unwrap().as_str());
                let inner = Box::new(parse_from(n.next()).unwrap());
                From::Filter(which, inner)
            }
            _ => unreachable!(),
        }),
    }
}

#[derive(Debug, PartialEq)]
pub enum Directive {
    Create,
    Append,
}

#[derive(Debug, PartialEq)]
pub struct Stmt {
    pub kind: Directive,
    pub path: String,
    pub from: From,
}

#[derive(Debug, PartialEq)]
pub enum From {
    File(String),
    Filter(String, Box<From>),
    Literal(String),
}

impl From {
    pub fn realize(&self) -> Result<Vec<u8>, ()> {
        match self {
            From::Literal(e) => Ok(Vec::from(e.as_str())),
            From::File(n) => {
                let mut buf = Vec::new();
                let mut f = File::open(n).unwrap();
                f.read_to_end(&mut buf).unwrap();
                Ok(buf)
            },
            From::Filter(which, inner) =>  {
                eprintln!("which: {}, inner: {:?}", which, inner);
                unimplemented!()
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let rf = include_str!("../tests/parser/basic");;
        let want = vec![Stmt {
            kind: Directive::Create,
            path: "test".to_string(),
            from: From::File("/dev/null".to_string()),
        }];
        let res = rulesfile(rf);
        match res {
            Err(e) => {
                panic!("{}", e);
            }
            Ok(got) => {
                assert_eq!(got, want);
            }
        }
    }
}
