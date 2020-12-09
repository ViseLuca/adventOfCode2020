use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _read_result = File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input);

    let string_vec: Vec<&str> = input.split("\n\n").collect();

    let valid_passports: usize = check_passports(string_vec);
    println!("{}", valid_passports);
}

fn has_required_fields(passport: &str) -> bool {
    let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let fields = passport
        .split_whitespace()
        .map(|field| field.split(':').collect::<Vec<&str>>()[0])
        .filter(|field| field != &"cid")
        .collect::<Vec<&str>>();


    fields.len() == required_fields.len()
}

fn check_passports(input: Vec<&str>) -> usize {
    input
        .iter()
        .filter(|p| has_required_fields(p))
        .count()
}
