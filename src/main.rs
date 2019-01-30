use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::str;
use structopt::StructOpt;

mod archive;
mod parser;
mod walk;

#[macro_use]
extern crate structopt;
extern crate pest;
#[macro_use]
extern crate pest_derive;

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
    let opt = Opt::from_args();

    let f = match opt.output {
        None => File::open("/dev/stdout"),
        Some(n) => File::create(n),
    }
    .unwrap();
    let ar = archive::Builder::new(f);

    let root = match opt.root {
        None => PathBuf::from("."),
        Some(n) => n,
    };

    let mut fs = Vec::new();
    for e in walk::find_rules(root) {
        let mut f = File::open(e).unwrap();
        fs.push(f);
    }
    println!("{:?}", fs);

    let mut bs = Vec::new();
    for mut f in fs {
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        bs.push(buf);
    }

    let mut stmts = Vec::new();
    for buf in bs {
        match parser::rulesfile(str::from_utf8(&buf).unwrap()) {
            Ok(v) => stmts.extend(v),
            Err(e) => println!("{:?}", e),
        }
    }
    println!("{:?}", stmts);
}
