#[derive(Debug)] // Adding the attribute to derive the Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let area = area(&rect1);

    println!("rect1 is {rect1:?} and its area is {area}"); // Printing the Rectangle instance using debug formatting
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
