// Associated functions

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let rect1 = Rectangle::square(40);

    if rect1.is_square() {
        println!("rect1 is a square")
    }
}
