/*
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

fn main() {
    println!("Hello, world!");
    /*
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    */
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();
    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("{:?} {:?} {:?} ", five,six,none);
    }
    {
        if let Some(vv) = Some(3) {
            println!("{}",vv);
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self){

    }
}

fn plus_one(x: Option<i32>)->Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}