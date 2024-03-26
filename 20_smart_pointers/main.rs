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
    // #[derive(Debug)]
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil
    // }

    // let list = List::Cons(1,
    //     Box::new(List::Cons(2,
    //         Box::new(List::Cons(3,
    //             Box::new(List::Nil)
    //         ))
    //     ))
    // );

    // println!("{:?}", list);

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

    // Running code on clean up with the Drop trait
    // The second trait important to the smart pointer pattern is Drop,
    // which lets you customize what happens when a value is about to go out of scope.
    // Drop trait will work for any type.
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("MyBox cleaned up")
        }
    }

    // Rc<T>, The reference counted smart pointer

    // There are cases when a single value might have multiple owners.
    // You have to enable multiple ownership explicitly by using the Rust type Rc<T>,
    // which is an abbreviation for reference counting.

    // The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use.
    // If there are zero references to a value, the value can be cleaned up without any references becoming invalid.
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }
    
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a)); // a moved
    // let c = Cons(4, Box::new(a)); // can't access a here

    // We could change the definition of Cons to hold references instead,
    // but then we would have to specify lifetime parameters.
    // By specifying lifetime parameters,
    // we would be specifying that every element in the list will live at least as long as the entire list.

    use std::rc::Rc;

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("Reference count: {}", Rc::strong_count(&a));
    let _b = List::Cons(3, Rc::clone(&a));
    println!("Reference count: {}", Rc::strong_count(&a));
    {
        let _c = List::Cons(4, Rc::clone(&a));
        println!("Reference count: {}", Rc::strong_count(&a));
    }

    println!("Reference count: {}", Rc::strong_count(&a));

}