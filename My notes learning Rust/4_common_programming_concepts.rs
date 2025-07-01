// Variables :
// In Rust, variables are immutable by default.
// This design choice promotes safety and easier concurrency.
// To make a variable mutable, the `mut` keyword must be used before its name, for example: `let mut x = 5;`.
// Attempting to reassign an immutable variable will result in a compile-time error (E0384).
// This error helps prevent bugs where one part of the code assumes a value won't change while another part modifies it.
// While immutability is favored, mutability can be convenient and signals intent to future readers of the code.
//
// Constants are similar to immutable variables but have key differences.
// They are always immutable and cannot use `mut`.
// Constants are declared using the `const` keyword instead of `let`.
// Their type must always be explicitly annotated, for example: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`.
// Constants can be declared in any scope, including the global scope.
// They can only be set to a constant expression, not the result of a value computed at runtime.
// Rust's naming convention for constants is all uppercase with underscores between words.
// Constants are useful for application-specific values that multiple parts of the program might need to know about.
// They also provide a single point of modification for hardcoded values.
//
// Shadowing allows you to declare a new variable with the same name as a previous one by using the `let` keyword again.
// The new variable effectively "shadows" the old one, meaning the compiler will see the new variable.
// Shadowing is different from using `mut` in two important ways:
// 1. Shadowing allows you to perform transformations on a value and then have the variable be immutable after those transformations are complete. If you try to reassign without `let`, you'll get a compile-time error.
// 2. Shadowing allows you to change the type of the value while reusing the same name (e.g., converting a string of spaces to a number representing its length).
// Using `mut` for a type change would result in a compile-time error (E0308) because `mut` does not allow changing a variable's type.
// Shadowing helps avoid the need for different variable names like `spaces_str` and `spaces_num`.

// Data Types :
// Every value in Rust has a data type, which tells Rust how to work with it.
// Rust is a statically typed language, meaning types are known at compile time.
// The compiler usually infers types, but explicit type annotations like `: u32`
// are needed when multiple types are possible, for example, with `parse()`.
//
// Data types are categorized into two subsets: scalar and compound.
//
// Scalar Types represent a single value. Rust has four primary scalar types:
// - Integers: Numbers without fractional components.
//   - Signed (i8, i16, i32, i64, i128) can be positive or negative.
//   - Unsigned (u8, u16, u32, u64, u128) are only positive.
//   - `isize` and `usize` are architecture-dependent (32 or 64 bits) and are
//     primarily used for indexing collections.
//   - The default integer type is i32.
//   - Integer literals can be decimal (98_222), hex (0xff), octal (0o77),
//     binary (0b1111_0000), or byte (b'A', u8 only). Underscores `_` can be
//     used for readability (e.g., 1_000).
//   - Integer overflow: In debug mode, it causes a program panic. In release
//     mode, it performs two's complement wrapping. Explicit handling methods
//     like `wrapping_*`, `checked_*`, `overflowing_*`, and `saturating_*`
//     are available for primitive numeric types.
// - Floating-point numbers: Numbers with decimal points.
//   - `f32` (32-bit) and `f64` (64-bit). The default is f64 for precision.
//   - All floating-point types are signed and follow the IEEE-754 standard.
// - Booleans: The `bool` type, with values `true` or `false`. They are one
//   byte in size and are commonly used in conditional expressions.
// - Characters: The `char` type, specified with single quotes (e.g., 'z').
//   They are four bytes in size and represent a Unicode Scalar Value, allowing
//   for a wide range of characters beyond ASCII.
//
// Numeric Operations: Rust supports standard mathematical operations: addition,
// subtraction, multiplication, division (integer division truncates toward zero),
// and remainder.
//
// Compound Types group multiple values into one type:
// - Tuples: A fixed-length way to group values of different types into a single
//   compound type.
//   - Created by a comma-separated list inside parentheses: `(500, 6.4, 1)`.
//   - Values can be extracted using pattern matching (destructuring):
//     `let (x, y, z) = tup;`.
//   - Elements can also be accessed directly by index: `tup.0`, `tup.1`.
//   - The empty tuple `()` is called "unit" and represents an empty value or
//     an empty return type.
// - Arrays: A fixed-length collection where all elements must have the same type.
//   - Values are written as a comma-separated list inside square brackets:
//     `[1, 2, 3, 4, 5]`.
//   - Array types are specified as `[type; length]` (e.g., `[i32; 5]`).
//   - Arrays can be initialized with a repeated value: `[3; 5]` creates an
//     array of five 3s.
//   - Elements are accessed by index: `a[0]`.
//   - Accessing an array element with an index that is out of bounds will cause
//     the program to panic at runtime, ensuring memory safety by preventing
//     access to invalid memory locations.
//   - For collections that can grow or shrink in size, the `Vec` (vector) type
//     from the standard library is generally preferred over arrays.

