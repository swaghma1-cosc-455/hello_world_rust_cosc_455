fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37,  20, 56, -18, 20, 3]; 

    let mut max: i32; 
    let mut min: i32; 
    let mut mean: f64; 

    //My code
    max = 0;
    min = numbers[0]; 

    mean = 0.0; 
    for number in numbers.iter()
    {
        if *number > max{
            max = *number; 
        }

        if *number < min {
            min = *number; 
        }

        mean = mean + *number as f64; 
    }

    mean = mean/numbers.len() as f64; 

    assert_eq!(max, 56);
    assert_eq!(min, -18); 
    assert_eq!(mean, 12.5);
    println!("Tests passed!");  
 
}
