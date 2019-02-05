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
            parser::From::Filter(which, inner) => {
                unreachable!("should only have File or Literal here")
            }
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
                    parser::From::Filter(which, inner) => {
                        unreachable!("should only have File or Literal here")
                    }
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
                parser::From::Filter(which, inner) => {
                    unreachable!("should only have File or Literal here")
                }
            });
    }

    pub fn finish(mut self) -> Result<()> {
        self.b.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::From;

    #[test]
    fn creation() {
        let mut buf = Vec::new();
        let mut w = Cursor::new(buf);
        let mut ar = Builder::new(w);

        let f = From::Literal(Vec::from("test"));
        let p = PathBuf::from("test/test");

        assert_eq!((), ar.create(p, f).unwrap());
    }

    /*
    #[test]
    fn appending() {
        unimplemented!()
    }
    */
}
