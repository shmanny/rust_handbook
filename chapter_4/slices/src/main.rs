fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    let s = String::from("hello world");

    println!("The first word is {}", word);

    let slice = &s[0..5];   // <-- The slice syntax references a part of a string with the beginning index
                            // leading to one past the last index of the item you want in the slice

    let slice2 = &s[6..11];

    println!("The first word is {} and the second word is {}", slice, slice2);

    let my_string = String::from("hello world");

    let my_string_literal = "hello world";

    // first word works on slices of strings whether they're partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    // first_word also works on string literals since they are technically slices
    // already
    let word = first_word(my_string_literal);


}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item)in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}