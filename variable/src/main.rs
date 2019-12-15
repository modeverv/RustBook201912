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
        for elem in a.iter().rev()  {
            print!("{} ", elem);
        }
        print!("\n");
        for n in (1..4).rev() {
            println!("{}",n);
        }

    }
    {
        println!("k2s: {}",k2s(100));
        println!("k2s: {}",s2k(100));
    }

}

fn k2s(templature: u32) -> u32 {
    templature - 10
}
fn s2k(templature: u32) -> u32 {
    templature + 10
}
