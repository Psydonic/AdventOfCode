
const FILENAME: &str = "resources/input.txt";

// load the file and parse it
fn parse_file(filename: &str) {
    use std::fs;
    
    let content: String = fs::read_to_string(filename)
        .expect("Error opening file");

    println!("{}", content);
}

fn main() {
    parse_file(FILENAME);
    println!("Hello, world!");
}


