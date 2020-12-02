use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _read_result = File::open("second.txt")
        .unwrap()
        .read_to_string(&mut input);

    let mut count_valid_passwords = 0;

    let rows_input: Vec<&str> = input.split("\n").collect();
    for i in 0..rows_input.len() - 1 {
        let row: Vec<&str> = rows_input[i].split(" ").collect();
        let positions: Vec<&str> = row[0].split("-").collect();

        let first_pos: usize = positions[0].parse::<usize>().unwrap() -1;
        let second_pos: usize = positions[1].parse::<usize>().unwrap() -1;

        let character: &char = &(row[1].replace(":", "").to_string().chars().collect::<Vec<char>>()[0]);
        let password: Vec<char> = row[2].to_string().chars().collect();

        let first_char: &char = &(password[first_pos]);
        let second_char: &char = &(password[second_pos]);

        if first_char == character && second_char != character {
            count_valid_passwords += 1;
        } else if first_char != character && second_char == character {
            count_valid_passwords += 1;
        }
    }

    println!("Valid passwords are: {}",count_valid_passwords);
}
