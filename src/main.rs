use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str;
use std::time::SystemTime;

mod archive;
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
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let f = match opt.output {
        None => File::open("/dev/stdout"),
        Some(n) => File::create(n),
    }
    .unwrap();
    let mut ar = archive::Builder::new(f);

    let root = match opt.root {
        None => PathBuf::from("."),
        Some(n) => n,
    };

    let mut fs = Vec::new();
    for e in walk::find_rules(root) {
        let f = File::open(e).unwrap();
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

    let now: DateTime<Utc> = DateTime::from(SystemTime::now());
    for s in stmts {
        match s.from {
            parser::From::File(n) => {
                let mut f = File::open(n).unwrap();
                match s.kind {
                    parser::Directive::Create => ar.create_file(s.path.into(), &mut f).unwrap(),
                    parser::Directive::Append => {
                        let mut buf = Vec::new();
                        f.read_to_end(&mut buf).unwrap();
                        ar.append(s.path.into(), buf)
                    }
                }
            }
            parser::From::Literal(lit) => match s.kind {
                parser::Directive::Create => ar
                    .create_literal(s.path.into(), Vec::from(lit), now)
                    .unwrap(),
                parser::Directive::Append => ar.append(s.path.into(), Vec::from(lit)),
            },
            parser::From::Filter(which, from) => {
                // Figure out if we need to run code.
                match which {
                    _ => unimplemented!(),
                };
                match s.kind {
                    parser::Directive::Create => unimplemented!(),
                    parser::Directive::Append => unimplemented!(),
                }
            }
        }
    }
}
