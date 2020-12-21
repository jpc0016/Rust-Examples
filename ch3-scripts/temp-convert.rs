// temp conversion exercise pg. 57
use std::env;

fn main() {
    // Load arguments from command line
    let args: Vec<String> = env::args().collect();

    // Print opening message
    //println!("This is a temperature (Celsius-Fahrenheit) conversion tool");
    //println!("Specify input temperature with flag -C/-F.  Ex: -C 21.8\n\n");

    // initialize
    let num: f32 = args[2].parse().unwrap();

    if args[1] == "-C" || args[1] == "-c" {
        let fahr = (num * (9.0 / 5.0)) + 32.0;
        println!("{}°C = {}°F", args[2], fahr);
    } else if args[1] == "-F" || args[1] == "-f" {
        let cels = (num - 32.0) * (5.0 / 9.0);
        println!("{}°F = {}°C", args[2], cels);
    } else {
        println!(
            "Incorrect argument.  Please pass '-F $Fahrenheit' or '-C Celsius' values
        to be converted"
        );
    }

    // println!("");
    // println!("The first argument is {}", args[1]);
    // println!("The second argument is {}", args[2]);
}
