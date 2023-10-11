use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10; // on the stack
    let b = Box::new(20); // on the heap (boxed int)
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Mutex::new(40));
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
/*
1. Safety
2. Immutability of data by default
3. Compile time check
*/