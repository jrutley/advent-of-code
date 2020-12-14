use regex::Regex;
use std::collections::HashMap;

// 8244823458677 is too low
fn main() {
    let input: Vec<String> = textfilereader::read_file_by_line("input.txt");
    let results = execute(input);
    let result: u64 = results.values().sum();
    println!("Total: {}", result);
}
fn execute(input: Vec<String>) -> HashMap<u64, u64> {
    let mut memory_locations = HashMap::new();

    let mut pos_mask: u64 = 0;
    let mut neg_mask: u64 = u64::MAX;
    let regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    for i in input {
        if i.starts_with("mask") {
            let mask = &i[7..];
            let pos_mask_str = mask.chars()
                .map(|m| if m == '1' { '1'} else {'0'})
                .collect::<String>();
            pos_mask = u64::from_str_radix(&pos_mask_str, 2).unwrap();
            println!("Set pos_mask to {}", pos_mask);
            let neg_mask_str = mask.chars().map(|m| if m == '0' { '0'} else {'1'}).collect::<String>();
            neg_mask = u64::from_str_radix(&neg_mask_str, 2).unwrap();
            println!("Set neg_mask to {}", neg_mask);
            println!("{:00b}", neg_mask);
        } else {
            for cap in regex.captures_iter(&i) {
                let mem_address = (&cap[1]).parse::<u64>().unwrap();
                let value = (&cap[2]).parse::<u64>().unwrap();
                let current_address_value = memory_locations.entry(mem_address).or_insert(0);

                println!("Addr {}: Set {} (currently {})", mem_address, value, current_address_value);
                //let interim = *current_address_value | value;
                let interim = value;

                let updated_value = interim | pos_mask;
                let final_value = updated_value & neg_mask;
                memory_locations.insert(mem_address, final_value);
            }
        }
    }
    for vals in memory_locations.keys() {
        println!("Memory address {}: {}", vals, memory_locations[vals]);
    }
    memory_locations
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! input_tests {
        ($($name:ident: $input:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (instruction, expected) = $input;
                let mut instructions: Vec<String> = vec!(
                    "mask = 000000X0X00000X00X0X000001X00XX1010X".to_string()
                );
                // mask int : 1024+16+4 = 1044
                instructions.push(instruction.to_string());
                let hash_map: HashMap<u64, u64> = execute(instructions);
                for x in hash_map.values().into_iter() {
                    assert_eq!(expected, *x);
                }
            }
        )*
        }
    }

    input_tests! {
        input_0: ("mem[1] = 1", 1045),
        input_1: ("mem[2] = 2", 1044),
        input_2: ("mem[3] = 4", 1044),
        input_3: ("mem[4] = 8", 1044),
        input_4: ("mem[5] = 16", 1044),
        input_5: ("mem[6] = 32", 1076),
    }

    #[test]
    fn test_sample_input() {
        let input = vec!(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string()
        );
        let result = execute(input);
        assert_eq!(result[&7], 101);
        assert_eq!(result[&8], 64);
    }
}