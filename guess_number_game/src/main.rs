use rand::Rng;
use std::cmp::Ordering;
use std::io; // import Standard input/output [I/O] library.

fn main() {
    println!("Welcom to Guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("========================");
        
        println!("Please input your guess.");

        let mut guess = String::new();

        // The "&" indicates that the argument is a reference,
        // Which gives a way to let multiple parts of your code access
        // one peice of data without needing to copy that data
        // into memory multiple times.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Read user input.

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter only numbers.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low..."),
            Ordering::Greater => println!("Too Hight..."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
