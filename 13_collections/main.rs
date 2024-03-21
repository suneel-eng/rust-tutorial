fn main() {
    // ============ Collections ===========
    // The data of collections is stored in the heap becuase the data varies while code runs.
    // We discuss some collections Vectors, Strings and Hasmaps.
    

    // ============= Vectors ============
    // Vectors are used to store list of values of smae type.
    // Vector puts all the values next to each other in the memory.

    let vector: Vec<i32> = Vec::new();

    let vector2 = vec![ 1, 2, 3 ];

    println!("{:?}", vector);

    println!("{:?}", vector2);

    let mut vector3: Vec<String> = Vec::new();

    vector3.push(String::from("Hello 1"));
    vector3.push(String::from("Hello 2"));
    vector3.push(String::from("Hello 3"));

    println!("{:?}", vector3);

    let number: &String = &vector3[2];
    println!("{number}");
    println!("Element at index 2 in vector3 is: {}", vector3[2]);

    match vector3.get(2) {
        Some(value) => println!("Element at index 2 in vector3 is: {}", value),
        None => println!("There is no element at index 2 in vector3.")
    }

    match vector3.get(5) {
        Some(value) => println!("Element at index 5 in vector3 is: {}", value),
        None => println!("There is no element at index 5 in vector3.")
    }

    for i in &vector3 {
        println!("> {i}");
    }

    // ============= Strings ==============

    // Create a new string
    let s = String::new();
    println!("This is empty {} string.", s);
    // Convert &str to string

    let data: &str = "Hello";
    let s: String = data.to_string();
    println!("Below line same as this {s}");
    // or
    let mut s: String = String::from(data);
    println!("{s}");
    // push string
    s.push_str("World");
    println!("After adding world: {s}");
    // push single letter
    s.push('l');
    println!("After adding l: {s}");
    let a = 10;

    let mut s: String = String::new();
    s = format!("The number is {a}");
    println!("After formatting: {s}");
    // Iterating over characters of a string
    for letter in s.chars() {
        println!("{letter}");
    }

    // Iterating over bytes of a string
    for byte in s.bytes() {
        println!("{byte}");
    }

    // ============ HashMaps ==============
    use std::collections::HashMap;

    let mut map = HashMap::new();

    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Red"), 20);

    println!("{:?}", map);
    println!("{:#?}", map); // pretty print

    // Accessing values from hasmap
    let score = map.get("Blue").copied().unwrap_or(0);

    println!("Blue team score is {score}.");

    // Iterating over each entry of hashmap
    for (key, value) in &map {
        println!("{key}: {value}");
    }

    // Add key value only if key is not present
    map.entry(String::from("Green")).or_insert(50);

    println!("{:#?}", map);
}