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
    // 所有権
    {
        let s = "hello";
        let mut s = String::from(s);
        s.push_str(",world");
        println!("{}",s);
    }
    {
        let x = 5;
        let y = x;
        println!("{}",y);
        // moveされたら後は知らん！
    }
    // 参照と借用
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("length of {} is {}.",s1,len);

        let mut s = String::from("world");
        {
            let r1 = &mut s;
            r1.push_str("hoge");
        }
        {
            let r2 = &mut s;
            r2.push_str("fuga");
        }
        {
            let mut s = String::from("hello world!");
            let hello = &s[0..5];
            let world = &s[6..11];
            let word = first_word(&s);
            let word2 = first_word2(&s);            
            //s.clear();
            println!("{}",word);
        }

    }
}

fn k2s(templature: u32) -> u32 {
    templature - 10
}
fn s2k(templature: u32) -> u32 {
    templature + 10
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}