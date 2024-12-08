
const FILE_NAME: &str = "resources/input.txt";

fn parse_file(filename: &str) -> Vec<Calibration> {
    use std::fs;

    let content: String = fs::read_to_string(filename).expect("failed to read file");
    
    return content.lines().map(|line| {
        let mut split = line.split(":");
        let target = split.next().unwrap().parse().expect("invalid target number");
        let numbers = split.next().unwrap().split_whitespace()
            .map(|number| number.parse().expect("invalid number")).collect();

        return Calibration{target, numbers}
    }).collect();
}




#[derive(Debug)]
struct Calibration {
    target: u64,
    numbers: Vec<u64>,
}

impl Calibration {
    fn is_possible(&self) -> bool {
        return self.calculate_results(&self.numbers).contains(&self.target);
    }

    fn calculate_results(&self, numbers: &Vec<u64> ) -> Vec<u64> {
        if numbers.len() == 1 {
            return numbers.to_vec();
        }

        let (initial, _last) = numbers.split_at(numbers.len() - 1);
        let last = _last.last().expect("");

        let sub_results = self.calculate_results(&initial.to_vec());

        let add: Vec<u64> = sub_results.iter().map(|number| number + last).collect();
        let mult: Vec<u64> = sub_results.iter().map(|number| number * last).collect();
        let concat: Vec<u64> = sub_results.iter().map(|number| format!("{}{}", number, last).parse().expect("")).collect();

        return vec![add, mult, concat].concat();
    }
    
}

fn main() {
    let calibrations = parse_file(FILE_NAME);

    let total: u64 = calibrations.iter()
        .filter(|calibration| calibration.is_possible())
        .map(|calibration| calibration.target).sum();

    println!("Part 1: {}", total);
}
