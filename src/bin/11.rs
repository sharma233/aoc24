advent_of_code::solution!(11);
use std::{collections::HashMap, hash::Hash};

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones:Vec<u64> = input.split(" ").map(|x| x.trim().parse::<u64>().unwrap()).collect();
    for _ in 0..25 {
        blink(&mut stones);
    }
    Some(stones.len() as u64)
}

pub fn blink(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        let stone = stones[i];
        if stone == 0 {
            stones[i] = 1;
        } else if stone.to_string().len() % 2 == 0 {
            stones.remove(i);
            let left_half = &stone.to_string()[0..stone.to_string().len() / 2];
            let right_half = &stone.to_string()[stone.to_string().len() / 2..];
            stones.insert(i, left_half.parse::<u64>().unwrap());
            stones.insert(i+1, right_half.parse::<u64>().unwrap());
            i = i + 1;
        } else {
            stones[i] = stones[i] * 2024;
        }
        i = i + 1;
    }
}

pub fn blink_p2(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut nums:HashMap<u64,u64> = HashMap::new();

    for stone in stones.keys() {
        let stone_count = stones.get(stone).unwrap();
        if *stone == 0 {
            match &nums.get(&1) {
                Some(count) => {nums.insert(1, *count+stone_count)},
                None => {nums.insert(1, *stone_count)}
            };
        } else if stone.to_string().len() % 2 == 0 {
            let left_half = &stone.to_string()[0..stone.to_string().len() / 2].parse::<u64>().unwrap();
            let right_half = &stone.to_string()[stone.to_string().len() / 2..].parse::<u64>().unwrap();

            match &nums.get(left_half) {
                Some(count) => {nums.insert(*left_half, *count+stone_count)},
                None => {nums.insert(*left_half, *stone_count)}
            };
            match &nums.get(right_half) {
                Some(count) => {nums.insert(*right_half, *count+stone_count)},
                None => {nums.insert(*right_half, *stone_count)}
            };
        } else {
            let num = stone * 2024;
            match &nums.get(&num) {
                Some(count) => {nums.insert(num, *count+stone_count)},
                None => {nums.insert(num, *stone_count)}
            };
        }
    }
    nums
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones_vec:Vec<u64> = input.split(" ").map(|x| x.trim().parse::<u64>().unwrap()).collect();
    let mut stones: HashMap<u64, u64> = HashMap::new();

    for i in stones_vec {
        stones.insert(i, 1);
    }

    for _ in 0..75 {
        stones = blink_p2(stones);
    }
    let mut total = 0;
    for i in stones.values() {
        total += i;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn day11_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
