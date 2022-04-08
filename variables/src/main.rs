fn main() {
    let unmutable = 3;
    // can't assign a new value to a unmutable variable
    // unmutable = 5;

    println!("unmutable has {0} as value and CAN'T be changed!", unmutable);

    let mut mutable = 6;
    println!("mutable has {0} as value and CAN be changed...", mutable);
    
    mutable = 8;
    println!("Actually mutable just changed to {0}", mutable);
    
}
