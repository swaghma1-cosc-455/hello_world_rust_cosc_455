use core::num;

use rand::prelude::*; 

fn main(){
    // let mut buffer = String::new(); 
    // println!("Enter a number: "); 
    // io::stdin().read_line(&mut buffer); 
    // println!("buffer is {}", buffer); 

    // let number: i32 = buffer.trim().parse().unwrap();

    // println!("number+1 is {}", number);  

    let number = random::<f64>(); 
    println!("number is {}", number); 

    let number = thread_rng().gen_range(1..11); 
    println!("number is {}", number); 
}