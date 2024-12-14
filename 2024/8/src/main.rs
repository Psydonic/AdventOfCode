use std::collections::{HashMap, HashSet};
use itertools::Itertools;

const FILENAME: &str = "resources/input.txt";

const EXTENSION: i32 = 200;

#[derive(Debug, Clone, Hash, Eq, PartialEq)] 
struct Antenna {
    x: i32,
    y: i32,
    freq: char
}

struct Map {
    antennae: HashMap<char, Vec<Antenna>>,
    width: i32,
    height: i32
}

impl Map {

    fn new(filename: &str) -> Map {
        use std::fs;
        let content: String = fs::read_to_string(filename).expect("Failed to read file");
        
        // iterate over the file 
        let mut antennae: Vec<Antenna> = vec![];
        for (y, line) in content.lines().enumerate() {
            for (x, cha) in line.chars().enumerate() {
                if cha != '.' {
                    antennae.push(Antenna{x: x as i32, y: y as i32, freq: cha});
                }
            }
        }
        
        let antennae = Map::group(antennae);
        let width = content.lines().next().expect("").len().try_into().unwrap();
        let height =  content.lines().count().try_into().unwrap();

        return Map{antennae, width, height};
    }

    fn group(antennae: Vec<Antenna>) -> HashMap<char, Vec<Antenna>> {
        let mut grouped_antennae: HashMap<char, Vec<Antenna>> = HashMap::new();
        
        for antenna in antennae {
            grouped_antennae
                .entry(antenna.freq)
                .or_insert_with(Vec::new)
                .push(antenna.clone());
        }

        return grouped_antennae;
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct AntiNode {
    x: i32,
    y: i32  // signed becuase could be off screen
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Pair {
    antenna1: Antenna,
    antenna2: Antenna,
    antinodes: Vec<AntiNode>
}

impl Pair {

    fn new(antenna1: Antenna, antenna2: Antenna) -> Pair {
        //calculate antenna diff 
        let dx = antenna1.x - antenna2.x;
        let dy = antenna1.y - antenna2.y;

        let mut antinodes: Vec<AntiNode> = Vec::new();
        for i in 0..EXTENSION {
            antinodes.push(AntiNode{x: antenna1.x + (i * dx), y: antenna1.y + (i * dy)});
            antinodes.push(AntiNode{x: antenna2.x - (i * dx), y: antenna2.y - (i * dy)});
        }

        return Pair{antenna1, antenna2, antinodes};
    }
}

fn main() {
    let map = Map::new(FILENAME);
    
    // create pair combinations for each group
    let mut antennae_pairings: HashSet<Pair> = HashSet::new();
    for (_key, group) in map.antennae {
        let group_pairings: HashSet<Pair> = group
            .iter()
            .combinations(2)
            .map(|pair| return Pair::new(
                pair[0].clone(), pair[1].clone(),
            ))
            .collect();
        antennae_pairings.extend(group_pairings);
    }

    // extract the antinates from all pairs
    let mut antinodes = HashSet::new();
    for pair in antennae_pairings {
        for antinode in pair.antinodes {
            if (antinode.x < map.width) && (antinode.x >= 0) && (antinode.y < map.height) && (antinode.y >= 0) {
                antinodes.insert(antinode);
            }
        }
    }
    
    // for each pair, calculate antinodes andd add
    println!("{:?}", antinodes.len());
}
