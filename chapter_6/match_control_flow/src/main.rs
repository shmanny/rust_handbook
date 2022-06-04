#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // use the match keyword to account for all enum variants
    match coin {
        // use curly braces if you have a multiline statement
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from state {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    println!("The value of a  quarter is {} cents", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("The value of a penny is {} cents", value_in_cents(Coin::Penny));

    let five = Some(5);
    println!("Add one to five: {:?}", plus_one(five));

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // use the underscore as a catch all
        _ => ()
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}