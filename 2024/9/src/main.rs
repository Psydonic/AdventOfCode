const FILENAME: &str = "resources/test.txt";

// A memory block has a size and an id
// id is none if the block is empty
#[derive(Debug, Clone)]
struct Block {
    pub size: u32,
    pub id: Option<usize>,
}

// a disk is made up of a list of blocks
// some are full and some are empty
#[derive(Debug, Clone)]
struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    // construct a disk from a file
    fn new(filename: &str) -> Self {
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


    // Compress the disk by moving the rightmost filled blocks to the leftmost free space,
    // splitting them if necessary.
    fn compress(&self) -> Disk {
        // create a new disk to hold the compressed version
        // i is our counter iterating forward, looking for empty blocks
        // j is our counter iterating backwards looking for full blocks
        // stop when i == j
       
        let mut compressed_blocks = vec![];
        let mut i = 0;
        let mut j = self.blocks.len() - 1;

        while i < j {
            // if the i block is full
            if self.blocks[i].id.is_some() {
                // move it to the compressed blocks list and increment
                compressed_blocks.push(self.blocks.get(i).unwrap().clone());    
                i += 1;
            } else {
                // i block is empty
                // if the j block is full
                if self.blocks[j].id.is_some() {
                    // if the space is larger than the block
                    if self.blocks[i].size > self.blocks[j].size {
                        // copy the block in
                        compressed_blocks.push(self.blocks[j].clone());
                        // copy a shrunk empty block in after
                        let shrunk_block = Block { 
                            size: self.blocks.get(i).unwrap().size - self.blocks.get(j).unwrap().size, 
                            id: None
                        };
                        compressed_blocks.push(shrunk_block);

                        // move both i and j
                        i += 1;
                        j -= 1;
                    // if the space is the same size as the block
                    } else if self.blocks[i].size == self.blocks[j].size {
                        // copy full block to the empty space
                        compressed_blocks.push(self.blocks[j].clone());
                        
                        // move  both i and j
                        i += 1;
                        j += 1;
                    // if the space is smaller than the block size
                    } else {
                        // calculate how much free space we have from 
                        // end of i to start of j
                        // TODO
                        
                        // iterate frm i to j, filling in space
                        // until the block is used up
                        // TODO
                    }
                } else {
                    // j block is empty, decrement
                    j -= 1;
                }
            }
        }

        Disk {blocks: compressed_blocks}
    }
    
    // calculate the checksum of the disk
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

fn main() {
    let disk = Disk::new(FILENAME);
    let compressed_disk = disk.compress();
    let checksum = compressed_disk.checksum();
    println!("{:?}", checksum);
}
