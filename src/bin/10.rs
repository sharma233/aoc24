advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in input.split("\n").collect::<Vec<&str>>() {
        if line != "" {
            grid.push(Vec::new());
            for c in line.chars() {
                if c == '.' {
                    grid.last_mut().unwrap().push(-1);
                } else {
                    if c != '\n' {
                        grid.last_mut().unwrap().push(c.to_string().parse::<i32>().unwrap());
                    }
                }
            }
        }
    }

    let mut total = 0;
    for (i, _row) in grid.iter().enumerate() {
        for (j, _col) in grid.iter().enumerate() {
            if grid[i][j] == 0 {
                let mut seen: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
                total += dfs(&grid, i as i32, j as i32, -1, &mut seen);
            }
        }
    }

    Some(total)
}

pub fn dfs(grid: &Vec<Vec<i32>>, start_index_row: i32, start_index_col: i32, prev: i32, seen: &mut Vec<Vec<bool>>) -> u64 {
    if (start_index_row < 0 || start_index_row >= grid.len() as i32) || (start_index_col < 0 || start_index_col >= grid[start_index_row as usize].len() as i32) {
        return 0;
    }

    if grid[start_index_row as usize][start_index_col as usize] - prev != 1 {
        return 0;
    }

    if grid[start_index_row as usize][start_index_col as usize] == 9  && !seen[start_index_row as usize][start_index_col as usize] {
        seen[start_index_row as usize][start_index_col as usize] = true;
        return 1;
    }

    seen[start_index_row as usize][start_index_col as usize] = true;

    dfs(grid, start_index_row, start_index_col + 1, grid[start_index_row as usize][start_index_col as usize], seen) +
    dfs(grid, start_index_row, start_index_col - 1, grid[start_index_row as usize][start_index_col as usize], seen) +
    dfs(grid, start_index_row + 1, start_index_col, grid[start_index_row as usize][start_index_col as usize], seen) +
    dfs(grid, start_index_row - 1, start_index_col, grid[start_index_row as usize][start_index_col as usize], seen)
}


pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in input.split("\n").collect::<Vec<&str>>() {
        if line != "" {
            grid.push(Vec::new());
            for c in line.chars() {
                if c == '.' {
                    grid.last_mut().unwrap().push(-1);
                } else {
                    if c != '\n' {
                        grid.last_mut().unwrap().push(c.to_string().parse::<i32>().unwrap());
                    }
                }
            }
        }
    }

    let mut total = 0;
    for (i, _row) in grid.iter().enumerate() {
        for (j, _col) in grid.iter().enumerate() {
            if grid[i][j] == 0 {
                let mut seen: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
                total += dfs_part_2(&grid, i as i32, j as i32, -1, &mut seen);
            }
        }
    }
    Some(total)
}

pub fn dfs_part_2(grid: &Vec<Vec<i32>>, start_index_row: i32, start_index_col: i32, prev: i32, seen: &mut Vec<Vec<bool>>) -> u64 {
    if (start_index_row < 0 || start_index_row >= grid.len() as i32) || (start_index_col < 0 || start_index_col >= grid[start_index_row as usize].len() as i32) {
        return 0;
    }

    if grid[start_index_row as usize][start_index_col as usize] - prev != 1 {
        return 0;
    }

    if grid[start_index_row as usize][start_index_col as usize] == 9 {
        return 1;
    }

    dfs_part_2(grid, start_index_row, start_index_col + 1, grid[start_index_row as usize][start_index_col as usize], seen) +
    dfs_part_2(grid, start_index_row, start_index_col - 1, grid[start_index_row as usize][start_index_col as usize], seen) +
    dfs_part_2(grid, start_index_row + 1, start_index_col, grid[start_index_row as usize][start_index_col as usize], seen) +
    dfs_part_2(grid, start_index_row - 1, start_index_col, grid[start_index_row as usize][start_index_col as usize], seen)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn day10_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
