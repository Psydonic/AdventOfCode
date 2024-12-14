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
    paths: u32
}

impl Trailhead {
    fn new(x: u16, y:  u16, grid: &Vec<Vec<u32>>) -> Self {
        let p = Point{x, y};

        // find all trails from point to a 9 in the grid
        let trails:Vec<Trail> = find_trails(&p, 9, grid);

        Self{start: p, trails: trails}
    }

}

// find all directions from the given point with the given height
fn find_trails(p: &Point, height: u8,  grid: &Vec<Vec<u32>>) -> Vec<Trail> {
    if height == 1 {
        let ones = vec![
            p.above(), 
            p.left(), 
            p.below(grid.len() as u16), 
            p.right(grid.get(0).unwrap().len() as u16)];

        let ones: Vec<Point> = ones
            .into_iter()
            .filter_map(|p| p)
            .collect();

        return ones.into_iter().map(|p1| {
            Trail{path: vec![p.clone(), p1]}
        }).collect();
    } else {
        // find all trails to hieght -1
        let sub_trails = find_trails(p, height - 1, grid);

        // for each sub trails, see if there is any valid trails ahead
        return step(trails, grid);
    }
}

// given a set of trails, advance them, removinf if failed
fn step(trails: Vec<Trail>, grid: &Vec<Vec<u32>>) -> Vec<Trail> {
    for trail in trails {
        let last = trail.path.last().expect("invalid starter trailhead");
        let height: u32 = trail.path.len().try_into().unwrap();

        // find the surrounding points with height
        let surrounding = vec![
            last.above(), 
            last.left(), 
            last.below(grid.len() as u16), 
            last.right(grid.get(0).unwrap().len() as u16)];
        let surrounding: Vec<Point> = surrounding
            .into_iter()
            .filter_map(|p| p)
            .filter(|p| p.in(grid) == height)
            .collect();
    }

    return vec![];
}

struct Trail {
    path: Vec<Point>
}

#[derive(Clone)]
struct Point {
    x: u16,
    y: u16  
}

impl Point {
    // count number of paths from this point
    fn count_paths(&self, grid: &Vec<Vec<u32>>, currentValue: u32) -> usize {
        // Base case if out of bounds
        if !self.is_valid(grid) {
            return 0;
        }

        // Base case if value is not current + 1
        if self.in(grid) != currentValue + 1:
            return 0;
        }

        // Base case if path is finished
        if self.in(grid) == 9 {
            return 1;
        }

        // Recuse on all neightbours
        return self.neighbours.count_paths(grid, self.in(grid)).sum();
    }
    
    fn is_valid(&self, grid: &Vec<Vec<u32>>) -> bool {
        let w = grid.get(0).unwrap().len();
        let h = grid.len();
        return (self.x < 0 || self.y < 0 || self.x >= w || self.y >= h);
    }

    fn in(&self, grid: &Vec<Vec<u32>>) -> u32{
        grid.get(self.y).unwrap(Vec<u32>).get(self.x).unwrap()
    }

    fn neighbours(&self, limit: usize) {
        let surroundings = vec![
            Point{}
        ]
    }
}

fn find_starting_points(grid: &Vec<Vec<u32>>) -> Vec<Point> {
    
    // find the zeros
    return grid.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &n)| {
            if n == 0 {
                // build a trailhead here
                Some(Point(
                        x.try_into().unwrap(), 
                        y.try_into().unwrap())
            } else {
                None
            }
        })
    })
    .collect();
}

fn main() { 
    let grid = parse_file(FILENAME);
    let startings_points = find_starting_points(&grid);
    let count: usize = startings_points
        .iter()
        .map(|p| p.count_paths(&grid, 0))
        .sum();

    println!("Count: {}", count);
}
