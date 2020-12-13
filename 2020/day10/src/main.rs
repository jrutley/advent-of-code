fn main() {
    let mut jolts: Vec<i32> = textfilereader::read_file_by_line("input.txt").into_iter()
        .map(|n| n.parse::<i32>().unwrap()).collect();
    jolts.sort();

    let (mut ones, mut threes) = (0, 1);
    let mut current = 0;
    for jolt in jolts {
        if jolt - current == 1 {
            ones += 1;
        } else if jolt - current == 3 {
            threes += 1;
        }
        current = jolt;
    }
    println!("Ones: {} Threes: {}", ones, threes);
    println!("Multiplied: {}", ones * threes);
}
