// if-statement example pg. 52
fn main() {
    let condition = true;
    // This statement evaluates to "if true, return 5; number = 5"
    // if condition = false, number would be 6!
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is {}", number);
}
