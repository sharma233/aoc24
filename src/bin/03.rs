advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
    Some(get_mults_total_for_str(input))
}

/**
 * For this part, we build up ranges where we should be parsing mults by first just looking for
 * don't() and do() statements. We use this to build up a vec of ranges where parsing can be
 * performed.
 *
 * For example if we have a string like:
 * mul(1,2).....don't()....mul(2,3)...do()...mul(3,4)
 * the vec produced should look like [0, (idx of first don't), (idx of first do), (total string len)]
 * Then we can process these in chunks of 2 and each chunk would give us a range where we should be
 * adding up all the mul commands
 **/
pub fn part_two(input: &str) -> Option<i32> {
    let re_mul = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mut prev = "do()";
    let mut ranges: Vec<usize> = vec![0];
    for mat in re_mul.captures_iter(input) {
        let mat_unwrapped = mat.get(0).unwrap();
        if prev != mat_unwrapped.as_str() {
            prev = mat_unwrapped.as_str();
            ranges.push(mat_unwrapped.start());
        }
    }
    if ranges.len() % 2 != 0 {
        ranges.push(input.len());
    }

    let mut total = 0;
    for window in ranges.chunks(2) {
        total += get_mults_total_for_str(&input[window[0]..window[1]]);
    }

    Some(total)
}

pub fn get_mults_total_for_str(input: &str) -> i32 {
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
    total
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
    fn day3_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
