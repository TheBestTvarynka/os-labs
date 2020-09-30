use simple_allocator::allocator::Allocator;

fn main() {
    let memory = vec![0; 50];
    let mut allocator = Allocator::new(memory);
    allocator.init();

    println!("{:?}", allocator.dump());

    println!("{}", allocator.alloc(4));
    println!("{:?}", allocator.dump());
    println!("{}", allocator.alloc(6));
    println!("{:?}", allocator.dump());
    println!("{}", allocator.alloc(5));
    println!("{:?}", allocator.dump());
    println!("{}", allocator.alloc(5));
    println!("{:?}", allocator.dump());

    allocator.dealloc(21);
    println!("{:?}", allocator.dump());
    allocator.realloc(10, 13);
    println!("{:?}", allocator.dump());
}
