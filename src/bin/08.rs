advent_of_code::solution!(8);
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    let sat_locations = get_sat_locations(input);
    let mut map = get_map(input);
    for (_k,v) in sat_locations.iter() {
        for (i, p1) in v.iter().enumerate() {
            for(j, p2) in v.iter().enumerate() {
                if i == j {
                    continue;
                }

                let anti_nodes = get_anti_nodes(p1, p2);
                for node in &anti_nodes {
                    if check_bounds(&node, &map) {
                        if map[node.0 as usize][node.1 as usize] != '#' {
                            map[node.0 as usize][node.1 as usize] = '#';
                            total += 1;
                        }
                    }
                }
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sat_locations = get_sat_locations(input);
    let mut freq_anti_nodes: HashSet<(i32, i32)>  = HashSet::new();
    let mut map = get_map(input);
    for (_k,v) in sat_locations.iter() {
        for (i, p1) in v.iter().enumerate() {
            for(j, p2) in v.iter().enumerate() {
                if i == j {
                    continue;
                }
                let anti_nodes = get_anti_nodes_part2(p1, p2, &map);
                for node in &anti_nodes {
                    if map[node.0 as usize][node.1 as usize] == '.' {
                        map[node.0 as usize][node.1 as usize] = '#';
                    }
                    freq_anti_nodes.insert((node.0, node.1));
                    freq_anti_nodes.insert((p2.0, p2.1));
                    freq_anti_nodes.insert((p1.0, p1.1));
                }
            }
        }
    }


    let total = freq_anti_nodes.iter().len();

    Some(total as u64)
}

pub fn get_anti_nodes(p1: &(i32, i32), p2: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut anti_nodes: Vec<(i32, i32)> = Vec::new();
    let diff_row = p1.0 - p2.0;
    let diff_col = p1.1 - p2.1;
    anti_nodes.push(((p1.0 + diff_row), (p1.1 + diff_col)));
    anti_nodes.push(((p2.0 - diff_row), (p2.1 - diff_col)));
    return anti_nodes;
}

pub fn get_anti_nodes_part2(p1: &(i32, i32), p2: &(i32, i32), map: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut anti_nodes: Vec<(i32, i32)> = Vec::new();
    let diff_row = p1.0 - p2.0;
    let diff_col = p1.1 - p2.1;

    let mut new_p1 = ((p1.0 + diff_row), (p1.1 + diff_col));
    let mut new_p2 = ((p2.0 - diff_row), (p2.1 - diff_col));
    while check_bounds(&new_p1, map) {
        anti_nodes.push(new_p1);
        new_p1 = ((new_p1.0 + diff_row), (new_p1.1 + diff_col));
    }

    while check_bounds(&new_p2, map) {
        anti_nodes.push(new_p2);
        new_p2 = ((new_p2.0 - diff_row), (new_p2.1 - diff_col));
    }
    return anti_nodes;
}

pub fn check_bounds(point: &(i32, i32), map: &Vec<Vec<char>>) -> bool {
    if point.0 < 0 || point.0 >= map.len()  as i32 {
        return false;
    }

    if point.1 < 0 || point.1 >= map[0].len() as i32 {
        return false;
    }

    return true;
}

pub fn get_sat_locations(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    let mut sat_locations: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (i, line) in input.split("\n").enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                match sat_locations.get_mut(&c) {
                    Some(locations) => {
                        locations.push((i as i32,j as i32));
                    },
                    None => {
                        let mut locations: Vec<(i32, i32)> = Vec::new();
                        locations.push((i as i32,j as i32));
                        sat_locations.insert(c, locations);
                    }
                }
            }
        }
    }
    sat_locations
}

pub fn get_map(input: &str) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in input.split("\n") {
        let mut row:Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        if !row.is_empty() {
            map.push(row);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn day8_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
