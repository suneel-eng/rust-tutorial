fn main() {
    // ========== Iterators ============

    // The iterator pattern allows you to perform some task on a sequence of items in turn.

    // Below code creates an iterator over the items in the vector v1 by calling the iter() method.
    
    let v1 = vec![ 1, 2, 3 ];

    println!("{:?}", v1);

    for val in &v1 {
        println!("Value: {}", &val);
    }

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Value: {}", val);
    }

    let mut v1_iter = v1.iter();

    println!("First value: {:?}", v1_iter.next());
    println!("Second value: {:?}", v1_iter.next());
    println!("Third value: {:?}", v1_iter.next());
    println!("Fourth value: {:?}", v1_iter.next());

    let v1 = vec![ 1, 2, 3 ];

    let v1_iter = v1.iter();

    let total: u32 = v1_iter.sum();

    println!("Total is {}", total);

    let v1: Vec<u32> = vec![ 1, 2, 3 ];

    let v2: Vec<u32> = v1.iter().map(|x| x + 1).collect();

    println!("v1 -> {:?}, v2 -> {:?}", v1, v2);
}