// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::{collections::HashMap, io};

fn main() {
    println!("Hello, world!");

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    departments.insert("Engineering".to_string(), Vec::new());
    departments.insert("Sales".to_string(), Vec::new());
    departments.insert("Research".to_string(), Vec::new());
    departments.insert("Marketing".to_string(), Vec::new());
    departments.insert("Public Affairs".to_string(), Vec::new());

    command_line_loop(departments);
}

fn command_line_loop(mut departments: HashMap<String, Vec<String>>) {
    loop {
        println!(
            "
            Welcome to the Department Viewer, here are the commands: \n
            Press V : View the whole department \n
            Press A : Add an employee to a department \n
            Press L : List the employees of a chosen departemnt
            Press X : Exit
            "
        );

        let departments_array = departments.keys();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error with input");

        let input = input.trim();

        if input.eq("V") {
            println!("{:?}", departments);
        } else if input.eq("A") {
            println!("Here are the departements: {:?}", departments_array);
            println!(
                "
                To add an employee to a department, write his name followed with a comma and the departments name. \n
                Like this: Isabella,Marketing
                "
            );

            let mut new_input = String::new();

            io::stdin()
                .read_line(&mut new_input)
                .expect("Error with input");

            let new_input = new_input.trim();

            let split_input: Vec<&str> = new_input.split(",").collect();

            if split_input.len() != 2 {
                continue;
            }

            let name = split_input[0];
            let department = split_input[1];

            let chosen_department = departments.get_mut(&department.to_string());

            if let Some(d) = chosen_department {
                d.push(name.to_string().clone());
                d.sort();
            }
        } else if input.eq("L") {
            println!("Here are the departements: {:?}", departments_array);
            println!(
                "
                Which department do you want to list?
                "
            );

            let mut new_input = String::new();

            io::stdin()
                .read_line(&mut new_input)
                .expect("Error with input");

            let new_input = new_input.trim();

            let chosen_department = departments.get(&new_input.to_string());

            if let Some(d) = chosen_department {
                println!("{:?}", d);
            }
        } else {
            println!("Very well, have a good day!");
            break;
        }
    }
}

// REVIEW:
//
// - Using `HashMap<String, Vec<String>>` is the correct ownership model for
//   this program. Since employee names and department names come from user
//   input, the map must own the data instead of storing `&str` references.
//   This avoids lifetime issues and ensures the data remains valid after
//   the input strings are dropped.
//
// - Trimming input immediately after `read_line()` (`let input = input.trim();`)
//   is good practice because `read_line()` always includes the trailing newline.
//   This makes command comparisons like `"V"` work correctly.
//
// - Creating `departments_array = departments.keys()` before the command
//   branching works, but it is only used in some branches. It might be clearer
//   to call `departments.keys()` only where it is actually needed.
//
// - The call to `departments.get_mut(&department.to_string())` allocates a new
//   `String` every time. A more efficient approach would be to store the
//   department as `String` earlier or use methods like `HashMap::entry` to
//   avoid unnecessary allocations.
//
// - The expression `d.push(name.to_string().clone())` contains an unnecessary
//   `clone()`. `to_string()` already creates a new owned `String`, so the clone
//   performs an extra allocation and can be removed.
//
// - Sorting the employee list (`d.sort()`) after insertion ensures the
//   department remains alphabetically ordered, which matches the requirement
//   of the exercise. This is a simple and correct approach, though it may be
//   slightly inefficient if the list grows large since the entire vector is
//   re-sorted each time.
//
// - The `"L"` command correctly retrieves a department with `get()` and prints
//   the employee list if the department exists. However, if the department is
//   not found, the program silently does nothing. Providing user feedback in
//   this case could improve the interface.
//
// - Overall, the program demonstrates good use of `HashMap`, `Vec`, ownership,
//   and borrowing. The command loop structure is clear, and the program
//   successfully separates input handling from the data structure logic.
//   With a few small improvements (earlier validation, avoiding extra
//   allocations, and minor ergonomic tweaks), this would be a clean and
//   idiomatic Rust CLI exercise solution.
