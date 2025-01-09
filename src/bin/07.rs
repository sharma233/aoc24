advent_of_code::solution!(7);

struct Equation {
    pub target: u64,
    pub numbers: Vec<u64>
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    let num_map = prep_data(input);
    for equation in num_map {
        if matches_target(equation.numbers[0], equation.target, equation.numbers[1..].to_vec()) {
            total += equation.target;
        }
    }
    Some(total)
}

pub fn matches_target(current_num:u64, target_num: u64, remaining_numbers: Vec<u64>) -> bool {
    if remaining_numbers.is_empty() {
        return current_num == target_num;
    }

    let (first,rest) = remaining_numbers.split_first().unwrap();
    matches_target(current_num * first, target_num, rest.to_vec()) || matches_target(current_num + first, target_num, rest.to_vec())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    let num_map = prep_data(input);
    for equation in num_map {
        if matches_target_concat(equation.numbers[0], equation.target, equation.numbers[1..].to_vec()) {
            total += equation.target;
        }
    }
    Some(total)
}

pub fn matches_target_concat(current_num:u64, target_num: u64, remaining_numbers: Vec<u64>) -> bool {
    if remaining_numbers.is_empty() {
        return current_num == target_num;
    }

    let (first,rest) = remaining_numbers.split_first().unwrap();
    let concat = (current_num.to_string() + first.to_string().as_str()).parse::<u64>().unwrap();
    matches_target_concat(current_num * first, target_num, rest.to_vec()) || matches_target_concat(current_num + first, target_num, rest.to_vec()) || matches_target_concat(concat, target_num, rest.to_vec())
}

fn prep_data(input: &str) -> Vec<Equation> {
    let mut num_map:Vec<Equation> = Vec::new();
    for line in input.split("\n") {
        if line == "" {
            break;
        }
        let parts: Vec<&str> = line.split(":").collect();
        let test_number = parts[0].parse::<u64>().unwrap();
        let mut remaining_numbers:Vec<u64> = Vec::new();
        for i in parts[1].trim().split(" ") {
            if i != "" {
                remaining_numbers.push(i.parse::<u64>().unwrap());
            }
        }
        num_map.push(Equation{target: test_number, numbers: remaining_numbers});
    }
    num_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
