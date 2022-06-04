#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    
    println!(
        "The area of the rectangle is {} square pixels",
        // here we're borrowing the struct so that main retains ownership of it
        area(&rect1)
    );
    
    println!("rect1 is {:#?}", rect1);
    dbg!(rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} 
