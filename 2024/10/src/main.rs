use std::collections::HashSet;

const FILENAME: &str = "resources/input.txt";

fn parse_file(filename: &str) -> Vec<Vec<u32>> {
    use std::fs;

    let content = fs::read_to_string(filename).expect("Error opening file");

    return content
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    // count number of paths from this point
    fn find_unique_destinations(&self, grid: &Vec<Vec<u32>>, current_value: u32) -> HashSet<Point> {
        
        // Base case if out of bounds
        if !self.is_valid(grid) {
            return HashSet::new();
        }

        // Base case if value is not current + 1
        if self.at(grid) != current_value {
            return HashSet::new();
        }

        // Base case if path is finished
        if self.at(grid) == 9 {
            return vec![self.clone()].into_iter().collect();
        }

        // Recuse on all neightbours
        return self
            .neighbours()
            .iter()
            .flat_map(|p| p.find_unique_destinations(grid, current_value + 1))
            .collect();
    }

    fn count_unique_paths(&self, grid: &Vec<Vec<u32>>, current_value: u32) -> usize {
        
        // Base case if out of bounds
        if !self.is_valid(grid) {
            return 0;
        }

        // Base case if value is not current + 1
        if self.at(grid) != current_value {
            return 0;
        }

        // Base case if path is finished
        if self.at(grid) == 9 {
            return 1;
        }

        // Recuse on all neightbours
        return self
            .neighbours()
            .iter()
            .map(|p| p.count_unique_paths(grid, current_value + 1))
            .sum();
    }

    fn is_valid(&self, grid: &Vec<Vec<u32>>) -> bool {
        let w = grid.get(0).unwrap().len().try_into().unwrap();
        let h = grid.len().try_into().unwrap();
        return self.x >= 0 && self.y >= 0 && self.x < w && self.y < h;
    }

    fn at(&self, grid: &Vec<Vec<u32>>) -> u32 {
        return *grid
            .get(self.y as usize)
            .unwrap()
            .get(self.x as usize)
            .unwrap();
    }

    fn neighbours(&self) -> Vec<Point> {
        return vec![
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
        ];
    }
}

fn main() {
    let grid = parse_file(FILENAME);

    let count: usize = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row
                .iter()
                .enumerate()
                .map(|(x, _n)| {
                    let p = Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        };
                    p.find_unique_destinations(&grid, 0).len()
                })
                .sum::<usize>()
        })
        .sum();

    println!("Part 1 Count: {}", count);

    let count: usize = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row
                .iter()
                .enumerate()
                .map(|(x, _n)| {
                    let p = Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        };
                    p.count_unique_paths(&grid, 0)
                })
                .sum::<usize>()
        })
        .sum();

    println!("Part 2 Count: {}", count);
}
