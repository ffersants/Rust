fn main() {
    let name = String::from("Pombão da Praça");

    let mut pombao = Pombo {
        name,
        age: 2,
        medicated: false 
    };
    
    println!("Medicated? {}", pombao.medicated);
    
    if pombao.medicated == false {
        println!("Giving medication...");
        pombao.give_medication();
        println!("Now {} is medicated!", pombao.name);
    }

    let can_fly = pombao.can_fly();
    println!("Can fly? {}", can_fly);

    pombao.say_something();
}

struct Pombo {
    name: String,
    age: u8,
    medicated: bool
}

impl Pombo {
    fn give_medication(&mut self){
        self.medicated = true;
    }
}

trait Animal {
    //it's just the method signature. Whoever implements Animal must 
    //have a can_fly method that returns bool
    fn can_fly(&self) -> bool;

    //this function can be inherited by the struct that implements this trait
    //but it can also be overridden
    fn say_something(&self) {
        println!("Speaking in native animal language...");
    }
}

//can_fly is implemented inside Pombo
impl Animal for Pombo{
    fn can_fly(&self) -> bool {
        true
    }

    //say_something can be overridden
    // fn say_something(&self){
    //     println!("Pruu pruuu pruu...")
    // }
}