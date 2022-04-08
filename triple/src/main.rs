fn main() {
    //no type definition
    let my_tuple = (5, true, "something");
    //with type definition
    let my_other_tuple: (&str, bool) = ("something", false);
    
    let (_val1, _val2, _val3) = my_tuple;
    println!("{}", my_other_tuple.1);
    println!("{:?}", my_tuple);
}
