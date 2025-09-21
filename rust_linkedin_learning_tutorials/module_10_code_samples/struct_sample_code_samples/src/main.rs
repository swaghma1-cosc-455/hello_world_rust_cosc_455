#[derive(Debug)]
#[derive(Clone)]

struct Shuttle{
    name: String, 
    crew_size: u8, 
    propellant: f64, 
}
fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"), 
        crew_size: 7, 
        propellant: 835958.0
    }; 

    let vehicle2 = Shuttle {
        ..vehicle.clone()
    };

    vehicle.crew_size = 6; 
    println!("Vehicle is {:?}", vehicle);

    println!("Vehicle2 is {:?}", vehicle2);

}
