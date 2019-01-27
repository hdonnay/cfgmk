use std::path::PathBuf;

extern crate walkdir;

use walkdir::{DirEntry, WalkDir};

/// Find_rules returns a Vec of paths to Rules files rooted at root.
pub fn find_rules(root: PathBuf) -> Vec<PathBuf> {
    let r = WalkDir::new(root);
    let mut acc = Vec::new();
    for e in r.into_iter()
        .filter_entry(filter_walk).into_iter() {
        let e = e.unwrap();
        if !e.file_type().is_dir() {
            acc.push(e.path().into())
        }
    }
    acc
}

fn filter_walk(e: &DirEntry) -> bool {
    !e.file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false) ||
    (!e.file_type().is_dir() &&
        e.file_name() == "Rules")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find() {
        let r = PathBuf::from("tests/walk");
        let rs = find_rules(r);
        assert_eq!(rs, [
            PathBuf::from("tests/walk/Rules"),
            PathBuf::from("tests/walk/sub/Rules"),
        ]);
    }
}
