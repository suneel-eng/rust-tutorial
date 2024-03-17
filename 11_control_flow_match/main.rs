fn main() {
    // ========== Match ========
    // Match is a pattern that executes the code by comparing given value.
    // it is similar to switch statements.

    let kid = "boy";

    match kid {
        "boy" => println!("male"),
        "girl" => println!("female"),
        _ => println!("other")
    }

    // We can assign match pattern to a variable
    let gender = match kid {
        "boy" => "male",
        "girl" => "female",
        _ => "other"
    };

    println!("{}", gender);
}