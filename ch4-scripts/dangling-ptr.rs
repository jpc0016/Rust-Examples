// dangling pointers example pg. 74

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    // remove '&'
    let s = String::from("hello");
    //&s // '&s' creates an error because a pointer to 's' is returned but 's' goes out of scope
    // thus the memory to 's' is deallocated.  Rust does not allow this.
    /* Specifically:
        error[E0106]: missing lifetime specifier
         --> dangling-ptr.rs:7:16
          |
        7 | fn dangle() -> &String {
          |                ^ expected named lifetime parameter
          = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    help: consider using the `'static` lifetime
        */

    // Return string directly instead
    s
}
