fn main() {
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;
        // breaks will only break the inner most loop
        // this inner loop twice until count increments to 2
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);

    let mut counter = 0;

    // results of loops can be returned by adding the value you want
    // after the break expression
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result of the expression is: {}", result);

    let mut counter = 3;

    while counter != 0 {
        println!("The counter is now {}", counter);
        counter -= 1;
    }

    println!("The counter is now {}", counter);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // can possibly panic if index is out of bounds
    while index < 5 {
        println!("The value of the array at index {} is {}", index, a[index]);
        index += 1;
    }

    // better to use for in
    for element in a {
        println!("The value is {}", element);
    }

    // for loops are preferred in rust because they're safer
    // even for writing a count down like done above
    for number in (1..4).rev() {
        println!("{}...", number);
    }

    println!("LIFTOFF!");
}

