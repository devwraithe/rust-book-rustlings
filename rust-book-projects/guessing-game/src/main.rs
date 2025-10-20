use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate sec_num a range between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

        // Mutable so it can change later
        let mut guess = String::new();

        // Attach CLI input to guess variable (mut comes in handy)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Use .trim() to remove whitespaces before parsing
        // `match` handles non-u32 inputs by prompting inputs (loop comes in handy)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // guess shadow

        println!("You guessed: {guess}");

        // Match guess to sec_num with conditions, break loop if match is equal
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
