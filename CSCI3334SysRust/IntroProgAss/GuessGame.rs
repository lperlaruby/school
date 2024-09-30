use std::io;

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Return 0 if the guess is correct
    } else if guess > secret {
        1 // Return 1 if the guess is too high
    } else {
        -1 // Return -1 if the guess is too low
    }
}

fn main() {
    let secret = 7; // Hard-coded secret number
    let mut attempts = 0; // Variable to count the number of guesses

    // Start an infinite loop to repeatedly guess the number
    loop {
        let mut input = String::new(); // Create a new string to store the input
        println!("Enter your guess: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input as an integer
        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue; // Skip this iteration and ask for input again
            }
        };

        attempts += 1; // Increment the guess count

        // Call check_guess to evaluate the guess
        match check_guess(guess, secret) {
            0 => {
                // If the guess is correct, print a congratulatory message and exit the loop
                println!("Congrats! You guessed the number correctly.");
                break; // Exit the loop
            },
            1 => {
                // If the guess is too high, print too high
                println!("Your guess is too high.");
            },
            -1 => {
                // If the guess is too low, print a corresponding message
                println!("Your guess is too low.");
            },
            _ => {
                // Handle unexpected cases
                println!("Unexpected value.");
            }
        }
    }

    // After the loop ends print the number of attempts it took
    println!("It took you {} attempts to guess the number.", attempts);
}


