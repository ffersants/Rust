fn main() {
    println!("Hello, world!");
    is_even(2);
}

fn is_even(number: u8) -> bool {
    let result: u8 = number % 2;
    //implicit return once it doesn't have the ;
    result == 0
}