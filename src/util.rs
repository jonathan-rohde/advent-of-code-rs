use std::fs;

pub fn read_file(file: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for line in fs::read_to_string(file).expect(format!("Unknown file: {}", file).as_str()).split("\n") {
        result.push(line.to_string())
    }
    result
}