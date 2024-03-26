fn main() {
    // ========== Smart Pointers ==============

    // ========== Box<T> smar pointer ==========
    // Boxes allow you to store data on the heap rather than the stack.
    // What remains on the stack is the pointer to the heap data.

    let b = Box::new(5); // 5 stored in the heap.

    println!("b = {}", b);

    drop(b); // deallocate b and it's values prior to the end of scope

    // Enabling recursive types with boxes
    
    // A value of recursive type can have another value of the same type as part of itself.
    // Ex: (1, (2, (3, Nil)))

    // Below code throws ERROR: recursive type `List` has infinite size
    // enum List {
    //     Cons(i32, List),
    //     Nil
    // }

    // let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));

    // To store recursive values we can use boxes
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil
    }

    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil)
            ))
        ))
    );

    println!("{:?}", list);

    // Defining our own smart pointer similar to Box<T>
    #[derive(Debug)]
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    println!("x = {}", x);
    println!("y = {:?}", y);
    println!("*y = {:?}", *y);
}