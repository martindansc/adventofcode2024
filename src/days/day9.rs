use std::{collections::VecDeque, error::Error, fs::read_to_string};

use crate::helpers::output::print_output;

fn read_input() -> Result<VecDeque<isize>, Box<dyn Error>> {
    let input_file = "inputs/day9a.txt";

    for line in read_to_string(input_file)?.lines() {
        let readed_line = line.to_string();
        let mut readed_line_split: VecDeque<&str> = readed_line.split_terminator("").collect();

        readed_line_split.pop_front();

        let mut readed_values: VecDeque<isize> = VecDeque::new();

        for value in readed_line_split {
            let val: isize = value.parse()?;
            readed_values.push_back(val);
        }

        return Ok(readed_values);
    }

    return Err(Box::from("Input should have one line"));
}

pub fn substract_from_last(queue: &mut VecDeque<isize>) -> (bool, bool) {
    let value_opt = queue.pop_back();
    if value_opt.is_none() {
        return (false, false);
    }

    let value = value_opt.unwrap();
    let mut deleted = false;

    if value > 1 {
        queue.push_back(value - 1);
    } else {
        deleted = true;
        queue.pop_back();
    }

    return (true, deleted);
}

pub fn checksum(queue: &mut VecDeque<isize>) -> (VecDeque<isize>, isize) {
    let mut sum = 0;

    let mut new_queue: VecDeque<isize> = VecDeque::new();

    let mut current_id_start = 0;
    let mut i = 0;
    while let Some(current_value) = queue.pop_front() {
        for _j in 0..current_value {
            new_queue.push_back(current_id_start);
            sum += i * current_id_start;
            i += 1;
        }

        current_id_start += 1;

        let empty_spaces: Option<isize> = queue.pop_front();
        if empty_spaces.is_some() {
            let empty_value = empty_spaces.unwrap();
            for _j in 0..empty_value {
                let substracted = substract_from_last(queue);
                if substracted.0 {
                    let mut v = current_id_start + (queue.len() / 2) as isize;

                    if substracted.1 && queue.len() / 2 > 0 {
                        v += 1;
                    }
                    new_queue.push_back(v);
                    sum += i * v;
                }
                i += 1;
            }
        }
    }

    return (new_queue, sum);
}

pub fn generate_data(queue: &VecDeque<isize>) -> VecDeque<isize> {
    let mut new_queue: VecDeque<isize> = VecDeque::new();

    let mut id = 0;
    for i in 0..queue.len() {
        let v = queue[i];
        for _i in 0..v {
            if i % 2 == 0 {
                new_queue.push_back(id);
            } else {
                new_queue.push_back(-1);
            }
        }

        if i % 2 == 0 {
            id += 1;
        }
    }

    return new_queue;
}

pub fn move_data(queue: &mut VecDeque<isize>) -> VecDeque<isize> {
    let mut new_queue: VecDeque<isize> = VecDeque::new();

    while let Some(mut ele) = queue.pop_front() {
        if ele == -1 {
            while let Some(ele2) = queue.pop_back() {
                if ele2 != -1 {
                    ele = ele2;
                    break;
                }
            }
        }

        if ele != -1 {
            new_queue.push_back(ele);
        }
    }

    return new_queue;
}

pub fn find_new_index_of_size(queue: &VecDeque<isize>, size: isize, max_i: isize) -> Option<isize> {
    let mut last_non_empty = 0;
    for i in 0..max_i {
        if queue[i as usize] != -1 {
            last_non_empty = i;
        }

        if i - last_non_empty >= size {
            return Some(last_non_empty + 1);
        }
    }

    return None;
}

pub fn move_data_2(queue: &mut VecDeque<isize>) {
    let mut last_id: isize = queue.back().unwrap().to_owned();

    let mut i: isize = queue.len() as isize;

    while last_id > 0 {
        let mut n: isize = queue[(i - 1) as usize];
        while n != last_id {
            i -= 1;
            n = queue[(i - 1) as usize];
        }

        let mut j: isize = i - 1;

        while queue[j as usize] == last_id {
            j -= 1;
        }

        j += 1;

        let size: isize = i - j;

        let new_index = find_new_index_of_size(queue, size, j);
        if new_index.is_some() {
            let start_index = new_index.unwrap();
            for ni in start_index..(start_index + size) {
                queue[ni as usize] = last_id;
            }

            for oi in j..(j + size) {
                queue[oi as usize] = -1;
            }
        }

        last_id -= 1;
        i -= size;
    }
}

pub fn sum_data(queue: &VecDeque<isize>) -> isize {
    let mut sum = 0;

    for i in 0..queue.len() {
        if queue[i] == -1 {
            continue;
        }

        sum += i as isize * queue[i];
    }

    return sum;
}

#[test]
pub fn day9a() -> Result<(), Box<dyn Error>> {
    let line: VecDeque<isize> = read_input()?;

    let mut data: VecDeque<isize> = generate_data(&line);
    let moved_data = move_data(&mut data);
    let sum = sum_data(&moved_data);

    return print_output("day9a".to_string(), sum);
}

#[test]
pub fn day9b() -> Result<(), Box<dyn Error>> {
    let line: VecDeque<isize> = read_input()?;

    let mut data: VecDeque<isize> = generate_data(&line);
    move_data_2(&mut data);
    let sum = sum_data(&data);

    return print_output("day9b".to_string(), sum);
}

#[test]
pub fn should_be_equal() {
    let mut line: VecDeque<isize> = read_input().unwrap();
    let mut data: VecDeque<isize> = generate_data(&line);

    let moved_data = move_data(&mut data);
    let moved_data2 = checksum(&mut line).0;

    for i in 0..moved_data.len() {
        let v1 = moved_data[i];
        let v2 = moved_data2[i];

        assert_eq!(v1, v2);
    }
}
