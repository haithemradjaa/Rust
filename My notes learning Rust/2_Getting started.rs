// # Getting Started with Rust
//
// This section guides you through installing Rust and setting up your development environment.
//
// ## Installation
//
// Rust is installed via rustup, a command-line tool for managing Rust versions and associated tools.
// An internet connection is required for the download.
//
// Rust's stability guarantees ensure that examples in this book will continue to compile with newer
// stable Rust versions.
//
// ### Command Line Notation
//
// Throughout this book, commands to be entered in a terminal start with $. You should not type
// the $ character; it indicates the command prompt. Lines without $ typically show command output.
// PowerShell-specific examples will use > instead of $.
//
// ### Installing rustup on Linux or macOS
//
// Open your terminal and run:
// $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
// This downloads and runs a script to install rustup and the latest stable Rust version.
// Upon successful installation, you should see: Rust is installed now. Great!
//
// You will also need a linker. If you encounter linker errors, install a C compiler:
// *   macOS:
//     $ xcode-select --install
// *   Linux: Install GCC or Clang (e.g., build-essential package on Ubuntu).
//
// ### Installing rustup on Windows
//
// Go to https://www.rust-lang.org/tools/install and follow the instructions.
// You will be prompted to install Visual Studio, which provides a linker and necessary native libraries.
// More help: https://rust-lang.github.io/rustup/installation/windows-msvc.html
//
// ## Troubleshooting
//
// To verify your Rust installation, open a shell and run:
// $ rustc --version
// You should see output similar to: rustc x.y.z (abcabcabc yyyy-mm-dd).
//
// If not, check that Rust is in your system's PATH variable:
// *   Windows CMD: > echo %PATH%
// *   PowerShell: > echo $env:Path
// *   Linux/macOS: $ echo $PATH
//
// For further help, refer to the Rust community page.
//
// ## Updating and Uninstalling
//
// *   Update Rust:
//     $ rustup update
// *   Uninstall Rust and rustup:
//     $ rustup self uninstall
//
// ## Local Documentation
//
// Rust installation includes local documentation. To open it in your browser:
// $ rustup doc
// This is useful for looking up standard library types and functions offline.
//
// ## Text Editors and Integrated Development Environments (IDEs)
//
// Any text editor can be used. Many editors and IDEs offer built-in Rust support.
// A current list can be found on the tools page (https://www.rust-lang.org/tools) of the Rust website.
//
// ## Working Offline with This Book
//
// Some examples use Rust packages beyond the standard library, requiring an internet connection
// or pre-downloaded dependencies. To cache dependencies for offline use:
// $ cargo new get-dependencies
// $ cd get-dependencies
// $ cargo add rand@0.8.5 trpl@0.2.0
// (Cargo will be explained later.) After running this, you can use the --offline flag with
// cargo commands throughout the book to use cached versions.

fn main () {
    println!("Hello, world!");
}

// Now that Rust is installed, it's time to write your first program: "Hello, world!".
// This book assumes basic command-line familiarity, but IDEs with Rust support are also an option.
//
// ## Creating a Project Directory
// It's suggested to create a `projects` directory in your home directory,
// and then a `hello_world` directory inside it.
//
// For Linux, macOS, PowerShell:
// $ mkdir ~/projects
// $ cd ~/projects
// $ mkdir hello_world
// $ cd hello_world
//
// For Windows CMD:
// > mkdir "%USERPROFILE%\projects"
// > cd /d "%USERPROFILE%\projects"
// > mkdir hello_world
// > cd hello_world
//
// ## Writing and Running a Rust Program
// Create a new source file named `main.rs` (Rust files end with `.rs`).
// Use underscores for multi-word filenames (e.g., `hello_world.rs`).
//
// Inside `main.rs`, add the following code:
// Filename: main.rs
// fn main() {
//     println!("Hello, world!");
// }
//
// Save the file. Then, in your terminal within the `~/projects/hello_world` directory,
// compile and run the program:
//
// For Linux or macOS:
// $ rustc main.rs
// $ ./main
// Hello, world!
//
// For Windows:
// > rustc main.rs
// > .\main
// Hello, world!
//
// If "Hello, world!" prints, congratulations! You've written your first Rust program.
//
// ## Anatomy of a Rust Program
// *   `fn main() { ... }`: Defines the `main` function, the entry point for every executable Rust program.
//     It takes no parameters and returns nothing.
//     Curly brackets `{}` are required for all function bodies.
// *   `println!("Hello, world!");`: This line prints text to the screen.
//     *   `println!`: Calls a Rust macro (indicated by the `!`). Macros generate code and extend Rust syntax.
//     *   `"Hello, world!"`: The string argument passed to the macro.
//     *   `;`: A semicolon indicates the end of an expression. Most lines of Rust code end with a semicolon.
//
// ## Compiling and Running Are Separate Steps
// Rust is an ahead-of-time (AOT) compiled language.
// 1.  **Compile**: Use `rustc main.rs`. This creates an executable file (e.g., `main` on Linux/macOS, `main.exe` on Windows).
//     On Windows, a `.pdb` debugging file is also created.
// 2.  **Run**: Execute the compiled binary (e.g., `./main` or `.\main`).
//
// This separation means you can compile a program and distribute the executable,
// and others can run it without needing Rust installed.
//
// While `rustc` is fine for simple programs, for larger projects, you'll use Cargo,
// which will be introduced next.

