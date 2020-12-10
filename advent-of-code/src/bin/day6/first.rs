use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _read_result = File::open(
        "/Users/luca/prima-projects/adventOfCode2020/advent-of-code/src/bin/day6/input.txt",
    )
    .unwrap()
    .read_to_string(&mut input);

    let string_vec: Vec<&str> = input.split("\n\n").filter(|c| !c.is_empty()).collect();

    let total: usize = check_answers(string_vec);
    println!("{:?}", total);
}

fn check_answers(vec: Vec<&str>) -> usize {
    vec.into_iter()
        .map(|row| {
            row.split_whitespace()
                .flat_map(|p| p.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}
