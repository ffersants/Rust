use std::collections::HashMap;

fn main() {
    let mut hsh = HashMap::new();

    hsh.insert(5, "Some val");
    hsh.insert(0, "Another val");

    println!("{:?}", hsh);

    match hsh.get(&4) {
        Some(val) => println!("Found the val {0}", val),
        None => println!("Nothing found!")
    }

    match hsh.get(&5) {
        Some(val) => println!("Found the val {0}", val),
        None => println!("Nothing found!")
    }

    hsh.remove(&5);

    println!("{:?}", hsh);
}
