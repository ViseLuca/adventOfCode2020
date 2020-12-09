use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _read_result = File::open(
        "/Users/luca/prima-projects/adventOfCode2020/advent-of-code/src/bin/day5/input.txt",
    )
    .unwrap()
    .read_to_string(&mut input);

    let string_vec: Vec<&str> = input.split("\n").collect();

    check_seat(string_vec);
}

fn check_seat(vec: Vec<&str>) {
    let max = vec
        .iter()
        .filter(|phrase| !phrase.to_string().is_empty())
        .map(|phrase| {
            let final_row = get_value(phrase);

            final_row
        })
        .sorted()
        .tuple_windows::<(_, _, _)>()
        .find_map(|(p, c, n)| {
            if c == p + 2 && n == c + 1 {
                Some(p + 1)
            } else {
                None
            }
        })
        .expect("Could not find any row with a single empty seat!");

    println!("{}", max);
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
