use regex::Regex;
use std::io::{BufRead};
use std::fs::File;

struct PasswordModel {
    min: usize,
    max: usize,
    character: char,
    password: String
}

fn main() {
    let filename = "input.txt";

    let file = File::open(filename).unwrap();
    let lines = get_inputs(file);

    let re = Regex::new(r"(\d+)-(\d+) (\w): (.*)").unwrap();
    let my_passwords = lines.iter().map(|line| {
        let captures = re.captures(&line).unwrap();
        PasswordModel {
            min: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            max: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            character: captures.get(3).unwrap().as_str().chars().nth(0).unwrap(),
            password: captures.get(4).unwrap().as_str().to_owned(),
        }
    }).collect::<Vec<PasswordModel>>();

    let password_count = valid_passwords(my_passwords).len();
    println!("{} valid passwords", password_count);
}

fn valid_passwords(passwords: Vec<PasswordModel>) -> Vec<PasswordModel> {
    passwords.into_iter().filter(|p| {
        let policy_char_len = p.password.chars().filter(|&c| c == p.character).collect::<Vec<char>>().len();
        policy_char_len >= p.min && policy_char_len <= p.max
    }).collect()
}

fn get_inputs(f: File) -> Vec::<String> {
    let lines = std::io::BufReader::new(f).lines().map(|line| line.unwrap());
    lines.collect()
}