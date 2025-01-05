advent_of_code::solution!(5);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut order_rules: Vec<&str> = Vec::new();
    let mut updates: Vec<&str> = Vec::new();
    let mut parsing_updates: bool = false;

    for line in lines {
        if line == "" {
            parsing_updates = true;
        } else if parsing_updates {
            updates.push(line);
        } else if !parsing_updates {
            order_rules.push(line);
        }
    }

    /*
     * Build up a map that contains a parent node -> Vec of all its children
     */
    let mut no_to_children: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in order_rules {
         let rule_nos: Vec<i32> = rule.to_string().split("|").map(|x| x.parse::<i32>().unwrap()).collect();
         let parent = rule_nos[0];
         let child = rule_nos[1];

         if !no_to_children.contains_key(&parent) {
             no_to_children.insert(parent, vec![child]);
         } else {
             let children = no_to_children.get_mut(&parent).unwrap();
             children.push(child);
         }
    }

    let mut valid_updates:Vec<Vec<i32>> = Vec::new();
    for update in updates {
        let mut valid_update = true;
        let ints: Vec<i32> = update.to_string().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        /* For every 2 elements, check if the 2nd element is contained in the children list
         * for the first. If it is, this update is still valid, otherwise it is not
         */
        for window in ints.windows(2) {
            let parent = window[0];
            let child = window[1];
            let children = match no_to_children.get(&parent) {
                Some(children) => children,
                None => { valid_update = false; break }
            };
            if !children.contains(&child) {
                valid_update = false;
                break;
            }
        }
        if valid_update {
            valid_updates.push(ints.clone());
        }
    }

    let mut total = 0;
    for valid_update in valid_updates {
        total += valid_update[valid_update.len()/2];
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut order_rules: Vec<&str> = Vec::new();
    let mut updates: Vec<&str> = Vec::new();
    let mut parsing_updates: bool = false;

    for line in lines {
        if line == "" {
            parsing_updates = true;
        } else if parsing_updates {
            updates.push(line);
        } else if !parsing_updates {
            order_rules.push(line);
        }
    }

    /*
     * Build up a map that contains a parent node -> Vec of all its children
     */
    let mut no_to_children: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in order_rules {
         let rule_nos: Vec<i32> = rule.to_string().split("|").map(|x| x.parse::<i32>().unwrap()).collect();
         let parent = rule_nos[0];
         let child = rule_nos[1];

         if !no_to_children.contains_key(&parent) {
             no_to_children.insert(parent, vec![child]);
         } else {
             let children = no_to_children.get_mut(&parent).unwrap();
             children.push(child);
         }
    }

    let mut valid_updates:Vec<Vec<i32>> = Vec::new();
    let mut invalid_updates:Vec<Vec<i32>> = Vec::new();
    for update in updates {
        let mut valid_update = true;
        let ints: Vec<i32> = update.to_string().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        /* For every 2 elements, check if the 2nd element is contained in the children list
         * for the first. If it is, this update is still valid, otherwise it is not
         */
        for window in ints.windows(2) {
            let parent = window[0];
            let child = window[1];
            let children = match no_to_children.get(&parent) {
                Some(children) => children,
                None => { valid_update = false; break }
            };
            if !children.contains(&child) {
                valid_update = false;
                break;
            }
        }
        if valid_update {
            valid_updates.push(ints.clone());
        } else {
            invalid_updates.push(ints.clone());
        }
    }

    // let's fix all the invalid ones
    let mut fixed_updates: Vec<Vec<i32>> = Vec::new();
    for mut invalid_update in invalid_updates {
        let mut fixed_update = Vec::new();
        let mut i = 0;
        while i < invalid_update.len() {
            //if current element is not the child of any other element in the list then push it to
            //the fixed list
            let mut is_child = false;
            for j in 0..invalid_update.len() {
                if no_to_children.contains_key(&invalid_update[j]) {
                    if no_to_children.get(&invalid_update[j]).unwrap().contains(&invalid_update[i]) {
                        is_child = true;
                    }
                }
            }
            if !is_child {
                fixed_update.push(invalid_update[i]);
                invalid_update.remove(i);
                i = 0;
            } else {
                i += 1;
            }
        }
        fixed_updates.push(fixed_update);
    }

    let mut total = 0;
    for valid_update in fixed_updates {
        total += valid_update[valid_update.len()/2];
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn day5_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
