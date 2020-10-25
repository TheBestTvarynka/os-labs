// use std::option::Option;
use std::vec::Vec;
use std::collections::HashMap;

// #[derive(Debug)]
// struct Page {
//     addr: u16,
//     next: Option<&'static mut Page>,
// }

// impl Page {
//     pub const fn new(addr: u16, next: Option<&'static mut Page>) -> Self {
//         Page {
//             addr,
//             next,
//         }
//     }
// }

#[derive(Debug)]
struct PageDescriptor {
    block_size: u16,  // size of block of the page
    counter: u16,     // amount of all empty block
    first_empty: u16, // pointer to first empty block
}

impl PageDescriptor {
    pub const fn new(block_size: u16, counter: u16, first_empty: u16) -> Self {
        PageDescriptor {
            block_size,
            counter,
            first_empty,
        }
    }
}

#[derive(Debug)]
pub struct Allocator {
    memory: Vec<u16>,
    descriptors: Vec<PageDescriptor>,
    pages: HashMap<u16, Vec<u16>>, // hashmap with lists with not-full pages
    page_size: usize,
    page_number: usize,
}

impl Allocator {
    pub fn new() -> Self {
        Allocator {
            memory: Vec::new(),
            descriptors: Vec::new(),  // one descriptor for each page
            pages: HashMap::new(),    // hashmap with lists with non-empty pages
            page_size: 256,
            page_number: 3,
        }
    }

    pub fn init(&mut self) {
        // self.memory = vec![0; 1536];  // 6 pages for 256 bytes for each
        self.memory = vec![0; self.page_size * self.page_number];  // 2 pages for 256 bytes for each
    }

    pub fn dump(&self) {
        println!("{:?}", self.descriptors);
        println!("{:?}", self.pages);

        println!("{:?}", &self.memory[0..256]);
        println!("");
        println!("{:?}", &self.memory[256..512]);
        println!("");
        println!("{:?}", &self.memory[512..768]);
        println!("");
    }

    pub fn alloc(&mut self, size: u16) -> i16 {
        let block_size = self.round_up(size);
        if size <= self.page_size as u16 / 2  {
            if !self.pages.contains_key(&block_size) {
                // take new page and format it with block with `block_size` sizes
                self.format_new_page(block_size);
            }
            // take first page from list and alloc memory in it
            let page = match self.pages.get(&block_size) {
                Some(val) => match val.first() {
                    Some(x) => *x,
                    None => panic!("Error"),
                },
                None => panic!("Error"),
            };
            let descriptor = &mut self.descriptors[(page / self.page_size as u16) as usize];
            // update descriptor of the page
            descriptor.counter = descriptor.counter - 1;
            let block_addr = descriptor.first_empty as usize;
            descriptor.first_empty = self.memory[block_addr];
              // it's unnecessary operation
              self.memory[block_addr] = 0;
              //
            // if page full then remove it
            if descriptor.counter == 0 {
                self.remove_page_from_list(block_size);
            }
            return block_addr as i16;
        } else {
            // allock one page or few pages
            let pages_amount = (block_size as f32 / self.page_size as f32).ceil() as usize;
            let size = pages_amount * self.page_size;
            if self.pages.contains_key(&(size as u16)) {
                let mut addr = 0;
                match self.pages.get_mut(&(size as u16)) {
                    Some(vec) => {
                        match vec.first() {
                            Some(x) => addr = *x,
                            None => {},
                        };
                        if vec.len() == 1 {
                            self.pages.remove(&(size as u16));
                        } else {
                            vec.remove(0);
                        }
                    },
                    None => panic!("Error"),
                };
                addr as i16
            } else {
                let start_addr = self.descriptors.len() as u16;
                if pages_amount + start_addr as usize > self.page_number {
                    panic!("Error: {}", );
                }
                let start_addr = start_addr * self.page_size as u16;
                self.descriptors.push(PageDescriptor::new(size as u16, 0, 0));
                for _i in 1..pages_amount {
                    self.descriptors.push(PageDescriptor::new(0, 0, 0));
                }
                start_addr as i16
            }
        }
    }

    pub fn realloc(&mut self, _addr: usize, _size: u16) -> i16 {
        -1
    }

    pub fn dealloc(&mut self, addr: usize) -> u16 {
        self.roung_to_pages(addr as u16)
    }

    //
    fn format_new_page(&mut self, block_size: u16) {
        let new_page = self.descriptors.len();
        if new_page >= self.page_number {
            panic!("Error");
        }
        let new_page_addr = new_page * self.page_size;
        for addr in (0..self.page_size).step_by(block_size as usize) {
            self.memory[addr + new_page_addr] = (addr + new_page_addr + block_size as usize) as u16;
        }
        self.descriptors.push(PageDescriptor::new(block_size, self.page_size as u16 / block_size, new_page_addr as u16));
        match self.pages.get_mut(&block_size) {
            Some(x) => {
                // x - list with pages (mutable reference to this list)
                x.insert(0, new_page_addr as u16);
                // self.pages.insert(block_size, x);
            },
            None => {
                let mut v = Vec::new();
                v.push(new_page_addr as u16);
                self.pages.insert(block_size, v);
            },
        };
    }

    fn remove_page_from_list(&mut self, block_size: u16) {
        match self.pages.get_mut(&block_size) {
            Some(x) => {
                // x - list with pages (mutable reference to this list)
                x.remove(0);
            },
            None => {},
        }
    }

    // round up passed size to nearest block size
    fn round_up(&self, size: u16) -> u16 {
        if size <= 4 {
            4
        } else {
            8
        }
    }

    fn roung_to_pages(&self, block_size: u16) -> u16 {
        (block_size as f32 / self.page_size as f32).ceil() as u16
    }
}