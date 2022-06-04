fn main() {
    // In the Cargo.toml file, there is a profile section that allows you to set "panic = 'abort'". 
    // This prevents the program from unwinding up the stack when the panic macro is executed to clean
    // up the memory. This will create a smaller binary that may be ideal for production releases. 

    // panic!("Crash and burn!");

    // We can use the backtrace of a panic message to figure out exactly where the panic macro was called 

    // An example of a panic being called in an external library:

    // Rust will call an index out of bounds error in this example unlike C which would allow you 
    // to access whatever is at that address in memory. This 
    let v = vec![1, 2, 3];
    v[99];

    // Run your program with RUST_BACKTRACE=1 in order to get the full back trace of a panic
}
