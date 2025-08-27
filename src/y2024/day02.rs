use crate::util::read_file;

pub fn solve() -> (String, String) {
    (solve_part1(), solve_part2())
}

fn solve_part1() -> String {
    let input = read_file("input/day02.txt");

    let mut result: usize = 0;
    for line in input {
        let data: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

        if is_sorted(&data) {
            result += 1;
        }
    }

    result.to_string()
}

fn solve_part2() -> String {
    let input = read_file("input/day02.txt");

    let mut result: usize = 0;
    for line in input {
        let data: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        if validate(&data) {
            result += 1;
        }
    }

    result.to_string()
}

fn validate(data: &Vec<i32>) -> bool {
    if is_sorted(data) {
        return true
    }
    for i in 0..data.len() {
        let part: Vec<i32> = remove_index(data, i);
        if is_sorted(&part) {
            return true
        }
    }
    false
}

fn remove_index(data: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for i in 0..data.len() {
        if i != index {
            result.push(data[i])
        }
    }
    result
}

fn is_sorted(data: &Vec<i32>) -> bool {
    let mut all_asc = true;
    let mut all_desc = true;
    for i in 0..data.len() - 1 {
        if !(1..=3).contains(&(data[i] - data[i + 1]).abs()) {
            return false;
        } else if data[i] < data[i + 1] {
            all_desc = false;
        } else if data[i] > data[i + 1] {
            all_asc = false;
        } else {
            return false;
        }
    }

    all_desc || all_asc
}