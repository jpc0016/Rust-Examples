/* Listing 2-1: Guessing game
*
* page 15
*
*/

// standard input/output library.  io library comes from the std library, hence the '::'
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /* create place to store user input
    'let' keyword is used to create a variable
    'mut' makes a variable mutable, meaning they are allowed to change
    String::new() creates a new empty string that is assigned to guess
    */
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
