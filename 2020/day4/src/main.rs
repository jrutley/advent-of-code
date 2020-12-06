#[macro_use]
extern crate lazy_static;

use regex::Regex;
use validator::{Validate}; //, ValidationError};

fn main() {
    let input = textfilereader::read_file_by_line("input.txt");
    let passports = build_passports(input);
    let valid: Vec<&Passport> = passports.iter().filter(|p| p.is_valid()).collect();
    println!("{}", valid.len());
}

#[derive(Debug, Default, Validate)]
struct Passport {
    #[validate(regex = "RE_EYE_COLOUR")]
    #[validate(regex(path = "RE_EYE_COLOUR"))]
    ecl: String,
    #[validate(regex = "RE_PID")]
    #[validate(regex(path = "RE_PID"))]
    pid: String,
    #[validate(range(min=2020,max=2030))]
    eyr: i32,
    #[validate(regex = "RE_HCL")]
    #[validate(regex(path = "RE_HCL"))]
    hcl: String,
    #[validate(range(min=1920,max=2002))]
    byr: i32,
    #[validate(range(min=2010,max=2020))]
    iyr: i32,
    cid: String,
    #[validate(regex = "RE_VALID_HEIGHT")]
    #[validate(regex(path = "RE_VALID_HEIGHT"))]
    hgt: String
}

lazy_static! {
    static ref RE_HCL: Regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
    static ref RE_EYE_COLOUR: Regex = Regex::new(r"^amb$|^blu$|^brn$|^gry$|^grn$|^hzl$|^oth$").unwrap();
    static ref RE_VALID_HEIGHT: Regex = Regex::new(r"^1[5-8]\dcm$|^19[0-3]cm$|^59in$|^6\din$|^7[0-6]in$").unwrap();
}

impl Passport {
    fn update(&mut self, field: &str, value: &str) {
        let v = value.to_owned();
        match field {
            "ecl" => self.ecl = v,
            "pid" => self.pid = v,
            "eyr" => self.eyr = value.parse::<i32>().unwrap(),
            "hcl" => self.hcl = v,
            "byr" => self.byr = value.parse::<i32>().unwrap(),
            "iyr" => self.iyr = value.parse::<i32>().unwrap(),
            "cid" => self.cid = v,
            "hgt" => self.hgt = v,
            _ => panic!("Couldn't parse field")
        }
    }
    fn is_valid(&self) -> bool {
        match self.validate() {
            Ok(_) => true,
            Err(_) => false
        }
    }
}

fn build_passports(input: Vec<String>) -> Vec<Passport> {
    let mut passports = vec!();

    let mut current_passport = Passport::default();
    for line in input {
        if line.len() == 0 {
            passports.push(current_passport);
            current_passport = Passport::default();
            continue;
        }
        let fields: Vec<&str> = line.split(' ').collect();

        for keyvalue in fields {
            let kv: Vec<&str> = keyvalue.split(':').collect();
            current_passport.update(kv[0], kv[1]);
        }
    }
    passports.push(current_passport);

    return passports
}