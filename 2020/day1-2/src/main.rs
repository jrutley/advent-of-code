use std::io::{BufRead};
use std::fs::File;

fn main() {
    let filename = "input.txt";

    let file = File::open(filename).unwrap();
    let lines = get_inputs(file);
    for x in 0..lines.len() {
        let x_value = lines[x];
        for y in x..lines.len() {
            let y_value = lines[y];
            for z in y..lines.len() {
                let z_value = lines[z];
                if x_value + y_value + z_value == 2020 {
                    println!("{}", x_value * y_value * z_value);
                    return;
                }
            }
        }
    }
}

fn get_inputs(f: File) -> Vec::<i32> {
    let lines = std::io::BufReader::new(f).lines().map(|line| line.unwrap().parse::<i32>().unwrap());
    lines.collect()
}