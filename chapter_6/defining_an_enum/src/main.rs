// Enums are defined similar to structs by defining all the types 
// in curly braces. You can also define an associated value for each enum
// variant in parantheses
enum IpAddrKind {
    V4(String),
    V6(String)
}

// Enums can also have different types and amounts associated with each variant 
enum IpAddrKindV2 {
    V4(u8, u8, u8, u8),
    V6(String)
}


// enums can also have methods defined on them
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    // Use a double colon to access enum types
    let version4 = IpAddrKind::V4;
    let version6 = IpAddrKind::V6;

    // call the function with the variants
    route(IpAddrKind::V4(String::from("127.0.0.1")));
    route(IpAddrKind::V6(String::from("::1")));

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));


    let home = IpAddrKindV2::V4(127, 0, 0, 1);
    let loopback = IpAddrKindV2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("some string");

    let absent_number: Option<i32> = None;


    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // The following wont work because the compiler won't let you add a definite value
    // to an optional value
    // let sum = x + y;
}


// define a function 
fn route(ip_kind: IpAddrKind) {}
