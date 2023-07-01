use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("::::::::GUESS THE SECRET NUMBER, ENTER 1 - 10:::::::");
    
    loop {
        let mut guess = String::new();
        io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
        let guess = guess.trim();
        println!("You guessed: {guess}");
        println!("Did you guess right?");
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You need to guess a number! Your input '{guess}' is not a number!");
                continue;
            }
        };

        //match is like switch statement
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too much sistah!"),
            Ordering::Equal => {
                println!("CORRECT! You win!");
                break;
            },
            Ordering::Less => println!("Too low brobro!")
        } 
    }

    /*  TO SHOW if statement
    if z == secret_number {
        println!("You guessed right!");
    } else {
        println!("You guessed wrong :(");
        main();
    }
    */
}
