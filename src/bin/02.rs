advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    //safe reports need to be either only increasing or decreasing
    //two adjacent levels should differ by at least 1 and at most 3
    let reports: Vec<&str> = input.split("\n").collect();

    let mut total_safe_reports = 0;
    for report in reports {
        let report_nums = report.split(" ").map(|x| x.parse::<i32>().unwrap());
        let report_nums_vec: Vec<i32> = report_nums.collect();
        if matches_criteria(report_nums_vec) {
            total_safe_reports += 1;
        }
    }

    Some(total_safe_reports)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports: Vec<&str> = input.split("\n").collect();

    let mut total_safe_reports = 0;
    for report in reports {
        let report_nums = report.split(" ").map(|x| x.parse::<i32>().unwrap());
        let report_nums_vec: Vec<i32> = report_nums.collect();

        if matches_criteria(report_nums_vec.clone()) {
            total_safe_reports += 1;
        } else {
            //brute force solution that just removes 1 elem each time and runs it through the
            //crtieria
            let mut idx = 0;
            let count = report_nums_vec.clone().len();
            while idx < count {
                let mut mut_vec = report_nums_vec.clone();
                mut_vec.remove(idx);
                if matches_criteria(mut_vec) {
                    total_safe_reports += 1;
                    break;
                }
                idx += 1;
            }
        }
    }
    Some(total_safe_reports)
}

pub fn matches_criteria(report_nums_vec: Vec<i32>) -> bool{
    let mut idx = 0;
    let mut always_decreasing = true;
    let mut always_increasing = true;
    let mut diff_under_3 = true;

    while idx <= report_nums_vec.len() - 2 {
        if (!always_increasing && !always_decreasing) || !diff_under_3 {
            //if we haven't tried modifying this array yet, remove the prev
            //element and then retry parsing it
            break;
        }

        let current_elem = report_nums_vec[idx];
        let next_elem = report_nums_vec[idx + 1];

        if next_elem > current_elem {
            always_decreasing = false;
        } else if next_elem < current_elem {
            always_increasing = false;
        }

        let diff = (current_elem - next_elem).abs();
        if  diff > 3 || diff <= 0 {
            diff_under_3 = false;
        }
        idx += 1;
    }

    if (always_increasing || always_decreasing) && diff_under_3 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
