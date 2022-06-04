fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);    // by passing a reference to s1 to calculate_length
                                        // we stopped s1 from being moved to calculate_length

    println!("The length of {} is {}", s1, len);

    let mut s = String::from("hello");

    // Example of a mutable reference:
    change(&mut s);

    let mut s2 = String::from("mutable reference");
    // The code below will fail because you can only have one mutable reference at a time
    // let r1 = &mut s2;
    // let r2 = &mut s2;

    // You can have multiple mutable references by creating a new scope: 
    {
        let r1 = &mut s2;
    } // r1 goes out of scope so it can be referenced again
    
    let r2 = &mut s2;

    // Mixing mutable and immutable references doesn't work either
    // let s3 = String::from("another mutable reference");
    // let r1 = &s3; <-- no problem
    // let r2 = &s3; <-- no problem
    // let r3 = &mut s3 <-- mutable borrow error occurs

    // If the scope of an immutable reference ends, then a mutable reference can be created:
    let mut s3 = String::from("another mutable reference");
    let r1 = &s3; 
    let r2 = &s3; // <-- Two immutable references can be made

    println!("printing references {} and {}", r1, r2);

    let r3 = &mut s3; // <-- This is allowed because r1 and r2 are now out of scope after println


    // This code won't compile because it references data that is no longer available
    // The string s in the function goes out of scope so the reference to it is invalid 
    // let dangling_reference = dangle();



}

fn calculate_length(s: &String) -> usize {
    s.len()
}   // s goes out of scope but because its a reference and does not have ownership over what it
    // refers to then nothing happens


// This function will not work because some_string is being borrowed and therefore cannot be modified
    // fn change(some_string: &String) {
//     s.push_str(", world")
// }


// By modifying the function signature to include the mut keyword, we can create a mutable
// reference to the string which allows us to modify it.
fn change(some_string: &mut String) {
    some_string.push_str(", world")
}


// fn dangle() -> &String {
//     let s = String::from("hello world"); 
//     &s
// }

// The solution is to return the string directly
fn no_dangle() -> String {
    let s = String::from("hello world");
    s
}