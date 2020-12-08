use std::collections::HashSet;

fn main() {
    let input: Vec<String> = textfilereader::read_file_by_line("input.txt");

    let groups = build_groups(input);

    let final_sum: usize = groups.iter().map(|x|x.len()).sum();
    println!("{}", final_sum);
}

fn build_groups(input: Vec<String>) -> Vec<HashSet<char>> {
    let mut groups: Vec<HashSet<char>> = vec!();

    let mut current_group: HashSet<char> = HashSet::new();
    for line in input {
        if line.len() == 0 {
            groups.push(current_group);
            current_group = HashSet::new();
            continue;
        }

        for c in line.chars() {
            current_group.insert(c);
        }
    }
    groups.push(current_group);

    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line() {
        let input = vec!("aabc".to_string());
        let inner = vec!('a','b','c').into_iter().collect::<HashSet<char>>();
        let expected: Vec<HashSet<char>> = vec!(inner);
        assert_eq!(build_groups(input), expected);
    }

    #[test]
    fn test_multi_line() {
        let input = vec!("abc".to_string(),"def".to_string());
        let inner = vec!('a','b','c','d','e','f').into_iter().collect::<HashSet<char>>();
        let expected: Vec<HashSet<char>> = vec!(inner);
        assert_eq!(build_groups(input), expected);
    }

    #[test]
    fn test_multi_groups() {
        let input = vec!("abc".to_string(),"def".to_string(), String::default(), "efg".to_string());
        let first_inner = vec!('a','b','c','d','e','f').into_iter().collect::<HashSet<char>>();
        let second_inner = vec!('e','f','g').into_iter().collect::<HashSet<char>>();
        let expected: Vec<HashSet<char>> = vec!(first_inner, second_inner);
        assert_eq!(build_groups(input), expected);
    }
}
