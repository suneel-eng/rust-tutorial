fn main() {
    // ========= Error Handling ==========
    // In rust, Error are two types.
    // 1) Recoverable errors.
    // 2) Unrecoverable errors.

    println!("Hello world!");
    // Unrecoverable errors with panic macro.
    // By default panics, print the failure message, unwind, clean up the stack and quit.

    // Comment below line to see panic in action.
    // panic!("crash and burn");

    // Recoverable errors with Result
    // Result is an enum.
    // enum Result<T, E> {
    //  Ok(T),
    //  Err(E)
    // }
    use std::fs::File;
    use std::io::ErrorKind;

    let greeting_file_result = File::open("hello.txt"); // The return type of [open] method is Result<T, E>

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(nfile) => nfile,
                Err(n_err) => panic!("Error occured: Unable to create file {:?}.", n_err)
            },
            _ => panic!("Problem with opening the file.")
        }
    };

    println!("greeting file {:?}", greeting_file);
}