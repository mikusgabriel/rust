fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // Here some_string is automatically de-referenced by Rust with (*).
    // Since the reference is mutable we are able to modify the variable.
}
