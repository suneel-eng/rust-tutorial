fn main() {
    // ========== Compound types =============
    // Compound types can group multiple types into one type.
    // Rust has two primitive compound types: 1) Tuple type and 2) Array type

    // ======= Tuple type ==========
    // Tuples are homogenous means tuples can contain vallues with different data types.
    // Tuple values has to be defined in "()".
    // Length of tuple is fixed. it can't be modify in run time.
    // Accessing out of range values will throw error.

    let tuple: (bool, u8, char, f64) = (true, 1, 'C', 3.23);

    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;
    let d = tuple.3;

    // Below line also works to get the values from tuple.
    // let (a, b, c, d) = tuple;

    println!("{a} {b} {c} {d}");

    // ========== Array type ===========
    // Arrays are Heterogenous means Arrays can contain values with same data type.
    // Arrays values has to be defined in "[]".
    // Length of array is fixed. it can't be modified in run time.
    // Accessing out of range values will throw error.

    let array: [ u8; 3 ] = [ 1, 2, 3 ];

    let a1 = array[0];
    let a2 = array[1];
    let a3 = array[2];
 
    // Below line also works to get the values from array.
    // let [ a1, a2, a3 ] = array;

    println!("{a1} {a2} {a3}");
}