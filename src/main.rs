use rand::Rng;
use std::io::{self, Write};

fn main() {
    // Settings (adjustable)
    let mut attempts = 5;
    let max_range = 20;

    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(1..=max_range); // Generate a random number between 1 and 50

    while attempts > 0 {
        // Prompt the user for input
        print!("Enter a number: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        // Read the input and handle potential errors
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input! Please try again.");
            continue; // Skip to the next iteration if there's an error reading the input
        }

        // Parse the input and handle invalid input
        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                continue; // Skip to the next iteration if the input isn't a valid number
            }
        };

        // Check if the guess is correct
        if guess == rand_num {
            println!("Congratulations! You guessed the number correctly!");
            return; // Exit immediately after a win
        }

        // Decrease attempts
        attempts -= 1;

        // Print feedback on the number of attempts remaining
        if attempts == 0 {
            println!(
                "Sorry, you didn't win this time. The correct number was: {}",
                rand_num
            );
            println!("Better luck next time!");
            return; // Exit immediately after a loss
        }

        // Give feedback based on the difference
        let difference = (rand_num - guess).abs();
        if difference <= (rand_num as f32 * 0.2) as i32 {
            println!("You're *really* hot! Almost there!");
        } else if difference <= (rand_num as f32 * 0.6) as i32 {
            println!("You're warm! Getting closer!");
        } else if difference <= (rand_num as f32 * 0.8) as i32 {
            println!("You're cold. Try a bit further!");
        } else {
            println!("You're way off! Try a big shift!");
        }
    }
}
