use std::collections::{HashMap, HashSet};
use itertools::Itertools;

const FILENAME: &str = "resources/input.txt";

#[derive(Debug, Clone, Hash, Eq, PartialEq)] 
struct Antenna {
    x: i32,
    y: i32,
    freq: char
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
    antinode1: AntiNode, // pair is the owner of the antinodes
    antinode2: AntiNode
}

impl Pair {

    fn new(antenna1: Antenna, antenna2: Antenna) -> Pair {
        //calculate antenna diff 
        let dx = antenna1.x - antenna2.x;
        let dy = antenna1.y - antenna2.y;

        let antinode1 = AntiNode{x: antenna1.x - dx, y: antenna1.y - dy};
        let antinode2 = AntiNode{x: antenna2.x + dx, y: antenna2.y + dy};

        return Pair{antenna1, antenna2, antinode1, antinode2};
    }
}

fn parse_file(filename: &str) -> Vec<Antenna> {
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

    return antennae;
}

fn group_and_pair(antennae: Vec<Antenna>) -> HashSet<Pair> {
    let mut grouped_antennae: HashMap<char, Vec<Antenna>> = HashMap::new();
    
    for antenna in antennae {
        grouped_antennae
            .entry(antenna.freq)
            .or_insert_with(Vec::new)
            .push(antenna.clone());
    }

    let mut antennae_pairings: HashSet<Pair> = HashSet::new();
    for (_key, group) in &grouped_antennae {
        let group_pairings: HashSet<Pair> = group
            .iter()
            .combinations(2)
            .map(|pair| return Pair::new(
                pair[0].clone(), pair[1].clone(),
            ))
            .collect();
        antennae_pairings.extend(group_pairings);
    }
 
    return antennae_pairings;
}

fn main() {
    let antennae = parse_file(FILENAME);
    
    // create pair combinations for each group
    let antennae_pairings = group_and_pair(antennae);

    // extract the antinates from all pairs
    let mut antinodes = HashSet::new();
    for pair in antennae_pairings {
        antinodes.insert(pair.antinode1.clone());
        antinodes.insert(pair.antinode2.clone());
    }

    // for each pair, calculate antinodes andd add
    println!("{:?}", antinodes.len());
}
