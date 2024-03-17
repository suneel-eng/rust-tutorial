fn main() {
    // ========== Functions ==========
    // A function is a group of statements that performs specific task.
    // Functions allows reusablility.
    // Function name should follow snake case syntax in rust.
    // Functions can be defined by using "fn" keyword.

    let lower_case = "hello world!";

    let upper_case = transform_to_uppercase(lower_case.to_string());

    println!("{}", upper_case);

}

// If a function needs arguments then you must need to define the data types of those arguments.
// If a function returns values then you must need to define the data types of those return values.
// By default, every function has a return type "unit" which is an empty tuple.
fn transform_to_uppercase(input: String) -> String {
    let output: String = input.to_uppercase();

    // In functions, last line will be returned. so defining return values with "return" keyword is optional.
    // If you want to return the value without "return" keyword, do not use ";" at the end of lastline.

    // When you want to return values in the middle of function then you need to use "return" keyword to return values.
    output
}