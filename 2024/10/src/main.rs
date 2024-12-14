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
    trails: Vec<Trail>
}

impl Trailhead {
    fn new(x: u16, y:  u16, grid: &Vec<Vec<u32>>) -> Self {
        let p = Point{x, y};

        // find all trails to a 9
        let trails:Vec<Trail> = find_trails(&p, 9, grid);

        Self{trails: trails}
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
        sub_trails
            .iter()
            .map(|trail| )
        return sub_trails;
    }
}

fn step(trails: Vec<Trail>, grid: &Vec<Vec<u32>>) -> Vec<Trail> {
    for trail in trails {
        let last = trail.path.last().;
        let height = trail.path.len();

        // find the surrounding points with height
        let surrounding = vec![
            last.above(), 
            last.left(), 
            last.below(grid.len() as u16), 
            last.right(grid.get(0).unwrap().len() as u16)];
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
    fn above(&self) -> Option<Point> {
        if self.y <= 0 {
            None
        } else {
            Some(Self{x: self.x, y: self.y - 1})
        }
    } 

    fn below(&self, limit: u16) -> Option<Point> {
        if self.y >= limit {
            None
        } else {
            Some(Self{x: self.x, y: self.y + 1})
        }
    } 

    fn left(&self) -> Option<Point> {
        if self.x <= 0 {
            None
        } else {
            Some(Self{x: self.x - 1, y: self.y})
        }
    }
    
    fn right(&self, limit: u16) -> Option<Point> {
        if self.x >= limit {
            None
        } else {
            Some(Self{x: self.x + 1, y: self.y})
        }
    }  
}

fn calculate_trailheads(grid: &Vec<Vec<u32>>) -> Vec<Trailhead> {
    
    // find the zeros
    return grid.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &n)| {
            if n == 0 {
                // build a trailhead here
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
