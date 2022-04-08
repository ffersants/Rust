fn main() {
    let name = String::from("Papagaio");

    let papagaio = Bird {
        age: 2,
        //just like in javascript
        name
    };

    papagaio.print_bird_name();
}

//works just like a class
struct Bird {
    name: String,
    age: u8
}

//defines struct methods
impl Bird {
    //&self is equivalent to javascript's "this"
    fn print_bird_name(&self){
        println!("The name of the bird is {}", self.name);
    }

    fn print_bird_age(&self){
        println!("The age of the bird is {}", self.age);
    }
}
