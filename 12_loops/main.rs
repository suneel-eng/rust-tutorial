fn main() {
    // ========== loop ===========
    // A loop defined with "loop" keyword is runs infinitely.
    // It only quits the loop by using "break" keyword or when program exits.

    let mut i = 0;

    loop {
        println!("I run forever.");

        if i == 5 {
            println!("But you can break me on some condition.");
            break;
        }

        i = i + 1;
    }

    // ============ while loop ===========
    // while loops execute the code until the condition is true.
    // using "break" keyword is not required. but you can use it whenever required.

    let mut number = 5;

    while number != 0 {
        println!("number is not equal to zero.");
        number -= 1;
    }

    println!("number is equal to zero.");

    // ============= for loops ===========
    // for loops execute the code for a given number of times.

    let array: [ u8; 5 ] = [ 1, 2, 3, 4, 5 ];

    for element in array.iter() {
        println!("Current element is: {}", element);
    }
}