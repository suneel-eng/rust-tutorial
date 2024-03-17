fn main() {
    // ======= If =========
    let x = 10;

    if x % 2 == 0 {
        println!("x is divisible by 2.");
    }

    // ======= If else =======
    let age = 13;

    if age >= 18 {
        println!("Major");
    } else {
        println!("Minor");
    }

    // ======== If else if ===========
    let number = 10;

    if number > 0 {
        println!("number is positive.");
    } else if number < 0 {
        println!("number is negative.");
    } else {
        println!("number is 0.");
    }

    // ======= if statement in let ==========
    let kid = "boy";

    let gender = if kid == "boy" { "male" } else { "female" };

    println!("{}", gender);
}