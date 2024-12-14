const FILENAME: &str = "resources/test.txt";

fn main() {
    use std::fs;
    let content = fs::read_to_string(FILENAME).expect("error reading file");
    let mut reversed = content.chars().rev();

    let mut checksum = 0;
    let mut counter = 0;

    let mut j = content.len() - 1;
    for (i, c) in content.chars().enumerate() {
        if i%2 == 0 { //if its a file block
            let id = i / 2;
            let block_size = c.to_digit(10).unwrap() as usize;

            checksum += block_size * id * (counter + 1);
            counter += block_size;
        } else { // empty block, need to fill in with end
            let empty_block_size = c.to_digit(10).unwrap() as usize;

            let id = j / 2;
            let end_block_size = reversed.next().unwrap().to_digit(10).unwrap() as usize;

            if empty_block_size > end_block_size {
                checksum += end_block_size * id * (counter + 1);
                counter += end_block_size
            }

        }
    }

    println!("{}", checksum);
}
