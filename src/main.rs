use std::cmp::Ordering;
use std::io;

use rand::Rng;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut attempts: i32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("error: Please enter a number");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                attempts += 1;
                println!("Too small!");
            }
            Ordering::Greater => {
                attempts += 1;
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win! This game took you {} attempts", attempts);
                break;
            }
        }
    }
}
