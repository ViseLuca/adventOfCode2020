use std::fs::File;
use std::io::Read;


fn main() {
    let mut input = String::new();
    let _read_result = File::open("/Users/luca/prima-projects/adventOfCode2020/advent-of-code/src/bin/day5/input.txt")
        .unwrap()
        .read_to_string(&mut input);

    let string_vec: Vec<&str> = input.split("\n").collect();

    let total: Option<i32> = check_seat(string_vec);
    println!("{:?}", total);
}

fn check_seat(vec: Vec<&str>) -> Option<i32> {
    let max = vec
        .iter()
        .filter(|phrase| !phrase.to_string().is_empty())
        .map(|phrase| {
            let vec_char: Vec<char> = phrase
                .chars()
                .collect();
            let mut max_row = 127;
            let mut min_row = 0;
            let mut final_row = 0;
            for ix in 0..6 {
                if vec_char[ix] == 'F' {
                    max_row = &max_row / 2;
                    if ix == 6 {
                        final_row = max_row
                    }
                } else {
                    min_row = &min_row + (&max_row / 2);
                    if ix == 6 {
                        final_row = min_row;
                    }
                }
            }
            let mut max_seat = 8;
            let mut min_seat = 0;
            let mut final_seat = 0;
            for ix in 7..9 {
                if vec_char[ix] == 'R' {
                    max_seat = &max_seat / 2;
                    if ix == 9 { final_seat = max_seat }
                } else {
                    min_seat = &min_seat + (&max_seat / 2);
                    if ix == 9 { final_seat = min_seat }
                }
            }

            final_row * 8 + final_seat
        })
        .max();

    max
}
