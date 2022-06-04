fn main() {
    // Strings are a data structure we can use to load data into 
    // Rustaceans will refer to string literals or mutable strings that data can
    // be loaded into interchangeably.

    // The Rust standard libary also has OsString, OsStr, CString and CStr
    // the variations that end in str referred to borrowed types whereas the
    // the ones that end in String are owned

    let mut s = String::new();
    
    // we can start with some initial data
    let data = "initial data";
    let s = data.to_string();

    // we can also call the method on string literal notation
    let s = "initial data".to_string();

    let s = String::from("initial contents");

    // Strings are UTF-8 encoded so we can include any character that's UTF-8 encoded:
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");


    // we can add to a string using the push_str method
    let mut s = String::from("bar");
    s.push_str("bar");

    // push_str takes a string slice because we don't want ownership of the parameter
    // here we want the ability to use s2 after we append the contents to s1 
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    // if push_str took ownership of s2, then we would not be able to print it here
    println!("s2 is {}", s2);

    // the push method takes a single character and appends it to a string
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // s1 cannot be used because it's been moved to s3 
    // s2 has an & because we're adding a reference to s2 to s1 
    let s3 = s1 + &s2;

    // Adding multiple strings together can become hard to read. 
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // we can use the format macro for more complicated scenarios
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = String::from("hello");
}
