use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I've picked a number between 0 and 9. Can you guess it?");

    // Generates a random number between 0 and 9
    let mut rng = rand::thread_rng(); // mut keyword make variable mutable
    let num_to_guess: u8 = rng.gen_range(0..10);

    println!("Please enter your guess:");

    // Createing a mutable variable to store the user's input
    let mut user_input = String::new();

    // Reading the user's input from the console
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    // Converting the user's input to a number
    let user_guess: u8 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            // Incase where the input is not a valid number
            println!("Please enter a valid number!");
            return;
        }
    };

    // Checking if the user's guess matches the generated number
    if num_to_guess == user_guess {
        println!("Congratulations! You guessed it: {}", user_guess);
    } else {
        println!(
            "Oops! You didn't guess correctly. The number was: {}",
            num_to_guess
        );
    }
}
