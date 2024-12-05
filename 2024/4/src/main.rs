// Importing necessary modules for file operations and reading lines
use std::fs::File;
use std::io::{BufRead, BufReader};
use xmas_state_machine::StateMachine;

// Defining the path to the input file
const INPUT_FILE: &str = "resources/input.txt";

// Function to read a file and convert its content into a 2D array of characters
fn read_file_to_2d_array(file_path: &str) -> Vec<Vec<char>> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let first_line = lines.next().expect("File is empty").expect("Failed to read line");

    let num_cols = first_line.len();
    let mut grid = Vec::with_capacity(num_cols);
    grid.push(first_line.chars().collect());
    
    for line in lines {
        let line = line.expect("Failed to read line");
        if line.len() != num_cols {
            panic!("All lines must have the same number of characters");
        }
        grid.push(line.chars().collect());
    }
    grid
}

// Main function
fn main() {
    let grid = read_file_to_2d_array(INPUT_FILE);
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let first_line = grid[0].iter().collect::<String>();

    let count = word_searcher(&grid);
    println!("Count of 'XMAS' occurrences: {}", count);
}

fn word_searcher(grid: &Vec<Vec<char>>) -> usize {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1),
    ];

    let mut count = 0;
    let mut xmas_state_machine = StateMachine::new("XMAS");
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for direction in &directions {

                let mut x = i as i32;
                let mut y = j as i32;
                // Check the current tile first
                if xmas_state_machine.next(grid[i][j]) {
                    // Move in the specified direction
                    for _ in 0..3 { // Adjusted to check the next 3 characters
                        x += direction.0;
                        y += direction.1;
                        if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[i].len() as i32 {
                            break;
                        }
                        if xmas_state_machine.next(grid[x as usize][y as usize]) {
                            count += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    count
}
