fn main() {
    println!("Hello, world!");

    // A package can contain many crates and is created by running the "cargo new" command
    // A crate is a binary or a library and the crate root is the source file that the compiler 
    // starts from and makes up the root module of your crate. The Cargo.toml file does not contain
    // any information about src/main.rs because the compiler knows this is the source file for the
    // binary crate of the same name. A package can contain at most one library crate and as many
    // binary crates as you want. 

    // A crate will group related functionality together so that it's functionality is easy to share 
    // across multiple projects. The crate's functionality is scoped to it's own namespace which makes
    // it easy to pull into our own project without worrying about running into naming collisions 
}
