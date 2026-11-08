struct Rectangle {
    width: u32,
    breadth: u32,
}

impl Rectangle {
    //Constructor
    fn new(width: u32, breadth: u32) -> Self {
        Self {width, breadth}
    }

    //Area
    fn area(&self) -> u32 {
        self.width * self.breadth
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.breadth *= factor;
    }
}

fn main() {
    let mut rect = Rectangle::new(21, 23);
    println!("Area of Heart is {}", rect.area());

    rect.scale(2);
    println!("Area after scaling is {}", rect.area())
}