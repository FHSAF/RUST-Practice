fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 40_i32;
    let e = add(add(a, b), add(c, d));
    println!("(a + b) + (c + d) = {}", e); // macros return code rather than value
    numbers();
    floating_point_hz();
    flow_control();
    while_duration();
    match_keyword();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn numbers()
{
    let twt = 20;
    let twto: i32 = 21;
    let twtt = 22i32;
    let sum = twt + twto + twtt;
    println!("{} + {} + {} = {}", twt, twto, twtt, sum);

    let one_ml: i64 = 1_000_000;
    println!("{}", one_ml.pow(2));

    let ftt = [
        42.0,
        42f32,
        42.0_f32
    ];

    println!("{:02}", ftt[2]);

    let three = 0b11;
    let thirty = 0o36;
    let three_100 = 0x12C;
    println!("base 10: {}, {}, {}", three, thirty, three_100);
    println!("base 2: {:b}, {:b}, {:b}", three, thirty, three_100);
    println!("base 8: {:o}, {:o}, {:o}", three, thirty, three_100);
    println!("base 16: {:x}, {:x}, {:x}", three, thirty, three_100);

    // Type casting 
    let v_a: i32 = 10;
    let v_b: f32 = 100.0;
    if v_a < (v_b as i32)
    {
        println!("Yes!");
    }
}

fn floating_point_hz()
{
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.1, 0.3);
    println!("abc: (f32)");
    println!("0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    println!("            {:x}", abc.2.to_bits());
    println!();

    println!("xyz: (f64)");
    println!("0.1 + 0.2 = {:x}", (xyz.0 + xyz.1).to_bits());
    println!("            {:x}", xyz.2.to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);
}

fn flow_control()
{
    let contrainer = [1, 2, 3, 4, 5, 10];
    for item in contrainer {
        println!("{}", item);
    }
    println!();
    // anonymous loop
    for _ in 0..10 {
        println!("Ok.")
    }
    println!();
}

use std::time::{Instant, Duration};

fn while_duration()
{
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("count: {}", count);
}

fn match_keyword()
{
    let haystack = [1, 20, 123, 42, 10, 100, 333];

    for item in &haystack {
        let result = match item{
            42 | 333 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}, {}", item, result);
        }
    }
}