// passing by reference example pg. 70

fn calculate_length(s: &String) -> usize {
    s.len(); // This value is returned so no assignment is needed
}

fn main() {
    // initialize string to "hello"
    let s1 = String::from("hello");

    // pass pointer to s1 string to calculate_length()
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
