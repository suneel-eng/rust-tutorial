fn main() {
    // ====== Enums =========
    // Enums give you a way of saying a value is one of possible set of values.
    // An enum is also a custom data type.
    // We have to use "enum" keyword to create enums.

    enum Directions {
        West,
        East,
        South,
        North
    }

    // We can access the possible value as below.
    let north = Directions::North;

    // ===== Enums with generics ========
    enum Option<T> {
        Some(T),
        None
    }

    let some = Some(10);
}