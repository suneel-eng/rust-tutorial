use std::io;

fn main() {
    println!("======== N-th Fibonacci number ===========");

    println!("Enter 'n' value:");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("");

    let n: u64 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!("Please enter a valid positive number.")
    };

    let n_position = n;
    let nth_value: u64 = fibonnaci_at_n(n);

    println!("Fibonacci value at {n_position}th position is: {nth_value}");
}

fn fibonnaci_at_n(n: u64) -> u64 {

    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fibonnaci_at_n(n - 1) + fibonnaci_at_n(n - 2);
}
