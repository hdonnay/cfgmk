use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::str;
use std::time::SystemTime;

mod archive;
mod filter;
mod parser;
mod walk;

#[macro_use]
extern crate structopt;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate chrono;

use chrono::{DateTime, Utc};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Output file, default stdout.
    #[structopt(short, parse(from_os_str))]
    output: Option<PathBuf>,
    /// Root of the tree to compile.
    #[structopt(parse(from_os_str))]
    root: Option<PathBuf>,
}

fn main() {
    let filters = filter::Builder::new()
        .add("cat", Box::new(filter::cat::filter))
        .finish();
    let opt = Opt::from_args();

    let stdout = io::stdout();
    let mut ar = match opt.output {
        None => archive::Builder::new(stdout.lock()),
        Some(n) => {
            let f = File::create(n).unwrap();
            archive::Builder::new(f)
        }
    };

    let root = match opt.root {
        None => PathBuf::from("."),
        Some(n) => n,
    };

    let mut fs = Vec::new();
    for e in walk::find_rules(root) {
        let f = File::open(e).unwrap();
        fs.push(f);
    }

    let mut bs = Vec::new();
    for mut f in fs {
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        bs.push(buf);
    }

    let mut stmts = Vec::new();
    for buf in bs {
        let name = str::from_utf8(&buf).unwrap();
        let r = parser::Rulesfile::new(&filters, name).unwrap();
        for s in r {
            stmts.push(s);
        }
    }

    let mut unseen = HashSet::new();
    for s in stmts {
        if !unseen.insert(s.path.clone()) {
            eprintln!("overwriting entry: {}", s.path);
        }
        match s.kind {
            parser::Directive::Create => ar.create(s.path.into(), s.from),
            _ => unimplemented!(),
        }
        .unwrap();
    }
}
