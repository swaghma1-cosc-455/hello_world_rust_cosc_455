fn main() {
    let mut value = 0b1111_0101u8; 
    println!("value is {}", value); 
    println!("value is {:08b}", value); 

    //Using the NOT operator
    value = !value; 
    println!("value is {:08b}", value); 

    //Using the AND operator
    value = value & 0b1111_0111; 
    println!("value is {:08b}", value);
    //Another use of AND operator
    println!("bit 6 is {}", value & 0b0100_0000); 

    value = value | 0b0100_0000; 
    println!("value is {:08b}", value); 

    value = value ^ 0b0101_0101; 
    println!("value is {:08b}", value); 

    value = value << 4; 
    println!("value is {:08b}", value); 

    value = value >> 2; 
    println!("value is {:08b}", value); 
}
