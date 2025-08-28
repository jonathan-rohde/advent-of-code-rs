use crate::util::read_file;

const source: &str = "input/day04_test.txt";

pub fn solve() -> (String, String) {
    let (part1, part2) = (solve_part1(), solve_part2());

    // assert_eq!(part1, "161".to_string());
    // assert_eq!(part2, "48".to_string());

    (part1, part2)
}

fn solve_part1() -> String {
    let input = read_file(source);
    String::from("tbd")
}

fn solve_part2() -> String {
    let input = read_file(source);
    String::from("not yet solved")
}