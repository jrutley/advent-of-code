use std::collections::HashMap;
use regex::Regex;

fn main() {
    let file_data = textfilereader::read_file_by_line("input.txt");
    let result = execute(file_data, "shiny gold");
    println!("bag colours that eventually contain at least one shiny gold bag: {}", result);
}

/*
input : [
    ["muted lavender", " 5 dull brown bags, 4 pale maroon bags, 2 drab orange bags."],
    ["plaid aqua", " 1 posh violet bag, 5 pale yellow bags, 4 bright salmon bags."]
]
output: {
    muted lavender: [(dull brown, 5), (pale maroon, 4), (drab orange, 2)],
    plaid aqua: [(posh violet, 1), (pale yellow, 5), (bright salmon, 4)]
}
*/
fn split_contained(input: &Vec<Vec<String>>) -> HashMap<String, Vec<(String, i32)>>{
    let child_regex = Regex::new(r"^ ?(\d+?)? (\w+ \w+) bag").unwrap();
    let mut result = HashMap::default();
    for i in input.iter() {
        //[muted lavender, 5 dull brown bags, 4 pale maroon bags, 2 drab orange bags.]
        let parent = i[0].to_owned();
        let children_raw = i[1].split(", ").map(|j| j.to_string()).collect::<Vec<String>>();
        let mut children = vec!();
        for c in children_raw {
            for cap in child_regex.captures_iter(&c) {
                let s = cap.get(1);
                if let Some(number) = s {
                    let count = number.as_str().parse::<i32>().unwrap();
                    children.push(((&cap[2]).to_string(), count));
                }
            }
        }
        result.insert(parent, children);
    }
    result
}

// file_data: [muted lavender bags contain 5 dull brown bags, 4 pale maroon bags, 2 drab orange bags.]
fn execute(file_data: Vec<String>, search_text: &str) -> i32 {
    let input: Vec<Vec<String>> = file_data.iter().map(|i| i
        .split(" bags contain")
        .map(|m| m.to_string())
        .collect::<Vec<String>>()
    ).collect::<Vec<Vec<String>>>();

    let map: HashMap<String, Vec<(String, i32)>> = split_contained(&input);

    search_children(search_text, &map) - 1
}

// map: "muted lavender: [(dull brown, 5), (pale maroon, 4), (drab orange, 2)]"
fn search_children(search_text: &str, map: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    let mut results = 0;
    for child in &map[search_text] {
        let (child_text, count) = child;
        results += count * search_children(&child_text, map);
    }
    1 + results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_containing_bags(){
        let file_data: Vec<String> = vec!(
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string()
        );
        let result = execute(file_data, "shiny gold");
        assert_eq!(result, 126);
    }
}
