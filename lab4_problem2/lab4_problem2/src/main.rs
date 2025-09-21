

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    /*To find the area of a rectangle, we need to find product of height 
    and width, so we return their product*/
    fn area(&self) -> f64 {
        self.height * self.width
    }

    /*Perimeter of a rectangle is sum of all sides
    and given by 2 * (length + width). Hence the function returns 
    that value from the height and width*/
    fn perimeter(&self) -> f64 {
        2.0 * (self.height + self.width)
    }

    /*A rectangle is a square if all its sides are equal, i.e, length and width are equal.
    Hence, we just return the boolean value of whether the height and width are equal or not */
    fn is_square(&self) -> bool {
        self.height == self.width
    }
}

fn main() {
    let rect = Rectangle::new(10.0, 5.0);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square? {}", rect.is_square());

    assert!(Rectangle::new(5.0, 5.0).is_square());
    assert!(!Rectangle::new(5.0, 6.0).is_square());
}
