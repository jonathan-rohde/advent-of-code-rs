mod y2024;
mod util;

struct ExecutionResult {
    result: String,
    timing: core::time::Duration
}

fn main() {
    // print_result(1, || y2024::day01::solve());
    // print_result(2, || y2024::day02::solve());
    print_result(3, || y2024::day03::solve());
}

fn format(value: (String, String)) -> String {
    format!("Part1: {}, Part2: {}", value.0, value.1)
}

fn execute(func: impl Fn() -> (String, String)) -> ExecutionResult {
    let start = std::time::Instant::now();
    let exec_result = func();
    let timing = start.elapsed();
    let result = format(exec_result);
    ExecutionResult {
        result,
        timing
    }
}

fn print_result(day: i16, func: impl Fn() -> (String, String)) {
    let result = execute(func);
    println!("=== Day {} ===\nResult: {}\nTime: {:#?}", day, result.result, result.timing);
}