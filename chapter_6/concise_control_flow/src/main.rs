fn main() {
    let config_max = Some(3u8);
    // if-let syntax lets us pattern match with a single variant
    if let Some(max) = config_max {
        println!("The max is configured to be {}", max);
    }
}
