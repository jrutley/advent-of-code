fn main() {
    let input = textfilereader::read_file_by_line("input.txt");
    for elem in input {
        println!("{}", elem);
    }
}
