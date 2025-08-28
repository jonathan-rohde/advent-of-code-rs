use crate::util::read_file;

const source: &str = "input/day04.txt";

pub fn solve() -> (String, String) {
    let (part1, part2) = (solve_part1(), solve_part2());

    // assert_eq!(part1, "18".to_string());
    // assert_eq!(part2, "48".to_string());

    (part1, part2)
}

fn solve_part1() -> String {
    let content = read_file(source);
    let input: Vec<Vec<String>> = parse_input(&content);
    let mut result = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            result += count_xmas(&input, x, y);
        }
    }

    result.to_string()
}

fn solve_part2() -> String {
    let content = read_file(source);
    let input: Vec<Vec<String>> = parse_input(&content);
    let mut result = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            result += count_x_mas(&input, x, y);
        }
    }

    result.to_string()
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut result = vec![];
    for i in 0..input.len() {
        let mut row = vec![];
        for c in input[i].chars() {
            row.push(c.to_string())
        }
        result.push(row);
    }
    result
}

fn count_xmas(input: &Vec<Vec<String>>, x: usize, y: usize) -> isize {
    let xmas = "XMAS";
    let samx = "SAMX";
    let mut result = 0;
    if (x + 3) < input[y].len() {
        let row = &input[y][x..x + 4].join("");
        result += inc_cond(row == xmas);
        result += inc_cond(row == samx);
    }
    if (y + 3) < input.len() {
        let col = format!(
            "{}{}{}{}",
            input[y][x],
            input[y + 1][x],
            input[y + 2][x],
            input[y + 3][x]
        );
        result += inc_cond(col == xmas);
        result += inc_cond(col == samx);
    }

    if x + 3 < input[y].len() {
        if y >= 3 {
            let diag = format!(
                "{}{}{}{}",
                input[y][x],
                input[y - 1][x + 1],
                input[y - 2][x + 2],
                input[y - 3][x + 3]
            );
            result += inc_cond(diag == xmas);
            result += inc_cond(diag == samx);
        }
        if y + 3 < input.len() {
            let diag = format!(
                "{}{}{}{}",
                input[y][x],
                input[y + 1][x + 1],
                input[y + 2][x + 2],
                input[y + 3][x + 3]
            );
            result += inc_cond(diag == xmas);
            result += inc_cond(diag == samx);
        }
    }

    result
}

fn count_x_mas(input: &Vec<Vec<String>>, x: usize, y: usize) -> isize {
    let mas = "MAS";
    let sam = "SAM";
    let mut result = 0;
    if (y > 0) & (y < input.len() - 1) {
        if (x > 0) & (x < input[y].len() - 1) {
            let diag_1 = format!("{}{}{}", input[y-1][x-1], input[y][x], input[y+1][x+1]);
            let diag_2 = format!("{}{}{}", input[y+1][x-1], input[y][x], input[y-1][x+1]);
            result += inc_cond(
                ((diag_1 == mas) | (diag_1 == sam)) & ((diag_2 == mas) | (diag_2 == sam))
            )
        }
    }
    result
}

fn inc_cond(cond: bool) -> isize {
    cond.into()
}
