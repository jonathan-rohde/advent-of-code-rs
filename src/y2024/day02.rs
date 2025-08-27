use crate::util::read_file;

const source: &str = "input/day02.txt";

pub fn solve() -> (String, String) {
    let (part1, part2) = (solve_part1(), solve_part2());

    // assert_eq!(part1, "2".to_string());
    // assert_eq!(part2, "4".to_string());

    (part1, part2)
}

fn solve_part1() -> String {
    let input = read_file(source);

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
    let input = read_file(source);

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
    let direction = get_direction(data);
    if let SortDirection::Unknown = direction {
        return false
    }

    for i in 0..data.len() - 1 {
        let sort = SortDirection::from_value(data[i], data[i+1]);
        if let SortDirection::Unknown = sort {
            return false
        }
        if direction != sort {
            return false;
        }
    }

    true
}

#[derive(Eq, PartialEq)]
enum SortDirection {
    Asc,
    Desc,
    Unknown
}

impl SortDirection {
    fn from_value(a: i32, b: i32) -> SortDirection {
        let distance = a - b;
        if (distance.abs() < 1) | (distance.abs() > 3) {
            return SortDirection::Unknown
        }
        if distance < 0 {
            return SortDirection::Asc
        }
        if distance > 0 {
            return SortDirection::Desc
        }
        SortDirection::Unknown

    }
}

fn get_direction(data: &Vec<i32>) -> SortDirection {
    SortDirection::from_value(data[0], data[1])
}