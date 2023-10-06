use std::io;
// Statically types
// Type inferring
// Scalar, and Compound are two type subsets
// Four primary scalar: Interger, Floating points, Booleans, and Characters
// . Number literals(Dec, Hex, Octal, Bin, Byte(u8 only))\
// Two primitive coumpoud types: tuples, and arrays
fn main() {

    let x = 2.01;
    let y: f32 = 3.01;
    println!("x = {x}, y = {y}");
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c}, {heart_eyed_cat}, {z}");

    // compounds
    // Tup: fixed size, of different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred}, {six_point_four}, {one}");
    // array: fixed size, of same types
    let arr: [i32; 5] = [21, 32, 43, 54, 65];

    println!("Please enter an index: ");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Indexed enterd was not a number!");

    let elem = arr[index];

    println!("For index {index} element is {elem}.");
}
