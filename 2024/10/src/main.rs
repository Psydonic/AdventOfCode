const FILENAME: &str = "resources/test.txt";

fn parse_file(filename: &str) -> Vec<Vec<u32>> {
    use std::fs;

    let content = fs::read_to_string(filename)
        .expect("Error opening file");

    return content
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        })
        .collect();
}

struct Trailhead {
    start: Point,
    trails: Vec<Trail>
}

impl Trailhead {
    fn new(x: u16, y:  u16, grid: &Vec<Vec<u32>>) -> Self {
        let p = Point{x, y};
        Self{start: p, trails: vec![]}
    }
}


struct Trail {
    path: Vec<Point>
}

struct Point {
    x: u16,
    y: u16
}

fn calculate_trailheads(grid: &Vec<Vec<u32>>) -> Vec<Trailhead> {
    
    // find the zeros
    return grid.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &n)| {
            if n == 0 {
                Some(Trailhead::new(
                        x.try_into().unwrap(), 
                        y.try_into().unwrap(),
                        &grid))
            } else {
                None
            }
        })
    })
    .collect();
}

fn main() { 
    let grid = parse_file(FILENAME);
    let trailheads = calculate_trailheads(&grid);
    let count: usize = trailheads
        .iter()
        .map(|t| t.trails.len())
        .sum();

    println!("Count: {}", count);
}
