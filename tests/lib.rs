#![feature(convert, path_ext)]

extern crate tape;
extern crate temporary;

use std::path::PathBuf;
use temporary::Directory;

#[test]
fn extract() {
    use std::fs::PathExt;

    let foo = PathBuf::from("tests").join("fixtures").join("foo.tar");
    let directory = Directory::new("tape").unwrap();

    let archive = tape::open(&foo).unwrap();
    assert!(archive.extract(directory.path()).is_ok());

    let bar = directory.path().join("bar.txt");
    assert!(bar.exists());
}
