use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Cursor, Read};

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

use crate::filter;

#[derive(Parser)]
#[grammar = "parser/rulesfile.pest"]
struct RulesParser;

pub struct Rulesfile<'a> {
    f: &'a filter::FilterMap,
    vec: Vec<Stmt>,
}

// TODO Use proptest crate.

// TODO Add Rulesfile conditional based on hostname, as `if 'name' ...`.

impl<'a> Rulesfile<'a> {
    pub fn new(f: &'a filter::FilterMap, path: &str) -> Result<Rulesfile<'a>, Error<Rule>> {
        let rp = RulesParser::parse(Rule::file, path)?;

        let mut stmts = Vec::new();
        for pair in rp {
            match pair.as_rule() {
                Rule::statement => {
                    let mut s = pair.into_inner();
                    let kind = s.next();
                    let path = s.next().unwrap();
                    let from = s.next().unwrap().into_inner().next();
                    stmts.push(Stmt {
                        cond: None,
                        kind: parse_kind(kind).unwrap(),
                        path: unescape(path.as_str()),
                        from: parse_from(from).unwrap(),
                    });
                }
                Rule::EOI => break,
                _ => unimplemented!(),
            }
        }

        Ok(Rulesfile { f, vec: stmts })
    }
}

impl<'a> Iterator for Rulesfile<'a> {
    type Item = Stmt;

    fn next(&mut self) -> Option<Self::Item> {
        match self.vec.pop() {
            Some(s) => Some(match s.from {
                From::Filter(_, _) => Stmt {
                    cond: s.cond,
                    kind: s.kind,
                    path: s.path,
                    from: self.f.reify(&s.from),
                },
                _ => s,
            }),
            None => None,
        }
    }
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
                From::Literal(Vec::from(unescape(n)))
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

#[derive(Debug, PartialEq, Clone)]
pub enum Directive {
    Create,
    Append,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Stmt {
    pub cond: Option<Conditional>,
    pub kind: Directive,
    pub path: String,
    pub from: From,
}

#[derive(Debug, PartialEq, Clone)]
pub enum From {
    File(String),
    Filter(String, Box<From>),
    Literal(Vec<u8>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Conditional {
    If(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {}
}
