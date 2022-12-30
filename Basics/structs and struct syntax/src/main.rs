#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,

    };

    dbg!(2*3);

    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 { // borrowing the struct
    rectangle.width * rectangle.height
}
