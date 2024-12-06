const FILENAME: &str = "resources/input.txt";
const SPLIT_INDEX: usize = 1176; 

// load the file and parse it
fn parse_file(filename: &str) -> Vec<String> {
    use std::fs;
    
    let content = fs::read_to_string(filename)
        .expect("Error opening file");

    content.split("\r\n").map(String::from).filter(|line| !line.is_empty()).collect()
}

fn parse_tuple(input: &str) -> (u32, u32) {
    let parts: Vec<&str> = input.split('|').collect();
    let first: u32 = parts[0].parse().expect("Invalid number");
    let second: u32 = parts[1].parse().expect("Invalid number");
    (first, second)
}

fn parse_integers(input: &str) -> Vec<u32> {
    input.split(',').map(|s| s.parse().expect("Invalid number")).collect()
}

use std::collections::HashMap;

fn compress_rules(rules: &Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
    let mut compressed: HashMap<u32, Vec<u32>> = HashMap::new();
    for (a, b) in rules {
        compressed.entry(*a).or_insert(Vec::new()).push(*b);
    }
    compressed
}

fn check_rules(integers: &Vec<u32>, compressed_rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (a, bs) in compressed_rules {
        if let Some(index_a) = integers.iter().position(|&x| x == *a) {
            for b in bs {
                if let Some(index_b) = integers.iter().position(|&x| x == *b) {
                    if index_a > index_b {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn fix(mut integers: Vec<u32>, compressed_rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    loop {
        let mut violation_detected = false;
        for (a, bs) in compressed_rules {
            if let Some(index_a) = integers.iter().position(|&x| x == *a) {
                for b in bs {
                    if let Some(index_b) = integers.iter().position(|&x| x == *b) {
                        if index_a < index_b {
                            integers.swap(index_a, index_b);
                            violation_detected = true;
                            break;
                        }
                    }
                }
                if violation_detected {
                    break;
                }
            }
        }
        if !violation_detected {
            break;
        }
    }
    integers
}

fn get_middle_value(integers: &Vec<u32>) -> Option<u32> {
    let length = integers.len();
    if length == 0 {
        None
    } else {
        Some(integers[length / 2])
    }
}

fn main() {
    let lines = parse_file(FILENAME);
    let (first_half, second_half) = lines.split_at(SPLIT_INDEX);

    let parsed_tuples: Vec<(u32, u32)> = first_half.iter().map(|line| parse_tuple(line)).collect();
    let parsed_integers: Vec<Vec<u32>> = second_half.iter().map(|line| parse_integers(line)).collect();

    let compressed_rules = compress_rules(&parsed_tuples);
    
    println!("{:?}", compressed_rules); // Print out the compressed rules

    let count: u32 = parsed_integers.iter()
        .filter(|integers| check_rules(integers, &compressed_rules))
        .flat_map(|integers| get_middle_value(integers))
        .sum();

    println!("Total count: {}", count);

    let count2: u32 = parsed_integers.iter()
        .filter(|integers| !check_rules(integers, &compressed_rules))
        .map(|integers| fix(integers.clone(), &compressed_rules))
        .flat_map(|integers| get_middle_value(&integers))
        .sum();

    println!("Total count2: {}", count2);
    
}


