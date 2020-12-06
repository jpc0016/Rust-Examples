/* Listing 2-1: Guessing game
*
* page 15
*
*/

// standard input/output library.  io library comes from the std library, hence the '::'
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    // Remove secret_number print statement
    // println!("The secret number is {}.", secret_number);

    loop {
        println!("Please input your guess.");

    /* create place to store user input
    'let' keyword is used to create a variable
    'mut' makes a variable mutable, meaning they are allowed to change
    String::new() creates a new empty string that is assigned to guess
    */
        let mut guess = String::new();

        // .expect() raises a message when an Err type is returned in .read_line()
        io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

        // change `expect()` statement to `match{}` expression to handle errors instead of
        // crashing the program
        let guess: u32 = match guess.trim().parse() {
            // parse() returns Result type which is enum of either 'Ok" or 'Err'
            Ok(num)=>num,
            Err(_)=>continue,

        };
                //.expect("Please type a number!");

                println!("You guessed: {}", guess);

        // Routine to compare contents in `secret_number` with `guess`
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                        // Break out of program when victory condition met
                                println!("You Win!");
                                break;
                                }
        }
    }

    // let x = 5;
    // let y = 10;
    // println!("\nThe numbers are {} and {}", x, y);


}
