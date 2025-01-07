advent_of_code::solution!(6);

struct ObstacleTracker {
    pub row: i32,
    pub col: i32,
    pub row_velocity: i32,
    pub col_velocity: i32,
}

struct GuardPos {
    pub row: i32,
    pub col: i32,
    pub row_velocity: i32,
    pub col_velocity: i32,
    pub obstacles: Option<Vec<GuardPos>>
}

impl GuardPos {
    fn move_guard(&mut self, map: &mut Vec<Vec<char>>) -> bool {
        //check next move pos
        let next_row = self.row + self.row_velocity;
        let next_col = self.col + self.col_velocity;
        if (next_row < 0 || next_row >= map[0].len().try_into().unwrap()) || (next_col < 0 || next_col >= map.len().try_into().unwrap()) {
            return true;
        }
        let next_pos = map[next_row as usize][next_col as usize];

        //move guard if good move else move right
        if next_pos == '.' || next_pos == 'X' {
            map[self.row as usize][self.col as usize] = 'X';
            self.row = next_row;
            self.col = next_col;
            map[next_row as usize][next_col as usize] = '^';
        } else if next_pos == '#' {
            if self.row_velocity != 0 {
                //flip and invert
                self.col_velocity = self.row_velocity * - 1;
                self.row_velocity = 0;
            } else {
                //flip
                self.row_velocity = self.col_velocity;
                self.col_velocity = 0;
            }
        }
        false
    }

    fn move_guard_detect_loop(&mut self, map: &mut Vec<Vec<char>>) -> bool {
        //check next move pos
        let next_row = self.row + self.row_velocity;
        let next_col = self.col + self.col_velocity;
        if (next_row < 0 || next_row >= map[0].len().try_into().unwrap()) || (next_col < 0 || next_col >= map.len().try_into().unwrap()) {
            return false;
        }
        let next_pos = map[next_row as usize][next_col as usize];

        //move guard if good move else move right
        if next_pos == '.' || next_pos == 'X' {
            map[self.row as usize][self.col as usize] = 'X';
            self.row = next_row;
            self.col = next_col;
            map[next_row as usize][next_col as usize] = '^';
        } else if next_pos == '#' {
            //check obstacle trackers, if found then we're in a loop
            //otherwise save in obstacle trackers and turn
            if let Some(_found) = self.obstacles.as_ref().unwrap().iter().find(|o| o.row == next_row && o.col == next_col && o.row_velocity == self.row_velocity && o.col_velocity == self.col_velocity) {
                return true;
            } else {
                self.obstacles.as_mut().unwrap().push(GuardPos {row: next_row, col: next_col, row_velocity: self.row_velocity, col_velocity: self.col_velocity, obstacles: None});
            }
            if self.row_velocity != 0 {
                //flip and invert
                self.col_velocity = self.row_velocity * - 1;
                self.row_velocity = 0;
            } else {
                //flip
                self.row_velocity = self.col_velocity;
                self.col_velocity = 0;
            }
        }
        self.move_guard_detect_loop(map)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut guard_pos = GuardPos {row: 0, col: 0, row_velocity: -1, col_velocity: 0, obstacles: Some(Vec::new())};
    let mut map: Vec<Vec<char>> = Vec::new();

    for (i, line) in input.split("\n").enumerate() {
        if line != "" {
            map.push(Vec::new());
        }
        for c in line.chars() {
            map[i].push(c);
            if c == '^' {
                guard_pos.row = i as i32;
                guard_pos.col = (map[i].len() - 1) as i32;
            }
        }
    }

    let mut left_map: bool = false;
    while !left_map {
        left_map = guard_pos.move_guard(&mut map);
    }

    let mut total = 0;
    for line in map {
        for c in line {
            if c == 'X' {
                total += 1;
            }
        }
    }
    Some(total + 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut guard_pos = GuardPos {row: 0, col: 0, row_velocity: -1, col_velocity: 0, obstacles: Some(Vec::new())};
    let mut map: Vec<Vec<char>> = Vec::new();

    for (i, line) in input.split("\n").enumerate() {
        if line != "" {
            map.push(Vec::new());
        }
        for c in line.chars() {
            map[i].push(c);
            if c == '^' {
                guard_pos.row = i as i32;
                guard_pos.col = (map[i].len() - 1) as i32;
            }
        }
    }

    let mut total = 0;
    for (i, _row) in map.iter().enumerate() {
        for (j, _col) in map[i].iter().enumerate() {
            if map[i][j] != '^' && map[i][j] != '#' {
                let mut new_map = map.clone();
                let mut new_guard_pos = GuardPos {
                    row: guard_pos.row,
                    col: guard_pos.col,
                    row_velocity: guard_pos.row_velocity,
                    col_velocity: guard_pos.col_velocity,
                    obstacles: Some(Vec::new())
                };
                new_map[i][j] = '#';
                if new_guard_pos.move_guard_detect_loop(&mut new_map) {
                    total += 1
                }
            } else {
                continue;
            }
        }
    }


    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn day6_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
