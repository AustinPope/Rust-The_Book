use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to, Guess the Number!");
    println!("Please enter your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101);


    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to real line");

//        let guess: u32 = guess.trim().parse()
//            .expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You did not enter a number! Try Again.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try Again."),
            Ordering::Greater => println!("Too big! Try Again."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
