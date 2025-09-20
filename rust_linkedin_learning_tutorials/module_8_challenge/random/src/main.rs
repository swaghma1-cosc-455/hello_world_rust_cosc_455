use std::io; 
use rand::prelude::*; 
fn main() {

    let number = thread_rng().gen_range(1..100); 

    let mut user_input = String::new(); 

    println!("Guess a number between 1 and 100: "); 
    io::stdin().read_line(&mut user_input); 

    
    let mut user_number: i32 = user_input.trim().parse().unwrap(); 

    while user_number != number{
        if user_number > number{
            println!("Too high! Try again"); 
        }
        else {
            println!("Too low! Try again"); 
        }

        user_input.clear(); 
        io::stdin().read_line(&mut user_input);
        user_number = user_input.trim().parse().unwrap(); 
    }
        println!("That's it! You got it. "); 
   
}
