# RUST Tutorial

## Recommended
If you are using VScode as your IDE, install **rust-analyzer** extension.

## Covered Topics

### 1) 15/03/2024 - hello world
A simple hello world program.

### 2) 15/03/2024 - cargo
**Cargo** is a package manager, debugger, tester and also a build tool for rust projects.

- Create a new rust project ```cargo new <project-name>```
- Build the project ```cargo build```
- Build and run the project ```cargo run```
- Run tests ```cargo test```
- Build code without executable to check errors ```cargo check```
- Build for production ```cargo build --release```

Every cargo project contains a ```Cargo.toml``` file as a manifest file of the project. This file contains the project related meta data like project name, version, dependencies, etc,.

If you see ```Cargo.lock``` file in your project, it is an auto-generated file by cargo. we don't touch this file.

### 3) 16/03/2024 - variable declarations
About let, mut and const keywords.

### 4) 16/03/2024 - scope and shadowing

### 5) 16/03/2024 - scalar types & data types
A scalar type represents the type of value. Rust has four primary scalar types: integers, characters, booleans and floating point numbers.

- Integer types are i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize and usize. ( signed integer types starts with i instead of u ). bit size of isize and usize depends on the system architecture.
- Floating point number types are f32 and f64 (default).
- Boolan type is bool.
- Character type is char.

### 6) 17/03/2024 - compound types
Covered about compound data types, arrays and tuples.

### 7) 17/03/2024 - functions

### 8) 17/03/2024 - structs
Covered about classic structs and tuple structs.

### 9) 17/03/2024 - enums

### 10) 17/03/2024 - control flow
Covered if, if else, if else if.

### 11) 17/03/2024 - control flow
Covered match.

### 12) 18/03/2024 - loops
Covered loop, while loop and for loop.

### Assignment 1: Guess the number game

### Assignment 2: Temperature converter

### Assignment 3: Nth Fibonacci number

### Assignment 4: Twelve days of christmas song lyrics

### 13) 21/03/2024 - collections
Covered vectors, strings and hash maps.

### 14) 21/03/2024 - error handling

### 15) 22/03/2024 - traits

### 16) 22/03/2024 - lifetime generics

### 17) 22/03/2024 - tests
- By default all tests run in parallel means they run in multithread. Sometimes you want to run the tests one at a time. then you can run the command ```cargo test -- --test-threads=1```.
- By default standard outputs using ```println!``` won't be printed on terminal. they only be printed if the test fails. If you want to show the standard output even the test success, then run the command ```cargo test -- --show-output```.
- If you want to run particular test you can run the command ```cargo test <test-name>```. This command runs every test that contains the test-name.
- If you want to run the ignored tests, run the command ```cargo test -- --ignored```.
- If you want to run the all tests including ignored tests, then run the command ```cargo test -- --include-ignored```.
- **Unit tests** are small and more focused, testing one module in isolation at a time and can test private interfaces.
- **Integration tests** are excercising multiple modules per test.
- After running ```cargo test``` command you see three sections of output include the unit tests, integration tests, and the doc tests. Note that if any test in a section fails, the following sections will not be run.
- If you want to run only particular integration test, run the command ```cargo test --test <test-name>```.