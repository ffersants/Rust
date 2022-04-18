fn Divide(dividend: i32, divisor: i32) -> Result<i32, &'static str> {
    if dividend % divisor != 0 {
        Err("Invalid values")
    }
    else {
        Ok(dividend/divisor)
    }
}

fn main() {
    let result = Divide(10, 3);
    println!("{:?}", result);

    match result {
        Ok(result) => println!("The divide operation ended with the result of {:?}", result),
        Err(err) => println!("{0}", err)
    }

    if result.is_ok() {
        println!("The divide operation ended with the result of {:?}", result.unwrap())
    }
    
    if result.is_err(){
        println!("Invalid values, the default value is {:?}", result.unwrap_or(0))
    }
}
