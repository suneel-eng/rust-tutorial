fn main() {
    // ======= [let] keyword ============
    // In resut variables are declare by using "let" keyword.
    let x = 5;

    // By default, variables with "let" declaration are immutable, means
    // the value of the variables can't be modified.
    
    // Below line causes error.
    // x = 6;
    println!("The value of x is {}", x);

    // ============ [mut] keyword =============
    // In order to create a mutable variable you need add "mut" keyword after "let" keyword.
    let mut y = 10;
    println!("The value of y is {}", y); // This line is kept as we need to read the mutable variable atleast once before updating.

    // Now, we can update the value of variable. below line won't throw an error.
    y = 20;
    println!("The value of y after updating is {}", y);

    // ============== [const] keyword ===========
    // To assign constants in rust you need to declare a variable by using "const" keyword.
    // Constant variables name should contain only capital letters and underscores.
    // you must have to assign a datatype to a constant variable.
    const LIMIT_NUMBER: i32 = 100;

    println!("The value of LIMIT_NUMBER is {}", LIMIT_NUMBER);
}