// Define the file name
const FILE_NAME: &str = "resources/input.txt";

// Function to open the file
fn open_file(file_name: &str) -> Result<std::fs::File, std::io::Error> {
    std::fs::File::open(file_name)
}

// Function to parse the file into two lists
fn parse_file_into_lists(file: std::fs::File) -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    use std::io::{BufRead, BufReader};

    let reader = BufReader::new(file);
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    // Parse each line into two lists
    for line in reader.lines() {
        let line = line?;
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid line format"));
        }
        list1.push(parts[0].parse::<i32>().unwrap_or(0));
        list2.push(parts[1].parse::<i32>().unwrap_or(0));
    }

    Ok((list1, list2))
}

// Function to calculate the sum of differences between two sorted lists
fn sum_of_differences(list1: &[i32], list2: &[i32]) -> i32 {
    let mut sum = 0;
    let mut list1 = list1.to_vec(); // Create a mutable copy
    let mut list2 = list2.to_vec(); // Create a mutable copy
    list1.sort();
    list2.sort();
    for (a, b) in list1.iter().zip(list2.iter()) {
        sum += (a - b).abs();
    }
    sum
}

// Function to calculate the similarity score between two lists
fn similarity_score(list1: &[i32], list2: &[i32]) -> i32 {
    let mut score = 0;
    for n in list1 {
        let o = list2.iter().filter(|&&x| x == *n).count() as i32;
        score += n * o;
    }
    score
}


// Main function
fn main() {
    // Step 1: Open the file
    let file = match open_file(FILE_NAME) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    // Step 2: Parse the file into two lists
    let (list1, list2) = match parse_file_into_lists(file) {
        Ok((list1, list2)) => (list1, list2),
        Err(e) => {
            eprintln!("Failed to parse file into lists: {}", e);
            return;
        }
    };

    // Step 3: Calculate the sum of differences for each pair of lists
    println!("Total sum of differences: {}", sum_of_differences(&list1, &list2));

    // Step 4: Calculate the similarity difference
    println!("Similarity score: {}", similarity_score(&list1, &list2));

}
