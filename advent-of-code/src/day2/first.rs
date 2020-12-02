use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _read_result = File::open("first.txt")
        .unwrap()
        .read_to_string(&mut input);

    let mut count_valid_passwords = 0;

    let rows_input: Vec<&str> = input.split("\n").collect();
    for i in 0..rows_input.len() - 1 {
        let row: Vec<&str> = rows_input[i].split(" ").collect();
        let range: Vec<&str> = row[0].split("-").collect();

        let min: i32 = range[0].parse().unwrap();
        let max: i32 = range[1].parse().unwrap();


        let character: char = row[1].replace(":", "").to_string().chars().collect::<Vec<char>>()[0];
        let password: String = row[2].to_string();

        let count_char: i32 = password
            .chars()
            .into_iter()
            .filter(|letter| letter == &character)
            .count() as i32
            ;

        if count_char <= max && count_char >= min {
            count_valid_passwords += 1;
        }
    }

    println!("Valid passwords are: {}",count_valid_passwords);
}
