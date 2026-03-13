// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
    println!("Hello, world!");

    println!("Please input a word into the Pig Latin converter: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let trimmed_input = input.trim();

    let output = pig_latin_converter(&trimmed_input[..].trim());

    println!("Input: {trimmed_input}, to Pig Latin -> {output}");
}

fn pig_latin_converter(input: &str) -> String {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'y'];

    let mut output = String::new();
    let first_char = input.chars().nth(0).unwrap();

    if vowels.contains(&first_char) {
        output.push_str(&format!("{input}-hay")[..]);
    } else {
        for i in 1..input.len() {
            output.push(input.chars().nth(i).unwrap());
        }
        output.push_str(&format!("{first_char}-hay")[..])
    }

    output
}

// REVIEW:
//
// - The program structure is good: `main` handles input/output while
//   `pig_latin_converter` contains the transformation logic. Separating
//   responsibilities like this is idiomatic Rust and improves readability.
//
// - The parameter type `&str` in `pig_latin_converter` is a good design
//   choice. It allows the function to accept both `String` and string
//   slices without requiring additional allocations.
//
// - In `main`, `&trimmed_input[..].trim()` is redundant because
//   `trim()` already returns a `&str`. Passing `trimmed_input` directly
//   would be simpler.
//
// - The `vowels` variable uses `Vec<char>`, which allocates on the heap.
//   Since the size is fixed and known at compile time, an array like
//   `['a', 'e', 'i', 'o', 'u', 'y']` would be more efficient.
//
// - The expression `input.chars().nth(0)` is less idiomatic than using
//   an iterator with `.next()`. It also scans the iterator from the start,
//   which is unnecessary when retrieving the first character.
//
// - The use of `unwrap()` on the first character can panic if the input
//   string is empty. Handling the empty-string case explicitly would make
//   the function safer.
//
// - The loop `for i in 1..input.len()` relies on `input.len()`, which
//   measures bytes rather than characters. This can cause incorrect
//   behavior for UTF-8 strings containing multi-byte characters.
//
// - Inside the loop, `input.chars().nth(i)` is inefficient because
//   `nth()` iterates from the beginning each time. This results in
//   quadratic time complexity. Iterating once with a `chars()` iterator
//   would be more efficient.
//
// - Using `format!("{input}-hay")` allocates a new string unnecessarily.
//   Appending with `push_str` (and possibly `push`) would avoid the extra
//   allocation and be more efficient.
//
// - Overall, the implementation is clear and readable for learning
//   purposes, but performance and UTF-8 safety could be improved by
//   iterating over characters directly instead of indexing.
