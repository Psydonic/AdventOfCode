// Importing necessary modules for file operations and reading lines
use std::fs::File;
use std::io::{BufRead, BufReader};

mod xmas_state_machine;
use xmas_state_machine::{XmasStateMachine, XmasState};

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
    let count = word_searcher(&grid);
    println!("Count of 'XMAS' occurrences: {}", count);

    let count2 = mas_cross_searcher(&grid);
    println!("Count of 'MAS' cross occurences: {}", count2);
}

fn mas_cross_searcher(grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for i in 1..grid.len()-1 {
        for j in 1..grid.len()-1 {

            let c = grid[i][j];
        
            if c == 'A' {
                let tl = grid[i-1][j-1];
                let tr = grid[i-1][j+1];
                let bl = grid[i+1][j-1];
                let br = grid[i+1][j+1];

                if (
                    (tl == 'M' && tr == 'M' && bl == 'S' && br == 'S') ||
                    (tl == 'M' && tr == 'S' && bl == 'M' && br == 'S') ||
                    (tl == 'S' && tr == 'S' && bl == 'M' && br == 'M') ||
                    (tl == 'S' && tr == 'M' && bl == 'S' && br == 'M') 
                    ) {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn word_searcher(grid: &Vec<Vec<char>>) -> usize {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1),
    ];

    let mut count = 0;
    let mut xmas_state_machine = XmasStateMachine::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for direction in directions.iter() {

                let mut x = i as i32;
                let mut y = j as i32;

                // Move in the specified direction
                for _ in 0..4 { // Adjusted to check the next 3 characters
                    println!("state: {:?}, {},{}, dir: {:?}, next: {}", xmas_state_machine.state, x, y, direction, grid[x as usize][y as usize]);
                    // if match fails then stop
                    if !xmas_state_machine.transform(grid[x as usize][y as usize]) {
                        break;
                    }

                    // if matched the full word then increment count
                    if xmas_state_machine.state == XmasState::XMAS {
                        count += 1;
                        println!("Found xmas at {},{}", x, y);
                        break;
                    }

                    x += direction.0;
                    y += direction.1;

                    // if off screen then stop
                    if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[i].len() as i32 {
                        break;
                    }
                }
            }
        }
    }
    count
}
