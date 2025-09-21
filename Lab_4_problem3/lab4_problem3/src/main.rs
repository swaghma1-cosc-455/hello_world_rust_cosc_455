struct Circle{
    radius: f64, 
}

//Defining a new implementation for Circle 
impl Circle {
    //Function to declare a new constructor for Circle from name
    fn new(radius: f64) -> Circle {
        Circle {radius}
    }

    //Function to return area of circle given PI * radius * radius, using 3.14 instead of PI
    fn area(&self) -> f64{
        3.14 * self.radius * self.radius
    }

    //Function to return circumference of a circle given by 2 * pi * radius, pi given by 3.14
    fn circumference(&self) -> f64{
        2.0 * 3.14 * self.radius
    }
}
fn main() {
    //Driver code to test the functions implemeted for struct Circle 

    //Initialize a new circle
    let circle = Circle::new(7.0);

    //Find the area of the circle
    println!("Area: {}", circle.area());

    //Find the circumference of the circle
    println!("Circumference: {}", circle.circumference());

}
