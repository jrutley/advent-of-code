fn main() {
    let input: Vec<String> = textfilereader::read_file_by_line("input.txt");
    let timestamp: i32 = input[0].parse().unwrap();
    let schedule = &input[1];
    let buses = get_buses(schedule);
    let (time_to_wait, bus_taken) = calculate(timestamp, buses);
    println!("Time to wait: {} Bus: {}", time_to_wait, bus_taken);
    println!("Answer: {}", time_to_wait * bus_taken);
}

fn get_buses(bus_input: &str) -> Vec<i32> {
    bus_input.split(',').filter(|&notx| notx != "x").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn calculate(timestamp: i32, buses: Vec<i32>) -> (i32, i32) {
    let bus_time: Vec<(i32,i32)> = buses.into_iter().map(|b| (b, b - (timestamp % b))).collect();
    let mut position:usize = 0;
    let mut min_position = usize::MAX;
    let mut min_wait: i32 = i32::MAX;
    for (z, wait_time) in bus_time.iter() {
        println!("Checking bus {} time {}", z, wait_time);
        if wait_time < &min_wait {
            min_position = position;
            min_wait = *wait_time;
        }
        position += 1;
    }
    bus_time[min_position]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_input(){
        let timestamp = 939;
        let schedule = "7,13,x,x,59,x,31,19";
        let buses = get_buses(schedule);
        let (time_to_wait, bus_taken) = calculate(timestamp, buses);
        println!("Time to wait: {} Bus: {}", time_to_wait, bus_taken);
        assert_eq!(295, time_to_wait * bus_taken)
    }
}