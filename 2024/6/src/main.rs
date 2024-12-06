use std::fs::File;
use std::io::{BufRead, BufReader};

mod objects;
use objects::{Board, Obstacle};
 
const FILE_NAME: &str = "resources/input.txt";

fn load_file_into_board(file_path: &str) -> Board {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| line.expect("Failed to read line").chars().collect())
        .collect();

    Board::new(grid)
}

fn part_one(board: &mut Board) {
    let mut visited = std::collections::HashSet::new();
    visited.insert((board.guard.x, board.guard.y)); // Ensure the start spot is added to the list

    while board.advance().is_ok() {}

    println!("The guard visited {} unique locations.", 
        board.guard.visited.iter().map(|(x, y, _)| (x, y)).collect::<std::collections::HashSet<_>>().len());
}

fn part_two(board: &mut Board) {
    // Initialize a counter for obstacles that cause the advance loop to panic
    let mut panic_obstacles_count = 0;

    // Iterate over each position on the board
    for x in 0..board.width {
        for y in 0..board.height {
            let mut board_clone = board.clone();
            // Check if there's no obstacle at the current position
            if !board_clone.obstacles.iter().any(|o| o.x == x && o.y == y) {
                if is_loopable(&mut board_clone, x, y) {
                    println!("Location ({}, {}) causes the advance loop to panic.", x, y);
                    panic_obstacles_count += 1;
                }
            }
        }
    }

    // Print the total count of obstacles that cause the advance loop to panic
    println!("The number of obstacles that cause the advance loop to panic is: {}", panic_obstacles_count);
}

fn is_loopable(board: &mut Board, x: i32, y: i32) -> bool {
    board.obstacles.push(Obstacle { x, y }); 

    loop {
        match board.advance() {
            Ok(_) => continue,
            Err(e) if e.kind() == std::io::ErrorKind::Other => {
                return true; // Loop detected
            },
            _ => {
                return false; // No loop detected
            },
        }
    }
}

fn main() {
    let board = load_file_into_board(FILE_NAME);
    
    part_one(&mut board.clone());
    part_two(&mut board.clone());
   
}
