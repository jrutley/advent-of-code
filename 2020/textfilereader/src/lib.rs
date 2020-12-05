use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn read_file_by_line(filename: &str) -> Vec::<String> {
    let file = File::open(filename).unwrap();
    get_inputs(file)
}

fn get_inputs(f: File) -> Vec::<String> {
    let lines = BufReader::new(f).lines().map(|line| line.unwrap());
    lines.collect()
}