//  Given a list of integers, use a vector and return the median (when sorted, the value in the
// middle position) and mode (the value that occurs most often; a hash map will be helpful here)
// of the list.

use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 0, 10000, 3, 3, 3, 6, 6, 6, 6, 6, 6, 6, 2, 4, 5, 1];

    println!("Original list: {v:?}");

    let sorted_v = sort(&mut v);

    let median = median(&sorted_v);

    let mode = mode(&sorted_v);

    println!(
        "This is the sorted list: {sorted_v:?}, the median is: {median} and the mode is: {mode}"
    );
}

fn sort(unsorted_vector: &mut [usize]) -> &[usize] {
    let size = unsorted_vector.len();
    let mut counter = 0;

    while size > counter {
        let mut smallest = &unsorted_vector[counter];

        let mut sec_counter = counter;

        // finding index of smallest value from start position
        for i in counter..size {
            if unsorted_vector[i] <= *smallest {
                smallest = &unsorted_vector[i];
                sec_counter = i;
            }
        }

        // swap current value and smallest value found
        unsorted_vector.swap(counter, sec_counter);

        counter += 1;
    }

    unsorted_vector
}

fn median(v: &[usize]) -> usize {
    let size = v.len();

    let median = v[size / 2];

    median.clone()
}

fn mode(v: &[usize]) -> usize {
    let mut h: HashMap<usize, usize> = HashMap::new();

    for val in v {
        let count = h.entry(*val).or_insert(0);

        *count += 1;
    }

    let mut biggest_val = 0;
    let mut biggest_key = 0;

    for (k, val) in h.drain() {
        if val > biggest_val {
            biggest_key = k;
            biggest_val = val;
        }
    }

    biggest_key
}

// REVIEW:
//
// - In `median`, `median.clone()` is unnecessary because `usize` implements
//   the `Copy` trait. Returning `median` directly is sufficient.
//
// - The `mode` implementation using `HashMap` and `entry().or_insert()` is
//   very idiomatic Rust and a good approach for counting occurrences.
//
// - Using slices (`&mut [usize]` and `&[usize]`) instead of `Vec<usize>`
//   in function parameters is good practice because it makes the functions
//   more flexible.
//
// - The `median` function assumes the vector is non-empty and does not
//   handle the even-length case (where the median would typically be the
//   average of the two middle values).
