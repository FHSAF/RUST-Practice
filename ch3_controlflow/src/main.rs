fn dumm() {
    let mut n:i64 = 10;
    n = loop {
        n += 1;
        if (n == 100)
        {
            break n*100;
        }
    };
    println!("{n} ");
}

fn main() {
    println!("Hello, world!");
}