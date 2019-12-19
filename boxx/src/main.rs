enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons,Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    println!("Hello, world!");
    let b = Box::new(5);
    println!("{}",b);
    {
        let list = Cons(1,
            Box::new(Cons(2,
                Box::new(Cons(3,
                    Box::new(Nil))))));
    }
    {
        let x = 5;
        let y = &x;
    
        assert_eq!(5, x);
        // assert_eq!(5, *y); error
    }
    {
        let x = 5;
        let y = Box::new(x);
    
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        let x = 5;
        let y = MyBox::new(x);
    
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        let c = CustomSmartPointer { data: String::from("my stuff") };      // 俺のもの
        let d = CustomSmartPointer { data: String::from("other stuff") };   // 別のもの
        println!("CustomSmartPointers created."); 
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // CustomSmartPointerをデータ`{}`とともにドロップするよ
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
