// String slicing example pgs. 76-77
// Find first occurrence of a word in a string

// Access a memory location and check for words. Returns index of the end of the word
fn first_word(s: &str) -> &str {
    // convert string to byte array since each element will be checked if it is a space
    let bytes = s.as_bytes();

    // iter() is a method that returns each element in a collection and enumerate()
    // wraps the result of iter() and returns each element as part of a tuple instead.
    // first tuple element is the index and second is a reference to the element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //return i; // return index of the space
            return &s[..i];
        }
    }
    // return string length if no space is found
    //s.len();
    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    // word value is 5
    let word = first_word(&s);

    // returning indeces is tedious.  String slices are much easier to use!
    let hello = &s[0..5]; // &s[..5] is also valid
    let world = &s[6..11]; // &s[6..] is also valid
}
