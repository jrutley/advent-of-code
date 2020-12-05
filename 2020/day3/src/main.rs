fn main() {
    let forest = build_forest();
    println!("Result: {}", forest)
}

fn build_forest() -> i32 {
    let lines = textfilereader::read_file_by_line("input.txt");

    let forest = replicate_forest_until_square(lines);
    let mut hit_counter = 0;
    let mut width_counter = 0;
    for height in &forest[..] {
        if height.chars().nth(width_counter) == Some('#') {
            hit_counter += 1;
        }
        width_counter += 3;
    }
    hit_counter
}

fn get_copy_multiplier(width: usize, height: usize) -> usize{
    // width: 31 height: 323
    // total width should be > 323
    // 323 / 31 = 10.4xxx
    // total size should be 341, therefore replicated 10 extra times
    math::round::ceil(height as f64 / width as f64, 0) as usize
}

fn replicate_forest_until_square(lines: Vec<String>) -> Vec<String>{
    let width = lines.first().unwrap().len();
    let multiplier = get_copy_multiplier(width, lines.len());
    lines.into_iter().map(|l| {
        let mut longer_str = String::with_capacity(multiplier * width);
        for _ in 0..multiplier {
            longer_str.push_str(&l);
        }
        return longer_str;
    }).collect()
}