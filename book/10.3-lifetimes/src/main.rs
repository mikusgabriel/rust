fn main() {
    println!("Hello, world!");
    let a = "Hello";
    let b = "World!";

    let longest = longest_with_an_announcement(&a, &b, "Bing Bong");

    println!("Longest is: {longest}");
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
