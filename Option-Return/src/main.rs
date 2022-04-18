fn Divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    let res1: Option<i32> = Divide(10, 2);
    println!("{0}", res1.unwrap());
    println!("{:?}", res1);

    let i = res1.is_some();
    println!("{:?}", i);
}
