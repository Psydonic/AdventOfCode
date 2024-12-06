#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn to_tuple(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    pub fn rotate_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Guard {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub visited: std::collections::HashSet<(i32, i32, Direction)>,
    pub start_x: i32,
    pub start_y: i32,
}

impl Guard {
    pub fn new(x: i32, y: i32, direction: Direction) -> Guard {
        Guard {
            x,
            y,
            direction,
            visited: std::collections::HashSet::new(),
            start_x: x,
            start_y: y,
        }
    }

    pub fn reset(&mut self) {
        self.x = self.start_x;
        self.y = self.start_y;
        self.direction = Direction::North;
        self.visited.clear();
    }

    pub fn advance(&mut self, obstacles: &Vec<Obstacle>, screen_width: i32, screen_height: i32) -> Result<(), std::io::Error> {
        let (dx, dy) = self.direction.to_tuple();
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if new_x < 0 || new_x >= screen_width || new_y < 0 || new_y >= screen_height {
            self.visited.insert((self.x, self.y, self.direction));
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Guard went off screen"
            ).into());
        }

        if !obstacles.iter().any(|o| o.x == new_x && o.y == new_y) {
            self.visited.insert((self.x, self.y, self.direction));

            self.x = new_x;
            self.y = new_y;
            if self.visited.contains(&(new_x, new_y, self.direction)) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Infinite loop detected"
                ));
            }
            Ok(())
        } else {
            self.direction = self.direction.rotate_right();
            return self.advance(obstacles, screen_width, screen_height);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Obstacle {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub guard: Guard,
    pub obstacles: Vec<Obstacle>,
}

impl Board {
    pub fn new(grid: Vec<Vec<char>>) -> Board {
        let mut guard = None;
        let mut obstacles = Vec::new();
        
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    '#' => obstacles.push(Obstacle { x: x as i32, y: y as i32 }),
                    '^' => guard = Some(Guard::new(x as i32, y as i32, Direction::North)),
                    _ => (),
                }
            }
        }
        let guard = guard.expect("No guard found in grid");
        Board {
            width: grid[0].len() as i32,
            height: grid.len() as i32,
            guard,
            obstacles,
        }
    }

    pub fn advance(&mut self) -> Result<(), std::io::Error> {
        self.guard.advance(&self.obstacles, self.width, self.height)
    }
}
