fn main() {
    let mut a = "It's the letter A";
    change(&a);
    println!("3 - {}", a);
}

fn change(mut var_to_change: &str){
    println!("1 - {}", var_to_change);
    var_to_change = "It's not letter A anymore";
    println!("2 - {}", var_to_change);
}