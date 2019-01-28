use std::collections::HashMap;
use std::path::PathBuf;
use std::io::{Write, Result};

extern crate tar;

pub struct Builder<T: Write> {
	m: HashMap<PathBuf, Vec<u8>>,
	b: tar::Builder<T>,
}

pub fn new<T: Write>(inner: T) -> Builder<T> {
	let mut ar = tar::Builder::new(inner);
    ar.follow_symlinks(false);
	Builder{
		m: HashMap::new(),
		b: ar,
	}
}

impl<T: Write> Builder<T> {
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
