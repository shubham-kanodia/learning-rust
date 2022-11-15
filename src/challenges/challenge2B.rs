use std::fs;

struct Position {
    x: i32,
    d: i32,
    aim: i32
}

pub fn solve() -> (i32, i32){
    let file_path = String::from("../inputs/challenge2B.txt");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let mut position = Position{x: 0, d: 0, aim:0};
    
    let directions: Vec<&str> = file_contents.split('\n').collect();

    for direction in directions {
        let split_direction: Vec<&str> = direction.split_whitespace().collect();
        let change = split_direction[1].parse::<i32>().unwrap();

        match split_direction[0] {
            "forward" => {
                position.x += change;
                position.d += change * position.aim;
            },
            "up" => {
                position.aim -= change;
            },
            "down" => {
                position.aim += change;
            },
            _ => println!()
        }
    }
    return (position.x, position.d);
}