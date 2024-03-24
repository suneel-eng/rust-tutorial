fn main() {
    // ========= Closures ===========
    // Rust's closures are anonymous functions you can save in a variable or pass as arguments to other functions.

    // Closures can capture values from their environment in three ways,
    // which directly map to the three ways a function can take a parameter:
    // borrowing immutably, borrowing mutably, and taking ownership.
    // The closure will decide which of these to use based on what the body of the function does with the captured values.


    let mut number: u32 = 10;

      // Below closure is modifying the captured value by taking mutable reference.
      let mut closure = || {
        println!("Calculating....");
        number += 5;
    };

    closure();
    closure();
    closure();

    println!("{}", number);

    let number: u32 = 10;

    // Below closure printing the captured value by taking immutable reference.
    let closure = || {
        println!("number is {}", number);
    };

    closure();

    let number = String::from("Hello");

    // Below closure is moving the captured value into other variable by moving ownership.
    let closure = || {
        let closure_number = number;
        println!("Closure number is {}", closure_number);
        // below line won't work as the number value is moved to other variable within closure.
        // println!("Original number is {}", number);
    };

    // below line won't work as the number value is moved to other variable within closure.
    // println!("Original number is {}", number);

    closure();

    // ============== Fn Traits ==============

    // The way a closure captures and handles values from the environment affects which traits the closure implements,
    // and traits are how functions and structs can specify what kinds of closures they can use.
    // Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion,
    // depending on how the closure’s body handles the values:

    // 1) FnOnce: Applies to closures that can be called once. All closures atleast implement this trait.
    // A closure that moves captured values out of its body will implement FnOnce trait.
    // 2) FnMut: Applies to closures that don't move captured values out of their body, but that might mutate
    // the captured value. These closures can be called more than once.
    // 3) Fn: Applies to closures that don't move captured values out of their body and that don't mutate captured values.
    // As well as closures that capture nothing from their environment. These closured can be called more than once.

    // Note: Functions can implement all three of Fn traits too.
}