// Functions :
// Functions are declared using the `fn` keyword.
// The `main` function is the entry point of many Rust programs.
// Rust convention for function and variable names is snake_case (lowercase with underscores).
// Function bodies are enclosed in curly brackets `{}`.
// Functions can be called by their name followed by parentheses `()`.
// The order of function definitions does not matter, as long as they are in scope.
//
// Functions can have parameters, which are special variables in the function's signature.
// You must declare the type of each parameter in the function signature (e.g., `x: i32`).
// Multiple parameters are separated by commas (e.g., `(value: i32, unit_label: char)`).
//
// Function bodies consist of statements and expressions.
// Statements perform actions and do not return a value (e.g., `let y = 6;`).
// Expressions evaluate to a resultant value (e.g., `5 + 6`, `x + 1`).
// Calling a function or macro is an an expression.
// A new scope block created with curly brackets `{}` can also be an expression.
// Expressions do not include ending semicolons.
// Adding a semicolon to an expression turns it into a statement, preventing it from returning a value.
// You cannot assign a `let` statement to another variable because statements do not return values.
//
// Functions can return values, declared after an arrow `->` in the signature (e.g., `-> i32`).
// The return value is typically the value of the final expression in the function body.
// If the final expression does not have a semicolon, its value is implicitly returned.
// Using the `return` keyword allows for early returns.
// If a function is declared to return a value but its body ends with a statement (due to a semicolon),
// it implicitly returns `()` (the unit type), leading to a type mismatch error.

// Comments :
// Comments in Rust are written using `//` for single-line comments and `/* ... */` for multi-line comments.
// Comments are not included in the compiled code.
//
// Single-line comments start with `//` and continue to the end of the line.

// Control Flow :
// Basic building blocks for controlling execution are if expressions and loops.
//
// if Expressions:
// Allow branching code based on conditions.
// Syntax: `if condition { ... } else { ... }`.
// The condition must always be a `bool`. Rust does not implicitly convert other types to `bool`.
// `else if` can handle multiple conditions; only the first true condition's block executes.
// `if` is an expression, meaning its result can be assigned to a variable.
// All branches (if, else if, else) of an `if` expression must return the same type.
//
// Repetition with Loops:
// Rust has three types of loops: `loop`, `while`, and `for`.
//
// `loop`:
// Executes a block of code repeatedly forever until explicitly stopped.
// `break` keyword stops the loop.
// `continue` keyword skips to the next iteration.
// A `loop` can return a value by placing it after the `break` expression.
// `return` exits the current function, not just the loop.
// Loop labels (e.g., `'label: loop { ... }`) can be used with `break 'label` or `continue 'label`
// to specify which nested loop to affect.
//
// `while`:
// Executes a block of code repeatedly as long as a given condition is true.
// More concise than combining `loop`, `if`, and `break` for conditional repetition.
// Can iterate over collections, but is less safe and potentially slower than `for` loops
// due to manual index management and runtime bounds checks.
//
// `for`:
// The most commonly used loop construct in Rust.
// Iterates safely and concisely over each item in a collection.
// Example: `for element in collection { ... }`.
// Eliminates the risk of out-of-bounds errors and is generally preferred for iterating collections.
// Can also be used to run code a certain number of times using a `Range` (e.g., `(1..4).rev()`).
