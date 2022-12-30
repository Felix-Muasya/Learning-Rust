
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
 
impl Rectangle { // implimenting the rectangle method
    fn area(&self) -> u32 { // borrowing the self instance
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2*self.width + 2*self.height
    }
}


impl Rectangle {

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}




fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("The perimeter of a rectangle is {} pixels", 
        rect1.perimeter());


    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 55,
    };

    println!("Can rect3 hold rect1? {}", rect3.can_hold(&rect1));
    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));

    let sq = Rectangle::square(3);
}


