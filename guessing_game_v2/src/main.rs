use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!\n");

    let range = 1..=100;
    let secret_number = rand::rng().random_range(range.clone());

    let max_attempts: u32 = 10;
    let mut attempts: u32 = 0;

    println!(
        "The range of Guess its between {} and {}\n",
        &range.start(),
        &range.end()
    );

    loop {
        if attempts >= max_attempts {
            println!("You lose, max number of attempts reached.");
            break
        }; 

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Remember that the guess must be a positive number!");
                continue;
            }
        };

        println!("You guessed: {guess}\n");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        
        attempts += 1;
        println!("Attempts Remaining {}.", max_attempts - attempts);
    }
}
