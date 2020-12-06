fn main() {
    let input = textfilereader::read_file_by_line("input.txt");
    let passports = build_passports(input);
    let valid: Vec<&Passport> = passports.iter().filter(|p| p.is_valid()).collect();
    println!("{}", valid.len());
}

#[derive(Debug, Default)]
struct Passport {
    ecl: String,
    pid: String,
    eyr: String,
    hcl: String,
    byr: String,
    iyr: String,
    cid: String,
    hgt: String
}

impl Passport {
    fn update(&mut self, field: &str, value: &str) {
        let v = value.to_owned();
        match field {
            "ecl" => self.ecl = v,
            "pid" => self.pid = v,
            "eyr" => self.eyr = v,
            "hcl" => self.hcl = v,
            "byr" => self.byr = v,
            "iyr" => self.iyr = v,
            "cid" => self.cid = v,
            "hgt" => self.hgt = v,
            _ => panic!("Couldn't parse field")
        }
    }
    fn is_valid(&self) -> bool {
        self.ecl.len() > 0 && self.pid.len() > 0 && self.eyr.len() > 0 && self.hcl.len() > 0 && self.byr.len() > 0 && self.iyr.len() > 0
        && self.hgt.len() > 0
        // && self.cid.len() > 0 // cid is optional
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
        let fields = line
            .split(' ')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|kv| kv.to_owned())
            .collect::<Vec<String>>();

        for kv in fields {
            let foo: Vec<&str> = kv.split(':').collect();
            current_passport.update(foo[0], foo[1]);
        }
    }
    passports.push(current_passport);

    return passports
}