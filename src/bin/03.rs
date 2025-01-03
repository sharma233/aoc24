advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
    let re_mul = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let re_nums = Regex::new(r"\d+,\d+").unwrap();
    let mut total = 0;
    for mat in re_mul.captures_iter(input) {
        let mul_expr = &mat[0];
        //for some reason using "\d+" with .captures() only returned one number so hacking around
        //it by matching the whole string with commas and splitting and parsing into ints
        let num_caps = re_nums.captures(mul_expr).unwrap();
        let nums_str = num_caps.get(0).unwrap().as_str();
        let nums: Vec<i32> = nums_str.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        total += nums[0] * nums[1];
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
