extern crate dirs;

use std::{fs, path, time};

// 100MB
const FILE_SIZE_LIMIT_MB: u64 = 1000000 * 100;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct File {
    path: String,
    size: u64,
}


fn main() {
    let start_time = time::SystemTime::now();

    println!("Running superfluous...");
    let home_dir = dirs::home_dir().expect("Failed reading home dir");
    let mut large_files: Vec<File> = vec![];

    scan_files(&home_dir, &mut large_files);

    large_files.sort_by(|a, b| b.size.cmp(&a.size));
    println!("Superfluous scan complete! The large files on your machine are as follows:");

    for file in large_files {
        println!("File: {}, Size: {}MB", file.path, file.size);
    }

    println!("Superfluous scan took {}ms", start_time.elapsed().unwrap().as_millis())
}

fn in_mb(bytes: u64) -> u64 {
    bytes / 1000000
}

fn scan_files(dir: &path::PathBuf, large_files: &mut Vec<File>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        match fs::metadata(&path) {
            Ok(metadata) => {
                match metadata.is_dir() {
                    true => scan_files(&path, large_files),
                    false => {
                        if metadata.len() > FILE_SIZE_LIMIT_MB {
                            large_files.push(
                                File { path: path.display().to_string(), size: in_mb(metadata.len()) }
                            )
                        }
                    }
                }
            },
            _ => {}
        }
    }
}