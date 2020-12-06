use std::thread;
use std::time::Duration;

fn new_func1(a: i32, b: i32) -> i32 {
    let mut res = 0;
    for i in 0..10 {
        if i > 8 {
            res = a + b;
        }
        if res > 0 {
            return res;
        }
    }
    res
}

fn func1(a: i32, b: i32) -> i32 {
    let mut res = 0;
    for i in 0..10 {
        thread::sleep(Duration::from_secs(1));
        if i > 8 {
            res = result_of_sum(a, b);
        }
        if res > 0 {
            return res;
        }
    }
    res
}

fn func2(a: i32, b: i32) -> i32 {
    let res = 0;
    for _i in 0..10 {
        let res = new_func1(a, b);
        if res > 0 {
            return res;
        }
    }
    res
}

fn result_of_sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", func2(51, 110));
}
