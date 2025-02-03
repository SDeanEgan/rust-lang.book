use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    /* guessing_game picks a number and has the user guess it in the 
    command line */
    
    let intro = "I'm a guessing game! I pick a number between 1 and \
100, and have you guess it. If you want to give up, just say \"quit\".\n";
    
    println!("{}", intro);
    println!("Guess my number!");
    
    let quit: String = String::from("quit");
    
    let secret = rand::thread_rng().gen_range(1..=100);
    
    loop {
        
        println!("Please input your guess: ");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Include a check for quitting the game
        if guess.trim().to_lowercase().eq(&quit) {
            println!("Bye for now!");
            break;
        }
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed {}", guess);
        
        match guess.cmp(&secret) {
            Ordering::Less => println!("That's too small."),
            Ordering::Greater => println!("That's too big."),
            Ordering::Equal => {
                println!("That's right! You win.");
                break;
            }
        }
    }
}
