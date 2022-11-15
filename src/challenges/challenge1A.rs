use std::fs;

pub fn solve() -> i32 {
    let file_path = String::from("../inputs/challenge1A.txt");

    // read_to_string returns a "Result" object. Function should return results
    // when errors are expected and recoverable

    let file_contents = fs::read_to_string(file_path).unwrap();

    // split by '\n' character, map to i32 type and collect into an i32 vector

    let numbers = file_contents
        .split('\n')
        .map(|elem| elem.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut count = 0;

    for idx in 1..numbers.len() {
        if numbers[idx - 1] < numbers[idx] {
            count += 1;
        }
    }

    return count;
}
