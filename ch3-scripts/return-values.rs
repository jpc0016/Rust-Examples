// returning values from a loop pg. 55
fn main() {
    // initialize counter to 0.  It will change so it must be mutable
    let mut counter = 0;

    let result = loop {
        // add 1 to counter at start of iteration
        counter += 1;

        // check if counter equals 10
        if counter == 10 {
            // return 10*2 (20) if counter equals 10
            break counter * 2;
        }
    }; // this is an assignment statement so it needs a semicolon

    // prints "The result is 20"
    println!("The result is {}", result);
}
