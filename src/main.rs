use std::path::PathBuf;
use structopt::StructOpt;
use std::io;
//use std::fs::File;

mod parser;
mod walk;

#[macro_use]
extern crate nom;
#[macro_use]
extern crate structopt;
extern crate tar;
use tar::Builder;

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

    let stdout = io::stdout();
    let mut ar = Builder::new(stdout.lock());
    ar.follow_symlinks(false);

    let root = match opt.root {
        None => PathBuf::from("."),
        Some(n) => n,
    };

    for e in walk::find_rules(root) {
        println!("found: {:?}", e);
    }
}
