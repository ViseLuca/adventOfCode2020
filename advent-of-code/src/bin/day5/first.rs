use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _read_result = File::open("input.txt").unwrap().read_to_string(&mut input);

    let string_vec: Vec<&str> = input.split("\n").collect();

    let total: Option<usize> = check_seat(string_vec);
    println!("{:?}", total);
}

fn check_seat(vec: Vec<&str>) -> Option<usize> {
    let max = vec
        .iter()
        .filter(|phrase| !phrase.to_string().is_empty())
        .map(|phrase| {
            let final_row = get_value(phrase);

            final_row
        })
        .max();

    max
}

fn get_value(phrase: &&str) -> usize {
    let (row, col) = phrase.split_at(7);
    let row = row.chars().fold(0, |acc, c| match c {
        'F' => acc * 2,
        'B' => acc * 2 + 1,
        _ => panic!("Invalid boarding pass!"),
    });
    let col = col.chars().fold(0, |acc, c| match c {
        'L' => acc * 2,
        'R' => acc * 2 + 1,
        _ => panic!("Invalid boarding pass!"),
    });
    row * 8 + col
}
