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

    struct Cat;

    struct Dog;

    struct Eagle;

    trait AnimalActions {
        fn speak(&self);
    }

    trait BirdActions {
        fn fly(&self);
    }

    impl AnimalActions for Cat {
        fn speak(&self) {
            println!("Meowwwww....");
        }
    }

    impl AnimalActions for Dog {
        fn speak(&self) {
            println!("Bhow Bhow Bhow ...")
        }
    }

    impl BirdActions for Eagle {
        fn fly(&self) {
            println!("I can fly...")
        }
    }

    let cat = Cat{};
    let dog = Dog{};
    let eagle = Eagle{};

    fn speak<T: AnimalActions>(object: T) {
        object.speak();
    }

    fn fly<T: BirdActions>(object: T) {
        object.fly();
    }

    speak(cat);
    speak(dog);

    fly(eagle);

    // fly(cat); // this will throw error as Cat struct not implemented BirdActions trait.


    // 3) Abstarction
    // The process of hiding the intricate code implementation details from the user and
    // just provides the user with the necessary information.
    
    // By default every field/method in struct is private. you can make them public by using pub keyword.


    // 4) Polymorphism
    // In computer science, polymorphism describes the concept of accessing objects of different types through the same interface.
    // For example, in a programming language exhibiting polymorphism,
    // class objects belonging to the same hierarchical tree may have functions with the same name,
    // but with different behaviors.

    // Using traits can be called as polymorphism
}