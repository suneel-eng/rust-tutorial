fn main() {
    // =============== Traits ============
    // A trait defines functionality a particular type has and can share with other types.
    // Traits are similar to a feature often called interfaces in other languages, although with some differences.
    #[derive(Debug)]
    struct Fruit {
        name: String,
        color: String,
        price: u32
    }

    impl Fruit {
        fn print_fruit_name(&self) {
            println!("Fruit name is {}", self.name);
        }

        fn print_fruit_color(&self) {
            println!("Fruit color is {}", self.color);
        }

        fn print_fruit_price(&self) {
            println!("Fruit price is {}", self.price);
        }
    }

    trait Summary {
        fn summarize(&self);
    }

    impl Summary for Fruit {
        fn summarize(&self) {
            println!("Price of {} of color {} is {}", self.name, self.color, self.price);
        }
    }

    let apple = Fruit {
        name: String::from("Apple"),
        color: String::from("Red"),
        price: 100
    };

    apple.print_fruit_name();
    apple.print_fruit_color();
    apple.print_fruit_price();

    println!("{:#?}", apple);

    apple.summarize();
}