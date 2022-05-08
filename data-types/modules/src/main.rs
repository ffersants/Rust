pub mod house {
    //pubs doesn't share the same scope
    pub mod bedroom1{
        pub fn is_light_on() -> bool{
            false
        }

        pub fn is_the_neighbours_bedroom_lights_on() -> bool{
            use super::bedroom2;
            bedroom2::is_light_on()
        }
    }

    pub mod bedroom2{
        pub fn is_light_on() -> bool{
            true
        }
    }
}

mod apartment;

fn main() {
    //:: is the notation defined to access modules and its contents
    println!("{:?}", house::bedroom1::is_light_on()); //false

    use house::bedroom2;
    println!("{:?}", bedroom2::is_light_on()); //true

    use house::bedroom1;
    println!("{:?}", bedroom1::is_the_neighbours_bedroom_lights_on());//true
    
    use apartment::apartment_bedroom2;
    println!("{:?}", apartment_bedroom2::is_light_on());
}
