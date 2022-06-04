struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // first argument always is &self for methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // subsequent arguments after self can be whatever
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // to create a "static" method we can write a fn without &self
    // these are often used as constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
 }

 // you can have multiple impl blocks
 impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
 }

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("The area of the rectangle is {} square pixels", rect1.area());

    if rect1.width() {
        println!("the rectangle has a nonzero width; the width is {}", rect1.width)
    }

    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));


    // Create a square 
    let square = Rectangle::square(3);
    println!("The variable square is a square because its width is {} and its height is {}", square.width, square.height)
}
