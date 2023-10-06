// Dangling pointer
#[derive(Debug)]
enum Cereal {
    Berley, Millet, Rice,
    Rye, Spelt, Wheat
}

fn main() {
    let mut Grains: Vec<Cereal> = vec![];
    Grains.push(Cereal::Rye);
    drop(Grains);
    println!("{:?}", Grains);
}
