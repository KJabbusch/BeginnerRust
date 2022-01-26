enum IpAddrKind {
    // we can specify what type of data we want for each variant in parentheses
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    // we could store each variant as its own struct, but they would be different types
    // the benefit of an enum is that all variants are stored in a single type

    Quit, // stores no data 
    Move { x: i32, y: i32 }, // stores an anonymous struct
    Write(String), // stores a String
    ChangeColor(i32, i32, i32), // stores 3 integers
}

impl Message {
    // we can implement methods on the enum
    fn some_function() {
        println!("called some_function");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main () {
    // localhost 
    let localhost: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddrKind) {
}
