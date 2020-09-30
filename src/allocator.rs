
pub struct Allocator {
    memory: Vec<u8>,
    align: u8,
}

impl Allocator {
    pub const fn new(memory: Vec<u8>) -> Self {
        Allocator {
            memory,
            align: 4,
        }
    }

    pub fn init(&mut self) {
        self.memory[1] = self.memory.len() as u8 - 3;
        self.memory[2] = 0;
    }

    pub fn dump(&self) -> &Vec<u8> {
        &self.memory
    }

    pub fn alloc(&mut self, size: u8) -> i8 {
        let size = self.align_up(size);
        let block_addr = self.find_empty_block(size);
        if block_addr == -1 {
            // out of memory
            return -1;
        }
        let block_addr = block_addr as usize;
        // 7 = 3 + 4 = header + min_block_size
        if self.memory[block_addr + 1] - size >= 7 {
            self.split_block(block_addr, size);
        } else {
            self.update_header(1, block_addr, size, self.memory[block_addr + 2]);
        }
        (block_addr + 3) as i8
    }

    pub fn realloc(&mut self, addr: usize, size: u8) -> i8 {
        let cur_size = self.memory[addr - 2];
        if cur_size > size {
            if cur_size - size >= 7 {
                // create new header
                let new_block = addr + 3 + size as usize;
                self.memory[new_block] = 0;
                self.memory[new_block + 1] = cur_size - size - 3;
                self.memory[new_block + 2] = size;
                // reset size of the block
                self.memory[addr + 1] = size;

                let next_block = new_block + 3 + self.memory[new_block + 1] as usize;
                if self.memory[next_block] == 0 {
                    self.join_unused_blocks(new_block, next_block);
                }
            }
            addr as i8
        } else {
            let addr = addr - 3;
            println!("up");
            let size = size as usize;
            let cur_size = cur_size as usize;

            let next_block = addr + 3 + self.memory[addr + 1] as usize;
            let next_block_size = self.memory[next_block + 1] as usize;
            println!("{} - {} - {}", next_block, self.memory[next_block], next_block_size);
            if self.memory[next_block] == 0 && next_block_size + 3 + cur_size >= size {
                println!("next block");
                if next_block_size + 3 + cur_size - size >= 7 {
                    let new_block = addr + 3 + size;
                    self.memory[new_block] = 0;
                    self.memory[new_block] = (next_block_size + cur_size - size) as u8;
                    self.memory[new_block] = size as u8;
                    self.memory[addr + 1] = size as u8;
                } else {
                    self.memory[addr + 1] += self.memory[next_block + 1] + 3;
                }
                addr as i8
            } else {
                println!("all deall");
                let new_addr = self.alloc(size as u8);
                if new_addr == -1 {
                    return new_addr;
                }
                let new_addr = new_addr as usize;
                // copy data from old block to new
                for i in 0..cur_size {
                    self.memory[i + new_addr + 3] = self.memory[i + 2 + addr];
                }
                self.dealloc(addr);
                new_addr as i8
            }
        }
    }

    pub fn dealloc(&mut self, addr: usize) {
        let mut addr = addr - 3;
        // mark block as unused
        self.memory[addr] = 0;
        if self.memory[addr + 2] != 0 {
            // check if prev block is also empty. if yes then join them
            let prev_addr = addr - self.memory[addr + 2] as usize - 3;
            if self.memory[prev_addr] == 0 {
                addr = self.join_unused_blocks(prev_addr, addr);
            }
        }
        if self.memory[addr + 1] != 0 {
            // check if next block is also empty. if yes then join them
            let next_addr = addr + 3 + self.memory[addr + 1] as usize;
            if self.memory[next_addr] == 0 {
                self.join_unused_blocks(addr, next_addr);
            }
        }
    }

    fn join_unused_blocks(&mut self, addr1: usize, addr2: usize) -> usize {
        let new_size = self.memory[addr1 + 1] + self.memory[addr2 + 1] + 3;
        self.update_header(0, addr1, new_size, self.memory[addr1 + 2]);
        addr1
    }

    fn split_block(&mut self, addr: usize, size: u8) {
        let full_size = self.memory[addr + 1];
        // update first block
        self.memory[addr] = 1;
        self.memory[addr + 1] = size;
        // create a second block
        let second_addr = addr + 3 + size as usize;
        self.memory[second_addr] = 0;
        self.memory[second_addr + 1] = full_size - size - 3;
        self.memory[second_addr + 2] = size;
    }

    fn update_header(&mut self, is_used: u8, addr: usize, size: u8, size_prev: u8) {
        self.memory[addr] = is_used;
        self.memory[addr + 1] = size;
        self.memory[addr + 2] = size_prev;
    }

    fn find_empty_block(&self, size: u8) -> i8 {
        let mut pointer = 0;
        while pointer < self.memory.len() {
            if self.memory[pointer] == 0 && self.memory[pointer + 1] >= size {
                return pointer as i8;
            }
            // 3 - size of the header
            pointer = pointer + (self.memory[pointer + 1] + 3) as usize;
        }
        -1
    }

    fn align_up(&self, size: u8) -> u8 {
        let remainder = size % self.align;
        if remainder == 0 {
            size // addr already aligned
        } else {
            size - remainder + self.align
        }
    }
}

