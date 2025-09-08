fn main() {
    let celsius_temp = 23.0; 
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp); 

    assert_eq!(fahrenheit_temp, 73.4); 
    println!("Test passed!"); 
}
/*My Code */

fn celsius_to_fahrenheit(celsius_temp: f64) -> f64 {
    return 1.8 * celsius_temp + 32.0; 
}
