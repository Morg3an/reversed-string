# String Reversal Program

## Overview

This project is part of the **Day 5** learning challenge in the 30-Day Rust Challenge. It demonstrates string manipulation concepts in Rust by implementing a program that reverses user-inputted strings.

## Features

- Allows the user to enter strings and outputs the reversed version.
- Provides a user-friendly interface with clear prompts and feedback.
- Continuous input handling: enter multiple strings without restarting the program.
- Exit option for convenience.

## Concepts Covered

- **String Manipulation:** Reversing strings, handling string types in Rust.
- **Input Handling:** Reading user input using `io::stdin`.
- **Error Management:** Graceful handling of invalid input.
- **Control Flow:** Loop-based program flow with conditional exit.

## Code Explanation

```rust
use std::io;

fn main() {
    println!("Welcome to the String Reversal Program!");

    loop {
        println!("\nEnter a string to reverse (or type 'exit' to quit):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Thank you for using the program. Goodbye!");
            break;
        }

        if input.is_empty() {
            println!("Please enter a valid string.");
            continue;
        }

        let reversed: String = input.chars().rev().collect();
        println!("Reversed string: {}", reversed);
    }
}
```

## How to Use
1. Clone the repository:
   ```bash
   git clone https://github.com/Morg3an/reversed-string.git
   ```
2. Navigate to the project directory:
   ```bash
   cd reversed-string
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the project:
   ```bash
   cargo run
   ```
5. Follow the on-screen instructions to select a shape and provide dimensions.

## Example Output
```
Welcome to the String Reversal Program!

Enter a string to reverse (or type 'exit' to quit):
hello
Reversed string: olleh

Enter a string to reverse (or type 'exit' to quit):
Rust
Reversed string: tsuR

Enter a string to reverse (or type 'exit' to quit):
exit
Thank you for using the program. Goodbye!

```

## Dependencies
This project uses the Rust standard library, and the rand crate for random number generation.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)