use std::io;
use std::cmp::Ordering;
use rand::Rng; 

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
   
    loop {
        println!("Please input your guess.");
        // Create a mutable string
        let mut guess = String::new();
        
        // Take in user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Faled to read line");
        
        // Try converting string to u32. Shadowing allows us to reuse
        // variable name
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // Print user's guess
        println!("You guessed: {}", guess);
        
        // Use match to handle all the possible cases that result from
        // output of .cmp 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