// // Cargo: Rust's Build System and Package Manager
// // Cargo is Rust's official build system and package manager. It is widely used by Rust developers to manage their projects because it automates many common tasks. These tasks include building your code, downloading external libraries (which are called dependencies), and compiling those libraries.
// // For simple Rust programs, Cargo primarily handles building your code. As your projects become more complex and require external libraries, Cargo simplifies the process of adding and managing these dependencies.
// // The vast majority of Rust projects utilize Cargo, and this book assumes you will be using it too. Cargo is typically installed automatically when you use the official Rust installers. You can verify its installation by running `cargo --version` in your terminal. If you encounter an error, you may need to install Cargo separately based on your Rust installation method.
//
// // Creating a Project with Cargo
// // To start a new Rust project using Cargo, navigate to your desired projects directory and execute the following commands:
// // $ cargo new hello_cargo
// // $ cd hello_cargo
// // The `cargo new` command creates a new directory (e.g., `hello_cargo`) and sets up a new Rust project within it.
// // Inside the new project directory, Cargo generates a standard structure:
// // *   `Cargo.toml`: This is the project's manifest file, written in TOML (Tom's Obvious, Minimal Language) format. It contains essential configuration information for your package, such as its `name`, `version`, and the `edition` of Rust to use. It also includes a `[dependencies]` section where you will list any external crates (Rust packages) your project relies on.
// // *   `src/main.rs`: This file contains the default "Hello, world!" Rust source code. Cargo expects all your project's source files to reside within the `src` directory. The top-level project directory is reserved for configuration files, documentation, and other non-code assets.
// // By default, Cargo also initializes a new Git repository for version control. This behavior can be customized using the `--vcs` flag with `cargo new`.
// // If you have an existing Rust project that was not created with Cargo, you can convert it by moving your source code into a `src` directory and then running `cargo init` in the project's root to generate a `Cargo.toml` file.
//
// // Building and Running a Cargo Project
// // From your project's root directory (e.g., `hello_cargo`), you can use Cargo commands to build and run your program:
// // *   `cargo build`: This command compiles your project. The resulting executable is placed in the `target/debug/` directory (e.g., `target/debug/hello_cargo` on Linux/macOS or `target\debug\hello_cargo.exe` on Windows). This is a debug build, suitable for development. The first time you run `cargo build`, Cargo also creates a `Cargo.lock` file, which precisely tracks the versions of all dependencies. This file is managed by Cargo and should not be edited manually.
// // *   `cargo run`: This is a convenient command that combines compilation and execution. It first checks if your code has changed and recompiles if necessary, then runs the generated executable. Most developers use `cargo run` during active development.
// // *   `cargo check`: This command quickly checks your code for compilation errors without actually producing an executable binary. It is significantly faster than `cargo build` and is highly recommended for frequent use while writing code to ensure your project remains compilable. Many Rustaceans run `cargo check` periodically to catch errors early.
//
// // Building for Release
// // When your project is ready for final deployment or benchmarking, you should compile it with optimizations using `cargo build --release`. This command generates an optimized executable in the `target/release/` directory. Optimized builds run faster but take longer to compile. This distinction allows for quick iteration during development (debug builds) and maximum performance for the final product (release builds).
//
// // Cargo as Convention
// // While `rustc` can compile simple single-file programs, Cargo's value becomes evident as projects grow in complexity, involving multiple files or external dependencies. Cargo streamlines the build process and dependency management significantly.
// // Using Cargo establishes a consistent and conventional project structure and workflow, which is beneficial for collaboration and working with existing Rust projects. The commands for building and running are consistent across all operating systems, simplifying development.
// // For example, to work on an existing project from a Git repository, you would typically:
// // $ git clone example.org/someproject
// // $ cd someproject
// // $ cargo build
// // This demonstrates how Cargo integrates seamlessly into common development workflows.
// // End Generation Here

