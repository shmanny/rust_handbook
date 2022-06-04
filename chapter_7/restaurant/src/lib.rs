// lib.rs here defines the crate root and the contents of this file
// are implicitly scoped to a modules named "crate"

// define a module using the "mod" keyword
// use a semi-colon in front of the mod statement to let rust know that it's pulling
// the module from another file of the same name
mod front_of_house;
pub use crate::front_of_house::hosting;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // to go back to a parent module, we can use the super keyword
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        // structs can be made public but their feilds are private by default
        // we can explicitly make fields public or private with the pub keyword. 
        // here we want toast to be public so the customer can pick the type of bread
        // they want but we want seasonal fruit to be private because only the chef can
        // decide this

        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // this method defines what type of fruit is available for breakfast during
        // the summer. We also need this as a constructor because seasonal fruit is a private field
        // and a public constructor is the only way we would be able to set it.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    // We made the enum here public so all of it's variants are also public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// When pulling structs and enums into scope, the idiomatic way is to specify the full path
use std::collections::HashMap;

// HashMap::new();

// The exception is when we want to use two items with the same name into the same scope
use std::fmt;
use std::io;

// both have a Result item and Rust needs to know which parent module they came from
// fmt::Result
// io::Result

// To avoid collisions, we can also use the "as" keyword to rename an item
// use std::io::Result as IoResult;


// To avoid using large lists, we can use nested paths to clean up large lists 
// we can change:
// use std::cmp::Ordering;
// use std::collections::binary_heap;
// to:
use std::{cmp::Ordering, collections::binary_heap};

// we can also merge paths by taking the following:
// use std::io;
// use std::io::Write;
// and merge it like so:
// use std::io::{self, Write};

// we can also use the glob operator to bring an entire path into scope
// use std::collections::*;

// by using pub in combination with use, external code can call into Breakfast
// this is known as re-exporting. This helps maintain a separate organization structure
// from internal vs external
pub use crate::back_of_house::Breakfast;

// we use the pub keyword to define the public api of the library crate
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // we can use the hosting namespace as well
    hosting::add_to_waitlist();


    // Order breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about type of bread we would like
    meal.toast = String::from("Wheat");
    println!("Id like {} toast please!", meal.toast);

    // the following line wont work because seasonal fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
