// Branches example pgs. 50-52

fn main() {
    let number = 6;

    if number % 4 == 0 {
        //println!("Condition was true!");
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        //println!("Condition was false!");
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }

}

/* Output:

~/Rust-Examples/branches_3$ cargo run
   Compiling branches_3 v0.1.0 (~/Rust-Examples/branches_3)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/branches_3`
Condition was true!

*/
