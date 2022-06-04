fn main() {
    // Vectors are a collection that stores data next to each other on the heap 

    // to create vectors we can use the following notation
    // the data type will be an in angled brackets and we can use the constructor
    let v: Vec<i32> = Vec::new();

    // The compiler can infer the type if we prepopulate the vector with values
    // its understood that the type here is i32 because that's the default int type
    let v2 = vec![1, 2, 3, 4, 5];

    // to make a vector mutable we use the "mut" keyword as before
    let mut v3 = Vec::new();
    v3.push(6);
    v3.push(7);
    v3.push(8);
    v3.push(9);

    // vectors get dropped when they go out of scope along with all the elements contained
    // in them

    // Referencing elements in a vector can be done via indexing or using the get method
    let third: &i32 = &v2[2];
    println!("The third element of the vector is {}", third);

    // using the get method lets us return a reference to the desired element as an optional
    // which gives us the ability to use match and handle the case where there is no element
    // appropriately

    match v.get(2) {
        Some(third) => println!("The third element of the vector is {}", third),
        None => println!("There is no third element in the vector"),
    }

    // accessing an element out of range using the [] and & method will cause the program to panic:
    let v4 = vec![1, 2, 3, 4, 5];
    // this will cause a panic which you use when you want to crash the program if it tries to read an element
    // out of range
    // let does_not_exist = &v4[100];

    // To handle out of range exception gracefully use the match syntax
    match v4.get(100) {
        Some(&element) => println!("This will never print {}", element),
        None => println!("Element doesn't exist so this will print")
    }

    // following the rules of references and borrowing, you can't have mutable and immutable references 
    // within the same scope

    let mut v5 = vec![1, 2, 3, 4, 5, 6];
    let first = &v5[0];
    v5.push(6);

    // This wont work because ownership is passed off and you can no longer use the "first" reference
    // println!("The first element of the vector is {}", first);

    // It looks like the above should work but it wont because all the elements in a vector are kept next to each
    // other and adding a new element to the end may cause the vector to run out of room. In this situation,
    // the entire vector would have to get deallocated and then reallocated somewhere else on the heap. 
    // This could invalidate the initial reference to the first element.

    // We can use a for in loop to iterate over a vector making immutable references to the elements inside
    let v6 = vec![100, 38, 49];
    for i in &v6 {
        println!("The current element is {}", i);
    }

    // We can use the mut keyword to iterate over a mutable reference to a vector. We use the dereference modifier (*)
    // to get the value of i before we can use the += to modify it
    let mut v7 = vec![100, 28, 39];
    for i in &mut v7 {
        *i += 50;
    }


    // Sometimes we need vectors that hold multiple types. In these cases, we can use an enum. Consider an example using
    // a spreadsheet cell:

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("hello"))
    ];

    // since all three types are variants of the spreadsheet cell enum, they can all go in the same vector

}
