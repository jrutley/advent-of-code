use std::io::{BufRead};
use std::fs::File;

fn main() {
    let filename = "input.txt";

    let file = File::open(filename).unwrap();
    let lines = get_inputs(file);
    for line in lines {
        println!("{}", line);
    }
}

fn get_inputs(f: File) -> Vec::<String> {
    let lines = std::io::BufReader::new(f).lines().map(|line| line.unwrap());
    lines.collect()
}