use std::fs;

fn get_most_frequent_bit(bits: Vec<char>) -> (char, char) {
    let ones_count = bits.iter().filter(|&elem| *elem == '1').count();
    let zeros_count = bits.len() - ones_count;

    if ones_count >= zeros_count {
        return ('1', '0');
    } else {
        return ('0', '1');
    }
}

pub fn solve() -> (i32, i32){
    let file_path = String::from("../inputs/challenge3B.txt");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let mut binary_inputs: Vec<&str> = file_contents.split('\n').collect();
    let mut binary_input_or = binary_inputs.clone();

    let single_input_len = binary_input_or[0].len();

    let temp_string = String::from("");
    let mut oxygen_gen_rating = temp_string.as_str();

    for idx in 0..single_input_len {
        let current_idx_bits: Vec<char> = binary_input_or.iter().map(|elem| elem.chars().nth(idx).unwrap()).collect();

        let (most_frequent, _least_frequent) = get_most_frequent_bit(current_idx_bits);

        binary_input_or = binary_input_or.into_iter().filter(|&elem| elem.chars().nth(idx).unwrap() == most_frequent).collect();


        if binary_input_or.len() == 1 {
            oxygen_gen_rating = binary_input_or[0];
            break;
        }
    }

    let temp_string2 = String::from("");
    let mut co2_rating = temp_string2.as_str();

    for idx in 0..single_input_len {
        let current_idx_bits: Vec<char> = binary_inputs.iter().map(|elem| elem.chars().nth(idx).unwrap()).collect();

        let (_most_frequent, least_frequent) = get_most_frequent_bit(current_idx_bits);

        binary_inputs = binary_inputs.into_iter().filter(|&elem| elem.chars().nth(idx).unwrap() == least_frequent).collect();

        if binary_inputs.len() == 1 {
            co2_rating = binary_inputs[0];
            break;
        }
    }

    let oxygen_value = i32::from_str_radix(&oxygen_gen_rating, 2).unwrap();
    let co2_value = i32::from_str_radix(&co2_rating, 2).unwrap();

    return (oxygen_value, co2_value);
}