use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    let f = File::open("hello.txt");

    // Here we use the Result<T, E> format to handle the cases of failure when trying to open the
    // file hello.txt. The Reuslt enum returns two possible generics depending on the success/failure
    // of the file open operation

    let f = match f {
        // In the case of success, we return the file 
        Ok(file) => file,

        // In the case of failure, we run a match operation against the error type
        Err(error) => match error.kind() {
            // If the file is not found, we create it and return another match operation
            ErrorKind::NotFound => match File::create("hello.text") {
                // If we successfully create the file, then we return the file 
                Ok(fc) => fc,
                // We panic and exit the program if the file fails to create
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // We handle any other error type that may come up when trying to open the file
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    // Same logic using closures instead of nested match statements
    // we use the unwrap_or_else method here to avoid dealing with a big nested match
    // statement

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem creating the file: {:?}", error);
        }
    });


    // We can also use the unwrap method to get a value from Result
    // here is an example of using the unwrap method. It will return the Ok value of result 
    // or it will panic 

    let f = File::open("hello.txt").unwrap();

    // Another option is to use the expect method to pass in an error message that will print 
    // if it panics
    let f = File::open("hello.txt").expect("File failed to open for some reaosn");

    // When a function calls something that might fail, you can return the error to the calling code
    // instead of handling the error in the fucntion itself. This is known as propagating the error. 

    fn read_username_from_text() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        // The initial call to open the file will return a Result enum that will
        // either fail or succeed. In case of failure, the error will bubble up and return to the
        // caller of the function

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        // We create a mutable string variable to read the contents of the file into the string and we 
        // return a result containing either the string or the Error if there is one. 
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }


    // Here is a more condensed way of writing the same code using the ? mark operator 
    // The ? mark operator works the same way as the match operator in that it will return 
    // the success result if the method passes and it will return an error if it fails
    
    // One way that the ? operator is different from a normal failure is that the ? operator 
    // will convert the error to whichever error type is defined in the return of the function.
    // This is useful when a failure returns a generic error type and you want it mapped to a specific 
    // type. 

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // Using the ? operator we can further condense this function by chaining them together

    fn read_username_from_filev2() -> Result<String, io::Error> {
        let mut s = String::new();
        // we chain the ? operator to reudce it down to one line
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }


    // We can shorten even further down to one line but doing the following
    fn read_username_from_filev3() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    fn last_char_of_the_firstline(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}

