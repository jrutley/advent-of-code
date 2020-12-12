fn main() {
    let file_data: Vec<i64> = textfilereader::read_file_by_line("input.txt").into_iter().map(|n| n.parse::<i64>().unwrap()).collect();
    let window_size = 25;
    let invalid_number = get_invalid_number(file_data, window_size);
    if invalid_number == None { println!("Couldn't find anomalous number"); return; }
    let target_number = invalid_number.unwrap();
    println!("{} is the first non-matching", target_number);
}

fn get_invalid_number(file_data: Vec<i64>, window_size: usize) -> Option<i64>{
    for numbers in file_data.windows(window_size + 1) {
        let target_number = numbers[window_size];
        if !did_match(target_number, &numbers[0..window_size], window_size-1) {
            return Some(target_number);
        }
    }
    None
}

fn did_match(target_number: i64, numbers: &[i64], window_size: usize) -> bool {
    for x in 0..=window_size {
        for y in x+1..=window_size {
            if numbers[x] + numbers[y] == target_number { return true; }
        }
    }
    false
}