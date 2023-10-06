
fn main() {
    // variable are immutable
    // const 
    let x: u32 = 30;
    let mut y: u32 = 60;
    println!("Prev y = {y}");
    y = 61;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("x = {x}, y={y} & const {THREE_HOURS_IN_SECONDS}");

    let x = x+x;
    {
        let x = x*x;
        println!("Inner scope x = {x}");
    }
    println!("Outer scope x = {x}");

    let spaces = "    ";

    println!("Spaces = {spaces}.");
    let spaces = spaces.len();

    println!("spaces = {spaces}.");
}

