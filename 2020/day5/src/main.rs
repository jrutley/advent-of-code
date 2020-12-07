// BFFBBFBLLR is incorrect!?

fn main() {
    let input: Vec<String> = textfilereader::read_file_by_line("input.txt");

    let mut ids: Vec<isize> =
        input.iter()
            .map(|i| seats_to_id(i))
            .collect();

    let missing_val = find_gap(&mut ids);

    if let Some(answer) = missing_val {
        println!("Position {}", answer);
        let seat = convert_to_seat(answer);
        println!("Missing entry: {}", seat);
    } else {
        println!("Couldn't find a gap")
    }
}

fn replace_chars(x: &str) -> String{
    x.chars().map(|c| if c == 'F' || c == 'L' {'0'} else {'1'}).collect()
}

fn seats_to_id(i: &str) -> isize {
    let bin_chars = &replace_chars(i);

    isize::from_str_radix(bin_chars, 2).unwrap()
}

fn convert_row(x: isize) -> String {
    let row_bin = format!("{0:07b}", x/8);
    row_bin.chars().map(|c| if c == '0' {'F'} else {'B'}).collect()
}
fn convert_col(x: isize) -> String {
    let col_bin = format!("{:03b}", x & 0x7);
    col_bin.chars().map(|c| if c == '0' {'L'} else {'R'}).collect()
}
fn convert_to_seat(x: isize) -> String {
    convert_row(x)+ &convert_col(x)
}

fn find_gap(sorted: &mut Vec<isize>) -> Option<isize> {
    sorted.sort();

    let first = sorted.first().unwrap();
    let mut counter = *first;
    for x in sorted {
        if *x != counter {
            return Some(counter)
        }
        counter += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_middle_unsorted() {
        let mut items: Vec<isize> = vec!(1,3,5,2);
        assert_eq!(find_gap(&mut items), Some(4));
    }

    #[test]
    fn test_missing_middle() {
        let mut items: Vec<isize> = vec!(1,2,3,5);
        assert_eq!(find_gap(&mut items), Some(4));
    }
    #[test]
    fn test_none_missing() {
        let mut items: Vec<isize> = vec!(1,2,3,4,5);
        assert_eq!(find_gap(&mut items), None);
    }
    #[test]
    fn test_convert_seat_to_id(){
        assert_eq!(seats_to_id("BFFFBBFRRR"), 567);
        assert_eq!(seats_to_id("FFFBBBFRRR"), 119);
        assert_eq!(seats_to_id("BBFFBBFRLL"), 820);
        assert_eq!(seats_to_id("FBFBBFFRLR"), 357);
    }

    #[test]
    fn test_convert_id_to_seat(){
        assert_eq!(convert_to_seat(567),"BFFFBBFRRR");
        assert_eq!(convert_to_seat(119), "FFFBBBFRRR");
        assert_eq!(convert_to_seat(820),"BBFFBBFRLL");
        assert_eq!(convert_to_seat(357),"FBFBBFFRLR");
    }
}