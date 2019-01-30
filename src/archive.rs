use std::collections::HashMap;
use std::io::{Result, Write};
use std::path::PathBuf;

extern crate tar;

type TarBuilder<'a> = tar::Builder<Box<dyn Write + 'a>>;

pub struct Builder<'a> {
    m: HashMap<PathBuf, Vec<u8>>,
    b: TarBuilder<'a>,
}

pub struct Entry {}

pub fn new<'a, T: Write + 'a>(w: T) -> Builder<'a> {
    let inner = Box::new(w) as Box<dyn Write + 'a>;
    let mut ar = tar::Builder::new(inner);
    ar.follow_symlinks(false);
    Builder {
        m: HashMap::new(),
        b: ar,
    }
}

impl<'a> Builder<'a> {
    fn create<T: Write>(&self, to: PathBuf, from: T) -> Result<()> {
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

        let mut b = new(backing);
        b.append(key, Vec::from(&want[..]));

        let key = PathBuf::from("testfile");
        let got = b.m.get(&key);
        assert_ne!(got, None);
        let got = got.unwrap();
        assert_eq!(want, got.as_slice());
    }
}
