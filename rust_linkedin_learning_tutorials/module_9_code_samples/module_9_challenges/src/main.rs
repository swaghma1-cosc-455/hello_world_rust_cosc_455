use std::env; 
use std::fs; 

fn main() {
    if env::args().len() <= 2{
        println!("Program requires atleast 2 arguments. "); 
        return; 
    }
    //Write a program to check if a specific person exists in a list of names
    //Print a message indicating whether or not their name was found
    //Argument 1 - Path to the text file with list of names
    //Argument 2 - Name to search
    //Example input cargo run roster.txt Stone

    let file_name = env::args().nth(1).unwrap(); 
    println!("File is {}", file_name); 

    let name_to_search = env::args().nth(2).unwrap(); 
    println!("Name is {}", name_to_search); 

    let contents = fs::read_to_string(file_name).unwrap();

    for line in contents.lines()
    {
        if line == name_to_search
        {
            println!("Found! Name exists."); 
            return; 
        }
    }

    println!("Not Found! Name does not exist."); 


}
