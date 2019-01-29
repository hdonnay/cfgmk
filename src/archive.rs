use std::collections::HashMap;
use std::io::{Result, Write};
use std::path::PathBuf;

extern crate tar;

type TarBuilder<'a> = tar::Builder<Box<dyn Write + 'a>>;

pub struct Builder<'a> {
    m: HashMap<PathBuf, Vec<u8>>,
    b: TarBuilder<'a>,
}

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
    fn create(&self) -> Result<()> {
        unimplemented!()
    }

    fn append(&mut self, to: PathBuf, data: Vec<u8>) -> Result<()> {
        if !self.m.contains_key(&to) {
            self.m.insert(to, data);
        } else {
            let buf = self.m.get_mut(&to).unwrap();
            buf.extend(&data);
        }
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
