use std::fs;

pub fn solve() -> i32 {
    let file_path = String::from("../inputs/challenge1B.txt");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let numbers = file_contents
        .split('\n')
        .map(|elem| elem.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut sm: i32 = numbers[0..3].iter().sum();
    let mut count = 0;

    for idx in 3..numbers.len() {
        let new_sum = sm - numbers[idx - 3] + numbers[idx];
        
        if new_sum > sm {
            count += 1;
        }
        sm = new_sum;
    }

    return count;
}
