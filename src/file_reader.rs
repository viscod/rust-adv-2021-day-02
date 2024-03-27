use std::{fs, path::Path};

#[allow(dead_code)]
pub fn read_from_file<P: AsRef<Path>>(path: P) -> Vec<u16> {
    let file_input = fs::read_to_string(path).expect("unable to read a file");

    file_input
        .lines()
        .map(|s| s.parse::<u16>().unwrap())
        .collect()
}
