
// Constants are globally declared 
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // 3.1 Variables and mutability

    // use mut to make var mutable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing lets you reuse a variable name;
    // Shadowing is different from mut because because the let 
    // keyword lets you perform transformations on the variable while keeping
    // the variable immutable 
    let x = 5;
    
    let x = x + 1;

    // Using inner curly braces to keep separate inner scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
        // The value of x in the inner scope is: 12
    }

    println!("The value of x is {}", x);
    // The value of x is 6

    // Shadowing lets you change a variable type 
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);


    // 3.2 Data Types

    // Scalar Types

    // integer: can be signed or unsigned up to 128 bits
    let int: u32 = 32;
    // float: f64 is the default but can be 32 or 64 
    let float: f64 = 3.0;
    // bool: self explanatory
    let boolean: bool = true;
    // char: 4 bytes
    let c: char = 'c';


    // Compound Types

    // tuple: can group different types together. can't grow in size
    let tup: (i32, f64, u8) = (-500, 5.0, 3);
    // use destructuring to pull values off of a tuple
    let (x, y, z) = tup;
    // you can also use indices 
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;


    // array: every element has same type and arrays are of fixed length
    // arrays are allocated on the stack and not the heap in rust
    // write arrays as literals
    let array = ["foo", "bar"];
    // declare type and length as so
    let array: [u32; 5] = [1, 2, 3, 4, 5];
    // or create an array with all the same elements
    let array = [3; 5];

    // Accessing an index thats out of bounds that is provided as an input will
    // result in an index out of bounds error

    // 3.3 Functions
}
