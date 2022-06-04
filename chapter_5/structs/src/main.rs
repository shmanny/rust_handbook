
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    // instantiating a struct works as follows; set values for all keys
    let user1 = build_user(String::from("hello@hello.com"), String::from("amansaran"));

    // access data using the dot notation
    let user1_email = user1.email;
    println!("User 1's email is {}", user1_email);
    // make structs mutable to change them with dot notation
    let mut user2 = User {
        email: String::from("hello@hello.com"),
        username: String::from("aman12345"),
        sign_in_count: 1,
        active: true,
    };

    user2.email = String::from("helo1234@hello.com");
    let user2_email = user2.email;
    println!("User 2's email is {}", user2_email);

    // theres an update syntax to copy a struct over while changing an attribute
    // similar to javascript spread operator. When the update syntax is used, the underlying data
    // from username is moved from user2 to user3. This makes user2 inactive and unusable. 
    // The active and sign_in_count fields are implement the copy trait so they're copied over. 
    let user3 = User {
        email: String::from("a+different+email@ghello.com"),
        ..user2
    };

    // Tuple structs can be used to name tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    // structs without any fields 
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
