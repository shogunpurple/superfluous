extern crate dirs;

use std::{fs};


fn main() {
    println!("Hello, world!");
    scan_files()
}

fn scan_files() {
    // TODO: update this to use pattern matching instead
    let home_dir = dirs::home_dir().expect("Failed reading home dir");

    println!("{}", home_dir.display());

    for entry in fs::read_dir(home_dir).unwrap() {
        let path = entry.unwrap().path();
        let metadata = fs::metadata(&path);

        println!("{}", metadata)
    }
}