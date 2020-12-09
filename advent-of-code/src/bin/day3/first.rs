use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let _read_result = File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input);

    let string_vec: Vec<&str> = input.split("\n").collect();

    println!("{}", tree_count(string_vec, 3, 1));
}

fn tree_count(lines: Vec<&str>, right: usize, down: usize) -> i64 {
    let (trees, _) = lines
        .iter()
        .step_by(down)
        .fold((0, 0), |(trees, line_p), row| {
            let element = row.chars().cycle().nth(line_p);
            (
                trees + (element.map(|el| if el == '#' { 1 } else { 0 }).unwrap_or(0)),
                line_p + right,
            )
        });
    trees
}
