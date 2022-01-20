use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // gen_range(1..=100) is the same as above

    //println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

    // we are SHADOWING the previous value of guess, which allows us to use the same variable name -- used often to convert one type to another type
        let guess: u32 = match guess.trim().parse() {
        // .trim() removes whitespace from the beginning and end of the string
        // .parse() method on string converts string into a number
        // the : after guess is a type annotation, which is a way to tell Rust what type we are expecting
        // u32 is a 32-bit unsigned integer
            Ok(num) => num,
            // if the parse method returns an Ok(num) value, we assign num to guess
            Err(_) => continue,
            // if the parse method returns an Err(_) value, we continue the loop and ask for a new guess
            // _ is a catchall value
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
                // we exit the loop when we find a match
            }
        }
    }
}
