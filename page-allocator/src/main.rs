use page_allocator::allocator::Allocator;

fn main() {
	let mut allocator = Allocator::new();
	allocator.init();
   
    allocator.dump();

    println!("{}", allocator.alloc(6));
    allocator.dump();

    println!("{}", allocator.alloc(5));
    allocator.dump();

    println!("{}", allocator.alloc(3));
    allocator.dump();

    println!("{}", allocator.alloc(4));
    allocator.dump();

    println!("{}", allocator.alloc(200));
    allocator.dump();
}
