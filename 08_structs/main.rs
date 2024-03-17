fn main() {
    // ============ Struct ==============
    // Struct is used to structure the related data.
    // it is similar to tuples but unlike tuples, we can give the type and name values in structs.
    // To define a struct we have to use "struct" keyword.

    // ======= Classic struct ===========
    // Classic structs are used to create custom type for an object
    struct Car {
        model: String,
        make: String,
        year: u32
    }

    // We define a new value with Car struct as below
    let car = Car {
        model: String::from("Thar"),
        make: String::from("Mahindra"),
        year: 2020
    };

    // We can access the values of struct by using dot notation as below
    let car_model = car.model;
    let car_make = car.make;
    let car_year = car.year;

    println!("Car model is: {car_model}, make is: {car_make}, year is: {car_year}");

    // ============ Tuple struct =============
    // Tuple struct are used to create a custom type for tuples.
    struct Point2D(u32, u32);

    let origin = Point2D(10, 20);

    println!("x: {}, y: {}", origin.0, origin.1);

    let Point2D(x, y) = origin;

    println!("x: {x}, y: {y}");
}