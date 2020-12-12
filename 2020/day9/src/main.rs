fn main() {
    let file_data: Vec<i64> = textfilereader::read_file_by_line("input.txt").into_iter().map(|n| n.parse::<i64>().unwrap()).collect();
    for numbers in file_data.windows(26) {
        let target_number = numbers[25];
        if !did_match(target_number, &numbers[0..24]) {
            println!("{} is the first non-matching", target_number);
            return 
        }
    }
}

fn did_match(target_number: i64, numbers: &[i64]) -> bool {
    for x in 0..24 {
        for y in x+1..24 {
            if numbers[x] + numbers[y] == target_number { return true; }
        }
    }
    false
}