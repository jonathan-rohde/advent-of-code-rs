use std::fs::read_dir;
use regex::Regex;
use crate::util::read_file;

const source: &str = "input/day03.txt";

pub fn solve() -> (String, String) {
    let (part1, part2) = (solve_part1(), solve_part2());

    // assert_eq!(part1, "161".to_string());
    // assert_eq!(part2, "48".to_string());

    (part1, part2)
}

fn solve_part1() -> String {
    let input = read_file(source);
    let mut result: u64 = 0;
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    for line in input {
        for (_, [a , b]) in regex.captures_iter(&line).map(|c| c.extract()) {
            result += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
        }
    }

    result.to_string()
}

fn solve_part2() -> String {
    let input = read_file(source);
    let mut result: u64 = 0;
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(do\(\))|(don't\(\))").unwrap();
    let regex2 = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut enabled = true;
    for line in input {
        let matches: Vec<_> = regex.find_iter(&line)/*.map(|m| m.as_str())*/.collect();
        for func_match in matches {
            if func_match.as_str() == "do()" {
                enabled = true;
            } else if func_match.as_str() == "don't()" {
                enabled = false;
            } else if enabled {
                let (full, [a, b]) = regex2.captures(func_match.as_str()).unwrap().extract();
                result += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
            }
        }
    }

    result.to_string()
}