
//Define a struct to represent a Rectangle
//width f64
//height f64
//Methods get_area() return width * height
//scale() scale weight and height by an input f64 value
//Associated - new(), accepts width and height

struct Rectangle{
    height: f64, 
    width: f64,
}


impl Rectangle{
    fn get_area(&self) -> f64 {
        return &self.width * &self.height; 
    }

    fn scale(&mut self, value: f64)
    {
        self.width *= value; 
        self.height *= value; 
    }

    fn new(height: f64, width: f64) -> Rectangle {
        return Rectangle {height: height, width: width}; 
    }
}


fn main() {
    let mut rect = Rectangle::new(1.2, 3.4); 
    assert_eq!(rect.get_area(), 4.08); 
    rect.scale(0.5); 
    assert_eq!(rect.get_area(), 1.02); 
    println!("Tests passed!"); 
}
