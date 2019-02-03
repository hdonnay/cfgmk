use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
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
    #[structopt(parse(from_os_str), default_value = ".")]
    root: PathBuf,
    /// Machine name.
    #[structopt(short)]
    name: Option<String>,
}

type VarMap = HashMap<String, HashMap<String, String>>;

fn main() {
    let opt = Opt::from_args();
    let name = match opt.name {
        None => {
            let mach_id = fs::read_to_string("/etc/machine-id")
                .unwrap_or("00000000000000000000000000000000".to_string());

            let mut mapfile = opt.root.clone();
            mapfile.push("host.yaml");
            let mapfile = File::open(mapfile).unwrap_or_else(|_| {
                eprintln!("unable to open host.yaml, no variables loaded");
                File::open("/dev/null").unwrap()
            });
            let mut name= String::from("");
            if let Ok(m) = serde_yaml::from_reader::<File, HashMap<String, String> >(mapfile) {
                name = m.get(&mach_id).unwrap().to_owned();
            }
            name
        }
        Some(name) => name,
    };

    let mut varfile = opt.root.clone();
    varfile.push("vars.yaml");
    let varfile = File::open(varfile).unwrap_or_else(|_| File::open("/dev/null").unwrap());
    let varmap: VarMap = match serde_yaml::from_reader(varfile) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("unable to open vars.yaml, no variables loaded: {:?}", e);
            VarMap::new()
        },
    };
    let mut filtermap: &HashMap<String, String> = &HashMap::new();
    if name != "" && varmap.len() != 0 {
        filtermap = varmap.get(&name).unwrap();
    };

    let simple = filter::simple::Builder::new()
        .mapper(filtermap)
        .finish();
    let filters = filter::Builder::new()
        .add("cat", Box::new(filter::cat::filter))
        .add("simple", simple)
        .finish();

    let stdout = io::stdout();
    let mut ar = match opt.output {
        None => archive::Builder::new(stdout.lock()),
        Some(n) => {
            let f = File::create(n).unwrap();
            archive::Builder::new(f)
        }
    };

    let mut fs = Vec::new();
    for e in walk::find_rules(opt.root) {
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
