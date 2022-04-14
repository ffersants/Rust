fn main() {
    //declaring an empty vector
    let mut i: Vec<u8> = vec![];

    i.push(1);
    println!("{:?}", i);

    i.remove(0);
    println!("{:?}", i);

}
