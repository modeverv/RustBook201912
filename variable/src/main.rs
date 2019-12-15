fn main() {
    println!("Hello, world!");
    {
        let x = 5;
        //x = 6;
        println!("x = {}", x);
        let mut xx = 5;
        println!("xx = {}", xx);
        xx = 6;
        println!("xx = {}", xx);
    }
    {
        let a = [1, 2, 3, 3];
        //let index = 10;
        //let element = a[index];
        //println!("{}", element);
        for elem in a.iter() {
            print!("{} ", elem);
        }
        println!();
    }
}
