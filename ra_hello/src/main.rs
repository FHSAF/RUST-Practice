fn greetings() {
    println!("Let's get Rusty!");
    let germen = "Grüß dich!";
    let persian = "سلامالیک!";
    let regions = [germen, persian];

    for region in regions.iter()
    {
        println!("{}", &region);
    }
}


fn main() {
    greetings();
}
