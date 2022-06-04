use std::{fmt::{Display, Debug}, cmp::{PartialOrd}};
// Defining a trait lets the compiler know about functionality that can be shared with other types
// We can use traits with generics to define behavior that a generic should have. Traits are similar to
// interfaces in other languages


// Here we declare a trait using the trait keyword and the trait's name which is summary in this case
// We also declare the trait as pub so that other crates depending on it can use it.

// We can import the Summary trait in other crates so they can implement it on their own types. The only caveat is that
// either the trait or the type need to be local to the crate that's using it. i.e. We can implement the Display trait on
// the Tweet type because Tweet is local but we can't implement Display on Vec<T> because neither of those are local.


pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;
}

pub trait SummaryV2 {
    // Here we specify default behavior for the summarize method by specifying a string to return. This is useful
    // for when you don't want to have to implement every method belonging to a trait and then you can override the methods
    // you actually do want to implement

    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}


// Below we implement the Summary trait for both a NewsArticle struct and a Tweet struct
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }

    fn summarize_author(&self) -> String {
        return format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }

    fn summarize_author(&self) -> String {
        return format!("{}", self.username)
    }
}


// We can use traits in parameters when defining a function by doing the following:
pub fn notify(item: &impl Summary) {
    // item must be a type the implements Summary and therefore we can call summarize
    println!("Breaking news! {}", item.summarize());
}

// The longer form version of using trait in parameters is the trait bound syntax which looks
// like the following which is useful for when you have multiple parameters that need the same trait:
pub fn notifyV2<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news! {}", item.summarize());
}

// We can also use the trait bound syntax to specify more than one trait:
pub fn notifyV3<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// using traits with where clause 
pub fn some_function<T, U>(t: &T, u: &U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug
{
    22
}

// define a function that returns a trait 
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: String::from("false"),
        retweet: String::from("false"),
    }
}

// the following wouldn't work because a function can only return a single type
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'c'];
    let largest = largest(&char_list);

    println!("The largest char is {}", largest);
}
