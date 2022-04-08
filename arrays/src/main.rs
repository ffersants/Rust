fn main() {
    let array_of_five_indexes: [u8; 5] = [1, 2, 3, 4, 5];
    println!("My array of five indexes {0}", array_of_five_indexes.len());

    //creates an array with five indexes and the number five in each index
    let auto_populed_array: [u8; 5] = [100; 5];

    //prints the array
    println!("{:?}", auto_populed_array);

    let arr = ["A", "B", "C", "D"];
    let arr_sliced = &arr[0..3];
    println!("{:?}", arr_sliced);
}
