const FILENAME: &str = "resources/test.txt";

fn parse_file(filename: &str) -> Vec<u32> {
    use std::fs;

    let content = fs::read_to_string(filename)
        .expect("failed to open file");

    content
        .split(" ")
        .into_iter()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn main() {
    let stones = parse_file(FILENAME);
    println!("{:?}", stones);
}
