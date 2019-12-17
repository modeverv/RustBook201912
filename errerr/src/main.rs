fn main() {
    println!("Hello, world!");
    let vec = vec!(1,2,3);
    //println!("{}",vec[99]);
    println!("{}",vec[1]);
    let a = read_username_from_file().expect("IO error");
    println!("{}",a)
}

use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}