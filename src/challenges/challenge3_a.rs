use std::fs;

fn get_most_frequent_bit(bits: Vec<char>) -> (char, char) {
    let ones_count = bits.iter().filter(|&elem| *elem == '1').count();

    if ones_count >= bits.len() / 2 {
        return ('1', '0');
    } else {
        return ('0', '1');
    }
}

pub fn solve() -> (i32, i32) {
    let file_path = String::from("../inputs/challenge3A.txt");
    let file_contents = fs::read_to_string(file_path).unwrap();
    let binary_numbers: Vec<&str> = file_contents.split('\n').collect();

    let num_of_chars = binary_numbers[0].len();

    let mut gamma_rate_str = String::from("");
    let mut epsilon_rate_str = String::from("");

    for idx in 0..num_of_chars {
        let nth_bits: Vec<char> = binary_numbers.iter().map(|&elem| elem.chars().nth(idx).unwrap()).collect();
        let (most, least) = get_most_frequent_bit(nth_bits);
        
        gamma_rate_str.push(most);
        epsilon_rate_str.push(least);

    }

    let gamma_value = i32::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_value = i32::from_str_radix(&epsilon_rate_str, 2).unwrap();

    return (gamma_value, epsilon_value);
    
}
