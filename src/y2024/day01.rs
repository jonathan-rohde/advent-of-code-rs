use crate::util::read_file;

pub fn solve() -> String {
    format!("{}\n{}", solve_part1(), solve_part2())
}

fn solve_part1() -> String {
    let (left, right) = get_data();

    let mut result: i64 = 0;

    for i in 0..left.len() {
        result += (&left[i] - &right[i]).abs()
    }

    result.to_string()
}

fn solve_part2() -> String {
    let (left, right) = get_data();
    let mut result: i64 = 0;

    for i in 0..left.len() {
        let it = left[i];
        let occur: i64 = right.iter().filter(|&item| item.eq(&it)).collect::<Vec<_>>().len() as i64;
        result += left[i] * occur;
    }

    result.to_string()
}

fn get_data() -> (Vec<i64>, Vec<i64>) {
    let lines = read_file("input/day01.txt");
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split("   ").collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    }

    left.sort();
    right.sort();
    (left, right)
}