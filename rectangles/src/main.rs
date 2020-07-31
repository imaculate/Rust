#[derive(Debug)]
struct Rectangle
{
    height: u32,
    width: u32
}

// Multiple impl blocks are allowed
impl Rectangle
{
    fn area(&self) -> u32
    {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.height > other.height && self.width > other.width
    }

    fn square(size: u32) -> Rectangle
    {
        Rectangle{
            height: size,
            width: size
        }
    }
}


fn main() {
    let rect1 = Rectangle {
        height: 50,
        width: 30
    };
    println!("Rect1 is {:#?}", rect1);

    let rect2 = Rectangle::square(40);
    println!("Rect2 is {:#?}", rect2);

    println!("The area of rect1 is {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
