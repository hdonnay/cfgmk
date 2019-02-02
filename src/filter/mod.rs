use std::collections::HashMap;

use crate::parser::From;

pub mod cat;
pub mod simple;

pub type Filter = Box<Fn(&From) -> From>;

pub struct Builder {
    map: HashMap<String, Filter>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            map: HashMap::new(),
        }
    }

    pub fn add(mut self, name: &str, f: Filter) -> Builder {
        self.map.insert(String::from(name), f);
        self
    }

    pub fn finish(self) -> FilterMap {
        FilterMap(self.map)
    }
}

pub struct FilterMap(HashMap<String, Filter>);

impl FilterMap {
    pub fn reify(&self, from: &From) -> From {
        match from {
            From::Filter(name, inner) => {
                let inner = self.reify(inner);
                let map = &self.0;
                let f = map.get(name);
                match f {
                    None => inner,
                    Some(func) => func(&inner),
                }
            }
            From::Literal(lit) => From::Literal(lit.to_vec()),
            From::File(f) => From::File(f.to_string()),
        }
    }
}
