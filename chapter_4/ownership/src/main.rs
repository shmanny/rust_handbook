fn main() {
    // string literals are stored on the stack because they're immutable and
    // a known and fixed size 
    let _s = "hello";

    
    // String type is mutable and stored on the heap because it's size is unknown at compile time
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // The concept of ownership does not allow variables to be shallow copied as in other languages
    // for example...
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); 
    // The above will produce an error because once s1 is "moved" to s2
    // This means that s1 is no longer valid and this is done to prevent double freeing bugs from occuring

    println!("s2 = {}", s2);

    // To perform a deep copy we can use the clone method:
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s is moved to the function and is no longer valid

    // if we tried to call s at this point, then we would get compile time errors

    let x = 5; // x comes into scope

    makes_copy(x);  // x moves into makes copy but it's an i32 so it has the Copy trait
                    // which means it's still valid

    let s1 = gives_ownership(); // the return value of gives_ownership is moved to s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved to s3 

    // s1 and s3 are dropped. s2 was already moved so it's invalid.
}

fn takes_ownership(some_string: String) {
    // ownership of some_string is passed here
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}