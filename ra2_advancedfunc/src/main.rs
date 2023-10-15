
use std::ops::{ Add };
use std::time::{ Duration };

fn life_time_add<'a, 'b>(i: &'a i32, j: &'b i32) -> i32
{
    *i + *j
}

fn generic_add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn arrays()
{
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arr = [one, two, blank1, blank2];
    for item in arr
    {
        print!("{:?}: ", item);
        for i in item.iter()
        {
            print!("\t{} + 10 = {} ", i, i+10);
        }
        let mut sum = 0;
        for j in 0..item.len() 
        {
            sum += item[j];
        }
        println!("\t((Σ{:?} = {})", item, sum);
    }
}

fn main()
{
    let floats = generic_add(1.1, 2.3);
    let ints = generic_add(21, 21);
    let durations = generic_add(
        Duration::new(5, 0),
        Duration::new(10, 0)
    );
    let a = 10;
    let b = 20;
    let c = life_time_add(&a, &b);
    println!("{}, {}, {:?}, c={}", floats, ints, durations, c);
    arrays();
}