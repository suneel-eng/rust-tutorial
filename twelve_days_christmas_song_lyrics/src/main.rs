fn main() {
    
    let days: [&str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let gifts: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves And ",
        "Three french hens, ",
        "Four calling birds, ",
        "Five golden rings, ",
        "Six geese a-laying, ",
        "Seven swans a-swimming, ",
        "Eight maids a-milking, ",
        "Nine ladies dancing, ",
        "Ten lords a-leaping, ",
        "Eleven pipers piping, ",
        "Twelve drummers drumming, "
    ];

    
    for (day_index, day) in days.iter().enumerate() {

        let mut day_index_for_gift = day_index;
        let mut day_gifts = String::new();

        'gift_loop: loop {
            day_gifts += gifts[day_index_for_gift];

            if day_index_for_gift == 0 {
                break 'gift_loop;
            }
            day_index_for_gift -= 1;
        }

        println!("On the {day} day of Christmas, My true love sent to me {day_gifts}.");
    }
}
