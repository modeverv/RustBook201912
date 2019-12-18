fn largest<T: PartialOrd + Copy>(list: &[T])->T{
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

struct Point<T> {
    x:T,
    y:T,
}
impl<T> Point<T> {
    fn x(&self)->&T {
        &self.x
    }
}
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
fn main() {
    println!("Hello, world!");
    let int_point = Point{x:5,y:10};
    let float_point = Point{x:1.0,y:4.2};
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        // もちろん、ご存知かもしれないようにね、みなさん
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
}

