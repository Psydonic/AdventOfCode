const FILENAME: &str = "resources/input.txt";
const ITER: u32 = 40;

fn parse_file(filename: &str) -> Vec<u64> {
    use std::fs;

    let content = fs::read_to_string(filename)
        .expect("failed to open file");

    content
        .split(" ")
        .into_iter()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn transform(n: u64) -> Vec<u64> {
    // if n is 0 return 1
    if n == 0 {
        return vec![1]
    }
    
    // if number of digits is even then split
    let length = n.ilog10() + 1;
    if length % 2 == 0 {
        let divisor = 10_u64.pow(length / 2);
        return vec![n / divisor, n % divisor]
    }
    
    // else mult by 2024
    return vec![n * 2024];
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|stone| transform(*stone))
        .collect()
}

fn breadth_first(stones: & mut Vec<u64>) -> usize {
    for i in 0..ITER {
        println!("{}, {}", i, stones.len());
        *stones = blink(stones);
    }
    
    stones.len()
}

fn depth_first(stones: & mut Vec<u64>) -> usize {
    stones
        .iter()
        .enumerate().
        map(|(i, s)| {
            let mut group: Vec<u64> = vec![*s];
            for j in 0..ITER {
                group = blink(&group);
                println!("{}, {}", i, j);   
            }
            
            return group.len();
        })
        .sum()
}

fn main() {
    let mut stones = parse_file(FILENAME);
    println!("{}", depth_first(& mut stones));
}
