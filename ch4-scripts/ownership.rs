// ownership example pgs. 69-70

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // returns length of string

    (s, length)
}

fn main() {
    // initialize string object
    let s1 = String::from("hello");

    // return tuple from calculate_length()
    // ownership is transferred from s1 to s2
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);
}
