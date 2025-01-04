advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for (i, e) in lines.iter().enumerate() {
        if e.len() > 0 {
            grid.push(Vec::new());
        }
        for c in e.chars() {
            grid[i].push(c);
        }
    }

    let search_word = "XMAS";
    let mut total = 0;

    for (i,e) in grid.iter().enumerate() {
        for (j,_e2) in e.iter().enumerate() {
            //search up
            if i >= search_word.len() - 1 {
                if search_for_word(&grid, i, j, -1, 0, search_word) {
                    total += 1;
                }
            }

            //search down
            if search_for_word(&grid, i, j, 1, 0, search_word) {
                total += 1;
            }

            //search right
            if search_for_word(&grid, i, j, 0, 1, search_word) {
                total += 1;
            }

            //search left
            if j >= search_word.len() - 1 {
                if search_for_word(&grid, i, j, 0, -1, search_word) {
                    total += 1;
                }
            }

            //search up-right
            if i >= search_word.len() - 1{
                if search_for_word(&grid, i, j, -1, 1, search_word) {
                    total += 1;
                }
            }

            //search down-right
            if search_for_word(&grid, i, j, 1, 1, search_word) {
                total += 1;
            }

            //search up-left
            if j >= search_word.len() - 1 && i >= search_word.len() - 1{
                if search_for_word(&grid, i, j, -1, -1, search_word) {
                    total += 1;
                }
            }

            //search down-left
            if j >= search_word.len() - 1{
                if search_for_word(&grid, i, j, 1, -1, search_word) {
                    total += 1;
                }
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for (i, e) in lines.iter().enumerate() {
        if e.len() > 0 {
            grid.push(Vec::new());
        }
        for c in e.chars() {
            grid[i].push(c);
        }
    }

    let search_word = "MAS";
    let search_word_reverse = "SAM";
    let mut total = 0;

    for (i,e) in grid.iter().enumerate() {
        for (j,_e2) in e.iter().enumerate() {
            if grid[i][j] == 'A' {
                let signed_i:i32 = i.try_into().unwrap();
                let signed_j:i32 = j.try_into().unwrap();

                let top_left_x:i32 = signed_i - 1;
                let top_left_y:i32 = signed_j - 1;

                let top_right_x:i32 = signed_i - 1;
                let top_right_y:i32 = signed_j + 1;

                let bot_right_x:i32 = signed_i + 1;
                let bot_right_y:i32 = signed_j + 1;

                let bot_left_x:i32 = signed_i + 1;
                let bot_left_y:i32 = signed_j - 1;

                //check bounds
                let grid_i_len:i32 = grid[i].len().try_into().unwrap();
                let grid_len:i32 = grid.len().try_into().unwrap();
                if top_left_x >= 0 && top_left_y >= 0 && top_right_x >= 0 && bot_left_y >= 0 && top_right_y < grid_i_len && bot_right_x < grid_len && bot_right_y < grid_i_len && bot_left_x < grid_len {
                    let mut str1 = String::new();
                    str1.push(grid[top_left_x as usize][top_left_y as usize]);
                    str1.push('A');
                    str1.push(grid[bot_right_x as usize][bot_right_y as usize]);

                    let mut str2 = String::new();
                    str2.push(grid[bot_left_x as usize][bot_left_y as usize]);
                    str2.push('A');
                    str2.push(grid[top_right_x as usize][top_right_y as usize]);

                    if str1 == search_word && str2 == search_word {
                        total += 1;
                    } else if str1 == search_word && str2 == search_word_reverse {
                        total += 1;
                    } else if str1 == search_word_reverse && str2 == search_word_reverse {
                        total += 1;
                    } else if str1 == search_word_reverse && str2 == search_word {
                        total += 1;
                    }
                }
            }
        }
    }

    Some(total)
}

pub fn search_for_word(grid: &Vec<Vec<char>>, start_x: usize, start_y: usize, x_step: i32, y_step: i32, search_word: &str) -> bool {
    let mut x:i32 = start_x.try_into().unwrap();
    let mut y:i32 = start_y.try_into().unwrap();
    let mut curr_word = String::new();

    while (x >= 0 && (x as usize) < grid.len()) && (y >= 0 && (y as usize) < grid[x as usize].len()) {
        curr_word.push(grid[x as usize][y as usize]);
        if curr_word.len() == search_word.len() {
            if curr_word == search_word {
                return true;
            } else {
                return false;
            }
        }
        y += y_step;
        x += x_step;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn day4_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
