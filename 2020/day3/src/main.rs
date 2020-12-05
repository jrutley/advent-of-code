fn main() {
    let forest = build_forest(7);

    let hit_counter =
        how_many_trees(1, 1, &forest) *
        how_many_trees(3, 1, &forest) *
        how_many_trees(5, 1, &forest) *
        how_many_trees(7, 1, &forest) *
        how_many_trees(1, 2, &forest);

    println!("Result: {}", hit_counter);
}

fn how_many_trees(horizontal: usize, vertical: usize, forest: &Vec<String>) -> i64 {
    let mut hit_counter = 0;
    let mut height_counter = 0;
    let mut width_counter = 0;
    for height in forest {
        if height_counter % vertical == 0 {
            if height.chars().nth(width_counter) == Some('#') {
                hit_counter += 1;
            }
            width_counter += horizontal;
        }
        height_counter += 1;
    }
    hit_counter
}

fn build_forest(max_size: usize) -> Vec<String> {
    let lines = textfilereader::read_file_by_line("input.txt");

    // This is pretty inefficient
    // Better to have only one "block" and iterate over it instead of making a giant graph
    replicate_forest_until_wide_enough(max_size, lines)
}

fn get_copy_multiplier(ratio: usize, width: usize, height: usize) -> usize{
    // width: 31 height: 323
    // total width should be at least 3 * 323 since the toboggan goes 3 across and 1 down
    // 323 * 3 / 31 = 31.2xxx
    // total size should be 341, therefore replicated 10 extra times
    math::round::ceil(height as f64 * ratio as f64 / width as f64, 0) as usize
}

fn replicate_forest_until_wide_enough(ratio: usize, lines: Vec<String>) -> Vec<String>{
    let width = lines.first().unwrap().len();
    let multiplier = get_copy_multiplier(ratio, width, lines.len());
    lines.into_iter().map(|l| {
        let mut longer_str = String::with_capacity(multiplier * width);
        for _ in 0..multiplier {
            longer_str.push_str(&l);
        }
        return longer_str;
    }).collect()
}