pub #[derive(Debug)]
struct Allocator {
	field: Type
}

impl Allocator {
	// add code here
	pub fn alloc(&mut self, size: u8) -> i8 {
	}
	
	pub fn realloc(&mut self, addr: usize, size: u8) -> i8 {
	}

	pub fn dealloc(&mut self, addr: usize) {
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