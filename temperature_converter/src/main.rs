use std::io;

fn main() {
    println!("== Temperature Converter ==");

    println!("Choose conversion:");
    println!("1) F to C.");
    println!("2) C to F.");

    loop {
        // variable to store user selection
        let mut selected_conversion: String = String::new();
        // read the user entered data
        io::stdin().read_line(&mut selected_conversion).expect("");
        // trim the data
        let selected_conversion = selected_conversion.trim();

        if selected_conversion == "1" {
            break convert_f_to_c();
        } else if selected_conversion == "2" {
            break convert_c_to_f();
        } else {
            println!("Please enter a valid option 1 or 2.");
            println!("===================================")
        }
    }
}

fn convert_f_to_c() {
    println!("Enter temp in F:");

    loop {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("");

        let temp: f64 = match temp.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter valid temperature.");
                println!("================================");
                continue;
            }
        };

        let temp_in_c: f64 = (temp - 32.0) / 1.8;

        println!("Temperature of {temp} Fahrenheit in celsius is {temp_in_c}.");

        break;
    }
}

fn convert_c_to_f() {
    println!("Enter temp in C:");

    loop {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("");

        let temp: f64 = match temp.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter valid temperature.");
                println!("================================");
                continue;
            }
        };

        let temp_in_f: f64 = (temp * 1.8) + 32.0;

        println!("Temperature of {temp} celsius in Fahrenheit is {temp_in_f}.");

        break;
    }
}
