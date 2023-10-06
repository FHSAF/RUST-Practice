fn main() {
    let pinguines = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = pinguines.lines();
    for (i, record) in records.enumerate()
    {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let fields: Vec<_> = record
            .split(',')
            .map(|filed| filed.trim())
            .collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}
