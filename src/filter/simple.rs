use crate::filter::Filter;
use crate::parser::From;
use std::collections::HashMap;

pub struct Builder {
    map: HashMap<String, String>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            map: HashMap::new(),
        }
    }

    pub fn mapper(mut self, m: HashMap<String, String>) -> Builder {
        for (k, v) in m.iter() {
            self.map.insert(k.to_string(), v.to_string());
        }
        self
    }

    pub fn finish(self) -> Filter {
        Box::new(|f| match f {
            _ => unimplemented!(),
            From::Literal(lit) => unimplemented!(),
            From::File(name) => unimplemented!(),
        })
    }
}
