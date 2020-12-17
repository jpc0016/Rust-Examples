// First user-defined function pgs. 43-44

// function syntax convention uses 'snake case'.  Use underscores between words.
// define parameters using colon notation: `variable: type`
// parameter definitions are separated by commas
fn another_function(x: i32, y: i32){
    //println!("I called another_function()!");
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn main() {
    println!("I am in main() function!");

    another_function(5, 9);
}
