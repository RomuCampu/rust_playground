fn main() {
    let four = IpAddressKind::V4(127, 0, 0, 1);
    let six = IpAddressKind::V6(String::from("::1"));

    println!("{:#?}\n{:#?}", four, six);

    let home = IpAddressKind::V4(127, 0, 0, 1);
    let loopback = IpAddressKind::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);
    
    //________________________________________________________//
    
    let ver4 = IpAddr::V4(IpV4Addr{ addr: (123, 111, 111, 22)});
    let ver6 = IpAddr::V6(IpV6Addr{ addr: String::from("::55" )});

    println!("{:#?}", ver4);
    println!("{:#?}", ver6);

    let some_num = Some(5);
    let some_str = Some("Some string");
    let nothing: Option<i32> = None;    

    println!("{:#?}\n{:#?}\n{:#?}", some_num, some_str, nothing);

}

#[derive(Debug)] 
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

    //________________________________________________________//


#[derive(Debug)]
enum IpAddr {
    V4(IpV4Addr),
    V6(IpV6Addr)
}

#[derive(Debug)]
struct IpV4Addr {
    addr: (u8, u8, u8, u8)
}

#[derive(Debug)]
struct IpV6Addr {
    addr: String
}


// #![allow(unused)]
// fn main() {
// enum Option<T> {
//     Some(T),
//     None,
// }
// }


// #[derive(Debug)] 
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write: String,
//     ChangeColor: (i32, i32, i32)
// }
 