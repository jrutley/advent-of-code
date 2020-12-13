fn main() {
    let input: Vec<String> = textfilereader::read_file_by_line("input.txt");
    let schedule = &input[1];
    let buses = get_buses(schedule);
    println!("Final count: {}", get_consecutive_times(buses));
}

fn get_buses(bus_input: &str) -> Vec<Option<i64>> {
    bus_input.split(',').map(|i| match i.parse::<i64>() { Ok(x) => Some(x), Err(_) => None } ).collect::<Vec<Option<i64>>>()
}

fn get_consecutive_times(buses: Vec<Option<i64>>) -> i64 {
    let mut counter: i64 = 0;
    let bus_length: i64 = (buses.len() - 1) as i64;
    let first_bus = buses[0].unwrap(); // We'll know right away if any input starts with x
    let mut result = false;
    while !result {
        result = inner_loop(counter, bus_length, &buses);
        counter += first_bus;
    }
    counter - first_bus
}

fn inner_loop(counter: i64, bus_length: i64, buses: &Vec<Option<i64>>) -> bool{
    let mut inner = 0;
    for i in counter..=counter+bus_length {
        if let Some(check) = buses[inner] {
            if i % check != 0 {
                return false;
            }
        }
        inner += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! input_tests {
        ($($name:ident: $input:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (schedule, expected) = $input;
                let buses = get_buses(schedule);
                assert_eq!(expected, get_consecutive_times(buses));
            }
        )*
        }
    }

    input_tests! {
        input_0: ("17,x,13,19", 3417),
        input_1: ("67,7,59,61", 754018),
        input_2: ("67,x,7,59,61", 779210),
        input_3: ("67,7,x,59,61", 1261476),
        input_4: ("1789,37,47,1889", 1202161486),
        input_sample: ("7,13,x,x,59,x,31,19", 1068781),
    }
}