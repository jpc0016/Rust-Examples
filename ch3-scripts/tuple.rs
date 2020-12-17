// printing tuple elements pg. 40

fn main() {
    // cannot use `:=` without specifying a type.  Would have to be `let tuple (i32, f64, u8)`
    let tuple = (500, 6.4, 1);
    // assign elements of tuple to respective variables
    let (x, y, z) = tuple;
    // access and print y from above tuple
    println!("The value of y is {}", y);
}
