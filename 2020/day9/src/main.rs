fn main() {
    let file_data: Vec<i64> = textfilereader::read_file_by_line("input.txt").into_iter().map(|n| n.parse::<i64>().unwrap()).collect();
    let window_size = 25;
    let invalid_number_position = get_invalid_number_position(&file_data, window_size);
    if invalid_number_position == None { println!("Couldn't find anomalous number"); return; }

    let target_number = file_data[invalid_number_position.unwrap()];

    println!("Target number is {}", target_number);
    let mut check_window_size = 2;
    while check_window_size < 50 {
        if let Some(result) = find_contiguous_set(target_number, &file_data, check_window_size) {
            println!("Sum of first and last numbers: {}", result);
            return;
        }
        check_window_size += 1;
    }
}

fn find_contiguous_set(target: i64, file_data: &Vec<i64>, window_size: usize)-> std::option::Option<i64> {
    for items in file_data.windows(window_size) {
        let sum: i64 = items.iter().sum();
        if sum == target {
            let smallest = items.iter().min().unwrap();
            let largest = items.iter().max().unwrap();
            return Some(smallest + largest);}
    }
    None
}

fn get_invalid_number_position(file_data: &Vec<i64>, window_size: usize) -> Option<usize>{
    let mut position = window_size;
    for numbers in file_data.windows(window_size + 1) {
        let target_number = numbers[window_size];
        if !did_match(target_number, &numbers[0..window_size], window_size-1) {
            return Some(position);
        }
        position += 1;
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