// Ownership is Rust's system for managing memory.
// It uses a set of rules checked by the compiler; violations prevent compilation.
// Ownership features do not slow down program execution.

// Memory is managed using the Stack and the Heap.
// Stack: Stores fixed-size values in a Last In, First Out (LIFO) order. Fast.
// Heap: Stores values of unknown or changing size. Slower, requires allocation.
// Data on the heap is accessed via a pointer stored on the stack.
// Ownership primarily manages data on the heap.

// The three core Ownership Rules are:
// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value is dropped (memory freed).

// A variable's scope is the range where it is valid, from declaration to scope end.

// The `String` type manages mutable, growable text data on the heap.
// String literals are immutable and hardcoded into the executable.
// `String::from()` requests memory on the heap.
// When a `String` variable goes out of scope, Rust automatically calls `drop` to free its memory.

// When assigning a `String` (e.g., `let s2 = s1;`), Rust performs a "move".
// The stack data (pointer, length, capacity) is copied, but the heap data is not.
// The original variable (`s1`) becomes invalid to prevent double-free errors.
// Rust never automatically creates expensive "deep" copies.

// Assigning a new `String` to an existing variable immediately frees the memory of the original `String`.

// To explicitly create a deep copy of `String` data, use the `clone()` method.
// `clone()` indicates a potentially expensive operation.

// Types with a known, fixed size (like integers, booleans, floats, chars, and tuples of these) are stored on the stack.
// These types implement the `Copy` trait.
// When assigned, they are trivially copied, and the original variable remains valid.
// Types that implement `Drop` cannot implement `Copy`.

// Passing a value to a function works like assignment: it moves or copies.
// If a `String` is passed, its ownership moves into the function, making it invalid outside.
// If a `Copy` type is passed, it's copied, and the original remains valid.

// Returning a value from a function also transfers ownership.
// To use a value in a function without transferring ownership, Rust uses "references".

