use crate::trees::ArenaTree;
use crate::trees::Node;
use std::collections::HashSet;
use regex::Regex;

mod trees;

#[derive(PartialEq, Default)]
struct Bag {
    description: String,
    colour: String
}

fn main() {
    let file_data = textfilereader::read_file_by_line("input.txt");
    let result = execute(file_data, "shiny gold");
    println!("bag colours that eventually contain at least one shiny gold bag: {}", result.len());
}

fn execute(file_data: Vec<String>, search_text: &str) -> HashSet<usize> {
    let input: Vec<Vec<String>> = file_data.iter().map(|i| i
        .split(" bags contain")
        .map(|m| m.to_string())
        .collect::<Vec<String>>()
    ).collect::<Vec<Vec<String>>>();

    let mut tree: ArenaTree<String> = trees::ArenaTree::default();

    insert_nodes(&mut tree, &input);
    link_nodes(&mut tree, &input);

    let search_node: usize = tree.node(search_text.to_string().into());
    let mut parent_hash = HashSet::new();
    let current_node = &tree.arena[search_node];
    add_parents_to_hash(&mut parent_hash, &tree, &current_node, 0);
    parent_hash
}

fn add_parents_to_hash(containing_node_ids: &mut HashSet<usize>, tree: &ArenaTree<String>, node: &Node<String>, depth: i32){
    if depth > 0 {
        containing_node_ids.insert(node.idx);
    }
    for p in node.parent.iter() {
        let current_node = &tree.arena[*p];
        add_parents_to_hash(containing_node_ids, tree, current_node, depth + 1);
    }
}

fn link_nodes(tree: &mut trees::ArenaTree<String>, input: &Vec<Vec<String>>){
    let child_regex = Regex::new(r"^ ?\d+ (\w+ \w+) bag").unwrap();
    for i in input.iter() {
        let parent = tree.node(i[0].to_owned().into());

        let children = i[1].split(", ").map(|j| j.to_string()).collect::<Vec<String>>();
        for c in children {
            for cap in child_regex.captures_iter(&c) {
                let child = tree.node((&cap[1]).to_string());
                tree.link_parent_child(parent, child);
            }
        }
    }
}

fn insert_nodes(tree: &mut trees::ArenaTree<String>, input: &Vec<Vec<String>>){
    for i in input.iter() {
        let first_half = i[0].to_owned();
        tree.node(first_half.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input(){
        let file_data: Vec<String> = vec!(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string()
        );
        let result = execute(file_data, "shiny gold");
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_direct_descendant_input(){
        let file_data: Vec<String> = vec!(
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string()
        );
        let result = execute(file_data, "shiny gold");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_group_single_line() {
        // "muted lavender bags contain 5 dull brown bags, 4 pale maroon bags.".to_string(),
        // "dull brown bags contain no other bags.".to_string(),
        // "pale maroon bags contain no other bags.".to_string()

        let input: Vec<Vec<String>> =
            vec!(
                vec!("muted lavender".to_string(), " 5 dull brown bags, 4 pale maroon bags.".to_string()),
                vec!("dull brown".to_string(), " no other bags.".to_string()),
                vec!("pale maroon".to_string(), " no other bags.".to_string())
            );
        let mut tree: trees::ArenaTree<String> = trees::ArenaTree::default();
        insert_nodes(&mut tree, &input);
        link_nodes(&mut tree, &input);

        assert_eq!(tree.size(), 3);
        let lavender = tree.node("muted lavender".into());
        assert_eq!(tree.arena[lavender].children.len(), 2);
    }
}

    //let x = "string".to_string().split("i").map(|i| i.to_string()).collect::<Vec<String>>();
    // for i in input.iter() {
    //     // let containing_bag = splits[0].split(' ').take(2).collect::<Vec<&str>>();
    //     let container = Bag{ description: containing_bag[0].to_string(), colour: containing_bag[1].to_string()};
    //     tree.node(container);
    // }

    // for i in input {
    //     let splits: Vec<&str> = i.split(" bags contain").collect();
    //     let contained_bag_list = splits[1].split(", ").collect::<Vec<&str>>();
    //     let tokenized_contained_bags = contained_bag_list.iter().map(|b| b.split(" ").take(3).collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    // }


    // let hello = tree.node("Hello".into());
    // let world = tree.node("World".into());
    // tree.link_parent_child(hello, world);
    // let exclamation = tree.node("!".into());
    // tree.link_parent_child(world, exclamation);
    // let period = tree.node(".".into());
    // let period_2 = tree.node(".".into());
    // assert_eq!(period, period_2);
    // tree.link_parent_child(world, period);
    // // tree.arena[world].children.push(period);
    // // tree.arena[period_2].parent = Some(world);

    // println!(
    //     "Total nodes: {}\nTotal edges: {}\nDepth of '.': {}\nDistance between World and !: {}",
    //     tree.size(),
    //     tree.edges(),
    //     tree.depth(period),
    //     tree.distance_between("World".into(), "!".into())
    // );
// }
