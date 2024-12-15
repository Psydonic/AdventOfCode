const FILENAME: &str = "resources/test.txt";

#[derive(Debug, Clone)]
struct Block {
    size: u32,
    id: Option<usize>,
}

#[derive(Debug, Clone)]
struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    // Compress the disk by moving the rightmost filled blocks to the leftmost free space,
    // splitting them if necessary.
    fn compress(&self) -> Disk {
        let mut compressed_blocks: Vec<Block> = Vec::new();
        let mut current_free_space: u32 = 0;
        let mut filled_blocks: Vec<Block> = Vec::new();

        // Separate filled and empty blocks
        for block in &self.blocks {
            match block.id {
                Some(_) => filled_blocks.push(block.clone()),
                None => current_free_space += block.size,
            }
        }

        // Process filled blocks from rightmost to leftmost
        while let Some(block) = filled_blocks.pop() {
            if current_free_space > 0 {
                if current_free_space > block.size {
                    // Free space larger than filled block
                    compressed_blocks.insert(0, Block {
                        size: block.size,
                        id: block.id,
                    });
                    current_free_space -= block.size;
                } else if current_free_space < block.size {
                    // Free space smaller than filled block
                    compressed_blocks.insert(0, Block {
                        size: current_free_space,
                        id: block.id,
                    });
                    compressed_blocks.insert(0, Block {
                        size: block.size - current_free_space,
                        id: block.id,
                    });
                    current_free_space = 0;
                } else {
                    // Free space equal to filled block
                    compressed_blocks.insert(0, Block {
                        size: block.size,
                        id: block.id,
                    });
                    current_free_space = 0;
                }
            } else {
                compressed_blocks.insert(0, block);
            }
        }

        // Append any remaining free space at the end
        if current_free_space > 0 {
            compressed_blocks.push(Block {
                size: current_free_space,
                id: None,
            });
        }

        Disk {
            blocks: compressed_blocks,
        }
    }

    fn checksum(&self) -> usize {
        let mut counter: usize = 0;
        let mut checksum: usize = 0;

        for block in &self.blocks {
            if block.id.is_none() {
                counter += block.size as usize;
                continue;
            }
            // else, multiply id by index
            checksum += block.size as usize * block.id.unwrap() * (counter + 1);
            counter += block.size as usize;
        }
        checksum
    }
}

fn parse_file(filename: &str) -> Disk {
    use std::fs;
    let content = fs::read_to_string(filename).expect("error reading file");

    let blocks = content
        .chars()
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let digit = c.to_digit(10).unwrap();
            let id = if i % 2 == 0 { Some(i / 2) } else { None };
            return Block { size: digit, id };
        })
        .collect();

    Disk { blocks }
}

fn main() {
    let disk = parse_file(FILENAME);
    let compressed_disk = disk.compress();
    let checksum = compressed_disk.checksum();
    println!("{:?}", checksum);
}
