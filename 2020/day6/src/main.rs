fn main() {
    let input: Vec<String> = textfilereader::read_file_by_line("input.txt");

    let groups = build_groups(input);

    let results = common_letters(groups);

    let final_sum: usize = results.iter().map(|x|x.len()).sum();
    println!("{}", final_sum);
}

fn common_letters(groups: Vec<Vec<String>>) -> Vec<Vec<char>> {
    let mut results = vec!();
    for group in groups {
        let mut current = group.first().unwrap().to_owned().chars().collect::<Vec<char>>();
        for person in group {
            current = current.into_iter().filter(|&c| person.contains(c)).collect();
        }

        results.push(current);
    }
    results
}
fn build_groups(input: Vec<String>) -> Vec<Vec<String>> {

    let mut groups: Vec<Vec<String>> = vec!();

    let mut current_group: Vec<String> = vec!();
    for line in input {
        if line.len() == 0 {
            groups.push(current_group);
            current_group = vec!();
            continue;
        }

        current_group.push(line);
    }
    groups.push(current_group);

    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_single_line() {
        let input = vec!("abc".to_string());
        let expected: Vec<Vec<String>> = vec!(vec!("abc".to_string()));
        assert_eq!(build_groups(input), expected);
    }

    #[test]
    fn test_group_multi_line() {
        let input = vec!("abc".to_string(),"bcd".to_string());
        let expected: Vec<Vec<String>> = vec!(vec!("abc".to_string(), "bcd".to_string()));
        assert_eq!(build_groups(input), expected);
    }

    #[test]
    fn test_group_multi_line_separate_groups() {
        let input = vec!("abc".to_string(),"bcd".to_string(),String::default(), "efg".to_string(),"hij".to_string());
        let expected: Vec<Vec<String>> = vec!(vec!("abc".to_string(), "bcd".to_string()), vec!("efg".to_string(), "hij".to_string()));
        assert_eq!(build_groups(input), expected);
    }

    #[test]
    fn test_single_line() {
        let input = vec!("abc".to_string());
        let inner = vec!('a','b','c').into_iter().collect::<Vec<char>>();
        let expected: Vec<Vec<char>> = vec!(inner);
        assert_eq!(common_letters(build_groups(input)), expected);
    }

    #[test]
    fn test_multi_line_none() {
        let input = vec!(
            "abc".to_string(),
            "def".to_string());
        let expected: Vec<Vec<char>> = vec!(vec!());
        assert_eq!(common_letters(build_groups(input)), expected);
    }

    #[test]
    fn test_multi_line_overlap() {
        let input = vec!("abc".to_string(),"cde".to_string());
        let inner = vec!('c').into_iter().collect::<Vec<char>>();
        let expected: Vec<Vec<char>> = vec!(inner);
        assert_eq!(common_letters(build_groups(input)), expected);
    }

    #[test]
    fn test_multi_groups() {
        let input = vec!("abc".to_string(),"def".to_string(), String::default(), "efg".to_string());
        let first_inner: Vec<char> = Vec::new();
        let second_inner = vec!('e','f','g').into_iter().collect::<Vec<char>>();
        let expected: Vec<Vec<char>> = vec!(first_inner, second_inner);
        assert_eq!(common_letters(build_groups(input)), expected);
    }
}
