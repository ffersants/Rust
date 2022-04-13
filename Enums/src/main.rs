fn main() {
    //required to derive the implemenation that makes this enum printable
    #[derive(Debug)]
    //a simple enum
    enum TypeIpAddr{
        ipv4,
        ipv6
    }

    //accessing an enum and printing it
    println!("{:?}", TypeIpAddr::ipv4);

    #[derive(Debug)]
    //defining the type of value that can be associated with an enum member
    enum TypeIpAddrAndValue{
        ipv4(String),
        ipv6(String)
    }
    
    //using an enum variant's constructor to get an instance of TypeIpAddrAndValue
    let i = TypeIpAddrAndValue::ipv4(String::from("10"));
    print_enum(i);

    fn print_enum(ip: TypeIpAddrAndValue){
        println!("{:?}", ip)
    }

    //we can change the signature of our enum variant's constructor
    #[derive(Debug)]
    enum Ip4Only{
        ipv4(u8, u8, u8, u8)
    }

    let ip4 = Ip4Only::ipv4(10,39,48,54);

}
