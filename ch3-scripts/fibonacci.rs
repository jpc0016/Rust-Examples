// fibonacci number generation pg. 57
use std::env;

// make function return type u32
fn fibonacci(x: u32) -> u32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    } else {
        return fibonacci(x - 1) + fibonacci(x - 2);
    }
}

fn main() {
    // load input number via argument
    let args: Vec<String> = env::args().collect();
    let num: u32 = args[1].parse().unwrap();

    // call fibonacci function
    let f: u32 = fibonacci(num);
    println!("{}", f);
}

/* Output:

$ ./fibonacci 4
3
$ ./fibonacci 8
21

*/
