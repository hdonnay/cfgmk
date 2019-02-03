use crate::filter::Filter;
use crate::parser::From;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str;

extern crate trimmer;
use trimmer::{Context, Parser, Template};

pub struct Builder {
    map: HashMap<String, String>,
    p: Parser,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            map: HashMap::new(),
            p: Parser::new(),
        }
    }

    pub fn mapper(mut self, m: &HashMap<String, String>) -> Builder {
        for (k, v) in m.iter() {
            self.map.insert(k.to_string(), v.to_string());
        }
        self
    }

    pub fn finish(self) -> Filter {
        Box::new(move |f| {
            let mut ctx = Context::new();
            ctx.set("data", &self.map);
            match f {
                _ => unimplemented!(),
                From::Literal(lit) => {
                    let tmpl = str::from_utf8(&lit).unwrap();
                    let t = self.p.parse(tmpl).unwrap();
                    let out = t.render(&ctx).unwrap();
                    From::Literal(Vec::from(out))
                }
                From::File(name) => {
                    let mut buf = Vec::new();
                    let mut f = File::open(name).unwrap();
                    f.read_to_end(&mut buf).unwrap();
                    let tmpl = str::from_utf8(&buf).unwrap();
                    let t = self.p.parse(tmpl).unwrap();
                    let out = t.render(&ctx).unwrap();
                    From::Literal(Vec::from(out))
                }
            }
        })
    }
}
