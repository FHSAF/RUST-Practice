// Dangling pointer
/* 
#[derive(Debug)]
enum Cereal {
    Berley, Millet, Rice,
    Rye, Spelt, Wheat
}

fn Dangling_pointer()
{
    let mut Grains: Vec<Cereal> = vec![];
    Grains.push(Cereal::Rye);
    drop(Grains);
    println!("{:?}", Grains);
}

use std::thread;
fn race_condition()
{
    let mut data = 100;
    thread::spawn(|| { data = 500; });
    thread::spawn(|| { data = 1000; });
    println!("{}", data);
}


fn buffer_overflow()
{
    let fruite = vec!["apple", "banana", "seb"];
    let buff_of = fruite[4];
    assert_eq!(buff_of, "malta");
}
*/

fn iterator_invalidation()
{
    let mut letters = vec!["a", "b", "c"];

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone());
    }
}
fn main() {
    iterator_invalidation();
}
