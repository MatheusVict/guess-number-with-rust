use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number");
    let secret_number: u32 = rand::thread_rng()
        .gen_range(1..=100);

    user_guess(secret_number);
}

fn user_guess(secret_number: u32) {
    let mut guesses: u32 = 0;
    loop {
        println!("Take a guess: ");

        let mut user_guess: String = String::new();

        io::stdin()
            .read_line(&mut user_guess).expect("invalid argument");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        guesses += 1;
        match user_guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won");
                break;
            }
            Ordering::Greater => println!("It's smaller"),
            Ordering::Less => println!("It's bigger"),
        }
    }

    println!("You used {guesses} tries");
}
