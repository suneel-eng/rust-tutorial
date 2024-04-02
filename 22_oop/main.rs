fn main() {
    // ====== OOP ( Opject Oriented Programming Concepts) =============

    // 1) Encapsulation
    // Encapsulation means that the implementation details of an object aren’t accessible to code using that object.
    // Therefore, the only way to interact with an object is through it's public API.

    mod Class {
        pub struct List {
            numbers: Vec<u8>
        }

        impl List {

            pub fn new() -> Self {
                List {
                    numbers: vec![]
                }
            }

            pub fn add(&mut self, num: u8) {
                self.numbers.push(num);
            }

            pub fn remove(&mut self) {
                self.numbers.pop();
            }

            pub fn items(&self) -> Vec<u8> {
                self.numbers.clone()
            }
        }
    }

    let mut list = Class::List::new();

    list.add(2);

    list.add(5);

    list.add(3);

    println!("{:?}", list.items());

    list.remove();

    println!("{:?}", list.items());

    // 2) Inheritence
    // Inheritance is a mechanism whereby an object can inherit elements from another object’s definition,
    // thus gaining the parent object’s data and behavior without you having to define them again.

    // 3) Abstarction

    

    

    // 4) Polymorphism
}