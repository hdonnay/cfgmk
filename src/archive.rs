use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result, Write};
use std::path::PathBuf;

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

    pub fn create_file(&mut self, to: PathBuf, from: &mut File) -> Result<()> {
        self.b.append_file(to, from)
    }

    pub fn create_filter(&mut self, to: PathBuf, from: (), at: DateTime<Utc>) -> Result<()> {
        unimplemented!()
    }

    pub fn create_literal(&mut self, to: PathBuf, from: Vec<u8>, at: DateTime<Utc>) -> Result<()> {
        unimplemented!()
    }

    pub fn append(&mut self, to: PathBuf, data: Vec<u8>) {
        self.m
            .entry(to)
            .and_modify(|v| v.extend(&data))
            .or_insert(data);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn append() {
        let backing: Vec<u8> = Vec::new();
        let key = PathBuf::from("testfile");
        let want = b"testcontent";

        let mut b = Builder::new(backing);
        b.append(key, Vec::from(&want[..]));

        let key = PathBuf::from("testfile");
        let got = b.m.get(&key);
        assert_ne!(got, None);
        let got = got.unwrap();
        assert_eq!(want, got.as_slice());
    }
}
