use std::io::{BufRead};
use std::fs::File;

fn main() {
    let filename = "input.txt";

    let file = File::open(filename).unwrap();
    let lines = get_inputs(file);
    for x in 0..lines.len() {
        for y in x..lines.len() {
            if lines[x] + lines[y] == 2020 {
                println!("{}", x * y);
                return;
            }
        }
    }
}

fn get_inputs(f: File) -> Vec::<i32> {
    let lines = std::io::BufReader::new(f).lines().map(|line| line.unwrap().parse::<i32>().unwrap());
    lines.collect()
}