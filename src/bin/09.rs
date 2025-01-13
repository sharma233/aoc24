advent_of_code::solution!(9);

struct FileBlock {
    pub file_id: i32,
    pub size: u32,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk_representation:Vec<String> = get_disk_representation(input);
    let mut first_pointer = disk_representation.iter().position(|x| *x == ".");
    let mut second_pointer = disk_representation.iter().rposition(|x| *x != ".");

    while let Some((mut first_ptr, mut second_ptr)) = first_pointer.zip(second_pointer) {
        if second_ptr < first_ptr {
            break;
        }
        while second_ptr > first_ptr &&
            disk_representation[first_ptr] == "." && first_ptr < disk_representation.len() &&
            disk_representation[second_ptr] != "." && second_ptr > 0 {
                disk_representation[first_ptr] = disk_representation[second_ptr].clone();
                disk_representation[second_ptr] = ".".to_string();
                first_ptr += 1;
                second_ptr -= 1;
        }
        first_pointer = disk_representation.iter().position(|x| *x == ".");
        second_pointer = disk_representation.iter().rposition(|x| *x != ".");
    }

    let mut checksum = 0;
    for (i, file) in disk_representation.iter().enumerate() {
        if *file != "." {
            checksum += i as u64 * file.parse::<u64>().unwrap();
        }
    }
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk_representation = get_disk_representation_blocks(input);
    let file_to_move_pointer = disk_representation.iter().rposition(|x| x.file_id != -1);
    let mut file_to_move_ptr = file_to_move_pointer.unwrap();
    let mut first_free_space = disk_representation.iter().position(|x| x.file_id == -1).unwrap();

    while file_to_move_ptr > first_free_space {
        for i in 0..file_to_move_ptr {
            if disk_representation[i].file_id == -1 {
                // if disk_representation[i].size == disk_representation[file_to_move_ptr].size {
                //     disk_representation.remove(i);
                //     disk_representation.insert(i, FileBlock{file_id: disk_representation[file_to_move_ptr].file_id,size: disk_representation[file_to_move_ptr].size});
                //     disk_representation[file_to_move_ptr].file_id = -1;
                //     first_free_space = disk_representation.iter().position(|x| x.file_id == -1).unwrap();
                //     break;
                // } else if disk_representation[i].size > disk_representation[file_to_move_ptr].size {
                if disk_representation[i].size >= disk_representation[file_to_move_ptr].size {
                    disk_representation[i].size = disk_representation[i].size - disk_representation[file_to_move_ptr].size;
                    disk_representation.insert(i, FileBlock{file_id: disk_representation[file_to_move_ptr].file_id,size: disk_representation[file_to_move_ptr].size});
                    file_to_move_ptr = file_to_move_ptr+1;
                    disk_representation[file_to_move_ptr].file_id = -1;
                    //disk_representation.remove(file_to_move_ptr);
                    first_free_space = disk_representation.iter().position(|x| x.file_id == -1).unwrap();
                    break;
                }
            }
        }
        file_to_move_ptr = disk_representation[0..file_to_move_ptr].iter().rposition(|x| x.file_id != -1).unwrap();
    }

    let mut disk_representation_ungrouped: Vec<FileBlock> = Vec::new();
    for file_block in &disk_representation {
        for _i in 0..file_block.size {
            disk_representation_ungrouped.push(FileBlock{file_id: file_block.file_id, size: 1});
        }
    }

    let mut checksum = 0;
    for (i, file) in disk_representation_ungrouped.iter().enumerate() {
        if file.file_id != -1 {
            checksum += i as u64 * file.file_id as u64;
        }
    }
    Some(checksum as u64)
}

fn get_disk_representation_blocks(input: &str) -> Vec<FileBlock>{
    let mut disk_representation: Vec<FileBlock> = Vec::new();

    let numbers = input.trim().chars().map(|x| x.to_string().parse::<u64>().unwrap());
    let mut file_idx = 0;
    for (i, n) in numbers.enumerate() {
        let file_idx_to_insert;
        if i % 2 == 0 {
            file_idx_to_insert = file_idx;
            file_idx += 1;
        } else {
            file_idx_to_insert = -1;
        }
        disk_representation.push(FileBlock{ file_id: file_idx_to_insert, size: n as u32});
    }

    disk_representation
}

pub fn get_disk_representation(input: &str) -> Vec<String> {
    let numbers = input.trim().chars().map(|x| x.to_string().parse::<u64>().unwrap());
    let mut disk_representation:Vec<String> = Vec::new();
    let mut file_idx = 0;
    for (i, n) in numbers.enumerate() {
        let char_to_insert;
        if i % 2 == 0 {
            char_to_insert = file_idx.to_string();
            file_idx += 1;
        } else {
            char_to_insert = '.'.to_string()
        };
        for _i in 0..n {
            disk_representation.push(char_to_insert.clone());
        }
    }
    disk_representation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_9_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn day9_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
