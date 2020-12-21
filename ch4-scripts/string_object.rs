// Sting Object demo pg. 63
fn main() {
    // This is a string OBJECT therefore it can be changed.  A string literal cannot.
    let mut s = String::from("Hello");
    // Append to string s
    s.push_str(", world!");
    // print result
    println!("{}", s);
}
