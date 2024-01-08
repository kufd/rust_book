fn main() {
    println!("1.0C is {}F", fahrenheit_to_celsius(1.0));
    println!();
    println!("20th fibonacci number is {}", get_fibonacci_number(20));
    println!();
    the_twelve_days_of_christmas();
}

fn the_twelve_days_of_christmas() {
    const DAYS_NUMBER: usize = 12;

    let presents = [
        "Twelve drummers drumming", "Eleven pipers piping", "Ten lords a-leaping", "Nine ladies dancing", "Eight maids a-milking", "Seven swans a-swimming",
        "Six geese a-laying", "Five golden rings", "Four calling birds", "Three French hens", "Two turtle doves", "And a partridge in a pear tree."
    ];

    let day_number_string = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let mut day_number = 0;

    while day_number < DAYS_NUMBER {
        day_number += 1;

        println!("On the {} day of Christmas", day_number_string[day_number - 1]);
        println!("My true love gave to me");

        if day_number == 1 {
            println!("A partridge in a pear tree.");
        } else {
            let mut present_number = DAYS_NUMBER - day_number;
            while present_number < DAYS_NUMBER {
                println!("{}", presents[present_number]);

                present_number += 1;
            }
        }

        println!();
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn get_fibonacci_number(position: u32) -> u64 {
    if position < 3 {
        return 1;
    }

    get_fibonacci_number(position - 2) + get_fibonacci_number(position - 1)
}
