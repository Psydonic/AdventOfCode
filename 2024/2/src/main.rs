// Define the file name
const FILE_NAME: &str = "resources/input.txt";

// Function to open the file
fn open_file(file_name: &str) -> Result<std::fs::File, std::io::Error> {
    std::fs::File::open(file_name)
}

// Function to parse the file into a list of lists, one list for each line, containing positive digits separated by space
fn parse_file_into_lists(file: std::fs::File) -> Result<Vec<Vec<i32>>, std::io::Error> {
    use std::io::{BufRead, BufReader};

    let reader = BufReader::new(file);
    let mut list_of_lists = Vec::new();

    // Parse each line into a list of positive digits
    for line in reader.lines() {
        let line = line?;
        let parts = line.split_whitespace()
                         .filter_map(|part| part.parse::<i32>().ok())
                         .filter(|&num| num > 0)
                         .collect::<Vec<_>>();
        list_of_lists.push(parts);
    }

    Ok(list_of_lists)
}

// Function to check if the numbers in the list are all increasing or all decreasing
fn is_safe(vector: &Vec<i32>) -> bool {
    check_order(vector) && check_rate(vector)
}

fn check_rate(vector: &Vec<i32>) -> bool {
    let differences: Vec<i32> = vector.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
    differences.iter().all(|&diff| diff >= 1 && diff <= 3)
}


fn check_order(vector: &Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;
    let mut prev = vector[0];

    for &num in vector.iter().skip(1) {
        if num > prev {
            is_decreasing = false;
        } else if num < prev {
            is_increasing = false;
        }
        prev = num;
    }

    is_increasing && !is_decreasing || is_decreasing && !is_increasing
}

fn generate_sublists(vector: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut sublists = Vec::new();
    for i in 0..vector.len() {
        let mut sublist = vector.clone();
        sublist.remove(i);
        sublists.push(sublist);
    }
    sublists
}

fn is_any_sublist_safe(vector: &Vec<i32>) -> bool {
    let sublists = generate_sublists(vector);
    sublists.iter().any(|sublist| is_safe(sublist))
}

fn main() {
    // Step 1: Open the file
    let file = match open_file(FILE_NAME) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    // Step 2: Parse the file into a list of lists
    let lists = match parse_file_into_lists(file) {
        Ok(lists) => lists,
        Err(e) => {
            eprintln!("Failed to parse file: {}", e);
            return;
        }
    };

    // Step 3: Calculate the number of lists that are safe
    let safe_lists_count = lists.iter().filter(|list| is_safe(list)).count();
    
    println!("Number of safe lists: {}", safe_lists_count);

    // Step 4: Create permutations for each list
    let safe_lists_count = lists.iter().filter(|list| is_any_sublist_safe(list)).count();
    
    println!("Number of safe lists v2: {}", safe_lists_count);
    
}
