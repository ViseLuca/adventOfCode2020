use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use regex::Regex;


fn main() {
    let mut input = String::new();
    let _read_result = File::open("/Users/luca/prima-projects/adventOfCode2020/advent-of-code/src/bin/day4/input.txt")
        .unwrap()
        .read_to_string(&mut input);

    let string_vec: Vec<&str> = input.split("\n\n").collect();

    let valid_passports: usize = check_passports(string_vec);
    println!("{}", valid_passports);
}

fn has_required_fields(passport: &str) -> bool {
    let regex_HCL: Regex = Regex::new(r"^#[0-9a-f]{6,6}$").unwrap();
    let regex_ECL: Regex = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    let regex_PID: Regex = Regex::new(r"^\d{9,9}$").unwrap();
    let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut h_map = HashMap::new();
    let fields = passport
        .split_whitespace()
        .map(|field| {
            let key_value = field.split(':').collect::<Vec<&str>>();
            h_map.insert(key_value[0].to_string(), key_value[1].to_string());
            key_value[0]
        })
        .filter(|field| field != &"cid")
        .collect::<Vec<&str>>();


    if fields.len() == required_fields.len() {
        let mut res = true;
        for (key, value) in &h_map {
            let is_valid = match key.as_str() {
                "byr" => check_byr(value),
                "iyr" => check_iyr(value),
                "eyr" => check_eyr(value),
                "hgt" => check_hgt(value),
                "hcl" => regex_HCL.is_match(value),
                "ecl" => regex_ECL.is_match(value),
                "pid" => regex_PID.is_match(value),
                "cid" => true,
                _ => false
            };

            res &= is_valid;
        }
        res
    } else { false }
}

fn check_byr(value: &str) -> bool {
    let num_value = value.to_string().parse::<i32>().unwrap();
    if num_value >= 1920 && num_value <= 2002 { true } else { false }
}

fn check_iyr(value: &str) -> bool {
    let num_value = value.to_string().parse::<i32>().unwrap();
    if num_value >= 2010 && num_value <= 2020 { true } else { false }
}

fn check_eyr(value: &str) -> bool {
    let num_value = value.to_string().parse::<i32>().unwrap();
    if num_value >= 2020 && num_value <= 2030 { true } else { false }
}

fn check_hgt(value: &str) -> bool {
    let value = value.to_string();
    if value.ends_with("in") {
        let val = value.strip_suffix("in").unwrap().parse::<i32>().unwrap();
        val >= 59 && val <= 76
    } else if value.ends_with("cm") {
        let val = value.strip_suffix("cm").unwrap().parse::<i32>().unwrap();
        val >= 150 && val <= 193
    } else {false }
}


fn check_passports(input: Vec<&str>) -> usize {
    input
        .iter()
        .filter(|p| has_required_fields(p))
        .count()
}
