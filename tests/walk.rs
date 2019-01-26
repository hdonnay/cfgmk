use std::path::PathBuf;

use cfgmk::walk;

#[test]
fn find() {
    let r = PathBuf::from("tests/walk");
    let rs = walk::find_rules(r);
    assert_eq!(rs, [
        PathBuf::from("tests/walk/Rules"),
        PathBuf::from("tests/walk/sub/Rules"),
    ]);
}
