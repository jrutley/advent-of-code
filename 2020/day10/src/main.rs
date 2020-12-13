fn main() {
    let jolts: Vec<i64> = textfilereader::read_file_by_line("input.txt").into_iter()
        .map(|n| n.parse::<i64>().unwrap()).collect();
    let result = get_combinations(jolts);
    println!("Total combinations: {}", result);
}

fn get_combinations(mut jolts: Vec<i64>) -> i64 {
    jolts.push(0);     // max of jolts + 3 gets added at the end
    jolts.sort();

    let slices = get_slices(jolts);
/*
    // slice of size 5 -> 7 combinations
    76>77>78>79>80
    76>77>78>80
    76>77>79>80
    76>78>79>80
    76>79>80
    76>78>80
    76>77>80
*/
    slices.iter().fold(1, |acc, cur| acc *
        match cur.len() {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => 0
        })
}

fn get_slices(jolts: Vec<i64>) -> Vec<Vec<i64>> {
    let mut slices : Vec<Vec<i64>> = vec!();
    let mut current_slice: Vec<i64> = vec!();

    for slice in jolts.windows(2) {
        if slice[1] - slice[0] == 1 {
            current_slice.push(slice[0]);
        }
        else {
            current_slice.push(slice[0]);
            slices.push(current_slice);
            current_slice = vec!();
        }
    }
    if current_slice.len() > 0 { slices.push(current_slice)}
    // Insert last element
    let last_elem = jolts.last().unwrap();
    let last_slice = slices.last_mut().unwrap();
    let last_added_elem = last_slice.last().unwrap();
    if last_elem - last_added_elem == 1 {
        last_slice.push(*last_elem);
    }
    else {
        slices.push(vec!(*last_elem));
    }

    // Need the +3
    let device = jolts.last().unwrap() + 3;
    slices.push(vec!(device));

    slices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_containing_bags() {
        let sorted_data: Vec<i64> = vec!(1,4,5,6,7,10,11,12,15,16,19);
        let result = get_combinations(sorted_data);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_larger_set() {
        let sorted_data: Vec<i64> = vec!(28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3);
        let result = get_combinations(sorted_data);
        assert_eq!(result, 19208);
    }
}
