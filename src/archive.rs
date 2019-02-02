use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Result, Write};
use std::path::PathBuf;
use std::time::SystemTime;

use crate::parser;

extern crate chrono;
extern crate tar;

use chrono::{DateTime, Utc};

type TarBuilder<'a> = tar::Builder<Box<dyn Write + 'a>>;

pub struct Builder<'a> {
    m: HashMap<PathBuf, Vec<u8>>,
    b: TarBuilder<'a>,
}

pub struct Entry<'a> {
    rd: Box<dyn Read + 'a>,
    hdr: tar::Header,
}

impl<'a> Entry<'a> {
    pub fn new<T: Read + 'a>(r: T, t: DateTime<Utc>) -> Entry<'a> {
        let mut hdr = tar::Header::new_ustar();
        hdr.set_mtime(t.timestamp() as u64);
        Entry {
            rd: Box::new(r) as Box<dyn Read + 'a>,
            hdr,
        }
    }

    pub fn from() {
        unimplemented!()
    }
}

impl<'a> Builder<'a> {
    pub fn new<T: Write + 'a>(w: T) -> Builder<'a> {
        let inner = Box::new(w) as Box<dyn Write + 'a>;
        let mut ar = tar::Builder::new(inner);
        ar.follow_symlinks(false);
        Builder {
            m: HashMap::new(),
            b: ar,
        }
    }

    pub fn create(&mut self, to: PathBuf, from: parser::From) -> Result<()> {
        match from {
            parser::From::File(name) => {
                let mut f = File::open(name).unwrap();
                self.b.append_file(to, &mut f)
            }
            parser::From::Literal(lit) => {
                let now: DateTime<Utc> = DateTime::from(SystemTime::now());
                let mut hdr = tar::Header::new_ustar();
                hdr.set_mtime(now.timestamp() as u64);
                self.b.append_data(&mut hdr, to, Cursor::new(lit))
            }
            _ => unimplemented!(),
        }
    }

    pub fn append(&mut self, to: PathBuf, from: parser::From) {
        self.m
            .entry(to)
            .and_modify(|v| {
                v.extend(match from.clone() {
                    parser::From::Literal(lit) => lit,
                    parser::From::File(name) => {
                        let mut f = File::open(name).unwrap();
                        let mut b = Vec::new();
                        debug_assert_ne!(f.read_to_end(&mut b).unwrap(), 0);
                        b
                    }
                    parser::From::Filter(which, inner) => panic!(),
                })
            })
            .or_insert_with(|| match from {
                parser::From::Literal(lit) => lit,
                parser::From::File(name) => {
                    let mut f = File::open(name).unwrap();
                    let mut b = Vec::new();
                    debug_assert_ne!(f.read_to_end(&mut b).unwrap(), 0);
                    b
                }
                parser::From::Filter(which, inner) => panic!(),
            });
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
