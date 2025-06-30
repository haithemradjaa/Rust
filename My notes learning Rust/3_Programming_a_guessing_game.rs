use std::io;
use rand::Rng;
use std::cmp::Ordering

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess :");

        let mut guess = String::new();

        io::stdin() // or std::io::stdin() if io is not imported before
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }

        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!")
                break;
            }
        }
    }
}

// This chapter introduces Rust concepts through a hands-on guessing game project. You'll learn about variables, methods, functions, and external libraries. The game will generate a random number between 1 and 100, prompt the player for a guess, and then indicate if the guess is too low, too high, or correct.
//
// To set up the project, navigate to your projects directory and use Cargo: `cargo new guessing_game` followed by `cd guessing_game`. Cargo creates a new directory with a `Cargo.toml` file (for project configuration, name, version, and dependencies) and a `src/main.rs` file containing a default "Hello, world!" program. You can compile and run this initial program using `cargo run`.
//
// The first step in the game is to process user input. In `src/main.rs`, you'll start by adding `use std::io;` to bring the standard input/output library into scope, which is necessary for handling user input. The `main` function is the program's entry point.
//
// You'll use `println!` to display messages to the user, such as "Guess the number!" and "Please input your guess."
//
// To store the user's guess, you'll create a mutable variable: `let mut guess = String::new();`. In Rust, variables are immutable by default, so `mut` is used to make `guess` changeable. `String::new()` is an associated function of the `String` type that creates a new, empty, growable string. The `::` syntax indicates an associated function.
//
// To receive input, `io::stdin()` gets a handle to the standard input. Then, `.read_line(&mut guess)` is called on this handle to read the user's input and append it to the `guess` string. The `&mut guess` part means you're passing a mutable reference to the `guess` variable, allowing `read_line` to modify its content without copying the entire string.
//
// The `read_line` method returns a `Result` type, which is an enum that can be either `Ok` (indicating success with a value) or `Err` (indicating failure with error information). To handle this, you'll chain `.expect("Failed to read line");` to the `read_line` call. If `read_line` returns an `Err`, `expect` will cause the program to crash and display the provided message. If it's `Ok`, `expect` simply returns the successful value (the number of bytes read). Rust will warn you if you don't handle the `Result` value.
//
// Finally, `println!("You guessed: {guess}");` prints the user's input. The curly brackets `{}` act as placeholders for values. You can put a variable name inside, like `{guess}`, or use empty curly brackets `{}` for expressions, providing the expression after a comma. For example, to print a variable `x` and the result of `y + 2`, you would describe it as `println!("x = {x} and y + 2 = {}", y + 2);` which would output 'x = 5 and y + 2 = 12' if x is 5 and y is 10.
//
// After implementing this code, you can run `cargo run` to test it. The program will prompt for input, and then print back what you typed.

// Notes on Programming the Guessing Game:
//
// *   **Generating the Secret Number:**
//     *   To enable random number generation, `use rand::Rng;` is added to bring the `Rng` trait into scope. This trait defines the methods used by random number generators.
//     *   Inside the `main` function, `rand::thread_rng()` is called. This function provides a random number generator that is local to the current thread and is seeded by the operating system, ensuring different numbers each run.
//     *   The `gen_range(1..=100)` method is then called on this generator. This method, part of the `Rng` trait, generates a random integer inclusively between 1 and 100.
//     *   A `println!` statement temporarily displays the `secret_number` for debugging purposes during development. This line will be removed in the final version of the game.
//     *   For exploring other functionalities of the `rand` crate, `cargo doc --open` can be used to build and view its local documentation in a browser.
//
// *   **Comparing the Guess to the Secret Number:**
//     *   The `use std::cmp::Ordering;` statement is added to import the `Ordering` enum from the standard library. `Ordering` has three variants: `Less`, `Greater`, and `Equal`, representing the possible outcomes of a comparison.
//     *   The `cmp` method is used to compare the user's `guess` with the `secret_number`. It takes a reference to the value being compared (e.g., `&secret_number`).
//     *   A `match` expression is then used to handle the `Ordering` variant returned by `cmp`. Each "arm" of the `match` expression consists of a pattern (e.g., `Ordering::Less`) and the code to execute if the value matches that pattern.
//     *   This `match` structure ensures that all three comparison outcomes are explicitly handled, providing appropriate feedback to the user ("Too small!", "Too big!", or "You win!").
//
// *   **Handling Type Mismatch and Input Conversion:**
//     *   Initially, attempting to compare the user's input (`guess`, which is a `String`) directly with the `secret_number` (an integer type) results in a "mismatched types" compilation error due to Rust's strong, static type system.
//     *   To resolve this, the `String` input must be converted into a numerical type. This is achieved with the line: `let guess: u32 = guess.trim().parse().expect("Please type a number!");`
//     *   **Shadowing:** Rust allows "shadowing" a variable, meaning a new variable with the same name (`guess`) can be declared, effectively replacing the previous `String` variable with a new one of type `u32`. This is a common pattern for type conversions.
//     *   **`trim()` method:** This method is called on the `String` `guess` to remove any leading or trailing whitespace, including the newline character (`\n` or `\r\n`) that `read_line` appends when the user presses Enter.
//     *   **`parse()` method:** This method attempts to convert the cleaned string into another type. We explicitly annotate the type as `u32` (`unsigned 32-bit integer`), which is a good default choice for small positive numbers. This type annotation also helps Rust infer that `secret_number` should also be a `u32`, ensuring consistent types for comparison.
//     *   **Error Handling with `expect()`:** The `parse()` method returns a `Result` type because the conversion might fail (e.g., if the input string is not a valid number). Similar to `read_line`, `expect("Please type a number!")` is used to handle this `Result`. If `parse()` returns an `Err`, the program will crash and display the provided message; otherwise, it returns the successfully parsed number.


