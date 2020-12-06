fn main() {
    let input: Vec<String> = textfilereader::read_file_by_line("input.txt");
    let binary =
        input.iter().map(|i| isize::from_str_radix(&replace_chars((&i).to_string()), 2).unwrap())
        .fold(0, isize::max);

    println!("{}", binary);
}

fn replace_chars(x: String) -> String{
    x.chars().map(|c| if c == 'F' || c == 'L' {'0'} else {'1'}).collect()
}