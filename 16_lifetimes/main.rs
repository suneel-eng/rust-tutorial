fn main() {
    // ========== Life time generics ===========
    // Every reference in rust has a lifetime, which is the scope for which that reference is valid.

    // Lifetimes are another kind of generic that we've already been using.
    // Rather than ensuring that a type has the behavior we want,
    // lifetimes ensure that references are valid as long as we need them to be.

    // The aim of lifetimes is to prevent dangling references, which cause a program to reference data other
    // than the data it's intended to reference.

    let r; // a variable declared with no initial value. variable is in outer scope.

    { // inner scope
        let x = 5; // variable x is declared with initial value 5.
        r = &x; // value of r is set as reference to x.

        // inner scope ends. all variables of inner scope will be dropped.
    }

    println!("r: {}", r); // compiler will throw error as the x is dropped in inner scope.

    // the solution is

    let x = 5;
    let r = &x;

    println!("r: {}", r);

    // ============ Three lifetime elision rules ===============
    // 1) The compiler assigns a lifetime parameter to each parameter that's a reference.
    // 2) If there is only one input lifetime parameter, that lifetime parameter is assigned to all output life parameters.
    // 3) If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
    // the lifetime of self is assigned to all output lifetime parameters.
}


// This function will throw error. because it won't satisfy lifetime elision rules.
// Solution is that, programmer has to set the lifetime paramters.
fn longest<'a>(string_1: &'a str, string_2: &'a str) -> &'a str {
    if string_1.len() > string_2.len() {
        string_1
    } else {
        string_2
    }
}