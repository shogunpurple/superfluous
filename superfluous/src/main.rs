extern crate dirs;

use std::{fs, path};


fn main() {
    println!("Hello, world!");
    // TODO: update this to use pattern matching instead
    let home_dir = dirs::home_dir().expect("Failed reading home dir");
    scan_files(&home_dir)
}

fn scan_files(dir: &path::PathBuf) {

    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let metadata = fs::metadata(&path).unwrap();

        match metadata.is_dir() {
            true => scan_files(&path),
            false => println!("File {}: Size = {}", path.display(), metadata.len())
        }
    }
}