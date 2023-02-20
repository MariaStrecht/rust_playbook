use std::io; // import input library
use rand::Rng;
use std::cmp::Ordering;

// define main function
fn main() {
    println!("Guess the number!");

    // one that is local to the current thread of execution and is seeded by the operating system
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // create variable to store string
        // let -> create new variable
        // mut -> mutable variable
        // in rust variables are imutable by default
        let mut guess = String::new(); // create a mutable variable, bounded to an empty instance of a String

        // read input and store it on variable guess
        // pass variable through reference
        // references are also imutable by default, so is needed to use '&mut'
        // read_line - returns an Result variable that can be "Ok"|"Err"
        // if Result is an Err value, "expect" will cause the program to crash and display the message, msg
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim method on a String instance will eliminate any whitespace at the beginning and end
        // - The trim method eliminates \n or \r\n
        // The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number. We need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type.

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
