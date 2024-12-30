advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    let lines: Vec<&str> = input.split("\n").collect();

    for line in lines {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        list1.push(numbers[0].parse::<i32>().unwrap());
        list2.push(numbers[1].parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();

    let mut diff = 0;
    for (pos, e) in list1.iter().enumerate() {
        let curr_diff = (e - list2[pos]).abs();
        diff += curr_diff;
    }
    Some(diff)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    let lines: Vec<&str> = input.split("\n").collect();

    for line in lines {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        list1.push(numbers[0].parse::<i32>().unwrap());
        list2.push(numbers[1].parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();

    let mut similarity_score = 0;
    let mut last_pos = 0;
    let mut last_similarity_score = 0;

    for (pos_i, i) in list1.iter().enumerate() {
        let mut count = 0;

        //if the number is the same just use the same score
        if pos_i > 0 && *i == list1[pos_i - 1] {
            similarity_score += last_similarity_score;
            continue;
        }

        //otherwise start looking from where we stopped last (since the lists are sorted)
        for (pos, j) in list2.iter().skip(last_pos).enumerate() {
            if i == j {
                count += 1;
            } else if i < j {
                //if we reach a number bigger than what we're looking for,
                //we can safely assume that we don't need to look at the rest of the list.
                last_pos = pos;
                break;
            }
        }
        similarity_score += i * count;
        last_similarity_score = similarity_score;
    }

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
