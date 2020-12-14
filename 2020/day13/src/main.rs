fn main() {
    let input: Vec<String> = textfilereader::read_file_by_line("input.txt");
    let schedule = &input[1];
    let buses = get_buses(schedule);
    let result = get_periods(buses);
    println!("{} is the result", result);
}

fn get_periods(buses: Vec<(u64, u64)>) -> u64 {
    let mut counter = 1;
    let mut result = 1;
    for (offset, bus_id) in buses {
        while (offset + result) % bus_id != 0 {
            result += counter;
        }
        counter *= bus_id;
    }
    result
}

fn get_buses(bus_input: &str) -> Vec<(u64, u64)> {
    bus_input.split(',').enumerate().filter(|&(_,x)| x != "x")
        .map(|(i, x)| (i as u64, x.parse::<u64>().unwrap())).collect::<Vec<_>>()
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
                assert_eq!(expected, get_periods(buses));
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