struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    {
        println!("Hello, world!");
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        user1.email = String::from("anotheremail@example.com");
        let mut black = Color(0, 0, 0);
        let mut origin = Point(0, 0, 0);
        black.0 = 10;
        origin.0 = 20;
    }
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!("area is {}", area(&rect1));
        println!("rect is {:?}", rect1);
        println!("rect area is {}",rect1.area());
    }
    {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
