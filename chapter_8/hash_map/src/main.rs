use std::collections::HashMap;

fn main() {
    // create a new hash map using the new method
    let mut scores = HashMap::new();

    // insert some values. Hash maps must be homogenous in that the keys must all
    // be of the same type and the values must all be of the same type
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Green"), 30);

    // Another way to construct a hash map is by using the collect method with an 
    // iterator of tuples

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // We can use underscores when defining the type of hash map because the rust compiler will
    // infer the types and set them appropriately
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();


    // For values that implement the Copy trait, they will be copied into the hash map but values that 
    // are owned will be moved 

    let first_string = String::from("foo");
    let second_string = String::from("bar");

    let mut new_map = HashMap::new();

    // first_string and second_string are now invalid as they have been moved to 
    // new_map
    new_map.insert(first_string, second_string);

    // if we insert references to values into a hash map, the values themselves will not
    // be moved into the hash map. The values that the references point to must be valid for 
    // at least as long as the hash map is valid. 

    // To access a value from a hash map we provide a key to the get method
    // the get method will return an Option that may or may not have the value wrapped
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // We can iterate over key value pairs in a hash map like so 
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // If we insert a new value with a key that's already been inserted, then it will
    // overwrite the previous value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Blue"), 25);

    // this will print {Blue: 25}
    println!("{:?}", scores);


    // We can use the entry method with the or_insert method to insert a value if it doesn't exist
    let mut scores = HashMap::new();
    // since the key with yellow does not exist, or_insert will insert 32
    scores.entry("Yellow").or_insert(32);

    // we can also update a hash map's value based on the value that's already there
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {

        // or_insert returns a mutable reference to the value for the key in the hash map
        // in order to assign to that value, we must first dereference count. The mutable reference
        // goes out of scope at the end of the for loop so all of these changes are safe and allowed
        // by the borrowing rules
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // Hash maps use use a hashing function called "SipHash" by default that can provide resistance to Denial of Service(DoS) 
    // attacks involving hash tables. This is not the fastest hashing algorithm available but the trade off of better security
    // in exchange for better performance is worth it. If you need a faster hash function, then you can specify a different 
    // hasher. A hasher is a type that implements the BuildHasher trait.
}
