use std::io;

fn main() {
    loop {
        println!("Enter a string to reverse (or type 'exit' to quit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        // Exit condition
        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // Check if input is empty
        if input.is_empty() {
            println!("You entered an empty string. Please try again.");
            continue;
        }

        // Reverse the string
        let reversed: String = input.chars().rev().collect();
        println!("Reversed string: {}", reversed);
    }
}